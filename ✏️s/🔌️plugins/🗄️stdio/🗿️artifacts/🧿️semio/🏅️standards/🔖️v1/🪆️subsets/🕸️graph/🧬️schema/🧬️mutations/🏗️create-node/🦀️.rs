//! 🏗️ `create-node` — brings a new id-keyed node into existence with its full initial payload
//! (per `create`'s canonical args). Nodes are id-keyed entities (not an ordered/index-addressed
//! collection), so this is `create`/`delete`, not `insert`/`remove`.

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphPort, SemioGraphSnapshot};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateNode {
    pub id: GraphNodeId,
    #[value(default)]
    pub kind: String,
    #[value(default)]
    pub label: String,
    #[value(default)]
    pub position: SemioPoint2,
    pub width: f64,
    pub height: f64,
    #[value(default)]
    pub ports: Vec<SemioGraphPort>,
    #[value(default)]
    pub properties: Vec<SemioValueEntry>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for CreateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<SemioGraphMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create node \"{}\"", self.id.value), &format!("Knoten \"{}\" erstellen", self.id.value))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.value.clone()]
    }
}
//#endregion 🔖️Payload
