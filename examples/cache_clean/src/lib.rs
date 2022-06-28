#[macro_use]
extern crate nginx_rs;
use nginx_rs::bindings::*;
use nginx_rs::core::*;
use nginx_rs::http::*;
use nginx_rs::{ ngx_modules, ngx_string, http_request_handler, ngx_null_command, ngx_log_debug_http };
use std::os::raw::{c_char, c_void};
use std::{ptr, slice};
use std::convert::TryInto;
use std::io;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(NgxModule)]
#[ngx_module_name(cache_clean)]
#[ngx_module_commands(
    CacheCleanCommand
    CacheCleanMethod,
    CacheCleanCommandDebug
)]
struct Module;

impl HTTPModule for Module {
    type MainConf = ();
    type SrvConf = ();
    type LocConf = LocConf;

    fn postconfiguration(cf: *mut ngx_conf_t) -> ngx_int_t {
        unsafe {
            let cmcf = ngx_http_conf_get_module_main_conf(cf, &ngx_http_core_module) as *mut ngx_http_core_main_conf_t;

            let h = ngx_array_push(&mut (*cmcf).phases[ngx_http_phases_NGX_HTTP_PRECONTENT_PHASE as usize].handlers) as *mut ngx_http_handler_pt;
            if h.is_null() {
                return ERROR.into();
            }

            *h = Some(ngx_http_cache_clean_access_handler);
        }

        OK.into()
    }
}

#[repr(C)]
struct PurgeCtx {
    cache_key: NgxStr,
    count: usize,
    debug: bool
}

http_request_handler!(ngx_http_cache_clean_access_handler, |request: &mut Request| {
    let hlcf = unsafe { request.get_module_loc_conf(&ngx_http_cache_clean_module) as *mut LocConf };
    let enabled = unsafe { (*hlcf).enabled };

    let mut purge_method = unsafe {
        String::from(&(*hlcf).purge_method)
    };

    if purge_method == "" {
        purge_method = "PURGE".to_string();
    }

    if !enabled || request.method().to_string_lossy() != purge_method {
        return DECLINED;
    }

    if !request.discard_request_body().is_ok() {
        return HTTP_INTERNAL_SERVER_ERROR.into();
    }

    let debug = unsafe { (*hlcf).debug };

    unsafe {
        let cache = request.get_module_loc_conf(&ngx_http_proxy_module) as *mut ngx_http_proxy_loc_conf_t;
        let cache_key = request.get_complex_value(&mut (*cache).cache_key as *mut ngx_http_complex_value_t).unwrap_or_else(|| {
            NgxStr::from("was void")
        });
        let cache_zone = unsafe { NgxStr::from_ngx_str((*(*cache).upstream.cache_zone).shm.name) };

        let pmcf = unsafe { request.get_module_main_conf(&ngx_http_proxy_module) as *mut ngx_http_proxy_main_conf_t };
        let caches = (*pmcf).caches.elts as *mut *mut ngx_http_file_cache_t;
        let count = (*pmcf).caches.nelts as ngx_uint_t;

        let mut t = *caches;
        let mut cache_path = NgxStr::from("");
        for i in 0..count {
            let _name = NgxStr::from_ngx_str((*(*t).shm_zone).shm.name);
            if _name == cache_zone {
                cache_path = NgxStr::from_ngx_str((*(*t).path).name);
                break;
            }
            t = t.add(1);
        }

        let mut data = PurgeCtx {
            cache_key,
            count: 0,
            debug
        };

        let mut tree: ngx_tree_ctx_t = ngx_tree_ctx_t {
            init_handler: None,
            file_handler: Some(ngx_http_purge_file_cache_delete_partial_file),
            pre_tree_handler: Some(ngx_http_purge_file_cache_noop),
            post_tree_handler: Some(ngx_http_purge_file_cache_noop),
            spec_handler: Some(ngx_http_purge_file_cache_noop),
            data: (&mut data) as *mut _ as *mut c_void,
            alloc: 0,
            log: (*request.connection()).log,
            access: 0,
            mtime: 0,
            fs_size: 0,
            size: 0
        };

        ngx_walk_tree(&mut tree as *mut ngx_tree_ctx_t, &mut cache_path.to_ngx_str() as *mut ngx_str_t);

        // Create body
        let body = format!("PURGED {} ITEMS FOR \"{}\"\n", data.count, cache_key);

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

        buf.set_last_buf(request.is_main());
        buf.set_last_in_chain(true);

        let mut out = ngx_chain_t { buf: buf.as_ngx_buf_mut(), next: ptr::null_mut() };
        request.output_filter(&mut out);

        DONE
    }
});

