//! 🗺️ `replace-surface` — whole-value swap of `face_id`'s underlying `BrepSurface`. Same
//! structured-payload reasoning as `replace-curve`: a NURBS surface's control-point grid is edited
//! piecewise by the editor, so `replace`, never `change`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepSurface, SemioBrepSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceSurface {
    pub face_id: String,
    pub new_surface: BrepSurface,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for ReplaceSurface {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "surface", kind: "replace-surface", record: "ReplacedSurface" };

    async fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Replace surface on face \"{}\"", self.face_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.face_id.clone()]
    }
}
//#endregion 🔖️Payload
