//! 🧊️ Puzzle 3d play app — the plugin's 3d editor surface: its `ArtifactEditor` impl (dispatch-only), the
//! structural-twin fixture document model its command/panel/window nodes mutate and render, the
//! attraction resolver that keeps every attracted object's pose derived from its attracting root,
//! and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️component.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/🧊️main`. This file dispatches and stitches.
//!
//! 🌉️ `ArtifactApp::Snapshot` is the `Puzzle3dPlaySnapshot` newtype over a bare
//! `serde_json::Value` fixture (see `crate::artifacts::puzzle3d::op`'s `🔖️ValueBridge`), not the typed
//! `Puzzle3dSnapshot` — the `Puzzle3dFixture` model below is this app's own structural twin of it,
//! and each action emits the granular typed operation delta
//! (`puzzle3d_operations_from_fixture_change`) turning the old fixture into the new one.

use crate::artifacts::puzzle3d::op::{puzzle3d_document_delta_operations, Puzzle3dMutation, Puzzle3dPlaySnapshot};
use crate::artifacts::puzzle3d::schema::Puzzle3dEngineCommand;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use crate::editor::puzzle3d::commands::{
    accept_suggestion, add_brush_object, add_object_kind, add_target_volume, apply_sun, close_vortex_suggestions, create_attraction, cycle_candidate, delete_attraction, delete_selection, delete_target_volume, duplicate_selection, engagement_abort,
    engagement_control_select, engagement_input, engagement_repeat_last, engagement_submit, fill_build_tick, focus_selection, hover_suggestion, open_vortex_suggestions, patch_inspector, register_brush_mesh, relocate_target_volume, rotate_selection,
    scale_selection, select_same_kind, set_active, set_active_example, set_automatic, set_brush_placement_overlap_budget, set_camera, set_chunk_size, set_depth_variable, set_fill_count, set_fixture_json, set_kind_weight, set_locale, set_manual,
    set_projection, set_proximity_radius, set_selectable_kind, set_selection_flag, set_snap_enabled, set_spacing, set_target_volume_flag, set_terminology, set_transform_gumball_flag, set_visible, set_vortex_direction, set_vortex_show,
    set_voxel_dims, suggestions_tick, translate_selection, world_relocate,
};
use crate::editor::puzzle3d::config::{Puzzle3dConfig, Puzzle3dConfigMutation, Puzzle3dRuntime};
use crate::editor::puzzle3d::modes::edit;
use crate::editor::puzzle3d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle3d::modes::edit::windows::main;
use crate::editor::puzzle3d::modes::edit::windows::main::utilities;
use crate::editor::puzzle3d::panels::{catalogue, document, inspection, settings as settings_panel};
use crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession;
use crate::editor::puzzle3d::presence::{Puzzle3dPresence, Puzzle3dPresenceMutation};
use crate::editor::puzzle3d::terminology::{puzzle3d_labels, puzzle3d_localized, puzzle3d_localized_phrase, Puzzle3dLabels};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{
    mesh_from_kind, panel_tab_element_id, panel_tab_first_draggable_element_id, window_element_id, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, ActionRef, AppIo, ArtifactEditor, ArtifactView, BuiltNode, ConfigView,
    Dialect, DialogDefinition, DraftView, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTarget, IntroductionDefinition, IntroductionInteraction, IntroductionPlacement,
    IntroductionStepDefinition, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, MergeMode, NoDraft, NoDraftMutation, PortMultiplicity, SelectionMethod, SelectionMode, SelectionSpec,
    ToolRef, UiNode, WindowEngagement, WindowMeasure, INTERACTION_SELECT_ACTION_ID, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID,
};
use store::EngineHandles;
// 🎭️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET (contract §2.1): `ArtifactEditor`
// replaces `ArtifactApp` as the authoring trait; `EditorApp<E>` is the runtime `ArtifactApp`
// adapter, needed by this file's own test harness (`VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>`).
// 🕹️ `InteractionView` is defined inside `semio_framework_plugin::app` (the plugin SDK's internal
// module) but is NOT re-exported at that crate's root alongside `ArtifactApp`/`ConfigView`/`DraftView`
// — a gap in the ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3 wave's `pub use app::{..}`
// list (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` around :12117), flagged to the
// coordinator rather than fixed here (framework file, out of this crate's remit). `app` itself is
// `pub`, so the full path below still resolves.
use semio_framework_plugin::app::InteractionView;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{LazyLock, Mutex};

//#region 🔖️Constants
pub const PUZZLE3D_PLAY_APP_ID: &str = "puzzle3d-play";
pub const PUZZLE3D_PLAY_CONTROLLER_ID: &str = "puzzle3d-play";
pub const PUZZLE3D_FIXTURE_SCHEMA: &str = "puzzle.3d.fixture";
pub const PUZZLE3D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";
pub const PUZZLE3D_EXAMPLE_NAKAGIN: &str = "nakagin-capsule-tower";
pub const PUZZLE3D_FALLBACK_MESH_KIND: &str = "box";
/// 🧰️ Host-owned active utility when none has been pressed yet — none. The transform gumball must be
/// pressed explicitly; an unset/cleared utility must not fall back to `transform` or the gumball
/// appears without an active transform tool.
pub const PUZZLE3D_DEFAULT_UTILITY: &str = "";
pub const PUZZLE3D_FILL_COUNT_MAX: u32 = 1000;
/// 🌀️ Window option: emit every object's vortices into the 3D scene.
pub const PUZZLE3D_VORTEX_SHOW_ALWAYS: &str = "always";
/// 🌀️ Window option: emit vortices only for hovered/selected objects (and vortex-only hover/selection).
pub const PUZZLE3D_VORTEX_SHOW_SELECTED: &str = "selected";
/// 🧭️ Window option: arrow tip points away from the vortex point along `direction`.
pub const PUZZLE3D_VORTEX_DIRECTION_OUTWARDS: &str = "outwards";
/// 🧭️ Window option: arrow tip ends on the vortex point; shaft starts at `point - direction * length`.
pub const PUZZLE3D_VORTEX_DIRECTION_INWARDS: &str = "inwards";
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the one interaction domain this app
/// declares — every previously-separate `Puzzle3dSelection` bag (object/vortex/attraction/
/// targetVolume/reference) plus the catalogue's kind rows collapse into one framework-owned domain,
/// distinguished by `DomainSelection.granularity` (`PUZZLE3D_GRANULARITY_*`) instead of a distinct
/// config field per kind.
pub const PUZZLE3D_INTERACTION_DOMAIN: &str = "vortex";
pub const PUZZLE3D_GRANULARITY_OBJECT: &str = "object";
pub const PUZZLE3D_GRANULARITY_VORTEX: &str = "vortex";
pub const PUZZLE3D_GRANULARITY_ATTRACTION: &str = "attraction";
pub const PUZZLE3D_GRANULARITY_TARGET_VOLUME: &str = "targetVolume";
pub const PUZZLE3D_GRANULARITY_REFERENCE: &str = "reference";
pub const PUZZLE3D_GRANULARITY_KIND: &str = "kind";

/// 🔢️ Monotone serial behind every app-minted object / attraction / target-volume id.
pub static PUZZLE3D_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// 🌉️ This app's own `Puzzle3dScene.fixture: Puzzle3dFixture` (and `ArtifactApp::Snapshot`) stays a
/// local structural-twin mirror of `crate::artifacts::puzzle3d::Puzzle3dSnapshot`, so the DSL-text
/// example fixtures are parsed once into the typed projection and re-serialized to the JSON string
/// this module's `serde_json::from_str::<Puzzle3dFixture>`/`.example(...)` call sites expect.
pub static CONCRETE_FOREST_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| parse_example_dsl(crate::artifacts::puzzle3d::dsl::PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT, "concrete-forest"));
pub static NAKAGIN_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| parse_example_dsl(crate::artifacts::puzzle3d::dsl::PUZZLE3D_NAKAGIN_EXAMPLE_TEXT, "nakagin"));
static CONCRETE_FOREST_EXAMPLE_FIXTURE: LazyLock<Puzzle3dFixture> = LazyLock::new(|| serde_json::from_str(CONCRETE_FOREST_EXAMPLE_JSON.as_str()).unwrap_or_else(|_| empty_fixture()));
static NAKAGIN_EXAMPLE_FIXTURE: LazyLock<Puzzle3dFixture> = LazyLock::new(|| serde_json::from_str(NAKAGIN_EXAMPLE_JSON.as_str()).unwrap_or_else(|_| empty_fixture()));

fn parse_example_dsl(dsl_text: &str, label: &str) -> String {
    let projection = <Puzzle3dSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).unwrap_or_else(|error| panic!("{label} example fixture parses as dsl: {error}"));
    serde_json::to_string(&projection).unwrap_or_else(|error| panic!("serialize {label} example fixture: {error}"))
}

pub fn puzzle3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PUZZLE3D_PLAY_CONTROLLER_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: builds a framework `interactionSelect`
/// action targeting one `(granularity, id)` pair in the `vortex` domain — replaces the deleted
/// `setSelection` action builders every document/catalogue tree row used to construct by hand.
pub fn puzzle3d_interaction_select(granularity: &str, id: &str) -> ActionDescriptor {
    let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
    puzzle3d_action(INTERACTION_SELECT_ACTION_ID, Some(json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })))
}
//#endregion 🔖️Constants

//#region 🔖️Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dVortex {
    pub id: String,
    #[serde(default, rename = "vortexKind")]
    pub vortex_kind: Option<String>,
    #[serde(default)]
    pub position: [f64; 3],
    #[serde(default)]
    pub direction: Option<[f64; 3]>,
    #[serde(default)]
    pub radius: Option<f64>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub locked: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dReferenceSource {
    #[serde(default)]
    pub url: String,
    #[serde(default, rename = "mediaKind")]
    pub media_kind: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dReference {
    pub id: String,
    #[serde(default)]
    pub source: Puzzle3dReferenceSource,
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default, rename = "widthWorld")]
    pub width_world: f64,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub hidden: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dObject {
    pub id: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default, rename = "objectKind")]
    pub object_kind: Option<String>,
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default)]
    pub orientation: Option<[f64; 4]>,
    #[serde(default)]
    pub scale: Option<Value>,
    #[serde(default, rename = "meshUrl")]
    pub mesh_url: Option<String>,
    #[serde(default)]
    pub vortices: Vec<Puzzle3dVortex>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub locked: bool,
    /// 🪣️ Live-viewport-only tag from `compose_fill_display` — this object's 0-based position in the
    /// fill plan's sequence, never persisted to the committed document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reveal_index: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dFixtureMeta {
    #[serde(default, rename = "kindCatalogs")]
    pub kind_catalogs: Option<Value>,
    #[serde(default, rename = "kindCompatibility")]
    pub kind_compatibility: Option<Value>,
}

/// 🧊️ Persisted oriented box constraining fill placement. Volume Brush creates axis-aligned
/// voxel-sized instances; the Transform gumball edits arbitrary oriented boxes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dTargetVolume {
    pub id: String,
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default)]
    pub orientation: Option<[f64; 4]>,
    #[serde(default)]
    pub scale: Option<Value>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub locked: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dAttraction {
    #[serde(default)]
    pub id: String,
    pub attracting: String,
    pub attracted: String,
    #[serde(default)]
    pub gap: f64,
    #[serde(default)]
    pub shift: f64,
    #[serde(default)]
    pub rise: f64,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default)]
    pub turn: f64,
    #[serde(default)]
    pub tilt: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dFixture {
    pub schema: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub meta: Puzzle3dFixtureMeta,
    #[serde(default)]
    pub objects: Vec<Puzzle3dObject>,
    #[serde(default)]
    pub attractions: Vec<Puzzle3dAttraction>,
    #[serde(default, rename = "targetVolumes")]
    pub target_volumes: Vec<Puzzle3dTargetVolume>,
    #[serde(default)]
    pub references: Vec<Puzzle3dReference>,
}

/// 🧾️ Transient render/mutation bundle pairing the persisted projection (the bare `Puzzle3dFixture`
/// json) with the app's ephemeral view state. Never persisted — the `VcsArtifactApp` store owns the
/// fixture and `Puzzle3dConfig` owns the runtime — but rebuilt per call so the panel/world/engagement
/// helpers keep one `&Puzzle3dScene` signature.
#[derive(Clone)]
pub struct Puzzle3dScene {
    pub fixture: Puzzle3dFixture,
    pub runtime: Puzzle3dRuntime,
    /// 🧰️ The effective interaction id for this render/mutation — transient, never persisted.
    pub active_utility: String,
}

pub fn empty_fixture() -> Puzzle3dFixture {
    Puzzle3dFixture { schema: PUZZLE3D_FIXTURE_SCHEMA.into(), domain: "architecture".into(), meta: Puzzle3dFixtureMeta::default(), objects: Vec::new(), attractions: Vec::new(), target_volumes: Vec::new(), references: Vec::new() }
}

pub fn default_fixture() -> Puzzle3dFixture {
    CONCRETE_FOREST_EXAMPLE_FIXTURE.clone()
}

pub fn nakagin_fixture() -> Puzzle3dFixture {
    NAKAGIN_EXAMPLE_FIXTURE.clone()
}

/// 🧾️ Materializes the transient scene from the persisted projection (bare fixture json) and the
/// app's current view state; an unparseable projection degrades to an empty board.
pub fn scene_from_projection(projection: &Value, runtime: Puzzle3dRuntime, active_utility: &str) -> Puzzle3dScene {
    let fixture = serde_json::from_value::<Puzzle3dFixture>(projection.clone()).unwrap_or_else(|_| empty_fixture());
    Puzzle3dScene { fixture, runtime, active_utility: active_utility.to_string() }
}

struct Puzzle3dExampleOperations {
    before: Value,
    after: Puzzle3dFixture,
    operations: Vec<Puzzle3dMutation>,
}

static PUZZLE3D_EXAMPLE_OPERATIONS: LazyLock<Vec<Puzzle3dExampleOperations>> = LazyLock::new(|| {
    let raw = vec![empty_fixture(), default_fixture(), nakagin_fixture()];
    let mut resolved = raw.clone();
    for fixture in &mut resolved {
        resolve_puzzle3d_attractions(fixture);
    }
    let mut before_values: Vec<Value> = raw.iter().chain(&resolved).filter_map(|fixture| serde_json::to_value(fixture).ok()).collect();
    before_values.dedup();
    let after_values: Vec<Value> = resolved.iter().filter_map(|fixture| serde_json::to_value(fixture).ok()).collect();
    let mut entries = Vec::new();
    for before in before_values {
        for (after, after_value) in resolved.iter().zip(&after_values) {
            entries.push(Puzzle3dExampleOperations { operations: puzzle3d_document_delta_operations(&before, after_value), before: before.clone(), after: after.clone() });
        }
    }
    entries
});

/// 🧮️ Document operations for a fixture mutation through the typed semantic delta vocabulary.
pub fn puzzle3d_operations_from_fixture_change(before: &Value, after_fixture: &Puzzle3dFixture) -> Vec<Puzzle3dMutation> {
    if let Some(entry) = PUZZLE3D_EXAMPLE_OPERATIONS.iter().find(|entry| &entry.before == before && &entry.after == after_fixture) {
        return entry.operations.clone();
    }
    let after = serde_json::to_value(after_fixture).unwrap_or_else(|_| before.clone());
    puzzle3d_document_delta_operations(before, &after)
}

/// 🔌️ `kit:in` seam helper: keyed UPSERT of `incoming` rows (each shaped `{"id": "...", ...}`) into
/// `catalogs[section]` (creating the section as an empty array if absent) — replaces any existing row
/// with the same `"id"`, else appends. Deterministic/order-independent in the resulting SET of ids (a
/// `multiplicity: Many` port may fan in from several producers across several `import_media` calls);
/// when two producers disagree on one id's content, the most-recently-applied wins.
fn puzzle3d_normalize_object_kind_row(mut row: Value) -> Value {
    let mesh_url = row.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).map(str::to_string);
    let has_rep = row.get("representations").and_then(Value::as_array).map(|rows| rows.iter().any(|rep| rep.get("url").and_then(Value::as_str).filter(|url| !url.is_empty()).is_some())).unwrap_or(false);
    let id = row.get("id").and_then(Value::as_str).unwrap_or("kind").to_string();
    if let Some(url) = mesh_url {
        if let Some(object) = row.as_object_mut() {
            if !has_rep {
                object.insert("representations".into(), json!([{ "id": format!("{id}:rep0"), "name": "default", "url": url, "mime": "", "description": "", "tags": [] }]));
            }
            object.remove("meshUrl");
        }
    }
    row
}

fn puzzle3d_upsert_catalog_rows(catalogs: &mut Value, section: &str, incoming: Option<&Value>) {
    let Some(incoming_rows) = incoming.and_then(Value::as_array) else {
        return;
    };
    if incoming_rows.is_empty() {
        return;
    }
    let existing = catalogs.as_object_mut().and_then(|object| object.entry(section).or_insert_with(|| json!([])).as_array_mut());
    let Some(existing) = existing else {
        return;
    };
    for row in incoming_rows {
        let row = if section == "objects" { puzzle3d_normalize_object_kind_row(row.clone()) } else { row.clone() };
        let Some(id) = row.get("id").and_then(Value::as_str) else {
            continue;
        };
        match existing.iter().position(|entry| entry.get("id").and_then(Value::as_str) == Some(id)) {
            Some(index) => existing[index] = row,
            None => existing.push(row),
        }
    }
}

/// 🪟️ B1: live window-instance ids from `Puzzle3dConfig::window_ids` (was host-pushed
/// `view_state.window_instances`) — falls back to `vec![kind_id]` when the list is empty (a
/// headless/test call that never populated it still gets exactly the one entry today's single-window
/// callers expect). puzzle3d has one window KIND that may be split into several INSTANCES, all
/// recorded flat in `window_ids`, so there is no per-kind filtering left to do.
fn window_instance_ids(config: &Puzzle3dConfig, kind_id: &str) -> Vec<String> {
    if config.window_ids.is_empty() {
        vec![kind_id.to_string()]
    } else {
        config.window_ids.clone()
    }
}

pub fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).filter(|ids: &Vec<String>| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
}

/** @emoji 🧭️ Whether `handle` may emit VCS operations from a fixture before/after delta — view-only actions skip the document snapshot entirely. */
fn puzzle3d_action_document_intent(action: &str) -> bool {
    matches!(
        action,
        "setFixtureJson"
            | "setActiveExample"
            | "addObjectKind"
            | "deleteSelection"
            | "duplicateSelection"
            | "translateSelection"
            | "rotateSelection"
            | "scaleSelection"
            | "transformEnd"
            | "worldRelocate"
            | "setSelectionFlag"
            | "patchInspector"
            | "engagementSubmit"
            | "engagementRepeatLast"
            | "createAttraction"
            | "deleteAttraction"
            | "addTargetVolume"
            | "deleteTargetVolume"
            | "setTargetVolumeFlag"
            | "addBrushObject"
            | "acceptSuggestion"
    )
}

//#region 🔖️Quaternions
pub fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]
}

pub fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
    let len = (ax * ax + ay * ay + az * az).sqrt();
    if len < 1e-8 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let half = angle * 0.5;
    let s = half.sin();
    [ax / len * s, ay / len * s, az / len * s, half.cos()]
}

pub fn quat_rotate_vector(quat: [f64; 4], vector: [f64; 3]) -> [f64; 3] {
    let [x, y, z, w] = quat;
    let vx = vector[0];
    let vy = vector[1];
    let vz = vector[2];
    let ix = w * vx + y * vz - z * vy;
    let iy = w * vy + z * vx - x * vz;
    let iz = w * vz + x * vy - y * vx;
    let iw = -x * vx - y * vy - z * vz;
    [ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x]
}
//#endregion 🔖️Quaternions

//#region 🔖️FixtureQueries
fn scale_value_mul(scale: &Option<Value>, sx: f64, sy: f64, sz: f64) -> Value {
    match scale {
        Some(Value::Array(values)) if values.len() >= 3 => json!([values[0].as_f64().unwrap_or(1.0) * sx, values[1].as_f64().unwrap_or(1.0) * sy, values[2].as_f64().unwrap_or(1.0) * sz,]),
        Some(Value::Number(value)) => {
            let factor = value.as_f64().unwrap_or(1.0);
            json!([factor * sx, factor * sy, factor * sz])
        }
        _ => json!([sx, sy, sz]),
    }
}

pub fn resolve_object_mesh_url(object: &Puzzle3dObject, meta: &Puzzle3dFixtureMeta) -> Option<String> {
    if let Some(url) = object.mesh_url.as_ref().filter(|url| !url.is_empty()) {
        return Some(url.clone());
    }
    let kind_id = object.object_kind.as_deref()?;
    let catalogs = meta.kind_catalogs.as_ref()?;
    let objects = catalogs.get("objects")?.as_array()?;
    for entry in objects {
        if entry.get("id").and_then(|v| v.as_str()) == Some(kind_id) {
            return entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string);
        }
    }
    None
}

pub fn collect_mesh_urls(fixture: &Puzzle3dFixture) -> Vec<String> {
    let mut urls = HashSet::new();
    for object in &fixture.objects {
        if let Some(url) = resolve_object_mesh_url(object, &fixture.meta) {
            urls.insert(url);
        }
    }
    if let Some(catalogs) = fixture.meta.kind_catalogs.as_ref() {
        if let Some(objects) = catalogs.get("objects").and_then(|v| v.as_array()) {
            for entry in objects {
                if let Some(url) = entry.get("meshUrl").and_then(|v| v.as_str()) {
                    urls.insert(url.to_string());
                }
            }
        }
    }
    urls.into_iter().collect()
}

pub fn object_scale_json(object: &Puzzle3dObject) -> [f64; 3] {
    match &object.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        _ => [1.0, 1.0, 1.0],
    }
}

pub fn target_volume_scale_json(volume: &Puzzle3dTargetVolume) -> [f64; 3] {
    match &volume.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        _ => [1.0, 1.0, 1.0],
    }
}

pub fn puzzle3d_vortex_full_id(object_id: &str, vortex_id: &str) -> String {
    if vortex_id.contains(':') {
        vortex_id.to_string()
    } else {
        format!("{object_id}:{vortex_id}")
    }
}

pub fn world_vortex_position(object: &Puzzle3dObject, vortex: &Puzzle3dVortex) -> [f64; 3] {
    let orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let rotated = quat_rotate_vector(orientation, vortex.position);
    [object.origin.first().copied().unwrap_or(0.0) + rotated[0], object.origin.get(1).copied().unwrap_or(0.0) + rotated[1], object.origin.get(2).copied().unwrap_or(0.0) + rotated[2]]
}

pub fn resolve_vortex_world_position(fixture: &Puzzle3dFixture, full_id: &str) -> Option<[f64; 3]> {
    for object in &fixture.objects {
        for vortex in &object.vortices {
            if puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id {
                return Some(world_vortex_position(object, vortex));
            }
        }
    }
    None
}

pub fn resolve_vortex_kind(fixture: &Puzzle3dFixture, full_id: &str) -> Option<String> {
    fixture.objects.iter().find_map(|object| object.vortices.iter().find(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id).and_then(|vortex| vortex.vortex_kind.clone()))
}

/// 🧲️ Permissive when the fixture declares no `kindCompatibility` rules at all — otherwise requires an explicit (or bidirectional) entry.
pub fn puzzle3d_kinds_compatible(fixture: &Puzzle3dFixture, source_kind: &str, target_kind: &str) -> bool {
    let Some(entries) = fixture.meta.kind_compatibility.as_ref().and_then(|value| value.as_array()) else {
        return true;
    };
    if entries.is_empty() {
        return true;
    }
    entries.iter().any(|entry| {
        let source = entry.get("source").and_then(|value| value.as_str()).unwrap_or("");
        let target = entry.get("target").and_then(|value| value.as_str()).unwrap_or("");
        let bidirectional = entry.get("bidirectional").and_then(|value| value.as_bool()).unwrap_or(false);
        (source == source_kind && target == target_kind) || (bidirectional && source == target_kind && target == source_kind)
    })
}

pub fn puzzle3d_catalog_entries<'a>(fixture: &'a Puzzle3dFixture, section: &str) -> &'a [Value] {
    fixture.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get(section)).and_then(|entries| entries.as_array()).map(Vec::as_slice).unwrap_or(&[])
}

pub fn puzzle3d_kind_ids(fixture: &Puzzle3dFixture, section: &str) -> Vec<String> {
    puzzle3d_catalog_entries(fixture, section).iter().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

pub fn next_object_id() -> String {
    let next = PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("object-{next}")
}

/// 🧊️ Seeds real vortices for a freshly placed object from its kind catalog's `vortices` templates, so it is immediately brushable instead of connector-less.
pub fn puzzle3d_vortices_from_kind_template(catalog_entry: &Value) -> Vec<Puzzle3dVortex> {
    catalog_entry
        .get("vortices")
        .and_then(|value| value.as_array())
        .map(|templates| {
            templates
                .iter()
                .enumerate()
                .map(|(index, template)| {
                    let position = template.get("position").and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]);
                    let direction = template.get("direction").and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok());
                    let radius = template.get("radius").and_then(|value| value.as_f64());
                    Puzzle3dVortex { id: format!("v{index}"), vortex_kind: template.get("vortexKind").and_then(|value| value.as_str()).map(str::to_string), position, direction, radius, hidden: false, locked: false }
                })
                .collect()
        })
        .unwrap_or_default()
}
//#endregion 🔖️FixtureQueries

//#region 🔖️SceneState
/// 🪣️ Whether the mode-level Fill tool currently authorizes fill planning and interaction.
pub fn puzzle3d_fill_tool_active(config: &Puzzle3dConfig) -> bool {
    config.active_tool_id.as_deref() == Some(fill_tool::TOOL_ID)
}

/// 🛠️ The effective interaction id threaded through `Puzzle3dScene.active_utility`: the per-window
/// utility (`active_utility_by_window_id` for `window_id`), UNLESS the mode-level fill tool is active
/// (`active_tool_id`), in which case fill wins. Fill keeps its viewport interaction even though it is
/// declared as a windowless tool, not a `WindowKindDefinition` utility.
pub fn puzzle3d_scene_active_utility(config: &Puzzle3dConfig, window_id: Option<&str>) -> String {
    if puzzle3d_fill_tool_active(config) {
        return fill_tool::TOOL_ID.to_string();
    }
    if let Some(wid) = window_id {
        if let Some(utility) = config.active_utility_by_window_id.get(wid) {
            return utility.clone();
        }
    }
    PUZZLE3D_DEFAULT_UTILITY.to_string()
}

/// 🎯️ The vortex the brush/suggestion machinery currently targets: an explicit vortex selection, else
/// the hovered vortex, else the first vortex of the hovered object.
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to fall back to
/// `runtime.selection.vortex_ids`/`hovered_vortex_full_id`/`hovered_object_id`, all now dissolved into
/// the framework-owned `vortex` interaction domain. `ArtifactApp::render` (this fn's only caller with
/// no `Puzzle3dActionCtx` in scope) never receives an `InteractionView`, so render-time brush-target
/// resolution has no live selection/hover to fall back to — callers holding a `Puzzle3dActionCtx`
/// should prefer `ctx.selected_vortex_ids().first()` before reaching for this. Flagged to the
/// coordinator as a framework-level gap (`ArtifactApp::render`/`context_menu` never gained the
/// `interaction: &InteractionView` parameter `handle`/`copy_fragment`/`cut_operations` did), not fixed
/// here (framework file, out of this crate's remit).
pub fn puzzle3d_brush_target_vortex(_envelope: &Puzzle3dScene) -> Option<String> {
    None
}
//#endregion 🔖️SceneState

//#region 🔖️FixtureEdits
/// 🧲️ Applies one absolute gumball translate (total delta from drag-start) onto a fixture snapshot.
pub fn puzzle3d_apply_translate(fixture: &mut Puzzle3dFixture, object_ids: &[String], volume_ids: &[String], dx: f64, dy: f64, dz: f64) {
    for object in &mut fixture.objects {
        if object_ids.contains(&object.id) {
            object.origin[0] += dx;
            object.origin[1] += dy;
            object.origin[2] += dz;
        }
    }
    for volume in fixture.target_volumes.iter_mut().filter(|volume| volume_ids.contains(&volume.id) && !volume.locked) {
        volume.origin[0] += dx;
        volume.origin[1] += dy;
        volume.origin[2] += dz;
    }
}

/// 🧲️ Applies one absolute gumball rotate (total axis-angle from drag-start) onto a fixture snapshot.
pub fn puzzle3d_apply_rotate(fixture: &mut Puzzle3dFixture, object_ids: &[String], volume_ids: &[String], ax: f64, ay: f64, az: f64, angle: f64) {
    let delta = quat_from_axis_angle(ax, ay, az, angle);
    for object in &mut fixture.objects {
        if object_ids.contains(&object.id) {
            let current = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            object.orientation = Some(quat_mul(delta, current));
        }
    }
    for volume in fixture.target_volumes.iter_mut().filter(|volume| volume_ids.contains(&volume.id) && !volume.locked) {
        let current = volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
        volume.orientation = Some(quat_mul(delta, current));
    }
}

/// 🧲️ Applies one absolute gumball scale (total factors from drag-start) onto a fixture snapshot.
pub fn puzzle3d_apply_scale(fixture: &mut Puzzle3dFixture, object_ids: &[String], volume_ids: &[String], sx: f64, sy: f64, sz: f64) {
    for object in &mut fixture.objects {
        if object_ids.contains(&object.id) {
            object.scale = Some(scale_value_mul(&object.scale, sx, sy, sz));
        }
    }
    for volume in fixture.target_volumes.iter_mut().filter(|volume| volume_ids.contains(&volume.id) && !volume.locked) {
        volume.scale = Some(scale_value_mul(&volume.scale, sx, sy, sz));
    }
}

/// 🙈️ Applies `hidden`/`locked` to the given ids of one entity kind — `"vortex"` ids are full ids (`objectId:vortexId`).
pub fn apply_puzzle3d_selection_flag(fixture: &mut Puzzle3dFixture, entity: &str, ids: &[String], flag: &str, value: bool) {
    if ids.is_empty() {
        return;
    }
    let ids: HashSet<&str> = ids.iter().map(String::as_str).collect();
    match entity {
        "object" => {
            for object in fixture.objects.iter_mut().filter(|object| ids.contains(object.id.as_str())) {
                if flag == "locked" {
                    object.locked = value;
                } else {
                    object.hidden = value;
                }
            }
        }
        "vortex" => {
            for object in fixture.objects.iter_mut() {
                for vortex in object.vortices.iter_mut() {
                    if ids.contains(puzzle3d_vortex_full_id(&object.id, &vortex.id).as_str()) {
                        if flag == "locked" {
                            vortex.locked = value;
                        } else {
                            vortex.hidden = value;
                        }
                    }
                }
            }
        }
        "reference" => {
            for reference in fixture.references.iter_mut().filter(|reference| ids.contains(reference.id.as_str())) {
                if flag == "locked" {
                    reference.locked = value;
                } else {
                    reference.hidden = value;
                }
            }
        }
        "targetVolume" => {
            for volume in fixture.target_volumes.iter_mut().filter(|volume| ids.contains(volume.id.as_str())) {
                if flag == "locked" {
                    volume.locked = value;
                } else {
                    volume.hidden = value;
                }
            }
        }
        _ => {}
    }
}

