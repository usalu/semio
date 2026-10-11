//! 🔗️ `set-style-based-on` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStyleBasedOn {
    pub(crate) id: String,
    pub(crate) based_on: Option<String>,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for SetStyleBasedOn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "style-based-on", kind: "set-style-based-on", record: "SetStyleBasedOn" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
        optional_plan_outcome(style_based_on_plan(base, &self.id, self.based_on.as_deref()))
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxMutation>, semio_framework_value::ValueError> {
        Ok(optional_plan_inverse(style_based_on_plan(base, &self.id, self.based_on.as_deref())))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set style based on", "Basis der Formatvorlage setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
