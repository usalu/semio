//! 🧩️ PresentationML (pptx) import — `ppt/presentation.xml`/`ppt/slides/slideN.xml` XML parse
//! into a `PptxPresentation`, real OPC package decode, and magic-shape sniff. Zip/OPC/XML
//! byte-level work is never reimplemented here: it is reused from the shared
//! `semio_s_artifact_stdio_zip::opc` layer.
//!
//! `p:spTree`'s DIRECT children (`p:sp`/`p:pic`/anything else) become one `PptxShape` each --
//! per ticket 26/08/11's W0 finding, the shape tree used to be flattened away entirely (every
//! `p:txBody`'s paragraphs concatenated, shape boundaries discarded). Shapes nested inside a
//! `p:grpSp` group, `p:graphicFrame` (charts/tables/SmartArt), `p:cxnSp` connectors, and anything
//! unrecognized fall back to `PptxShape::Other{node}` as logical XML.

use crate::standards::v_ecma_376::subsets::base::schema::vocabulary::{attr_val,element_children,resolve_office_document_relationship};
use crate::standards::v_ecma_376::subsets::base::schema::refusal::{PptxError};
use crate::{
    schema::snapshot::{pptx_part_is_xml, PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxXmlPart},
    PptxSnapshot,
};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument, XmlNode};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_from_text};
use semio_s_artifact_stdio_zip::opc;

//#region 🔖️Codec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_pptx(data: &[u8]) -> Result<PptxSnapshot, PptxError> {
    let mut opc = opc::decode_opc(data)?;
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
    opc.parts = binary_parts;
    let snapshot = PptxSnapshot::from_parts(opc, xml_parts);
    project_presentation(&snapshot.opc, &snapshot.xml_parts)?;
    Ok(snapshot)
}
//#endregion 🔖️Codec

//#region 🔖️Sniff
/// 🕵️ Real pptx sniff: OPC-shaped bytes whose root officeDocument relationship (Transitional or
/// Strict) resolves under `ppt/` — disambiguates from docx/xlsx sharing the same zip magic and
/// OPC shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_pptx_bytes(data: &[u8]) -> bool {
    let Ok(opc) = opc::decode_opc(data) else { return false };
    match resolve_office_document_relationship(&opc) {
        Some(path) => path.starts_with("ppt/"),
        None => false,
    }
}
//#endregion 🔖️Sniff

use crate::standards::v_ecma_376::subsets::base::schema::inferences::presentation::*;
