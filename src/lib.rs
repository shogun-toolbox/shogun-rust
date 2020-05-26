mod bindings;

pub mod shogun {
    use std::ffi::{CString, CStr, c_void};
    use std::str::Utf8Error;
    use crate::bindings;
    use std::fmt;

    pub struct Version {
        version_ptr: *mut bindings::version_t,
    }
    
    pub struct Machine {
        machine_ptr: *mut bindings::machine_t,
    }

    impl Version {
        pub fn new() -> Version {
            Version{
                version_ptr: unsafe{ bindings::create_version() },
            }
        }
    
        pub fn main_version(&self) -> Result<&'static str, Utf8Error> {
            let char_ptr = unsafe { bindings::get_version_main(self.version_ptr) };
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            c_str.to_str()
        }
    }

    impl Machine {
        pub fn new(machine_name: &'static str) -> Machine {
            let c_string = CString::new(machine_name).expect("CString::new failed");
            Machine {
                machine_ptr: unsafe { bindings::create_machine(c_string.as_ptr()) }
            }
        }
    }
    
    impl Drop for Version {
        fn drop(&mut self) {
            unsafe { bindings::destroy_version(self.version_ptr) };
        }
    }

    impl Drop for Machine {
        fn drop(&mut self) {
            unsafe { bindings::destroy_machine(self.machine_ptr) };
        }
    }

    impl fmt::Display for Machine {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let c_repr = unsafe{ bindings::to_string(self.machine_ptr as *const _ as *const bindings::machine_t) };
            let repr = unsafe { CStr::from_ptr(c_repr) };
            write!(f, "{}", repr.to_str().expect("Failed to get SGObject representation"))
        }
    }
}
