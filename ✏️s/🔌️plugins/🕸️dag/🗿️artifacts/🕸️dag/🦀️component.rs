//! 🔀️ DAG artifact — the document entity this plugin's app edits.
//!
//! Unlike most constitutional apps, `DagDocument`'s fields and the `DAG_DOCUMENT_SCHEMA` constant are
//! NOT owned by this crate — they live in the shared DAG kernel crate ([`infinite_board_port_directed_dag`],
//! `framework/kernel/infinite/board/port/directed/dag/rs`) because the DAG board is shared infrastructure
//! used by more than this play app. This module re-exports the app-facing surface so sibling taxonomy
//! nodes (`⚙️engine`, `🔺️diff`, `🗣️dsl`, `🔧️op`, `🎒️pack`, `📡️spr`) depend on a stable app-owned name
//! instead of every node reaching into the kernel path directly.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️Types
pub use infinite_board_port_directed_dag::{DagDocument, DAG_DOCUMENT_SCHEMA};
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::dag::create_dag_app`'s `🔖️Manifest` region. `source_format`/`schema` deliberately stay
/// the `"flow.dag"` literal (not `DAG_DOCUMENT_SCHEMA`) — verbatim from the pre-migration manifest, not a
/// migration-time change.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "graph.dag".into(),
        name: "DAG".into(),
        source_format: "flow.dag".into(),
        component_kind: "dag".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Dag },
        schema: "flow.dag".into(),
        export_formats: vec![],
        import_formats: vec![],
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
        assert_eq!(artifact_kind().component_kind, "dag");
    }

    #[test]
    fn dag_document_schema_matches_the_kernel_constant() {
        assert_eq!(DAG_DOCUMENT_SCHEMA, infinite_board_port_directed_dag::DAG_DOCUMENT_SCHEMA);
    }
}
//#endregion 🧪️Tests
