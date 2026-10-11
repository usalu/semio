//! ⚙️ `set-drawing-namespace` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDrawingNamespace {
    pub(crate) namespace: String,
}

impl protocol::MutationKind<PptxSnapshot, PptxTransitionalMutation> for SetDrawingNamespace {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "drawing-namespace", kind: "set-drawing-namespace", record: "SetDrawingNamespace" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        protocol::MutationOutcome::new(diff_retarget_namespace(base, DRAWING_NAMESPACES, &self.namespace))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxTransitionalMutation>, semio_framework_value::ValueError> {
        Ok(drawing_namespace_inverse(base))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set drawing namespace", "Zeichnungsnamensraum setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
