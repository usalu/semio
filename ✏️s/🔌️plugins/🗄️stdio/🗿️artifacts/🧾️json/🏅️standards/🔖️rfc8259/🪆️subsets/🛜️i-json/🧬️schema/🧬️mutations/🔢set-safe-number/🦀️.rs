//! 🔢 `set-safe-number` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSafeNumber {
    pub(crate) path: JsonPath,
    pub(crate) lexeme: String,
}

impl protocol::MutationKind<JsonSnapshot, JsonIJsonMutation> for SetSafeNumber {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "safe-number", kind: "set-safe-number", record: "SetSafeNumber" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonIJsonMutation as Mutation<JsonSnapshot>>::Diff> {
        let Self { path, lexeme } = self;
        if !matches!(resolve(&base.value, path), Some(JsonValue::Number { .. })) {
            return protocol::MutationOutcome::error(CODE_TARGET_MISSING, "set-safe-number: the addressed path does not hold a number, and this verb writes a number over a number so that its own inverse is always another set-safe-number", target_of(path));
        }
        if !is_safe_number_lexeme(lexeme) {
            return protocol::MutationOutcome::fatal(
                CODE_INVARIANT,
                format!("set-safe-number: integer {lexeme} exceeds ±{MAX_SAFE_INTEGER_MAGNITUDE} = ±(2^53-1) and is not exactly representable as an IEEE-754 double -- RFC 7493 §2.2 forbids it in I-JSON"),
                target_of(path),
            );
        }
        delegated(Ok(JsonMutation::SetScalar(SetScalarPayload { path: path.clone(), value: JsonValue::Number { lexeme: lexeme.clone() } })), base)
    }
    fn inverse(&self, base: &JsonSnapshot) -> Result<Vec<JsonIJsonMutation>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        Ok(match resolve(&base.value, path) {
            Some(JsonValue::Number { lexeme }) => vec![JsonIJsonMutation::SetSafeNumber(Self { path: path.clone(), lexeme: lexeme.clone() })],
            _ => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set safe number", "Sichere Zahl setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
