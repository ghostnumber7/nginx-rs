use crate::bindings::*;
use crate::core::Status;

pub struct HTTPStatus(pub ngx_uint_t);

impl From<HTTPStatus> for Status {
    fn from(s: HTTPStatus) -> Status {
        Status(s.0 as ngx_int_t)
    }
}

impl From<HTTPStatus> for ngx_uint_t {
    fn from(s: HTTPStatus) -> ngx_uint_t {
        s.0
    }
}

/// Nginx HTTP Status code constants
/// DOCS: https://www.nginx.com/resources/wiki/extending/api/http/
pub const HTTP_CONTINUE: HTTPStatus = HTTPStatus(NGX_HTTP_CONTINUE as ngx_uint_t);
pub const HTTP_SWITCHING_PROTOCOLS: HTTPStatus = HTTPStatus(NGX_HTTP_SWITCHING_PROTOCOLS as ngx_uint_t);
pub const HTTP_PROCESSING: HTTPStatus = HTTPStatus(NGX_HTTP_PROCESSING as ngx_uint_t);
pub const HTTP_OK: HTTPStatus = HTTPStatus(NGX_HTTP_OK as ngx_uint_t);
pub const HTTP_CREATED: HTTPStatus = HTTPStatus(NGX_HTTP_CREATED as ngx_uint_t);
pub const HTTP_ACCEPTED: HTTPStatus = HTTPStatus(NGX_HTTP_ACCEPTED as ngx_uint_t);
pub const HTTP_NO_CONTENT: HTTPStatus = HTTPStatus(NGX_HTTP_NO_CONTENT as ngx_uint_t);
pub const HTTP_PARTIAL_CONTENT: HTTPStatus = HTTPStatus(NGX_HTTP_PARTIAL_CONTENT as ngx_uint_t);
pub const HTTP_SPECIAL_RESPONSE: HTTPStatus = HTTPStatus(NGX_HTTP_SPECIAL_RESPONSE as ngx_uint_t);
pub const HTTP_MOVED_PERMANENTLY: HTTPStatus = HTTPStatus(NGX_HTTP_MOVED_PERMANENTLY as ngx_uint_t);
pub const HTTP_MOVED_TEMPORARILY: HTTPStatus = HTTPStatus(NGX_HTTP_MOVED_TEMPORARILY as ngx_uint_t);
pub const HTTP_SEE_OTHER: HTTPStatus = HTTPStatus(NGX_HTTP_SEE_OTHER as ngx_uint_t);
pub const HTTP_NOT_MODIFIED: HTTPStatus = HTTPStatus(NGX_HTTP_NOT_MODIFIED as ngx_uint_t);
pub const HTTP_TEMPORARY_REDIRECT: HTTPStatus = HTTPStatus(NGX_HTTP_TEMPORARY_REDIRECT as ngx_uint_t);
pub const HTTP_BAD_REQUEST: HTTPStatus = HTTPStatus(NGX_HTTP_BAD_REQUEST as ngx_uint_t);
pub const HTTP_UNAUTHORIZED: HTTPStatus = HTTPStatus(NGX_HTTP_UNAUTHORIZED as ngx_uint_t);
pub const HTTP_FORBIDDEN: HTTPStatus = HTTPStatus(NGX_HTTP_FORBIDDEN as ngx_uint_t);
pub const HTTP_NOT_FOUND: HTTPStatus = HTTPStatus(NGX_HTTP_NOT_FOUND as ngx_uint_t);
pub const HTTP_NOT_ALLOWED: HTTPStatus = HTTPStatus(NGX_HTTP_NOT_ALLOWED as ngx_uint_t);
pub const HTTP_REQUEST_TIME_OUT: HTTPStatus = HTTPStatus(NGX_HTTP_REQUEST_TIME_OUT as ngx_uint_t);
pub const HTTP_CONFLICT: HTTPStatus = HTTPStatus(NGX_HTTP_CONFLICT as ngx_uint_t);
pub const HTTP_LENGTH_REQUIRED: HTTPStatus = HTTPStatus(NGX_HTTP_LENGTH_REQUIRED as ngx_uint_t);
pub const HTTP_PRECONDITION_FAILED: HTTPStatus = HTTPStatus(NGX_HTTP_PRECONDITION_FAILED as ngx_uint_t);
pub const HTTP_REQUEST_ENTITY_TOO_LARGE: HTTPStatus = HTTPStatus(NGX_HTTP_REQUEST_ENTITY_TOO_LARGE as ngx_uint_t);
pub const HTTP_REQUEST_URI_TOO_LARGE: HTTPStatus = HTTPStatus(NGX_HTTP_REQUEST_URI_TOO_LARGE as ngx_uint_t);
pub const HTTP_UNSUPPORTED_MEDIA_TYPE: HTTPStatus = HTTPStatus(NGX_HTTP_UNSUPPORTED_MEDIA_TYPE as ngx_uint_t);
pub const HTTP_RANGE_NOT_SATISFIABLE: HTTPStatus = HTTPStatus(NGX_HTTP_RANGE_NOT_SATISFIABLE as ngx_uint_t);
pub const HTTP_TOO_MANY_REQUESTS: HTTPStatus = HTTPStatus(NGX_HTTP_TOO_MANY_REQUESTS as ngx_uint_t);
pub const HTTP_CLOSE: HTTPStatus = HTTPStatus(NGX_HTTP_CLOSE as ngx_uint_t);
pub const HTTP_NGINX_CODES: HTTPStatus = HTTPStatus(NGX_HTTP_NGINX_CODES as ngx_uint_t);
pub const HTTP_REQUEST_HEADER_TOO_LARGE: HTTPStatus = HTTPStatus(NGX_HTTP_REQUEST_HEADER_TOO_LARGE as ngx_uint_t);
pub const HTTPS_CERT_ERROR: HTTPStatus = HTTPStatus(NGX_HTTPS_CERT_ERROR as ngx_uint_t);
pub const HTTPS_NO_CERT: HTTPStatus = HTTPStatus(NGX_HTTPS_NO_CERT as ngx_uint_t);
pub const HTTP_TO_HTTPS: HTTPStatus = HTTPStatus(NGX_HTTP_TO_HTTPS as ngx_uint_t);
pub const HTTP_CLIENT_CLOSED_REQUEST: HTTPStatus = HTTPStatus(NGX_HTTP_CLIENT_CLOSED_REQUEST as ngx_uint_t);
pub const HTTP_INTERNAL_SERVER_ERROR: HTTPStatus = HTTPStatus(NGX_HTTP_INTERNAL_SERVER_ERROR as ngx_uint_t);
pub const HTTP_NOT_IMPLEMENTED: HTTPStatus = HTTPStatus(NGX_HTTP_NOT_IMPLEMENTED as ngx_uint_t);
pub const HTTP_BAD_GATEWAY: HTTPStatus = HTTPStatus(NGX_HTTP_BAD_GATEWAY as ngx_uint_t);
pub const HTTP_SERVICE_UNAVAILABLE: HTTPStatus = HTTPStatus(NGX_HTTP_SERVICE_UNAVAILABLE as ngx_uint_t);
pub const HTTP_GATEWAY_TIME_OUT: HTTPStatus = HTTPStatus(NGX_HTTP_GATEWAY_TIME_OUT as ngx_uint_t);
pub const HTTP_INSUFFICIENT_STORAGE: HTTPStatus = HTTPStatus(NGX_HTTP_INSUFFICIENT_STORAGE as ngx_uint_t);
