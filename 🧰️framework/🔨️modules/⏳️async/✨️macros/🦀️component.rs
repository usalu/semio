//! 🧪️ `semio_framework_async_macros` — `#[async_test]`: keeps a test fn's literal `async fn` in
//! source (the repo's universal-async convention) while expanding it to a plain `#[test] fn` that
//! drives the body through an inline, dependency-free thread-park executor.
//!
//! 🚫️async: E3 proc-macro entry point — this whole crate is exempt from the repo's universal-async
//! convention, not just its `#[proc_macro_attribute]` entry: a `#[proc_macro_attribute]` fn's ABI is
//! `fn(TokenStream, TokenStream) -> TokenStream` (verified: tagging one `async fn` is rejected by
//! rustc with "attribute proc macro has incorrect signature", not merely a lint), and every helper it
//! calls runs at COMPILE TIME with no executor anywhere in the picture to hand an `async fn` to —
//! there is nothing here for `async` to mean.
//!
//! ✂️ `#[async_test]` deliberately emits its `block_on` executor NESTED inside each generated test
//! fn's own body (not as a shared crate-level item): a proc-macro's `quote!`-emitted identifiers use
//! call-site hygiene, so two `#[async_test]`-expanded fns in the same module would collide on a
//! shared top-level name. Nesting the executor gives each expansion its own scope for free — plain
//! Rust name resolution, no macro hygiene tricks needed.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, ItemFn};

//#region 🔖️Expand
/// 🧪️ Rejects a non-`async fn`, rejects generics/parameters (a `#[test]` fn can have neither),
/// then swaps `async` for `#[test]` and wraps the original body in a call to a per-test nested
/// `block_on` — see the module doc for why this crate is plain sync Rust throughout.
pub fn expand_async_test(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(syn::Error::new_spanned(attr, "#[async_test] takes no arguments"));
    }
    let input: ItemFn = syn::parse2(item)?;
    if input.sig.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            &input.sig.fn_token,
            "#[async_test] can only be applied to an `async fn` — the whole point is expanding an async test body to a sync #[test] harness",
        ));
    }
    if !input.sig.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.sig.generics,
            "#[async_test] does not support generic test functions — the compiler's test harness always calls a test fn with zero type arguments",
        ));
    }
    if !input.sig.inputs.is_empty() {
        return Err(syn::Error::new_spanned(&input.sig.inputs, "#[async_test] functions must take no arguments, same as a plain #[test] fn"));
    }

    let other_attrs: Vec<&Attribute> = input.attrs.iter().collect();
    let vis = &input.vis;
    let ident = &input.sig.ident;
    let output = &input.sig.output;
    let block = &input.block;
    let block_on_ident = syn::Ident::new("__semio_async_test_block_on", ident.span());

    Ok(quote! {
        #[test]
        #(#other_attrs)*
        #vis fn #ident() #output {
            fn #block_on_ident<F: std::future::Future>(fut: F) -> F::Output {
                struct ThreadWaker(std::thread::Thread);
                impl std::task::Wake for ThreadWaker {
                    fn wake(self: std::sync::Arc<Self>) {
                        self.0.unpark();
                    }
                    fn wake_by_ref(self: &std::sync::Arc<Self>) {
                        self.0.unpark();
                    }
                }
                let mut fut = std::pin::pin!(fut);
                let waker = std::task::Waker::from(std::sync::Arc::new(ThreadWaker(std::thread::current())));
                let mut cx = std::task::Context::from_waker(&waker);
                loop {
                    match fut.as_mut().poll(&mut cx) {
                        std::task::Poll::Ready(value) => return value,
                        std::task::Poll::Pending => std::thread::park(),
                    }
                }
            }
            #block_on_ident(async move #block)
        }
    })
}
//#endregion 🔖️Expand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn rejects_non_async_fn() {
        let item = quote! { fn plain() {} };
        let err = expand_async_test(TokenStream::new(), item).expect_err("must reject a non-async fn");
        assert!(err.to_string().contains("async fn"));
    }

    #[test]
    fn rejects_arguments() {
        let item = quote! { async fn takes_one(x: i32) {} };
        let err = expand_async_test(TokenStream::new(), item).expect_err("must reject fn arguments");
        assert!(err.to_string().contains("no arguments"));
    }

    #[test]
    fn rejects_generics() {
        let item = quote! { async fn generic<T>() {} };
        let err = expand_async_test(TokenStream::new(), item).expect_err("must reject generic test fns");
        assert!(err.to_string().contains("generic"));
    }

    #[test]
    fn rejects_macro_arguments() {
        let attr: TokenStream = quote! { some_arg };
        let item = quote! { async fn foo() {} };
        let err = expand_async_test(attr, item).expect_err("must reject #[async_test(..)] arguments");
        assert!(err.to_string().contains("no arguments"));
    }

    #[test]
    fn expands_unit_return_to_valid_sync_test() {
        let item = quote! {
            async fn my_case() {
                let x = 1;
                assert_eq!(x, 1);
            }
        };
        let expanded = expand_async_test(TokenStream::new(), item).expect("expansion should succeed");
        let text = expanded.to_string();
        assert!(text.contains("# [test]"));
        assert!(!text.contains("async fn my_case"));
        syn::parse2::<syn::File>(quote! { #expanded }).expect("expanded code should parse as valid Rust");
    }

    #[test]
    fn expands_result_return_type() {
        let item = quote! {
            async fn returns_result() -> Result<(), String> {
                Ok(())
            }
        };
        let expanded = expand_async_test(TokenStream::new(), item).expect("expansion should succeed");
        let text = expanded.to_string();
        assert!(text.contains("Result"));
        syn::parse2::<syn::File>(quote! { #expanded }).expect("expanded code should parse as valid Rust");
    }

    #[test]
    fn preserves_should_panic_and_ignore_in_either_order() {
        let a = quote! {
            #[should_panic(expected = "boom")]
            async fn a() { panic!("boom") }
        };
        let expanded_a = expand_async_test(TokenStream::new(), a).expect("expansion should succeed");
        assert!(expanded_a.to_string().contains("should_panic"));

        let b = quote! {
            #[ignore]
            async fn b() {}
        };
        let expanded_b = expand_async_test(TokenStream::new(), b).expect("expansion should succeed");
        assert!(expanded_b.to_string().contains("ignore"));
    }

    #[test]
    fn preserves_cfg_and_doc_comments() {
        let item = quote! {
            #[cfg(feature = "some")]
            /// a doc comment
            async fn documented() {}
        };
        let expanded = expand_async_test(TokenStream::new(), item).expect("expansion should succeed");
        let text = expanded.to_string();
        assert!(text.contains("cfg"));
        assert!(text.contains("doc"));
    }
}
//#endregion 🧪️Tests
