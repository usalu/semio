//! 📥️ Deserialize `stdio.stl` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::stl::StlSnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse stl text into a StlSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<StlSnapshot, store::TextError> {
    crate::artifacts::stl::engine::decode_stl_ascii(&from.to_body())
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

/// 📥 Parse DSL/text bytes via txt then stl.
pub fn deserialize_text(text: &str) -> Result<StlSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
