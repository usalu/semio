//! 🧩️ `set-conformance-attribute` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetConformanceAttribute {
    pub(crate) value: String,
}

impl protocol::MutationKind<DocxSnapshot, DocxTransitionalMutation> for SetConformanceAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "conformance-attribute", kind: "set-conformance-attribute", record: "SetConformanceAttribute" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
        protocol::MutationOutcome::new(diff_conformance_attribute(base, Some(&self.value)))
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxTransitionalMutation>, semio_framework_value::ValueError> {
        Ok(conformance_attribute_inverse(base, true))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set conformance attribute", "Konformitätsattribut setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
