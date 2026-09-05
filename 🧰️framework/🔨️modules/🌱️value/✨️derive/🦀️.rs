//! ✨️ `semio_framework_value_derive` — `#[derive(ToValue, FromValue)]` with `#[value(...)]`
//! container/field attributes, mirroring the subset of `#[serde(...)]` actually used under `✏️s/`
//! (see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
//! 🔍️research/📓️serde-replacement-surface.md` §Survey).
//!
//! Whole crate is sync (E3): a proc-macro entry point's signature is language-fixed to
//! `fn(TokenStream) -> TokenStream` and rustc rejects an `async fn` here outright — see
//! `semio-framework-schema-derive`'s identical header note, this crate follows the same shape.
//!
//! `#[value(crate = "path::to::value_root")]` (container): overrides the crate path every emitted
//! call site (`ToValue`, `FromValue`, `DslValue`, `ValueError`) is qualified with, defaulting to
//! `::semio_framework_os_kernel` when absent — a container with no `crate` attribute emits
//! byte-identical code to before this attribute existed. Mirrors `#[serde(crate = "…")]`. Exists so
//! a crate BELOW `os-kernel` in the dependency DAG (e.g. `semio-framework-actor`, which
//! `os-kernel` itself depends on, so depending back would be a Cargo cycle) can still use this
//! derive by pointing it at wherever it reexports `DslValue`/`ToValue`/`FromValue`/`ValueError`
//! from instead — `#[value(crate = "crate::value")]` etc.
//!
//! Supported container attributes: `rename_all = "camelCase" | "kebab-case" | "lowercase" |
//! "snake_case"`, `tag = "…"` (internally-tagged enum), `tag = "…" + content = "…"`
//! (adjacently-tagged enum). A `tag`-less enum derives too: an all-unit-variant enum becomes a
//! bare `DslValue::String` of the variant's wire name, matching serde's own default
//! representation for a data-less enum (`SelectionMode::Single` → `"single"`, not
//! `{"tag":"single"}`); a `tag`-less enum with at least one data-carrying variant derives as
//! EXTERNALLY-tagged (serde's own default enum representation when no `#[serde(tag = …)]` is
//! present) — a unit variant is still the bare wire-name string, a single-unnamed-field or
//! named-field variant becomes a one-key object `{"VariantName": <payload>}`. `default`
//! (struct-only; every field on the struct falls back to its own `Default::default()`, or the
//! type's own if the type itself is `Default`, on a missing key), `deny_unknown_fields`.
//!
//! Supported field attributes: `rename = "…"`, `default` (bare), `default = "path"`,
//! `skip_serializing_if = "path"`, `serialize_with = "path"` (`fn(&FieldType) ->
//! DslValue`, replaces the `ToValue::to_value` call for that field), `deserialize_with = "path"`
//! (`fn(DslValue) -> Result<FieldType, ValueError>`, replaces the `FromValue::from_value` call —
//! combine with bare `default` for a "missing key defaults, present key goes through the custom
//! fn" split, the `deserialize_double_option` shape), `with = "path"` (shorthand for
//! `serialize_with = "path::to_value"` + `deserialize_with = "path::from_value"`; an explicit
//! `serialize_with`/`deserialize_with` given alongside `with` wins for that one direction), `skip`
//! (struct fields only — omitted entirely on serialize; `Default::default()`, or `default =
//! "path"` alongside it, on deserialize, with no lookup against the wire object at all), `flatten`
//! (struct fields only — on serialize, splices the field's own object entries straight into the
//! parent object instead of nesting under the field's wire name; on deserialize, collects every
//! entry NOT claimed by a sibling field into that field's own `FromValue::from_value`). Combining
//! `flatten` with `deny_unknown_fields` on the same struct is a `compile_error!`, matching serde's
//! own restriction — the two are inherently at odds, since a flattened field's whole point is to
//! absorb keys the container does not itself recognize.
//! A missing `Option<T>` field decodes as `None` without requiring `#[value(default)]`, matching
//! serde for both structs and named enum-variant payloads.
//!
//! `#[value(transparent)]` (container, struct-only): the struct must have exactly one field
//! (named or unnamed) — the whole struct forwards straight to/from that field's own
//! `ToValue`/`FromValue`, no object wrapper.
//!
//! A single-field TUPLE struct (`struct Foo(pub u32);`, no `#[value(...)]` at all) derives as
//! transparent AUTOMATICALLY, with no attribute needed — `ToValue` emits exactly what the inner
//! field's own `ToValue::to_value` emits, `FromValue` decodes the inner type and wraps it back in
//! `Self`. This is the newtype-wrapper idiom (`id_newtype!`-style `pub struct FooId(pub u32)`),
//! distinct from `#[value(transparent)]` on a NAMED-field struct: the tuple case needs no
//! attribute because a one-field tuple struct has no other sensible wire representation (there is
//! no field name to key an object under). A tuple struct with more than one field, or a unit
//! struct, still hits the `named-field structs … not tuple/unit structs` error below — only the
//! exactly-one-field tuple shape gets this transparent treatment.
//!
//! A generic struct/enum gets an AUTOMATIC `Param: ToValue` (resp. `FromValue`) bound synthesized
//! per own type parameter by default — mirrors `serde_derive`'s own auto-inference default, and
//! is correct for every generic type this derive has been applied to so far (each parameter is
//! always reached through a `ToValue::to_value`/`FromValue::from_value` field access). Override
//! with `#[value(bound = "P1: Trait1, P2: Trait2, …")]` (container) for the rare case a parameter
//! is unused (e.g. behind `PhantomData`, so the auto bound would be an unsatisfiable-in-practice
//! over-constraint) or needs a different bound shape — both the `ToValue` and `FromValue` impl
//! get the SAME literal predicates you write, so write one valid for both (e.g.
//! `"K: ToValue + FromValue"` if a field of type `K` needs both).
//!
//! `deny_unknown_fields` is enforced for `Data::Struct` (unknown keys in the decoded object become
//! a `ValueError`) AND for every `Data::Enum` representation, with "unknown field" scoped
//! differently per representation to match what serde itself would reject:
//! - **unit-only** (bare-string) enums: not applicable — the wire form is a single string matched
//!   exactly against the variant names, so there is no object and no extra-key slot to smuggle
//!   anything into; an unrecognized string is already a hard `"unknown variant"` error regardless
//!   of this attribute. Setting the attribute here is accepted and does nothing extra.
//! - **externally tagged** (no `tag`, mixed variants): the outer object is inherently exactly one
//!   key (`{"VariantName": payload}` — enforced unconditionally via an `entries.len() != 1` check,
//!   independent of this attribute), so the only enforcement `deny_unknown_fields` adds is on a
//!   NAMED-field variant's own payload keys (checked against that variant's known field names). A
//!   single-unnamed-field variant's payload is handed whole to that field type's own `FromValue` —
//!   its unknown-field policy is that type's business, not this container's.
//! - **adjacently tagged** (`tag` + `content`): checked at two independent levels — the outer
//!   object's keys must be a subset of `{tag, content}` (checked once, before the tag is even
//!   read, since it does not depend on which variant matched), and a NAMED-field variant's
//!   `content` object keys must be a subset of just that variant's own field names (the tag never
//!   appears inside `content`, only alongside it at the outer level). A single-unnamed-field
//!   variant's `content` payload is again that field type's own business.
//! - **internally tagged** (`tag` only, fields inline beside it): checked per matched variant,
//!   since the allowed key set depends on which variant the tag names — a unit variant only
//!   allows the bare `{tag}` key; a named-field variant allows `{tag} ∪ its own field names`. A
//!   single-unnamed-field variant hands the entries object to that field type's own `FromValue`
//!   with the tag key STRIPPED first (encode never puts it there either — see the payload-facing
//!   `Fields::Unnamed`/`None` arm in `expand_from_value` — so a payload type carrying its own
//!   `deny_unknown_fields` must not see it), and no further check is added here: the payload type
//!   decides its own policy for everything else.
//!
//! `rename_all_fields = "…"` (container, tagged/externally-tagged enums only): cases an enum
//! variant's OWN named fields independently of `rename_all`, which continues to case the variant
//! tags themselves — exactly serde's split between the two attributes. When only one of the pair
//! is given, that single case covers both tags and fields (serde's default too). Found live in
//! `📇️directory/🧬️schema` (`tag` cased one way, fields cased `camelCase` another).
//!
//! An enum variant's OWN named field (unlike a plain struct field) supports only `rename`,
//! `default`, `skip`, and `skip_serializing_if` — `skip` omits the field on serialize and always
//! falls back to `default`/`Default::default()` on deserialize (no wire lookup at all), and
//! `skip_serializing_if = "path"` omits the field on serialize when `path(&field)` is `true`,
//! exactly like their plain-struct-field counterparts. `flatten`/`with`/`serialize_with`/
//! `deserialize_with` on an enum variant's own named field remain Deliberately NOT supported (rare
//! in the survey — under 5 occurrences repo-wide) and are now a `compile_error!` naming the field
//! rather than a silent no-op — a crate needing one of these keeps it hand-written (`impl
//! ToValue`/`impl FromValue` directly) rather than deriving. Also Deliberately NOT supported: tuple
//! variants with more than one unnamed field.

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
    rename_all_fields: Option<String>,
    tag: Option<String>,
    content: Option<String>,
    default: bool,
    deny_unknown_fields: bool,
    transparent: bool,
    bound: Option<String>,
    crate_path: Option<String>,
}

