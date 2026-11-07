//! ✨️ `semio_framework_value_derive` — `#[derive(ToValue, FromValue)]` with `#[value(...)]` [Design notes](README.md#source-notes-1).

use quote::{quote};
use syn::{Data, DeriveInput, Fields};

#[path = "🧭️typed-path/🦀️.rs"]
mod typed_path;
use typed_path::*;
#[path = "🎯️expand-entries/🦀️.rs"]
mod expand_entries;
pub use expand_entries::*;
#[path = "🎚️controlled-output/🦀️.rs"]
mod controlled_output;
use controlled_output::*;

//#region 🔖️Case
/// 🐫 Splits a `snake_case` field ident into lowercase words.
fn split_words_snake(ident: &str) -> Vec<String> {
    ident.split('_').filter(|s| !s.is_empty()).map(|s| s.to_lowercase()).collect()
}

/// 🔪️ Splits a `PascalCase` variant ident into lowercase words at each uppercase boundary.
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
    rename_all_fields: Option<String>,
    tag: Option<String>,
    content: Option<String>,
    default: bool,
    deny_unknown_fields: bool,
    transparent: bool,
    send_only: bool,
    boxed_original: bool,
    receive_recursive: bool,
    bound: Option<String>,
    crate_path: Option<String>,
    native_owned_path: Option<String>,
    equality_path: Option<String>,
    retire_with: Option<String>,
    default_controlled: Option<String>,
    deserialize_controlled_with: Option<String>,
    validate: Option<String>,
    validate_controlled: Option<String>,
}

impl ContainerAttrs {
    /// 🐳️ Variant `rename_all` overrides container `rename_all_fields`; an explicit field rename
    /// wins in `field_wire_name`. Container `rename_all` controls enum variant names only.
    fn field_rename_all(&self, variant: &VariantAttrs) -> Option<String> {
        variant.rename_all.clone().or_else(|| self.rename_all_fields.clone())
    }
}

#[derive(Default)]
struct VariantAttrs {
    rename: Option<String>,
    rename_all: Option<String>,
}

#[derive(Default, Clone)]
struct FieldAttrs {
    rename: Option<String>,
    required: bool,
    default: FieldDefault,
    skip_serializing_if: Option<String>,
    serialize_with: Option<String>,
    serialize_controlled_with: Option<String>,
    deserialize_with: Option<String>,
    deserialize_controlled_with: Option<String>,
    default_controlled: Option<String>,
    retire_with: Option<String>,
    with: Option<String>,
    flatten: bool,
    skip: bool,
}

impl FieldAttrs {
    /// 🩹 `with = "path"` shorthand resolved for the serialize direction: an explicit
    /// `serialize_with` wins, else `path::to_value` when `with` is set, else `None` (the plain
    /// `ToValue::to_value` call).
    fn effective_serialize_with(&self) -> Option<String> {
        self.serialize_with.clone().or_else(|| self.with.as_ref().map(|path| format!("{path}::to_value")))
    }

    /// 🔎️ `with = "path"` shorthand resolved for the deserialize direction — sibling of
    /// `effective_serialize_with` above.
    fn effective_deserialize_with(&self) -> Option<String> {
        self.deserialize_with.clone().or_else(|| self.with.as_ref().map(|path| format!("{path}::from_value")))
    }
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
            let key = meta.path.get_ident().map(ToString::to_string).ok_or_else(|| meta.error("expected a #[value(...)] identifier"))?;
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
            "content" => out.content = value,
            "default" => out.default = true,
            "deny_unknown_fields" => out.deny_unknown_fields = true,
            "transparent" => out.transparent = true,
            "send_only" => out.send_only = true,
            "boxed_original" => out.boxed_original = true,
            "receive_recursive" if value.is_none() => out.receive_recursive = true,
            "bound" => out.bound = value,
            "rename_all_fields" => out.rename_all_fields = value,
            "crate" => out.crate_path = value,
            "native_owned" => out.native_owned_path = value,
            "equality" => out.equality_path = value,
            "retire_with" => out.retire_with = value,
            "default_controlled" => out.default_controlled = value,
            "deserialize_controlled_with" => out.deserialize_controlled_with = value,
            "validate" => out.validate = Some(value.ok_or_else(|| syn::Error::new_spanned(&attrs[0], "value semantic validation requires a function path"))?),
            "validate_controlled" => out.validate_controlled = Some(value.ok_or_else(|| syn::Error::new_spanned(&attrs[0], "controlled value semantic validation requires a function path"))?),
            other => return Err(syn::Error::new_spanned(&attrs[0], format!("#[value(...)] does not support container attribute `{other}`"))),
        }
    }
    Ok(out)
}

