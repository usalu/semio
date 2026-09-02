//! 🔀️ `semio_framework_dispatch_macros` — the O1 (drop-dyn-dispatch) replacement mechanism.
//!
//! Two macros that work together across crate boundaries:
//!
//! - `#[dyn_enum]` (attribute, on a trait declaration) re-emits the trait UNCHANGED and additionally
//!   emits a hidden, `#[macro_export]`ed `macro_rules! __semio_dispatch_<TraitName>` that has CAPTURED
//!   the trait's method signatures as literal tokens. This is the standard technique for cross-crate
//!   signature capture: a `macro_rules!` item, once `#[macro_export]`ed, is reachable from any other
//!   crate via `use that_crate::__semio_dispatch_<TraitName>;` (verified empirically — see
//!   `📓️terra-dyn-enum-macro-report.md` §"cross-crate probe" — `#[macro_export]` binds the macro at the
//!   CRATE ROOT, and even same-crate call sites in a different module need that same `use`).
//! - `dyn_enum_close!` (function-like, `dyn_enum_close! { enum E: Trait { V(Ty), .. } }`, at the site that
//!   closes the set) parses the small DSL, emits the real `enum`, one `impl From<VariantTy> for E` per
//!   variant (`From` is E1 — an externally-declared trait, its `fn from` stays sync), and an invocation
//!   of the captured `__semio_dispatch_<TraitName>!` macro that expands to `impl Trait for E`, `match`ing
//!   `self` and delegating every method (`.await` present exactly when the trait method is `async`).
//!
//! ## Why every fn in this crate is plain `fn`, never `async fn`
//!
//! O1 mandates the literal `async` keyword on every first-party fn, with exactly five exception classes
//! (E1–E5, `📌️important.md`). **This whole crate falls under E3** ("proc-macro entry points"), not just
//! its two `#[proc_macro]`/`#[proc_macro_attribute]` entries: `#[proc_macro_derive]`/`#[proc_macro]`/
//! `#[proc_macro_attribute]` functions MUST have the exact signature `fn(TokenStream) -> TokenStream`
//! (rustc calls them synchronously during macro expansion — there is no executor at that point in
//! compilation, so `async fn` there is not merely disallowed by convention, it is a hard compile error).
//! Every helper this crate calls is reached exclusively from that synchronous call graph, transitively,
//! so making a helper `async fn` would force an `.await` with nothing to drive it — there is no legal
//! place in a proc-macro crate for an E5 executor bridge (E5 is capped at one per crate and this crate
//! does zero I/O/waiting, so even that exception does not apply). **This is not a style choice**: I
//! verified it by compiling the sibling `semio-framework-schema-derive` crate, which the asyncify tooling
//! left with `pub async fn derive_artifact_schema(..)`, and it fails with rustc's own words: `error:
//! derive proc macro has incorrect signature … expected fn(TokenStream) -> TokenStream, found
//! fn(TokenStream) -> impl Future<..>` (pasted verbatim in the report). That sibling crate — and
//! `draw-fsm-macros` — are currently BROKEN by the same blind async-ification; flagged, not fixed here
//! (out of my packet's path scope).

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    Attribute, FnArg, Ident, ItemTrait, Pat, Path, Receiver, Signature, Token, TraitItem, TraitItemFn, Type, TypeParamBound, Visibility,
};

//#region 🔖️Receiver classification

/// 🖐️ The four receiver shapes `dyn_enum` can delegate through. Anything else (no receiver at all,
/// or an explicit self-type other than `Arc<Self>` — `Box<Self>`/`Rc<Self>`/`Pin<&mut Self>`/…) is a
/// hard rejection, reported at the method that has it.
enum ReceiverKind {
    ByValue,
    ByRef,
    ByMutRef,
    Arc,
}

/// 🔎 Reads a method's `self` receiver into a [`ReceiverKind`], or an error naming the method when
/// there is no receiver at all (an associated function — cannot be delegated through an enum VALUE)
/// or the receiver is an explicit self-type this macro does not understand.
fn classify_receiver(method_name: &Ident, sig: &Signature) -> syn::Result<ReceiverKind> {
    let Some(receiver) = sig.receiver() else {
        return Err(syn::Error::new_spanned(
            sig,
            format!(
                "dyn_enum: `{method_name}` has no `self` receiver (it is an associated function) and cannot be \
                 delegated through an enum VALUE — give it a `&self`/`&mut self`/`self`/`self: Arc<Self>` \
                 receiver, or move it out of this trait"
            ),
        ));
    };
    classify_receiver_ty(method_name, receiver)
}

