//! 📤️ Serialize `stdio.ifc` to stdio.txt.
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &IfcSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let bytes = crate::standards::v4::subsets::any::io::encode_ifc4_document(&semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::to_part21_document(from)).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error)))?;
    Ok(TxtSnapshot::from_body(std::str::from_utf8(&bytes).expect("IFC4 writer emits UTF-8")))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_text(from: &IfcSnapshot) -> Result<String, store::PackError> {
    Ok(pack::snapshot::ArtifactDsl::print_dsl(&serialize(from)?))
}
