//! 🔖️ `remove-conformance-attribute` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveConformanceAttribute {}

impl protocol::MutationKind<DocxSnapshot, DocxTransitionalMutation> for RemoveConformanceAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "conformance-attribute", kind: "remove-conformance-attribute", record: "RemoveConformanceAttribute" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
        protocol::MutationOutcome::new(diff_conformance_attribute(base, None))
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxTransitionalMutation>, semio_framework_value::ValueError> {
        Ok(conformance_attribute_inverse(base, false))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove conformance attribute", "Konformitätsattribut entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
