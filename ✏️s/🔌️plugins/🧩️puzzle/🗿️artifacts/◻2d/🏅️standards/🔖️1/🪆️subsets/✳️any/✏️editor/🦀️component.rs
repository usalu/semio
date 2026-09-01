//! 🧩️ Puzzle 2d play app — the plugin's 2d play app: its `ArtifactApp` impl (dispatch-only), the
//! transient `Puzzle2dScene` bundle its command/panel/window nodes mutate and render, the shared
//! fixture helpers they build on, and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️component.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/<window>`. This file dispatches and stitches.
//!
//! 🌉️ `ArtifactApp::Snapshot` is the `Puzzle2dPlaySnapshot` newtype over a bare
//! `serde_json::Value` fixture (see `crate::artifacts::puzzle2d::op`'s `🔖️ValueBridge`), not the typed
//! `Puzzle2dSnapshot`. Ordinary commands derive granular typed deltas; mounted fill continuations
//! bypass whole fixture materialization and publish their already-prepared typed mutations directly.

use crate::artifacts::puzzle2d::op::{puzzle2d_document_delta_operations, Puzzle2dMutation, Puzzle2dPlaySnapshot};
use crate::editor::puzzle2d::commands::{
    add_node, apply_board_events, cancel_slot, commit_slot, cycle_candidate, delete_selection, duplicate_selection, engagement_abort, engagement_control_select, engagement_input, engagement_submit, fill_session_begin, fill_session_clear,
    fill_session_step, focus_selection, force_layout, lod_scale_json, open_slot, patch_inspector, redraw_handles, select_same_kind, set_active_example, set_active_utility, set_brush_kind_weights, set_brush_node_size, set_camera, set_candidate_index,
    set_fill_count, set_grid_factor, set_grid_snap_enabled, set_locale, set_lod_mode_for_pane, set_selection_flag, set_suggestion_offset, set_terminology,
};
use crate::editor::puzzle2d::config::{Puzzle2dConfig, Puzzle2dConfigMutation, Puzzle2dPlayRuntime};
use crate::editor::puzzle2d::engine::board_host::puzzle_board_host;
use crate::editor::puzzle2d::engine::{BoardHost, Puzzle2dExtension};
use crate::editor::puzzle2d::modes::edit;
use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::{brush as brush_utility, select as select_utility};
use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
use crate::editor::puzzle2d::panels::{catalogue, document, inspection};
use crate::editor::puzzle2d::presence::{Puzzle2dPresence, Puzzle2dPresenceMutation};
use crate::editor::puzzle2d::terminology::{is_de_locale, puzzle2d_labels, Puzzle2dLabels};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppIo, AppLabels, ArtifactEditor, ArtifactPresentation, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect, DraftView,
    Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTarget, InteractiveJobClassification, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm,
    MediaPortDirection, MediaPortSpec, MediaType, MergeMode, NoDraft, NoDraftMutation, PortMultiplicity, SelectionMethod, SelectionMode, SelectionSpec, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, UiNode, WindowEngagement, WindowMeasure,
    INTERACTION_SELECT_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID,
};
// 🕹️ `InteractionView` — see puzzle3d's identical import comment (missing top-level re-export from
// `semio_framework_plugin`, flagged to the coordinator, not fixed here).
use semio_framework_plugin::app::InteractionView;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use store::EngineHandles;

//#region 🔖️Constants
pub const PUZZLE2D_PLAY_CONTROLLER_ID: &str = "puzzle2d-play";
pub const PUZZLE2D_PLAY_SURFACE_ID: &str = "puzzle2d.play.composite";
pub const PUZZLE2D_FIXTURE_SCHEMA: &str = "puzzle.2d.fixture";
pub const PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID: &str = crate::examples::puzzle2d::concrete_forest::ID;
pub const PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID: &str = crate::examples::puzzle2d::nakagin_capsule_tower::ID;

/// 🪟️ The three canvas pane KIND ids — a different id space from the window body keys (see
/// `🎮️commands/🎲️apply-board-events`'s `PUZZLE2D_WINDOW_BODY_KEYS`): these key utilities, engagements and measures.
pub const PUZZLE2D_PANES: [&str; 3] = [overview::WINDOW_KIND_ID, detail::WINDOW_KIND_ID, selection::WINDOW_KIND_ID];
pub const PUZZLE2D_LOD_MODE_AUTOMATIC: &str = "automatic";
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the one interaction domain this app
/// declares — the deleted `Puzzle2dConfig::selected_ids` flat bag (nodes and their nested handles
/// alike) collapses into one framework-owned domain, one flat granularity (no real parent/child
/// structure was ever modeled for it).
pub const PUZZLE2D_INTERACTION_DOMAIN: &str = "vortex";
pub const PUZZLE2D_GRANULARITY_NODE: &str = "node";

const BOARD_DEFAULT_WIDTH: u32 = 1024;
const BOARD_DEFAULT_HEIGHT: u32 = 768;

/// 🧵 Reuses the manifest's canonical, initialization-owned example payload so an interactive
/// command never repeats DSL decoding inside its bounded worker step.
pub fn concrete_forest_example_json() -> String {
    crate::examples::puzzle2d::concrete_forest::SOURCE.document_json().to_owned()
}
pub fn nakagin_example_json() -> String {
    crate::examples::puzzle2d::nakagin_capsule_tower::SOURCE.document_json().to_owned()
}
//#endregion 🔖️Constants

//#region 🔖️Scene
/// 🧾️ Transient render/mutation bundle pairing the persisted projection (the bare fixture json) with
/// the app's view config. It is never persisted — the `VcsArtifactApp` store owns the fixture as its
/// projection and `Puzzle2dConfig` owns the runtime — but rebuilding it per call lets the panel,
/// canvas and engagement helpers keep one `&Puzzle2dScene` signature.
pub struct Puzzle2dScene {
    pub fixture: Value,
    pub runtime: Puzzle2dPlayRuntime,
    /// 🧰️ The host-owned active utility for this render/mutation, sourced from
    /// `Puzzle2dConfig::active_utility_by_window_id` (defaulting to `select`) — never a document field.
    pub active_utility: String,
}

pub fn default_empty_fixture() -> Value {
    json!({
        "schema": PUZZLE2D_FIXTURE_SCHEMA,
        "nodes": [],
        "edges": []
    })
}

pub fn puzzle2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PUZZLE2D_PLAY_CONTROLLER_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: builds a framework `interactionSelect`
/// action targeting one `(granularity, id)` pair in the `vortex` domain — replaces the deleted
/// `setSelection` action builders.
pub fn puzzle2d_interaction_select(granularity: &str, id: &str) -> ActionDescriptor {
    let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
    puzzle2d_action(INTERACTION_SELECT_ACTION_ID, Some(json!({ "domainId": PUZZLE2D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })))
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `vortex` domain declaration —
/// the deleted `Puzzle2dConfig::selected_ids` flat bag collapses into one framework-owned domain,
/// `Flat` hierarchy (no parent/child structure was ever modeled for it).
fn puzzle2d_interaction_definition() -> InteractionDefinition {
    InteractionDefinition {
        id: PUZZLE2D_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Vortex", "Vortex"),
        granularities: vec![GranularityDefinition { id: PUZZLE2D_GRANULARITY_NODE.into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle-dot".into() }],
        hierarchy: HierarchyProvider::Flat,
        hover: HoverSpec { enabled: true, transitive: false, channels: vec!["pointer".into()], broadcast: true },
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}

/// 🪟️ B1: was host-pushed `view_state.window_instances` filtered by `window_kind_id`; puzzle2d has
/// three DISTINCT pane kinds (unlike puzzle3d's split-top/perspective, which are several instances of
/// ONE kind), and `Puzzle2dConfig` carries no field that ever differs between two instances of the
/// SAME pane kind, so a self-maintained multi-instance registry would only ever produce
/// byte-identical duplicate entries here. Always exactly one instance, keyed by the pane kind id.
fn window_instance_ids(pane: &str) -> Vec<String> {
    vec![pane.to_string()]
}

/// 🧰️ B1: the host-owned active utility for `window_id`'s pane, now real VCS'd config — see
/// `🎮️commands/🧰️set-active-utility`, the only writer.
pub fn puzzle2d_active_utility(config: &Puzzle2dConfig, window_id: Option<&str>) -> String {
    if let Some(wid) = window_id {
        if let Some(utility) = config.active_utility_by_window_id.get(wid) {
            return utility.clone();
        }
    }
    select_utility::UTILITY_ID.into()
}

/// 🎯️ `semio_framework_plugin::selection_ids`'s "ids" array plus a singular "id" fallback —
/// this app's actions accept either shape depending on the caller.
pub fn selection_ids(args: Option<&Value>) -> Vec<String> {
    let ids = semio_framework_plugin::selection_ids(args);
    if !ids.is_empty() {
        return ids;
    }
    args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(|id| vec![id.to_string()]).unwrap_or_default()
}

/// 🎥️ The camera lives on `Puzzle2dConfig` — session-only view state, never a fixture field.
pub fn runtime_camera(runtime: &Puzzle2dPlayRuntime) -> (f64, f64, f64) {
    (runtime.camera_x, runtime.camera_y, runtime.camera_zoom)
}

pub fn fixture_nodes(fixture: &Value) -> &[Value] {
    fixture.get("nodes").and_then(|value| value.as_array()).map_or(&[][..], |values| values.as_slice())
}

pub fn fixture_edges(fixture: &Value) -> &[Value] {
    fixture.get("edges").and_then(|value| value.as_array()).map_or(&[][..], |values| values.as_slice())
}

pub fn kind_catalog_entries<'a>(fixture: &'a Value, key: &str) -> Option<&'a [Value]> {
    fixture.get("meta").and_then(|value| value.get("kindCatalogs")).and_then(|value| value.get(key)).and_then(|value| value.as_array()).map(|values| values.as_slice())
}

fn manifest_catalog_rows(kinds: &[graph::manifest::KindDef]) -> Value {
    Value::Array(
        kinds
            .iter()
            .map(|kind| {
                let mut row = serde_json::Map::new();
                row.insert("id".to_string(), json!(kind.id));
                row.insert("name".to_string(), json!(kind.name));
                if let Some(Value::Object(presentation)) = kind.presentation.as_ref() {
                    for (key, value) in presentation {
                        row.insert(key.clone(), value.clone());
                    }
                }
                Value::Object(row)
            })
            .collect(),
    )
}

/// 🗂️ Board engine catalogs for a manifest id. Every shipped puzzle2d document names a
/// `meta.manifestId` and carries no catalogs of its own, so this — not `meta.kindCatalogs` — is where
/// their node/handle kinds actually come from. Each row is the manifest row's `id`/`name` merged with
/// its flattened `presentation`. Port kinds without a `presentation.color` are dropped because the
/// engine rejects a colourless handle kind outright, which would discard the whole catalog push.
pub fn manifest_board_kind_catalogs_json(manifest_id: &str) -> Option<String> {
    let manifest = graph::manifest::manifest_by_id(manifest_id)?;
    let visual_port_kinds: Vec<graph::manifest::KindDef> = manifest.port_kinds.iter().filter(|kind| kind.presentation.as_ref().is_some_and(|p| p.get("color").is_some())).cloned().collect();
    Some(
        json!({
            "handleKinds": manifest_catalog_rows(&visual_port_kinds),
            "wireKinds": manifest_catalog_rows(&manifest.wire_kinds),
            "nodeKinds": manifest_catalog_rows(&manifest.node_kinds),
            "edgeKinds": manifest_catalog_rows(&manifest.edge_kinds),
        })
        .to_string(),
    )
}

fn catalog_row_subset(row: &Value, keys: &[&str]) -> Value {
    let mut out = serde_json::Map::new();
    for key in keys {
        match row.get(*key) {
            Some(value) if !value.is_null() => {
                out.insert((*key).to_string(), value.clone());
            }
            _ => {}
        }
    }
    Value::Object(out)
}

fn catalog_rows_subset(catalogs: &Value, slice: &str, keys: &[&str]) -> Option<Value> {
    let rows = catalogs.get(slice).and_then(Value::as_array)?;
    Some(Value::Array(rows.iter().map(|row| catalog_row_subset(row, keys)).collect()))
}

/// 🗂️ Projects the document's `meta.kindCatalogs` onto the board engine's catalog contract. The
/// document owns `nodes`/`handles`/`edges`/`wires` ([`Puzzle2dKindCatalogs`]) while
/// `BoardHost::set_board_kind_catalogs_from_json` reads `nodeKinds`/`handleKinds`/`edgeKinds`/`wireKinds`
/// and rejects any row still carrying the document's `label`, so each row is narrowed to the keys the
/// engine actually reads. Without this the engine's `node_kinds` map stays empty and every
/// brush/fill candidate lookup silently yields nothing. A slice absent from the document is left out
/// entirely rather than emitted empty, because the engine reads an omitted array as "leave that
/// slice alone" and an empty one as "clear it".
///
/// Documents that carry no `meta.kindCatalogs` of their own — which is every shipped example, they
/// name a `meta.manifestId` instead — resolve their catalogs from the compile-time manifest registry
/// via [`manifest_board_kind_catalogs_json`].
pub fn board_kind_catalogs_json(fixture: &Value) -> Option<String> {
    let meta = fixture.get("meta");
    meta.and_then(|meta| meta.get("kindCatalogs"))
        .and_then(document_board_kind_catalogs_json)
        .or_else(|| meta.and_then(|meta| meta.get("manifestId")).and_then(Value::as_str).and_then(manifest_board_kind_catalogs_json))
}

/// 🗂️ The `meta.kindCatalogs` half of [`board_kind_catalogs_json`]. Returns `None` when the document
/// contributes no node kinds, so a document carrying an empty catalog bundle still falls through to
/// its manifest rather than clearing the engine's catalogs.
fn document_board_kind_catalogs_json(catalogs: &Value) -> Option<String> {
    let node_kinds = catalogs.get("nodes").and_then(Value::as_array).map(|rows| {
        Value::Array(
            rows.iter()
                .map(|row| {
                    let mut node = catalog_row_subset(row, &["id", "name", "icon", "color", "shape", "scale"]);
                    let handles: Vec<Value> = row.get("handles").and_then(Value::as_array).map_or_else(Vec::new, |templates| {
                        templates
                            .iter()
                            .filter(|template| template.get("handleKind").and_then(Value::as_str).is_some_and(|kind| !kind.trim().is_empty()))
                            .map(|template| catalog_row_subset(template, &["handleKind", "angle", "radius"]))
                            .collect()
                    });
                    node["handles"] = Value::Array(handles);
                    node
                })
                .collect(),
        )
    });
    let mut out = serde_json::Map::new();
    for (key, rows) in [
        ("handleKinds", catalog_rows_subset(catalogs, "handles", &["id", "name", "color", "defaultWireKind", "scale"])),
        ("wireKinds", catalog_rows_subset(catalogs, "wires", &["id", "name", "defaultEdgeKind"])),
        ("nodeKinds", node_kinds),
        ("edgeKinds", catalog_rows_subset(catalogs, "edges", &["id", "name", "color", "stroke", "pattern", "sourceTip", "targetTip", "directed"])),
    ] {
        if let Some(rows) = rows {
            out.insert(key.to_string(), rows);
        }
    }
    let contributes_node_kinds = out.get("nodeKinds").and_then(Value::as_array).is_some_and(|rows| !rows.is_empty());
    contributes_node_kinds.then(|| Value::Object(out).to_string())
}

/// 🗂️ The kind ids present in the document itself, used whenever the fixture carries no explicit
/// `meta.kindCatalogs` slice.
pub fn inferred_kind_entries(fixture: &Value, field: &str) -> Vec<Value> {
    let mut ids = BTreeSet::new();
    match field {
        "nodes" => {
            for node in fixture_nodes(fixture) {
                if let Some(kind) = node.get("nodeKind").and_then(|value| value.as_str()) {
                    ids.insert(kind.to_string());
                }
            }
        }
        "handles" => {
            for node in fixture_nodes(fixture) {
                if let Some(handles) = node.get("handles").and_then(|value| value.as_array()) {
                    for handle in handles {
                        if let Some(kind) = handle.get("handleKind").and_then(|value| value.as_str()) {
                            ids.insert(kind.to_string());
                        }
                    }
                }
            }
        }
        "edges" => {
            for edge in fixture_edges(fixture) {
                if let Some(kind) = edge.get("edgeKind").and_then(|value| value.as_str()) {
                    ids.insert(kind.to_string());
                }
            }
        }
        _ => {}
    }
    ids.into_iter().map(|id| json!({ "id": id, "name": id })).collect()
}

pub fn puzzle2d_kind_ids(fixture: &Value, field: &str) -> Vec<String> {
    let inferred = inferred_kind_entries(fixture, field);
    let entries = kind_catalog_entries(fixture, field).unwrap_or(inferred.as_slice());
    entries.iter().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

fn new_node_id(prefix: &str) -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    format!("{prefix}-{}", NEXT.fetch_add(1, Ordering::Relaxed))
}

pub fn puzzle_extension_id() -> &'static str {
    let _extension = Puzzle2dExtension;
    "puzzle.2d"
}
//#endregion 🔖️Scene

//#region 🔖️FixtureEdits
pub fn add_node_to_fixture(fixture: &mut Value, kind: Option<&str>, args: Option<&Value>) {
    let Some(obj) = fixture.as_object_mut() else {
        return;
    };
    let nodes = obj.entry("nodes".to_string()).or_insert_with(|| json!([]));
    let Some(nodes) = nodes.as_array_mut() else {
        return;
    };
    let node_kind = kind.unwrap_or("node");
    let id = new_node_id("node");
    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let shape = args.and_then(|value| value.get("shape")).and_then(|value| value.as_str()).unwrap_or("circle");
    let mut node = json!({
        "id": id,
        "nodeKind": node_kind,
        "shape": shape,
        "x": x,
        "y": y,
        "text": id,
        "anchor": "fixed",
        "handles": []
    });
    if shape == "rectangle" {
        node["width"] = json!(args.and_then(|value| value.get("width")).and_then(|value| value.as_f64()).unwrap_or(48.0));
        node["height"] = json!(args.and_then(|value| value.get("height")).and_then(|value| value.as_f64()).unwrap_or(48.0));
    } else {
        node["radius"] = json!(args.and_then(|value| value.get("radius")).and_then(|value| value.as_f64()).unwrap_or(24.0));
    }
    if let Some(icon_kind) = args.and_then(|value| value.get("iconKind")) {
        node["iconKind"] = icon_kind.clone();
    }
    nodes.push(node);
}

pub fn delete_selection_from_fixture(fixture: &mut Value, selected: &[String]) {
    if selected.is_empty() {
        return;
    }
    let selected: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let node_ids: HashSet<String> = fixture_nodes(fixture).iter().filter_map(|node| node.get("id").and_then(|value| value.as_str())).filter(|id| selected.contains(id)).map(str::to_string).collect();
    let handle_ids: HashSet<String> = fixture_nodes(fixture)
        .iter()
        .flat_map(|node| node.get("handles").and_then(|value| value.as_array()).into_iter().flatten().filter_map(|handle| handle.get("id").and_then(|value| value.as_str())))
        .filter(|id| selected.contains(id))
        .map(str::to_string)
        .collect();
    if let Some(nodes) = fixture.get_mut("nodes").and_then(|value| value.as_array_mut()) {
        *nodes = nodes
            .iter()
            .filter(|node| node.get("id").and_then(|value| value.as_str()).is_none_or(|id| !node_ids.contains(id)))
            .map(|node| {
                let mut next = node.clone();
                if let Some(handles) = next.get_mut("handles").and_then(|value| value.as_array_mut()) {
                    handles.retain(|handle| handle.get("id").and_then(|value| value.as_str()).is_none_or(|id| !handle_ids.contains(id)));
                }
                next
            })
            .collect();
    }
    if let Some(edges) = fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
        edges.retain(|edge| {
            let id_ok = edge.get("id").and_then(|value| value.as_str()).is_none_or(|id| !selected.contains(id));
            let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("");
            let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("");
            id_ok && !node_ids.contains(source) && !node_ids.contains(target) && !handle_ids.contains(source) && !handle_ids.contains(target)
        });
    }
}

