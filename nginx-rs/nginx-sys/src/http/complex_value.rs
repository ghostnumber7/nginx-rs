use crate::bindings::*;
use crate::http::Request;
use crate::core::NgxStr;
use crate::ngx_null_string;

/// Representation of a borrowed [Nginx complex value].
///
/// [Nginx complex value]: <https://nginx.org/en/docs/dev/development_guide.html#http_complex_values>
pub struct NgxComplexValue(pub *mut ngx_http_complex_value_t);

impl NgxComplexValue {
    /// Checks if we have a `null` pointer reference
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Yields a [`NgxStr`] with the [complex value] for a [`Request`]
    ///
    /// /// [complex value]: https://nginx.org/en/docs/dev/development_guide.html#http_complex_values
    pub fn get_value<'a>(&self, request: &'a Request) -> &'a NgxStr {
      if self.is_null() {
        unsafe {
          return NgxStr::from_ngx_str(ngx_null_string!());
        }
      }
      request.get_complex_value(self.0)
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

impl Default for NgxComplexValue {
  fn default() -> NgxComplexValue {
    NgxComplexValue(std::ptr::null_mut())
  }
}

impl From<NgxComplexValue> for *mut ngx_http_complex_value_t {
    fn from(s: NgxComplexValue) -> *mut ngx_http_complex_value_t {
        s.0
    }
}
