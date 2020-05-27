mod bindings;

pub mod shogun {

    mod details {
        use crate::bindings;
        use std::ffi::CStr;
        pub fn sgobject_to_string<T>(obj: *const T) -> &'static str {
            let c_repr = unsafe{ bindings::to_string(obj as *const _ as *const bindings::sgobject_t) };
            let repr = unsafe { CStr::from_ptr(c_repr) };
            repr.to_str().expect("Failed to get SGObject representation")
        }
    }

    use std::ffi::{CString, CStr};
    use std::str::Utf8Error;
    use crate::bindings;
    use shogun_rust_procedural::SGObject;
    use std::fmt;

    trait SGObject: fmt::Display {
        fn to_string(&self) -> &str;
    }
    
    pub struct Version {
        version_ptr: *mut bindings::version_t,
    }

    #[derive(SGObject)]
    pub struct Machine {
        ptr: *mut bindings::sgobject,
    }

    #[derive(SGObject)]
    pub struct Kernel {
        ptr: *mut bindings::sgobject,
    }

    impl Version {
        pub fn new() -> Self {
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
    
    impl Drop for Version {
        fn drop(&mut self) {
            unsafe { bindings::destroy_version(self.version_ptr) };
        }
    }
}
