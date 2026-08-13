//! 🏗️ `create-node` — brings a new id-keyed node into existence with its full initial payload
//! (per `create`'s canonical args). Nodes are id-keyed entities (not an ordered/index-addressed
//! collection), so this is `create`/`delete`, not `insert`/`remove`.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphPort, SemioGraphSnapshot};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNode {
    pub id: GraphNodeId,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub position: SemioPoint2,
    #[serde(default)]
    pub ports: Vec<SemioGraphPort>,
    #[serde(default)]
    pub properties: Vec<SemioValueEntry>,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for CreateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    fn diff(&self, base: &SemioGraphSnapshot) -> <SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create node \"{}\"", self.id.value)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.value.clone()]
    }
}
//#endregion 🔖️Payload
