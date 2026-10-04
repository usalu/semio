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
//! `::semio_framework_value` when absent — a container with no `crate` attribute emits
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
//! Supported field attributes: `rename = "…"`, `required` (a present wire key is mandatory even
//! for `Option<T>`), `default` (bare), `default = "path"`,
//! `skip_serializing_if = "path"`, `serialize_with = "path"` (`fn(&FieldType) ->
//! DslValue`, replaces the `ToValue::to_value` call for that field), `deserialize_with = "path"`
//! (`fn(DslValue) -> Result<FieldType, ValueError>`, replaces the `FromValue::from_value` call —
//! combine with bare `default` for a "missing key defaults, present key goes through the custom
//! fn" split, the `deserialize_double_option` shape), `with = "path"` (shorthand for
//! `serialize_with = "path::to_value"` + `deserialize_with = "path::from_value"`; an explicit
//! `serialize_with`/`deserialize_with` given alongside `with` wins for that one direction).
//! `serialize_controlled_with` supplies the explicit borrowed serializer under cumulative
//! `NativeEncodeControl`; ordinary custom serializers are refused by controlled output.
//! `skip`
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
//! An internally tagged single-unnamed-field variant whose payload does NOT encode to an object
//! (a `String`, a number, an array — serde refuses this shape outright, this derive does not) is
//! carried as the single entry `{"value": <payload>}` beside the tag. Decoding is the inverse of
//! that runtime branch, in that order: the tag-stripped object is offered to the payload's
//! `FromValue` first, and only when that fails is a lone `value` entry unwrapped and offered
//! bare. The order matters — a payload type whose only field is itself named `value` produces an
//! indistinguishable key set, and object-first decodes both correctly.
//!
//! `rename_all_fields = "…"` on an enum supplies its named variant fields' default casing.
//! `rename_all = "…"` on a variant overrides that default for its own fields; an explicit field
//! `rename` takes precedence over both. Container `rename_all` controls variant tags only, so an
//! enum declaring only that attribute wires its named variant fields under their Rust identifiers
//! verbatim. These scopes match Serde's container, variant and field attributes.
//!
//! An enum variant's OWN named field (unlike a plain struct field) supports only `rename`,
//! `required`, `default`, `skip`, and `skip_serializing_if` — `skip` omits the field on serialize and always
//! falls back to `default`/`Default::default()` on deserialize (no wire lookup at all), and
//! `skip_serializing_if = "path"` omits the field on serialize when `path(&field)` is `true`,
//! exactly like their plain-struct-field counterparts. `flatten`/`with`/`serialize_with`/
//! `deserialize_with` on an enum variant's own named field remain Deliberately NOT supported (rare
//! in the survey — under 5 occurrences repo-wide) and are now a `compile_error!` naming the field
//! rather than a silent no-op — a crate needing one of these keeps it hand-written (`impl
//! ToValue`/`impl FromValue` directly) rather than deriving. Also Deliberately NOT supported: tuple
//! variants with more than one unnamed field.

use quote::{quote, format_ident};
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
    retire_with: Option<String>,
    default_controlled: Option<String>,
}

impl ContainerAttrs {
    /// 🐫 Variant `rename_all` overrides container `rename_all_fields`; an explicit field rename
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
            "bound" => out.bound = value,
            "rename_all_fields" => out.rename_all_fields = value,
            "crate" => out.crate_path = value,
            "retire_with" => out.retire_with = value,
            "default_controlled" => out.default_controlled = value,
            other => return Err(syn::Error::new_spanned(&attrs[0], format!("#[value(...)] does not support container attribute `{other}`"))),
        }
    }
    Ok(out)
}

/// 🧭️ Resolves `#[value(crate = "path::to::value_root")]` to the crate-path prefix every emitted
/// call site interpolates as `#value_crate::Type` — defaults to `::semio_framework_value` when
/// absent, so a container with no `crate` attribute emits byte-identical code to before this
/// attribute existed. Lets a sub-kernel crate (e.g. `semio-framework-actor`, which cannot depend on
/// `semio-framework-os-kernel` without a Cargo cycle) reexport `DslValue`/`ToValue`/`FromValue`/
/// `ValueError` from wherever it actually gets them and point the derive there instead.
fn container_crate_path(container: &ContainerAttrs) -> syn::Path {
    let path = container.crate_path.as_deref().unwrap_or("::semio_framework_value");
    syn::parse_str(path).expect("valid #[value(crate = \"...\")] path")
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
                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("unknown field `{}`", __key)));
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

//#region 🔖️VariantFields
/// 🎯 Rejects `flatten` on an enum variant's own named field with a `compile_error!` naming the
/// field, instead of the previous silent drop — module docs call `flatten` "Deliberately NOT
/// supported" on a variant's named fields (splicing into an already-tagged object is ambiguous),
/// but nothing enforced that until now. `rename`, `required`, `default`, `skip`, `skip_serializing_if`,
/// `serialize_with`/`deserialize_with`/`with` ARE supported on a variant's own named field (see
/// `variant_field_to_value_push`/`variant_field_from_value_read` below) — `default` and the
/// `*_with` trio already worked in practice (🏪️store's `ArtifactActorMsg::LocalMutations`/
/// `ArtifactEvent::RemoteMutations`/`ArtifactMutationsSaved.envelope` all rely on
/// `serialize_with`/`deserialize_with` on an internally-tagged variant's own field to route
/// `MutationEnvelope` through its hand-written bridge), `skip`/`skip_serializing_if` did not (both
/// fixed below — same silent-drop bug class).
fn check_variant_field_attrs_supported(field: &syn::Field, attrs: &FieldAttrs) -> syn::Result<()> {
    if attrs.flatten {
        return Err(syn::Error::new_spanned(field, format!("#[value(...)] does not support `flatten` on enum variant field `{}` (only plain struct fields support it)", field.ident.as_ref().expect("named field"))));
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

//#region 🧭️TypedPath
fn field_to_path_call(
    field: &NamedField,
    access: &proc_macro2::TokenStream,
    path: &proc_macro2::TokenStream,
    method: &syn::Ident,
    value_crate: &syn::Path,
) -> proc_macro2::TokenStream {
    match field.attrs.effective_serialize_with() {
        Some(serializer) => {
            let serializer: syn::Path = syn::parse_str(&serializer).expect("valid serialize_with path");
            quote! { #value_crate::ToValue::#method(&#serializer(#access), #path) }
        }
        None => quote! { #value_crate::ToValue::#method(#access, #path) },
    }
}

fn struct_to_path_body(fields: &[NamedField], value_crate: &syn::Path, method: &str) -> proc_macro2::TokenStream {
    let method = syn::Ident::new(method, proc_macro2::Span::call_site());
    let direct_arms = fields.iter().filter(|field| !field.attrs.skip && !field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_to_path_call(field, &quote! { &self.#ident }, &quote! { __rest }, &method, value_crate);
        let omitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! {
                if #predicate(&self.#ident) {
                    return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                }
            }
        });
        quote! {
            #wire_name => {
                #omitted
                #call.map_err(|error| error.under(__segment))
            }
        }
    });
    let flatten_attempts = fields.iter().filter(|field| !field.attrs.skip && field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let call = field_to_path_call(field, &quote! { &self.#ident }, &quote! { path }, &method, value_crate);
        quote! {
            if let Ok(value) = #call {
                return Ok(value);
            }
        }
    });
    let root = if method == "value_shape_at_path" {
        let normal_counts = fields.iter().filter(|field| !field.attrs.skip && !field.attrs.flatten).map(|field| {
            let ident = &field.ident;
            match &field.attrs.skip_serializing_if {
                Some(predicate) => {
                    let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                    quote! { if !#predicate(&self.#ident) { __len += 1; } }
                }
                None => quote! { __len += 1; },
            }
        });
        let flattened_counts = fields.iter().filter(|field| !field.attrs.skip && field.attrs.flatten).map(|field| {
            let ident = &field.ident;
            let call = field_to_path_call(field, &quote! { &self.#ident }, &quote! { &[] }, &method, value_crate);
            quote! {
                match #call? {
                    #value_crate::ValueShape::Object { len } => __len = __len.checked_add(len).ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::OwnershipLimit, "object length overflow"))?,
                    _ => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "flattened field is not an object")),
                }
            }
        });
        quote! {
            if path.is_empty() {
                let mut __len = 0usize;
                #(#normal_counts)*
                #(#flattened_counts)*
                return Ok(#value_crate::ValueShape::Object { len: __len });
            }
        }
    } else {
        quote! {
            if path.is_empty() {
                return Ok(#value_crate::ToValue::to_value(self));
            }
        }
    };
    quote! {
        #root
        let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
        match *__segment {
            #(#direct_arms,)*
            _ => {
                #(#flatten_attempts)*
                Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)))
            }
        }
    }
}

