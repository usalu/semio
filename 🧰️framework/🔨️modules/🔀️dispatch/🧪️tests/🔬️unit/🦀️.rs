
use super::*;
use quote::quote;

fn parses_as_items(tokens: &TokenStream) {
    syn::parse2::<syn::File>(quote! { #tokens }).unwrap_or_else(|error| {
        panic!("generated code did not parse as valid Rust: {error}\n---\n{tokens}");
    });
}

//#region 🔖️`#[dyn_enum]` — happy paths

#[test]
fn dyn_enum_attribute_reemits_trait_and_emits_dispatch_macro() {
    let input = quote! {
        pub trait Greeter {
            async fn greet(&self, name: &str) -> String;
            fn loud(&self) -> bool { false }
        }
    };
    let expanded = expand_dyn_enum_attribute(TokenStream::new(), input).expect("expansion should succeed");
    parses_as_items(&expanded);
    let text = expanded.to_string();
    assert!(text.contains("trait Greeter"), "trait must be re-emitted unchanged");
    assert!(text.contains("__semio_dispatch_Greeter"), "dispatch macro must be named after the trait");
    assert!(text.contains("macro_export"));
    assert!(text.contains("doc (hidden)") || text.contains("doc(hidden)"));
}

#[test]
fn dyn_enum_attribute_rejects_extra_attribute_args() {
    let attr = quote! { some_arg };
    let item = quote! { trait T { async fn f(&self); } };
    let error = expand_dyn_enum_attribute(attr, item).expect_err("non-empty attribute args must be rejected");
    assert!(error.to_string().contains("takes no arguments"));
}

//#endregion

//#region 🔖️Structural rejections — surfaced inside the captured macro

#[test]
fn analyze_rejects_associated_type() {
    let item_trait: ItemTrait = syn::parse_quote! {
        trait T {
            type Assoc;
            async fn f(&self);
        }
    };
    let body = analyze_and_build_body(&item_trait);
    assert!(body.to_string().contains("compile_error"));
    assert!(body.to_string().contains("associated type"));
}

#[test]
fn analyze_rejects_associated_const() {
    let item_trait: ItemTrait = syn::parse_quote! {
        trait T {
            const N: u32;
            async fn f(&self);
        }
    };
    let body = analyze_and_build_body(&item_trait);
    assert!(body.to_string().contains("compile_error"));
    assert!(body.to_string().contains("associated const"));
}

#[test]
fn analyze_rejects_method_without_receiver() {
    let item_trait: ItemTrait = syn::parse_quote! {
        trait T {
            async fn f() -> u32;
        }
    };
    let body = analyze_and_build_body(&item_trait);
    assert!(body.to_string().contains("compile_error"));
    assert!(body.to_string().contains("no `self` receiver") || body.to_string().contains("no self receiver"));
}

#[test]
fn analyze_rejects_destructuring_parameter_pattern() {
    let item_trait: ItemTrait = syn::parse_quote! {
        trait T {
            async fn f(&self, (a, b): (u32, u32));
        }
    };
    let body = analyze_and_build_body(&item_trait);
    assert!(body.to_string().contains("compile_error"));
    assert!(body.to_string().contains("plain identifier"));
}

#[test]
fn analyze_rejects_unsupported_explicit_self_type() {
    let item_trait: ItemTrait = syn::parse_quote! {
        trait T {
            async fn f(self: std::rc::Rc<Self>);
        }
    };
    let body = analyze_and_build_body(&item_trait);
    assert!(body.to_string().contains("compile_error"));
}

#[test]
fn analyze_rejects_arc_self_mixed_with_mut_self() {
    let item_trait: ItemTrait = syn::parse_quote! {
        trait T {
            async fn a(self: std::sync::Arc<Self>);
            async fn b(&mut self);
        }
    };
    let body = analyze_and_build_body(&item_trait);
    assert!(body.to_string().contains("compile_error"));
    assert!(body.to_string().contains("mixes a `self : Arc < Self >`") || body.to_string().contains("mixes a"));
}

#[test]
fn analyze_combines_multiple_errors() {
    let item_trait: ItemTrait = syn::parse_quote! {
        trait T {
            type Assoc;
            const N: u32;
            async fn f();
        }
    };
    let body = analyze_and_build_body(&item_trait);
    let text = body.to_string();
    let occurrences = text.matches("compile_error").count();
    assert!(occurrences >= 3, "expected one compile_error per distinct defect, got {occurrences} in: {text}");
}

//#endregion

//#region 🔖️Delegation shape

