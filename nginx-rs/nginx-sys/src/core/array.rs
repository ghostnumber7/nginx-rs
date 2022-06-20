use crate::bindings::*;
use crate::core::NgxStr;
use std::{ops::{Index, IndexMut}, usize};

#[derive(Clone, Debug)]
pub struct NgxArray(pub Vec<NgxStr>);

impl NgxArray {
  pub fn from_ngx_array_t (arr: ngx_array_t) -> NgxArray {
    let mut ret = Vec::new();
    let mut t = arr.elts as *mut ngx_str_t;
    if t.is_null() {
        return NgxArray(ret);
    }
    let nelts: usize = arr.nelts;
    for _ in 1..nelts {
      if t.is_null() {
        break;
      }
      t = unsafe { t.add(1) };
      unsafe {
        ret.push(NgxStr::from_ngx_str(*t));
      }
    }
    NgxArray(ret)
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn as_vec(&self) -> Vec<NgxStr> {
    self.0.clone()
  }

  pub fn get(&self, i: usize) -> Option<&NgxStr> {
    (&self.0).get(i)
  }
}

impl From<ngx_array_t> for NgxArray {
  fn from(arr: ngx_array_t) -> Self {
    NgxArray::from_ngx_array_t(arr)
  }
}

impl From<NgxArray> for Vec<NgxStr> {
  fn from(arr: NgxArray) -> Self {
    arr.0
  }
}

impl Index<usize> for NgxArray {
  type Output = NgxStr;
  fn index(&self, s: usize) -> &NgxStr {
    &self.0[s]
  }
}

impl IndexMut<usize> for NgxArray {
  fn index_mut(&mut self, s: usize) -> &mut NgxStr {
    &mut self.0[s]
  }
}

#[macro_export]
macro_rules! ngx_conf_set_str_array_slot {
    ($struct:ident, $prop:ident) => {
      fn handler (cf: *mut ngx_conf_t, cmd: *mut ngx_command_t, conf: *mut c_void) -> *mut c_char {
        unsafe {
            let conf = &mut *(conf as *mut $struct);
            conf.$prop = NgxArray::from(*(*cf).args);
        }
        ptr::null_mut()
      }
    };
}
