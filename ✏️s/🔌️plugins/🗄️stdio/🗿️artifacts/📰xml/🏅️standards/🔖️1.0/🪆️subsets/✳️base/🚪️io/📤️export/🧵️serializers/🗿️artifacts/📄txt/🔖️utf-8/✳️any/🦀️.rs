//! 📤️ Serialize `stdio.xml` to stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::xml::XmlSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 📤️ Encode xml into a TxtSnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &XmlSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = String::from_utf8(from.export_utf8().map_err(store::PackError::Schema)?).map_err(|error| store::PackError::Schema(error.to_string()))?;
    Ok(TxtSnapshot::from_body(&text))
}

/// 📤️ Encode as txt DSL.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_text(from: &XmlSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
