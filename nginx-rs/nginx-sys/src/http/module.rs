use crate::bindings::*;
use crate::core::*;

use std::os::raw::{c_void, c_char};
use core::ptr;

pub trait Merge {
    fn merge(&mut self, prev: &Self);
}

impl Merge for () {
    fn merge(&mut self, _prev: &Self) {}
}

pub trait NgxCommand {
    const TYPE: u32;
    const CONF: ngx_uint_t;
    const OFFSET: ngx_uint_t = 0;
    const POST: *mut c_void = ptr::null_mut();

    fn handler (cf: *mut ngx_conf_t, cmd: *mut ngx_command_t, conf: *mut c_void) -> *mut c_char;
}

pub trait HTTPModuleContext {
    fn get_http_main_conf_ (_cf: *mut ngx_conf_t) -> *mut c_void {
        ptr::null_mut()
    }

    fn get_http_srv_conf_ (_cf: *mut ngx_conf_t) -> *mut c_void {
        ptr::null_mut()
    }

    fn get_http_loc_conf_ (_cf: *mut ngx_conf_t) -> *mut c_void {
        ptr::null_mut()
    }
}

pub trait HTTPModule: HTTPModuleContext {
    type MainConf; // + crate::const_field_offset::FieldOffsets;
    type SrvConf: Merge; // + crate::const_field_offset::FieldOffsets;
    type LocConf: Merge; // + crate::const_field_offset::FieldOffsets;

    fn get_http_main_conf<'a> (_cf: *mut ngx_conf_t) -> &'a mut Self::MainConf {
        unsafe { &mut *(Self::get_http_main_conf_(_cf) as *mut Self::MainConf) }
    }

    fn get_http_srv_conf<'a> (_cf: *mut ngx_conf_t) -> &'a mut Self::SrvConf {
        unsafe { &mut *(Self::get_http_srv_conf_(_cf) as *mut Self::SrvConf) }
    }

    fn get_http_loc_conf<'a> (_cf: *mut ngx_conf_t) -> &'a mut Self::LocConf {
        unsafe { &mut *(Self::get_http_loc_conf_(_cf) as *mut Self::LocConf) }
    }

    unsafe extern "C" fn preconfiguration(_cf: *mut ngx_conf_t) -> ngx_int_t {
        OK.into()
    }

    fn postconfiguration(_cf: *mut ngx_conf_t) -> ngx_int_t {
        OK.into()
    }

    unsafe extern "C" fn postconfiguration_(_cf: *mut ngx_conf_t) -> ngx_int_t {
        Some(Self::postconfiguration(_cf)).unwrap_or_else(|| ERROR.into())
    }

    unsafe extern "C" fn create_main_conf(cf: *mut ngx_conf_t) -> *mut c_void {
        let mut pool = Pool::from_ngx_pool((*cf).pool);
        // pool.allocate::<Self::MainConf>(Default::default()) as *mut c_void
        pool.allocate_size::<Self::MainConf>() as *mut c_void
    }

    unsafe extern "C" fn init_main_conf(_cf: *mut ngx_conf_t, _conf: *mut c_void) -> *mut c_char {
        ptr::null_mut()
    }

    unsafe extern "C" fn create_srv_conf(cf: *mut ngx_conf_t) -> *mut c_void {
        let mut pool = Pool::from_ngx_pool((*cf).pool);
        // pool.allocate::<Self::SrvConf>(Default::default()) as *mut c_void
        pool.allocate_size::<Self::SrvConf>() as *mut c_void
    }

    unsafe extern "C" fn merge_srv_conf(_cf: *mut ngx_conf_t, prev: *mut c_void, conf: *mut c_void) -> *mut c_char {
        let prev = &mut *(prev as *mut Self::SrvConf);
        let conf = &mut *(conf as *mut Self::SrvConf);
        conf.merge(prev);
        ptr::null_mut()
    }

    unsafe extern "C" fn create_loc_conf(cf: *mut ngx_conf_t) -> *mut c_void {
        let mut pool = Pool::from_ngx_pool((*cf).pool);
        // pool.allocate::<Self::LocConf>(Default::default()) as *mut c_void
        pool.allocate_size::<Self::LocConf>() as *mut c_void
    }

    unsafe extern "C" fn merge_loc_conf(_cf: *mut ngx_conf_t, prev: *mut c_void, conf: *mut c_void) -> *mut c_char {
        let prev = &mut *(prev as *mut Self::LocConf);
        let conf = &mut *(conf as *mut Self::LocConf);
        conf.merge(prev);
        ptr::null_mut()
    }
}
