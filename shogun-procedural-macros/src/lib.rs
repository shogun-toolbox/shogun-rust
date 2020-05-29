extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(SGObject)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let mut lower_name_string = "name".to_string();
    let mut create_name_string = "create".to_string();

    for c in name.to_string().chars() {
        if c.is_uppercase() {
            lower_name_string.push('_');
            create_name_string.push('_');
            lower_name_string.push_str(&c.to_lowercase().to_string());
            create_name_string.push_str(&c.to_lowercase().to_string());
        }
        else {
            lower_name_string.push(c);
            create_name_string.push(c);
        }
    }

    let lower_name_ident = Ident::new(&lower_name_string, name.span());
    let create_name_ident = Ident::new(&create_name_string, name.span());

    let tokens = quote! {
        impl #name {
            pub fn new(#lower_name_ident: &'static str) -> Result<Self, String> {
                let c_string = CString::new(#lower_name_ident).expect("CString::new failed");
                let c_ptr = unsafe { shogun_sys::#create_name_ident(c_string.as_ptr()) };
                unsafe {
                match c_ptr {
                    shogun_sys::sgobject_result { return_code: shogun_sys::RETURN_CODE_SUCCESS,
                                      result: shogun_sys::sgobject_result_ResultUnion { result: ptr } } => {
                                        Ok(#name { ptr })
                                    },
                    shogun_sys::sgobject_result { return_code: shogun_sys::RETURN_CODE_ERROR,
                        result: shogun_sys::sgobject_result_ResultUnion { error: msg } } => {
                        let c_error_str = CStr::from_ptr(msg);
                        Err(format!("{}", c_error_str.to_str().expect("Failed to get error")))
                    },
                    _ => Err(format!("Unexpected return."))
                }
            }
            }
            pub fn get(&self, parameter_name: &'static str) -> Result<Box<dyn std::any::Any>, String> {
                let c_string = CString::new(parameter_name).expect("CString::new failed");
                let c_visitor = unsafe { shogun_sys::sgobject_get(self.ptr, c_string.as_ptr())};
                let c_visitor_type = unsafe {shogun_sys::get_cvisitor_type(c_visitor)};
                match c_visitor_type {
                    shogun_sys::TYPE_FLOAT32 => Ok(unsafe {Box::from_raw(shogun_sys::get_cvisitor_pointer(c_visitor) as *mut f32)}),
                    shogun_sys::TYPE_FLOAT64 => Ok(unsafe {Box::from_raw(shogun_sys::get_cvisitor_pointer(c_visitor) as *mut f64)}),
                    shogun_sys::TYPE_INT32 => Ok(unsafe {Box::from_raw(shogun_sys::get_cvisitor_pointer(c_visitor) as *mut i32)}),
                    shogun_sys::TYPE_INT64 => Ok(unsafe {Box::from_raw(shogun_sys::get_cvisitor_pointer(c_visitor) as *mut i64)}),
                    shogun_sys::TYPE_SGOBJECT => {
                        let obj = unsafe { shogun_sys::get_cvisitor_pointer(c_visitor) as *mut shogun_sys::sgobject_t };
                        let obj_type = unsafe {shogun_sys::sgobject_derived_type(obj)};
                        match obj_type {
                            shogun_sys::SG_TYPE_SG_KERNEL => Ok(Box::new(Kernel{ptr: obj})),
                            shogun_sys::SG_TYPE_SG_DISTANCE => Ok(Box::new(Distance{ptr: obj})),
                            shogun_sys::SG_TYPE_SG_MACHINE => Ok(Box::new(Machine{ptr: obj})),
                            shogun_sys::SG_TYPE_SG_FEATURES => Ok(Box::new(Features{ptr: obj})),
                            shogun_sys::SG_TYPE_SG_FILE => Ok(Box::new(File{ptr: obj})),
                            shogun_sys::SG_TYPE_SG_COMBINATION_RULE => Ok(Box::new(CombinationRule{ptr: obj})),
                            shogun_sys::SG_TYPE_SG_EVALUATION => Ok(Box::new(Evaluation{ptr: obj})),
                            _ => Err(format!("Cannot handle type")),
                        }
                    },
                    _ => {
                        let c_typename = unsafe { CStr::from_ptr(shogun_sys::get_cvisitor_typename(c_visitor)) };
                        Err(format!("Cannot handle type {}", c_typename.to_str().expect("Failed to get typename")))
                    },
                }
            }

            pub fn put<T>(&self, parameter_name: &'static str, parameter_value: &T) -> Result<(), String>
            where T: SGObjectPut {
                parameter_value.sgobject_put(self.ptr, parameter_name)
            }
        }

        impl SGObjectPut for #name {
            fn sgobject_put(&self, obj: *mut shogun_sys::sgobject, parameter_name: &'static str) -> Result<(), String> {
                unsafe {
                    let c_string = CString::new(parameter_name).expect("CString::new failed");
                    let type_erased_parameter = std::mem::transmute::<*mut shogun_sys::sgobject, *const std::ffi::c_void>(self.ptr);
                    details::handle_result(&shogun_sys::sgobject_put(obj, c_string.as_ptr(), type_erased_parameter, shogun_sys::TYPE_SGOBJECT))
                }
            }
        }

        impl SGObject for #name {
            fn to_string(&self) -> &str {
                return details::sgobject_to_string(self.ptr);
            }
        }

        impl Drop for #name {
            fn drop(&mut self) {
                unsafe { shogun_sys::destroy_sgobject(self.ptr) };
            }
        }

        impl fmt::Display for #name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", SGObject::to_string(self))
            }
        }
    };

    tokens.into()
}
