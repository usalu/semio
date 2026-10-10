//! 🧬️ Direct insert-array-element mutation owner.
use crate::schema::diff::{JsonArrayAdded, JsonArrayDiff, JsonDiff, JsonValueDiff};
use crate::schema::mutation_support::{diff_at_path, resolve, JsonPath};
use crate::schema::snapshot::JsonValue;
use crate::JsonSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertArrayElementMutation {
    pub path: JsonPath,
    pub index: usize,
    pub value: JsonValue,
}

pub type InsertArrayElementPayload = InsertArrayElementMutation;

impl protocol::MutationKind<JsonSnapshot, super::JsonMutation> for InsertArrayElementMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "array-element", kind: "insert-array-element", record: "InsertedArrayElement" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<JsonDiff> {
        protocol::MutationOutcome::new(match resolve(&base.value, &self.path) {
            Some(JsonValue::Array { items }) => {
                diff_at_path(&self.path, Some(JsonValueDiff::Array { diff: JsonArrayDiff { removed: Vec::new(), modified: Vec::new(), added: vec![JsonArrayAdded { index: self.index.min(items.len()), item: self.value.clone() }] } }))
            }
            _ => JsonDiff::default(),
        })
    }

    fn inverse(&self, base: &JsonSnapshot) -> Result<Vec<super::JsonMutation>, semio_framework_value::ValueError> {
        Ok(match resolve(&base.value, &self.path) {
            Some(JsonValue::Array { items }) => vec![super::JsonMutation::RemoveArrayElement(super::RemoveArrayElementMutation { path: self.path.clone(), index: self.index.min(items.len()) })],
            _ => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert Array Element", "Array-Element einfügen")
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-array-element".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
