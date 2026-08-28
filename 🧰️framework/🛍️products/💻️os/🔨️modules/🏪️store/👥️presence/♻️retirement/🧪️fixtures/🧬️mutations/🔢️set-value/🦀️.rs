use super::{Value, ValueMutation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-value")]
pub struct SetValue { pub n: i32 }

impl crate::os_spr::MutationKind<Value, ValueMutation> for SetValue {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "set", entity: "value", kind: "set-value", record: "SetValue" };
    fn diff(&self, _base: &Value) -> crate::os_spr::MutationOutcome<Value> {
        crate::os_spr::MutationOutcome::new(Value(self.n))
    }
    fn inverse(&self, base: &Value) -> Vec<ValueMutation> {
        vec![ValueMutation::SetValue(Self { n: base.0 })]
    }
    fn label(&self) -> String { "Set Value".into() }
    fn target(&self) -> Vec<String> { vec!["value".into()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_leaf_contract() { super::super::assert_fixture_descriptor::<SetValue>(include_str!("🔣️.json")); }
}
