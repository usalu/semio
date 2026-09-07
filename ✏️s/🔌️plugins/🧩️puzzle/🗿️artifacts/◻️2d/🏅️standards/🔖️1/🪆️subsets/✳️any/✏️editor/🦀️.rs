//! 🧩️ Puzzle 2d play app — the plugin's 2d play app: its `ArtifactApp` impl (dispatch-only), the
//! transient `Puzzle2dScene` bundle its command/panel/window nodes mutate and render, the shared
//! fixture helpers they build on, and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/<window>`. This file dispatches and stitches.
//!
//! 🌉️ `ArtifactApp::Snapshot` is the `Puzzle2dPlaySnapshot` newtype over a bare
//! `serde_json::Value` fixture (see `crate::artifacts::puzzle2d::op`'s `🔖️ValueBridge`), not the typed
//! `Puzzle2dSnapshot`. Ordinary commands derive granular typed deltas; mounted fill continuations
//! bypass whole fixture materialization and publish their already-prepared typed mutations directly.

use crate::artifacts::puzzle2d::op::{puzzle2d_document_delta_operations, Puzzle2dMutation, Puzzle2dPlaySnapshot};
use crate::editor::puzzle2d::commands::{
    add_node, apply_board_events, cancel_slot, commit_slot, cycle_candidate, delete_selection, duplicate_selection, engagement_abort, engagement_control_select, engagement_input, engagement_submit, focus_selection, force_layout, lod_scale_json,
    open_slot, patch_inspector, select_same_kind, set_active_example, set_active_utility, set_brush_kind_weights, set_brush_node_size, set_camera, set_candidate_index, set_fill_count, set_grid_factor, set_grid_snap_enabled, set_locale,
    set_lod_mode_for_pane, set_selection_flag, set_suggestion_offset, set_terminology,
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
use crate::editor::puzzle2d::terminology::{puzzle2d_config_locale, puzzle2d_labels};
pub use crate::editor::puzzle2d::terminology::{puzzle2d_localized, puzzle2d_localized_phrase};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppIo, AppLabels, ArtifactEditor, ArtifactPresentation, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect, DraftView,
    Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTarget, InteractiveJobClassification, Label, LocalizedLabel, Media, MediaClass, MediaForm,
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
    let dsl_args = args.map(dsl::DslValue::from);
    let ids = semio_framework_plugin::selection_ids(dsl_args.as_ref());
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
                if let Some(dsl::DslValue::Object(presentation)) = kind.presentation.as_ref() {
                    for (key, value) in presentation {
                        row.insert(key.clone(), Value::from(value));
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
/// document owns `nodes`/`🐙️handles`/`edges`/`wires` ([`Puzzle2dKindCatalogs`]) while
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
            pub(crate) fn action_id(&self) -> &'static str {
                match self {
                    $(Puzzle2dCommand::$Variant { .. } => $id),*
                }
            }

            fn window_id(&self) -> Option<&str> {
                match self {
                    $(Puzzle2dCommand::$Variant { window_id, .. } => window_id.as_deref()),*
                }
            }

            pub(crate) fn args(&self) -> Option<&Value> {
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
        
        .group("selection", |m| { m.item(item("selectSameKind", if is_de { "Gleiche Art auswählen" } else { "Select same kind" }, "layers", "selectSameKind", None, false, false)) })
        
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
    fn scene_for(fixture: Value, config: &Puzzle2dConfig, window_id: Option<&str>) -> Puzzle2dScene {
        let active_utility = puzzle2d_active_utility(config, window_id);
        Puzzle2dScene { fixture, runtime: config.clone(), active_utility }
    }
}

//#region 🧵️RetainedCommands
/// 🧵️ The 2D editor actions that carry an exact app-owned tool proof. Only these may declare
/// [`InteractiveJobClassification::Migrated`] — UI dispatch resolves a controller/owner/factory/tool
/// /schema proof for every migrated verb, and one without a registered factory is refused outright.
pub(crate) const PUZZLE2D_RETAINED_TOOL_IDS: &[&str] = &[
    "setActiveExample",
    "forceLayout",
    "addNode",
    "applyBoardEvents",
    "reorganize",
    "brushCancelSlot",
    "brushCommitSlot",
    "brushCycleCandidate",
    "brushFillSessionAdopt",
    "brushFillSessionBegin",
    "brushFillSessionCancel",
    "brushFillSessionClear",
    "brushFillSessionDiscard",
    "brushFillSessionRetry",
    "brushFillSessionStep",
    "brushOpenSlot",
    "brushSetCandidateIndex",
    "deleteSelection",
    "duplicateSelection",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementSubmit",
    "focusSelection",
    "lodScaleJson",
    "patchInspectorNodes",
    "redrawHandles",
    "selectSameKind",
    "setBrushKindWeights",
    "setBrushNodeSize",
    "setCamera",
    "setFillCount",
    "setGridFactor",
    "setGridSnapEnabled",
    "setLocale",
    "setLodModeForPane",
    "setSelectionFlag",
    "setSuggestionOffset",
    "setTerminology",
];
const PUZZLE2D_RETAINED_PAYLOAD_SCHEMA: &str = "puzzle.2d.fixture.tool-command.v1";

/// 🎬️ The retained verbs whose whole completion is [`puzzle2d_dispatch_emit`] — one `🎮️commands/*`
/// arm run over a rebuilt scene/board host, then the same document delta and config snapshot
/// `handle` derives. Everything outside this list carries a bespoke `Work` (`setActiveExample`,
/// `forceLayout`/`reorganize`, the fill session) or an isolated reducer (`addNode`).
const PUZZLE2D_GENERIC_TOOL_IDS: &[&str] = &[
    "brushCancelSlot",
    "brushCommitSlot",
    "brushCycleCandidate",
    "brushOpenSlot",
    "brushSetCandidateIndex",
    "deleteSelection",
    "duplicateSelection",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementSubmit",
    "focusSelection",
    "patchInspectorNodes",
    "setBrushKindWeights",
    "setBrushNodeSize",
    "setCamera",
    "setGridFactor",
    "setGridSnapEnabled",
    "setLocale",
    "setLodModeForPane",
    "setSelectionFlag",
    "setSuggestionOffset",
    "setTerminology",
];

/// 🫙️ The two verbs whose `🎮️commands/*` arm is empty by construction — `selectSameKind` has no
/// channel to write framework-owned selection back, and `lodScaleJson` only reads a pure engine LOD
/// table and declares [`UiDirtyScope::None`]. Both publish nothing, so both complete through
/// `NoopPuzzleCommandWork` under a solo `HostOnly` contract rather than the dispatch pipeline.
const PUZZLE2D_HOST_ONLY_TOOL_IDS: &[&str] = &["lodScaleJson", "selectSameKind"];

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
        ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "forceLayout", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "redrawHandles", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "brushCancelSlot", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "brushCycleCandidate", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "brushOpenSlot", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "brushSetCandidateIndex", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "engagementAbort", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "engagementControlSelect", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "focusSelection", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setBrushKindWeights", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setBrushNodeSize", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setGridFactor", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setGridSnapEnabled", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setLodModeForPane", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSuggestionOffset", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setTerminology", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "applyBoardEvents", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "brushCommitSlot", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "brushFillSessionAdopt", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "brushFillSessionBegin", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "brushFillSessionCancel", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "brushFillSessionClear", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "brushFillSessionDiscard", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "brushFillSessionRetry", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "brushFillSessionStep", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "duplicateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "patchInspectorNodes", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setFillCount", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSelectionFlag", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "lodScaleJson", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "selectSameKind", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}

//#region 📬️StorePreparation
/// 📦️ Fixed envelope one retained `Puzzle2dConfig` commit may occupy. `brush_candidates` is the only
/// variable-length field (one entry per compatible brush candidate on the open slot), so 64 KiB
/// leaves two orders of magnitude of headroom over a real slot while still refusing an unbounded root.
const PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES: usize = 65_536;

struct Puzzle2dConfigStorePreparation {
    base: Option<store::SnapshotRead<Puzzle2dConfig>>,
    mutation: Option<Puzzle2dConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(Puzzle2dConfig, Vec<Puzzle2dConfigMutation>, Puzzle2dConfigMutation, usize)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Puzzle2dConfig, Puzzle2dConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    phase: u8,
    cancelled: bool,
    closing: bool,
}

/// 🎚️ Config-lane preparation factory — the precondition every `Config` publication contract above
/// depends on (`build_config_store_one_item_preparation_factory`); mirrors
/// `Puzzle3dConfigStorePreparationFactory`.
struct Puzzle2dConfigStorePreparationFactory;

/// 🌉️ Measures the fully encoded config root against the fixed store envelope. `Puzzle2dConfig` is
/// encoded through the same in-house `dsl::json` writer its `OpBinary` codec uses, so the measured
/// length is the length the store actually retains.
fn puzzle2d_config_store_bounded_bytes(value: &Puzzle2dConfig) -> Result<usize, String> {
    let encoded = dsl::json::to_json_string(value);
    if encoded.len() > PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Puzzle2d Config Store root exceeds its fixed envelope".to_string());
    }
    Ok(encoded.len())
}

/// 🎚️ Both `Puzzle2dConfigMutation` variants are admitted, each bounded by the post-image the store
/// would retain: `Snapshot` carries a whole runtime root (every generic-reduce Config completion),
/// `Fill` a fixed-shape `Puzzle2dFillRuntime` (the fill session's own lane).
fn puzzle2d_config_store_mutation_bytes(mutation: &Puzzle2dConfigMutation) -> Option<usize> {
    match mutation {
        Puzzle2dConfigMutation::Snapshot { config } => puzzle2d_config_store_bounded_bytes(config).ok(),
        Puzzle2dConfigMutation::Fill { .. } => {
            let encoded = dsl::json::to_json_string(mutation);
            (encoded.len() <= PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES).then_some(encoded.len())
        }
    }
}

fn puzzle2d_config_store_edit(forward: Puzzle2dConfigMutation, inverse: Vec<Puzzle2dConfigMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<Puzzle2dConfigMutation> {
    let id = format!("puzzle2d-config-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

impl store::ArtifactStoreOneItemPreparation<Puzzle2dConfig, Puzzle2dConfigMutation> for Puzzle2dConfigStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::{Mutation as _, MutationDiff as _};
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() || self.phase >= 2 {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        match self.phase {
            0 => {
                let base = self.base.as_ref().ok_or_else(|| "Puzzle2d Config preparation lost its exact base root".to_string())?;
                let mutation = self.mutation.take().ok_or_else(|| "Puzzle2d Config preparation lost its mutation owner".to_string())?;
                if puzzle2d_config_store_mutation_bytes(&mutation).is_none() {
                    return Err("Puzzle2d Config preparation rejected its exact mutation envelope".into());
                }
                let completed_bytes = puzzle2d_config_store_bounded_bytes(base.get())?;
                let inverse = mutation.inverse(base.get());
                let post = mutation.diff(base.get()).into_parts().0.apply(base.get()).map_err(|_| "Puzzle2d Config mutation could not produce its post root".to_string())?;
                self.candidate = Some((post, inverse, mutation, completed_bytes));
                self.phase = 1;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: completed_bytes as u64, digest: [0; 32] };
                Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            1 => {
                let (post, inverse, mutation, completed_bytes) = self.candidate.take().ok_or_else(|| "Puzzle2d Config preparation lost its semantic candidate".to_string())?;
                let authority = self.authority.as_ref().ok_or_else(|| "Puzzle2d Config preparation lost its Store authority".to_string())?;
                let prepared = authority.prepare_one_item(puzzle2d_config_store_edit(mutation, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
                self.phase = 2;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: completed_bytes as u64, digest: prepared.edit_digest() };
                self.prepared = Some(prepared);
                Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
            }
            _ => Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)),
        }
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Puzzle2dConfig, Puzzle2dConfigMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Puzzle2dConfig, Puzzle2dConfigMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.candidate.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Puzzle2d Config preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.prepared.is_none()
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<Puzzle2dConfig, Puzzle2dConfigMutation> for Puzzle2dConfigStorePreparationFactory {
    fn preflight(&self, mutation: &Puzzle2dConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Puzzle2d Config preparation rejected its lane or description".into());
        }
        let retained_bytes = puzzle2d_config_store_mutation_bytes(mutation).ok_or_else(|| "Puzzle2d Config preparation rejected its exact mutation".to_string())?;
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Puzzle2dConfig, Puzzle2dConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Puzzle2dConfig, Puzzle2dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Puzzle2dConfig, Puzzle2dConfigMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Puzzle2dConfigStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            candidate: None,
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            phase: 0,
            cancelled: false,
            closing: false,
        }))
    }
}