fn classify_receiver_ty(method_name: &Ident, receiver: &Receiver) -> syn::Result<ReceiverKind> {
    if receiver.colon_token.is_none() {
        return Ok(match (&receiver.reference, &receiver.mutability) {
            (Some(_), Some(_)) => ReceiverKind::ByMutRef,
            (Some(_), None) => ReceiverKind::ByRef,
            (None, _) => ReceiverKind::ByValue,
        });
    }
    if is_arc_self_type(&receiver.ty) {
        return Ok(ReceiverKind::Arc);
    }
    Err(syn::Error::new_spanned(
        &receiver.ty,
        format!(
            "dyn_enum: `{method_name}` has an explicit `self: {}` receiver — only `&self`, `&mut self`, `self` \
             and `self: Arc<Self>` are supported for enum delegation",
            quote!(#receiver)
        ),
    ))
}

/// 🔗 Structural check for `Arc<Self>` (no type resolution available in a proc-macro — this matches the
/// LAST path segment being literally `Arc` with one generic type argument that is literally `Self`).
fn is_arc_self_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else { return false };
    let Some(segment) = type_path.path.segments.last() else { return false };
    if segment.ident != "Arc" {
        return false;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else { return false };
    matches!(
        args.args.first(),
        Some(syn::GenericArgument::Type(Type::Path(inner))) if inner.path.is_ident("Self")
    )
}

//#endregion 🔖️Receiver classification

//#region 🔖️Parameter analysis

/// 📛 Every non-`self` parameter must bind a plain identifier (`x: T`, optionally `mut x: T`) so the
/// generated delegate can forward it by name — a destructuring pattern (`(a, b): (T, U)`) has no single
/// expression that reproduces it as a call argument. Returns the forwarding identifiers AND a
/// `mut`-stripped clone of each `FnArg` (a `mut` pattern the delegate only READS, never mutates, would
/// otherwise warn `unused_mut` — this repo's gates treat warnings as errors in some lanes).
fn simple_ident_params(method_name: &Ident, sig: &Signature) -> syn::Result<(Vec<Ident>, Punctuated<FnArg, Token![,]>)> {
    let mut names = Vec::new();
    let mut inputs = Punctuated::new();
    for arg in &sig.inputs {
        match arg {
            FnArg::Receiver(receiver) => inputs.push(FnArg::Receiver(receiver.clone())),
            FnArg::Typed(pat_type) => {
                let Pat::Ident(pat_ident) = pat_type.pat.as_ref() else {
                    return Err(syn::Error::new_spanned(
                        &pat_type.pat,
                        format!(
                            "dyn_enum: `{method_name}` has a parameter pattern that is not a plain identifier — \
                             rename it (e.g. `(a, b): (T, U)` → `pair: (T, U)`) so the generated delegate can \
                             forward it by name"
                        ),
                    ));
                };
                names.push(pat_ident.ident.clone());
                let mut pat_type = pat_type.clone();
                if let Pat::Ident(pat_ident) = pat_type.pat.as_mut() {
                    pat_ident.mutability = None;
                }
                inputs.push(FnArg::Typed(pat_type));
            }
        }
    }
    Ok((names, inputs))
}

//#endregion 🔖️Parameter analysis

//#region 🔖️`#[dyn_enum]` — trait capture

/// 🪄️ Re-emits `item` (a trait declaration) UNCHANGED, plus a hidden `__semio_dispatch_<Name>!`
/// `macro_rules!` capturing its method signatures for `dyn_enum_enum!` to close later. Never fails the
/// trait declaration itself — a structural blocker (no-receiver method, associated type/const, a
/// parameter that cannot be forwarded, or `self: Arc<Self>` mixed with `&mut self` on the same trait)
/// is instead baked into the captured macro as a `compile_error!`, so it surfaces exactly where someone
/// tries to CLOSE an enum over the trait, not merely because the trait was annotated.
pub fn expand_dyn_enum_attribute(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(syn::Error::new_spanned(attr, "dyn_enum: the attribute takes no arguments"));
    }
    let item_trait: ItemTrait = syn::parse2(item)?;
    let dispatch_macro = build_dispatch_macro(&item_trait);
    Ok(quote! {
        #item_trait
        #dispatch_macro
    })
}

/// 🏗️ Builds the whole `#[macro_export] macro_rules! __semio_dispatch_<Name> { .. }` item.
fn build_dispatch_macro(item_trait: &ItemTrait) -> TokenStream {
    let macro_name = dispatch_macro_ident(&item_trait.ident);
    let body = analyze_and_build_body(item_trait);
    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #macro_name {
            ($enum_name:ident { $($variant:ident($ty:ty)),* $(,)? }) => {
                #body
            };
        }
    }
}