pub fn value_as_vec3(value: &Value) -> Option<[f64; 3]> {
    let array = value.as_array()?;
    Some([array.first()?.as_f64()?, array.get(1)?.as_f64()?, array.get(2)?.as_f64()?])
}

/** @emoji 📐️ Resolves one numeric-field edit: an absolute `value` (typed entry) wins when present,
 * otherwise a `delta` (stepper nudge) is added to `current` — offset-preserving across a multi-select
 * where `current` differs per entity. `None` when neither parses. */
fn puzzle3d_resolve_number_edit(current: f64, value: Option<&Value>, delta: Option<&Value>) -> Option<f64> {
    if let Some(absolute) = value.and_then(Value::as_f64) {
        return Some(absolute);
    }
    delta.and_then(Value::as_f64).map(|delta| current + delta)
}

/** @emoji 📐️ Settings counterpart to `puzzle3d_resolve_number_edit`: reads `value`/`delta` directly
 * out of an action's `args`, for single global settings (not per-entity multi-select) whose stepper
 * dispatches straight to their own dedicated action. */
pub fn puzzle3d_absolute_or_delta(args: Option<&Value>, current: f64) -> Option<f64> {
    puzzle3d_resolve_number_edit(current, args.and_then(|value| value.get("value")), args.and_then(|value| value.get("delta")))
}

/** @emoji 📐️ Parses a nested stepper-group field id as `"<base>.<axis>"` (`x`/`y`/`z`/`w`), returning
 * the axis index when `field` names a component of `base` — the dot-path convention
 * `ui_inspector_vec3_group`/the inspector's quaternion group use for their per-axis actions. */
fn puzzle3d_axis_index(field: &str, base: &str) -> Option<usize> {
    match field.strip_prefix(base)?.strip_prefix('.')? {
        "x" => Some(0),
        "y" => Some(1),
        "z" => Some(2),
        "w" => Some(3),
        _ => None,
    }
}

/// 🔎️ Generic inspector edit dispatcher — `entity`/`field` select the target, `ids` scope it (full ids for vortices, `objectId:vortexId`).
/// `hidden`/`locked` delegate to `apply_puzzle3d_selection_flag` (shared with the non-inspector toggle path); every other field
/// resolves via `value` (absolute) or `delta` (relative, added to each entity's own current component).
pub fn apply_puzzle3d_inspector_patch(fixture: &mut Puzzle3dFixture, entity: &str, ids: &[String], field: &str, value: Option<&Value>, delta: Option<&Value>) {
    if ids.is_empty() {
        return;
    }
    if field == "hidden" || field == "locked" {
        if let Some(pressed) = value.and_then(Value::as_bool) {
            apply_puzzle3d_selection_flag(fixture, entity, ids, field, pressed);
        }
        return;
    }
    let id_set: HashSet<&str> = ids.iter().map(String::as_str).collect();
    match entity {
        "object" => {
            for object in fixture.objects.iter_mut().filter(|object| id_set.contains(object.id.as_str())) {
                match field {
                    "label" => object.label = value.and_then(Value::as_str).map(str::to_string),
                    "objectKind" => object.object_kind = value.and_then(Value::as_str).map(str::to_string),
                    "meshUrl" => object.mesh_url = value.and_then(Value::as_str).map(str::to_string),
                    "origin" => {
                        if let Some(origin) = value.and_then(value_as_vec3) {
                            object.origin = origin;
                        }
                    }
                    _ => {
                        if let Some(axis) = puzzle3d_axis_index(field, "origin") {
                            if let Some(updated) = puzzle3d_resolve_number_edit(object.origin[axis], value, delta) {
                                object.origin[axis] = updated;
                            }
                        } else if let Some(axis) = puzzle3d_axis_index(field, "scale") {
                            let mut scale = object_scale_json(object);
                            if let Some(updated) = puzzle3d_resolve_number_edit(scale[axis], value, delta) {
                                scale[axis] = updated;
                                object.scale = Some(json!(scale));
                            }
                        } else if let Some(axis) = puzzle3d_axis_index(field, "orientation") {
                            let mut quat = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                            if let Some(updated) = puzzle3d_resolve_number_edit(quat[axis], value, delta) {
                                quat[axis] = updated;
                                object.orientation = Some(quat_normalize(quat));
                            }
                        }
                    }
                }
            }
        }
        "vortex" => {
            for object in fixture.objects.iter_mut() {
                for vortex in object.vortices.iter_mut() {
                    if !id_set.contains(puzzle3d_vortex_full_id(&object.id, &vortex.id).as_str()) {
                        continue;
                    }
                    match field {
                        "vortexKind" => vortex.vortex_kind = value.and_then(Value::as_str).map(str::to_string),
                        "position" => {
                            if let Some(position) = value.and_then(value_as_vec3) {
                                vortex.position = position;
                            }
                        }
                        "direction" => {
                            if let Some(direction) = value.and_then(value_as_vec3) {
                                vortex.direction = Some(direction);
                            }
                        }
                        "radius" => {
                            if let Some(updated) = puzzle3d_resolve_number_edit(vortex.radius.unwrap_or(0.35), value, delta) {
                                vortex.radius = Some(updated);
                            }
                        }
                        _ => {
                            if let Some(axis) = puzzle3d_axis_index(field, "position") {
                                if let Some(updated) = puzzle3d_resolve_number_edit(vortex.position[axis], value, delta) {
                                    vortex.position[axis] = updated;
                                }
                            } else if let Some(axis) = puzzle3d_axis_index(field, "direction") {
                                let mut direction = vortex.direction.unwrap_or([0.0, 0.0, 1.0]);
                                if let Some(updated) = puzzle3d_resolve_number_edit(direction[axis], value, delta) {
                                    direction[axis] = updated;
                                    vortex.direction = Some(direction);
                                }
                            }
                        }
                    }
                }
            }
        }
        "attraction" => {
            for attraction in fixture.attractions.iter_mut().filter(|attraction| id_set.contains(attraction.id.as_str())) {
                match field {
                    "attracting" => {
                        if let Some(text) = value.and_then(Value::as_str) {
                            attraction.attracting = text.into();
                        }
                    }
                    "attracted" => {
                        if let Some(text) = value.and_then(Value::as_str) {
                            attraction.attracted = text.into();
                        }
                    }
                    "gap" => {
                        if let Some(v) = puzzle3d_resolve_number_edit(attraction.gap, value, delta) {
                            attraction.gap = v;
                        }
                    }
                    "shift" => {
                        if let Some(v) = puzzle3d_resolve_number_edit(attraction.shift, value, delta) {
                            attraction.shift = v;
                        }
                    }
                    "rise" => {
                        if let Some(v) = puzzle3d_resolve_number_edit(attraction.rise, value, delta) {
                            attraction.rise = v;
                        }
                    }
                    "rotation" => {
                        if let Some(v) = puzzle3d_resolve_number_edit(attraction.rotation, value, delta) {
                            attraction.rotation = v;
                        }
                    }
                    "turn" => {
                        if let Some(v) = puzzle3d_resolve_number_edit(attraction.turn, value, delta) {
                            attraction.turn = v;
                        }
                    }
                    "tilt" => {
                        if let Some(v) = puzzle3d_resolve_number_edit(attraction.tilt, value, delta) {
                            attraction.tilt = v;
                        }
                    }
                    _ => {}
                }
            }
        }
        "reference" => {
            for reference in fixture.references.iter_mut().filter(|reference| id_set.contains(reference.id.as_str())) {
                match field {
                    "sourceUrl" => {
                        if let Some(text) = value.and_then(Value::as_str) {
                            reference.source.url = text.into();
                        }
                    }
                    "mediaKind" => reference.source.media_kind = value.and_then(Value::as_str).map(str::to_string),
                    "origin" => {
                        if let Some(origin) = value.and_then(value_as_vec3) {
                            reference.origin = origin;
                        }
                    }
                    "widthWorld" => {
                        if let Some(width) = puzzle3d_resolve_number_edit(reference.width_world, value, delta) {
                            reference.width_world = width;
                        }
                    }
                    _ => {
                        if let Some(axis) = puzzle3d_axis_index(field, "origin") {
                            if let Some(updated) = puzzle3d_resolve_number_edit(reference.origin[axis], value, delta) {
                                reference.origin[axis] = updated;
                            }
                        }
                    }
                }
            }
        }
        "targetVolume" => {
            for volume in fixture.target_volumes.iter_mut().filter(|volume| id_set.contains(volume.id.as_str())) {
                match field {
                    "origin" => {
                        if let Some(origin) = value.and_then(value_as_vec3) {
                            volume.origin = origin;
                        }
                    }
                    _ => {
                        if let Some(axis) = puzzle3d_axis_index(field, "origin") {
                            if let Some(updated) = puzzle3d_resolve_number_edit(volume.origin[axis], value, delta) {
                                volume.origin[axis] = updated;
                            }
                        } else if let Some(axis) = puzzle3d_axis_index(field, "scale") {
                            let mut scale = target_volume_scale_json(volume);
                            if let Some(updated) = puzzle3d_resolve_number_edit(scale[axis], value, delta) {
                                scale[axis] = updated;
                                volume.scale = Some(json!(scale));
                            }
                        } else if let Some(axis) = puzzle3d_axis_index(field, "orientation") {
                            let mut quat = volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                            if let Some(updated) = puzzle3d_resolve_number_edit(quat[axis], value, delta) {
                                quat[axis] = updated;
                                volume.orientation = Some(quat_normalize(quat));
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

/// 🎯️ Mirrors the host's client-side zoom-to-selection framing math so a keybinding/engagement-token
/// driven focus (which bypasses that host interception) still produces a sensible camera. Camera-only:
/// writes `envelope.runtime.camera` (session-only per-window state), never the shared `fixture`.
pub fn apply_puzzle3d_focus_selection(envelope: &mut Puzzle3dScene, selected_object_ids: &[String]) {
    let selected_origins: Vec<[f64; 3]> = envelope.fixture.objects.iter().filter(|object| selected_object_ids.contains(&object.id)).map(|object| object.origin).collect();
    if selected_origins.is_empty() {
        return;
    }
    let count = selected_origins.len() as f64;
    let mut center = [0.0, 0.0, 0.0];
    for origin in &selected_origins {
        center[0] += origin[0];
        center[1] += origin[1];
        center[2] += origin[2];
    }
    center = [center[0] / count, center[1] / count, center[2] / count];
    let max_distance = selected_origins
        .iter()
        .map(|origin| {
            let dx = origin[0] - center[0];
            let dy = origin[1] - center[1];
            let dz = origin[2] - center[2];
            (dx * dx + dy * dy + dz * dz).sqrt()
        })
        .fold(1.0_f64, f64::max);
    let distance = max_distance * 3.0 + 2.0;
    envelope.runtime.camera.position = [center[0] + distance * 0.6, center[1] - distance * 0.6, center[2] + distance * 0.5];
    envelope.runtime.camera.target = center;
}
//#endregion 🔖️FixtureEdits

//#region 🔖️EngineBridge
/// 🎯️ Bridges this app's own document model into `⚙️engine`'s `SceneConfig` wire shape — schema
/// translation between two independently-evolved Rust types, not a wasm-bindgen boundary.
fn scene_config_json(envelope: &Puzzle3dScene) -> String {
    json!({
        "fixture": {
            "objects": envelope.fixture.objects,
            "attractions": envelope.fixture.attractions,
            "targetVolumes": envelope.fixture.target_volumes,
        },
        "kindCatalogs": envelope.fixture.meta.kind_catalogs,
        "kindCompatibility": envelope.fixture.meta.kind_compatibility.clone().unwrap_or(json!([])),
        "overlapBudget": envelope.runtime.overlap_budget,
        "seed": 1,
        "weights": {
            "objectWeights": envelope.runtime.object_kind_weights,
            "vortexWeights": envelope.runtime.vortex_kind_weights,
        }
    })
    .to_string()
}

/// 🧊️ Scales the unit box fallback (`mesh_from_kind` extent 1.0) past the collision engine's minimum
/// brush mesh extent (2.0), otherwise its registration is a silent no-operation and brush candidates
/// never populate before a real GLB arrives.
const PUZZLE3D_FALLBACK_MESH_SCALE: f32 = 4.0;

fn scaled_mesh_positions(positions: &[f32], scale: f32) -> Vec<f32> {
    positions.iter().map(|value| value * scale).collect()
}

/// 🧊️ Pushes the current scene into the precompute session and seeds the box fallback for URLs with
/// no mesh yet, so a real GLB registered earlier via `registerBrushMesh` survives every resync.
pub fn sync_precompute_session(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    if let Ok(scene) = serde_json::from_str::<crate::artifacts::puzzle3d::schema::SceneConfig>(&scene_config_json(envelope)) {
        let _ = session.dispatch(Puzzle3dEngineCommand::SetScene { scene });
    }
    let fallback = mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND);
    let fallback_positions = scaled_mesh_positions(&fallback.positions, PUZZLE3D_FALLBACK_MESH_SCALE);
    if !session.has_mesh(PUZZLE3D_FALLBACK_MESH_KIND) {
        session.register_mesh_fallback(PUZZLE3D_FALLBACK_MESH_KIND, &fallback_positions, &fallback.indices);
    }
    for url in collect_mesh_urls(&envelope.fixture) {
        if !session.has_mesh(&url) {
            session.register_mesh_fallback(&url, &fallback_positions, &fallback.indices);
        }
    }
}

fn restored_precompute_session(envelope: &Puzzle3dScene, checkpoint: &[u8]) -> Puzzle3dPrecomputeSession {
    let mut session = Puzzle3dPrecomputeSession::new();
    sync_precompute_session(&mut session, envelope);
    session.restore_persisted_fill(checkpoint);
    session
}

pub fn sync_precompute_weights(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    let object_weights = envelope.runtime.object_kind_weights.iter().map(|(k, v)| (k.clone(), *v)).collect();
    let vortex_weights = envelope.runtime.vortex_kind_weights.iter().map(|(k, v)| (k.clone(), *v)).collect();
    let _ = session.dispatch(Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights });
}

/// ⏱️ Bounded to one small chunk per call — `handle` runs synchronously on the UI thread and the host
/// redrives this via 120ms `suggestionsTick`/`fillBuildTick` ticks, so a large per-call budget here is
/// exactly what froze the UI: hundreds of Monte-Carlo collision task units, blocking, every tick.
pub fn drive_precompute(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    sync_precompute_session(session, envelope);
    session.precompute_step_lane(crate::artifacts::puzzle3d::schema::PrecomputeLane::Brush, 8);
}

/// 🎯️ `dispatch`'s `Fixture` outcome is the precompute schema's own typed fixture shape, distinct from
/// this app's `Puzzle3dFixture` document model — bridged through one JSON round trip (schema translation
/// between two independently-evolved Rust types) exactly like `scene_config_json` bridges the other
/// direction.
pub fn fixture_from_engine_fixture(envelope: &Puzzle3dScene, fixture: &crate::artifacts::puzzle3d::schema::Fixture) -> Option<Puzzle3dScene> {
    let parsed = serde_json::to_value(fixture).ok()?;
    let mut next = envelope.clone();
    next.fixture.objects = serde_json::from_value(parsed.get("objects")?.clone()).ok()?;
    next.fixture.attractions = parsed.get("attractions").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
    next.fixture.target_volumes = parsed.get("targetVolumes").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
    Some(next)
}

#[derive(Deserialize, Clone)]
struct Puzzle3dFillDisplayPayload {
    #[serde(default)]
    objects: Vec<Puzzle3dObject>,
    #[serde(default)]
    attractions: Vec<Puzzle3dAttraction>,
}

#[derive(Clone)]
struct FillDisplayMemo {
    plan_count: u32,
    available_count: u32,
    applied_count: u32,
    payload: Puzzle3dFillDisplayPayload,
}

fn fill_display_payload_from_fixture(fixture: &crate::artifacts::puzzle3d::schema::Fixture) -> Option<Puzzle3dFillDisplayPayload> {
    serde_json::to_value(fixture).ok().and_then(|value| serde_json::from_value(value).ok())
}

fn append_fill_display_tail(fixture: &mut Puzzle3dFixture, payload: &Puzzle3dFillDisplayPayload, applied_count: u32, available_count: u32) {
    let reveal_count = (available_count - applied_count) as usize;
    let objects_tail_start = payload.objects.len().saturating_sub(reveal_count);
    fixture.objects.extend(payload.objects.iter().skip(objects_tail_start).cloned());
    let attractions_tail_start = payload.attractions.len().saturating_sub(reveal_count);
    fixture.attractions.extend(payload.attractions.iter().skip(attractions_tail_start).cloned());
}

fn puzzle3d_fixture_with_fill_display_memo(mut fixture: Puzzle3dFixture, precompute: &Puzzle3dPrecomputeSession, applied_count: u32, available_count: u32, memo: &Mutex<Option<FillDisplayMemo>>) -> Puzzle3dFixture {
    if available_count <= applied_count {
        return fixture;
    }
    let plan_count: u32 = precompute.fill_progress_summary().count as u32;
    let cached = memo.lock().ok().and_then(|guard| guard.as_ref().filter(|entry| entry.plan_count == plan_count && entry.available_count == available_count && entry.applied_count == applied_count).cloned());
    let payload = if let Some(entry) = cached {
        entry.payload
    } else {
        let payload = precompute.compose_fill_display(available_count).and_then(|engine_fixture| fill_display_payload_from_fixture(&engine_fixture));
        if let Some(payload) = payload {
            if let Ok(mut guard) = memo.lock() {
                *guard = Some(FillDisplayMemo { plan_count, available_count, applied_count, payload: payload.clone() });
            }
            payload
        } else {
            return fixture;
        }
    };
    append_fill_display_tail(&mut fixture, &payload, applied_count, available_count);
    fixture
}

//#endregion 🔖️EngineBridge

//#region 🔖️AttractionResolve
/// 📐️ Attraction placement math — a quaternion-only port of the compose kernel's `compute_child_plane`
/// so it composes directly with `Puzzle3dObject.orientation`. Every attraction is directed
/// (`attracting` → `attracted`); an attracted object's world pose is derived from the attracting
/// vortex's world pose plus the 6 connection-style parameters (gap/shift/rise/rotation/turn/tilt,
/// angles in degrees, same semantics as compose connections).
const PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE: f64 = 0.01;

fn vec3_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn vec3_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn vec3_scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

fn vec3_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn vec3_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn vec3_len(a: [f64; 3]) -> f64 {
    vec3_dot(a, a).sqrt()
}

fn vec3_normalize(a: [f64; 3]) -> [f64; 3] {
    let len = vec3_len(a);
    if len < 1e-12 {
        a
    } else {
        vec3_scale(a, 1.0 / len)
    }
}

fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}

fn quat_conjugate(q: [f64; 4]) -> [f64; 4] {
    [-q[0], -q[1], -q[2], q[3]]
}

fn quat_normalize(q: [f64; 4]) -> [f64; 4] {
    let len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if len < 1e-12 {
        [0.0, 0.0, 0.0, 1.0]
    } else {
        [q[0] / len, q[1] / len, q[2] / len, q[3] / len]
    }
}

/// 🧭️ The quaternion rotating unit vector `from` onto unit vector `to`.
fn puzzle3d_quaternion_from_unit_vectors(from: [f64; 3], to: [f64; 3]) -> [f64; 4] {
    let r = vec3_dot(from, to) + 1.0;
    let quat = if r < 0.000_001 {
        if from[0].abs() > from[2].abs() {
            [-from[1], from[0], 0.0, 0.0]
        } else {
            [0.0, -from[2], from[1], 0.0]
        }
    } else {
        let c = vec3_cross(from, to);
        [c[0], c[1], c[2], r]
    };
    quat_normalize(quat)
}

/// 🧲️ The align-quaternion special case for when the attracted vortex is already (anti)parallel to the
/// attracting vortex. Falls back to an alternate cross axis when the attracting direction is exactly
/// ±Z — a double-degenerate corner the compose kernel's own branch doesn't otherwise guard.
fn puzzle3d_attraction_align_quat(parent_dir: [f64; 3], child_dir: [f64; 3]) -> [f64; 4] {
    let reverse_child = vec3_scale(child_dir, -1.0);
    let cross_vec = vec3_cross(parent_dir, reverse_child);
    if vec3_len(cross_vec) < PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE {
        if parent_dir[2].abs() < PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE {
            puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], [0.0, 0.0, -1.0])
        } else {
            let mut axis = vec3_cross([0.0, 0.0, 1.0], parent_dir);
            if vec3_len(axis) < 1e-9 {
                axis = vec3_cross([1.0, 0.0, 0.0], parent_dir);
            }
            let axis = vec3_normalize(axis);
            let half = std::f64::consts::FRAC_PI_2;
            quat_normalize([axis[0] * half.sin(), axis[1] * half.sin(), axis[2] * half.sin(), half.cos()])
        }
    } else {
        puzzle3d_quaternion_from_unit_vectors(reverse_child, parent_dir)
    }
}

/// 📌️ Resolves an attraction endpoint (`objectId:vortexId`) to its owning object id and its vortex's
/// LOCAL (object-frame) position/direction — the frame the connector math expects, before the object's
/// own world transform is applied.
pub fn puzzle3d_local_vortex_geom(fixture: &Puzzle3dFixture, full_id: &str) -> Option<(String, [f64; 3], [f64; 3])> {
    for object in &fixture.objects {
        for vortex in &object.vortices {
            if puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id {
                return Some((object.id.clone(), vortex.position, vortex.direction.unwrap_or([0.0, 0.0, -1.0])));
            }
        }
    }
    None
}

/// 🔗️ Resolves an attraction's `attracting`/`attracted` vortex full-ids to their owning object ids.
/// Returns `None` for dangling references or same-object attractions (legal today but not a resolvable
/// directed edge).
fn puzzle3d_attraction_object_ids(fixture: &Puzzle3dFixture, attraction: &Puzzle3dAttraction) -> Option<(String, String)> {
    let attracting_object = puzzle3d_local_vortex_geom(fixture, &attraction.attracting)?.0;
    let attracted_object = puzzle3d_local_vortex_geom(fixture, &attraction.attracted)?.0;
    if attracting_object == attracted_object {
        return None;
    }
    Some((attracting_object, attracted_object))
}

/// 📐️ Forward attraction placement — given the attracting object's world pose (`t_a`/`q_a`), both
/// vortices' LOCAL position/direction, and the 6 connection-style parameters (angles in degrees),
/// returns the attracted object's world pose.
#[allow(clippy::too_many_arguments)]
fn puzzle3d_attraction_child_pose(t_a: [f64; 3], q_a: [f64; 4], p_a: [f64; 3], d_a: [f64; 3], p_b: [f64; 3], d_b: [f64; 3], gap: f64, shift: f64, rise: f64, rotation_deg: f64, turn_deg: f64, tilt_deg: f64) -> ([f64; 3], [f64; 4]) {
    let parent_dir = vec3_normalize(d_a);
    let child_dir = vec3_normalize(d_b);
    let align_q = puzzle3d_attraction_align_quat(parent_dir, child_dir);

    let pq = puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir);
    let gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0]);
    let shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0]);
    let raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0]);

    let rotate_q = quat_from_axis_angle(parent_dir[0], parent_dir[1], parent_dir[2], -deg_to_rad(rotation_deg));
    let turn_axis = quat_rotate_vector(rotate_q, raise_dir);
    let tilt_axis = quat_rotate_vector(rotate_q, shift_dir);
    let turn_q = quat_from_axis_angle(turn_axis[0], turn_axis[1], turn_axis[2], deg_to_rad(turn_deg));
    let tilt_q = quat_from_axis_angle(tilt_axis[0], tilt_axis[1], tilt_axis[2], deg_to_rad(tilt_deg));

    let mut orientation_local = quat_conjugate(align_q);
    orientation_local = quat_mul(orientation_local, quat_conjugate(rotate_q));
    orientation_local = quat_mul(orientation_local, quat_conjugate(turn_q));
    orientation_local = quat_mul(orientation_local, quat_conjugate(tilt_q));
    let orientation_local = quat_normalize(orientation_local);

    let offset = vec3_add(vec3_add(t_a, p_a), vec3_add(vec3_add(vec3_scale(gap_dir, gap), vec3_scale(shift_dir, shift)), vec3_scale(raise_dir, rise)));
    let t_b = vec3_sub(quat_rotate_vector(orientation_local, offset), p_b);
    let q_b = quat_normalize(quat_mul(orientation_local, q_a));
    (t_b, q_b)
}

/// 🔁️ Inverse of `puzzle3d_attraction_child_pose` — given the attracted object's CURRENT world pose,
/// derives the 6 parameters that reproduce it exactly, so moving/rotating an attracted object never
/// causes a resolve-triggered snap-back and creating an attraction never moves either endpoint.
#[allow(clippy::too_many_arguments)]
pub fn derive_attraction_params(t_a: [f64; 3], q_a: [f64; 4], p_a: [f64; 3], d_a: [f64; 3], p_b: [f64; 3], d_b: [f64; 3], t_b: [f64; 3], q_b: [f64; 4]) -> (f64, f64, f64, f64, f64, f64) {
    let parent_dir = vec3_normalize(d_a);
    let child_dir = vec3_normalize(d_b);
    let align_q = puzzle3d_attraction_align_quat(parent_dir, child_dir);
    let pq = puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir);
    let gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0]);
    let shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0]);
    let raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0]);

    let orientation_local = quat_normalize(quat_mul(q_b, quat_conjugate(q_a)));

    let offset = quat_rotate_vector(quat_conjugate(orientation_local), vec3_add(t_b, p_b));
    let diff = vec3_sub(vec3_sub(offset, t_a), p_a);
    let gap = vec3_dot(diff, gap_dir);
    let shift = vec3_dot(diff, shift_dir);
    let rise = vec3_dot(diff, raise_dir);

    let residual = quat_mul(align_q, orientation_local);
    let m = quat_mul(quat_mul(quat_conjugate(pq), residual), pq);
    let col_x = quat_rotate_vector(m, [1.0, 0.0, 0.0]);
    let col_y = quat_rotate_vector(m, [0.0, 1.0, 0.0]);

    let clamp = |v: f64| v.clamp(-1.0, 1.0);
    let tilt_rad = -(clamp(col_y[2])).asin();
    let (rotation_rad, turn_rad) = if (col_y[2].abs() - 1.0).abs() < 1e-6 {
        (col_x[1].atan2(col_x[0]), 0.0)
    } else {
        let col_z = quat_rotate_vector(m, [0.0, 0.0, 1.0]);
        ((-col_x[2]).atan2(col_z[2]), col_y[0].atan2(col_y[1]))
    };

    (gap, shift, rise, rad_to_deg(rotation_rad), rad_to_deg(turn_rad), rad_to_deg(tilt_rad))
}

/// 🌲️ Resolves every attracted object's world pose from its attracting root, over a directed BFS per
/// weakly-connected component. Roots are in-degree-zero objects; a component that is a pure cycle (the
/// "donut" case) picks the lexicographically smallest object id in that component as a deterministic
/// root. Multiple incoming attractions to the same object are resolved first-visit-wins. Idempotent:
/// re-running against already-resolved poses reproduces them exactly. Returns, for every non-root
/// object touched, the attraction index that positioned it — callers (e.g. `translateSelection`) use
/// this to rederive params before a direct move so resolving never snaps it back.
pub fn resolve_puzzle3d_attractions(fixture: &mut Puzzle3dFixture) -> HashMap<String, usize> {
    let mut edges: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut all_object_ids: Vec<String> = fixture.objects.iter().map(|object| object.id.clone()).collect();
    all_object_ids.sort();
    for id in &all_object_ids {
        in_degree.entry(id.clone()).or_insert(0);
    }
    for (index, attraction) in fixture.attractions.iter().enumerate() {
        if let Some((attracting_id, attracted_id)) = puzzle3d_attraction_object_ids(fixture, attraction) {
            edges.entry(attracting_id).or_default().push((attracted_id.clone(), index));
            *in_degree.entry(attracted_id).or_insert(0) += 1;
        }
    }

    fn find(parent_of: &mut HashMap<String, String>, id: &str) -> String {
        let mut current = id.to_string();
        while parent_of[&current] != current {
            let grandparent = parent_of[&parent_of[&current]].clone();
            parent_of.insert(current.clone(), grandparent.clone());
            current = grandparent;
        }
        current
    }
    fn union(parent_of: &mut HashMap<String, String>, a: &str, b: &str) {
        let root_a = find(parent_of, a);
        let root_b = find(parent_of, b);
        if root_a != root_b {
            parent_of.insert(root_a, root_b);
        }
    }
    let mut parent_of: HashMap<String, String> = all_object_ids.iter().map(|id| (id.clone(), id.clone())).collect();
    for (attracting_id, targets) in &edges {
        for (attracted_id, _) in targets {
            union(&mut parent_of, attracting_id, attracted_id);
        }
    }

    let mut components: HashMap<String, Vec<String>> = HashMap::new();
    for id in &all_object_ids {
        let root = find(&mut parent_of, id);
        components.entry(root).or_default().push(id.clone());
    }
    let mut component_keys: Vec<String> = components.keys().cloned().collect();
    component_keys.sort();

    let mut incoming: HashMap<String, usize> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();

    for component_key in component_keys {
        let mut members = components.remove(&component_key).unwrap_or_default();
        members.sort();
        let roots: Vec<String> = members.iter().filter(|id| in_degree.get(id.as_str()).copied().unwrap_or(0) == 0).cloned().collect();
        let seed_roots: Vec<String> = if roots.is_empty() { vec![members[0].clone()] } else { roots };

        let mut queue: std::collections::VecDeque<String> = std::collections::VecDeque::new();
        for root in &seed_roots {
            if visited.insert(root.clone()) {
                queue.push_back(root.clone());
            }
        }
        while let Some(current_id) = queue.pop_front() {
            let Some(targets) = edges.get(&current_id) else { continue };
            for (attracted_id, attraction_index) in targets.clone() {
                if visited.contains(&attracted_id) {
                    continue;
                }
                let attraction = fixture.attractions[attraction_index].clone();
                let (Some((_, p_a, d_a)), Some((_, p_b, d_b))) = (puzzle3d_local_vortex_geom(fixture, &attraction.attracting), puzzle3d_local_vortex_geom(fixture, &attraction.attracted)) else { continue };
                let Some(attracting_object) = fixture.objects.iter().find(|object| object.id == current_id) else { continue };
                let t_a = attracting_object.origin;
                let q_a = attracting_object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                let (t_b, q_b) = puzzle3d_attraction_child_pose(t_a, q_a, p_a, d_a, p_b, d_b, attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt);
                if let Some(attracted_object) = fixture.objects.iter_mut().find(|object| object.id == attracted_id) {
                    attracted_object.origin = t_b;
                    attracted_object.orientation = Some(q_b);
                }
                incoming.insert(attracted_id.clone(), attraction_index);
                visited.insert(attracted_id.clone());
                queue.push_back(attracted_id);
            }
        }
    }
    incoming
}

