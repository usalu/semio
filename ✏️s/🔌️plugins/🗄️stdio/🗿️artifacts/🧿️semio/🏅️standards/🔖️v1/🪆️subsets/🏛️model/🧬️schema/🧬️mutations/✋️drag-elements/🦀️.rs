//! ✋️ `drag-elements` — a relative drag of a set of model elements by one common world offset (ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12, §17.6, §20.15). The gesture's own inputs (which elements, which offset)
//! are the payload, so editing the drag in history re-derives every placement from whatever base it replays on. Editors that
//! compose this subset as a content child (cad's four model panes) yield one per gumball or tool translate.

use super::*;

//#region 🔖️Payload
/// ✋️ `drag-elements` payload — the element ids it moves and the world offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DragElements {
    pub targets: Vec<String>,
    pub offset: [f64; 3],
}

impl DragElements {
    /// 🟰️ A zero offset moves nothing.
    pub fn identity(&self) -> bool {
        self.offset == [0.0; 3]
    }
}

impl protocol::MutationKind<SemioModelSnapshot, SemioModelMutation> for DragElements {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "elements", kind: "drag-elements", record: "DraggedElements" };

    fn diff(&self, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
        if self.offset.iter().any(|component| !component.is_finite()) {
            return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", self.targets.clone());
        }
        let [dx, dy, dz] = self.offset;
        relative_placement_diff(&self.targets, self.identity(), base, |placement| {
            placement.translation.x += dx;
            placement.translation.y += dy;
            placement.translation.z += dz;
        })
    }
    fn inverse(&self, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
        Ok(relative_placement_inverse(&self.targets, self.identity() || self.offset.iter().any(|component| !component.is_finite()), base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (items_en, items_de) = element_count_label(self.targets.len());
        let (offset_en, offset_de) = vector_label(self.offset);
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {items_en} by {offset_en}"), &format!("{items_de} um {offset_de} ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Payload
