use crate::bindings::*;

use std::os::raw::c_void;

/// A macro which gets the module’s [http core configuration object] from the configuration object.
///
/// [http core configuration object]: <https://www.nginx.com/resources/wiki/extending/api/configuration/#ngx-http-conf-get-module-main-conf>
///
/// # Safety
/// The caller has provided a valid non-null pointer to a valid `ngx_conf_t` and `ngx_module_t` reference.
pub unsafe fn ngx_http_conf_get_module_main_conf(cf: *mut ngx_conf_t, module: &ngx_module_t)  -> *mut c_void {
    let http_conf_ctx = (*cf).ctx as *mut ngx_http_conf_ctx_t;
    *(*http_conf_ctx).main_conf.add(module.ctx_index)
}

/// A macro which gets the module’s [http server block configuration object] from the configuration object
///
/// [http server block configuration object]: <https://www.nginx.com/resources/wiki/extending/api/configuration/#ngx-http-conf-get-module-srv-conf>
///
/// # Safety
/// The caller has provided a valid non-null pointer to a valid `ngx_conf_t` and `ngx_module_t` reference.
pub unsafe fn ngx_http_conf_get_module_srv_conf(cf: *mut ngx_conf_t, module: &ngx_module_t)  -> *mut c_void {
    let http_conf_ctx = (*cf).ctx as *mut ngx_http_conf_ctx_t;
    *(*http_conf_ctx).srv_conf.add(module.ctx_index)
}

/// A macro which gets the module’s [http core configuration object] from the request object
///
/// [http core configuration object]: <https://www.nginx.com/resources/wiki/extending/api/configuration/#ngx-http-conf-get-module-loc-conf>
///
/// # Safety
/// The caller has provided a valid non-null pointer to a valid `ngx_conf_t` and `ngx_module_t` reference.
pub unsafe fn ngx_http_conf_get_module_loc_conf(cf: *mut ngx_conf_t, module: &ngx_module_t)  -> *mut c_void {
    let http_conf_ctx = (*cf).ctx as *mut ngx_http_conf_ctx_t;
    *(*http_conf_ctx).loc_conf.add(module.ctx_index)
}
