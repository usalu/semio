//! 🧩️ Puzzle 2d play app — the plugin's 2d play app: its `ArtifactApp` impl (dispatch-only), the
//! transient `Puzzle2dScene` bundle its command/panel/window nodes mutate and render, the shared
//! fixture helpers they build on, and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️component.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/<window>`. This file dispatches and stitches.
//!
//! 🌉️ `ArtifactApp::Snapshot` is the `Puzzle2dPlaySnapshot` newtype over a bare
//! `serde_json::Value` fixture (see `crate::artifacts::puzzle2d::op`'s `🔖️ValueBridge`), not the typed
//! `Puzzle2dSnapshot` — every helper below therefore works on `Value`, and each action emits the
//! granular typed operation delta (`puzzle2d_document_delta_operations`) turning the old fixture into
//! the new one.

use crate::apps::puzzle2d::presence::{Puzzle2dPresence, Puzzle2dPresenceMutation};
use crate::apps::puzzle2d::commands::{board, brush, camera, engagement, example, grid, locale, lod, node, selection as selection_commands, utility};
use crate::apps::puzzle2d::config::{Puzzle2dConfig, Puzzle2dConfigMutation, Puzzle2dPlayRuntime};
use crate::apps::puzzle2d::modes::edit;
use crate::apps::puzzle2d::modes::edit::tools::fill;
use crate::apps::puzzle2d::modes::edit::windows::overview::utilities::{brush as brush_utility, select as select_utility};
use crate::apps::puzzle2d::modes::edit::windows::{detail, overview, selection};
use crate::apps::puzzle2d::panels::{catalogue, document, inspection};
use crate::apps::puzzle2d::terminology::{is_de_locale, puzzle2d_labels, Puzzle2dLabels};
use crate::artifacts::puzzle2d::dsl as puzzle2d_dsl;
use crate::artifacts::puzzle2d::engine::board_host::puzzle_board_host;
use crate::artifacts::puzzle2d::engine::{BoardHost, Puzzle2dExtension};
use crate::artifacts::puzzle2d::op::{puzzle2d_document_delta_operations, Puzzle2dMutation, Puzzle2dPlaySnapshot};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::HostEffect;
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, 
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppIo, AppLabels, ArtifactPresentation, ConfigView, ArtifactApp, ArtifactView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm,
    MediaPortDirection, MediaPortSpec, MediaType, PortMultiplicity, UiNode, WindowEngagement, WindowMeasure, SET_ACTIVE_UTILITY_ACTION_ID,
};
use store::EngineHandles;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};

//#region 🔖️Constants
pub const PUZZLE2D_PLAY_APP_ID: &str = "puzzle2d-play";
pub const PUZZLE2D_PLAY_CONTROLLER_ID: &str = "puzzle2d-play";
pub const PUZZLE2D_PLAY_SURFACE_ID: &str = "puzzle2d.play.composite";
pub const PUZZLE2D_FIXTURE_SCHEMA: &str = "puzzle.2d.fixture";
pub const PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID: &str = crate::examples::puzzle2d::concrete_forest::ID;
pub const PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID: &str = crate::examples::puzzle2d::nakagin_capsule_tower::ID;

/// 🪟️ The three canvas pane KIND ids — a different id space from the window body keys (see
/// `🎮️commands/🎲️board`'s `PUZZLE2D_WINDOW_BODY_KEYS`): these key utilities, engagements and measures.
pub const PUZZLE2D_PANES: [&str; 3] = [overview::WINDOW_KIND_ID, detail::WINDOW_KIND_ID, selection::WINDOW_KIND_ID];
pub const PUZZLE2D_LOD_MODE_AUTOMATIC: &str = "automatic";

const BOARD_DEFAULT_WIDTH: u32 = 1024;
const BOARD_DEFAULT_HEIGHT: u32 = 768;


/// 🌉️ This app's own fixture (and `ArtifactApp::Snapshot`) stays a bare `serde_json::Value`, so the
/// DSL-text example fixtures are parsed once into the typed `Puzzle2dSnapshot` and re-serialized to
/// the JSON string this module's `serde_json::from_str`/`.example(...)` call sites expect. The typed
/// bridge still carries a mandatory `camera` block — strip it before handing the JSON back, since the
/// play app's own document must never carry a `"camera"` key (see `setCamera`'s `ActionKind::View`):
/// leaving it in would permanently trip `puzzle2d_document_delta_operations`'s known-keys guard on
/// every subsequent action.
fn parse_example_dsl_without_camera(dsl_text: &str, label: &str) -> String {
    let projection = <Puzzle2dSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).unwrap_or_else(|error| panic!("{label} example fixture parses as dsl: {error}"));
    let mut value = serde_json::to_value(&projection).unwrap_or_else(|error| panic!("serialize {label} example fixture: {error}"));
    if let Some(object) = value.as_object_mut() {
        object.remove("camera");
    }
    serde_json::to_string(&value).unwrap_or_else(|error| panic!("re-serialize {label} example fixture: {error}"))
}

pub fn concrete_forest_example_json() -> String { parse_example_dsl_without_camera(crate::examples::puzzle2d::concrete_forest::DSL_TEXT, "concrete-forest") }
pub fn nakagin_example_json() -> String { parse_example_dsl_without_camera(crate::examples::puzzle2d::nakagin_capsule_tower::DSL_TEXT, "nakagin") }
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

fn example_fixture(json_text: &str) -> Value {
    serde_json::from_str(json_text).unwrap_or_else(|_| default_empty_fixture())
}