fn dispatch_macro_ident(trait_name: &Ident) -> Ident {
    format_ident!("__semio_dispatch_{}", trait_name)
}

/// 🧮 Either the real `impl Trait for $enum_name { .. }` (plus supertrait assertions), or a
/// `compile_error!` standing in for it — chosen ONCE here, since the trait's own shape (not the
/// variant list, which is not known until `dyn_enum_enum!` runs) is what determines whether it can be
/// delegated at all.
fn analyze_and_build_body(item_trait: &ItemTrait) -> TokenStream {
    let mut errors: Vec<syn::Error> = Vec::new();
    let mut methods: Vec<&TraitItemFn> = Vec::new();

    for trait_item in &item_trait.items {
        match trait_item {
            TraitItem::Fn(method) => methods.push(method),
            TraitItem::Type(assoc_type) => errors.push(syn::Error::new_spanned(
                assoc_type,
                format!(
                    "dyn_enum: associated type `{}` cannot be enum-delegated (an enum has no single type to give \
                     it) — remove it from this trait, or hand-write `impl {} for <YourEnum>` instead of closing \
                     it with dyn_enum!",
                    assoc_type.ident, item_trait.ident
                ),
            )),
            TraitItem::Const(assoc_const) => errors.push(syn::Error::new_spanned(
                assoc_const,
                format!(
                    "dyn_enum: associated const `{}` cannot be enum-delegated (an enum has no single value to \
                     give it) — remove it from this trait, or hand-write `impl {} for <YourEnum>` instead of \
                     closing it with dyn_enum!",
                    assoc_const.ident, item_trait.ident
                ),
            )),
            other => errors.push(syn::Error::new_spanned(other, "dyn_enum: unrecognized trait item — only methods are supported for enum delegation")),
        }
    }

    let mut receiver_kinds: Vec<ReceiverKind> = Vec::new();
    for method in &methods {
        match classify_receiver(&method.sig.ident, &method.sig) {
            Ok(kind) => receiver_kinds.push(kind),
            Err(error) => errors.push(error),
        }
    }
    let has_arc_receiver = receiver_kinds.iter().any(|kind| matches!(kind, ReceiverKind::Arc));
    let has_mut_ref_receiver = receiver_kinds.iter().any(|kind| matches!(kind, ReceiverKind::ByMutRef));
    if has_arc_receiver && has_mut_ref_receiver {
        errors.push(syn::Error::new_spanned(
            &item_trait.ident,
            format!(
                "dyn_enum: `{}` mixes a `self: Arc<Self>` method with a `&mut self` method — this cannot be \
                 auto-delegated (an `Arc<Self>` method requires every variant to store `Arc<Concrete>`, and \
                 `&mut self` cannot safely reach through a shared `Arc`); split the trait, or hand-write the \
                 `&mut self` method's delegation yourself",
                item_trait.ident
            ),
        ));
    }

    let mut delegate_methods = Vec::new();
    if errors.is_empty() {
        for method in &methods {
            match build_delegate_method(method) {
                Ok(tokens) => delegate_methods.push(tokens),
                Err(error) => errors.push(error),
            }
        }
    }

    if let Some(combined) = errors.into_iter().reduce(|mut first, next| {
        first.combine(next);
        first
    }) {
        return combined.to_compile_error();
    }

    let trait_ident = &item_trait.ident;
    let (impl_generics, _, where_clause) = item_trait.generics.split_for_impl();
    let supertrait_assertions = build_supertrait_assertions(item_trait);

    quote! {
        // 🔇 `unused_variables`: a zero-variant closing enum degenerates every method's match to zero
        // arms, so every non-`self` PARAMETER goes unreferenced in that impl — correct, not a defect
        // (`match *self {}` is exhaustive precisely because it is never reachable). `unused_mut`: never
        // actually needed (parameter patterns are `mut`-stripped in `simple_ident_params`), kept as a
        // second, cheap belt-and-braces guard against the SAME zero-arm shape. Neither allow adds a
        // bound or changes a signature — R7 is satisfied by lint suppression, not by rewriting types.
        #[allow(unused_variables, unused_mut)]
        impl #impl_generics #trait_ident for $enum_name #where_clause {
            #(#delegate_methods)*
        }
        #(#supertrait_assertions)*
    }
}