/// 🙈️ Patches `hidden`/`locked` onto every selected node, handle, and edge in the fixture.
pub fn apply_selection_flag(fixture: &mut Value, selected: &[String], flag: &str, value: bool) {
    if selected.is_empty() {
        return;
    }
    let selected: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let key = if flag == "locked" { "locked" } else { "hidden" };
    if let Some(nodes) = fixture.get_mut("nodes").and_then(|entry| entry.as_array_mut()) {
        for node in nodes.iter_mut() {
            let node_selected = node.get("id").and_then(|entry| entry.as_str()).is_some_and(|id| selected.contains(id));
            if let Some(handles) = node.get_mut("handles").and_then(|entry| entry.as_array_mut()) {
                for handle in handles.iter_mut() {
                    let handle_selected = handle.get("id").and_then(|entry| entry.as_str()).is_some_and(|id| selected.contains(id));
                    if handle_selected {
                        if let Some(obj) = handle.as_object_mut() {
                            obj.insert(key.to_string(), json!(value));
                        }
                    }
                }
            }
            if node_selected {
                if let Some(obj) = node.as_object_mut() {
                    obj.insert(key.to_string(), json!(value));
                }
            }
        }
    }
    if let Some(edges) = fixture.get_mut("edges").and_then(|entry| entry.as_array_mut()) {
        for edge in edges.iter_mut() {
            let edge_selected = edge.get("id").and_then(|entry| entry.as_str()).is_some_and(|id| selected.contains(id));
            if edge_selected {
                if let Some(obj) = edge.as_object_mut() {
                    obj.insert(key.to_string(), json!(value));
                }
            }
        }
    }
}

/// 📋️ Clones every selected node (+24/+24 offset, fresh node+handle ids) and any edge whose both endpoints were cloned; returns the new node ids.
pub fn duplicate_selection_in_fixture(fixture: &mut Value, selected: &[String]) -> Vec<String> {
    if selected.is_empty() {
        return Vec::new();
    }
    let selected_set: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let mut id_remap: HashMap<String, String> = HashMap::new();
    let mut new_ids: Vec<String> = Vec::new();

    let source_nodes: Vec<Value> = fixture_nodes(fixture).iter().filter(|node| node.get("id").and_then(|value| value.as_str()).is_some_and(|id| selected_set.contains(id))).cloned().collect();

    let new_nodes: Vec<Value> = source_nodes
        .into_iter()
        .map(|mut node| {
            let old_id = node.get("id").and_then(|value| value.as_str()).unwrap_or_default().to_string();
            let new_id = new_node_id("node");
            id_remap.insert(old_id, new_id.clone());
            if let Some(obj) = node.as_object_mut() {
                obj.insert("id".into(), json!(new_id));
                if let Some(x) = obj.get("x").and_then(|value| value.as_f64()) {
                    obj.insert("x".into(), json!(x + 24.0));
                }
                if let Some(y) = obj.get("y").and_then(|value| value.as_f64()) {
                    obj.insert("y".into(), json!(y + 24.0));
                }
                if let Some(handles) = obj.get_mut("handles").and_then(|value| value.as_array_mut()) {
                    for handle in handles.iter_mut() {
                        let old_handle_id = handle.get("id").and_then(|value| value.as_str()).unwrap_or_default().to_string();
                        let suffix = old_handle_id.rsplit(':').next().unwrap_or(old_handle_id.as_str());
                        let new_handle_id = format!("{new_id}:{suffix}");
                        id_remap.insert(old_handle_id, new_handle_id.clone());
                        if let Some(hobj) = handle.as_object_mut() {
                            hobj.insert("id".into(), json!(new_handle_id));
                        }
                    }
                }
            }
            new_ids.push(new_id);
            node
        })
        .collect();

    if let Some(nodes) = fixture.get_mut("nodes").and_then(|value| value.as_array_mut()) {
        nodes.extend(new_nodes);
    }

    let new_edges: Vec<Value> = fixture_edges(fixture)
        .iter()
        .filter_map(|edge| {
            let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("");
            let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("");
            let (new_source, new_target) = (id_remap.get(source)?, id_remap.get(target)?);
            let mut clone = edge.clone();
            if let Some(obj) = clone.as_object_mut() {
                obj.insert("id".into(), json!(new_node_id("edge")));
                obj.insert("source".into(), json!(new_source));
                obj.insert("target".into(), json!(new_target));
            }
            Some(clone)
        })
        .collect();
    if !new_edges.is_empty() {
        if let Some(edges) = fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
            edges.extend(new_edges);
        }
    }

    new_ids
}

/// 🎯️ Every node/handle id sharing a `nodeKind`/`handleKind` with anything currently selected.
pub fn select_same_kind_ids(fixture: &Value, selected: &[String]) -> Vec<String> {
    let selected_set: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let mut node_kinds: HashSet<&str> = HashSet::new();
    let mut handle_kinds: HashSet<&str> = HashSet::new();
    for node in fixture_nodes(fixture) {
        if node.get("id").and_then(|value| value.as_str()).is_some_and(|id| selected_set.contains(id)) {
            if let Some(kind) = node.get("nodeKind").and_then(|value| value.as_str()) {
                node_kinds.insert(kind);
            }
        }
        for handle in node.get("handles").and_then(|value| value.as_array()).into_iter().flatten() {
            if handle.get("id").and_then(|value| value.as_str()).is_some_and(|id| selected_set.contains(id)) {
                if let Some(kind) = handle.get("handleKind").and_then(|value| value.as_str()) {
                    handle_kinds.insert(kind);
                }
            }
        }
    }
    let mut ids: Vec<String> = Vec::new();
    for node in fixture_nodes(fixture) {
        if node.get("nodeKind").and_then(|value| value.as_str()).is_some_and(|kind| node_kinds.contains(kind)) {
            if let Some(id) = node.get("id").and_then(|value| value.as_str()) {
                ids.push(id.to_string());
            }
        }
        for handle in node.get("handles").and_then(|value| value.as_array()).into_iter().flatten() {
            if handle.get("handleKind").and_then(|value| value.as_str()).is_some_and(|kind| handle_kinds.contains(kind)) {
                if let Some(id) = handle.get("id").and_then(|value| value.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }
    }
    ids
}

/// 🎥️ Writes an `{ x, y, zoom }` camera payload into the config — session-only view state, never the fixture.
pub fn set_runtime_camera(runtime: &mut Puzzle2dPlayRuntime, camera: &Value) {
    if let Some(x) = camera.get("x").and_then(Value::as_f64) {
        runtime.camera_x = x;
    }
    if let Some(y) = camera.get("y").and_then(Value::as_f64) {
        runtime.camera_y = y;
    }
    if let Some(zoom) = camera.get("zoom").and_then(Value::as_f64) {
        runtime.camera_zoom = zoom;
    }
}

/** @emoji 📐️ Patches `field` on every selected node: an absolute `value` sets it directly on all
 * of them, otherwise a numeric `delta` is added to each node's own current `field` value —
 * offset-preserving across a multi-select where nodes start at different positions. */
pub fn patch_inspector_nodes(fixture: &mut Value, ids: &[String], field: &str, value: Option<&Value>, delta: Option<&Value>) {
    if let Some(nodes) = fixture.get_mut("nodes").and_then(|entry| entry.as_array_mut()) {
        for node in nodes {
            let Some(id) = node.get("id").and_then(|entry| entry.as_str()).map(str::to_string) else {
                continue;
            };
            if !ids.is_empty() && !ids.contains(&id) {
                continue;
            }
            let resolved = if let Some(absolute) = value {
                Some(absolute.clone())
            } else if let Some(delta) = delta.and_then(Value::as_f64) {
                let current = node.get(field).and_then(Value::as_f64).unwrap_or(0.0);
                Some(json!(current + delta))
            } else {
                None
            };
            if let (Some(obj), Some(resolved)) = (node.as_object_mut(), resolved) {
                obj.insert(field.to_string(), resolved);
            }
        }
    }
}

/// 🎲️ Re-mints a node id when it collides with an existing one — client-side brush serials restart every session.
fn unique_node_id(fixture: &Value, candidate: String) -> String {
    if fixture_nodes(fixture).iter().any(|node| node.get("id").and_then(|value| value.as_str()) == Some(candidate.as_str())) {
        new_node_id("node")
    } else {
        candidate
    }
}

fn unique_edge_id(fixture: &Value, candidate: String) -> String {
    if fixture_edges(fixture).iter().any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(candidate.as_str())) {
        new_node_id("edge")
    } else {
        candidate
    }
}

/// 🖌️ Splices one brush placement (a node, plus the edge back to its source handle) into the fixture.
pub fn apply_brush_place_payload(fixture: &mut Value, payload: &Value) {
    let node_id = unique_node_id(fixture, payload.get("nodeId").and_then(|value| value.as_str()).map_or_else(|| new_node_id("node"), str::to_string));
    let edge_id = unique_edge_id(fixture, payload.get("edgeId").and_then(|value| value.as_str()).map_or_else(|| new_node_id("edge"), str::to_string));
    let node_kind = payload.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("node");
    let x = payload.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0);
    let y = payload.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0);
    let shape = payload.get("shape").and_then(|value| value.as_str()).unwrap_or("circle");
    let mut node = json!({
        "id": node_id,
        "nodeKind": node_kind,
        "shape": shape,
        "x": x,
        "y": y,
        "text": node_kind,
        "handles": payload.get("handles").cloned().unwrap_or_else(|| json!([])),
    });
    if shape == "rectangle" {
        node["width"] = json!(payload.get("width").and_then(|value| value.as_f64()).unwrap_or(48.0));
        node["height"] = json!(payload.get("height").and_then(|value| value.as_f64()).unwrap_or(48.0));
    } else {
        node["radius"] = json!(payload.get("radius").and_then(|value| value.as_f64()).unwrap_or(24.0));
    }
    if let Some(icon) = payload.get("iconKind") {
        node["iconKind"] = icon.clone();
    }
    if let Some(nodes) = fixture.get_mut("nodes").and_then(|value| value.as_array_mut()) {
        nodes.push(node);
    }
    let source = payload.get("sourceHandleId").and_then(|value| value.as_str()).unwrap_or("");
    if !source.is_empty() {
        if let Some(edges) = fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
            edges.push(json!({
                "id": edge_id,
                "edgeKind": "link",
                "source": source,
                "target": format!("{node_id}:v{}", payload.get("targetHandleIndex").and_then(|value| value.as_u64()).unwrap_or(0)),
            }));
        }
    }
}
//#endregion 🔖️FixtureEdits

//#region 🔖️BoardHostSync
/// 🧱️ The expensive half of syncing `host` from `envelope`: a full `clear_scene()` + rebuild of
/// every node/handle/edge plus the kind-catalog/kind-compat re-push. Only needed when the fixture
/// content actually changed — gated by `last_synced_fixture` in `handle`.
fn sync_host_fixture_content(host: &mut BoardHost, envelope: &Puzzle2dScene) {
    let _ = host.parse_fixture_v1(&envelope.fixture);
    if let Some(json) = board_kind_catalogs_json(&envelope.fixture) {
        let _ = host.set_board_kind_catalogs_from_json(&json);
    }
    if let Some(compat) = envelope.fixture.get("meta").and_then(|value| value.get("kindCompatibility")).or_else(|| envelope.fixture.get("kindCompatibility")) {
        if let Ok(json) = serde_json::to_string(compat) {
            let _ = host.set_handle_link_compat_from_json(&json);
        }
    }
}

/// 🪶️ The cheap half of syncing `host` from `envelope`: plain setters mirroring ephemeral view state
/// (selection/utility/grid/LOD/…) — must run on every action regardless of whether the fixture
/// content changed, since this state itself changes every action.
fn sync_host_runtime_state(host: &mut BoardHost, envelope: &Puzzle2dScene, selected_ids: &[String]) {
    host.set_size(BOARD_DEFAULT_WIDTH, BOARD_DEFAULT_HEIGHT, 1.0);
    host.set_selection_ids(selected_ids);
    host.set_active_utility(&envelope.active_utility);
    let overview_lod_mode = envelope.runtime.lod_mode_by_pane.get(overview::WINDOW_KIND_ID).map_or(PUZZLE2D_LOD_MODE_AUTOMATIC, String::as_str);
    if overview_lod_mode == PUZZLE2D_LOD_MODE_AUTOMATIC {
        host.set_automatic_lod(true);
    } else {
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label(overview_lod_mode);
    }
    host.set_grid_snap_enabled(envelope.runtime.grid_snap_enabled);
    let _ = host.set_grid_factor(envelope.runtime.grid_factor);
    host.set_suggestion_offset(envelope.runtime.suggestion_offset);
    if let Ok(weights_json) = serde_json::to_string(&json!({
        "nodeWeights": envelope.runtime.node_kind_weights,
        "handleWeights": envelope.runtime.handle_kind_weights,
    })) {
        host.set_brush_kind_weights(&weights_json);
    }
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the marquee method is
    // framework-owned now (`interactionSelect`'s `method` arg) — the board engine still needs SOME
    // default to hit-test with, so it keeps "rectangle" rather than reading a deleted config field.
    host.set_selection_options("rectangle", "replace", true, true, true);
}

fn sync_host_from_envelope(host: &mut BoardHost, envelope: &Puzzle2dScene) {
    sync_host_fixture_content(host, envelope);
    sync_host_runtime_state(host, envelope, &[]);
}

/// 🪞️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: used to re-sync
/// `envelope.runtime.selected_ids` from `host.selection` for engine-driven selection changes (e.g.
/// `delete_selection`, brush commit) — selection is framework-owned now and `handle` has no channel
/// to write it back (see puzzle3d's `select-same-kind` doc comment for the identical limitation), so
/// this no longer reconciles anything selection-shaped. Camera is deliberately NOT mirrored here:
/// every action that moves the camera already writes the config's camera fields directly — re-deriving
/// it from `host.camera` here used to blindly overwrite that write with the *pre-action* host camera.
pub fn apply_host_events(host: &mut BoardHost, envelope: &mut Puzzle2dScene) {
    let events_raw = drain_board_events_json(host);
    apply_board_events::apply_board_events_from_json(&events_raw, envelope);
}

/// 📨️ Retires the board host's bounded owned-event queue into its public JSON envelope.
pub fn drain_board_events_json(host: &mut BoardHost) -> String {
    let mut output = String::from("[");
    let mut first = true;
    while let Some(event) = host.pop_owned_event() {
        if !first {
            output.push(',');
        }
        first = false;
        event.write_json(&mut output);
    }
    output.push(']');
    output
}
//#endregion 🔖️BoardHostSync

//#region 🔖️UiScopes
/// 🐢️ Narrow `UiDirtyScope` shared by pure view/selection/camera actions that only touch the 3
/// canvas panes (never a panel or engagement/measure/utility refresh).
pub fn puzzle2d_window_only_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: apply_board_events::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    }
}

/// 🐢️ Narrow `UiDirtyScope` for actions that additionally change the engagement bar (active utility,
/// brush weights, LOD/grid settings, engagement text input) but never touch document content.
pub fn puzzle2d_window_and_engagements_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: apply_board_events::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: true,
        measures: false,
        labels: false,
    }
}

/// 🐢️ Narrow `UiDirtyScope` for settings surfaced in the measures sidebar (LOD mode, grid, brush
/// weights, suggestion offset) but that never touch document content or the engagement bar.
pub fn puzzle2d_window_and_measures_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: apply_board_events::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: true,
        labels: false,
    }
}

/// 🐢️ Narrow `UiDirtyScope` for a runtime-only selection change: the 3 canvas panes plus the
/// layers/properties panels (which highlight the selection) and the engagement bar.
pub fn puzzle2d_select_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: apply_board_events::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: vec![document::PUZZLE2D_PLAY_BODY_LAYERS.to_string(), inspection::PUZZLE2D_PLAY_BODY_PROPERTIES.to_string()],
        utilities: false,
        tools: false,
        engagements: true,
        measures: false,
        labels: false,
    }
}
//#endregion 🔖️UiScopes

