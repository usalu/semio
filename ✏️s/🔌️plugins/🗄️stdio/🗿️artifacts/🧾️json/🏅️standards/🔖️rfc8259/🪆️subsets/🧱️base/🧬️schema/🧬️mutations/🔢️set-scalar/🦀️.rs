//! 🧬️ Direct set-scalar mutation owner.
use crate::schema::diff::{JsonDiff, JsonValueDiff};
use crate::schema::mutation_support::{diff_at_path, resolve, JsonPath};
use crate::schema::snapshot::JsonValue;
use crate::JsonSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetScalarMutation {
    pub path: JsonPath,
    pub value: JsonValue,
}

pub type SetScalarPayload = SetScalarMutation;

impl protocol::MutationKind<JsonSnapshot, super::JsonMutation> for SetScalarMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "scalar", kind: "set-scalar", record: "SetScalar" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<JsonDiff> {
        protocol::MutationOutcome::new(match resolve(&base.value, &self.path) {
            Some(old) if old != &self.value => diff_at_path(&self.path, Some(JsonValueDiff::Replace { value: self.value.clone() })),
            _ => JsonDiff::default(),
        })
    }

    fn inverse(&self, base: &JsonSnapshot) -> Result<Vec<super::JsonMutation>, semio_framework_value::ValueError> {
        Ok(match resolve(&base.value, &self.path) {
            Some(old) if old != &self.value => vec![super::JsonMutation::SetScalar(Self { path: self.path.clone(), value: old.clone() })],
            _ => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Scalar", "Skalar setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["set-scalar".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
