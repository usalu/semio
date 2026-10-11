//! 📤 `remove-array-element` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveArrayElement {
    pub(crate) path: JsonPath,
    pub(crate) index: usize,
}

impl protocol::MutationKind<JsonSnapshot, JsonIJsonMutation> for RemoveArrayElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "array-element", kind: "remove-array-element", record: "RemoveArrayElement" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonIJsonMutation as Mutation<JsonSnapshot>>::Diff> {
        let Self { path, index } = self;
        delegated(Ok(JsonMutation::RemoveArrayElement(RemoveArrayElementPayload { path: path.clone(), index: *index })), base)
    }
    fn inverse(&self, base: &JsonSnapshot) -> Result<Vec<JsonIJsonMutation>, semio_framework_value::ValueError> {
        let Self { path, index } = self;
        Ok(match resolve(&base.value, path) {
            Some(JsonValue::Array { items }) => items.get(*index).map(|item| vec![JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path: path.clone(), index: *index, value: item.clone() })]).unwrap_or_default(),
            _ => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove array element", "Array-Element entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