fn field_to_key_call(field: &NamedField, access: &proc_macro2::TokenStream, path: &proc_macro2::TokenStream, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    match field.attrs.effective_serialize_with() {
        Some(serializer) => {
            let serializer: syn::Path = syn::parse_str(&serializer).expect("valid serialize_with path");
            quote! { #value_crate::ToValue::value_key_at_path(&#serializer(#access), #path, index) }
        }
        None => quote! { #value_crate::ToValue::value_key_at_path(#access, #path, index) },
    }
}

fn struct_key_at_path_body(fields: &[NamedField], value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let root_steps = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let admitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! { !#predicate(&self.#ident) }
        }).unwrap_or_else(|| quote! { true });
        if field.attrs.flatten {
            let shape = field_to_path_call(field, &quote! { &self.#ident }, &quote! { &[] }, &syn::Ident::new("value_shape_at_path", proc_macro2::Span::call_site()), value_crate);
            let key = field_to_key_call(field, &quote! { &self.#ident }, &quote! { &[] }, value_crate);
            quote! {
                if #admitted {
                    let #value_crate::ValueShape::Object { len } = #shape? else {
                        return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "flattened field is not an object"));
                    };
                    if index < __offset + len {
                        let index = index - __offset;
                        return #key;
                    }
                    __offset += len;
                }
            }
        } else {
            quote! {
                if #admitted {
                    if index == __offset { return Ok(#wire_name.to_owned()); }
                    __offset += 1;
                }
            }
        }
    });
    let direct_arms = fields.iter().filter(|field| !field.attrs.skip && !field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_to_key_call(field, &quote! { &self.#ident }, &quote! { __rest }, value_crate);
        let omitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! {
                if #predicate(&self.#ident) {
                    return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                }
            }
        });
        quote! {
            #wire_name => {
                #omitted
                #call.map_err(|error| error.under(__segment))
            }
        }
    });
    let flatten_attempts = fields.iter().filter(|field| !field.attrs.skip && field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let call = field_to_key_call(field, &quote! { &self.#ident }, &quote! { path }, value_crate);
        quote! { if let Ok(key) = #call { return Ok(key); } }
    });
    quote! {
        if path.is_empty() {
            let mut __offset = 0usize;
            #(#root_steps)*
            return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {__offset}")));
        }
        let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
        match *__segment {
            #(#direct_arms,)*
            _ => {
                #(#flatten_attempts)*
                Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)))
            }
        }
    }
}

fn field_edit_call(
    field: &NamedField,
    access: &proc_macro2::TokenStream,
    path: &proc_macro2::TokenStream,
    edit: &proc_macro2::TokenStream,
    value_crate: &syn::Path,
) -> proc_macro2::TokenStream {
    match (field.attrs.effective_serialize_with(), field.attrs.effective_deserialize_with()) {
        (Some(serializer), Some(deserializer)) => {
            let serializer: syn::Path = syn::parse_str(&serializer).expect("valid serialize_with path");
            let deserializer: syn::Path = syn::parse_str(&deserializer).expect("valid deserialize_with path");
            quote! {
                {
                    let mut __field_value = #serializer(&*#access);
                    #value_crate::FromValue::edit_value_at_path(&mut __field_value, #path, #edit)?;
                    let __replacement = #deserializer(__field_value)?;
                    *#access = __replacement;
                    Ok::<(), #value_crate::ValueError>(())
                }
            }
        }
        (Some(_), None) => quote! {
            Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::UnsupportedOwner, "custom wire field has no matching decoder for a typed-path edit"))
        },
        (None, Some(deserializer)) => {
            let deserializer: syn::Path = syn::parse_str(&deserializer).expect("valid deserialize_with path");
            quote! {
                if #path.is_empty() {
                    match #edit {
                        #value_crate::ValueEdit::Set(value) => {
                            let __replacement = #deserializer(value)?;
                            *#access = __replacement;
                            Ok::<(), #value_crate::ValueError>(())
                        }
                        #value_crate::ValueEdit::Insert(_) | #value_crate::ValueEdit::InsertAt { .. } => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot insert a required field")),
                        #value_crate::ValueEdit::Remove => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot remove a required field")),
                    }
                } else {
                    Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::UnsupportedOwner, "custom wire field has no matching encoder for a nested typed-path edit"))
                }
            }
        }
        (None, None) => quote! { #value_crate::FromValue::edit_value_at_path(#access, #path, #edit) },
    }
}

fn struct_edit_path_body(fields: &[NamedField], value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let direct_arms = fields.iter().filter(|field| !field.attrs.skip && !field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_edit_call(field, &quote! { &mut self.#ident }, &quote! { __rest }, &quote! { edit }, value_crate);
        quote! { #wire_name => #call.map_err(|error| error.under(__segment)) }
    });
    let flatten_attempts = fields.iter().filter(|field| !field.attrs.skip && field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let call = field_edit_call(field, &quote! { &mut self.#ident }, &quote! { path }, &quote! { edit.clone() }, value_crate);
        quote! {
            if #call.is_ok() {
                return Ok(());
            }
        }
    });
    quote! {
        if path.is_empty() {
            return match edit {
                #value_crate::ValueEdit::Set(value) => {
                    let replacement = <Self as #value_crate::FromValue>::from_value(value)?;
                    *self = replacement;
                    Ok(())
                }
                #value_crate::ValueEdit::Insert(_) | #value_crate::ValueEdit::InsertAt { .. } => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot insert at the record root")),
                #value_crate::ValueEdit::Remove => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot remove the record root")),
            };
        }
        let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
        match *__segment {
            #(#direct_arms,)*
            _ => {
                #(#flatten_attempts)*
                Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)))
            }
        }
    }
}

fn variant_named_fields(named: &syn::FieldsNamed, container: &ContainerAttrs, variant: &VariantAttrs) -> syn::Result<Vec<NamedField>> {
    named
        .named
        .iter()
        .map(|field| {
            let attrs = parse_field_attrs(&field.attrs)?;
            check_variant_field_attrs_supported(field, &attrs)?;
            let ident = field.ident.clone().expect("named field");
            let wire_name = field_wire_name(&ident.to_string(), &attrs.rename, &container.field_rename_all(variant));
            Ok(NamedField { ident, wire_name, attrs, is_option: type_is_option(&field.ty) })
        })
        .collect()
}

fn named_variant_edit_dispatch(fields: &[NamedField], path: &proc_macro2::TokenStream, edit: &proc_macro2::TokenStream, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let arms = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_edit_call(field, &quote! { #ident }, &quote! { __rest }, edit, value_crate);
        quote! { #wire_name => #call.map_err(|error| error.under(__segment)) }
    });
    quote! {
        let (__segment, __rest) = #path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot structurally replace an enum variant payload"))?;
        match *__segment {
            #(#arms,)*
            _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment))),
        }
    }
}

