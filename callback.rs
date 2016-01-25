use std::ops::Fn;
use std:ffi:*;

struct Callback<T> {
  context: *c_void,
  callback: extern fn (value: T, context: *c_void)
}

impl<T> Callback<T> {
  pub fn new(callback: extern fn(value: T, context: *c_void), context: *c_void) {
    Callback {
      callback: callback,
      context: context
    }
  }
}

impl<T> Fn<T, ()> for Callback<T> {
  pub fn call(&self, value: T) {
    unsafe {
      self.callback(value, self.context);
    }
  }
}
