//! 🧩️ SpreadsheetML package decoding and byte-shape sniffing.

use crate::schema::{refusal::XlsxError, vocabulary::REL_TYPE_OFFICE_DOCUMENT_STRICT};
use crate::schema::snapshot::{xlsx_part_is_xml, XlsxXmlPart};
use crate::XlsxSnapshot;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text;
use semio_s_artifact_stdio_zip::opc::{self, REL_TYPE_OFFICE_DOCUMENT};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_xlsx(data: &[u8]) -> Result<XlsxSnapshot, XlsxError> {
    let mut opc = opc::decode_opc(data)?;
    let mut xml_parts = Vec::new();
    let mut binary_parts = Vec::new();
    for part in std::mem::take(&mut opc.parts) {
        if xlsx_part_is_xml(&part.path, &part.content_type) {
            let text = String::from_utf8(part.bytes).map_err(|_| XlsxError::Xml { part: part.path.clone(), detail: "not valid utf-8".into() })?;
            let document = xml_document_from_text(&text).map_err(|detail| XlsxError::Xml { part: part.path.clone(), detail })?;
            xml_parts.push(XlsxXmlPart { path: part.path, content_type: part.content_type, document });
        } else {
            binary_parts.push(part);
        }
    }
    opc.parts = binary_parts;
    let snapshot = XlsxSnapshot::from_parts(opc, xml_parts);
    snapshot.validate_authority()?;
    Ok(snapshot)
}
//#endregion 🔖️Codec

//#region 🔖️Sniff
/// 🕵️ Real xlsx sniff: OPC-shaped bytes whose root officeDocument relationship resolves to a
/// SpreadsheetML workbook content type, independent from the package author's chosen part path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_xlsx_bytes(data: &[u8]) -> bool {
    let Ok(opc) = opc::decode_opc(data) else { return false };
    let path = opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).or_else(|| opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT_STRICT));
    path.and_then(|path| opc.content_types.resolve(&path)).is_some_and(|content_type| content_type.contains("spreadsheetml.sheet.main"))
}
//#endregion 🔖️Sniff
