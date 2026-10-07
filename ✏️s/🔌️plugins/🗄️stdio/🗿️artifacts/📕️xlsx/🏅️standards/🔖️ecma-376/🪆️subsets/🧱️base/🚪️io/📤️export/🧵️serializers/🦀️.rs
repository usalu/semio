//! 🧵️ SpreadsheetML physical package serialization.

use crate::schema::refusal::XlsxError;
use crate::XlsxSnapshot;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_to_text_checked;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_xlsx(snap: &XlsxSnapshot) -> Result<Vec<u8>, XlsxError> {
    snap.validate_authority()?;
    let mut opc = snap.opc.clone();
    let mut paths: std::collections::HashSet<String> = opc.parts.iter().map(|part| part.path.clone()).collect();
    for part in &snap.xml_parts {
        if !paths.insert(part.path.clone()) {
            return Err(XlsxError::Malformed(format!("duplicate OPC part authority: {}", part.path)));
        }
        let text = xml_document_to_text_checked(&part.document).map_err(|detail| XlsxError::Xml { part: part.path.clone(), detail })?;
        opc.set_part(&part.path, &part.content_type, text.into_bytes());
    }
    Ok(semio_s_artifact_stdio_zip::opc::encode_opc_with_package_order(&opc)?)
}
//#endregion 🔖️Codec