/// 🧩️ The Artifact-lane sibling of the Config preparation above. A `Puzzle2dMutation` is admitted
/// generically through `protocol::Mutation`/`protocol::MutationDiff` rather than an allowlist,
/// because one retained completion emits whatever the granular document delta produced (create /
/// delete node, connect / disconnect handles, manifest, compatibility, catalogs) one per store turn.
struct Puzzle2dArtifactStorePreparationFactory;

struct Puzzle2dArtifactStorePreparation {
    base: Option<store::SnapshotRead<Puzzle2dPlaySnapshot>>,
    mutation: Option<Puzzle2dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(Puzzle2dPlaySnapshot, Vec<Puzzle2dMutation>, Puzzle2dMutation)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Puzzle2dPlaySnapshot, Puzzle2dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    phase: u8,
    cancelled: bool,
    closing: bool,
}

fn puzzle2d_artifact_store_edit(forward: Puzzle2dMutation, inverse: Vec<Puzzle2dMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<Puzzle2dMutation> {
    let id = format!("puzzle2d-artifact-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<Puzzle2dPlaySnapshot, Puzzle2dMutation> for Puzzle2dArtifactStorePreparationFactory {
    fn preflight(&self, _mutation: &Puzzle2dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Puzzle2d Artifact preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Puzzle2dPlaySnapshot, Puzzle2dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Puzzle2dPlaySnapshot, Puzzle2dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Puzzle2dPlaySnapshot, Puzzle2dMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Puzzle2dArtifactStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            candidate: None,
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            phase: 0,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Puzzle2dPlaySnapshot, Puzzle2dMutation> for Puzzle2dArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::{Mutation as _, MutationDiff as _};
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() || self.phase >= 2 {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        match self.phase {
            0 => {
                let base = self.base.as_ref().ok_or_else(|| "Puzzle2d Artifact preparation lost its exact base root".to_string())?;
                let mutation = self.mutation.take().ok_or_else(|| "Puzzle2d Artifact preparation lost its mutation owner".to_string())?;
                let inverse = mutation.inverse(base.get());
                let post = mutation.diff(base.get()).into_parts().0.apply(base.get()).map_err(|_| "Puzzle2d Artifact mutation could not produce its post root".to_string())?;
                self.candidate = Some((post, inverse, mutation));
                self.phase = 1;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: [0; 32] };
                Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            1 => {
                let (post, inverse, mutation) = self.candidate.take().ok_or_else(|| "Puzzle2d Artifact preparation lost its semantic candidate".to_string())?;
                let authority = self.authority.as_ref().ok_or_else(|| "Puzzle2d Artifact preparation lost its Store authority".to_string())?;
                let prepared = authority.prepare_one_item(puzzle2d_artifact_store_edit(mutation, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
                self.phase = 2;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: 1, digest: prepared.edit_digest() };
                self.prepared = Some(prepared);
                Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
            }
            _ => Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)),
        }
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Puzzle2dPlaySnapshot, Puzzle2dMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Puzzle2dPlaySnapshot, Puzzle2dMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.candidate.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Puzzle2d Artifact preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️StorePreparation

/// 🖌️ Upper bound on the events one board flush may carry into a single retained step. The browser
/// flushes a handful of events per interaction (`PUZZLE2D_FLUSH_NOW_EVENT_NAMES` in `🖥️Board2dHost`), so
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

/// 🎬️ THE dispatch pipeline — the one implementation both [`ArtifactEditor::handle`]'s batch path and
/// every retained generic reduce run: rebuild the scene and a fresh board host from
/// `(command, before, config, selection)`, run the `🎮️commands/*` arm, replay the host's owned
/// events, then derive the granular document delta and the config snapshot. `operation` is the
/// committed public authority a mounted continuation carries and is simply `None` for a retained
/// work, which never sees an `ArtifactView`.
fn puzzle2d_dispatch_emit(command: &Puzzle2dCommand, before: Value, config: &Puzzle2dConfig, selection: &protocol::DomainSelection, operation: Option<semio_framework_plugin::AppOperationContext>) -> Emit<Puzzle2dMutation, Puzzle2dConfigMutation> {
    let (action, args, window_id) = (command.action_id(), command.args(), command.window_id());
    let active_utility = puzzle2d_active_utility(config, window_id);
    let mut scene = Puzzle2dPlayApp::scene_for(before.clone(), config, window_id);
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
        let ctx = &mut Puzzle2dActionCtx { host: &host, scene: &mut scene, window_id, active_utility, selection, effects: &mut effects, artifact_mutations: &mut artifact_mutations, ui_scope: &mut ui_scope, operation };
        match action {
            "selectSameKind" => select_same_kind::select_same_kind(ctx),
            "deleteSelection" => delete_selection::delete_selection(ctx),
            "duplicateSelection" => duplicate_selection::duplicate_selection(ctx),
            "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),
            "addNode" => add_node::add_node(ctx, args),
            "patchInspectorNodes" => patch_inspector::patch_inspector(ctx, args),
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
    Emit { artifact_mutations: operations, config_mutations, coalesce_key: None, effects, ui_scope, ..Default::default() }
}

/// 🖌️ `applyBoardEvents` is the single verb the browser's board session commits through — `brushPlace`,
/// `select`, `edgeCreate`/`edgeDelete`, `nodeDelete` and `camera` all arrive in its `eventsJson` batch
/// (`🖥️Board2dHost/🟦️.tsx`). It runs [`puzzle2d_dispatch_emit`] — the same pipeline `handle` does —
/// minus the `ArtifactView` a retained work never sees, so the committed `AppOperationContext` is absent.
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
    let selection = interaction.selection.get(PUZZLE2D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    Ok(puzzle2d_dispatch_emit(command, snapshot.0.clone(), config, &selection, None))
}

/// 🗂️ Upper bound on the entities one selection-acting retained step may touch. Nakagin — this
/// artifact's largest example — carries 180 nodes, 179 edges and 358 handles, so a whole-board
/// selection is 717 entities: this ceiling admits that with headroom while still refusing an
/// unbounded selection well under the shared `PUZZLE_COMMAND_WORK_ITEMS` (4,096) budget.
const PUZZLE2D_SELECTION_BATCH_LIMIT: usize = 1_024;

/// 🧮️ One work item per entity a selection-acting verb rewrites, one for every other generic verb
/// (each is a fixed-shape config/host setter whose cost is independent of document size). An
/// oversized selection returns `None` and is refused by the retained preflight rather than silently
/// truncated. `patchInspectorNodes` may address an explicit `ids` argument instead of the selection.
fn puzzle2d_generic_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
    let action = command.action_id();
    if !PUZZLE2D_GENERIC_TOOL_IDS.contains(&action) {
        return None;
    }
    if !matches!(action, "patchInspectorNodes" | "setSelectionFlag" | "deleteSelection" | "duplicateSelection") {
        return Some(1);
    }
    let selected = interaction.selection.get(PUZZLE2D_INTERACTION_DOMAIN).map_or(0, |selection| selection.ids.len());
    let addressed = command.args().filter(|_| action == "patchInspectorNodes").and_then(|args| args.get("ids")).and_then(Value::as_array).map_or(selected, Vec::len);
    (addressed <= PUZZLE2D_SELECTION_BATCH_LIMIT).then_some(addressed.max(1))
}

/// 🎬️ The retained completion for every [`PUZZLE2D_GENERIC_TOOL_IDS`] verb — one bounded first step
/// through [`puzzle2d_dispatch_emit`], the same body `handle` runs.
fn puzzle2d_generic_reduce(
    command: &Puzzle2dCommand,
    snapshot: &Puzzle2dPlaySnapshot,
    config: &Puzzle2dConfig,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, Fault> {
    if !PUZZLE2D_GENERIC_TOOL_IDS.contains(&command.action_id()) {
        return Err(Fault::from("puzzle2d-generic-command-mismatch"));
    }
    let selection = interaction.selection.get(PUZZLE2D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    Ok(puzzle2d_dispatch_emit(command, snapshot.0.clone(), config, &selection, None))
}

/// 🛍️ Stage hand-offs [`Puzzle2dActiveExampleWork`] spends outside its per-item cursors (one per
/// [`Puzzle2dExampleStage`] transition) — the batch driver's step budget on top of `extent`.
const PUZZLE2D_EXAMPLE_STAGE_STEPS: usize = 8;

/// 🛍️ Drives [`Puzzle2dActiveExampleWork`] — the SINGLE implementation of the example load — to its
/// terminal emit for the batch dispatch path, so `handle` and the retained job share one state
/// machine instead of a second self-chaining `Effect::DispatchAction` ladder.
fn puzzle2d_active_example_emit(command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, config: &Puzzle2dConfig) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, Fault> {
    use crate::retained_command::PuzzleCommandWork as _;
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let mut work = Puzzle2dActiveExampleWork::default();
    let bound = work.extent(command, snapshot, &interaction).ok_or_else(|| Fault::from("puzzle2d-example-exceeds-capacity"))?;
    for _ in 0..bound.saturating_add(PUZZLE2D_EXAMPLE_STAGE_STEPS) {
        if let crate::retained_command::PuzzleCommandWorkStep::Complete(emit) = work.step(command, snapshot, config, &interaction, &hover)? {
            return Ok(emit);
        }
    }
    Err(Fault::from("puzzle2d-example-did-not-terminate"))
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
    let node = <crate::artifacts::puzzle2d::Puzzle2dNode as dsl::FromValue>::from_value(dsl::DslValue::from(&node)).map_err(|_| Fault::from("puzzle2d-add-node-malformed"))?;
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
                    let row = <crate::artifacts::puzzle2d::Puzzle2dKindCompatibility as dsl::FromValue>::from_value(dsl::DslValue::from(source)).map_err(|_| Fault::from("puzzle2d-example-compatibility-malformed"))?;
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

/// 🧲️ Admission ceilings for one force-layout run. `MAX_NODES` admits the real
/// `🏗️nakagin-capsule-tower` fixture (180 nodes / 179 edges / 358 handles) with room to spare — the
/// former 64-node ceiling made `forceLayout` structurally dead on the only non-trivial example this
/// artifact ships. `MAX_HANDLES` is checked by [`Puzzle2dForceLayoutWork::extent`] as well as by the
/// `Handles` stage, so an over-wide document is refused at preflight instead of faulting mid-run.
const PUZZLE2D_FORCE_MAX_NODES: usize = 512;
const PUZZLE2D_FORCE_MAX_EDGES: usize = 4_096;
const PUZZLE2D_FORCE_MAX_HANDLES: usize = 4_096;

/// 🔁️ Iteration schedule. A spring embedder's cost per iteration is `n·(n−1)/2` repulsion pairs plus
/// one pass per edge, so a fixed 420 iterations is affordable at 64 nodes and ~7 million pair
/// evaluations at 180. Iterations are therefore derived from the graph's own per-iteration cost
/// ([`puzzle2d_force_iterations`]) so total pair+spring work stays inside
/// [`PUZZLE2D_FORCE_WORK_BUDGET`]: small graphs keep the full 420-iteration quality, Nakagin gets
/// 122, and the 512-node ceiling gets the 24-iteration floor.
const PUZZLE2D_FORCE_ITERATIONS_MAX: u32 = 420;
const PUZZLE2D_FORCE_ITERATIONS_MIN: u32 = 24;
const PUZZLE2D_FORCE_WORK_BUDGET: u64 = 2_000_000;

/// 🍰️ Per-`step()` chunk sizes. One retained step is one checkpoint boundary, so a step must carry a
/// batch of force units rather than a single one — 8,192 pair evaluations are a few hundred
/// microseconds against the 7,500 µs step budget, while one pair per step would need ~7 million
/// checkpoints for Nakagin. `extent()` counts these chunks, which is what makes the declared budget
/// match the work actually performed.
const PUZZLE2D_FORCE_UNITS_PER_STEP: usize = 8_192;
const PUZZLE2D_FORCE_NODES_PER_STEP: usize = 512;
const PUZZLE2D_FORCE_SCAN_PER_STEP: usize = 256;
const PUZZLE2D_FORCE_ITERATION_STAGE_STEPS: usize = 4;
const PUZZLE2D_FORCE_PROLOGUE_STAGE_STEPS: usize = 8;

/// 🔢️ Unordered node pairs the repulsion pass visits per iteration.
fn puzzle2d_force_pairs(nodes: usize) -> u64 {
    let nodes = nodes as u64;
    nodes.saturating_mul(nodes.saturating_sub(1)) / 2
}

/// 🔁️ Iterations for a graph of `nodes` nodes and `edges` edges — `WORK_BUDGET / cost_per_iteration`,
/// clamped to `[MIN, MAX]`. Derived from the raw document counts (not the visible subset) so
/// [`Puzzle2dForceLayoutWork::extent`] and [`Puzzle2dForceLayoutWork::step`] always agree.
fn puzzle2d_force_iterations(nodes: usize, edges: usize) -> u32 {
    let cost = puzzle2d_force_pairs(nodes).saturating_add(edges as u64);
    if cost == 0 {
        return 1;
    }
    u32::try_from(PUZZLE2D_FORCE_WORK_BUDGET / cost).unwrap_or(PUZZLE2D_FORCE_ITERATIONS_MAX).clamp(PUZZLE2D_FORCE_ITERATIONS_MIN, PUZZLE2D_FORCE_ITERATIONS_MAX)
}

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
    tool_id: &'static str,
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
        Self::new("forceLayout")
    }
}

impl Puzzle2dForceLayoutWork {
    /// 🧲️ `forceLayout` and `reorganize` are the same simulation behind two verbs, so the admitted
    /// tool id travels with the work — the retained job's decode phase rejects a payload whose
    /// action id does not equal [`crate::retained_command::PuzzleCommandWork::tool_id`].
    fn new(tool_id: &'static str) -> Self {
        Self {
            tool_id,
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

    /// 🌡️ Simulated-annealing factor for the current iteration against the size-derived schedule.
    fn cool(&self, iterations: u32) -> f64 {
        (1.0 - f64::from(self.iteration) / f64::from(iterations.max(1))).max(0.08)
    }

    fn scan_node_one(&mut self, nodes: &[Value]) -> Result<(), Fault> {
        let Some(node) = nodes.get(self.node_cursor) else {
            if self.positions.is_empty() {
                self.stage = Puzzle2dForceStage::Emit;
                return Ok(());
            }
            if self.finite_count > 0 {
                self.center[0] /= self.finite_count as f64;
                self.center[1] /= self.finite_count as f64;
            }
            self.stage = Puzzle2dForceStage::Edges;
            return Ok(());
        };
        let object = node.as_object().ok_or_else(|| Fault::from("puzzle2d-force-node-not-object"))?;
        if !Self::visible(object) {
            self.node_cursor += 1;
            return Ok(());
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
        Ok(())
    }

    fn scan_handle_one(&mut self, nodes: &[Value]) -> Result<(), Fault> {
        let object = nodes.get(self.node_cursor).and_then(Value::as_object).ok_or_else(|| Fault::from("puzzle2d-force-node-owner-lost"))?;
        let handles = object.get("handles").and_then(Value::as_array);
        let Some(handle) = handles.and_then(|handles| handles.get(self.handle_cursor)) else {
            self.node_cursor += 1;
            self.stage = Puzzle2dForceStage::Nodes;
            return Ok(());
        };
        self.handle_cursor += 1;
        let Some(handle_object) = handle.as_object() else { return Ok(()) };
        if !Self::visible(handle_object) {
            return Ok(());
        }
        let Some(handle_id) = handle_object.get("id").and_then(Value::as_str) else { return Ok(()) };
        let node_id = object.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle2d-force-node-id-owner-lost"))?;
        let added_bytes = handle_id.len().saturating_add(node_id.len());
        if self.handle_to_node.len() >= PUZZLE2D_FORCE_MAX_HANDLES || self.retained_bytes.checked_add(added_bytes).map_or(true, |bytes| bytes > crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES) {
            return Err(Fault::from("puzzle2d-force-handle-capacity"));
        }
        self.retained_bytes += added_bytes;
        self.handle_to_node.insert(handle_id.to_string(), node_id.to_string());
        Ok(())
    }

    fn scan_edge_one(&mut self, edges: Option<&Vec<Value>>) -> Result<(), Fault> {
        let Some(edge) = edges.and_then(|edges| edges.get(self.edge_cursor)) else {
            self.stage = Puzzle2dForceStage::Seed;
            return Ok(());
        };
        self.edge_cursor += 1;
        let Some(object) = edge.as_object() else { return Ok(()) };
        if !Self::visible(object) {
            return Ok(());
        }
        let (Some(source), Some(target)) = (object.get("source").and_then(Value::as_str), object.get("target").and_then(Value::as_str)) else {
            return Ok(());
        };
        let source_node = self.handle_to_node.get(source).map_or(source, String::as_str);
        let target_node = self.handle_to_node.get(target).map_or(target, String::as_str);
        let (Some(&a), Some(&b)) = (self.id_to_index.get(source_node), self.id_to_index.get(target_node)) else {
            return Ok(());
        };
        let pair = (a.min(b), a.max(b));
        if a != b && self.edge_set.insert(pair) {
            if self.edges.len() >= PUZZLE2D_FORCE_MAX_EDGES {
                return Err(Fault::from("puzzle2d-force-edge-capacity"));
            }
            self.edges.push(pair);
        }
        Ok(())
    }

    fn seed_one(&mut self) {
        if self.seed_cursor >= self.positions.len() {
            self.center = [0.0, 0.0];
            self.stage = Puzzle2dForceStage::Center;
            return;
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
    }

    fn center_one(&mut self) {
        if let Some(position) = self.positions.get(self.center_cursor) {
            self.center[0] += position[0];
            self.center[1] += position[1];
            self.center_cursor += 1;
            return;
        }
        if !self.positions.is_empty() {
            self.center[0] /= self.positions.len() as f64;
            self.center[1] /= self.positions.len() as f64;
        }
        self.stage = Puzzle2dForceStage::Reset;
        self.force_cursor = 0;
    }

    fn reset_one(&mut self) {
        if self.force_cursor < self.forces.len() {
            self.forces[self.force_cursor] = [0.0, 0.0];
            self.force_cursor += 1;
            return;
        }
        self.pair_i = 0;
        self.pair_j = 1;
        self.stage = Puzzle2dForceStage::Repel;
    }

    fn repel_one(&mut self, iterations: u32) {
        let count = self.positions.len();
        if self.pair_i >= count || self.pair_j >= count {
            self.edge_cursor = 0;
            self.stage = Puzzle2dForceStage::Springs;
            return;
        }
        let i = self.pair_i;
        let j = self.pair_j;
        let dx = self.positions[j][0] - self.positions[i][0];
        let dy = self.positions[j][1] - self.positions[i][1];
        let distance = dx.hypot(dy).max(1e-4);
        let repulsion = 6500.0 * self.cool(iterations) * (self.radii[i] * self.radii[j]).max(1.0) / (distance * distance);
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
    }

    fn spring_one(&mut self, iterations: u32) {
        let Some(&(i, j)) = self.edges.get(self.edge_cursor) else {
            self.force_cursor = 0;
            self.stage = Puzzle2dForceStage::Integrate;
            return;
        };
        let dx = self.positions[j][0] - self.positions[i][0];
        let dy = self.positions[j][1] - self.positions[i][1];
        let distance = dx.hypot(dy).max(1e-4);
        let magnitude = 0.028 * self.cool(iterations) * (distance - 140.0);
        let fx = dx / distance * magnitude;
        let fy = dy / distance * magnitude;
        self.forces[i][0] += fx;
        self.forces[i][1] += fy;
        self.forces[j][0] -= fx;
        self.forces[j][1] -= fy;
        self.edge_cursor += 1;
    }

    fn integrate_one(&mut self, iterations: u32) {
        if self.force_cursor >= self.positions.len() {
            self.iteration += 1;
            self.force_cursor = 0;
            self.stage = if self.iteration >= iterations { Puzzle2dForceStage::Emit } else { Puzzle2dForceStage::Reset };
            return;
        }
        let index = self.force_cursor;
        let delta_time = 0.85 * self.cool(iterations).sqrt();
        let mut velocity = [(self.velocities[index][0] + self.forces[index][0] * delta_time) * 0.88, (self.velocities[index][1] + self.forces[index][1] * delta_time) * 0.88];
        let speed = velocity[0].hypot(velocity[1]);
        if speed > 48.0 {
            velocity[0] *= 48.0 / speed;
            velocity[1] *= 48.0 / speed;
        }
        self.velocities[index] = velocity;
        self.positions[index][0] += velocity[0] * delta_time;
        self.positions[index][1] += velocity[1] * delta_time;
        self.force_cursor += 1;
    }

    fn emit_one(&mut self) -> bool {
        let index = self.force_cursor;
        let Some(position) = self.positions.get(index).copied() else {
            self.stage = Puzzle2dForceStage::Complete;
            return true;
        };
        if self.original[index] != Some(position) {
            self.mutations.push(crate::artifacts::puzzle2d::mutations::move_node(self.node_ids[index].clone(), position[0], position[1]));
        }
        self.force_cursor += 1;
        false
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dForceLayoutWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    /// 📐️ One unit of the declared extent is one `step()` call, not one force evaluation: the
    /// prologue scans `nodes + handles + edges` in [`PUZZLE2D_FORCE_SCAN_PER_STEP`] chunks, each of
    /// the [`puzzle2d_force_iterations`] iterations costs one reset pass, one integrate pass, and the
    /// repulsion/spring chunks, and the epilogue emits one mutation pass. Handles are counted here
    /// too, so a document that would trip `PUZZLE2D_FORCE_MAX_HANDLES` mid-run is refused at
    /// preflight instead.
    fn extent(&self, command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        if command.action_id() != self.tool_id || snapshot.0.get("schema").and_then(Value::as_str) != Some(PUZZLE2D_FIXTURE_SCHEMA) {
            return None;
        }
        let nodes = snapshot.0.get("nodes")?.as_array()?;
        let edges = snapshot.0.get("edges").and_then(Value::as_array).map_or(0, Vec::len);
        let handles = nodes.iter().filter_map(|node| node.get("handles")).filter_map(Value::as_array).map(Vec::len).sum::<usize>();
        if nodes.len() > PUZZLE2D_FORCE_MAX_NODES || edges > PUZZLE2D_FORCE_MAX_EDGES || handles > PUZZLE2D_FORCE_MAX_HANDLES {
            return None;
        }
        let pairs = usize::try_from(puzzle2d_force_pairs(nodes.len())).ok()?;
        let node_pass = nodes.len().div_ceil(PUZZLE2D_FORCE_NODES_PER_STEP).max(1);
        let iteration = pairs.div_ceil(PUZZLE2D_FORCE_UNITS_PER_STEP).checked_add(edges.div_ceil(PUZZLE2D_FORCE_UNITS_PER_STEP))?.checked_add(node_pass.checked_mul(2)?)?.checked_add(PUZZLE2D_FORCE_ITERATION_STAGE_STEPS)?;
        let items = nodes
            .len()
            .checked_add(handles)?
            .checked_add(edges)?
            .div_ceil(PUZZLE2D_FORCE_SCAN_PER_STEP)
            .checked_add(node_pass.checked_mul(3)?)?
            .checked_add(PUZZLE2D_FORCE_PROLOGUE_STAGE_STEPS)?
            .checked_add(iteration.checked_mul(usize::try_from(puzzle2d_force_iterations(nodes.len(), edges)).ok()?)?)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items.max(1))
    }

    /// 🍰️ Each call drains one chunk of the current stage and yields a progress boundary, so the
    /// number of `step()` calls matches [`Puzzle2dForceLayoutWork::extent`] instead of running one
    /// force evaluation per checkpoint.
    fn step(
        &mut self,
        _command: &Puzzle2dCommand,
        snapshot: &Puzzle2dPlaySnapshot,
        _config: &Puzzle2dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        let nodes = snapshot.0.get("nodes").and_then(Value::as_array).ok_or_else(|| Fault::from("puzzle2d-force-nodes-missing"))?;
        let edges = snapshot.0.get("edges").and_then(Value::as_array);
        let iterations = puzzle2d_force_iterations(nodes.len(), edges.map_or(0, Vec::len));
        match self.stage {
            Puzzle2dForceStage::Nodes | Puzzle2dForceStage::Handles => {
                for _ in 0..PUZZLE2D_FORCE_SCAN_PER_STEP {
                    match self.stage {
                        Puzzle2dForceStage::Nodes => self.scan_node_one(nodes)?,
                        Puzzle2dForceStage::Handles => self.scan_handle_one(nodes)?,
                        _ => break,
                    }
                }
                Ok(Self::progress("puzzle2d-force-node", "Reading layout node", "Layout-Knoten wird gelesen"))
            }
            Puzzle2dForceStage::Edges => {
                for _ in 0..PUZZLE2D_FORCE_SCAN_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Edges {
                        break;
                    }
                    self.scan_edge_one(edges)?;
                }
                Ok(Self::progress("puzzle2d-force-edge", "Indexing layout edge", "Layout-Kante wird indiziert"))
            }
            Puzzle2dForceStage::Seed => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Seed {
                        break;
                    }
                    self.seed_one();
                }
                Ok(Self::progress("puzzle2d-force-seed", "Seeding layout node", "Layout-Knoten wird initialisiert"))
            }
            Puzzle2dForceStage::Center => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Center {
                        break;
                    }
                    self.center_one();
                }
                Ok(Self::progress("puzzle2d-force-center", "Centering layout", "Layout wird zentriert"))
            }
            Puzzle2dForceStage::Reset => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Reset {
                        break;
                    }
                    self.reset_one();
                }
                Ok(Self::progress("puzzle2d-force-reset", "Resetting layout force", "Layout-Kraft wird zurückgesetzt"))
            }
            Puzzle2dForceStage::Repel => {
                for _ in 0..PUZZLE2D_FORCE_UNITS_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Repel {
                        break;
                    }
                    self.repel_one(iterations);
                }
                Ok(Self::progress("puzzle2d-force-repel", "Applying layout repulsion", "Layout-Abstoßung wird angewendet"))
            }
            Puzzle2dForceStage::Springs => {
                for _ in 0..PUZZLE2D_FORCE_UNITS_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Springs {
                        break;
                    }
                    self.spring_one(iterations);
                }
                Ok(Self::progress("puzzle2d-force-spring", "Applying layout spring", "Layout-Feder wird angewendet"))
            }
            Puzzle2dForceStage::Integrate => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Integrate {
                        break;
                    }
                    self.integrate_one(iterations);
                }
                Ok(Self::progress("puzzle2d-force-integrate", "Integrating layout node", "Layout-Knoten wird integriert"))
            }
            Puzzle2dForceStage::Emit => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.emit_one() {
                        let mutations = std::mem::take(&mut self.mutations);
                        return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: mutations, ui_scope: UiDirtyScope::Full, ..Default::default() }));
                    }
                }
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

