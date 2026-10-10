//! 🧬️ Direct remove-member mutation owner.
use crate::schema::diff::{JsonDiff, JsonObjectDiff, JsonValueDiff};
use crate::schema::mutation_support::{diff_at_path, resolve, JsonPath};
use crate::schema::snapshot::JsonValue;
use crate::JsonSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveMemberMutation {
    pub path: JsonPath,
    pub key: String,
}

pub type RemoveMemberPayload = RemoveMemberMutation;

impl protocol::MutationKind<JsonSnapshot, super::JsonMutation> for RemoveMemberMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "member", kind: "remove-member", record: "RemovedMember" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<JsonDiff> {
        protocol::MutationOutcome::new(match resolve(&base.value, &self.path) {
            Some(JsonValue::Object { members }) if members.iter().any(|member| member.key == self.key) => {
                diff_at_path(&self.path, Some(JsonValueDiff::Object { diff: JsonObjectDiff { removed: vec![self.key.clone()], modified: Vec::new(), added: Vec::new() } }))
            }
            _ => JsonDiff::default(),
        })
    }

    fn inverse(&self, base: &JsonSnapshot) -> Result<Vec<super::JsonMutation>, semio_framework_value::ValueError> {
        Ok(match resolve(&base.value, &self.path) {
            Some(JsonValue::Object { members }) => members
                .iter()
                .position(|member| member.key == self.key)
                .map(|position| vec![super::JsonMutation::SetMember(super::SetMemberMutation { path: self.path.clone(), key: self.key.clone(), value: members[position].value.clone(), index: Some(position) })])
                .unwrap_or_default(),
            _ => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove Member", "Eigenschaft entfernen")
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-member".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
