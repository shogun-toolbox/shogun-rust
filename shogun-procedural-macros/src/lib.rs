extern crate proc_macro;
extern crate shogun_sys;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Item};
use regex::Regex;

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
                #name::create(#lower_name_ident)
            }
        }

        impl SGObject for #name {
            type DerivedObject = #name;
            fn create (name: &'static str) -> Result<Self::DerivedObject, String> {
                let c_string = CString::new(name).expect("CString::new failed");
                let c_ptr = unsafe { shogun_sys::#create_name_ident(c_string.as_ptr()) };
                unsafe {
                    match c_ptr {
                        shogun_sys::sgobject_result { return_code: shogun_sys::RETURN_CODE_SUCCESS,
                                        result: shogun_sys::sgobject_result_ResultUnion { result: ptr } } => Ok(#name { ptr }),
                        shogun_sys::sgobject_result { return_code: shogun_sys::RETURN_CODE_ERROR,
                            result: shogun_sys::sgobject_result_ResultUnion { error: msg } } => {
                            let c_error_str = CStr::from_ptr(msg);
                            Err(format!("{}", c_error_str.to_str().expect("Failed to get error")))
                        },
                        _ => Err(format!("Unexpected return."))
                    }
                }
            }

            fn to_string(&self) -> &str {
                return details::sgobject_to_string(self.ptr);
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

        impl SGObjectFromPtr for #name {
            type DerivedObject = #name;
            fn from_ptr(ptr: *mut shogun_sys::sgobject) -> Self::DerivedObject {
                #name { ptr }
            }
        }

        impl HasSGObjectPtr for #name {
            fn get_ptr(&self) -> *mut shogun_sys::sgobject {
                self.ptr
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

fn from_sg_enum_to_rust_type(sg_enum: &str) -> String {
    let mut result = String::new();
    result.push_str(&sg_enum[0..1]);
    let mut chars = (&sg_enum[1..]).chars().fuse();
    // from LABELS to Labels
    // or from COMBINATION_RULE to CombinationRule
    while let Some(c) = chars.next() {
        match c {
            '_' => {
                match chars.next() {
                    Some(c) => result.push(c),
                    _ => break,
                }
            }
            _ => result.push(c.to_lowercase().collect::<Vec<_>>()[0]),
        }
    }
    result
}

fn from_primitive_type_to_rust_type(type_enum: &str) -> Option<String> {
    // unfortunately the type names are quite different in rust
    // so need to do some manual mapping
    match type_enum {
        "INT32" => Some("i32".to_string()),
        "INT64" => Some("i64".to_string()),
        "FLOAT32" => Some("f32".to_string()),
        "FLOAT64" => Some("f64".to_string()),
        "SGOBJECT" => None,
        _ => panic!(format!("Unknown type enum {}", type_enum)),
    }
}

#[proc_macro]
pub fn getter_reflection(_input: TokenStream) -> TokenStream {
    
    let string = include_str!("bindings.rs");
    let syntax = syn::parse_file(string).expect("Unable to parse file");

    let mut sg_primitive_types_ident = Vec::new();
    let mut sg_types_ident = Vec::new();
    let mut sg_primitive_types_enum_ident = Vec::new();
    let mut sg_types_enum_ident = Vec::new();

    let sg_type_matcher = Regex::new(r"^SG_TYPE_SG_(.*)$").unwrap();
    let primitive_type_matcher = Regex::new(r"^TYPE_(.*)$").unwrap();

    for item in syntax.items {
        match item {
            Item::Const(item) => {
                if item.ident.to_string().starts_with("SG_TYPE") {
                    let enum_value = &item.ident.to_string();
                    let raw_type_name = sg_type_matcher.captures(enum_value).unwrap().get(1).unwrap().as_str();
                    let type_name = from_sg_enum_to_rust_type(&raw_type_name);
                    sg_types_enum_ident.push(Ident::new(enum_value, item.ident.span()));
                    sg_types_ident.push(Ident::new(&type_name, item.ident.span()));
                }
                else if item.ident.to_string().starts_with("TYPE_") {
                    let enum_value = &item.ident.to_string();
                    let raw_type_name = primitive_type_matcher.captures(enum_value).unwrap().get(1).unwrap().as_str();
                    let type_name = from_primitive_type_to_rust_type(&raw_type_name);
                    match type_name {
                        Some(x) => {
                            sg_primitive_types_enum_ident.push(Ident::new(enum_value, item.ident.span()));
                            sg_primitive_types_ident.push(Ident::new(&x, item.ident.span()));
                        },
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    let result = quote! {
        /// Parameter getter  
        fn get(&self, parameter_name: &'static str) -> Result<Box<dyn std::any::Any>, String> {
            unsafe {
                let c_string = CString::new(parameter_name).expect("CString::new failed");
                let c_visitor = shogun_sys::sgobject_get(self.get_ptr(), c_string.as_ptr());
                let c_visitor_type = shogun_sys::get_cvisitor_type(c_visitor);
                match c_visitor_type {
                    #(shogun_sys::#sg_primitive_types_enum_ident => Ok(Box::from_raw(shogun_sys::get_cvisitor_pointer(c_visitor) as *mut #sg_primitive_types_ident)),)*
                    shogun_sys::TYPE_SGOBJECT => {
                        let obj = shogun_sys::get_cvisitor_pointer(c_visitor) as *mut shogun_sys::sgobject_t ;
                        let obj_type = shogun_sys::sgobject_derived_type(obj);
                        match obj_type {
                            #(shogun_sys::#sg_types_enum_ident => Ok(Box::new(#sg_types_ident {ptr: obj })),)*
                            _ => Err(format!("Cannot handle type")),
                        }
                    },
                    _ => {
                        let c_typename = CStr::from_ptr(shogun_sys::get_cvisitor_typename(c_visitor));
                        Err(format!("Cannot handle type {}", c_typename.to_str().expect("Failed to get typename")))
                    },
                }
            }
        }
    };
    result.into()
}