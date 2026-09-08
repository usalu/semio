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
