//! 📝️ Text representation codec surface for `stdio.avi` (snapshot).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1_0::subsets::any::schema::snapshot::*;
use framework_schema::ArtifactSchema;

impl store::ArtifactDsl for AviSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_AVI_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record=semio_framework_dsl_record::parse(body,&Self::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits{max_bytes:272*1024*1024,..semio_framework_diagnostic::Limits::default()},mode:semio_framework_dsl_record::SourceMode::Document})?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;
