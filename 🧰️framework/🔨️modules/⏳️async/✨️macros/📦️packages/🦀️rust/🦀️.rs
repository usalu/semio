//! 📦️ Package glue — proc-macro crate root; implementation in owner `🦀️.rs`.

#[path = "../../🦀️.rs"]
mod component;

use proc_macro::TokenStream;

/// 🧪️ Keeps a test fn's literal `async` in source while expanding it to a sync `#[test]` harness
/// that drives the body through an inline, dependency-free `block_on`. See the owner module doc for
/// why this whole crate is deliberately plain sync Rust rather than following the repo's universal
/// `async fn` convention.
#[proc_macro_attribute]
pub fn async_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    component::expand_async_test(attr.into(), item.into()).unwrap_or_else(|e| e.to_compile_error()).into()
}
