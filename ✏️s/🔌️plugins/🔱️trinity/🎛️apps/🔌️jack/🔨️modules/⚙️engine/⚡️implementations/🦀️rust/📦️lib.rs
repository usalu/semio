//! ⚙️ Trinity Jack app — headless compute (constitutional: engine).
//!
//! 📌️ Deviation from the constitutional-split recipe: the query-language compute itself
//! (`run_jack_query` and friends) lives in `trinity_jack` (the shared Jack query-language crate,
//! used by both `jack`'s UI and `trinity_rewrite`'s `apply_rule`) — see the ticket report for why
//! it stays there rather than moving here. This crate holds the one document-level pure helper.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use trinity_ram::{Camera, GraphFixture};

/// 📦️ An empty trinity graph fixture — the app's zero-state initial document.
pub fn empty_jack_document() -> GraphFixture {
    trinity_ram::empty_trinity_graph_fixture()
}

//#region 🔖️Config
/// 🎯️ Ephemeral editor selection range (offsets into the jack query text) — a DSL-bindable twin of
/// the pre-B1 `TrinityJackRuntime::editor_selection`'s inline anonymous struct.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct JackEditorSelection {
    pub start: u64,
    pub end: u64,
}

/// 🧮️ B1: jack's real `DocumentApp::Config` — absorbs every former `TrinityJackRuntime` (app-struct
/// `RefCell`) field: node selection, the live node-graph viewport camera (seeded once from the
/// initial fixture's seed-only `camera` field, then only ever written by `nodeGraphViewport` — see
/// `trinity_ram::GraphFixture::camera`'s own doc comment), the active fixture/example id, the jack
/// query draft + its last result, the three engagement-input drafts, the reorganize epoch, the
/// editor's text selection, the per-window LOD mode, a completion-request revision counter, and the
/// BCP-47 locale tag (mirrors `shooting_engine::ShootingConfig::locale`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "jackcfg")]
#[dsl(layout = "lines")]
pub struct JackConfig {
    pub selected_node_ids: Vec<String>,
    #[dsl(block)]
    pub camera: Camera,
    pub active_fixture_id: String,
    pub jack_query: String,
    pub jack_result_json: String,
    pub editor_engagement_input: String,
    pub graph_engagement_input: String,
    pub results_engagement_input: String,
    pub reorganize_epoch: u64,
    #[dsl(block)]
    pub editor_selection: Option<JackEditorSelection>,
    pub lod_mode_by_window: BTreeMap<String, String>,
    pub revision: u64,
    pub locale: String,
}

impl Default for JackConfig {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
            camera: Camera::default(),
            active_fixture_id: String::new(),
            jack_query: String::new(),
            jack_result_json: String::new(),
            editor_engagement_input: String::new(),
            graph_engagement_input: String::new(),
            results_engagement_input: String::new(),
            reorganize_epoch: 0,
            editor_selection: None,
            lod_mode_by_window: BTreeMap::new(),
            revision: 0,
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(JackConfig);

//#endregion 🔖️Config

//#region 🔖️Io
/// 🔌️ Jack's typed media I/O surface (`AppDefinition.io`) — the implicit document in/out pair (a
/// `trinity.graph` document) plus one extra fan-out output port, `graph:out`, so a jack window can
/// feed its live query-graph projection into other graph-consuming workflow nodes (e.g. `rewrite`'s
/// `graph:in`). Reuses `"graph.trinity"`, the artifact kind `create_trinity_jack_app` already
/// declares via `.artifact_kind(...)`, as both the port's `kind_id` and this app's document schema's
/// media type.
pub fn jack_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: trinity_ram::TRINITY_GRAPH_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Graph, form: semio_framework_plugin::MediaForm::Trinity },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "graph:out".into(),
            label: "Graph".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Graph, form: semio_framework_plugin::MediaForm::Trinity },
            kind_id: Some("graph.trinity".into()),
            required: false,
            multiplicity: semio_framework_plugin::PortMultiplicity::Many,
        }],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "graph.trinity".into(), name: "Trinity Graph".into(), dimension: "graph".into(), component_kind: "trinity".into() },
    }
}
//#endregion 🔖️Io

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_jack_document_has_no_nodes_or_edges() {
        let fixture = empty_jack_document();
        assert!(fixture.nodes.is_empty());
        assert!(fixture.edges.is_empty());
    }

    #[test]
    fn jack_config_default_has_empty_selection_and_default_locale() {
        let config = JackConfig::default();
        assert!(config.selected_node_ids.is_empty());
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.camera, Camera::default());
    }

    #[test]
    fn jack_config_dsl_round_trips() {
        let mut config = JackConfig::default();
        config.selected_node_ids = vec!["n1".into(), "n2".into()];
        config.jack_query = "MATCH (a:Piece) RETURN a".into();
        config.editor_selection = Some(JackEditorSelection { start: 3, end: 9 });
        config.lod_mode_by_window.insert("trinity-jack-graph".into(), "compact".into());
        store::test_support::assert_dsl_round_trip(&config);
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn jack_io_declares_graph_out_fan_out_port() {
        let io = jack_io();
        assert_eq!(io.document_schema, trinity_ram::TRINITY_GRAPH_SCHEMA);
        assert_eq!(io.artifact.id, "graph.trinity");
        let graph_out = io.ports.iter().find(|port| port.id == "graph:out").expect("graph:out declared");
        assert_eq!(graph_out.kind_id.as_deref(), Some("graph.trinity"));
        assert_eq!(graph_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    }
}
//#endregion 🧪️Tests
