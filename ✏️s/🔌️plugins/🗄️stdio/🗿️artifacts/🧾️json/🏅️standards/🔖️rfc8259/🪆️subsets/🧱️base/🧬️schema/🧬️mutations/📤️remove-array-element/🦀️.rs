//! 🧬️ Direct remove-array-element mutation owner.
use crate::schema::diff::{JsonArrayDiff, JsonDiff, JsonValueDiff};
use crate::schema::mutation_support::{diff_at_path, resolve, JsonPath};
use crate::schema::snapshot::JsonValue;
use crate::JsonSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveArrayElementMutation {
    pub path: JsonPath,
    pub index: usize,
}

pub type RemoveArrayElementPayload = RemoveArrayElementMutation;

impl protocol::MutationKind<JsonSnapshot, super::JsonMutation> for RemoveArrayElementMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "array-element", kind: "remove-array-element", record: "RemovedArrayElement" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<JsonDiff> {
        protocol::MutationOutcome::new(match resolve(&base.value, &self.path) {
            Some(JsonValue::Array { items }) if self.index < items.len() => diff_at_path(&self.path, Some(JsonValueDiff::Array { diff: JsonArrayDiff { removed: vec![self.index], modified: Vec::new(), added: Vec::new() } })),
            _ => JsonDiff::default(),
        })
    }

    fn inverse(&self, base: &JsonSnapshot) -> Result<Vec<super::JsonMutation>, semio_framework_value::ValueError> {
        Ok(match resolve(&base.value, &self.path) {
            Some(JsonValue::Array { items }) => items.get(self.index).map(|item| vec![super::JsonMutation::InsertArrayElement(super::InsertArrayElementMutation { path: self.path.clone(), index: self.index, value: item.clone() })]).unwrap_or_default(),
            _ => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove Array Element", "Array-Element entfernen")
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-array-element".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
