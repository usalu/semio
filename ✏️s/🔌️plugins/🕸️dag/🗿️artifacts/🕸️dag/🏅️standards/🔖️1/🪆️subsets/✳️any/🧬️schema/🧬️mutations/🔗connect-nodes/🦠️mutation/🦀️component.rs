//! 🔗 DAG mutation — `ConnectNodes`: creates a port-to-port edge relationship between two nodes.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use infinite_board_port_directed_dag::EdgeRouteStyle;
use graph::manifest::PropertyBag;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔗 `connect-nodes` payload — edge `id` plus both endpoint strings (`"<nodeId>@<portId>"`) and
/// the edge's own route/property payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectNodes {
    pub id: String,
    pub source: String,
    pub target: String,
    pub route_style: EdgeRouteStyle,
    pub properties: PropertyBag,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_nodes(id: String, source: String, target: String, route_style: EdgeRouteStyle, properties: PropertyBag) -> DagMutation {
    DagMutation::ConnectNodes(ConnectNodes { id, source, target, route_style, properties })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ConnectNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "nodes", kind: "connect-nodes", record: "ConnectedNodes" };

    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Connect \"{}\" to \"{}\"", self.source, self.target)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
