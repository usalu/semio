//! 🧵️ PresentationML (pptx) export — `PptxPresentation` → `ppt/presentation.xml`/
//! `ppt/slides/slideN.xml` XML render, and the OPC package assembly/sync around it. Zip/OPC/XML
//! byte-level work is never reimplemented here: it is reused from the shared
//! `semio_s_artifact_stdio_zip::opc` layer. `ppt/slideMasters`/`ppt/slideLayouts`/`ppt/theme` are
//! unmodeled boilerplate every real reader still needs to open the package validly: they are
//! synthesized once (fixed minimal-but-schema-shaped constants) when building a package from
//! scratch, while decoded packages preserve their logical XML documents.

use crate::standards::v_ecma_376::subsets::base::schema::vocabulary::{attr,resolve_office_document_relationship,A_NS,PRESENTATION_CONTENT_TYPE,PRESENTATION_PART,P_NS,REL_TYPE_SLIDE,REL_TYPE_SLIDE_LAYOUT,REL_TYPE_SLIDE_MASTER,REL_TYPE_THEME,R_NS,SLIDE_CONTENT_TYPE,SLIDE_LAYOUT_CONTENT_TYPE,SLIDE_LAYOUT_PART,SLIDE_MASTER_CONTENT_TYPE,SLIDE_MASTER_PART,THEME_CONTENT_TYPE,THEME_PART};
use crate::standards::v_ecma_376::subsets::base::schema::refusal::{PptxError};
use crate::{
    schema::snapshot::{pptx_part_is_xml, PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxTransform},
    PptxSnapshot,
};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument, XmlNode};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_to_text};
use semio_s_artifact_stdio_zip::opc::{OpcPackage, OpcRelationship, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT};

pub fn encode_pptx(snap: &PptxSnapshot) -> Result<Vec<u8>, PptxError> {
    let mut opc = snap.opc.clone();
    let mut xml_paths = std::collections::HashSet::new();
    for part in &snap.xml_parts {
        if !pptx_part_is_xml(&part.path, &part.content_type) {
            return Err(PptxError::Malformed(format!("logical XML part {} has a non-XML content type", part.path)));
        }
        if !xml_paths.insert(part.path.as_str()) {
            return Err(PptxError::Malformed(format!("duplicate logical XML part {}", part.path)));
        }
        let bytes = semio_s_artifact_stdio_zip::opc::xml_document_to_opc_text_checked(&part.document).map_err(|detail| PptxError::Xml { part: part.path.clone(), detail })?.into_bytes();
        if opc.content_types.resolve(&part.path) == Some(part.content_type.as_str()) {
            if let Some(existing) = opc.parts.iter_mut().find(|candidate| candidate.path == part.path) {
                existing.content_type = part.content_type.clone();
                existing.bytes = bytes;
            } else {
                opc.parts.push(semio_s_artifact_stdio_zip::opc::OpcPart { path: part.path.clone(), content_type: part.content_type.clone(), bytes });
            }
        } else {
            opc.set_part(&part.path, &part.content_type, bytes);
        }
    }
    for part in &snap.opc.parts {
        if pptx_part_is_xml(&part.path, &part.content_type) {
            return Err(PptxError::Malformed(format!("XML part {} is stored as opaque OPC bytes", part.path)));
        }
        if xml_paths.contains(part.path.as_str()) {
            return Err(PptxError::Malformed(format!("part {} has both XML and binary authorities", part.path)));
        }
    }
    Ok(semio_s_artifact_stdio_zip::opc::encode_opc_with_package_order(&opc)?)
}
//#endregion 🔖️Codec

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