fn named_variant_to_path_dispatch(fields: &[NamedField], path: &proc_macro2::TokenStream, method: &str, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
    let arms = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_to_path_call(field, &quote! { #ident }, &quote! { __rest }, &method_ident, value_crate);
        let omitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! {
                if #predicate(#ident) {
                    return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                }
            }
        });
        quote! {
            #wire_name => {
                #omitted
                #call.map_err(|error| error.under(__segment))
            }
        }
    });
    let root = if method == "value_shape_at_path" {
        let counts = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
            let ident = &field.ident;
            match &field.attrs.skip_serializing_if {
                Some(predicate) => {
                    let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                    quote! { if !#predicate(#ident) { __len += 1; } }
                }
                None => quote! { __len += 1; },
            }
        });
        quote! {
            if #path.is_empty() {
                let mut __len = 0usize;
                #(#counts)*
                return Ok(#value_crate::ValueShape::Object { len: __len });
            }
        }
    } else {
        let pushes = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
            let ident = &field.ident;
            let wire_name = &field.wire_name;
            let value = match field.attrs.effective_serialize_with() {
                Some(serializer) => {
                    let serializer: syn::Path = syn::parse_str(&serializer).expect("valid serialize_with path");
                    quote! { #serializer(#ident) }
                }
                None => quote! { #value_crate::ToValue::to_value(#ident) },
            };
            match &field.attrs.skip_serializing_if {
                Some(predicate) => {
                    let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                    quote! { if !#predicate(#ident) { __entries.push((#wire_name.to_owned(), #value)); } }
                }
                None => quote! { __entries.push((#wire_name.to_owned(), #value)); },
            }
        });
        quote! {
            if #path.is_empty() {
                let mut __entries = Vec::new();
                #(#pushes)*
                return Ok(#value_crate::DslValue::Object(__entries));
            }
        }
    };
    quote! {
        #root
        let (__segment, __rest) = #path.split_first().expect("non-empty variant path checked above");
        match *__segment {
            #(#arms,)*
            _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment))),
        }
    }
}

fn named_variant_key_dispatch(fields: &[NamedField], path: &proc_macro2::TokenStream, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let root_steps = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let admitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! { !#predicate(#ident) }
        }).unwrap_or_else(|| quote! { true });
        quote! {
            if #admitted {
                if index == __offset { return Ok(#wire_name.to_owned()); }
                __offset += 1;
            }
        }
    });
    let arms = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_to_key_call(field, &quote! { #ident }, &quote! { __rest }, value_crate);
        quote! { #wire_name => #call.map_err(|error| error.under(__segment)) }
    });
    quote! {
        if #path.is_empty() {
            let mut __offset = 0usize;
            #(#root_steps)*
            return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {__offset}")));
        }
        let (__segment, __rest) = #path.split_first().expect("non-empty path checked above");
        match *__segment {
            #(#arms,)*
            _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment))),
        }
    }
}

