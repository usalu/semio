//! 🧩️ PresentationML (pptx) import — `ppt/presentation.xml`/`ppt/slides/slideN.xml` XML parse
//! into a `PptxPresentation`, real OPC package decode, and magic-shape sniff. Zip/OPC/XML
//! byte-level work is never reimplemented here: it is reused from the shared
//! `crate::artifacts::zip::opc` layer.
//!
//! `p:spTree`'s DIRECT children (`p:sp`/`p:pic`/anything else) become one `PptxShape` each --
//! per ticket 26/08/11's W0 finding, the shape tree used to be flattened away entirely (every
//! `p:txBody`'s paragraphs concatenated, shape boundaries discarded). Shapes nested inside a
//! `p:grpSp` group, `p:graphicFrame` (charts/tables/SmartArt), `p:cxnSp` connectors, and anything
//! unrecognized fall back to `PptxShape::Other{node}` as logical XML.

use super::super::super::{attr_val, element_children, find_child, resolve_office_document_relationship, PptxError};
use crate::artifacts::pptx::{
    schema::snapshot::{pptx_part_is_xml, PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxXmlPart},
    PptxSnapshot,
};
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, XmlDocument, XmlNode};
use crate::artifacts::zip::opc;

//#region 🔖️TextXml
async fn run_from_xml(node: &XmlNode) -> Option<PptxRun> {
    let XmlNode::Element { name, children, .. } = node else { return None };
    if name != "a:r" {
        return None;
    }
    let mut bold = false;
    let mut italic = false;
    let mut font_size = None;
    let mut text = String::new();
    for rc in children {
        let XmlNode::Element { name, attrs, children: inner } = rc else { continue };
        match name.as_str() {
            "a:rPr" => {
                if let Some(b) = attr_val(attrs, "b") {
                    bold = b == "1";
                }
                if let Some(i) = attr_val(attrs, "i") {
                    italic = i == "1";
                }
                if let Some(sz) = attr_val(attrs, "sz").and_then(|v| v.parse::<u32>().ok()) {
                    font_size = Some(sz / 100);
                }
            }
            "a:t" => {
                for t in inner {
                    if let XmlNode::Text { text: t } = t {
                        text.push_str(t);
                    }
                }
            }
            _ => {}
        }
    }
    Some(PptxRun { text, bold, italic, font_size })
}

async fn paragraph_from_xml(node: &XmlNode) -> PptxParagraph {
    let mut runs = Vec::new();
    for c in element_children(node) {
        if let Some(r) = run_from_xml(c) {
            runs.push(r);
        }
    }
    PptxParagraph { runs }
}

/// 🔎️ Every `a:p` directly inside `p:txBody` (direct children only -- a real `p:txBody` never
/// nests paragraphs inside anything else).
async fn text_frame_from_xml(tx_body: &XmlNode) -> Vec<PptxParagraph> {
    element_children(tx_body).iter().filter(|c| matches!(c, XmlNode::Element { name, .. } if name == "a:p")).map(paragraph_from_xml).collect()
}
//#endregion 🔖️TextXml

//#region 🔖️ShapeXml
/// 🔎️ Reads `p:spPr/a:xfrm`'s `a:off`/`a:ext` (defaulting each missing field to `0`, same
/// convention this codec pair uses for "not present in the XML").
async fn position_from_xml(shape_children: &[XmlNode]) -> crate::artifacts::pptx::schema::snapshot::PptxTransform {
    use crate::artifacts::pptx::schema::snapshot::PptxTransform;
    let Some(sp_pr) = find_child(shape_children, "p:spPr") else { return PptxTransform::default() };
    let Some(xfrm) = find_child(element_children(sp_pr), "a:xfrm") else { return PptxTransform::default() };
    let xfrm_children = element_children(xfrm);
    let (mut x, mut y, mut cx, mut cy) = (0i64, 0i64, 0i64, 0i64);
    if let Some(XmlNode::Element { attrs, .. }) = find_child(xfrm_children, "a:off") {
        x = attr_val(attrs, "x").and_then(|v| v.parse().ok()).unwrap_or(0);
        y = attr_val(attrs, "y").and_then(|v| v.parse().ok()).unwrap_or(0);
    }
    if let Some(XmlNode::Element { attrs, .. }) = find_child(xfrm_children, "a:ext") {
        cx = attr_val(attrs, "cx").and_then(|v| v.parse().ok()).unwrap_or(0);
        cy = attr_val(attrs, "cy").and_then(|v| v.parse().ok()).unwrap_or(0);
    }
    PptxTransform { x, y, cx, cy }
}

