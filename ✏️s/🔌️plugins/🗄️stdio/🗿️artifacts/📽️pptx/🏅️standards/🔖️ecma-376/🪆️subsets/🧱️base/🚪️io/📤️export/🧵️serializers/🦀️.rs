//! 🧵️ PresentationML (pptx) export — `PptxPresentation` → `ppt/presentation.xml`/
//! `ppt/slides/slideN.xml` XML render, and the OPC package assembly/sync around it. Zip/OPC/XML
//! byte-level work is never reimplemented here: it is reused from the shared
//! `semio_s_artifact_stdio_zip::opc` layer. `ppt/slideMasters`/`ppt/slideLayouts`/`ppt/theme` are
//! unmodeled boilerplate every real reader still needs to open the package validly: they are
//! synthesized once (fixed minimal-but-schema-shaped constants) when building a package from
//! scratch, while decoded packages preserve their logical XML documents.

use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::refusal::PptxError;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::pptx_part_is_xml;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxParagraph;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxPresentation;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxRun;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxShape;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxSlide;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxTransform;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxSnapshot;

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
