
use super::*;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn svg_snapshot(attrs: Vec<XmlAttr>) -> SvgSnapshot {
    SvgSnapshot { schema: crate::STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: XmlDocument { root: Some(XmlNode::Element { name: "svg".into(), attrs, children: Vec::new() }), doctype: None, declaration: None, prolog: Vec::new() } }
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
    assert_eq!(compute_svg_dimensions(&SvgSnapshot { schema: crate::STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: XmlDocument { root: None, doctype: None, declaration: None, prolog: Vec::new() } }), SvgDimensions::default());
}
