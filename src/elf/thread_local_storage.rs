use core::ffi::c_void;

#[repr(C)]
pub(crate) struct ThreadControlBlock {
    pub thread_pointee: [u8; 0],
    pub thread_pointer_register: *mut c_void,
    pub dynamic_thread_vector: *mut c_void,
    pub _padding: [usize; 3],
    pub canary: usize,
}

#[repr(C)]
pub(crate) union DynamicThreadVectorItem {
    pub pointer: *mut c_void,
    pub generation_counter: usize,
}