/// 🧰️ Rederives every attraction's 6 params from its endpoints' CURRENT poses. Used after merging
/// externally computed poses (brush/fill placement via the collision-aware engine, which knows nothing
/// about gap/shift/rise/rotation/turn/tilt) so the follow-up resolve reproduces those poses exactly
/// instead of re-deriving a bare port-to-port docking that could visibly jump the just-placed object.
pub fn puzzle3d_rederive_all_attractions(fixture: &mut Puzzle3dFixture) {
    let ids: Vec<String> = fixture.attractions.iter().map(|attraction| attraction.id.clone()).collect();
    for id in ids {
        let Some(attraction) = fixture.attractions.iter().find(|attraction| attraction.id == id).cloned() else { continue };
        let (Some((attracting_id, p_a, d_a)), Some((attracted_id, p_b, d_b))) = (puzzle3d_local_vortex_geom(fixture, &attraction.attracting), puzzle3d_local_vortex_geom(fixture, &attraction.attracted)) else { continue };
        let pose = |object_id: &str| fixture.objects.iter().find(|object| object.id == object_id).map(|object| (object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])));
        let (Some((t_a, q_a)), Some((t_b, q_b))) = (pose(&attracting_id), pose(&attracted_id)) else { continue };
        let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(t_a, q_a, p_a, d_a, p_b, d_b, t_b, q_b);
        if let Some(attraction) = fixture.attractions.iter_mut().find(|attraction| attraction.id == id) {
            attraction.gap = gap;
            attraction.shift = shift;
            attraction.rise = rise;
            attraction.rotation = rotation;
            attraction.turn = turn;
            attraction.tilt = tilt;
        }
    }
}

/// ✋️ After a direct move/rotate on selected objects, rederives the 6 params of every moved object's
/// incoming attraction (per the `incoming` map from a prior `resolve_puzzle3d_attractions` call) from
/// its NEW pose, so the follow-up resolve reproduces that pose exactly instead of snapping the object
/// back to its old one.
pub fn puzzle3d_rederive_moved_attractions(fixture: &mut Puzzle3dFixture, moved_ids: &[String], incoming: &HashMap<String, usize>) {
    for object_id in moved_ids {
        let Some(&attraction_index) = incoming.get(object_id) else { continue };
        let Some(attraction) = fixture.attractions.get(attraction_index).cloned() else { continue };
        let (Some((attracting_id, p_a, d_a)), Some((_, p_b, d_b))) = (puzzle3d_local_vortex_geom(fixture, &attraction.attracting), puzzle3d_local_vortex_geom(fixture, &attraction.attracted)) else { continue };
        let Some(t_a_q_a) = fixture.objects.iter().find(|object| object.id == attracting_id).map(|object| (object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]))) else { continue };
        let Some(t_b_q_b) = fixture.objects.iter().find(|object| &object.id == object_id).map(|object| (object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]))) else { continue };
        let (t_a, q_a) = t_a_q_a;
        let (t_b, q_b) = t_b_q_b;
        let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(t_a, q_a, p_a, d_a, p_b, d_b, t_b, q_b);
        if let Some(attraction) = fixture.attractions.get_mut(attraction_index) {
            attraction.gap = gap;
            attraction.shift = shift;
            attraction.rise = rise;
            attraction.rotation = rotation;
            attraction.turn = turn;
            attraction.tilt = tilt;
        }
    }
}
//#endregion 🔖️AttractionResolve

//#region 🔖️Distribution
/// 🎲️ Nested object/vortex distribution — one group per object kind (header slider = P(object)),
/// vortex children are the **global** vortex catalog shown as joint P(object)×P(vortex). Moving an
/// object header scales its children; the sum of every nested joint across all objects is 1. Shared by
/// the Fill tool and the Brush utility options, so it lives here rather than in either of them.
pub fn puzzle3d_uniform_kind_weights(ids: &[String]) -> HashMap<String, f64> {
    if ids.is_empty() {
        return HashMap::new();
    }
    let weight = 1.0 / ids.len() as f64;
    ids.iter().map(|id| (id.clone(), weight)).collect()
}

pub fn puzzle3d_normalize_kind_weight_group(weights: &HashMap<String, f64>, kind_ids: &[String], changed_id: &str, new_value: f64) -> HashMap<String, f64> {
    if kind_ids.is_empty() {
        return HashMap::new();
    }
    if kind_ids.len() == 1 {
        return HashMap::from([(kind_ids[0].clone(), 1.0)]);
    }
    let new_value = new_value.clamp(0.0, 1.0);
    let others: Vec<&String> = kind_ids.iter().filter(|id| id.as_str() != changed_id).collect();
    let remainder = (1.0 - new_value).max(0.0);
    let other_sum: f64 = others.iter().map(|id| weights.get(*id).copied().unwrap_or(0.0)).sum();
    let mut next = HashMap::new();
    next.insert(changed_id.to_string(), new_value);
    if remainder <= f64::EPSILON {
        for id in others {
            next.insert((*id).clone(), 0.0);
        }
        return next;
    }
    if other_sum <= f64::EPSILON {
        let each = remainder / others.len() as f64;
        for id in others {
            next.insert((*id).clone(), each);
        }
    } else {
        for id in others {
            let old = weights.get(id).copied().unwrap_or(0.0);
            next.insert((*id).clone(), old / other_sum * remainder);
        }
    }
    next
}

pub fn puzzle3d_ensure_catalog_kind_weights(weights: &mut HashMap<String, f64>, kind_ids: &[String]) {
    if kind_ids.is_empty() {
        return;
    }
    if weights.is_empty() || kind_ids.iter().any(|id| !weights.contains_key(id)) {
        *weights = puzzle3d_uniform_kind_weights(kind_ids);
        return;
    }
    let sum: f64 = kind_ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum();
    if (sum - 1.0).abs() > 0.001 {
        for id in kind_ids {
            if let Some(weight) = weights.get_mut(id) {
                *weight /= sum;
            }
        }
    }
}

fn puzzle3d_object_kind_label(fixture: &Puzzle3dFixture, object_kind_id: &str) -> String {
    puzzle3d_catalog_entries(fixture, "objects")
        .iter()
        .find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(object_kind_id))
        .and_then(|entry| entry.get("label").and_then(|value| value.as_str()).or_else(|| entry.get("name").and_then(|value| value.as_str())))
        .unwrap_or(object_kind_id)
        .to_string()
}

pub fn puzzle3d_joint_vortex_weight(object_weight: f64, vortex_weight: f64) -> f64 {
    object_weight * vortex_weight
}

/// 🎲️ Vortex-kind sliders under an object row — displayed value is the **final** joint percentage
/// `P(object) × P(vortex)`. Every **global** vortex kind is listed under each object so the sum of all
/// nested joint percentages across the tree is 1 (not a local simplex per object). Editing converts
/// back to relative `P(vortex)` on the shared vortex simplex. Disabled when the parent object weight
/// is 0. Step tracks ~1% of the object weight for a smooth `[0, P(object)]` range.
pub fn puzzle3d_joint_vortex_measures(object_kind_id: &str, object_weight: f64, vortex_kind_ids: &[String], vortex_weights: &HashMap<String, f64>) -> Vec<WindowMeasure> {
    let object_kind_zero = object_weight <= f64::EPSILON;
    let joint_max = if object_kind_zero { 1.0 } else { object_weight };
    let joint_step = if object_kind_zero { 0.01 } else { (object_weight * 0.01).max(0.0001) };
    let fallback = if vortex_kind_ids.is_empty() { 0.0 } else { 1.0 / vortex_kind_ids.len() as f64 };
    vortex_kind_ids
        .iter()
        .map(|vortex_kind_id| {
            let vortex_weight = vortex_weights.get(vortex_kind_id).copied().unwrap_or(fallback);
            let joint = puzzle3d_joint_vortex_weight(object_weight, vortex_weight);
            WindowMeasure::Slider {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-joint-vortex-{object_kind_id}-{vortex_kind_id}"),
                label: Some(vortex_kind_id.clone()),
                value: joint,
                min: 0.0,
                max: joint_max,
                step: Some(joint_step),
                ready: None,
                loading: None,
                waiting: None,
                disabled: if object_kind_zero { Some(true) } else { None },
                reveal: None,
                on_change: puzzle3d_action("setVortexKindWeight", Some(json!({ "kindId": vortex_kind_id, "objectKindId": object_kind_id }))),
            }
        })
        .collect()
}

pub fn puzzle3d_distribution_children(envelope: &Puzzle3dScene, default_open: Option<bool>) -> Vec<WindowMeasure> {
    let object_ids = puzzle3d_kind_ids(&envelope.fixture, "objects");
    let vortex_kind_ids = puzzle3d_kind_ids(&envelope.fixture, "vortices");
    object_ids
        .iter()
        .map(|object_kind_id| {
            let object_weight = envelope.runtime.object_kind_weights.get(object_kind_id).copied().unwrap_or_else(|| if object_ids.is_empty() { 0.0 } else { 1.0 / object_ids.len() as f64 });
            let label = puzzle3d_object_kind_label(&envelope.fixture, object_kind_id);
            WindowMeasure::Group {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution-object-{object_kind_id}"),
                label,
                default_open,
                active_utility_id: None,
                value: Some(object_weight),
                min: Some(0.0),
                max: Some(1.0),
                step: Some(0.01),
                ready: None,
                loading: None,
                waiting: None,
                on_change: Some(puzzle3d_action("setObjectKindWeight", Some(json!({ "kindId": object_kind_id })))),
                children: puzzle3d_joint_vortex_measures(object_kind_id, object_weight, &vortex_kind_ids, &envelope.runtime.vortex_kind_weights),
            }
        })
        .collect()
}

pub fn puzzle3d_distribution_group(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels, default_open: Option<bool>) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution"),
        label: labels.distribution.into(),
        default_open,
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: puzzle3d_distribution_children(envelope, Some(false)),
    }
}
//#endregion 🔖️Distribution

//#region 🔖️UiScopes
pub fn puzzle3d_viewport_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false }
}

/// 🐢️ Background fill planning only mutates the main world body's `fillBuild` interaction JSON and the
/// fill-count slider range in the fill tool's measures — never panels, engagements, window measures or
/// labels. Emitting `Full` on every 120ms tick was half of the fill-utility stall.
pub fn puzzle3d_fill_build_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: Vec::new(), utilities: false, tools: true, engagements: false, measures: false, labels: false }
}

/// 🐢️ Fill/distribution slider gestures refresh the world body, fill-tool measures and utility-option
/// window measures — never the full shell chrome.
pub fn puzzle3d_fill_options_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: Vec::new(), utilities: false, tools: true, engagements: false, measures: true, labels: false }
}

/// 🐢️ Suggestion collision ticking only refreshes the world body's suggestion-menu interaction JSON.
pub fn puzzle3d_suggestions_tick_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false }
}

/// 🐢️ Mid-drag gumball scratch only refreshes the world composite body — never the full shell.
pub fn puzzle3d_transform_drag_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false }
}

//#endregion 🔖️UiScopes

//#region 🔖️Puzzle3dCommand
/// @emoji 🎯️ B1: `Puzzle3dPlayApp::Command` — the SOLE dispatch surface, one variant per declared
/// action (mirrors every `.mutation(...)`/`.view_action(...)` id `create_puzzle3d_app` registers
/// below). Each variant carries `window_id` (was host-pushed `view_state.window_id`) plus `args` (the
/// action's original `{...}` JSON payload, unchanged) — `handle` reconstructs the exact
/// `(action, args, window_id)` triple every `🎮️commands/*` arm expects, so each arm's internal
/// `args.get("field")` extraction stays byte-for-byte identical to the pre-B1 implementation.
///
/// ⚠️ `OpBinary` is a plain JSON-bytes bridge (NOT `#[derive(dsl::DslOps)]`, and NOT the framework's
/// `app_commands!` macro): a generic `args: Value` field is not representable in the DSL grammar those
/// target, so adopting them would silently rewrite this app's wire format. Keep this macro's variant
/// list, its order and its action-id literals byte-for-byte stable.
macro_rules! puzzle3d_command_variants {
    ($($Variant:ident = $id:tt),* $(,)?) => {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Puzzle3dCommand {
            $($Variant { window_id: Option<String>, args: Option<Value> }),*
        }

        impl Puzzle3dCommand {
            /// 🏷️ The action id this variant was declared under — used both for `command_id()`
            /// (command-log labeling / registry kind-discipline) and to reconstruct the exact
            /// `action: &str` `handle` dispatches on.
            fn action_id(&self) -> &'static str {
                match self {
                    $(Puzzle3dCommand::$Variant { .. } => $id),*
                }
            }

            fn window_id(&self) -> Option<&str> {
                match self {
                    $(Puzzle3dCommand::$Variant { window_id, .. } => window_id.as_deref()),*
                }
            }

            fn args(&self) -> Option<&Value> {
                match self {
                    $(Puzzle3dCommand::$Variant { args, .. } => args.as_ref()),*
                }
            }

            /// 🎯️ Reverse of `action_id()` — builds the typed command used by both the host's
            /// transitional `{action,args}` bridge and the testkit dispatch helper.
            fn from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Option<Self> {
                match action {
                    $($id => Some(Puzzle3dCommand::$Variant { window_id, args })),*,
                    _ => None,
                }
            }
        }
    };
}

puzzle3d_command_variants! {
    OpenAddObjectDialog = "openAddObjectDialog",
    TransformBegin = "transformBegin",
    TransformEnd = "transformEnd",
    TranslateSelection = "translateSelection",
    RotateSelection = "rotateSelection",
    ScaleSelection = "scaleSelection",
    SetFixtureJson = "setFixtureJson",
    SetActiveExample = "setActiveExample",
    SetActiveUtility = SET_ACTIVE_UTILITY_ACTION_ID,
    SetActiveTool = SET_ACTIVE_TOOL_ACTION_ID,
    AddObjectKind = "addObjectKind",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    SelectSameKindSelection = "selectSameKindSelection",
    SetCamera = "setCamera",
    SetProjection = "setProjection",
    SetProjectionParam = "setProjectionParam",
    SetVortexShow = "setVortexShow",
    SetVortexDirection = "setVortexDirection",
    RelocateTargetVolume = "relocateTargetVolume",
    WorldRelocate = "worldRelocate",
    ToggleSun = "toggleSun",
    SetSunAzimuth = "setSunAzimuth",
    SetSunElevation = "setSunElevation",
    SetSunIntensity = "setSunIntensity",
    SetLodAutomatic = "setLodAutomatic",
    SetLodDepthVariable = "setLodDepthVariable",
    SetGridVisible = "setGridVisible",
    SetLodManual = "setLodManual",
    SetGridSnapEnabled = "setGridSnapEnabled",
    SetGridSpacing = "setGridSpacing",
    SetProximityRadius = "setProximityRadius",
    SetChunkSize = "setChunkSize",
    SetSelectableKind = "setSelectableKind",
    SetSelectionFlag = "setSelectionFlag",
    PatchInspector = "patchInspector",
    FocusSelection = "focusSelection",
    EngagementInput = "engagementInput",
    EngagementSubmit = "engagementSubmit",
    EngagementRepeatLast = "engagementRepeatLast",
    EngagementAbort = "engagementAbort",
    CreateAttraction = "createAttraction",
    DeleteAttraction = "deleteAttraction",
    SetTransformGumballFlag = "setTransformGumballFlag",
    SetVoxelDims = "setVoxelDims",
    AddTargetVolume = "addTargetVolume",
    DeleteTargetVolume = "deleteTargetVolume",
    SetTargetVolumeFlag = "setTargetVolumeFlag",
    EngagementControlSelect = "engagementControlSelect",
    AddBrushObject = "addBrushObject",
    SetFillCount = "setFillCount",
    SetFillCountStep = "setFillCountStep",
    SetBrushPlacementOverlapBudget = "setBrushPlacementOverlapBudget",
    SetObjectKindWeight = "setObjectKindWeight",
    SetVortexKindWeight = "setVortexKindWeight",
    CycleBrushCandidate = "cycleBrushCandidate",
    CycleBrushCandidateBack = "cycleBrushCandidateBack",
    OpenVortexSuggestions = "openVortexSuggestions",
    CloseVortexSuggestions = "closeVortexSuggestions",
    HoverSuggestion = "hoverSuggestion",
    AcceptSuggestion = "acceptSuggestion",
    SuggestionsTick = "suggestionsTick",
    FillBuildTick = "fillBuildTick",
    RegisterBrushMesh = "registerBrushMesh",
    WorldPointerDown = "worldPointerDown",
    // 🗣️ B1: locale/terminology used to be host-pushed `ViewState` fields with no app-level action of
    // their own; now that `ViewState` is gone from the app-facing surface, they need a real Command.
    SetLocale = "setLocale",
    SetTerminology = "setTerminology",
}

impl protocol::OpBinary for Puzzle3dCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}
//#endregion 🔖️Puzzle3dCommand

//#region 🔖️ActionContext
/// 🎬️ Everything one `🎮️commands/*` arm may read or write. The prologue/epilogue around the dispatch
/// match (window-option materialization, precompute sync, chrome effects, delta computation, config
/// snapshotting) stays in [`Puzzle3dPlayApp::handle_action_impl`]; an arm only mutates this bundle.
pub struct Puzzle3dActionCtx<'a> {
    /// 🧠️ The app's long-lived precompute session and gumball scratch — every arm reaching them goes
    /// through `borrow_mut()`.
    pub app: &'a Puzzle3dPlayApp,
    pub scene: &'a mut Puzzle3dScene,
    /// 🪟️ The window instance this action targets (already defaulted to the main window).
    pub window_id: &'a str,
    /// 🎛️ The pre-action config snapshot, for the few arms that must read state the scene runtime's
    /// materialized copy does not carry.
    pub config: &'a Puzzle3dConfig,
    /// 🕹️ Read-only view of the framework-owned `vortex` interaction domain (current selection —
    /// ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). Retained selection-acting verbs
    /// (delete/duplicate/focus/rotate/scale/translate-selection, select-same-kind,
    /// set-selection-flag, engagement-control-select) read `.selection(PUZZLE3D_INTERACTION_DOMAIN)`
    /// here instead of the deleted `Puzzle3dConfig` selection fields.
    pub interaction: &'a InteractionView<'a>,
    pub ui_scope: &'a mut UiDirtyScope,
    pub effects: &'a mut Vec<Effect>,
    /// 🛑️ Set by an arm that must skip the whole epilogue (window save, delta, config snapshot).
    pub abort: bool,
}

impl<'a> Puzzle3dActionCtx<'a> {
    /// 🕹️ The current `vortex`-domain selected ids, only when `granularity` is `granularity_id` —
    /// every retained selection-acting verb (rotate/scale/translate/delete/duplicate-selection,
    /// set-selection-flag, select-same-kind) reads exactly one granularity's ids this way, since a
    /// `DomainSelection` only ever carries one active granularity at a time.
    fn selected_ids(&self, granularity_id: &str) -> Vec<String> {
        let selection = semio_framework::io::resolve_ready(self.interaction.selection(PUZZLE3D_INTERACTION_DOMAIN));
        if selection.granularity == granularity_id {
            selection.ids.clone()
        } else {
            Vec::new()
        }
    }
    pub fn selected_object_ids(&self) -> Vec<String> {
        self.selected_ids(PUZZLE3D_GRANULARITY_OBJECT)
    }
    pub fn selected_vortex_ids(&self) -> Vec<String> {
        self.selected_ids(PUZZLE3D_GRANULARITY_VORTEX)
    }
    pub fn selected_attraction_ids(&self) -> Vec<String> {
        self.selected_ids(PUZZLE3D_GRANULARITY_ATTRACTION)
    }
    pub fn selected_target_volume_ids(&self) -> Vec<String> {
        self.selected_ids(PUZZLE3D_GRANULARITY_TARGET_VOLUME)
    }
    pub fn selected_reference_ids(&self) -> Vec<String> {
        self.selected_ids(PUZZLE3D_GRANULARITY_REFERENCE)
    }
}
/// 🏷️ Admits dynamic puzzle labels into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref().to_string())
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d label admission failed"))
}

/// 🌳️ Admits fallibly assembled puzzle nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        nodes.try_push(value?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d node admission failed"))?;
    }
    Ok(nodes)
}
//#endregion 🔖️ActionContext

