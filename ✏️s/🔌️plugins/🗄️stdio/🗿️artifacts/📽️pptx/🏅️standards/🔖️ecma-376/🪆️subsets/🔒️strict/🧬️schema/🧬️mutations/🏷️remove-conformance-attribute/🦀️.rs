//! 🏷️ `remove-conformance-attribute` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveConformanceAttribute {}

impl protocol::MutationKind<PptxSnapshot, PptxStrictMutation> for RemoveConformanceAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "conformance-attribute", kind: "remove-conformance-attribute", record: "RemoveConformanceAttribute" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        protocol::MutationOutcome::new(diff_conformance_attribute(base, None))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxStrictMutation>, semio_framework_value::ValueError> {
        Ok(if conformance_attribute(base).is_some() { conformance_attribute_inverse(base, false) } else { Vec::new() })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove conformance attribute", "Konformitätsattribut entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