//#region 🔖️Puzzle2dCommand
/// @emoji 🎯️ B1: `Puzzle2dPlayApp::Command` — the SOLE dispatch surface, one variant per declared
/// action (mirrors every `.mutation(...)`/`.view_action(...)`/`.action_with(...)` id
/// `create_puzzle2d_app` registers below, plus the framework-injected `setActiveUtility` and the
/// `setLocale`/`setTerminology` B1 additions). Each variant carries `window_id` plus `args` (the
/// action's original `{...}` JSON payload, unchanged) — `handle` reconstructs the exact
/// `(action, args, window_id)` triple every `🎮️commands/*` arm expects, so each arm's internal
/// `args.get("field")` extraction stays byte-for-byte identical to the pre-B1 implementation.
///
/// ⚠️ `OpBinary` is a plain JSON-bytes bridge (NOT `#[derive(dsl::DslOps)]`, and NOT the framework's
/// `app_commands!` macro): a generic `args: Value` field is not representable in the DSL grammar
/// those target, so adopting them would silently rewrite this app's wire format. Keep this macro's
/// variant list byte-for-byte stable.
macro_rules! puzzle2d_command_variants {
    ($($Variant:ident = $id:tt),* $(,)?) => {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Puzzle2dCommand {
            $($Variant { window_id: Option<String>, args: Option<Value> }),*
        }

        impl Puzzle2dCommand {
            /// 🏷️ The action id this variant was declared under — used both for `command_id()`
            /// (command-log labeling / registry kind-discipline) and to reconstruct the exact
            /// `action: &str` `handle` dispatches on.
            fn action_id(&self) -> &'static str {
                match self {
                    $(Puzzle2dCommand::$Variant { .. } => $id),*
                }
            }

            fn window_id(&self) -> Option<&str> {
                match self {
                    $(Puzzle2dCommand::$Variant { window_id, .. } => window_id.as_deref()),*
                }
            }

            fn args(&self) -> Option<&Value> {
                match self {
                    $(Puzzle2dCommand::$Variant { args, .. } => args.as_ref()),*
                }
            }

            fn try_from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Option<Self> {
                match action {
                    $($id => Some(Puzzle2dCommand::$Variant { window_id, args })),*,
                    _ => None,
                }
            }

            #[cfg(test)]
            fn from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Self {
                Self::try_from_action(action, args, window_id)
                    .unwrap_or_else(|| panic!("unknown puzzle2d action id in test: {action}"))
            }
        }
    };
}

puzzle2d_command_variants! {
    AddNode = "addNode",
    SetActiveExample = "setActiveExample",
    SetActiveExampleStep = "setActiveExampleStep",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    ForceLayout = "forceLayout",
    FocusSelection = "focusSelection",
    SelectSameKind = "selectSameKind",
    SetSelectionFlag = "setSelectionFlag",
    PatchInspectorNodes = "patchInspectorNodes",
    RedrawHandles = "redrawHandles",
    Reorganize = "reorganize",
    ApplyBoardEvents = "applyBoardEvents",
    SetFillCount = "setFillCount",
    BrushFillSessionStep = "brushFillSessionStep",
    BrushFillSessionAdopt = "brushFillSessionAdopt",
    BrushFillSessionCancel = "brushFillSessionCancel",
    BrushFillSessionRetry = "brushFillSessionRetry",
    BrushFillSessionDiscard = "brushFillSessionDiscard",
    BrushCommitSlot = "brushCommitSlot",
    SetCamera = "setCamera",
    EngagementInput = "engagementInput",
    EngagementSubmit = "engagementSubmit",
    EngagementAbort = "engagementAbort",
    EngagementControlSelect = "engagementControlSelect",
    SetLodModeForPane = "setLodModeForPane",
    SetGridSnapEnabled = "setGridSnapEnabled",
    SetGridFactor = "setGridFactor",
    SetBrushKindWeights = "setBrushKindWeights",
    SetBrushNodeSize = "setBrushNodeSize",
    SetSuggestionOffset = "setSuggestionOffset",
    BrushCycleCandidate = "brushCycleCandidate",
    BrushSetCandidateIndex = "brushSetCandidateIndex",
    BrushOpenSlot = "brushOpenSlot",
    BrushCancelSlot = "brushCancelSlot",
    BrushFillSessionBegin = "brushFillSessionBegin",
    BrushFillSessionClear = "brushFillSessionClear",
    LodScaleJson = "lodScaleJson",
    SetActiveUtility = SET_ACTIVE_UTILITY_ACTION_ID,
    // 🗣️ B1: locale/terminology used to be host-pushed `ViewModel` fields with no app-level action of
    // their own; now that `ViewModel` is gone from the app-facing surface, they need a real Command.
    SetLocale = "setLocale",
    SetTerminology = "setTerminology",
}

impl protocol::OpBinary for Puzzle2dCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}
//#endregion 🔖️Puzzle2dCommand

//#region 🔖️ActionContext
/// 🎬️ Everything one `🎮️commands/*` arm may read or write. The prologue/epilogue around the dispatch
/// match (host sync, host-event replay, delta computation, config snapshotting) stays in
/// [`ArtifactApp::handle`]; an arm only mutates this bundle.
pub struct Puzzle2dActionCtx<'a> {
    /// 🎲️ The app's long-lived board engine — every arm reaching it goes through `borrow_mut()`.
    pub host: &'a RefCell<BoardHost>,
    pub scene: &'a mut Puzzle2dScene,
    /// 🪟️ The window instance this action was dispatched from, when the caller named one.
    pub window_id: Option<&'a str>,
    /// 🧰️ The active utility resolved for `window_id` BEFORE this action ran.
    pub active_utility: String,
    /// 🕹️ Read-only view of the framework-owned `vortex` interaction domain (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — retained selection-acting verbs read
    /// `.selected_ids()` here instead of the deleted `Puzzle2dConfig::selected_ids` field.
    pub selection: &'a protocol::DomainSelection,
    pub effects: &'a mut Vec<Effect>,
    pub artifact_mutations: &'a mut Vec<Puzzle2dMutation>,
    pub ui_scope: &'a mut UiDirtyScope,
    /// 🪪️ Exact public command authority retained by framework continuations.
    pub operation: Option<semio_framework_plugin::AppOperationContext>,
}

impl<'a> Puzzle2dActionCtx<'a> {
    pub fn selected_ids(&self) -> Vec<String> {
        self.selection.ids.clone()
    }
}

/// 🏷️ Admits dynamic puzzle labels into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref().to_string()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d label admission failed"))
}

/// 🌳️ Admits fallibly assembled puzzle nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        nodes.try_push(value?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d node admission failed"))?;
    }
    Ok(nodes)
}
//#endregion 🔖️ActionContext

//#region 🔖️ContextMenu
/// 🖱️ On-demand puzzle 2d board context menu from selection snapshot. Grouped disclosure:
/// toggleHidden/toggleLocked/duplicate/focusSelection stay top-level (the four most frequent
/// verbs); selectSameKind folds into the "selection" taxonomy group; deleteSelection stays the
/// destructive tail. `organize_context_menu` (applied automatically at the
/// `VcsArtifactApp::context_menu` funnel) sorts groups into `RIBBON_PARENT_CATEGORIES` order and
/// inserts the pre-destructive separator itself, so no manual `.separator()` calls are needed here.
async fn puzzle2d_context_menu_items(registry: &semio_framework_plugin::AppActionRegistry, fixture: &Value, selected: &[String], is_de: bool) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, ContextMenuItemSpec, Menu};
    // 🧩️ Bespoke-row helper (dynamic label/icon/args/disabled per selection state — not a plain
    // declared-action lookup) — appended via `Menu::item(...)`, the documented escape hatch.
    let item = |id: &str, label: &str, icon: &str, action: &str, args: Option<Value>, destructive: bool, disabled: bool| ContextMenuItemSpec {
        id: id.into(),
        label: Some(label.into()),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args: semio_framework_plugin::optional_json_to_dsl(args),
        destructive: destructive.then_some(true),
        disabled: disabled.then_some(true),
        ..Default::default()
    };
    if selected.is_empty() {
        return Menu::of(registry).await.item(item("selectAll", if is_de { "Alles auswählen" } else { "Select All" }, "select-all", "selectAll", None, false, false)).await.build().await;
    }
    let selected_set: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let mut entities: Vec<&Value> = Vec::new();
    let mut has_selected_node = false;
    if let Some(nodes) = fixture.get("nodes").and_then(|v| v.as_array()) {
        for node in nodes {
            if node.get("id").and_then(|v| v.as_str()).is_some_and(|id| selected_set.contains(id)) {
                entities.push(node);
                has_selected_node = true;
            }
            if let Some(handles) = node.get("handles").and_then(|v| v.as_array()) {
                for handle in handles {
                    if handle.get("id").and_then(|v| v.as_str()).is_some_and(|id| selected_set.contains(id)) {
                        entities.push(handle);
                    }
                }
            }
        }
    }
    if let Some(edges) = fixture.get("edges").and_then(|v| v.as_array()) {
        for edge in edges {
            if edge.get("id").and_then(|v| v.as_str()).is_some_and(|id| selected_set.contains(id)) {
                entities.push(edge);
            }
        }
    }
    let any_visible = entities.iter().any(|entity| entity.get("hidden").and_then(|v| v.as_bool()) != Some(true));
    let any_unlocked = entities.iter().any(|entity| entity.get("locked").and_then(|v| v.as_bool()) != Some(true));
    let phrase = selection_count_phrase(is_de, &[(selected.len(), if is_de { "Element" } else { "item" }, if is_de { "Elemente" } else { "items" })]).await;
    let hide_label = match (any_visible, is_de) {
        (true, true) => "Ausblenden",
        (true, false) => "Hide",
        (false, true) => "Einblenden",
        (false, false) => "Show",
    };
    let lock_label = match (any_unlocked, is_de) {
        (true, true) => "Sperren",
        (true, false) => "Lock",
        (false, true) => "Entsperren",
        (false, false) => "Unlock",
    };
    Menu::of(registry)
        .await
        .item(item("toggleHidden", hide_label, if any_visible { "eye-off" } else { "eye" }, "setSelectionFlag", Some(json!({ "flag": "hidden", "value": any_visible })), false, false))
        .await
        .item(item("toggleLocked", lock_label, if any_unlocked { "lock" } else { "lock-open" }, "setSelectionFlag", Some(json!({ "flag": "locked", "value": any_unlocked })), false, false))
        .await
        .item(item("duplicate", if is_de { "Duplizieren" } else { "Duplicate" }, "copy", "duplicateSelection", None, false, !has_selected_node))
        .await
        .item(item("focusSelection", if is_de { "Auf Auswahl zoomen" } else { "Zoom to selection" }, "crosshair", "focusSelection", None, false, false))
        .await
        .group("selection", |m| async move { m.item(item("selectSameKind", if is_de { "Gleiche Art auswählen" } else { "Select same kind" }, "layers", "selectSameKind", None, false, false)).await })
        .await
        .item(item("deleteSelection", &format!("{} ({phrase})", if is_de { "Löschen" } else { "Delete" }), "trash", "deleteSelection", None, true, false))
        .await
        .build()
        .await
}
//#endregion 🔖️ContextMenu

//#region 🔖️PlayApp
/// 🧩️ Puzzle-2d play app. Owns the `BoardHost` engine; the persisted document (the bare fixture json)
/// lives in the wrapping `VcsArtifactApp`'s operation store and the view state in `Puzzle2dConfig`.
#[derive(Default, Clone, Copy)]
pub struct Puzzle2dPlayApp;

impl Puzzle2dPlayApp {
    fn scene_for(fixture: Value, config: &Puzzle2dConfig, window_id: Option<&str>) -> Puzzle2dScene {
        let active_utility = puzzle2d_active_utility(config, window_id);
        Puzzle2dScene { fixture, runtime: config.clone(), active_utility }
    }
}

fn is_fill_session_action(action: &str) -> bool {
    matches!(action, "setFillCount" | "brushFillSessionBegin" | "brushFillSessionStep" | "brushFillSessionClear" | "brushFillSessionAdopt" | "brushFillSessionCancel" | "brushFillSessionRetry" | "brushFillSessionDiscard")
}

fn dispatch_fill_session_action(action: &str, args: Option<&Value>, ctx: &mut set_fill_count::Puzzle2dFillActionCtx<'_>) {
    match action {
        "setFillCount" => set_fill_count::set_fill_count(ctx, args),
        "brushFillSessionBegin" => fill_session_begin::fill_session_begin(ctx, args),
        "brushFillSessionStep" => fill_session_step::fill_session_step(ctx, args),
        "brushFillSessionClear" => fill_session_clear::fill_session_clear(ctx),
        "brushFillSessionAdopt" => set_fill_count::adopt_fill_job(ctx, args.and_then(|value| value.get("generation")).and_then(Value::as_u64)),
        "brushFillSessionCancel" => set_fill_count::cancel_fill_job(ctx, args.and_then(|value| value.get("generation")).and_then(Value::as_u64)),
        "brushFillSessionRetry" => set_fill_count::retry_fill_job(ctx),
        "brushFillSessionDiscard" => set_fill_count::discard_fill_job(ctx, args.and_then(|value| value.get("generation")).and_then(Value::as_u64)),
        _ => {}
    }
}

//#region 🧵️RetainedCommands
/// 🧵️ The 2D editor actions that carry an exact app-owned tool proof. Only these may declare
/// [`InteractiveJobClassification::Migrated`] — UI dispatch resolves a controller/owner/factory/tool
/// /schema proof for every migrated verb, and one without a registered factory is refused outright.
pub(crate) const PUZZLE2D_RETAINED_TOOL_IDS: &[&str] = &["setActiveExample", "forceLayout", "addNode", "applyBoardEvents"];
const PUZZLE2D_RETAINED_PAYLOAD_SCHEMA: &str = "puzzle.2d.fixture.tool-command.v1";

struct Puzzle2dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Puzzle2dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: PUZZLE2D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Puzzle2dRetainedCommandJobFactory {
    type Payload = crate::retained_command::RetainedPuzzleCommandPayload<EditorApp<Puzzle2dPlayApp>>;
    type Job = crate::retained_command::RetainedPuzzleCommandJob<EditorApp<Puzzle2dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PUZZLE2D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        crate::retained_command::puzzle_command_contract()
    }

    fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(crate::retained_command::RetainedPuzzleCommandJob::new(operation, payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > crate::retained_command::PUZZLE_COMMAND_RAW_BYTES {
            return Err((ToolJobFactoryError::new("Puzzle 2d retained command rejects an oversized wire owner"), input, checkpoint));
        }
        match checkpoint {
            Some(checkpoint) => {
                if let Err(error) = crate::retained_command::RetainedPuzzleCommandJob::validate_wire_checkpoint(operation, &payload, &input, &checkpoint) {
                    return Err((error, input, Some(checkpoint)));
                }
                Ok(crate::retained_command::RetainedPuzzleCommandJob::from_validated_wire_checkpoint(operation, payload, input, checkpoint))
            }
            None => Ok(crate::retained_command::RetainedPuzzleCommandJob::from_wire(operation, payload, input)),
        }
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Puzzle2dRetainedCommandJobFactory {
    type Owner = EditorApp<Puzzle2dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PUZZLE2D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE2D_FIXTURE_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "forceLayout", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "applyBoardEvents", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ];
}

/// 🖌️ Upper bound on the events one board flush may carry into a single retained step. The browser
/// flushes a handful of events per interaction (`PUZZLE2D_FLUSH_NOW_EVENT_NAMES` in `Board2dHost`), so
/// one bounded step covers a real interaction; an oversized batch is refused rather than silently
/// truncated.
const PUZZLE2D_BOARD_EVENT_BATCH_LIMIT: usize = 256;

fn puzzle2d_board_events_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if command.action_id() != "applyBoardEvents" {
        return None;
    }
    let events = command.args().and_then(|args| args.get("eventsJson")).and_then(Value::as_str).unwrap_or("[]");
    let parsed: Value = serde_json::from_str(events).ok()?;
    let count = parsed.as_array().map_or(0, Vec::len);
    (count <= PUZZLE2D_BOARD_EVENT_BATCH_LIMIT).then_some(count.max(1))
}

/// 🖌️ `applyBoardEvents` is the single verb the browser's board session commits through — `brushPlace`,
/// `select`, `edgeCreate`/`edgeDelete`, `nodeDelete` and `camera` all arrive in its `eventsJson` batch
/// (`Board2dHost/🟦️component.tsx`). It reruns the same pipeline `handle` does — scene, board host,
/// runtime sync, the action, host-event drain, document delta — minus the `ArtifactView` that a
/// retained work never sees, so the committed `AppOperationContext` is simply absent here.
fn puzzle2d_board_events_reduce(
    command: &Puzzle2dCommand,
    snapshot: &Puzzle2dPlaySnapshot,
    config: &Puzzle2dConfig,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, Fault> {
    if command.action_id() != "applyBoardEvents" {
        return Err(Fault::from("puzzle2d-board-events-command-mismatch"));
    }
    let before = snapshot.0.clone();
    let window_id = command.window_id();
    let active_utility = puzzle2d_active_utility(config, window_id);
    let mut scene = Puzzle2dPlayApp::scene_for(before.clone(), config, window_id);
    let selection = interaction.selection(PUZZLE2D_INTERACTION_DOMAIN);
    let host = RefCell::new(BoardHost::default());
    sync_host_runtime_state(&mut host.borrow_mut(), &scene, &selection.ids);
    let mut effects: Vec<Effect> = Vec::new();
    let mut artifact_mutations = Vec::new();
    let mut ui_scope = UiDirtyScope::Full;
    {
        let ctx = &mut Puzzle2dActionCtx {
            host: &host,
            scene: &mut scene,
            window_id,
            active_utility,
            selection: &selection,
            effects: &mut effects,
            artifact_mutations: &mut artifact_mutations,
            ui_scope: &mut ui_scope,
            operation: None,
        };
        apply_board_events::apply_board_events(ctx, command.args());
    }
    apply_host_events(&mut host.borrow_mut(), &mut scene);
    let mut operations = puzzle2d_document_delta_operations(&before, &scene.fixture);
    operations.append(&mut artifact_mutations);
    if !operations.is_empty() && matches!(ui_scope, UiDirtyScope::None) {
        ui_scope = UiDirtyScope::Full;
    }
    let config_mutations = if &scene.runtime != config { vec![Puzzle2dConfigMutation::Snapshot { config: scene.runtime }] } else { Vec::new() };
    Ok(Emit { artifact_mutations: operations, config_mutations, coalesce_key: None, effects, ui_scope, ..Default::default() })
}

fn puzzle2d_retained_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command.action_id(), "addNode").then_some(1)
}