/// 🧭️ Resolves `#[value(crate = "path::to::value_root")]` to the crate-path prefix every emitted [Design notes](README.md#source-notes-365).
fn container_crate_path(container: &ContainerAttrs) -> syn::Path {
    let path = container.crate_path.as_deref().unwrap_or("::semio_framework_value");
    syn::parse_str(path).expect("valid #[value(crate = \"...\")] path")
}

/// 🛬️ Resolves the crate that owns `native_decoding`: an explicit `native_owned` key, else the authored `crate` root that supplies everything, else the value facade.
fn container_native_owned_path(container: &ContainerAttrs) -> syn::Path {
    let path = container.native_owned_path.as_deref().or(container.crate_path.as_deref()).unwrap_or("::semio_framework_value");
    syn::parse_str(path).expect("valid #[value(native_owned = \"...\")] path")
}

/// 🟰️ Resolves the crate that owns `retained_equality`: an explicit `equality` key, else the authored `crate` root that supplies everything, else the owner crate.
fn container_equality_path(container: &ContainerAttrs) -> syn::Result<syn::Path> {
    let path = container.equality_path.as_deref().or(container.crate_path.as_deref()).unwrap_or("::semio_framework_value");
    syn::parse_str(path).map_err(|error| syn::Error::new(error.span(), "#[value(equality = \"...\")] requires a valid path"))
}

fn parse_variant_attrs(attrs: &[syn::Attribute]) -> syn::Result<VariantAttrs> {
    let mut out = VariantAttrs::default();
    for (key, value) in parse_value_meta(attrs)? {
        match key.as_str() {
            "rename" => out.rename = value,
            "rename_all" => out.rename_all = value,
            other => return Err(syn::Error::new_spanned(&attrs[0], format!("#[value(...)] does not support variant attribute `{other}`"))),
        }
    }
    Ok(out)
}

fn parse_field_attrs(attrs: &[syn::Attribute]) -> syn::Result<FieldAttrs> {
    let mut out = FieldAttrs::default();
    for (key, value) in parse_value_meta(attrs)? {
        match key.as_str() {
            "rename" => out.rename = value,
            "required" => out.required = true,
            "default" => out.default = value.map_or(FieldDefault::Bare, FieldDefault::Path),
            "skip_serializing_if" => out.skip_serializing_if = value,
            "serialize_with" => out.serialize_with = value,
            "serialize_controlled_with" => out.serialize_controlled_with = value,
            "deserialize_with" => out.deserialize_with = value,
            "deserialize_controlled_with" => out.deserialize_controlled_with = value,
            "default_controlled" => out.default_controlled = value,
            "retire_with" => out.retire_with = value,
            "with" => out.with = value,
            "flatten" => out.flatten = true,
            "skip" => out.skip = true,
            other => return Err(syn::Error::new_spanned(&attrs[0], format!("#[value(...)] does not support field attribute `{other}`"))),
        }
    }
    Ok(out)
}

