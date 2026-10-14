//! 📤️ Serialize `stdio.ifc` to stdio.txt.
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &IfcSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = semio_s_artifact_stdio_contract::part21::write_part21(&semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::to_part21_document(from));
    Ok(TxtSnapshot::from_body(&text))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_text(from: &IfcSnapshot) -> Result<String, store::PackError> {
    Ok(pack::snapshot::ArtifactDsl::print_dsl(&serialize(from)?))
}
