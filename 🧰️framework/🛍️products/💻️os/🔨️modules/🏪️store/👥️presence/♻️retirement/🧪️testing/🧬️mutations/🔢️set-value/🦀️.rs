use super::{Value, ValueMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl_derive::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-value")]
pub struct SetValue {
    pub n: i32,
}

impl semio_framework_os_command::MutationKind<Value, ValueMutation> for SetValue {
    const SEMANTICS: semio_framework_os_command::SemanticDescriptor = semio_framework_os_command::SemanticDescriptor { verb: "set", entity: "value", kind: "set-value", record: "SetValue" };
    fn diff(&self, _base: &Value) -> protocol::MutationOutcome<Value> {
        protocol::MutationOutcome::new(Value(self.n))
    }
    fn inverse(&self, base: &Value) -> Result<Vec<ValueMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![ValueMutation::SetValue(Self { n: base.0 })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Value", "Wert setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["value".into()]
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🧪️fixture-mutations-set-value/🦀️.rs"]
mod tests;