/// 🧬️ Clones `generics` and adds the `where` bound this impl needs for each of its OWN type [Design notes](README.md#source-notes-423).
fn generics_with_bound(generics: &syn::Generics, bound: &Option<String>, trait_path: &proc_macro2::TokenStream) -> syn::Generics {
    let type_param_idents: Vec<syn::Ident> = generics.type_params().map(|param| param.ident.clone()).collect();
    let mut generics = generics.clone();
    let where_clause = generics.make_where_clause();
    match bound {
        Some(bound) => {
            for predicate in bound.split(',') {
                let predicate = predicate.trim();
                if predicate.is_empty() {
                    continue;
                }
                let predicate: syn::WherePredicate = syn::parse_str(predicate).expect("valid #[value(bound = \"...\")] where predicate");
                where_clause.predicates.push(predicate);
            }
        }
        None => {
            for ident in &type_param_idents {
                let predicate: syn::WherePredicate = syn::parse_quote! { #ident: #trait_path };
                where_clause.predicates.push(predicate);
            }
        }
    }
    generics
}
//#endregion 🔖️Attrs

//#region 🔖️StructPlan
struct NamedField {
    ident: syn::Ident,
    wire_name: String,
    attrs: FieldAttrs,
    is_option: bool,
}

fn variant_path_bindings(fields: &[NamedField]) -> (Vec<proc_macro2::TokenStream>, Vec<NamedField>) {
    fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let original = &field.ident;
            let binding = quote::format_ident!("__semio_value_field_{index}");
            (
                if field.attrs.skip { quote! { #original: _ } } else { quote! { #original: #binding } },
                NamedField { ident: binding, wire_name: field.wire_name.clone(), attrs: field.attrs.clone(), is_option: field.is_option },
            )
        })
        .unzip()
}

fn type_is_option(ty: &syn::Type) -> bool {
    let syn::Type::Path(path) = ty else { return false };
    path.qself.is_none() && path.path.segments.last().is_some_and(|segment| segment.ident == "Option")
}

fn named_fields(fields: &Fields, container: &ContainerAttrs) -> syn::Result<Vec<NamedField>> {
    let Fields::Named(named) = fields else {
        return Err(syn::Error::new_spanned(fields, "#[derive(ToValue, FromValue)] supports named-field structs (and #[value(tag = \"…\")] enums), not tuple/unit structs"));
    };
    let out: Vec<NamedField> = named
        .named
        .iter()
        .map(|field| {
            let attrs = parse_field_attrs(&field.attrs)?;
            let ident = field.ident.clone().expect("named field");
            let wire_name = field_wire_name(&ident.to_string(), &attrs.rename, &container.rename_all);
            Ok(NamedField { ident, wire_name, attrs, is_option: type_is_option(&field.ty) })
        })
        .collect::<syn::Result<_>>()?;
    if container.deny_unknown_fields && out.iter().any(|field| field.attrs.flatten) {
        return Err(syn::Error::new_spanned(named, "#[value(...)] does not support combining `flatten` with `deny_unknown_fields` (matches serde's own restriction)"));
    }
    Ok(out)
}

fn to_value_object_entries(fields: &[NamedField], value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let pushes = fields.iter().map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        if field.attrs.skip {
            return quote! {};
        }
        let value_expr = match field.attrs.effective_serialize_with() {
            Some(path) => {
                let path: syn::Path = syn::parse_str(&path).expect("valid serialize_with path");
                quote! { #path(&self.#ident) }
            }
            None => quote! { #value_crate::ToValue::to_value(&self.#ident) },
        };
        if field.attrs.flatten {
            return quote! {
                if let #value_crate::DslValue::Object(__flat_entries) = #value_expr {
                    entries.extend(__flat_entries);
                }
            };
        }
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
        let mut entries: Vec<(String, #value_crate::DslValue)> = Vec::new();
        #(#pushes)*
    }
}

/// 🛡️ Emits a loop rejecting any key of the `Vec<(String, DslValue)>`-shaped expression
/// `entries_expr` that is not present in `allowed` — the `deny_unknown_fields` enforcement shared
/// by struct bodies (see `from_value_struct_fields` below) and every enum representation in
/// `expand_from_value` (module docs above spell out what "unknown field" scopes to per
/// representation).
fn deny_unknown_keys(entries_expr: &proc_macro2::TokenStream, allowed: &[String], value_crate: &syn::Path) -> proc_macro2::TokenStream {
    quote! {
        for (__key, _) in #entries_expr.iter() {
            if ![#(#allowed),*].contains(&__key.as_str()) {
                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("unknown field `{}`", __key)));
            }
        }
    }
}

