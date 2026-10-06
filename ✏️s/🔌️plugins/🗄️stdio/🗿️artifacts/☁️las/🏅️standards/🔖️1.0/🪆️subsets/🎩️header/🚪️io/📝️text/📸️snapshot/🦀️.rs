//! 📝️ Text representation codec surface for `stdio.las` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type LasSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod pack_codec {
use crate::standards::v1_0::subsets::any::schema::snapshot::{LasHeader, LasPoint, LasSnapshot, LasVlr};
use crate::standards::v1_0::subsets::any::io::binary::snapshot::pack::*;
impl store::ArtifactDsl for LasSnapshot {
    const EXTENSION: &'static str = "las";
    fn envelope_id() -> &'static str { "stdio.las" }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map(|(_, body)| body).unwrap_or(text);
        let record = semio_framework_dsl_record::parse(body, &Snapshot::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Ok(Snapshot::__dsl_from_record(&record)?.into())
    }
    fn print_dsl(&self) -> String {
        let snapshot = Snapshot::from(self);
        let body = semio_framework_dsl_record::print(&snapshot.__dsl_to_record(), &Snapshot::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id("stdio.las", store::semio_format::Component::Dsl, 1).expect("valid LAS snapshot identity");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