impl ContainerAttrs {
    /// 🐫 The case an enum variant's OWN named fields wire under: `rename_all_fields` when set
    /// (independent of variant-tag casing), else `rename_all` (serde's default — the same case
    /// covers both variant tags and their fields when only one attribute is given).
    fn field_rename_all(&self) -> Option<String> {
        self.rename_all_fields.clone().or_else(|| self.rename_all.clone())
    }
}

#[derive(Default, Clone)]
struct FieldAttrs {
    rename: Option<String>,
    default: FieldDefault,
    skip_serializing_if: Option<String>,
    serialize_with: Option<String>,
    deserialize_with: Option<String>,
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

    /// 🩹 `with = "path"` shorthand resolved for the deserialize direction — sibling of
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
            "content" => out.content = value,
            "default" => out.default = true,
            "deny_unknown_fields" => out.deny_unknown_fields = true,
            "transparent" => out.transparent = true,
            "bound" => out.bound = value,
            "rename_all_fields" => out.rename_all_fields = value,
            "crate" => out.crate_path = value,
            other => return Err(syn::Error::new_spanned(&attrs[0], format!("#[value(...)] does not support container attribute `{other}`"))),
        }
    }
    Ok(out)
}

/// 🧭️ Resolves `#[value(crate = "path::to::value_root")]` to the crate-path prefix every emitted
/// call site interpolates as `#value_crate::Type` — defaults to `::semio_framework_os_kernel` when
/// absent, so a container with no `crate` attribute emits byte-identical code to before this
/// attribute existed. Lets a sub-kernel crate (e.g. `semio-framework-actor`, which cannot depend on
/// `semio-framework-os-kernel` without a Cargo cycle) reexport `DslValue`/`ToValue`/`FromValue`/
/// `ValueError` from wherever it actually gets them and point the derive there instead.
fn container_crate_path(container: &ContainerAttrs) -> syn::Path {
    let path = container.crate_path.as_deref().unwrap_or("::semio_framework_os_kernel");
    syn::parse_str(path).expect("valid #[value(crate = \"...\")] path")
}

