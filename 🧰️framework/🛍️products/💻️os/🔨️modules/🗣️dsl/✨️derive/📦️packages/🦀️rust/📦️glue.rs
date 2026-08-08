//! 🧬️ `dsl_derive` — compiles `#[dsl(...)]`-annotated struct/enum declarations into
//! `dsl::DslField`/`dsl::DslVariants` bindings (nested usage composes through), so a technology
//! declares its grammar instead of hand-writing a parser/printer. Analyze → IR → emit.
//!
//! P6: `DslDocument`/`DslOps` no longer emit `DocumentDsl`/`DocumentPack`/`OpText`/`OpBinary` —
//! those traits are handcrafted per artifact. `DslRecord` stays for field helpers only.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

//#region 🔖️Attrs
#[derive(Default, Clone)]
struct ContainerAttrs {
    extension: Option<String>,
    id: Option<String>,
    keyword: Option<String>,
    lines_layout: bool,
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

fn parse_container_attrs(input: &DeriveInput) -> ContainerAttrs {
    let mut out = ContainerAttrs::default();
    for attr in &input.attrs {
        if !attr.path().is_ident("dsl") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("extension") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.extension = Some(value.value());
            } else if meta.path.is_ident("id") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.id = Some(value.value());
            } else if meta.path.is_ident("keyword") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.keyword = Some(value.value());
            } else if meta.path.is_ident("layout") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.lines_layout = value.value() == "lines";
            }
            Ok(())
        });
    }
    out
}

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
    /// `#[dsl(statements)] Box<T>` (or bare `T`) — exactly one required tagged value (`layer:
    /// Box<DrawLayerNode>` on an `AddLayer` operation), the non-optional counterpart of
    /// `OptionStatements`: same `Shape::Statements` reuse, but errors if the count isn't exactly 1
    /// rather than treating 0 as `None`.
    RequiredStatements(Box<Type>),
    Bytes64,
    /// `#[dsl(table)] Vec<T>` (`T: DslRecord`) — Structure-of-Arrays columnar `Shape::Table`.
    /// `to_value`/`from_value` are identical to `VecList` (both produce `FieldValue::List(Vec<
    /// FieldValue::Record>)`) — only the `Shape` differs, so every binder/diff path downstream
    /// keeps working unchanged.
    VecTable(Box<Type>),
}

/// @emoji 🪆️ Strips `macro_rules!`-introduced invisible-delimiter `Type::Group` wrappers so a type
/// captured through a `:ty` metavariable — then re-emitted through another technology-local
/// declarative macro (e.g. an `entity_input!`-style struct-generating macro) before ever reaching
/// this derive — still structurally matches `Type::Path` here exactly like directly-written source.
/// Without this, `Option<T>`/`Vec<T>`/`Box<T>`/`BTreeMap<..>` fields declared through such a wrapping
/// macro silently fall through to plain `FieldKind::Scalar` instead of being classified as
/// optional/list/map, since the wrapper hides the outer `Path` segment from a bare `matches!`.
fn strip_groups(ty: &Type) -> &Type {
    let mut ty = ty;
    while let Type::Group(group) = ty {
        ty = &group.elem;
    }
    ty
}

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

fn is_vec_u8(ty: &Type) -> bool {
    inner_of(ty, "Vec").is_some_and(|inner| matches!(strip_groups(&inner), Type::Path(p) if p.path.is_ident("u8")))
}

