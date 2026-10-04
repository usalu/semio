//! 📤️ Serialize `stdio.ifc.2x3` to stdio.txt.

use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &Ifc2x3Snapshot) -> Result<TxtSnapshot, store::PackError> {
    let bytes = crate::standards::v2x3::engine::encode_ifc2x3(from).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))?;
    let text = String::from_utf8(bytes).map_err(|e| store::PackError::from(semio_framework_value::ValueError::from(e)))?;
    Ok(TxtSnapshot::from_body(&text))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_text(from: &Ifc2x3Snapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
