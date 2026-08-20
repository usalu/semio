//! 📥️ Deserialize `stdio.stl` from stdio.txt.

use crate::artifacts::stl::StlSnapshot;
use crate::artifacts::txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 📥 Parse stl text into a StlSnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &TxtSnapshot) -> Result<StlSnapshot, store::TextError> {
    crate::artifacts::stl::engine::decode_stl_ascii(&from.to_body()).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

/// 📥 Parse DSL/text bytes via txt then stl.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_text(text: &str) -> Result<StlSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
