use crate::bindings::*;
use crate::http::Request;
use crate::core::NgxStr;
use crate::ngx_null_string;
/// Representation of a borrowed [Nginx complex value].
///
/// [Nginx complex value]: <https://nginx.org/en/docs/dev/development_guide.html#http_complex_values>
#[repr(transparent)]
pub struct NgxComplexValue(pub ngx_http_complex_value_t);

impl NgxComplexValue {
    /// Yields a [`NgxStr`] with the [complex value] for a [`Request`]
    ///
    /// /// [complex value]: https://nginx.org/en/docs/dev/development_guide.html#http_complex_values
    pub fn get_value<'a>(&self, request: &'a Request) -> NgxStr {
      let cv = (self as *const NgxComplexValue as *mut NgxComplexValue) as *mut ngx_http_complex_value_t;
      request.get_complex_value(cv)
        .unwrap_or_else(|| unsafe {
          NgxStr::from_ngx_str(ngx_null_string!())
        })
    }
}

impl Copy for NgxComplexValue {}

impl Clone for NgxComplexValue {
  fn clone(&self) -> NgxComplexValue {
      *self
  }
}

#[macro_export]
macro_rules! ngx_http_set_complex_value_slot {
    ($struct:ident, $prop:ident) => {
      const OFFSET: ngx_uint_t = <$struct>::$prop;

      fn handler (cf: *mut ngx_conf_t, cmd: *mut ngx_command_t, conf: *mut c_void) -> *mut c_char {
          unsafe {
              ngx_http_set_complex_value_slot(cf, cmd, conf)
          }
      }
    };
}
