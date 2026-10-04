//#region 🧬️TestDocumentMutationAggregate
#[path = "📝️set-test-count/🦀️.rs"]
pub mod set_count;
#[path = "🏷️set-label/🦀️.rs"]
pub mod set_label;
#[path = "🧒️set-slot-children/🦀️.rs"]
pub mod set_slot_children;
pub(crate) use set_count::SetCount;
pub(crate) use set_label::SetLabel;
pub(crate) use set_slot_children::SetSlotChildren;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[serde(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot=super::TestSnapshot,diff=super::TestDiff,schema="plugin.testkit.document")]
pub(crate) enum TestMutation {
    SetCount(SetCount),
    SetLabel(SetLabel),
    SetSlotChildren(SetSlotChildren),
}

impl protocol::OpText for TestMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        for (keyword, spec) in <Self as semio_framework_dsl_record::DslVariants>::variants() {
            let prefix = format!("{keyword} ");
            if line == keyword || line.starts_with(&prefix) {
                let body = line[keyword.len()..].trim_start();
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(&keyword, &semio_framework_dsl_record::parse(body, &(spec.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let spec = <Self as semio_framework_dsl_record::DslVariants>::variants().into_iter().find(|(candidate, _)| candidate == &keyword).expect("declared variant").1;
        let body = semio_framework_dsl_record::print(&record, &(spec.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for TestMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

#[cfg(test)]
#[path = "../../../../🧪️tests/🧬️test-app-document-mutation-roster-unit/🦀️.rs"]
mod tests;
//#endregion 🧬️TestDocumentMutationAggregate