fn parse_field_attrs(attrs: &[syn::Attribute]) -> syn::Result<FieldAttrs> {
    let mut out = FieldAttrs::default();
    for (key, value) in parse_value_meta(attrs)? {
        match key.as_str() {
            "rename" => out.rename = value,
            "default" => out.default = value.map_or(FieldDefault::Bare, FieldDefault::Path),
            "skip_serializing_if" => out.skip_serializing_if = value,
            "serialize_with" => out.serialize_with = value,
            "deserialize_with" => out.deserialize_with = value,
            "with" => out.with = value,
            "flatten" => out.flatten = true,
            "skip" => out.skip = true,
            other => return Err(syn::Error::new_spanned(&attrs[0], format!("#[value(...)] does not support field attribute `{other}`"))),
        }
    }
    Ok(out)
}

/// 🧬️ Clones `generics` and adds the `where` bound this impl needs for each of its OWN type
/// parameters: by default, one `Param: #trait_path` predicate per type parameter (mirrors
/// `serde_derive`'s own auto-inference default — every generic struct/enum this derive has seen
/// so far needs exactly this, an owned field access through `ToValue::to_value`/
/// `FromValue::from_value` on that parameter). `#[value(bound = "P1: Trait1, P2: Trait2, …")]`
/// overrides this entirely (both impls get the SAME literal predicates you write — see the module
/// docs' `bound` entry) for the rare case a parameter is unused (e.g. behind `PhantomData`) or
/// needs a different bound shape than the uniform default.
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

fn type_is_option(ty: &syn::Type) -> bool {
    let syn::Type::Path(path) = ty else { return false };
    path.qself.is_none() && path.path.segments.last().is_some_and(|segment| segment.ident == "Option")
}

