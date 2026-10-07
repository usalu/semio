//! 📦️ Package glue — proc-macro crate root; implementation in owner `🦀️.rs`.

#[path = "⚙️expansion/🦀️.rs"]
mod component;

#[path = "🧬️retained-clone/🦀️.rs"]
mod retained_clone;

#[path = "🚪️io/📝️text/📸️snapshot/🦀️.rs"]
mod owned_json;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// 📚️ Compiles a package-relative source JSON asset into an owned intrinsic value.
#[proc_macro]
pub fn owned_json_file(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as syn::LitStr);
    owned_json::expand(&path).unwrap_or_else(|error| error.to_compile_error()).into()
}

/// 🗃️ Implements `value::ToValue` for a `#[value(...)]`-annotated struct or enum.
#[proc_macro_derive(ToValue, attributes(value))]
pub fn derive_to_value(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    component::expand_to_value(&derive_input).unwrap_or_else(|e| e.to_compile_error()).into()
}

/// 🗃️ Implements `value::FromValue` for a `#[value(...)]`-annotated struct or enum.
#[proc_macro_derive(FromValue, attributes(value))]
pub fn derive_from_value(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    component::expand_from_value(&derive_input).unwrap_or_else(|e| e.to_compile_error()).into()
}

/// 🧬️ Implements bounded native-owner cloning for a struct or enum.
#[proc_macro_derive(RetainedClone)]
pub fn derive_retained_clone(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    retained_clone::expand_retained_clone(&derive_input).unwrap_or_else(|error| error.to_compile_error()).into()
}

/// ♻️ Implements exact incremental retirement for every owned field of a struct or enum.
#[proc_macro_derive(RetireOwned)]
pub fn derive_retire_owned(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    retained_clone::expand_retire_owned(&derive_input).unwrap_or_else(|error| error.to_compile_error()).into()
}
