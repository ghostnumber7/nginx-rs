use syn::DeriveInput;
use proc_macro2::TokenStream;

pub fn expand_command(attr: TokenStream, item: TokenStream) -> Result<TokenStream, String> {
  let item: DeriveInput = syn::parse2(item).unwrap();

  let ident = &item.ident;
  let item_name = attr.to_string();
  let handler_ident = format_ident!("{}_handler_", item_name);

  let expanded = quote! {
    #item

    impl #ident {
      const NGX_COMMAND: ngx_command_t = ngx_command_t {
        name: ngx_string!(#item_name),
        type_: #ident::TYPE as ngx_uint_t,
        set: Some(#ident::#handler_ident),
        conf: #ident::CONF as ngx_uint_t,
        offset: #ident::OFFSET as ngx_uint_t,
        post: #ident::POST,
      };

      #[no_mangle]
      unsafe extern "C" fn #handler_ident(cf: *mut ngx_conf_t, cmd: *mut ngx_command_t, conf: *mut c_void) -> *mut c_char {
        #ident::handler(cf, cmd, conf)
      }
    }
  };

  Ok(expanded)
}
