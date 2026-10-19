//! 🧩️ Type shape of the record derive: the field kind and the type classifiers for options, lists, maps and byte vectors. See [`super`].

use super::*;

//#region 🔖️TypeShape
pub(super) enum FieldKind {
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
pub(super) fn classify_field(ty: &Type, attrs: &FieldAttrs) -> (FieldKind, Type) {
    if let Some(inner) = inner_of(ty, "Option") {
        if attrs.statements {
            return (FieldKind::OptionStatements(Box::new(inner.clone())), inner);
        }
                if attrs.required { return (FieldKind::Scalar,ty.clone()); }
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