/// 🕸️ Admission ceilings and chunk size for one `redrawHandles` run. The legacy handler serialized
/// the entire fixture and called `apply_edge_handle_snap_to_fixture_v1_json` once, with no size
/// guard and no yield point; these bound the same three passes so a large board is refused at
/// preflight rather than blocking a step.
const PUZZLE2D_REDRAW_MAX_NODES: usize = 4_096;
const PUZZLE2D_REDRAW_MAX_EDGES: usize = 4_096;
const PUZZLE2D_REDRAW_MAX_HANDLES: usize = 8_192;
const PUZZLE2D_REDRAW_UNITS_PER_STEP: usize = 256;
const PUZZLE2D_REDRAW_STAGE_STEPS: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle2dRedrawStage {
    Nodes,
    Handles,
    Edges,
    Emit,
    Complete,
    Closing,
}

/// 🔵️ The snap geometry of one node, mirroring the engine's own `NodeShapeSnap`: a node without a
/// finite centre — or a circle without a radius, or a rectangle without extents — has no rim to
/// snap onto and is skipped, exactly as `apply_edge_handle_snap_to_fixture_v1_value` skips it.
#[derive(Clone, Copy)]
struct Puzzle2dRedrawShape {
    center: [f64; 2],
    rectangle: Option<[f64; 2]>,
}

