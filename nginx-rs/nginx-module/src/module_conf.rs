use syn::{ spanned::Spanned, DeriveInput };
use crate::commands::expand_commands;
use proc_macro2::{ Delimiter, TokenStream, TokenTree };

pub fn expand_module(attr: TokenStream, item: DeriveInput) -> Result<TokenStream, String> {
  let mod_name_str = attr.to_string();
  let ctx_fn_name = format_ident!("ngx_http_{}_module_ctx", mod_name_str);
  let mod_name = format_ident!("ngx_http_{}_module", mod_name_str);
  let commands_name = format_ident!("ngx_http_{}_module_commands", mod_name_str);

  // let item: DeriveInput = syn::parse(item).unwrap();
  let item_name = &item.ident;

  let expanded: TokenStream = quote! {
    #[no_mangle]
    pub static mut #mod_name: ngx_module_t = ngx_module_t {
        ctx_index: ngx_uint_t::max_value(),
        index: ngx_uint_t::max_value(),
        name: ptr::null_mut(),
        spare0: 0,
        spare1: 0,
        version: nginx_version as ngx_uint_t,
        signature: NGX_RS_MODULE_SIGNATURE.as_ptr() as *const c_char,

        ctx: &#ctx_fn_name as *const _ as *mut _,
        commands: unsafe { &#commands_name[0] as *const _ as *mut _ },
        type_: NGX_HTTP_MODULE as ngx_uint_t,

        init_master: None,
        init_module: None,
        init_process: None,
        init_thread: None,
        exit_thread: None,
        exit_process: None,
        exit_master: None,

        spare_hook0: 0,
        spare_hook1: 0,
        spare_hook2: 0,
        spare_hook3: 0,
        spare_hook4: 0,
        spare_hook5: 0,
        spare_hook6: 0,
        spare_hook7: 0,
    };

    #[doc(hidden)]
    #[no_mangle]
    static #ctx_fn_name: ngx_http_module_t = ngx_http_module_t {
        preconfiguration: Some(#item_name::preconfiguration),
        postconfiguration: Some(#item_name::postconfiguration_),

        create_main_conf: Some(#item_name::create_main_conf),
        init_main_conf: Some(#item_name::init_main_conf),

        create_srv_conf: Some(#item_name::create_srv_conf),
        merge_srv_conf: Some(#item_name::merge_srv_conf),

        create_loc_conf: Some(#item_name::create_loc_conf),
        merge_loc_conf: Some(#item_name::merge_loc_conf),
    };

    impl HTTPModuleContext for #item_name {
      fn get_http_main_conf_ (_cf: *mut ngx_conf_t) -> *mut c_void {
          unsafe { ngx_http_conf_get_module_main_conf(_cf, &#mod_name) }
      }

      fn get_http_srv_conf_ (_cf: *mut ngx_conf_t) -> *mut c_void {
          unsafe { ngx_http_conf_get_module_srv_conf(_cf, &#mod_name) }
      }

      fn get_http_loc_conf_ (_cf: *mut ngx_conf_t) -> *mut c_void {
          unsafe { ngx_http_conf_get_module_loc_conf(_cf, &#mod_name) }
      }
    }
  };

  Ok(expanded)
}

pub fn expand_module_derive (item: TokenStream) -> Result<TokenStream, String> {
  let item: DeriveInput = syn::parse2(item).unwrap();

  let mut commands = quote!(__void_ngx_module_commands);
  let mut module_name = quote!(__void_ngx_module_name);
  let mut has_module_name = false;
  let mut has_module_commands = false;

  for a in &item.attrs {
    if let Some(i) = a.path.get_ident() {
      if i == "ngx_module_name" {
          let mut token_it = a.tokens.clone().into_iter();
          if let (Some(TokenTree::Group(g)), None) =
              (token_it.next(), token_it.next())
          {
              if g.delimiter() == Delimiter::Parenthesis {
                  module_name = g.stream();
                  has_module_name = true;
                  continue;
              }
          }
          return Ok(
            quote_spanned! {
              a.span() => compile_error! { "ngx_module_name attribute must an ident" }
            }
          );
      } else if i == "ngx_module_commands" {
        let mut token_it = a.tokens.clone().into_iter();
          if let (Some(TokenTree::Group(g)), None) =
              (token_it.next(), token_it.next())
          {
              if g.delimiter() == Delimiter::Parenthesis {
                  commands = g.stream();
                  has_module_commands = true;
                  continue;
              }
          }
          return Ok(
            quote_spanned! {
              a.span() => compile_error! { "ngx_module_commands attribute must an ident list" }
            }
          );
      }
    }
  }

  if !has_module_name {
    return Ok(
      quote_spanned! {
        item.span() => compile_error! { "ngx_module_name attribute is required" }
      }
    );
  }

  if !has_module_commands {
    return Ok(
      quote_spanned! {
        item.span() => compile_error! { "ngx_module_commands attribute is required" }
      }
    );
  }

  let module_conf = expand_module(module_name.clone(), item).unwrap();
  let module_commands = expand_commands(module_name, commands).unwrap();

  let expanded = quote! {
    #module_conf
    #module_commands
  };

  Ok(expanded)
}