/// 🧭️ Classifies one `p:spTree` DIRECT child into a typed `PptxShape` (`p:sp`/`p:pic` get real
/// per-kind typing; everything else -- `p:graphicFrame`, `p:grpSp`, `p:cxnSp`, unrecognized --
/// falls back to `Other{node}`, preserving its logical XML tree).
async fn shape_from_xml_node(node: &XmlNode) -> PptxShape {
    let XmlNode::Element { name, children, .. } = node else { return PptxShape::Other { node: node.clone() } };
    match name.as_str() {
        "p:sp" => {
            let ph_type = find_child(children, "p:nvSpPr").and_then(|nv| find_child(element_children(nv), "p:nvPr")).and_then(|nvpr| find_child(element_children(nvpr), "p:ph")).map(|ph| match ph {
                XmlNode::Element { attrs, .. } => attr_val(attrs, "type").unwrap_or("body").to_string(),
                _ => "body".to_string(),
            });
            let position = position_from_xml(children);
            let text_frame = find_child(children, "p:txBody").map(text_frame_from_xml).unwrap_or_default();
            match ph_type {
                Some(kind) => PptxShape::Placeholder { kind, text_frame, position },
                None => PptxShape::TextBox { text_frame, position },
            }
        }
        "p:pic" => {
            let blip_rel_id = find_child(children, "p:blipFill")
                .and_then(|fill| find_child(element_children(fill), "a:blip"))
                .and_then(|blip| match blip {
                    XmlNode::Element { attrs, .. } => attr_val(attrs, "r:embed"),
                    _ => None,
                })
                .unwrap_or_default()
                .to_string();
            let position = position_from_xml(children);
            PptxShape::Picture { blip_rel_id, position }
        }
        _ => PptxShape::Other { node: node.clone() },
    }
}
//#endregion 🔖️ShapeXml

//#region 🔖️SlideXml
/// 🔎️ Collects every `p:spTree` DIRECT child (skipping the group's own `p:nvGrpSpPr`/
/// `p:grpSpPr` container elements) into one `PptxShape` each, in document order -- shape
/// BOUNDARIES are preserved (not flattened away like the pre-migration model).
async fn collect_shapes(root: &XmlNode) -> Vec<PptxShape> {
    let XmlNode::Element { children, .. } = root else { return Vec::new() };
    let Some(c_sld) = find_child(children, "p:cSld") else { return Vec::new() };
    let Some(sp_tree) = find_child(element_children(c_sld), "p:spTree") else { return Vec::new() };
    element_children(sp_tree).iter().filter(|c| !matches!(c, XmlNode::Element { name, .. } if name == "p:nvGrpSpPr" || name == "p:grpSpPr")).map(shape_from_xml_node).collect()
}
//#endregion 🔖️SlideXml

//#region 🔖️PresentationXml
async fn presentation_slide_rids_from_xml(doc: &XmlDocument, part: &str) -> Result<Vec<String>, PptxError> {
    let bad = |detail: String| PptxError::Xml { part: part.into(), detail };
    let root = doc.root.as_ref().ok_or_else(|| bad("empty document".into()))?;
    let XmlNode::Element { name, children, .. } = root else { return Err(bad("root is not an element".into())) };
    if name != "p:presentation" {
        return Err(bad(format!("expected <p:presentation>, got <{name}>")));
    }
    let sld_id_lst = children.iter().find_map(|c| match c {
        XmlNode::Element { name, children, .. } if name == "p:sldIdLst" => Some(children),
        _ => None,
    });
    let mut out = Vec::new();
    if let Some(children) = sld_id_lst {
        for c in children {
            let XmlNode::Element { name, attrs, .. } = c else { continue };
            if name != "p:sldId" {
                continue;
            }
            let rid = attr_val(attrs, "r:id").ok_or_else(|| bad("<p:sldId> missing r:id".into()))?.to_string();
            out.push(rid);
        }
    }
    Ok(out)
}
//#endregion 🔖️PresentationXml

