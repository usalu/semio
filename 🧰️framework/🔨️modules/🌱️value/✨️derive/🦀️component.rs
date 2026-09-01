//! ✨️ `semio_framework_value_derive` — `#[derive(ToValue, FromValue)]` with `#[value(...)]`
//! container/field attributes, mirroring the subset of `#[serde(...)]` actually used under `✏️s/`
//! (see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
//! 🔍️research/📓️serde-replacement-surface.md` §Survey).
//!
//! Whole crate is sync (E3): a proc-macro entry point's signature is language-fixed to
//! `fn(TokenStream) -> TokenStream` and rustc rejects an `async fn` here outright — see
//! `semio-framework-schema-derive`'s identical header note, this crate follows the same shape.
//!
//! Supported container attributes: `rename_all = "camelCase" | "kebab-case" | "lowercase" |
//! "snake_case"`, `tag = "…"` (internally-tagged enum — the ONLY enum representation this derive
//! supports, matching every enum in the survey), `default` (struct-only; every field on the
//! struct falls back to its own `Default::default()`, or the type's own if the type itself is
//! `Default`, on a missing key), `deny_unknown_fields`.
//!
//! Supported field attributes: `rename = "…"`, `default` (bare), `default = "path"`,
//! `skip_serializing_if = "path"`.
//!
//! Deliberately NOT supported (rare in the survey — under 5 occurrences repo-wide each): `tag +
//! content` (adjacently-tagged), `flatten`, `transparent`, `bound(...)`,
//! `serialize_with`/`deserialize_with`, `rename_all_fields`. A crate needing one of these keeps
//! it hand-written (`impl ToValue`/`impl FromValue` directly) rather than deriving.

use quote::quote;
use syn::{Data, DeriveInput, Fields};

//#region 🔖️Case
/// 🐫 Splits a `snake_case` field ident into lowercase words.
fn split_words_snake(ident: &str) -> Vec<String> {
    ident.split('_').filter(|s| !s.is_empty()).map(|s| s.to_lowercase()).collect()
}

/// 🐫 Splits a `PascalCase` variant ident into lowercase words at each uppercase boundary.
fn split_words_pascal(ident: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    for ch in ident.chars() {
        if ch.is_uppercase() && !current.is_empty() {
            words.push(std::mem::take(&mut current).to_lowercase());
        }
        current.push(ch);
    }
    if !current.is_empty() {
        words.push(current.to_lowercase());
    }
    words
}

fn words_to_camel(words: &[String]) -> String {
    let mut out = String::new();
    for (index, word) in words.iter().enumerate() {
        if index == 0 {
            out.push_str(word);
        } else {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                out.extend(first.to_uppercase());
                out.push_str(chars.as_str());
            }
        }
    }
    out
}

fn words_to_kebab(words: &[String]) -> String {
    words.join("-")
}

fn words_to_lower(words: &[String]) -> String {
    words.join("")
}

fn words_to_snake(words: &[String]) -> String {
    words.join("_")
}

/// 🎨️ Applies a `rename_all` case name to `words` (already lowercased word-split).
fn apply_case(words: &[String], case: &str) -> Option<String> {
    match case {
        "camelCase" => Some(words_to_camel(words)),
        "kebab-case" => Some(words_to_kebab(words)),
        "lowercase" => Some(words_to_lower(words)),
        "snake_case" => Some(words_to_snake(words)),
        _ => None,
    }
}

fn field_wire_name(ident: &str, rename: &Option<String>, rename_all: &Option<String>) -> String {
    if let Some(rename) = rename {
        return rename.clone();
    }
    if let Some(case) = rename_all {
        if let Some(cased) = apply_case(&split_words_snake(ident), case) {
            return cased;
        }
    }
    ident.to_string()
}

fn variant_wire_name(ident: &str, rename: &Option<String>, rename_all: &Option<String>) -> String {
    if let Some(rename) = rename {
        return rename.clone();
    }
    if let Some(case) = rename_all {
        if let Some(cased) = apply_case(&split_words_pascal(ident), case) {
            return cased;
        }
    }
    ident.to_string()
}
//#endregion 🔖️Case