#[test]
fn build_delegate_method_awaits_only_async_methods() {
    let item_trait: ItemTrait = syn::parse_quote! {
        trait T {
            async fn a(&self) -> u32;
            fn b(&self) -> u32;
        }
    };
    let TraitItem::Fn(async_method) = &item_trait.items[0] else { unreachable!() };
    let TraitItem::Fn(sync_method) = &item_trait.items[1] else { unreachable!() };
    let async_tokens = build_delegate_method(async_method).expect("ok").to_string();
    let sync_tokens = build_delegate_method(sync_method).expect("ok").to_string();
    assert!(async_tokens.contains(". await") || async_tokens.contains(".await"));
    assert!(!sync_tokens.contains("await"));
}

#[test]
fn build_delegate_method_strips_mut_from_forwarded_params() {
    let item_trait: ItemTrait = syn::parse_quote! {
        trait T {
            fn f(&self, mut x: u32) -> u32;
        }
    };
    let TraitItem::Fn(method) = &item_trait.items[0] else { unreachable!() };
    let tokens = build_delegate_method(method).expect("ok").to_string();
    assert!(!tokens.contains("mut x"), "generated delegate must not warn unused_mut: {tokens}");
    assert!(tokens.contains("inner . f (x)") || tokens.contains("inner.f(x)"));
}

#[test]
fn build_delegate_method_arc_self_clones_the_variant() {
    let item_trait: ItemTrait = syn::parse_quote! {
        trait T {
            async fn f(self: std::sync::Arc<Self>) -> u32;
        }
    };
    let TraitItem::Fn(method) = &item_trait.items[0] else { unreachable!() };
    let tokens = build_delegate_method(method).expect("ok").to_string();
    assert!(tokens.contains("inner . clone ()") || tokens.contains("inner.clone()"));
    assert!(tokens.contains("match * self") || tokens.contains("match *self"));
    assert!(tokens.contains("ref inner"), "Arc<Self> must bind by `ref`, not move: {tokens}");
}

#[test]
fn build_delegate_method_preserves_generics_and_where_clause() {
    let item_trait: ItemTrait = syn::parse_quote! {
        trait T {
            fn f<X>(&self, x: X) -> X where X: Clone;
        }
    };
    let TraitItem::Fn(method) = &item_trait.items[0] else { unreachable!() };
    let tokens = build_delegate_method(method).expect("ok").to_string();
    assert!(tokens.contains("< X >") || tokens.contains("<X>"));
    assert!(tokens.contains("where"));
}

//#endregion

//#region 🔖️Supertrait assertions

#[test]
fn build_supertrait_assertions_skips_auto_traits() {
    let item_trait: ItemTrait = syn::parse_quote! {
        trait T: Send + Sync {
            async fn f(&self);
        }
    };
    let assertions = build_supertrait_assertions(&item_trait);
    assert!(assertions.is_empty(), "Send/Sync must never be asserted — R3 forbids treating them as ordinary bounds");
}

#[test]
fn build_supertrait_assertions_covers_real_supertraits() {
    let item_trait: ItemTrait = syn::parse_quote! {
        trait T: std::fmt::Debug {
            async fn f(&self);
        }
    };
    let assertions = build_supertrait_assertions(&item_trait);
    assert_eq!(assertions.len(), 1);
    let text = assertions[0].to_string();
    assert!(text.contains("Debug"));
    assert!(text.contains("$ enum_name") || text.contains("$enum_name"), "expected a `$enum_name` metavariable reference: {text}");
}

//#endregion

//#region 🔖️`dyn_enum!` — parse + codegen

#[test]
fn dyn_enum_call_expands_enum_from_impls_and_dispatch_invocation() {
    let input = quote! {
        #[derive(Debug)]
        pub enum Members: SpaceMember {
            Text(TextStore),
            Sketch(SketchStore),
        }
    };
    let expanded = expand_dyn_enum_call(input).expect("expansion should succeed");
    parses_as_items(&expanded);
    let text = expanded.to_string();
    assert!(text.contains("enum Members"));
    assert!(text.contains("Text (TextStore)") || text.contains("Text(TextStore)"));
    assert!(text.contains("From < TextStore > for Members") || text.contains("From<TextStore> for Members"));
    assert!(!text.contains("use "), "must be a BARE invocation — no `use`, see the E-52234 doc comment: {text}");
    assert!(text.contains("__semio_dispatch_SpaceMember ! { Members") || text.contains("__semio_dispatch_SpaceMember! { Members"));
}

