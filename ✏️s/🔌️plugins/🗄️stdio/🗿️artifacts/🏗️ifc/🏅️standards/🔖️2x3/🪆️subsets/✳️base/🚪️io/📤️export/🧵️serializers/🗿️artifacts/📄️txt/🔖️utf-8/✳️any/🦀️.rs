//! 📤️ Serialize `stdio.ifc.2x3` to stdio.txt.

use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::txt::TxtSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &Ifc2x3Snapshot) -> Result<TxtSnapshot, store::PackError> {
    let bytes = crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(from).map_err(store::PackError::Schema)?;
    let text = String::from_utf8(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(TxtSnapshot::from_body(&text))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_text(from: &Ifc2x3Snapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
