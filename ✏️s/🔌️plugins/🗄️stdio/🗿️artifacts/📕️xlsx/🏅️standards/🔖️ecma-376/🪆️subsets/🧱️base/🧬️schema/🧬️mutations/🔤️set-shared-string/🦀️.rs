//! 🔠️ `set-shared-string` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSharedString {
    pub(crate) index: usize,
    pub(crate) value: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) node: Option<XmlNode>,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxMutation> for SetSharedString {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "shared-string", kind: "set-shared-string", record: "SetSharedString" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
        plan_outcome(canonical_edit::set_shared_string_plan(base, self.index, &self.value, self.node.as_ref()))
    }

    fn inverse(&self, base: &XlsxSnapshot) -> Result<Vec<XlsxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(canonical_edit::set_shared_string_plan(base, self.index, &self.value, self.node.as_ref())))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set shared string", "Gemeinsame Zeichenfolge setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