#[test]
fn dyn_enum_call_supports_zero_variants() {
    let input = quote! {
        pub enum NoMembers: SpaceMember {}
    };
    let expanded = expand_dyn_enum_call(input).expect("expansion should succeed");
    parses_as_items(&expanded);
    let text = expanded.to_string();
    assert!(text.contains("enum NoMembers { }") || text.contains("enum NoMembers {}"));
}

#[test]
fn dyn_enum_call_two_invocations_for_the_same_trait_in_one_module_both_resolve() {
    // ✌️ A real enum plus its `NoMembers`-shaped empty sibling, closing the SAME trait, in the SAME
    // module — requirement 4's shape. Bare invocation (no `use`) never collides, unlike `use ...;
    // use ...;` would (`E0252`, verified — see the doc comment on `expand_dyn_enum_call`).
    let first = expand_dyn_enum_call(quote! { pub enum Members: SpaceMember { A(ConcreteA) } }).expect("first expansion");
    let second = expand_dyn_enum_call(quote! { pub enum NoMembers: SpaceMember {} }).expect("second expansion");
    assert!(first.to_string().contains("__semio_dispatch_SpaceMember"));
    assert!(second.to_string().contains("__semio_dispatch_SpaceMember"));
}

#[test]
fn dyn_enum_call_qualified_trait_path_still_uses_the_trait_last_segment_for_the_dispatch_macro_name() {
    let input = quote! {
        pub enum Members: other_crate::deep::module::Trait {
            A(ConcreteA),
        }
    };
    let expanded = expand_dyn_enum_call(input).expect("expansion should succeed");
    let text = expanded.to_string();
    assert!(text.contains("__semio_dispatch_Trait ! { Members") || text.contains("__semio_dispatch_Trait! { Members"), "the dispatch macro name always derives from the trait's LAST path segment, regardless of how it was qualified: {text}");
}

#[test]
fn dyn_enum_call_rejects_malformed_variant() {
    let input = quote! {
        pub enum Members: SpaceMember {
            NotAVariant,
        }
    };
    let error = expand_dyn_enum_call(input).expect_err("a variant without a parenthesized type must be rejected");
    let _ = error;
}

//#endregion

//#region 🔖️End-to-end (within this crate): full trait → full enum, output re-parses

#[test]
fn end_to_end_mixed_receivers_default_body_generic_method_parses_as_valid_rust() {
    let trait_tokens = quote! {
        pub trait Store {
            async fn read(&self, key: &str) -> Option<String>;
            async fn write(&mut self, key: &str, value: String);
            fn describe(&self) -> &'static str { "store" }
            fn map_default<X: Default>(&self) -> X { X::default() }
        }
    };
    let attribute_expansion = expand_dyn_enum_attribute(TokenStream::new(), trait_tokens).expect("attribute expansion");
    parses_as_items(&attribute_expansion);

    let enum_tokens = quote! {
        pub enum Stores: Store {
            Text(TextStore),
            Kv(KvStore),
        }
    };
    let call_expansion = expand_dyn_enum_call(enum_tokens).expect("call expansion");
    parses_as_items(&call_expansion);

    let mut whole_file = attribute_expansion.to_string();
    whole_file.push_str(&call_expansion.to_string());
    let combined: TokenStream = whole_file.parse().expect("combined tokens must re-lex");
    parses_as_items(&combined);
}

#[test]
fn end_to_end_forty_plus_methods_does_not_blow_up() {
    let mut method_defs = TokenStream::new();
    for index in 0..45u32 {
        let name = format_ident!("m{index}");
        let is_async = index % 2 == 0;
        let asyncness = if is_async {
            quote! { async }
        } else {
            quote! {}
        };
        method_defs.extend(quote! {
            #asyncness fn #name(&self, x: u32) -> u32;
        });
    }
    let trait_tokens = quote! {
        pub trait Big {
            #method_defs
        }
    };
    let attribute_expansion = expand_dyn_enum_attribute(TokenStream::new(), trait_tokens).expect("attribute expansion");
    parses_as_items(&attribute_expansion);
    assert_eq!(attribute_expansion.to_string().matches("fn m").count(), 45 * 2, "trait re-emitted once, delegate emitted once, per method");

    let enum_tokens = quote! {
        pub enum Bigs: Big {
            A(ConcreteA),
            B(ConcreteB),
            C(ConcreteC),
        }
    };
    let call_expansion = expand_dyn_enum_call(enum_tokens).expect("call expansion");
    parses_as_items(&call_expansion);
}

//#endregion
