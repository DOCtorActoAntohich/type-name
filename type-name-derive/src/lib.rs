use proc_macro::TokenStream;

/// Automagically implements `type_name::ShortTypeName`.
#[proc_macro_derive(ShortTypeName)]
pub fn derive_as_type_name_trait(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("Cannot parse struct definition");
    let type_identifier = ast.ident;
    let generated_impl = quote::quote! {
        impl ShortTypeName for #type_identifier {
            fn as_type_name() -> &'static str {
                stringify!(#type_identifier)
            }
        }
    };
    generated_impl.into()
}