/// 🕸️ `redrawHandles` as a bounded, resumable walk. `Nodes`/`Handles` index every visible handle to
/// its owning node, `Edges` resolves each visible edge into one angle per endpoint (last edge wins
/// on a shared handle, matching the engine), and `Emit` publishes one `replace-node-handle` per
/// handle whose angle actually changed — the same delta the legacy whole-fixture call produced
/// through [`puzzle2d_document_delta_operations`], minus the unbounded step.
struct Puzzle2dRedrawHandlesWork {
    stage: Puzzle2dRedrawStage,
    node_cursor: usize,
    handle_cursor: usize,
    edge_cursor: usize,
    shapes: Vec<Option<Puzzle2dRedrawShape>>,
    handle_locations: HashMap<String, (usize, usize)>,
    angles: std::collections::BTreeMap<(usize, usize), f64>,
    mutations: Vec<Puzzle2dMutation>,
    retained_bytes: usize,
}

impl Default for Puzzle2dRedrawHandlesWork {
    fn default() -> Self {
        Self {
            stage: Puzzle2dRedrawStage::Nodes,
            node_cursor: 0,
            handle_cursor: 0,
            edge_cursor: 0,
            shapes: Vec::with_capacity(PUZZLE2D_REDRAW_MAX_NODES),
            handle_locations: HashMap::with_capacity(PUZZLE2D_REDRAW_MAX_HANDLES),
            angles: std::collections::BTreeMap::new(),
            mutations: Vec::with_capacity(PUZZLE2D_REDRAW_MAX_HANDLES),
            retained_bytes: 0,
        }
    }
}

impl Puzzle2dRedrawHandlesWork {
    fn shape(object: &serde_json::Map<String, Value>) -> Option<Puzzle2dRedrawShape> {
        let cx = object.get("x").and_then(Value::as_f64)?;
        let cy = object.get("y").and_then(Value::as_f64)?;
        if object.get("shape").and_then(Value::as_str) == Some("rectangle") {
            let width = object.get("width").and_then(Value::as_f64)?;
            let height = object.get("height").and_then(Value::as_f64)?;
            return Some(Puzzle2dRedrawShape { center: [cx, cy], rectangle: Some([width, height]) });
        }
        object.get("radius").and_then(Value::as_f64)?;
        Some(Puzzle2dRedrawShape { center: [cx, cy], rectangle: None })
    }

    /// 🧭️ Delegates to the board engine's own conventions — east-zero for circles, north-zero for
    /// rectangles — so a chunked redraw and the engine's whole-fixture snap agree numerically.
    fn angle_toward(from: Puzzle2dRedrawShape, toward: Puzzle2dRedrawShape) -> Option<f64> {
        let center = crate::editor::puzzle2d::engine::Point::new(from.center[0], from.center[1]);
        let target = crate::editor::puzzle2d::engine::Point::new(toward.center[0], toward.center[1]);
        if crate::editor::puzzle2d::engine::distance_between(center, target) <= 1e-9 {
            return None;
        }
        Some(match from.rectangle {
            Some([width, height]) => crate::editor::puzzle2d::engine::rectangle_handle_angle_toward(center, width, height, target),
            None => crate::editor::puzzle2d::engine::circle_handle_angle_toward(center, target),
        })
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle2dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn admit_bytes(&mut self, bytes: usize) -> Result<(), Fault> {
        let next = self.retained_bytes.checked_add(bytes).ok_or_else(|| Fault::from("puzzle2d-redraw-byte-capacity"))?;
        if next > crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES {
            return Err(Fault::from("puzzle2d-redraw-byte-capacity"));
        }
        self.retained_bytes = next;
        Ok(())
    }

    fn index_node_one(&mut self, nodes: &[Value]) -> Result<(), Fault> {
        let Some(node) = nodes.get(self.node_cursor) else {
            self.edge_cursor = 0;
            self.stage = Puzzle2dRedrawStage::Edges;
            return Ok(());
        };
        let object = node.as_object().ok_or_else(|| Fault::from("puzzle2d-redraw-node-not-object"))?;
        if self.shapes.len() >= PUZZLE2D_REDRAW_MAX_NODES {
            return Err(Fault::from("puzzle2d-redraw-node-capacity"));
        }
        let visible = crate::editor::puzzle2d::engine::board_json_visible_or_true(object);
        self.shapes.push(visible.then(|| Self::shape(object)).flatten());
        if !visible {
            self.node_cursor += 1;
            return Ok(());
        }
        self.handle_cursor = 0;
        self.stage = Puzzle2dRedrawStage::Handles;
        Ok(())
    }

    fn index_handle_one(&mut self, nodes: &[Value]) -> Result<(), Fault> {
        let object = nodes.get(self.node_cursor).and_then(Value::as_object).ok_or_else(|| Fault::from("puzzle2d-redraw-node-owner-lost"))?;
        let Some(handle) = object.get("handles").and_then(Value::as_array).and_then(|handles| handles.get(self.handle_cursor)) else {
            self.node_cursor += 1;
            self.stage = Puzzle2dRedrawStage::Nodes;
            return Ok(());
        };
        let index = self.handle_cursor;
        self.handle_cursor += 1;
        let Some(handle_object) = handle.as_object() else { return Ok(()) };
        if !crate::editor::puzzle2d::engine::board_json_visible_or_true(handle_object) {
            return Ok(());
        }
        let Some(handle_id) = handle_object.get("id").and_then(Value::as_str) else { return Ok(()) };
        if self.handle_locations.len() >= PUZZLE2D_REDRAW_MAX_HANDLES {
            return Err(Fault::from("puzzle2d-redraw-handle-capacity"));
        }
        self.admit_bytes(handle_id.len())?;
        self.handle_locations.insert(handle_id.to_string(), (self.node_cursor, index));
        Ok(())
    }

    fn resolve_edge_one(&mut self, edges: Option<&Vec<Value>>) -> Result<(), Fault> {
        let Some(edge) = edges.and_then(|edges| edges.get(self.edge_cursor)) else {
            self.stage = Puzzle2dRedrawStage::Emit;
            return Ok(());
        };
        self.edge_cursor += 1;
        let Some(object) = edge.as_object() else { return Ok(()) };
        if !crate::editor::puzzle2d::engine::board_json_visible_or_true(object) {
            return Ok(());
        }
        let Some((source, target)) = crate::editor::puzzle2d::engine::fixture_edge_handle_ids_from_object(object) else { return Ok(()) };
        let (Some(&source_location), Some(&target_location)) = (self.handle_locations.get(source), self.handle_locations.get(target)) else {
            return Ok(());
        };
        let (Some(Some(source_shape)), Some(Some(target_shape))) = (self.shapes.get(source_location.0).copied(), self.shapes.get(target_location.0).copied()) else {
            return Ok(());
        };
        if let Some(angle) = Self::angle_toward(source_shape, target_shape) {
            self.angles.insert(source_location, angle);
        }
        if let Some(angle) = Self::angle_toward(target_shape, source_shape) {
            self.angles.insert(target_location, angle);
        }
        Ok(())
    }

    fn publish_handle_one(&mut self, nodes: &[Value]) -> Result<bool, Fault> {
        let Some(((node_index, handle_index), angle)) = self.angles.pop_first() else {
            self.stage = Puzzle2dRedrawStage::Complete;
            return Ok(true);
        };
        let object = nodes.get(node_index).and_then(Value::as_object).ok_or_else(|| Fault::from("puzzle2d-redraw-node-owner-lost"))?;
        let node_id = object.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle2d-redraw-node-id-missing"))?;
        let handle = object.get("handles").and_then(Value::as_array).and_then(|handles| handles.get(handle_index)).ok_or_else(|| Fault::from("puzzle2d-redraw-handle-owner-lost"))?;
        let original = <crate::artifacts::puzzle2d::Puzzle2dHandle as dsl::FromValue>::from_value(dsl::DslValue::from(handle)).map_err(|_| Fault::from("puzzle2d-redraw-handle-malformed"))?;
        let mut next = original.clone();
        next.angle = angle;
        if next == original {
            return Ok(false);
        }
        self.admit_bytes(node_id.len().saturating_add(next.id.len()))?;
        self.mutations.push(crate::artifacts::puzzle2d::mutations::replace_node_handle(node_id.to_string(), next.id.clone(), next));
        Ok(false)
    }

    fn pop_one(&mut self) -> bool {
        if self.mutations.pop().is_some() {
            return true;
        }
        if self.angles.pop_first().is_some() {
            return true;
        }
        if self.shapes.pop().is_some() {
            return true;
        }
        if let Some(key) = self.handle_locations.keys().next().cloned() {
            self.handle_locations.remove(&key);
            return true;
        }
        false
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dRedrawHandlesWork {
    fn tool_id(&self) -> &'static str {
        "redrawHandles"
    }

    /// 📐️ One index chunk per [`PUZZLE2D_REDRAW_UNITS_PER_STEP`] node-or-handle records, one chunk
    /// per that many edges, one chunk per that many published handles, plus one stage handover each.
    fn extent(&self, command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        if command.action_id() != "redrawHandles" || snapshot.0.get("schema").and_then(Value::as_str) != Some(PUZZLE2D_FIXTURE_SCHEMA) {
            return None;
        }
        let nodes = snapshot.0.get("nodes")?.as_array()?;
        let edges = snapshot.0.get("edges").and_then(Value::as_array).map_or(0, Vec::len);
        let handles = nodes.iter().filter_map(|node| node.get("handles")).filter_map(Value::as_array).map(Vec::len).sum::<usize>();
        if nodes.len() > PUZZLE2D_REDRAW_MAX_NODES || edges > PUZZLE2D_REDRAW_MAX_EDGES || handles > PUZZLE2D_REDRAW_MAX_HANDLES {
            return None;
        }
        let items = nodes
            .len()
            .checked_add(handles)?
            .div_ceil(PUZZLE2D_REDRAW_UNITS_PER_STEP)
            .checked_add(edges.div_ceil(PUZZLE2D_REDRAW_UNITS_PER_STEP))?
            .checked_add(handles.div_ceil(PUZZLE2D_REDRAW_UNITS_PER_STEP))?
            .checked_add(PUZZLE2D_REDRAW_STAGE_STEPS)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items.max(1))
    }

    fn step(
        &mut self,
        _command: &Puzzle2dCommand,
        snapshot: &Puzzle2dPlaySnapshot,
        _config: &Puzzle2dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        let nodes = snapshot.0.get("nodes").and_then(Value::as_array).ok_or_else(|| Fault::from("puzzle2d-redraw-nodes-missing"))?;
        match self.stage {
            Puzzle2dRedrawStage::Nodes | Puzzle2dRedrawStage::Handles => {
                for _ in 0..PUZZLE2D_REDRAW_UNITS_PER_STEP {
                    match self.stage {
                        Puzzle2dRedrawStage::Nodes => self.index_node_one(nodes)?,
                        Puzzle2dRedrawStage::Handles => self.index_handle_one(nodes)?,
                        _ => break,
                    }
                }
                Ok(Self::progress("puzzle2d-redraw-node", "Indexing board node", "Board-Knoten wird indiziert"))
            }
            Puzzle2dRedrawStage::Edges => {
                let edges = snapshot.0.get("edges").and_then(Value::as_array);
                for _ in 0..PUZZLE2D_REDRAW_UNITS_PER_STEP {
                    if self.stage != Puzzle2dRedrawStage::Edges {
                        break;
                    }
                    self.resolve_edge_one(edges)?;
                }
                Ok(Self::progress("puzzle2d-redraw-edge", "Resolving handle angle", "Anschlusswinkel wird bestimmt"))
            }
            Puzzle2dRedrawStage::Emit => {
                for _ in 0..PUZZLE2D_REDRAW_UNITS_PER_STEP {
                    if self.publish_handle_one(nodes)? {
                        let mutations = std::mem::take(&mut self.mutations);
                        return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: mutations, ui_scope: UiDirtyScope::Full, ..Default::default() }));
                    }
                }
                Ok(Self::progress("puzzle2d-redraw-emit", "Publishing handle angle", "Anschlusswinkel wird veröffentlicht"))
            }
            Puzzle2dRedrawStage::Complete => Err(Fault::from("puzzle2d-redraw-complete-repolled")),
            Puzzle2dRedrawStage::Closing => Err(Fault::from("puzzle2d-redraw-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle2dRedrawStage::Closing;
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
        self.stage == Puzzle2dRedrawStage::Closing && self.shapes.is_empty() && self.handle_locations.is_empty() && self.angles.is_empty() && self.mutations.is_empty()
    }
}
//#endregion 🧵️RetainedCommands

//#region 🧵️ReservedJobs
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, JobFault, JobPayloadStream, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::app::ArtifactToolCompletionRejection;
use semio_framework_plugin::{ArtifactReservedJob, ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion, EphemeralEmit, MediaPayload, PluginCloseStep};

/// 🔌️ `import-media`'s own route cap. A `Media` reserved-tool input never travels through
/// `RetainedToolWireInput` — the framework builds it with an empty `raw_wire` — so this bounds the
/// admitted structured fragment rather than a wire payload, and is unrelated to the 8,192-byte
/// `crate::retained_command::PUZZLE_COMMAND_RAW_BYTES` the four retained verbs share.
const PUZZLE2D_IMPORT_RAW_BYTES: usize = 65_536;
/// 🔢️ Fixed per-collection descriptor budget for one `kit:in` fragment — an oversized collection is
/// refused outright rather than silently truncated.
const PUZZLE2D_IMPORT_SEMANTIC_ITEMS: usize = 64;
const PUZZLE2D_IMPORT_DECODED_ITEMS: usize = 4_096;
const PUZZLE2D_IMPORT_WORK_UNITS: u64 = 4_096;
const PUZZLE2D_IMPORT_OUTPUT_BYTES: usize = 1_048_576;
/// 🧬️ At most one `connect-kind-compatibility` per admitted relation row, plus the single
/// `replace-kind-catalogs` that carries the merged bundle.
const PUZZLE2D_IMPORT_MUTATION_ITEMS: usize = PUZZLE2D_IMPORT_SEMANTIC_ITEMS + 1;
const PUZZLE2D_IMPORT_TOOL_ID: &str = "import-media";
const PUZZLE2D_IMPORT_PORT: &str = "kit:in";
const PUZZLE2D_IMPORT_PAYLOAD_SCHEMA: &str = "puzzle.2d.reserved.import-media.v1";
/// 🗂️ The exact root keys a `kit.catalog` fragment may carry — the shape
/// `Block2dPlayApp::export_media("catalog:out")` produces via
/// `crate::artifacts::block2d::schema::inferences::puzzle2d_manifest_fragment`, which is puzzle2d's
/// own manifest vocabulary (`s/plugin/puzzle/app/2d/manifest/🔣️.json`), not block3d's.
const PUZZLE2D_IMPORT_ROOT_KEYS: &[&str] = &["schema", "id", "name", "axes", "portKinds", "wireKinds", "edgeKinds", "nodeKinds", "kindCompatibility"];
const PUZZLE2D_IMPORT_COLLECTIONS: &[&str] = &["portKinds", "wireKinds", "edgeKinds", "nodeKinds", "kindCompatibility"];

/// 🏭️ puzzle2d's app-owned `import-media` factory — the root policy's importer-cohort census
/// (`📊️p8yj-importer-cohorts.json`) requires every `ArtifactApp` owning an inbound media port to own
/// an explicit resumable importer; the framework registers none on an app's behalf, so without this
/// registration `qualified_tool_proof("import-media")` never resolves and every `kit:in` delivery
/// fails closed.
struct Puzzle2dImportJobFactory {
    keys: [ToolFactoryKey; 1],
}

impl Puzzle2dImportJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: [ToolFactoryKey::new(controller_id, PUZZLE2D_IMPORT_TOOL_ID)] }
    }
}

