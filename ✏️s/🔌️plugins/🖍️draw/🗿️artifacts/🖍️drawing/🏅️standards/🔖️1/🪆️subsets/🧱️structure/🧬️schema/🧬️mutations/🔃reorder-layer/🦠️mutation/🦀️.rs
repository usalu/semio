//! 🔃 Drawing mutation — `ReorderLayer`: repositions (and optionally re-parents) an existing layer to
//! a FINAL-state `(parent_id, index)` address — never spatial.
use crate::diff::DrawingDiff;
use crate::mutations::DrawingMutation;
use crate::DrawingSnapshot;

//#region 🔖️Mutation
/// 🔃 `reorder-layer` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "reorder-layer")]
pub struct ReorderLayer {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub parent_id: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
    pub index: usize,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn reorder_layer(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, parent_id: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>, index: usize) -> DrawingMutation {
    DrawingMutation::ReorderLayer(ReorderLayer { layer_id, parent_id, index })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for ReorderLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "layer", kind: "reorder-layer", record: "ReorderedLayer" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reorder layer \"{}\"", self.layer_id), &format!("Reihenfolge von Ebene \"{}\" ändern", self.layer_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
