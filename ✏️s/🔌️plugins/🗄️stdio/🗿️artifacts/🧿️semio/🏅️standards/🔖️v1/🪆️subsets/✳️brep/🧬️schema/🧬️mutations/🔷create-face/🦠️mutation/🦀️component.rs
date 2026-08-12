//! 🏗️ `create-face` — brings a new id-keyed face into existence over `surface`, bounded by `outer_loop` (and optional `inner_loops`) — both must already exist in `base`: no `create-loop` verb exists (SMO ruled `Loop`/`Coedge` carry no `PersistentLabel` and are excluded), so loops are established only via `ArtifactStore::reset`/import, never incrementally through this facet. A duplicate `id` already present in `base` is a no-op.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepSurface;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFace {
    pub id: String,
    pub outer_loop: String,
    #[serde(default)]
    pub inner_loops: Vec<String>,
    pub surface: BrepSurface,
    pub orientation: bool,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for CreateFace {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "face", kind: "create-face", record: "CreatedFace" };

    fn diff(&self, base: &SemioBrepSnapshot) -> <SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create face \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
