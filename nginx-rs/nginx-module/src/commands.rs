use proc_macro2::TokenStream;

pub fn expand_commands(module_name: TokenStream, attr: TokenStream) -> Result<TokenStream, String> {
  let iter = attr.into_iter();
  let mut commands = vec![];
  iter
    .for_each(|f| {
      let value = TokenStream::from(f);
      let try_parse = syn::parse2::<syn::Ident>(value);

      if let Ok(item) = try_parse {
        commands.push(item)
      }
    });

  let conf_ident = format_ident!("ngx_http_{}_module_commands", module_name.to_string());

  let expanded = quote! {
    #[no_mangle]
    static mut #conf_ident: [ngx_command_t; 3] = [
      #(
        #commands::NGX_COMMAND,
      )*
      ngx_null_command!(),
    ];
  };

  Ok(expanded)
}