//#region 🔖️ContextMenu
/// 🖱️ Bespoke row builder — every row here carries a localized (`Puzzle3dLabels`) label/icon that the
/// declared `ActionDefinition` (English-only) cannot resolve, so each row is emitted via `Menu::item`
/// rather than `Menu::action`. Grouping/ordering/the pre-destructive separator are still handled by
/// `Menu::group` + the `organize_context_menu` funnel in `context_menu`.
fn puzzle3d_context_menu_row(id: &str, label: impl Into<String>, icon: &str, action: &str, args: Option<Value>, destructive: bool) -> semio_framework_plugin::ContextMenuItemSpec {
    semio_framework_plugin::ContextMenuItemSpec {
        id: id.into(),
        label: Some(label.into()),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args: semio_framework_plugin::optional_json_to_dsl(args),
        destructive: destructive.then_some(true),
        ..Default::default()
    }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the per-granularity ids
/// `ContextMenuRequest.surface.selection` carries — the CLIENT-supplied, always-available substitute
/// for `runtime.selection` at context-menu time (unlike `render`, `context_menu` never had a live
/// config selection to read even before this ticket; `ContextMenuSurfaceTarget.selection` is the
/// framework's own sanctioned channel for it).
#[derive(Default)]
struct Puzzle3dContextSelection {
    object_ids: Vec<String>,
    vortex_ids: Vec<String>,
    attraction_ids: Vec<String>,
    target_volume_ids: Vec<String>,
    reference_ids: Vec<String>,
}

impl Puzzle3dContextSelection {
    fn from_surface(surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Self {
        let mut out = Self::default();
        let Some(surface) = surface else {
            return out;
        };
        for group in &surface.selection {
            let ids = group.ids.clone();
            match group.domain.as_str() {
                "node" | PUZZLE3D_GRANULARITY_OBJECT => out.object_ids.extend(ids),
                PUZZLE3D_GRANULARITY_VORTEX => out.vortex_ids.extend(ids),
                PUZZLE3D_GRANULARITY_ATTRACTION => out.attraction_ids.extend(ids),
                PUZZLE3D_GRANULARITY_TARGET_VOLUME => out.target_volume_ids.extend(ids),
                PUZZLE3D_GRANULARITY_REFERENCE => out.reference_ids.extend(ids),
                _ => {}
            }
        }
        out
    }
}

fn puzzle3d_context_menu_items(envelope: &Puzzle3dScene, selection: &Puzzle3dContextSelection, labels: &Puzzle3dLabels, registry: &semio_framework_plugin::AppActionRegistry) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::Menu;
    if !selection.object_ids.is_empty() {
        let all_hidden = envelope.fixture.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).all(|object| object.hidden);
        let all_locked = envelope.fixture.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).all(|object| object.locked);
        let count = selection.object_ids.len();
        let phrase = if count == 1 { format!("1 {}", labels.object.as_str()) } else { format!("{count} {}", labels.objects.as_str()) };
        return semio_framework::io::resolve_ready(async {
            Menu::of(registry)
                .await
                .item(puzzle3d_context_menu_row("duplicate", labels.duplicate, "copy", "duplicateSelection", None, false))
                .await
                .item(puzzle3d_context_menu_row("select-same-kind", labels.select_same_kind, "layers", "selectSameKindSelection", None, false))
                .await
                .item(puzzle3d_context_menu_row("zoom", labels.zoom_to_selection, "crosshair", "zoomToSelection", None, false))
                .await
                .group("hand", |m| async {
                    m.item(puzzle3d_context_menu_row("hide-show", if all_hidden { labels.show } else { labels.hide }, if all_hidden { "eye" } else { "eye-off" }, "setSelectionFlag", Some(json!({ "flag": "hidden", "value": !all_hidden })), false))
                        .await
                        .item(puzzle3d_context_menu_row(
                            "lock-unlock",
                            if all_locked { labels.unlock } else { labels.lock },
                            if all_locked { "lock-open" } else { "lock" },
                            "setSelectionFlag",
                            Some(json!({ "flag": "locked", "value": !all_locked })),
                            false,
                        ))
                        .await
                })
                .await
                .item(puzzle3d_context_menu_row("delete", format!("{} ({phrase})", labels.delete.as_str()), "trash", "deleteSelection", None, true))
                .await
                .build()
                .await
        });
    }
    if !selection.vortex_ids.is_empty() {
        let mut menu = semio_framework::io::resolve_ready(Menu::of(registry));
        if let [only] = selection.vortex_ids.as_slice() {
            menu = semio_framework::io::resolve_ready(menu.item(puzzle3d_context_menu_row("suggest", labels.suggest_objects, "sparkles", "openVortexSuggestions", Some(json!({ "fullId": only })), false)));
        }
        return semio_framework::io::resolve_ready(async {
            menu.item(puzzle3d_context_menu_row("zoom", labels.zoom_to_selection, "crosshair", "zoomToSelection", None, false)).await.item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteSelection", None, true)).await.build().await
        });
    }
    if let Some(id) = selection.attraction_ids.first() {
        return semio_framework::io::resolve_ready(async { Menu::of(registry).await.item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteAttraction", Some(json!({ "id": id })), true)).await.build().await });
    }
    if let Some(id) = selection.target_volume_ids.first() {
        let target_volume = envelope.fixture.target_volumes.iter().find(|volume| &volume.id == id);
        let hidden = target_volume.is_some_and(|volume| volume.hidden);
        let locked = target_volume.is_some_and(|volume| volume.locked);
        return semio_framework::io::resolve_ready(async {
            Menu::of(registry)
                .await
                .group("targets", |m| async {
                    m.item(puzzle3d_context_menu_row("hide-show", if hidden { labels.show } else { labels.hide }, if hidden { "eye" } else { "eye-off" }, "setTargetVolumeFlag", Some(json!({ "id": id, "flag": "hidden", "value": !hidden })), false))
                        .await
                        .item(puzzle3d_context_menu_row(
                            "lock-unlock",
                            if locked { labels.unlock } else { labels.lock },
                            if locked { "lock-open" } else { "lock" },
                            "setTargetVolumeFlag",
                            Some(json!({ "id": id, "flag": "locked", "value": !locked })),
                            false,
                        ))
                        .await
                })
                .await
                .item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteTargetVolume", Some(json!({ "id": id })), true))
                .await
                .build()
                .await
        });
    }
    if selection.reference_ids.first().is_some() {
        return semio_framework::io::resolve_ready(async {
            Menu::of(registry)
                .await
                .item(puzzle3d_context_menu_row("zoom", labels.zoom_to_selection, "crosshair", "zoomToSelection", None, false))
                .await
                .item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteSelection", None, true))
                .await
                .build()
                .await
        });
    }
    Vec::new()
}
//#endregion 🔖️ContextMenu

//#region 🔖️PlayApp
// 🧩️ Puzzle-3d play app. Owns the precompute engine and the gumball scratch session; the persisted
// document (bare `Puzzle3dFixture` json) lives in the wrapping `VcsArtifactApp`'s operation store and
// the view state in `Puzzle3dConfig`. Each action rehydrates the engine from the projection, mutates
// a transient [`Puzzle3dScene`], then emits the granular operation delta.
//
// 🧲️ Gumball drags use a scratch-commit session (`transform_drag_active` + `transform_base` /
// `transform_scratch`): mid-drag ticks accumulate incremental deltas onto the scratch and emit no
// operations; `transformEnd` commits the base→scratch fixture delta once.
/// 🧠 Long-lived worker-portable play session — `ArtifactApp` methods are associated fns (no
/// `&self`), so the precompute/gumball scratch lives behind one process-owned lock until
/// `EngineHandles` carries it. A thread-local session loses resumable fill state whenever the
/// shared worker pool resumes a command on another worker.
fn with_puzzle3d_app_for<R>(config: &Puzzle3dConfig, f: impl FnOnce(&Puzzle3dPlayApp) -> R) -> R {
    let app = Puzzle3dPlayApp::default();
    if !config.fill_checkpoint.is_empty() {
        app.precompute.borrow_mut().restore_persisted_fill(&config.fill_checkpoint);
    }
    f(&app)
}

fn with_puzzle3d_app<R>(f: impl FnOnce(&Puzzle3dPlayApp) -> R) -> R {
    let app = Puzzle3dPlayApp::default();
    f(&app)
}

pub(crate) fn with_puzzle3d_app_mut<R>(f: impl FnOnce(&mut Puzzle3dPlayApp) -> R) -> R {
    let mut app = Puzzle3dPlayApp::default();
    f(&mut app)
}

pub struct Puzzle3dPlayApp {
    pub(crate) precompute: std::cell::RefCell<Puzzle3dPrecomputeSession>,
    pub(crate) transform_drag_active: std::cell::RefCell<bool>,
    pub(crate) transform_base: std::cell::RefCell<Option<Puzzle3dFixture>>,
    pub(crate) transform_scratch: std::cell::RefCell<Option<Puzzle3dFixture>>,
    /// 👻️ Per-`key` monotone counter for `gesture_preview` — see `//#region 🔖️GesturePreview`.
    preview_seq: std::cell::RefCell<u64>,
    fill_display_memo: Mutex<Option<FillDisplayMemo>>,
    geometry_cache: Mutex<Option<(u64, String, String)>>,
    document_tree_cache: Mutex<Option<(u64, BuiltNode)>>,
}

impl Default for Puzzle3dPlayApp {
    fn default() -> Self {
        Self {
            precompute: std::cell::RefCell::new(Puzzle3dPrecomputeSession::new()),
            transform_drag_active: std::cell::RefCell::new(false),
            transform_base: std::cell::RefCell::new(None),
            transform_scratch: std::cell::RefCell::new(None),
            preview_seq: std::cell::RefCell::new(0),
            fill_display_memo: Mutex::new(None),
            geometry_cache: Mutex::new(None),
            document_tree_cache: Mutex::new(None),
        }
    }
}

impl Puzzle3dPlayApp {
    fn geometry_jsons(&self, fixture: &Puzzle3dFixture) -> (String, String) {
        let fingerprint = main::fixture_geometry_fingerprint(fixture);
        let mut cache = self.geometry_cache.lock().expect("geometry cache");
        if cache.as_ref().is_none_or(|(fp, _, _)| *fp != fingerprint) {
            *cache = Some((fingerprint, main::world_instances_geometry_json(fixture), main::world_meshes_json(fixture)));
            *self.document_tree_cache.lock().expect("document cache") = None;
        }
        let (_, instances, meshes) = cache.as_ref().expect("geometry cache populated");
        (instances.clone(), meshes.clone())
    }

    fn document_tree_cached(&self, fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
        document::render(fixture, labels)
    }

    /// 🎬️ Snapshots the live fixture as the gumball drag base and clears any prior scratch.
    fn begin_transform_session(&self, projection: &Value) {
        let fixture = serde_json::from_value::<Puzzle3dFixture>(projection.clone()).unwrap_or_else(|_| empty_fixture());
        *self.transform_drag_active.borrow_mut() = true;
        *self.transform_base.borrow_mut() = Some(fixture);
        *self.transform_scratch.borrow_mut() = None;
    }

    /// 🧹️ Drops an in-progress gumball scratch without committing.
    pub(crate) fn clear_transform_session(&self) {
        *self.transform_drag_active.borrow_mut() = false;
        *self.transform_base.borrow_mut() = None;
        *self.transform_scratch.borrow_mut() = None;
    }

    /// 🧲️ One mid-drag gumball tick: accumulates an incremental delta onto `transform_scratch`
    /// (seeded from the drag-start base) and emits zero operations (scratch-commit pattern b).
    ///
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: takes the already-resolved
    /// target ids rather than `&InteractionView` directly — `InteractionView` has no public
    /// constructor (its fields are `pub(crate)` to `semio_framework_plugin`, flagged to the
    /// coordinator as a testability gap), so this inherent method stays constructible from this
    /// crate's own in-file tests; `handle_action_impl` resolves the ids from `interaction` before
    /// calling in.
    pub(crate) fn transform_drag_tick(&self, action: &str, args: Option<&Value>, projection: &Value, object_ids: &[String], volume_ids: &[String]) -> Emit<Puzzle3dMutation, Puzzle3dConfigMutation> {
        if self.transform_base.borrow().is_none() {
            self.begin_transform_session(projection);
        }
        let object_ids = if volume_ids.is_empty() { mesh_selection_ids(args, object_ids) } else { Vec::new() };
        let mut scratch = self.transform_scratch.borrow().clone().or_else(|| self.transform_base.borrow().clone()).unwrap_or_else(empty_fixture);
        let axis = |key: &str, fallback: f64| args.and_then(|value| value.get(key)).and_then(|value| value.as_f64()).unwrap_or(fallback);
        match action {
            "translateSelection" => puzzle3d_apply_translate(&mut scratch, &object_ids, &volume_ids, axis("dx", 0.0), axis("dy", 0.0), axis("dz", 0.0)),
            "rotateSelection" => puzzle3d_apply_rotate(&mut scratch, &object_ids, &volume_ids, axis("ax", 0.0), axis("ay", 0.0), axis("az", 0.0), axis("angle", 0.0)),
            "scaleSelection" => puzzle3d_apply_scale(&mut scratch, &object_ids, &volume_ids, axis("sx", 1.0), axis("sy", 1.0), axis("sz", 1.0)),
            _ => {}
        }
        *self.transform_scratch.borrow_mut() = Some(scratch);
        {
            let next = self.preview_seq.borrow().wrapping_add(1);
            *self.preview_seq.borrow_mut() = next;
        }
        Emit { ui_scope: puzzle3d_transform_drag_scope(), ..Default::default() }
    }

    /// 📌️ Commits the whole gumball drag as ONE fixture delta (base → scratch), resolving attractions
    /// once. 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: takes the already-resolved
    /// object ids — see `transform_drag_tick`'s doc comment for why (no public `InteractionView`
    /// constructor exists outside `semio_framework_plugin`).
    pub(crate) fn commit_transform(&self, projection: &Value, object_ids: &[String]) -> Emit<Puzzle3dMutation, Puzzle3dConfigMutation> {
        *self.transform_drag_active.borrow_mut() = false;
        let Some(mut scratch) = self.transform_scratch.borrow_mut().take() else {
            *self.transform_base.borrow_mut() = None;
            return Emit::default();
        };
        *self.transform_base.borrow_mut() = None;
        let incoming = resolve_puzzle3d_attractions(&mut scratch);
        puzzle3d_rederive_moved_attractions(&mut scratch, object_ids, &incoming);
        resolve_puzzle3d_attractions(&mut scratch);
        let operations = puzzle3d_operations_from_fixture_change(projection, &scratch);
        if operations.is_empty() {
            Emit { ui_scope: puzzle3d_transform_drag_scope(), ..Default::default() }
        } else {
            Emit::commit(operations, "Transform selection")
        }
    }

    /// 🖼️ Fixture used for world render — live scratch while a gumball drag is in progress.
    fn render_fixture(&self, projection: &Value) -> Puzzle3dFixture {
        if let Some(scratch) = self.transform_scratch.borrow().as_ref() {
            return scratch.clone();
        }
        serde_json::from_value::<Puzzle3dFixture>(projection.clone()).unwrap_or_else(|_| empty_fixture())
    }

    //#region 🔖️GesturePreview
    /// 👻️ CW7 db+protocol+vcs-slimming campaign, "preview law for gesture apps": the live gumball
    /// drag's current fixture state, expressed as the same document-delta operations
    /// `commit_transform` would eventually emit for real — anchored to the drag-start snapshot
    /// (`transform_base`), never to the previous preview tick, so a preview built from this stays
    /// correct even when the lossy, uncredited preview lane drops every message but the latest.
    /// `None` outside an active drag; this reads `transform_base`/`transform_scratch` only, never
    /// emits or mutates a `Puzzle3dMutation`.
    ///
    /// 🚧️ Deliberately unwired beyond this accessor — `framework/sync::SyncSession::publish_preview`
    /// is host-only and unreachable from this WASI-P2 sandboxed plugin crate, and
    /// `store::BackboneMessage` has no preview-shaped variant to relay one through.
    /// `#[allow(dead_code)]`: exercised by `🧪️Tests` only until a host bridge exists.
    #[allow(dead_code)]
    pub(crate) fn gesture_preview(&self) -> Option<(&'static str, u64, Vec<u8>)> {
        let base_binding = self.transform_base.borrow();
        let base = base_binding.as_ref()?;
        let scratch_binding = self.transform_scratch.borrow();
        let scratch = scratch_binding.as_ref()?;
        let before = serde_json::to_value(base).ok()?;
        let operations = puzzle3d_operations_from_fixture_change(&before, scratch);
        let payload = json!({ "operations": operations });
        Some(("gesture:transform", *self.preview_seq.borrow(), serde_json::to_vec(&payload).ok()?))
    }
    //#endregion 🔖️GesturePreview

    /// 🧾️ Rebuilds the transient render bundle for one `(projection, config, window)` triple, with the
    /// window instance's own view-local options materialized onto the runtime.
    fn scene_for(&self, projection: &Value, config: &Puzzle3dConfig, window_id: &str) -> Puzzle3dScene {
        let active_utility = puzzle3d_scene_active_utility(config, Some(window_id));
        let mut runtime_for_window = config.clone();
        runtime_for_window.load_window(window_id);
        scene_from_projection(projection, runtime_for_window, &active_utility)
    }

    /// @emoji 🧩️ B1: the pure per-action core, dispatched into by `ArtifactApp::handle` with
    /// `action`/`args`/`window_id` reconstructed 1:1 from the typed `Puzzle3dCommand`. Everything past
    /// this adapter boundary reads/writes the passed-in `Puzzle3dConfig` snapshot and returns a real
    /// `Emit` (document + config operations) instead of mutating `self`.
    fn handle_action_impl(
        &self,
        action: &str,
        args: Option<&Value>,
        window_id: Option<&str>,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        config: &Puzzle3dConfig,
        interaction: &InteractionView<'_>,
    ) -> Emit<Puzzle3dMutation, Puzzle3dConfigMutation> {
        // 🗨️ Shell-only effect (no document interaction, hence no scene/before/after scaffolding
        // below): opens the declared "addObject" dialog over a glass veil.
        if action == "openAddObjectDialog" {
            return Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(120), dialog_id: "addObject".into(), args: None });
        }
        if action == "transformBegin" {
            self.begin_transform_session(doc.snapshot.value());
            return Emit::default();
        }
        let interaction_selection = semio_framework::io::resolve_ready(interaction.selection(PUZZLE3D_INTERACTION_DOMAIN));
        let (transform_object_ids, transform_volume_ids) = if interaction_selection.granularity == PUZZLE3D_GRANULARITY_TARGET_VOLUME {
            (Vec::new(), interaction_selection.ids.clone())
        } else if interaction_selection.granularity == PUZZLE3D_GRANULARITY_OBJECT {
            (interaction_selection.ids.clone(), Vec::new())
        } else {
            (Vec::new(), Vec::new())
        };
        if action == "transformEnd" {
            return self.commit_transform(doc.snapshot.value(), &transform_object_ids);
        }
        if *self.transform_drag_active.borrow() && matches!(action, "translateSelection" | "rotateSelection" | "scaleSelection") {
            return self.transform_drag_tick(action, args, doc.snapshot.value(), &transform_object_ids, &transform_volume_ids);
        }
        let document_action = puzzle3d_action_document_intent(action);
        let before = document_action.then(|| doc.snapshot.value().clone());
        let active_utility_initial = puzzle3d_scene_active_utility(config, window_id);
        // 🪟️ This action targets exactly one window instance — materialize ITS view-local options onto
        // the scene runtime before handling, and snapshot them back out (via `save_window`) so a
        // grid/LOD/selection/vortex/sun mutation never leaks into another window's options. Fill count
        // / distribution / overlap stay on the flat runtime and are shared.
        let wid = window_id.map(str::to_string).unwrap_or_else(|| main::WINDOW_KIND_ID.into());
        let mut runtime_for_window = config.clone();
        // 🪟️ B1: self-maintaining window registry — was host-pushed `view_state.window_instances`; now
        // the app itself remembers every window instance id it has ever been dispatched an action for,
        // so `window_engagements`/`window_measures` still see every live split pane.
        if !runtime_for_window.window_ids.iter().any(|id| id == &wid) {
            runtime_for_window.window_ids.push(wid.clone());
        }
        runtime_for_window.load_window(&wid);
        let mut scene = scene_from_projection(doc.snapshot.value(), runtime_for_window, &active_utility_initial);
        let mut ui_scope = UiDirtyScope::Full;
        let mut effects = Vec::new();
        let uses_precompute = puzzle3d_action_uses_precompute(action);
        if uses_precompute {
            sync_precompute_session(&mut self.precompute.borrow_mut(), &scene);
            self.precompute.borrow_mut().restore_persisted_fill(&config.fill_checkpoint);
            self.precompute.borrow_mut().set_fill_applied_count(config.fill_applied_count);
        }
        let mut ctx = Puzzle3dActionCtx { app: self, scene: &mut scene, window_id: &wid, config, interaction, ui_scope: &mut ui_scope, effects: &mut effects, abort: false };
        dispatch_puzzle3d_action(&mut ctx, action, args);
        let aborted = ctx.abort;
        if aborted {
            return Emit::default();
        }
        ui_scope = match action {
            "setCamera" | "setProjection" | "setProjectionParam" | "focusSelection" => puzzle3d_viewport_scope(),
            _ => ui_scope,
        };
        let next_active_utility = scene.active_utility.clone();
        scene.runtime.fill_checkpoint = if document_action && action != "setFillCount" {
            Vec::new()
        } else if uses_precompute {
            self.precompute.borrow().fill_checkpoint_bytes()
        } else {
            config.fill_checkpoint.clone()
        };
        scene.runtime.save_window(&wid);
        let operations = if let Some(before) = before.as_ref() {
            puzzle3d_operations_from_fixture_change(before, &scene.fixture)
        } else {
            debug_assert!(!puzzle3d_action_document_intent(action));
            Vec::new()
        };
        let coalesce_key = match action {
            "translateSelection" => Some("gumball-translate".to_string()),
            "rotateSelection" => Some("gumball-rotate".to_string()),
            "scaleSelection" => Some("gumball-scale".to_string()),
            "setFillCount" => Some("fill-count".to_string()),
            _ => None,
        };
        // 🧰️🛠️ Programmatic utility/tool switches (engagement submit/abort, suggestions, fill) push the
        // active utility/tool back into the host session; `setActiveUtility`/`setActiveTool` themselves
        // never re-emit (the command IS the direct switch, so this arm self-excludes). Fill transitions
        // go through `SetActiveTool` exclusively — the window's real utility is untouched by entering or
        // leaving the fill tool; a genuine utility transition (not involving fill on either side) still
        // emits `SetActiveUtility` exactly as before.
        let is_direct_utility_switch = matches!(action, x if x == SET_ACTIVE_UTILITY_ACTION_ID || x == SET_ACTIVE_TOOL_ACTION_ID);
        let initial_is_fill_tool = active_utility_initial == fill_tool::TOOL_ID;
        let next_is_fill_tool = next_active_utility == fill_tool::TOOL_ID;
        if !is_direct_utility_switch && next_is_fill_tool != initial_is_fill_tool {
            effects.push(Effect::SetActiveTool { tool_id: if next_is_fill_tool { fill_tool::TOOL_ID.into() } else { String::new() } });
        }
        if !is_direct_utility_switch && !next_is_fill_tool && !initial_is_fill_tool && next_active_utility != active_utility_initial {
            effects.push(Effect::SetActiveUtility { window_id: wid, utility_id: next_active_utility });
        }
        // 🧮️ B1: only a REAL config change becomes a `Puzzle3dConfigMutation` — `PartialEq` (derived)
        // makes this cheap, and keeps a pure read-only action (e.g. a re-materialize/re-save of an
        // already-idle window's options) from creating a no-op undo entry.
        let config_mutations = if &scene.runtime != config { vec![Puzzle3dConfigMutation::Snapshot { config: scene.runtime }] } else { Vec::new() };
        Emit { artifact_mutations: operations, config_mutations, coalesce_key, effects, ui_scope, ..Default::default() }
    }
}

/// 🎬️ Dispatch only: every arm's behaviour lives in its `🎮️commands/<group>/🦀️component.rs` free
/// function. No behaviour lives in this match.
fn dispatch_puzzle3d_action(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    match action {
        "setFixtureJson" => set_fixture_json::set_fixture_json(ctx, args),
        "setActiveExample" => set_active_example::set_active_example(ctx, args),
        "selectSameKindSelection" => select_same_kind::select_same_kind(ctx),
        "setSelectableKind" => set_selectable_kind::set_selectable_kind(ctx, args),
        "addObjectKind" => add_object_kind::add_object_kind(ctx, args),
        "deleteSelection" => delete_selection::delete_selection(ctx),
        "duplicateSelection" => duplicate_selection::duplicate_selection(ctx),
        "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),
        "patchInspector" => patch_inspector::patch_inspector(ctx, args),
        "createAttraction" => create_attraction::create_attraction(ctx, args),
        "deleteAttraction" => delete_attraction::delete_attraction(ctx, args),
        "addTargetVolume" => add_target_volume::add_target_volume(ctx, args),
        "deleteTargetVolume" => delete_target_volume::delete_target_volume(ctx, args),
        "setTargetVolumeFlag" => set_target_volume_flag::set_target_volume_flag(ctx, args),
        "relocateTargetVolume" => relocate_target_volume::relocate_target_volume(ctx, args),
        "setCamera" => set_camera::set_camera(ctx, args),
        "setProjection" | "setProjectionParam" => set_projection::set_projection(ctx, action, args),
        "focusSelection" => focus_selection::focus_selection(ctx),
        "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => apply_sun::apply(ctx, action, args),
        "setLodAutomatic" => set_automatic::set_automatic(ctx, args),
        "setLodDepthVariable" => set_depth_variable::set_depth_variable(ctx, args),
        "setLodManual" => set_manual::set_manual(ctx, args),
        "setGridVisible" => set_visible::set_visible(ctx, args),
        "setGridSnapEnabled" => set_snap_enabled::set_snap_enabled(ctx, args),
        "setGridSpacing" => set_spacing::set_spacing(ctx, args),
        "setProximityRadius" => set_proximity_radius::set_proximity_radius(ctx, args),
        "setChunkSize" => set_chunk_size::set_chunk_size(ctx, args),
        "setBrushPlacementOverlapBudget" => set_brush_placement_overlap_budget::set_brush_placement_overlap_budget(ctx, args),
        "setVoxelDims" => set_voxel_dims::set_voxel_dims(ctx, args),
        "setTransformGumballFlag" => set_transform_gumball_flag::set_transform_gumball_flag(ctx, args),
        "setVortexShow" => set_vortex_show::set_vortex_show(ctx, args),
        "setVortexDirection" => set_vortex_direction::set_vortex_direction(ctx, args),
        "translateSelection" => translate_selection::translate_selection(ctx, args),
        "rotateSelection" => rotate_selection::rotate_selection(ctx, args),
        "scaleSelection" => scale_selection::scale_selection(ctx, args),
        "worldRelocate" => world_relocate::world_relocate(ctx, args),
        "addBrushObject" => add_brush_object::add_brush_object(ctx, args),
        "cycleBrushCandidate" | "cycleBrushCandidateBack" => cycle_candidate::cycle_candidate(ctx, action, args),
        "openVortexSuggestions" => open_vortex_suggestions::open_vortex_suggestions(ctx, args),
        "closeVortexSuggestions" => close_vortex_suggestions::close_vortex_suggestions(ctx),
        "hoverSuggestion" => hover_suggestion::hover_suggestion(ctx, args),
        "acceptSuggestion" => accept_suggestion::accept_suggestion(ctx, args),
        "suggestionsTick" => suggestions_tick::suggestions_tick(ctx),
        "registerBrushMesh" => register_brush_mesh::register_brush_mesh(ctx, args),
        "engagementControlSelect" => engagement_control_select::engagement_control_select(ctx, args),
        "fillBuildTick" => fill_build_tick::fill_build_tick(ctx),
        "setObjectKindWeight" | "setVortexKindWeight" => set_kind_weight::set_kind_weight(ctx, action, args),
        "engagementInput" => engagement_input::engagement_input(ctx, args),
        "engagementSubmit" => engagement_submit::engagement_submit(ctx, args),
        "engagementRepeatLast" => engagement_repeat_last::engagement_repeat_last(ctx),
        "engagementAbort" => engagement_abort::engagement_abort(ctx),
        "setLocale" => set_locale::set_locale(ctx, args),
        "setTerminology" => set_terminology::set_terminology(ctx, args),
        SET_ACTIVE_UTILITY_ACTION_ID | SET_ACTIVE_TOOL_ACTION_ID => set_active::set_active(ctx, action, args),
        "worldPointerDown" => {}
        _ => {}
    }
}

fn puzzle3d_action_uses_precompute(action: &str) -> bool {
    matches!(
        action,
        "setBrushPlacementOverlapBudget"
            | "addBrushObject"
            | "cycleBrushCandidate"
            | "cycleBrushCandidateBack"
            | "openVortexSuggestions"
            | "acceptSuggestion"
            | "suggestionsTick"
            | "registerBrushMesh"
            | "setFillCount"
            | "fillBuildTick"
            | "setObjectKindWeight"
            | "setVortexKindWeight"
            | "engagementRepeatLast"
    )
}

impl ArtifactEditor for Puzzle3dPlayApp {
    const DIALECT: Dialect = crate::artifacts::puzzle3d::PUZZLE3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE3D_FIXTURE_SCHEMA;
    type Snapshot = Puzzle3dPlaySnapshot;
    type Mutation = Puzzle3dMutation;
    type Config = Puzzle3dConfig;
    type ConfigMutation = Puzzle3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = Puzzle3dPresence;
    type PresenceMutation = Puzzle3dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = Puzzle3dCommand;

    /// 📎 Ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d: replaces the old
    /// `crate::editor::puzzle3d::config::schema::register_app_schema()` self-registering call, which
    /// puzzle's plugin root used to reach `.setup()` for — `register_document_app`/`document_app`
    /// now call this automatically the moment `Puzzle3dPlayApp` is bound to a plugin, exactly like
    /// `🗒️note`'s own `app_schema` override.
    async fn app_schema() -> Option<artifact_schema::AppSchemaDescriptor> {
        Some(crate::editor::puzzle3d::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> Puzzle3dPlaySnapshot {
        LazyLock::force(&NAKAGIN_EXAMPLE_FIXTURE);
        LazyLock::force(&PUZZLE3D_EXAMPLE_OPERATIONS);
        let snapshot = Puzzle3dPlaySnapshot::new(serde_json::to_value(default_fixture()).unwrap_or_else(|_| serde_json::to_value(empty_fixture()).unwrap_or(Value::Null)));
        let config = Puzzle3dConfig::default();
        let active_utility = puzzle3d_scene_active_utility(&config, None);
        let scene = scene_from_projection(snapshot.value(), config, &active_utility);
        let mut app = Puzzle3dPlayApp::default();
        sync_precompute_session(&mut app.precompute.borrow_mut(), &scene);
        snapshot
    }

    /// 🏷️ Maps each `Puzzle3dCommand` variant back to the action id it was declared under.
    async fn command_id(command: &Puzzle3dCommand) -> &'static str {
        command.action_id()
    }

    /// 🎯️ Maps the host's transitional `{action,args}` wire onto Puzzle 3D's closed command
    /// enum until React and wgpu send `OpBinary` command bytes directly.
    async fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let window_id = args.and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(Value::as_str).map(str::to_string);
        Puzzle3dCommand::from_action(action, args.cloned(), window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 3D action '{action}'")))
    }

    /// @emoji 🧩️ Thin typed-command adapter — reconstructs the exact `(action, args, window_id)`
    /// triple `handle_action_impl` expects from the typed `Puzzle3dCommand`.
    async fn handle(
        command: &Puzzle3dCommand,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation, Self::DraftMutation>, Fault> {
        Ok(with_puzzle3d_app_for(&cfg.snapshot, |app| {
            if command.action_id() == "fillBuildTick" {
                if let Some(emit) = fill_build_tick::fill_build_tick_cached(app, &cfg.snapshot) {
                    return emit;
                }
            }
            if matches!(command.action_id(), "setFillCount" | set_fill_count::STEP_ACTION_ID) {
                let mut precompute = app.precompute.borrow_mut();
                if !cfg.snapshot.fill_checkpoint.is_empty() && !precompute.restore_persisted_fill(&cfg.snapshot.fill_checkpoint) {
                    let active_utility = puzzle3d_scene_active_utility(&cfg.snapshot, command.window_id());
                    let scene = scene_from_projection(doc.snapshot.value(), cfg.snapshot.clone(), &active_utility);
                    sync_precompute_session(&mut precompute, &scene);
                    precompute.restore_persisted_fill(&cfg.snapshot.fill_checkpoint);
                }
                precompute.set_fill_applied_count(cfg.snapshot.fill_applied_count);
                return if command.action_id() == "setFillCount" { set_fill_count::begin(&mut precompute, &cfg.snapshot, command.args()) } else { set_fill_count::step(&mut precompute, &cfg.snapshot, command.args()) };
            }
            app.handle_action_impl(command.action_id(), command.args(), command.window_id(), doc, &cfg.snapshot, interaction)
        }))
    }

    /// 🕹️ `vortex` domain topology (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
    /// objects and every root-level entity kind (vortex marker, attraction, target volume, reference,
    /// catalogue kind) as one flat forest, except object-owned vortex markers whose parent is the
    /// object they mark — the one real nesting relationship this app's document carries, replacing
    /// what `hoveredVortexFullId`'s ad hoc highlighting used to do by hand.
    async fn interaction_topology(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle3dConfig>) -> semio_framework_plugin::InteractionTopology {
        let snapshot = doc.snapshot.typed();
        let mut ordered = Vec::new();
        for object in &snapshot.objects {
            ordered.push(semio_framework_plugin::TopologyNode { id: object.id.clone(), granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), parent: None });
            for vortex in &object.vortices {
                ordered.push(semio_framework_plugin::TopologyNode { id: puzzle3d_vortex_full_id(&object.id, &vortex.id), granularity: PUZZLE3D_GRANULARITY_VORTEX.into(), parent: Some(object.id.clone()) });
            }
        }
        for attraction in &snapshot.attractions {
            ordered.push(semio_framework_plugin::TopologyNode { id: attraction.id.clone(), granularity: PUZZLE3D_GRANULARITY_ATTRACTION.into(), parent: None });
        }
        for volume in &snapshot.target_volumes {
            ordered.push(semio_framework_plugin::TopologyNode { id: volume.id.clone(), granularity: PUZZLE3D_GRANULARITY_TARGET_VOLUME.into(), parent: None });
        }
        for reference in &snapshot.references {
            ordered.push(semio_framework_plugin::TopologyNode { id: reference.id.clone(), granularity: PUZZLE3D_GRANULARITY_REFERENCE.into(), parent: None });
        }
        if let Some(catalogs) = &snapshot.meta.kind_catalogs {
            for kind in &catalogs.objects {
                ordered.push(semio_framework_plugin::TopologyNode { id: kind.id.clone(), granularity: PUZZLE3D_GRANULARITY_KIND.into(), parent: None });
            }
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(PUZZLE3D_INTERACTION_DOMAIN.to_string(), semio_framework_plugin::DomainTopology { ordered });
        semio_framework_plugin::InteractionTopology { domains }
    }

    /// 🔌️ Declares puzzle3d's typed media I/O surface — the implicit document ports plus the flagship
    /// `kit:in` seam: an input port accepting `Kit×Type` media tagged `kit.catalog`, fanning IN from
    /// potentially many producers (`multiplicity: Many`).
    async fn io() -> Option<AppIo> {
        Some(puzzle3d_io())
    }

    /// 🎞️ `kit:in` seam: normalizes an incoming `kit.catalog` fragment (`objectKinds`/`vortexKinds`/
    /// `cableKinds`/`attractionKinds`/`kindCompatibility`) into puzzle3d's own `meta.kind_catalogs`
    /// vocabulary (`objects`/`vortices`/`cables`/`attractions`) and upserts it (keyed by row `id`,
    /// deterministic/order-independent — safe for `multiplicity: Many` fan-in) via the same
    /// `puzzle3d_operations_from_fixture_change` delta bridge every other fixture-mutating action
    /// already uses, so this never mutates anything directly — only real, undoable operations.
    async fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "kit:in" {
            return Err(MediaError::NotImplemented);
        }
        let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "kit:in only accepts a Structured (JSON) payload".into()));
        };
        let fragment: Value = serde_json::from_str(json.as_str()).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let mut fixture: Puzzle3dFixture = serde_json::from_value(doc.snapshot.value().clone()).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;

        let mut catalogs = fixture.meta.kind_catalogs.clone().unwrap_or_else(|| json!({ "objects": [], "vortices": [], "cables": [], "attractions": [] }));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "objects", fragment.get("objectKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "vortices", fragment.get("vortexKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "cables", fragment.get("cableKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "attractions", fragment.get("attractionKinds"));
        fixture.meta.kind_catalogs = Some(catalogs);

        if let Some(incoming_compat) = fragment.get("kindCompatibility").and_then(Value::as_array) {
            let mut compat: Vec<Value> = fixture.meta.kind_compatibility.as_ref().and_then(Value::as_array).cloned().unwrap_or_default();
            for row in incoming_compat {
                let source = row.get("source").and_then(Value::as_str).unwrap_or_default();
                let target = row.get("target").and_then(Value::as_str).unwrap_or_default();
                match compat.iter().position(|entry| entry.get("source").and_then(Value::as_str) == Some(source) && entry.get("target").and_then(Value::as_str) == Some(target)) {
                    Some(index) => compat[index] = row.clone(),
                    None => compat.push(row.clone()),
                }
            }
            fixture.meta.kind_compatibility = Some(Value::Array(compat));
        }

        let operations = puzzle3d_operations_from_fixture_change(doc.snapshot.value(), &fixture);
        Ok(Emit::mutations(operations))
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let node = with_puzzle3d_app_for(&cfg.snapshot, |app| -> semio_framework_plugin::UiAssemblyResult<_> {
            let (base_body_key, window_id_from_key) = body_key.split_once(':').map(|(b, w)| (b, Some(w))).unwrap_or((body_key, None));
            let config = cfg.snapshot;
            let wid = window_id_from_key.or_else(|| config.window_ids.first().map(String::as_str)).unwrap_or(main::WINDOW_KIND_ID);
            let active_utility = puzzle3d_scene_active_utility(config, Some(wid));
            let mut runtime_for_window = config.clone();
            if !runtime_for_window.window_ids.iter().any(|id| id == wid) {
                runtime_for_window.window_ids.push(wid.to_string());
            }
            runtime_for_window.load_window(wid);
            let precompute_scene = app.scene_for(doc.snapshot.value(), config, wid);
            {
                let mut precompute = app.precompute.borrow_mut();
                sync_precompute_session(&mut precompute, &precompute_scene);
                precompute.restore_persisted_fill(&config.fill_checkpoint);
                precompute.set_fill_applied_count(config.fill_applied_count);
            }
            let precompute = app.precompute.borrow();
            // 🪣️ Additive-only: appends just the not-yet-committed fill-plan tail onto the live fixture —
            // safe even during a live gumball scratch drag, since it never touches/replaces any
            // already-present object (the dragged one included).
            let fill_available = precompute.fill_available_count();
            let fixture = puzzle3d_fixture_with_fill_display_memo(app.render_fixture(doc.snapshot.value()), &precompute, config.fill_applied_count, fill_available, &app.fill_display_memo);
            let envelope = Puzzle3dScene { fixture, runtime: runtime_for_window, active_utility };
            let labels = puzzle3d_labels(config);
            match base_body_key {
                main::BODY_KEY => {
                    let (instances_json, meshes_json) = app.geometry_jsons(&envelope.fixture);
                    main::render(&envelope, &precompute, instances_json, meshes_json)
                }
                document::BODY_KEY => app.document_tree_cached(&envelope.fixture, labels),
                catalogue::BODY_KEY => catalogue::render(&envelope, labels),
                inspection::BODY_KEY => inspection::render(&envelope, labels),
                settings_panel::BODY_KEY => settings_panel::render(&envelope, labels),
                _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}")))
                    .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d unknown-body label admission failed")),
            }
        })?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    async fn window_engagements(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>) -> HashMap<String, WindowEngagement> {
        with_puzzle3d_app_for(&cfg.snapshot, |app| {
            let config = cfg.snapshot;
            let labels = puzzle3d_labels(config);
            // 🪟️ One entry per live window INSTANCE (split top/perspective panes are two instances of the
            // same kind) — each built from ITS OWN materialized options, never the shared kind entry.
            window_instance_ids(config, main::WINDOW_KIND_ID)
                .into_iter()
                .map(|wid| {
                    let envelope = app.scene_for(doc.snapshot.value(), config, &wid);
                    (wid, main::engagement(&envelope, labels))
                })
                .collect()
        })
    }

    async fn window_measures(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        with_puzzle3d_app_for(&cfg.snapshot, |app| {
            let config = cfg.snapshot;
            let labels = puzzle3d_labels(config);
            window_instance_ids(config, main::WINDOW_KIND_ID)
                .into_iter()
                .map(|wid| {
                    let envelope = app.scene_for(doc.snapshot.value(), config, &wid);
                    let precompute = restored_precompute_session(&envelope, &config.fill_checkpoint);
                    (wid, main::window_measures(&envelope, &precompute, labels))
                })
                .collect()
        })
    }

    async fn tool_measures(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        with_puzzle3d_app_for(&cfg.snapshot, |app| {
            let config = cfg.snapshot;
            let wid = config.window_ids.first().map(String::as_str).unwrap_or(main::WINDOW_KIND_ID);
            let labels = puzzle3d_labels(config);
            let envelope = app.scene_for(doc.snapshot.value(), config, wid);
            let precompute = restored_precompute_session(&envelope, &config.fill_checkpoint);
            HashMap::from([(fill_tool::TOOL_ID.to_string(), fill_tool::measures(&envelope, &precompute, labels))])
        })
    }

    async fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let config = cfg.snapshot;
        let labels = puzzle3d_labels(config);
        let wid = config.window_ids.first().map(String::as_str).unwrap_or(main::WINDOW_KIND_ID);
        let active_utility = puzzle3d_scene_active_utility(config, Some(wid));
        let envelope = scene_from_projection(doc.snapshot.value(), config.clone(), &active_utility);
        let selection = Puzzle3dContextSelection::from_surface(request.surface.as_ref());
        puzzle3d_context_menu_items(&envelope, &selection, labels, registry)
    }
}
//#endregion 🔖️PlayApp

