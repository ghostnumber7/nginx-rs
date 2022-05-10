use crate::bindings::*;

use std::slice;
use std::str::{self, Utf8Error};
use std::borrow::Cow;

/// Static string initializer for [`ngx_str_t`].
///
/// The resulting byte string is always nul-terminated (just like a C string).
///
/// [`ngx_str_t`]: https://nginx.org/en/docs/dev/development_guide.html#string_overview
#[macro_export]
macro_rules! ngx_string {
    ($s:expr) => {
        {
            ngx_str_t { len: $s.len(), data: concat!($s, "\0").as_ptr() as *mut u8 }
        }
    };
}

/// Static empty string initializer for [`ngx_str_t`].
///
/// [`ngx_str_t`]: https://nginx.org/en/docs/dev/development_guide.html#string_overview
#[macro_export]
macro_rules! ngx_null_string {
    () => {
        ngx_str_t { len: 0, data: ::std::ptr::null_mut() }
    };
}

/// Representation of a borrowed [Nginx string].
///
/// [Nginx string]: <https://nginx.org/en/docs/dev/development_guide.html#string_overview>
#[repr(transparent)]
pub struct NgxStr(pub ngx_str_t);

impl NgxStr {
    /// Create an [`NgxStr`] from an [`ngx_str_t`].
    ///
    /// [`ngx_str_t`]: <https://nginx.org/en/docs/dev/development_guide.html#string_overview>
    ///
    /// # Safety
    ///
    /// The caller has provided a valid `ngx_str_t` with a `data` pointer that points
    /// to range of bytes of at least `len` bytes, whose content remains valid and doesn't
    /// change for the lifetime of the returned `NgxStr`.
    pub unsafe fn from_ngx_str(str: ngx_str_t) -> NgxStr {
        NgxStr(str)
    }

    /// Access the [`NgxStr`] as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.0.data, self.0.len).into()
        }
    }

    /// Yields a `&str` slice if the [`NgxStr`] contains valid UTF-8.
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.as_bytes())
    }

    /// Converts an [`NgxStr`] into a [`Cow<str>`], replacing invalid UTF-8 sequences.
    ///
    /// See [`String::from_utf8_lossy`].
    pub fn to_string_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(self.as_bytes())
    }

    /// Returns `true` if the [`NgxStr`] is empty, otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.0.len == 0
    }
}

impl Copy for NgxStr {}

impl Clone for NgxStr {
  fn clone(&self) -> NgxStr {
      *self
  }
}

impl From<&[u8]> for NgxStr {
    fn from(bytes: &[u8]) -> Self {
        NgxStr(ngx_str_t { len: bytes.len(), data: bytes.as_ptr() as *mut u8 })
    }
}

impl From<&str> for NgxStr {
    fn from(s: &str) -> Self {
        s.as_bytes().into()
    }
}

impl From<&NgxStr> for String {
    fn from(s: &NgxStr) -> Self {
        if let Ok(res) = s.to_str() {
            return String::from(res);
        }
        String::new()
    }
}

impl AsRef<[u8]> for NgxStr {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Default for NgxStr {
    fn default() -> Self {
        // SAFETY: The null `ngx_str_t` is always a valid Nginx string.
        unsafe {
            NgxStr::from_ngx_str(ngx_null_string!())
        }
    }
}
