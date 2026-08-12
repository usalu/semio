//! ⚙️ Puzzle 2d artifact — headless compute over the board scene. Puzzle 2d's engine is a thin
//! domain facade over the `infinite-board` port-directed graph kernel: it re-exports that kernel's
//! whole surface (`BoardEngine`, `BoardHost`, the vector `canvas`, the force/hierarchical layouts)
//! under one name, tags it with the `puzzle.2d` canvas extension, and owns the plugin `setup:` hook.
//!
//! 📚️ Sibling topic files: `🦀️board_host.rs` (the themed host constructors + their scene/selection/
//! hit-test laws), `🦀️linking.rs` (handle-to-handle wiring and compatibility laws), `🦀️brush.rs`
//! (brush slot/fill session laws), `🦀️layout.rs` (the redraw layout dispatcher), `🦀️icons.rs`
//! (the build-script-generated metabolism icon table and the SVG icon codec laws).
//!
//! 🧭️ Placement rule for helpers reaching across nodes: a helper with exactly ONE consumer lives in
//! that consumer's file; two or more consumers put it here. Helpers taking an app-only view-state
//! type (`Puzzle2dConfig`, `Puzzle2dScene`) never come here — artifacts must not depend on apps.

use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

pub use canvas::{CubicBez, Point, Vec2};
pub use graph::canvas;
pub use graph::{
    apply_edge_handle_snap_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_value, apply_normal_undirected_redraw_layout_to_fixture_v1_json,
    apply_redraw_layout_to_fixture_v1_json as apply_ported_redraw_layout_to_fixture_v1_json, apply_undirected_force_graph_layout_to_fixture_v1_json, apply_undirected_force_graph_layout_to_fixture_v1_value, GraphExtension,
};
pub use infinite_board_port_directed_normal::{self as graph, *};
pub use semio_s_mindmap as mindmap;

//#region 🔖️Puzzle2dExtension
/// 🧩️ Puzzle 2d domain extension over the property graph canvas.
#[derive(Clone, Debug, Default)]
pub struct Puzzle2dExtension;

impl canvas::CanvasExtension for Puzzle2dExtension {
    fn extension_id(&self) -> &str {
        "puzzle.2d"
    }
}

impl GraphExtension for Puzzle2dExtension {}
//#endregion 🔖️Puzzle2dExtension