impl ToolJobFactory for Puzzle2dImportJobFactory {
    type Payload = ArtifactReservedToolJob;
    type Job = ArtifactReservedToolJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PUZZLE2D_IMPORT_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(PUZZLE2D_IMPORT_RAW_BYTES, PUZZLE2D_IMPORT_DECODED_ITEMS, PUZZLE2D_IMPORT_WORK_UNITS, PUZZLE2D_IMPORT_OUTPUT_BYTES, 7_500, 1, 1)
    }

    fn create_job(&mut self, operation: semio_framework_job::Operation, mut payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        payload.bind_operation(operation)?;
        Ok(payload)
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Puzzle2dImportJobFactory {
    type Owner = EditorApp<Puzzle2dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = &[PUZZLE2D_IMPORT_TOOL_ID];
    const DOCUMENT_SCHEMA: &'static str = PUZZLE2D_FIXTURE_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: PUZZLE2D_IMPORT_TOOL_ID, lanes: &[ArtifactToolPublicationLane::Artifact] }];
}

fn puzzle2d_job_payload(cx: &mut StepContext<'_>, stream: JobPayloadStream, bytes: &[u8]) -> RetainedJobPayload {
    match cx.payload_from_bytes(stream, bytes) {
        Ok(payload) => payload,
        Err(rejected) => {
            drop(rejected.into_source());
            RetainedJobPayload::empty(stream)
        }
    }
}

fn puzzle2d_job_fault(cx: &mut StepContext<'_>, detail: &str) -> StepOutcome {
    let bytes = detail.as_bytes();
    let bounded = &bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)];
    StepOutcome::Fault(JobFault { detail: puzzle2d_job_payload(cx, JobPayloadStream::Fault, bounded) })
}

fn puzzle2d_import_checkpoint(stage: u8, cursor: usize, decoded_items: usize, progress: u64, cx: &mut StepContext<'_>) -> StepOutcome {
    let mut state = [0u8; 25];
    state[0] = stage;
    state[1..9].copy_from_slice(&(cursor as u64).to_le_bytes());
    state[9..17].copy_from_slice(&(decoded_items as u64).to_le_bytes());
    state[17..25].copy_from_slice(&progress.to_le_bytes());
    StepOutcome::CheckpointReady(Checkpoint { state: puzzle2d_job_payload(cx, JobPayloadStream::CheckpointState, &state), applied_progress: progress })
}

fn puzzle2d_retire_string_step(owner: &mut String, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    if let Some(bytes) = owner.chars().next_back().map(char::len_utf8) {
        if bytes > maximum_bytes {
            return Ok(Some(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }));
        }
        owner.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }));
    }
    if owner.capacity() == 0 {
        return Ok(None);
    }
    let bytes = owner.capacity();
    if bytes > maximum_bytes {
        return Err(Fault::from("puzzle2d import string backing exceeds its bounded disposal byte slice"));
    }
    *owner = String::new();
    Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }))
}

fn puzzle2d_retire_vec_backing<T>(owners: &mut Vec<T>, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    if !owners.is_empty() || owners.capacity() == 0 {
        return Ok(None);
    }
    let bytes = owners.capacity().saturating_mul(size_of::<T>());
    if bytes > maximum_bytes {
        return Err(Fault::from("puzzle2d import vector backing exceeds its bounded disposal byte slice"));
    }
    *owners = Vec::new();
    Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }))
}

fn puzzle2d_import_text(row: &Value, key: &str) -> String {
    row.get(key).and_then(Value::as_str).unwrap_or_default().to_string()
}

fn puzzle2d_import_identity(row: &Value) -> Option<&str> {
    row.get("id").and_then(Value::as_str).filter(|id| !id.trim().is_empty())
}

fn puzzle2d_import_presentation<'a>(row: &'a Value, key: &str) -> Option<&'a Value> {
    row.get("presentation").and_then(|presentation| presentation.get(key))
}

fn puzzle2d_import_presentation_text(row: &Value, key: &str) -> String {
    puzzle2d_import_presentation(row, key).and_then(Value::as_str).unwrap_or_default().to_string()
}

/// 🏷️ Manifest rows carry `name` and no `label`; the catalog row wants both, so an absent label
/// mirrors the name rather than inventing a third spelling.
fn puzzle2d_import_label(row: &Value, name: &str) -> String {
    row.get("label").and_then(Value::as_str).filter(|label| !label.trim().is_empty()).map_or_else(|| name.to_string(), str::to_string)
}

fn puzzle2d_import_handle_template(node_kind_id: &str, index: usize, template: &Value) -> Option<crate::artifacts::puzzle2d::Puzzle2dHandleTemplate> {
    let handle_kind = template.get("handleKind").and_then(Value::as_str).filter(|kind| !kind.trim().is_empty())?;
    let name = puzzle2d_import_text(template, "name");
    Some(crate::artifacts::puzzle2d::Puzzle2dHandleTemplate {
        id: puzzle2d_import_identity(template).map_or_else(|| format!("{node_kind_id}-h{index}"), str::to_string),
        label: puzzle2d_import_label(template, &name),
        name,
        description: puzzle2d_import_text(template, "description"),
        icon: puzzle2d_import_text(template, "icon"),
        handle_kind: Some(handle_kind.to_string()),
        angle: template.get("angle").and_then(Value::as_f64).filter(|angle| angle.is_finite()).unwrap_or_default(),
        t: template.get("t").and_then(Value::as_f64).filter(|t| t.is_finite()),
        mandatory: template.get("mandatory").and_then(Value::as_bool),
        radius: template.get("radius").and_then(Value::as_f64).filter(|radius| radius.is_finite()),
    })
}

