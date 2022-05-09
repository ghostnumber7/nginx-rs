#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(improper_ctypes)] // warning: `extern` block uses type `[u64; 4]`, which is not FFI-safe

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// export constants with original names to keep API standard
pub const NGX_HTTP_LOC_CONF_OFFSET: ngx_uint_t = NGX_RS_HTTP_LOC_CONF_OFFSET;
pub const NGX_HTTP_MAIN_CONF_OFFSET: ngx_uint_t = NGX_RS_HTTP_MAIN_CONF_OFFSET;
pub const NGX_HTTP_SRV_CONF_OFFSET: ngx_uint_t = NGX_RS_HTTP_SRV_CONF_OFFSET;
pub const NGX_MAIL_MAIN_CONF_OFFSET: ngx_uint_t = NGX_RS_MAIL_MAIN_CONF_OFFSET;
pub const NGX_MAIL_SRV_CONF_OFFSET: ngx_uint_t = NGX_RS_MAIL_SRV_CONF_OFFSET;
pub const NGX_STREAM_MAIN_CONF_OFFSET: ngx_uint_t = NGX_RS_STREAM_MAIN_CONF_OFFSET;
pub const NGX_STREAM_SRV_CONF_OFFSET: ngx_uint_t = NGX_RS_STREAM_SRV_CONF_OFFSET;
