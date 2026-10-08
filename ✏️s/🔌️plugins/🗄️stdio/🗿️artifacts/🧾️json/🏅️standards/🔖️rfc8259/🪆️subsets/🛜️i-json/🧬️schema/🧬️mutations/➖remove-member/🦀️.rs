//! ➖ `remove-member` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveMember {
    pub(crate) path: JsonPath,
    pub(crate) key: String,
}

impl protocol::MutationKind<JsonSnapshot, JsonIJsonMutation> for RemoveMember {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "member", kind: "remove-member", record: "RemoveMember" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonIJsonMutation as Mutation<JsonSnapshot>>::Diff> {
        let Self { path, key } = self;
        delegated(Ok(JsonMutation::RemoveMember(RemoveMemberPayload { path: path.clone(), key: key.clone() })), base)
    }
    fn inverse(&self, base: &JsonSnapshot) -> Result<Vec<JsonIJsonMutation>, semio_framework_value::ValueError> {
        let Self { path, key } = self;
        Ok(match resolve(&base.value, path) {
            Some(JsonValue::Object { members }) => members
                .iter()
                .position(|member| &member.key == key)
                .map(|position| vec![JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: path.clone(), key: key.clone(), value: members[position].value.clone(), index: Some(position) })])
                .unwrap_or_default(),
            _ => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove member", "Eigenschaft entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
