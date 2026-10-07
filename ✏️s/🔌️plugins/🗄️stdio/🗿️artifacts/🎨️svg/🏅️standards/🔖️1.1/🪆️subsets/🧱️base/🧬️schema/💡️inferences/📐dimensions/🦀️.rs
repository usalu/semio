//! 📐 `dimensions` — one named inference: the root `<svg>` element's intrinsic size, honestly
//! derived from whatever sizing attributes the document actually carries (SVG 1.1 §7.10 lets
//! `width`/`height` and `viewBox` disagree or be individually absent — this never fabricates a
//! value neither attribute provides). A vector format has no pixel grid of its own, so unlike the
//! raster stdio formats this intentionally has no `bitDepth`/`hasAlpha`/`pixelCount` — those
//! concepts don't apply here.

use crate::SvgSnapshot;

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

/// 📐️ Computes intrinsic size directly from decoded attribute owners.
pub fn compute_svg_dimensions(snapshot:&crate::SvgSnapshot)->SvgDimensions {
    use crate::schema::snapshot::{SvgNode,SvgAttributeValue as V};
    let Some(SvgNode::Element{name,attrs,..})=&snapshot.doc.root else{return SvgDimensions::default()};
    if name!="svg"&&!name.ends_with(":svg"){return SvgDimensions::default()}
    let view_box=attrs.iter().find(|a|a.name=="viewBox").and_then(|a|match &a.value{V::ViewBox(value)=>Some(value),_=>None});
    let length=|name:&str,fallback:f64|attrs.iter().find(|a|a.name==name).and_then(|a|match &a.value{V::Length(value)=>Some(value.magnitude),V::Number(value)=>Some(*value),_=>None}).unwrap_or(fallback);
    SvgDimensions{width:length("width",view_box.map_or(0.0,|v|v.width)),height:length("height",view_box.map_or(0.0,|v|v.height))}
}
//#endregion 🔖️Dimensions

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