//#region 🔖️DocumentHelpers
pub fn empty_puzzle2d_snapshot() -> Puzzle2dSnapshot {
    Puzzle2dSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🔌️ The plugin `setup:` hook (`semio_plugin!{ setup: ... }`): registers every puzzle app's host
/// exports exactly once at plugin load.
///
/// 🚧️ The 🖐️5d port adds its own `crate::apps::puzzle5d::register_puzzle5d_exports()` line right here.
pub fn register() {
    crate::artifacts::puzzle2d::io_registry::register();

    register_pilot_languages();
    register_artifact_schemas();
    register_artifact_inferences();
    register_app_schemas();
    crate::apps::puzzle2d::register_puzzle2d_exports();
    crate::apps::puzzle3d::register_puzzle3d_exports();
    crate::apps::puzzle5d::register_puzzle5d_exports();
}

/// 📎 Registers all puzzle app schema descriptors (config + presence facets) into the open
/// app-schema registry — mirrors `register_artifact_schemas()` above, one entry per puzzle play app.
pub fn register_app_schemas() {
    crate::apps::puzzle2d::config::schema::register_app_schema();
    crate::apps::puzzle5d::config::schema::register_app_schema();
    crate::apps::puzzle3d::config::schema::register_app_schema();
}

/// 📎 Registers all puzzle artifact schema descriptors into the OS-wide catalog.
pub fn register_artifact_schemas() {
    artifact_schema::register_artifact_schema_descriptor(
        crate::artifacts::puzzle2d::schema::puzzle2d_artifact_schema_descriptor(),
    );
    artifact_schema::register_artifact_schema_descriptor(
        crate::artifacts::puzzle3d::schema::puzzle3d_artifact_schema_descriptor(),
    );
    artifact_schema::register_artifact_schema_descriptor(
        crate::artifacts::puzzle5d::schema::puzzle5d_artifact_schema_descriptor(),
    );
}

/// 💡️ Registers all puzzle artifact 💡️inference schema descriptors into the OS-wide inference
/// catalog — sibling to `register_artifact_schemas()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Puzzle2d has no
/// InferredFields registered yet (fan-out follow-up).
pub fn register_artifact_inferences() {
    artifact_schema::register_artifact_inference_descriptor(
        crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::inferences::puzzle3d_artifact_inference_descriptor(),
    );
    artifact_schema::register_artifact_inference_descriptor(
        crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::inferences::puzzle5d_artifact_inference_descriptor(),
    );
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "puzzle.puzzle2d",
        extension: Some("puzzle2d"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::puzzle2d::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::puzzle2d::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::puzzle2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::puzzle2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("puzzle.puzzle2d"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "puzzle.puzzle2d.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::puzzle2d::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::puzzle2d::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::puzzle2d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::puzzle2d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("puzzle.puzzle2d.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "puzzle.puzzle2d.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::puzzle2d::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::puzzle2d::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("puzzle.puzzle2d.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "2d.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::puzzle2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::puzzle2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("2d.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "2d.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::puzzle2d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::puzzle2d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("2d.spr"),
    });
}

//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle2d::engine::canvas::Point;

    #[test]
    fn computes_handle_positions_and_edge_curves() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 300.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        let curve = engine.edge_curve(100).expect("edge curve should exist");
        let p0 = curve.p0();
        let p1 = curve.p1();
        let p2 = curve.p2();
        let p3 = curve.p3();
        let cap = 8.0;
        assert!((p0.x() - (40.0 + cap)).abs() < 0.001);
        assert!(p0.y().abs() < 0.001);
        assert!((p3.x() - (260.0 - cap)).abs() < 0.001);
        assert!(p3.y().abs() < 0.001);
        let source_radial = p0 - Point::ZERO;
        let arm0 = p1 - p0;
        let align0 = normalize_or_zero(source_radial).dot(normalize_or_zero(arm0));
        let target_approach = Point::new(300.0, 0.0) - p3;
        let arm1 = p3 - p2;
        let align1 = normalize_or_zero(target_approach).dot(normalize_or_zero(arm1));
        assert!(align0 > 0.99);
        assert!(align1 > 0.99);
    }

    #[test]
    fn drags_nodes_without_rebuilding_the_scene_catalog() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 30.0, true);

        engine.pointer_down(0.0, 0.0, false);
        engine.pointer_move(60.0, 25.0);
        engine.pointer_up(60.0, 25.0);

        let node = engine.nodes.get(&1).expect("node should remain in the engine");
        assert_eq!(node.center, Point::new(60.0, 25.0));

        let events = engine.drain_events();
        assert!(events.iter().any(|event| matches!(event, BoardEvent::SelectionChanged { node_ids, .. } if node_ids == &vec![1])));
        assert!(events.iter().any(|event| matches!(event, BoardEvent::NodeMoved { id: 1, x, y } if (*x - 60.0).abs() < 0.001 && (*y - 25.0).abs() < 0.001)));
    }

    #[test]
    fn hit_tests_handles_before_nodes_and_edges() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 200.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        let handle_point = handle_position(engine.nodes.get(&1).unwrap(), engine.handles.get(&10).unwrap());
        engine.pointer_down(handle_point.x, handle_point.y, false);

        let events = engine.drain_events();
        assert!(events.iter().any(|event| matches!(event, BoardEvent::SelectionChanged { handle_ids, .. } if handle_ids == &vec![10])));
    }

    #[test]
    fn renders_snapshot_for_nodes_handles_and_edges() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 10.0, 20.0, 18.0, true);
        engine.create_node(2, 120.0, 20.0, 18.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        let snapshot = engine.render_snapshot();
        assert_eq!(snapshot.nodes.len(), 2);
        assert_eq!(snapshot.handles.len(), 2);
        assert_eq!(snapshot.edges.len(), 1);
    }

    #[test]
    fn engine_extend_pick_keeps_node_when_adding_handle() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 300.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        engine.pointer_down(0.0, 0.0, false);
        let _ = engine.drain_events();
        let hp = handle_position(engine.nodes.get(&1).unwrap(), engine.handles.get(&10).unwrap());
        engine.pointer_down(hp.x, hp.y, true);
        let events = engine.drain_events();
        let last = events.iter().rev().find_map(|event| match event {
            BoardEvent::SelectionChanged { node_ids, handle_ids, edge_ids } => Some((node_ids.clone(), handle_ids.clone(), edge_ids.clone())),
            _ => None,
        });
        let Some((node_ids, handle_ids, edge_ids)) = last else {
            panic!("expected SelectionChanged");
        };
        assert!(node_ids.contains(&1));
        assert!(handle_ids.contains(&10));
        assert!(edge_ids.is_empty());
    }
}
//#endregion 🧪️Tests


//#region 🔖️ArtifactEngine
/// ⚙️ UI-independent puzzle2d artifact engine — owns the full artifact; `snapshot()` is its persisted subset.
pub struct Puzzle2dEngine {
    artifact: crate::artifacts::puzzle2d::schema::Puzzle2dArtifact,
    snapshot: crate::artifacts::puzzle2d::Puzzle2dSnapshot,
}

impl Puzzle2dEngine {
    pub fn new(snapshot: crate::artifacts::puzzle2d::Puzzle2dSnapshot) -> Self {
        let artifact = crate::artifacts::puzzle2d::schema::Puzzle2dArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> crate::artifacts::puzzle2d::Puzzle2dSnapshot {
        self.snapshot
    }
}
//#endregion 🔖️ArtifactEngine
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::Puzzle2dComposer as Puzzle2dAnyComposer;
    use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::Puzzle2dBuilder as Puzzle2dAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const PUZZLE2D_DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle2d", standard: StandardId("1"), subset: SubsetId("*") };
    const PUZZLE2D_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::puzzle2d::Puzzle2dSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == PUZZLE2D_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => Puzzle2dAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => Puzzle2dAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "Puzzle2dComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == PUZZLE2D_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::puzzle2d::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "Puzzle2dComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle2d::io::export::serializers::artifacts::svg::v1_1::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    fn compose_export_pdf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle2d::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle2d::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle2d::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle2d::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    fn compose_export_dxf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle2d::io::export::serializers::artifacts::dxf::v_r12::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DXF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<Puzzle2dAnyComposer>(),
            ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[PUZZLE2D_DIALECT], compose: compose_export_svg },
            ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[PUZZLE2D_DIALECT], compose: compose_export_pdf },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[PUZZLE2D_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[PUZZLE2D_DIALECT], compose: compose_export_json },
            ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[PUZZLE2D_DIALECT], compose: compose_export_dwg },
            ComposerEntry { writes: EXPORT_DXF_DIALECT, reads: &[PUZZLE2D_DIALECT], compose: compose_export_dxf },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
