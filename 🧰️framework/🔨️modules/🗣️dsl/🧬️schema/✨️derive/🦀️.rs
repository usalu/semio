//! 🧬️ Generic record, scalar and tagged-variant compilation under explicit owned composition.
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

/// 🧬️ Derives the complete generic record construction contract.
#[proc_macro_derive(DslRecord, attributes(dsl))]
pub fn derive_record(input: TokenStream) -> TokenStream { emit_record(parse_macro_input!(input as DeriveInput)).into() }

/// 🏷️ Derives a generic unit-variant scalar field.
#[proc_macro_derive(DslScalar, attributes(dsl))]
pub fn derive_scalar(input: TokenStream) -> TokenStream { emit_scalar(parse_macro_input!(input as DeriveInput)).into() }

/// 🪆️ Derives generic tagged variants without product operation traits.
#[proc_macro_derive(DslEnum, attributes(dsl))]
pub fn derive_enum(input: TokenStream) -> TokenStream { emit_enum(parse_macro_input!(input as DeriveInput)).into() }

/// 🧩️ Composes complete generic record bindings into a higher declaration.
#[proc_macro]
pub fn record_binding(input: TokenStream) -> TokenStream { emit_record(parse_macro_input!(input as DeriveInput)).into() }

/// 📑️ Composes only the caller-named ordinary record projection members.
#[proc_macro]
pub fn record_projection(input: TokenStream) -> TokenStream { let input = parse_macro_input!(input as ProjectionInput); emit_projection(input.declaration, input.spec, input.to, input.from).into() }

/// 🌿️ Composes generic tagged variant bindings into a higher declaration.
#[proc_macro]
pub fn variant_binding(input: TokenStream) -> TokenStream { emit_enum(parse_macro_input!(input as DeriveInput)).into() }

struct ProjectionInput { spec: syn::Ident, to: syn::Ident, from: syn::Ident, declaration: DeriveInput }
impl syn::parse::Parse for ProjectionInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let spec = input.parse()?; input.parse::<syn::Token![,]>()?;
        let to = input.parse()?; input.parse::<syn::Token![,]>()?;
        let from = input.parse()?; input.parse::<syn::Token![;]>()?;
        Ok(Self { spec, to, from, declaration: input.parse()? })
    }
}

#[derive(Default, Clone)]
struct ContainerAttrs {
    keyword: Option<String>,
    lines_layout: bool,
    retire_with: Option<syn::Path>,
}

#[derive(Default, Clone)]
struct FieldAttrs {
    key: Option<String>,
    positional: bool,
    list: bool,
    tuple: bool,
    statements: bool,
    block: bool,
    base64: bool,
    flatten: bool,
    table: bool,
    /// `#[dsl(unit = "GPa")]` — a scalar `f64`/`f32` field prints/parses as `Shape::Quantity`
    /// (glued unit suffix) instead of plain `Shape::Float`.
    unit: Option<String>,
    /// `#[dsl(angle = "deg")]` — same mechanism as `unit`, `Shape::Angle` instead.
    angle: Option<String>,
    /// `#[dsl(refs = "material")]` — a scalar `String`/`Option<String>` field prints/parses as
    /// `Shape::Ref(kind)` instead of plain `Shape::Text`.
    refs: Option<String>,
    /// `#[dsl(defines = "material")]` — the anchor side of `refs`: this field's `FieldSpec.defines`
    /// is set so `LanguageService::validate` knows which field, in a record of this kind, other
    /// records' `Shape::Ref("material")` fields are expected to resolve against.
    defines: Option<String>,
    /// `#[dsl(lang = "jack")]` — a scalar `String` field prints/parses as `Shape::Embed(lang)`
    /// (fenced verbatim in Document mode) instead of plain `Shape::Text`.
    lang: Option<String>,
    /// `#[dsl(lang_from = "language_id")]` — fence language from a sibling Text field at print/parse time.
    lang_from: Option<String>,
    /// `#[dsl(coord)]` — a `[f64; 3]` (or any `DslField` array) field prints/parses as
    /// `Shape::Coord(3)` (`@x,y,z`) instead of a bare comma tuple.
    coord: bool,
    /// `#[dsl(dir)]` — same mechanism as `coord`, `Shape::Dir` (`^x,y,z`) instead.
    dir: bool,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn parse_container_attrs(input: &DeriveInput) -> ContainerAttrs {
    let mut out = ContainerAttrs::default();
    for attr in &input.attrs {
        if !attr.path().is_ident("dsl") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("keyword") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.keyword = Some(value.value());
            } else if meta.path.is_ident("layout") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.lines_layout = value.value() == "lines";
            } else if meta.path.is_ident("retire_with") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.retire_with = Some(value.parse()?);
            }
            Ok(())
        });
    }
    out
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn parse_field_attrs(attrs: &[syn::Attribute]) -> FieldAttrs {
    let mut out = FieldAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("dsl") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("key") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.key = Some(value.value());
            } else if meta.path.is_ident("positional") {
                out.positional = true;
            } else if meta.path.is_ident("list") {
                out.list = true;
            } else if meta.path.is_ident("tuple") {
                out.tuple = true;
            } else if meta.path.is_ident("statements") {
                out.statements = true;
            } else if meta.path.is_ident("block") {
                out.block = true;
            } else if meta.path.is_ident("base64") {
                out.base64 = true;
            } else if meta.path.is_ident("flatten") {
                out.flatten = true;
            } else if meta.path.is_ident("table") {
                out.table = true;
            } else if meta.path.is_ident("unit") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.unit = Some(value.value());
            } else if meta.path.is_ident("angle") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.angle = Some(value.value());
            } else if meta.path.is_ident("refs") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.refs = Some(value.value());
            } else if meta.path.is_ident("defines") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.defines = Some(value.value());
            } else if meta.path.is_ident("lang") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.lang = Some(value.value());
            } else if meta.path.is_ident("lang_from") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.lang_from = Some(value.value());
            } else if meta.path.is_ident("coord") {
                out.coord = true;
            } else if meta.path.is_ident("dir") {
                out.dir = true;
            }
            Ok(())
        });
    }
    out
}
//#endregion 🔖️Attrs

//#region 🔖️TypeShape
enum FieldKind {
    Scalar,
    OptionScalar(Box<Type>),
    VecList(Box<Type>),
    VecTuple(Box<Type>),
    VecStatements(Box<Type>),
    /// `#[dsl(statements, block)]` — same tagged-variant collection as `VecStatements`, but wrapped
    /// in `{ ... }` so it can sit anywhere in field order (not just as an unbounded trailing field).
    VecBlockStatements(Box<Type>),
    /// `BTreeMap<String, V>` — `V` must itself implement `DslField`; keys print sorted.
    MapField(Box<Type>),
    /// `#[dsl(statements)] Option<T>` — a "sum type" scalar field (`fill: Option<FillStyle>`,
    /// exactly one of several keyword-tagged variants, or none) rather than a collection. Reuses
    /// `Shape::Statements`/`DslVariants` at 0-or-1 length instead of a new shape: a record isn't
    /// allowed more than one *bare* `Statements` field, but two `Option<T>` fields of this kind can
    /// coexist because each is dispatched by its own field key (always paired with `#[dsl(block)]`
    /// in practice, since an un-blocked one would hit that same one-per-record limit).
    OptionStatements(Box<Type>),
    /// 🏷️ `#[dsl(statements)] Box<T>` — exactly one required tagged value (`layer:
    /// Box<DrawLayerNode>` on an `AddLayer` operation), the non-optional counterpart of
    /// `OptionStatements`: same `Shape::Statements` reuse, but errors if the count isn't exactly 1
    /// rather than treating 0 as `None`.
    RequiredStatements(Box<Type>),
    RequiredInlineStatements(Box<Type>),
    Bytes64,
    /// `#[dsl(table)] Vec<T>` (`T: DslRecord`) — Structure-of-Arrays columnar `Shape::Table`.
    /// `to_value`/`from_value` are identical to `VecList` (both produce `FieldValue::List(Vec<
    /// FieldValue::Record>)`) — only the `Shape` differs, so every binder/diff path downstream
    /// keeps working unchanged.
    VecTable(Box<Type>),
}

