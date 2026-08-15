//! 📥️ Deserialize `stdio.obj` from stdio.txt.

use crate::artifacts::obj::ObjSnapshot;
use crate::artifacts::txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse obj text into a ObjSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<ObjSnapshot, store::TextError> {
    crate::artifacts::obj::engine::decode_obj(&from.to_body()).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

/// 📥 Parse DSL/text bytes via txt then obj.
pub fn deserialize_text(text: &str) -> Result<ObjSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