//#region 🔖️Manifest
/// 🔌️ Declares puzzle3d's typed media I/O surface — the implicit document ports plus the flagship
/// `kit:in` seam: an input port accepting `Kit×Type` media tagged `kit.catalog`, fanning IN from
/// potentially many producers (`multiplicity: Many`).
///
/// 🎯️ A free function, not an inline `ArtifactApp::io()` body, because BOTH the trait method and the
/// `AppBuilder` need it: the trait method serves the runtime, while `.io(..)` on the builder is what
/// puts `document_schema` into the published `AppDefinition`. Inlining it in only the trait method
/// left the manifest's `io` empty, so a host reading the manifest could not route a document to this
/// surface at all — caught by the demonstrator bundle's `every_pane_declares_a_document_schema`.
pub fn puzzle3d_io() -> AppIo {
    semio_framework::io::resolve_ready(
        semio_framework::io::resolve_ready(AppIo::from_document(
            "puzzle.3d",
            MediaType { class: MediaClass::ThreeD, form: MediaForm::Design },
            semio_framework_plugin::ArtifactPresentation { id: "3d.puzzle".into(), name: "3D Puzzle".into(), dimension: "3d".into(), component_kind: "puzzle3d".into() },
        ))
        .with_ports(vec![MediaPortSpec {
            id: "kit:in".into(),
            label: "Kit Catalog".into(),
            direction: MediaPortDirection::In,
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            kind_id: Some("kit.catalog".into()),
            required: false,
            multiplicity: PortMultiplicity::Many,
        }]),
    )
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `vortex` domain declaration —
/// one granularity per previously-distinct `Puzzle3dSelection` bag (object/vortex/attraction/
/// targetVolume/reference) plus `kind` for the catalogue's rows. `Topology` hierarchy (see
/// `Puzzle3dPlayApp::interaction_topology`) makes the object→vortex-marker nesting available for a
/// future transitive hover; every other granularity is a flat root today.
fn puzzle3d_interaction_definition() -> InteractionDefinition {
    let granularity = |id: &str, label: LocalizedLabel, icon: &str| GranularityDefinition { id: id.into(), label, icon_id: icon.into() };
    InteractionDefinition {
        id: PUZZLE3D_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Vortex", "Vortex"),
        granularities: vec![
            granularity(PUZZLE3D_GRANULARITY_OBJECT, puzzle3d_localized(|l| l.object), "box"),
            granularity(PUZZLE3D_GRANULARITY_VORTEX, puzzle3d_localized(|l| l.vortex), "sparkles"),
            granularity(PUZZLE3D_GRANULARITY_ATTRACTION, puzzle3d_localized(|l| l.attraction), "link"),
            granularity(PUZZLE3D_GRANULARITY_TARGET_VOLUME, puzzle3d_localized(|l| l.target_volume), "box-select"),
            granularity(PUZZLE3D_GRANULARITY_REFERENCE, puzzle3d_localized(|l| l.reference), "image"),
            granularity(PUZZLE3D_GRANULARITY_KIND, puzzle3d_localized(|l| l.kind), "layers"),
        ],
        hierarchy: HierarchyProvider::Topology,
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

/// 🎭️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET (contract §2.4): `Editor::builder`
/// derives the surface id from `DIALECT`+`ROLE` (no hand-written label/id), returns `AppDefinition`
/// (not `App`), and has no `.example(...)`/`.workflow(...)` methods — the old
/// `.example(PUZZLE3D_EXAMPLE_CONCRETE_FOREST, …)` / `.example(PUZZLE3D_EXAMPLE_NAKAGIN, …)` /
/// `.workflow("puzzle3d", "Puzzle 3D", "model")` calls this builder used to end with are DROPPED
/// here, not ported (contract §7.4's `App { definition, examples }` split: `.editor::<E>(def)` only
/// takes the definition, so `AppBuilder`'s own `examples` vec is discarded either way) — flagged as a
/// known gap for the coordinator, not silently lost.
pub fn create_puzzle3d_app() -> semio_framework_plugin::AppDefinition {
    let envelope = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    Editor::builder(crate::artifacts::puzzle3d::PUZZLE3D_DIALECT)
            .document(["semio", "puzzle", "3d"])
            .artifact_kind(crate::artifacts::puzzle3d::artifact_kind())
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "Aggregator"])
            .mode_def(edit::definition())
            .default_mode_id(edit::PUZZLE3D_PLAY_MODE_EDIT)
            .io(puzzle3d_io())
            .window_kind_def(main::definition(&envelope, &Puzzle3dLabels::NATIVE_EN))
            .interaction(puzzle3d_interaction_definition())
            .window_kind_interactions(main::WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE3D_INTERACTION_DOMAIN)])
            .default_layout(edit::layout())
            .panel_tab_def(document::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            .panel_tab_def(settings_panel::definition())
            .keybinding("escape", "engagementAbort")
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("mod+d", "duplicateSelection")
            .keybinding("tab", "cycleBrushCandidate")
            .keybinding("shift+tab", "cycleBrushCandidateBack")
            .keybinding("f", "focusSelection")
            // 🔧️ Document-mutating operations (emit VCS operations through the before/after fixture delta).
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Rohdaten festlegen"), ActionKind::Mutation) })
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("addObjectKind", puzzle3d_localized_phrase(|l| l.object, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).category("selection")))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).category("create")))
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .mutation("transformEnd", LocalizedLabel::native("Transform End", "Transformieren beenden"))
            .mutation("worldRelocate", puzzle3d_localized_phrase(|l| l.object, |w| format!("Relocate {w}"), |w| format!("{w} verlagern")))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).category("hand")))
            .mutation("patchInspector", LocalizedLabel::native("Patch Inspector", "Inspektor aktualisieren"))
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .mutation("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"))
            .mutation("createAttraction", puzzle3d_localized_phrase(|l| l.attraction, |w| format!("Create {w}"), |w| format!("{w} erstellen")))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("deleteAttraction", puzzle3d_localized_phrase(|l| l.attraction, |w| format!("Delete {w}"), |w| format!("{w} löschen")), ActionKind::Mutation).category("targets")))
            .mutation("addTargetVolume", puzzle3d_localized_phrase(|l| l.target_volume, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("deleteTargetVolume", LocalizedLabel::native("Delete Target Volume", "Zielvolumen löschen"), ActionKind::Mutation).category("targets")))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("setTargetVolumeFlag", LocalizedLabel::native("Set Target Volume Flag", "Zielvolumenmarkierung festlegen"), ActionKind::Mutation).category("targets")))
            .mutation("addBrushObject", puzzle3d_localized_phrase(|l| l.object, |w| format!("Add Brush {w}"), |w| format!("Pinsel-{w} hinzufügen")))
            .mutation("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(set_fill_count::STEP_ACTION_ID, LocalizedLabel::native("Set Fill Count Step", "Füllanzahl-Schritt"), ActionKind::Mutation) })
            .mutation("acceptSuggestion", LocalizedLabel::native("Accept Suggestion", "Vorschlag annehmen"))
            // 🗨️ Shell-only effect (no document mutation): opens the "addObject" dialog.
            .shell_action("openAddObjectDialog", puzzle3d_localized_phrase(|l| l.object, |w| format!("Add {w}…"), |w| format!("{w} hinzufügen…")))
            // 👁️ Ephemeral view state — selection, hover, camera scratch, utility-parameter runtime.
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("setProjection", LocalizedLabel::native("Set Projection", "Projektion festlegen"))
            .view_action("setProjectionParam", LocalizedLabel::native("Set Projection Parameter", "Projektionsparameter festlegen"))
            .view_action("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("selectSameKindSelection", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).category("selection")))
            .view_action("setVortexShow", puzzle3d_localized_phrase(|l| l.vortex_show, |w| format!("Set {w}"), |w| format!("{w} festlegen")))
            .view_action("setVortexDirection", puzzle3d_localized_phrase(|l| l.vortex_direction, |w| format!("Set {w}"), |w| format!("{w} festlegen")))
            .view_action("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"))
            .view_action("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"))
            .view_action("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"))
            .view_action("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"))
            .view_action("setLodAutomatic", LocalizedLabel::native("Set Lod Automatic", "Detailstufe automatisch"))
            .view_action("setLodDepthVariable", LocalizedLabel::native("Set Lod Depth Variable", "Detailstufen-Tiefe festlegen"))
            .view_action("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster anzeigen"))
            .view_action("setLodManual", LocalizedLabel::native("Set Lod Manual", "Detailstufe manuell"))
            .view_action("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"))
            .view_action("setGridSpacing", LocalizedLabel::native("Set Grid Spacing", "Rasterabstand festlegen"))
            .view_action("setProximityRadius", LocalizedLabel::native("Set Proximity Radius", "Näheradius festlegen"))
            .view_action("setChunkSize", LocalizedLabel::native("Set Chunk Size", "Blockgröße festlegen"))
            .view_action("setSelectableKind", LocalizedLabel::native("Set Selectable Kind", "Auswählbare Art festlegen"))
            .view_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"))
            .view_action("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"))
            .view_action("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"))
            .view_action("setTransformGumballFlag", LocalizedLabel::native("Set Transform Gumball Flag", "Transformieren-Griff festlegen"))
            .view_action("transformBegin", LocalizedLabel::native("Transform Begin", "Transformieren beginnen"))
            .view_action("setVoxelDims", LocalizedLabel::native("Set Voxel Dims", "Voxel-Abmessungen festlegen"))
            .view_action("relocateTargetVolume", LocalizedLabel::native("Relocate Target Volume", "Zielvolumen verlagern"))
            .view_action("setBrushPlacementOverlapBudget", LocalizedLabel::native("Set Brush Placement Overlap Budget", "Pinsel-Überlappungsbudget festlegen"))
            .view_action("setObjectKindWeight", puzzle3d_localized_phrase(|l| l.object, |w| format!("Set {w} Kind Weight"), |w| format!("{w}-Art-Gewicht festlegen")))
            .view_action("setVortexKindWeight", puzzle3d_localized_phrase(|l| l.vortex, |w| format!("Set {w} Kind Weight"), |w| format!("{w}-Art-Gewicht festlegen")))
            .view_action("cycleBrushCandidate", LocalizedLabel::native("Cycle Brush Candidate", "Pinselkandidat wechseln"))
            .view_action("cycleBrushCandidateBack", LocalizedLabel::native("Cycle Brush Candidate Back", "Pinselkandidat rückwärts wechseln"))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("openVortexSuggestions", puzzle3d_localized_phrase(|l| l.vortex, |w| format!("Open {w} Suggestions"), |w| format!("{w}-Vorschläge öffnen")), ActionKind::View).category("tools")))
            .view_action("closeVortexSuggestions", puzzle3d_localized_phrase(|l| l.vortex, |w| format!("Close {w} Suggestions"), |w| format!("{w}-Vorschläge schließen")))
            .view_action("hoverSuggestion", LocalizedLabel::native("Hover Suggestion", "Vorschlag überfahren"))
            .view_action("suggestionsTick", LocalizedLabel::native("Suggestions Tick", "Vorschläge-Takt"))
            .view_action("fillBuildTick", LocalizedLabel::native("Fill Build Tick", "Füllaufbau-Takt"))
            .view_action("registerBrushMesh", LocalizedLabel::native("Register Brush Mesh", "Pinsel-Mesh registrieren"))
            .view_action("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"))
            // 📝️ Staged argument forms for the panel-visible create/query actions (P1).
            .action_args("addObjectKind", vec![
                ActionArgDef::select("objectKind", puzzle3d_localized(|l| l.kind), vec![ActionArgOption::new("Object", puzzle3d_localized(|l| l.object))]).default_value("Object"),
            ])
            // 🧰️ Flat per-window set of utilities; no utility is active until the host presses one — the
            // transform gumball exposes translate and rotate together via Move/Rotate flags.
            .utility(utilities::transform::definition())
            .utility(utilities::brush::definition(puzzle3d_localized(|l| l.brush)))
            .utility(utilities::volume_brush::definition(puzzle3d_localized(|l| l.volume_brush)))
            .utility(utilities::world_relocate::definition())
            .window_kind_utilities(main::WINDOW_KIND_ID, vec![
                utilities::transform::UTILITY_ID.into(),
                utilities::brush::UTILITY_ID.into(),
                utilities::volume_brush::UTILITY_ID.into(),
                utilities::world_relocate::UTILITY_ID.into(),
            ])
            // 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility — it keeps
            // its viewport interaction via `Puzzle3dConfig::active_tool_id`.
            .tool(fill_tool::definition(puzzle3d_localized(|l| l.fill)))
            .mode_tools(edit::PUZZLE3D_PLAY_MODE_EDIT, vec![semio_framework::io::resolve_ready(ToolRef::new(fill_tool::TOOL_ID))])
            // 🎓️ Reference introduction: a short first-run walkthrough of the viewport, the catalogue
            // panel, adding an object, and the Transform utility.
            .introduction(IntroductionDefinition {
                title: puzzle3d_localized_phrase(|l| l.window_main, |w| format!("Welcome to {w}"), |w| format!("Willkommen bei {w}")),
                steps: vec![
                    IntroductionStepDefinition::new(
                        "welcome",
                        puzzle3d_localized_phrase(|l| l.window_main, |w| format!("Welcome to {w}"), |w| format!("Willkommen bei {w}")),
                        LocalizedLabel::native(
                            "A quick tour of the viewport, utilities, and panels before you start composing.",
                            "Eine kurze Tour durch Ansicht, Hilfsmittel und Paneele, bevor Sie mit dem Zusammenfügen beginnen.",
                        ),
                    ),
                    IntroductionStepDefinition::new(
                        "viewport",
                        LocalizedLabel::native("The Viewport", "Die 3D-Ansicht"),
                        LocalizedLabel::native(
                            "This is your 3D scene — orbit, pan, and zoom to look around.",
                            "Das ist Ihre 3D-Szene — orbitieren, verschieben und zoomen Sie, um sich umzusehen.",
                        ),
                    )
                        .introduce(window_element_id(main::WINDOW_KIND_ID))
                        .interact(vec![
                            semio_framework::io::resolve_ready(IntroductionInteraction::zoom(main::WINDOW_KIND_ID, "Zoom")),
                            semio_framework::io::resolve_ready(IntroductionInteraction::pan(main::WINDOW_KIND_ID, "Pan")),
                            semio_framework::io::resolve_ready(IntroductionInteraction::orbit(main::WINDOW_KIND_ID, "Orbit")),
                        ]),
                    IntroductionStepDefinition::new(
                        "catalogue",
                        LocalizedLabel::native("The Catalogue", "Der Katalog"),
                        puzzle3d_localized_phrase(|l| l.objects, |w| format!("Browse the {w} available to place from here."), |w| format!("Durchstöbern Sie hier die verfügbaren {w}.")),
                    )
                        .introduce(panel_tab_element_id(semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID))
                        .placement(IntroductionPlacement::Right),
                    IntroductionStepDefinition::new(
                        "add-object",
                        puzzle3d_localized_phrase(|l| l.object, |w| format!("Add a {w}"), |w| format!("{w} hinzufügen")),
                        puzzle3d_localized_phrase(
                            |l| l.object,
                            |w| format!("Drag the first {w} from the catalogue into the viewport."),
                            |_w| "Ziehen Sie den ersten Eintrag per Drag-and-Drop aus dem Katalog in die 3D-Ansicht.".to_string(),
                        ),
                    )
                        .introduce(panel_tab_first_draggable_element_id(semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID))
                        .show(vec![panel_tab_element_id(semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID), window_element_id(main::WINDOW_KIND_ID)])
                        .placement(IntroductionPlacement::Right)
                        .interact(vec![semio_framework::io::resolve_ready(IntroductionInteraction::action("addObjectKind", "Add an object"))]),
                    IntroductionStepDefinition::new(
                        "transform-utility",
                        puzzle3d_localized_phrase(|l| l.objects, |w| format!("Transform {w}"), |w| format!("{w} transformieren")),
                        puzzle3d_localized_phrase(
                            |l| l.objects,
                            |w| format!("Activate the Transform utility to move and rotate {w} in the scene."),
                            |w| format!("Aktivieren Sie das Transformieren-Hilfsmittel, um {w} zu verschieben und zu drehen."),
                        ),
                    )
                        .introduce(utilities::transform::UTILITY_ID)
                        .show(vec![window_element_id(main::WINDOW_KIND_ID)])
                        .interact(vec![semio_framework::io::resolve_ready(IntroductionInteraction::utility(utilities::transform::UTILITY_ID, "Activate Transform"))]),
                ],
            })
            // 🗨️ Reference dialog opened by `openAddObjectDialog`, driving the existing `addObjectKind`
            // operation's `objectKind` select arg.
            .dialog(
                DialogDefinition::new(
                    "addObject",
                    puzzle3d_localized_phrase(|l| l.object, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")),
                    ActionRef::new("addObjectKind"),
                )
                    .body(puzzle3d_localized_phrase(
                        |l| l.object,
                        |w| format!("Choose the kind of {w} to add to the scene."),
                        |_w| "Wählen Sie die Art zum Hinzufügen.".to_string(),
                    ))
                    .args(vec![
                        ActionArgDef::select("objectKind", puzzle3d_localized(|l| l.kind), vec![ActionArgOption::new("Object", puzzle3d_localized(|l| l.object))]).default_value("Object").required(),
                    ])
                    .submit_label(LocalizedLabel::native("Add", "Hinzufügen")),
            )
            .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
            .build_definition()
}

