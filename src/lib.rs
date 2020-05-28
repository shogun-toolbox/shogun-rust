mod bindings;

pub mod shogun {

    mod details {
        use crate::bindings;
        use std::ffi::CStr;
        pub fn sgobject_to_string<T>(obj: *const T) -> &'static str {
            let c_repr =
                unsafe { bindings::to_string(obj as *const _ as *const bindings::sgobject_t) };
            let repr = unsafe { CStr::from_ptr(c_repr) };
            repr.to_str()
                .expect("Failed to get SGObject representation")
        }
    }

    use crate::bindings;
    use shogun_rust_procedural::SGObject;
    use std::ffi::{CStr, CString};
    use std::fmt;
    use std::str::Utf8Error;

    trait SGObject: fmt::Display {
        fn to_string(&self) -> &str;
    }

    pub trait SGObjectPut {
        fn sgobject_put(&self, obj: *mut bindings::sgobject, name: &'static str) -> Option<&'static str>;
    }

    macro_rules! add_sgobject_put_type {
        ($put_type:ty, $enum_value:expr) => {
            impl SGObjectPut for $put_type {
                fn sgobject_put(&self, obj: *mut bindings::sgobject, parameter_name: &'static str) -> Option<&'static str> {
                    unsafe {
                        let c_string = CString::new(parameter_name).expect("CString::new failed");
                        let type_erased_parameter = std::mem::transmute::<&$put_type, *const std::ffi::c_void>(&self);
                        match bindings::sgobject_put(obj, c_string.as_ptr(), type_erased_parameter, $enum_value) {
                            bindings::sgobject_put_result {
                                return_code: bindings::RETURN_CODE_ERROR,
                                error: msg,
                            } => {
                                let c_error_str = CStr::from_ptr(msg);
                                Some(c_error_str.to_str().expect("Failed to get error"))
                            },
                            _ => None,
                        }
                    }
                }
            }
        }
    }

    add_sgobject_put_type!(i32, bindings::TYPE_INT32);
    add_sgobject_put_type!(i64, bindings::TYPE_INT64);
    add_sgobject_put_type!(f32, bindings::TYPE_FLOAT32);
    add_sgobject_put_type!(f64, bindings::TYPE_FLOAT64);

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

    #[derive(SGObject)]
    pub struct Distance {
        ptr: *mut bindings::sgobject,
    }

    impl Version {
        pub fn new() -> Self {
            Version {
                version_ptr: unsafe { bindings::create_version() },
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
