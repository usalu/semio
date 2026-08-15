//! 🏗️ `create-solid` — brings a new id-keyed solid into existence with its full initial `shells` membership list (each flagged void/non-void, referencing already-existing shells). A duplicate `id` already present in `base` is a no-op.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::BrepSolidShell;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSolid {
    pub id: String,
    #[serde(default)]
    pub shells: Vec<BrepSolidShell>,
}

impl protocol::MutationKind<SemioBrepSnapshot, SemioBrepMutation> for CreateSolid {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "solid", kind: "create-solid", record: "CreatedSolid" };

    fn diff(&self, base: &SemioBrepSnapshot) -> <SemioBrepMutation as protocol::Mutation<SemioBrepSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create solid \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
