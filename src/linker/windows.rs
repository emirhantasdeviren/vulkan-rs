use std::ffi::CString;
use std::marker::PhantomData;
use std::marker::PhantomPinned;
use std::ptr::NonNull;

#[repr(C)]
struct HINSTANCE__ {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

struct HInstance {
    handle: NonNull<HINSTANCE__>,
    _marker: PhantomData<HINSTANCE__>,
}

type FarProc = unsafe extern "system" fn() -> i64;

extern "system" {
    fn LoadLibraryA(lib_file_name: *const i8) -> *mut HINSTANCE__;
    fn GetProcAddress(module: *mut HINSTANCE__, proc_name: *const i8) -> Option<FarProc>;
    fn FreeLibrary(lib_module: *mut HINSTANCE__) -> i32;
}

pub struct DynamicLibrary {
    module: HInstance,
}

impl DynamicLibrary {
    pub fn new(file_name: &str) -> Self {
        let file_name_c = CString::new(file_name).unwrap();
        // SAFETY: `file_name_c` is valid CString. Thus we can call the function.
        let handle = unsafe { LoadLibraryA(file_name_c.as_ptr()) };

        let module = HInstance {
            handle: NonNull::new(handle).expect("Could not load library."),
            _marker: PhantomData,
        };

        Self { module }
    }

    pub fn get_proc_addr(&self, proc_name: &str) -> FarProc {
        let proc_name_c = CString::new(proc_name).unwrap();
        // SAFETY: Since self is alive we have valid HMODULE and we have valid CString.
        // We can call the function and it will return non-null fn pointer since fn pointers
        // can't be null.
        unsafe {
            GetProcAddress(self.module.handle.as_ptr(), proc_name_c.as_ptr())
                .expect("Could not retrieve address of an exported function or variable.")
        }
    }
}

impl Drop for DynamicLibrary {
    fn drop(&mut self) {
        // SAFETY: Since self is alive we can safely free the library.
        println!("Dropped Library");
        unsafe { FreeLibrary(self.module.handle.as_ptr()) };
    }
}