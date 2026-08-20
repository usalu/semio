//! 📤️ Serialize `stdio.csv` to stdio.txt.

use crate::artifacts::csv::CsvSnapshot;
use crate::artifacts::txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 📤️ Encode csv into a TxtSnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &CsvSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::csv::schema::snapshot::encode_csv(from);
    Ok(TxtSnapshot::from_body(&text))
}

/// 📤️ Encode as txt DSL.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_text(from: &CsvSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
