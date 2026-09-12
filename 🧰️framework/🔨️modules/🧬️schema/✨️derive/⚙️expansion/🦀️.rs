//! ✨️ `semio_framework_schema_derive` — `#[derive(ArtifactSchema)]` with `#[artifact_schema]` /
//! `#[state]` / `#[derived]` / `#[child]` / `#[link_slot]`.
//!
//! Whole crate is sync (E3): a proc-macro entry point's signature is language-fixed to
//! `fn(TokenStream) -> TokenStream` and rustc rejects an `async fn` here outright (a proc macro
//! runs inside rustc at compile time, where there is no executor to poll it). Since none of this
//! crate's helpers do I/O, keeping every fn in the file sync — not just the tagged entry points —
//! avoids threading `block_on` through code that has nothing to await.

use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericArgument, Meta, PathArguments, Type};

//#region 🔖️Helpers
/// 🐪 snake_case ident → canonical camelCase JSON field name.
fn snake_to_camel(name: &str) -> String {
    let mut out = String::new();
    let mut upper = false;
    for ch in name.chars() {
        if ch == '_' {
            upper = true;
            continue;
        }
        if upper {
            out.extend(ch.to_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}

fn parse_artifact_id(input: &DeriveInput) -> syn::Result<String> {
    for attr in &input.attrs {
        if !attr.path().is_ident("artifact_schema") {
            continue;
        }
        let mut id = None;
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("id") {
                let value: syn::LitStr = meta.value()?.parse()?;
                id = Some(value.value());
                Ok(())
            } else {
                Err(meta.error("unsupported artifact_schema attribute"))
            }
        })?;
        return id.ok_or_else(|| syn::Error::new_spanned(attr, "artifact_schema requires id = \"…\""));
    }
    Err(syn::Error::new_spanned(&input.ident, "missing #[artifact_schema(id = \"…\")]"))
}

/// 🗂️ A field sits on exactly ONE of two orthogonal axes: it is either STATE (one of the four
/// lanes, `#[state(artifact|config|presence|transient)]`) or DERIVED (`#[derived]`, computed from a
/// snapshot and therefore not state at all). Carrying both, or neither, is a compile error.
enum FieldAxis {
    State(String),
    Derived,
}

fn parse_field_axis(field: &syn::Field) -> syn::Result<FieldAxis> {
    let derived = field.attrs.iter().find(|attr| attr.path().is_ident("derived"));
    let state = field.attrs.iter().find(|attr| attr.path().is_ident("state"));
    if let (Some(derived), Some(_)) = (derived, state) {
        return Err(syn::Error::new_spanned(derived, "a field is either #[state(…)] or #[derived], never both"));
    }
    if derived.is_some() {
        return Ok(FieldAxis::Derived);
    }
    let Some(attr) = state else {
        return Err(syn::Error::new_spanned(field, "missing #[state(…)] or #[derived] on field"));
    };
    let Meta::List(list) = &attr.meta else {
        return Err(syn::Error::new_spanned(attr, "expected #[state(artifact|config|presence|transient)]"));
    };
    let tokens = list.tokens.to_string().replace(' ', "");
    let variant = match tokens.as_str() {
        "artifact" => "Artifact",
        "config" => "Config",
        "presence" => "Presence",
        "transient" => "Transient",
        other => {
            return Err(syn::Error::new_spanned(
                attr,
                format!("unknown state class `{other}` — the only four lanes are artifact, config, presence, transient"),
            ))
        }
    };
    Ok(FieldAxis::State(variant.to_string()))
}

/// 🧭️ Classifies a field's composition role (CHILD / LINK / neither) by matching the LAST path
/// segment of its type syntactically — this proc-macro crate never resolves types, only names.
enum CompositionFieldKind {
    Child,
    Link { many: bool },
    None,
}

/// 🎁️ Strips one level of `Option<…>`, since a slot's presence-or-absence is orthogonal to its
/// `many` cardinality — `Option<ArtifactChild<T>>` is still a single (non-`many`) child slot.
fn unwrap_option(ty: &Type) -> &Type {
    let Type::Path(type_path) = ty else { return ty };
    let Some(segment) = type_path.path.segments.last() else { return ty };
    if segment.ident != "Option" {
        return ty;
    }
    let PathArguments::AngleBracketed(args) = &segment.arguments else { return ty };
    match args.args.first() {
        Some(GenericArgument::Type(inner)) => inner,
        _ => ty,
    }
}

fn last_segment_ident(ty: &Type) -> Option<&syn::Ident> {
    let Type::Path(type_path) = ty else { return None };
    type_path.path.segments.last().map(|segment| &segment.ident)
}

fn vec_element_type(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else { return None };
    let segment = type_path.path.segments.last()?;
    if segment.ident != "Vec" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &segment.arguments else { return None };
    match args.args.first() {
        Some(GenericArgument::Type(inner)) => Some(inner),
        _ => None,
    }
}

fn classify_composition_field(ty: &Type) -> CompositionFieldKind {
    let ty = unwrap_option(ty);
    if let Some(element) = vec_element_type(ty) {
        let element = unwrap_option(element);
        return match last_segment_ident(element).map(|ident| ident.to_string()).as_deref() {
            Some("ArtifactChild") => CompositionFieldKind::Child,
            Some("ArtifactLink") => CompositionFieldKind::Link { many: true },
            _ => CompositionFieldKind::None,
        };
    }
    match last_segment_ident(ty).map(|ident| ident.to_string()).as_deref() {
        Some("ArtifactChild") => CompositionFieldKind::Child,
        Some("ArtifactLink") => CompositionFieldKind::Link { many: false },
        _ => CompositionFieldKind::None,
    }
}

/// 📎️ `#[child(kind = "s.stdio.mesh")]` — REQUIRED on every `ArtifactChild<T>` / `Vec<ArtifactChild<T>>` field.
fn parse_child_kind(field: &syn::Field) -> syn::Result<String> {
    for attr in &field.attrs {
        if !attr.path().is_ident("child") {
            continue;
        }
        let mut kind = None;
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("kind") {
                let value: syn::LitStr = meta.value()?.parse()?;
                kind = Some(value.value());
                Ok(())
            } else {
                Err(meta.error("unsupported child attribute"))
            }
        })?;
        return kind.ok_or_else(|| syn::Error::new_spanned(attr, "child requires kind = \"…\""));
    }
    Err(syn::Error::new_spanned(field, "ArtifactChild field requires #[child(kind = \"…\")]"))
}

