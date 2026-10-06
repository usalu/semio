//! 📝️ Text representation codec surface for `stdio.binary` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_raw::subsets::any::schema::mutations::*;
use crate::schema::diff::{diff_set_snapshot, BinaryDiff, ByteSplice};
use crate::BinarySnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};

/// 🎙️ Handcrafted `OpText` (P6: `semio_framework_dsl_record_derive::DslEnum` emits `DslVariants` only) — one-line grammar via
/// the derived `RecordSpec`/`DslVariants`. Body is the same ~15-line shape every `DslOps`-derived
/// enum's `OpText` impl uses (see `SpaceMutation`, `FlowMutationDsl` for the framework-side
/// precedent this copies verbatim).
impl OpText for BinaryMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let spec = (spec_fn.ordinary)();
            let wire_keyword = spec.keyword.as_deref().unwrap_or(keyword);
            let probe = format!("{} ", wire_keyword);
            if line == wire_keyword || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &spec, &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown operation line '{line}'"), semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}
}
pub use mutations_codec::*;
