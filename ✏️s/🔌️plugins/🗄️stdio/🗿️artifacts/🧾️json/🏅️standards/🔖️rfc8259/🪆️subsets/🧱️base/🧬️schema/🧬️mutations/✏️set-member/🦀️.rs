//! 🧬️ Direct set-member mutation owner.
use crate::schema::diff::{JsonDiff, JsonObjectAdded, JsonObjectDiff, JsonObjectModified, JsonValueDiff};
use crate::schema::mutation_support::{diff_at_path, resolve, JsonPath};
use crate::schema::snapshot::JsonValue;
use crate::JsonSnapshot;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetMemberMutation {
    pub path: JsonPath,
    pub key: String,
    pub value: JsonValue,
    /// 🧭️ Where a newly added member lands among its object's members; `None` appends.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

pub type SetMemberPayload = SetMemberMutation;

/// 🧱️ The one row this kind writes for a member that already exists: the new scalar in its own kind's field when the kind is stable, else the whole new value.
fn replaced_row(existing: &JsonValue, new: &JsonValue) -> Option<JsonValueDiff> {
    match (existing, new) {
        _ if existing == new => None,
        (JsonValue::Bool { .. }, JsonValue::Bool { value }) => Some(JsonValueDiff::Bool { value: *value }),
        (JsonValue::Number { .. }, JsonValue::Number { lexeme }) => Some(JsonValueDiff::Number { lexeme: lexeme.clone() }),
        (JsonValue::String { .. }, JsonValue::String { value }) => Some(JsonValueDiff::String { value: value.clone() }),
        _ => Some(JsonValueDiff::Replace { value: new.clone() }),
    }
}

impl protocol::MutationKind<JsonSnapshot, super::JsonMutation> for SetMemberMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "member", kind: "set-member", record: "SetMember" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<JsonDiff> {
        protocol::MutationOutcome::new(match resolve(&base.value, &self.path) {
            Some(JsonValue::Object { members }) => match members.iter().find(|member| member.key == self.key) {
                Some(existing) => {
                    let leaf = replaced_row(&existing.value, &self.value);
                    diff_at_path(&self.path, leaf.map(|diff| JsonValueDiff::Object { diff: JsonObjectDiff { removed: Vec::new(), added: Vec::new(), modified: vec![JsonObjectModified { key: self.key.clone(), diff }] } }))
                }
                None => diff_at_path(
                    &self.path,
                    Some(JsonValueDiff::Object { diff: JsonObjectDiff { removed: Vec::new(), modified: Vec::new(), added: vec![JsonObjectAdded { index: self.index.unwrap_or(members.len()).min(members.len()), key: self.key.clone(), item: self.value.clone() }] } }),
                ),
            },
            _ => JsonDiff::default(),
        })
    }

    fn inverse(&self, base: &JsonSnapshot) -> Result<Vec<super::JsonMutation>, semio_framework_value::ValueError> {
        Ok(match resolve(&base.value, &self.path) {
            Some(JsonValue::Object { members }) => match members.iter().find(|member| member.key == self.key) {
                Some(existing) if existing.value == self.value => Vec::new(),
                Some(existing) => vec![super::JsonMutation::SetMember(Self { path: self.path.clone(), key: self.key.clone(), value: existing.value.clone(), index: None })],
                None => vec![super::JsonMutation::RemoveMember(super::RemoveMemberMutation { path: self.path.clone(), key: self.key.clone() })],
            },
            _ => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Member", "Eigenschaft setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["set-member".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
