use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[link(name = "libdispatch")]
extern {
  fn dispatch_get_main_queue() -> dispatch_queue_t;
  fn dispatch_source_create(type: dispatch_source_type_t, handle: uintptr_t, mask: u64, queue: dispatch_queue_t) -> dispatch_source_t;
  fn dispatch_source_merge_data(source: dispatch_source_t, data: u64);
  fn dispatch_source_get_data(source: dispatch_source_t) -> u64;
}

#[no_mangle]
pub extern fn create_iterator(min: i32, max: i32, callback: extern fn(n: i32, context: *c_void), context: *c_void) -> *mut ThreadedIterator {
  let (it, tx) = ThreadedIterator::create(callback, context);
  let it_ptr = unsafe {transmute(it)};
  load_data_into_iterator(tx, min, max, || {wakeup_ui_thread(it_ptr);});
  it_ptr
}

fn load_data_into_iterator<F>(tx: Sender<i32>, min: i32, max: i32, send_callback: F) where F: Fn() {
  thread::spawn(move || {
    for n in min..max {
      tx.send(n).unwrap();
      send_callback();
      thread::sleep(Duration::from_millis(500));
    }
  });
}

fn wakeup_ui_thread(iterator: *mut ThreadedIterator) {
  unsafe {
    dispatch_async_f(dispatch_get_main_queue(), iterator, drain_channel);
  }
}

#[no_mangle]
extern fn drain_channel(iterator: *mut ThreadedIterator) {
  unsafe {
    *iterator.drain();
  }
}


struct ThreadedIterator {
  receiver: Receiver<i32>,
  callback_context: *c_void,
  callback: extern fn(n: i32)
}

impl ThreadedIterator {
  
  fn create(callback: extern fn(n: i32, context: *c_void), callback_context: *c_void) {
    
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let it = box ThreadedIterator {
      callback: callback,
      callback_context: callback_context,
      receiver: rx
    };

    (it, tx)
  }

  fn drain(&self) {
    while match self.receiver.try_recv() {
      Ok(n) =>
        self.callback(n, self.callback_context);
        true,
      Err() =>
        false
    }
  }

}
