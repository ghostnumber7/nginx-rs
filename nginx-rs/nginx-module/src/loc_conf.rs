use syn::DeriveInput;
use syn::Data;
use syn::Fields;
use proc_macro2::TokenStream;

pub fn expand_loc_conf(input: &DeriveInput) -> Result<TokenStream, String>  {
    let result = match input.data {
        Data::Struct(ref s) => handle_loc_struct(input, &s.fields),
        Data::Union(_) => panic!("doesn't work with unions yet"),
        Data::Enum(ref _e) => panic!("doesn't work with enums yet")
    };

    // https://docs.rs/const-field-offset/latest/const_field_offset/derive.FieldOffsets.html
    let expanded = quote! {
        #[repr(C)]
        #[derive(nginx_rs::const_field_offset::FieldOffsets)]
        #[const_field_offset(nginx_rs::const_field_offset)]
        #result
    };

    Ok(expanded)
}

fn handle_loc_struct(ast: &DeriveInput, fields: &Fields) -> TokenStream {
    let item_name = &ast.ident;

    match *fields {
        Fields::Named(ref fields) => {
             let fnames = fields.named.iter().map(|f| {
                 let ident = &f.ident;
                 let target_ident = match ident {
                     Some(ident) => format_ident!("{}", ident.to_string()),
                     None => panic!("{}", "ident name not found")
                 };

                 let doc = format!(
                     "const field offset for [`{}`]({}) attribute",
                     ident.clone().unwrap().to_string(),
                     quote!(#ident)
                 );

                 quote! {
                     #[doc = #doc]
                    const #target_ident: ngx_uint_t = #item_name::FIELD_OFFSETS.#ident.get_byte_offset();
                }
             });

            let doc = format!(
                "Helper constants implementation containing the offsets of the fields of the struct [`{}`]\n\n\
                Generated from the `#[ngx_loc_conf]` macro from the [`nginx-rs`]({}) crate.\n\n\
                Uses macro `#[derive(FieldOffsets)]` from crate [`const-field-offset`]({}).",
                quote!(#item_name),
                quote!(nginx_module),
                quote!(const_field_offset)
            );

            let expanded = quote! {
                #ast

                #[no_mangle]
                #[doc = #doc]
                impl #item_name {
                    #(
                        #fnames
                    )*
                }
            };

            proc_macro::TokenStream::from(expanded).into()
        },
        Fields::Unit => {
            quote!(0)
        },
        Fields::Unnamed(ref _fields) => {
            quote!(0)
        },
    }
}
