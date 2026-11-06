//! 📦️ Package glue — proc-macro crate root; implementation in owner `🦀️.rs`.

use semio_framework_value_derive_expansion as component;

#[path="📽️projection/🦀️.rs"]
mod snapshot_projection;

#[path = "🧬️retained-clone/🦀️.rs"]
mod retained_clone;

#[path = "🏭️factory/🦀️.rs"]
mod factory;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// 🏭️ Proves that each concrete factory field is Copy before declaring its payload heap-empty.
#[proc_macro_derive(FactoryPayloadRetirement, attributes(factory_child, factory_owned, value))]
pub fn derive_factory_payload_retirement(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    factory::expand(&input).unwrap_or_else(|error| error.to_compile_error()).into()
}

/// 🗃️ Implements `value::ToValue` for a `#[value(...)]`-annotated struct or enum.
#[proc_macro_derive(ToValue, attributes(value))]
pub fn derive_to_value(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    component::expand_to_value(&derive_input).unwrap_or_else(|e| e.to_compile_error()).into()
}

/// 🌱️ Implements `value::FromValue` for a `#[value(...)]`-annotated struct or enum.
#[proc_macro_derive(FromValue, attributes(value))]
pub fn derive_from_value(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    component::expand_from_value(&derive_input).unwrap_or_else(|e| e.to_compile_error()).into()
}

/// 🚪️ Emits value implementations for an existing native type from its IO-owned representation declaration.
#[proc_macro]
pub fn value_codec(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut projection = true;
    let mut admission = true;
    let mut canonical = false;
    let mut receiving = false;
    for attribute in &input.attrs {
        if attribute.path().is_ident("derive") {
            projection = false;
            admission = false;
            canonical = false;
            receiving = false;
            if let Err(error) = attribute.parse_nested_meta(|meta| {
                if meta.path.is_ident("ToValue") { projection = true; }
                else if meta.path.is_ident("FromValue") { admission = true; }
                else if meta.path.is_ident("CanonicalJsonTree") { canonical = true; }
                else if meta.path.is_ident("NativeReceive") { receiving = true; }
                else { return Err(meta.error("value_codec supports ToValue, FromValue, CanonicalJsonTree and NativeReceive")); }
                Ok(())
            }) { return error.to_compile_error().into(); }
        }
    }
    let expanded = (|| -> syn::Result<proc_macro2::TokenStream> {
        let mut output = proc_macro2::TokenStream::new();
        if projection { output.extend(component::expand_to_value(&input)?); }
        if admission { output.extend(component::expand_from_value(&input)?); }
        if canonical { output.extend(component::expand_canonical_tree(&input)?); }
        if receiving {
            let syn::Data::Struct(data)=&input.data else{return Err(syn::Error::new_spanned(&input,"existing native receiving requires a concrete record owner"))};
            let syn::Fields::Named(fields)=&data.fields else{return Err(syn::Error::new_spanned(&input,"existing native receiving requires exhaustive named fields"))};
            let name=&input.ident;let names=fields.named.iter().map(|field|field.ident.as_ref().unwrap()).collect::<Vec<_>>();let types=fields.named.iter().map(|field|&field.ty).collect::<Vec<_>>();
            output.extend(quote::quote!(const _:fn(#name)->(#(#types,)*)=|#name{#(#names,)*}|(#(#names,)*);));
            output.extend(component::expand_native_receive(&input)?);
        }
        Ok(output)
    })();
    expanded.unwrap_or_else(|error| error.to_compile_error()).into()
}

/// 🧬️ Implements bounded native-owner cloning for a struct or enum; `#[retained_clone(bitwise)]` on a `Copy` struct of bitwise fields makes vectors and arrays of it clone in bulk pages.
#[proc_macro_derive(RetainedClone, attributes(retained_clone,value))]
pub fn derive_retained_clone(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    retained_clone::expand_retained_clone(&derive_input).unwrap_or_else(|error| error.to_compile_error()).into()
}

/// ♻️ Implements exact incremental retirement for every owned field of a struct or enum.
#[proc_macro_derive(RetireOwned, attributes(value))]
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

/// 📽️ Composes original immutable record field projections with paid assembly and cancellation.
#[proc_macro_derive(SnapshotProjection, attributes(value))]
pub fn derive_snapshot_projection(input:TokenStream)->TokenStream{let input=parse_macro_input!(input as DeriveInput);snapshot_projection::expand(&input).unwrap_or_else(|error|error.to_compile_error()).into()}

#[path="🟰️equality/🦀️.rs"] mod retained_equality;
/// 🟰️ Derives bounded native field equality independently of codec skips, defaults, and roles.
#[proc_macro_derive(RetainedEquality,attributes(value))]
pub fn derive_retained_equality(input:TokenStream)->TokenStream{let input=parse_macro_input!(input as DeriveInput);retained_equality::expand(&input).unwrap_or_else(|error|error.to_compile_error()).into()}

/// 🛬️ Derives funded native receiving from genuine original fields independently of cold wire admission.
#[proc_macro_derive(NativeReceive,attributes(value))]
pub fn derive_native_receive(input:TokenStream)->TokenStream{let input=parse_macro_input!(input as DeriveInput);component::expand_native_receive(&input).unwrap_or_else(|error|error.to_compile_error()).into()}

#[path="👁️borrow/🦀️.rs"]
mod native_json_borrow;
/// 👁️ Emits bounded borrowed fields over the declared value wire shape.
#[proc_macro_derive(NativeJsonBorrow,attributes(value,borrowed_json))]
pub fn derive_native_json_borrow(input:TokenStream)->TokenStream{let input=parse_macro_input!(input as DeriveInput);native_json_borrow::expand(&input).unwrap_or_else(|error|error.to_compile_error()).into()}

/// 🧩️ Checks and projects an external type through a domain-local borrowed JSON protocol.
#[proc_macro]
pub fn native_json_codec(input:TokenStream)->TokenStream{let input=parse_macro_input!(input as DeriveInput);native_json_borrow::codec(&input).unwrap_or_else(|error|error.to_compile_error()).into()}