/// 🪆️ Strips `macro_rules!`-introduced invisible-delimiter `Type::Group` wrappers so a type
/// captured through a `:ty` metavariable — then re-emitted through another technology-local
/// declarative macro (e.g. an `entity_input!`-style struct-generating macro) before ever reaching
/// this derive — still structurally matches `Type::Path` here exactly like directly-written source.
/// Without this, `Option<T>`/`Vec<T>`/`Box<T>`/`BTreeMap<..>` fields declared through such a wrapping
/// macro silently fall through to plain `FieldKind::Scalar` instead of being classified as
/// optional/list/map, since the wrapper hides the outer `Path` segment from a bare `matches!`.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn strip_groups(ty: &Type) -> &Type {
    let mut ty = ty;
    while let Type::Group(group) = ty {
        ty = &group.elem;
    }
    ty
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn inner_of(ty: &Type, wrapper: &str) -> Option<Type> {
    let Type::Path(path) = strip_groups(ty) else { return None };
    let segment = path.path.segments.last()?;
    if segment.ident != wrapper {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else { return None };
    args.args.iter().find_map(|arg| match arg {
        syn::GenericArgument::Type(t) => Some(t.clone()),
        _ => None,
    })
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn is_vec_u8(ty: &Type) -> bool {
    inner_of(ty, "Vec").is_some_and(|inner| matches!(strip_groups(&inner), Type::Path(p) if p.path.is_ident("u8")))
}

/// 🗺️ Extracts `V` from `BTreeMap<String, V>` — `None` for any other type, including a
/// `BTreeMap` keyed by something other than `String` (the engine's `Shape::Map` is string-keyed
/// only, matching every hand-rolled `{ key=value }` grammar it replaces).
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn btreemap_string_value(ty: &Type) -> Option<Type> {
    let Type::Path(path) = strip_groups(ty) else { return None };
    let segment = path.path.segments.last()?;
    if segment.ident != "BTreeMap" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else { return None };
    let types: Vec<&Type> = args
        .args
        .iter()
        .filter_map(|arg| match arg {
            syn::GenericArgument::Type(t) => Some(t),
            _ => None,
        })
        .collect();
    let [key, value] = types.as_slice() else { return None };
    matches!(strip_groups(key), Type::Path(p) if p.path.is_ident("String")).then(|| (*value).clone())
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn classify_field(ty: &Type, attrs: &FieldAttrs) -> (FieldKind, Type) {
    if let Some(inner) = inner_of(ty, "Option") {
        if attrs.statements {
            return (FieldKind::OptionStatements(Box::new(inner.clone())), inner);
        }
        return (FieldKind::OptionScalar(Box::new(inner.clone())), inner);
    }
    if attrs.base64 && is_vec_u8(ty) {
        return (FieldKind::Bytes64, ty.clone());
    }
    if attrs.statements {
        if let Some(inner) = inner_of(ty, "Box") {
            return (FieldKind::RequiredStatements(Box::new(inner.clone())), inner);
        }
    }
    if let Some(value_ty) = btreemap_string_value(ty) {
        return (FieldKind::MapField(Box::new(value_ty.clone())), value_ty);
    }
    if let Some(inner) = inner_of(ty, "Vec").or_else(|| inner_of(ty, "PagedList")) {
        if attrs.statements {
            let kind = if attrs.block { FieldKind::VecBlockStatements(Box::new(inner.clone())) } else { FieldKind::VecStatements(Box::new(inner.clone())) };
            return (kind, inner);
        }
        if attrs.tuple {
            return (FieldKind::VecTuple(Box::new(inner.clone())), inner);
        }
        if attrs.table {
            return (FieldKind::VecTable(Box::new(inner.clone())), inner);
        }
        return (FieldKind::VecList(Box::new(inner.clone())), inner);
    }
    if attrs.statements { return (FieldKind::RequiredInlineStatements(Box::new(ty.clone())), ty.clone()); }
    (FieldKind::Scalar, ty.clone())
}
//#endregion 🔖️TypeShape

//#region 🔖️RecordCodegen
struct FieldPlan {
    ident: syn::Ident,
    id: u16,
    key: String,
    positional: Option<u16>,
    optional: bool,
    kind: FieldKind,
    elem_ty: Type,
    field_ty: Type,
    /// `#[dsl(block)]` on a field whose `FieldKind` doesn't already imply its own `{ }` wrapping
    /// (`VecBlockStatements` handles that itself) — wraps whatever shape that kind would otherwise
    /// produce in `Shape::Block`, e.g. a single nested `#[derive(DslRecord)]` field printed as a
    /// bare `camera { x=0 y=0 zoom=1 }` line instead of a `camera=...` attribute.
    block: bool,
    /// `#[dsl(unit = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    unit: Option<String>,
    /// `#[dsl(angle = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    angle: Option<String>,
    /// `#[dsl(refs = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    refs: Option<String>,
    /// `#[dsl(defines = "...")]` — sets `FieldSpec.defines`, independent of `Shape`.
    defines: Option<String>,
    /// `#[dsl(lang = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    lang: Option<String>,
    lang_from: Option<String>,
    /// `#[dsl(coord)]`, only meaningful for `FieldKind::Scalar`/`OptionScalar` on an array type.
    coord: bool,
    /// `#[dsl(dir)]`, ditto.
    dir: bool,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn plan_fields(fields: &Fields) -> Vec<FieldPlan> {
    let mut positional_counter: u16 = 0;
    let mut out = Vec::new();
    for (index, field) in fields.iter().enumerate() {
        let attrs = parse_field_attrs(&field.attrs);
        let ident = field.ident.clone().expect("dsl_derive only supports named fields");
        let (kind, elem_ty) = classify_field(&field.ty, &attrs);
        let key = attrs.key.clone().unwrap_or_else(|| to_kebab(&ident.to_string()));
        let optional = matches!(kind, FieldKind::OptionScalar(_) | FieldKind::OptionStatements(_));
        let positional = if attrs.positional {
            let p = positional_counter;
            positional_counter += 1;
            Some(p)
        } else {
            None
        };
        let block = attrs.block && !matches!(kind, FieldKind::VecBlockStatements(_));
        out.push(FieldPlan {
            ident,
            id: index as u16,
            key,
            positional,
            optional,
            kind,
            elem_ty,
            field_ty: field.ty.clone(),
            block,
            unit: attrs.unit.clone(),
            angle: attrs.angle.clone(),
            refs: attrs.refs.clone(),
            defines: attrs.defines.clone(),
            lang: attrs.lang.clone(),
            lang_from: attrs.lang_from.clone(),
            coord: attrs.coord,
            dir: attrs.dir,
        });
    }
    out
}

/// 🏗️ Builds the shared generic record and variant field fragments.
/// bodies: the `RecordSpec` field-spec expressions, the struct→`RecordValue` conversion, and the
/// `RecordValue`→struct conversion.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn record_codegen(fields: &Fields) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<syn::Ident>) {
    let plans = plan_fields(fields);
    let mut spec_exprs = Vec::new();
    let mut to_value_stmts = Vec::new();
    let mut from_value_stmts = Vec::new();
    let mut field_idents = Vec::new();

    for plan in &plans {
        let FieldPlan { ident, id, key, positional, optional, kind, elem_ty, field_ty, block, unit, angle, refs, defines, lang, lang_from, coord, dir } = plan;
        // A `#[dsl(unit = "...")]`/`#[dsl(angle = "...")]` scalar field's Shape is resolved at
        // spec-build time via `dsl::__rt::unit_for_derive` — same lazy-per-call pattern every other
        // `fn() -> RecordSpec`-backed Shape in this engine already uses, so an unknown unit symbol
        // surfaces as a panic the first time the generated spec runs (caught by that app's own
        // RecordSpec-law tests), never silently.
        let quantity_shape_override: Option<proc_macro2::TokenStream> = if let Some(symbol) = unit {
            Some(quote! { ::semio_framework_dsl_record::Shape::Quantity(::semio_framework_dsl_record::__rt::unit_for_derive(#symbol)) })
        } else if let Some(symbol) = angle {
            Some(quote! { ::semio_framework_dsl_record::Shape::Angle(::semio_framework_dsl_record::__rt::unit_for_derive(#symbol)) })
        } else if let Some(kind) = refs {
            Some(quote! { ::semio_framework_dsl_record::Shape::Ref(#kind) })
        } else if let Some(from) = lang_from {
            let embed_lang_key = plans.iter().find(|p| p.ident == from.as_str()).map_or_else(|| to_kebab(from), |p| p.key.clone());
            Some(quote! { ::semio_framework_dsl_record::Shape::EmbedFrom(#embed_lang_key) })
        } else if let Some(l) = lang {
            Some(quote! { ::semio_framework_dsl_record::Shape::Embed(#l) })
        } else if *coord {
            Some(quote! { ::semio_framework_dsl_record::Shape::Coord(3) })
        } else if *dir {
            Some(quote! { ::semio_framework_dsl_record::Shape::Dir })
        } else {
            None
        };
        let defines_expr = match defines {
            Some(kind) => quote! { .defines(#kind) },
            None => quote! {},
        };
        field_idents.push(ident.clone());
        let pos_expr = match positional {
            Some(p) => quote! { .positional(#p as u8) },
            None => quote! {},
        };
        let opt_expr = if *optional {
            quote! { .optional() }
        } else {
            quote! {}
        };

        // `DslField::shape`/`DslVariants::variants` are E4 (sync, fn-pointer transitivity — see
        // R9), so `shape_expr` never needs `.await`. `DslField::to_value`/`from_value` and
        // `DslVariants::to_named_record`/`from_named_record` stay `async`, so every value-level
        // call below is `.await`ed; a `Vec`/`Map` field can't `.await` per-element inside
        // `Iterator::map` (R10 residue shape 1), so those go through a sequential loop instead of
        // `.map().collect()`.
        let (shape_expr, to_value_expr, from_value_expr): (proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream) = match kind {
            FieldKind::Scalar => (
                quantity_shape_override.clone().unwrap_or_else(|| quote! { <#elem_ty as ::semio_framework_dsl_record::DslField>::shape() }),
                quote! { ::semio_framework_dsl_record::DslField::to_value(&self.#ident) },
                quote! { <#elem_ty as ::semio_framework_dsl_record::DslField>::from_value(value).map_err(|message|::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,message,::semio_framework_dsl_record::TextSpan::at(1,1)))? },
            ),
            FieldKind::Bytes64 => (
                quote! { ::semio_framework_dsl_record::Shape::Bytes64 },
                quote! { ::semio_framework_dsl_record::FieldValue::Bytes64(self.#ident.clone()) },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Bytes64(bytes) => bytes.clone(),
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Bytes64, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::OptionScalar(inner) => (
                quantity_shape_override.clone().unwrap_or_else(|| quote! { <#inner as ::semio_framework_dsl_record::DslField>::shape() }),
                quote! {
                    match &self.#ident {
                        Some(v) => ::semio_framework_dsl_record::DslField::to_value(v),
                        None => ::semio_framework_dsl_record::FieldValue::Absent,
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Absent => None,
                        other => Some(<#inner as ::semio_framework_dsl_record::DslField>::from_value(other).map_err(|message|::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,message,::semio_framework_dsl_record::TextSpan::at(1,1)))?),
                    }
                },
            ),
            FieldKind::VecList(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::List(Box::new(<#inner as ::semio_framework_dsl_record::DslField>::shape())) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::semio_framework_dsl_record::DslField::to_value(v)); }
                        ::semio_framework_dsl_record::FieldValue::List(__items)
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::List(items) => {
                            <#field_ty as ::semio_framework_dsl_record::DslSequence<#inner>>::from_decoded(items.iter().map(|v| <#inner as ::semio_framework_dsl_record::DslField>::from_value(v).map_err(|message|::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,message,::semio_framework_dsl_record::TextSpan::at(1,1)))))?
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected List, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            // Same `to_value`/`from_value` as `VecList` (both produce `FieldValue::List(Record)`)
            // — only the `Shape` differs (`Table` vs `List(Record)`), which is what makes the
            // printer emit compact SoA instead of verbose AoS for this field.
            FieldKind::VecTable(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Table(<#inner>::__dsl_spec_producer()) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::semio_framework_dsl_record::DslField::to_value(v)); }
                        ::semio_framework_dsl_record::FieldValue::List(__items)
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::List(items) => {
                            <#field_ty as ::semio_framework_dsl_record::DslSequence<#inner>>::from_decoded(items.iter().map(|v| <#inner as ::semio_framework_dsl_record::DslField>::from_value(v).map_err(|message|::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,message,::semio_framework_dsl_record::TextSpan::at(1,1)))))?
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected List, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::VecTuple(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Tuple(Box::new(<#inner as ::semio_framework_dsl_record::DslField>::shape()), None) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::semio_framework_dsl_record::DslField::to_value(v)); }
                        ::semio_framework_dsl_record::FieldValue::Tuple(__items)
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Tuple(items) => {
                            <#field_ty as ::semio_framework_dsl_record::DslSequence<#inner>>::from_decoded(items.iter().map(|v| <#inner as ::semio_framework_dsl_record::DslField>::from_value(v).map_err(|message|::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,message,::semio_framework_dsl_record::TextSpan::at(1,1)))))?
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Tuple, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::VecStatements(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants()) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::semio_framework_dsl_record::DslVariants::to_named_record(v)); }
                        ::semio_framework_dsl_record::FieldValue::Statements(__items)
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Statements(items) => {
                            <#field_ty as ::semio_framework_dsl_record::DslSequence<#inner>>::from_decoded(items.iter().map(|(keyword, record)| <#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record(keyword, record)))?
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Statements, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::VecBlockStatements(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Block(Box::new(::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants()))) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::semio_framework_dsl_record::DslVariants::to_named_record(v)); }
                        ::semio_framework_dsl_record::FieldValue::Block(Box::new(::semio_framework_dsl_record::FieldValue::Statements(__items)))
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Block(inner_value) => match inner_value.as_ref() {
                            ::semio_framework_dsl_record::FieldValue::Statements(items) => {
                                <#field_ty as ::semio_framework_dsl_record::DslSequence<#inner>>::from_decoded(items.iter().map(|(keyword, record)| <#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record(keyword, record)))?
                            }
                            other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Statements inside Block, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                        },
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Block, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::MapField(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Map(Box::new(<#inner as ::semio_framework_dsl_record::DslField>::shape())) },
                quote! {
                    {
                        let mut __entries = Vec::with_capacity(self.#ident.len());
                        for (k, v) in self.#ident.iter() { __entries.push((k.clone(), ::semio_framework_dsl_record::DslField::to_value(v))); }
                        ::semio_framework_dsl_record::FieldValue::Map(__entries)
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Map(entries) => {
                            let mut __out = ::std::collections::BTreeMap::new();
                            for (k, v) in entries.iter() { __out.insert(k.clone(), <#inner as ::semio_framework_dsl_record::DslField>::from_value(v).map_err(|message|::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,message,::semio_framework_dsl_record::TextSpan::at(1,1)))?); }
                            __out
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Map, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::OptionStatements(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants()) },
                quote! {
                    ::semio_framework_dsl_record::FieldValue::Statements(match &self.#ident {
                        Some(v) => vec![::semio_framework_dsl_record::DslVariants::to_named_record(v)],
                        None => vec![],
                    })
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Absent => None,
                        ::semio_framework_dsl_record::FieldValue::Statements(items) if items.is_empty() => None,
                        ::semio_framework_dsl_record::FieldValue::Statements(items) if items.len() == 1 => {
                            Some(<#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record(&items[0].0, &items[0].1)?)
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected 0 or 1 tagged values, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::RequiredStatements(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants()) },
                quote! { ::semio_framework_dsl_record::FieldValue::Statements(vec![::semio_framework_dsl_record::DslVariants::to_named_record(self.#ident.as_ref())]) },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Statements(items) if items.len() == 1 => {
                            Box::new(<#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record(&items[0].0, &items[0].1)?)
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected exactly 1 tagged value, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::RequiredInlineStatements(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants()) },
                quote! { ::semio_framework_dsl_record::FieldValue::Statements(vec![::semio_framework_dsl_record::DslVariants::to_named_record(&self.#ident)]) },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Statements(items) if items.len() == 1 => {
                            <#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record(&items[0].0, &items[0].1)?
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected exactly 1 tagged value, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
        };

        // `#[dsl(block)]` on a field whose own `FieldKind` doesn't already imply `{ }` wrapping
        // (`VecBlockStatements` does that itself) — generically wraps whatever shape the match
        // above produced, e.g. turning a nested `#[derive(DslRecord)]` scalar field into a bare
        // `camera { x=0 y=0 zoom=1 }` line instead of a `camera=...` attribute.
        //
        // `FieldValue::Absent` (an `Option<T>` field's `None`) is deliberately NOT wrapped: an
        // empty `stroke { }` would reparse as "a record whose every field is absent", not "no
        // record at all" — `StrokeStyle`'s own non-optional fields would then fail with "expected
        // a 4-item Tuple, found Absent" instead of the field itself just being omitted, exactly
        // like an ordinary (non-block) optional field already is.
        let (shape_expr, to_value_expr, from_value_expr) = if *block {
            (
                quote! { ::semio_framework_dsl_record::Shape::Block(Box::new(#shape_expr)) },
                quote! {
                    match #to_value_expr {
                        ::semio_framework_dsl_record::FieldValue::Absent => ::semio_framework_dsl_record::FieldValue::Absent,
                        other => ::semio_framework_dsl_record::FieldValue::Block(Box::new(other)),
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Block(inner) => { let value = inner.as_ref(); #from_value_expr },
                        ::semio_framework_dsl_record::FieldValue::Absent => { let value = &::semio_framework_dsl_record::FieldValue::Absent; #from_value_expr },
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Block, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            )
        } else {
            (shape_expr, to_value_expr, from_value_expr)
        };

        spec_exprs.push(quote! {
            ::semio_framework_dsl_record::FieldSpec::new(#id, #key, #shape_expr) #pos_expr #opt_expr #defines_expr
        });
        to_value_stmts.push(quote! {
            record.fields.insert(#id, #to_value_expr);
        });
        let local = field_local(ident);
        from_value_stmts.push(quote! {
            let #local = {
                let value = record.get(#id).ok_or_else(|| ::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("missing field '{}'", #key),::semio_framework_dsl_record::TextSpan::at(1,1)))?;
                #from_value_expr
            };
        });
    }

    (spec_exprs, to_value_stmts, from_value_stmts, field_idents)
}

/// 🏭️ Emits declared field metadata without borrowing an ordinary allocating shape factory.
fn schema_record_codegen(fields:&Fields)->Vec<proc_macro2::TokenStream>{
    let plans=plan_fields(fields);
    plans.iter().map(|plan|{
        let FieldPlan{id,key,positional,optional,kind,elem_ty,block,unit,angle,refs,defines,lang,lang_from,coord,dir,..}=plan;
        let refinement=if let Some(symbol)=unit{Some(quote!{::semio_framework_dsl_record::Shape::Quantity(::semio_framework_dsl_record::__rt::unit_for_derive(#symbol))})}else if let Some(symbol)=angle{Some(quote!{::semio_framework_dsl_record::Shape::Angle(::semio_framework_dsl_record::__rt::unit_for_derive(#symbol))})}else if let Some(kind)=refs{Some(quote!{::semio_framework_dsl_record::Shape::Ref(#kind)})}else if let Some(from)=lang_from{let key=plans.iter().find(|plan|plan.ident==from.as_str()).map_or_else(||to_kebab(from),|plan|plan.key.clone());Some(quote!{::semio_framework_dsl_record::Shape::EmbedFrom(#key)})}else if let Some(language)=lang{Some(quote!{::semio_framework_dsl_record::Shape::Embed(#language)})}else if *coord{Some(quote!{::semio_framework_dsl_record::Shape::Coord(3)})}else if *dir{Some(quote!{::semio_framework_dsl_record::Shape::Dir})}else{None};
        let shape=match kind{
            FieldKind::Scalar=>refinement.unwrap_or_else(||quote!{<#elem_ty as ::semio_framework_dsl_record::DslField>::shape_controlled(control)?}),
            FieldKind::OptionScalar(inner)=>refinement.unwrap_or_else(||quote!{<#inner as ::semio_framework_dsl_record::DslField>::shape_controlled(control)?}),
            FieldKind::Bytes64=>quote!{::semio_framework_dsl_record::Shape::Bytes64},
            FieldKind::VecList(inner)=>quote!{::semio_framework_dsl_record::Shape::List(::semio_framework_dsl_record::producer::boxed(<#inner as ::semio_framework_dsl_record::DslField>::shape_controlled(control)?,control)?)},
            FieldKind::VecTuple(inner)=>quote!{::semio_framework_dsl_record::Shape::Tuple(::semio_framework_dsl_record::producer::boxed(<#inner as ::semio_framework_dsl_record::DslField>::shape_controlled(control)?,control)?,None)},
            FieldKind::VecTable(inner)=>quote!{::semio_framework_dsl_record::Shape::Table(<#inner>::__dsl_spec_producer())},
            FieldKind::MapField(inner)=>quote!{::semio_framework_dsl_record::Shape::Map(::semio_framework_dsl_record::producer::boxed(<#inner as ::semio_framework_dsl_record::DslField>::shape_controlled(control)?,control)?)},
            FieldKind::VecStatements(inner)|FieldKind::OptionStatements(inner)|FieldKind::RequiredStatements(inner)|FieldKind::RequiredInlineStatements(inner)=>quote!{::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants_controlled(control)?)},
            FieldKind::VecBlockStatements(inner)=>quote!{::semio_framework_dsl_record::Shape::Block(::semio_framework_dsl_record::producer::boxed(::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants_controlled(control)?),control)?)},
        };
        let shape=if *block{quote!{::semio_framework_dsl_record::Shape::Block(::semio_framework_dsl_record::producer::boxed(#shape,control)? )}}else{shape};
        let position=positional.map(|position|quote!{.positional(#position as u8)});let optional=if *optional{quote!{.optional()}}else{quote!{}};let defines=defines.as_ref().map(|kind|quote!{.defines(#kind)});
        quote!{let field=::semio_framework_dsl_record::producer::field(#id,#key,#shape,control)? #position #optional #defines;fields.push(field);control.step()?;}
    }).collect()
}
//#endregion 🔖️RecordCodegen

/// 🛬️ Generates explicit controlled bindings from the same authored field plans.
fn controlled_record_codegen(fields:&Fields)->Vec<proc_macro2::TokenStream>{
    plan_fields(fields).iter().map(|plan|{
        let FieldPlan{ident,id,key,kind,elem_ty,field_ty,block,..}=plan;
        let expression=match kind {
            FieldKind::Scalar=>quote!{control.scoped_stage(|control|<#elem_ty as ::semio_framework_dsl_record::DslField>::from_value_controlled(value,control))?},
            FieldKind::Bytes64=>quote!{match value{::semio_framework_dsl_record::FieldValue::Bytes64(bytes)=>{control.step()?;control.copy_bytes(bytes)?},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Bytes64"))}},
            FieldKind::OptionScalar(inner)=>quote!{match value{::semio_framework_dsl_record::FieldValue::Absent=>{control.step()?;None},other=>Some(control.scoped_stage(|control|<#inner as ::semio_framework_dsl_record::DslField>::from_value_controlled(other,control))?)}},
            FieldKind::VecList(inner)|FieldKind::VecTable(inner)=>quote!{match value{::semio_framework_dsl_record::FieldValue::List(items)=>::semio_framework_dsl_record::__rt::decode_list_controlled::<#inner,#field_ty>(items,control)?,_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected List"))}},
            FieldKind::VecTuple(inner)=>quote!{match value{::semio_framework_dsl_record::FieldValue::Tuple(items)=>::semio_framework_dsl_record::__rt::decode_list_controlled::<#inner,#field_ty>(items,control)?,_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Tuple"))}},
            FieldKind::MapField(inner)=>quote!{control.scoped_stage(|control|<::std::collections::BTreeMap<String,#inner> as ::semio_framework_dsl_record::DslField>::from_value_controlled(value,control))?},
            FieldKind::VecStatements(inner)=>controlled_statements(inner,field_ty),
            FieldKind::VecBlockStatements(inner)=>{let expression=controlled_statements(inner,field_ty);quote!{match value{::semio_framework_dsl_record::FieldValue::Block(inner)=>{let value=inner.as_ref();#expression},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Block"))}}},
            FieldKind::OptionStatements(inner)=>quote!{match value{::semio_framework_dsl_record::FieldValue::Absent=>None,::semio_framework_dsl_record::FieldValue::Statements(items) if items.is_empty()=>None,::semio_framework_dsl_record::FieldValue::Statements(items) if items.len()==1=>Some(control.scoped_stage(|control|<#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record_controlled(&items[0].0,&items[0].1,control))?),_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected0or1 tagged values"))}},
            FieldKind::RequiredStatements(inner)=>quote!{match value{::semio_framework_dsl_record::FieldValue::Statements(items) if items.len()==1=>{control.charge(::std::mem::size_of::<#inner>())?;Box::new(control.scoped_stage(|control|<#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record_controlled(&items[0].0,&items[0].1,control))?)},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected exactly1 tagged value"))}},
            FieldKind::RequiredInlineStatements(inner)=>quote!{match value{::semio_framework_dsl_record::FieldValue::Statements(items) if items.len()==1=>{control.scoped_stage(|control|<#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record_controlled(&items[0].0,&items[0].1,control))?},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected exactly1 tagged value"))}},
        };
        let expression=if *block {quote!{match value{::semio_framework_dsl_record::FieldValue::Block(inner)=>{let value=inner.as_ref();#expression},::semio_framework_dsl_record::FieldValue::Absent=>{let value=&::semio_framework_dsl_record::FieldValue::Absent;#expression},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Block"))}}}else{expression};
        let retire=retire_field_codegen(plan,quote!{value});
        let ident=field_local(ident);
        quote!{let #ident=::semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.scoped_stage(|control|{let value=record.get(#id).ok_or_else(||::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("missing field '{}'",#key)))?;Ok::<_,::semio_framework_value::ValueError>(#expression)}).map_err(|error|error.under(#key))?,|value|{#retire});}
    }).collect()
}

/// 🛫️ Projects explicit declared fields with cumulative ownership and known workloads.
fn encoding_record_codegen(fields:&Fields,bindings:bool)->Vec<proc_macro2::TokenStream>{
    plan_fields(fields).iter().map(|plan|{
        let FieldPlan{ident,id,key,kind,elem_ty,block,..}=plan;
        let source=if bindings{let local=field_local(ident);quote!{#local}}else{quote!{&self.#ident}};
        let projection=match kind{
            FieldKind::Scalar=>quote!{<#elem_ty as ::semio_framework_dsl_record::DslField>::to_value_controlled(value,control)?},
            FieldKind::Bytes64=>quote!{::semio_framework_dsl_record::FieldValue::Bytes64(control.copy_bytes(value)?)},
            FieldKind::OptionScalar(inner)=>quote!{match value{Some(value)=><#inner as ::semio_framework_dsl_record::DslField>::to_value_controlled(value,control)?,None=>::semio_framework_dsl_record::FieldValue::Absent}},
            FieldKind::VecList(_)|FieldKind::VecTable(_)=>quote!{::semio_framework_dsl_record::FieldValue::List(::semio_framework_dsl_record::native_encoding::project_list(value,control)?)},
            FieldKind::VecTuple(_)=>quote!{::semio_framework_dsl_record::FieldValue::Tuple(::semio_framework_dsl_record::native_encoding::project_list(value,control)?)},
            FieldKind::MapField(_)=>quote!{::semio_framework_dsl_record::native_encoding::project_map(value,control)?},
            FieldKind::VecStatements(_)=>quote!{::semio_framework_dsl_record::native_encoding::project_statements(value,control)?},
            FieldKind::VecBlockStatements(_)=>quote!{{control.charge(::std::mem::size_of::<::semio_framework_dsl_record::FieldValue>())?;::semio_framework_dsl_record::FieldValue::Block(Box::new(::semio_framework_dsl_record::native_encoding::project_statements(value,control)?))}},
            FieldKind::OptionStatements(_)=>quote!{match value{Some(value)=>::semio_framework_dsl_record::native_encoding::project_statements(::std::slice::from_ref(value),control)?,None=>::semio_framework_dsl_record::FieldValue::Statements(Vec::new())}},
            FieldKind::RequiredStatements(_)=>quote!{::semio_framework_dsl_record::native_encoding::project_statements(::std::slice::from_ref(value.as_ref()),control)?},
            FieldKind::RequiredInlineStatements(_)=>quote!{::semio_framework_dsl_record::native_encoding::project_statements(::std::slice::from_ref(value),control)?},
        };
        let projection=if *block{quote!{match #projection{::semio_framework_dsl_record::FieldValue::Absent=>::semio_framework_dsl_record::FieldValue::Absent,value=>{let value=::semio_framework_dsl_record::__rt::DecodedFieldOwner::new(value,::semio_framework_dsl_record::native_encoding::retire_field);control.charge(::std::mem::size_of::<::semio_framework_dsl_record::FieldValue>())?;::semio_framework_dsl_record::FieldValue::Block(Box::new(value.take()))}}}}else{projection};
        quote!{let field=control.scoped_stage(|control|{control.begin_stage(0)?;let value=#source;Ok::<_,::semio_framework_value::ValueError>(#projection)}).map_err(|error|error.under(#key))?;record.insert(#id,field)?;control.step()?;}
    }).collect()
}

/// 🧹️ Delegates each completed field to its declared field or variant retirement.
fn retire_field_codegen(plan:&FieldPlan,value:proc_macro2::TokenStream)->proc_macro2::TokenStream{
    let FieldPlan{kind,elem_ty,..}=plan;
    match kind {
        FieldKind::Scalar=>quote!{<#elem_ty as ::semio_framework_dsl_record::DslField>::retire_decoded(#value);},
        FieldKind::Bytes64=>quote!{drop(#value);},
        FieldKind::OptionScalar(inner)=>quote!{if let Some(value)=#value{<#inner as ::semio_framework_dsl_record::DslField>::retire_decoded(value);}},
        FieldKind::VecList(inner)|FieldKind::VecTable(inner)|FieldKind::VecTuple(inner)=>quote!{for value in #value{<#inner as ::semio_framework_dsl_record::DslField>::retire_decoded(value);}},
        FieldKind::MapField(inner)=>quote!{<::std::collections::BTreeMap<String,#inner> as ::semio_framework_dsl_record::DslField>::retire_decoded(#value);},
        FieldKind::VecStatements(inner)|FieldKind::VecBlockStatements(inner)=>quote!{for value in #value{<#inner as ::semio_framework_dsl_record::DslVariants>::retire_decoded_variant(value);}},
        FieldKind::OptionStatements(inner)=>quote!{if let Some(value)=#value{<#inner as ::semio_framework_dsl_record::DslVariants>::retire_decoded_variant(value);}},
        FieldKind::RequiredStatements(inner)=>quote!{<#inner as ::semio_framework_dsl_record::DslVariants>::retire_decoded_variant(*#value);},
        FieldKind::RequiredInlineStatements(inner)=>quote!{<#inner as ::semio_framework_dsl_record::DslVariants>::retire_decoded_variant(#value);},
    }
}

/// 🌲️ Builds record retirement from explicit field plans or an authored domain lifecycle.
fn record_retirement_codegen(fields:&Fields,retire_with:Option<&syn::Path>)->proc_macro2::TokenStream{
    if let Some(owner)=retire_with{return quote!{#owner(self);};}
    let plans=plan_fields(fields);let idents=field_inits(&plans.iter().map(|plan|plan.ident.clone()).collect::<Vec<_>>());
    let retirements=plans.iter().map(|plan|{let ident=field_local(&plan.ident);retire_field_codegen(plan,quote!{#ident})});
    quote!{let Self{#(#idents),*}=self;#(#retirements)*}
}

/// 🌿️ Builds a controlled tagged-list binding without borrowing an unchecked constructor.
fn controlled_statements(inner:&Type,field_ty:&Type)->proc_macro2::TokenStream{
    quote!{match value{::semio_framework_dsl_record::FieldValue::Statements(items)=>::semio_framework_dsl_record::__rt::decode_statements_controlled::<#inner,#field_ty>(items,control)?,_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Statements"))}}
}


/// 🔎️ Reads declared source slots directly for the retained native projection cursor.
fn retained_record_codegen(fields:&Fields,from_bindings:bool)->proc_macro2::TokenStream{
    let plans=plan_fields(fields);let ids=plans.iter().map(|plan|plan.id);
    let arms=plans.iter().enumerate().map(|(index,plan)|{
        let FieldPlan{ident,kind,elem_ty,block,..}=plan;
        let value=if from_bindings{let local=field_local(ident);quote!{#local}}else{quote!{&self.#ident}};
        let projection=match kind{
            FieldKind::Scalar=>quote!{<#elem_ty as ::semio_framework_dsl_record::DslField>::projection_view(value,path)},
            FieldKind::OptionScalar(inner)=>quote!{match value{Some(value)=><#inner as ::semio_framework_dsl_record::DslField>::projection_view(value,path),None if path.is_empty()=>Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Absent),None=>Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::VecList(inner)|FieldKind::VecTable(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::List(value.len()))}else{<#inner as ::semio_framework_dsl_record::DslField>::projection_view(value.get(path[0]).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..])}},
            FieldKind::VecTuple(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Tuple(value.len()))}else{<#inner as ::semio_framework_dsl_record::DslField>::projection_view(value.get(path[0]).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..])}},
            FieldKind::Bytes64=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Bytes(value))}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::MapField(inner)=>quote!{<::std::collections::BTreeMap<String,#inner> as ::semio_framework_dsl_record::DslField>::projection_view(value,path)},
            FieldKind::VecStatements(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Statements(value.len()))}else{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_view(value.get(path[0]).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..])}},
            FieldKind::VecBlockStatements(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Block)}else if path[0]==0{let path=&path[1..];if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Statements(value.len()))}else{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_view(value.get(path[0]).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..])}}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::OptionStatements(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Statements(usize::from(value.is_some())))}else if path[0]==0{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_view(value.as_ref().ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..])}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::RequiredStatements(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Statements(1))}else if path[0]==0{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_view(value.as_ref(),&path[1..])}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::RequiredInlineStatements(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Statements(1))}else if path[0]==0{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_view(value,&path[1..])}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            _=>quote!{Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::UnsupportedOwner,"tagged source owner has no retained projection"))},
        };
        let projection=if *block{quote!{if path.is_empty(){match {#projection}?{::semio_framework_dsl_record::native_encoding::FieldProjectionView::Absent=>Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Absent),_=>Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Block)}}else if path[0]==0{let path=&path[1..];#projection}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}}}else{projection};
        quote!{#index=>{let value=#value;let path=&path[1..];#projection}}
    });
    quote!{if path.is_empty(){return Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Record(&[#(#ids),*]))}match path[0]{#(#arms,)*_=>Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}}
}

fn retained_record_key_codegen(fields:&Fields,from_bindings:bool)->proc_macro2::TokenStream{
    let plans=plan_fields(fields);let arms=plans.iter().enumerate().map(|(ordinal,plan)|{
        let FieldPlan{ident,kind,elem_ty,block,..}=plan;
        let value=if from_bindings{let local=field_local(ident);quote!{#local}}else{quote!{&self.#ident}};
        let projection=match kind{
            FieldKind::Scalar=>quote!{<#elem_ty as ::semio_framework_dsl_record::DslField>::projection_key(value,path,index)},
            FieldKind::OptionScalar(inner)=>quote!{match value{Some(value)=><#inner as ::semio_framework_dsl_record::DslField>::projection_key(value,path,index),None=>Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::VecList(inner)|FieldKind::VecTable(inner)|FieldKind::VecTuple(inner)=>quote!{let child=*path.first().ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?;<#inner as ::semio_framework_dsl_record::DslField>::projection_key(value.get(child).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..],index)},
            FieldKind::MapField(inner)=>quote!{<::std::collections::BTreeMap<String,#inner> as ::semio_framework_dsl_record::DslField>::projection_key(value,path,index)},
            FieldKind::VecStatements(inner)=>quote!{if path.is_empty(){Ok(<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_identity(value.get(index).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?).0)}else{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_key(value.get(path[0]).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..],index)}},
            FieldKind::VecBlockStatements(inner)=>quote!{if path.first()==Some(&0){let path=&path[1..];if path.is_empty(){Ok(<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_identity(value.get(index).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?).0)}else{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_key(value.get(path[0]).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..],index)}}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::OptionStatements(inner)=>quote!{let value=value.as_ref().ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?;if path.is_empty()&&index==0{Ok(<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_identity(value).0)}else if path.first()==Some(&0){<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_key(value,&path[1..],index)}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::RequiredStatements(inner)=>quote!{if path.is_empty()&&index==0{Ok(<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_identity(value.as_ref()).0)}else if path.first()==Some(&0){<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_key(value.as_ref(),&path[1..],index)}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::RequiredInlineStatements(inner)=>quote!{if path.is_empty()&&index==0{Ok(<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_identity(value).0)}else if path.first()==Some(&0){<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_key(value,&path[1..],index)}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            _=>quote!{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())},
        };
        let projection=if *block{quote!{if path.first()==Some(&0){let path=&path[1..];#projection}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}}}else{projection};
        quote!{#ordinal=>{let value=#value;let path=&path[1..];#projection}}
    });quote!{if path.is_empty(){return Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}match path[0]{#(#arms,)*_=>Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}}
}

fn emit_record(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslRecord only supports structs").to_compile_error();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let field_initializers = field_inits(&field_idents);
    let controlled_stmts=controlled_record_codegen(&data.fields);
    let schema_stmts=schema_record_codegen(&data.fields);let schema_count=schema_stmts.len();
    let encoding_stmts=encoding_record_codegen(&data.fields,false);
    let encoding_count=encoding_stmts.len();
    let retained_projection=retained_record_codegen(&data.fields,false);
    let retained_keys=retained_record_key_codegen(&data.fields,false);
    let controlled_fields=field_idents.iter().map(|ident|{let local=field_local(ident);quote!{#ident:#local.take()}});
    let retirement=record_retirement_codegen(&data.fields,container.retire_with.as_ref());
    let schema_keyword_expr=match &container.keyword{Some(keyword)=>quote!{Some(#keyword)},None=>quote!{None}};
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout {
        quote! { ::semio_framework_dsl_record::RecordLayout::Lines }
    } else {
        quote! { ::semio_framework_dsl_record::RecordLayout::Inline }
    };

    let borrowed=borrowed_record_owner_codegen(&input,true);
    let expanded = quote! {
        #borrowed
        impl #name {
            // 🚫️async: E4 — its VALUE is stored as the fn pointer in `Shape::Record(Self::__dsl_spec)`
            // below (`DslField::shape` is itself E4 for the same reason — see R9), and `spec_exprs`
            // (built from the now-sync `DslField::shape`/`DslVariants::variants`) needs no executor.
            pub fn __dsl_spec() -> ::semio_framework_dsl_record::RecordSpec {
                ::semio_framework_dsl_record::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            /// 🏭️ Owns exactly the declared metadata fields under either native control.
            pub fn __dsl_spec_controlled<C: ::semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<::semio_framework_dsl_record::RecordSpec,::semio_framework_value::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(#schema_count)?;let mut fields=control.allocate_vec(#schema_count)?;#(#schema_stmts)*::semio_framework_dsl_record::producer::record(#schema_keyword_expr,#layout_expr,fields,control)}))
            }
            /// 🪆️ Retains lazy owner factories without constructing their metadata.
            pub fn __dsl_spec_producer()->::semio_framework_dsl_record::RecordSpecProducer{::semio_framework_dsl_record::RecordSpecProducer{ordinary:Self::__dsl_spec,decoding:|control|Self::__dsl_spec_controlled(control),encoding:|control|Self::__dsl_spec_controlled(control)}}
            pub fn __dsl_to_record(&self) -> ::semio_framework_dsl_record::RecordValue {
                let mut record = ::semio_framework_dsl_record::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            /// 🛫️ Projects complete named fields under caller-owned output admission.
            pub fn __dsl_to_record_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::semio_framework_dsl_record::RecordValue,::semio_framework_value::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(#encoding_count)?;let mut record=::semio_framework_dsl_record::native_encoding::EncodedRecord::new(#encoding_count,control)?;#(#encoding_stmts)*Ok::<_,::semio_framework_value::ValueError>(record.take())}))
            }
            pub fn __dsl_from_record(record: &::semio_framework_dsl_record::RecordValue) -> Result<Self, ::semio_framework_dsl_record::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_initializers),* })
            }
            /// 🛬️ Binds owned typed fields with cumulative allocation and interior cancellation.
            pub fn __dsl_from_record_controlled(record:&::semio_framework_dsl_record::RecordValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{
                control.checkpoint()?;
                #(#controlled_stmts)*
                Ok(Self{#(#controlled_fields),*})
            }
        }

        impl ::semio_framework_dsl_record::DslField for #name {
            fn projection_view(&self,path:&[usize])->Result<::semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,::semio_framework_value::ValueError>{#retained_projection}
            fn projection_key(&self,path:&[usize],index:usize)->Result<&str,::semio_framework_value::ValueError>{#retained_keys}
            fn retire_decoded(self){#retirement}
            // 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
            fn shape() -> ::semio_framework_dsl_record::Shape {
                ::semio_framework_dsl_record::Shape::Record(Self::__dsl_spec_producer())
            }
            fn shape_controlled<C: ::semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<::semio_framework_dsl_record::Shape,::semio_framework_value::ValueError>{control.checkpoint()?;Ok(::semio_framework_dsl_record::Shape::Record(Self::__dsl_spec_producer()))}
            fn to_value(&self) -> ::semio_framework_dsl_record::FieldValue {
                ::semio_framework_dsl_record::FieldValue::Record(self.__dsl_to_record())
            }
            fn from_value(value: &::semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
                match value {
                    ::semio_framework_dsl_record::FieldValue::Record(record) => Self::__dsl_from_record(record).map_err(|e| e.message),
                    other => Err(format!("expected Record, found {other:?}")),
                }
            }
            fn from_value_controlled(value:&::semio_framework_dsl_record::FieldValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{
                match value{::semio_framework_dsl_record::FieldValue::Record(record)=>Self::__dsl_from_record_controlled(record,control),_=>Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Record"))}
            }
            fn from_record_controlled(record:&::semio_framework_dsl_record::RecordValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{Self::__dsl_from_record_controlled(record,control)}
            fn to_value_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::semio_framework_dsl_record::FieldValue,::semio_framework_value::ValueError>{Self::__dsl_to_record_controlled(self,control).map(::semio_framework_dsl_record::FieldValue::Record)}
            fn to_record_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::semio_framework_dsl_record::RecordValue,::semio_framework_value::ValueError>{Self::__dsl_to_record_controlled(self,control)}
        }
    };
    expanded.into()
}

fn emit_scalar(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslScalar only supports unit-variant enums").to_compile_error();
    };
    let mut variant_tags = Vec::new();
    let mut schema_tags=Vec::new();
    let mut match_to_ordinal = Vec::new();
    let mut match_from_ordinal = Vec::new();
    for (ordinal, variant) in data.variants.iter().enumerate() {
        if !matches!(variant.fields, Fields::Unit) {
            return syn::Error::new_spanned(variant, "DslScalar only supports unit variants").to_compile_error();
        }
        let attrs = parse_field_attrs(&variant.attrs);
        let variant_ident = variant.ident.clone();
        let tag = attrs.key.unwrap_or_else(|| to_kebab(&variant_ident.to_string()));
        let ordinal = ordinal as u32;
        variant_tags.push(quote! { (#tag.to_string(), #ordinal) });
        schema_tags.push(quote!{variants.push((control.copy_text(#tag)?,#ordinal));control.step()?;});
        match_to_ordinal.push(quote! { #name::#variant_ident => #ordinal });
        match_from_ordinal.push(quote! { #ordinal => Ok(#name::#variant_ident) });
    }

    let schema_count=schema_tags.len();
    let borrowed=borrowed_scalar_owner_codegen(&input);
    let expanded = quote! {
        #borrowed
        impl ::semio_framework_dsl_record::DslField for #name {
            // 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
            fn shape() -> ::semio_framework_dsl_record::Shape {
                ::semio_framework_dsl_record::Shape::Enum(vec![ #(#variant_tags),* ])
            }
            fn shape_controlled<C: ::semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<::semio_framework_dsl_record::Shape,::semio_framework_value::ValueError>{control.scoped_stage(|control|{control.begin_stage(#schema_count)?;let mut variants=control.allocate_vec(#schema_count)?;#(#schema_tags)*Ok(::semio_framework_dsl_record::Shape::Enum(variants))})}
            fn to_value(&self) -> ::semio_framework_dsl_record::FieldValue {
                ::semio_framework_dsl_record::FieldValue::Enum(match self { #(#match_to_ordinal),* })
            }
            fn projection_view(&self,path:&[usize])->Result<::semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,::semio_framework_value::ValueError>{if !path.is_empty(){return Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Enum(match self{#(#match_to_ordinal),*}))}
            fn from_value(value: &::semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
                match value {
                    ::semio_framework_dsl_record::FieldValue::Enum(ordinal) => match *ordinal {
                        #(#match_from_ordinal,)*
                        other => Err(format!("unknown enum ordinal {other}")),
                    },
                    other => Err(format!("expected Enum, found {other:?}")),
                }
            }
            fn from_value_controlled(value:&::semio_framework_dsl_record::FieldValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{control.step()?;match value{::semio_framework_dsl_record::FieldValue::Enum(ordinal)=>match *ordinal{#(#match_from_ordinal,)*other=>Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown enum ordinal {other}")))},_=>Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected declared Enum"))}}
            fn to_value_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::semio_framework_dsl_record::FieldValue,::semio_framework_value::ValueError>{control.step()?;Ok(::semio_framework_dsl_record::FieldValue::Enum(match self{#(#match_to_ordinal),*}))}
        }
    };
    expanded.into()
}

fn dsl_variants_codegen(name: &syn::Ident, data: &syn::DataEnum, retire_with:Option<&syn::Path>) -> proc_macro2::TokenStream {
    let mut variants_exprs = Vec::new();
    let mut schema_variant_stmts=Vec::new();
    let mut schema_methods=Vec::new();
    let mut to_named_arms = Vec::new();
    let mut encoding_arms=Vec::new();
    let mut from_named_arms = Vec::new();
    let mut controlled_arms=Vec::new();
    let mut retirement_arms=Vec::new();
    let mut variant_identity_arms=Vec::new();let mut variant_projection_arms=Vec::new();let mut variant_key_arms=Vec::new();

    for (variant_index,variant) in data.variants.iter().enumerate() {
        let attrs = parse_field_attrs(&variant.attrs);
        let variant_ident = variant.ident.clone();
        let keyword = attrs.key.clone().unwrap_or_else(|| to_kebab(&variant_ident.to_string()));
        let fields = &variant.fields;

        // A single-field tuple variant (`Shape(DrawShapeBody)`) delegates entirely to its inner
        // type's own `DslField` impl — its `RecordSpec` IS the inner type's, not a wrapper with one
        // positional field, so a body already declared with `#[derive(DslRecord)]` (its own keyword,
        // its own fields) prints/parses completely unchanged whether reached through the enum or on
        // its own.
        if let Fields::Unnamed(unnamed) = fields {
            if unnamed.unnamed.len() == 1 {
                let inner_ty = &unnamed.unnamed[0].ty;
                variants_exprs.push(quote!{(#keyword.to_string(),::semio_framework_dsl_record::__rt::newtype_variant_producer::<#inner_ty>())});
                schema_variant_stmts.push(quote!{variants.push((control.copy_text(#keyword)?,::semio_framework_dsl_record::__rt::newtype_variant_producer::<#inner_ty>()));control.step()?;});
                to_named_arms.push(quote! {
                    #name::#variant_ident(inner) => (#keyword.to_string(), ::semio_framework_dsl_record::__rt::newtype_variant_to_record(inner))
                });
                from_named_arms.push(quote! {
                    #keyword => Ok(#name::#variant_ident(::semio_framework_dsl_record::__rt::newtype_variant_from_record::<#inner_ty>(record)?))
                });
                encoding_arms.push(quote!{#name::#variant_ident(inner)=>{let keyword=control.copy_text(#keyword)?;let record=<#inner_ty as ::semio_framework_dsl_record::DslField>::to_record_controlled(inner,control)?;Ok((keyword,record))}});
                controlled_arms.push(quote!{#keyword=>Ok(#name::#variant_ident(control.scoped_stage(|control|<#inner_ty as ::semio_framework_dsl_record::DslField>::from_record_controlled(record,control))?))});
                retirement_arms.push(quote!{#name::#variant_ident(inner)=><#inner_ty as ::semio_framework_dsl_record::DslField>::retire_decoded(inner)});
                variant_identity_arms.push(quote!{#name::#variant_ident(_)=>(#keyword,#variant_index,::semio_framework_dsl_record::__rt::newtype_variant_producer::<#inner_ty>())});
                variant_projection_arms.push(quote!{#name::#variant_ident(inner)=>{if !matches!(<#inner_ty as ::semio_framework_dsl_record::DslField>::projection_view(inner,&[])?,::semio_framework_dsl_record::native_encoding::FieldProjectionView::Record(_)){return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"newtype variant requires an original Record source"));}<#inner_ty as ::semio_framework_dsl_record::DslField>::projection_view(inner,path)}});
                variant_key_arms.push(quote!{#name::#variant_ident(inner)=>{if !matches!(<#inner_ty as ::semio_framework_dsl_record::DslField>::projection_view(inner,&[])?,::semio_framework_dsl_record::native_encoding::FieldProjectionView::Record(_)){return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"newtype variant requires an original Record source"));}<#inner_ty as ::semio_framework_dsl_record::DslField>::projection_key(inner,path,index)}});

                continue;
            }
        }

        let (spec_exprs, _to_value_stmts, from_value_stmts, field_idents) = record_codegen(fields);
        let controlled_stmts=controlled_record_codegen(fields);
        let controlled_fields=field_idents.iter().map(|ident|{let local=field_local(ident);quote!{#ident:#local.take()}});

        let schema_name=quote::format_ident!("__dsl_variant_spec_{}",variant_index);let controlled_name=quote::format_ident!("__dsl_variant_spec_{}_controlled",variant_index);let producer_name=quote::format_ident!("__dsl_variant_spec_{}_producer",variant_index);
        let schema_stmts=schema_record_codegen(fields);let schema_count=schema_stmts.len();
        schema_methods.push(quote!{
            fn #schema_name()->::semio_framework_dsl_record::RecordSpec{::semio_framework_dsl_record::RecordSpec::new_owned(Some(#keyword.to_string()),::semio_framework_dsl_record::RecordLayout::Inline,vec![#(#spec_exprs),*])}
            fn #controlled_name<C: ::semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<::semio_framework_dsl_record::RecordSpec,::semio_framework_value::ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(#schema_count)?;let mut fields=control.allocate_vec(#schema_count)?;#(#schema_stmts)*::semio_framework_dsl_record::producer::record(Some(#keyword),::semio_framework_dsl_record::RecordLayout::Inline,fields,control)}))}
            fn #producer_name()->::semio_framework_dsl_record::RecordSpecProducer{::semio_framework_dsl_record::RecordSpecProducer{ordinary:Self::#schema_name,decoding:|control|Self::#controlled_name(control),encoding:|control|Self::#controlled_name(control)}}
        });
        variants_exprs.push(quote!{(#keyword.to_string(),Self::#producer_name())});
        schema_variant_stmts.push(quote!{variants.push((control.copy_text(#keyword)?,Self::#producer_name()));control.step()?;});

        // Build a per-variant to-record conversion using the field bindings from a `match` on
        // `self`, since (unlike `DslRecord`) the fields live inside an enum variant, not `self.field`.
        // A true unit variant (`Variant`, no braces at all) needs a bare match pattern — `Variant {}`
        // is only valid Rust for a variant that was itself declared with (empty) braces.
        let field_binds: Vec<proc_macro2::TokenStream> = field_idents.iter().map(|f| { let local = field_local(f); quote! { #f: #local } }).collect();
        let to_value_stmts_for_variant: Vec<proc_macro2::TokenStream> = record_codegen_to_value_from_bindings(fields);
        let is_unit = matches!(fields, Fields::Unit);
        let match_pattern = if is_unit {
            quote! { #name::#variant_ident }
        } else {
            quote! { #name::#variant_ident { #(#field_binds),* } }
        };
        let construct_expr = if is_unit {
            quote! { #name::#variant_ident }
        } else {
            let inits = field_inits(&field_idents);
            quote! { #name::#variant_ident { #(#inits),* } }
        };
        to_named_arms.push(quote! {
            #match_pattern => {
                let mut record = ::semio_framework_dsl_record::RecordValue::default();
                #(#to_value_stmts_for_variant)*
                (#keyword.to_string(), record)
            }
        });
        from_named_arms.push(quote! {
            #keyword => {
                #(#from_value_stmts)*
                Ok(#construct_expr)
            }
        });
        let encoding_stmts=encoding_record_codegen(fields,true);let encoding_count=encoding_stmts.len();
        encoding_arms.push(quote!{#match_pattern=>{control.begin_stage(#encoding_count)?;let mut record=::semio_framework_dsl_record::native_encoding::EncodedRecord::new(#encoding_count,control)?;#(#encoding_stmts)*let keyword=control.copy_text(#keyword)?;Ok((keyword,record.take()))}});
        let controlled_construct=if is_unit{quote!{#name::#variant_ident}}else{quote!{#name::#variant_ident{#(#controlled_fields),*}}};
        controlled_arms.push(quote!{#keyword=>{#(#controlled_stmts)* Ok(#controlled_construct)}});
        let plans=plan_fields(fields);let retirements=plans.iter().map(|plan|{let ident=field_local(&plan.ident);retire_field_codegen(plan,quote!{#ident})});
        retirement_arms.push(quote!{#match_pattern=>{#(#retirements)*}});
        let projected=retained_record_codegen(fields,true);let projected_key=retained_record_key_codegen(fields,true);
        variant_identity_arms.push(quote!{#match_pattern=>(#keyword,#variant_index,Self::#producer_name())});
        variant_projection_arms.push(quote!{#match_pattern=>{#projected}});
        variant_key_arms.push(quote!{#match_pattern=>{#projected_key}});

    }

    let retirement=if let Some(owner)=retire_with{quote!{#owner(self);}}else{quote!{match self{#(#retirement_arms),*}}};

    let schema_variant_count=schema_variant_stmts.len();
    quote! {
        impl #name{#(#schema_methods)*}
        impl ::semio_framework_dsl_record::DslVariants for #name {
            fn retire_decoded_variant(self){#retirement}
            fn projected_variant_identity(&self)->(&'static str,usize,::semio_framework_dsl_record::RecordSpecProducer){match self{#(#variant_identity_arms),*}}
            fn projected_variant_view(&self,path:&[usize])->Result<::semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,::semio_framework_value::ValueError>{match self{#(#variant_projection_arms),*}}
            fn projected_variant_key(&self,path:&[usize],index:usize)->Result<&str,::semio_framework_value::ValueError>{match self{#(#variant_key_arms),*}}
            // 🚫️async: E4 — see `DslVariants::variants`'s tag on the trait.
            fn variants() -> Vec<(String, ::semio_framework_dsl_record::RecordSpecProducer)> {
                vec![ #(#variants_exprs),* ]
            }
            fn variants_controlled<C: ::semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<Vec<(String,::semio_framework_dsl_record::RecordSpecProducer)>,::semio_framework_value::ValueError>{control.scoped_stage(|control|{control.begin_stage(#schema_variant_count)?;let mut variants=control.allocate_vec(#schema_variant_count)?;#(#schema_variant_stmts)*Ok(variants)})}
            fn to_named_record(&self) -> (String, ::semio_framework_dsl_record::RecordValue) {
                match self { #(#to_named_arms),* }
            }
            fn to_named_record_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<(String,::semio_framework_dsl_record::RecordValue),::semio_framework_value::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|match self{#(#encoding_arms),*}))
            }
            fn from_named_record(keyword: &str, record: &::semio_framework_dsl_record::RecordValue) -> Result<Self, ::semio_framework_dsl_record::TextError> {
                match keyword {
                    #(#from_named_arms,)*
                    other => Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown keyword '{other}'"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                }
            }
            fn from_named_record_controlled(keyword:&str,record:&::semio_framework_dsl_record::RecordValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{
                control.step()?;
                match keyword{#(#controlled_arms,)*_=>Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"unknown declared keyword"))}
            }
        }
    }
}


fn emit_enum(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslEnum only supports enums").to_compile_error();
    };
    let container=parse_container_attrs(&input);
    let ordinary=dsl_variants_codegen(&name,data,container.retire_with.as_ref());
    let borrowed=borrowed_variants_owner_codegen(&name,data);
    quote!{#ordinary #borrowed}
}

fn to_kebab(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut out = String::with_capacity(name.len() + 4);
    for (i, &c) in chars.iter().enumerate() {
        if c == '_' || c == '-' {
            if !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            continue;
        }
        if c.is_uppercase() {
            let prev = if i == 0 { None } else { chars.get(i - 1).copied() };
            let next = chars.get(i + 1).copied();
            // A new word starts at an uppercase letter that follows a lowercase/digit
            // (`SetCamera` -> boundary before `C`) OR that follows another uppercase letter but
            // is itself followed by a lowercase one (`HTTPServer` -> boundary before the `S` that
            // starts "Server", not between every letter of the "HTTP" acronym).
            let boundary = match prev {
                Some(p) if p.is_lowercase() || p.is_ascii_digit() => true,
                Some(p) if p.is_uppercase() => next.is_some_and(|n| n.is_lowercase()),
                _ => false,
            };
            if boundary && !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// 🔒️ Reserved local for one authored field: generated statements bind authored fields only under
/// `__semio_field_<name>`, so no authored name (`field`, `record`, `control`, `value`, …) can shadow a
/// generated local or be captured by one. Law: `🗣️dsl/🧪️tests/🧪️hygienic-bindings`.
fn field_local(ident: &syn::Ident) -> syn::Ident {
    quote::format_ident!("__semio_field_{}", ident)
}

/// 🧷️ Struct/variant initializers `name: __semio_field_name` over the reserved locals.
fn field_inits(idents: &[syn::Ident]) -> Vec<proc_macro2::TokenStream> {
    idents.iter().map(|ident| { let local = field_local(ident); quote! { #ident: #local } }).collect()
}

/// 🏗️ Like the `to_value` half of `record_codegen`, but reading from bare local bindings
/// (`ident`) instead of `self.ident` — what a `match self { Variant { fields... } => ... }` arm
/// needs, since enum variant fields aren't reached through `self.field` syntax.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn record_codegen_to_value_from_bindings(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    let plans = plan_fields(fields);
    plans
        .iter()
        .map(|plan| {
            let FieldPlan { ident, id, kind, block, .. } = plan;
            let ident = &field_local(ident);
            let to_value_expr: proc_macro2::TokenStream = match kind {
                FieldKind::Scalar => quote! { ::semio_framework_dsl_record::DslField::to_value(#ident) },
                FieldKind::Bytes64 => quote! { ::semio_framework_dsl_record::FieldValue::Bytes64(#ident.clone()) },
                FieldKind::OptionScalar(_) => quote! {
                    match #ident {
                        Some(v) => ::semio_framework_dsl_record::DslField::to_value(v),
                        None => ::semio_framework_dsl_record::FieldValue::Absent,
                    }
                },
                FieldKind::VecList(_) | FieldKind::VecTable(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::semio_framework_dsl_record::DslField::to_value(v)); }
                        ::semio_framework_dsl_record::FieldValue::List(__items)
                    }
                },
                FieldKind::VecTuple(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::semio_framework_dsl_record::DslField::to_value(v)); }
                        ::semio_framework_dsl_record::FieldValue::Tuple(__items)
                    }
                },
                FieldKind::VecStatements(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::semio_framework_dsl_record::DslVariants::to_named_record(v)); }
                        ::semio_framework_dsl_record::FieldValue::Statements(__items)
                    }
                },
                FieldKind::VecBlockStatements(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::semio_framework_dsl_record::DslVariants::to_named_record(v)); }
                        ::semio_framework_dsl_record::FieldValue::Block(Box::new(::semio_framework_dsl_record::FieldValue::Statements(__items)))
                    }
                },
                FieldKind::MapField(_) => quote! {
                    {
                        let mut __entries = Vec::with_capacity(#ident.len());
                        for (k, v) in #ident.iter() { __entries.push((k.clone(), ::semio_framework_dsl_record::DslField::to_value(v))); }
                        ::semio_framework_dsl_record::FieldValue::Map(__entries)
                    }
                },
                FieldKind::OptionStatements(_) => quote! {
                    ::semio_framework_dsl_record::FieldValue::Statements(match #ident {
                        Some(v) => vec![::semio_framework_dsl_record::DslVariants::to_named_record(v)],
                        None => vec![],
                    })
                },
                FieldKind::RequiredStatements(_) => quote! { ::semio_framework_dsl_record::FieldValue::Statements(vec![::semio_framework_dsl_record::DslVariants::to_named_record(#ident.as_ref())]) },
                FieldKind::RequiredInlineStatements(_) => quote! { ::semio_framework_dsl_record::FieldValue::Statements(vec![::semio_framework_dsl_record::DslVariants::to_named_record(#ident)]) },
            };
            let to_value_expr = if *block {
                quote! {
                    match #to_value_expr {
                        ::semio_framework_dsl_record::FieldValue::Absent => ::semio_framework_dsl_record::FieldValue::Absent,
                        other => ::semio_framework_dsl_record::FieldValue::Block(Box::new(other)),
                    }
                }
            } else {
                to_value_expr
            };
            quote! { record.fields.insert(#id, #to_value_expr); }
        })
        .collect()
}
//#endregion 🔖️VariantHelpers

fn emit_projection(input: DeriveInput, spec: syn::Ident, to: syn::Ident, from: syn::Ident) -> proc_macro2::TokenStream {
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "RecordProjection only supports structs").to_compile_error();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let field_initializers = field_inits(&field_idents);
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout {
        quote! { ::semio_framework_dsl_record::RecordLayout::Lines }
    } else {
        quote! { ::semio_framework_dsl_record::RecordLayout::Inline }
    };

    let borrowed=borrowed_record_owner_codegen(&input,false);
    let expanded = quote! {
        #borrowed
        impl #name {
            // 🚫️async: E1 pure accessor consumed by the same `spec_exprs` shape `DslRecord`'s
            // `__dsl_spec` uses — E4-transitively sync, see R9.
            pub fn #spec() -> ::semio_framework_dsl_record::RecordSpec {
                ::semio_framework_dsl_record::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            pub fn #to(&self) -> ::semio_framework_dsl_record::RecordValue {
                let mut record = ::semio_framework_dsl_record::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            pub fn #from(record: &::semio_framework_dsl_record::RecordValue) -> Result<Self, ::semio_framework_dsl_record::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_initializers),* })
            }
        }

    };
    expanded
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 🧬️ Produces static field identities and finite lazy record edges from the actual authored plan.
fn borrowed_record_codegen(fields:&Fields,owner:&syn::Ident)->Vec<proc_macro2::TokenStream>{
    let plans=plan_fields(fields);
    plans.iter().map(|plan|{
        let FieldPlan{id,key,positional,optional,kind,elem_ty,block,unit,angle,refs,defines,lang,lang_from,coord,dir,..}=plan;
        let refinement=if let Some(symbol)=unit{Some(quote!{::semio_framework_dsl_record::BorrowedShape::Quantity(::semio_framework_dsl_record::borrowed_unit(#symbol))})}else if let Some(symbol)=angle{Some(quote!{::semio_framework_dsl_record::BorrowedShape::Angle(::semio_framework_dsl_record::borrowed_unit(#symbol))})}else if let Some(kind)=refs{Some(quote!{::semio_framework_dsl_record::BorrowedShape::Ref(#kind)})}else if let Some(from)=lang_from{let key=plans.iter().find(|plan|plan.ident==from.as_str()).map_or_else(||to_kebab(from),|plan|plan.key.clone());Some(quote!{::semio_framework_dsl_record::BorrowedShape::EmbedFrom(#key)})}else if let Some(lang)=lang{Some(quote!{::semio_framework_dsl_record::BorrowedShape::Embed(#lang)})}else if *coord{Some(quote!{::semio_framework_dsl_record::BorrowedShape::Coord(3)})}else if *dir{Some(quote!{::semio_framework_dsl_record::BorrowedShape::Dir})}else{None};
        let elem_ty=borrowed_owner_type(elem_ty,owner);
        let shape=match kind{
            FieldKind::Scalar|FieldKind::OptionScalar(_)=>refinement.unwrap_or_else(||quote!{<#elem_ty as ::semio_framework_dsl_record::BorrowedDslField>::SHAPE}),
            FieldKind::VecList(inner)=>{let inner=borrowed_owner_type(inner,owner);quote!{::semio_framework_dsl_record::BorrowedShape::List(::semio_framework_dsl_record::borrowed_field_shape::<#inner>)}},
            FieldKind::VecTuple(inner)=>{let inner=borrowed_owner_type(inner,owner);quote!{::semio_framework_dsl_record::BorrowedShape::Tuple(::semio_framework_dsl_record::borrowed_field_shape::<#inner>,None)}},
            FieldKind::VecStatements(inner)|FieldKind::OptionStatements(inner)|FieldKind::RequiredStatements(inner)|FieldKind::RequiredInlineStatements(inner)=>{let inner=borrowed_owner_type(inner,owner);quote!{::semio_framework_dsl_record::BorrowedShape::Statements(<#inner as ::semio_framework_dsl_record::BorrowedDslVariants>::VARIANTS)}},
            FieldKind::VecBlockStatements(inner)=>{let inner=borrowed_owner_type(inner,owner);quote!{::semio_framework_dsl_record::BorrowedShape::Block(||::semio_framework_dsl_record::BorrowedShape::Statements(<#inner as ::semio_framework_dsl_record::BorrowedDslVariants>::VARIANTS))}},
            FieldKind::MapField(inner)=>{let inner=borrowed_owner_type(inner,owner);quote!{::semio_framework_dsl_record::BorrowedShape::Map(::semio_framework_dsl_record::borrowed_field_shape::<#inner>)}},
            FieldKind::Bytes64=>quote!{::semio_framework_dsl_record::BorrowedShape::Bytes64},
            FieldKind::VecTable(inner)=>{let inner=borrowed_owner_type(inner,owner);quote!{::semio_framework_dsl_record::BorrowedShape::Table(::semio_framework_dsl_record::borrowed_record::<#inner>)}},
        };
        let shape=if *block{quote!{::semio_framework_dsl_record::BorrowedShape::Block(||#shape)}}else{shape};
        let position=match positional{Some(position)=>quote!{Some(#position as u8)},None=>quote!{None}};
        let defines=match defines{Some(defines)=>quote!{Some(#defines)},None=>quote!{None}};
        quote!{::semio_framework_dsl_record::BorrowedFieldSpec{id:#id,key:#key,position:#position,shape:#shape,optional:#optional,flatten:false,defines:#defines,is_call_name:false}}
    }).collect()
}

/// 🔁️ Resolves an authored Self edge to its concrete type in a uniquely named static table.
fn borrowed_owner_type(ty:&Type,owner:&syn::Ident)->proc_macro2::TokenStream{
    fn resolve(tokens:proc_macro2::TokenStream,owner:&syn::Ident)->proc_macro2::TokenStream{tokens.into_iter().map(|token|match token{proc_macro2::TokenTree::Ident(ident)if ident=="Self"=>proc_macro2::TokenTree::Ident(owner.clone()),proc_macro2::TokenTree::Group(group)=>{let mut next=proc_macro2::Group::new(group.delimiter(),resolve(group.stream(),owner));next.set_span(group.span());proc_macro2::TokenTree::Group(next)},other=>other}).collect()}
    resolve(quote!{#ty},owner)
}

/// 📑️ Adds only the explicit static metadata roles owned by this record composition.
fn borrowed_record_owner_codegen(input:&DeriveInput,field_role:bool)->proc_macro2::TokenStream{
    let name=&input.ident;
    let Data::Struct(data)=&input.data else{return syn::Error::new_spanned(input,"borrowed record metadata requires a struct").to_compile_error()};
    let container=parse_container_attrs(input);
    let keyword=match container.keyword{Some(keyword)=>quote!{Some(#keyword)},None=>quote!{None}};
    let layout=if container.lines_layout{quote!{::semio_framework_dsl_record::RecordLayout::Lines}}else{quote!{::semio_framework_dsl_record::RecordLayout::Inline}};
    let fields=borrowed_record_codegen(&data.fields,name);
    let count=fields.len();let table=quote::format_ident!("__DSL_BORROWED_{}_FIELDS",name);
    let field=if field_role{quote!{impl ::semio_framework_dsl_record::BorrowedDslField for #name{const SHAPE : ::semio_framework_dsl_record::BorrowedShape=::semio_framework_dsl_record::BorrowedShape::Record(::semio_framework_dsl_record::borrowed_record::<Self>);}}}else{quote!{}};
    quote!{static #table:[::semio_framework_dsl_record::BorrowedFieldSpec;#count]=[#(#fields),*];impl ::semio_framework_dsl_record::BorrowedDslRecord for #name{const RECORD : ::semio_framework_dsl_record::BorrowedRecordSpec=::semio_framework_dsl_record::BorrowedRecordSpec{keyword:#keyword,layout:#layout,fields:&#table};}#field}
}

/// 🏷️ Compiles literal scalar tags into static enum metadata without owned label copies.
fn borrowed_scalar_owner_codegen(input:&DeriveInput)->proc_macro2::TokenStream{
    let name=&input.ident;
    let Data::Enum(data)=&input.data else{return syn::Error::new_spanned(input,"borrowed scalar metadata requires an enum").to_compile_error()};
    let mut labels=Vec::new();
    for(index,variant)in data.variants.iter().enumerate(){if !matches!(variant.fields,Fields::Unit){return syn::Error::new_spanned(variant,"borrowed scalar metadata requires unit variants").to_compile_error()}let ordinal=index as u32;let attrs=parse_field_attrs(&variant.attrs);let label=attrs.key.unwrap_or_else(||to_kebab(&variant.ident.to_string()));labels.push(quote!{(#label,#ordinal)});}
    let count=labels.len();let table=quote::format_ident!("__DSL_BORROWED_{}_ENUM",name);
    quote!{static #table:[(&'static str,u32);#count]=[#(#labels),*];impl ::semio_framework_dsl_record::BorrowedDslField for #name{const SHAPE : ::semio_framework_dsl_record::BorrowedShape=::semio_framework_dsl_record::BorrowedShape::Enum(&#table);}}
}

/// 🌿️ Preserves literal tagged identities and delegates newtypes to their actual static record owner.
fn borrowed_variants_owner_codegen(name:&syn::Ident,data:&syn::DataEnum)->proc_macro2::TokenStream{
    let mut methods=Vec::new();let mut entries=Vec::new();let mut identities=Vec::new();let mut tables=Vec::new();
    for(index,variant)in data.variants.iter().enumerate(){
        let tag=&variant.ident;let attrs=parse_field_attrs(&variant.attrs);let keyword=attrs.key.unwrap_or_else(||to_kebab(&tag.to_string()));let make=quote::format_ident!("__dsl_borrowed_variant_{}",index);let spec=quote::format_ident!("__DSL_BORROWED_VARIANT_SPEC_{}",index);
        let(body,pattern)=match &variant.fields{
            Fields::Unnamed(fields)if fields.unnamed.len()==1=>{let inner=borrowed_owner_type(&fields.unnamed[0].ty,name);(quote!{<#inner as ::semio_framework_dsl_record::BorrowedDslRecord>::RECORD},quote!{Self::#tag(_)})},
            Fields::Named(_)=>{let fields=borrowed_record_codegen(&variant.fields,name);let count=fields.len();let table=quote::format_ident!("__DSL_BORROWED_{}_VARIANT_{}_FIELDS",name,index);tables.push(quote!{static #table:[::semio_framework_dsl_record::BorrowedFieldSpec;#count]=[#(#fields),*];});(quote!{::semio_framework_dsl_record::BorrowedRecordSpec{keyword:Some(#keyword),layout : ::semio_framework_dsl_record::RecordLayout::Inline,fields:&#table}},quote!{Self::#tag{..}})},
            Fields::Unit=>{let table=quote::format_ident!("__DSL_BORROWED_{}_VARIANT_{}_FIELDS",name,index);tables.push(quote!{static #table:[::semio_framework_dsl_record::BorrowedFieldSpec;0]=[];});(quote!{::semio_framework_dsl_record::BorrowedRecordSpec{keyword:Some(#keyword),layout : ::semio_framework_dsl_record::RecordLayout::Inline,fields:&#table}},quote!{Self::#tag})},
            _=>return syn::Error::new_spanned(variant,"borrowed tagged metadata requires named, unit, or one record newtype variant").to_compile_error(),
        };
        methods.push(quote!{const #spec : ::semio_framework_dsl_record::BorrowedRecordSpec=#body;fn #make()->::semio_framework_dsl_record::BorrowedRecordSpec{Self::#spec}});entries.push(quote!{(#keyword,#name::#make as fn()->::semio_framework_dsl_record::BorrowedRecordSpec)});identities.push(quote!{#pattern=>(#keyword,#index,Self::#make())});
    }
    let count=entries.len();let table=quote::format_ident!("__DSL_BORROWED_{}_VARIANTS",name);
    quote!{#(#tables)*impl #name{#(#methods)*}static #table:[(&'static str,fn()->::semio_framework_dsl_record::BorrowedRecordSpec);#count]=[#(#entries),*];impl ::semio_framework_dsl_record::BorrowedDslVariants for #name{const VARIANTS:&'static[(&'static str,fn()->::semio_framework_dsl_record::BorrowedRecordSpec)]=&#table;fn projected_borrowed_variant_identity(&self)->(&'static str,usize,::semio_framework_dsl_record::BorrowedRecordSpec){match self{#(#identities),*}}fn projected_borrowed_variant_view(&self,path:&[usize])->Result<::semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,::semio_framework_value::ValueError>{<Self as ::semio_framework_dsl_record::DslVariants>::projected_variant_view(self,path)}fn projected_borrowed_variant_key(&self,path:&[usize],index:usize)->Result<&str,::semio_framework_value::ValueError>{<Self as ::semio_framework_dsl_record::DslVariants>::projected_variant_key(self,path,index)}}}
}
