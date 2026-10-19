//! an area painted, a region grip dragged, a brush stamped, a wire dropped, a wire cut, a delete — reaches the app as
//! 🧱️ Laws of the board tools (`🧫️fixtures/🧫️board-tools`): every board gesture that is not a selection transform —
//! its release and is ONE tool transaction of `<appId>#<kind>` holding exactly its leaves; a release that changes
//! nothing and a flush of transient rows alone commit nothing. Driven through the registered app, one
//! `applyBoardEvents` dispatch per flush; `🐍️.py` beside this file re-derives the corpus independently.

use crate::editor::puzzle2d::snapshot::{PUZZLE2D_INTERACTION_DOMAIN, puzzle2d_interaction_definition};
use crate::editor::puzzle2d::config::Puzzle2dConfig;
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_pack_json::{json, Value};
#[allow(unused_imports)]
use crate::editor::puzzle2d::snapshot::{puzzle2d_scene_for, puzzle2d_scene_with, PUZZLE2D_GENERIC_TOOL_IDS, PUZZLE2D_RETAINED_TOOL_IDS, PUZZLE2D_ADD_NODE_DIALOG_ID, PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, PUZZLE2D_GRANULARITY_EDGE, PUZZLE2D_GRANULARITY_HANDLE, PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, Puzzle2dInteractionSnapshot, Puzzle2dScene, puzzle2d_active_utility, puzzle2d_addresses_locked_entity, puzzle2d_entity_hidden, puzzle2d_entity_label, puzzle2d_node_kind_arg, puzzle2d_select_scope, puzzle2d_selected_node_ids, puzzle2d_selection_is_locked, puzzle2d_window_and_engagements_scope, puzzle2d_window_only_scope};
#[allow(unused_imports)]
use crate::editor::puzzle2d::engine::board_host::drain_board_events_json;
#[allow(unused_imports)]
use crate::editor::puzzle2d::recorder::{puzzle2d_copy_fragment_from, puzzle2d_cut_operations_from, puzzle2d_paste_operations_on};
#[allow(unused_imports)]
use crate::editor::puzzle2d::commands::set_active_example;
#[allow(unused_imports)]
use crate::editor::puzzle2d::config::Puzzle2dConfigMutation;
#[allow(unused_imports)]
use crate::editor::puzzle2d::engine::{BoardHost, Point};
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit;
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit::tools::fill;
#[allow(unused_imports)]
use crate::editor::puzzle2d::precompute::fill as fill_run;
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::{area_brush as area_brush_utility, brush as brush_utility, select as select_utility};
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
#[allow(unused_imports)]
use crate::editor::puzzle2d::panels::{artifact, catalogue, inspection, settings};
#[allow(unused_imports)]
use crate::editor::puzzle2d::presence::{Puzzle2dPresence, Puzzle2dPresenceMutation};
#[allow(unused_imports)]
use crate::editor::puzzle2d::window::{self, Puzzle2dWindowConfig, Puzzle2dWindowTransient};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
#[allow(unused_imports)]
use semio_framework::kernel::UiDirtyScope;
#[allow(unused_imports)]
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PastePlacement};
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgOption;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ActionKind;
#[allow(unused_imports)]
use semio_framework_plugin::ActionRef;
#[allow(unused_imports)]
use semio_framework_plugin::AppIo;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactEditor;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactPresentation;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolFactoryRegistry;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationContract;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationLane;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::DialogDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::DraftView;
#[allow(unused_imports)]
use semio_framework_plugin::Editor;
#[allow(unused_imports)]
use semio_framework_plugin::EditorApp;
#[allow(unused_imports)]
use semio_framework_plugin::Emit;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionRef;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionTarget;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionVerb;
#[allow(unused_imports)]
use semio_framework_plugin::InteractiveJobClassification;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::MediaClass;
#[allow(unused_imports)]
use semio_framework_plugin::MediaForm;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPortDirection;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPortSpec;
#[allow(unused_imports)]
use semio_framework_plugin::MediaType;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::PortMultiplicity;
#[allow(unused_imports)]
use semio_framework_plugin::ToolFactoryKey;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactoryError;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagement;
#[allow(unused_imports)]
use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet};
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest};
#[allow(unused_imports)]
use crate::editor::puzzle2d::kit::*;
#[allow(unused_imports)]
use crate::editor::puzzle2d::retained::*;
#[allow(unused_imports)]
use crate::editor::puzzle2d::session::*;
use crate::editor::puzzle2d::*;
use crate::editor::puzzle2d::testkit::*;
use semio_framework::kernel::HistoryEntry;
use semio_framework_plugin::InvocationResult;
use semio_framework_test_fixtures::fixture_text;