pub fn puzzle2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(PUZZLE2D_PLAY_CONTROLLER_ID).action(action, args)
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
/// `🎮️commands/🧰️utility`, the only writer.
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
    if let Some(catalogs) = envelope.fixture.get("meta").and_then(|value| value.get("kindCatalogs")) {
        if let Ok(json) = serde_json::to_string(catalogs) {
            let _ = host.set_board_kind_catalogs_from_json(&json);
        }
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
fn sync_host_runtime_state(host: &mut BoardHost, envelope: &Puzzle2dScene) {
    host.set_size(BOARD_DEFAULT_WIDTH, BOARD_DEFAULT_HEIGHT, 1.0);
    host.set_selection_ids(&envelope.runtime.selected_ids);
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
    host.set_selection_options(&envelope.runtime.selection_method, "replace", true, true, true);
}

fn sync_host_from_envelope(host: &mut BoardHost, envelope: &Puzzle2dScene) {
    sync_host_fixture_content(host, envelope);
    sync_host_runtime_state(host, envelope);
}

/// 🪞️ Re-syncs `envelope.runtime.selected_ids` from `host` for engine-driven selection changes (e.g.
/// `delete_selection`, brush commit). Camera is deliberately NOT mirrored here: every action that
/// moves the camera already writes the config's camera fields directly — re-deriving it from
/// `host.camera` here used to blindly overwrite that write with the *pre-action* host camera.
pub fn apply_host_events(host: &mut BoardHost, envelope: &mut Puzzle2dScene) {
    let events_raw = host.drain_events_json();
    board::apply_board_events_from_json(&events_raw, envelope);
    envelope.runtime.selected_ids = host.selection.iter().cloned().collect();
}
//#endregion 🔖️BoardHostSync

//#region 🔖️UiScopes
/// 🐢️ Narrow `UiDirtyScope` shared by pure view/selection/camera actions that only touch the 3
/// canvas panes (never a panel or engagement/measure/utility refresh).
pub fn puzzle2d_window_only_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: board::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(), panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false }
}

/// 🐢️ Narrow `UiDirtyScope` for actions that additionally change the engagement bar (active utility,
/// brush weights, LOD/grid settings, engagement text input) but never touch document content.
pub fn puzzle2d_window_and_engagements_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: board::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(), panel_bodies: Vec::new(), utilities: false, tools: false, engagements: true, measures: false, labels: false }
}

/// 🐢️ Narrow `UiDirtyScope` for settings surfaced in the measures sidebar (LOD mode, grid, brush
/// weights, suggestion offset) but that never touch document content or the engagement bar.
pub fn puzzle2d_window_and_measures_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: board::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(), panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: true, labels: false }
}

/// 🐢️ Narrow `UiDirtyScope` for a runtime-only selection change: the 3 canvas panes plus the
/// layers/properties panels (which highlight the selection) and the engagement bar.
pub fn puzzle2d_select_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: board::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
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

            /// 🧪️ Test-only reverse of `action_id()` — builds the variant for a given action id, for
            /// the testkit's `dispatch(...)` helper. Panics on an unknown action id (a test bug, not
            /// a runtime path).
            #[cfg(test)]
            fn from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Self {
                match action {
                    $($id => Puzzle2dCommand::$Variant { window_id, args }),*,
                    other => panic!("unknown puzzle2d action id in test: {other}"),
                }
            }
        }
    };
}

puzzle2d_command_variants! {
    AddNode = "addNode",
    SetActiveExample = "setActiveExample",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    ForceLayout = "forceLayout",
    FocusSelection = "focusSelection",
    SelectAll = "selectAll",
    ClearSelection = "clearSelection",
    SelectSameKind = "selectSameKind",
    SetSelectionFlag = "setSelectionFlag",
    PatchInspectorNodes = "patchInspectorNodes",
    RedrawHandles = "redrawHandles",
    Reorganize = "reorganize",
    ApplyBoardEvents = "applyBoardEvents",
    SetFillCount = "setFillCount",
    BrushFillSessionStep = "brushFillSessionStep",
    BrushCommitSlot = "brushCommitSlot",
    SetCamera = "setCamera",
    SetSelection = "setSelection",
    DocumentSelect = "documentSelect",
    EngagementInput = "engagementInput",
    EngagementSubmit = "engagementSubmit",
    EngagementAbort = "engagementAbort",
    EngagementControlSelect = "engagementControlSelect",
    SetLodModeForPane = "setLodModeForPane",
    SetGridSnapEnabled = "setGridSnapEnabled",
    SetGridFactor = "setGridFactor",
    SetSelectionMethod = "setSelectionMethod",
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
    pub effects: &'a mut Vec<HostEffect>,
    pub ui_scope: &'a mut UiDirtyScope,
}
//#endregion 🔖️ActionContext

//#region 🔖️ContextMenu
/// 🖱️ On-demand puzzle 2d board context menu from selection snapshot. Grouped disclosure:
/// toggleHidden/toggleLocked/duplicate/focusSelection stay top-level (the four most frequent
/// verbs); selectSameKind folds into the "selection" taxonomy group; deleteSelection stays the
/// destructive tail. `organize_context_menu` (applied automatically at the
/// `VcsArtifactApp::context_menu` funnel) sorts groups into `RIBBON_PARENT_CATEGORIES` order and
/// inserts the pre-destructive separator itself, so no manual `.separator()` calls are needed here.
fn puzzle2d_context_menu_items(registry: &semio_framework_plugin::AppActionRegistry, fixture: &Value, selected: &[String], is_de: bool) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
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
        return Menu::of(registry).item(item("selectAll", if is_de { "Alles auswählen" } else { "Select All" }, "select-all", "selectAll", None, false, false)).build();
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
    let phrase = selection_count_phrase(is_de, &[(selected.len(), if is_de { "Element" } else { "item" }, if is_de { "Elemente" } else { "items" })]);
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
        .item(item("toggleHidden", hide_label, if any_visible { "eye-off" } else { "eye" }, "setSelectionFlag", Some(json!({ "flag": "hidden", "value": any_visible })), false, false))
        .item(item("toggleLocked", lock_label, if any_unlocked { "lock" } else { "lock-open" }, "setSelectionFlag", Some(json!({ "flag": "locked", "value": any_unlocked })), false, false))
        .item(item("duplicate", if is_de { "Duplizieren" } else { "Duplicate" }, "copy", "duplicateSelection", None, false, !has_selected_node))
        .item(item("focusSelection", if is_de { "Auf Auswahl zoomen" } else { "Zoom to selection" }, "crosshair", "focusSelection", None, false, false))
        .group("selection", |m| m.item(item("selectSameKind", if is_de { "Gleiche Art auswählen" } else { "Select same kind" }, "layers", "selectSameKind", None, false, false)))
        .item(item("deleteSelection", &format!("{} ({phrase})", if is_de { "Löschen" } else { "Delete" }), "trash", "deleteSelection", None, true, false))
        .build()
}
//#endregion 🔖️ContextMenu

