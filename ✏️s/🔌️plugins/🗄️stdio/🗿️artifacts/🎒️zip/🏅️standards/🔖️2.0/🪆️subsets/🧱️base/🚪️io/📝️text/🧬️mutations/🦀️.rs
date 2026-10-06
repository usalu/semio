//! 📝️ Text representation codec surface for `stdio.zip` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v2_0::subsets::base::schema::mutations::*;
use crate::schema::diff::{self, ZipDiff};
use crate::schema::snapshot::ZipEntry;
use crate::ZipSnapshot;

impl protocol::OpText for ZipMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec) in &variants {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = semio_framework_dsl_record::parse(line, &(spec.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits { max_bytes: 64 * 1024 * 1024, ..semio_framework_diagnostic::Limits::default() }, mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown ZIP operation '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }

    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec = variants.iter().find(|(name, _)| name == &keyword).map(|(_, spec)| *spec).expect("ZIP operation spec");
        semio_framework_dsl_record::print(&record, &(spec.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}
}
pub use mutations_codec::*;
