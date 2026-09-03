//! 📤️ Serialize `stdio.md` to stdio.txt.

use crate::artifacts::md::MdSnapshot;
use crate::artifacts::txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 📤️ Encode md into a TxtSnapshot (rendered markdown text -- see `render_markdown_blocks`'s
/// doc comment for the documented normal form this renders to).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &MdSnapshot) -> Result<TxtSnapshot, store::PackError> {
    Ok(TxtSnapshot::from_body(&crate::artifacts::md::standards::v_commonmark::subsets::any::io::export::serializers::render_markdown_blocks(&from.blocks)))
}

/// 📤️ Encode as txt DSL.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_text(from: &MdSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
