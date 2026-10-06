//! 🚧 scaffolded by W1b — text representation marker for `stdio.wav.snapshot`. Full grammar-backed
//! parse/print lands in W2/W3.
pub const TEXT_MARKER: &str = "stdio.wav.snapshot";

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::riff_pcm::subsets::any::schema::snapshot::*;
use framework_schema::ArtifactSchema;

/// 🎼️ Encodes every owned logical snapshot field independently of ordinary file syntax.
impl store::ArtifactDsl for WavSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_WAV_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits { max_bytes: 32 * 1024 * 1024, ..semio_framework_diagnostic::Limits::default() }, mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;