/// 📎️ `#[link_slot(roles("base", "material"))]` — OPTIONAL on `ArtifactLink` / `Vec<ArtifactLink>` fields,
/// empty when absent. Named `link_slot`, NOT `link` — `link` is a real Rust built-in attribute (extern
/// block FFI linking) and colliding with it is a hard `E0659`/`E0539`/`E0459` compile error, not merely
/// a lint, once the attribute is actually applied to a field.
fn parse_link_roles(field: &syn::Field) -> syn::Result<Vec<String>> {
    for attr in &field.attrs {
        if !attr.path().is_ident("link_slot") {
            continue;
        }
        let mut roles = Vec::new();
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("roles") {
                let content;
                syn::parenthesized!(content in meta.input);
                let list = content.parse_terminated(<syn::LitStr as syn::parse::Parse>::parse, syn::Token![,])?;
                for lit in list {
                    roles.push(lit.value());
                }
                Ok(())
            } else {
                Err(meta.error("unsupported link attribute"))
            }
        })?;
        return Ok(roles);
    }
    Ok(Vec::new())
}
//#endregion 🔖️Helpers

//#region 🔖️ArtifactSchema
pub fn expand_artifact_schema(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let id = parse_artifact_id(input)?;
    let ident = &input.ident;
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(ident, "ArtifactSchema only supports structs"));
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(ident, "ArtifactSchema requires named fields"));
    };

    let mut field_entries = Vec::new();
    let mut derived_entries = Vec::new();
    let mut child_entries = Vec::new();
    let mut child_visits = Vec::new();
    let mut link_entries = Vec::new();
    for field in &fields.named {
        let name = field.ident.as_ref().ok_or_else(|| syn::Error::new_spanned(field, "named field required"))?;
        let camel = snake_to_camel(&name.to_string());
        match parse_field_axis(field)? {
            FieldAxis::State(variant) => {
                let variant_ident = syn::Ident::new(&variant, name.span());
                field_entries.push(quote! {
                    (#camel, ::semio_framework_schema::StateClass::#variant_ident)
                });
            }
            FieldAxis::Derived => derived_entries.push(quote! { #camel }),
        }

        if field.attrs.iter().any(|attr| attr.path().is_ident("child")) {
            let kind = parse_child_kind(field)?;
            let kind_lit = syn::LitStr::new(&kind, name.span());
            let name_lit = syn::LitStr::new(&camel, name.span());
            let ty = &field.ty;
            child_entries.push(quote! {
                ::semio_framework_schema::ChildSlotSpec { name: #name_lit, kind: #kind_lit, many: <#ty as ::semio_framework_schema::ChildFieldRefs>::MANY }
            });
            child_visits.push(quote! {
                ::semio_framework_schema::ChildFieldRefs::visit_child_field(&self.#name, #name_lit, visitor)?;
            });
            continue;
        }
        match classify_composition_field(&field.ty) {
            CompositionFieldKind::Child => { parse_child_kind(field)?; }
            CompositionFieldKind::Link { many } => {
                let roles = parse_link_roles(field)?;
                let name_lit = syn::LitStr::new(&camel, name.span());
                let role_lits = roles.iter().map(|role| syn::LitStr::new(role, name.span()));
                link_entries.push(quote! {
                    ::semio_framework_schema::LinkSlotSpec { name: #name_lit, roles: &[#(#role_lits),*], many: #many }
                });
            }
            CompositionFieldKind::None => {}
        }
    }

    let id_lit = syn::LitStr::new(&id, ident.span());
    Ok(quote! {
        impl ::semio_framework_schema::ArtifactSchemaFields for #ident {
            async fn artifact_schema_id() -> &'static str {
                #id_lit
            }
            async fn field_states() -> &'static [(&'static str, ::semio_framework_schema::StateClass)] {
                &[#(#field_entries),*]
            }
            async fn derived_fields() -> &'static [&'static str] {
                &[#(#derived_entries),*]
            }
        }

        impl ::semio_framework_schema::ArtifactCompositionFields for #ident {
            fn visit_child_refs<'a, V: ::semio_framework_schema::ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
                #(#child_visits)*
                Ok(())
            }
            fn child_slots() -> &'static [::semio_framework_schema::ChildSlotSpec] {
                &[#(#child_entries),*]
            }
            fn link_slots() -> &'static [::semio_framework_schema::LinkSlotSpec] {
                &[#(#link_entries),*]
            }
        }
    })
}
//#endregion 🔖️ArtifactSchema