fn puzzle2d_import_node_kind(row: &Value) -> Option<crate::artifacts::puzzle2d::Puzzle2dCatalogNodeKind> {
    let id = puzzle2d_import_identity(row)?;
    let name = row.get("name").and_then(Value::as_str).unwrap_or(id).to_string();
    let handles = puzzle2d_import_presentation(row, "handles").and_then(Value::as_array).map_or_else(Vec::new, |templates| templates.iter().enumerate().filter_map(|(index, template)| puzzle2d_import_handle_template(id, index, template)).collect());
    Some(crate::artifacts::puzzle2d::Puzzle2dCatalogNodeKind {
        id: id.to_string(),
        label: puzzle2d_import_label(row, &name),
        name,
        description: puzzle2d_import_text(row, "description"),
        icon: puzzle2d_import_presentation_text(row, "icon"),
        image: puzzle2d_import_presentation_text(row, "image"),
        unit: puzzle2d_import_text(row, "unit"),
        is_abstract: row.get("abstract").and_then(Value::as_bool).unwrap_or_default(),
        base_kinds: Vec::new(),
        representations: Vec::new(),
        handles,
        attributes: Vec::new(),
        authors: Vec::new(),
    })
}

fn puzzle2d_import_handle_kind(row: &Value) -> Option<crate::artifacts::puzzle2d::Puzzle2dCatalogHandleKind> {
    let id = puzzle2d_import_identity(row)?;
    Some(crate::artifacts::puzzle2d::Puzzle2dCatalogHandleKind {
        id: id.to_string(),
        code: None,
        label: Some(puzzle2d_import_label(row, row.get("name").and_then(Value::as_str).unwrap_or(id))),
        order: None,
        compatible_with: Vec::new(),
        description: puzzle2d_import_text(row, "description"),
        icon: puzzle2d_import_presentation_text(row, "icon"),
        color: puzzle2d_import_presentation_text(row, "color"),
        default_wire_kind: puzzle2d_import_presentation_text(row, "defaultWireKind"),
    })
}

fn puzzle2d_import_edge_kind(row: &Value) -> Option<crate::artifacts::puzzle2d::Puzzle2dCatalogEdgeKind> {
    let id = puzzle2d_import_identity(row)?;
    let name = row.get("name").and_then(Value::as_str).unwrap_or(id).to_string();
    Some(crate::artifacts::puzzle2d::Puzzle2dCatalogEdgeKind {
        id: id.to_string(),
        label: puzzle2d_import_label(row, &name),
        name,
        description: puzzle2d_import_text(row, "description"),
        icon: puzzle2d_import_presentation_text(row, "icon"),
        color: puzzle2d_import_presentation_text(row, "color"),
    })
}

fn puzzle2d_import_wire_kind(row: &Value) -> Option<crate::artifacts::puzzle2d::Puzzle2dCatalogWireKind> {
    let id = puzzle2d_import_identity(row)?;
    let name = row.get("name").and_then(Value::as_str).unwrap_or(id).to_string();
    Some(crate::artifacts::puzzle2d::Puzzle2dCatalogWireKind {
        id: id.to_string(),
        label: puzzle2d_import_label(row, &name),
        name,
        description: puzzle2d_import_text(row, "description"),
        icon: puzzle2d_import_presentation_text(row, "icon"),
        color: puzzle2d_import_presentation_text(row, "color"),
        default_edge_kind: puzzle2d_import_presentation_text(row, "defaultEdgeKind"),
    })
}

/// 🗂️ Id-keyed upsert — deterministic and order-independent, so a `multiplicity: Many` port that fans
/// in several catalog producers converges on the same bundle whatever order they arrive in. Reports
/// whether the bundle actually changed, so an idempotent re-delivery emits no operation at all.
fn puzzle2d_upsert_catalog_row<T: PartialEq>(rows: &mut Vec<T>, incoming: T, matches: impl Fn(&T) -> bool) -> bool {
    match rows.iter().position(|row| matches(row)) {
        Some(index) if rows[index] == incoming => false,
        Some(index) => {
            rows[index] = incoming;
            true
        }
        None => {
            rows.push(incoming);
            true
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle2dImportStage {
    Decode,
    HandleKinds,
    NodeKinds,
    EdgeKinds,
    WireKinds,
    Compatibility,
    CatalogMutation,
    Complete,
}

/// 🎞️ `kit:in`'s concrete resumable importer: it normalizes one `kit.catalog` manifest fragment
/// (`portKinds`/`wireKinds`/`edgeKinds`/`nodeKinds`/`kindCompatibility`) into puzzle2d's own typed
/// `meta.kindCatalogs` vocabulary (`handles`/`wires`/`edges`/`nodes`) and its `meta.kindCompatibility`
/// table, one row per bounded step, and publishes the result as real undoable `Puzzle2dMutation`s —
/// never a direct state write. The producer this pairs with is `Block2dPlayApp`'s `"catalog:out"`
/// port, whose payload IS this exact 2d manifest shape (see
/// `🧱️block/🗿️artifacts/◻️2d/…/💡️inferences/🦀️.rs`'s `puzzle2d_manifest_fragment`).
struct Puzzle2dImportJob {
    port: String,
    media_json: Option<String>,
    snapshot: Option<std::sync::Arc<Puzzle2dPlaySnapshot>>,
    fragment: Option<Value>,
    catalogs: crate::artifacts::puzzle2d::Puzzle2dKindCatalogs,
    compatibility: Vec<crate::artifacts::puzzle2d::Puzzle2dKindCompatibility>,
    mutations: Vec<Puzzle2dMutation>,
    stage: Puzzle2dImportStage,
    cursor: usize,
    decoded_items: usize,
    progress: u64,
    catalog_changed: bool,
    completion: Option<ArtifactToolCompletion<EditorApp<Puzzle2dPlayApp>>>,
    pending_completion_rejection: Option<ArtifactToolCompletionRejection<EditorApp<Puzzle2dPlayApp>>>,
    completed: bool,
    closing: bool,
}

impl Puzzle2dImportJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>, port: String, media: Media) -> Self {
        let media_json = match media.payload {
            MediaPayload::Structured { json, .. } => Some(json),
            MediaPayload::Binary { .. } => None,
        };
        Self {
            port,
            media_json,
            snapshot: Some(request.snapshot),
            fragment: None,
            catalogs: crate::artifacts::puzzle2d::Puzzle2dKindCatalogs::default(),
            compatibility: Vec::new(),
            mutations: Vec::new(),
            stage: Puzzle2dImportStage::Decode,
            cursor: 0,
            decoded_items: 0,
            progress: 0,
            catalog_changed: false,
            completion: Some(request.completion),
            pending_completion_rejection: None,
            completed: false,
            closing: false,
        }
    }

    fn checkpoint(&self, cx: &mut StepContext<'_>) -> StepOutcome {
        puzzle2d_import_checkpoint(self.stage as u8, self.cursor, self.decoded_items, self.progress, cx)
    }

    fn rows(&self, key: &str) -> &[Value] {
        self.fragment.as_ref().and_then(|fragment| fragment.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
    }

    fn meta(&self, key: &str) -> Option<&Value> {
        self.snapshot.as_ref().and_then(|snapshot| snapshot.0.get("meta")).and_then(|meta| meta.get(key)).filter(|value| !value.is_null())
    }

    fn push_mutation(&mut self, mutation: Puzzle2dMutation) -> Result<(), &'static str> {
        if self.mutations.len() >= PUZZLE2D_IMPORT_MUTATION_ITEMS {
            return Err("puzzle2d kit:in mutation limit exceeded");
        }
        self.mutations.push(mutation);
        Ok(())
    }

    fn decode(&mut self, cx: &mut StepContext<'_>) -> Option<StepOutcome> {
        if self.port != PUZZLE2D_IMPORT_PORT {
            return Some(puzzle2d_job_fault(cx, "puzzle2d import only implements kit:in"));
        }
        let Some(media_json) = self.media_json.as_ref() else {
            return Some(puzzle2d_job_fault(cx, "puzzle2d kit:in requires a structured payload"));
        };
        let Ok(fragment) = serde_json::from_str::<Value>(media_json) else {
            return Some(puzzle2d_job_fault(cx, "puzzle2d kit:in payload is not valid json"));
        };
        if fragment.as_object().is_none() {
            return Some(puzzle2d_job_fault(cx, "puzzle2d kit:in root must be an object"));
        }
        if fragment.as_object().is_some_and(|root| root.keys().any(|key| !PUZZLE2D_IMPORT_ROOT_KEYS.contains(&key.as_str()))) {
            return Some(puzzle2d_job_fault(cx, "puzzle2d kit:in root contains an unknown field"));
        }
        if fragment.get("schema").is_some_and(|value| value.as_str() != Some("manifest")) {
            return Some(puzzle2d_job_fault(cx, "puzzle2d kit:in schema must be manifest when present"));
        }
        if PUZZLE2D_IMPORT_COLLECTIONS.iter().any(|key| fragment.get(*key).is_some_and(|value| value.as_array().is_none())) {
            return Some(puzzle2d_job_fault(cx, "puzzle2d kit:in collection must be an array"));
        }
        if PUZZLE2D_IMPORT_COLLECTIONS.iter().any(|key| fragment.get(*key).and_then(Value::as_array).is_some_and(|rows| rows.len() > PUZZLE2D_IMPORT_SEMANTIC_ITEMS)) {
            return Some(puzzle2d_job_fault(cx, "puzzle2d kit:in collection exceeds its fixed descriptor cap"));
        }
        let existing_catalogs = self.meta("kindCatalogs").cloned();
        let existing_compatibility = self.meta("kindCompatibility").and_then(|value| value.as_array().cloned()).unwrap_or_default();
        let fragment_items = PUZZLE2D_IMPORT_COLLECTIONS.iter().try_fold(0usize, |total, key| total.checked_add(fragment.get(*key).and_then(Value::as_array).map_or(0, Vec::len)));
        let catalog_items = existing_catalogs.as_ref().map(|catalogs| ["nodes", "handles", "edges", "wires"].into_iter().map(|slice| catalogs.get(slice).and_then(Value::as_array).map_or(0, Vec::len)).sum::<usize>()).unwrap_or_default();
        self.decoded_items = match fragment_items.and_then(|items| items.checked_add(catalog_items)).and_then(|items| items.checked_add(existing_compatibility.len())) {
            Some(items) if items <= PUZZLE2D_IMPORT_DECODED_ITEMS => items,
            _ => return Some(puzzle2d_job_fault(cx, "puzzle2d kit:in decoded item limit exceeded")),
        };
        self.catalogs = match existing_catalogs.as_ref() {
            Some(value) => match <crate::artifacts::puzzle2d::Puzzle2dKindCatalogs as dsl::FromValue>::from_value(dsl::DslValue::from(value)) {
                Ok(catalogs) => catalogs,
                Err(_) => return Some(puzzle2d_job_fault(cx, "puzzle2d kit:in cannot read the document's own kind catalogs")),
            },
            None => crate::artifacts::puzzle2d::Puzzle2dKindCatalogs::default(),
        };
        for row in &existing_compatibility {
            match <crate::artifacts::puzzle2d::Puzzle2dKindCompatibility as dsl::FromValue>::from_value(dsl::DslValue::from(row)) {
                Ok(parsed) => self.compatibility.push(parsed),
                Err(_) => return Some(puzzle2d_job_fault(cx, "puzzle2d kit:in cannot read the document's own kind relations")),
            }
        }
        self.catalog_changed = existing_catalogs.is_none() && PUZZLE2D_IMPORT_COLLECTIONS.iter().any(|key| fragment.get(*key).and_then(Value::as_array).is_some_and(|rows| !rows.is_empty()));
        self.fragment = Some(fragment);
        None
    }
}

impl InteractiveJob for Puzzle2dImportJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle2d_job_fault(cx, "puzzle2d import completion remains rejected");
        }
        match self.stage {
            Puzzle2dImportStage::Decode => {
                cx.set_stage("puzzle2d-import-decode");
                if let Some(outcome) = self.decode(cx) {
                    return outcome;
                }
                self.stage = Puzzle2dImportStage::HandleKinds;
                self.cursor = 0;
            }
            Puzzle2dImportStage::HandleKinds => {
                cx.set_stage("puzzle2d-import-handle-kind");
                let admitted = self.rows("portKinds").get(self.cursor).map(puzzle2d_import_handle_kind);
                match admitted {
                    Some(None) => return puzzle2d_job_fault(cx, "puzzle2d kit:in port kind is missing its identity"),
                    Some(Some(kind)) => {
                        let id = kind.id.clone();
                        let changed = puzzle2d_upsert_catalog_row(&mut self.catalogs.handles, kind, |row| row.id == id);
                        self.catalog_changed |= changed;
                        self.cursor += 1;
                    }
                    None => {
                        self.stage = Puzzle2dImportStage::NodeKinds;
                        self.cursor = 0;
                    }
                }
            }
            Puzzle2dImportStage::NodeKinds => {
                cx.set_stage("puzzle2d-import-node-kind");
                let admitted = self.rows("nodeKinds").get(self.cursor).map(puzzle2d_import_node_kind);
                match admitted {
                    Some(None) => return puzzle2d_job_fault(cx, "puzzle2d kit:in node kind is missing its identity"),
                    Some(Some(kind)) => {
                        let id = kind.id.clone();
                        let changed = puzzle2d_upsert_catalog_row(&mut self.catalogs.nodes, kind, |row| row.id == id);
                        self.catalog_changed |= changed;
                        self.cursor += 1;
                    }
                    None => {
                        self.stage = Puzzle2dImportStage::EdgeKinds;
                        self.cursor = 0;
                    }
                }
            }
            Puzzle2dImportStage::EdgeKinds => {
                cx.set_stage("puzzle2d-import-edge-kind");
                let admitted = self.rows("edgeKinds").get(self.cursor).map(puzzle2d_import_edge_kind);
                match admitted {
                    Some(None) => return puzzle2d_job_fault(cx, "puzzle2d kit:in edge kind is missing its identity"),
                    Some(Some(kind)) => {
                        let id = kind.id.clone();
                        let changed = puzzle2d_upsert_catalog_row(&mut self.catalogs.edges, kind, |row| row.id == id);
                        self.catalog_changed |= changed;
                        self.cursor += 1;
                    }
                    None => {
                        self.stage = Puzzle2dImportStage::WireKinds;
                        self.cursor = 0;
                    }
                }
            }
            Puzzle2dImportStage::WireKinds => {
                cx.set_stage("puzzle2d-import-wire-kind");
                let admitted = self.rows("wireKinds").get(self.cursor).map(puzzle2d_import_wire_kind);
                match admitted {
                    Some(None) => return puzzle2d_job_fault(cx, "puzzle2d kit:in wire kind is missing its identity"),
                    Some(Some(kind)) => {
                        let id = kind.id.clone();
                        let changed = puzzle2d_upsert_catalog_row(&mut self.catalogs.wires, kind, |row| row.id == id);
                        self.catalog_changed |= changed;
                        self.cursor += 1;
                    }
                    None => {
                        self.stage = Puzzle2dImportStage::Compatibility;
                        self.cursor = 0;
                    }
                }
            }
            Puzzle2dImportStage::Compatibility => {
                cx.set_stage("puzzle2d-import-kind-relation");
                let admitted = self.rows("kindCompatibility").get(self.cursor).cloned();
                let Some(row) = admitted else {
                    self.stage = Puzzle2dImportStage::CatalogMutation;
                    self.cursor = 0;
                    self.progress = self.progress.saturating_add(1);
                    cx.consume_fuel(1);
                    return self.checkpoint(cx);
                };
                let Ok(parsed) = <crate::artifacts::puzzle2d::Puzzle2dKindCompatibility as dsl::FromValue>::from_value(dsl::DslValue::from(&row)) else {
                    return puzzle2d_job_fault(cx, "puzzle2d kit:in kind relation is malformed");
                };
                let existing = self.compatibility.iter().position(|entry| entry.source == parsed.source && entry.target == parsed.target);
                if !existing.is_some_and(|index| self.compatibility[index] == parsed) {
                    match existing {
                        Some(index) => self.compatibility[index] = parsed.clone(),
                        None => self.compatibility.push(parsed.clone()),
                    }
                    if let Err(error) = self.push_mutation(crate::artifacts::puzzle2d::mutations::connect_kind_compatibility(parsed.source, parsed.target, parsed.bidirectional, parsed.important, parsed.specificity)) {
                        return puzzle2d_job_fault(cx, error);
                    }
                }
                self.cursor += 1;
            }
            Puzzle2dImportStage::CatalogMutation => {
                cx.set_stage("puzzle2d-import-kind-catalogs");
                if self.catalog_changed {
                    let mutation = crate::artifacts::puzzle2d::mutations::replace_kind_catalogs(Some(std::mem::take(&mut self.catalogs)));
                    if let Err(error) = self.push_mutation(mutation) {
                        return puzzle2d_job_fault(cx, error);
                    }
                    self.catalog_changed = false;
                }
                self.stage = Puzzle2dImportStage::Complete;
            }
            Puzzle2dImportStage::Complete => {
                cx.set_stage("puzzle2d-import-publish");
                if !self.completed {
                    let mutations = std::mem::take(&mut self.mutations);
                    let Some(completion) = self.completion.as_ref() else {
                        return puzzle2d_job_fault(cx, "puzzle2d import lost its completion authority");
                    };
                    if !completion.has_mounted_consumer() {
                        return puzzle2d_job_fault(cx, "puzzle2d import completion consumer is absent");
                    }
                    if let Err(rejected) = completion.complete(Ok(Emit { artifact_mutations: mutations, ui_scope: UiDirtyScope::Full, ..Default::default() }), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(rejected);
                        return puzzle2d_job_fault(cx, &message);
                    }
                    self.completed = true;
                }
                return StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) });
            }
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        self.checkpoint(cx)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        match ArtifactReservedJob::close_step(self, maximum_items, maximum_bytes) {
            Ok(PluginCloseStep::Pending { released_items, released_bytes }) => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
            Ok(PluginCloseStep::AwaitingInput { .. } | PluginCloseStep::Blocked { .. }) | Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
            Ok(PluginCloseStep::Complete) if ArtifactReservedJob::terminal_is_empty(self) => semio_framework_job::InteractiveJobCloseStep::Complete,
            Ok(PluginCloseStep::Complete) => semio_framework_job::InteractiveJobCloseStep::Blocked,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        ArtifactReservedJob::terminal_is_empty(self)
    }
}

