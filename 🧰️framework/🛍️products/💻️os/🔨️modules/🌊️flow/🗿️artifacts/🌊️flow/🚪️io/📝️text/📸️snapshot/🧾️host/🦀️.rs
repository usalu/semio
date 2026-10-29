//! 🚪️ Native text representation of the Flow host snapshot over the model's component codecs.


use crate::io;
use crate::{CameraJson, FlowHostSnapshot, WidgetLayout};

/// 📜️ Handcrafted ArtifactDsl (P6): derive no longer emits ArtifactDsl/ArtifactPack.
/// 📜️ Handcrafted ArtifactDsl (P6): the record derive does not emit the text envelope, so the persisted owner wraps its own derived record.
impl pack::snapshot::ArtifactDsl for FlowHostSnapshot {
    const EXTENSION: &'static str = "flow";

    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }

    fn parse_dsl(text: &str) -> Result<Self, crate::os_store::TextError> {
        let body = match crate::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid flow envelope id");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}