/// @emoji 🗺️ Extracts `V` from `BTreeMap<String, V>` — `None` for any other type, including a
/// `BTreeMap` keyed by something other than `String` (the engine's `Shape::Map` is string-keyed
/// only, matching every hand-rolled `{ key=value }` grammar it replaces).
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
    if let Some(inner) = inner_of(ty, "Vec") {
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

/// @emoji 🏗️ Builds the three code fragments shared by `DslRecord`/`DslDocument`/`DslOps` variant
/// bodies: the `RecordSpec` field-spec expressions, the struct→`RecordValue` conversion, and the
/// `RecordValue`→struct conversion.
fn record_codegen(fields: &Fields) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<syn::Ident>) {
    let plans = plan_fields(fields);
    let mut spec_exprs = Vec::new();
    let mut to_value_stmts = Vec::new();
    let mut from_value_stmts = Vec::new();
    let mut field_idents = Vec::new();

    for plan in &plans {
        let FieldPlan { ident, id, key, positional, optional, kind, elem_ty, block, unit, angle, refs, defines, lang, lang_from, coord, dir } = plan;
        // A `#[dsl(unit = "...")]`/`#[dsl(angle = "...")]` scalar field's Shape is resolved at
        // spec-build time via `dsl::__rt::unit_for_derive` — same lazy-per-call pattern every other
        // `fn() -> RecordSpec`-backed Shape in this engine already uses, so an unknown unit symbol
        // surfaces as a panic the first time the generated spec runs (caught by that app's own
        // RecordSpec-law tests), never silently.
        let quantity_shape_override: Option<proc_macro2::TokenStream> = if let Some(symbol) = unit {
            Some(quote! { ::dsl::Shape::Quantity(::dsl::__rt::unit_for_derive(#symbol)) })
        } else if let Some(symbol) = angle {
            Some(quote! { ::dsl::Shape::Angle(::dsl::__rt::unit_for_derive(#symbol)) })
        } else if let Some(kind) = refs {
            Some(quote! { ::dsl::Shape::Ref(#kind) })
        } else if let Some(from) = lang_from {
            let embed_lang_key = plans
                .iter()
                .find(|p| p.ident.to_string() == *from)
                .map(|p| p.key.clone())
                .unwrap_or_else(|| to_kebab(from));
            Some(quote! { ::dsl::Shape::EmbedFrom(#embed_lang_key) })
        } else if let Some(l) = lang {
            Some(quote! { ::dsl::Shape::Embed(#l) })
        } else if *coord {
            Some(quote! { ::dsl::Shape::Coord(3) })
        } else if *dir {
            Some(quote! { ::dsl::Shape::Dir })
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

        let (shape_expr, to_value_expr, from_value_expr): (proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream) = match kind {
            FieldKind::Scalar => (
                quantity_shape_override.clone().unwrap_or_else(|| quote! { <#elem_ty as ::dsl::DslField>::shape() }),
                quote! { ::dsl::DslField::to_value(&self.#ident) },
                quote! { <#elem_ty as ::dsl::DslField>::from_value(value).map_err(::dsl::__rt::field_error)? },
            ),
            FieldKind::Bytes64 => (
                quote! { ::dsl::Shape::Bytes64 },
                quote! { ::dsl::FieldValue::Bytes64(self.#ident.clone()) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Bytes64(bytes) => bytes.clone(),
                        other => return Err(::dsl::__rt::field_error(format!("expected Bytes64, found {other:?}"))),
                    }
                },
            ),
            FieldKind::OptionScalar(inner) => (
                quantity_shape_override.clone().unwrap_or_else(|| quote! { <#inner as ::dsl::DslField>::shape() }),
                quote! {
                    match &self.#ident {
                        Some(v) => ::dsl::DslField::to_value(v),
                        None => ::dsl::FieldValue::Absent,
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Absent => None,
                        other => Some(<#inner as ::dsl::DslField>::from_value(other).map_err(::dsl::__rt::field_error)?),
                    }
                },
            ),
            FieldKind::VecList(inner) => (
                quote! { ::dsl::Shape::List(Box::new(<#inner as ::dsl::DslField>::shape())) },
                quote! { ::dsl::FieldValue::List(self.#ident.iter().map(|v| ::dsl::DslField::to_value(v)).collect()) },
                quote! {
                    match value {
                        ::dsl::FieldValue::List(items) => items.iter().map(|v| <#inner as ::dsl::DslField>::from_value(v)).collect::<Result<Vec<_>, String>>().map_err(::dsl::__rt::field_error)?,
                        other => return Err(::dsl::__rt::field_error(format!("expected List, found {other:?}"))),
                    }
                },
            ),
            // Same `to_value`/`from_value` as `VecList` (both produce `FieldValue::List(Record)`)
            // — only the `Shape` differs (`Table` vs `List(Record)`), which is what makes the
            // printer emit compact SoA instead of verbose AoS for this field.
            FieldKind::VecTable(inner) => (
                quote! { ::dsl::Shape::Table(<#inner>::__dsl_spec as fn() -> ::dsl::RecordSpec) },
                quote! { ::dsl::FieldValue::List(self.#ident.iter().map(|v| ::dsl::DslField::to_value(v)).collect()) },
                quote! {
                    match value {
                        ::dsl::FieldValue::List(items) => items.iter().map(|v| <#inner as ::dsl::DslField>::from_value(v)).collect::<Result<Vec<_>, String>>().map_err(::dsl::__rt::field_error)?,
                        other => return Err(::dsl::__rt::field_error(format!("expected List, found {other:?}"))),
                    }
                },
            ),
            FieldKind::VecTuple(inner) => (
                quote! { ::dsl::Shape::Tuple(Box::new(<#inner as ::dsl::DslField>::shape()), None) },
                quote! { ::dsl::FieldValue::Tuple(self.#ident.iter().map(|v| ::dsl::DslField::to_value(v)).collect()) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Tuple(items) => items.iter().map(|v| <#inner as ::dsl::DslField>::from_value(v)).collect::<Result<Vec<_>, String>>().map_err(::dsl::__rt::field_error)?,
                        other => return Err(::dsl::__rt::field_error(format!("expected Tuple, found {other:?}"))),
                    }
                },
            ),
            FieldKind::VecStatements(inner) => (
                quote! { ::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants()) },
                quote! { ::dsl::FieldValue::Statements(self.#ident.iter().map(|v| ::dsl::DslVariants::to_named_record(v)).collect()) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Statements(items) => items
                            .iter()
                            .map(|(keyword, record)| <#inner as ::dsl::DslVariants>::from_named_record(keyword, record))
                            .collect::<Result<Vec<_>, ::dsl::TextError>>()?,
                        other => return Err(::dsl::__rt::field_error(format!("expected Statements, found {other:?}"))),
                    }
                },
            ),
            FieldKind::VecBlockStatements(inner) => (
                quote! { ::dsl::Shape::Block(Box::new(::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants()))) },
                quote! { ::dsl::FieldValue::Block(Box::new(::dsl::FieldValue::Statements(self.#ident.iter().map(|v| ::dsl::DslVariants::to_named_record(v)).collect()))) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Block(inner_value) => match inner_value.as_ref() {
                            ::dsl::FieldValue::Statements(items) => items
                                .iter()
                                .map(|(keyword, record)| <#inner as ::dsl::DslVariants>::from_named_record(keyword, record))
                                .collect::<Result<Vec<_>, ::dsl::TextError>>()?,
                            other => return Err(::dsl::__rt::field_error(format!("expected Statements inside Block, found {other:?}"))),
                        },
                        other => return Err(::dsl::__rt::field_error(format!("expected Block, found {other:?}"))),
                    }
                },
            ),
            FieldKind::MapField(inner) => (
                quote! { ::dsl::Shape::Map(Box::new(<#inner as ::dsl::DslField>::shape())) },
                quote! { ::dsl::FieldValue::Map(self.#ident.iter().map(|(k, v)| (k.clone(), ::dsl::DslField::to_value(v))).collect()) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Map(entries) => entries
                            .iter()
                            .map(|(k, v)| Ok((k.clone(), <#inner as ::dsl::DslField>::from_value(v).map_err(::dsl::__rt::field_error)?)))
                            .collect::<Result<::std::collections::BTreeMap<String, _>, ::dsl::TextError>>()?,
                        other => return Err(::dsl::__rt::field_error(format!("expected Map, found {other:?}"))),
                    }
                },
            ),
            FieldKind::OptionStatements(inner) => (
                quote! { ::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants()) },
                quote! {
                    ::dsl::FieldValue::Statements(match &self.#ident {
                        Some(v) => vec![::dsl::DslVariants::to_named_record(v)],
                        None => vec![],
                    })
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Absent => None,
                        ::dsl::FieldValue::Statements(items) if items.is_empty() => None,
                        ::dsl::FieldValue::Statements(items) if items.len() == 1 => {
                            Some(<#inner as ::dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1)?)
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected 0 or 1 tagged values, found {other:?}"))),
                    }
                },
            ),
            FieldKind::RequiredStatements(inner) => (
                quote! { ::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants()) },
                quote! { ::dsl::FieldValue::Statements(vec![::dsl::DslVariants::to_named_record(self.#ident.as_ref())]) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Statements(items) if items.len() == 1 => {
                            Box::new(<#inner as ::dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1)?)
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected exactly 1 tagged value, found {other:?}"))),
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
                quote! { ::dsl::Shape::Block(Box::new(#shape_expr)) },
                quote! {
                    match #to_value_expr {
                        ::dsl::FieldValue::Absent => ::dsl::FieldValue::Absent,
                        other => ::dsl::FieldValue::Block(Box::new(other)),
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Block(inner) => { let value = inner.as_ref(); #from_value_expr },
                        ::dsl::FieldValue::Absent => { let value = &::dsl::FieldValue::Absent; #from_value_expr },
                        other => return Err(::dsl::__rt::field_error(format!("expected Block, found {other:?}"))),
                    }
                },
            )
        } else {
            (shape_expr, to_value_expr, from_value_expr)
        };

        spec_exprs.push(quote! {
            ::dsl::FieldSpec::new(#id, #key, #shape_expr) #pos_expr #opt_expr #defines_expr
        });
        to_value_stmts.push(quote! {
            record.fields.insert(#id, #to_value_expr);
        });
        from_value_stmts.push(quote! {
            let #ident = {
                let value = record.get(#id).ok_or_else(|| ::dsl::__rt::field_error(format!("missing field '{}'", #key)))?;
                #from_value_expr
            };
        });
    }

    (spec_exprs, to_value_stmts, from_value_stmts, field_idents)
}
//#endregion 🔖️RecordCodegen

//#region 🔖️DslRecord
#[proc_macro_derive(DslRecord, attributes(dsl))]
pub fn derive_dsl_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslRecord only supports structs").to_compile_error().into();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout {
        quote! { ::dsl::RecordLayout::Lines }
    } else {
        quote! { ::dsl::RecordLayout::Inline }
    };

    let expanded = quote! {
        impl #name {
            pub fn __dsl_spec() -> ::dsl::RecordSpec {
                ::dsl::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            pub fn __dsl_to_record(&self) -> ::dsl::RecordValue {
                let mut record = ::dsl::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            pub fn __dsl_from_record(record: &::dsl::RecordValue) -> Result<Self, ::dsl::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_idents),* })
            }
        }

        impl ::dsl::DslField for #name {
            fn shape() -> ::dsl::Shape {
                ::dsl::Shape::Record(Self::__dsl_spec)
            }
            fn to_value(&self) -> ::dsl::FieldValue {
                ::dsl::FieldValue::Record(self.__dsl_to_record())
            }
            fn from_value(value: &::dsl::FieldValue) -> Result<Self, String> {
                match value {
                    ::dsl::FieldValue::Record(record) => Self::__dsl_from_record(record).map_err(|e| e.message),
                    other => Err(format!("expected Record, found {other:?}")),
                }
            }
        }
    };
    expanded.into()
}
//#endregion 🔖️DslRecord

//#region 🔖️DslDocument
#[proc_macro_derive(DslDocument, attributes(dsl))]
pub fn derive_dsl_document(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let envelope_id = match container.id.or(container.extension) {
        Some(id) => id,
        None => {
            return syn::Error::new_spanned(
                &input,
                "DslDocument requires #[dsl(id = \"plugin.artifact\")] or #[dsl(extension = \"...\")]",
            )
            .to_compile_error()
            .into();
        }
    };
    let extension_suffix = envelope_id.rsplit('.').next().unwrap_or(&envelope_id);
    let envelope_id_lit = envelope_id.as_str();
    let extension_suffix_lit = extension_suffix;
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslDocument only supports structs").to_compile_error().into();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout {
        quote! { ::dsl::RecordLayout::Lines }
    } else {
        quote! { ::dsl::RecordLayout::Inline }
    };

    let expanded = quote! {
        impl #name {
            pub fn __dsl_spec() -> ::dsl::RecordSpec {
                ::dsl::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            pub fn __dsl_to_record(&self) -> ::dsl::RecordValue {
                let mut record = ::dsl::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            pub fn __dsl_from_record(record: &::dsl::RecordValue) -> Result<Self, ::store::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_idents),* })
            }
            /// ✉️ Envelope constants for handcrafted DocumentDsl/DocumentPack wiring (P6: derive no longer emits those traits).
            pub const __DSL_ENVELOPE_ID: &'static str = #envelope_id_lit;
            pub const __DSL_EXTENSION: &'static str = #extension_suffix_lit;
        }

        // A document type can also be nested as an ordinary field (e.g. a "whole document
        // snapshot" operation variant), so it needs `DslField` too, not just `store::DocumentDsl`.
        impl ::dsl::DslField for #name {
            fn shape() -> ::dsl::Shape {
                ::dsl::Shape::Record(Self::__dsl_spec)
            }
            fn to_value(&self) -> ::dsl::FieldValue {
                ::dsl::FieldValue::Record(self.__dsl_to_record())
            }
            fn from_value(value: &::dsl::FieldValue) -> Result<Self, String> {
                match value {
                    ::dsl::FieldValue::Record(record) => Self::__dsl_from_record(record).map_err(|e| e.message),
                    other => Err(format!("expected Record, found {other:?}")),
                }
            }
        }

    };
    expanded.into()
}
//#endregion 🔖️DslDocument

//#region 🔖️DslDiff
/// @emoji 🧬️ W1 foundation of the `handcrafted-grammar-for-every-artifact` diff track (design ruling
/// B-R4): emits a `protocol::DiffCodec` impl from the SAME `RecordSpec`-generation machinery
/// `#[derive(DslRecord)]`/`#[derive(DslDocument)]` already use — a diff is structurally just another
/// record, so this reuses `record_codegen` verbatim rather than reinventing field lowering. Unlike
/// `DslDocument` there is no `EXTENSION`/file-extension concept (a diff is never opened as its own
/// file) and no `DocumentPack` (the pack/binary side is `DiffCodec::encode_diff`/`decode_diff`
/// instead, routed through the same `store::pack_rt` the `DocumentPack` impl above uses — every
/// crate that already derives an operation/document alongside its diff already depends on `store`).
#[proc_macro_derive(DslDiff, attributes(dsl))]
pub fn derive_dsl_diff(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslDiff only supports structs").to_compile_error().into();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout {
        quote! { ::dsl::RecordLayout::Lines }
    } else {
        quote! { ::dsl::RecordLayout::Inline }
    };

    let expanded = quote! {
        impl #name {
            pub fn __dsl_diff_spec() -> ::dsl::RecordSpec {
                ::dsl::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            pub fn __dsl_diff_to_record(&self) -> ::dsl::RecordValue {
                let mut record = ::dsl::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            pub fn __dsl_diff_from_record(record: &::dsl::RecordValue) -> Result<Self, ::dsl::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_idents),* })
            }
        }

        impl ::protocol::DiffCodec for #name {
            fn print_diff(&self) -> String {
                ::dsl::print(&self.__dsl_diff_to_record(), &Self::__dsl_diff_spec(), ::dsl::JoinMode::Inline)
            }
            fn parse_diff(line: &str) -> Result<Self, ::dsl::TextError> {
                let record = ::dsl::parse(line, &Self::__dsl_diff_spec(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;
                Self::__dsl_diff_from_record(&record)
            }
            fn encode_diff(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
                ::store::pack_rt::encode_document(&Self::__dsl_diff_spec(), &self.__dsl_diff_to_record(), &::store::PackEncodeOptions::default()).map_err(::protocol::ProtocolError::from)
            }
            fn decode_diff(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
                let (record, _report) = ::store::pack_rt::decode_document(bytes, &Self::__dsl_diff_spec(), &::store::PackDecodeOptions::default()).map_err(::protocol::ProtocolError::from)?;
                Self::__dsl_diff_from_record(&record).map_err(|error| ::protocol::ProtocolError::Malformed { what: "diff record", offset: 0, detail: error.to_string() })
            }
        }
    };
    expanded.into()
}
//#endregion 🔖️DslDiff

//#region 🔖️DslScalar
#[proc_macro_derive(DslScalar, attributes(dsl))]
pub fn derive_dsl_scalar(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslScalar only supports unit-variant enums").to_compile_error().into();
    };
    let mut variant_tags = Vec::new();
    let mut match_to_ordinal = Vec::new();
    let mut match_from_ordinal = Vec::new();
    for (ordinal, variant) in data.variants.iter().enumerate() {
        if !matches!(variant.fields, Fields::Unit) {
            return syn::Error::new_spanned(variant, "DslScalar only supports unit variants").to_compile_error().into();
        }
        let attrs = parse_field_attrs(&variant.attrs);
        let variant_ident = variant.ident.clone();
        let tag = attrs.key.unwrap_or_else(|| to_kebab(&variant_ident.to_string()));
        let ordinal = ordinal as u32;
        variant_tags.push(quote! { (#tag.to_string(), #ordinal) });
        match_to_ordinal.push(quote! { #name::#variant_ident => #ordinal });
        match_from_ordinal.push(quote! { #ordinal => Ok(#name::#variant_ident) });
    }

    let expanded = quote! {
        impl ::dsl::DslField for #name {
            fn shape() -> ::dsl::Shape {
                ::dsl::Shape::Enum(vec![ #(#variant_tags),* ])
            }
            fn to_value(&self) -> ::dsl::FieldValue {
                ::dsl::FieldValue::Enum(match self { #(#match_to_ordinal),* })
            }
            fn from_value(value: &::dsl::FieldValue) -> Result<Self, String> {
                match value {
                    ::dsl::FieldValue::Enum(ordinal) => match *ordinal {
                        #(#match_from_ordinal,)*
                        other => Err(format!("unknown enum ordinal {other}")),
                    },
                    other => Err(format!("expected Enum, found {other:?}")),
                }
            }
        }
    };
    expanded.into()
}
//#endregion 🔖️DslScalar

//#region 🔖️DslOps
/// @emoji 🌿️ Builds the `impl ::dsl::DslVariants for #name` block shared by `DslEnum` (data-only
/// tagged enums, e.g. a recursive block tree) and `DslOps` (operation enums, which additionally get
/// `store::OpText` on top of this same `DslVariants` foundation).
fn dsl_variants_codegen(name: &syn::Ident, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let mut variants_exprs = Vec::new();
    let mut to_named_arms = Vec::new();
    let mut from_named_arms = Vec::new();

    for variant in &data.variants {
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
                variants_exprs.push(quote! {
                    (#keyword.to_string(), ::dsl::__rt::newtype_variant_spec::<#inner_ty> as fn() -> ::dsl::RecordSpec)
                });
                to_named_arms.push(quote! {
                    #name::#variant_ident(inner) => (#keyword.to_string(), ::dsl::__rt::newtype_variant_to_record(inner))
                });
                from_named_arms.push(quote! {
                    #keyword => Ok(#name::#variant_ident(::dsl::__rt::newtype_variant_from_record::<#inner_ty>(record)?))
                });
                continue;
            }
        }

        let (spec_exprs, _to_value_stmts, from_value_stmts, field_idents) = record_codegen(fields);

        variants_exprs.push(quote! {
            (#keyword.to_string(), (|| ::dsl::RecordSpec::new_owned(Some(#keyword.to_string()), ::dsl::RecordLayout::Inline, vec![ #(#spec_exprs),* ])) as fn() -> ::dsl::RecordSpec)
        });

        // Build a per-variant to-record conversion using the field bindings from a `match` on
        // `self`, since (unlike `DslRecord`) the fields live inside an enum variant, not `self.field`.
        // A true unit variant (`Variant`, no braces at all) needs a bare match pattern — `Variant {}`
        // is only valid Rust for a variant that was itself declared with (empty) braces.
        let field_binds: Vec<proc_macro2::TokenStream> = field_idents.iter().map(|f| quote! { #f }).collect();
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
            quote! { #name::#variant_ident { #(#field_idents),* } }
        };
        to_named_arms.push(quote! {
            #match_pattern => {
                let mut record = ::dsl::RecordValue::default();
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
    }

    quote! {
        impl ::dsl::DslVariants for #name {
            fn variants() -> Vec<(String, fn() -> ::dsl::RecordSpec)> {
                vec![ #(#variants_exprs),* ]
            }
            fn to_named_record(&self) -> (String, ::dsl::RecordValue) {
                match self { #(#to_named_arms),* }
            }
            fn from_named_record(keyword: &str, record: &::dsl::RecordValue) -> Result<Self, ::dsl::TextError> {
                match keyword {
                    #(#from_named_arms,)*
                    other => Err(::dsl::__rt::field_error(format!("unknown keyword '{other}'"))),
                }
            }
        }
    }
}

#[proc_macro_derive(DslOps, attributes(dsl))]
pub fn derive_dsl_ops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslOps only supports enums").to_compile_error().into();
    };
    let variants_impl = dsl_variants_codegen(&name, data);

    // P6: DslOps emits DslVariants only — OpText/OpBinary must be handcrafted per artifact.
    variants_impl.into()
}
//#endregion 🔖️DslOps

//#region 🔖️DslEnum
/// @emoji 🌳️ Tagged-record enum whose variants are plain data (a recursive block tree, a wire
/// node kind, ...) rather than a `Mutation` — implements `::dsl::DslVariants` only, so it can be
/// used inside `#[dsl(statements)]`/`#[dsl(statements, block)]` collection fields without also
/// gaining (and having to satisfy the bounds of) `store::OpText`.
#[proc_macro_derive(DslEnum, attributes(dsl))]
pub fn derive_dsl_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslEnum only supports enums").to_compile_error().into();
    };
    dsl_variants_codegen(&name, data).into()
}
//#endregion 🔖️DslEnum

//#region 🔖️VariantHelpers
/// @emoji 🔡️ Converts a Rust identifier (`PascalCase`/`camelCase`/`snake_case`, any mix) into
/// lowercase `kebab-case` — the unified syntax law's key/keyword/tag convention. Falls back to
/// this whenever no explicit `#[dsl(key = "...")]` override is given, for variant keywords,
/// record field keys, and `DslScalar` variant tags alike, so `SetCamera` -> `set-camera`,
/// `airtightness_n50` -> `airtightness-n50`, `HTTPServer` -> `http-server`.
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

/// @emoji 🏗️ Like the `to_value` half of `record_codegen`, but reading from bare local bindings
/// (`ident`) instead of `self.ident` — what a `match self { Variant { fields... } => ... }` arm
/// needs, since enum variant fields aren't reached through `self.field` syntax.
fn record_codegen_to_value_from_bindings(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    let plans = plan_fields(fields);
    plans
        .iter()
        .map(|plan| {
            let FieldPlan { ident, id, kind, block, .. } = plan;
            let to_value_expr: proc_macro2::TokenStream = match kind {
                FieldKind::Scalar => quote! { ::dsl::DslField::to_value(#ident) },
                FieldKind::Bytes64 => quote! { ::dsl::FieldValue::Bytes64(#ident.clone()) },
                FieldKind::OptionScalar(_) => quote! {
                    match #ident {
                        Some(v) => ::dsl::DslField::to_value(v),
                        None => ::dsl::FieldValue::Absent,
                    }
                },
                FieldKind::VecList(_) | FieldKind::VecTable(_) => quote! { ::dsl::FieldValue::List(#ident.iter().map(|v| ::dsl::DslField::to_value(v)).collect()) },
                FieldKind::VecTuple(_) => quote! { ::dsl::FieldValue::Tuple(#ident.iter().map(|v| ::dsl::DslField::to_value(v)).collect()) },
                FieldKind::VecStatements(_) => quote! { ::dsl::FieldValue::Statements(#ident.iter().map(|v| ::dsl::DslVariants::to_named_record(v)).collect()) },
                FieldKind::VecBlockStatements(_) => quote! { ::dsl::FieldValue::Block(Box::new(::dsl::FieldValue::Statements(#ident.iter().map(|v| ::dsl::DslVariants::to_named_record(v)).collect()))) },
                FieldKind::MapField(_) => quote! { ::dsl::FieldValue::Map(#ident.iter().map(|(k, v)| (k.clone(), ::dsl::DslField::to_value(v))).collect()) },
                FieldKind::OptionStatements(_) => quote! {
                    ::dsl::FieldValue::Statements(match #ident {
                        Some(v) => vec![::dsl::DslVariants::to_named_record(v)],
                        None => vec![],
                    })
                },
                FieldKind::RequiredStatements(_) => quote! { ::dsl::FieldValue::Statements(vec![::dsl::DslVariants::to_named_record(#ident.as_ref())]) },
            };
            let to_value_expr = if *block {
                quote! {
                    match #to_value_expr {
                        ::dsl::FieldValue::Absent => ::dsl::FieldValue::Absent,
                        other => ::dsl::FieldValue::Block(Box::new(other)),
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
