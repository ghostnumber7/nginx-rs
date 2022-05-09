use crate::bindings::*;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct Status(pub ngx_int_t);

impl Status {
    pub fn is_ok(&self) -> bool {
        self == &OK
    }
}

impl From<Status> for ngx_int_t {
    fn from(s: Status) -> ngx_int_t {
        s.0
    }
}

/// Nginx common return codes
/// DOC: https://nginx.org/en/docs/dev/development_guide.html#common_return_codes
pub const OK: Status = Status(NGX_OK as ngx_int_t);
pub const ERROR: Status = Status(NGX_ERROR as ngx_int_t);
pub const AGAIN: Status = Status(NGX_AGAIN as ngx_int_t);
pub const DECLINED: Status = Status(NGX_DECLINED as ngx_int_t);
pub const BUSY: Status = Status(NGX_BUSY as ngx_int_t);
pub const DONE: Status = Status(NGX_DONE as ngx_int_t);
pub const ABORT: Status = Status(NGX_ABORT as ngx_int_t);
