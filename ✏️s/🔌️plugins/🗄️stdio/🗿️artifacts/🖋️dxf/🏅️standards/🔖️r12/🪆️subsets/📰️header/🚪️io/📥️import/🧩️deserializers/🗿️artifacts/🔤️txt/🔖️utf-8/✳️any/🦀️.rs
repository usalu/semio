//! 📥️ Deserialize `stdio.dxf` from stdio.txt.

use crate::DxfSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 📥 Parse dxf text into a DxfSnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &TxtSnapshot) -> Result<DxfSnapshot, semio_framework_diagnostic::TextError> {
    crate::standards::v_r12::subsets::any::io::text::snapshot::parse_dxf_document(&from.to_body()).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}

/// 📥 Parse DSL/text bytes via txt then dxf.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_text(text: &str) -> Result<DxfSnapshot, semio_framework_diagnostic::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