static BOARD_TOOLS_CORPUS: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../🧫️fixtures/🧫️board-tools/🔣️.json"));

/// 🧫️ The corpus, its schema tag checked.
fn corpus() -> Value {
    let corpus: Value = semio_framework_pack_json::from_json_str((*BOARD_TOOLS_CORPUS), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the board-tools corpus parses");
    assert_eq!(corpus["schema"], "s.puzzle2d.board-tools.v1");
    corpus
}

/// 📬️ One flush of `rows` from the overview window.
fn flush(app: &mut Puzzle2dApp, rows: &Value) -> InvocationResult {
    dispatch(app, "applyBoardEvents", Some(&json!({ "eventsJson": rows.to_string() })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("applyBoardEvents")
}

/// 🧾️ The applied history rows one dispatch upserted that carry document ops.
fn edit_rows(result: &InvocationResult) -> Vec<HistoryEntry> {
    result.history_patch.as_ref().map(|patch| patch.upserts.iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).cloned().collect()).unwrap_or_default()
}

/// 🔢️ How many rows `snapshot` holds under `key`.
fn count(snapshot: &Value, key: &str) -> u64 {
    snapshot.get(key).and_then(Value::as_array).map_or(0, |rows| rows.len() as u64)
}

/// 🎯️ `rows` with every `$region` id replaced by the id of the board's first target region.
fn addressed(rows: &Value, snapshot: &Value) -> Value {
    let region = snapshot.get("targetRegions").and_then(Value::as_array).and_then(|regions| regions.first()).and_then(|region| region.get("id")).cloned().unwrap_or(Value::Null);
    let mut rows = rows.clone();
    for row in rows.as_array_mut().into_iter().flatten() {
        if row["payload"]["id"] == "$region" {
            set_at(row, "payload/id", region.clone());
        }
    }
    rows
}

/// 🧱️ LAW (design §22.32 a): every corpus case — one gesture is ONE transaction of its tool, a release that changes
/// nothing commits nothing, and the document after the flush holds what the corpus states.
#[test]
fn every_board_tool_release_is_one_transaction_of_its_tool() {
    let corpus = corpus();
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("case name");
        let mut app = select_tool_history_tests::seeded_app(&corpus["board"]);
        for rows in case["setup"].as_array().into_iter().flatten() {
            let rows = addressed(rows, &fixture_of(&app));
            assert_eq!(committed_edits(&flush(&mut app, &rows)), 1, "{name}: every setup flush is one edit");
        }
        let before = fixture_of(&app);
        let result = flush(&mut app, &addressed(&case["rows"], &before));
        let expect = &case["expect"];
        let rows = edit_rows(&result);
        assert_eq!(rows.len() as u64, expect["edits"].as_u64().expect("edits"), "{name}: history rows");
        assert_eq!(committed_edits(&result) as u64, expect["edits"].as_u64().expect("edits"), "{name}: committed edits");
        let after = fixture_of(&app);
        for key in ["nodes", "edges", "targetRegions"] {
            assert_eq!(count(&after, key), expect[key].as_u64().expect("count"), "{name}: {key}");
        }
        match expect["tool"].as_str() {
            Some(tool) => {
                let transaction = rows[0].transaction.as_ref().unwrap_or_else(|| panic!("{name}: the row is keyed by its tool transaction"));
                assert!(transaction.id.starts_with("tx-"), "{name}: {transaction:?}");
                assert_eq!(transaction.tool, format!("{}#{tool}", corpus["appTool"].as_str().expect("appTool")), "{name}: the tool that authored the row");
            }
            None => {
                assert_eq!(after, before, "{name}: zero trace leaves the document as it was");
                assert!(result.mutations.is_empty(), "{name}: no inline mutation either");
            }
        }
        if let Some(region) = expect.get("region") {
            for key in ["x", "y", "width", "height"] {
                assert_eq!(after["targetRegions"][0][key].as_f64(), region[key].as_f64(), "{name}: region {key}");
            }
        }
        close_app(&mut app);
    }
}

/// 🫥️ LAW (F24): a flush of transient kinds alone — a hover, a selection, a candidate page, nothing — commits nothing,
/// however a host batches it: no applied row, no printed op, no inline mutation. The framework's command log still upserts
/// the verb's own row for the typed operation (un-applied, without ops); a host that lists such rows shows them.
#[test]
fn a_flush_of_transient_rows_commits_nothing() {
    let corpus = corpus();
    let mut app = select_tool_history_tests::seeded_app(&corpus["board"]);
    for case in corpus["cases"].as_array().expect("cases").iter().filter(|case| case["transient"] == true) {
        let result = flush(&mut app, &case["rows"]);
        let upserts: Vec<(u64, bool, usize)> = result.history_patch.as_ref().map(|patch| patch.upserts.iter().map(|entry| (entry.seq, entry.applied, entry.op_lines.len())).collect()).unwrap_or_default();
        assert!(upserts.iter().all(|(_, applied, ops)| !applied && *ops == 0), "{}: a transient flush applies nothing and prints no op: {upserts:?}", case["name"]);
        assert_eq!(committed_edits(&result), 0, "{}: no edit", case["name"]);
        assert!(result.mutations.is_empty(), "{}: and no inline mutation", case["name"]);
    }
    close_app(&mut app);
}

/// 🗺️ LAW (F16): the `vortex` domain declares an app-supplied topology that names every id the board paints, in board
/// order — each node with its handles under it, each edge, each target region — which is what `selectAll` enumerates
/// and what keeps a selection inside the document.
#[test]
fn the_interaction_topology_names_every_board_entity() {
    let mut board = corpus()["board"].clone();
    set_at(&mut board, "targetRegions", json!([{ "id": "region-1", "x": 0.5, "y": 0.5, "width": 10.5, "height": 10.5, "hidden": false, "locked": false }]));
    set_at(&mut board, "edges", json!([{ "id": "edge-left-mid", "source": "left:v0", "target": "mid:v0" }]));
    let snapshot = Puzzle2dPlaySnapshot::new(semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(&(board))).expect("typed fixture admits"));
    let region = snapshot.typed().target_regions.first().map(|region| region.id.to_string_owner()).expect("the pushed region decodes");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = Puzzle2dConfig::default();
    let topology = Puzzle2dPlayApp::interaction_topology(&doc, &ConfigView { snapshot: &config, window: None }).expect("the topology builds");
    let named: Vec<(&str, &str, Option<&str>)> = topology.domains[PUZZLE2D_INTERACTION_DOMAIN].ordered.iter().map(|node| (node.id.as_str(), node.granularity.as_str(), node.parent.as_deref())).collect();
    assert_eq!(named, vec![("left", "node", None), ("left:v0", "handle", Some("left")), ("mid", "node", None), ("mid:v0", "handle", Some("mid")), ("pin", "node", None), ("edge-left-mid", "edge", None), (region.as_str(), "node", None)]);
    assert_eq!(puzzle2d_interaction_definition().hierarchy, HierarchyProvider::Topology, "the domain declares that the app supplies it");
}
