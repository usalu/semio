//! 📥️ Deserialize `stdio.ifc` from stdio.txt.
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::STDIO_IFC_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_txt::TxtSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &TxtSnapshot) -> Result<IfcSnapshot, semio_framework_diagnostic::TextError> {
    let document = semio_s_artifact_stdio_contract::part21::parse_part21(from.to_body().trim()).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ifc parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    Ok(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::from_part21_document(STDIO_IFC_DOCUMENT_SCHEMA, &document))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_text(text: &str) -> Result<IfcSnapshot, semio_framework_diagnostic::TextError> {
    deserialize(&<TxtSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?)
}
