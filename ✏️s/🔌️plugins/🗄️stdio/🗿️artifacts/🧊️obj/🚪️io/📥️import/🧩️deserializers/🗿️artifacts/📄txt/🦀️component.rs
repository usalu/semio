//! 📥️ Deserialize `stdio.obj` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::obj::{ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse obj text into a ObjSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<ObjSnapshot, store::TextError> {
    let (vertices, faces) = crate::artifacts::obj::schema::snapshot::parse_obj_text(from.text.as_str())
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    Ok(ObjSnapshot { schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(), vertices, faces })
}

/// 📥 Parse DSL/text bytes via txt then obj.
pub fn deserialize_text(text: &str) -> Result<ObjSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
