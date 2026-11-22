//! 🧩️ SpreadsheetML package decoding and byte-shape sniffing.

use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::refusal::XlsxError;
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::{workbook_media_type_matches, REL_TYPE_OFFICE_DOCUMENT_STRICT};
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::xlsx_part_is_xml;
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxXmlPart;
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text;
use semio_s_artifact_stdio_zip::opc::{self, REL_TYPE_OFFICE_DOCUMENT};

/// 📥️ Reads either an internal XLSX snapshot or the original physical workbook.
pub fn decode_xlsx_source(data: &[u8]) -> Result<XlsxSnapshot, store::PackError> {
    if data.starts_with(&store::semio_format::BINARY_MAGIC) {
        return <XlsxSnapshot as pack::snapshot::ArtifactPack>::decode_pack(data);
    }
    decode_xlsx(data).map_err(|error| store::PackError::from(semio_framework_value::ValueError::from(error)))
}

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
    path.and_then(|path| opc.content_types.resolve(&path)).is_some_and(workbook_media_type_matches)
}
/// 🧭️ ECMA-376 subsets told apart by the root namespace of the workbook part: Strict declares `purl.oclc.org/ooxml`, Transitional declares `schemas.openxmlformats.org`.
const SUBSET_NAMESPACES: [(&str, &str); 2] = [
    ("strict", "http://purl.oclc.org/ooxml/spreadsheetml/main"),
    ("transitional", "http://schemas.openxmlformats.org/spreadsheetml/2006/main"),
];

/// 🧭️ Names the ECMA-376 subset a physical workbook is written in, from the namespace of its main workbook part (`None` for bytes that are no OPC package, have no workbook part or declare neither namespace).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_xlsx_subset(data: &[u8]) -> Option<&'static str> {
    let opc = opc::decode_opc(data).ok()?;
    let path = opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).or_else(|| opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT_STRICT))?;
    let part = opc.parts.iter().find(|part| part.path.trim_start_matches('/') == path.trim_start_matches('/'))?;
    let head = &part.bytes[..part.bytes.len().min(4096)];
    SUBSET_NAMESPACES.iter().find(|(_, namespace)| head.windows(namespace.len()).any(|window| window == namespace.as_bytes())).map(|(subset, _)| *subset)
}
//#endregion 🔖️Sniff
