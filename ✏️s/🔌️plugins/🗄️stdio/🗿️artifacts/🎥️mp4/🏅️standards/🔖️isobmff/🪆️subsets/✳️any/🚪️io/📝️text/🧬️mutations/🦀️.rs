//! 📝️ Text representation codec surface for `stdio.mp4` (mutations) — the real op text
//! codec is `protocol::OpText` in ../🦀️.rs (`print_op`/`parse_op`, one structured
//! named-record line per op); this leaf documents that shape via the grammar file below.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::isobmff::subsets::any::schema::mutations::*;
use crate::standards::isobmff::subsets::any::schema::diff::{IndexedAdded, IndexedDiff, IndexedModified, Mp4Diff, Mp4SampleDiff, Mp4TrackDiff};
use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Snapshot, Mp4Track};
#[cfg(test)]
use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Movie, Mp4TrackMetadata};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

/// 🎙️ Structured operation text through the shared `DslVariants` record machinery.
impl OpText for Mp4Mutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits { max_bytes: 32 * 1024 * 1024, ..semio_framework_diagnostic::Limits::default() }, mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(candidate, _)| candidate == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}
}
pub use mutations_codec::*;
