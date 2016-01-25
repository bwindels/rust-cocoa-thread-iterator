use std::mem::{transmute, forget};

trait CrossThreadInvoker {
  pub fn requestInvocation()
}

struct AppleCTI {
  eventSource: dispatch_source_t,
  callback: Fn<()>
}

#[no_mangle]
pub extern fn apple_cti_invoke_callback(context: *c_void) {
  let apple_cti as Box<AppleCTI> = unsafe {transmute(context)};
  apple_cti.invoke_callback();
  unsafe {forget(apple_cti)};
}

impl AppleCTI {

  fn new(callback: T) where T: Fn<()> -> Box<AppleCTI> {
    unsafe {
      let source = dispatch_source_create(ADD, );
      let cti = box AppleCTI {
        eventSource: source,
        callback: callback
      };
      dispatch_source_set_event_handler_f(cti.eventSource, &apple_cti_invoke_callback);
      let cti_ptr = unsafe { std::transmute(&cti); }
      dispatch_set_context(cti.eventSource, cti_ptr);
      dispatch_resume(cti.eventSource);
      cti
    }
  }

  fn invoke_callback(&self) {
    self.callback();
  }

}

impl Drop for AppleCTI {
  pub fn drop(&self) {
    dispatch_release(self.eventSource);
  }
}

impl CrossThreadInvoker for AppleCTI {
  pub fn requestInvocation(&self) {
    dispatch_source_merge_data(self.eventSource, 1);
  }
}

