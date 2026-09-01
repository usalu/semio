//#region 🧬️DependencyContributionMutationRoster
//! 🧬️ Transparent builder contribution mutation roster.
use super::{DependencyTestDiff, DependencyTestSnapshot};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "➕️add-value/🦀️.rs"]
mod add_value;
pub use add_value::AddValue;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::Mutations, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = DependencyTestSnapshot, diff = DependencyTestDiff, schema = "dep-target.document")]
pub enum DependencyTestOp { AddValue(AddValue) }

impl protocol::OpText for DependencyTestOp {
    fn parse_op(line: &str) -> Result<Self, dsl::TextError> {
        for (keyword, spec_fn) in <Self as dsl::DslVariants>::variants() {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(&keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown builder operation '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let spec = <Self as dsl::DslVariants>::variants().into_iter().find(|(name, _)| name == &keyword).map(|(_, spec)| spec()).expect("owned operation schema");
        dsl::print(&record, &spec, dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for DependencyTestOp {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}
//#endregion 🧬️DependencyContributionMutationRoster
