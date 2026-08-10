//! 🔀️ DAG artifact — the document entity this plugin's app edits.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

pub const DAG_DOCUMENT_SCHEMA: &str = "dag.dag";

pub use crate::artifacts::dag::snapshot::schema::{default_snapshot, DagSnapshot};
pub use infinite_board_port_directed_dag::{
    DagEdgePatch, DagFixtureEdge, DagNodeKind, DagNodePatch, DagNodeSpec, DagPreviewContent, IoPortSpec,
};

//#region 🔖️Domain
/// 🎥️ Viewport camera for the DAG canvas (plugin-owned; distinct from framework `dag` kernel helpers).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DagCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for DagCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

impl From<DagCamera> for infinite_board_port_directed_dag::DagCamera {
    fn from(value: DagCamera) -> Self {
        Self { x: value.x, y: value.y, zoom: value.zoom }
    }
}

impl From<infinite_board_port_directed_dag::DagCamera> for DagCamera {
    fn from(value: infinite_board_port_directed_dag::DagCamera) -> Self {
        Self { x: value.x, y: value.y, zoom: value.zoom }
    }
}
//#endregion 🔖️Domain

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "graph.dag".into(),
        name: "DAG".into(),
        source_format: DAG_DOCUMENT_SCHEMA.into(),
        component_kind: "dag".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Dag },
        schema: DAG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_declares_the_graph_dag_component_kind() {
        assert_eq!(artifact_kind().id, "graph.dag");
        assert_eq!(artifact_kind().schema, DAG_DOCUMENT_SCHEMA);
    }

    #[test]
    fn default_snapshot_matches_document_schema() {
        assert_eq!(default_snapshot().schema, DAG_DOCUMENT_SCHEMA);
    }
}
//#endregion 🧪️Tests
