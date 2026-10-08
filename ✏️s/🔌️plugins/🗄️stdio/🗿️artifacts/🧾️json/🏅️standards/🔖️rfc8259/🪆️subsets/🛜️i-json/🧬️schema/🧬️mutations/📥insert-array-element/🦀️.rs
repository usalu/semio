//! 📥 `insert-array-element` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertArrayElement {
    pub(crate) path: JsonPath,
    pub(crate) index: usize,
    pub(crate) value: JsonValue,
}

impl protocol::MutationKind<JsonSnapshot, JsonIJsonMutation> for InsertArrayElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "array-element", kind: "insert-array-element", record: "InsertArrayElement" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonIJsonMutation as Mutation<JsonSnapshot>>::Diff> {
        let Self { path, index, value } = self;
        delegated(Ok(JsonMutation::InsertArrayElement(InsertArrayElementPayload { path: path.clone(), index: *index, value: value.clone() })), base)
    }
    fn inverse(&self, base: &JsonSnapshot) -> Result<Vec<JsonIJsonMutation>, semio_framework_value::ValueError> {
        let Self { path, index, .. } = self;
        Ok(match resolve(&base.value, path) {
            Some(JsonValue::Array { items }) => vec![JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path: path.clone(), index: (*index).min(items.len()) })],
            _ => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert array element", "Array-Element einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
