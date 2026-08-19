//! 📦️ Package glue — proc-macro crate root; implementation in owner `🦀️component.rs`.
//!
//! `#![allow(async_fn_in_trait)]`: every trait this crate declares in its own tests carries `async fn`
//! methods (universal async, O1), which trips rustc's "use of `async fn` in public traits is discouraged
//! .. auto trait bounds cannot be specified" lint on every one of them. Per ruling **R7**: silence it
//! with this crate-root allow, NEVER by adding `+ Send` to a signature (R3 — guest futures are
//! deliberately `?Send`; Send comes structurally from the concrete enum, never from a bound) and NEVER
//! by making the method sync. Every crate that DECLARES a `#[dyn_enum]` trait needs this same one-line
//! allow at ITS OWN crate root — `dyn_enum`/`dyn_enum!` cannot inject a crate-level inner attribute
//! across the boundary into a caller's crate, so this is a per-consuming-crate responsibility, called
//! out in `📓️terra-dyn-enum-macro-report.md`'s "applying dyn_enum: the recipe".
#![allow(async_fn_in_trait)]

#[path = "../../🦀️component.rs"]
mod component;

use proc_macro::TokenStream;

/// 🗃️ Re-emits a trait declaration unchanged and captures its method signatures into a hidden,
/// `#[macro_export]`ed `__semio_dispatch_<TraitName>!` for `dyn_enum!` to close later.
#[proc_macro_attribute]
pub fn dyn_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    component::expand_dyn_enum_attribute(attr.into(), item.into()).unwrap_or_else(|error| error.to_compile_error()).into()
}

/// 🗃️ `dyn_enum_close! { enum Members: Trait { Text(TextStore), Sketch(SketchStore) } }` — the enum,
/// its `From<VariantTy>` impls, and the delegating `impl Trait for Members`. Named `dyn_enum_close`,
/// NOT `dyn_enum` — Rust's macro namespace is flat across attribute/derive/function-like macros, so an
/// attribute macro and a function-like macro cannot share one name in the same crate (verified: `E0428
/// the name 'dyn_enum' is defined multiple times`, `📓️terra-dyn-enum-macro-report.md`).
#[proc_macro]
pub fn dyn_enum_close(input: TokenStream) -> TokenStream {
    component::expand_dyn_enum_call(input.into()).unwrap_or_else(|error| error.to_compile_error()).into()
}