/// 🚚 One delegating method: `match self { $(Self::$variant(inner) => inner.name(args).await,)* }` for
/// `&self`/`&mut self`/`self` (uniform via match ergonomics — verified against real rustc, see report),
/// or `match &*self { $(Self::$variant(inner) => inner.clone().name(args).await,)* }` for `self:
/// Arc<Self>` (every variant's inner type must itself be `Arc<Concrete>` — `inner.clone()` is then the
/// cheap refcount bump that reproduces the `Arc<Concrete>` receiver the concrete impl expects).
fn build_delegate_method(method: &TraitItemFn) -> syn::Result<TokenStream> {
    let sig = &method.sig;
    let method_name = &sig.ident;
    let receiver_kind = classify_receiver(method_name, sig)?;
    let (arg_names, inputs) = simple_ident_params(method_name, sig)?;

    let asyncness = &sig.asyncness;
    let generics = &sig.generics;
    let where_clause = &sig.generics.where_clause;
    let output = &sig.output;
    let dot_await = asyncness.map(|_| quote! { .await });

    // 🎯 `*self` (a DEREF'd PLACE, not the bare reference `self`) for every receiver that isn't owned —
    // verified against real rustc for BOTH arm counts: `match self {}` on `&Self`/`&mut Self` is
    // rejected ("references are always considered inhabited", E0004), but `match *self {}` is accepted
    // (an empty match on the deref'd, genuinely-uninhabited `Self` place). For the N-arm case the SAME
    // `*self` scrutinee, combined with an explicit `ref`/`ref mut` binding mode (never bare — relying on
    // match-ergonomics-through-a-deref alone was ambiguous enough to be worth pinning down explicitly),
    // still binds `inner` at the right reference kind without a move-out-of-borrow error — also
    // verified. `self: Arc<Self>` derefs the SAME way (`*self` on `Arc<Self>` yields the `Self` place
    // through `Arc`'s `Deref`), so all three reference-shaped receivers share one scrutinee/pattern
    // template; only owned `self` (no `Deref` to go through) uses the bare, unref'd form.
    let (scrutinee, pattern_binding, receiver_expr) = match receiver_kind {
        ReceiverKind::ByValue => (quote! { self }, quote! { inner }, quote! { inner }),
        ReceiverKind::ByRef => (quote! { *self }, quote! { ref inner }, quote! { inner }),
        ReceiverKind::ByMutRef => (quote! { *self }, quote! { ref mut inner }, quote! { inner }),
        ReceiverKind::Arc => (quote! { *self }, quote! { ref inner }, quote! { inner.clone() }),
    };
    let call = quote! { #receiver_expr.#method_name(#(#arg_names),*) #dot_await };

    Ok(quote! {
        #asyncness fn #method_name #generics (#inputs) #output #where_clause {
            match #scrutinee {
                $( Self::$variant(#pattern_binding) => #call, )*
            }
        }
    })
}

/// ⛓️ For every REAL supertrait bound (anything beyond the auto-marker traits `Send`/`Sync`/`Unpin`,
/// `Sized`, and lifetime bounds — those are satisfied structurally by any concrete enum whose variants
/// satisfy them, per ruling R3, and asserting them would be pointless), emits a monomorphic assertion
/// that fails to compile with the native "the trait bound `$enum_name: Bound` is not satisfied" error —
/// naming the missing bound clearly — UNLESS a `dyn_enum!` caller has already hand-written that impl for
/// the enum, in which case the assertion silently passes. This is deliberately NOT a hard block: R3
/// forbids ADDING Send/Sync bounds, but a REAL (non-auto) supertrait genuinely needs a manual impl, and
/// this only detects and reports that — never adds anything to the generated `impl Trait for $enum_name`
/// itself.
fn build_supertrait_assertions(item_trait: &ItemTrait) -> Vec<TokenStream> {
    item_trait
        .supertraits
        .iter()
        .filter_map(|bound| match bound {
            TypeParamBound::Trait(trait_bound) => {
                let last = trait_bound.path.segments.last()?;
                if matches!(last.ident.to_string().as_str(), "Send" | "Sync" | "Unpin" | "Sized") {
                    return None;
                }
                let path = &trait_bound.path;
                let assert_fn = format_ident!("__dyn_enum_requires_manual_impl_of_{}_for", last.ident);
                Some(quote! {
                    #[allow(non_snake_case, unused)]
                    const _: fn() = || {
                        fn #assert_fn<T: #path>() {}
                        #assert_fn::<$enum_name>();
                    };
                })
            }
            TypeParamBound::Lifetime(_) | TypeParamBound::PreciseCapture(_) | TypeParamBound::Verbatim(_) => None,
            _ => None,
        })
        .collect()
}

//#endregion 🔖️`#[dyn_enum]` — trait capture

//#region 🔖️`dyn_enum!` — enum closing site

/// 📥️ `dyn_enum! { #[derive(Debug)] pub enum Members: Trait { Text(TextStore), Sketch(SketchStore) } }`.
struct DynEnumInput {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    trait_path: Path,
    variants: Punctuated<DynEnumVariant, Token![,]>,
}

struct DynEnumVariant {
    ident: Ident,
    ty: Type,
}

impl Parse for DynEnumInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        input.parse::<Token![enum]>()?;
        let ident: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let trait_path: Path = input.parse()?;
        let content;
        braced!(content in input);
        let variants = content.parse_terminated(DynEnumVariant::parse, Token![,])?;
        Ok(Self { attrs, vis, ident, trait_path, variants })
    }
}

