//! 🌳 `set-top-level` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTopLevel {
    pub(crate) root: JsonIJsonRoot,
}

impl protocol::MutationKind<JsonSnapshot, JsonIJsonMutation> for SetTopLevel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "top-level", kind: "set-top-level", record: "SetTopLevel" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonIJsonMutation as Mutation<JsonSnapshot>>::Diff> {
        let Self { root } = self;
        delegated(Ok(JsonMutation::SetScalar(SetScalarPayload { path: Vec::new(), value: root.to_value() })), base)
    }
    fn inverse(&self, base: &JsonSnapshot) -> Result<Vec<JsonIJsonMutation>, semio_framework_value::ValueError> {
        Ok(JsonIJsonRoot::from_value(&base.value).map(|root| vec![JsonIJsonMutation::SetTopLevel(Self { root })]).unwrap_or_default())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set top level", "Wurzelwert setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
