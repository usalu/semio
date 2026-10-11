//! 🧩️ `set-conformance-attribute` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetConformanceAttribute {
    pub(crate) value: String,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxTransitionalMutation> for SetConformanceAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "conformance-attribute", kind: "set-conformance-attribute", record: "SetConformanceAttribute" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
        protocol::MutationOutcome::new(diff_conformance_attribute(base, Some(&self.value)))
    }

    fn inverse(&self, base: &XlsxSnapshot) -> Result<Vec<XlsxTransitionalMutation>, semio_framework_value::ValueError> {
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