impl Parse for DynEnumVariant {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let content;
        parenthesized!(content in input);
        let ty: Type = content.parse()?;
        Ok(Self { ident, ty })
    }
}

/// 🪄️ Expands the `dyn_enum!` DSL into the real `enum`, one `impl From<VariantTy>` per variant, and a
/// BARE invocation of the trait's captured `__semio_dispatch_<Name>!` macro — deliberately no `use`.
///
/// Emitting `use crate::__semio_dispatch_<Name>;` (a same-crate ABSOLUTE path) here would trip a real,
/// verified-against-rustc restriction: `error: macro-expanded 'macro_export' macros from the current
/// crate cannot be referred to by absolute paths` (rust-lang/rust#52234) — `__semio_dispatch_<Name>` is
/// exactly that (it was produced by `#[dyn_enum]`'s OWN expansion, not hand-written), and the lint fires
/// on ANY absolute-path reference to it from the SAME crate, `use` or a qualified `crate::name!(..)`
/// invocation alike. It is future-incompatible (today `warn`-level here because the workspace downgrades
/// the whole `future_incompatible` group; a future rustc makes it a hard error) and would otherwise
/// spam every one of the ~90 same-crate applications.
///
/// The fix that sidesteps it entirely: a BARE, unqualified invocation relies on ordinary `macro_rules!`
/// textual scoping instead of the crate-root/absolute-path mechanism `#[macro_export]` adds on top —
/// verified working, zero warnings, for the common case where `dyn_enum!`'s trait and its closing enum
/// share a module and the trait comes first textually (true of every family in this program: the trait
/// declaration is `#[dyn_enum]`-annotated once, upstream of every enum that closes it). Two (or more)
/// `dyn_enum!` invocations for the SAME trait in the SAME module — e.g. a real enum plus its
/// `NoMembers`-shaped empty sibling (requirement 4) — both resolve the SAME bare name without conflict
/// (unlike `use`, which cannot import one name twice — `E0252`, also verified). **Recipe**: if a
/// `dyn_enum!` call site is in a DIFFERENT module or crate than the trait declaration, write `use
/// crate::__semio_dispatch_<TraitName>;` (or `use other_crate::…`) yourself, immediately above the
/// `dyn_enum!` call — this is the one piece of cross-module/cross-crate wiring `dyn_enum!` cannot inject
/// silently, documented in `📓️terra-dyn-enum-macro-report.md`'s "applying dyn_enum: the recipe".
pub fn expand_dyn_enum_call(input: TokenStream) -> syn::Result<TokenStream> {
    let parsed: DynEnumInput = syn::parse2(input)?;
    let DynEnumInput { attrs, vis, ident, trait_path, variants } = parsed;

    let variant_idents: Vec<&Ident> = variants.iter().map(|variant| &variant.ident).collect();
    let variant_types: Vec<&Type> = variants.iter().map(|variant| &variant.ty).collect();

    let enum_def = quote! {
        #(#attrs)*
        #vis enum #ident {
            #( #variant_idents(#variant_types) ),*
        }
    };

    let from_impls = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let ty = &variant.ty;
        quote! {
            impl ::core::convert::From<#ty> for #ident {
                fn from(value: #ty) -> Self {
                    Self::#variant_ident(value)
                }
            }
        }
    });

    let Some(trait_last) = trait_path.segments.last() else {
        return Err(syn::Error::new_spanned(&trait_path, "dyn_enum!: empty trait path"));
    };
    let macro_ident = dispatch_macro_ident(&trait_last.ident);

    Ok(quote! {
        #enum_def
        #(#from_impls)*
        #macro_ident! {
            #ident { #( #variant_idents(#variant_types) ),* }
        }
    })
}

//#endregion 🔖️`dyn_enum!` — enum closing site

//#region 🧪️Tests

#[cfg(test)]
mod tests {
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
}

//#endregion 🧪️Tests