fn named_fields(fields: &Fields, container: &ContainerAttrs) -> syn::Result<Vec<NamedField>> {
    let syn::Fields::Named(named) = fields else {
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
    // 🛡️ Serde itself rejects `flatten` alongside `deny_unknown_fields` on the same struct — a
    // flattened field's whole job is to absorb keys the container does not itself recognize, which
    // is the exact opposite of an unknown-key check — so this derive rejects the same combination
    // up front instead of silently picking one behavior over the other.
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
                return Err(#value_crate::ValueError::new(format!("unknown field `{}`", __key)));
            }
        }
    }
}

fn from_value_struct_fields(fields: &[NamedField], container: &ContainerAttrs, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    // 🌾 The wire keys a `flatten` field is entitled to absorb are everything NOT claimed by a
    // sibling — so the deny-check's own allow-list (when no field flattens) and each flatten
    // field's own "remaining entries" filter both key off this same non-flatten name list.
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
        let missing = match (&field.attrs.default, container.default) {
            (FieldDefault::Path(path), _) => {
                let path: syn::Path = syn::parse_str(path).expect("valid default path");
                quote! { #path() }
            }
            (FieldDefault::Bare, _) | (FieldDefault::None, true) => quote! { ::std::default::Default::default() },
            (FieldDefault::None, false) if field.is_option => quote! { ::std::default::Default::default() },
            (FieldDefault::None, false) => quote! {
                return Err(#value_crate::ValueError::new(format!("missing field `{}`", #wire_name)))
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

//#region 🔖️VariantFields
/// 🎯 Rejects `flatten` on an enum variant's own named field with a `compile_error!` naming the
/// field, instead of the previous silent drop — module docs call `flatten` "Deliberately NOT
/// supported" on a variant's named fields (splicing into an already-tagged object is ambiguous),
/// but nothing enforced that until now. `rename`, `default`, `skip`, `skip_serializing_if`,
/// `serialize_with`/`deserialize_with`/`with` ARE supported on a variant's own named field (see
/// `variant_field_to_value_push`/`variant_field_from_value_read` below) — `default` and the
/// `*_with` trio already worked in practice (🏪️store's `ArtifactActorMsg::LocalMutations`/
/// `ArtifactEvent::RemoteMutations`/`ArtifactMutationsSaved.envelope` all rely on
/// `serialize_with`/`deserialize_with` on an internally-tagged variant's own field to route
/// `MutationEnvelope` through its hand-written bridge), `skip`/`skip_serializing_if` did not (both
/// fixed below — same silent-drop bug class).
fn check_variant_field_attrs_supported(field: &syn::Field, attrs: &FieldAttrs) -> syn::Result<()> {
    if attrs.flatten {
        return Err(syn::Error::new_spanned(
            field,
            format!("#[value(...)] does not support `flatten` on enum variant field `{}` (only plain struct fields support it)", field.ident.as_ref().expect("named field")),
        ));
    }
    Ok(())
}

/// 🎯 Emits one named enum-variant field's `ToValue` push into the accumulator `push_into`
/// (`content_entries` for externally/adjacently-tagged, `__out_entries` for internally-tagged),
/// honoring `skip` (omit unconditionally), `skip_serializing_if` (omit conditionally), and
/// `serialize_with`/`with` (replaces the default `ToValue::to_value` call) — mirrors
/// `to_value_object_entries`'s struct-field handling of the same attributes. Fixes the silent
/// wire-shape bug where `skip`/`skip_serializing_if` were parsed off an enum variant field and then
/// never consulted, so the field was always emitted via the default `ToValue::to_value` regardless
/// of a `serialize_with` naming a different one.
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

/// 🎯 Emits one named enum-variant field's `FromValue` read out of `entries_ident` (a
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
    let missing = match &field_attrs.default {
        FieldDefault::Path(path) => {
            let path: syn::Path = syn::parse_str(path).expect("valid default path");
            quote! { #path() }
        }
        FieldDefault::Bare => quote! { ::std::default::Default::default() },
        FieldDefault::None if type_is_option(&field.ty) => quote! { ::std::default::Default::default() },
        FieldDefault::None => quote! {
            return Err(#value_crate::ValueError::new(format!("missing field `{}`", #wire_name)))
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

//#region 🔖️Expand
pub fn expand_to_value(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let container = parse_container_attrs(&input.attrs)?;
    let value_crate = container_crate_path(&container);
    let generics = generics_with_bound(&input.generics, &container.bound, &quote! { #value_crate::ToValue });
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(data) if container.transparent => match &data.fields {
            Fields::Named(named) if named.named.len() == 1 => {
                let ident = named.named.first().expect("checked len == 1").ident.clone().expect("named field");
                quote! { #value_crate::ToValue::to_value(&self.#ident) }
            }
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => quote! { #value_crate::ToValue::to_value(&self.0) },
            other => return Err(syn::Error::new_spanned(other, "#[value(transparent)] requires exactly one field")),
        },
        // 🆔 Single-field tuple struct (`struct Foo(pub u32);`): automatic transparent newtype —
        // see the module docs' `#[value(transparent)]` entry for why this needs no attribute.
        Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => {
            quote! { #value_crate::ToValue::to_value(&self.0) }
        }
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            let entries = to_value_object_entries(&fields, &value_crate);
            quote! {
                #entries
                #value_crate::DslValue::Object(entries)
            }
        }
        Data::Enum(data) if container.tag.is_none() && data.variants.iter().all(|variant| matches!(variant.fields, Fields::Unit)) => {
            let arms = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_field_attrs(&variant.attrs).unwrap_or_default();
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                quote! { Self::#variant_ident => #value_crate::DslValue::String(#wire_variant.to_string()) }
            });
            quote! {
                match *self { #(#arms),* }
            }
        }
        Data::Enum(data) if container.tag.is_none() => {
            // 🏷️ Externally-tagged (serde's own default enum representation when no `#[serde(tag
            // = …)]` is present): a unit variant is still the bare wire-name string, a
            // single-unnamed-field or named-field variant becomes a one-key object
            // `{"VariantName": <payload>}`.
            let arms = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_field_attrs(&variant.attrs).unwrap_or_default();
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                let arm: syn::Result<proc_macro2::TokenStream> = match &variant.fields {
                    Fields::Unit => Ok(quote! {
                        Self::#variant_ident => #value_crate::DslValue::String(#wire_variant.to_string())
                    }),
                    Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => Ok(quote! {
                        Self::#variant_ident(payload) => #value_crate::DslValue::object([
                            (#wire_variant.to_string(), #value_crate::ToValue::to_value(payload)),
                        ])
                    }),
                    Fields::Named(named) => {
                        let push_into = quote! { content_entries };
                        let pushes = named.named.iter().map(|field| {
                            let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                            let ident = field.ident.clone().expect("named field");
                            let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all());
                            variant_field_to_value_push(field, &field_attrs, &wire_name, &ident, &push_into, &value_crate)
                        }).collect::<syn::Result<Vec<_>>>()?;
                        let idents = variant_destructure_patterns(named);
                        Ok(quote! {
                            Self::#variant_ident { #(#idents),* } => {
                                let mut content_entries: Vec<(String, #value_crate::DslValue)> = Vec::new();
                                #(#pushes)*
                                #value_crate::DslValue::object([
                                    (#wire_variant.to_string(), #value_crate::DslValue::Object(content_entries)),
                                ])
                            }
                        })
                    }
                    other => Err(syn::Error::new_spanned(other, "#[derive(ToValue)] externally-tagged enum variants must be unit, a single unnamed payload, or named fields")),
                };
                arm
            }).collect::<syn::Result<Vec<_>>>()?;
            quote! {
                match self { #(#arms),* }
            }
        }
        Data::Enum(data) => {
            let Some(tag) = &container.tag else {
                unreachable!("the tag.is_none() arm above already handles every tag-less enum");
            };
            let arms = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_field_attrs(&variant.attrs).unwrap_or_default();
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                let arm: syn::Result<proc_macro2::TokenStream> = match (&variant.fields, &container.content) {
                    (Fields::Unit, _) => Ok(quote! {
                        Self::#variant_ident => #value_crate::DslValue::object([(#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string()))])
                    }),
                    (Fields::Unnamed(unnamed), Some(content)) if unnamed.unnamed.len() == 1 => Ok(quote! {
                        Self::#variant_ident(payload) => #value_crate::DslValue::object([
                            (#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string())),
                            (#content.to_string(), #value_crate::ToValue::to_value(payload)),
                        ])
                    }),
                    (Fields::Unnamed(unnamed), None) if unnamed.unnamed.len() == 1 => Ok(quote! {
                        Self::#variant_ident(payload) => {
                            let mut entries = match #value_crate::ToValue::to_value(payload) {
                                #value_crate::DslValue::Object(entries) => entries,
                                other => vec![("value".to_string(), other)],
                            };
                            entries.insert(0, (#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string())));
                            #value_crate::DslValue::Object(entries)
                        }
                    }),
                    (Fields::Named(named), Some(content)) => {
                        let push_into = quote! { content_entries };
                        let pushes = named.named.iter().map(|field| {
                            let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                            let ident = field.ident.clone().expect("named field");
                            let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all());
                            variant_field_to_value_push(field, &field_attrs, &wire_name, &ident, &push_into, &value_crate)
                        }).collect::<syn::Result<Vec<_>>>()?;
                        let idents = variant_destructure_patterns(named);
                        Ok(quote! {
                            Self::#variant_ident { #(#idents),* } => {
                                let mut content_entries: Vec<(String, #value_crate::DslValue)> = Vec::new();
                                #(#pushes)*
                                #value_crate::DslValue::object([
                                    (#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string())),
                                    (#content.to_string(), #value_crate::DslValue::Object(content_entries)),
                                ])
                            }
                        })
                    }
                    (Fields::Named(named), None) => {
                        // 🛡️ `__out_entries`, not `entries` — a user field literally named `entries`
                        // (e.g. `SemioValue::Map { entries: Vec<SemioValueEntry> }`) would otherwise
                        // shadow the accumulator once `#(#idents),*` destructures it into scope, making
                        // `ToValue::to_value(#ident)` resolve to the accumulator itself (an owned
                        // `Vec<(String, DslValue)>`) instead of the field's `&Vec<SemioValueEntry>`.
                        let push_into = quote! { __out_entries };
                        let pushes = named.named.iter().map(|field| {
                            let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                            let ident = field.ident.clone().expect("named field");
                            let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all());
                            variant_field_to_value_push(field, &field_attrs, &wire_name, &ident, &push_into, &value_crate)
                        }).collect::<syn::Result<Vec<_>>>()?;
                        let idents = variant_destructure_patterns(named);
                        Ok(quote! {
                            Self::#variant_ident { #(#idents),* } => {
                                let mut __out_entries: Vec<(String, #value_crate::DslValue)> = vec![(#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string()))];
                                #(#pushes)*
                                #value_crate::DslValue::Object(__out_entries)
                            }
                        })
                    }
                    (other, _) => Err(syn::Error::new_spanned(other, "#[derive(ToValue)] enum variants must be unit, a single unnamed payload, or named fields")),
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
        impl #impl_generics #value_crate::ToValue for #name #ty_generics #where_clause {
            fn to_value(&self) -> #value_crate::DslValue {
                #body
            }
        }
    })
}

pub fn expand_from_value(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let container = parse_container_attrs(&input.attrs)?;
    let value_crate = container_crate_path(&container);
    let generics = generics_with_bound(&input.generics, &container.bound, &quote! { #value_crate::FromValue });
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(data) if container.transparent => match &data.fields {
            Fields::Named(named) if named.named.len() == 1 => {
                let ident = named.named.first().expect("checked len == 1").ident.clone().expect("named field");
                quote! { Ok(Self { #ident: #value_crate::FromValue::from_value(value)? }) }
            }
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => quote! { Ok(Self(#value_crate::FromValue::from_value(value)?)) },
            other => return Err(syn::Error::new_spanned(other, "#[value(transparent)] requires exactly one field")),
        },
        // 🆔 Single-field tuple struct: sibling of `expand_to_value`'s identical guard above.
        Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => {
            quote! { Ok(Self(#value_crate::FromValue::from_value(value)?)) }
        }
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            let reads = from_value_struct_fields(&fields, &container, &value_crate);
            quote! {
                let __entries = #value_crate::DslValue::into_object(value)?;
                #reads
            }
        }
        Data::Enum(data) if container.tag.is_none() && data.variants.iter().all(|variant| matches!(variant.fields, Fields::Unit)) => {
            let arms = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_field_attrs(&variant.attrs).unwrap_or_default();
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                quote! { #wire_variant => Self::#variant_ident, }
            });
            quote! {
                let __s = match value { #value_crate::DslValue::String(s) => s, other => return Err(#value_crate::ValueError::new(format!("expected a string, found {other:?}"))) };
                Ok(match __s.as_str() {
                    #(#arms)*
                    other => return Err(#value_crate::ValueError::new(format!("unknown variant `{other}`"))),
                })
            }
        }
        Data::Enum(data) if container.tag.is_none() => {
            // 🏷️ Externally-tagged (serde's own default enum representation when no `#[serde(tag
            // = …)]` is present) — mirrors `expand_to_value`'s sibling arm above.
            let string_arms = data.variants.iter().filter(|variant| matches!(variant.fields, Fields::Unit)).map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_field_attrs(&variant.attrs).unwrap_or_default();
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                quote! { #wire_variant => return Ok(Self::#variant_ident), }
            });
            let object_arms = data.variants.iter().filter(|variant| !matches!(variant.fields, Fields::Unit)).map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_field_attrs(&variant.attrs).unwrap_or_default();
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                let arm: syn::Result<proc_macro2::TokenStream> = match &variant.fields {
                    Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                        let payload_ty = &unnamed.unnamed[0].ty;
                        Ok(quote! {
                            #wire_variant => Self::#variant_ident(<#payload_ty as #value_crate::FromValue>::from_value(__payload)?),
                        })
                    }
                    Fields::Named(named) => {
                        let field_wire_names: Vec<String> = named.named.iter().map(|field| {
                            let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                            let ident = field.ident.clone().expect("named field");
                            field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all())
                        }).collect();
                        let deny_check = if container.deny_unknown_fields {
                            deny_unknown_keys(&quote! { __variant_entries }, &field_wire_names, &value_crate)
                        } else {
                            quote! {}
                        };
                        let entries_ident = quote! { __variant_entries };
                        let reads = named.named.iter().map(|field| {
                            let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                            let ident = field.ident.clone().expect("named field");
                            let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all());
                            variant_field_from_value_read(field, &field_attrs, &wire_name, &ident, &entries_ident, &value_crate)
                        }).collect::<syn::Result<Vec<_>>>()?;
                        let idents = named.named.iter().map(|field| field.ident.clone().expect("named field"));
                        Ok(quote! {
                            #wire_variant => {
                                let __variant_entries = #value_crate::DslValue::into_object(__payload)?;
                                #deny_check
                                #(#reads)*
                                Self::#variant_ident { #(#idents),* }
                            },
                        })
                    }
                    other => Err(syn::Error::new_spanned(other, "#[derive(FromValue)] externally-tagged enum variants must be unit, a single unnamed payload, or named fields")),
                };
                arm
            }).collect::<syn::Result<Vec<_>>>()?;
            quote! {
                if let #value_crate::DslValue::String(__s) = &value {
                    match __s.as_str() {
                        #(#string_arms)*
                        _ => {}
                    }
                }
                let __entries = #value_crate::DslValue::into_object(value)?;
                if __entries.len() != 1 {
                    return Err(#value_crate::ValueError::new(format!("expected an externally-tagged enum object with exactly one key, found {} keys", __entries.len())));
                }
                let (__key, __payload) = __entries.into_iter().next().expect("checked len == 1 above");
                Ok(match __key.as_str() {
                    #(#object_arms)*
                    other => return Err(#value_crate::ValueError::new(format!("unknown variant `{other}`"))),
                })
            }
        }
        Data::Enum(data) => {
            let Some(tag) = &container.tag else {
                unreachable!("the tag.is_none() arm above already handles every tag-less enum");
            };
            let arms = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_field_attrs(&variant.attrs).unwrap_or_default();
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                let arm: syn::Result<proc_macro2::TokenStream> = match (&variant.fields, &container.content) {
                    (Fields::Unit, Some(_)) => Ok(quote! {
                        #wire_variant => Self::#variant_ident,
                    }),
                    (Fields::Unit, None) => {
                        // 🛡️ Internally-tagged unit variant: the whole entries object is nothing
                        // but the tag, so `deny_unknown_fields` allows exactly `{tag}`.
                        let deny_check = if container.deny_unknown_fields {
                            deny_unknown_keys(&quote! { __entries }, &[tag.clone()], &value_crate)
                        } else {
                            quote! {}
                        };
                        Ok(quote! {
                            #wire_variant => { #deny_check Self::#variant_ident },
                        })
                    }
                    (Fields::Unnamed(unnamed), Some(_)) if unnamed.unnamed.len() == 1 => {
                        let payload_ty = &unnamed.unnamed[0].ty;
                        Ok(quote! {
                            #wire_variant => Self::#variant_ident(<#payload_ty as #value_crate::FromValue>::from_value(__content()?)?),
                        })
                    }
                    (Fields::Unnamed(unnamed), None) if unnamed.unnamed.len() == 1 => {
                        // 🩹 Strip the tag key before handing the object to the payload type's
                        // own `FromValue` — `expand_to_value`'s sibling arm never puts the tag
                        // INTO the payload's own entries (it prepends the tag after taking the
                        // payload's `to_value()`, so the payload never emits it either), so
                        // leaving the tag in here was a decode/encode asymmetry: a payload type
                        // that itself carries `#[value(deny_unknown_fields)]` would reject its
                        // own valid wire form because the wrapper's tag key looked unknown to it.
                        let payload_ty = &unnamed.unnamed[0].ty;
                        Ok(quote! {
                            #wire_variant => Self::#variant_ident(<#payload_ty as #value_crate::FromValue>::from_value(#value_crate::DslValue::Object(__entries.iter().filter(|(__k, _)| __k != #tag).cloned().collect()))?),
                        })
                    }
                    (Fields::Named(named), content_key) => {
                        let source = if content_key.is_some() { quote! { __content()?.into_object()? } } else { quote! { __entries.clone() } };
                        let field_wire_names: Vec<String> = named.named.iter().map(|field| {
                            let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                            let ident = field.ident.clone().expect("named field");
                            field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all())
                        }).collect();
                        let deny_check = if container.deny_unknown_fields {
                            let allowed: Vec<String> = if content_key.is_some() {
                                field_wire_names.clone()
                            } else {
                                let mut allowed = vec![tag.clone()];
                                allowed.extend(field_wire_names.clone());
                                allowed
                            };
                            deny_unknown_keys(&quote! { __variant_entries }, &allowed, &value_crate)
                        } else {
                            quote! {}
                        };
                        let entries_ident = quote! { __variant_entries };
                        let reads = named.named.iter().map(|field| {
                            let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                            let ident = field.ident.clone().expect("named field");
                            let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all());
                            variant_field_from_value_read(field, &field_attrs, &wire_name, &ident, &entries_ident, &value_crate)
                        }).collect::<syn::Result<Vec<_>>>()?;
                        let idents = named.named.iter().map(|field| field.ident.clone().expect("named field"));
                        Ok(quote! {
                            #wire_variant => {
                                let __variant_entries = #source;
                                #deny_check
                                #(#reads)*
                                Self::#variant_ident { #(#idents),* }
                            },
                        })
                    }
                    (other, _) => Err(syn::Error::new_spanned(other, "#[derive(FromValue)] enum variants must be unit, a single unnamed payload, or named fields")),
                };
                arm
            }).collect::<syn::Result<Vec<_>>>()?;
            let content_helper = match &container.content {
                Some(content) => quote! {
                    let __content = || -> ::core::result::Result<#value_crate::DslValue, #value_crate::ValueError> {
                        __entries.iter().find(|(k, _)| k == #content).map(|(_, v)| v.clone()).ok_or_else(|| #value_crate::ValueError::new(format!("missing content field `{}`", #content)))
                    };
                },
                None => quote! {},
            };
            // 🛡️ Adjacently-tagged outer-level `deny_unknown_fields`: the allowed key set here is
            // just `{tag, content}` regardless of which variant matches, so this check runs once,
            // before the tag is even read — unlike the internally-tagged case (no `content`),
            // where the allowed set depends on the variant and is checked per-arm above instead.
            let outer_deny_check = match (&container.content, container.deny_unknown_fields) {
                (Some(content), true) => deny_unknown_keys(&quote! { __entries }, &[tag.clone(), content.clone()], &value_crate),
                _ => quote! {},
            };
            quote! {
                let __entries = #value_crate::DslValue::into_object(value)?;
                #outer_deny_check
                let __tag = __entries.iter().find(|(k, _)| k == #tag).map(|(_, v)| v.clone()).ok_or_else(|| #value_crate::ValueError::new(format!("missing tag field `{}`", #tag)))?;
                let __tag = match __tag { #value_crate::DslValue::String(s) => s, other => return Err(#value_crate::ValueError::new(format!("expected a string tag, found {other:?}"))) };
                #content_helper
                Ok(match __tag.as_str() {
                    #(#arms)*
                    other => return Err(#value_crate::ValueError::new(format!("unknown `{}` variant `{other}`", #tag))),
                })
            }
        }
        Data::Union(_) => return Err(syn::Error::new_spanned(&input.ident, "#[derive(FromValue)] does not support unions")),
    };

    Ok(quote! {
        impl #impl_generics #value_crate::FromValue for #name #ty_generics #where_clause {
            fn from_value(value: #value_crate::DslValue) -> ::core::result::Result<Self, #value_crate::ValueError> {
                #body
            }
        }
    })
}
//#endregion 🔖️Expand
