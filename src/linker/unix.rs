use std::ffi::{c_void, CString};
use std::marker::PhantomData;
use std::ptr::NonNull;

const RTLD_LAZY: i32 = 0x00001;
const RTLD_LOCAL: i32 = 0;

pub struct DynamicLibrary {
    handle: NonNull<c_void>,
    _marker: PhantomData<c_void>,
}

#[link(name = "dl")]
extern "system" {
    fn dlopen(file: *const i8, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, name: *const i8) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> i32;
}

impl DynamicLibrary {
    pub fn new(file_name: &str) -> Self {
        let file_name_c = CString::new(file_name).unwrap();
        // SAFETY: `file_name_c` is valid CString. Thus we can call the function.
        let handle = unsafe { dlopen(file_name_c.as_ptr(), RTLD_LAZY | RTLD_LOCAL) };

        Self {
            handle: NonNull::new(handle).expect("Could not load library."),
            _marker: PhantomData,
        }
    }

    pub fn get_proc_addr(&self, name: &str) -> *mut c_void {
        let name_c = CString::new(name).unwrap();
        // SAFETY: Since self is alive we have a non-null handle to library and we have valid
        // CString. We can call the function and it will return non-null pointer.
        // If it is null we panic
        unsafe {
            let sym = dlsym(self.handle.as_ptr(), name_c.as_ptr());

            if sym.is_null() {
                panic!("Could not retrieve address associated with symbol");
            } else {
                sym
            }
        }
    }
}

impl Drop for DynamicLibrary {
    fn drop(&mut self) {
        // SAFETY: Since self is alive we have valid handle to library.
        let result = unsafe { dlclose(self.handle.as_ptr()) };
        if result == 0 {
            println!("Dropped dl");
        }
    }
}
