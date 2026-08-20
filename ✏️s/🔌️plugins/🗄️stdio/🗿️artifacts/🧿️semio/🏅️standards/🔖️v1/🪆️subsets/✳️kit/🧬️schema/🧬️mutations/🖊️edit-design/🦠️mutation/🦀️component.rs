//! 🖊️️ `edit-design` — replaces the matching DESIGN's `pieces`/`connections` wholesale (BASE-state
//! addressing by `id`; `name` untouched). A design's arrangement is one authored unit — `edit`
//! replaces an authored content body per `📓️taxonomy.md`, the same shape `✳️text`'s `edit-run`
//! uses one level down for a run's `content`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitConnection, SemioKitPiece, SemioKitSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditDesign {
    pub id: String,
    pub pieces: Vec<SemioKitPiece>,
    pub connections: Vec<SemioKitConnection>,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for EditDesign {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "design", kind: "edit-design", record: "EditedDesign" };

    async fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Edit design {}", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