#[ngx_loc_conf]
/// Define location block configuration
struct LocConf {
    enabled: bool,
    purge_method: NgxStr,
    debug: bool,
}

impl Merge for LocConf {
    fn merge(&mut self, prev: &LocConf) {
        if !self.enabled {
            self.enabled = prev.enabled;
        }
        if !self.debug {
            self.debug = prev.debug;
        }
        if self.purge_method.is_empty() {
            self.purge_method = prev.purge_method;
        }
    }
}

#[ngx_command(cache_clean)]
struct CacheCleanCommand;

impl NgxCommand for CacheCleanCommand {
    const TYPE: u32 = NGX_HTTP_LOC_CONF|NGX_CONF_NOARGS;
    const CONF: ngx_uint_t = NGX_HTTP_LOC_CONF_OFFSET;

    fn handler (cf: *mut ngx_conf_t, _cmd: *mut ngx_command_t, conf: *mut c_void) -> *mut c_char {
        unsafe {
            let conf = &mut *(conf as *mut LocConf);

            if !conf.enabled {
                conf.enabled = true;
            }

            ptr::null_mut()
        }
    }
}

#[ngx_command(cache_clean_debug)]
struct CacheCleanCommandDebug;

impl NgxCommand for CacheCleanCommandDebug {
    const TYPE: u32 = NGX_HTTP_LOC_CONF|NGX_CONF_NOARGS;
    const CONF: ngx_uint_t = NGX_HTTP_LOC_CONF_OFFSET;

    fn handler (cf: *mut ngx_conf_t, _cmd: *mut ngx_command_t, conf: *mut c_void) -> *mut c_char {
        unsafe {
            let conf = &mut *(conf as *mut LocConf);

            if !conf.enabled {
                conf.enabled = true;
                conf.debug = true;
            }

            ptr::null_mut()
        }
    }
}

#[ngx_command(cache_clean_method)]
struct CacheCleanMethod;

impl NgxCommand for CacheCleanMethod {
    const TYPE: u32 = NGX_HTTP_LOC_CONF|NGX_CONF_TAKE1;
    const CONF: ngx_uint_t = NGX_HTTP_LOC_CONF_OFFSET;
    ngx_conf_set_str_slot!(LocConf, purge_method);
}

#[no_mangle]
unsafe extern "C" fn ngx_http_purge_file_cache_delete_partial_file(ctx: *mut ngx_tree_ctx_t, path: *mut ngx_str_t) -> ngx_int_t {
    let mut should_delete = false;
    let mut data = &mut *((*ctx).data as *mut PurgeCtx);
    let cache_key = data.cache_key;
    let debug = data.debug;

    if cache_key.len() == 0 {
        return NGX_OK.try_into().unwrap();
    }

    let partial_match = cache_key.to_string_lossy().chars().position(|c| c == '*');

    let _path = NgxStr::from_ngx_str(*path);

    let file = match File::open(_path.to_string()) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read title from {:?}", &path),
    };
    let mut reader = BufReader::new(file);
    let mut count = 0;
    let mut file_hash = String::new();

    loop {
        let mut buffer = String::new();
        reader.read_line(&mut buffer);

        count = count + 1;
        if buffer.len() > 5 && "KEY: " == &buffer[..5] {
            file_hash = String::from(buffer[5..].trim());
            should_delete = match partial_match {
                Some(pos) => &cache_key.to_string_lossy()[0..(pos - 1)] == &file_hash[0..(pos - 1)],
                None => cache_key == NgxStr::from(file_hash.as_str())
            };
            break;
        }
        if count == 3 {
            break;
        }
    }

    if should_delete {
        data.count = data.count + 1;
        let res = match std::fs::remove_file(_path.to_string()) {
            Ok(_) => "OK".to_string(),
            Err(err) => err.to_string()
        };
        if debug {
            ngx_log_debug!(NGX_LOG_DEBUG_HTTP, (*ctx).log,
                    "[cache_clean_module]: Deleting cache file \"{}\" (cache_key=\"{}\") - {}", _path, file_hash, res);
        }
    }

    NGX_OK.try_into().unwrap()
}

#[no_mangle]
unsafe extern "C" fn ngx_http_purge_file_cache_noop(ctx: *mut ngx_tree_ctx_t, path: *mut ngx_str_t) -> ngx_int_t {
    NGX_OK.try_into().unwrap()
}

ngx_modules!(ngx_http_cache_clean_module);

extern "C" {
    pub static ngx_http_proxy_module: ngx_module_t;
}
