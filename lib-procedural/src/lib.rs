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