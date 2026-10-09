//! 📦️ Package glue — proc-macro crate root; implementation in owner `🦀️.rs`.

#![feature(proc_macro_tracked_path, proc_macro_tracked_env)]

#[path = "../../🏃️process/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/🦀️.rs"]
mod compiler_resources;

fn observe_compiler_resources<R>(run: impl FnOnce() -> R) -> R {
    compiler_resources::with_compiler_resources_v1(env!("CARGO_MANIFEST_DIR"), concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs"), proc_macro::Span::call_site().local_file(), proc_macro::tracked::env_var("SEMIO_COMPILER_RESOURCE_ROOT").ok(), |path| proc_macro::tracked::path(path), run)
}

#[path = "⚙️expansion/🦀️.rs"]
mod component;

#[path = "🧬️retained-clone/🦀️.rs"]
mod retained_clone;

#[path = "🚪️io/📝️text/📸️snapshot/🦀️.rs"]
mod owned_json;

#[path = "🏭️factory/🦀️.rs"]
mod factory;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// 🏭️ Proves that each concrete factory field is Copy before declaring its payload heap-empty.
#[proc_macro_derive(FactoryPayloadRetirement, attributes(factory_child, factory_owned))]
pub fn derive_factory_payload_retirement(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    factory::expand(&input).unwrap_or_else(|error| error.to_compile_error()).into()
}

/// 📚️ Compiles a package-relative source JSON asset into an owned intrinsic value.
#[proc_macro]
pub fn owned_json_file(input: TokenStream) -> TokenStream {
    observe_compiler_resources(|| {
        let path = parse_macro_input!(input as syn::LitStr);
        owned_json::expand(&path).unwrap_or_else(|error| error.to_compile_error()).into()
    })
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

/// 🧵️ Derives immutable native canonical field roles through an explicit first-party owner.
#[proc_macro_derive(CanonicalJsonTree, attributes(value,canonical_json))]
pub fn derive_canonical_json_tree(input:TokenStream)->TokenStream {
 let input=parse_macro_input!(input as DeriveInput);
 component::expand_canonical_tree(&input).unwrap_or_else(|error|error.to_compile_error()).into()
}
