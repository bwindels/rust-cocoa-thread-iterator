/* automatically generated by rust-bindgen */

pub type dispatch_function_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::libc::c_void)
                              -> ()>;
pub type dispatch_time_t = uint64_t;
pub enum Struct__os_object_s { }
pub enum Struct_dispatch_object_s { }
pub enum Struct_dispatch_continuation_s { }
pub enum Struct_dispatch_queue_s { }
pub enum Struct_dispatch_queue_attr_s { }
pub enum Struct_dispatch_group_s { }
pub enum Struct_dispatch_source_s { }
pub enum Struct_dispatch_mach_s { }
pub enum Struct_dispatch_mach_msg_s { }
pub enum Struct_dispatch_timer_aggregate_s { }
pub enum Struct_dispatch_source_attr_s { }
pub enum Struct_dispatch_semaphore_s { }
pub enum Struct_dispatch_data_s { }
pub enum Struct_dispatch_io_s { }
pub enum Struct_dispatch_operation_s { }
pub enum Struct_dispatch_disk_s { }
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed1 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed1 {
    pub unsafe fn _os_obj(&mut self) -> *mut *mut Struct__os_object_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _do(&mut self) -> *mut *mut Struct_dispatch_object_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _dc(&mut self) -> *mut *mut Struct_dispatch_continuation_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _dq(&mut self) -> *mut *mut Struct_dispatch_queue_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _dqa(&mut self) -> *mut *mut Struct_dispatch_queue_attr_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _dg(&mut self) -> *mut *mut Struct_dispatch_group_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _ds(&mut self) -> *mut *mut Struct_dispatch_source_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _dm(&mut self) -> *mut *mut Struct_dispatch_mach_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _dmsg(&mut self) -> *mut *mut Struct_dispatch_mach_msg_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _dta(&mut self)
     -> *mut *mut Struct_dispatch_timer_aggregate_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _dsa(&mut self) -> *mut *mut Struct_dispatch_source_attr_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _dsema(&mut self) -> *mut *mut Struct_dispatch_semaphore_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _ddata(&mut self) -> *mut *mut Struct_dispatch_data_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _dchannel(&mut self) -> *mut *mut Struct_dispatch_io_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _doperation(&mut self)
     -> *mut *mut Struct_dispatch_operation_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _ddisk(&mut self) -> *mut *mut Struct_dispatch_disk_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed1 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type dispatch_object_t = Union_Unnamed1;
pub type dispatch_block_t = ::libc::c_void;
pub type dispatch_queue_t = *mut Struct_dispatch_queue_s;
pub type dispatch_queue_priority_t = ::libc::c_long;
pub type dispatch_qos_class_t = qos_class_t;
pub type dispatch_queue_attr_t = *mut Struct_dispatch_queue_attr_s;
pub type Enum_Unnamed2 = ::libc::c_uint;
pub const DISPATCH_BLOCK_BARRIER: ::libc::c_uint = 1;
pub const DISPATCH_BLOCK_DETACHED: ::libc::c_uint = 2;
pub const DISPATCH_BLOCK_ASSIGN_CURRENT: ::libc::c_uint = 4;
pub const DISPATCH_BLOCK_NO_QOS_CLASS: ::libc::c_uint = 8;
pub const DISPATCH_BLOCK_INHERIT_QOS_CLASS: ::libc::c_uint = 16;
pub const DISPATCH_BLOCK_ENFORCE_QOS_CLASS: ::libc::c_uint = 32;
pub type dispatch_block_flags_t = ::libc::c_ulong;
pub type dispatch_source_t = *mut Struct_dispatch_source_s;
pub enum Struct_dispatch_source_type_s { }
pub type dispatch_source_type_t = *const Struct_dispatch_source_type_s;
pub type dispatch_source_mach_send_flags_t = ::libc::c_ulong;
pub type dispatch_source_memorypressure_flags_t = ::libc::c_ulong;
pub type dispatch_source_proc_flags_t = ::libc::c_ulong;
pub type dispatch_source_vnode_flags_t = ::libc::c_ulong;
pub type dispatch_source_timer_flags_t = ::libc::c_ulong;
pub type dispatch_group_t = *mut Struct_dispatch_group_s;
pub type dispatch_semaphore_t = *mut Struct_dispatch_semaphore_s;
pub type dispatch_once_t = ::libc::c_long;
pub type dispatch_data_t = *mut Struct_dispatch_data_s;
pub type dispatch_data_applier_t = ::libc::c_void;
pub type dispatch_fd_t = ::libc::c_int;
pub type dispatch_io_t = *mut Struct_dispatch_io_s;
pub type dispatch_io_type_t = ::libc::c_ulong;
pub type dispatch_io_handler_t = ::libc::c_void;
pub type dispatch_io_close_flags_t = ::libc::c_ulong;
pub type dispatch_io_interval_flags_t = ::libc::c_ulong;
#[link(name = "dispatch")]
extern "C" {
    pub static mut _dispatch_main_q: Struct_dispatch_queue_s;
    pub static mut _dispatch_queue_attr_concurrent:
               Struct_dispatch_queue_attr_s;
    pub static _dispatch_source_type_data_add: Struct_dispatch_source_type_s;
    pub static _dispatch_source_type_data_or: Struct_dispatch_source_type_s;
    pub static _dispatch_source_type_mach_send: Struct_dispatch_source_type_s;
    pub static _dispatch_source_type_mach_recv: Struct_dispatch_source_type_s;
    pub static _dispatch_source_type_memorypressure:
               Struct_dispatch_source_type_s;
    pub static _dispatch_source_type_proc: Struct_dispatch_source_type_s;
    pub static _dispatch_source_type_read: Struct_dispatch_source_type_s;
    pub static _dispatch_source_type_signal: Struct_dispatch_source_type_s;
    pub static _dispatch_source_type_timer: Struct_dispatch_source_type_s;
    pub static _dispatch_source_type_vnode: Struct_dispatch_source_type_s;
    pub static _dispatch_source_type_write: Struct_dispatch_source_type_s;
    pub static mut _dispatch_data_empty: Struct_dispatch_data_s;
    pub static _dispatch_data_destructor_free: dispatch_block_t;
    pub static _dispatch_data_destructor_munmap: dispatch_block_t;
}
#[link(name = "dispatch")]
extern "C" {
    pub fn os_retain(object: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn os_release(object: *mut ::libc::c_void) -> ();
    pub fn dispatch_time(when: dispatch_time_t, delta: int64_t)
     -> dispatch_time_t;
    pub fn dispatch_walltime(when: *const Struct_timespec, delta: int64_t)
     -> dispatch_time_t;
    pub fn dispatch_retain(object: dispatch_object_t) -> ();
    pub fn dispatch_release(object: dispatch_object_t) -> ();
    pub fn dispatch_get_context(object: dispatch_object_t)
     -> *mut ::libc::c_void;
    pub fn dispatch_set_context(object: dispatch_object_t,
                                context: *mut ::libc::c_void) -> ();
    pub fn dispatch_set_finalizer_f(object: dispatch_object_t,
                                    finalizer: dispatch_function_t) -> ();
    pub fn dispatch_suspend(object: dispatch_object_t) -> ();
    pub fn dispatch_resume(object: dispatch_object_t) -> ();
    pub fn dispatch_wait(object: *mut ::libc::c_void,
                         timeout: dispatch_time_t) -> ::libc::c_long;
    pub fn dispatch_notify(object: *mut ::libc::c_void,
                           queue: dispatch_object_t,
                           notification_block: dispatch_block_t) -> ();
    pub fn dispatch_cancel(object: *mut ::libc::c_void) -> ();
    pub fn dispatch_testcancel(object: *mut ::libc::c_void) -> ::libc::c_long;
    pub fn dispatch_debug(object: dispatch_object_t,
                          message: *const ::libc::c_char, ...) -> ();
    pub fn dispatch_debugv(object: dispatch_object_t,
                           message: *const ::libc::c_char, ap: va_list) -> ();
    pub fn dispatch_async(queue: dispatch_queue_t, block: dispatch_block_t)
     -> ();
    pub fn dispatch_async_f(queue: dispatch_queue_t,
                            context: *mut ::libc::c_void,
                            work: dispatch_function_t) -> ();
    pub fn dispatch_sync(queue: dispatch_queue_t, block: dispatch_block_t)
     -> ();
    pub fn dispatch_sync_f(queue: dispatch_queue_t,
                           context: *mut ::libc::c_void,
                           work: dispatch_function_t) -> ();
    pub fn dispatch_apply(iterations: size_t, queue: dispatch_queue_t,
                          block: ::libc::c_void) -> ();
    pub fn dispatch_apply_f(iterations: size_t, queue: dispatch_queue_t,
                            context: *mut ::libc::c_void,
                            work:
                                ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                               *mut ::libc::c_void,
                                                                           arg2:
                                                                               size_t)
                                                          -> ()>) -> ();
    pub fn dispatch_get_current_queue() -> dispatch_queue_t;
    pub fn dispatch_get_global_queue(identifier: ::libc::c_long,
                                     flags: ::libc::c_ulong)
     -> dispatch_queue_t;
    pub fn dispatch_queue_attr_make_with_qos_class(attr:
                                                       dispatch_queue_attr_t,
                                                   qos_class:
                                                       dispatch_qos_class_t,
                                                   relative_priority:
                                                       ::libc::c_int)
     -> dispatch_queue_attr_t;
    pub fn dispatch_queue_create(label: *const ::libc::c_char,
                                 attr: dispatch_queue_attr_t)
     -> dispatch_queue_t;
    pub fn dispatch_queue_get_label(queue: dispatch_queue_t)
     -> *const ::libc::c_char;
    pub fn dispatch_queue_get_qos_class(queue: dispatch_queue_t,
                                        relative_priority_ptr:
                                            *mut ::libc::c_int)
     -> dispatch_qos_class_t;
    pub fn dispatch_set_target_queue(object: dispatch_object_t,
                                     queue: dispatch_queue_t) -> ();
    pub fn dispatch_main() -> ();
    pub fn dispatch_after(when: dispatch_time_t, queue: dispatch_queue_t,
                          block: dispatch_block_t) -> ();
    pub fn dispatch_after_f(when: dispatch_time_t, queue: dispatch_queue_t,
                            context: *mut ::libc::c_void,
                            work: dispatch_function_t) -> ();
    pub fn dispatch_barrier_async(queue: dispatch_queue_t,
                                  block: dispatch_block_t) -> ();
    pub fn dispatch_barrier_async_f(queue: dispatch_queue_t,
                                    context: *mut ::libc::c_void,
                                    work: dispatch_function_t) -> ();
    pub fn dispatch_barrier_sync(queue: dispatch_queue_t,
                                 block: dispatch_block_t) -> ();
    pub fn dispatch_barrier_sync_f(queue: dispatch_queue_t,
                                   context: *mut ::libc::c_void,
                                   work: dispatch_function_t) -> ();
    pub fn dispatch_queue_set_specific(queue: dispatch_queue_t,
                                       key: *const ::libc::c_void,
                                       context: *mut ::libc::c_void,
                                       destructor: dispatch_function_t) -> ();
    pub fn dispatch_queue_get_specific(queue: dispatch_queue_t,
                                       key: *const ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn dispatch_get_specific(key: *const ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn dispatch_block_create(flags: dispatch_block_flags_t,
                                 block: dispatch_block_t) -> dispatch_block_t;
    pub fn dispatch_block_create_with_qos_class(flags: dispatch_block_flags_t,
                                                qos_class:
                                                    dispatch_qos_class_t,
                                                relative_priority:
                                                    ::libc::c_int,
                                                block: dispatch_block_t)
     -> dispatch_block_t;
    pub fn dispatch_block_perform(flags: dispatch_block_flags_t,
                                  block: dispatch_block_t) -> ();
    pub fn dispatch_block_wait(block: dispatch_block_t,
                               timeout: dispatch_time_t) -> ::libc::c_long;
    pub fn dispatch_block_notify(block: dispatch_block_t,
                                 queue: dispatch_queue_t,
                                 notification_block: dispatch_block_t) -> ();
    pub fn dispatch_block_cancel(block: dispatch_block_t) -> ();
    pub fn dispatch_block_testcancel(block: dispatch_block_t)
     -> ::libc::c_long;
    pub fn dispatch_source_create(_type: dispatch_source_type_t,
                                  handle: uintptr_t, mask: ::libc::c_ulong,
                                  queue: dispatch_queue_t)
     -> dispatch_source_t;
    pub fn dispatch_source_set_event_handler(source: dispatch_source_t,
                                             handler: dispatch_block_t) -> ();
    pub fn dispatch_source_set_event_handler_f(source: dispatch_source_t,
                                               handler: dispatch_function_t)
     -> ();
    pub fn dispatch_source_set_cancel_handler(source: dispatch_source_t,
                                              handler: dispatch_block_t)
     -> ();
    pub fn dispatch_source_set_cancel_handler_f(source: dispatch_source_t,
                                                handler: dispatch_function_t)
     -> ();
    pub fn dispatch_source_cancel(source: dispatch_source_t) -> ();
    pub fn dispatch_source_testcancel(source: dispatch_source_t)
     -> ::libc::c_long;
    pub fn dispatch_source_get_handle(source: dispatch_source_t) -> uintptr_t;
    pub fn dispatch_source_get_mask(source: dispatch_source_t)
     -> ::libc::c_ulong;
    pub fn dispatch_source_get_data(source: dispatch_source_t)
     -> ::libc::c_ulong;
    pub fn dispatch_source_merge_data(source: dispatch_source_t,
                                      value: ::libc::c_ulong) -> ();
    pub fn dispatch_source_set_timer(source: dispatch_source_t,
                                     start: dispatch_time_t,
                                     interval: uint64_t, leeway: uint64_t)
     -> ();
    pub fn dispatch_source_set_registration_handler(source: dispatch_source_t,
                                                    handler: dispatch_block_t)
     -> ();
    pub fn dispatch_source_set_registration_handler_f(source:
                                                          dispatch_source_t,
                                                      handler:
                                                          dispatch_function_t)
     -> ();
    pub fn dispatch_group_create() -> dispatch_group_t;
    pub fn dispatch_group_async(group: dispatch_group_t,
                                queue: dispatch_queue_t,
                                block: dispatch_block_t) -> ();
    pub fn dispatch_group_async_f(group: dispatch_group_t,
                                  queue: dispatch_queue_t,
                                  context: *mut ::libc::c_void,
                                  work: dispatch_function_t) -> ();
    pub fn dispatch_group_wait(group: dispatch_group_t,
                               timeout: dispatch_time_t) -> ::libc::c_long;
    pub fn dispatch_group_notify(group: dispatch_group_t,
                                 queue: dispatch_queue_t,
                                 block: dispatch_block_t) -> ();
    pub fn dispatch_group_notify_f(group: dispatch_group_t,
                                   queue: dispatch_queue_t,
                                   context: *mut ::libc::c_void,
                                   work: dispatch_function_t) -> ();
    pub fn dispatch_group_enter(group: dispatch_group_t) -> ();
    pub fn dispatch_group_leave(group: dispatch_group_t) -> ();
    pub fn dispatch_semaphore_create(value: ::libc::c_long)
     -> dispatch_semaphore_t;
    pub fn dispatch_semaphore_wait(dsema: dispatch_semaphore_t,
                                   timeout: dispatch_time_t)
     -> ::libc::c_long;
    pub fn dispatch_semaphore_signal(dsema: dispatch_semaphore_t)
     -> ::libc::c_long;
    pub fn dispatch_once(predicate: *mut dispatch_once_t,
                         block: dispatch_block_t) -> ();
    pub fn dispatch_once_f(predicate: *mut dispatch_once_t,
                           context: *mut ::libc::c_void,
                           function: dispatch_function_t) -> ();
    pub fn dispatch_data_create(buffer: *const ::libc::c_void, size: size_t,
                                queue: dispatch_queue_t,
                                destructor: dispatch_block_t)
     -> dispatch_data_t;
    pub fn dispatch_data_get_size(data: dispatch_data_t) -> size_t;
    pub fn dispatch_data_create_map(data: dispatch_data_t,
                                    buffer_ptr: *mut *const ::libc::c_void,
                                    size_ptr: *mut size_t) -> dispatch_data_t;
    pub fn dispatch_data_create_concat(data1: dispatch_data_t,
                                       data2: dispatch_data_t)
     -> dispatch_data_t;
    pub fn dispatch_data_create_subrange(data: dispatch_data_t,
                                         offset: size_t, length: size_t)
     -> dispatch_data_t;
    pub fn dispatch_data_apply(data: dispatch_data_t,
                               applier: dispatch_data_applier_t) -> u8;
    pub fn dispatch_data_copy_region(data: dispatch_data_t, location: size_t,
                                     offset_ptr: *mut size_t)
     -> dispatch_data_t;
    pub fn dispatch_read(fd: dispatch_fd_t, length: size_t,
                         queue: dispatch_queue_t, handler: ::libc::c_void)
     -> ();
    pub fn dispatch_write(fd: dispatch_fd_t, data: dispatch_data_t,
                          queue: dispatch_queue_t, handler: ::libc::c_void)
     -> ();
    pub fn dispatch_io_create(_type: dispatch_io_type_t, fd: dispatch_fd_t,
                              queue: dispatch_queue_t,
                              cleanup_handler: ::libc::c_void)
     -> dispatch_io_t;
    pub fn dispatch_io_create_with_path(_type: dispatch_io_type_t,
                                        path: *const ::libc::c_char,
                                        oflag: ::libc::c_int, mode: mode_t,
                                        queue: dispatch_queue_t,
                                        cleanup_handler: ::libc::c_void)
     -> dispatch_io_t;
    pub fn dispatch_io_create_with_io(_type: dispatch_io_type_t,
                                      io: dispatch_io_t,
                                      queue: dispatch_queue_t,
                                      cleanup_handler: ::libc::c_void)
     -> dispatch_io_t;
    pub fn dispatch_io_read(channel: dispatch_io_t, offset: off_t,
                            length: size_t, queue: dispatch_queue_t,
                            io_handler: dispatch_io_handler_t) -> ();
    pub fn dispatch_io_write(channel: dispatch_io_t, offset: off_t,
                             data: dispatch_data_t, queue: dispatch_queue_t,
                             io_handler: dispatch_io_handler_t) -> ();
    pub fn dispatch_io_close(channel: dispatch_io_t,
                             flags: dispatch_io_close_flags_t) -> ();
    pub fn dispatch_io_barrier(channel: dispatch_io_t,
                               barrier: dispatch_block_t) -> ();
    pub fn dispatch_io_get_descriptor(channel: dispatch_io_t)
     -> dispatch_fd_t;
    pub fn dispatch_io_set_high_water(channel: dispatch_io_t,
                                      high_water: size_t) -> ();
    pub fn dispatch_io_set_low_water(channel: dispatch_io_t,
                                     low_water: size_t) -> ();
    pub fn dispatch_io_set_interval(channel: dispatch_io_t,
                                    interval: uint64_t,
                                    flags: dispatch_io_interval_flags_t)
     -> ();
}
