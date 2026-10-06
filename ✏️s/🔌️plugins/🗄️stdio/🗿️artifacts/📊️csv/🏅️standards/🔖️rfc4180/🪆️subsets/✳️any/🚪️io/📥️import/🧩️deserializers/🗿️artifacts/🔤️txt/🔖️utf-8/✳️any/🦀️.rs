//! 📥️ Deserialize `stdio.csv` from stdio.txt.

use crate::CsvSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 📥 Parse csv text into a CsvSnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &TxtSnapshot) -> Result<CsvSnapshot, semio_framework_diagnostic::TextError> {
    Ok(crate::standards::v_rfc4180::subsets::any::io::text::snapshot::decode_csv_with(&from.to_body(), true))
}

/// 📥 Parse DSL/text bytes via txt then csv.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_text(text: &str) -> Result<CsvSnapshot, semio_framework_diagnostic::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
