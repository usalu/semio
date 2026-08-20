//! 📐 `dimensions` — one named inference: the root `<svg>` element's intrinsic size, honestly
//! derived from whatever sizing attributes the document actually carries (SVG 1.1 §7.10 lets
//! `width`/`height` and `viewBox` disagree or be individually absent — this never fabricates a
//! value neither attribute provides). A vector format has no pixel grid of its own, so unlike the
//! raster stdio formats this intentionally has no `bitDepth`/`hasAlpha`/`pixelCount` — those
//! concepts don't apply here.

use crate::artifacts::svg::schema::snapshot::{svg_element_from_xml_node, SvgElement};
use crate::artifacts::svg::SvgSnapshot;
use crate::artifacts::xml::schema::snapshot::XmlNode;
use serde::{Deserialize, Serialize};

//#region 🔖️Dimensions
/// 📐️ Root `<svg>` intrinsic size. `width`/`height` prefer the element's own `width`/`height`
/// attributes (SVG 1.1 §7.10's "intrinsic size"), falling back to `viewBox`'s width/height (§7.11)
/// when the attribute is absent or unparseable; `0.0` when neither is present.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SvgDimensions {
    pub width: f64,
    pub height: f64,
}

/// 🔢️ Strips a trailing CSS length unit (`px`/`%`/`pt`/...) and parses the leading numeric run —
/// SVG 1.1 §7.10's `<length>` grammar allows either a bare number or a number+unit pair.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_length(s: &str) -> Option<f64> {
    let trimmed = s.trim();
    let end = trimmed.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == '+' || c == 'e' || c == 'E')).unwrap_or(trimmed.len());
    if end == 0 {
        return None;
    }
    trimmed[..end].parse::<f64>().ok()
}

/// 📐️ Computes [`SvgDimensions`] from a snapshot's root element — pure, total (never panics),
/// `SvgDimensions::default()` for a document with no root or a non-`<svg>` root.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_svg_dimensions(snapshot: &SvgSnapshot) -> SvgDimensions {
    let Some(root @ XmlNode::Element { .. }) = &snapshot.doc.root else {
        return SvgDimensions::default();
    };
    let Ok(SvgElement::Svg { view_box, width, height, .. }) = svg_element_from_xml_node(root) else {
        return SvgDimensions::default();
    };
    let (view_box_width, view_box_height) = view_box.map(|vb| (vb.width, vb.height)).unwrap_or((0.0, 0.0));
    let width = width.as_deref().and_then(parse_length).unwrap_or(view_box_width);
    let height = height.as_deref().and_then(parse_length).unwrap_or(view_box_height);
    SvgDimensions { width, height }
}
//#endregion 🔖️Dimensions

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlDocument};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn svg_snapshot(attrs: Vec<XmlAttr>) -> SvgSnapshot {
        SvgSnapshot { schema: crate::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: XmlDocument { root: Some(XmlNode::Element { name: "svg".into(), attrs, children: Vec::new() }), doctype: None, declaration: None, prolog: Vec::new() } }
    }

    #[semio_framework_async_macros::async_test]
    async fn prefers_width_height_attrs_over_view_box() {
        let snapshot = svg_snapshot(vec![XmlAttr { name: "width".into(), value: "42px".into() }, XmlAttr { name: "height".into(), value: "24".into() }, XmlAttr { name: "viewBox".into(), value: "0 0 100 100".into() }]);
        assert_eq!(compute_svg_dimensions(&snapshot), SvgDimensions { width: 42.0, height: 24.0 });
    }

    #[semio_framework_async_macros::async_test]
    async fn falls_back_to_view_box_when_width_height_absent() {
        let snapshot = svg_snapshot(vec![XmlAttr { name: "viewBox".into(), value: "0 0 100 50".into() }]);
        assert_eq!(compute_svg_dimensions(&snapshot), SvgDimensions { width: 100.0, height: 50.0 });
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_document_yields_zero_dimensions() {
        assert_eq!(compute_svg_dimensions(&SvgSnapshot { schema: crate::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: XmlDocument { root: None, doctype: None, declaration: None, prolog: Vec::new() } }), SvgDimensions::default());
    }
}
//#endregion 🧪️Tests