fn puzzle2d_retained_reduce(
    command: &Puzzle2dCommand,
    _snapshot: &Puzzle2dPlaySnapshot,
    _config: &Puzzle2dConfig,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, Fault> {
    if command.action_id() != "addNode" {
        return Err(Fault::from("puzzle2d-retained-command-mismatch"));
    }
    let mut fixture = json!({ "nodes": [] });
    add_node_to_fixture(&mut fixture, command.args().and_then(|args| args.get("kind")).and_then(Value::as_str), command.args());
    let node = fixture.get_mut("nodes").and_then(Value::as_array_mut).and_then(Vec::pop).ok_or_else(|| Fault::from("puzzle2d-add-node-owner-lost"))?;
    let node = serde_json::from_value(node).map_err(|_| Fault::from("puzzle2d-add-node-malformed"))?;
    Ok(Emit { artifact_mutations: vec![crate::artifacts::puzzle2d::mutations::create_node(node, None)], ui_scope: UiDirtyScope::Full, ..Default::default() })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle2dExampleStage {
    ClearEdges,
    ClearNodes,
    Manifest,
    ClearCompatibility,
    AddCompatibility,
    Catalogs,
    Nodes,
    Edges,
    Complete,
    Closing,
}

struct Puzzle2dActiveExampleWork {
    stage: Puzzle2dExampleStage,
    source_cursor: usize,
    target_cursor: usize,
    mutations: Vec<Puzzle2dMutation>,
}

impl Default for Puzzle2dActiveExampleWork {
    fn default() -> Self {
        Self { stage: Puzzle2dExampleStage::ClearEdges, source_cursor: 0, target_cursor: 0, mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS) }
    }
}

impl Puzzle2dActiveExampleWork {
    fn target(command: &Puzzle2dCommand) -> &'static crate::artifacts::puzzle2d::Puzzle2dSnapshot {
        let id = set_active_example::canonical_example_id(command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or(""));
        set_active_example::target(id)
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle2dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dActiveExampleWork {
    fn tool_id(&self) -> &'static str {
        "setActiveExample"
    }

    fn extent(&self, command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let target = Self::target(command);
        let source_nodes = snapshot.0.get("nodes").and_then(Value::as_array).map_or(0, Vec::len);
        let source_edges = snapshot.0.get("edges").and_then(Value::as_array).map_or(0, Vec::len);
        let source_compatibility = snapshot.0.get("meta").and_then(|meta| meta.get("kindCompatibility")).and_then(Value::as_array).map_or(0, Vec::len);
        let items = source_nodes
            .checked_add(source_edges)?
            .checked_add(source_compatibility)?
            .checked_add(target.nodes.len())?
            .checked_add(target.edges.len())?
            .checked_add(target.meta.kind_compatibility.len())?
            .checked_add(2)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle2dCommand,
        snapshot: &Puzzle2dPlaySnapshot,
        config: &Puzzle2dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        let target = Self::target(command);
        match self.stage {
            Puzzle2dExampleStage::ClearEdges => {
                let source = snapshot.0.get("edges").and_then(Value::as_array).and_then(|rows| rows.get(self.source_cursor));
                if let Some(id) = source.and_then(|row| row.get("id")).and_then(Value::as_str) {
                    self.mutations.push(crate::artifacts::puzzle2d::mutations::disconnect_handles(id.to_string()));
                    self.source_cursor += 1;
                    return Ok(Self::progress("puzzle2d-example-clear-edge", "Removing existing edge", "Bestehende Kante wird entfernt"));
                }
                self.source_cursor = 0;
                self.stage = Puzzle2dExampleStage::ClearNodes;
                Ok(Self::progress("puzzle2d-example-clear-node", "Removing existing node", "Bestehender Knoten wird entfernt"))
            }
            Puzzle2dExampleStage::ClearNodes => {
                let source = snapshot.0.get("nodes").and_then(Value::as_array).and_then(|rows| rows.get(self.source_cursor));
                if let Some(id) = source.and_then(|row| row.get("id")).and_then(Value::as_str) {
                    self.mutations.push(crate::artifacts::puzzle2d::mutations::delete_node(id.to_string()));
                    self.source_cursor += 1;
                    return Ok(Self::progress("puzzle2d-example-clear-node", "Removing existing node", "Bestehender Knoten wird entfernt"));
                }
                self.source_cursor = 0;
                self.stage = Puzzle2dExampleStage::Manifest;
                Ok(Self::progress("puzzle2d-example-manifest", "Updating example manifest", "Beispielmanifest wird aktualisiert"))
            }
            Puzzle2dExampleStage::Manifest => {
                let current = snapshot.0.get("meta").and_then(|meta| meta.get("manifestId")).and_then(Value::as_str);
                if current != target.meta.manifest_id.as_deref() {
                    self.mutations.push(crate::artifacts::puzzle2d::mutations::change_manifest_id(target.meta.manifest_id.clone()));
                }
                self.stage = Puzzle2dExampleStage::ClearCompatibility;
                Ok(Self::progress("puzzle2d-example-clear-compatibility", "Removing kind relation", "Artbeziehung wird entfernt"))
            }
            Puzzle2dExampleStage::ClearCompatibility => {
                let source = snapshot.0.get("meta").and_then(|meta| meta.get("kindCompatibility")).and_then(Value::as_array).and_then(|rows| rows.get(self.source_cursor));
                if let Some(source) = source {
                    let row: crate::artifacts::puzzle2d::Puzzle2dKindCompatibility = serde_json::from_value(source.clone()).map_err(|_| Fault::from("puzzle2d-example-compatibility-malformed"))?;
                    self.mutations.push(crate::artifacts::puzzle2d::mutations::disconnect_kind_compatibility(row.source, row.target));
                    self.source_cursor += 1;
                    return Ok(Self::progress("puzzle2d-example-clear-compatibility", "Removing kind relation", "Artbeziehung wird entfernt"));
                }
                self.stage = Puzzle2dExampleStage::AddCompatibility;
                Ok(Self::progress("puzzle2d-example-add-compatibility", "Adding kind relation", "Artbeziehung wird hinzugefügt"))
            }
            Puzzle2dExampleStage::AddCompatibility => {
                if let Some(row) = target.meta.kind_compatibility.get(self.target_cursor) {
                    self.mutations.push(crate::artifacts::puzzle2d::mutations::connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity));
                    self.target_cursor += 1;
                    return Ok(Self::progress("puzzle2d-example-add-compatibility", "Adding kind relation", "Artbeziehung wird hinzugefügt"));
                }
                self.target_cursor = 0;
                self.stage = Puzzle2dExampleStage::Catalogs;
                Ok(Self::progress("puzzle2d-example-catalogs", "Replacing kind catalogs", "Artkataloge werden ersetzt"))
            }
            Puzzle2dExampleStage::Catalogs => {
                self.mutations.push(crate::artifacts::puzzle2d::mutations::replace_kind_catalogs(target.meta.kind_catalogs.clone()));
                self.stage = Puzzle2dExampleStage::Nodes;
                Ok(Self::progress("puzzle2d-example-node", "Adding example node", "Beispielknoten wird hinzugefügt"))
            }
            Puzzle2dExampleStage::Nodes => {
                if let Some(node) = target.nodes.get(self.target_cursor) {
                    self.mutations.push(crate::artifacts::puzzle2d::mutations::create_node(node.clone(), None));
                    self.target_cursor += 1;
                    return Ok(Self::progress("puzzle2d-example-node", "Adding example node", "Beispielknoten wird hinzugefügt"));
                }
                self.target_cursor = 0;
                self.stage = Puzzle2dExampleStage::Edges;
                Ok(Self::progress("puzzle2d-example-edge", "Adding example edge", "Beispielkante wird hinzugefügt"))
            }
            Puzzle2dExampleStage::Edges => {
                if let Some(edge) = target.edges.get(self.target_cursor) {
                    self.mutations.push(crate::artifacts::puzzle2d::mutations::connect_handles(
                        edge.id.clone(), edge.source.clone(), edge.target.clone(), edge.edge_kind.clone(), edge.gap, edge.shift, edge.rise, edge.rotation, edge.turn, edge.tilt, edge.x, edge.y, edge.source_tip.clone(), edge.target_tip.clone(),
                    ));
                    self.target_cursor += 1;
                    return Ok(Self::progress("puzzle2d-example-edge", "Adding example edge", "Beispielkante wird hinzugefügt"));
                }
                self.stage = Puzzle2dExampleStage::Complete;
                let generation = config.example_load_generation.saturating_add(1);
                let mut next = Puzzle2dPlayRuntime::default();
                next.example_load_generation = generation;
                let mutations = std::mem::take(&mut self.mutations);
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit {
                    artifact_mutations: mutations,
                    config_mutations: vec![Puzzle2dConfigMutation::Snapshot { config: next }],
                    coalesce_key: Some(format!("setActiveExample:{generation}")),
                    ui_scope: UiDirtyScope::Full,
                    ..Default::default()
                }))
            }
            Puzzle2dExampleStage::Complete => Err(Fault::from("puzzle2d-example-complete-repolled")),
            Puzzle2dExampleStage::Closing => Err(Fault::from("puzzle2d-example-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle2dExampleStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle2dExampleStage::Closing && self.mutations.is_empty()
    }
}

const PUZZLE2D_FORCE_MAX_NODES: usize = 64;
const PUZZLE2D_FORCE_MAX_EDGES: usize = 512;
const PUZZLE2D_FORCE_MAX_HANDLES: usize = 512;
const PUZZLE2D_FORCE_ITERATIONS: u32 = 420;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle2dForceStage {
    Nodes,
    Handles,
    Edges,
    Seed,
    Center,
    Reset,
    Repel,
    Springs,
    Integrate,
    Emit,
    Complete,
    Closing,
}

struct Puzzle2dForceLayoutWork {
    stage: Puzzle2dForceStage,
    node_cursor: usize,
    handle_cursor: usize,
    edge_cursor: usize,
    seed_cursor: usize,
    center_cursor: usize,
    force_cursor: usize,
    pair_i: usize,
    pair_j: usize,
    iteration: u32,
    rng: u64,
    center: [f64; 2],
    finite_count: usize,
    node_ids: Vec<String>,
    raw_node_indices: Vec<usize>,
    original: Vec<Option<[f64; 2]>>,
    positions: Vec<[f64; 2]>,
    velocities: Vec<[f64; 2]>,
    forces: Vec<[f64; 2]>,
    radii: Vec<f64>,
    id_to_index: HashMap<String, usize>,
    handle_to_node: HashMap<String, String>,
    edges: Vec<(usize, usize)>,
    edge_set: HashSet<(usize, usize)>,
    mutations: Vec<Puzzle2dMutation>,
    retained_bytes: usize,
}

impl Default for Puzzle2dForceLayoutWork {
    fn default() -> Self {
        Self {
            stage: Puzzle2dForceStage::Nodes,
            node_cursor: 0,
            handle_cursor: 0,
            edge_cursor: 0,
            seed_cursor: 0,
            center_cursor: 0,
            force_cursor: 0,
            pair_i: 0,
            pair_j: 1,
            iteration: 0,
            rng: 0x5eedfaced0,
            center: [0.0, 0.0],
            finite_count: 0,
            node_ids: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            raw_node_indices: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            original: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            positions: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            velocities: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            forces: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            radii: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            id_to_index: HashMap::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            handle_to_node: HashMap::with_capacity(PUZZLE2D_FORCE_MAX_HANDLES),
            edges: Vec::with_capacity(PUZZLE2D_FORCE_MAX_EDGES),
            edge_set: HashSet::with_capacity(PUZZLE2D_FORCE_MAX_EDGES),
            mutations: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            retained_bytes: 0,
        }
    }
}

impl Puzzle2dForceLayoutWork {
    fn visible(object: &serde_json::Map<String, Value>) -> bool {
        object.get("hidden").and_then(Value::as_bool).map_or_else(|| object.get("visible").and_then(Value::as_bool).unwrap_or(true), |hidden| !hidden)
    }

    fn radius(node: &Value) -> f64 {
        let Some(object) = node.as_object() else { return 32.0 };
        if object.get("shape").and_then(Value::as_str) == Some("rectangle") {
            let width = object.get("width").and_then(Value::as_f64).unwrap_or(40.0);
            let height = object.get("height").and_then(Value::as_f64).unwrap_or(40.0);
            return ((width * width + height * height).sqrt() * 0.5).max(8.0);
        }
        object.get("radius").and_then(Value::as_f64).filter(|radius| radius.is_finite() && *radius > 0.0).unwrap_or(32.0)
    }

    fn split_mix64(mut value: u64) -> u64 {
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D049BB133111EB);
        value ^ (value >> 31)
    }

    fn random_unit(&mut self) -> f64 {
        self.rng = Self::split_mix64(self.rng);
        (self.rng as f64) / (u64::MAX as f64)
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle2dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn pop_one(&mut self) -> bool {
        macro_rules! pop {
            ($field:ident) => {
                if self.$field.pop().is_some() {
                    return true;
                }
            };
        }
        pop!(mutations);
        pop!(edges);
        pop!(radii);
        pop!(forces);
        pop!(velocities);
        pop!(positions);
        pop!(original);
        pop!(raw_node_indices);
        pop!(node_ids);
        if let Some(key) = self.handle_to_node.keys().next().cloned() {
            self.handle_to_node.remove(&key);
            return true;
        }
        if let Some(key) = self.id_to_index.keys().next().cloned() {
            self.id_to_index.remove(&key);
            return true;
        }
        if let Some(edge) = self.edge_set.iter().next().copied() {
            self.edge_set.remove(&edge);
            return true;
        }
        false
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dForceLayoutWork {
    fn tool_id(&self) -> &'static str {
        "forceLayout"
    }

    fn extent(&self, command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        if command.action_id() != "forceLayout" || snapshot.0.get("schema").and_then(Value::as_str) != Some(PUZZLE2D_FIXTURE_SCHEMA) {
            return None;
        }
        let nodes = snapshot.0.get("nodes")?.as_array()?.len();
        let edges = snapshot.0.get("edges").and_then(Value::as_array).map_or(0, Vec::len);
        (nodes <= PUZZLE2D_FORCE_MAX_NODES && edges <= PUZZLE2D_FORCE_MAX_EDGES).then(|| nodes.saturating_add(edges).max(1))
    }

    fn step(
        &mut self,
        _command: &Puzzle2dCommand,
        snapshot: &Puzzle2dPlaySnapshot,
        _config: &Puzzle2dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        let nodes = snapshot.0.get("nodes").and_then(Value::as_array).ok_or_else(|| Fault::from("puzzle2d-force-nodes-missing"))?;
        match self.stage {
            Puzzle2dForceStage::Nodes => {
                let Some(node) = nodes.get(self.node_cursor) else {
                    if self.positions.is_empty() {
                        self.stage = Puzzle2dForceStage::Emit;
                        return Ok(Self::progress("puzzle2d-force-emit", "Preparing layout result", "Layout-Ergebnis wird vorbereitet"));
                    }
                    if self.finite_count > 0 {
                        self.center[0] /= self.finite_count as f64;
                        self.center[1] /= self.finite_count as f64;
                    }
                    self.stage = Puzzle2dForceStage::Edges;
                    return Ok(Self::progress("puzzle2d-force-edges", "Indexing layout edges", "Layout-Kanten werden indiziert"));
                };
                let object = node.as_object().ok_or_else(|| Fault::from("puzzle2d-force-node-not-object"))?;
                if !Self::visible(object) {
                    self.node_cursor += 1;
                    return Ok(Self::progress("puzzle2d-force-node", "Reading layout node", "Layout-Knoten wird gelesen"));
                }
                let id = object.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle2d-force-node-id-missing"))?;
                if self.node_ids.len() >= PUZZLE2D_FORCE_MAX_NODES || self.retained_bytes.checked_add(id.len()).map_or(true, |bytes| bytes > crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES) {
                    return Err(Fault::from("puzzle2d-force-node-capacity"));
                }
                let x = object.get("x").and_then(Value::as_f64);
                let y = object.get("y").and_then(Value::as_f64);
                let original = match (x, y) {
                    (Some(x), Some(y)) if x.is_finite() && y.is_finite() => Some([x, y]),
                    _ => None,
                };
                let index = self.positions.len();
                self.retained_bytes += id.len();
                self.id_to_index.insert(id.to_string(), index);
                self.node_ids.push(id.to_string());
                self.raw_node_indices.push(self.node_cursor);
                self.original.push(original);
                self.positions.push(original.unwrap_or([0.0, 0.0]));
                if let Some(position) = original {
                    self.center[0] += position[0];
                    self.center[1] += position[1];
                    self.finite_count += 1;
                }
                self.velocities.push([0.0, 0.0]);
                self.forces.push([0.0, 0.0]);
                self.radii.push(Self::radius(node));
                self.handle_cursor = 0;
                self.stage = Puzzle2dForceStage::Handles;
                Ok(Self::progress("puzzle2d-force-node", "Reading layout node", "Layout-Knoten wird gelesen"))
            }
            Puzzle2dForceStage::Handles => {
                let object = nodes.get(self.node_cursor).and_then(Value::as_object).ok_or_else(|| Fault::from("puzzle2d-force-node-owner-lost"))?;
                let handles = object.get("handles").and_then(Value::as_array);
                let Some(handle) = handles.and_then(|handles| handles.get(self.handle_cursor)) else {
                    self.node_cursor += 1;
                    self.stage = Puzzle2dForceStage::Nodes;
                    return Ok(Self::progress("puzzle2d-force-node", "Reading layout node", "Layout-Knoten wird gelesen"));
                };
                self.handle_cursor += 1;
                let Some(handle_object) = handle.as_object() else { return Ok(Self::progress("puzzle2d-force-handle", "Reading layout handle", "Layout-Anschluss wird gelesen")) };
                if !Self::visible(handle_object) {
                    return Ok(Self::progress("puzzle2d-force-handle", "Reading layout handle", "Layout-Anschluss wird gelesen"));
                }
                let Some(handle_id) = handle_object.get("id").and_then(Value::as_str) else { return Ok(Self::progress("puzzle2d-force-handle", "Reading layout handle", "Layout-Anschluss wird gelesen")) };
                let node_id = object.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle2d-force-node-id-owner-lost"))?;
                let added_bytes = handle_id.len().saturating_add(node_id.len());
                if self.handle_to_node.len() >= PUZZLE2D_FORCE_MAX_HANDLES || self.retained_bytes.checked_add(added_bytes).map_or(true, |bytes| bytes > crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES) {
                    return Err(Fault::from("puzzle2d-force-handle-capacity"));
                }
                self.retained_bytes += added_bytes;
                self.handle_to_node.insert(handle_id.to_string(), node_id.to_string());
                Ok(Self::progress("puzzle2d-force-handle", "Reading layout handle", "Layout-Anschluss wird gelesen"))
            }
            Puzzle2dForceStage::Edges => {
                let edges = snapshot.0.get("edges").and_then(Value::as_array);
                let Some(edge) = edges.and_then(|edges| edges.get(self.edge_cursor)) else {
                    self.stage = Puzzle2dForceStage::Seed;
                    return Ok(Self::progress("puzzle2d-force-seed", "Seeding layout node", "Layout-Knoten wird initialisiert"));
                };
                self.edge_cursor += 1;
                let Some(object) = edge.as_object() else { return Ok(Self::progress("puzzle2d-force-edge", "Indexing layout edge", "Layout-Kante wird indiziert")) };
                if !Self::visible(object) {
                    return Ok(Self::progress("puzzle2d-force-edge", "Indexing layout edge", "Layout-Kante wird indiziert"));
                }
                let (Some(source), Some(target)) = (object.get("source").and_then(Value::as_str), object.get("target").and_then(Value::as_str)) else {
                    return Ok(Self::progress("puzzle2d-force-edge", "Indexing layout edge", "Layout-Kante wird indiziert"));
                };
                let source_node = self.handle_to_node.get(source).map_or(source, String::as_str);
                let target_node = self.handle_to_node.get(target).map_or(target, String::as_str);
                let (Some(&a), Some(&b)) = (self.id_to_index.get(source_node), self.id_to_index.get(target_node)) else {
                    return Ok(Self::progress("puzzle2d-force-edge", "Indexing layout edge", "Layout-Kante wird indiziert"));
                };
                let pair = (a.min(b), a.max(b));
                if a != b && self.edge_set.insert(pair) {
                    if self.edges.len() >= PUZZLE2D_FORCE_MAX_EDGES {
                        return Err(Fault::from("puzzle2d-force-edge-capacity"));
                    }
                    self.edges.push(pair);
                }
                Ok(Self::progress("puzzle2d-force-edge", "Indexing layout edge", "Layout-Kante wird indiziert"))
            }
            Puzzle2dForceStage::Seed => {
                if self.seed_cursor >= self.positions.len() {
                    self.center = [0.0, 0.0];
                    self.stage = Puzzle2dForceStage::Center;
                    return Ok(Self::progress("puzzle2d-force-center", "Centering layout", "Layout wird zentriert"));
                }
                if self.positions[self.seed_cursor][0].hypot(self.positions[self.seed_cursor][1]) < 1e-9 {
                    let t = self.seed_cursor as f64;
                    let angle = t * 2.399_963_229_728_653_5;
                    let radius = 10.0 + t.sqrt() * 22.0;
                    let jitter_x = (self.random_unit() - 0.5) * 6.0;
                    let jitter_y = (self.random_unit() - 0.5) * 6.0;
                    self.positions[self.seed_cursor] = [self.center[0] + radius * angle.cos() + jitter_x, self.center[1] + radius * angle.sin() + jitter_y];
                }
                self.seed_cursor += 1;
                Ok(Self::progress("puzzle2d-force-seed", "Seeding layout node", "Layout-Knoten wird initialisiert"))
            }
            Puzzle2dForceStage::Center => {
                if let Some(position) = self.positions.get(self.center_cursor) {
                    self.center[0] += position[0];
                    self.center[1] += position[1];
                    self.center_cursor += 1;
                    return Ok(Self::progress("puzzle2d-force-center", "Centering layout", "Layout wird zentriert"));
                }
                if !self.positions.is_empty() {
                    self.center[0] /= self.positions.len() as f64;
                    self.center[1] /= self.positions.len() as f64;
                }
                self.stage = Puzzle2dForceStage::Reset;
                self.force_cursor = 0;
                Ok(Self::progress("puzzle2d-force-iteration", "Running layout iteration", "Layout-Iteration wird ausgeführt"))
            }
            Puzzle2dForceStage::Reset => {
                if self.force_cursor < self.forces.len() {
                    self.forces[self.force_cursor] = [0.0, 0.0];
                    self.force_cursor += 1;
                    return Ok(Self::progress("puzzle2d-force-reset", "Resetting layout force", "Layout-Kraft wird zurückgesetzt"));
                }
                self.pair_i = 0;
                self.pair_j = 1;
                self.stage = Puzzle2dForceStage::Repel;
                Ok(Self::progress("puzzle2d-force-repel", "Applying layout repulsion", "Layout-Abstoßung wird angewendet"))
            }
            Puzzle2dForceStage::Repel => {
                let count = self.positions.len();
                if self.pair_i >= count || self.pair_j >= count {
                    self.edge_cursor = 0;
                    self.stage = Puzzle2dForceStage::Springs;
                    return Ok(Self::progress("puzzle2d-force-spring", "Applying layout spring", "Layout-Feder wird angewendet"));
                }
                let i = self.pair_i;
                let j = self.pair_j;
                let dx = self.positions[j][0] - self.positions[i][0];
                let dy = self.positions[j][1] - self.positions[i][1];
                let distance = dx.hypot(dy).max(1e-4);
                let cool = (1.0 - self.iteration as f64 / PUZZLE2D_FORCE_ITERATIONS as f64).max(0.08);
                let repulsion = 6500.0 * cool * (self.radii[i] * self.radii[j]).max(1.0) / (distance * distance);
                let fx = dx / distance * -repulsion;
                let fy = dy / distance * -repulsion;
                self.forces[i][0] += fx;
                self.forces[i][1] += fy;
                self.forces[j][0] -= fx;
                self.forces[j][1] -= fy;
                self.pair_j += 1;
                if self.pair_j >= count {
                    self.pair_i += 1;
                    self.pair_j = self.pair_i.saturating_add(1);
                }
                Ok(Self::progress("puzzle2d-force-repel", "Applying layout repulsion", "Layout-Abstoßung wird angewendet"))
            }
            Puzzle2dForceStage::Springs => {
                let Some(&(i, j)) = self.edges.get(self.edge_cursor) else {
                    self.force_cursor = 0;
                    self.stage = Puzzle2dForceStage::Integrate;
                    return Ok(Self::progress("puzzle2d-force-integrate", "Integrating layout node", "Layout-Knoten wird integriert"));
                };
                let dx = self.positions[j][0] - self.positions[i][0];
                let dy = self.positions[j][1] - self.positions[i][1];
                let distance = dx.hypot(dy).max(1e-4);
                let cool = (1.0 - self.iteration as f64 / PUZZLE2D_FORCE_ITERATIONS as f64).max(0.08);
                let magnitude = 0.028 * cool * (distance - 140.0);
                let fx = dx / distance * magnitude;
                let fy = dy / distance * magnitude;
                self.forces[i][0] += fx;
                self.forces[i][1] += fy;
                self.forces[j][0] -= fx;
                self.forces[j][1] -= fy;
                self.edge_cursor += 1;
                Ok(Self::progress("puzzle2d-force-spring", "Applying layout spring", "Layout-Feder wird angewendet"))
            }
            Puzzle2dForceStage::Integrate => {
                if self.force_cursor >= self.positions.len() {
                    self.iteration += 1;
                    if self.iteration >= PUZZLE2D_FORCE_ITERATIONS {
                        self.force_cursor = 0;
                        self.stage = Puzzle2dForceStage::Emit;
                        return Ok(Self::progress("puzzle2d-force-emit", "Preparing layout mutation", "Layout-Mutation wird vorbereitet"));
                    }
                    self.force_cursor = 0;
                    self.stage = Puzzle2dForceStage::Reset;
                    return Ok(Self::progress("puzzle2d-force-iteration", "Running layout iteration", "Layout-Iteration wird ausgeführt"));
                }
                let index = self.force_cursor;
                let cool = (1.0 - self.iteration as f64 / PUZZLE2D_FORCE_ITERATIONS as f64).max(0.08);
                let delta_time = 0.85 * cool.sqrt();
                let mut velocity = [
                    (self.velocities[index][0] + self.forces[index][0] * delta_time) * 0.88,
                    (self.velocities[index][1] + self.forces[index][1] * delta_time) * 0.88,
                ];
                let speed = velocity[0].hypot(velocity[1]);
                if speed > 48.0 {
                    velocity[0] *= 48.0 / speed;
                    velocity[1] *= 48.0 / speed;
                }
                self.velocities[index] = velocity;
                self.positions[index][0] += velocity[0] * delta_time;
                self.positions[index][1] += velocity[1] * delta_time;
                self.force_cursor += 1;
                Ok(Self::progress("puzzle2d-force-integrate", "Integrating layout node", "Layout-Knoten wird integriert"))
            }
            Puzzle2dForceStage::Emit => {
                let index = self.force_cursor;
                let Some(position) = self.positions.get(index).copied() else {
                    self.stage = Puzzle2dForceStage::Complete;
                    let mutations = std::mem::take(&mut self.mutations);
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: mutations, ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                if self.original[index] != Some(position) {
                    self.mutations.push(crate::artifacts::puzzle2d::mutations::move_node(self.node_ids[index].clone(), position[0], position[1]));
                }
                self.force_cursor += 1;
                Ok(Self::progress("puzzle2d-force-emit", "Preparing layout mutation", "Layout-Mutation wird vorbereitet"))
            }
            Puzzle2dForceStage::Complete => Err(Fault::from("puzzle2d-force-complete-repolled")),
            Puzzle2dForceStage::Closing => Err(Fault::from("puzzle2d-force-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle2dForceStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.pop_one() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle2dForceStage::Closing
            && self.node_ids.is_empty()
            && self.raw_node_indices.is_empty()
            && self.original.is_empty()
            && self.positions.is_empty()
            && self.velocities.is_empty()
            && self.forces.is_empty()
            && self.radii.is_empty()
            && self.id_to_index.is_empty()
            && self.handle_to_node.is_empty()
            && self.edges.is_empty()
            && self.edge_set.is_empty()
            && self.mutations.is_empty()
    }
}

//#endregion 🧵️RetainedCommands

impl ArtifactEditor for Puzzle2dPlayApp {
    const DIALECT: Dialect = crate::artifacts::puzzle2d::PUZZLE2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE2D_FIXTURE_SCHEMA;
    type Snapshot = Puzzle2dPlaySnapshot;
    type Mutation = Puzzle2dMutation;
    type Config = Puzzle2dConfig;
    type ConfigMutation = Puzzle2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = Puzzle2dPresence;
    type PresenceMutation = Puzzle2dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = Puzzle2dCommand;

    /// 📎 Ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d: replaces the old
    /// `crate::editor::puzzle2d::config::schema::register_app_schema()` self-registering call, which
    /// puzzle's plugin root used to reach `.setup()` for — `register_document_app`/`document_app`
    /// now call this automatically the moment `Puzzle2dPlayApp` is bound to a plugin, exactly like
    /// `🗒️note`'s own `app_schema` override.
    fn app_schema() -> Option<artifact_schema::AppSchemaDescriptor> {
        Some(crate::editor::puzzle2d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Puzzle2dPlaySnapshot {
        set_active_example::warm_examples();
        Puzzle2dPlaySnapshot(serde_json::to_value(default_empty_fixture()).unwrap_or(Value::Null))
    }

    /// 🏷️ Maps each `Puzzle2dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle2dCommand) -> &'static str {
        command.action_id()
    }

    fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let window_id = args.and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(Value::as_str).map(str::to_string);
        Puzzle2dCommand::try_from_action(action, args.cloned(), window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 2D action '{action}'")))
    }

    fn mounted_job_prepare_snapshot_read(operation: semio_framework_plugin::AppRenderOperationContext, snapshot: &Self::Snapshot) -> bool {
        set_fill_count::prepare_snapshot_read(operation, snapshot)
    }

    fn pending_effects(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>) -> Vec<Effect> {
        set_fill_count::reconcile_snapshot_read(doc, cfg)
    }

    /// 🎬️ Dispatches mounted fill continuations before document materialization; ordinary commands then
    /// sync the board host, delegate to `🎮️commands/*`, replay host events, and derive granular deltas.
    fn handle(
        command: &Puzzle2dCommand,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation, Self::DraftMutation>, Fault> {
        let config = cfg.snapshot;
        let (action, args, window_id) = (command.action_id(), command.args(), command.window_id());
        if action == "setActiveExample" {
            return Ok(set_active_example::begin_active_example(config, args));
        }
        if action == set_active_example::STEP_ACTION_ID {
            return Ok(set_active_example::step_active_example(doc.snapshot, config, args));
        }
        if is_fill_session_action(action) {
            let mut runtime = crate::editor::puzzle2d::config::Puzzle2dFillRuntime::from_config(config);
            let mut effects = Vec::new();
            let mut artifact_mutations = Vec::new();
            let mut boundary_fault = None;
            {
                let ctx = &mut set_fill_count::Puzzle2dFillActionCtx { runtime: &mut runtime, effects: &mut effects, artifact_mutations: &mut artifact_mutations, operation: doc.operation_optional().cloned(), boundary_fault: &mut boundary_fault };
                dispatch_fill_session_action(action, args, ctx);
            }
            if let Some(code) = boundary_fault {
                return Err(Fault::from(code));
            }
            let config_mutations = if runtime.differs_from(config) { vec![Puzzle2dConfigMutation::Fill { runtime }] } else { Vec::new() };
            return Ok(Emit { artifact_mutations, config_mutations, coalesce_key: None, effects, ui_scope: UiDirtyScope::Full, ..Default::default() });
        }
        let before = doc.snapshot.0.clone();
        let active_utility = puzzle2d_active_utility(config, window_id);
        let mut scene = Self::scene_for(before.clone(), config, window_id);
        let selection = interaction.selection(PUZZLE2D_INTERACTION_DOMAIN);
        // 🐚️ ArtifactApp::handle is pure (no &self) — rebuild a fresh BoardHost from the document
        // each call. The previous last_synced_fixture cache lived on &self and cannot return.
        let host = RefCell::new(BoardHost::default());
        {
            let mut host_mut = host.borrow_mut();
            if action != "applyBoardEvents" {
                sync_host_fixture_content(&mut host_mut, &scene);
                let _ = drain_board_events_json(&mut host_mut);
            }
            sync_host_runtime_state(&mut host_mut, &scene, &selection.ids);
        }
        let mut effects: Vec<Effect> = Vec::new();
        let mut artifact_mutations = Vec::new();
        // 🐢️ Default to Full (safe: every unrecognized/rare action re-renders everything); the
        // narrow-tier arms below override it to the smallest scope that actually covers what they touch.
        let mut ui_scope = UiDirtyScope::Full;
        {
            let ctx = &mut Puzzle2dActionCtx {
                host: &host,
                scene: &mut scene,
                window_id,
                active_utility,
                selection: &selection,
                effects: &mut effects,
                artifact_mutations: &mut artifact_mutations,
                ui_scope: &mut ui_scope,
                operation: doc.operation_optional().cloned(),
            };
            match action {
                "selectSameKind" => select_same_kind::select_same_kind(ctx),
                "deleteSelection" => delete_selection::delete_selection(ctx),
                "duplicateSelection" => duplicate_selection::duplicate_selection(ctx),
                "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),
                "addNode" => add_node::add_node(ctx, args),
                "patchInspectorNodes" => patch_inspector::patch_inspector(ctx, args),
                "redrawHandles" => redraw_handles::redraw_handles(ctx),
                "forceLayout" | "reorganize" => force_layout::force_layout(ctx),
                "setCamera" => set_camera::set_camera(ctx, args),
                "focusSelection" => focus_selection::focus_selection(ctx),
                SET_ACTIVE_UTILITY_ACTION_ID => set_active_utility::set_active_utility(ctx, args),
                "engagementInput" => engagement_input::engagement_input(ctx, args),
                "engagementSubmit" => engagement_submit::engagement_submit(ctx, args),
                "engagementAbort" => engagement_abort::engagement_abort(ctx, args),
                "engagementControlSelect" => engagement_control_select::engagement_control_select(ctx, args),
                "setLodModeForPane" => set_lod_mode_for_pane::set_lod_mode_for_pane(ctx, args),
                "lodScaleJson" => lod_scale_json::lod_scale_json(ctx),
                "setGridSnapEnabled" => set_grid_snap_enabled::set_grid_snap_enabled(ctx, args),
                "setGridFactor" => set_grid_factor::set_grid_factor(ctx, args),
                "setBrushKindWeights" => set_brush_kind_weights::set_brush_kind_weights(ctx, args),
                "setBrushNodeSize" => set_brush_node_size::set_brush_node_size(ctx, args),
                "setSuggestionOffset" => set_suggestion_offset::set_suggestion_offset(ctx, args),
                "brushCycleCandidate" => cycle_candidate::cycle_candidate(ctx, args),
                "brushSetCandidateIndex" => set_candidate_index::set_candidate_index(ctx, args),
                "brushOpenSlot" => open_slot::open_slot(ctx, args),
                "brushCommitSlot" => commit_slot::commit_slot(ctx),
                "brushCancelSlot" => cancel_slot::cancel_slot(ctx),
                "applyBoardEvents" => apply_board_events::apply_board_events(ctx, args),
                "setLocale" => set_locale::set_locale(ctx, args),
                "setTerminology" => set_terminology::set_terminology(ctx, args),
                _ => {}
            }
        }
        apply_host_events(&mut host.borrow_mut(), &mut scene);
        let mut operations = puzzle2d_document_delta_operations(&before, &scene.fixture);
        operations.append(&mut artifact_mutations);
        // 🐢️ Safety net: a `None` scope claims nothing needs re-rendering — never pair that with an
        // actual document mutation (would silently desync remote clients' UI from the committed operation).
        if !operations.is_empty() && matches!(ui_scope, UiDirtyScope::None) {
            ui_scope = UiDirtyScope::Full;
        }
        // 🧮️ B1: only a REAL config change becomes a `Puzzle2dConfigMutation` — `PartialEq` (derived)
        // makes this cheap, and keeps a pure read-only action from creating a no-op undo entry.
        let config_mutations = if &scene.runtime != config { vec![Puzzle2dConfigMutation::Snapshot { config: scene.runtime }] } else { Vec::new() };
        // 🎥️ No action coalesces anymore: `setCamera` used to be the sole `coalesce_key` writer, but it
        // is now a View-kind action that never touches the document.
        Ok(Emit { artifact_mutations: operations, config_mutations, coalesce_key: None, effects, ui_scope, ..Default::default() })
    }

    /// 🔌️ Declares puzzle2d's typed media I/O surface — the implicit document ports plus `kit:in`
    /// (see `import_media` below for why it stays `NotImplemented`) and `design:out`.
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<Puzzle2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "s.puzzle.puzzle2d@1/*#editor",
        document_schema: "puzzle.2d.fixture",
        factory: "Puzzle2dRetainedCommandJobFactory",
        factory_type: Puzzle2dRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(8_192, 512, 1, 262_144, 7_500, 1, 1),
        tools: ["setActiveExample", "forceLayout", "addNode", "applyBoardEvents"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Puzzle2dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::app::ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !PUZZLE2D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(Fault::from("puzzle2d-command-tool-mismatch"));
        }
        let work: Box<dyn crate::retained_command::PuzzleCommandWork<EditorApp<Self>>> = match request.command.action_id() {
            "setActiveExample" => Box::new(Puzzle2dActiveExampleWork::default()),
            "forceLayout" => Box::new(Puzzle2dForceLayoutWork::default()),
            "addNode" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new("addNode", puzzle2d_retained_reduce, puzzle2d_retained_extent)),
            "applyBoardEvents" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new("applyBoardEvents", puzzle2d_board_events_reduce, puzzle2d_board_events_extent)),
            _ => return Err(Fault::from("puzzle2d-command-tool-unmapped")),
        };
        let payload = crate::retained_command::RetainedPuzzleCommandPayload {
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            interaction_state: request.interaction_state,
            interaction_hover: request.interaction_hover,
            completion: request.completion,
            command_id: Puzzle2dCommand::action_id,
            work,
        };
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn io() -> Option<AppIo> {
        let io = semio_framework::io::resolve_ready(AppIo::from_document("puzzle.2d", MediaType { class: MediaClass::TwoD, form: MediaForm::Design }, ArtifactPresentation { id: "2d.puzzle".into(), name: "2D Puzzle".into(), dimension: "2d".into(), component_kind: "puzzle2d".into() }));
        Some(semio_framework::io::resolve_ready(io.with_ports(vec![
                    MediaPortSpec {
                        id: "kit:in".into(),
                        label: "Kit Catalog".into(),
                        direction: MediaPortDirection::In,
                        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                        kind_id: Some("kit.catalog".into()),
                        required: false,
                        multiplicity: PortMultiplicity::Many,
                    },
                    MediaPortSpec {
                        id: "design:out".into(),
                        label: "Puzzle Design".into(),
                        direction: MediaPortDirection::Out,
                        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Design },
                        kind_id: Some("2d.puzzle".into()),
                        required: false,
                        multiplicity: PortMultiplicity::Many,
                    },
                ])))
    }

    /// 🚧️ `kit:in` is declared (see `io()`) so a future producer/consumer pairing is possible, but
    /// puzzle2d's own "kind catalogs" (node/handle kind weights, keyed by this app's own node/handle
    /// kind vocabulary) are structurally UNRELATED to `kit.catalog`'s shape (block3d's object/vortex-kind
    /// vocabulary — meshes, 3D vortex positions, cable/attraction kinds), unlike puzzle3d's `kit:in`,
    /// which DOES share block3d's object-kind vocabulary. There is no honest mapping to fabricate, so
    /// this always reports `NotImplemented` — no normalization is attempted.
    fn import_media(_port: &str, _media: &Media, _doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation, Self::DraftMutation>, MediaError> {
        Err(MediaError::NotImplemented)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let config = cfg.snapshot;
        let document_json = doc.snapshot.0.to_string();
        // 🪟️ `body_key` already determines the pane deterministically, so the active utility resolves
        // off the real targeted pane instead of an ambiguous stand-in.
        let pane = match body_key {
            overview::BODY_KEY => Some(overview::WINDOW_KIND_ID),
            detail::BODY_KEY => Some(detail::WINDOW_KIND_ID),
            selection::BODY_KEY => Some(selection::WINDOW_KIND_ID),
            _ => None,
        };
        let envelope = Self::scene_for(doc.snapshot.0.clone(), config, pane);
        let labels = puzzle2d_labels(config);
        let node = match body_key {
            overview::BODY_KEY => overview::render(&document_json, &envelope)?,
            detail::BODY_KEY => detail::render(&document_json, &envelope)?,
            selection::BODY_KEY => selection::render(&document_json, &envelope)?,
            document::PUZZLE2D_PLAY_BODY_LAYERS => document::render(&envelope, labels)?,
            catalogue::PUZZLE2D_PLAY_BODY_CATALOGUE => catalogue::render(&envelope.fixture, labels)?,
            inspection::PUZZLE2D_PLAY_BODY_PROPERTIES => inspection::render(&envelope, labels)?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d unknown-body label admission failed"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    fn window_engagements(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>) -> HashMap<String, WindowEngagement> {
        let config = cfg.snapshot;
        let labels = puzzle2d_labels(config);
        // 🪟️ One entry per live window INSTANCE of each pane kind — see `window_instance_ids`'s
        // docstring for why puzzle2d always has exactly one instance per pane (no split tracking).
        PUZZLE2D_PANES
            .iter()
            .flat_map(|pane| {
                window_instance_ids(pane).into_iter().map(|wid| {
                    let envelope = Self::scene_for(doc.snapshot.0.clone(), config, Some(&wid));
                    (wid, edit::puzzle2d_engagement(&envelope, &puzzle_board_host(), pane, labels))
                })
            })
            .collect()
    }

    fn window_measures(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let labels = puzzle2d_labels(config);
        PUZZLE2D_PANES
            .iter()
            .flat_map(|pane| {
                window_instance_ids(pane).into_iter().map(|wid| {
                    let envelope = Self::scene_for(doc.snapshot.0.clone(), config, Some(&wid));
                    let measures = match *pane {
                        detail::WINDOW_KIND_ID => detail::window_measures(&envelope, labels),
                        selection::WINDOW_KIND_ID => selection::window_measures(&envelope, labels),
                        _ => overview::window_measures(&envelope, labels),
                    };
                    (wid, measures)
                })
            })
            .collect()
    }

    fn tool_measures(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let envelope = Self::scene_for(doc.snapshot.0.clone(), config, None);
        let labels = puzzle2d_labels(config);
        HashMap::from([(fill::TOOL_ID.to_string(), vec![fill::measures(&envelope, labels)])])
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let config = cfg.snapshot;
        let is_de = is_de_locale(config);
        let selected: Vec<String> = request.surface.as_ref().map(|surface| surface.selection.iter().flat_map(|g| g.ids.iter().cloned()).collect()).unwrap_or_default();
        semio_framework::io::resolve_ready(puzzle2d_context_menu_items(registry, &doc.snapshot.0, &selected, is_de))
    }
}
//#endregion 🔖️PlayApp

//#region 🔖️Manifest
/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector/engagement-bound
/// vocabulary dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn puzzle2d_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 🗺️ Builds the full `LocalizedLabel` matrix for one `Puzzle2dLabels` field — for the static
/// manifest, which must carry every (terminology, locale) cell up front rather than a single
/// resolved-at-render-time `LabelText` (see `terminology::puzzle2d_labels`).
pub fn puzzle2d_localized(field: impl Fn(&Puzzle2dLabels) -> semio_framework_plugin::LabelText) -> LocalizedLabel {
    LocalizedLabel::from_fn(|terminology, locale| field(Puzzle2dLabels::labels(locale, terminology)).as_str().to_string())
}

/// 🎭️✏️ `.example_source(...)` (×2, concrete-forest + nakagin) and `.workflow("puzzle2d", …)` were
/// dropped, not ported — `EditorBuilder` has neither method (contract §2.4: `.editor::<E>(def:
/// AppDefinition)` only takes the definition, `App.examples` has no seam on this builder). Flagged to
/// the coordinator, not silently lost; see `📚️examples/🎬️demo-session` for this subset's own example
/// facet, the likely intended replacement mechanism.
pub fn create_puzzle2d_app() -> semio_framework_plugin::AppDefinition {
    let mut host = puzzle_board_host();
    let envelope = Puzzle2dScene { fixture: default_empty_fixture(), runtime: Puzzle2dPlayRuntime::default(), active_utility: select_utility::UTILITY_ID.into() };
    sync_host_from_envelope(&mut host, &envelope);
    let labels = puzzle2d_labels(&Puzzle2dConfig::default());
    Editor::builder(Puzzle2dPlayApp::DIALECT)
            .document(["semio", "puzzle", "2d"])
            .artifact_kind(crate::artifacts::puzzle2d::artifact_kind())
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "puzzle", "2d"])
            .mode_def(edit::definition())
            .default_mode_id(edit::PUZZLE2D_PLAY_MODE_EDIT)
            .window_kind_def(overview::definition(&envelope, &host, labels))
            .window_kind_def(detail::definition(&envelope, &host, labels))
            .window_kind_def(selection::definition(&envelope, &host, labels))
            .interaction(puzzle2d_interaction_definition())
            .window_kind_interactions(overview::WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE2D_INTERACTION_DOMAIN)])
            .window_kind_interactions(detail::WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE2D_INTERACTION_DOMAIN)])
            .window_kind_interactions(selection::WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE2D_INTERACTION_DOMAIN)])
            .panel_tab_def(document::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            // ✏️ Palette-visible content operations.
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .action_with(puzzle2d_internal_action(set_active_example::STEP_ACTION_ID, LocalizedLabel::native("Set Active Example Step", "Aktives-Beispiel-Schritt"), ActionKind::Mutation))
            // 🗂️ Referenced by `puzzle2d_context_menu_items` — categorized for grouped-context-menu disclosure.
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection")))
            .keybinding("delete,backspace", "deleteSelection")
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).with_category("create")))
            .mutation("forceLayout", LocalizedLabel::native("Force Layout", "Kraftbasiertes Layout"))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"), ActionKind::Mutation).with_category("view")))
            // 👁️ Palette-visible ephemeral view/selection commands.
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("selectSameKind", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).with_category("selection")))
            // 🔧️ Internal content operations — inspector/panel/board/import-bound, not palette commands.
            .action_with(semio_framework::io::resolve_ready(puzzle2d_internal_action("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).with_category("settings")))
            .action_with(puzzle2d_internal_action("patchInspectorNodes", LocalizedLabel::native("Patch Inspector Nodes", "Inspektorknoten aktualisieren"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("redrawHandles", LocalizedLabel::native("Redraw Handles", "Anschlüsse neu zeichnen"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("applyBoardEvents", LocalizedLabel::native("Apply Board Events", "Board-Ereignisse anwenden"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("brushFillSessionStep", LocalizedLabel::native("Brush Fill Session Step", "Pinsel-Füllsitzung-Schritt"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("brushFillSessionAdopt", LocalizedLabel::native("Adopt Fill Result", "Füllergebnis übernehmen"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("brushFillSessionCancel", LocalizedLabel::native("Cancel Fill", "Füllen abbrechen"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("brushFillSessionRetry", LocalizedLabel::native("Retry Fill", "Füllen erneut versuchen"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("brushFillSessionDiscard", LocalizedLabel::native("Discard Fill Session", "Füllsitzung verwerfen"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("brushCommitSlot", LocalizedLabel::native("Brush Commit Slot", "Pinsel-Platz übernehmen"), ActionKind::Mutation))
            // 🖱️ Internal pointer/gesture/engagement view vocabulary — pure runtime/host state, emit no operations.
            // 🎥️ `setCamera` is session-only view state, so it belongs in this View-kind group.
            .action_with(puzzle2d_internal_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View))
            .action_with(puzzle2d_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setLodModeForPane", LocalizedLabel::native("Set LOD Mode For Pane", "LOD-Modus für Bereich festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setBrushKindWeights", LocalizedLabel::native("Set Brush Kind Weights", "Pinsel-Artgewichte festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setBrushNodeSize", LocalizedLabel::native("Set Brush Node Size", "Pinsel-Knotengröße festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setSuggestionOffset", LocalizedLabel::native("Set Suggestion Offset", "Vorschlagsversatz festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushCycleCandidate", LocalizedLabel::native("Brush Cycle Candidate", "Pinselkandidat wechseln"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushSetCandidateIndex", LocalizedLabel::native("Brush Set Candidate Index", "Pinselkandidatenindex festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushOpenSlot", LocalizedLabel::native("Brush Open Slot", "Pinsel-Platz öffnen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushCancelSlot", LocalizedLabel::native("Brush Cancel Slot", "Pinsel-Platz abbrechen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushFillSessionBegin", LocalizedLabel::native("Brush Fill Session Begin", "Pinsel-Füllsitzung beginnen"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("brushFillSessionClear", LocalizedLabel::native("Brush Fill Session Clear", "Pinsel-Füllsitzung leeren"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("lodScaleJson", LocalizedLabel::native("LOD Scale Json", "LOD-Skalierung-Json"), ActionKind::View))
            // 📝️ Staged palette args for the two content commands that need a target.
            .action_args("addNode", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![ActionArgOption::new("node", LocalizedLabel::native("Node", "Knoten"))]).required().default_value("node"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, puzzle2d_localized(|l| l.example_concrete_forest)),
                    ActionArgOption::new(PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin Capsule Tower")),
                ]).required().default_value(PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID),
            ])
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("forceLayout", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("applyBoardEvents", InteractiveJobClassification::Migrated)
            .action_interactive_job("brushCancelSlot", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("brushCommitSlot", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("brushCycleCandidate", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("brushFillSessionAdopt", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("brushFillSessionBegin", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("brushFillSessionCancel", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("brushFillSessionClear", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("brushFillSessionDiscard", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("brushFillSessionRetry", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("brushFillSessionStep", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("brushOpenSlot", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("brushSetCandidateIndex", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("duplicateSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("engagementAbort", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("engagementControlSelect", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("engagementInput", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("focusSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("lodScaleJson", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("patchInspectorNodes", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("redrawHandles", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("reorganize", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("selectSameKind", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setActiveExampleStep", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setBrushKindWeights", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setBrushNodeSize", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setCamera", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setFillCount", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setGridFactor", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setGridSnapEnabled", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setLodModeForPane", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setSelectionFlag", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setSuggestionOffset", InteractiveJobClassification::BatchOnlyPendingRewrite)
            // 🧰️ Canvas utilities — one exclusive set, active utility host-owned (never a document
            // operation); bound to the interactive overview pane by that window's own definition.
            .utility(select_utility::definition(puzzle2d_localized(|l| l.select)))
            .utility(brush_utility::definition(puzzle2d_localized(|l| l.brush)))
            // 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
            .tool(fill::definition(puzzle2d_localized(|l| l.fill)))
            .default_layout(edit::layout())
            .build_definition()
}

/// 📥️ Tier C DWG import — the puzzle-2d fixture only supports circle/rectangle nodes (no polygonal
/// outlines), so this always returns an empty board; never errors on a structurally valid DWG.
/// The DWG's extents no longer frame a camera here: the camera is session-only `Puzzle2dConfig`
/// state, and this import path produces a bare document with no live app instance to receive it.
/// Only exercised by its own `#[cfg(test)]` test below — never wired to a live `register_*` caller
/// after the artifacts-only-plugin-architecture migration removed `register_dwg_import_handler`
/// (see the `🔖️Register` region below) — so this is `#[cfg(test)]`-gated rather than a real
/// production entry point.
#[cfg(test)]
#[allow(clippy::unnecessary_wraps, reason = "the fallible signature matches the historical `semio_framework_os::register_dwg_import_handler` shape this once fed; puzzle-2d simply has no failure mode.")]
pub(crate) fn puzzle2d_document_json_from_dwg(_drawing: &semio_s_plugin_stdio::artifacts::dwg::DwgDrawing) -> Result<Value, String> {
    Ok(default_empty_fixture())
}

// 🗂️ `Puzzle2dPlaySnapshot`'s pack<->dsl codec (so `framework/sync`'s `FolderEndpoint::Pack` can
// print/parse puzzle-2d play documents without depending on this crate's concrete
// `Projection`/`Mutation` types) is now declared via `.document_codec::<Puzzle2dPlayApp>()` on
// `crate::artifacts::puzzle2d::declaration()` (ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`
// M1) — the old side-effecting `register_puzzle2d_exports()` wrapper (this app file's only caller of
// `register_document_codec_for_app`) is gone.
//#endregion 🔖️Manifest

//#region 🔖️Register
/// 🖼️ Registers the `"2d.puzzle"` SVG/DWG media export-import bridge with the OS host — no
/// `ArtifactDeclaration` field covers this OS-host media registry (see `declaration()`'s own doc in
/// the artifact root), so it stays wired through `🧩️puzzle/🦀️component.rs`'s `.setup()`. Rehomed
/// here from the deleted artifact-side `⚙️engine` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1e: `register_media_io` reaches this app's own
/// `puzzle2d_document_json_from_dwg` callback directly, so it now lives beside it instead of
/// crossing an artifact→app boundary). The SVG-export half of the bridge
/// (`puzzle2d_snapshot_to_drawing`/`puzzle2d_document_json_to_svg`) was never rewired to a real
/// caller after that move and was deleted as dead code (ticket
/// 26/08/17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS) — SVG export should be
/// re-derived from `io_dispatch`'s real `ComposerEntry` chain if/when this bridge is needed again.
///
/// 🚪️ Ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1: the
/// `register_dwg_import_handler` call this used to make is DELETED, not migrated --
/// `🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`'s `io_registry::entries()`
/// already carries a real `ComposerEntry` for `"s.stdio.dwg"` (`DEP_DWG`), reachable via
/// `io_dispatch` once the OS media pipeline's `native_kind` bridging bug is fixed (`component_kind`
/// = `"puzzle2d"`, not the raw `"2d.puzzle"` workflow kind id) -- and it is a strict improvement
/// over what this call registered: `puzzle2d_document_json_from_dwg` (still kept, still exercised
/// by its own test below) always returns an EMPTY board regardless of input (Tier C, no polygonal
/// outlines supported), while the artifact-level DWG deserializer does real entity parsing.
/// `register_2d_export_handlers` is a separate function, not in this wave's five-function scope.
//#endregion 🔖️Register

//#region 🧪️Testkit
/// 🧪️ The one puzzle2d-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
/// instead of re-deriving a store/dispatch/render scaffold of its own.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::{ActionMeta, App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type Puzzle2dApp = VcsArtifactApp<EditorApp<Puzzle2dPlayApp>>;

    pub fn meta(actor: &str) -> ActionMeta {
        semio_framework_plugin::testkit::meta(actor)
    }

    pub fn app() -> Puzzle2dApp {
        std::sync::LazyLock::force(&crate::examples::puzzle2d::nakagin_capsule_tower::SOURCE);
        std::sync::LazyLock::force(&crate::examples::puzzle2d::concrete_forest::SOURCE);
        semio_framework::io::resolve_ready(semio_framework_plugin::testkit::new_app::<EditorApp<Puzzle2dPlayApp>>())
    }

    /// 🧾️ `assert_declared_actions_bridge_to_commands`/`new_app_with_registry` still take a `fn() ->
    /// App` manifest (framework testkit gap, not this packet's to fix — see the sibling `w2-cad-report`
    /// "SDK gaps" §3); `create_puzzle2d_app` now returns `AppDefinition`, so this wraps it.
    fn puzzle2d_manifest_for_testkit() -> App {
        App { definition: create_puzzle2d_app(), examples: Vec::new() }
    }

    /// 🧰️ A registry-backed app so kind discipline (View/Shell actions must emit no operations) and the
    /// utility contract are enforced exactly as in production.
    pub fn app_with_registry() -> Puzzle2dApp {
        semio_framework::io::resolve_ready(semio_framework_plugin::testkit::new_app_with_registry::<EditorApp<Puzzle2dPlayApp>>(puzzle2d_manifest_for_testkit))
    }

    /// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
    /// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
    /// `Self::Command` channel). Reconstructs the `Puzzle2dCommand` from the same
    /// `(action, args, window_id)` triple every pre-B1 test already passed.
    pub fn dispatch(app: &mut Puzzle2dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
        // 🕰️ Framework-reserved verbs (undo/redo/checkpoint/…/the six interaction verbs) stay on
        // `handle_action` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM added
        // interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
        // setInteractionGranularity to this reserved set.
        if matches!(
            action,
            "undo"
                | "redo"
                | "commitCheckpoint"
                | "createAlternative"
                | "switchAlternative"
                | "checkoutCheckpoint"
                | "copy"
                | "cut"
                | "paste"
                | "revertToCommand"
                | "historyFilter"
                | "noteShellCommand"
                | "interactionSelect"
                | "interactionHover"
                | "clearSelection"
                | "selectAll"
                | "setSelectionMode"
                | "setInteractionGranularity"
        ) {
            return semio_framework::io::resolve_ready(app.handle_action(action, args, &meta("local")));
        }
        semio_framework::io::resolve_ready(app.dispatch_typed(Puzzle2dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), &meta("local")))
    }

    /// 🧵️ Drives the same host-owned `DispatchAction` continuation used in production until the example is complete.
    pub fn finish_example_load(app: &mut Puzzle2dApp, mut result: InvocationResult) -> usize {
        for step in 0..4_096 {
            let next = result.requested_effects.into_iter().find_map(|effect| match effect {
                Effect::DispatchAction { action, args, .. } if action == set_active_example::STEP_ACTION_ID => Some(args.map(|value| semio_framework::from_dsl_value::<Value>(value).expect("example step args decode"))),
                _ => None,
            });
            let Some(args) = next else { return step };
            result = dispatch(app, set_active_example::STEP_ACTION_ID, args.as_ref(), None).expect("advance example load");
            assert!(result.mutations.len() <= set_active_example::MAX_MUTATIONS_PER_STEP, "one resumable example step exceeded its fixed semantic mutation bound");
        }
        panic!("example load did not finish within its deterministic step bound");
    }

    pub fn load_example(app: &mut Puzzle2dApp, example_id: &str) -> usize {
        let result = dispatch(app, "setActiveExample", Some(&json!({ "exampleId": example_id })), None).expect("begin example load");
        finish_example_load(app, result)
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionSelect`
    /// for one `(granularity, id)` pair in the `vortex` domain — the test-side replacement for the
    /// deleted `setSelection` action.
    pub fn select_id(app: &mut Puzzle2dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
        let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
        dispatch(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE2D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None)
    }

    pub fn concrete_forest_app() -> Puzzle2dApp {
        let mut app = app();
        load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
        app
    }

    /// 🖼️ The rendered body, serialized — every panel/window assertion greps this string.
    pub fn render_body(app: &mut Puzzle2dApp, body_key: &str) -> String {
        let tree = semio_framework::io::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render");
        let mut stack = vec![&tree.root];
        while let Some(node) = stack.pop() {
            if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
                if surface.doc_schema.as_str() == <semio_framework_ui_scene::Board2dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA {
                    let scene: semio_framework_ui_scene::Board2dScene = semio_framework_ui_scene::decode(surface).expect("decode board scene");
                    return serde_json::to_string(&json!({ "schema": surface.doc_schema, "board2d": scene })).expect("serialize board scene");
                }
            }
            stack.extend(node.children.iter());
        }
        serde_json::to_string(&tree.root).expect("serialize rendered node")
    }

    /// 🧾️ A standalone `Puzzle2dScene` for the measure/engagement builders that take one directly.
    pub fn scene(fixture: Value, runtime: Puzzle2dPlayRuntime, active_utility: &str) -> Puzzle2dScene {
        Puzzle2dScene { fixture, runtime, active_utility: active_utility.into() }
    }

    pub fn fixture_of(app: &Puzzle2dApp) -> Value {
        app.snapshot().expect("projection").0
    }

    pub fn first_node_id(app: &Puzzle2dApp) -> String {
        fixture_nodes(&fixture_of(app))[0].get("id").and_then(|value| value.as_str()).expect("node id").to_string()
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::testkit::*;
    use super::*;
    use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
    use semio_framework_plugin::PluginApp;
    use store::{Backbone, BackboneMessage, MemoryBackbone};

    fn cohort_routes_are_cursorized(source: &str) -> bool {
        [
            r#""forceLayout" => Box::new(Puzzle2dForceLayoutWork::default())"#,
            r#""setActiveExample" => Box::new(Puzzle2dActiveExampleWork::default())"#,
            "Puzzle2dForceStage::Nodes",
            "Puzzle2dForceStage::Handles",
            "Puzzle2dForceStage::Edges",
            "Puzzle2dForceStage::Repel",
            "Puzzle2dForceStage::Springs",
            "Puzzle2dForceStage::Integrate",
            "Puzzle2dForceStage::Emit",
            "Puzzle2dExampleStage::ClearEdges",
            "Puzzle2dExampleStage::ClearNodes",
            "Puzzle2dExampleStage::AddCompatibility",
            "Puzzle2dExampleStage::Nodes",
            "Puzzle2dExampleStage::Edges",
            r#"matches!(command.action_id(), "addNode").then_some(1)"#,
        ]
        .into_iter()
        .all(|marker| source.contains(marker))
            && !source.contains(r#""forceLayout" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
            && !source.contains(r#""setActiveExample" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
    }

    #[test]
    fn cohort_hostile_static_law_rejects_one_grant_complex_routes_and_missing_cursors() {
        let source = include_str!("🦀️component.rs");
        assert!(cohort_routes_are_cursorized(source));
        for (retained, direct) in [
            (r#""forceLayout" => Box::new(Puzzle2dForceLayoutWork::default())"#, r#""forceLayout" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle2d_retained_reduce, puzzle2d_retained_extent))"#),
            (r#""setActiveExample" => Box::new(Puzzle2dActiveExampleWork::default())"#, r#""setActiveExample" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle2d_retained_reduce, puzzle2d_retained_extent))"#),
        ] {
            assert!(!cohort_routes_are_cursorized(&source.replace(retained, direct)));
        }
        for marker in [
            "Puzzle2dForceStage::Nodes",
            "Puzzle2dForceStage::Handles",
            "Puzzle2dForceStage::Edges",
            "Puzzle2dForceStage::Repel",
            "Puzzle2dForceStage::Springs",
            "Puzzle2dForceStage::Integrate",
            "Puzzle2dForceStage::Emit",
            "Puzzle2dExampleStage::ClearEdges",
            "Puzzle2dExampleStage::ClearNodes",
            "Puzzle2dExampleStage::AddCompatibility",
            "Puzzle2dExampleStage::Nodes",
            "Puzzle2dExampleStage::Edges",
        ] {
            assert!(!cohort_routes_are_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing retained cursor was falsely accepted: {marker}");
        }
    }

    fn mounted_fill_dispatch_contract(source: &str) -> bool {
        let production = source.split("//#region 🧪️Tests").next().unwrap_or(source);
        let Some(handle) = production.find("async fn handle(") else { return false };
        let Some(fill_relative) = production[handle..].find("if is_fill_session_action(action) {") else { return false };
        let fill = handle + fill_relative;
        let Some(normal_relative) = production[fill..].find("let before = doc.snapshot.0.clone();") else { return false };
        let normal = fill + normal_relative;
        let prefix = &production[handle..fill];
        let branch = &production[fill..normal];
        production.contains("fn mounted_job_prepare_snapshot_read")
            && production.contains("async fn pending_effects")
            && branch.contains("dispatch_fill_session_action(action, args, ctx)")
            && branch.contains("set_fill_count::Puzzle2dFillActionCtx")
            && branch.contains("Puzzle2dFillRuntime::from_config(config)")
            && branch.contains("Puzzle2dConfigMutation::Fill { runtime }")
            && branch.contains("boundary_fault")
            && branch.contains("artifact_mutations")
            && !prefix.contains("doc.snapshot.0.clone()")
            && !branch.contains("doc.snapshot.0.clone()")
            && !branch.contains("config.clone()")
            && !branch.contains("Puzzle2dConfigMutation::Snapshot")
            && !branch.contains("Vec<Value>")
            && !branch.contains("BTreeMap")
            && !branch.contains("BoardHost")
            && !branch.contains("RefCell")
            && !branch.contains("sync_host_fixture_content")
            && !branch.contains("puzzle2d_document_delta_operations")
    }

    /// 🧱️ The mounted fill branch excludes whole ArtifactView and dynamic config snapshot work.
    #[test]
    fn mounted_fill_dispatch_whole_artifact_and_config_mutations_are_rejected() {
        let source = include_str!("🦀️component.rs");
        assert!(mounted_fill_dispatch_contract(source));
        let cloned = source.replacen("if is_fill_session_action(action) {", "if is_fill_session_action(action) { let _whole = doc.snapshot.0.clone();", 1);
        assert!(!mounted_fill_dispatch_contract(&cloned));
        let rebuilt = source.replacen("dispatch_fill_session_action(action, args, ctx);", "sync_host_fixture_content(&mut host.borrow_mut(), &scene); dispatch_fill_session_action(action, args, ctx);", 1);
        assert!(!mounted_fill_dispatch_contract(&rebuilt));
        let diffed = source.replacen("dispatch_fill_session_action(action, args, ctx);", "let _whole = puzzle2d_document_delta_operations(&doc.snapshot.0, &scene.fixture); dispatch_fill_session_action(action, args, ctx);", 1);
        assert!(!mounted_fill_dispatch_contract(&diffed));
        let default_host = source.replacen(
            "let mut runtime = crate::editor::puzzle2d::config::Puzzle2dFillRuntime::from_config(config);",
            "let host = RefCell::new(BoardHost::default()); let mut runtime = crate::editor::puzzle2d::config::Puzzle2dFillRuntime::from_config(config);",
            1,
        );
        assert!(!mounted_fill_dispatch_contract(&default_host));
        let config_clone = source.replacen("if is_fill_session_action(action) {", "if is_fill_session_action(action) { let _whole_config = config.clone();", 1);
        assert!(!mounted_fill_dispatch_contract(&config_clone));
        let snapshot = source.replacen("Puzzle2dConfigMutation::Fill { runtime }", "Puzzle2dConfigMutation::Snapshot { config: config.clone() }", 1);
        assert!(!mounted_fill_dispatch_contract(&snapshot));
    }

    /// 🎥️ Recovers the rendered pane camera `(x, y, zoom)` from a rendered `UiNode`'s embedded
    /// `Board2dScene.cameraJson` — the only externally observable surface for the runtime camera
    /// (the camera is never a document field, so it cannot be read back off `app.snapshot()`).
    fn rendered_camera(rendered: &str) -> (f64, f64, f64) {
        fn find_camera_json(value: &Value) -> Option<String> {
            if let Some(json) = value.get("cameraJson").and_then(Value::as_str) {
                return Some(json.to_string());
            }
            match value {
                Value::Object(map) => map.values().find_map(find_camera_json),
                Value::Array(items) => items.iter().find_map(find_camera_json),
                _ => None,
            }
        }
        let value: Value = serde_json::from_str(rendered).expect("rendered node parses");
        let camera_json = find_camera_json(&value).expect("rendered scene must carry cameraJson");
        let camera: Value = serde_json::from_str(&camera_json).expect("cameraJson parses");
        (camera.get("x").and_then(Value::as_f64).unwrap_or(f64::NAN), camera.get("y").and_then(Value::as_f64).unwrap_or(f64::NAN), camera.get("zoom").and_then(Value::as_f64).unwrap_or(f64::NAN))
    }

    //#region 🔖️Operations
    #[semio_framework_async_macros::async_test]
    async fn add_node_action_emits_upsert_op_and_appends_node() {
        let mut app = app();
        let result = dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
        assert_eq!(result.mutations.len(), 1, "addNode must emit exactly one granular operation");
        assert_eq!(fixture_nodes(&fixture_of(&app)).len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_example_loads_concrete_forest_via_operations() {
        let mut app = app();
        let begin = dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None).expect("begin example load");
        assert!(begin.mutations.is_empty(), "the initiating interaction step must only enqueue resumable work");
        assert!(begin.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == set_active_example::STEP_ACTION_ID)));
        finish_example_load(&mut app, begin);
        assert!(!fixture_nodes(&fixture_of(&app)).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn newer_example_load_supersedes_a_stale_continuation() {
        let mut app = app();
        let stale_begin = dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None).expect("begin stale load");
        let stale_args = stale_begin.requested_effects.into_iter().find_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == set_active_example::STEP_ACTION_ID => args.map(|value| semio_framework::from_dsl_value::<Value>(value).expect("stale args decode")),
            _ => None,
        });
        let active_begin = dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID })), None).expect("begin active load");
        let stale = dispatch(&mut app, set_active_example::STEP_ACTION_ID, stale_args.as_ref(), None).expect("stale step is a no-op");
        assert!(stale.mutations.is_empty());
        assert!(stale.requested_effects.is_empty());
        finish_example_load(&mut app, active_begin);
        assert!(!fixture_edges(&fixture_of(&app)).is_empty());
    }

    /// 📦️ `Puzzle2dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
    /// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
    /// `serde_json::Value` bridge impls).
    #[semio_framework_async_macros::async_test]
    async fn puzzle2d_play_projection_pack_round_trips() {
        let app = concrete_forest_app();
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&app.snapshot().expect("projection"));
    }

    #[semio_framework_async_macros::async_test]
    async fn select_then_delete_selection_removes_the_node() {
        let mut app = app_with_registry();
        dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
        let node_id = first_node_id(&app);
        select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &node_id).expect("select");
        dispatch(&mut app, "deleteSelection", None, None).expect("delete");
        assert!(fixture_nodes(&fixture_of(&app)).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = app();
        dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add");
        assert_eq!(fixture_nodes(&fixture_of(&app)).len(), 1);
        dispatch(&mut app, "undo", None, None).expect("undo");
        assert_eq!(fixture_nodes(&fixture_of(&app)).len(), 0);
        dispatch(&mut app, "redo", None, None).expect("redo");
        assert_eq!(fixture_nodes(&fixture_of(&app)).len(), 1);
    }
    //#endregion 🔖️Operations

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`). Deliberately
    /// dispatches through a standalone typed `Puzzle2dStore` — NOT through `Puzzle2dPlayApp`/
    /// `Puzzle2dPlaySnapshot` (the `🔖️ValueBridge` `serde_json::Value` wrapper this app still uses)
    /// — since `Puzzle2dMutation`'s canonical `Mutation<Puzzle2dSnapshot>` impl (not its
    /// `Mutation<Value>` bridge impl) is what the CW7 law is about.
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::puzzle2d::spr::Puzzle2dStore;
        use crate::artifacts::puzzle2d::{Puzzle2dNode, PUZZLE_2D_SCHEMA};
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand};

        let mut store = Puzzle2dStore::new(create_document_envelope(PUZZLE_2D_SCHEMA, "puzzle2d", Puzzle2dSnapshot::default(), None)).await.expect("store");
        let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
        store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::artifacts::puzzle2d::mutations::create_node(node, None)], description: None }).await.expect("apply");
        let envelope = store.envelope();
        let edit: &Edit<Puzzle2dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
        semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle2dSnapshot, Puzzle2dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())).await;
    }
    //#endregion 🔖️CommandEnvelopeTests

    //#region 🔖️BoardEvents
    /// 🎥️ `setCamera` is session-only view state: a camera drag never creates a VCS edit, so there is
    /// nothing to coalesce and nothing for `undo` to revert.
    #[semio_framework_async_macros::async_test]
    async fn set_camera_is_session_only_and_never_undoable() {
        let mut app = app();
        for x in [1.0, 2.0, 3.0] {
            let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "x": x, "y": 0.0, "zoom": 1.0 } })), None).expect("camera");
            assert!(result.mutations.is_empty(), "setCamera must never produce a document operation");
        }
        let rendered = render_body(&mut app, overview::BODY_KEY);
        assert_eq!(rendered_camera(&rendered).0, 3.0, "the camera must update immediately in the rendered scene");
        let undo = dispatch(&mut app, "undo", None, None).expect("undo");
        assert!(undo.mutations.is_empty(), "there is no document edit to undo");
        let rendered_after_undo = render_body(&mut app, overview::BODY_KEY);
        assert_eq!(rendered_camera(&rendered_after_undo).0, 3.0, "the camera is session state — undo must not revert it");
    }

    /// 🐢️ Regression test for a perf-round-2 bug: `parse_fixture_v1` always `clear_scene()`s then
    /// rebuilds, so every edge looked "new" and got re-`push_event`'d as `edgeCreate` — which
    /// `apply_host_events` then replayed into the fixture on the *next* action, duplicating every edge
    /// once per action forever.
    #[semio_framework_async_macros::async_test]
    async fn repeated_actions_do_not_duplicate_edges() {
        let mut app = app();
        load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
        let edge_count = |app: &Puzzle2dApp| fixture_edges(&fixture_of(app)).len();
        let before = edge_count(&app);
        assert!(before > 0, "fixture must have edges for this regression test to be meaningful");
        let node_id = first_node_id(&app);
        for _ in 0..5 {
            dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None).expect("select");
        }
        assert_eq!(edge_count(&app), before, "selecting repeatedly must not grow the edges array");
    }

    /// 🪞️ Regression test: `applyBoardEvents`'s `select` case only mutated the runtime, never the
    /// host, so `apply_host_events`'s `host.selection`-is-truth re-sync silently reverted the
    /// selection to whatever the host held before the action (empty, on a fresh sync).
    #[semio_framework_async_macros::async_test]
    async fn apply_board_events_select_persists_across_the_next_action() {
        let mut app = concrete_forest_app();
        let node_id = first_node_id(&app);
        dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None).expect("select");
        assert!(render_body(&mut app, overview::BODY_KEY).contains(&node_id), "selection must be visible immediately after the select action");
        // A second, unrelated action used to silently clear the selection via the stale `host.selection` re-sync.
        dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": "[]" })), None).expect("no-operation");
        assert!(render_body(&mut app, overview::BODY_KEY).contains(&node_id), "selection must survive a subsequent unrelated action");
    }

    /// 🪞️ Regression test: `apply_host_events` used to epsilon-compare `host.camera` (still the
    /// *pre-action* value) against the runtime and blindly overwrite it, reverting a plain `camera`
    /// board event (used for the live wheel-zoom echo) before it ever committed.
    #[semio_framework_async_macros::async_test]
    async fn apply_board_events_camera_event_commits() {
        let mut app = app();
        let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "camera", "payload": { "x": 5.0, "y": 6.0, "zoom": 1.2 } }]).to_string() })), None).expect("camera event");
        assert!(result.mutations.is_empty(), "a camera board event must never produce a document operation");
        let (x, y, zoom) = rendered_camera(&render_body(&mut app, overview::BODY_KEY));
        assert_eq!(x, 5.0);
        assert_eq!(y, 6.0);
        assert_eq!(zoom, 1.2);
    }

    /// 🐢️ A pure selection change is runtime state, not document state — it must not produce any
    /// operations (previously it fell back to a whole-document replace once the edge-duplication bug
    /// made `before` and `after` genuinely diverge).
    #[semio_framework_async_macros::async_test]
    async fn select_action_emits_no_operations() {
        let mut app = concrete_forest_app();
        let node_id = first_node_id(&app);
        let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None).expect("select");
        assert!(result.mutations.is_empty(), "selection must not produce document operations");
    }
    //#endregion 🔖️BoardEvents

    //#region 🔖️UiScope
    /// 🐢️ Perf round 3: a select event must declare a narrow `Partial` ui_scope (the 3 canvas panes +
    /// layers/properties panels + engagements) — never `Full`, or the shell's batched `refresh-ui`
    /// call degrades back to fetching everything on every select.
    #[semio_framework_async_macros::async_test]
    async fn select_action_declares_partial_ui_scope() {
        let mut app = concrete_forest_app();
        let node_id = first_node_id(&app);
        let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None).expect("select");
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
                // 🐢️ Regression: `window_bodies` must list the window *body keys* (matched against
                // `AppDefinition.windowKinds[].bodyKey` by the shell's `buildUiRefreshRequest`), not
                // the pane/kind-id constants (`PUZZLE2D_PANES`) — those are a different id space.
                assert_eq!(window_bodies, vec![overview::BODY_KEY, detail::BODY_KEY, selection::BODY_KEY], "window_bodies must be body keys, not pane ids");
                assert!(panel_bodies.contains(&document::PUZZLE2D_PLAY_BODY_LAYERS.to_string()));
                assert!(panel_bodies.contains(&inspection::PUZZLE2D_PLAY_BODY_PROPERTIES.to_string()));
                assert!(engagements, "select must refresh the engagement bar");
                assert!(!measures, "select must not force a measures refresh");
                assert!(!utilities);
                assert!(!tools);
                assert!(!labels);
            }
            other => panic!("expected a Partial ui_scope for select, got {other:?}"),
        }
    }

    /// 🐢️ Perf round 3: a camera-only board event touches only the 3 canvas panes — no panels,
    /// engagements, measures, or utilities.
    #[semio_framework_async_macros::async_test]
    async fn camera_event_declares_window_only_ui_scope() {
        let mut app = app();
        let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "camera", "payload": { "x": 1.0, "y": 2.0, "zoom": 1.0 } }]).to_string() })), None).expect("camera event");
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
                assert_eq!(window_bodies.len(), 3);
                assert!(panel_bodies.is_empty(), "a config-only camera event does not dirty command history");
                assert!(!engagements && !measures && !utilities && !tools && !labels);
            }
            other => panic!("expected a Partial ui_scope for a camera event, got {other:?}"),
        }
    }

    /// 🐢️ Perf round 3: an empty `applyBoardEvents` batch (no-operation) must declare nothing beyond the
    /// history panel body — the empty View action neither logs an edit nor dirties a surface.
    #[semio_framework_async_macros::async_test]
    async fn empty_board_events_declare_none_ui_scope() {
        let mut app = app();
        let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": "[]" })), None).expect("no-operation");
        assert_eq!(result.ui_scope, UiDirtyScope::None);
    }

    /// 🐢️ Perf round 3: cold-tier structural actions (document operations) must keep the safe `Full`
    /// default — no puzzle2d scope helper narrows them.
    #[semio_framework_async_macros::async_test]
    async fn add_node_action_declares_full_ui_scope() {
        let mut app = app();
        let result = dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
        assert!(matches!(result.ui_scope, UiDirtyScope::Full), "addNode must stay Full, got {:?}", result.ui_scope);
    }
    //#endregion 🔖️UiScope

    //#region 🔖️Manifest
    #[test]
    fn app_definition_has_three_lod_pane_window_kinds() {
        let definition = create_puzzle2d_app();
        let ids: Vec<&str> = definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
        assert_eq!(ids, vec![overview::WINDOW_KIND_ID, detail::WINDOW_KIND_ID, selection::WINDOW_KIND_ID]);
        for window in &definition.window_kinds {
            assert!(window.options.engagement.as_option().is_some(), "pane {} must have engagement", window.id);
            assert!(!window.options.measures.is_empty(), "pane {} must have LOD/suggestion measures", window.id);
        }
    }

    /// 🧰️ The app declares exactly the select/brush canvas utilities and binds them to the interactive
    /// overview pane; fill is declared as a mode-level tool instead.
    #[test]
    fn utility_registry_declares_utilities() {
        let definition = create_puzzle2d_app();
        let ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(ids, vec![select_utility::UTILITY_ID, brush_utility::UTILITY_ID]);
        let overview_window = definition.window_kinds.iter().find(|window| window.id == overview::WINDOW_KIND_ID).expect("overview pane");
        let overview_utilities: Vec<&str> = overview_window.utilities.iter().map(|utility| utility.as_str()).collect();
        assert_eq!(overview_utilities, vec![select_utility::UTILITY_ID, brush_utility::UTILITY_ID]);
        assert!(overview_window.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID), "declaring utilities must inject the setActiveUtility action");
        // 🧰️ D-1: select/brush are this window's whole exclusive utility set, NOT a sub-collection, so
        // each carries `group: None` and renders as a flat utility bar icon (never one collapsed dropdown).
        for utility in &definition.utilities {
            assert_eq!(utility.group, None, "utility {} must render flat (no shared group)", utility.id);
        }
    }

    /// 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
    #[test]
    fn tool_registry_declares_fill_tool() {
        use semio_framework_plugin::{ToolRef, SET_ACTIVE_TOOL_ACTION_ID};
        let definition = create_puzzle2d_app();
        let tool_ids: Vec<&str> = definition.tools.iter().map(|tool| tool.id.as_str()).collect();
        assert_eq!(tool_ids, vec![fill::TOOL_ID]);
        assert_eq!(definition.modes[0].tools, vec![semio_framework::io::resolve_ready(ToolRef::new(fill::TOOL_ID))]);
        assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID), "declaring tools must inject the setActiveTool action");
    }

    /// 🎥️ The camera is session-only runtime state, never a document field — a DWG import (which has
    /// no live app instance to receive a runtime write) must produce a bare empty board with no
    /// `"camera"` key at all, regardless of the drawing's extents.
    #[test]
    fn dwg_import_returns_empty_board_with_no_camera_field() {
        let drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing { extmin: [0.0, 0.0, 0.0], extmax: [100.0, 200.0, 0.0], ..semio_s_plugin_stdio::artifacts::dwg::DwgDrawing::default() };
        let fixture = puzzle2d_document_json_from_dwg(&drawing).unwrap();
        assert_eq!(fixture.get("schema").and_then(|value| value.as_str()), Some(PUZZLE2D_FIXTURE_SCHEMA));
        assert!(fixture_nodes(&fixture).is_empty());
        assert!(fixture.get("camera").is_none(), "the document must never carry a camera field");
    }
    //#endregion 🔖️Manifest

    //#region 🔖️Convergence
    /// 🧪️ Definitional convergence proof: two instances on one backbone make DISJOINT node edits
    /// (each adds its own node) and, after exchanging operations, both converge to contain BOTH nodes —
    /// impossible under whole-document `setSnapshot` snapshots, which would clobber one side.
    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_disjoint_node_edits_via_backbone() {
        let mut instance_a = app();
        let mut instance_b = app();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://puzzle2d-convergence", "mem://puzzle2d-convergence").await;
        instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
        instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");

        dispatch(&mut instance_a, "addNode", Some(&json!({ "kind": "seed" })), None).expect("a adds node");
        dispatch(&mut instance_b, "addNode", Some(&json!({ "kind": "other" })), None).expect("b adds node");

        // A neutral history action always calls store.dispatch(), which pumps inbound operations first.
        dispatch(&mut instance_a, "commitCheckpoint", None, None).expect("pump a");
        dispatch(&mut instance_b, "commitCheckpoint", None, None).expect("pump b");

        assert_eq!(fixture_nodes(&fixture_of(&instance_a)).len(), 2, "instance A must contain both nodes");
        assert_eq!(fixture_nodes(&fixture_of(&instance_b)).len(), 2, "instance B must contain both nodes");
    }

    #[semio_framework_async_macros::async_test]
    async fn ingest_operations_is_idempotent() {
        let mut sender = app();
        let (near, mut far) = MemoryBackbone::pair("mem://puzzle2d-doc", "mem://puzzle2d-doc").await;
        sender.attach_backbone(store::Backbones::Memory(near)).await.expect("attach");
        dispatch(&mut sender, "addNode", Some(&json!({ "kind": "seed" })), None).expect("add");

        let mut envelopes = Vec::new();
        for message in far.receive().await.expect("receive") {
            if let BackboneMessage::Mutations { envelopes: operations } = message {
                envelopes.extend(operations);
            }
        }
        assert!(!envelopes.is_empty(), "the applied operation must flow onto the channel");
        let operations = envelopes;

        let mut receiver = app();
        receiver.ingest_operations(&operations).await.expect("ingest once");
        receiver.ingest_operations(&operations).await.expect("ingest twice");
        assert_eq!(fixture_nodes(&fixture_of(&receiver)).len(), 1, "feeding the same operation twice must not double-apply");
    }
    //#endregion 🔖️Convergence

    //#region 🔖️Registry
    /// 🧰️ B1: `setActiveUtility` is a real typed `Puzzle2dCommand` now (was a host-applied `ViewModel`
    /// notification): switching utilities must still emit no DOCUMENT operations — the new value lands
    /// in `Puzzle2dConfig::active_utility_by_window_id` as a config operation instead.
    #[semio_framework_async_macros::async_test]
    async fn utility_switch_emits_no_ops_and_no_history() {
        let mut app = app_with_registry();
        let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": brush_utility::UTILITY_ID })), Some(overview::WINDOW_KIND_ID)).expect("switch utility");
        assert!(result.mutations.is_empty(), "a utility switch must not produce document operations");
        let can_undo = dispatch(&mut app, "undo", None, None);
        assert!(can_undo.map_or(true, |r| r.mutations.is_empty()), "a utility switch must not have created a document undo step");
    }

    /// 🧭️ Kind discipline: every View-declared runtime/host action must run through the registry
    /// without tripping the "must not emit operations" guard (proving each is correctly classified).
    #[semio_framework_async_macros::async_test]
    async fn view_actions_emit_no_ops_through_the_registry() {
        let mut app = app_with_registry();
        load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
        let node_id = first_node_id(&app);
        select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &node_id).expect("select");
        let view_dispatches: Vec<(&str, Value)> = vec![
            ("setCamera", json!({ "camera": { "x": 7.0, "y": 8.0, "zoom": 1.5 } })),
            ("selectSameKind", Value::Null),
            ("setGridSnapEnabled", json!({ "enabled": true })),
            ("setGridFactor", json!({ "value": 2.0 })),
            ("setLodModeForPane", json!({ "pane": overview::WINDOW_KIND_ID, "value": "detail" })),
            ("setBrushKindWeights", json!({ "kindId": "node", "value": 0.5 })),
            ("setBrushNodeSize", json!({ "size": 12.0 })),
            ("setSuggestionOffset", json!({ "value": 40.0 })),
            ("engagementInput", json!({ "pane": overview::WINDOW_KIND_ID, "value": "brush" })),
            ("engagementSubmit", json!({ "pane": overview::WINDOW_KIND_ID, "value": "brush" })),
            ("engagementAbort", json!({ "pane": overview::WINDOW_KIND_ID })),
            ("brushCycleCandidate", json!({ "forward": true })),
            ("brushSetCandidateIndex", json!({ "index": 0 })),
            ("brushFillSessionBegin", json!({ "maxCount": 4, "seed": 1 })),
            ("brushFillSessionClear", Value::Null),
            ("lodScaleJson", Value::Null),
        ];
        for (action, args) in view_dispatches {
            let args_ref = (!args.is_null()).then_some(&args);
            let result = dispatch(&mut app, action, args_ref, None).unwrap_or_else(|error| panic!("view action '{action}' must not error: {error:?}"));
            assert!(result.mutations.is_empty(), "view action '{action}' must not emit document operations");
        }
    }

    /// 🗂️ Grouped-context-menu disclosure: the top-level row budget stays small (leaves+groups
    /// combined) and the known `deleteSelection` destructive row stays last.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
        use semio_framework_plugin::{ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};

        let mut app = app_with_registry();
        load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
        let node_id = first_node_id(&app);
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `context_menu` reads the
        // CLIENT-supplied `request.surface.selection` now (selection is framework-owned, no live
        // config field to derive it from) — see `context_menu`'s own doc comment.
        let request = ContextMenuRequest {
            menu: UiMenuRef { id: "puzzle2d".into(), args: None },
            surface: Some(ContextMenuSurfaceTarget { surface_id: "puzzle2d".into(), kind: "board".into(), hits: Vec::new(), selection: vec![ContextMenuSelectionGroup { domain: PUZZLE2D_GRANULARITY_NODE.into(), ids: vec![node_id] }], text: None }),
            window_instance_id: None,
            point: None,
        };
        let menu = semio_framework::io::resolve_ready(app.context_menu(&request));
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("grouped disclosure menu should not be empty");
        assert_eq!(last.id, "deleteSelection", "the destructive row must stay last as a top-level leaf");
        assert_eq!(last.destructive, Some(true), "the destructive row must carry destructive: true");
    }
    //#endregion 🔖️Registry
}
//#endregion 🧪️Tests
