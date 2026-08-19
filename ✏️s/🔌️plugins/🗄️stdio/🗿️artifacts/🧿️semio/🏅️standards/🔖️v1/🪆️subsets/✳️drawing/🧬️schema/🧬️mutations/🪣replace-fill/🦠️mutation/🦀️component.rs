//! 🪣️️ `replace-fill` — whole-value swap of the named style's `fill` (SMO-approved `replace-fill`
//! verb). ⚠️ Addressed by `style_name`, not `node_id`: fill lives on the real name-keyed
//! `DrawStyle` collection (referenced BY NAME from `DrawNode.style`), not per-node. `DrawStyle.
//! fill` is `Option<SemioRgba>` (a flat color) in this snapshot's real current shape — NOT the
//! gradient-carrying `FillStyle` enum SMO's ruling verified elsewhere (`✏️s/🔌️plugins/🖍️draw/…`);
//! see `📓️wave4-reports/drawing-triads-report.md`'s `## sharedFileRequests` for the gap this
//! flags. Modeled as `replace` (matching SMO's binding verb name) rather than `change`, since it
//! is a whole-`Option` swap on a nullable field.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioRgba;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceFill {
    pub style_name: String,
    pub new_fill: Option<SemioRgba>,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for ReplaceFill {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "fill", kind: "replace-fill", record: "ReplacedFill" };

    async fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace fill of style \"{}\"", self.style_name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.style_name.clone()]
    }
}
//#endregion 🔖️Payload