//#region 🔖️Projection
/// 🧭️ Derives the typed presentation view from the authoritative logical XML parts.
pub(crate) async fn project_presentation(opc: &opc::OpcPackage, xml_parts: &[PptxXmlPart]) -> Result<PptxPresentation, PptxError> {
    let presentation_path = resolve_office_document_relationship(opc).ok_or(PptxError::MissingPresentationRelationship)?;
    let presentation = xml_parts.iter().find(|part| part.path == presentation_path).ok_or_else(|| PptxError::MissingPart(presentation_path.clone()))?;
    let slide_rids = presentation_slide_rids_from_xml(&presentation.document, &presentation_path)?;
    let pres_rels = opc.relationships_for(&presentation_path);
    let mut slides = Vec::with_capacity(slide_rids.len());
    for rid in slide_rids {
        let rel = pres_rels.iter().find(|relationship| relationship.id == rid).ok_or_else(|| PptxError::Malformed(format!("presentation references unknown relationship id {rid}")))?;
        let path = opc::resolve_relationship_target(&presentation_path, &rel.target);
        let slide = xml_parts.iter().find(|part| part.path == path).ok_or_else(|| PptxError::MissingPart(path.clone()))?;
        let shapes = slide.document.root.as_ref().map(collect_shapes).unwrap_or_default();
        slides.push(PptxSlide { shapes });
    }
    Ok(PptxPresentation { slides })
}
//#endregion 🔖️Projection

//#region 🔖️Codec
pub async fn decode_pptx(data: &[u8]) -> Result<PptxSnapshot, PptxError> {
    let mut opc = opc::decode_opc(data)?;
    let presentation_path = resolve_office_document_relationship(&opc).ok_or(PptxError::MissingPresentationRelationship)?;
    let bytes = opc.part_bytes(&presentation_path).ok_or_else(|| PptxError::MissingPart(presentation_path.clone()))?;
    let text = String::from_utf8(bytes.to_vec()).map_err(|_| PptxError::Xml { part: presentation_path.clone(), detail: "not valid utf-8".into() })?;
    let xml = xml_document_from_text(&text).map_err(|e| PptxError::Xml { part: presentation_path.clone(), detail: e })?;
    let slide_rids = presentation_slide_rids_from_xml(&xml, &presentation_path)?;

    let pres_rels = opc.relationships_for(&presentation_path);
    let mut slides = Vec::with_capacity(slide_rids.len());
    for rid in &slide_rids {
        let rel = pres_rels.iter().find(|r| &r.id == rid).ok_or_else(|| PptxError::Malformed(format!("presentation references unknown relationship id {rid}")))?;
        let path = opc::resolve_relationship_target(&presentation_path, &rel.target);
        let bytes = opc.part_bytes(&path).ok_or_else(|| PptxError::MissingPart(path.clone()))?;
        let text = String::from_utf8(bytes.to_vec()).map_err(|_| PptxError::Xml { part: path.clone(), detail: "not valid utf-8".into() })?;
        let slide_xml = xml_document_from_text(&text).map_err(|e| PptxError::Xml { part: path.clone(), detail: e })?;
        let shapes = slide_xml.root.as_ref().map(collect_shapes).unwrap_or_default();
        slides.push(PptxSlide { shapes });
    }

    let mut xml_parts = Vec::new();
    let mut binary_parts = Vec::new();
    for part in std::mem::take(&mut opc.parts) {
        if pptx_part_is_xml(&part.path, &part.content_type) {
            let text = String::from_utf8(part.bytes).map_err(|_| PptxError::Xml { part: part.path.clone(), detail: "not valid utf-8".into() })?;
            let document = xml_document_from_text(&text).map_err(|detail| PptxError::Xml { part: part.path.clone(), detail })?;
            xml_parts.push(PptxXmlPart { path: part.path, content_type: part.content_type, document });
        } else {
            binary_parts.push(part);
        }
    }
    xml_parts.sort_by(|left, right| left.path.cmp(&right.path));
    binary_parts.sort_by(|left, right| left.path.cmp(&right.path));
    opc.parts = binary_parts;
    let presentation = PptxPresentation { slides };
    debug_assert!(matches!(project_presentation(&opc, &xml_parts), Ok(projected) if projected == presentation));
    Ok(PptxSnapshot::from_parts(opc, xml_parts, presentation))
}
//#endregion 🔖️Codec

//#region 🔖️Sniff
/// 🕵️ Real pptx sniff: OPC-shaped bytes whose root officeDocument relationship (Transitional or
/// Strict) resolves under `ppt/` — disambiguates from docx/xlsx sharing the same zip magic and
/// OPC shape.
pub async fn sniff_pptx_bytes(data: &[u8]) -> bool {
    let Ok(opc) = opc::decode_opc(data) else { return false };
    match resolve_office_document_relationship(&opc) {
        Some(path) => path.starts_with("ppt/"),
        None => false,
    }
}
//#endregion 🔖️Sniff
