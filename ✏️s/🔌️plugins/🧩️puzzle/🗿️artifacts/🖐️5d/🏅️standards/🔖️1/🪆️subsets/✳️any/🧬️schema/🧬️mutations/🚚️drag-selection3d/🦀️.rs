//! 🚚️ Puzzle5d mutation — `DragSelection3d`: a relative, parametric drag of a set of parts and target volumes in the world by one
//! offset. The gesture's own inputs (which ids, which offset) are the payload, so editing the drag in history re-derives
//! every origin from whatever base it replays on.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection_items,puzzle5d_selection_triple,Puzzle5dMutation};

use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🚚️ `drag-selection3d` payload — part and target-volume ids (classified by document membership) and the world offset every one of them
/// moves by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "drag-selection3d")]
pub struct DragSelection3d {
    pub targets: Vec<String>,
    pub offset: [f64; 3],
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for DragSelection3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "selection", kind: "drag-selection3d", record: "DraggedSelection3d" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (offset_en, offset_de) = puzzle5d_selection_triple(self.offset);
        let (en, de) = puzzle5d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {en} by {offset_en}"), &format!("{de} um {offset_de} ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_selection_3d(targets: Vec<String>, offset: [f64; 3]) -> Puzzle5dMutation {
    Puzzle5dMutation::DragSelection3d(DragSelection3d { targets, offset })
}