//#region 🔖️PlayApp
/// 🧩️ Puzzle-2d play app. Owns the `BoardHost` engine; the persisted document (the bare fixture json)
/// lives in the wrapping `VcsArtifactApp`'s operation store and the view state in `Puzzle2dConfig`.
#[derive(Default, Clone, Copy)]
pub struct Puzzle2dPlayApp;

impl Puzzle2dPlayApp {
    fn scene_for(fixture: serde_json::Value, config: &Puzzle2dConfig, window_id: Option<&str>) -> Puzzle2dScene {
        let active_utility = puzzle2d_active_utility(config, window_id);
        Puzzle2dScene {
            fixture,
            runtime: config.clone(),
            active_utility,
        }
    }
}

impl ArtifactApp for Puzzle2dPlayApp {
    const APP_ID: &'static str = PUZZLE2D_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE2D_FIXTURE_SCHEMA;
    type Snapshot = Puzzle2dPlaySnapshot;
    type Mutation = Puzzle2dMutation;
    type Config = Puzzle2dConfig;
    type ConfigMutation = Puzzle2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = Puzzle2dPresence;
    type PresenceMutation = Puzzle2dPresenceMutation;
    type Command = Puzzle2dCommand;

    fn initial_snapshot() -> Puzzle2dPlaySnapshot {
        Puzzle2dPlaySnapshot(serde_json::to_value(default_empty_fixture()).unwrap_or(serde_json::Value::Null))
    }

