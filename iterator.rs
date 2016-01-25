use std::sync::mpsc::{Sender, Receiver};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;
#[no_mangle]
pub extern fn create_iterator(min: i32, max: i32, callback: extern fn(n: i32, context: *c_void), context: *c_void) -> *mut ThreadedIterator {
  let callback = Callback<i32>::new(callback, context);
  let (it, tx) = ThreadedIterator<i32>::create(callback);
  let drain = AppleCTI::new(|| it.drain());
  load_data_into_iterator(tx, min, max, drain);
  let it_ptr = unsafe {transmute(it)};
  it_ptr
}

fn load_data_into_iterator<F>(tx: Sender<i32>, min: i32, max: i32, drain: &CrossThreadInvoker) {
  thread::spawn(move || {
    for n in min..max {
      tx.send(n).unwrap();
      drain.requestInvocation();
      thread::sleep(Duration::from_millis(500));
    }
  });
}


struct ThreadedIterator<T> {
  receiver: Receiver<T>
  callback: Fn<T>,
  is_canceled: AtomicBool
}

impl<T: Send> ThreadedIterator<T> {
  
  fn create(callback: Fn<T>, drainer) {
    
    let (tx, rx): (Sender<T>, Receiver<T>) = mpsc::channel();
    let uiConsumer = AppleUIThreadDrainer::new(rx, |value: T| {

    });

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
        self.callback(n);
        true,
      Err() =>
        false
    }
  }

  pub fn is_canceled(&self) -> bool {
    self.is_canceled.load(Ordering::Acquire)
  }



}