impl ArtifactReservedJob for Puzzle2dImportJob {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        self.closing = true;
        if maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                if let Some(step) = emit.close_child_one(maximum_items, maximum_bytes) {
                    return Ok(step);
                }
            }
            self.pending_completion_rejection = None;
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.mutations.pop().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(step) = puzzle2d_retire_vec_backing(&mut self.mutations, maximum_bytes)? {
            return Ok(step);
        }
        if self.compatibility.pop().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(step) = puzzle2d_retire_vec_backing(&mut self.compatibility, maximum_bytes)? {
            return Ok(step);
        }
        macro_rules! retire_catalog_slice {
            ($slice:expr) => {
                if $slice.pop().is_some() {
                    return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
                }
                if let Some(step) = puzzle2d_retire_vec_backing(&mut $slice, maximum_bytes)? {
                    return Ok(step);
                }
            };
        }
        retire_catalog_slice!(self.catalogs.nodes);
        retire_catalog_slice!(self.catalogs.handles);
        retire_catalog_slice!(self.catalogs.edges);
        retire_catalog_slice!(self.catalogs.wires);
        if let Some(fragment) = self.fragment.take() {
            let bytes = match &fragment {
                Value::Object(object) => object.len().saturating_mul(size_of::<Value>()),
                Value::Array(values) => values.len().saturating_mul(size_of::<Value>()),
                _ => size_of::<Value>(),
            };
            if bytes > maximum_bytes {
                self.fragment = Some(fragment);
                return Err(Fault::from("puzzle2d kit:in fragment exceeds its bounded disposal byte slice"));
            }
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(text) = self.media_json.as_mut() {
            if let Some(step) = puzzle2d_retire_string_step(text, maximum_bytes)? {
                return Ok(step);
            }
            self.media_json = None;
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(step) = puzzle2d_retire_string_step(&mut self.port, maximum_bytes)? {
            return Ok(step);
        }
        if self.snapshot.as_ref().is_some_and(|snapshot| std::sync::Arc::strong_count(snapshot) == 1) {
            return Ok(PluginCloseStep::Blocked { reason: "puzzle2d import snapshot has no mounted retained authority" });
        }
        if self.snapshot.take().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.completion.as_ref().is_some_and(|completion| !completion.has_mounted_consumer()) {
            return Ok(PluginCloseStep::Blocked { reason: "puzzle2d import completion has no mounted consumer authority" });
        }
        if self.completion.take().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.port.is_empty()
            && self.port.capacity() == 0
            && self.media_json.is_none()
            && self.snapshot.is_none()
            && self.fragment.is_none()
            && self.completion.is_none()
            && self.pending_completion_rejection.is_none()
            && self.mutations.is_empty()
            && self.mutations.capacity() == 0
            && self.compatibility.is_empty()
            && self.compatibility.capacity() == 0
            && self.catalogs.nodes.is_empty()
            && self.catalogs.nodes.capacity() == 0
            && self.catalogs.handles.is_empty()
            && self.catalogs.handles.capacity() == 0
            && self.catalogs.edges.is_empty()
            && self.catalogs.edges.capacity() == 0
            && self.catalogs.wires.is_empty()
            && self.catalogs.wires.capacity() == 0
    }
}
//#endregion 🧵️ReservedJobs

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

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Puzzle2dConfigStorePreparationFactory))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Puzzle2dArtifactStorePreparationFactory))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

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

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let window_id = args.and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(dsl::DslValue::as_str).map(str::to_string);
        let args = args.map(Value::from);
        Puzzle2dCommand::try_from_action(action, args, window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 2D action '{action}'")))
    }

    /// 🎬️ Routes the example load through its own `Work` and mounted fill continuations through the
    /// fill runtime, both before document materialization; every other command runs the one shared
    /// [`puzzle2d_dispatch_emit`] pipeline the retained generic reduce runs.
    fn handle(
        command: &Puzzle2dCommand,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation, Self::DraftMutation>, Fault> {
        let config = cfg.snapshot;
        let (action, args) = (command.action_id(), command.args());
        if action == "setActiveExample" {
            return puzzle2d_active_example_emit(command, doc.snapshot, config);
        }
        // 🪣️ Every fill verb is `Migrated`: the session lives inside `Puzzle2dFillSessionWork`, which
        // owns the capture ingress, the search job and the placement cursor across its own steps. There
        // is nothing a single synchronous `handle` call could run that would match it, so this path
        // refuses rather than silently diverging (see `🎮️commands/🧮️set-fill-count/🦀️.rs`).
        if set_fill_count::is_fill_session_action(action) {
            return Err(Fault::from("puzzle2d-fill-requires-retained-job"));
        }
        let before = doc.snapshot.0.clone();
        Ok(puzzle2d_dispatch_emit(command, before, config, interaction.selection(PUZZLE2D_INTERACTION_DOMAIN), doc.operation_optional().cloned()))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle2d@1/*#editor",
        document_schema: "puzzle.2d.fixture",
        factory: "Puzzle2dRetainedCommandJobFactory",
        factory_type: Puzzle2dRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(8_192, 512, 1, 262_144, 7_500, 1, 1),
        tools: [
            "setActiveExample",
            "forceLayout",
            "addNode",
            "applyBoardEvents",
            "reorganize",
            "brushCancelSlot",
            "brushCommitSlot",
            "brushCycleCandidate",
            "brushFillSessionAdopt",
            "brushFillSessionBegin",
            "brushFillSessionCancel",
            "brushFillSessionClear",
            "brushFillSessionDiscard",
            "brushFillSessionRetry",
            "brushFillSessionStep",
            "brushOpenSlot",
            "brushSetCandidateIndex",
            "deleteSelection",
            "duplicateSelection",
            "engagementAbort",
            "engagementControlSelect",
            "engagementInput",
            "engagementSubmit",
            "focusSelection",
            "lodScaleJson",
            "patchInspectorNodes",
            "redrawHandles",
            "selectSameKind",
            "setBrushKindWeights",
            "setBrushNodeSize",
            "setCamera",
            "setFillCount",
            "setGridFactor",
            "setGridSnapEnabled",
            "setLocale",
            "setLodModeForPane",
            "setSelectionFlag",
            "setSuggestionOffset",
            "setTerminology"
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Puzzle2dImportJobFactory::new(&controller))?;
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
            "reorganize" => Box::new(Puzzle2dForceLayoutWork::new("reorganize")),
            "addNode" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new("addNode", puzzle2d_retained_reduce, puzzle2d_retained_extent)),
            "applyBoardEvents" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new("applyBoardEvents", puzzle2d_board_events_reduce, puzzle2d_board_events_extent)),
            generic if PUZZLE2D_GENERIC_TOOL_IDS.contains(&generic) => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(generic, puzzle2d_generic_reduce, puzzle2d_generic_extent)),
            host_only if PUZZLE2D_HOST_ONLY_TOOL_IDS.contains(&host_only) => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(host_only)),
            "redrawHandles" => Box::new(Puzzle2dRedrawHandlesWork::default()),
            fill if set_fill_count::is_fill_session_action(fill) => Box::new(set_fill_count::Puzzle2dFillSessionWork::new(fill)),
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

    /// 🔌️ Declares puzzle2d's typed media I/O surface — the implicit document ports plus `kit:in` and
    /// `design:out`. `kit:in`'s producer is `Block2dPlayApp`'s `"catalog:out"` port, whose `kit.catalog`
    /// payload is literally a puzzle2d manifest fragment (`portKinds`/`wireKinds`/`edgeKinds`/
    /// `nodeKinds`/`kindCompatibility` — `crate::artifacts::block2d::schema::inferences::puzzle2d_manifest_fragment`),
    /// so this is a 2d↔2d vocabulary, not block3d's object/vortex-kind one; `Puzzle2dImportJob` does the
    /// normalization one bounded row per step.
    fn io() -> Option<AppIo> {
        let io = semio_framework::io::resolve_ready(AppIo::from_document("puzzle.2d", MediaType { class: MediaClass::TwoD, form: MediaForm::Design }, ArtifactPresentation { id: "2d.puzzle".into(), name: "2D Puzzle".into(), dimension: "2d".into(), component_kind: "puzzle2d".into() }));
        Some(semio_framework::io::resolve_ready(io.with_ports(vec![
                    MediaPortSpec {
                        id: PUZZLE2D_IMPORT_PORT.into(),
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

    /// 🎞️ `import-media` is the only reserved route puzzle2d owns: the framework registers no
    /// importer on an app's behalf, and every inbound media delivery is routed exclusively through
    /// this builder (`dispatch_import_media` → `build_artifact_reserved_media_job`), never through the
    /// unbounded one-shot `ArtifactApp::import_media` seam. `copy`/`cut`/`paste` stay on the
    /// framework's own reserved factories — puzzle2d owns no clipboard fragment vocabulary of its own,
    /// exactly like puzzle3d.
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if request.tool_id.as_str() != PUZZLE2D_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(Fault::from("puzzle2d import-media admits a decoded media value, never a wire payload"));
        }
        let ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(Fault::from("puzzle2d import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(ArtifactReservedToolJob::new(Puzzle2dImportJob::new(request, port, media))))
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
        let labels = puzzle2d_labels(config).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.localization.unsupported", "puzzle2d locale or terminology is not recognized"))?;
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
        let Some(labels) = puzzle2d_labels(config) else {
            return HashMap::new();
        };
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
        let Some(labels) = puzzle2d_labels(config) else {
            return HashMap::new();
        };
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
        let Some(labels) = puzzle2d_labels(config) else {
            return HashMap::new();
        };
        HashMap::from([(fill::TOOL_ID.to_string(), vec![fill::measures(&envelope, labels)])])
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let config = cfg.snapshot;
        let Some(locale) = puzzle2d_config_locale(config) else {
            return Vec::new();
        };
        let is_de = locale == semio_framework_plugin::Locale::De;
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

/// 🎭️✏️ `.example_source(...)` (×2, concrete-forest + nakagin) and `.workflow("puzzle2d", …)` were
/// dropped, not ported — `EditorBuilder` has neither method (contract §2.4: `.editor::<E>(def:
/// AppDefinition)` only takes the definition, `App.examples` has no seam on this builder). Flagged to
/// the coordinator, not silently lost; see `📚️examples/🎬️demo-session` for this subset's own example
/// facet, the likely intended replacement mechanism.
pub fn create_puzzle2d_app() -> semio_framework_plugin::AppDefinition {
    let mut host = puzzle_board_host();
    let envelope = Puzzle2dScene { fixture: default_empty_fixture(), runtime: Puzzle2dPlayRuntime::default(), active_utility: select_utility::UTILITY_ID.into() };
    sync_host_from_envelope(&mut host, &envelope);
    let labels = puzzle2d_labels(&Puzzle2dConfig::default()).expect("default puzzle2d locale and terminology axes are explicit");
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
            // 🗣️ Locale and terminology are real user-facing settings verbs (mirrors puzzle3d's own
            // `.view_action` pair); the labels stay inline `LocalizedLabel::native` like every other
            // action here, since `puzzle2d_localized` resolves a `Puzzle2dLabels` field, not a phrase.
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            .view_action("setTerminology", LocalizedLabel::native("Set Terminology", "Terminologie festlegen"))
            // 🗂️ Referenced by `puzzle2d_context_menu_items` — categorized for grouped-context-menu disclosure.
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .keybinding("delete,backspace", "deleteSelection")
            .action_with(ActionDefinition::bounded_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).with_category("create"))
            .mutation("forceLayout", LocalizedLabel::native("Force Layout", "Kraftbasiertes Layout"))
            .action_with(ActionDefinition::bounded_catalog("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"), ActionKind::Mutation).with_category("view"))
            // 👁️ Palette-visible ephemeral view/selection commands.
            .action_with(ActionDefinition::bounded_catalog("selectSameKind", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).with_category("selection"))
            // 🔧️ Internal content operations — inspector/panel/board/import-bound, not palette commands.
            .action_with(puzzle2d_internal_action("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).with_category("settings"))
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
            .action_interactive_job("applyBoardEvents", InteractiveJobClassification::Migrated)
            .action_interactive_job("brushCancelSlot", InteractiveJobClassification::Migrated)
            .action_interactive_job("brushCommitSlot", InteractiveJobClassification::Migrated)
            .action_interactive_job("brushCycleCandidate", InteractiveJobClassification::Migrated)
            .action_interactive_job("brushFillSessionAdopt", InteractiveJobClassification::Migrated)
            .action_interactive_job("brushFillSessionBegin", InteractiveJobClassification::Migrated)
            .action_interactive_job("brushFillSessionCancel", InteractiveJobClassification::Migrated)
            .action_interactive_job("brushFillSessionClear", InteractiveJobClassification::Migrated)
            .action_interactive_job("brushFillSessionDiscard", InteractiveJobClassification::Migrated)
            .action_interactive_job("brushFillSessionRetry", InteractiveJobClassification::Migrated)
            .action_interactive_job("brushFillSessionStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("brushOpenSlot", InteractiveJobClassification::Migrated)
            .action_interactive_job("brushSetCandidateIndex", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementAbort", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementControlSelect", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("focusSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("forceLayout", InteractiveJobClassification::Migrated)
            .action_interactive_job("lodScaleJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchInspectorNodes", InteractiveJobClassification::Migrated)
            .action_interactive_job("redrawHandles", InteractiveJobClassification::Migrated)
            .action_interactive_job("reorganize", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectSameKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushKindWeights", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushNodeSize", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFillCount", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridFactor", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSnapEnabled", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLocale", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodModeForPane", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSuggestionOffset", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTerminology", InteractiveJobClassification::Migrated)
            // 🧰️ Canvas utilities — one exclusive set, active utility host-owned (never a document
            // operation); bound to the interactive overview pane by that window's own definition.
            .utility(select_utility::definition(puzzle2d_localized(|l| l.select)))
            .utility(brush_utility::definition(puzzle2d_localized(|l| l.brush)))
            // 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
            .tool(fill::definition(puzzle2d_localized(|l| l.fill)))
            .default_layout(edit::layout())
            .build_definition()
}

// 🗂️ `Puzzle2dPlaySnapshot`'s pack<->dsl codec (so `framework/sync`'s `FolderEndpoint::Pack` can
// print/parse puzzle-2d play documents without depending on this crate's concrete
// `Projection`/`Mutation` types) is now declared via `.document_codec::<Puzzle2dPlayApp>()` on
// `crate::artifacts::puzzle2d::declaration()` (ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`
// M1) — the old side-effecting `register_puzzle2d_exports()` wrapper (this app file's only caller of
// `register_document_codec_for_app`) is gone.
//#endregion 🔖️Manifest

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
    /// 🛍️ One dispatch is the whole load: `setActiveExample` drives `Puzzle2dActiveExampleWork` to its
    /// terminal emit (retained job and batch path alike), so no `DispatchAction` continuation ladder
    /// remains to pump. Returns the mutation count the load committed.
    pub fn load_example(app: &mut Puzzle2dApp, example_id: &str) -> usize {
        let result = dispatch(app, "setActiveExample", Some(&json!({ "exampleId": example_id })), None).expect("load example");
        assert!(result.requested_effects.is_empty(), "the example load must not request a continuation effect");
        result.mutations.len()
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
        let source = include_str!("🦀️.rs");
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

    /// 🪣️ The fill family is retained-only: `handle` must refuse it outright, the mounted store-lease
    /// hooks the removed session registry needed must be gone, and every verb must resolve to the
    /// bespoke session work.
    fn fill_session_retained_only_contract(source: &str) -> bool {
        let production = source.split("//#region 🧪️Tests").next().unwrap_or(source);
        let Some(handle) = production.find("    fn handle(") else { return false };
        let Some(fill_relative) = production[handle..].find("if set_fill_count::is_fill_session_action(action) {") else { return false };
        let fill = handle + fill_relative;
        let Some(normal_relative) = production[fill..].find("let before = doc.snapshot.0.clone();") else { return false };
        let branch = &production[fill..fill + normal_relative];
        branch.contains("puzzle2d-fill-requires-retained-job")
            && !branch.contains("Puzzle2dFillActionCtx")
            && !branch.contains("Puzzle2dConfigMutation")
            && !branch.contains("artifact_mutations")
            && !production.contains("fn mounted_job_prepare_snapshot_read")
            && !production.contains("fn pending_effects")
            && !production.contains("dispatch_fill_session_action")
            && production.contains("fill if set_fill_count::is_fill_session_action(fill) => Box::new(set_fill_count::Puzzle2dFillSessionWork::new(fill))")
            && set_fill_count::PUZZLE2D_FILL_SESSION_ACTIONS.iter().all(|action| PUZZLE2D_RETAINED_TOOL_IDS.contains(action))
    }

    /// 🧱️ Reviving the mounted fill dispatch path — the action context, a config mutation published
    /// from `handle`, or the store-lease hooks the process-global session registry needed — fails the
    /// retained-only law.
    #[test]
    fn mounted_fill_dispatch_revivals_are_rejected() {
        let source = include_str!("🦀️.rs");
        assert!(fill_session_retained_only_contract(source));
        let ctx = source.replacen(
            "            return Err(Fault::from(\"puzzle2d-fill-requires-retained-job\"));",
            "            let ctx = set_fill_count::Puzzle2dFillActionCtx {};\n            return Err(Fault::from(\"puzzle2d-fill-requires-retained-job\"));",
            1,
        );
        assert!(!fill_session_retained_only_contract(&ctx));
        let published = source.replacen("            return Err(Fault::from(\"puzzle2d-fill-requires-retained-job\"));", "            return Ok(Emit { config_mutations: vec![Puzzle2dConfigMutation::Fill { runtime }], ..Default::default() });", 1);
        assert!(!fill_session_retained_only_contract(&published));
        let lease = source.replacen("    fn handle(", "    fn pending_effects(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>) -> Vec<Effect> { Vec::new() }\n\n    fn handle(", 1);
        assert!(!fill_session_retained_only_contract(&lease));
        let unmapped = source.replacen("fill if set_fill_count::is_fill_session_action(fill) => Box::new(set_fill_count::Puzzle2dFillSessionWork::new(fill))", "fill if false => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(fill))", 1);
        assert!(!fill_session_retained_only_contract(&unmapped));
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

    /// 🛍️ The example load commits granular operations from ONE dispatch — the retained
    /// `Puzzle2dActiveExampleWork` state machine, driven to its terminal emit, with no
    /// `Effect::DispatchAction` continuation ladder left to pump.
    #[semio_framework_async_macros::async_test]
    async fn set_active_example_loads_concrete_forest_via_operations() {
        let mut app = app();
        let result = dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None).expect("load example");
        assert!(result.requested_effects.is_empty(), "the example load must not request a continuation effect");
        assert!(!result.mutations.is_empty(), "the example load must commit granular operations");
        assert!(!fixture_nodes(&fixture_of(&app)).is_empty());
    }

    /// 🔁️ Loading a second example clears the first one out of the document rather than merging into
    /// it — the work's `ClearEdges`/`ClearNodes` stages run against the live snapshot every time.
    #[semio_framework_async_macros::async_test]
    async fn a_newer_example_load_replaces_the_previous_document() {
        let mut app = app();
        load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
        let forest_nodes = fixture_nodes(&fixture_of(&app)).len();
        load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
        assert!(!fixture_edges(&fixture_of(&app)).is_empty());
        assert_ne!(fixture_nodes(&fixture_of(&app)).len(), forest_nodes, "the second load must replace, not append to, the first example");
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
