//! ✋️ Drawing mutation — `DragLayers`: a relative, parametric drag of a set of layers by one world-space offset. The
//! gesture's own inputs (which layers, which offset) are the payload, so editing the drag in history re-derives every
//! layer transform from whatever base it replays on.
use crate::mutations::{drawing_label_layers, drawing_label_number, DrawingMutation};
use crate::DrawingSnapshot;

//#region 🔖️Mutation
/// ✋️ `drag-layers` payload — the dragged layer ids and the world-space offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "drag-layers")]
pub struct DragLayers {
    pub targets: semio_framework_value::list::PagedList<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, {usize::MAX}>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_layers(targets: semio_framework_value::list::PagedList<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, {usize::MAX}>, dx: f64, dy: f64) -> DrawingMutation {
    DrawingMutation::DragLayers(DragLayers { targets, dx, dy })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for DragLayers {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "layers", kind: "drag-layers", record: "DraggedLayers" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de)) = (drawing_label_number(self.dx), drawing_label_number(self.dy));
        let (en, de) = drawing_label_layers(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {en} by ({dx_en}, {dy_en})"), &format!("{de} um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.iter().map(|target| target.to_string_owner()).collect()
    }
}
//#endregion 🔖️Mutation
