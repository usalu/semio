//! ser json to txt
use crate::schema::snapshot::write_json_pretty;
use crate::JsonSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &JsonSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = write_json_pretty(&from.value);
    Ok(TxtSnapshot::from_body(&text))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_text(from: &JsonSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
