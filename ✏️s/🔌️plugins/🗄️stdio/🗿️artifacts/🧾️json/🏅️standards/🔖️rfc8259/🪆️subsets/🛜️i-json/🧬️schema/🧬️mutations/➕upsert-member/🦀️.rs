//! ➕ `upsert-member` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpsertMember {
    pub(crate) path: JsonPath,
    pub(crate) key: String,
    pub(crate) value: JsonValue,
    /// 🧭️ Where a newly added member lands among its object's members; `None` appends.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
}

impl protocol::MutationKind<JsonSnapshot, JsonIJsonMutation> for UpsertMember {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "member", kind: "upsert-member", record: "UpsertMember" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonIJsonMutation as Mutation<JsonSnapshot>>::Diff> {
        let Self { path, key, value, index } = self;
        delegated(Ok(JsonMutation::SetMember(SetMemberPayload { path: path.clone(), key: key.clone(), value: value.clone(), index: *index })), base)
    }
    fn inverse(&self, base: &JsonSnapshot) -> Result<Vec<JsonIJsonMutation>, semio_framework_value::ValueError> {
        let Self { path, key, .. } = self;
        Ok(match resolve(&base.value, path) {
            Some(JsonValue::Object { members }) => match members.iter().find(|member| &member.key == key) {
                Some(existing) => vec![JsonIJsonMutation::UpsertMember(Self { path: path.clone(), key: key.clone(), value: existing.value.clone(), index: None })],
                None => vec![JsonIJsonMutation::RemoveMember(remove_member::RemoveMember { path: path.clone(), key: key.clone() })],
            },
            _ => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Upsert member", "Eigenschaft einfügen oder aktualisieren")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
