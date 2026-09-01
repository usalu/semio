//! 🖌️️ `change-stroke-color` — sets one scalar field (`DrawStyle.stroke`, name-keyed) on the named
//! style. SMO's binding ruling decomposes stroke into independently-set fields rather than one
//! `update`/`replace-stroke` facet: an editor sets color without touching width. Addressed by
//! `style_name` (the real name-keyed collection this snapshot's stroke lives on), not `node_id` —
//! `DrawStyle` is referenced BY NAME from `DrawNode.style`, it is not a per-node struct.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioRgba;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeStrokeColor {
    pub style_name: String,
    pub new_color: Option<SemioRgba>,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for ChangeStrokeColor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "stroke-color", kind: "change-stroke-color", record: "ChangedStrokeColor" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change stroke color of style \"{}\"", self.style_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.style_name.clone()]
    }
}
//#endregion 🔖️Payload
