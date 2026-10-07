use super::*;
use crate::schema::snapshot::{SvgAttr, SvgDocument, SvgNode};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn svg_snapshot(attrs: Vec<SvgAttr>) -> SvgSnapshot {
    SvgSnapshot { schema: crate::STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: SvgDocument { root: Some(SvgNode::Element { name: "svg".into(), attrs, children: Vec::new() }), doctype: None, declaration: None, prolog: Vec::new(), epilog: Vec::new() } }
}

#[semio_framework_async_macros::async_test]
async fn prefers_width_height_attrs_over_view_box() {
    let snapshot = svg_snapshot(vec![SvgAttr {name:"width".into(),value:crate::schema::snapshot::SvgAttributeValue::Length(crate::schema::snapshot::SvgLength{magnitude:42.0,unit:"px".into()})}, SvgAttr {name:"height".into(),value:crate::schema::snapshot::SvgAttributeValue::Length(crate::schema::snapshot::SvgLength{magnitude:24.0,unit:"".into()})}, SvgAttr {name:"viewBox".into(),value:crate::schema::snapshot::SvgAttributeValue::ViewBox(crate::schema::snapshot::ViewBox{min_x:0.0,min_y:0.0,width:100.0,height:100.0})}]);
    assert_eq!(compute_svg_dimensions(&snapshot), SvgDimensions { width: 42.0, height: 24.0 });
}

#[semio_framework_async_macros::async_test]
async fn falls_back_to_view_box_when_width_height_absent() {
    let snapshot = svg_snapshot(vec![SvgAttr {name:"viewBox".into(),value:crate::schema::snapshot::SvgAttributeValue::ViewBox(crate::schema::snapshot::ViewBox{min_x:0.0,min_y:0.0,width:100.0,height:50.0})}]);
    assert_eq!(compute_svg_dimensions(&snapshot), SvgDimensions { width: 100.0, height: 50.0 });
}

#[semio_framework_async_macros::async_test]
async fn empty_document_yields_zero_dimensions() {
    assert_eq!(compute_svg_dimensions(&SvgSnapshot { schema: crate::STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: SvgDocument { root: None, doctype: None, declaration: None, prolog: Vec::new(), epilog: Vec::new() } }), SvgDimensions::default());
}
