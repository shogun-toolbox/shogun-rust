extern crate proc_macro;
use syn::{parse_macro_input, DeriveInput, Ident};
use quote::quote;
use proc_macro::TokenStream;

#[proc_macro_derive(SGObject)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let lower_name = format!("name_{}", name);
    let create_name = format!("create_{}", name);
    
    let lower_name_ident = Ident::new(lower_name.to_lowercase().as_str(), name.span());
    let create_name_ident = Ident::new(create_name.to_lowercase().as_str(), name.span());
    
    let tokens = quote! {
        impl #name {
            pub fn new(#lower_name_ident: &'static str) -> Self {
                let c_string = CString::new(#lower_name_ident).expect("CString::new failed");
                #name {
                    ptr: unsafe { bindings::#create_name_ident(c_string.as_ptr()) }
                }
            }
            pub fn get(&self, parameter_name: &'static str) -> Result<Box<dyn std::any::Any>, String> {
                let c_string = CString::new(parameter_name).expect("CString::new failed");
                let c_visitor = unsafe { bindings::sgobject_get(self.ptr, c_string.as_ptr())};
                let c_visitor_type = unsafe {bindings::get_cvisitor_type(c_visitor)};
                match c_visitor_type {
                    bindings::TYPE_FLOAT32 => Ok(unsafe {Box::from_raw(bindings::get_cvisitor_pointer(c_visitor) as *mut f32)}),
                    bindings::TYPE_FLOAT64 => Ok(unsafe {Box::from_raw(bindings::get_cvisitor_pointer(c_visitor) as *mut f64)}),
                    bindings::TYPE_INT32 => Ok(unsafe {Box::from_raw(bindings::get_cvisitor_pointer(c_visitor) as *mut i32)}),
                    bindings::TYPE_INT64 => Ok(unsafe {Box::from_raw(bindings::get_cvisitor_pointer(c_visitor) as *mut i64)}),
                    // bindings::TYPE_SGOBJECT => {
                    //     Ok(unsafe {Box::from_raw(bindings::get_cvisitor_pointer(c_visitor) as *mut bindings::sgobject_t)})
                    // },
                    x => {
                        let c_typename = unsafe { CStr::from_ptr(bindings::get_cvisitor_typename(c_visitor)) };
                        Err(format!("Cannot handle type {}", c_typename.to_str().expect("Failed to get typename")))
                    },
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
                unsafe { bindings::destroy_sgobject(self.ptr) };
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