extern crate shogun_sys;

pub mod shogun {

    mod details {
        use std::ffi::CStr;
        pub fn sgobject_to_string<T>(obj: *const T) -> &'static str {
            let c_repr =
                unsafe { shogun_sys::to_string(obj as *const _ as *const shogun_sys::sgobject_t) };
            let repr = unsafe { CStr::from_ptr(c_repr) };
            repr.to_str()
                .expect("Failed to get SGObject representation")
        }

        pub fn handle_result(result: &shogun_sys::Result) -> Result<(), String> {
            unsafe {
                match result {
                    shogun_sys::Result {
                        return_code: shogun_sys::RETURN_CODE_ERROR,
                        error: msg,
                    } => {
                        let c_error_str = CStr::from_ptr(*msg);
                        Err(format!("{}", c_error_str.to_str().expect("Failed to get error")))
                    }
                    shogun_sys::Result {
                        return_code: shogun_sys::RETURN_CODE_SUCCESS,
                        error: _,
                    } => Ok(()),
                    _ => Err("Unexpected return.".to_string())
                }
            }
        }
    }

    use shogun_rust_procedural::{SGObject, getter_reflection};
    use std::ffi::{CStr, CString};
    use std::fmt;
    extern crate ndarray;
    use ndarray::Array2;

    /// Struct owns a *mut shogun_sys::sgobject
    pub trait HasSGObjectPtr {
        fn get_ptr(&self) -> *mut shogun_sys::sgobject;
    }

    /// The trait that all SGObject derived types have to implement
    pub trait SGObject: fmt::Display + HasSGObjectPtr {
        /// The SGObject derived type
        type DerivedObject;
        /// Factory to generate new DerivedObject types from a string
        fn create(name: &'static str) -> Result<Self::DerivedObject, String>;
        /// Setter for any type that implements the SGObjectPut trait 
        fn put<T>(&self, parameter_name: &'static str, parameter_value: &T) -> Result<(), String>
        where T: SGObjectPut {
            parameter_value.sgobject_put(self.get_ptr(), parameter_name)
        }
        getter_reflection!{}
        /// String representation of the struct
        fn to_string(&self) -> &str;
    }

    pub trait SGObjectFromPtr {
        type DerivedObject;
        fn from_ptr(ptr: *mut shogun_sys::sgobject) -> Self::DerivedObject;
    }

    /// Trait for types that can be put in an SGObject 
    pub trait SGObjectPut {
        fn sgobject_put(&self, obj: *mut shogun_sys::sgobject, name: &'static str) -> Result<(), String>;
    }

    pub fn handle_sgobject_result<T>(result: &shogun_sys::sgobject_result) -> Result<T::DerivedObject, String>
    where T: SGObjectFromPtr {
        unsafe {
            match result {
                shogun_sys::sgobject_result { return_code: shogun_sys::RETURN_CODE_SUCCESS,
                                result: shogun_sys::sgobject_result_ResultUnion { result: ptr } } => {
                                    Ok(T::from_ptr(*ptr))
                                },
                shogun_sys::sgobject_result { return_code: shogun_sys::RETURN_CODE_ERROR,
                    result: shogun_sys::sgobject_result_ResultUnion { error: msg } } => {
                    let c_error_str = CStr::from_ptr(*msg);
                    Err(format!("{}", c_error_str.to_str().expect("Failed to get error")))
                },
                _ => Err(format!("Unexpected return."))
            }
        }
    }

    macro_rules! add_sgobject_put_type {
        ($put_type:ty, $enum_value:expr) => {
            impl SGObjectPut for $put_type {
                fn sgobject_put(&self, obj: *mut shogun_sys::sgobject, parameter_name: &'static str) -> Result<(), String> {
                    unsafe {
                        let c_string = CString::new(parameter_name).expect("CString::new failed");
                        let type_erased_parameter = std::mem::transmute::<&$put_type, *const std::ffi::c_void>(&self);
                        details::handle_result(&shogun_sys::sgobject_put(obj, c_string.as_ptr(), type_erased_parameter, $enum_value))
                    }
                }
            }
        }
    }

    add_sgobject_put_type!(i32, shogun_sys::TYPE_INT32);
    add_sgobject_put_type!(i64, shogun_sys::TYPE_INT64);
    add_sgobject_put_type!(f32, shogun_sys::TYPE_FLOAT32);
    add_sgobject_put_type!(f64, shogun_sys::TYPE_FLOAT64);

    pub struct Version {
        version_ptr: *mut shogun_sys::version_t,
    }

    #[derive(SGObject)]
    pub struct Machine {
        ptr: *mut shogun_sys::sgobject,
    }

    #[derive(SGObject)]
    pub struct Kernel {
        ptr: *mut shogun_sys::sgobject,
    }

    #[derive(SGObject)]
    pub struct Distance {
        ptr: *mut shogun_sys::sgobject,
    }

    #[derive(SGObject)]
    pub struct Features {
        ptr: *mut shogun_sys::sgobject,
    }

    #[derive(SGObject)]
    pub struct File {
        ptr: *mut shogun_sys::sgobject,
    }

    #[derive(SGObject)]
    pub struct CombinationRule {
        ptr: *mut shogun_sys::sgobject,
    }

    #[derive(SGObject)]
    pub struct Labels {
        ptr: *mut shogun_sys::sgobject,
    }

    #[derive(SGObject)]
    pub struct Evaluation {
        ptr: *mut shogun_sys::sgobject,
    }
    pub trait MatrixToFeatures {
        fn create_features_from_matrix(&self) -> Result<Features, String>;
    }

    macro_rules! add_matrix_type {
        ($array_type:ty, $enum_value:expr) => {
            impl MatrixToFeatures for Array2<$array_type> {
                fn create_features_from_matrix(&self) -> Result<Features, String> {
                    let n_rows = self.nrows();
                    let n_cols = self.ncols();
                    unsafe {
                        let data = self.as_ptr();
                        let type_erased_matrix = std::mem::transmute::<*const $array_type, *const std::ffi::c_void>(data);
                        match shogun_sys::create_features_from_data(type_erased_matrix, n_rows as u32, n_cols as u32, $enum_value) {
                            shogun_sys::sgobject_result { return_code: shogun_sys::RETURN_CODE_SUCCESS,
                                result: shogun_sys::sgobject_result_ResultUnion { result: ptr } } => {
                                  Ok(Features { ptr })
                              },
                            shogun_sys::sgobject_result { return_code: shogun_sys::RETURN_CODE_ERROR,
                                result: shogun_sys::sgobject_result_ResultUnion { error: msg } } => {
                                let c_error_str = CStr::from_ptr(msg);
                                Err(format!("{}", c_error_str.to_str().expect("Failed to get error")))
                            },
                            _ => Err("Unexpected return.".to_string())
                        }
                    }
                }
            }
            impl SGObjectPut for Array2<$array_type> {
                fn sgobject_put(&self, obj: *mut shogun_sys::sgobject, parameter_name: &'static str) -> Result<(), String> {
                    let n_rows = self.nrows() as u32;
                    let n_cols = self.ncols() as u32;
                    unsafe {
                        let data = self.as_ptr();
                        let c_string = CString::new(parameter_name).expect("CString::new failed");
                        let type_erased_matrix = std::mem::transmute::<*const $array_type, *const std::ffi::c_void>(data);
                        details::handle_result(&shogun_sys::sgobject_put_array(obj, c_string.as_ptr(), type_erased_matrix, n_rows, n_cols, $enum_value))
                    }
                }
            }       
        };
    }

    add_matrix_type!(f32, shogun_sys::TYPE_FLOAT32);
    add_matrix_type!(f64, shogun_sys::TYPE_FLOAT64);
    add_matrix_type!(i32, shogun_sys::TYPE_INT32);
    add_matrix_type!(i64, shogun_sys::TYPE_INT64);

    impl Features {
        pub fn from_array<T>(array: &Array2<T>) -> Result<Features, String>
        where Array2<T>: MatrixToFeatures {
            array.create_features_from_matrix()
        }

        pub fn from_file(file: &File) -> Result<Features, String> {
            unsafe {
                let c_ptr = shogun_sys::create_features_from_file(file.ptr);
                handle_sgobject_result::<Self>(&c_ptr)
            }
        }
    }

    impl Kernel {
        pub fn init(&mut self, lhs: &Features, rhs: &Features) -> Result<(), String> {
            unsafe {
                details::handle_result(&shogun_sys::init_kernel(self.ptr, lhs.ptr, rhs.ptr))
            }
        }
    }

    impl Machine {
        pub fn train(&mut self, features: &Features) -> Result<(), String> {
            unsafe {
                details::handle_result(&shogun_sys::train_machine(self.ptr, features.ptr))
            }
        }
        pub fn apply(&self, features: &Features) -> Result<Labels, String> {
            unsafe {
                let c_ptr = shogun_sys::apply_machine(self.ptr, features.ptr);
                handle_sgobject_result::<Labels>(&c_ptr)
            }
        }

        pub fn apply_multiclass(&self, features: &Features) -> Result<Labels, String> {
            unsafe {
                let c_ptr = shogun_sys::apply_multiclass_machine(self.ptr, features.ptr);
                handle_sgobject_result::<Labels>(&c_ptr)
            }
        }
    }

    impl File {
        pub fn read_csv(filepath: String) -> Result<Self, String> {
            unsafe {
                let c_string = CString::new(filepath).expect("CString::new failed");
                let c_ptr = shogun_sys::read_csvfile(c_string.as_ptr());
                handle_sgobject_result::<Self>(&c_ptr)
            }
        }
    }

    impl Labels {
        pub fn from_file(file: &File) -> Result<Labels, String> {
            unsafe {
                let c_ptr = shogun_sys::create_labels_from_file(file.ptr);
                handle_sgobject_result::<Labels>(&c_ptr)
            }
        }
    }

    impl Evaluation {
        pub fn evaluate(&self, y_pred: &Labels, y_true: &Labels) -> Result<f64, String> {
            unsafe {
                let c_ptr = shogun_sys::evaluate_labels(self.ptr, y_pred.ptr, y_true.ptr);
                match c_ptr {
                    shogun_sys::float64_result { return_code: shogun_sys::RETURN_CODE_SUCCESS,
                                    result: shogun_sys::float64_result_ResultFloat64Union { result: value } } => {
                                        Ok( value )
                                    },
                    shogun_sys::float64_result { return_code: shogun_sys::RETURN_CODE_ERROR,
                        result: shogun_sys::float64_result_ResultFloat64Union { error: msg } } => {
                        let c_error_str = CStr::from_ptr(msg);
                        Err(format!("{}", c_error_str.to_str().expect("Failed to get error")))
                    },
                    _ => Err(format!("Unexpected return."))
                }
            }
        }
    }

    impl Version {
        pub fn new() -> Self {
            Version {
                version_ptr: unsafe { shogun_sys::create_version() },
            }
        }

        pub fn main_version(&self) -> Result<String, String> {
            let char_ptr = unsafe { shogun_sys::get_version_main(self.version_ptr) };
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            match c_str.to_str() {
                Err(x) => Err(x.to_string()),
                Ok(x) => Ok(x.to_string()),
            }
        }
    }


    pub fn set_num_threads(n_threads: i32) {
        unsafe {shogun_sys::set_parallel_threads(n_threads)};
    }

    impl Drop for Version {
        fn drop(&mut self) {
            unsafe { shogun_sys::destroy_version(self.version_ptr) };
        }
    }
}
