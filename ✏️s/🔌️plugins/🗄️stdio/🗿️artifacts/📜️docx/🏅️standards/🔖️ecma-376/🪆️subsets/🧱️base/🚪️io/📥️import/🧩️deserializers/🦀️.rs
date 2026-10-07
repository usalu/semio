//! 🧩️ WordprocessingML (docx) import — `word/document.xml`/`word/styles.xml` XML parse into a
//! `DocxDocument`, real OPC package decode, and magic-shape sniff. Zip/OPC/XML byte-level work is
//! never reimplemented here: it is reused from the shared `semio_s_artifact_stdio_zip::opc` layer and,
//! transitively, `semio_s_artifact_stdio_zip::engine` + `semio_s_artifact_stdio_xml::schema::snapshot`.

use crate::standards::v_ecma_376::subsets::base::schema::refusal::{DocxError};
use crate::standards::v_ecma_376::subsets::base::schema::vocabulary::{MAIN_DOCUMENT_PART,REL_TYPE_STYLES,STRICT_REL_TYPE_OFFICE_DOCUMENT,STRICT_REL_TYPE_STYLES,STYLES_PART};
use crate::DocxSnapshot;
use crate::schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow, DocxXmlPart, DocxXmlParts, docx_part_is_xml};
use crate::standards::v_ecma_376::subsets::base::schema::namespaces::{is_word_name, scoped_bindings, word_attr, word_local_name};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument, XmlNode};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_from_text};
use semio_s_artifact_stdio_zip::opc::{self, REL_TYPE_OFFICE_DOCUMENT};

pub fn decode_docx(data: &[u8]) -> Result<DocxSnapshot, DocxError> {
    let mut opc = opc::decode_opc(data)?;
    let mut xml_parts = Vec::new();
    let mut binary_parts = Vec::with_capacity(opc.parts.len());
    for part in std::mem::take(&mut opc.parts) {
        if docx_part_is_xml(&part.path, &part.content_type) {
            let text = String::from_utf8(part.bytes).map_err(|_| DocxError::Xml { part: part.path.clone(), detail: "not valid utf-8".into() })?;
            let document = xml_document_from_text(&text).map_err(|detail| DocxError::Xml { part: part.path.clone(), detail })?;
            xml_parts.push(DocxXmlPart::try_from_document(part.path, part.content_type, document)?);
        } else {
            binary_parts.push(part);
        }
    }
    opc.parts = binary_parts;
    let snapshot = DocxSnapshot::from_parts(opc, xml_parts)?;
    snapshot.validate_authority()?;
    project_snapshot_document(&snapshot)?;
    Ok(snapshot)
}
//#endregion 🔖️Codec

//#region 🔖️Sniff
/// 🕵️ Recognizes the relationship-selected WordprocessingML main part regardless of its path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_docx_bytes(data: &[u8]) -> bool {
    let Ok(opc) = opc::decode_opc(data) else { return false };
    opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT)
        .or_else(|| opc.resolve_relationship("", STRICT_REL_TYPE_OFFICE_DOCUMENT))
        .and_then(|path| opc.part(&path))
        .is_some_and(|part| part.content_type == crate::standards::v_ecma_376::subsets::base::schema::vocabulary::MAIN_DOCUMENT_CONTENT_TYPE)
}
//#endregion 🔖️Sniff

use crate::standards::v_ecma_376::subsets::base::schema::inferences::document::*;
