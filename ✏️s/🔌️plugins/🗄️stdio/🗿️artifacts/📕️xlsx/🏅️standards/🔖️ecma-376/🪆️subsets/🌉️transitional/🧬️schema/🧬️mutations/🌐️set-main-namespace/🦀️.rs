//! 🔩️ `set-main-namespace` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetMainNamespace {
    pub(crate) namespace: String,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxTransitionalMutation> for SetMainNamespace {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "main-namespace", kind: "set-main-namespace", record: "SetMainNamespace" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
        protocol::MutationOutcome::new(diff_retarget_namespace(base, MAIN_NAMESPACES, &self.namespace))
    }

    fn inverse(&self, base: &XlsxSnapshot) -> Result<Vec<XlsxTransitionalMutation>, semio_framework_value::ValueError> {
        Ok(namespace_inverse(base))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set main namespace", "Hauptnamensraum setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
