//! 🔤 `set-string` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetString {
    pub(crate) path: JsonPath,
    pub(crate) value: String,
}

impl protocol::MutationKind<JsonSnapshot, JsonIJsonMutation> for SetString {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "string", kind: "set-string", record: "SetString" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonIJsonMutation as Mutation<JsonSnapshot>>::Diff> {
        let Self { path, value } = self;
        if !matches!(resolve(&base.value, path), Some(JsonValue::String { .. })) {
            return protocol::MutationOutcome::error(CODE_TARGET_MISSING, "set-string: the addressed path does not hold a string, and this verb writes a string over a string so that its own inverse is always another set-string", target_of(path));
        }
        if let Some(offending) = value.chars().find(|c| is_unicode_noncharacter(*c)) {
            return protocol::MutationOutcome::fatal(CODE_INVARIANT, format!("set-string: the value carries the Unicode noncharacter U+{:04X} -- RFC 7493 §2.4 forbids noncharacters in I-JSON text", offending as u32), target_of(path));
        }
        delegated(Ok(JsonMutation::SetScalar(SetScalarPayload { path: path.clone(), value: JsonValue::String { value: value.clone() } })), base)
    }
    fn inverse(&self, base: &JsonSnapshot) -> Result<Vec<JsonIJsonMutation>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        Ok(match resolve(&base.value, path) {
            Some(JsonValue::String { value }) => vec![JsonIJsonMutation::SetString(Self { path: path.clone(), value: value.clone() })],
            _ => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set string", "Zeichenkette setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