//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ The one puzzle3d-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
/// instead of re-deriving a store/dispatch/render scaffold of its own.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::{testkit, ActionMeta, App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type Puzzle3dApp = VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>;

    pub fn meta(actor: &str) -> ActionMeta {
        testkit::meta(actor)
    }

    pub fn app() -> Puzzle3dApp {
        semio_framework::io::resolve_ready(testkit::new_app::<EditorApp<Puzzle3dPlayApp>>())
    }

    /// 🌉️ `new_app_with_registry`'s `manifest: fn() -> App` shape predates the `AppDefinition`-returning
    /// `create_puzzle3d_app()` convention (contract §2.4 / SDK gap 3) — this tiny local wrapper bridges
    /// the two, mirroring `📓️w2-cad-report.md`'s recipe step 7.
    pub fn puzzle3d_manifest_for_testkit() -> App {
        App { definition: create_puzzle3d_app(), examples: Vec::new() }
    }

    /// 🧰️ A registry-backed app so kind discipline (View/Shell actions must emit no operations) and the
    /// utility contract are enforced exactly as in production.
    pub fn app_with_registry() -> Puzzle3dApp {
        semio_framework::io::resolve_ready(testkit::new_app_with_registry::<EditorApp<Puzzle3dPlayApp>>(puzzle3d_manifest_for_testkit))
    }

    /// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
    /// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
    /// `Self::Command` channel). Reconstructs the `Puzzle3dCommand` from the same
    /// `(action, args, window_id)` triple every pre-B1 test already passed.
    pub fn dispatch(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
        // 🕰️ Framework-reserved verbs (undo/redo/checkpoint/…/the six interaction verbs) stay on
        // `handle_action` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM added
        // interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
        // setInteractionGranularity to this reserved set.
        if matches!(
            action,
            "undo"
                | "redo"
                | "checkpoint"
                | "commitCheckpoint"
                | "createAlternative"
                | "switchAlternative"
                | "checkoutCheckpoint"
                | "alternative"
                | "revertToCommand"
                | "historyFilter"
                | "noteShellCommand"
                | "copy"
                | "cut"
                | "paste"
                | "interactionSelect"
                | "interactionHover"
                | "clearSelection"
                | "selectAll"
                | "setSelectionMode"
                | "setInteractionGranularity"
        ) {
            return app.handle_action(action, args, &meta("local")).await;
        }
        app.dispatch_typed(Puzzle3dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)).unwrap_or_else(|| panic!("unknown puzzle3d action id in test: {action}")), &meta("local")).await
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionSelect`
    /// for one `(granularity, id)` pair in the `vortex` domain — the test-side replacement for the
    /// deleted `worldPick`/`worldSelect`/`worldVortexSelect`/`setSelection` actions.
    pub fn select_id(app: &mut Puzzle3dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
        let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
        dispatch(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None).await
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionHover`
    /// for one `(granularity, id)` pair on the `pointer` channel in the `vortex` domain — the
    /// test-side replacement for the deleted `worldHover`/`setHover`/`worldVortexHover`/`setKindHover`
    /// actions. `id: None` clears the hover (mirrors the old "hover nothing" call shape).
    pub fn hover_id(app: &mut Puzzle3dApp, granularity: &str, id: Option<&str>) -> Result<InvocationResult, Fault> {
        let targets: Vec<InteractionTarget> = id.map(|id| InteractionTarget { granularity: granularity.into(), id: id.into() }).into_iter().collect();
        let targets_json = serde_json::to_string(&targets).unwrap_or_default();
        dispatch(app, "interactionHover", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "channel": "pointer", "targets": targets_json })), None).await
    }

    /// 🖼️ The rendered body, as JSON — every panel/window assertion navigates this value.
    pub fn render_body(app: &mut Puzzle3dApp, body_key: &str) -> Value {
        let tree = semio_framework::io::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render");
        let mut stack = vec![&tree.root];
        let mut fallback_scene = None;
        while let Some(node) = stack.pop() {
            if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
                if surface.doc_schema == <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA {
                    let scene: semio_framework_ui_scene::World3dScene = semio_framework_ui_scene::decode(surface).expect("decode world scene");
                    if scene.interaction_json.is_some() {
                        return json!({ "schema": surface.doc_schema, "world3d": scene });
                    }
                    fallback_scene = Some(json!({ "schema": surface.doc_schema, "world3d": scene }));
                }
            }
            stack.extend(node.children.iter());
        }
        if let Some(scene) = fallback_scene {
            return scene;
        }
        serde_json::to_value(tree).expect("serialize rendered node")
    }

    /// 🪟️ The world composite body for one window INSTANCE — the `<body>:<windowInstanceId>` form is
    /// how a split pane asks for its own materialized options (see `ArtifactApp::render`).
    pub fn render_window(app: &mut Puzzle3dApp, window_id: &str) -> Value {
        render_body(app, &format!("{}:{window_id}", main::BODY_KEY))
    }

    pub fn render_composite(app: &mut Puzzle3dApp) -> Value {
        render_body(app, main::BODY_KEY)
    }

    pub fn projection_of(app: &Puzzle3dApp) -> Value {
        semio_framework::io::resolve_ready(app.snapshot()).expect("projection").value().clone()
    }

    pub fn object_count(app: &Puzzle3dApp) -> usize {
        projection_of(app).get("objects").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0)
    }

    pub fn first_object_id(app: &Puzzle3dApp) -> String {
        projection_of(app).get("objects").and_then(Value::as_array).and_then(|objects| objects.first()).and_then(|object| object.get("id")).and_then(Value::as_str).expect("first object id").to_string()
    }

    pub fn vortex_full_ids(app: &Puzzle3dApp) -> Vec<String> {
        let projection = projection_of(app);
        let mut ids = Vec::new();
        for object in projection.get("objects").and_then(Value::as_array).into_iter().flatten() {
            let object_id = object.get("id").and_then(Value::as_str).unwrap_or_default();
            for vortex in object.get("vortices").and_then(Value::as_array).into_iter().flatten() {
                if let Some(vortex_id) = vortex.get("id").and_then(Value::as_str) {
                    ids.push(puzzle3d_vortex_full_id(object_id, vortex_id));
                }
            }
        }
        ids
    }

    pub fn first_vortex_full_id(app: &Puzzle3dApp) -> String {
        vortex_full_ids(app).into_iter().next().expect("seed vortex")
    }

    //#region 🔖️SceneProbes
    fn scene_field(node: &Value, field: &str) -> Value {
        node.pointer(&format!("/world3d/{field}")).and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
    }

    pub fn instances_of(node: &Value) -> Vec<Value> {
        scene_field(node, "instancesJson").as_array().cloned().unwrap_or_default()
    }

    pub fn instance_count(node: &Value) -> usize {
        instances_of(node).len()
    }

    pub fn vortices_of(node: &Value) -> Vec<Value> {
        scene_field(node, "vorticesJson").as_array().cloned().unwrap_or_default()
    }

    pub fn interaction_of(node: &Value) -> Value {
        scene_field(node, "interactionJson")
    }

    pub fn selection_of(node: &Value) -> Value {
        scene_field(node, "selectionJson")
    }

    pub fn lod_of(node: &Value) -> Value {
        scene_field(node, "lodJson")
    }

    pub fn camera_of(node: &Value) -> Value {
        scene_field(node, "cameraJson")
    }

    pub fn brush_preview_of(node: &Value) -> Value {
        scene_field(node, "brushPreviewJson")
    }
    //#endregion 🔖️SceneProbes

    //#region 🔖️MeasureProbes
    /// 🔍️ Depth-first search for a `WindowMeasure::Slider`'s value by id, descending into groups (the
    /// fill-count slider nests inside the fill tool's measure group rather than sitting on the engagement).
    pub fn find_measure_slider(measures: &[WindowMeasure], slider_id: &str) -> Option<f64> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Slider { id, value, .. } if id == slider_id => Some(*value),
            WindowMeasure::Group { children, .. } => find_measure_slider(children, slider_id),
            _ => None,
        })
    }

    pub fn find_measure_slider_max(measures: &[WindowMeasure], slider_id: &str) -> Option<f64> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Slider { id, max, .. } if id == slider_id => Some(*max),
            WindowMeasure::Group { children, .. } => find_measure_slider_max(children, slider_id),
            _ => None,
        })
    }

    pub fn find_measure_slider_ready(measures: &[WindowMeasure], slider_id: &str) -> Option<f64> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Slider { id, ready, .. } if id == slider_id => *ready,
            WindowMeasure::Group { children, .. } => find_measure_slider_ready(children, slider_id),
            _ => None,
        })
    }

    pub fn find_measure_select(measures: &[WindowMeasure], select_id: &str) -> Option<String> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Select { id, value, .. } if id == select_id => Some(value.clone()),
            WindowMeasure::Group { children, .. } => find_measure_select(children, select_id),
            _ => None,
        })
    }

    pub fn find_measure_toggle(measures: &[WindowMeasure], toggle_id: &str) -> Option<bool> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Toggle { id, pressed, .. } if id == toggle_id => Some(*pressed),
            WindowMeasure::Group { children, .. } => find_measure_toggle(children, toggle_id),
            _ => None,
        })
    }

    /// 🎯️ Top-level utility tag of a `WindowMeasure::Group` by id, or `None` when the group is absent.
    pub fn measure_group_tag(measures: &[WindowMeasure], group_id: &str) -> Option<Option<String>> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Group { id, active_utility_id, .. } if id == group_id => Some(active_utility_id.clone()),
            _ => None,
        })
    }

    /// 🪣️ How far background fill planning has preloaded, read off the fill tool's own count slider.
    pub fn fill_ready(app: &mut Puzzle3dApp) -> f64 {
        semio_framework::io::resolve_ready(app.tool_measures()).get(fill_tool::TOOL_ID).and_then(|tool_measures| find_measure_slider_ready(tool_measures, "puzzle3d-fill-count")).unwrap_or(0.0)
    }

    /// 🪣️ Drives `fillBuildTick` until planning has reached `target` placements (or the budget runs out).
    pub fn drive_fill_until_ready(app: &mut Puzzle3dApp, target: f64) -> f64 {
        for _ in 0..256 {
            dispatch(app, "fillBuildTick", None, None).await.expect("fillBuildTick");
            if fill_ready(app) >= target {
                break;
            }
            with_puzzle3d_app_mut(|inner| inner.precompute.borrow_mut().drive_enqueued_fill_job_for_test(64));
        }
        fill_ready(app)
    }

    /// 🧵️ Drives the same generation-tagged fill continuation the host executes in production.
    pub fn finish_fill_count(app: &mut Puzzle3dApp, mut result: InvocationResult) -> (usize, std::time::Duration) {
        let mut max_step = std::time::Duration::ZERO;
        for step in 0..1_024 {
            let next = result.requested_effects.into_iter().find_map(|effect| match effect {
                Effect::DispatchAction { action, args, .. } if action == set_fill_count::STEP_ACTION_ID => Some(args.map(|value| semio_framework::from_dsl_value::<Value>(value).expect("fill-count step args decode"))),
                _ => None,
            });
            let Some(args) = next else {
                return (step, max_step);
            };
            let started = std::time::Instant::now();
            result = dispatch(app, set_fill_count::STEP_ACTION_ID, args.as_ref(), None).await.expect("advance fill-count materialization");
            let elapsed = started.elapsed();
            max_step = max_step.max(elapsed);
            assert!(result.mutations.len() <= set_fill_count::MAX_PLACEMENTS_PER_STEP * 2, "one fill-count continuation exceeded its fixed semantic mutation bound");
        }
        panic!("fill-count materialization did not finish within its deterministic step bound");
    }

    pub fn set_fill_count_and_finish(app: &mut Puzzle3dApp, value: u32, window_id: Option<&str>) -> (usize, std::time::Duration) {
        let result = dispatch(app, "setFillCount", Some(&json!({ "value": value })), window_id).await.expect("begin setFillCount");
        finish_fill_count(app, result).await
    }
    //#endregion 🔖️MeasureProbes

    /// 🖱️ `context_menu()` through the `VcsArtifactApp` funnel (already-organized rows).
    pub fn context_menu_direct(app: &mut Puzzle3dApp) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{ContextMenuRequest, UiMenuRef};
        let request = ContextMenuRequest { menu: UiMenuRef { id: "world3d".into(), args: None }, surface: None, window_instance_id: None, point: None };
        semio_framework::io::resolve_ready(app.context_menu(&request))
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `context_menu` reads the
    /// CLIENT-supplied `request.surface.selection` now (selection is framework-owned, no live config
    /// field to derive it from) — the test-side replacement for the deleted `contextMenuAt` command's
    /// "select then open the menu" round trip.
    pub fn context_menu_for_selection(app: &mut Puzzle3dApp, granularity: &str, id: &str) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};
        let request = ContextMenuRequest {
            menu: UiMenuRef { id: "world3d".into(), args: None },
            surface: Some(ContextMenuSurfaceTarget { surface_id: "world3d".into(), kind: "world3d".into(), hits: Vec::new(), selection: vec![ContextMenuSelectionGroup { domain: granularity.into(), ids: vec![id.to_string()] }], text: None }),
            window_instance_id: None,
            point: None,
        };
        semio_framework::io::resolve_ready(app.context_menu(&request))
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::testkit::*;
    use super::*;
    use crate::editor::puzzle3d::config::Puzzle3dCamera;
    use protocol::MutationDiff;
    use semio_framework_plugin::{testkit as framework_testkit, EditorApp, PluginApp};

    #[test]
    fn two_documents_carry_independent_serialized_checkpoints() {
        let first = Puzzle3dConfig::default();
        let second = Puzzle3dConfig::default();
        assert!(first.fill_checkpoint.is_empty());
        assert!(second.fill_checkpoint.is_empty());
        assert_eq!(serde_json::to_value(first).unwrap(), serde_json::to_value(second).unwrap());
    }

    //#region 🔖️Operations
    #[semio_framework_async_macros::async_test]
    async fn renders_world_scene() {
        let mut app = app();
        assert!(render_composite(&mut app).to_string().contains("world-3d"));
    }

    #[semio_framework_async_macros::async_test]
    async fn initial_snapshot_is_the_concrete_forest_fixture() {
        let app = app();
        assert_eq!(projection_of(&app).get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_FIXTURE_SCHEMA));
        assert!(object_count(&app) > 0, "the concrete-forest default fixture ships with objects");
    }

    /// 📦️ `Puzzle3dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
    /// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
    /// `serde_json::Value` bridge impls), reusing the default concrete-forest fixture.
    #[semio_framework_async_macros::async_test]
    async fn puzzle3d_play_projection_pack_round_trips() {
        let app = app();
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&semio_framework::io::resolve_ready(app.snapshot()).expect("projection"));
    }

    #[semio_framework_async_macros::async_test]
    async fn open_add_object_dialog_emits_the_open_dialog_effect_with_no_document_change() {
        let mut app = app();
        let before = object_count(&app);
        let result = dispatch(&mut app, "openAddObjectDialog", None, None).await.expect("openAddObjectDialog");
        assert!(
            matches!(result.requested_effects.as_slice(), [Effect::OpenDialog { dialog_id, args, .. }] if dialog_id == "addObject" && args.is_none()),
            "expected a single OpenDialog effect for the addObject dialog, got {:?}",
            result.requested_effects,
        );
        assert_eq!(object_count(&app), before, "opening the dialog does not mutate the document");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_example_swaps_the_document_and_undo_restores_it() {
        let mut app = app();
        let loaded = object_count(&app);
        assert!(loaded > 0);
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
        assert_eq!(object_count(&app), 0, "empty example clears the objects");
        dispatch(&mut app, "undo", None, None).await.expect("undo");
        assert_eq!(object_count(&app), loaded, "undo restores the concrete-forest objects");
        dispatch(&mut app, "redo", None, None).await.expect("redo");
        assert_eq!(object_count(&app), 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn nakagin_example_loads_via_operations() {
        let mut app = app();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin");
        let projection = projection_of(&app);
        assert_eq!(projection.get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_FIXTURE_SCHEMA));
        assert!(projection.get("objects").and_then(|value| value.as_array()).is_some_and(|objects| !objects.is_empty()));
    }

    #[semio_framework_async_macros::async_test]
    async fn document_and_inspector_panels_render() {
        let mut app = app();
        for body in [document::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY, settings_panel::BODY_KEY] {
            assert!(!render_body(&mut app, body).to_string().is_empty());
        }
    }
    //#endregion 🔖️Operations

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`). Deliberately
    /// dispatches through a standalone typed `Puzzle3dStore` — NOT through `Puzzle3dPlayApp`/
    /// `Puzzle3dPlaySnapshot` (the `🔖️ValueBridge` `serde_json::Value` wrapper this app still uses)
    /// — since `Puzzle3dMutation`'s canonical `Mutation<Puzzle3dSnapshot>` impl (not its
    /// `Mutation<Value>` bridge impl) is what the CW7 law is about.
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::puzzle3d::spr::Puzzle3dStore;
        use crate::artifacts::puzzle3d::{Puzzle3dObject as TypedObject, PUZZLE_3D_SCHEMA};
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand};

        let mut store = semio_framework::io::resolve_ready(Puzzle3dStore::new(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", Puzzle3dSnapshot::default(), None))).expect("store");
        let object = TypedObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
        semio_framework::io::resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::artifacts::puzzle3d::mutations::create_object(object, None)], description: None })).expect("apply");
        let envelope = semio_framework::io::resolve_ready(store.envelope());
        let edit: &Edit<Puzzle3dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
        semio_framework::io::resolve_ready(semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle3dSnapshot, Puzzle3dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())));
    }
    //#endregion 🔖️CommandEnvelopeTests

    //#region 🔖️Inspector
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove a
    /// selected object's Origin nests x/y/z steppers. `panels::inspection::render` has no live
    /// selection to switch on any more (see that module's doc comment — `ArtifactApp::render` never
    /// gained an `InteractionView` parameter) and always falls through to the document summary now,
    /// selected or not — this proves that degraded floor instead of the since-unreachable steppers.
    #[semio_framework_async_macros::async_test]
    async fn selected_object_inspector_nests_origin_into_x_y_z_steppers() {
        let mut app = app_with_registry();
        let object_id = first_object_id(&app);
        select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
        let json = render_body(&mut app, inspection::BODY_KEY).to_string();
        assert!(json.contains("puzzle3d-play-inspector.empty"), "render has no InteractionView, so the inspector cannot key off the selection and always shows the document summary: {json}");
    }

    fn object_origin_x(app: &Puzzle3dApp, object_id: &str) -> f64 {
        projection_of(app)
            .get("objects")
            .and_then(Value::as_array)
            .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
            .and_then(|object| object.get("origin").and_then(Value::as_array).and_then(|origin| origin.first()).and_then(Value::as_f64))
            .expect("origin.x")
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_inspector_origin_axis_sets_absolute_value_and_preserves_other_axes() {
        let mut app = app();
        let object_id = first_object_id(&app);
        let before_y = projection_of(&app)
            .get("objects")
            .and_then(|value| value.as_array())
            .and_then(|objects| objects.first())
            .and_then(|object| object.get("origin"))
            .and_then(|value| value.as_array())
            .and_then(|origin| origin.get(1))
            .and_then(|value| value.as_f64())
            .expect("origin.y");
        dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "field": "origin.x", "value": 42.5 })), None).await.expect("patchInspector");
        let projection = projection_of(&app);
        let objects = projection.get("objects").and_then(|value| value.as_array()).expect("objects");
        let object = objects.iter().find(|object| object.get("id").and_then(|value| value.as_str()) == Some(object_id.as_str())).expect("patched object");
        let origin = object.get("origin").and_then(|value| value.as_array()).expect("origin");
        assert_eq!(origin[0].as_f64(), Some(42.5), "origin.x should be set to the absolute value");
        assert_eq!(origin[1].as_f64(), Some(before_y), "origin.y should be untouched by an origin.x edit");
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_inspector_origin_axis_delta_offsets_each_selected_object_from_its_own_current_value() {
        let mut app = app();
        let id_a = first_object_id(&app);
        dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [10.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
        let id_b = projection_of(&app).get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("id")).and_then(Value::as_str).expect("added object id").to_string();
        assert_ne!(id_a, id_b, "the added object must be distinct from the first fixture object");
        let x_a_before = object_origin_x(&app, &id_a);
        let x_b_before = object_origin_x(&app, &id_b);
        assert_ne!(x_a_before, x_b_before, "the two objects must start at different x values for this test to prove per-object offset preservation");
        dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "ids": [id_a.clone(), id_b.clone()], "field": "origin.x", "delta": 3.0 })), None).await.expect("patchInspector");
        assert_eq!(object_origin_x(&app, &id_a), x_a_before + 3.0, "a delta edit adds to each object's own current x");
        assert_eq!(object_origin_x(&app, &id_b), x_b_before + 3.0, "a delta edit preserves each object's own starting offset");
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the inspector chrome no longer
    /// renders per-entity stepper controls to inspect (see the sibling test's doc comment above), so
    /// this now proves the same "resolve selection without embedding ids" contract one level down, at
    /// `patchInspector`'s own `interaction.selection(vortex)` fallback (`commands::patch_inspector`) —
    /// a bare `field`/`value` patch (no `ids` arg) must resolve against whatever the `vortex` domain's
    /// `object` granularity currently holds.
    #[semio_framework_async_macros::async_test]
    async fn inspector_field_actions_resolve_selection_without_embedding_ids() {
        let mut app = app_with_registry();
        let object_id = first_object_id(&app);
        select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
        dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "field": "origin.x", "value": 42.5 })), None).await.expect("patchInspector without ids");
        assert_eq!(object_origin_x(&app, &object_id), 42.5, "patchInspector must resolve the patched object from the live selection, not an embedded id");
    }
    //#endregion 🔖️Inspector

    //#region 🔖️Manifest
    #[semio_framework_async_macros::async_test]
    async fn app_definition_has_the_main_world_window() {
        let definition = create_puzzle3d_app();
        assert!(definition.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn app_definition_declares_the_add_object_dialog() {
        let definition = create_puzzle3d_app();
        let dialog = definition.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog declared");
        assert_eq!(dialog.submit_action.as_str(), "addObjectKind");
        assert_eq!(dialog.args.len(), 1);
    }

    /// 📌️ The four declared panel tabs are present (the framework injects its own tabs alongside, so
    /// this asserts presence, never a total count).
    #[semio_framework_async_macros::async_test]
    async fn app_definition_declares_its_four_panel_tabs() {
        let definition = create_puzzle3d_app();
        let body_keys: Vec<&str> = definition.panel_tabs.iter().filter_map(|tab| tab.body_key.as_deref()).collect();
        for expected in [document::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY, settings_panel::BODY_KEY] {
            assert!(body_keys.contains(&expected), "panel tab body {expected} must be declared, got {body_keys:?}");
        }
    }

    /// 🌉️ Every declared action must bridge through `command_from_action` and round-trip
    /// `command_id` via the shared framework harness.
    #[semio_framework_async_macros::async_test]
    async fn every_declared_action_bridges_to_a_command() {
        semio_framework::io::resolve_ready(semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<Puzzle3dPlayApp>>(puzzle3d_manifest_for_testkit));
        assert!(semio_framework::io::resolve_ready(Puzzle3dPlayApp::command_from_action("noSuchAction", None)).is_err());
    }

    /// 🌉️ Every declared app action (framework-injected verbs never reach `Puzzle3dCommand::from_action`
    /// by design, so they simply fall through the `None` branch below) round-trips through the
    /// macro-generated `Puzzle3dCommand::from_action`/`action_id` pair as well as the `ArtifactApp`
    /// bridge asserted above.
    #[semio_framework_async_macros::async_test]
    async fn every_declared_action_round_trips_through_the_command_enum() {
        let definition = create_puzzle3d_app();
        for action in definition.window_kinds.iter().flat_map(|window| window.actions.iter()) {
            let Some(command) = Puzzle3dCommand::from_action(&action.id, None, None) else {
                continue;
            };
            assert_eq!(command.action_id(), action.id.as_str(), "declared action {} must round-trip through Puzzle3dCommand", action.id);
        }
    }

    /// 🗣️ B1: manifest text is baked into `AppDefinition`/`App` as `LocalizedLabel` and resolved
    /// directly via `.resolve(Terminology, Locale)` — no shell round-trip needed to assert on it.
    #[semio_framework_async_macros::async_test]
    async fn app_definition_labels_resolve_german_reuse_branded_for_aggregator() {
        use semio_framework_plugin::{Locale, Terminology};
        let definition = create_puzzle3d_app();
        let def = &definition;
        let (terminology, locale) = (Terminology::Reuse, Locale::De);
        let actions = || def.window_kinds.iter().flat_map(|window| window.actions.iter());
        let action = |id: &str| actions().find(|entry| entry.id == id).unwrap_or_else(|| panic!("{id} action declared"));
        assert_eq!(def.modes.iter().find(|entry| entry.id == "edit").expect("edit mode").label.resolve(terminology, locale), "Bearbeiten");
        assert_eq!(def.window_kinds.iter().find(|entry| entry.id == main::WINDOW_KIND_ID).expect("window kind").label.resolve(terminology, locale), "Aggregator");
        let dialog = def.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog");
        assert_eq!(dialog.title.resolve(terminology, locale), "Baukomponente hinzufügen");
        assert_eq!(dialog.submit_label.resolve(terminology, locale), "Hinzufügen");
        let arg = dialog.args.iter().find(|entry| entry.id == "objectKind").expect("objectKind arg");
        let option = match arg.control() {
            semio_framework_plugin::ActionArgControl::Select { options } => options.iter().find(|entry| entry.value == "Object").cloned().expect("Object option"),
            _ => panic!("objectKind arg is not a select"),
        };
        assert_eq!(option.label.resolve(terminology, locale), "Baukomponente");
        assert_eq!(action("addObjectKind").label.resolve(terminology, locale), "Baukomponente hinzufügen");
        assert_eq!(action("openVortexSuggestions").label.resolve(terminology, locale), "Verbindungspunkt-Vorschläge öffnen");
        assert_eq!(action("createAttraction").label.resolve(terminology, locale), "Verbindung erstellen");
        assert_eq!(def.utilities.iter().find(|entry| entry.id == utilities::transform::UTILITY_ID).expect("transform utility").label.resolve(terminology, locale), "Transformieren");
        // 🎭️✏️ `create_puzzle3d_app()` no longer registers examples (see its own doc comment — `Editor::builder`
        // has no `.example(...)` and `AppDefinition` carries no `examples` field), so the concrete-forest
        // example's German label can no longer be asserted here; dropped, not silently left stale.
        let framework_interaction_actions = [
            INTERACTION_SELECT_ACTION_ID,
            semio_framework_plugin::INTERACTION_HOVER_ACTION_ID,
            semio_framework_plugin::CLEAR_SELECTION_ACTION_ID,
            semio_framework_plugin::SELECT_ALL_ACTION_ID,
            semio_framework_plugin::SET_SELECTION_MODE_ACTION_ID,
            semio_framework_plugin::SET_INTERACTION_GRANULARITY_ACTION_ID,
        ];
        for entry in actions() {
            if framework_interaction_actions.contains(&entry.id.as_str()) {
                continue;
            }
            let text = entry.label.resolve(terminology, locale);
            assert!(!text.contains("Hover") && !text.contains("Pick") && !text.contains("hovern"), "leftover English/mistranslation in {}: {text}", entry.id);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn app_definition_labels_stay_english_native_without_brand_locks() {
        use semio_framework_plugin::{Locale, Terminology};
        let definition = create_puzzle3d_app();
        let def = &definition;
        let (terminology, locale) = (Terminology::Native, Locale::En);
        let action = |id: &str| def.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == id).unwrap_or_else(|| panic!("{id} action declared"));
        assert_eq!(def.modes.iter().find(|entry| entry.id == "edit").expect("edit mode").label.resolve(terminology, locale), "Edit");
        assert_eq!(def.window_kinds.iter().find(|entry| entry.id == main::WINDOW_KIND_ID).expect("window kind").label.resolve(terminology, locale), "Puzzle 3D");
        assert_eq!(def.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog").title.resolve(terminology, locale), "Add Object");
        assert_eq!(action("addObjectKind").label.resolve(terminology, locale), "Add Object");
    }

    #[semio_framework_async_macros::async_test]
    async fn document_and_kinds_trees_use_german_reuse_section_labels() {
        let mut app = app();
        dispatch(&mut app, "setLocale", Some(&json!({ "value": "de" })), None).await.expect("setLocale");
        dispatch(&mut app, "setTerminology", Some(&json!({ "value": "reuse" })), None).await.expect("setTerminology");
        let document_json = render_body(&mut app, document::BODY_KEY).to_string();
        let kinds = render_body(&mut app, catalogue::BODY_KEY).to_string();
        let measures_json = serde_json::to_string(&semio_framework::io::resolve_ready(app.window_measures())).unwrap();
        assert!(document_json.contains("Baukomponenten"), "document tree objects section");
        assert!(document_json.contains("Verbindungen"), "document tree attractions section");
        assert!(document_json.contains("Referenzen"), "document tree references section");
        assert!(document_json.contains("Zielvolumina"), "document tree target volumes section");
        assert!(kinds.contains("Kabel"), "catalogue cables section");
        assert!(kinds.contains("Verbindungen"), "catalogue attractions section");
        assert!(!document_json.contains("\"Attractions\"") && !kinds.contains("\"Attractions\""), "English Attractions must not appear");
        assert!(!kinds.contains("\"Cables\""), "English Cables must not appear");
        assert!(measures_json.contains("Verbindungen"), "select measures attractions toggle");
        assert!(!measures_json.contains("\"Attractions\""), "select measures must not hardcode Attractions");
    }

    #[semio_framework_async_macros::async_test]
    async fn main_window_utilities_lead_with_transform_without_select_tool_and_no_default_utility() {
        let definition = create_puzzle3d_app();
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert!(!utility_ids.contains(&"select"), "puzzle 3d must not declare a select utility");
        assert!(!utility_ids.contains(&"scale"), "puzzle 3d must not declare a scale utility");
        assert!(!utility_ids.contains(&fill_tool::TOOL_ID), "fill is a mode-level tool, not a window utility");
        let window = definition.window_kinds.iter().find(|window| window.id == main::WINDOW_KIND_ID).expect("main window");
        let main_utilities: Vec<&str> = window.utilities.iter().map(|utility| utility.as_str()).collect();
        assert_eq!(main_utilities.first().copied(), Some(utilities::transform::UTILITY_ID));
        assert!(!main_utilities.contains(&"select"));
        assert!(!main_utilities.contains(&fill_tool::TOOL_ID), "fill must not be bound to the main window as a utility");
        assert_eq!(PUZZLE3D_DEFAULT_UTILITY, "", "unset/cleared host utility must not impersonate transform");
    }

    /// 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
    #[semio_framework_async_macros::async_test]
    async fn tool_registry_declares_fill_tool() {
        let definition = create_puzzle3d_app();
        let tool_ids: Vec<&str> = definition.tools.iter().map(|tool| tool.id.as_str()).collect();
        assert_eq!(tool_ids, vec![fill_tool::TOOL_ID]);
        assert_eq!(definition.modes[0].tools, vec![semio_framework::io::resolve_ready(ToolRef::new(fill_tool::TOOL_ID))]);
        assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID), "declaring tools must inject the setActiveTool action");
    }
    //#endregion 🔖️Manifest

    //#region 🔖️Suggestions
    #[semio_framework_async_macros::async_test]
    async fn context_menu_at_selects_vortex_and_prepends_suggest_objects() {
        let mut app = app();
        let vortex = first_vortex_full_id(&app);
        let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex);
        let menu_json = serde_json::to_string(&menu).unwrap();
        assert!(menu_json.contains("Suggest objects"), "menu should be {menu_json}");
        assert!(menu_json.contains("openVortexSuggestions"));
        assert!(menu_json.contains("sparkles"), "menu should include suggest icon: {menu_json}");
        assert!(menu_json.contains("Zoom to selection"), "menu should include zoom: {menu_json}");
        assert!(menu_json.contains("deleteSelection"), "menu should include delete: {menu_json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn context_menu_at_selects_target_volume_and_set_target_volume_flag_toggles_hidden() {
        let mut app = app();
        dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
        let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
        let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id);
        let menu_json = serde_json::to_string(&menu).unwrap();
        assert!(menu_json.contains("setTargetVolumeFlag"), "menu should be {menu_json}");
        assert!(menu_json.contains("menu.group.targets"), "hide/lock rows should be grouped under targets: {menu_json}");
        assert_eq!(menu.last().and_then(|item| item.destructive), Some(true), "destructive delete must be the last top-level row: {menu_json}");
        dispatch(&mut app, "setTargetVolumeFlag", Some(&json!({ "id": volume_id, "flag": "hidden", "value": true })), None).await.expect("setTargetVolumeFlag");
        let hidden = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("hidden")).and_then(Value::as_bool);
        assert_eq!(hidden, Some(true));
    }

    /// 🗂️ Grouped-disclosure contract for the object-selection branch: the top-level menu stays
    /// scannable (leaves + groups + separator combined) and the destructive `deleteSelection` row is
    /// the last top-level entry (`organize_context_menu` inserts the separator ahead of it).
    #[semio_framework_async_macros::async_test]
    async fn context_menu_at_selects_object_groups_flags_and_keeps_delete_last() {
        let mut app = app();
        dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
        let object_id = first_object_id(&app);
        let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id);
        assert!(menu.len() <= 9, "top-level menu should stay scannable, got {} rows: {menu:?}", menu.len());
        let menu_json = serde_json::to_string(&menu).unwrap();
        assert!(menu_json.contains("menu.group.hand"), "hide/lock rows should be grouped under hand: {menu_json}");
        assert!(menu_json.contains("duplicateSelection"), "menu should be {menu_json}");
        assert_eq!(menu.last().map(|item| item.id.as_str()), Some("delete"), "delete must be the last top-level row: {menu_json}");
        assert_eq!(menu.last().and_then(|item| item.destructive), Some(true), "delete must be marked destructive: {menu_json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn open_vortex_suggestions_opens_the_suggestion_popup() {
        let mut app = app();
        let vortex = first_vortex_full_id(&app);
        let result = dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 12.0, "y": 34.0 })), None).await.expect("openVortexSuggestions");
        assert!(
            result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })),
            "opening a one-shot suggestion must not switch the host-owned utility or tool: {:?}",
            result.requested_effects,
        );
        let interaction = interaction_of(&render_composite(&mut app));
        assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "context-menu suggestion stays in the current selection mode");
        let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
        assert_eq!(menu.get("open").and_then(Value::as_bool), Some(true));
        assert_eq!(menu.get("x").and_then(Value::as_f64), Some(12.0));
        assert_eq!(menu.get("y").and_then(Value::as_f64), Some(34.0));
        assert_eq!(menu.get("vortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
        assert!(menu.get("windowId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "suggestion menu is scoped to the opening window: {menu}");
    }

    #[semio_framework_async_macros::async_test]
    async fn open_vortex_suggestions_records_explicit_window_id() {
        let mut app = app();
        let vortex = first_vortex_full_id(&app);
        dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 8.0, "y": 16.0, "windowId": main::WINDOW_INSTANCE_TOP })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("openVortexSuggestions");
        let interaction = interaction_of(&render_composite(&mut app));
        let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
        assert_eq!(menu.get("windowId").and_then(Value::as_str), Some(main::WINDOW_INSTANCE_TOP));
        assert_eq!(menu.get("vortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
    }

    #[semio_framework_async_macros::async_test]
    async fn accept_suggestion_with_full_id_places_even_if_selection_was_cleared() {
        let mut app = app();
        let vortex = first_vortex_full_id(&app);
        dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
        let before_count = object_count(&app);
        // 🧹️ Simulate the split-pane outside-dismiss race clearing vortex selection before accept.
        dispatch(&mut app, "clearSelection", None, None).await.expect("clearSelection");
        let result = dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": vortex })), None).await.expect("acceptSuggestion");
        assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "accept must not switch utility/tool: {:?}", result.requested_effects);
        assert!(object_count(&app) > before_count, "accept with fullId must place even after selection clear");
        let interaction = interaction_of(&render_composite(&mut app));
        assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    }

    #[semio_framework_async_macros::async_test]
    async fn close_vortex_suggestions_clears_the_menu() {
        let mut app = app();
        let vortex = first_vortex_full_id(&app);
        dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
        dispatch(&mut app, "closeVortexSuggestions", None, None).await.expect("closeVortexSuggestions");
        let interaction = interaction_of(&render_composite(&mut app));
        assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    }

    /// 🖱️ Hovering a row in the suggestion popup must live-update the 3D brush preview (rendered by
    /// `world_brush_preview_json`, which reads `runtime.brush_candidate_index`) to the hovered
    /// candidate, so the UI can highlight it in 3D before the user clicks — without switching the
    /// host-owned active utility into brush mode.
    #[semio_framework_async_macros::async_test]
    async fn hover_suggestion_updates_the_brush_candidate_index_and_live_preview() {
        let mut app = app();
        let vortex = first_vortex_full_id(&app);
        dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
        let composite = render_composite(&mut app);
        let interaction = interaction_of(&composite);
        assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "suggestion hover must not enter brush mode");
        assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(0), "opening suggestions starts hover at the first candidate");
        let candidates = interaction.pointer("/suggestionMenu/candidates").and_then(Value::as_array).cloned().unwrap_or_default();
        assert!(!candidates.is_empty(), "suggestion candidates should be present");
        assert!(candidates[0].get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "candidates carry object-kind color: {candidates:?}");
        assert!(candidates[0].get("icon").and_then(Value::as_str).is_some_and(|icon| !icon.is_empty()), "candidates carry icon: {candidates:?}");
        let preview = brush_preview_of(&composite);
        assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "the live preview must target the vortex the suggestion menu was opened on");
        assert!(preview.get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "the live preview must resolve to a real candidate object kind");
        assert!(preview.get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "brush preview carries object-kind color: {preview}");

        dispatch(&mut app, "hoverSuggestion", Some(&json!({ "index": 1 })), None).await.expect("hoverSuggestion");
        let composite = render_composite(&mut app);
        let interaction = interaction_of(&composite);
        assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(1), "hovering a different row must move the tracked candidate index");
        let preview = brush_preview_of(&composite);
        assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "the preview must keep targeting the same vortex while only the hovered candidate changes");
        assert!(preview.get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "hovered brush preview still carries color: {preview}");
    }

    #[semio_framework_async_macros::async_test]
    async fn accept_suggestion_appends_an_object_and_closes_the_menu() {
        let mut app = app();
        let object_count_before = object_count(&app);
        let vortex = first_vortex_full_id(&app);
        dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
        let result = dispatch(&mut app, "acceptSuggestion", None, None).await.expect("acceptSuggestion");
        assert_eq!(object_count(&app), object_count_before + 1);
        assert!(
            result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })),
            "accepting a one-shot suggestion must leave the host-owned utility/tool unchanged: {:?}",
            result.requested_effects,
        );
        let composite = render_composite(&mut app);
        let interaction = interaction_of(&composite);
        assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
        assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"));
        assert!(interaction.get("hoveredVortexFullId").is_none_or(|value| value.is_null()), "accept must clear sticky vortex hover");
        let selected_vortices = vortices_of(&composite).iter().filter(|entry| entry.get("selected").and_then(Value::as_bool) == Some(true)).count();
        assert_eq!(selected_vortices, 0, "one-shot accept must leave no sticky vortex selection");
    }

    /// 🧹️ A failed place (unknown vortex) must still close the suggestion menu — otherwise
    /// `suggestionMenu.open` stays true and every split pane's regular context menu is gated shut.
    #[semio_framework_async_macros::async_test]
    async fn accept_suggestion_closes_menu_even_when_placement_fails() {
        // 🕹️ `hover_id` dispatches through the real `interactionHover` verb, which resolves the
        // `vortex` domain against `self.registry` — a plain `app()` carries no registry (see
        // `testkit::new_app`'s doc), so this needs the registry-backed `app_with_registry()`.
        let mut app = app_with_registry();
        let vortex = first_vortex_full_id(&app);
        hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
        dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).await.expect("openVortexSuggestions");
        let before = interaction_of(&render_composite(&mut app));
        assert_eq!(before.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
        let object_count_before = object_count(&app);
        dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": "missing-object::missing-vortex" })), None).await.expect("acceptSuggestion");
        assert_eq!(object_count(&app), object_count_before, "unknown-vortex accept must not place");
        let interaction = interaction_of(&render_composite(&mut app));
        assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()), "failed accept must still dismiss the suggestion menu");
    }

    #[semio_framework_async_macros::async_test]
    async fn close_vortex_suggestions_clears_sticky_hover() {
        let mut app = app_with_registry();
        let vortex = first_vortex_full_id(&app);
        hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
        dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
        dispatch(&mut app, "closeVortexSuggestions", None, None).await.expect("closeVortexSuggestions");
        let interaction = interaction_of(&render_composite(&mut app));
        assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    }

    /// 🧰️ Context-menu / Alt+right-click suggestions are a one-shot placement: opening and accepting
    /// must leave whatever host-owned utility was already active (e.g. transform) untouched.
    #[semio_framework_async_macros::async_test]
    async fn open_and_accept_vortex_suggestions_preserve_active_utility() {
        let mut app = app();
        dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("activate transform");
        let vortex = first_vortex_full_id(&app);
        let open = dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 0.0, "y": 0.0 })), Some(main::WINDOW_KIND_ID)).await.expect("openVortexSuggestions");
        assert!(open.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "opening suggestions must not emit utility/tool switches: {:?}", open.requested_effects);
        let open_node = render_window(&mut app, main::WINDOW_KIND_ID);
        let open_interaction = interaction_of(&open_node);
        assert_eq!(open_interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "transform remains non-brush scene mode during suggestions");
        assert_eq!(open_interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
        assert!(brush_preview_of(&open_node).get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "one-shot suggestions still emit a placement preview without entering brush mode");
        let accept = dispatch(&mut app, "acceptSuggestion", None, Some(main::WINDOW_KIND_ID)).await.expect("acceptSuggestion");
        assert!(accept.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "accepting suggestions must not emit utility/tool switches: {:?}", accept.requested_effects);
        let accept_interaction = interaction_of(&render_window(&mut app, main::WINDOW_KIND_ID));
        assert!(accept_interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
        assert_eq!(accept_interaction.get("activeUtility").and_then(Value::as_str), Some("select"));
    }
    //#endregion 🔖️Suggestions

    //#region 🔖️WindowOptions
    #[semio_framework_async_macros::async_test]
    async fn grid_window_options_control_one_visible_grid_spacing() {
        let mut app = app();
        dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), None).await.expect("setGridVisible");
        dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 7.5 })), None).await.expect("setGridSpacing");
        let lod = lod_of(&render_composite(&mut app));
        assert_eq!(lod.get("showLodGrid").and_then(Value::as_bool), Some(false));
        assert_eq!(lod.get("gridFactor").and_then(Value::as_f64), Some(7.5));
        let measures = semio_framework::io::resolve_ready(app.window_measures());
        let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
        assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid")), Some(None));
        assert_eq!(find_measure_slider(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing")), Some(7.5));
    }

    /// 🪟️ Two window instances of the same kind (a split top/perspective pane pair) must never share
    /// window options — toggling grid visibility in one instance must leave every other instance's
    /// grid untouched, both in its measures chrome and in its own rendered scene.
    #[semio_framework_async_macros::async_test]
    async fn window_options_are_local_to_the_window_instance_not_shared_across_split_panes() {
        let mut app = app();
        let second_window = "puzzle3d-main-2";
        let toggle_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible");

        // Register both instances by dispatching a no-op-ish view action from each.
        dispatch(&mut app, "worldPointerDown", None, Some(main::WINDOW_KIND_ID)).await.expect("register base window");
        dispatch(&mut app, "worldPointerDown", None, Some(second_window)).await.expect("register second window");

        // Both instances start visible (the type default).
        let initial_measures = semio_framework::io::resolve_ready(app.window_measures());
        assert_eq!(find_measure_toggle(initial_measures.get(main::WINDOW_KIND_ID).expect("base measures"), &toggle_id), Some(true));
        assert_eq!(find_measure_toggle(initial_measures.get(second_window).expect("second measures"), &toggle_id), Some(true));

        // Hide the grid, but ONLY on the second window instance.
        dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(second_window)).await.expect("setGridVisible on second window");

        let measures_after = semio_framework::io::resolve_ready(app.window_measures());
        assert_eq!(find_measure_toggle(measures_after.get(main::WINDOW_KIND_ID).expect("base measures"), &toggle_id), Some(true), "the base window instance's grid must stay visible");
        assert_eq!(find_measure_toggle(measures_after.get(second_window).expect("second measures"), &toggle_id), Some(false), "only the targeted window instance's grid toggles off");

        // The rendered scenes agree: the base window still draws its LOD grid, the second does not.
        assert_eq!(lod_of(&render_window(&mut app, main::WINDOW_KIND_ID)).get("showLodGrid").and_then(Value::as_bool), Some(true));
        assert_eq!(lod_of(&render_window(&mut app, second_window)).get("showLodGrid").and_then(Value::as_bool), Some(false));
    }

    /// 🎥️ `setCamera`/`setProjection`/`setProjectionParam`/`focusSelection` moved off the document —
    /// they are View-kind and must never emit VCS operations, no matter what they mutate.
    #[semio_framework_async_macros::async_test]
    async fn camera_actions_are_view_actions_that_emit_no_artifact_mutations() {
        let app_definition = create_puzzle3d_app();
        for action_id in ["setCamera", "setProjection", "setProjectionParam", "focusSelection"] {
            let def = app_definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("{action_id} declared"));
            assert_eq!(def.kind, ActionKind::View, "{action_id} must be a View action — camera is session-only, never a VCS edit");
        }
        let mut live = app_with_registry();
        let before = projection_of(&live);
        let result = dispatch(&mut live, "setCamera", Some(&json!({ "camera": { "position": [1.0, 2.0, 3.0], "target": [4.0, 5.0, 6.0], "zoom": 2.5 } })), None).await.expect("setCamera");
        assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
        assert_eq!(projection_of(&live), before, "setCamera must not mutate the document");
    }

    /// 🪟️📷️ Orbiting one window instance's camera must never move any sibling instance's camera, and
    /// must never touch the shared document.
    #[semio_framework_async_macros::async_test]
    async fn set_camera_is_per_window_and_leaves_sibling_windows_and_the_document_untouched() {
        let mut app = app();
        let window_a = "puzzle3d-main-a";
        let window_b = "puzzle3d-main-b";
        dispatch(&mut app, "worldPointerDown", None, Some(window_a)).await.expect("register a");
        dispatch(&mut app, "worldPointerDown", None, Some(window_b)).await.expect("register b");

        let before_document = projection_of(&app);
        let camera_b_before = camera_of(&render_window(&mut app, window_b));

        let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [11.0, 22.0, 33.0], "target": [1.0, 2.0, 3.0], "zoom": 4.0 } })), Some(window_a)).await.expect("setCamera on window A");
        assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
        assert_eq!(projection_of(&app), before_document, "setCamera must never mutate the shared document");

        let camera_a_after = camera_of(&render_window(&mut app, window_a));
        assert_eq!(camera_a_after.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "window A's own rendered camera picks up the new pose");
        assert_eq!(camera_of(&render_window(&mut app, window_b)), camera_b_before, "window B's rendered camera must be unaffected by window A's setCamera");
    }

    #[semio_framework_async_macros::async_test]
    async fn vortex_show_window_option_defaults_to_selected_and_switches_to_always() {
        let mut app = app();
        let all_vortex_ids = vortex_full_ids(&app);
        assert!(!all_vortex_ids.is_empty(), "fixture must expose vortices");
        let measures = semio_framework::io::resolve_ready(app.window_measures());
        let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
        assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_SELECTED));

        assert!(vortices_of(&render_composite(&mut app)).is_empty(), "Selected mode must hide vortices while idle");

        dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow always");
        let measures_always = semio_framework::io::resolve_ready(app.window_measures());
        let window_measures_always = measures_always.get(main::WINDOW_KIND_ID).expect("main window measures");
        assert_eq!(find_measure_select(window_measures_always, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_ALWAYS));
        assert_eq!(vortices_of(&render_composite(&mut app)).len(), all_vortex_ids.len(), "Always mode must emit every vortex while idle");

        dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_SELECTED })), None).await.expect("setVortexShow selected");
        assert!(vortices_of(&render_composite(&mut app)).is_empty(), "switching back to Selected must hide idle vortices");
    }

    #[semio_framework_async_macros::async_test]
    async fn vortex_direction_window_option_defaults_to_outwards_and_switches_to_inwards() {
        let mut app = app();
        let measures = semio_framework::io::resolve_ready(app.window_measures());
        let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
        assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS));

        dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow always");
        let outwards_vortices = vortices_of(&render_composite(&mut app));
        assert!(!outwards_vortices.is_empty(), "fixture must expose vortices");
        assert!(outwards_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

        dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), None).await.expect("setVortexDirection inwards");
        let measures_inwards = semio_framework::io::resolve_ready(app.window_measures());
        let window_measures_inwards = measures_inwards.get(main::WINDOW_KIND_ID).expect("main window measures");
        assert_eq!(find_measure_select(window_measures_inwards, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS));
        assert!(vortices_of(&render_composite(&mut app)).iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
    }

    #[semio_framework_async_macros::async_test]
    async fn vortex_direction_option_is_local_to_the_window_instance() {
        let mut app = app();
        let second_window = "puzzle3d-main-2";
        dispatch(&mut app, "worldPointerDown", None, Some(main::WINDOW_KIND_ID)).await.expect("register base window");
        dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(main::WINDOW_KIND_ID)).await.expect("setVortexShow always on base");
        dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(second_window)).await.expect("setVortexShow always on second");
        dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), Some(second_window)).await.expect("setVortexDirection inwards on second window");

        let base_vortices = vortices_of(&render_window(&mut app, main::WINDOW_KIND_ID));
        assert!(!base_vortices.is_empty(), "the base window must still emit vortices");
        assert!(base_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

        let second_vortices = vortices_of(&render_window(&mut app, second_window));
        assert!(second_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
    }
    //#endregion 🔖️WindowOptions

    //#region 🔖️Fill
    #[semio_framework_async_macros::async_test]
    async fn fill_build_tick_is_ignored_when_fill_tool_is_inactive() {
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `handle_action_impl` now takes
        // an `InteractionView`, which no external crate can construct directly (its fields are
        // `pub(crate)` to `semio_framework_plugin`, no public constructor exists — flagged to the
        // coordinator as a testability gap). Routed through the real `dispatch()`/`with_puzzle3d_app`
        // machinery instead, which builds one internally.
        //
        // 🕹️ Pre-existing (unrelated to this ticket) framework testability gap in
        // `VcsArtifactApp::dispatch_typed`/`finish_recorded`: `dispatch_typed` always tags its call to
        // `finish_recorded` with the literal verb `"typed-command"` (never the real action id), so
        // `finish_recorded`'s `self.registry.get(verb)` lookup can never resolve `fillBuildTick`'s
        // declared `ActionKind::View` — `skip_history_panel` is unreachable via this path regardless of
        // registry population, and every `dispatch_typed` call that logs anything (`log_generation`
        // advances) picks up a `Partial { panel_bodies: ["framework.body.history"] }` refresh. Confirmed
        // present before this ticket too (`finish_recorded`'s registry lookup, not its
        // `ActionKind::View | ActionKind::Interaction` match arm, is what fails) — flagged to the
        // coordinator, not fixed here (framework file, out of this crate's remit). Asserts the real
        // regression guard (no progression while inactive) plus the weaker-but-true scope bound (never
        // a `Full` refresh) instead of the unreachable exact `None`.
        let mut app = app();
        dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("activate fill");
        dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": Value::Null })), None).await.expect("deactivate fill");
        let before = with_puzzle3d_app(|inner| inner.precompute.borrow().fill_progress_summary());
        for _ in 0..64 {
            let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
            assert!(!matches!(result.ui_scope, UiDirtyScope::Full), "an inactive fill tick must never force a full app refresh");
        }
        let after = with_puzzle3d_app(|inner| inner.precompute.borrow().fill_progress_summary());
        assert_eq!(after, before, "stale or queued fill ticks must not advance planning after the Fill tool is deactivated");
    }

    #[semio_framework_async_macros::async_test]
    async fn fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job() {
        let mut app = app_with_registry();
        dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("activate fill");
        let before = with_puzzle3d_app(|inner| inner.precompute.borrow().fill_progress_summary());
        let first = dispatch(&mut app, "fillBuildTick", None, None).await.expect("enqueue fill");
        let after = with_puzzle3d_app(|inner| inner.precompute.borrow().fill_progress_summary());
        assert_eq!(after, before, "the view action must not execute a solver transition inline");
        assert!(matches!(
            first.requested_effects.as_slice(),
            [Effect::SpawnJob { kind, placement: semio_framework_plugin::kernel::JobPlacement::Isolated, .. }] if kind == crate::editor::puzzle3d::precompute::FILL_JOB_KIND
        ));
        let second = dispatch(&mut app, "fillBuildTick", None, None).await.expect("poll fill");
        assert!(!second.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { .. })), "a live fill request must not be enqueued twice");
    }

    #[semio_framework_async_macros::async_test]
    async fn fill_build_tick_only_plans_available_slider_range() {
        // 🐢️ `drive_precompute` is bounded to a small per-call budget (the fix for the UI-freeze bug:
        // a single action must never grind the whole precompute queue synchronously), so the build
        // converges over several ticks — exactly like the real 120ms `fillBuildTick` loop.
        let mut app = app_with_registry();
        let object_count_before = object_count(&app);
        dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
        drive_fill_until_ready(&mut app, 4.0).await;
        let measures = semio_framework::io::resolve_ready(app.tool_measures());
        let tool_measures = measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
        match find_measure_slider(tool_measures, "puzzle3d-fill-count") {
            Some(value) => assert_eq!(value, 0.0, "background planning must not change the selected fill count"),
            None => panic!("expected a fill-count slider in the fill tool measures"),
        }
        assert_eq!(object_count(&app), object_count_before, "background planning must not append generated objects below the slider count");
        assert_eq!(find_measure_slider_max(tool_measures, "puzzle3d-fill-count"), Some(PUZZLE3D_FILL_COUNT_MAX as f64), "fill slider range stays fixed at the fill count max");
        let available_count = find_measure_slider_ready(tool_measures, "puzzle3d-fill-count").expect("expected a fill-count slider ready extent") as usize;
        assert!(available_count > 0, "the fill slider ready extent must expose collision-free compatible placements");
        let begin = dispatch(&mut app, "setFillCount", Some(&json!({ "value": available_count })), None).await.expect("setFillCount");
        assert_eq!(object_count(&app), object_count_before, "the slider gesture only publishes the reveal cutoff; document materialization is resumable");
        let immediate = render_composite(&mut app);
        assert_eq!(instance_count(&immediate), object_count_before + available_count, "the complete planned prefix is previewed immediately before document continuations finish");
        assert_eq!(interaction_of(&immediate).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(available_count as u64), "the reveal cutoff updates in the initiating interaction step");
        let (_, max_step) = finish_fill_count(&mut app, begin).await;
        assert!(max_step < std::time::Duration::from_millis(8), "every fill-count continuation must remain below the hard 8 ms interaction ceiling");
        assert_eq!(object_count(&app), object_count_before + available_count, "the fill slider must materialize exactly its available placement count");
        assert_eq!(instance_count(&render_composite(&mut app)), object_count_before + available_count, "the viewport must show every materialized fill object immediately");
        let initial_fill_ids: HashSet<String> = projection_of(&app).get("objects").and_then(Value::as_array).into_iter().flatten().skip(object_count_before).filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect();
        // 🪪️ Incidental actions re-sync the applied document into the precompute session. That used to
        // rebuild `fill.base` around the materialized objects, after which the slider could neither
        // remove them nor replan — reproduce with a hover sync before clearing.
        let hovered_id = first_object_id(&app);
        hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(&hovered_id)).await.expect("interactionHover after fill");
        let reduced = available_count / 2;
        set_fill_count_and_finish(&mut app, reduced as u32, None).await;
        assert_eq!(object_count(&app), object_count_before + reduced, "sliding down after an incidental sync must still remove fill objects from the document");
        let reduced_render = render_composite(&mut app);
        // 🪣️ The viewport keeps showing the FULL available plan (tagged revealIndex) even after
        // reducing — hiding is a client-side reveal-cutoff concern now, not a server-side instance
        // count concern; only the document (checked above) and the committed cutoff actually shrink.
        assert_eq!(instance_count(&reduced_render), object_count_before + available_count, "the viewport still exposes the full plan for instant re-reveal — nothing was discarded");
        assert_eq!(interaction_of(&reduced_render).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(reduced as u64), "the committed reveal cutoff tracks the reduced count");
        // 🔽️🔼️ Prefix-stable plan: moving back up to a count that was already planned before must be
        // INSTANT — no replanning, no `fillBuildTick` catch-up dispatch.
        set_fill_count_and_finish(&mut app, available_count as u32, None).await;
        assert_eq!(object_count(&app), object_count_before + available_count, "moving back up within the preserved plan is instant, not gated on another fillBuildTick");
        let target_measures = semio_framework::io::resolve_ready(app.tool_measures());
        let target_tool_measures = target_measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
        assert_eq!(find_measure_slider(target_tool_measures, "puzzle3d-fill-count"), Some(available_count as f64));
        let restored_fill_ids: HashSet<String> = projection_of(&app).get("objects").and_then(Value::as_array).into_iter().flatten().skip(object_count_before).filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect();
        assert_eq!(restored_fill_ids, initial_fill_ids, "up-down-up restores the exact same planned objects — the plan is prefix-stable, never discarded and re-rolled");
        set_fill_count_and_finish(&mut app, 0, None).await;
        assert_eq!(object_count(&app), object_count_before, "moving the fill slider to zero must remove every generated object");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_fill_count_clamps_to_available_and_no_longer_dispatches_catch_up() {
        // 🔒️ Requesting more than is currently planned must clamp (never leave `runtime.fill_count`
        // and the applied document disagreeing), and `fillBuildTick` must never self-dispatch another
        // `setFillCount` — the viewport already shows every planned piece (tagged `revealIndex`), so
        // there is nothing left for a catch-up round trip to accomplish.
        let mut app = app();
        let object_count_before = object_count(&app);
        dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
        let available_count = drive_fill_until_ready(&mut app, PUZZLE3D_FILL_COUNT_MAX as f64).await as u32;
        assert!(available_count > 0, "the maximum-delta timing proof requires a planned prefix");
        // Request far beyond what a single tick could have planned.
        let (steps, max_step) = set_fill_count_and_finish(&mut app, PUZZLE3D_FILL_COUNT_MAX, None).await;
        assert!(steps <= available_count.div_ceil(set_fill_count::MAX_PLACEMENTS_PER_STEP as u32) as usize, "a maximum slider request must use only fixed-size continuation chunks");
        assert!(max_step < std::time::Duration::from_millis(8), "maximum-delta fill materialization measured {max_step:?}; every continuation must remain below 8 ms");
        let measures = semio_framework::io::resolve_ready(app.tool_measures());
        let tool_measures = measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
        let clamped = find_measure_slider(tool_measures, "puzzle3d-fill-count").expect("fill-count slider value");
        assert!(clamped <= available_count as f64, "runtime.fill_count must clamp to what's actually planned, not the raw request");
        assert_eq!(clamped as usize, object_count(&app) - object_count_before, "the clamped measure value must match what the document actually materialized");
        let tick = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick after an above-ready request");
        assert!(
            !tick.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "setFillCount")),
            "fillBuildTick must never self-dispatch setFillCount — the clamp at commit time means fill_count can never run ahead of what's planned"
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn newer_fill_count_request_cancels_a_stale_continuation() {
        let mut app = app_with_registry();
        dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
        let ready = drive_fill_until_ready(&mut app, 4.0).await as u32;
        assert!(ready > 0, "need a planned prefix for cancellation");
        let first = dispatch(&mut app, "setFillCount", Some(&json!({ "value": ready })), None).await.expect("begin first request");
        let stale_args = first.requested_effects.into_iter().find_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == set_fill_count::STEP_ACTION_ID => args.map(|value| semio_framework::from_dsl_value::<Value>(value).expect("stale fill-count args decode")),
            _ => None,
        });
        let second = dispatch(&mut app, "setFillCount", Some(&json!({ "value": 0 })), None).await.expect("supersede first request");
        finish_fill_count(&mut app, second).await;
        let before = projection_of(&app);
        let stale = dispatch(&mut app, set_fill_count::STEP_ACTION_ID, stale_args.as_ref(), None).await.expect("stale continuation is a no-op");
        assert!(stale.mutations.is_empty() && stale.requested_effects.is_empty(), "a stale generation must not mutate or requeue");
        assert_eq!(projection_of(&app), before, "a stale continuation cannot revive a superseded fill target");
    }

    #[semio_framework_async_macros::async_test]
    async fn fill_render_reveals_the_full_available_plan_tagged_with_reveal_index() {
        // 🪣️ `render()` composes EVERY currently-planned piece (not just the committed `fill_count`),
        // each tagged `revealIndex` — the viewport applies its own live, main-thread cutoff to show or
        // hide them per drag value with zero WASM round trips. The committed cutoff is separately
        // exposed as `interactionJson.revealCutoffs["puzzle3d-fill"]`.
        let mut app = app();
        let object_count_before = object_count(&app);
        dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
        let ready = drive_fill_until_ready(&mut app, 3.0).await as usize;
        assert!(ready >= 3, "fill planning must expose at least three ready placements");
        assert_eq!(object_count(&app), object_count_before, "background planning must not mutate the document before setFillCount");

        let rendered = render_composite(&mut app);
        assert_eq!(instance_count(&rendered), object_count_before + ready, "render must already expose every planned piece, tagged for client-side reveal");
        let instances = instances_of(&rendered);
        let reveal_indices: Vec<u64> = instances.iter().skip(object_count_before).filter_map(|instance| instance.get("revealIndex").and_then(Value::as_u64)).collect();
        assert_eq!(reveal_indices.len(), ready, "every planned (not-yet-committed) instance must carry revealIndex");
        let mut sorted_indices = reveal_indices.clone();
        sorted_indices.sort_unstable();
        assert_eq!(sorted_indices, (0..ready as u64).collect::<Vec<_>>(), "revealIndex is a dense 0-based sequence matching plan order");
        // 🪣️ Untagged objects omit the `revealIndex` key entirely — a `null` would compare as `0`
        // against the host's boot cutoff and hide every ordinary object.
        let base_reveal_keys = instances.iter().take(object_count_before).filter(|instance| instance.get("revealIndex").is_some()).count();
        assert_eq!(base_reveal_keys, 0, "base (non-plan) objects never carry a revealIndex key, not even a null one");
        let interaction = interaction_of(&rendered);
        assert_eq!(interaction.pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(0), "nothing committed yet — the reveal cutoff mirrors runtime.fill_count (0)");
        assert_eq!(interaction.pointer("/fillBuild/appliedCount").and_then(Value::as_u64), Some(0));

        set_fill_count_and_finish(&mut app, ready as u32, None).await;
        let after_commit = render_composite(&mut app);
        assert_eq!(instance_count(&after_commit), object_count_before + ready, "instance count is unchanged by commit — only the cutoff (and document) advanced");
        let committed_interaction = interaction_of(&after_commit);
        assert_eq!(committed_interaction.pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(ready as u64));
        assert_eq!(committed_interaction.pointer("/fillBuild/appliedCount").and_then(Value::as_u64), Some(ready as u64));
    }

    /// 🪣️ Fill count drives the shared document + reveal cutoff — split top/perspective panes must
    /// never disagree about which planned objects are visible after a slider commit on either pane.
    #[semio_framework_async_macros::async_test]
    async fn fill_count_is_shared_across_split_panes_reveal_cutoffs_and_instances() {
        let mut app = app();
        let top = main::WINDOW_INSTANCE_TOP;
        let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
        dispatch(&mut app, "worldPointerDown", None, Some(perspective)).await.expect("register perspective");
        dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), Some(top)).await.expect("select fill tool");
        let ready = drive_fill_until_ready(&mut app, 3.0).await as u32;
        assert!(ready >= 3, "need a planned fill prefix to assert cross-pane sync");

        // Commit from the top pane only — the perspective pane must still track the same cutoff.
        let committed = ready.min(3);
        set_fill_count_and_finish(&mut app, committed as u32, Some(top)).await;

        let top_render = render_window(&mut app, top);
        let perspective_render = render_window(&mut app, perspective);
        assert_eq!(interaction_of(&top_render).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(committed as u64), "top pane reveal cutoff must track the committed fill count");
        assert_eq!(interaction_of(&perspective_render).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(committed as u64), "perspective pane must share the same reveal cutoff — fill is document-global, not per-window");
        assert_eq!(instance_count(&top_render), instance_count(&perspective_render), "both panes must emit the same instance list for the shared fill plan");

        let instance_ids = |node: &Value| -> Vec<String> { instances_of(node).iter().filter_map(|instance| instance.get("id").and_then(Value::as_str).map(str::to_string)).collect() };
        assert_eq!(instance_ids(&top_render), instance_ids(&perspective_render), "top and perspective must show the exact same object ids after a fill slider commit");

        // Sliding from the other pane must keep both panes in lockstep.
        let reduced = committed.saturating_sub(1);
        set_fill_count_and_finish(&mut app, reduced as u32, Some(perspective)).await;
        let top_after = render_window(&mut app, top);
        let perspective_after = render_window(&mut app, perspective);
        assert_eq!(interaction_of(&top_after).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(reduced as u64));
        assert_eq!(interaction_of(&perspective_after).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(reduced as u64));
        assert_eq!(instance_count(&top_after), instance_count(&perspective_after));
    }

    #[semio_framework_async_macros::async_test]
    async fn seeded_objects_omit_reveal_index_so_the_boot_cutoff_cannot_hide_them() {
        let mut app = app();
        let rendered = render_composite(&mut app);
        let instances = instances_of(&rendered);
        assert!(!instances.is_empty(), "the default fixture seeds at least one object");
        for instance in &instances {
            assert!(instance.get("revealIndex").is_none(), "seeded object {} must omit revealIndex — a null coerces to 0 and the boot cutoff would hide its mesh", instance.get("id").and_then(Value::as_str).unwrap_or("?"));
        }
        assert_eq!(interaction_of(&rendered).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(0), "the boot cutoff really is 0 — this is the value that hid every mesh while revealIndex serialized as null");
    }

    #[semio_framework_async_macros::async_test]
    async fn fill_count_measure_shows_planning_progress_while_precompute_incomplete() {
        let mut session = Puzzle3dPrecomputeSession::new();
        let scene = Puzzle3dScene { fixture: nakagin_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: fill_tool::TOOL_ID.into() };
        sync_precompute_session(&mut session, &scene);
        session.precompute_step(1);
        match fill_tool::count_measure(&scene, &session, &Puzzle3dLabels::NATIVE_EN) {
            WindowMeasure::Slider { label: Some(label), max, ready, loading, .. } => {
                assert_eq!(label, Puzzle3dLabels::NATIVE_EN.count.as_str(), "fill count label stays fixed as Count while planning");
                assert_eq!(max, PUZZLE3D_FILL_COUNT_MAX as f64, "fill slider max stays fixed while planning");
                let ready = ready.expect("planning must expose a ready extent");
                assert!(ready >= 0.0 && ready <= max, "ready extent must lie on the fixed range");
                assert_eq!(loading, Some(true), "planning must mark the measure tree leaf as loading");
            }
            other => panic!("expected a slider measure, got {other:?}"),
        }
    }
    //#endregion 🔖️Fill

    //#region 🔖️Distribution
    #[semio_framework_async_macros::async_test]
    async fn puzzle3d_normalize_kind_weight_group_redistributes_siblings_proportionally() {
        let ids = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let mut weights = HashMap::from([("a".to_string(), 0.2), ("b".to_string(), 0.3), ("c".to_string(), 0.5)]);
        weights = puzzle3d_normalize_kind_weight_group(&weights, &ids, "a", 0.5);
        let sum: f64 = ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum();
        assert!((sum - 1.0).abs() < 1e-9, "simplex must stay at 1, got {sum}");
        assert!((weights.get("a").copied().unwrap_or(0.0) - 0.5).abs() < 1e-9);
        // b:c were 0.3:0.5 — remainder 0.5 splits 0.3/0.8 and 0.5/0.8
        assert!((weights.get("b").copied().unwrap_or(0.0) - 0.5 * 0.3 / 0.8).abs() < 1e-9);
        assert!((weights.get("c").copied().unwrap_or(0.0) - 0.5 * 0.5 / 0.8).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn puzzle3d_vortex_measure_exposes_joint_weight_scaled_by_object() {
        let object_ids = vec!["Object".to_string(), "Placed".to_string()];
        let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
        let object_weights = puzzle3d_uniform_kind_weights(&object_ids);
        let vortex_weights = HashMap::from([("c-b".to_string(), 0.75), ("b-s".to_string(), 0.25)]);
        let object_weight = *object_weights.get("Object").unwrap();
        let measures = puzzle3d_joint_vortex_measures("Object", object_weight, &vortex_ids, &vortex_weights);
        match &measures[0] {
            WindowMeasure::Slider { value, max, step, disabled, .. } => {
                let expected_joint = puzzle3d_joint_vortex_weight(object_weight, 0.75);
                assert!((*value - expected_joint).abs() < 1e-9, "slider must show P(object)×P(vortex), got {value}");
                assert!((*max - object_weight).abs() < 1e-9, "joint range max is P(object)");
                assert_eq!(*step, Some(object_weight * 0.01), "step tracks 1% of P(object)");
                assert_eq!(*disabled, None);
            }
            other => panic!("expected vortex slider, got {other:?}"),
        }
        let raised = puzzle3d_normalize_kind_weight_group(&object_weights, &object_ids, "Object", 0.8);
        let raised_weight = *raised.get("Object").unwrap();
        let raised_measures = puzzle3d_joint_vortex_measures("Object", raised_weight, &vortex_ids, &vortex_weights);
        match (&measures[0], &raised_measures[0]) {
            (WindowMeasure::Slider { value: before, .. }, WindowMeasure::Slider { value: after, .. }) => {
                assert!(*after > *before, "raising P(object) must raise joint vortex percentages");
                assert!((*after - raised_weight * 0.75).abs() < 1e-9);
            }
            _ => panic!("expected vortex sliders"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn puzzle3d_distribution_lists_global_vortices_and_joints_sum_to_one() {
        let fixture = nakagin_fixture();
        let object_ids = puzzle3d_kind_ids(&fixture, "objects");
        let vortex_ids = puzzle3d_kind_ids(&fixture, "vortices");
        assert!(object_ids.len() >= 2, "default fixture needs multiple object kinds");
        assert!(vortex_ids.len() >= 2, "default fixture needs multiple vortex kinds");
        let object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
        let vortex_kind_weights = puzzle3d_uniform_kind_weights(&vortex_ids);
        let scene = Puzzle3dScene { fixture, runtime: Puzzle3dRuntime { object_kind_weights, vortex_kind_weights, ..Puzzle3dRuntime::default() }, active_utility: fill_tool::TOOL_ID.into() };
        let distribution_children = puzzle3d_distribution_children(&scene, Some(true));
        assert_eq!(distribution_children.len(), object_ids.len());
        let mut joint_sum = 0.0;
        for measure in &distribution_children {
            let WindowMeasure::Group { children, value: Some(object_weight), .. } = measure else {
                panic!("expected object-kind group");
            };
            assert_eq!(children.len(), vortex_ids.len(), "each object must list the full global vortex catalog");
            let local_sum: f64 = children
                .iter()
                .map(|child| match child {
                    WindowMeasure::Slider { value, .. } => *value,
                    _ => panic!("expected vortex slider"),
                })
                .sum();
            assert!((local_sum - object_weight).abs() < 1e-6, "under one object joints sum to P(object), not 1");
            joint_sum += local_sum;
        }
        assert!((joint_sum - 1.0).abs() < 1e-6, "all nested joint percentages across objects must sum to 1, got {joint_sum}");
    }

    #[semio_framework_async_macros::async_test]
    async fn puzzle3d_object_weight_change_scales_joint_sampling_product() {
        let object_ids = vec!["Object".to_string(), "Placed".to_string()];
        let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
        let mut object_weights = puzzle3d_uniform_kind_weights(&object_ids);
        let vortex_weights = puzzle3d_uniform_kind_weights(&vortex_ids);
        object_weights = puzzle3d_normalize_kind_weight_group(&object_weights, &object_ids, "Object", 0.6);
        let object_weight = *object_weights.get("Object").unwrap();
        let vortex_weight = *vortex_weights.get("c-b").unwrap();
        let joint_before = puzzle3d_joint_vortex_weight(0.5, vortex_weight);
        let joint_after = puzzle3d_joint_vortex_weight(object_weight, vortex_weight);
        assert!(joint_after > joint_before);
    }

    /// 🚫️ Zero object-kind weight disables every vortex slider under that kind — anything × 0 is 0.
    #[semio_framework_async_macros::async_test]
    async fn zero_object_kind_weight_disables_joint_vortex_sliders() {
        let labels = puzzle3d_labels(&Puzzle3dConfig::default());
        let session = Puzzle3dPrecomputeSession::new();
        let fixture = nakagin_fixture();
        let object_ids = puzzle3d_kind_ids(&fixture, "objects");
        assert!(!object_ids.is_empty(), "default fixture must expose object kinds");
        let zeroed_id = object_ids[0].clone();
        let mut object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
        object_kind_weights = puzzle3d_normalize_kind_weight_group(&object_kind_weights, &object_ids, &zeroed_id, 0.0);
        assert!(object_kind_weights.get(&zeroed_id).copied().unwrap_or(1.0) <= f64::EPSILON);
        let scene = Puzzle3dScene { fixture, runtime: Puzzle3dRuntime { object_kind_weights, ..Puzzle3dRuntime::default() }, active_utility: fill_tool::TOOL_ID.into() };
        let fill_measures = fill_tool::measures(&scene, &session, labels);
        let distribution_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution");
        let distribution_children = fill_measures
            .iter()
            .find_map(|measure| match measure {
                WindowMeasure::Group { id, children, .. } if id == &distribution_id => Some(children.as_slice()),
                _ => None,
            })
            .expect("fill must expose a Distribution group");
        let zeroed_group_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution-object-{zeroed_id}");
        let zeroed_group = distribution_children.iter().find(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == &zeroed_group_id)).expect("zeroed object kind must appear in distribution");
        match zeroed_group {
            WindowMeasure::Group { value: Some(value), children, .. } => {
                assert!(*value <= f64::EPSILON, "object-kind header must read 0%");
                assert!(!children.is_empty(), "object kind must still list vortex sliders");
                assert!(children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: Some(true), value, .. } if *value <= f64::EPSILON)), "every joint vortex slider under a 0% object kind must be disabled at 0%");
            }
            other => panic!("expected object-kind group, got {other:?}"),
        }
        let live_group = distribution_children.iter().find(|measure| match measure {
            WindowMeasure::Group { id, value: Some(value), .. } if id != &zeroed_group_id => *value > f64::EPSILON,
            _ => false,
        });
        if let Some(WindowMeasure::Group { children, .. }) = live_group {
            assert!(children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: None | Some(false), .. })), "joint vortex sliders under a non-zero object kind must stay enabled");
        }
    }

    /// 🎯️ Fill tool measures expose count + nested distribution tree under the Fill toggle; the Volume
    /// Brush voxel dims live in a utility-options group in the window's own measures.
    #[semio_framework_async_macros::async_test]
    async fn fill_and_brush_params_are_tagged_utility_options_not_engagement_controls() {
        let labels = puzzle3d_labels(&Puzzle3dConfig::default());
        let session = Puzzle3dPrecomputeSession::new();
        let fill_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: fill_tool::TOOL_ID.into() };
        let fill_measures = fill_tool::measures(&fill_scene, &session, labels);
        let distribution_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution");
        assert!(!fill_measures.iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "puzzle3d-play-tool-options-fill")), "fill must not wrap its options in a nested Fill group — the tool toggle already owns that row");
        assert_eq!(measure_group_tag(&fill_measures, &distribution_id), Some(None));
        let distribution_children = fill_measures
            .iter()
            .find_map(|measure| match measure {
                WindowMeasure::Group { id, children, .. } if id == &distribution_id => Some(children.as_slice()),
                _ => None,
            })
            .expect("fill must expose a Distribution group");
        assert!(!distribution_children.is_empty(), "distribution must list object-kind groups");
        assert!(distribution_children.iter().all(|measure| matches!(measure, WindowMeasure::Group { value: Some(_), on_change: Some(_), .. })), "each object-kind group must carry a header weight slider");
        assert!(
            distribution_children.iter().all(|measure| match measure {
                WindowMeasure::Group { label, value: Some(_), on_change: Some(_), .. } => !label.contains('%'),
                _ => false,
            }),
            "object-kind group labels must not embed percentages — the header slider owns the value readout"
        );
        assert!(
            distribution_children.iter().any(|measure| match measure {
                WindowMeasure::Group { children, .. } => children.iter().any(|child| matches!(child, WindowMeasure::Slider { label: Some(label), .. } if !label.contains('%'))),
                _ => false,
            }),
            "vortex joint sliders must label kinds without embedding percentages"
        );
        assert!(find_measure_toggle(&fill_measures, "puzzle3d-edit-volumes").is_none(), "fill must not carry edit-volumes toggle");
        assert!(find_measure_slider(&fill_measures, "puzzle3d-voxel-w").is_none(), "fill must not carry voxel-dimension sliders");
        assert!(find_measure_slider(&fill_measures, "puzzle3d-fill-count").is_some(), "fill-count slider always lives in the fill tool measures");
        assert!(
            !main::window_measures(&fill_scene, &session, labels).iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id.contains("fill"))),
            "fill must no longer surface in window_measures — it is a mode-level tool, not a window utility"
        );
        let volume_brush_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::volume_brush::UTILITY_ID.into() };
        let volume_brush_measures = main::window_measures(&volume_brush_scene, &session, labels);
        assert_eq!(measure_group_tag(&volume_brush_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-volume-brush")), Some(Some(utilities::volume_brush::UTILITY_ID.into())));
        assert!(find_measure_slider(&volume_brush_measures, "puzzle3d-voxel-w").is_some(), "volume brush utility exposes voxel width slider");
        let fill_engagement = main::engagement(&fill_scene, &Puzzle3dLabels::NATIVE_EN);
        assert!(fill_engagement.control.is_none() && fill_engagement.controls.is_none(), "fill engagement HUD must no longer carry the relocated controls");
        let brush_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::brush::UTILITY_ID.into() };
        assert_eq!(measure_group_tag(&main::window_measures(&brush_scene, &session, labels), &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())));
        let brush_engagement = main::engagement(&brush_scene, &Puzzle3dLabels::NATIVE_EN);
        assert!(brush_engagement.control.is_none() && brush_engagement.controls.is_none(), "brush engagement HUD must no longer carry the relocated control");
        // 🖌️ Positive case: opening a vortex's suggestions selects it and drives precompute so real
        // candidates exist — the brush Utility Options group must then surface, tagged for "brush".
        let mut app = app();
        let vortex = first_vortex_full_id(&app);
        dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
        let brush_app_measures = semio_framework::io::resolve_ready(app.window_measures());
        let window_measures = brush_app_measures.get(main::WINDOW_KIND_ID).expect("main window measures");
        assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())), "the brush Utility Options group surfaces once there are candidates to place");
    }
    //#endregion 🔖️Distribution

    //#region 🔖️UiScope
    #[semio_framework_async_macros::async_test]
    async fn fill_build_tick_is_a_view_action_with_narrow_ui_scope() {
        let definition = create_puzzle3d_app();
        let def = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == "fillBuildTick").expect("fillBuildTick declared");
        assert_eq!(def.kind, ActionKind::View, "fillBuildTick must stay a View action — it only advances background planning");
        let mut live = app();
        dispatch(&mut live, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
        let result = dispatch(&mut live, "fillBuildTick", None, None).await.expect("fillBuildTick");
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
                assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
                assert!(panel_bodies.is_empty());
                assert!(tools, "fill planning must refresh the fill-count slider range in the fill tool's measures");
                assert!(!measures);
                assert!(!engagements);
                assert!(!utilities);
                assert!(!labels);
            }
            other => panic!("expected a Partial ui_scope for fillBuildTick, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn set_fill_count_declares_narrow_ui_scope() {
        let mut app = app();
        dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
        let result = dispatch(&mut app, "setFillCount", Some(&json!({ "value": 1 })), None).await.expect("setFillCount");
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
                assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
                assert!(panel_bodies.is_empty());
                assert!(tools);
                assert!(!measures);
                assert!(!engagements);
                assert!(!utilities);
                assert!(!labels);
            }
            other => panic!("expected a Partial ui_scope for setFillCount, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn set_object_kind_weight_declares_fill_options_ui_scope() {
        let mut app = app();
        dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
        let object_ids = puzzle3d_kind_ids(&nakagin_fixture(), "objects");
        let kind_id = object_ids.first().expect("object kind");
        let result = dispatch(&mut app, "setObjectKindWeight", Some(&json!({ "kindId": kind_id, "value": 0.75 })), None).await.expect("setObjectKindWeight");
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
                assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
                assert!(panel_bodies.is_empty());
                assert!(tools);
                assert!(measures, "distribution sliders live in tool + window measures");
                assert!(!engagements);
                assert!(!utilities);
                assert!(!labels);
            }
            other => panic!("expected a Partial ui_scope for setObjectKindWeight, got {other:?}"),
        }
    }

    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `set_hover_is_a_view_action_with_no_ops_after_document_mutation`
    // and `world_pick_declares_selection_ui_scope` deleted — both dispatched the now-deleted
    // `setHover`/`worldPick` actions and asserted on the deleted `Effect::PatchWorld3dChrome`
    // push-setter effect (selection/hover are framework-owned actions now, dispatched exclusively
    // through the six reserved `interactionSelect`-family verbs; see `select_id`/`hover_id`).
    //#endregion 🔖️UiScope

    //#region 🔖️Utilities
    #[semio_framework_async_macros::async_test]
    async fn add_object_kind_honors_drop_origin() {
        let mut app = app();
        let before = object_count(&app);
        dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.5, 3.5, 0.0] })), None).await.expect("addObjectKind");
        assert_eq!(object_count(&app), before + 1);
        let projection = projection_of(&app);
        let object = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).expect("added object");
        let origin = object.get("origin").and_then(Value::as_array).expect("origin array");
        assert_eq!(origin.first().and_then(Value::as_f64), Some(2.5));
        assert_eq!(origin.get(1).and_then(Value::as_f64), Some(3.5));
        assert_eq!(origin.get(2).and_then(Value::as_f64), Some(0.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_object_kind_materializes_the_declared_kind_default() {
        // 📝️ P1 arg form: firing addObjectKind with no args must materialize the declared `objectKind`
        // default and emit the object-add operation under registry enforcement.
        let mut app = app_with_registry();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
        let before = object_count(&app);
        let result = dispatch(&mut app, "addObjectKind", None, None).await.expect("addObjectKind");
        assert!(!result.mutations.is_empty(), "addObjectKind is a Mutation that emits mutations");
        assert_eq!(object_count(&app), before + 1, "the materialized default kind adds exactly one object");
        let projection = projection_of(&app);
        let kind = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("objectKind")).and_then(Value::as_str);
        assert_eq!(kind, Some("Object"), "the declared objectKind default was materialized host-side");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_utility_emits_no_ops_and_no_history_entry() {
        // 🧰️ Switching utilities is the framework-injected View action: no document operations, no undo
        // entry, no re-emitted utility-switch effect (the command IS the direct switch).
        let mut app = app_with_registry();
        let before = projection_of(&app);
        let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), None).await.expect("switch utility");
        assert!(result.mutations.is_empty(), "utility switching never emits document operations");
        assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
        assert_eq!(projection_of(&app), before, "utility switching does not mutate the document");
    }

    #[semio_framework_async_macros::async_test]
    async fn engagement_exposes_no_utility_switch_options() {
        // 🧰️ select/brush/fill switching lives only on the framework utility bar; the engagement HUD
        // must not duplicate it as options.
        let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
        let engagement = main::engagement(&scene, &Puzzle3dLabels::NATIVE_EN);
        assert!(engagement.options.is_none(), "the puzzle3d engagement must not re-expose utility switching as options");
    }

    #[semio_framework_async_macros::async_test]
    async fn transform_engagement_does_not_block_background_deselect() {
        let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::transform::UTILITY_ID.into() };
        assert_eq!(main::engagement(&scene, &Puzzle3dLabels::NATIVE_EN).session_active, Some(false));
    }
    //#endregion 🔖️Utilities

    //#region 🔖️WorldSelection
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the app-owned `worldSelect`
    /// command is deleted — selection now goes exclusively through the framework's `interactionSelect`
    /// verb (`select_id`), which is view-only by construction (`dispatch_interaction_action` never
    /// touches `self.store`). Proves the `vortex` domain wiring reaches that same guarantee.
    #[semio_framework_async_macros::async_test]
    async fn world_select_emits_no_artifact_mutations() {
        let mut app = app_with_registry();
        let before = projection_of(&app);
        let object_id = first_object_id(&app);
        let result = select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
        assert!(result.mutations.is_empty(), "interactionSelect is framework-owned and view-only, must not diff the document");
        assert_eq!(projection_of(&app), before);
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: `selectionJson`'s `ids`
    /// used to mirror the live pick. `world_selection_json` has no `InteractionView` to draw from any
    /// more (see that function's doc comment) and always emits an empty `ids: []` now — the real
    /// selection is verified below via `VcsArtifactApp::interaction_state()` instead, the framework's
    /// own sanctioned test-visible source of truth.
    #[semio_framework_async_macros::async_test]
    async fn world_pick_keeps_instances_geometry_json_stable() {
        let mut app = app_with_registry();
        let instances_before = instances_of(&render_composite(&mut app));
        let object_id = first_object_id(&app);
        select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
        let after = render_composite(&mut app);
        assert_eq!(instances_of(&after), instances_before, "picking must never perturb instance geometry");
        assert_eq!(semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()), Some(vec![object_id]));
    }

    #[semio_framework_async_macros::async_test]
    async fn world_pick_null_clears_without_reselecting_first_object() {
        let mut app = app_with_registry();
        let object_id = first_object_id(&app);
        select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
        assert!(semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_some_and(|selection| !selection.ids.is_empty()));
        dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear");
        assert!(
            semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()),
            "clicking empty background must clear, never fall back to reselecting the first object"
        );
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove that
    /// world-picking a LOCKED object clears the selection instead of selecting it — the app-owned
    /// `worldPick` command, which could read `object.locked` before deciding, is deleted.
    /// `interactionSelect` is a framework-generic id/merge verb blind to app data, and this app's
    /// `interaction_topology` does not filter locked ids out of the `vortex` domain either (see that
    /// function's doc) — "never select a locked object" is now entirely a host click→pick
    /// translation concern, not reachable from this crate. Proves the Rust-side floor instead: the
    /// `vortex` domain has no lock awareness, so selecting a locked object's id still succeeds.
    #[semio_framework_async_macros::async_test]
    async fn world_pick_locked_object_clears_like_background() {
        let mut app = app_with_registry();
        let object_id = first_object_id(&app);
        select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
        dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "flag": "locked", "value": true })), None).await.expect("lock");
        let instances = instances_of(&render_composite(&mut app));
        assert_eq!(instances.first().and_then(|entry| entry.get("disabled")).and_then(Value::as_bool), Some(true));
        select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select locked object");
        assert_eq!(
            semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()),
            Some(vec![object_id]),
            "the vortex domain has no lock awareness — this now succeeds, the host must gate locked picks itself"
        );
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove
    /// `PUZZLE3D_VORTEX_SHOW_SELECTED` reveals markers on hover/selection (`worldHover`/`worldPick`,
    /// both deleted — selection/hover are framework-owned now). `render` has no `InteractionView` to
    /// check against (see `object_vortices_visible`'s doc comment), so `Selected` mode degrades to
    /// "never reveal" until that framework gap closes — this now proves that degraded floor instead.
    #[semio_framework_async_macros::async_test]
    async fn world_vortices_stay_hidden_in_selected_mode_pending_the_render_interaction_gap() {
        let mut app = app();
        let all_vortex_ids = vortex_full_ids(&app);
        assert!(!all_vortex_ids.is_empty(), "fixture must expose vortices");
        assert!(vortices_of(&render_composite(&mut app)).is_empty(), "Selected mode with no render-time interaction access must hide every vortex marker");
        dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow");
        assert!(!vortices_of(&render_composite(&mut app)).is_empty(), "Always mode must still reveal every vortex marker");
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `worldVortexSelect`/`worldPick`
    /// are deleted; a `vortex`-domain `DomainSelection` only ever carries one granularity at a time
    /// (see `Puzzle3dActionCtx::selected_ids`'s doc), so a `merge: "replace"` pick at a different
    /// granularity inherently replaces the whole prior selection — verified against
    /// `interaction_state()` (the render-time `selectionJson`/`vorticesJson` fields carry no live ids
    /// any more, per `world_selection_json`'s known-gap doc comment).
    #[semio_framework_async_macros::async_test]
    async fn world_pick_object_replaces_vortex_selection() {
        let mut app = app_with_registry();
        let vortex = first_vortex_full_id(&app);
        select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
        let object_id = first_object_id(&app);
        select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
        let selection = semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
        assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
        assert_eq!(selection.ids, vec![object_id]);
    }

    #[semio_framework_async_macros::async_test]
    async fn world_vortex_select_clears_object_selection() {
        let mut app = app_with_registry();
        let object_id = first_object_id(&app);
        select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
        let vortex = first_vortex_full_id(&app);
        select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
        let selection = semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
        assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_VORTEX);
        assert_eq!(selection.ids, vec![vortex]);
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove a
    /// PERSISTED "default merge mode" (`selection_mode_default`, a config field — on this crate's
    /// DELETE list) flips `worldVortexSelect`'s implicit merge between replace/invertive. The
    /// framework has no equivalent persisted concept: `interactionSelect`'s `merge` arg is supplied
    /// explicitly on every dispatch (the host decides per click — e.g. a held modifier key — never
    /// defaulted from stored state). Proves the underlying `merge: "invertive"` primitive still
    /// toggles a second target back into the selection instead.
    #[semio_framework_async_macros::async_test]
    async fn world_vortex_click_replaces_until_invertive_mode_is_selected() {
        let mut app = app_with_registry();
        let vortices = vortex_full_ids(&app);
        assert!(vortices.len() >= 2, "fixture must expose two vortices");
        select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[0]).await.expect("select first vortex");
        select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[1]).await.expect("replace with second vortex");
        let replaced = semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
        assert_eq!(replaced.ids, vec![vortices[1].clone()]);

        let targets = serde_json::to_string(&vec![InteractionTarget { granularity: PUZZLE3D_GRANULARITY_VORTEX.into(), id: vortices[0].clone() }]).unwrap_or_default();
        dispatch(&mut app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "invertive", "method": "pick" })), None).await.expect("invertive toggle");
        let invertive = semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
        assert_eq!(invertive.ids.len(), 2, "invertive merge toggles the first vortex back into the selection alongside the second");
    }
    //#endregion 🔖️WorldSelection

    //#region 🔖️Gumball
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: `gumballActive` used to
    /// require BOTH the transform utility active AND a live object selection — `render`'s
    /// `gumball_active` has no `InteractionView` to check selection against any more (see that
    /// function's doc comment) and always degrades to `false`. `transformMode`/`gumballConfig` never
    /// depended on selection (only on the active utility, per-window), so those stay meaningfully
    /// tested; the selection setup and the once-`true` gumball assertion are gone.
    #[semio_framework_async_macros::async_test]
    async fn gumball_active_only_for_transform_utilities_with_object_selection() {
        let mut app = app_with_registry();
        let object_id = first_object_id(&app);
        select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
        let idle_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID));
        assert_eq!(idle_selection.get("gumballActive").and_then(Value::as_bool), Some(false), "selection alone must not show the gumball");
        assert!(idle_selection.get("transformMode").is_none(), "non-transform utility must not emit transformMode");

        dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
        let transform_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID));
        assert_eq!(transform_selection.get("gumballActive").and_then(Value::as_bool), Some(false), "render has no InteractionView, so gumballActive can no longer track the live selection");
        assert_eq!(transform_selection.get("transformMode").and_then(Value::as_str), Some("transform"));
        assert_eq!(transform_selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
        assert_eq!(transform_selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(true));

        dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
        let brush_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID));
        assert_eq!(brush_selection.get("gumballActive").and_then(Value::as_bool), Some(false));
        assert!(brush_selection.get("transformMode").is_none());
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: `gumballActive` no
    /// longer proves anything per-window (always `false` — see the sibling test's doc comment above);
    /// `transformMode` never depended on selection, only on each window instance's own
    /// `active_utility_by_window_id`, so it stays the meaningful per-window-isolation proof here.
    #[semio_framework_async_macros::async_test]
    async fn transform_utility_is_local_to_the_window_instance_not_shared_across_split_panes() {
        let mut app = app();
        let top = main::WINDOW_INSTANCE_TOP;
        let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
        dispatch(&mut app, "worldPointerDown", None, Some(perspective)).await.expect("register perspective");
        dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(top)).await.expect("transform on top");
        let top_selection = selection_of(&render_window(&mut app, top));
        assert_eq!(top_selection.get("transformMode").and_then(Value::as_str), Some("transform"), "transform on top pane must switch that pane's own scene mode");
        let perspective_selection = selection_of(&render_window(&mut app, perspective));
        assert!(perspective_selection.get("transformMode").is_none(), "perspective pane must not inherit top pane's transform utility");
    }

    #[semio_framework_async_macros::async_test]
    async fn transform_utility_options_expose_move_and_rotate_flags() {
        let labels = puzzle3d_labels(&Puzzle3dConfig::default());
        let session = Puzzle3dPrecomputeSession::new();
        let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::transform::UTILITY_ID.into() };
        let measures = main::window_measures(&scene, &session, labels);
        assert_eq!(measure_group_tag(&measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-transform")), Some(Some(utilities::transform::UTILITY_ID.into())));
        assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-move"), Some(true));
        assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-rotate"), Some(true));
        let mut app = app();
        dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
        dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("disable rotate");
        let selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID));
        assert_eq!(selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
        assert_eq!(selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(false));
        let app_measures = semio_framework::io::resolve_ready(app.window_measures());
        let window_measures = app_measures.get(main::WINDOW_KIND_ID).expect("main window measures");
        assert_eq!(find_measure_toggle(window_measures, "puzzle3d-transform-rotate"), Some(false));
    }

    fn object_origin(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
        projection_of(app)
            .get("objects")
            .and_then(Value::as_array)
            .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
            .and_then(|object| object.get("origin").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
            .unwrap_or_default()
    }

    #[semio_framework_async_macros::async_test]
    async fn gumball_translate_drag_coalesces_into_one_edit() {
        // 🌀️ Unbracketed translate ticks still coalesce via AmendLast (compat path without transformBegin).
        let mut app = app();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
        dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
        let object_id = first_object_id(&app);
        let start = object_origin(&app, &object_id);
        for dx in [1.0, 2.0, 3.0] {
            dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id], "dx": dx, "dy": 0.0, "dz": 0.0 })), None).await.expect("drag tick");
        }
        let dragged = object_origin(&app, &object_id);
        assert!((dragged[0] - start[0] - 6.0).abs() < 1e-9, "three ticks accumulate 1+2+3 on x");
        dispatch(&mut app, "undo", None, None).await.expect("undo");
        assert_eq!(object_origin(&app, &object_id), start, "one undo restores the whole coalesced gumball drag");
    }

    #[semio_framework_async_macros::async_test]
    async fn gumball_transform_session_commits_once_on_end() {
        // 🧲️ Scratch-commit: mid-drag ticks emit ZERO operations; transformEnd commits ONE edit from
        // base→scratch. Incremental host deltas accumulate on scratch — 1 then 5 → final +6.
        let mut app = app_with_registry();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
        dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
        let object_id = first_object_id(&app);
        dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
        select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
        let start = object_origin(&app, &object_id);
        dispatch(&mut app, "transformBegin", None, None).await.expect("begin");
        let tick_a = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("tick a");
        let tick_b = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id], "dx": 5.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("tick b");
        assert!(tick_a.mutations.is_empty() && tick_b.mutations.is_empty(), "mid-drag transform ticks emit no operations");
        assert_eq!(object_origin(&app, &object_id), start, "document stays at the drag-start pose mid-drag");
        let preview: Vec<f64> = instances_of(&render_window(&mut app, main::WINDOW_KIND_ID))
            .iter()
            .find(|instance| instance.get("id").and_then(Value::as_str) == Some(object_id.as_str()))
            .and_then(|instance| instance.get("position").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
            .unwrap_or_default();
        assert!((preview[0] - start[0] - 6.0).abs() < 1e-9, "scratch render accumulates incremental ticks");
        let end = dispatch(&mut app, "transformEnd", None, None).await.expect("end");
        assert_eq!(end.mutations.len(), 1, "the whole drag commits as exactly one operation");
        assert!((object_origin(&app, &object_id)[0] - start[0] - 6.0).abs() < 1e-9, "transformEnd lands on the accumulated total");
        dispatch(&mut app, "undo", None, None).await.expect("undo");
        assert_eq!(object_origin(&app, &object_id), start, "one undo restores the whole scratch-committed gumball drag");
        dispatch(&mut app, "transformBegin", None, None).await.expect("begin again");
        dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id], "dx": 2.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("second drag tick");
        dispatch(&mut app, "transformEnd", None, None).await.expect("second end");
        assert!((object_origin(&app, &object_id)[0] - start[0] - 2.0).abs() < 1e-9, "a second gumball drag session works from the restored base");
    }
    //#endregion 🔖️Gumball

    //#region 🔖️GesturePreview
    /// 🔬️ CW7 preview-law seam: `Puzzle3dPlayApp::gesture_preview` reads `transform_base`/
    /// `transform_scratch` only, never a `Puzzle3dMutation` — exercised directly against
    /// `Puzzle3dPlayApp` (bypassing the `VcsArtifactApp` wrapper, which has no accessor into the
    /// inner app) since `transform_drag_tick` is the natural per-tick gesture handler.
    #[semio_framework_async_macros::async_test]
    async fn gesture_preview_is_none_without_an_active_transform_drag() {
        let app = Puzzle3dPlayApp::default();
        assert!(app.gesture_preview().is_none(), "no live gumball drag, nothing to preview");
    }

    #[semio_framework_async_macros::async_test]
    async fn gesture_preview_reflects_the_live_gumball_drag_and_clears_on_commit() {
        let app = Puzzle3dPlayApp::default();
        let fixture = default_fixture();
        let object_id = fixture.objects[0].id.clone();
        let projection = serde_json::to_value(&fixture).unwrap();
        *app.transform_drag_active.borrow_mut() = true;
        let no_volumes: Vec<String> = Vec::new();

        let tick_a = app.transform_drag_tick("translateSelection", Some(&json!({ "ids": [object_id.clone()], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), &projection, &[object_id.clone()], &no_volumes);
        assert!(tick_a.artifact_mutations.is_empty(), "mid-drag ticks emit zero operations (scratch-commit pattern)");
        let (key, seq_after_a, payload_a) = app.gesture_preview().expect("a live gumball drag is previewable");
        assert_eq!(key, "gesture:transform");
        let value_a: Value = serde_json::from_slice(&payload_a).expect("payload is valid json");
        assert!(!value_a["operations"].as_array().expect("operations array").is_empty(), "the delta anchored to the drag-start snapshot must reflect the first tick");

        let tick_b = app.transform_drag_tick("translateSelection", Some(&json!({ "ids": [object_id.clone()], "dx": 5.0, "dy": 0.0, "dz": 0.0 })), &projection, &[object_id.clone()], &no_volumes);
        assert!(tick_b.artifact_mutations.is_empty());
        let (_, seq_after_b, payload_b) = app.gesture_preview().expect("still live mid-drag");
        assert!(seq_after_b > seq_after_a, "seq is monotone per tick, for staleness detection on the receiving end");
        assert_ne!(payload_a, payload_b, "the base-anchored delta accumulates both ticks, not just the latest one");

        let end = app.commit_transform(&projection, &[object_id]);
        assert_eq!(end.artifact_mutations.len(), 1, "the whole drag commits as exactly one real operation");
        assert!(app.gesture_preview().is_none(), "the drag ended: nothing left to preview, and the commit above already carried the real operation");
    }

    #[semio_framework_async_macros::async_test]
    async fn gesture_preview_is_a_pure_read_never_mutating_the_transform_scratch() {
        let app = Puzzle3dPlayApp::default();
        let fixture = default_fixture();
        let object_id = fixture.objects[0].id.clone();
        let projection = serde_json::to_value(&fixture).unwrap();
        *app.transform_drag_active.borrow_mut() = true;
        app.transform_drag_tick("translateSelection", Some(&json!({ "ids": [object_id.clone()], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), &projection, &[object_id], &[]);
        let scratch_before = app.transform_scratch.borrow().clone();
        let _ = app.gesture_preview();
        let _ = app.gesture_preview();
        assert_eq!(*app.transform_scratch.borrow(), scratch_before, "gesture_preview must never mutate the live transform scratch it reads");
    }
    //#endregion 🔖️GesturePreview

    //#region 🔖️KitInPort
    /// 🔌️ The flagship `kit:in` seam: feeding a `kit.catalog` fragment shaped exactly like block3d's
    /// `puzzle3d_catalog_fragment` (`objectKinds`/`vortexKinds`, camelCase) through
    /// `Puzzle3dPlayApp::import_media` must normalize `objectKinds` → `objects` / `vortexKinds` →
    /// `vortices` and, after applying the returned operations, land that object kind inside
    /// `meta.kind_catalogs.objects` (and the vortex kind inside `.vortices`).
    #[semio_framework_async_macros::async_test]
    async fn kit_in_import_media_upserts_object_and_vortex_kinds_into_meta_kind_catalogs() {
        let app = Puzzle3dPlayApp::default();
        let projection = semio_framework::io::resolve_ready(Puzzle3dPlayApp::initial_snapshot());
        let history = semio_framework::io::resolve_ready(semio_framework_plugin::HistoryView::empty());
        let doc = semio_framework::io::resolve_ready(ArtifactView::new(&projection, &history));

        let fragment = json!({
            "schema": "manifest",
            "objectKinds": [{
                "id": "capsule",
                "name": "capsule",
                "label": "Capsule",
                "meshUrl": "/mesh/capsule.glb",
                "vortices": [{ "id": "v0", "vortexKind": "door", "position": [0.0, 0.0, 0.0], "direction": [0.0, 1.0, 0.0], "radius": 0.3 }],
            }],
            "vortexKinds": [{ "id": "door", "name": "door", "label": "Door", "color": "#ff0000", "defaultCableKind": "" }],
            "cableKinds": [],
            "attractionKinds": [],
            "kindCompatibility": [{ "source": "door", "target": "door", "bidirectional": true }],
        });
        let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

        let emit = semio_framework::io::resolve_ready(Puzzle3dPlayApp::import_media("kit:in", &media, &doc)).expect("kit:in import_media succeeds");
        assert!(!emit.artifact_mutations.is_empty(), "importing a non-empty fragment must emit real operations");

        let mut next_projection = projection.value().clone();
        for operation in &emit.artifact_mutations {
            next_projection = protocol::Mutation::<Value>::diff(operation, &next_projection).diff().apply(&next_projection).expect("valid mutation diff");
        }

        let objects = next_projection.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("objects catalog present");
        assert!(objects.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")), "the imported object kind must appear in meta.kind_catalogs.objects");
        let capsule = objects.iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).unwrap();
        assert_eq!(capsule.pointer("/representations/0/url").and_then(Value::as_str), Some("/mesh/capsule.glb"));
        assert_eq!(capsule.pointer("/vortices/0/vortexKind").and_then(Value::as_str), Some("door"), "the per-object vortex template keeps its vortexKind after normalization");

        let vortices = next_projection.pointer("/meta/kindCatalogs/vortices").and_then(Value::as_array).expect("vortices catalog present");
        assert!(vortices.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some("door")), "the imported vortex kind must appear in meta.kind_catalogs.vortices");

        let compatibility = next_projection.pointer("/meta/kindCompatibility").and_then(Value::as_array).expect("kind compatibility present");
        assert!(compatibility.iter().any(|entry| entry.get("source").and_then(Value::as_str) == Some("door") && entry.get("target").and_then(Value::as_str) == Some("door")));
    }

    /// 🔁️ Re-importing the SAME fragment (a second producer edge, or a redelivered message on a
    /// `multiplicity: Many` port) must upsert idempotently — no duplicate rows.
    #[semio_framework_async_macros::async_test]
    async fn kit_in_import_media_is_idempotent_on_repeated_delivery() {
        let app = Puzzle3dPlayApp::default();
        let projection = semio_framework::io::resolve_ready(Puzzle3dPlayApp::initial_snapshot());
        let history = semio_framework::io::resolve_ready(semio_framework_plugin::HistoryView::empty());
        let mut current = projection.value().clone();

        let fragment = json!({
            "objectKinds": [{ "id": "capsule", "name": "capsule", "label": "Capsule", "meshUrl": "/mesh/capsule.glb", "vortices": [] }],
            "vortexKinds": [],
            "cableKinds": [],
            "attractionKinds": [],
            "kindCompatibility": [],
        });
        let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

        for _ in 0..2 {
            let doc_projection = Puzzle3dPlaySnapshot::new(current.clone());
            let doc = semio_framework::io::resolve_ready(ArtifactView::new(&doc_projection, &history));
            let emit = semio_framework::io::resolve_ready(Puzzle3dPlayApp::import_media("kit:in", &media, &doc)).expect("kit:in import_media succeeds");
            for operation in &emit.artifact_mutations {
                current = protocol::Mutation::<Value>::diff(operation, &current).diff().apply(&current).expect("valid mutation diff");
            }
        }

        let objects = current.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("objects catalog present");
        assert_eq!(objects.iter().filter(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).count(), 1, "repeated delivery of the same fragment must upsert, never duplicate");
    }

    #[semio_framework_async_macros::async_test]
    async fn kit_in_port_is_declared_on_the_app_io() {
        let app = Puzzle3dPlayApp::default();
        let io = semio_framework::io::resolve_ready(Puzzle3dPlayApp::io()).expect("puzzle3d declares an AppIo");
        let port = io.ports.iter().find(|port| port.id == "kit:in").expect("kit:in port declared");
        assert_eq!(port.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(port.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
        assert!(matches!(port.multiplicity, PortMultiplicity::Many));
    }
    //#endregion 🔖️KitInPort

    //#region 🔖️Convergence
    /// 🧪️ Definitional convergence proof: two instances on one backbone make DISJOINT object edits and,
    /// after exchanging operations, both converge to contain BOTH objects — impossible under
    /// whole-document `setSnapshot` snapshots, which would clobber one side.
    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_disjoint_object_edits_via_backbone() {
        use store::MemoryBackbone;
        let mut instance_a = app();
        let mut instance_b = app();
        let seeded = object_count(&instance_a);
        let (backbone_a, backbone_b) = semio_framework::io::resolve_ready(MemoryBackbone::pair("mem://puzzle3d-convergence", "mem://puzzle3d-convergence"));
        semio_framework::io::resolve_ready(instance_a.attach_backbone(store::Backbones::Memory(backbone_a))).expect("attach a");
        semio_framework::io::resolve_ready(instance_b.attach_backbone(store::Backbones::Memory(backbone_b))).expect("attach b");

        dispatch(&mut instance_a, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("a adds object");
        dispatch(&mut instance_b, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("b adds object");

        // A neutral history action always calls store.dispatch(), which pumps inbound operations first.
        dispatch(&mut instance_a, "commitCheckpoint", None, None).await.expect("pump a");
        dispatch(&mut instance_b, "commitCheckpoint", None, None).await.expect("pump b");

        assert_eq!(object_count(&instance_a), seeded + 2, "instance A must contain both objects");
        assert_eq!(object_count(&instance_b), seeded + 2, "instance B must contain both objects");
        let _ = Puzzle3dCamera::default();
    }

    //#endregion 🔖️Convergence
}
//#endregion 🧪️Tests
