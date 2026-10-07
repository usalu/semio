//! 🧵️ WordprocessingML (docx) export — `DocxDocument` → `word/document.xml`/`word/styles.xml`
//! XML render, and the OPC package assembly/sync around it. Zip/OPC/XML byte-level work is never
//! reimplemented here: it is reused from the shared `semio_s_artifact_stdio_zip::opc` layer and,
//! transitively, `semio_s_artifact_stdio_zip::engine` + `semio_s_artifact_stdio_xml::schema::snapshot`.

use crate::standards::v_ecma_376::subsets::base::schema::refusal::{DocxError};
use crate::standards::v_ecma_376::subsets::base::schema::vocabulary::{MAIN_DOCUMENT_CONTENT_TYPE,MAIN_DOCUMENT_PART,REL_TYPE_STYLES,STYLES_CONTENT_TYPE,STYLES_PART,STYLES_REL_TARGET,STRICT_REL_TYPE_OFFICE_DOCUMENT,W_NS};
use crate::{
    schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow, DocxXmlPart},
    DocxSnapshot,
};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_to_text_checked};
use semio_s_artifact_stdio_zip::opc::{self, OpcPackage, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};

/// 📦️ Serializes each authoritative XML part exactly once alongside non-XML OPC payloads.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_docx(snap: &DocxSnapshot) -> Result<Vec<u8>, DocxError> {
    snap.validate_authority()?;
    snap.project_document()?;
    let materialization_bytes = snap.xml_parts.iter().try_fold(snap.opc.materialization_owned_bytes()?, |total, part| {
        total
            .checked_add(part.document.materialization_owned_bytes()?)
            .ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "DOCX save materialization ownership overflow"))
    })?;
    let mut callback = |_| true;
    let mut control = semio_framework_value::NativeEncodeControl::new(materialization_bytes, &mut callback);
    let mut package = snap.opc.materialize_package(&mut control)?;
    let mut paths: std::collections::HashSet<String> = package.parts.iter().map(|part| part.path.clone()).collect();
    for part in snap.xml_parts.iter() {
        if !paths.insert(part.path.clone()) {
            return Err(DocxError::Malformed(format!("duplicate OPC part authority: {}", part.path)));
        }
        let document = part.materialize_document(&mut control)?;
        let text = xml_document_to_text_checked(&document).map_err(|detail| DocxError::Xml { part: part.path.clone(), detail })?;
        package.set_part(&part.path, &part.content_type, text.into_bytes());
    }
    let main_path = package
        .resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT)
        .or_else(|| package.resolve_relationship("", STRICT_REL_TYPE_OFFICE_DOCUMENT))
        .ok_or(DocxError::MissingMainDocumentRelationship)?;
    if !snap.xml_parts.iter().any(|part| part.path == main_path) {
        return Err(DocxError::MissingPart(main_path));
    }
    Ok(opc::encode_opc_with_package_order(&package)?)
}
//#endregion 🔖️Codec

use crate::standards::v_ecma_376::subsets::base::schema::construction::*;
