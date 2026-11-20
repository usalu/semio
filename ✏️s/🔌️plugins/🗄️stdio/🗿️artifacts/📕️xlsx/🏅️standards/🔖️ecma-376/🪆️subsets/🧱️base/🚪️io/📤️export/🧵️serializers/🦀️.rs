//! 🧵️ SpreadsheetML physical package serialization.

use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::refusal::XlsxError;
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_to_text_checked;
use semio_s_artifact_stdio_zip::opc::{encode_opc_with_package_order, OpcPart};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_xlsx(snap: &XlsxSnapshot) -> Result<Vec<u8>, XlsxError> {
    snap.validate_authority()?;
    let mut opc = snap.opc.clone();
    for part in &snap.xml_parts {
        let text = xml_document_to_text_checked(&part.document).map_err(|detail| XlsxError::Xml { part: part.path.clone(), detail })?;
        opc.parts.push(OpcPart { path: part.path.clone(), content_type: part.content_type.clone(), bytes: text.into_bytes() });
    }
    Ok(encode_opc_with_package_order(&opc)?)
}
//#endregion 🔖️Codec