//#region 🔖️Attrs
#[derive(Default)]
struct ContainerAttrs {
    rename_all: Option<String>,
    tag: Option<String>,
    default: bool,
    deny_unknown_fields: bool,
}

#[derive(Default, Clone)]
struct FieldAttrs {
    rename: Option<String>,
    default: FieldDefault,
    skip_serializing_if: Option<String>,
}

#[derive(Default, Clone)]
enum FieldDefault {
    #[default]
    None,
    Bare,
    Path(String),
}

/// 🧾️ Reads every `#[value(...)]` attribute on `attrs` into `(key, Option<string-value>)` pairs
/// — `None` for a bare flag (`default`, `deny_unknown_fields`), `Some(..)` for `key = "…"`.
fn parse_value_meta(attrs: &[syn::Attribute]) -> syn::Result<Vec<(String, Option<String>)>> {
    let mut out = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("value") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            let key = meta.path.get_ident().map(std::string::ToString::to_string).ok_or_else(|| meta.error("expected a #[value(...)] identifier"))?;
            if meta.input.peek(syn::Token![=]) {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.push((key, Some(value.value())));
            } else {
                out.push((key, None));
            }
            Ok(())
        })?;
    }
    Ok(out)
}

fn parse_container_attrs(attrs: &[syn::Attribute]) -> syn::Result<ContainerAttrs> {
    let mut out = ContainerAttrs::default();
    for (key, value) in parse_value_meta(attrs)? {
        match key.as_str() {
            "rename_all" => out.rename_all = value,
            "tag" => out.tag = value,
            "default" => out.default = true,
            "deny_unknown_fields" => out.deny_unknown_fields = true,
            other => return Err(syn::Error::new_spanned(&attrs[0], format!("#[value(...)] does not support container attribute `{other}`"))),
        }
    }
    Ok(out)
}

fn parse_field_attrs(attrs: &[syn::Attribute]) -> syn::Result<FieldAttrs> {
    let mut out = FieldAttrs::default();
    for (key, value) in parse_value_meta(attrs)? {
        match key.as_str() {
            "rename" => out.rename = value,
            "default" => out.default = value.map_or(FieldDefault::Bare, FieldDefault::Path),
            "skip_serializing_if" => out.skip_serializing_if = value,
            other => return Err(syn::Error::new_spanned(&attrs[0], format!("#[value(...)] does not support field attribute `{other}`"))),
        }
    }
    Ok(out)
}
//#endregion 🔖️Attrs

//#region 🔖️StructPlan
struct NamedField {
    ident: syn::Ident,
    wire_name: String,
    attrs: FieldAttrs,
}

fn named_fields(fields: &Fields, container: &ContainerAttrs) -> syn::Result<Vec<NamedField>> {
    let syn::Fields::Named(named) = fields else {
        return Err(syn::Error::new_spanned(fields, "#[derive(ToValue, FromValue)] supports named-field structs (and #[value(tag = \"…\")] enums), not tuple/unit structs"));
    };
    named
        .named
        .iter()
        .map(|field| {
            let attrs = parse_field_attrs(&field.attrs)?;
            let ident = field.ident.clone().expect("named field");
            let wire_name = field_wire_name(&ident.to_string(), &attrs.rename, &container.rename_all);
            Ok(NamedField { ident, wire_name, attrs })
        })
        .collect()
}

