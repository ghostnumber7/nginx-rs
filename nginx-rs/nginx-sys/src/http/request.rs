use crate::{ bindings::*, ngx_null_string };
use crate::core::*;

use crate::http::status::*;

use std::os::raw::c_void;

/// Define a static request handler.
///
/// Handlers are expected to take a single [`Request`] argument and return a [`Status`].
#[macro_export]
macro_rules! http_request_handler {
    ( $name: ident, $handler: expr ) => {
        #[no_mangle]
        extern "C" fn $name(r: *mut ngx_http_request_t) -> ngx_int_t {
            let status: Status = $handler(unsafe { &mut $crate::http::Request::from_ngx_http_request(r) });
            status.0
        }
    };
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Request(ngx_http_request_t);

impl Request {
    /// Create a [`Request`] from an [`ngx_http_request_t`].
    ///
    /// [`ngx_http_request_t`]: https://nginx.org/en/docs/dev/development_guide.html#http_request
    ///
    /// # Safety
    /// The caller has provided a valid non-null pointer to a valid `ngx_http_request_t`
    /// which shares the same representation as `Request`.
    pub unsafe fn from_ngx_http_request<'a>(r: *mut ngx_http_request_t) -> &'a mut Request {
        &mut *r.cast::<Request>()
    }

    /// Is this the main request (as opposed to a subrequest)?
    pub fn is_main(&self) -> bool {
        let main = self.0.main.cast();
        std::ptr::eq(self, main)
    }

    /// Request pool.
    pub fn pool(&self) -> Pool {
        // SAFETY: This request is allocated from `pool`, thus must be a valid pool.
        unsafe {
            Pool::from_ngx_pool(self.0.pool)
        }
    }

    /// Pointer to a [`ngx_http_cache_t`] client connection object.
    ///
    /// [`ngx_http_cache_t`]: https://nginx.org/en/docs/dev/development_guide.html#cache
    pub fn cache(&self) -> *mut ngx_http_cache_t {
        self.0.cache
    }

    // modules, modules_n

    /// Pointer to a [`ngx_connection_t`] client connection object.
    ///
    /// [`ngx_connection_t`]: https://nginx.org/en/docs/dev/development_guide.html#connection
    pub fn connection(&self) -> *mut ngx_connection_t {
        self.0.connection
    }

    pub fn upstream(&self) -> *mut ngx_http_upstream_t {
        self.0.upstream
    }

    /// Module main configuration.
    pub fn get_module_main_conf(&self, module: &ngx_module_t) -> *mut c_void {
        unsafe {
            *self.0.main_conf.add(module.ctx_index)
        }
    }

    /// Module server configuration.
    pub fn get_module_srv_conf(&self, module: &ngx_module_t) -> *mut c_void {
        unsafe {
            *self.0.srv_conf.add(module.ctx_index)
        }
    }

    /// Module location configuration.
    pub fn get_module_loc_conf(&self, module: &ngx_module_t) -> *mut c_void {
        unsafe {
            *self.0.loc_conf.add(module.ctx_index)
        }
    }

    /// Get the value of a [complex value].
    ///
    /// [complex value]: https://nginx.org/en/docs/dev/development_guide.html#http_complex_values
    pub fn get_complex_value(&self, cv: *mut ngx_http_complex_value_t) -> Option<NgxStr> {
        let r = (self as *const Request as *mut Request).cast();

        // SAFETY: `ngx_http_complex_value` does not mutate `r` or `val` and guarentees that
        // a valid Nginx string is stored in `value` if it successfully returns.
        unsafe {
            let mut value = ngx_null_string!();

            if !cv.is_null() {
                let val = cv as *const ngx_http_complex_value_t as *mut ngx_http_complex_value_t;
                if ngx_http_complex_value(r, val, &mut value) != NGX_OK as ngx_int_t {
                    return None;
                }
            }

            Some(NgxStr::from_ngx_str(value))
        }
    }

    /// Discard (read and ignore) the [request body].
    ///
    /// [request body]: https://nginx.org/en/docs/dev/development_guide.html#http_request_body
    pub fn discard_request_body(&mut self) -> Status
    {
        unsafe {
            Status(ngx_http_discard_request_body(&mut self.0))
        }
    }

    /// Client HTTP [User-Agent].
    ///
    /// [User-Agent]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent
    pub fn user_agent(&self) -> NgxStr {
        unsafe {
            NgxStr::from_ngx_str((*self.0.headers_in.user_agent).value)
        }
    }

    /// Client HTTP [Method].
    ///
    /// [Method]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
    pub fn method(&self) -> NgxStr {
        unsafe {
            NgxStr::from_ngx_str(self.0.method_name)
        }
    }

    /// Set HTTP status of response.
    pub fn set_status(&mut self, status: HTTPStatus) {
        self.0.headers_out.status = status.into();
    }

    /// Set response body [Content-Length].
    ///
    /// [Content-Length]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Length
    pub fn set_content_length_n(&mut self, n: usize) {
        self.0.headers_out.content_length_n = n as off_t;
    }

    /// Send the output header.
    ///
    /// Do not call this function until all output headers are set.
    pub fn send_header(&mut self) -> Status {
        unsafe {
            Status(ngx_http_send_header(&mut self.0))
        }
    }

    /// Flag indicating that the output does not require a body.
    ///
    /// For example, this flag is used by `HTTP HEAD` requests.
    pub fn header_only(&self) -> bool {
        self.0.header_only() != 0
    }

    /// Send the [response body].
    ///
    /// This function can be called multiple times.
    /// Set the `last_buf` flag in the last body buffer.
    ///
    /// [response body]: https://nginx.org/en/docs/dev/development_guide.html#http_request_body
    pub fn output_filter(&mut self, body: &mut ngx_chain_t) -> Status {
        unsafe {
            Status(ngx_http_output_filter(&mut self.0, body))
        }
    }
}