fn enum_key_at_path_body(data: &syn::DataEnum, container: &ContainerAttrs, value_crate: &syn::Path) -> syn::Result<proc_macro2::TokenStream> {
    if data.variants.is_empty() {
        return Ok(quote! { match *self {} });
    }
    let arms = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_attrs = parse_variant_attrs(&variant.attrs)?;
            let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
            match &variant.fields {
                Fields::Unit => {
                    let dispatch = if let Some(tag) = &container.tag {
                        quote! {
                            if path.is_empty() {
                                return if index == 0 { Ok(#tag.to_owned()) } else { Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 1"))) };
                            }
                            Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("variant `{}` has no object child at the requested path", #wire_variant)))
                        }
                    } else {
                        quote! { Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected an object, found unit variant `{}`", #wire_variant))) }
                    };
                    Ok(quote! { Self::#variant_ident => { #dispatch } })
                }
                Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                    let dispatch = if container.tag.is_none() {
                        quote! {
                            if path.is_empty() {
                                return if index == 0 { Ok(#wire_variant.to_owned()) } else { Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 1"))) };
                            }
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #value_crate::ToValue::value_key_at_path(payload, __rest, index).map_err(|error| error.under(__segment))
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        quote! {
                            if path.is_empty() {
                                return match index {
                                    0 => Ok(#tag.to_owned()),
                                    1 => Ok(#content.to_owned()),
                                    _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 2"))),
                                };
                            }
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected an object below `{}`, found `{}`", #content, __segment)));
                            }
                            #value_crate::ToValue::value_key_at_path(payload, __rest, index).map_err(|error| error.under(__segment))
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        quote! {
                            if path.is_empty() {
                                if index == 0 { return Ok(#tag.to_owned()); }
                                return match #value_crate::ToValue::value_shape_at_path(payload, &[])? {
                                    #value_crate::ValueShape::Object { len } if index <= len => #value_crate::ToValue::value_key_at_path(payload, &[], index - 1),
                                    #value_crate::ValueShape::Object { len } => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {}", len + 1))),
                                    _ if index == 1 => Ok("value".to_owned()),
                                    _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 2"))),
                                };
                            }
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "enum tag is not an object"));
                            }
                            if matches!(#value_crate::ToValue::value_shape_at_path(payload, &[])?, #value_crate::ValueShape::Object { .. }) {
                                return #value_crate::ToValue::value_key_at_path(payload, path, index);
                            }
                            if !path.first().is_some_and(|segment| *segment == "value") {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", path[0])));
                            }
                            #value_crate::ToValue::value_key_at_path(payload, &path[1..], index)
                        }
                    };
                    Ok(quote! { Self::#variant_ident(payload) => { #dispatch } })
                }
                Fields::Named(named) => {
                    let fields = variant_named_fields(named, container, &variant_attrs)?;
                    let (bindings, fields) = variant_path_bindings(&fields);
                    let pattern = quote! { Self::#variant_ident { #(#bindings),* } };
                    let dispatch = if container.tag.is_none() {
                        let named = named_variant_key_dispatch(&fields, &quote! { __rest }, value_crate);
                        quote! {
                            if path.is_empty() {
                                return if index == 0 { Ok(#wire_variant.to_owned()) } else { Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 1"))) };
                            }
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #named
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let named = named_variant_key_dispatch(&fields, &quote! { __rest }, value_crate);
                        quote! {
                            if path.is_empty() {
                                return match index {
                                    0 => Ok(#tag.to_owned()), 1 => Ok(#content.to_owned()),
                                    _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 2"))),
                                };
                            }
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected an object below `{}`, found `{}`", #content, __segment)));
                            }
                            #named
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let root_steps = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
                            let ident = &field.ident;
                            let wire_name = &field.wire_name;
                            let admitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
                                let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                                quote! { !#predicate(#ident) }
                            }).unwrap_or_else(|| quote! { true });
                            quote! {
                                if #admitted {
                                    if index == __offset { return Ok(#wire_name.to_owned()); }
                                    __offset += 1;
                                }
                            }
                        });
                        let named_path = named_variant_key_dispatch(&fields, &quote! { path }, value_crate);
                        quote! {
                            if path.is_empty() {
                                if index == 0 { return Ok(#tag.to_owned()); }
                                let mut __offset = 1usize;
                                #(#root_steps)*
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {__offset}")));
                            }
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "enum tag is not an object"));
                            }
                            #named_path
                        }
                    };
                    Ok(quote! { #pattern => { #dispatch } })
                }
                other => Err(syn::Error::new_spanned(other, "#[derive(ToValue)] enum variants must be unit, a single unnamed payload, or named fields")),
            }
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! { match self { #(#arms),* } })
}

fn enum_to_path_body(data: &syn::DataEnum, container: &ContainerAttrs, value_crate: &syn::Path, method: &str) -> syn::Result<proc_macro2::TokenStream> {
    if data.variants.is_empty() {
        return Ok(quote! { match *self {} });
    }
    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
    let shape = method == "value_shape_at_path";
    let tag_value = |wire_variant: &str| {
        if shape {
            quote! { Ok(#value_crate::ValueShape::String) }
        } else {
            quote! { Ok(#value_crate::DslValue::String(#wire_variant.to_owned())) }
        }
    };
    let arms = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_attrs = parse_variant_attrs(&variant.attrs)?;
            let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
            match &variant.fields {
                Fields::Unit => {
                    let tag_result = tag_value(&wire_variant);
                    let root = if shape {
                        if container.tag.is_some() {
                            quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 1 }); } }
                        } else {
                            quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::String); } }
                        }
                    } else {
                        quote! {}
                    };
                    let dispatch = if let Some(tag) = &container.tag {
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment == #tag && __rest.is_empty() {
                                #tag_result
                            } else {
                                Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("variant `{}` has no child `{}`", #wire_variant, __segment)))
                            }
                        }
                    } else {
                        quote! { #root Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("variant `{}` has no children", #wire_variant))) }
                    };
                    Ok(quote! { Self::#variant_ident => { #dispatch } })
                }
                Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                    let call = quote! { #value_crate::ToValue::#method_ident(payload, __rest) };
                    let dispatch = if container.tag.is_none() {
                        let root = if shape { quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 1 }); } } } else { quote! {} };
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #call.map_err(|error| error.under(__segment))
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let tag_result = tag_value(&wire_variant);
                        let root = if shape { quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 2 }); } } } else { quote! {} };
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment == #tag && __rest.is_empty() {
                                return #tag_result;
                            }
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                            }
                            #call.map_err(|error| error.under(__segment))
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let tag_result = tag_value(&wire_variant);
                        let root = if shape {
                            quote! {
                                if path.is_empty() {
                                    return match #value_crate::ToValue::value_shape_at_path(payload, &[])? {
                                        #value_crate::ValueShape::Object { len } => Ok(#value_crate::ValueShape::Object { len: len.checked_add(1).ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::OwnershipLimit, "object length overflow"))? }),
                                        _ => Ok(#value_crate::ValueShape::Object { len: 2 }),
                                    };
                                }
                            }
                        } else {
                            quote! {}
                        };
                        quote! {
                            #root
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                if path.len() == 1 { return #tag_result; }
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "enum tag has no children"));
                            }
                            if matches!(#value_crate::ToValue::value_shape_at_path(payload, &[])?, #value_crate::ValueShape::Object { .. }) {
                                return #value_crate::ToValue::#method_ident(payload, path);
                            }
                            if !path.first().is_some_and(|segment| *segment == "value") {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", path[0])));
                            }
                            #value_crate::ToValue::#method_ident(payload, &path[1..])
                        }
                    };
                    Ok(quote! { Self::#variant_ident(payload) => { #dispatch } })
                }
                Fields::Named(named) => {
                    let fields = variant_named_fields(named, container, &variant_attrs)?;
                    let (bindings, fields) = variant_path_bindings(&fields);
                    let pattern = quote! { Self::#variant_ident { #(#bindings),* } };
                    let dispatch = if container.tag.is_none() {
                        let named_dispatch = named_variant_to_path_dispatch(&fields, &quote! { __rest }, method, value_crate);
                        let root = if shape { quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 1 }); } } } else { quote! {} };
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #named_dispatch
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let tag_result = tag_value(&wire_variant);
                        let named_dispatch = named_variant_to_path_dispatch(&fields, &quote! { __rest }, method, value_crate);
                        let root = if shape { quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 2 }); } } } else { quote! {} };
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment == #tag && __rest.is_empty() { return #tag_result; }
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                            }
                            #named_dispatch
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let tag_result = tag_value(&wire_variant);
                        let named_dispatch = named_variant_to_path_dispatch(&fields, &quote! { path }, method, value_crate);
                        let root = if shape {
                            let counts = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
                                let ident = &field.ident;
                                match &field.attrs.skip_serializing_if {
                                    Some(predicate) => {
                                        let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                                        quote! { if !#predicate(#ident) { __len += 1; } }
                                    }
                                    None => quote! { __len += 1; },
                                }
                            });
                            quote! {
                                if path.is_empty() {
                                    let mut __len = 1usize;
                                    #(#counts)*
                                    return Ok(#value_crate::ValueShape::Object { len: __len });
                                }
                            }
                        } else {
                            quote! {}
                        };
                        quote! {
                            #root
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                if path.len() == 1 { return #tag_result; }
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "enum tag has no children"));
                            }
                            #named_dispatch
                        }
                    };
                    Ok(quote! { #pattern => { #dispatch } })
                }
                other => Err(syn::Error::new_spanned(other, "#[derive(ToValue)] enum variants must be unit, a single unnamed payload, or named fields")),
            }
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let root = if shape {
        quote! {}
    } else {
        quote! { return Ok(#value_crate::ToValue::to_value(self)); }
    };
    Ok(quote! {
        if path.is_empty() { #root }
        match self { #(#arms),* }
    })
}

fn enum_edit_path_body(data: &syn::DataEnum, container: &ContainerAttrs, value_crate: &syn::Path) -> syn::Result<proc_macro2::TokenStream> {
    if data.variants.is_empty() {
        return Ok(quote! { match *self {} });
    }
    let arms = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_attrs = parse_variant_attrs(&variant.attrs)?;
            let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
            match &variant.fields {
                Fields::Unit => Ok(quote! {
                    Self::#variant_ident => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("variant `{}` has no editable payload", #wire_variant)))
                }),
                Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                    let dispatch = if container.tag.is_none() {
                        quote! {
                            let (__segment, __rest) = path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "missing external variant path"))?;
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #value_crate::FromValue::edit_value_at_path(payload, __rest, edit).map_err(|error| error.under(__segment))
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        quote! {
                            let (__segment, __rest) = path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "missing adjacent enum path"))?;
                            if *__segment == #tag {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot edit an enum tag in place"));
                            }
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                            }
                            #value_crate::FromValue::edit_value_at_path(payload, __rest, edit).map_err(|error| error.under(__segment))
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        quote! {
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot edit an enum tag in place"));
                            }
                            match #value_crate::FromValue::edit_value_at_path(payload, path, edit.clone()) {
                                Ok(()) => Ok(()),
                                Err(object_error) if path.first().is_some_and(|segment| *segment == "value") => {
                                    #value_crate::FromValue::edit_value_at_path(payload, &path[1..], edit).map_err(|_| object_error)
                                }
                                Err(error) => Err(error),
                            }
                        }
                    };
                    Ok(quote! { Self::#variant_ident(payload) => { #dispatch } })
                }
                Fields::Named(named) => {
                    let fields = variant_named_fields(named, container, &variant_attrs)?;
                    let (bindings, fields) = variant_path_bindings(&fields);
                    let pattern = quote! { Self::#variant_ident { #(#bindings),* } };
                    let dispatch = if container.tag.is_none() {
                        let named_dispatch = named_variant_edit_dispatch(&fields, &quote! { __rest }, &quote! { edit }, value_crate);
                        quote! {
                            let (__segment, __rest) = path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "missing external variant path"))?;
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #named_dispatch
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let named_dispatch = named_variant_edit_dispatch(&fields, &quote! { __rest }, &quote! { edit }, value_crate);
                        quote! {
                            let (__segment, __rest) = path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "missing adjacent enum path"))?;
                            if *__segment == #tag {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot edit an enum tag in place"));
                            }
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                            }
                            #named_dispatch
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let named_dispatch = named_variant_edit_dispatch(&fields, &quote! { path }, &quote! { edit }, value_crate);
                        quote! {
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot edit an enum tag in place"));
                            }
                            #named_dispatch
                        }
                    };
                    Ok(quote! { #pattern => { #dispatch } })
                }
                other => Err(syn::Error::new_spanned(other, "#[derive(FromValue)] enum variants must be unit, a single unnamed payload, or named fields")),
            }
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        if path.is_empty() {
            return match edit {
                #value_crate::ValueEdit::Set(value) => {
                    let replacement = <Self as #value_crate::FromValue>::from_value(value)?;
                    *self = replacement;
                    Ok(())
                }
                #value_crate::ValueEdit::Insert(_) | #value_crate::ValueEdit::InsertAt { .. } => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot insert at the enum root")),
                #value_crate::ValueEdit::Remove => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot remove the enum root")),
            };
        }
        match self { #(#arms),* }
    })
}
//#endregion 🧭️TypedPath

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
        Data::Enum(data) if data.variants.is_empty() => quote! { match *self {} },
        Data::Enum(data) if container.tag.is_none() && data.variants.iter().all(|variant| matches!(variant.fields, Fields::Unit)) => {
            let arms = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                Ok(quote! { Self::#variant_ident => #value_crate::DslValue::String(#wire_variant.to_string()) })
            }).collect::<syn::Result<Vec<_>>>()?;
            quote! {
                match *self { #(#arms),* }
            }
        }
        Data::Enum(data) if container.tag.is_none() => {
            // 🏷️ Externally-tagged (serde's own default enum representation when no `#[serde(tag
            // = …)]` is present): a unit variant is still the bare wire-name string, a
            // single-unnamed-field or named-field variant becomes a one-key object
            // `{"VariantName": <payload>}`.
            let arms = data
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
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
                            let pushes = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_to_value_push(field, &field_attrs, &wire_name, &ident, &push_into, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
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
                })
                .collect::<syn::Result<Vec<_>>>()?;
            quote! {
                match self { #(#arms),* }
            }
        }
        Data::Enum(data) => {
            let Some(tag) = &container.tag else {
                unreachable!("the tag.is_none() arm above already handles every tag-less enum");
            };
            let arms = data
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
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
                            let pushes = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_to_value_push(field, &field_attrs, &wire_name, &ident, &push_into, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
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
                            let pushes = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_to_value_push(field, &field_attrs, &wire_name, &ident, &push_into, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
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
                })
                .collect::<syn::Result<Vec<_>>>()?;
            quote! {
                match self { #(#arms),* }
            }
        }
        Data::Union(_) => return Err(syn::Error::new_spanned(&input.ident, "#[derive(ToValue)] does not support unions")),
    };

    let (path_body, shape_body, key_body) = match &input.data {
        Data::Struct(data) if container.transparent => match &data.fields {
            Fields::Named(named) if named.named.len() == 1 => {
                let ident = named.named.first().expect("checked len == 1").ident.clone().expect("named field");
                (
                    quote! { #value_crate::ToValue::value_at_path(&self.#ident, path) },
                    quote! { #value_crate::ToValue::value_shape_at_path(&self.#ident, path) },
                    quote! { #value_crate::ToValue::value_key_at_path(&self.#ident, path, index) },
                )
            }
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => (
                quote! { #value_crate::ToValue::value_at_path(&self.0, path) },
                quote! { #value_crate::ToValue::value_shape_at_path(&self.0, path) },
                quote! { #value_crate::ToValue::value_key_at_path(&self.0, path, index) },
            ),
            other => return Err(syn::Error::new_spanned(other, "#[value(transparent)] requires exactly one field")),
        },
        Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => (
            quote! { #value_crate::ToValue::value_at_path(&self.0, path) },
            quote! { #value_crate::ToValue::value_shape_at_path(&self.0, path) },
            quote! { #value_crate::ToValue::value_key_at_path(&self.0, path, index) },
        ),
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            (
                struct_to_path_body(&fields, &value_crate, "value_at_path"),
                struct_to_path_body(&fields, &value_crate, "value_shape_at_path"),
                struct_key_at_path_body(&fields, &value_crate),
            )
        }
        Data::Enum(data) => (
            enum_to_path_body(data, &container, &value_crate, "value_at_path")?,
            enum_to_path_body(data, &container, &value_crate, "value_shape_at_path")?,
            enum_key_at_path_body(data, &container, &value_crate)?,
        ),
        Data::Union(_) => unreachable!("union rejected above"),
    };

    let controlled_body=controlled_to_body(input,&container,&value_crate)?;

    Ok(quote! {
        impl #impl_generics #value_crate::ToValue for #name #ty_generics #where_clause {
            fn to_value(&self) -> #value_crate::DslValue {
                #body
            }
            fn to_value_controlled(&self,control:&mut #value_crate::NativeEncodeControl<'_>)->::core::result::Result<#value_crate::DslValue,#value_crate::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|{#controlled_body}))
            }

            fn value_at_path(&self, path: &[&str]) -> ::core::result::Result<#value_crate::DslValue, #value_crate::ValueError> {
                #path_body
            }

            fn value_shape_at_path(&self, path: &[&str]) -> ::core::result::Result<#value_crate::ValueShape, #value_crate::ValueError> {
                #shape_body
            }

            fn value_key_at_path(&self, path: &[&str], index: usize) -> ::core::result::Result<String, #value_crate::ValueError> {
                #key_body
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
                let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                Ok(quote! { #wire_variant => Ok(Self::#variant_ident), })
            }).collect::<syn::Result<Vec<_>>>()?;
            quote! {
                let __s = match value { #value_crate::DslValue::String(s) => s, other => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))) };
                match __s.as_str() {
                    #(#arms)*
                    other => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("unknown variant `{other}`"))),
                }
            }
        }
        Data::Enum(data) if container.tag.is_none() => {
            // 🏷️ Externally-tagged (serde's own default enum representation when no `#[serde(tag
            // = …)]` is present) — mirrors `expand_to_value`'s sibling arm above.
            let string_arms = data.variants.iter().filter(|variant| matches!(variant.fields, Fields::Unit)).map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                Ok(quote! { #wire_variant => return Ok(Self::#variant_ident), })
            }).collect::<syn::Result<Vec<_>>>()?;
            let object_arms = data
                .variants
                .iter()
                .filter(|variant| !matches!(variant.fields, Fields::Unit))
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                    let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                    let arm: syn::Result<proc_macro2::TokenStream> = match &variant.fields {
                        Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                            let payload_ty = &unnamed.unnamed[0].ty;
                            Ok(quote! {
                                #wire_variant => Self::#variant_ident(<#payload_ty as #value_crate::FromValue>::from_value(__payload)?),
                            })
                        }
                        Fields::Named(named) => {
                            let field_wire_names: Vec<String> = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs))
                                })
                                .collect();
                            let deny_check = if container.deny_unknown_fields {
                                deny_unknown_keys(&quote! { __variant_entries }, &field_wire_names, &value_crate)
                            } else {
                                quote! {}
                            };
                            let entries_ident = quote! { __variant_entries };
                            let reads = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_from_value_read(field, &field_attrs, &wire_name, &ident, &entries_ident, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
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
                })
                .collect::<syn::Result<Vec<_>>>()?;
            quote! {
                if let #value_crate::DslValue::String(__s) = &value {
                    match __s.as_str() {
                        #(#string_arms)*
                        _ => {}
                    }
                }
                let __entries = #value_crate::DslValue::into_object(value)?;
                if __entries.len() != 1 {
                    return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected an externally-tagged enum object with exactly one key, found {} keys", __entries.len())));
                }
                let (__key, __payload) = __entries.into_iter().next().expect("checked len == 1 above");
                Ok(match __key.as_str() {
                    #(#object_arms)*
                    other => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("unknown variant `{other}`"))),
                })
            }
        }
        Data::Enum(data) => {
            let Some(tag) = &container.tag else {
                unreachable!("the tag.is_none() arm above already handles every tag-less enum");
            };
            let arms = data
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                    let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                    let arm: syn::Result<proc_macro2::TokenStream> = match (&variant.fields, &container.content) {
                        (Fields::Unit, Some(_)) => Ok(quote! {
                            #wire_variant => Self::#variant_ident,
                        }),
                        (Fields::Unit, None) => {
                            // 🛡️ Internally-tagged unit variant: the whole entries object is nothing
                            // but the tag, so `deny_unknown_fields` allows exactly `{tag}`.
                            let deny_check = if container.deny_unknown_fields {
                                deny_unknown_keys(&quote! { __entries }, std::slice::from_ref(tag), &value_crate)
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
                            //
                            // 🪆 The `__scalar` arm is the exact inverse of `expand_to_value`'s runtime
                            // branch: a payload whose `to_value()` is NOT an object cannot be spliced
                            // beside the tag, so the encoder carries it as the single entry
                            // `{"value": <scalar>}`. Which of the two shapes a given wire object is
                            // cannot be decided from the payload TYPE at expansion time (a struct whose
                            // only field is literally named `value` produces the same key set), so the
                            // object form is attempted first and the carrier is unwrapped only when it
                            // fails — that ordering decodes both shapes correctly, where a key-shape
                            // test alone would mis-decode `struct P { value: String }`.
                            let payload_ty = &unnamed.unnamed[0].ty;
                            Ok(quote! {
                                #wire_variant => Self::#variant_ident({
                                    let __payload: Vec<(String, #value_crate::DslValue)> = __entries.iter().filter(|(__k, _)| __k != #tag).cloned().collect();
                                    match <#payload_ty as #value_crate::FromValue>::from_value(#value_crate::DslValue::Object(__payload.clone())) {
                                        ::core::result::Result::Ok(__decoded) => __decoded,
                                        ::core::result::Result::Err(__object_error) => match __payload.as_slice() {
                                            [(__k, __scalar)] if __k == "value" => <#payload_ty as #value_crate::FromValue>::from_value(__scalar.clone())?,
                                            _ => return ::core::result::Result::Err(__object_error),
                                        },
                                    }
                                }),
                            })
                        }
                        (Fields::Named(named), content_key) => {
                            let source = if content_key.is_some() {
                                quote! { __content()?.into_object()? }
                            } else {
                                quote! { __entries.clone() }
                            };
                            let field_wire_names: Vec<String> = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs))
                                })
                                .collect();
                            let deny_check = if container.deny_unknown_fields {
                                let allowed: Vec<String> = if content_key.is_some() {
                                    field_wire_names
                                } else {
                                    let mut allowed = vec![tag.clone()];
                                    allowed.extend(field_wire_names);
                                    allowed
                                };
                                deny_unknown_keys(&quote! { __variant_entries }, &allowed, &value_crate)
                            } else {
                                quote! {}
                            };
                            let entries_ident = quote! { __variant_entries };
                            let reads = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_from_value_read(field, &field_attrs, &wire_name, &ident, &entries_ident, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
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
                })
                .collect::<syn::Result<Vec<_>>>()?;
            let content_helper = match &container.content {
                Some(content) => quote! {
                    let __content = || -> ::core::result::Result<#value_crate::DslValue, #value_crate::ValueError> {
                        __entries.iter().find(|(k, _)| k == #content).map(|(_, v)| v.clone()).ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing content field `{}`", #content)))
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
                let __tag = __entries.iter().find(|(k, _)| k == #tag).map(|(_, v)| v.clone()).ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing tag field `{}`", #tag)))?;
                let __tag = match __tag { #value_crate::DslValue::String(s) => s, other => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected a string tag, found {other:?}"))) };
                #content_helper
                Ok(match __tag.as_str() {
                    #(#arms)*
                    other => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("unknown `{}` variant `{other}`", #tag))),
                })
            }
        }
        Data::Union(_) => return Err(syn::Error::new_spanned(&input.ident, "#[derive(FromValue)] does not support unions")),
    };

    let controlled_body = controlled_from_body(input, &container, &value_crate)?;
    let retirement_body = controlled_retirement_body(input, &container, &value_crate)?;
    let controlled_default_method=if let Some(path)=&container.default_controlled{let path:syn::Path=syn::parse_str(path)?;quote!{fn default_value_controlled(control:&mut #value_crate::NativeDecodeControl<'_>)->Result<Self,#value_crate::ValueError>{#path(control)}}}else{quote!{}};
    let edit_body = match &input.data {
        Data::Struct(data) if container.transparent => match &data.fields {
            Fields::Named(named) if named.named.len() == 1 => {
                let ident = named.named.first().expect("checked len == 1").ident.clone().expect("named field");
                quote! { #value_crate::FromValue::edit_value_at_path(&mut self.#ident, path, edit) }
            }
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => quote! { #value_crate::FromValue::edit_value_at_path(&mut self.0, path, edit) },
            other => return Err(syn::Error::new_spanned(other, "#[value(transparent)] requires exactly one field")),
        },
        Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => {
            quote! { #value_crate::FromValue::edit_value_at_path(&mut self.0, path, edit) }
        }
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            struct_edit_path_body(&fields, &value_crate)
        }
        Data::Enum(data) => enum_edit_path_body(data, &container, &value_crate)?,
        Data::Union(_) => unreachable!("union rejected above"),
    };

    Ok(quote! {
        impl #impl_generics #value_crate::FromValue for #name #ty_generics #where_clause {
            fn from_value(value: #value_crate::DslValue) -> ::core::result::Result<Self, #value_crate::ValueError> {
                #body
            }

            fn from_value_controlled(value: &#value_crate::DslValue, control: &mut #value_crate::NativeDecodeControl<'_>) -> ::core::result::Result<Self, #value_crate::ValueError> {
                control.scoped_depth(64, |control| control.scoped_stage(|control| { #controlled_body }))
            }
            #controlled_default_method
            fn retire_decoded(self) { #retirement_body }
            fn edit_value_at_path(&mut self, path: &[&str], edit: #value_crate::ValueEdit) -> ::core::result::Result<(), #value_crate::ValueError> {
                #edit_body
            }
        }
    })
}
//#endregion 🔖️Expand

fn controlled_field_decode(ty:&syn::Type,attrs:&FieldAttrs,value:proc_macro2::TokenStream,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    if attrs.effective_deserialize_with().is_some()&&attrs.retire_with.is_none(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom controlled value conversion requires explicit retirement"))})}
    if let Some(path)=attrs.deserialize_controlled_with.clone(){
        let path:syn::Path=syn::parse_str(&path)?;return Ok(quote!{#path(#value,control)})
    }
    if attrs.effective_deserialize_with().is_some(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom value conversion has no controlled constructor"))})}
    Ok(quote!{<#ty as #c::FromValue>::from_value_controlled(#value,control)})
}

fn controlled_field_default(ty:&syn::Type,attrs:&FieldAttrs,container:&ContainerAttrs,wire:&str,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    if attrs.skip&&attrs.retire_with.is_none(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "skipped controlled default requires explicit retirement"))})}
    if attrs.required&&!attrs.skip{return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, format!("missing field `{}`",#wire)))})}
    if let Some(path)=&attrs.default_controlled{let path:syn::Path=syn::parse_str(path)?;return Ok(quote!{#path(control)})}
    if matches!(attrs.default,FieldDefault::Path(_))||attrs.skip{return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom or skipped default has no controlled constructor"))})}
    if matches!(attrs.default,FieldDefault::Bare)||container.default||type_is_option(ty){
        if attrs.effective_deserialize_with().is_some(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom value default has no controlled constructor"))})}
        return Ok(quote!{<#ty as #c::FromValue>::default_value_controlled(control)})
    }
    Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, format!("missing field `{}`",#wire)))})
}

fn controlled_field_retire(ty:&syn::Type,attrs:&FieldAttrs,value:proc_macro2::TokenStream,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    if let Some(path)=&attrs.retire_with{let path:syn::Path=syn::parse_str(path)?;return Ok(quote!{#path(#value)})}
    if attrs.skip||attrs.effective_deserialize_with().is_some(){return Ok(quote!{drop(#value)})}
    Ok(quote!{<#ty as #c::FromValue>::retire_decoded(#value)})
}

fn controlled_named_fields(fields:&syn::FieldsNamed,container:&ContainerAttrs,rename:&Option<String>,entries:proc_macro2::TokenStream,extra:&[String],constructor:proc_macro2::TokenStream,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    let mut reads=Vec::new();let mut members=Vec::new();let mut names=Vec::new();
    for field in &fields.named{let attrs=parse_field_attrs(&field.attrs)?;let ident=field.ident.as_ref().unwrap();let wire=field_wire_name(&ident.to_string(),&attrs.rename,rename);if !attrs.flatten{names.push(wire.clone())}}
    let allowed:Vec<_>=names.iter().chain(extra.iter()).collect();
    let deny=if container.deny_unknown_fields{quote!{#c::DslValue::deny_fields_controlled(#entries,&[#(#allowed),*],control)?;}}else{quote!{}};
    for field in &fields.named{
        let attrs=parse_field_attrs(&field.attrs)?;let ident=field.ident.as_ref().unwrap();let ty=&field.ty;let wire=field_wire_name(&ident.to_string(),&attrs.rename,rename);let guard=format_ident!("__owned_{}",ident);
        let missing=controlled_field_default(ty,&attrs,container,&wire,c)?;
        let expression=if attrs.skip{missing}else if attrs.flatten{
            let decode=controlled_field_decode(ty,&attrs,quote!{__remaining.get()},c)?;
            quote!{{let __remaining=<#c::DslValue as #c::FromValue>::guard_decoded(#c::DslValue::filtered_object_controlled(#entries,&[#(#names),*],control)?);#decode}}
        }else{
            let decode=controlled_field_decode(ty,&attrs,quote!{__field},c)?;
            quote!{match #c::DslValue::field_controlled(#entries,#wire,control)?{Some(__field)=>#decode,None=>#missing}}
        };
        let retire=controlled_field_retire(ty,&attrs,quote!{__value},c)?;
        if attrs.effective_deserialize_with().is_some()||attrs.skip||attrs.retire_with.is_some(){
            reads.push(quote!{let #guard:#ty=#expression.map_err(|error:#c::ValueError|error.under(#wire))?;let #guard= #c::DecodedValue::new(#guard,|__value:#ty|{#retire});control.step()?;});
        }else{
            reads.push(quote!{let #guard=<#ty as #c::FromValue>::guard_decoded(#expression.map_err(|error:#c::ValueError|error.under(#wire))?);control.step()?;});
        }
        members.push(quote!{#ident:#guard.take()});
    }
    let count=fields.named.len();
    Ok(quote!{#deny control.begin_stage(#count)?;#(#reads)* Ok(#constructor{#(#members),*})})
}

fn type_mentions_owner(ty:&syn::Type,owner:&syn::Ident)->bool {
    match ty {
        syn::Type::Path(path)=>path.path.segments.last().is_some_and(|segment| {
            segment.ident==*owner||segment.ident=="Self"||match &segment.arguments {
                syn::PathArguments::AngleBracketed(arguments)=>arguments.args.iter().any(|argument|match argument {
                    syn::GenericArgument::Type(ty)=>type_mentions_owner(ty,owner),
                    syn::GenericArgument::AssocType(binding)=>type_mentions_owner(&binding.ty,owner),
                    _=>false,
                }),
                _=>false,
            }
        }),
        syn::Type::Array(array)=>type_mentions_owner(&array.elem,owner),
        syn::Type::Slice(slice)=>type_mentions_owner(&slice.elem,owner),
        syn::Type::Tuple(tuple)=>tuple.elems.iter().any(|ty|type_mentions_owner(ty,owner)),
        syn::Type::Paren(paren)=>type_mentions_owner(&paren.elem,owner),
        syn::Type::Group(group)=>type_mentions_owner(&group.elem,owner),
        syn::Type::Reference(reference)=>type_mentions_owner(&reference.elem,owner),
        syn::Type::Ptr(pointer)=>type_mentions_owner(&pointer.elem,owner),
        _=>false,
    }
}

fn controlled_from_body(input:&DeriveInput,container:&ContainerAttrs,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    let recursive=input.data.clone();
    let mentions=match &recursive{Data::Struct(data)=>data.fields.iter().any(|f|type_mentions_owner(&f.ty,&input.ident)),Data::Enum(data)=>data.variants.iter().flat_map(|v|v.fields.iter()).any(|f|type_mentions_owner(&f.ty,&input.ident)),_=>false};
    if mentions&&container.retire_with.is_none(){return Ok(quote!{control.checkpoint()?;Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "recursive value owner requires explicit controlled retirement"))})}
    match &input.data{
        Data::Struct(data) if container.transparent||matches!(&data.fields,Fields::Unnamed(f)if f.unnamed.len()==1)=>{
            let field=data.fields.iter().next().unwrap();let attrs=parse_field_attrs(&field.attrs)?;let ty=&field.ty;let decode=controlled_field_decode(ty,&attrs,quote!{value},c)?;
            Ok(if let Some(ident)=&field.ident{quote!{control.begin_stage(0)?;Ok(Self{#ident:#decode?})}}else{quote!{control.begin_stage(0)?;Ok(Self(#decode?))}})
        },
        Data::Struct(data)=>{
            let Fields::Named(fields)=&data.fields else{return Err(syn::Error::new_spanned(&data.fields,"controlled value requires named fields"))};
            let fields=controlled_named_fields(fields,container,&container.rename_all,quote!{__entries},&[],quote!{Self},c)?;
            Ok(quote!{let __entries=value.object_controlled(control)?;#fields})
        },
        Data::Enum(data)=>{
            let mut string_arms=Vec::new();let mut object_arms=Vec::new();
            for variant in &data.variants{
                let attrs=parse_variant_attrs(&variant.attrs)?;let ident=&variant.ident;let wire=variant_wire_name(&ident.to_string(),&attrs.rename,&container.rename_all);
                if matches!(variant.fields,Fields::Unit){
                    string_arms.push(quote!{#wire=>Ok(Self::#ident),});
                    let deny=if container.deny_unknown_fields&&container.content.is_none(){let allowed:Vec<_>=container.tag.iter().collect();quote!{#c::DslValue::deny_fields_controlled(__entries,&[#(#allowed),*],control)?;}}else{quote!{}};
                    object_arms.push(quote!{#wire=>{#deny Ok(Self::#ident)},});continue
                }
                let payload=match(&container.tag,&container.content){
                    (None,_)=>quote!{__payload},
                    (Some(_),Some(content))=>quote!{#c::DslValue::field_controlled(__entries,#content,control)?.ok_or_else(||#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "missing enum content"))?},
                    (Some(_),None)=>quote!{value},
                };
                let body=match &variant.fields{
                    Fields::Unnamed(fields)if fields.unnamed.len()==1=>{
                        let field=fields.unnamed.first().unwrap();let ty=&field.ty;
                        if let(Some(tag),None)=(&container.tag,&container.content){
                            quote!{
                                let __remaining=<#c::DslValue as #c::FromValue>::guard_decoded(#c::DslValue::filtered_object_controlled(__entries,&[#tag],control)?);
                                match <#ty as #c::FromValue>::from_value_controlled(__remaining.get(),control){
                                    Ok(payload)=>Ok(Self::#ident(payload)),
                                    Err(error)=>match __remaining.get(){#c::DslValue::Object(fields)=>match fields.as_slice(){[(key,payload)]if key=="value"=>Ok(Self::#ident(<#ty as #c::FromValue>::from_value_controlled(payload,control)?)),_=>Err(error)},_=>Err(error)}
                                }
                            }
                        }else{let attrs=parse_field_attrs(&field.attrs)?;let decode=controlled_field_decode(ty,&attrs,payload,c)?;quote!{Ok(Self::#ident(#decode?))}}
                    },
                    Fields::Named(fields)=>{
                        let extra=if container.content.is_none(){container.tag.iter().cloned().collect::<Vec<_>>()}else{Vec::new()};
                        let body=controlled_named_fields(fields,container,&container.field_rename_all(&attrs),quote!{__variant_entries},&extra,quote!{Self::#ident},c)?;
                        quote!{let __variant_entries=(#payload).object_controlled(control)?;#body}
                    },
                    _=>return Err(syn::Error::new_spanned(&variant.fields,"unsupported controlled enum fields"))
                };
                object_arms.push(quote!{#wire=>{#body},});
            }
            if container.tag.is_none()&&data.variants.iter().all(|v|matches!(v.fields,Fields::Unit)){
                return Ok(quote!{control.begin_stage(1)?;control.step()?;let #c::DslValue::String(tag)=value else{return Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "expected enum string"))};match tag.as_str(){#(#string_arms)*_=>Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "unknown enum variant"))}})
            }
            if let Some(tag)=&container.tag{
                let deny=if let(Some(content),true)=(&container.content,container.deny_unknown_fields){quote!{#c::DslValue::deny_fields_controlled(__entries,&[#tag,#content],control)?;}}else{quote!{}};
                Ok(quote!{let __entries=value.object_controlled(control)?;#deny let __tag=#c::DslValue::field_controlled(__entries,#tag,control)?.ok_or_else(||#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "missing enum tag"))?;let #c::DslValue::String(__tag)=__tag else{return Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "expected enum string tag"))};match __tag.as_str(){#(#object_arms)*_=>Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "unknown enum variant"))}})
            }else{
                Ok(quote!{control.begin_stage(0)?;if let #c::DslValue::String(tag)=value{return match tag.as_str(){#(#string_arms)*_=>Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "unknown enum variant"))}}let __entries=value.object_controlled(control)?;let[(__tag,__payload)]=__entries else{return Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "externally tagged enum requires one field"))};match __tag.as_str(){#(#object_arms)*_=>Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "unknown enum variant"))}})
            }
        },
        Data::Union(_)=>Err(syn::Error::new_spanned(input,"controlled unions unsupported"))
    }
}

fn controlled_retirement_body(input:&DeriveInput,container:&ContainerAttrs,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    if let Some(path)=&container.retire_with{let path:syn::Path=syn::parse_str(path)?;return Ok(quote!{#path(self);})}
    let body=|fields:&Fields,constructor:proc_macro2::TokenStream|->syn::Result<proc_macro2::TokenStream>{
        let mut names=Vec::new();let mut retirements=Vec::new();
        for(index,field)in fields.iter().enumerate(){let name=field.ident.clone().unwrap_or_else(||format_ident!("__field_{index}"));let attrs=parse_field_attrs(&field.attrs)?;retirements.push(controlled_field_retire(&field.ty,&attrs,quote!{#name},c)?);names.push(name);}
        let pattern=match fields{Fields::Named(_)=>quote!{#constructor{#(#names),*}},Fields::Unnamed(_)=>quote!{#constructor(#(#names),*)},Fields::Unit=>constructor};
        Ok(quote!{#pattern=>{#(#retirements;)*}})
    };
    match &input.data{Data::Struct(data)=>{let arm=body(&data.fields,quote!{Self})?;Ok(quote!{match self{#arm}})},Data::Enum(data)=>{let arms=data.variants.iter().map(|v|{let name=&v.ident;body(&v.fields,quote!{Self::#name})}).collect::<syn::Result<Vec<_>>>()?;Ok(quote!{match self{#(#arms),*}})},Data::Union(_)=>Ok(quote!{drop(self)})}
}

fn controlled_field_encode(attrs:&FieldAttrs,value:proc_macro2::TokenStream,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
 if let Some(path)=&attrs.serialize_controlled_with{let path:syn::Path=syn::parse_str(path)?;return Ok(quote!{#path(#value,control)})}
 if attrs.effective_serialize_with().is_some(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom value conversion has no controlled encoder"))})}
 Ok(quote!{#c::ToValue::to_value_controlled(#value,control)})
}

fn controlled_named_output(fields:&syn::FieldsNamed,rename:&Option<String>,source_self:bool,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
 let count=fields.named.len();let mut flags=Vec::new();let mut capacities=Vec::new();let mut pushes=Vec::new();
 for(index,field)in fields.named.iter().enumerate(){
  let ident=field.ident.as_ref().unwrap();let flag=format_ident!("__emit_{index}");let attrs=parse_field_attrs(&field.attrs)?;let wire=field_wire_name(&ident.to_string(),&attrs.rename,rename);let alias=format_ident!("__source_field_{index}");let access=if source_self{quote!{&self.#ident}}else{quote!{#alias}};
  let flatten=attrs.flatten&&source_self;
  let emit=if attrs.skip{quote!{false}}else if !flatten{if let Some(path)=&attrs.skip_serializing_if{let path:syn::Path=syn::parse_str(path)?;quote!{!#path(#access)}}else{quote!{true}}}else{quote!{true}};
  flags.push(quote!{let #flag=#emit;control.step()?;});
  if attrs.skip{pushes.push(quote!{control.step()?;});continue}
  if !flatten{capacities.push(quote!{::core::primitive::usize::from(#flag)});}
  let value=controlled_field_encode(&attrs,access,c)?;
  let push=if flatten{quote!{#c::DslValue::flatten_encoding_controlled(__output.get_mut(),#value?,control)?;}}else{quote!{#c::DslValue::push_encoding_controlled(__output.get_mut(),#wire,#value?,control)?;}};
  pushes.push(quote!{if #flag{#push}control.step()?;});
 }
 Ok(quote!{control.begin_stage(#count)?;#(#flags)*control.begin_stage(#count)?;let mut __output=#c::DslValue::object_encoding_controlled(0usize #( + #capacities )*,control)?;#(#pushes)*Ok(#c::DslValue::Object(__output.take()))})
}

fn controlled_to_body(input:&DeriveInput,container:&ContainerAttrs,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
 match &input.data{
  Data::Enum(data)if data.variants.is_empty()=>Ok(quote!{match *self{}}),
  Data::Struct(data)if container.transparent||matches!(&data.fields,Fields::Unnamed(fields)if fields.unnamed.len()==1)=>{
   let field=data.fields.iter().next().ok_or_else(||syn::Error::new_spanned(input,"transparent output requires field"))?;let access=if let Some(ident)=&field.ident{quote!{&self.#ident}}else{quote!{&self.0}};controlled_field_encode(&parse_field_attrs(&field.attrs)?,access,c)
  }
  Data::Struct(data)=>{let Fields::Named(fields)=&data.fields else{return Err(syn::Error::new_spanned(input,"controlled output requires named fields"))};controlled_named_output(fields,&container.rename_all,true,c)}
  Data::Enum(data)=>{
   let mut arms=Vec::new();
   for variant in &data.variants{
    let ident=&variant.ident;let attrs=parse_variant_attrs(&variant.attrs)?;let wire=variant_wire_name(&ident.to_string(),&attrs.rename,&container.rename_all);
    let(pattern,payload)=match &variant.fields{
     Fields::Unit=>(quote!{Self::#ident},None),
     Fields::Unnamed(fields)if fields.unnamed.len()==1=>{let field=fields.unnamed.first().unwrap();(quote!{Self::#ident(__payload)},Some(controlled_field_encode(&parse_field_attrs(&field.attrs)?,quote!{__payload},c)?))},
     Fields::Named(fields)=>{let idents=fields.named.iter().enumerate().map(|(index,field)|{let name=field.ident.as_ref().unwrap();let alias=format_ident!("__source_field_{index}");if parse_field_attrs(&field.attrs)?.skip{Ok(quote!{#name:_})}else{Ok(quote!{#name:#alias})}}).collect::<syn::Result<Vec<_>>>()?;(quote!{Self::#ident{#(#idents),*}},Some(controlled_named_output(fields,&container.field_rename_all(&attrs),false,c)?))},
     _=>return Err(syn::Error::new_spanned(variant,"unsupported controlled output fields"))
    };
    let body=match(&container.tag,&container.content,payload){
     (None,_,None)=>quote!{control.copy_text(#wire).map(#c::DslValue::String)},
     (None,_,Some(payload))=>quote!{let __payload=#c::DslValue::guard_encoded((||->::core::result::Result<#c::DslValue,#c::ValueError>{#payload})()?);let mut __wrapper=#c::DslValue::object_encoding_controlled(1,control)?;#c::DslValue::push_encoding_controlled(__wrapper.get_mut(),#wire,__payload.take(),control)?;Ok(#c::DslValue::Object(__wrapper.take()))},
     (Some(tag),content,payload)=>{
      let count=if payload.is_some(){2usize}else{1};let payload_push=match(payload,content){
       (None,_)=>quote!{},
       (Some(payload),Some(content))=>quote!{let __payload=#c::DslValue::guard_encoded((||->::core::result::Result<#c::DslValue,#c::ValueError>{#payload})()?);#c::DslValue::push_encoding_controlled(__wrapper.get_mut(),#content,__payload.take(),control)?;},
       (Some(payload),None)=>quote!{let __payload=#c::DslValue::guard_encoded((||->::core::result::Result<#c::DslValue,#c::ValueError>{#payload})()?);if matches!(__payload.get(),#c::DslValue::Object(_)){#c::DslValue::flatten_encoding_controlled(__wrapper.get_mut(),__payload.take(),control)?;}else{#c::DslValue::push_encoding_controlled(__wrapper.get_mut(),"value",__payload.take(),control)?;}}
      };
      quote!{let mut __wrapper=#c::DslValue::object_encoding_controlled(#count,control)?;let __tag=control.copy_text(#wire).map(#c::DslValue::String)?;#c::DslValue::push_encoding_controlled(__wrapper.get_mut(),#tag,__tag,control)?;#payload_push Ok(#c::DslValue::Object(__wrapper.take()))}
     }
    };
    arms.push(quote!{#pattern=>{#body}});
   }
   Ok(quote!{match self{#(#arms),*}})
  }
  Data::Union(_)=>Err(syn::Error::new_spanned(input,"controlled output unions unsupported"))
 }
}