fn to_value_object_entries(fields: &[NamedField]) -> proc_macro2::TokenStream {
    let pushes = fields.iter().map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let value_expr = quote! { ::semio_framework_os_kernel::ToValue::to_value(&self.#ident) };
        match &field.attrs.skip_serializing_if {
            Some(path) => {
                let path: syn::Path = syn::parse_str(path).expect("valid skip_serializing_if path");
                quote! {
                    if !#path(&self.#ident) {
                        entries.push((#wire_name.to_string(), #value_expr));
                    }
                }
            }
            None => quote! {
                entries.push((#wire_name.to_string(), #value_expr));
            },
        }
    });
    quote! {
        let mut entries: Vec<(String, ::semio_framework_os_kernel::DslValue)> = Vec::new();
        #(#pushes)*
    }
}

fn from_value_struct_fields(fields: &[NamedField], container: &ContainerAttrs) -> proc_macro2::TokenStream {
    let reads = fields.iter().map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let missing = match (&field.attrs.default, container.default) {
            (FieldDefault::Path(path), _) => {
                let path: syn::Path = syn::parse_str(path).expect("valid default path");
                quote! { #path() }
            }
            (FieldDefault::Bare, _) | (FieldDefault::None, true) => quote! { ::std::default::Default::default() },
            (FieldDefault::None, false) => quote! {
                return Err(::semio_framework_os_kernel::ValueError::new(format!("missing field `{}`", #wire_name)))
            },
        };
        quote! {
            let #ident = match __entries.iter().find(|(k, _)| k == #wire_name) {
                Some((_, value)) => ::semio_framework_os_kernel::FromValue::from_value(value.clone()).map_err(|error| error.under(#wire_name))?,
                None => #missing,
            };
        }
    });
    let idents = fields.iter().map(|field| &field.ident);
    quote! {
        #(#reads)*
        Ok(Self { #(#idents),* })
    }
}
//#endregion 🔖️StructPlan

//#region 🔖️Expand
pub fn expand_to_value(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let container = parse_container_attrs(&input.attrs)?;

    let body = match &input.data {
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            let entries = to_value_object_entries(&fields);
            quote! {
                #entries
                ::semio_framework_os_kernel::DslValue::Object(entries)
            }
        }
        Data::Enum(data) => {
            let Some(tag) = &container.tag else {
                return Err(syn::Error::new_spanned(&input.ident, "#[derive(ToValue)] on an enum requires #[value(tag = \"…\")] (internally-tagged) — no other enum representation is supported"));
            };
            let arms = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &None, &container.rename_all);
                let arm: syn::Result<proc_macro2::TokenStream> = match &variant.fields {
                    Fields::Unit => Ok(quote! {
                        Self::#variant_ident => ::semio_framework_os_kernel::DslValue::object([(#tag.to_string(), ::semio_framework_os_kernel::DslValue::String(#wire_variant.to_string()))])
                    }),
                    Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => Ok(quote! {
                        Self::#variant_ident(payload) => {
                            let mut entries = match ::semio_framework_os_kernel::ToValue::to_value(payload) {
                                ::semio_framework_os_kernel::DslValue::Object(entries) => entries,
                                other => vec![("value".to_string(), other)],
                            };
                            entries.insert(0, (#tag.to_string(), ::semio_framework_os_kernel::DslValue::String(#wire_variant.to_string())));
                            ::semio_framework_os_kernel::DslValue::Object(entries)
                        }
                    }),
                    Fields::Named(named) => {
                        let pushes = named.named.iter().map(|field| {
                            let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                            let ident = field.ident.clone().expect("named field");
                            let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.rename_all);
                            quote! { entries.push((#wire_name.to_string(), ::semio_framework_os_kernel::ToValue::to_value(#ident))); }
                        });
                        let idents = named.named.iter().map(|field| field.ident.clone().expect("named field"));
                        Ok(quote! {
                            Self::#variant_ident { #(#idents),* } => {
                                let mut entries: Vec<(String, ::semio_framework_os_kernel::DslValue)> = vec![(#tag.to_string(), ::semio_framework_os_kernel::DslValue::String(#wire_variant.to_string()))];
                                #(#pushes)*
                                ::semio_framework_os_kernel::DslValue::Object(entries)
                            }
                        })
                    }
                    other => Err(syn::Error::new_spanned(other, "#[derive(ToValue)] enum variants must be unit, a single unnamed payload, or named fields")),
                };
                arm
            }).collect::<syn::Result<Vec<_>>>()?;
            quote! {
                match self { #(#arms),* }
            }
        }
        Data::Union(_) => return Err(syn::Error::new_spanned(&input.ident, "#[derive(ToValue)] does not support unions")),
    };

    Ok(quote! {
        impl #impl_generics ::semio_framework_os_kernel::ToValue for #name #ty_generics #where_clause {
            fn to_value(&self) -> ::semio_framework_os_kernel::DslValue {
                #body
            }
        }
    })
}

pub fn expand_from_value(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let container = parse_container_attrs(&input.attrs)?;

    let body = match &input.data {
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            let reads = from_value_struct_fields(&fields, &container);
            quote! {
                let __entries = ::semio_framework_os_kernel::DslValue::into_object(value)?;
                #reads
            }
        }
        Data::Enum(data) => {
            let Some(tag) = &container.tag else {
                return Err(syn::Error::new_spanned(&input.ident, "#[derive(FromValue)] on an enum requires #[value(tag = \"…\")] (internally-tagged) — no other enum representation is supported"));
            };
            let arms = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &None, &container.rename_all);
                let arm: syn::Result<proc_macro2::TokenStream> = match &variant.fields {
                    Fields::Unit => Ok(quote! {
                        #wire_variant => Self::#variant_ident,
                    }),
                    Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                        let payload_ty = &unnamed.unnamed[0].ty;
                        Ok(quote! {
                            #wire_variant => Self::#variant_ident(<#payload_ty as ::semio_framework_os_kernel::FromValue>::from_value(::semio_framework_os_kernel::DslValue::Object(__entries.clone()))?),
                        })
                    }
                    Fields::Named(named) => {
                        let reads = named.named.iter().map(|field| {
                            let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                            let ident = field.ident.clone().expect("named field");
                            let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.rename_all);
                            let missing = match &field_attrs.default {
                                FieldDefault::Path(path) => {
                                    let path: syn::Path = syn::parse_str(path).expect("valid default path");
                                    quote! { #path() }
                                }
                                FieldDefault::Bare => quote! { ::std::default::Default::default() },
                                FieldDefault::None => quote! {
                                    return Err(::semio_framework_os_kernel::ValueError::new(format!("missing field `{}`", #wire_name)))
                                },
                            };
                            quote! {
                                let #ident = match __entries.iter().find(|(k, _)| k == #wire_name) {
                                    Some((_, value)) => ::semio_framework_os_kernel::FromValue::from_value(value.clone()).map_err(|error| error.under(#wire_name))?,
                                    None => #missing,
                                };
                            }
                        });
                        let idents = named.named.iter().map(|field| field.ident.clone().expect("named field"));
                        Ok(quote! {
                            #wire_variant => {
                                #(#reads)*
                                Self::#variant_ident { #(#idents),* }
                            },
                        })
                    }
                    other => Err(syn::Error::new_spanned(other, "#[derive(FromValue)] enum variants must be unit, a single unnamed payload, or named fields")),
                };
                arm
            }).collect::<syn::Result<Vec<_>>>()?;
            quote! {
                let __entries = ::semio_framework_os_kernel::DslValue::into_object(value)?;
                let __tag = __entries.iter().find(|(k, _)| k == #tag).map(|(_, v)| v.clone()).ok_or_else(|| ::semio_framework_os_kernel::ValueError::new(format!("missing tag field `{}`", #tag)))?;
                let __tag = match __tag { ::semio_framework_os_kernel::DslValue::String(s) => s, other => return Err(::semio_framework_os_kernel::ValueError::new(format!("expected a string tag, found {other:?}"))) };
                Ok(match __tag.as_str() {
                    #(#arms)*
                    other => return Err(::semio_framework_os_kernel::ValueError::new(format!("unknown `{}` variant `{other}`", #tag))),
                })
            }
        }
        Data::Union(_) => return Err(syn::Error::new_spanned(&input.ident, "#[derive(FromValue)] does not support unions")),
    };

    Ok(quote! {
        impl #impl_generics ::semio_framework_os_kernel::FromValue for #name #ty_generics #where_clause {
            fn from_value(value: ::semio_framework_os_kernel::DslValue) -> ::core::result::Result<Self, ::semio_framework_os_kernel::ValueError> {
                #body
            }
        }
    })
}
//#endregion 🔖️Expand
