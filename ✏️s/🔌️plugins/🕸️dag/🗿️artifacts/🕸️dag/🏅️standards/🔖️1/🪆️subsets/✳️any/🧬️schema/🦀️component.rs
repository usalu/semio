//! 🧬️ DAG artifact schema — every field of the artifact with its state class.

use crate::artifacts::dag::{DagCamera, DagFixtureEdge, DagNodeSpec, DAG_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full DAG artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub nodes: Vec<DagNodeSpec>,
    #[state(persistent)]
    #[serde(default)]
    pub edges: Vec<DagFixtureEdge>,
    #[state(shared_ui)]
    #[serde(default)]
    pub selected_node_ids: Vec<String>,
    #[state(local_ui)]
    pub camera: DagCamera,
    #[state(local_ui)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DagArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::dag::default_snapshot())
    }
}

impl DagArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::dag::DagSnapshot {
        crate::artifacts::dag::DagSnapshot {
            schema: self.schema.clone(),
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::dag::DagSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            nodes: snapshot.nodes,
            edges: snapshot.edges,
            selected_node_ids: Vec::new(),
            camera: DagCamera::default(),
            locale: "en-US".into(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::dag::DagSnapshot) {
        self.schema = snapshot.schema;
        self.nodes = snapshot.nodes;
        self.edges = snapshot.edges;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.dag.dag` — twenty handcrafted schema leaves.
pub fn dag_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.dag.dag",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