    /// 🏷️ Maps each `Puzzle2dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle2dCommand) -> &'static str {
        command.action_id()
    }

    /// 🎬️ Dispatch only: sync the board host, delegate to the owning `🎮️commands/*` arm, then replay
    /// the host's own events and turn the mutated scene into the granular operation delta plus a
    /// config snapshot. No behaviour lives in this match.
    fn handle(command: &Puzzle2dCommand, doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation, Self::DraftMutation>, Fault> {
        let config = cfg.snapshot;
        let (action, args, window_id) = (command.action_id(), command.args(), command.window_id());
        let before = doc.snapshot.0.clone();
        let active_utility = puzzle2d_active_utility(config, window_id);
        let mut scene = Self::scene_for(before.clone(), config, window_id);
        // 🐚️ ArtifactApp::handle is pure (no &self) — rebuild a fresh BoardHost from the document
        // each call. The previous last_synced_fixture cache lived on &self and cannot return.
        let host = RefCell::new(BoardHost::default());
        {
            let mut host_mut = host.borrow_mut();
            sync_host_fixture_content(&mut host_mut, &scene);
            // 🧹️ `parse_fixture_v1` always `clear_scene()`s then rebuilds, so it unconditionally emits
            // an `edgeCreate` for every edge as a side effect of parsing — not a real structural
            // change. Discard that parse-induced noise now so `apply_host_events` below only sees
            // events genuinely produced by *this* action's own engine calls.
            let _ = host_mut.drain_events_json();
            sync_host_runtime_state(&mut host_mut, &scene);
        }
        let mut effects: Vec<HostEffect> = Vec::new();
        // 🐢️ Default to Full (safe: every unrecognized/rare action re-renders everything); the
        // narrow-tier arms below override it to the smallest scope that actually covers what they touch.
        let mut ui_scope = UiDirtyScope::Full;
        {
            let ctx = &mut Puzzle2dActionCtx { host: &host, scene: &mut scene, window_id, active_utility, effects: &mut effects, ui_scope: &mut ui_scope };
            match action {
                "setSelection" | "documentSelect" => selection_commands::set_selection(ctx, args),
                "selectAll" => selection_commands::select_all(ctx),
                "clearSelection" => selection_commands::clear_selection(ctx),
                "selectSameKind" => selection_commands::select_same_kind(ctx),
                "deleteSelection" => selection_commands::delete_selection(ctx),
                "duplicateSelection" => selection_commands::duplicate_selection(ctx),
                "setSelectionFlag" => selection_commands::set_selection_flag(ctx, args),
                "setSelectionMethod" => selection_commands::set_selection_method(ctx, args),
                "addNode" => node::add_node(ctx, args),
                "patchInspectorNodes" => node::patch_inspector(ctx, args),
                "redrawHandles" => node::redraw_handles(ctx),
                "forceLayout" | "reorganize" => node::force_layout(ctx),
                "setCamera" => camera::set_camera(ctx, args),
                "focusSelection" => camera::focus_selection(ctx),
                "setActiveExample" => example::set_active_example(ctx, args),
                SET_ACTIVE_UTILITY_ACTION_ID => utility::set_active_utility(ctx, args),
                "engagementInput" => engagement::engagement_input(ctx, args),
                "engagementSubmit" => engagement::engagement_submit(ctx, args),
                "engagementAbort" => engagement::engagement_abort(ctx, args),
                "engagementControlSelect" => engagement::engagement_control_select(ctx, args),
                "setLodModeForPane" => lod::set_lod_mode_for_pane(ctx, args),
                "lodScaleJson" => lod::lod_scale_json(ctx),
                "setGridSnapEnabled" => grid::set_grid_snap_enabled(ctx, args),
                "setGridFactor" => grid::set_grid_factor(ctx, args),
                "setBrushKindWeights" => brush::set_brush_kind_weights(ctx, args),
                "setBrushNodeSize" => brush::set_brush_node_size(ctx, args),
                "setSuggestionOffset" => brush::set_suggestion_offset(ctx, args),
                "brushCycleCandidate" => brush::cycle_candidate(ctx, args),
                "brushSetCandidateIndex" => brush::set_candidate_index(ctx, args),
                "brushOpenSlot" => brush::open_slot(ctx, args),
                "brushCommitSlot" => brush::commit_slot(ctx),
                "brushCancelSlot" => brush::cancel_slot(ctx),
                "setFillCount" => brush::set_fill_count(ctx, args),
                "brushFillSessionBegin" => brush::fill_session_begin(ctx, args),
                "brushFillSessionStep" => brush::fill_session_step(ctx, args),
                "brushFillSessionClear" => brush::fill_session_clear(ctx),
                "applyBoardEvents" => board::apply_board_events(ctx, args),
                "setLocale" => locale::set_locale(ctx, args),
                "setTerminology" => locale::set_terminology(ctx, args),
                _ => {}
            }
        }
        apply_host_events(&mut host.borrow_mut(), &mut scene);
        let operations = puzzle2d_document_delta_operations(&before, &scene.fixture);
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
    fn io() -> Option<AppIo> {
        Some(
            AppIo::from_document("puzzle.2d", MediaType { class: MediaClass::TwoD, form: MediaForm::Design }, ArtifactPresentation { id: "2d.puzzle".into(), name: "2D Puzzle".into(), dimension: "2d".into(), component_kind: "puzzle2d".into() })
                .with_ports(vec![
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
                ]),
        )
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

    fn render(body_key: &str, doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>) -> UiNode {
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
        match body_key {
            overview::BODY_KEY => overview::render(&document_json, &envelope),
            detail::BODY_KEY => detail::render(&document_json, &envelope),
            selection::BODY_KEY => selection::render(&document_json, &envelope),
            document::PUZZLE2D_PLAY_BODY_LAYERS => document::render(&envelope, labels),
            catalogue::PUZZLE2D_PLAY_BODY_CATALOGUE => catalogue::render(&envelope.fixture, labels),
            inspection::PUZZLE2D_PLAY_BODY_PROPERTIES => inspection::render(&envelope, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
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
                    (wid, edit::puzzle2d_engagement(&envelope, &crate::artifacts::puzzle2d::engine::board_host::puzzle_board_host(), pane, labels))
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
        let mut selected = config.selected_ids.clone();
        if let Some(surface) = request.surface.as_ref() {
            let ids: Vec<String> = surface.selection.iter().flat_map(|g| g.ids.iter().cloned()).collect();
            if !ids.is_empty() {
                selected = ids;
            }
        }
        puzzle2d_context_menu_items(registry, &doc.snapshot.0, &selected, is_de)
    }
}
//#endregion 🔖️PlayApp

//#region 🔖️Manifest
/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector/engagement-bound
/// vocabulary dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn puzzle2d_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, kind) }
}

/// 🗺️ Builds the full `LocalizedLabel` matrix for one `Puzzle2dLabels` field — for the static
/// manifest, which must carry every (terminology, locale) cell up front rather than a single
/// resolved-at-render-time `LabelText` (see `terminology::puzzle2d_labels`).
pub fn puzzle2d_localized(field: impl Fn(&Puzzle2dLabels) -> semio_framework_plugin::LabelText) -> LocalizedLabel {
    LocalizedLabel::from_fn(|terminology, locale| field(Puzzle2dLabels::labels(locale, terminology)).as_str().to_string())
}

pub fn create_puzzle2d_app() -> App {
    let mut host = puzzle_board_host();
    let envelope = Puzzle2dScene { fixture: default_empty_fixture(), runtime: Puzzle2dPlayRuntime::default(), active_utility: select_utility::UTILITY_ID.into() };
    sync_host_from_envelope(&mut host, &envelope);
    let labels = puzzle2d_labels(&Puzzle2dConfig::default());
    App::from_builder(
        App::builder(PUZZLE2D_PLAY_APP_ID, LocalizedLabel::native("Puzzle 2D", "Puzzle 2D"))
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
            .panel_tab_def(document::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            // ✏️ Palette-visible content operations.
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            // 🗂️ Referenced by `puzzle2d_context_menu_items` — categorized for grouped-context-menu disclosure.
            .action_with(ActionDefinition::new_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .keybinding("delete,backspace", "deleteSelection")
            .action_with(ActionDefinition::new_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).with_category("create"))
            .mutation("forceLayout", LocalizedLabel::native("Force Layout", "Kraftbasiertes Layout"))
            .action_with(ActionDefinition::new_catalog("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"), ActionKind::Mutation).with_category("view"))
            // 👁️ Palette-visible ephemeral view/selection commands.
            .action_with(ActionDefinition::new_catalog("selectAll", LocalizedLabel::native("Select All", "Alles auswählen"), ActionKind::View).with_category("selection"))
            .view_action("clearSelection", LocalizedLabel::native("Clear Selection", "Auswahl aufheben"))
            .action_with(ActionDefinition::new_catalog("selectSameKind", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).with_category("selection"))
            // 🔧️ Internal content operations — inspector/panel/board/import-bound, not palette commands.
            .action_with(puzzle2d_internal_action("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).with_category("settings"))
            .action_with(puzzle2d_internal_action("patchInspectorNodes", LocalizedLabel::native("Patch Inspector Nodes", "Inspektorknoten aktualisieren"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("redrawHandles", LocalizedLabel::native("Redraw Handles", "Anschlüsse neu zeichnen"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("applyBoardEvents", LocalizedLabel::native("Apply Board Events", "Board-Ereignisse anwenden"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("brushFillSessionStep", LocalizedLabel::native("Brush Fill Session Step", "Pinsel-Füllsitzung-Schritt"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("brushCommitSlot", LocalizedLabel::native("Brush Commit Slot", "Pinsel-Platz übernehmen"), ActionKind::Mutation))
            // 🖱️ Internal pointer/gesture/engagement view vocabulary — pure runtime/host state, emit no operations.
            // 🎥️ `setCamera` is session-only view state, so it belongs in this View-kind group.
            .action_with(puzzle2d_internal_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("documentSelect", LocalizedLabel::native("Document Select", "Dokument auswählen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View))
            .action_with(puzzle2d_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setLodModeForPane", LocalizedLabel::native("Set LOD Mode For Pane", "LOD-Modus für Bereich festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setSelectionMethod", LocalizedLabel::native("Set Selection Method", "Auswahlmethode festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setBrushKindWeights", LocalizedLabel::native("Set Brush Kind Weights", "Pinsel-Artgewichte festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setBrushNodeSize", LocalizedLabel::native("Set Brush Node Size", "Pinsel-Knotengröße festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setSuggestionOffset", LocalizedLabel::native("Set Suggestion Offset", "Vorschlagsversatz festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushCycleCandidate", LocalizedLabel::native("Brush Cycle Candidate", "Pinselkandidat wechseln"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushSetCandidateIndex", LocalizedLabel::native("Brush Set Candidate Index", "Pinselkandidatenindex festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushOpenSlot", LocalizedLabel::native("Brush Open Slot", "Pinsel-Platz öffnen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushCancelSlot", LocalizedLabel::native("Brush Cancel Slot", "Pinsel-Platz abbrechen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushFillSessionBegin", LocalizedLabel::native("Brush Fill Session Begin", "Pinsel-Füllsitzung beginnen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushFillSessionClear", LocalizedLabel::native("Brush Fill Session Clear", "Pinsel-Füllsitzung leeren"), ActionKind::View))
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
            // 🧰️ Canvas utilities — one exclusive set, active utility host-owned (never a document
            // operation); bound to the interactive overview pane by that window's own definition.
            .utility(select_utility::definition(puzzle2d_localized(|l| l.select)))
            .utility(brush_utility::definition(puzzle2d_localized(|l| l.brush)))
            // 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
            .tool(fill::definition(puzzle2d_localized(|l| l.fill)))
            .default_layout(edit::layout()),
    )
    .example_source(&*crate::examples::puzzle2d::concrete_forest::SOURCE)
    .example_source(&*crate::examples::puzzle2d::nakagin_capsule_tower::SOURCE)
    .workflow("puzzle2d", "Puzzle 2D", "layout")
}

//#region 🔖️SvgBridge
/// 🌉️ Builds a real `semio/drawing` (`SemioDrawingSnapshot`) scene graph straight out of this
/// artifact's own `Puzzle2dSnapshot`: circle/rectangle nodes become closed paths (circles as two
/// `ArcTo` semicircles), edges become straight lines between their real resolved handle rim
/// positions (`handle_position_on_circle`/`handle_position_on_rectangle` — the SAME kernel math
/// the interactive board itself hit-tests and snaps against, not reinvented geometry), and node
/// `text` becomes a `Text` node. Bounding-box canvas with a fixed margin (the camera is
/// session-only view state, never persisted, so it plays no part in the document export).
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn puzzle2d_snapshot_to_drawing(
    snapshot: &Puzzle2dSnapshot,
) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot {
    use crate::artifacts::puzzle2d::engine::{handle_position_on_circle, handle_position_on_rectangle, Point};
    use crate::artifacts::puzzle2d::Puzzle2dNode;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::engine::geometry::{SemioPoint2, SemioRgba, SemioTransform};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{
        DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA,
    };

    const NODE_STYLE: &str = "puzzle2d.node";
    const EDGE_STYLE: &str = "puzzle2d.edge";
    const DEFAULT_RADIUS: f64 = 24.0;
    const DEFAULT_WIDTH: f64 = 48.0;
    const DEFAULT_HEIGHT: f64 = 48.0;
    const MARGIN: f64 = 40.0;

    fn is_rectangle(node: &Puzzle2dNode) -> bool {
        node.shape.as_deref() == Some("rectangle") || (node.shape.is_none() && node.width.is_some())
    }
    fn half_extents(node: &Puzzle2dNode) -> (f64, f64) {
        if is_rectangle(node) {
            (node.width.unwrap_or(DEFAULT_WIDTH) / 2.0, node.height.unwrap_or(DEFAULT_HEIGHT) / 2.0)
        } else {
            let r = node.radius.unwrap_or(DEFAULT_RADIUS);
            (r, r)
        }
    }

    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for node in &snapshot.nodes {
        let (hw, hh) = half_extents(node);
        min_x = min_x.min(node.x - hw);
        max_x = max_x.max(node.x + hw);
        min_y = min_y.min(node.y - hh);
        max_y = max_y.max(node.y + hh);
    }
    if !min_x.is_finite() {
        min_x = 0.0;
        min_y = 0.0;
        max_x = BOARD_DEFAULT_WIDTH as f64;
        max_y = BOARD_DEFAULT_HEIGHT as f64;
    }
    let origin_x = min_x - MARGIN;
    let origin_y = min_y - MARGIN;
    let canvas_width = (max_x - min_x) + 2.0 * MARGIN;
    let canvas_height = (max_y - min_y) + 2.0 * MARGIN;

    let mut handle_owner: HashMap<&str, usize> = HashMap::new();
    for (index, node) in snapshot.nodes.iter().enumerate() {
        for handle in &node.handles {
            handle_owner.insert(handle.id.as_str(), index);
        }
    }
    let resolve_handle = |handle_id: &str| -> Option<SemioPoint2> {
        let &owner = handle_owner.get(handle_id)?;
        let node = &snapshot.nodes[owner];
        let handle = node.handles.iter().find(|h| h.id == handle_id)?;
        let center = Point::new(node.x, node.y);
        let world = if is_rectangle(node) {
            handle_position_on_rectangle(center, node.width.unwrap_or(DEFAULT_WIDTH), node.height.unwrap_or(DEFAULT_HEIGHT), handle.angle)
        } else {
            handle_position_on_circle(center, node.radius.unwrap_or(DEFAULT_RADIUS), handle.angle)
        };
        Some(SemioPoint2 { x: world.x() - origin_x, y: world.y() - origin_y })
    };

    let mut children: Vec<DrawNode> = Vec::new();
    for edge in &snapshot.edges {
        if edge.visible == Some(false) {
            continue;
        }
        if let (Some(from), Some(to)) = (resolve_handle(&edge.source), resolve_handle(&edge.target)) {
            children.push(DrawNode::Path { segments: vec![PathSegment::MoveTo { to: from }, PathSegment::LineTo { to }], style: Some(EDGE_STYLE.into()) });
        }
    }
    for node in &snapshot.nodes {
        if node.visible == Some(false) {
            continue;
        }
        let cx = node.x - origin_x;
        let cy = node.y - origin_y;
        let path = if is_rectangle(node) {
            let (hw, hh) = half_extents(node);
            DrawNode::Path {
                segments: vec![
                    PathSegment::MoveTo { to: SemioPoint2 { x: cx - hw, y: cy - hh } },
                    PathSegment::LineTo { to: SemioPoint2 { x: cx + hw, y: cy - hh } },
                    PathSegment::LineTo { to: SemioPoint2 { x: cx + hw, y: cy + hh } },
                    PathSegment::LineTo { to: SemioPoint2 { x: cx - hw, y: cy + hh } },
                    PathSegment::Close,
                ],
                style: Some(NODE_STYLE.into()),
            }
        } else {
            let r = node.radius.unwrap_or(DEFAULT_RADIUS);
            DrawNode::Path {
                segments: vec![
                    PathSegment::MoveTo { to: SemioPoint2 { x: cx - r, y: cy } },
                    PathSegment::ArcTo { rx: r, ry: r, x_rotation: 0.0, large_arc: true, sweep: true, to: SemioPoint2 { x: cx + r, y: cy } },
                    PathSegment::ArcTo { rx: r, ry: r, x_rotation: 0.0, large_arc: true, sweep: true, to: SemioPoint2 { x: cx - r, y: cy } },
                    PathSegment::Close,
                ],
                style: Some(NODE_STYLE.into()),
            }
        };
        children.push(path);
        if let Some(text) = node.text.as_deref().filter(|t| !t.is_empty()) {
            children.push(DrawNode::Text { value: text.to_string(), at: SemioPoint2 { x: cx, y: cy }, style: None });
        }
    }

    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: canvas_width.max(1.0), height: canvas_height.max(1.0), background: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) },
        styles: vec![
            DrawStyle { name: NODE_STYLE.into(), fill: Some(SemioRgba { r: 0.86, g: 0.91, b: 0.98, a: 1.0 }), stroke: Some(SemioRgba { r: 0.20, g: 0.32, b: 0.52, a: 1.0 }), stroke_width: Some(1.5), opacity: None },
            DrawStyle { name: EDGE_STYLE.into(), fill: None, stroke: Some(SemioRgba { r: 0.35, g: 0.35, b: 0.38, a: 1.0 }), stroke_width: Some(1.0), opacity: None },
        ],
        layers: vec![DrawLayer { id: "0".into(), name: "puzzle2d".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
    }
}

/// 🌉️ Real drawing export: `Puzzle2dSnapshot` → `SemioDrawingSnapshot` → (via stdio's real
/// `semio/drawing`→`svg` bridge, called through `io_dispatch` — never hand-rolled SVG here) →
/// SVG XML text. Replaces the previous generic `title_card_svg` placeholder.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn puzzle2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    use semio_framework_plugin::{io_dispatch, Dialect, ErasedComposeSource, IoDirection, IoKey, IoPayload, StandardId, SubsetId};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
    use semio_s_plugin_stdio::artifacts::svg::SvgSnapshot;

    let snapshot: Puzzle2dSnapshot = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
    let drawing = puzzle2d_snapshot_to_drawing(&snapshot);
    let width = drawing.canvas.width.round().max(1.0) as u32;
    let height = drawing.canvas.height.round().max(1.0) as u32;

    const DRAWING_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
    const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };
    let key = IoKey {
        artifact_kind: DRAWING_DIALECT.artifact_kind.to_string(),
        standard: DRAWING_DIALECT.standard.0.to_string(),
        subset: DRAWING_DIALECT.subset.0.to_string(),
        direction: IoDirection::Export,
        format_kind: SVG_DIALECT.artifact_kind.to_string(),
        format_standard: SVG_DIALECT.standard.0.to_string(),
        format_subset: SVG_DIALECT.subset.0.to_string(),
    };
    let source = ErasedComposeSource {
        dialect: DRAWING_DIALECT,
        payload: IoPayload::Binary(<SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(&drawing)),
    };
    let composed = io_dispatch(&key, std::slice::from_ref(&source)).map_err(|e| e.message)?;
    let svg_bytes = match composed.payload {
        IoPayload::Binary(bytes) => bytes,
        IoPayload::Text(text) => text.into_bytes(),
    };
    let svg = <SvgSnapshot as store::ArtifactPack>::decode_pack(&svg_bytes).map_err(|e| format!("puzzle2d svg decode: {e:?}"))?;
    let printed = <SvgSnapshot as store::ArtifactDsl>::print_dsl(&svg);
    let body = store::semio_format::split_text_preamble(&printed).map(|(_, rest)| rest.to_string()).unwrap_or(printed);
    Ok((body, width, height))
}
//#endregion 🔖️SvgBridge

/// 📥️ Tier C DWG import — the puzzle-2d fixture only supports circle/rectangle nodes (no polygonal
/// outlines), so this always returns an empty board; never errors on a structurally valid DWG.
/// The DWG's extents no longer frame a camera here: the camera is session-only `Puzzle2dConfig`
/// state, and this import path produces a bare document with no live app instance to receive it.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[allow(clippy::unnecessary_wraps, reason = "the fallible signature is fixed by `semio_framework_os::register_dwg_import_handler`; puzzle-2d simply has no failure mode.")]
fn puzzle2d_document_json_from_dwg(_drawing: &semio_framework::DwgDrawing) -> Result<Value, String> {
    Ok(default_empty_fixture())
}

/// 🗂️ Registers `Puzzle2dPlaySnapshot`'s pack<->dsl codec under its real `document_schema()` string
/// so `framework/sync`'s `FolderEndpoint::Pack` can print/parse puzzle-2d play documents without
/// depending on this crate's concrete `Projection`/`Mutation` types, plus the 2d export/import
/// handlers. Called by the plugin `setup:` hook (`crate::artifacts::puzzle2d::engine::register`).
pub fn register_puzzle2d_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Puzzle2dPlayApp>(PUZZLE2D_FIXTURE_SCHEMA);
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    {
        semio_framework_os::register_2d_export_handlers("2d.puzzle", "puzzle2d", puzzle2d_document_json_to_svg);
        semio_framework_os::register_dwg_import_handler("2d.puzzle", puzzle2d_document_json_from_dwg);
    }
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ The one puzzle2d-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
/// instead of re-deriving a store/dispatch/render scaffold of its own.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::{ActionMeta, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type Puzzle2dApp = VcsArtifactApp<Puzzle2dPlayApp>;

    pub fn meta(actor: &str) -> ActionMeta {
        semio_framework_plugin::testkit::meta(actor)
    }

    pub fn app() -> Puzzle2dApp {
        semio_framework_plugin::testkit::new_app::<Puzzle2dPlayApp>()
    }

    /// 🧰️ A registry-backed app so kind discipline (View/Shell actions must emit no operations) and the
    /// utility contract are enforced exactly as in production.
    pub fn app_with_registry() -> Puzzle2dApp {
        semio_framework_plugin::testkit::new_app_with_registry::<Puzzle2dPlayApp>(create_puzzle2d_app)
    }

    /// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
    /// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
    /// `Self::Command` channel). Reconstructs the `Puzzle2dCommand` from the same
    /// `(action, args, window_id)` triple every pre-B1 test already passed.
    pub fn dispatch(app: &mut Puzzle2dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
        // 🕰️ Framework-reserved verbs (undo/redo/checkpoint/…) stay on `handle_action`.
        if matches!(action, "undo" | "redo" | "commitCheckpoint" | "createAlternative" | "switchAlternative" | "checkoutCheckpoint" | "copy" | "cut" | "paste" | "revertToCommand" | "historyFilter" | "noteShellCommand") {
            return app.handle_action(action, args, &meta("local"));
        }
        app.dispatch_typed(Puzzle2dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), &meta("local"))
    }

    pub fn concrete_forest_app() -> Puzzle2dApp {
        let mut app = app();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None).expect("load concrete forest");
        app
    }

    /// 🖼️ The rendered body, serialized — every panel/window assertion greps this string.
    pub fn render_body(app: &mut Puzzle2dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("serialize rendered node")
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
    use semio_framework_plugin::{PluginApp, FRAMEWORK_HISTORY_BODY_KEY};
    use store::{Backbone, BackboneMessage, MemoryBackbone};

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
    #[test]
    fn add_node_action_emits_upsert_op_and_appends_node() {
        let mut app = app();
        let result = dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
        assert_eq!(result.mutations.len(), 1, "addNode must emit exactly one granular operation");
        assert_eq!(fixture_nodes(&fixture_of(&app)).len(), 1);
    }

    #[test]
    fn set_active_example_loads_concrete_forest_via_operations() {
        let mut app = app();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None).expect("load example");
        assert!(!fixture_nodes(&fixture_of(&app)).is_empty());
    }

    /// 📦️ `Puzzle2dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
    /// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
    /// `serde_json::Value` bridge impls).
    #[test]
    fn puzzle2d_play_projection_pack_round_trips() {
        let app = concrete_forest_app();
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&app.snapshot().expect("projection"));
    }

    #[test]
    fn select_then_delete_selection_removes_the_node() {
        let mut app = app();
        dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
        let node_id = first_node_id(&app);
        dispatch(&mut app, "setSelection", Some(&json!({ "ids": [node_id] })), None).expect("select");
        dispatch(&mut app, "deleteSelection", None, None).expect("delete");
        assert!(fixture_nodes(&fixture_of(&app)).is_empty());
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
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
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::puzzle2d::spr::Puzzle2dStore;
        use crate::artifacts::puzzle2d::{Puzzle2dNode, PUZZLE_2D_SCHEMA};
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand};

        let mut store = Puzzle2dStore::new(create_document_envelope(PUZZLE_2D_SCHEMA, "puzzle2d", Puzzle2dSnapshot::default(), None));
        let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
        store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::artifacts::puzzle2d::mutations::create_node(node, None)], description: None }).expect("apply");
        let edit: &Edit<Puzzle2dMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle2dSnapshot, Puzzle2dMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests

    //#region 🔖️BoardEvents
    /// 🎥️ `setCamera` is session-only view state: a camera drag never creates a VCS edit, so there is
    /// nothing to coalesce and nothing for `undo` to revert.
    #[test]
    fn set_camera_is_session_only_and_never_undoable() {
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
    #[test]
    fn repeated_actions_do_not_duplicate_edges() {
        let mut app = app();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID })), None).expect("load nakagin");
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
    #[test]
    fn apply_board_events_select_persists_across_the_next_action() {
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
    #[test]
    fn apply_board_events_camera_event_commits() {
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
    #[test]
    fn select_action_emits_no_operations() {
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
    #[test]
    fn select_action_declares_partial_ui_scope() {
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
    #[test]
    fn camera_event_declares_window_only_ui_scope() {
        let mut app = app();
        let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "camera", "payload": { "x": 1.0, "y": 2.0, "zoom": 1.0 } }]).to_string() })), None).expect("camera event");
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
                assert_eq!(window_bodies.len(), 3);
                assert_eq!(panel_bodies, vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()], "window-only scope still gains the history panel body");
                assert!(!engagements && !measures && !utilities && !tools && !labels);
            }
            other => panic!("expected a Partial ui_scope for a camera event, got {other:?}"),
        }
    }

    /// 🐢️ Perf round 3: an empty `applyBoardEvents` batch (no-operation) must declare nothing beyond the
    /// history panel body — the View action still logs a command-history entry, but no board surface is dirtied.
    #[test]
    fn empty_board_events_declare_none_ui_scope() {
        let mut app = app();
        let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": "[]" })), None).expect("no-operation");
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
                assert!(window_bodies.is_empty(), "empty board events must not dirty any window body");
                assert_eq!(panel_bodies, vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()]);
                assert!(!engagements && !measures && !utilities && !tools && !labels);
            }
            other => panic!("empty board events must declare only the history panel body, got {other:?}"),
        }
    }

    /// 🐢️ Perf round 3: cold-tier structural actions (document operations) must keep the safe `Full`
    /// default — no puzzle2d scope helper narrows them.
    #[test]
    fn add_node_action_declares_full_ui_scope() {
        let mut app = app();
        let result = dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
        assert!(matches!(result.ui_scope, UiDirtyScope::Full), "addNode must stay Full, got {:?}", result.ui_scope);
    }
    //#endregion 🔖️UiScope

    //#region 🔖️Manifest
    #[test]
    fn app_definition_has_three_lod_pane_window_kinds() {
        let app = create_puzzle2d_app();
        let ids: Vec<&str> = app.definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
        assert_eq!(ids, vec![overview::WINDOW_KIND_ID, detail::WINDOW_KIND_ID, selection::WINDOW_KIND_ID]);
        for window in &app.definition.window_kinds {
            assert!(window.options.engagement.as_option().is_some(), "pane {} must have engagement", window.id);
            assert!(!window.options.measures.is_empty(), "pane {} must have LOD/suggestion measures", window.id);
        }
    }

    /// 🧰️ The app declares exactly the select/brush canvas utilities and binds them to the interactive
    /// overview pane; fill is declared as a mode-level tool instead.
    #[test]
    fn utility_registry_declares_utilities() {
        let definition = create_puzzle2d_app().definition;
        let ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(ids, vec![select_utility::UTILITY_ID, brush_utility::UTILITY_ID]);
        let overview_window = definition.window_kinds.iter().find(|window| window.id == overview::WINDOW_KIND_ID).expect("overview pane");
        let overview_utilities: Vec<&str> = overview_window.utilities.iter().map(|utility| utility.as_str()).collect();
        assert_eq!(overview_utilities, vec![select_utility::UTILITY_ID, brush_utility::UTILITY_ID]);
        assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID), "declaring utilities must inject the setActiveUtility action");
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
        let definition = create_puzzle2d_app().definition;
        let tool_ids: Vec<&str> = definition.tools.iter().map(|tool| tool.id.as_str()).collect();
        assert_eq!(tool_ids, vec![fill::TOOL_ID]);
        assert_eq!(definition.modes[0].tools, vec![ToolRef::new(fill::TOOL_ID)]);
        assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID), "declaring tools must inject the setActiveTool action");
    }

    /// 🎥️ The camera is session-only runtime state, never a document field — a DWG import (which has
    /// no live app instance to receive a runtime write) must produce a bare empty board with no
    /// `"camera"` key at all, regardless of the drawing's extents.
    #[test]
    fn dwg_import_returns_empty_board_with_no_camera_field() {
        let drawing = semio_framework::DwgDrawing { extmin: [0.0, 0.0, 0.0], extmax: [100.0, 200.0, 0.0], ..semio_framework::DwgDrawing::default() };
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
    #[test]
    fn two_instances_converge_disjoint_node_edits_via_backbone() {
        let mut instance_a = app();
        let mut instance_b = app();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://puzzle2d-convergence", "mem://puzzle2d-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        dispatch(&mut instance_a, "addNode", Some(&json!({ "kind": "seed" })), None).expect("a adds node");
        dispatch(&mut instance_b, "addNode", Some(&json!({ "kind": "other" })), None).expect("b adds node");

        // A neutral history action always calls store.dispatch(), which pumps inbound operations first.
        dispatch(&mut instance_a, "commitCheckpoint", None, None).expect("pump a");
        dispatch(&mut instance_b, "commitCheckpoint", None, None).expect("pump b");

        assert_eq!(fixture_nodes(&fixture_of(&instance_a)).len(), 2, "instance A must contain both nodes");
        assert_eq!(fixture_nodes(&fixture_of(&instance_b)).len(), 2, "instance B must contain both nodes");
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        let mut sender = app();
        let (near, mut far) = MemoryBackbone::pair("mem://puzzle2d-doc", "mem://puzzle2d-doc");
        sender.attach_backbone(Box::new(near)).expect("attach");
        dispatch(&mut sender, "addNode", Some(&json!({ "kind": "seed" })), None).expect("add");

        let mut envelopes = Vec::new();
        for message in far.receive().expect("receive") {
            if let BackboneMessage::Mutations { envelopes: operations } = message {
                envelopes.extend(operations);
            }
        }
        assert!(!envelopes.is_empty(), "the applied operation must flow onto the channel");
        let operations = envelopes;

        let mut receiver = app();
        receiver.ingest_operations(&operations).expect("ingest once");
        receiver.ingest_operations(&operations).expect("ingest twice");
        assert_eq!(fixture_nodes(&fixture_of(&receiver)).len(), 1, "feeding the same operation twice must not double-apply");
    }
    //#endregion 🔖️Convergence

    //#region 🔖️Registry
    /// 🧰️ B1: `setActiveUtility` is a real typed `Puzzle2dCommand` now (was a host-applied `ViewModel`
    /// notification): switching utilities must still emit no DOCUMENT operations — the new value lands
    /// in `Puzzle2dConfig::active_utility_by_window_id` as a config operation instead.
    #[test]
    fn utility_switch_emits_no_ops_and_no_history() {
        let mut app = app_with_registry();
        let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": brush_utility::UTILITY_ID })), Some(overview::WINDOW_KIND_ID)).expect("switch utility");
        assert!(result.mutations.is_empty(), "a utility switch must not produce document operations");
        let can_undo = dispatch(&mut app, "undo", None, None);
        assert!(can_undo.map_or(true, |r| r.mutations.is_empty()), "a utility switch must not have created a document undo step");
    }

    /// 🧭️ Kind discipline: every View-declared runtime/host action must run through the registry
    /// without tripping the "must not emit operations" guard (proving each is correctly classified).
    #[test]
    fn view_actions_emit_no_ops_through_the_registry() {
        let mut app = app_with_registry();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None).expect("load example");
        let node_id = first_node_id(&app);
        let view_dispatches: Vec<(&str, Value)> = vec![
            ("setSelection", json!({ "ids": [node_id] })),
            ("setCamera", json!({ "camera": { "x": 7.0, "y": 8.0, "zoom": 1.5 } })),
            ("selectAll", Value::Null),
            ("selectSameKind", Value::Null),
            ("clearSelection", Value::Null),
            ("setSelectionMethod", json!({ "method": "lasso" })),
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
    #[test]
    fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
        use semio_framework_plugin::{ContextMenuRequest, UiMenuRef};

        let mut app = app_with_registry();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None).expect("load example");
        let node_id = first_node_id(&app);
        dispatch(&mut app, "setSelection", Some(&json!({ "ids": [node_id] })), None).expect("select node");
        let request = ContextMenuRequest { menu: UiMenuRef { id: "puzzle2d".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("grouped disclosure menu should not be empty");
        assert_eq!(last.id, "deleteSelection", "the destructive row must stay last as a top-level leaf");
        assert_eq!(last.destructive, Some(true), "the destructive row must carry destructive: true");
    }
    //#endregion 🔖️Registry
}
//#endregion 🧪️Tests
