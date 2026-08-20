//! ➰ `replace-curve` — whole-value swap of `edge_id`'s underlying `BrepCurve`. SMO's corrected
//! ruling: a NURBS curve has control points the editor edits individually, so this is `replace`,
//! never `change` — the discriminator is whether the editor ever manipulates the value's interior
//! piecewise, and it does here.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, SemioBrepSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceCurve {
    pub edge_id: String,
    pub new_curve: BrepCurve,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for ReplaceCurve {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "curve", kind: "replace-curve", record: "ReplacedCurve" };

    async fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Replace curve on edge \"{}\"", self.edge_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.edge_id.clone()]
    }
}
//#endregion 🔖️Payload
