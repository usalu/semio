//! 📦️ Package glue — proc-macro crate root; implementation in owner `🦀️.rs`.
//!
//! `#[dyn_enum]` re-emits every `async fn` of a trait as `fn .. -> impl Future<Output = T>`, so no
//! consuming crate needs `#![allow(async_fn_in_trait)]`; `Send` stays structural (R3), never a `+ Send` bound.

#[path = "../../🦀️.rs"]
mod component;

use proc_macro::TokenStream;

/// 🗃️ Re-emits a trait declaration unchanged and captures its method signatures into a hidden,
/// `#[macro_export]`ed `__semio_dispatch_<TraitName>!` for `dyn_enum!` to close later.
#[proc_macro_attribute]
pub fn dyn_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    component::expand_dyn_enum_attribute(attr.into(), item.into()).unwrap_or_else(|error| error.to_compile_error()).into()
}

/// 📪️ `dyn_enum_close! { enum Members: Trait { Text(TextStore), Sketch(SketchStore) } }` — the enum,
/// its `From<VariantTy>` impls, and the delegating `impl Trait for Members`. Named `dyn_enum_close`,
/// NOT `dyn_enum` — Rust's macro namespace is flat across attribute/derive/function-like macros, so an
/// attribute macro and a function-like macro cannot share one name in the same crate (verified: `E0428
/// the name 'dyn_enum' is defined multiple times`, `📓️terra-dyn-enum-macro-report.md`).
#[proc_macro]
pub fn dyn_enum_close(input: TokenStream) -> TokenStream {
    component::expand_dyn_enum_call(input.into()).unwrap_or_else(|error| error.to_compile_error()).into()
}