fn from_value_struct_fields(fields: &[NamedField], container: &ContainerAttrs, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let non_flatten_names: Vec<String> = fields.iter().filter(|field| !field.attrs.flatten).map(|field| field.wire_name.clone()).collect();
    let deny_check = if container.deny_unknown_fields {
        deny_unknown_keys(&quote! { __entries }, &non_flatten_names, value_crate)
    } else {
        quote! {}
    };
    let reads = fields.iter().map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        if field.attrs.skip {
            let missing = match &field.attrs.default {
                FieldDefault::Path(path) => {
                    let path: syn::Path = syn::parse_str(path).expect("valid default path");
                    quote! { #path() }
                }
                FieldDefault::Bare | FieldDefault::None => quote! { ::std::default::Default::default() },
            };
            return quote! { let #ident = #missing; };
        }
        if field.attrs.flatten {
            let remaining = quote! {
                #value_crate::DslValue::Object(__entries.iter().filter(|(__k, _)| ![#(#non_flatten_names),*].contains(&__k.as_str())).cloned().collect())
            };
            let found = match field.attrs.effective_deserialize_with() {
                Some(path) => {
                    let path: syn::Path = syn::parse_str(&path).expect("valid deserialize_with path");
                    quote! { #path(#remaining).map_err(|error: #value_crate::ValueError| error.under(#wire_name))? }
                }
                None => quote! { #value_crate::FromValue::from_value(#remaining).map_err(|error| error.under(#wire_name))? },
            };
            return quote! { let #ident = #found; };
        }
        let missing = match (&field.attrs.default, container.default, field.attrs.required) {
            (_, _, true) => quote! {
                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing field `{}`", #wire_name)))
            },
            (FieldDefault::Path(path), _, false) => {
                let path: syn::Path = syn::parse_str(path).expect("valid default path");
                quote! { #path() }
            }
            (FieldDefault::Bare, _, false) | (FieldDefault::None, true, false) => quote! { ::std::default::Default::default() },
            (FieldDefault::None, false, false) if field.is_option => quote! { ::std::default::Default::default() },
            (FieldDefault::None, false, false) => quote! {
                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing field `{}`", #wire_name)))
            },
        };
        let found = match field.attrs.effective_deserialize_with() {
            Some(path) => {
                let path: syn::Path = syn::parse_str(&path).expect("valid deserialize_with path");
                quote! { #path(value.clone()).map_err(|error: #value_crate::ValueError| error.under(#wire_name))? }
            }
            None => quote! { #value_crate::FromValue::from_value(value.clone()).map_err(|error| error.under(#wire_name))? },
        };
        quote! {
            let #ident = match __entries.iter().find(|(k, _)| k == #wire_name) {
                Some((_, value)) => #found,
                None => #missing,
            };
        }
    });
    let idents = fields.iter().map(|field| &field.ident);
    quote! {
        #deny_check
        #(#reads)*
        Ok(Self { #(#idents),* })
    }
}
//#endregion 🔖️StructPlan

/// 🎯 Rejects `flatten` on an enum variant's own named field with a `compile_error!` naming the [Design notes](README.md#source-notes-631).
fn check_variant_field_attrs_supported(field: &syn::Field, attrs: &FieldAttrs) -> syn::Result<()> {
    if attrs.flatten {
        return Err(syn::Error::new_spanned(field, format!("#[value(...)] does not support `flatten` on enum variant field `{}` (only plain struct fields support it)", field.ident.as_ref().expect("named field"))));
    }
    Ok(())
}

/// 📤️ Emits one named enum-variant field's `ToValue` push into the accumulator `push_into` [Design notes](README.md#source-notes-650).
fn variant_field_to_value_push(field: &syn::Field, field_attrs: &FieldAttrs, wire_name: &str, ident: &syn::Ident, push_into: &proc_macro2::TokenStream, value_crate: &syn::Path) -> syn::Result<proc_macro2::TokenStream> {
    check_variant_field_attrs_supported(field, field_attrs)?;
    if field_attrs.skip {
        return Ok(quote! {});
    }
    let value_expr = match field_attrs.effective_serialize_with() {
        Some(path) => {
            let path: syn::Path = syn::parse_str(&path).expect("valid serialize_with path");
            quote! { #path(#ident) }
        }
        None => quote! { #value_crate::ToValue::to_value(#ident) },
    };
    Ok(match &field_attrs.skip_serializing_if {
        Some(path) => {
            let path: syn::Path = syn::parse_str(path).expect("valid skip_serializing_if path");
            quote! {
                if !#path(#ident) {
                    #push_into.push((#wire_name.to_string(), #value_expr));
                }
            }
        }
        None => quote! {
            #push_into.push((#wire_name.to_string(), #value_expr));
        },
    })
}

/// 📖️ Emits one named enum-variant field's `FromValue` read out of `entries_ident` (a
/// `Vec<(String, DslValue)>`-shaped expression), honoring `skip` (bypass the wire lookup entirely
/// and always fall back to `default`/`Default::default()` — mirrors `from_value_struct_fields`'s
/// struct-field handling), the pre-existing `default` handling, and `deserialize_with`/`with`
/// (replaces the default `FromValue::from_value` call). Sibling of `variant_field_to_value_push`
/// above.
fn variant_field_from_value_read(field: &syn::Field, field_attrs: &FieldAttrs, wire_name: &str, ident: &syn::Ident, entries_ident: &proc_macro2::TokenStream, value_crate: &syn::Path) -> syn::Result<proc_macro2::TokenStream> {
    check_variant_field_attrs_supported(field, field_attrs)?;
    if field_attrs.skip {
        let missing = match &field_attrs.default {
            FieldDefault::Path(path) => {
                let path: syn::Path = syn::parse_str(path).expect("valid default path");
                quote! { #path() }
            }
            FieldDefault::Bare | FieldDefault::None => quote! { ::std::default::Default::default() },
        };
        return Ok(quote! { let #ident = #missing; });
    }
    let missing = match (&field_attrs.default, field_attrs.required) {
        (_, true) => quote! {
            return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing field `{}`", #wire_name)))
        },
        (FieldDefault::Path(path), false) => {
            let path: syn::Path = syn::parse_str(path).expect("valid default path");
            quote! { #path() }
        }
        (FieldDefault::Bare, false) => quote! { ::std::default::Default::default() },
        (FieldDefault::None, false) if type_is_option(&field.ty) => quote! { ::std::default::Default::default() },
        (FieldDefault::None, false) => quote! {
            return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing field `{}`", #wire_name)))
        },
    };
    let found = match field_attrs.effective_deserialize_with() {
        Some(path) => {
            let path: syn::Path = syn::parse_str(&path).expect("valid deserialize_with path");
            quote! { #path(value.clone()).map_err(|error: #value_crate::ValueError| error.under(#wire_name))? }
        }
        None => quote! { #value_crate::FromValue::from_value(value.clone()).map_err(|error| error.under(#wire_name))? },
    };
    Ok(quote! {
        let #ident = match #entries_ident.iter().find(|(k, _)| k == #wire_name) {
            Some((_, value)) => #found,
            None => #missing,
        };
    })
}
/// 🧹 Builds the `Self::Variant { … }` destructure pattern for `ToValue`'s per-field push
/// generation — a `skip` field destructures as `ident: _` (still exhaustive) instead of binding an
/// unused local, since `variant_field_to_value_push` intentionally never reads a skipped field's
/// binding. Every other field destructures as the shorthand `ident`, unchanged from before.
fn variant_destructure_patterns(named: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    named
        .named
        .iter()
        .map(|field| {
            let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
            let ident = field.ident.clone().expect("named field");
            if field_attrs.skip {
                quote! { #ident: _ }
            } else {
                quote! { #ident }
            }
        })
        .collect()
}
//#endregion 🔖️VariantFields

#[path="../🧵️canonical/🦀️.rs"]
mod canonical_tree;
pub fn expand_canonical_tree(input:&DeriveInput)->syn::Result<proc_macro2::TokenStream>{canonical_tree::expand(input)}

/// 🌱️ Resolves the authored value namespace for an actual typed generator owner.
pub fn declared_value_crate(input:&DeriveInput)->syn::Result<syn::Path>{Ok(container_crate_path(&parse_container_attrs(&input.attrs)?))}
/// 🔐️ Reports whether the owner keeps exclusive Send-only custody and therefore issues no shared original retirement.
pub fn declared_send_only(input:&DeriveInput)->syn::Result<bool>{Ok(parse_container_attrs(&input.attrs)?.send_only)}
/// 🏗️ Resolves the authored owner namespace of structural retained equality for an actual typed generator owner.
pub fn declared_equality_crate(input:&DeriveInput)->syn::Result<syn::Path>{container_equality_path(&parse_container_attrs(&input.attrs)?)}
#[path="../🛬️receive/🦀️.rs"]mod native_receiving;
pub fn expand_native_receive(input:&DeriveInput)->syn::Result<proc_macro2::TokenStream>{native_receiving::expand(input)}

/// 📦️ A defining owner explicitly opts into unique allocation field retirement.
pub fn declared_boxed_original(input:&DeriveInput)->syn::Result<bool>{Ok(parse_container_attrs(&input.attrs)?.boxed_original)}
