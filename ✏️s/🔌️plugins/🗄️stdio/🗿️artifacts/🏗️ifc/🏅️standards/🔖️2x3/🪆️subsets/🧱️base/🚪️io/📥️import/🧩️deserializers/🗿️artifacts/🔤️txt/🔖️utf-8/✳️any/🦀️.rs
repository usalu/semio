//! 📥️ Deserialize `stdio.ifc.2x3` from stdio.txt.

use semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &TxtSnapshot) -> Result<Ifc2x3Snapshot, semio_framework_diagnostic::TextError> {
    crate::standards::v2x3::subsets::base::io::decode_ifc2x3(from.to_body().as_bytes()).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ifc2x3 parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_text(text: &str) -> Result<Ifc2x3Snapshot, semio_framework_diagnostic::TextError> {
    deserialize(&<TxtSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?)
}
