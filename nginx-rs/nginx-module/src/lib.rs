extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

mod loc_conf;
mod module_conf;
mod command;
mod commands;

use proc_macro::TokenStream;
use syn::DeriveInput;
use loc_conf::expand_loc_conf;
use module_conf::expand_module_derive;
use command::expand_command;

#[proc_macro_attribute]
pub fn ngx_loc_conf(_: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input: DeriveInput = syn::parse(input).unwrap();

    match expand_loc_conf(&input) {
        Ok(expanded) => expanded.into(),
        Err(msg) => panic!("{}", msg),
    }
}

/// Initializes an Nginx module definition from a `struct`
///
/// # Panics
/// The module initialization will fail if either `ngx_module_name` or `ngx_module_commands`
/// are missing.
///
/// # Examples
/// ```
/// #[derive(NgxModule)]
/// #[ngx_module_name(hello_world)]
/// #[ngx_module_commands(
///     HelloWorldCommand1
///     HelloWorldCommand2
/// )]
/// struct HelloWorldModule;
/// ```
#[proc_macro_derive(NgxModule, attributes(ngx_module_name, ngx_module_commands))]
pub fn ngx_module_derive(item: TokenStream) -> TokenStream {
    match expand_module_derive(item.into()) {
        Ok(expanded) => expanded.into(),
        Err(msg) => panic!("{}", msg),
    }
}

#[proc_macro_attribute]
pub fn ngx_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    match expand_command(attr.into(), item.into()) {
        Ok(expanded) => expanded.into(),
        Err(msg) => panic!("{}", msg),
    }
}
