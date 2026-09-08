//! 📐 `dimensions` — one named inference: the root `<svg>` element's intrinsic size, honestly
//! derived from whatever sizing attributes the document actually carries (SVG 1.1 §7.10 lets
//! `width`/`height` and `viewBox` disagree or be individually absent — this never fabricates a
//! value neither attribute provides). A vector format has no pixel grid of its own, so unlike the
//! raster stdio formats this intentionally has no `bitDepth`/`hasAlpha`/`pixelCount` — those
//! concepts don't apply here.

use crate::schema::snapshot::{svg_element_from_xml_node, SvgElement};
use crate::SvgSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;

//#region 🔖️Dimensions
/// 📐️ Root `<svg>` intrinsic size. `width`/`height` prefer the element's own `width`/`height`
/// attributes (SVG 1.1 §7.10's "intrinsic size"), falling back to `viewBox`'s width/height (§7.11)
/// when the attribute is absent or unparseable; `0.0` when neither is present.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
    let (view_box_width, view_box_height) = view_box.map_or((0.0, 0.0), |vb| (vb.width, vb.height));
    let width = width.as_deref().and_then(parse_length).unwrap_or(view_box_width);
    let height = height.as_deref().and_then(parse_length).unwrap_or(view_box_height);
    SvgDimensions { width, height }
}
//#endregion 🔖️Dimensions

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
