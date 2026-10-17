//! 🏷️ Attribute parsing of the record derive: the container and field attribute records and their parsers. See [`super`].

use super::*;

//#region 🔖️Attrs
#[derive(Default, Clone)]
struct ContainerAttrs {
    pub(super) keyword: Option<String>,
    pub(super) lines_layout: bool,
    pub(super) retire_with: Option<syn::Path>,
}

#[derive(Default, Clone)]
pub(super) struct FieldAttrs {
    pub(super) skip: bool,
    pub(super) key: Option<String>,
    pub(super) positional: bool,
    list: bool,
    pub(super) tuple: bool,
    pub(super) statements: bool,
    pub(super) block: bool,
    pub(super) base64: bool,
    pub(super) flatten: bool,
    pub(super) table: bool,
    /// `#[dsl(unit = "GPa")]` — a scalar `f64`/`f32` field prints/parses as `Shape::Quantity`
    /// (glued unit suffix) instead of plain `Shape::Float`.
    pub(super) unit: Option<String>,
    /// `#[dsl(angle = "deg")]` — same mechanism as `unit`, `Shape::Angle` instead.
    pub(super) angle: Option<String>,
    /// `#[dsl(refs = "material")]` — a scalar `String`/`Option<String>` field prints/parses as
    /// `Shape::Ref(kind)` instead of plain `Shape::Text`.
    pub(super) refs: Option<String>,
    /// `#[dsl(defines = "material")]` — the anchor side of `refs`: this field's `FieldSpec.defines`
    /// is set so `LanguageService::validate` knows which field, in a record of this kind, other
    /// records' `Shape::Ref("material")` fields are expected to resolve against.
    pub(super) defines: Option<String>,
    /// `#[dsl(lang = "jack")]` — a scalar `String` field prints/parses as `Shape::Embed(lang)`
    /// (fenced verbatim in Document mode) instead of plain `Shape::Text`.
    pub(super) lang: Option<String>,
    /// `#[dsl(lang_from = "language_id")]` — fence language from a sibling Text field at print/parse time.
    pub(super) lang_from: Option<String>,
    /// `#[dsl(coord)]` — a `[f64; 3]` (or any `DslField` array) field prints/parses as
    /// `Shape::Coord(3)` (`@x,y,z`) instead of a bare comma tuple.
    pub(super) coord: bool,
    /// `#[dsl(dir)]` — same mechanism as `coord`, `Shape::Dir` (`^x,y,z`) instead.
    pub(super) dir: bool,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
pub(super) fn parse_container_attrs(input: &DeriveInput) -> ContainerAttrs {
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
pub(super) fn parse_field_attrs(attrs: &[syn::Attribute]) -> FieldAttrs {
    let mut out = FieldAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("dsl") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                out.skip = true;
            } else if meta.path.is_ident("key") {
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
