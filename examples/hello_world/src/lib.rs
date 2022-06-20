#[macro_use]
extern crate nginx_rs;
use nginx_rs::bindings::*;
use nginx_rs::core::*;
use nginx_rs::http::*;
use nginx_rs::{ ngx_modules, ngx_string, http_request_handler, ngx_null_command, ngx_log_debug_http };
use std::os::raw::{c_char, c_void};
use std::ptr;

#[derive(NgxModule)]
#[ngx_module_name(hello_world)]
#[ngx_module_commands(
    HelloWorldCommand,
    HelloWorldTextCommand,
    HelloWorldMultipleCommand
)]
struct Module;

impl HTTPModule for Module {
    type MainConf = ();
    type SrvConf = ();
    type LocConf = LocConf;

    fn postconfiguration(cf: *mut ngx_conf_t) -> ngx_int_t {
        unsafe {
            let cmcf = ngx_http_conf_get_module_main_conf(cf, &ngx_http_core_module) as *mut ngx_http_core_main_conf_t;

            let h = ngx_array_push(&mut (*cmcf).phases[ngx_http_phases_NGX_HTTP_ACCESS_PHASE as usize].handlers) as *mut ngx_http_handler_pt;
            if h.is_null() {
                return ERROR.into();
            }

            *h = Some(ngx_http_hello_world_access_handler);
        }

        OK.into()
    }
}

/// Define location block configuration
#[ngx_loc_conf]
struct LocConf {
    text: *mut NgxComplexValue,
    enabled: bool,
    multiple_value: NgxArray,
}

impl Merge for LocConf {
    fn merge(&mut self, prev: &LocConf) {
        if self.text.is_null() && !prev.text.is_null() {
            self.text = prev.text;
        }

        if !self.enabled {
            self.enabled = prev.enabled;
        }

        if self.multiple_value.is_empty() && !prev.multiple_value.is_empty() {
            self.multiple_value = prev.multiple_value.clone();
        }
    }
}

#[ngx_command(hello_world)]
struct HelloWorldCommand;

impl NgxCommand for HelloWorldCommand {
    const TYPE: u32 = NGX_HTTP_LOC_CONF|NGX_CONF_NOARGS;
    const CONF: ngx_uint_t = NGX_HTTP_LOC_CONF_OFFSET;

    fn handler (cf: *mut ngx_conf_t, _cmd: *mut ngx_command_t, conf: *mut c_void) -> *mut c_char {
        unsafe {
            let conf = &mut *(conf as *mut LocConf);

            if !conf.enabled {
                conf.enabled = true;
                let clcf = ngx_http_conf_get_module_loc_conf(cf, &ngx_http_core_module) as *mut ngx_http_core_loc_conf_t;
                (*clcf).handler = Some(ngx_http_hello_world_handler);
            }

            ptr::null_mut()
        }
    }
}

#[ngx_command(hello_world_text)]
struct HelloWorldTextCommand;

impl NgxCommand for HelloWorldTextCommand {
    const TYPE: u32 = NGX_HTTP_MAIN_CONF|NGX_HTTP_LOC_CONF|NGX_CONF_TAKE1;
    const CONF: ngx_uint_t = NGX_HTTP_LOC_CONF_OFFSET;
    ngx_http_set_complex_value_slot!(LocConf, text);
}

#[ngx_command(hello_world_multiple)]
struct HelloWorldMultipleCommand;

impl NgxCommand for HelloWorldMultipleCommand {
    const TYPE: u32 = NGX_HTTP_LOC_CONF|NGX_CONF_TAKE4;
    const CONF: ngx_uint_t = NGX_HTTP_LOC_CONF_OFFSET;
    ngx_conf_set_str_array_slot!(LocConf, multiple_value);
}

http_request_handler!(ngx_http_hello_world_access_handler, |request: &mut Request| {
    let hlcf = unsafe { request.get_module_loc_conf(&ngx_http_hello_world_module) as *mut LocConf };
    let enabled = unsafe { (*hlcf).enabled };

    if enabled {
        unsafe {
            let items = &(*hlcf).multiple_value;
            ngx_log_debug_http!(request, "called access_handler. method = {} items={:#?}", request.method().to_string_lossy(), items);
        }
    }

    OK
});

http_request_handler!(ngx_http_hello_world_handler, |request: &mut Request| {
    ngx_log_debug_http!(request, "http hello_world handler");

    // Ignore client request body if any
    if !request.discard_request_body().is_ok() {
        return HTTP_INTERNAL_SERVER_ERROR.into();
    }

    let hlcf = unsafe { request.get_module_loc_conf(&ngx_http_hello_world_module) as *mut LocConf };

    // get text complex value
    let text = unsafe {
        let text = (*hlcf).text;
        if text.is_null() {
            NgxStr::from_ngx_str(ngx_null_string!())
        } else {
            (*text).get_value(request)
        }
    };

    // Create body
    let user_agent = request.user_agent();
    let body = format!("Hello, {}!\n", if text.is_empty() {
        user_agent.to_string_lossy()
    } else {
        text.to_string_lossy()
    });

    // Send header
    request.set_status(HTTP_OK);
    request.set_content_length_n(body.len());
    let status = request.send_header();
    if status == ERROR || status > OK || request.header_only() {
        return status;
    }

    // Send body
    let mut buf = match request.pool().create_buffer_from_str(&body) {
        Some(buf) => buf,
        None => return HTTP_INTERNAL_SERVER_ERROR.into(),
    };
    assert!(&buf.as_bytes()[..7] == b"Hello, ");
    buf.set_last_buf(request.is_main());
    buf.set_last_in_chain(true);

    let mut out = ngx_chain_t { buf: buf.as_ngx_buf_mut(), next: ptr::null_mut() };
    request.output_filter(&mut out)
});

ngx_modules!(ngx_http_hello_world_module);
