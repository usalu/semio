//! 👯️ Puzzle 5d play app — the plugin's unified 2d+3d play app: its `DocumentApp` impl
//! (dispatch-only), the structural-twin document model its command/panel/window nodes mutate and
//! render, the shared scene/engine/brush helpers those nodes reach for, and the manifest that
//! stitches them together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️component.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/{◻2d,🧊️3d}`. This file dispatches and stitches.
//!
//! 🌉️ `DocumentApp::Projection` is the `Puzzle5dPlayProjection` newtype over a bare
//! `serde_json::Value` document (see `crate::artifacts::puzzle5d::op`'s `🔖️ValueBridge`), not the
//! typed `Puzzle5dProjection` — the `Puzzle5dDocument` model below is this app's own structural twin
//! of it, and each action emits the granular typed operation delta
//! (`puzzle5d_operations_from_document_change`) turning the old document into the new one.

use crate::apps::puzzle5d::commands::{board, brush, camera, engagement, example, fill, grid, hover, lod, part, patch, selection as selection_commands, sun, transform, utility};
use crate::apps::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dConfig, Puzzle5dConfigOperation, Puzzle5dRuntime, Puzzle5dSelection};
use crate::apps::puzzle5d::modes::edit;
use crate::apps::puzzle5d::modes::edit::windows::{board2d, world3d};
use crate::apps::puzzle5d::panels::{catalogue, document as document_panel, inspection};
use crate::apps::puzzle5d::terminology::{puzzle5d_is_de_locale, puzzle5d_labels, puzzle5d_localized, Puzzle5dLabels};
use crate::artifacts::puzzle5d::engine::{BrushPlacePayload, Puzzle5dPrecomputeSession};
use crate::artifacts::puzzle5d::op::{puzzle5d_document_delta_operations, Puzzle5dOperation, Puzzle5dPlayProjection};
use crate::artifacts::puzzle5d::Puzzle5dProjection;
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, HostEffect, PasteAnchor, PastePlacement};
use semio_framework_plugin::{NoDraft, NoDraftOperation, DraftView, 
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppIo, ArtifactPresentation, ConfigView, DocumentApp, DocumentView, Emit, Fault, IconName, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm,
    MediaPortDirection, MediaPortSpec, MediaType, PortMultiplicity, SelectionSet, UiNode, UiTreeItemNode, WindowEngagement, WindowMeasure, SET_ACTIVE_UTILITY_ACTION_ID,
};
use store::EngineHandles;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

//#region 🔖️Constants
pub const PUZZLE5D_PLAY_APP_ID: &str = "puzzle5d-play";
pub const PUZZLE5D_PLAY_CONTROLLER_ID: &str = "puzzle5d-play";
pub const PUZZLE5D_PLAY_WINDOWS: [&str; 2] = [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID];
pub const PUZZLE5D_SCHEMA: &str = "puzzle.5d";
pub const PUZZLE5D_BOARD_FIXTURE_SCHEMA: &str = "puzzle.2d.fixture";
pub const PUZZLE5D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";
pub const PUZZLE5D_EXAMPLE_NAKAGIN: &str = "nakagin-capsule-tower";

pub const PUZZLE5D_FALLBACK_MESH_KIND: &str = "box";
/// 🧰️ Host-owned active utility (`Puzzle5dConfig::active_utility_by_window_id`) when the host hasn't set one yet — the first declared utility.
pub const PUZZLE5D_DEFAULT_UTILITY: &str = "select";
pub const PUZZLE5D_FILL_COUNT_MAX: u32 = 1000;
pub const PUZZLE5D_LOD_MODE_AUTOMATIC: &str = "automatic";
pub const PUZZLE5D_SUGGESTION_OFFSET_MIN: f64 = 0.0;
pub const PUZZLE5D_SUGGESTION_OFFSET_MAX: f64 = 160.0;
pub const PUZZLE5D_SUGGESTION_OFFSET_STEP: f64 = 4.0;
pub const PUZZLE5D_DEFAULT_SUGGESTION_OFFSET: f64 = 80.0;
pub const PUZZLE5D_DEFAULT_PART_RADIUS: f64 = 20.0;
pub const PUZZLE5D_BOARD_PLACEMENT_GAP: f64 = 16.0;
pub const PUZZLE5D_PROXIMITY_RADIUS: f64 = 0.75;

/// 🌉️ This app's own scratch fixture stays a local structural-twin mirror (`Puzzle5dDocument`) of
/// `crate::artifacts::puzzle5d::Puzzle5dProjection` — see that artifact's `🔖️ValueBridge` region — so
/// the DSL-text example fixtures are parsed once into the typed projection and re-serialized to the
/// JSON string this module's `document_from_json`/`.example(...)` call sites expect.
fn concrete_forest_example_json() -> String { parse_example_dsl(crate::artifacts::puzzle5d::dsl::PUZZLE5D_CONCRETE_FOREST_EXAMPLE_TEXT, "concrete-forest") }
fn nakagin_example_json() -> String { parse_example_dsl(crate::artifacts::puzzle5d::dsl::PUZZLE5D_NAKAGIN_EXAMPLE_TEXT, "nakagin") }

fn parse_example_dsl(dsl_text: &str, label: &str) -> String {
    let projection = <Puzzle5dProjection as store::DocumentDsl>::parse_dsl(dsl_text).unwrap_or_else(|error| panic!("{label} example fixture parses as dsl: {error}"));
    serde_json::to_string(&projection).unwrap_or_else(|error| panic!("serialize {label} example fixture: {error}"))
}


pub fn puzzle5d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(PUZZLE5D_PLAY_CONTROLLER_ID).action(action, args)
}

pub fn next_part_id() -> String {
    let next = {
        let hex = blake3::hash(concat!(file!(), line!()).as_bytes()).to_hex();
        u64::from_str_radix(&hex[..8], 16).unwrap_or(1)
    };
    format!("part-{next}")
}

pub fn next_fastener_id() -> String {
    let next = {
        let hex = blake3::hash(concat!(file!(), line!()).as_bytes()).to_hex();
        u64::from_str_radix(&hex[..8], 16).unwrap_or(1)
    };
    format!("fastener-{next}")
}
//#endregion 🔖️Constants

//#region 🔖️Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dGrip2d {
    #[serde(default)]
    pub angle: f64,
    #[serde(default, rename = "gripKind")]
    pub grip_kind: String,
    #[serde(default)]
    pub radius: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dGrip3d {
    #[serde(default)]
    pub position: [f64; 3],
    #[serde(default)]
    pub direction: Option<[f64; 3]>,
    #[serde(default)]
    pub radius: f64,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dGrip {
    pub id: String,
    #[serde(default, rename = "gripKind")]
    pub grip_kind: String,
    #[serde(default, rename = "2d")]
    pub grip_2d: Puzzle5dGrip2d,
    #[serde(default, rename = "3d")]
    pub grip_3d: Puzzle5dGrip3d,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dFastener {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(default, rename = "fastenerKind", skip_serializing_if = "Option::is_none")]
    pub fastener_kind: Option<String>,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dPart2d {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub shape: String,
    #[serde(default)]
    pub radius: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default)]
    pub text: String,
    #[serde(default, rename = "iconKind", skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dPart3d {
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default, rename = "meshUrl")]
    pub mesh_url: Option<String>,
    #[serde(default)]
    pub orientation: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dPart {
    pub id: String,
    #[serde(rename = "partKind")]
    pub part_kind: String,
    #[serde(default, rename = "2d")]
    pub part_2d: Puzzle5dPart2d,
    #[serde(default, rename = "3d")]
    pub part_3d: Puzzle5dPart3d,
    #[serde(default)]
    pub grips: Vec<Puzzle5dGrip>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dDocument {
    pub schema: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub parts: Vec<Puzzle5dPart>,
    #[serde(default)]
    pub fasteners: Vec<Puzzle5dFastener>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    #[serde(default, rename = "kindCatalogs")]
    pub kind_catalogs: Option<Value>,
    #[serde(default, rename = "kindCompatibility")]
    pub kind_compatibility: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

pub fn empty_document() -> Puzzle5dDocument {
    Puzzle5dDocument { schema: PUZZLE5D_SCHEMA.into(), domain: "architecture".into(), parts: Vec::new(), fasteners: Vec::new(), meta: None, kind_catalogs: None, kind_compatibility: None, label: None }
}

pub fn document_from_json(json_text: &str) -> Puzzle5dDocument {
    serde_json::from_str::<Puzzle5dDocument>(json_text).unwrap_or_else(|_| empty_document())
}

pub fn default_document() -> Puzzle5dDocument {
    document_from_json(concrete_forest_example_json().as_str())
}

/// 🧮️ Document ops for a document mutation — normalizes `before` through the same typed
/// round-trip as `after` so View-kind actions that only touch runtime never trip the
/// "must not emit operations" guard when the live store still holds a typed-projection-shaped
/// projection from a prior op apply.
pub fn puzzle5d_operations_from_document_change(before: &Value, after_document: &Puzzle5dDocument) -> Vec<Puzzle5dOperation> {
    let before_normalized = serde_json::to_value(serde_json::from_value::<Puzzle5dDocument>(before.clone()).unwrap_or_else(|_| empty_document())).unwrap_or_else(|_| before.clone());
    let after = serde_json::to_value(after_document).unwrap_or_else(|_| before_normalized.clone());
    puzzle5d_document_delta_operations(&before_normalized, &after)
}

/// 🪟️ B1: puzzle5d has exactly two window KINDS (2D and 3D), each single-instance — unlike puzzle3d's
/// split top/perspective panes (two INSTANCES of one kind), puzzle5d's own dispatch never distinguishes
/// a window instance id from its kind id (every action matches the literal kind id via
/// `PUZZLE5D_PLAY_WINDOWS.contains(&window)`), so this needs none of `Puzzle3dConfig`'s self-maintained
/// `window_ids`/`load_window`/`save_window` machinery — each kind's sole instance id is the kind id
/// itself. Kept as a named helper (rather than inlining `vec![kind_id.to_string()]`) purely so
/// `window_engagements`/`window_measures` read the same "one entry per live window instance" shape
/// `DocumentApp`'s doc comment describes, and so a future genuine multi-instance need has one seam to extend.
pub fn window_instance_ids(kind_id: &str) -> Vec<String> {
    vec![kind_id.to_string()]
}

pub fn puzzle5d_grip_full_id(part_id: &str, grip_id: &str) -> String {
    if grip_id.contains(':') {
        grip_id.to_string()
    } else {
        format!("{part_id}:{grip_id}")
    }
}

/// 📐️ Resolves one numeric-field edit: an absolute `value` (typed entry) wins when present,
/// otherwise a `delta` (stepper nudge) is added to `current`. `None` when neither parses.
pub fn puzzle5d_resolve_number_edit(current: f64, value: Option<&Value>, delta: Option<&Value>) -> Option<f64> {
    if let Some(absolute) = value.and_then(Value::as_f64) {
        return Some(absolute);
    }
    delta.and_then(Value::as_f64).map(|delta| current + delta)
}

/// 📐️ Parses a nested stepper-group field id as `"<base>.<axis>"` (`x`/`y`/`z`), returning the axis
/// index when `field` names a component of `base` — the dot-path convention `ui_inspector_vec3_group`
/// uses for its per-axis actions.
pub fn puzzle5d_axis_index(field: &str, base: &str) -> Option<usize> {
    match field.strip_prefix(base)?.strip_prefix('.')? {
        "x" => Some(0),
        "y" => Some(1),
        "z" => Some(2),
        _ => None,
    }
}

pub fn resolve_part_mesh_url(part: &Puzzle5dPart, kind_catalogs: Option<&Value>) -> Option<String> {
    if let Some(url) = part.part_3d.mesh_url.as_ref().filter(|url| !url.is_empty()) {
        return Some(url.clone());
    }
    resolve_part_kind_mesh_url(&part.part_kind, kind_catalogs)
}

pub fn resolve_part_kind_mesh_url(part_kind: &str, kind_catalogs: Option<&Value>) -> Option<String> {
    let parts = kind_catalogs?.get("parts")?.as_array()?;
    parts.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(part_kind)).and_then(|entry| entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string))
}

pub fn collect_mesh_urls(document: &Puzzle5dDocument) -> Vec<String> {
    let mut urls = HashSet::new();
    for part in &document.parts {
        if let Some(url) = resolve_part_mesh_url(part, document.kind_catalogs.as_ref()) {
            urls.insert(url);
        }
    }
    if let Some(parts) = document.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("parts")).and_then(|v| v.as_array()) {
        for entry in parts {
            if let Some(url) = entry.get("meshUrl").and_then(|v| v.as_str()) {
                urls.insert(url.to_string());
            }
        }
    }
    urls.into_iter().collect()
}

fn part_kind_grip_templates(document: &Puzzle5dDocument, part_kind: &str) -> Vec<Value> {
    document
        .kind_catalogs
        .as_ref()
        .and_then(|catalogs| catalogs.get("parts"))
        .and_then(|parts| parts.as_array())
        .and_then(|parts| parts.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(part_kind)))
        .and_then(|entry| entry.get("grips"))
        .and_then(|grips| grips.as_array())
        .cloned()
        .unwrap_or_default()
}

pub fn grips_from_templates(document: &Puzzle5dDocument, part_kind: &str) -> Vec<Puzzle5dGrip> {
    part_kind_grip_templates(document, part_kind)
        .iter()
        .enumerate()
        .map(|(index, template)| {
            let grip_kind = template.get("gripKind").and_then(|v| v.as_str()).unwrap_or("grip").to_string();
            let grip_2d: Puzzle5dGrip2d = template.get("2d").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
            let grip_3d: Puzzle5dGrip3d = template.get("3d").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
            Puzzle5dGrip { id: format!("v{index}"), grip_kind, grip_2d, grip_3d }
        })
        .collect()
}

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

pub fn world_grip_position(part: &Puzzle5dPart, grip: &Puzzle5dGrip) -> [f64; 3] {
    let orientation = part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let rotated = quat_rotate_vector(orientation, grip.grip_3d.position);
    [part.part_3d.origin[0] + rotated[0], part.part_3d.origin[1] + rotated[1], part.part_3d.origin[2] + rotated[2]]
}

pub fn world_grip_direction(part: &Puzzle5dPart, grip: &Puzzle5dGrip) -> [f64; 3] {
    let direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
    quat_rotate_vector(part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), direction)
}

pub fn resolve_grip_world_position(document: &Puzzle5dDocument, full_id: &str) -> Option<[f64; 3]> {
    for part in &document.parts {
        for grip in &part.grips {
            if puzzle5d_grip_full_id(&part.id, &grip.id) == full_id {
                return Some(world_grip_position(part, grip));
            }
        }
    }
    None
}

pub fn find_part_by_grip_full_id<'a>(document: &'a Puzzle5dDocument, full_id: &str) -> Option<(&'a Puzzle5dPart, &'a Puzzle5dGrip)> {
    for part in &document.parts {
        for grip in &part.grips {
            if puzzle5d_grip_full_id(&part.id, &grip.id) == full_id {
                return Some((part, grip));
            }
        }
    }
    None
}

fn strip_tree_prefix(id: &str) -> &str {
    for prefix in ["puzzle5d-play-document.part.", "puzzle5d-play-document.grip.", "puzzle5d-play-document.fastener."] {
        if let Some(rest) = id.strip_prefix(prefix) {
            return rest;
        }
    }
    id
}

pub fn classify_selection(document: &Puzzle5dDocument, ids: &[String]) -> Puzzle5dSelection {
    let part_ids: HashSet<&str> = document.parts.iter().map(|part| part.id.as_str()).collect();
    let fastener_ids: HashSet<&str> = document.fasteners.iter().map(|fastener| fastener.id.as_str()).collect();
    let grip_ids: HashSet<String> = document.parts.iter().flat_map(|part| part.grips.iter().map(|grip| puzzle5d_grip_full_id(&part.id, &grip.id))).collect();
    let mut selection = Puzzle5dSelection::default();
    for raw in ids {
        let id = strip_tree_prefix(raw);
        if part_ids.contains(id) {
            selection.part_ids.push_unique(id.to_string());
        } else if fastener_ids.contains(id) {
            selection.fastener_ids.push_unique(id.to_string());
        } else if grip_ids.contains(id) {
            selection.grip_ids.push_unique(id.to_string());
        }
    }
    selection
}

pub fn selection_flat_ids(selection: &Puzzle5dSelection) -> Vec<String> {
    selection.part_ids.iter().chain(selection.grip_ids.iter()).chain(selection.fastener_ids.iter()).cloned().collect()
}

pub fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()).filter(|ids| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
}

pub fn remove_parts(document: &mut Puzzle5dDocument, part_ids: &[String]) {
    let removed_grips: Vec<String> = document.parts.iter().filter(|part| part_ids.contains(&part.id)).flat_map(|part| part.grips.iter().map(|grip| puzzle5d_grip_full_id(&part.id, &grip.id))).collect();
    document.parts.retain(|part| !part_ids.contains(&part.id));
    document.fasteners.retain(|fastener| !removed_grips.contains(&fastener.source) && !removed_grips.contains(&fastener.target));
}

pub fn remove_grips(document: &mut Puzzle5dDocument, grip_full_ids: &[String]) {
    if grip_full_ids.is_empty() {
        return;
    }
    for part in &mut document.parts {
        let part_id = part.id.clone();
        part.grips.retain(|grip| !grip_full_ids.contains(&puzzle5d_grip_full_id(&part_id, &grip.id)));
    }
    document.fasteners.retain(|fastener| !grip_full_ids.contains(&fastener.source) && !grip_full_ids.contains(&fastener.target));
}

pub fn set_part_2d_position(document: &mut Puzzle5dDocument, part_id: &str, x: Option<f64>, y: Option<f64>) {
    if let Some(part) = document.parts.iter_mut().find(|part| part.id == part_id) {
        if let Some(x) = x {
            part.part_2d.x = x;
        }
        if let Some(y) = y {
            part.part_2d.y = y;
        }
    }
}

pub fn part_scale_json(part: &Puzzle5dPart) -> [f64; 3] {
    match &part.part_3d.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        Some(Value::Number(value)) => {
            let factor = value.as_f64().unwrap_or(1.0);
            [factor, factor, factor]
        }
        _ => [1.0, 1.0, 1.0],
    }
}

/// 🎨️ Palette drop: creates a free paired part at the flat drop point, deriving the volume origin from the nearest peer part's offset.
pub fn add_palette_part(envelope: &mut Puzzle5dScene, part_kind: &str, x: f64, y: f64) {
    let flat_to_world = 1.0 / 48.0;
    let origin = envelope.document.parts.first().map_or([x * flat_to_world, -y * flat_to_world, 0.0], |peer| {
        [peer.part_3d.origin[0] + (x - peer.part_2d.x) * flat_to_world, peer.part_3d.origin[1] - (y - peer.part_2d.y) * flat_to_world, peer.part_3d.origin[2]]
    });
    let id = next_part_id();
    let mesh_url = resolve_part_kind_mesh_url(part_kind, envelope.document.kind_catalogs.as_ref());
    let grips = grips_from_templates(&envelope.document, part_kind);
    envelope.document.parts.push(Puzzle5dPart {
        id: id.clone(),
        part_kind: part_kind.into(),
        part_2d: Puzzle5dPart2d { x, y, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: part_kind.into(), icon_kind: None, hidden: None, locked: None },
        part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
        grips,
    });
    envelope.runtime.selection = Puzzle5dSelection { part_ids: SelectionSet::from_ids(vec![id]), grip_ids: SelectionSet::default(), fastener_ids: SelectionSet::default() };
}
//#endregion 🔖️Document

//#region 🔖️Scene
/// 🧾️ Transient render/mutation bundle pairing the persisted projection (the bare `Puzzle5dDocument`
/// json) with the app's view state. Never persisted — the `VcsDocumentApp` store owns the document
/// and the wrapping store owns the VCS-tracked `Puzzle5dConfig` — but rebuilt per call so the
/// board/world/engagement helpers keep their `&scene` signatures.
#[derive(Clone)]
pub struct Puzzle5dScene {
    pub document: Puzzle5dDocument,
    pub runtime: Puzzle5dRuntime,
    /// 🧰️ The active utility for this window — transient, never persisted.
    pub active_utility: String,
}

/// 🧾️ Materializes the transient scene from the persisted projection (bare document json) and the
/// app's current view state; an unparseable projection degrades to an empty document.
pub fn scene_from_projection(projection: &Value, runtime: Puzzle5dRuntime, active_utility: &str) -> Puzzle5dScene {
    let document = serde_json::from_value::<Puzzle5dDocument>(projection.clone()).unwrap_or_else(|_| empty_document());
    Puzzle5dScene { document, runtime, active_utility: active_utility.to_string() }
}

/// 🧰️ B1: the active utility for `window_id`, from `Puzzle5dConfig::active_utility_by_window_id` — falls
/// back to [`PUZZLE5D_DEFAULT_UTILITY`] when the window has never had a utility switch recorded yet.
pub fn puzzle5d_scene_active_utility(config: &Puzzle5dConfig, window_id: Option<&str>) -> String {
    if let Some(wid) = window_id {
        if let Some(utility) = config.active_utility_by_window_id.get(wid) {
            return utility.clone();
        }
    }
    PUZZLE5D_DEFAULT_UTILITY.to_string()
}

/// 🧭️ The select/brush/fill interaction mode the world engine reads, derived from the flat active utility
/// (the transform gumball utilities `move`/`rotate`/`scale` and `worldRelocate` all present as `select`).
pub fn puzzle5d_scene_mode(active_utility: &str) -> &str {
    match active_utility {
        "brush" => "brush",
        "fill" => "fill",
        _ => "select",
    }
}

/// 🎚️ The gumball handle the world engine draws when a transform utility is active.
pub fn puzzle5d_transform_handle(active_utility: &str) -> Option<&'static str> {
    match active_utility {
        "move" => Some("move"),
        "rotate" => Some("rotate"),
        "scale" => Some("scale"),
        _ => None,
    }
}

/// 🧭️ Whether the active utility is a transform gumball mode.
pub fn puzzle5d_transform_utility_active(active_utility: &str) -> bool {
    puzzle5d_transform_handle(active_utility).is_some()
}

/// 🕹️ Whether the world gumball should render for the current selection and utility.
pub fn puzzle5d_gumball_active(runtime: &Puzzle5dRuntime, active_utility: &str) -> bool {
    !runtime.selection.part_ids.is_empty() && puzzle5d_transform_utility_active(active_utility)
}

pub fn gumball_target_world(envelope: &Puzzle5dScene) -> Option<[f64; 3]> {
    let selected: Vec<&Puzzle5dPart> = envelope.document.parts.iter().filter(|part| envelope.runtime.selection.part_ids.contains(&part.id)).collect();
    if selected.is_empty() {
        return None;
    }
    let mut sum = [0.0, 0.0, 0.0];
    for part in &selected {
        sum[0] += part.part_3d.origin[0];
        sum[1] += part.part_3d.origin[1];
        sum[2] += part.part_3d.origin[2];
    }
    let count = selected.len() as f64;
    Some([sum[0] / count, sum[1] / count, sum[2] / count])
}
//#endregion 🔖️Scene

//#region 🔖️Engine
/// 🧠️ Maps the unified 5d kind bundle to the puzzle 3d engine naming (`objects` with `vortices` templates, `vortices`, `cables`).
fn engine_kind_catalogs_value(document: &Puzzle5dDocument) -> Option<Value> {
    let catalogs = document.kind_catalogs.as_ref()?;
    let objects: Vec<Value> = catalogs
        .get("parts")
        .and_then(|parts| parts.as_array())
        .into_iter()
        .flatten()
        .map(|entry| {
            let mut object = entry.clone();
            let vortices: Vec<Value> = entry
                .get("grips")
                .and_then(|grips| grips.as_array())
                .into_iter()
                .flatten()
                .map(|template| {
                    let volume = template.get("3d").cloned().unwrap_or(json!({}));
                    json!({
                        "vortexKind": template.get("gripKind").cloned().unwrap_or(json!("grip")),
                        "position": volume.get("position").cloned().unwrap_or(json!([0.0, 0.0, 0.0])),
                        "direction": volume.get("direction").cloned().unwrap_or(json!([0.0, 0.0, -1.0])),
                        "radius": volume.get("radius").cloned().unwrap_or(json!(0.36)),
                    })
                })
                .collect();
            if let Some(object) = object.as_object_mut() {
                object.remove("grips");
                object.insert("vortices".into(), json!(vortices));
            }
            object
        })
        .collect();
    Some(json!({
        "objects": objects,
        "vortices": catalogs.get("grips").cloned().unwrap_or(json!([])),
        "cables": catalogs.get("ropes").cloned().unwrap_or(json!([])),
    }))
}

fn scene_config_json(envelope: &Puzzle5dScene) -> String {
    let objects: Vec<Value> = envelope
        .document
        .parts
        .iter()
        .map(|part| {
            json!({
                "id": part.id,
                "objectKind": part.part_kind,
                "meshUrl": resolve_part_mesh_url(part, envelope.document.kind_catalogs.as_ref()),
                "origin": part.part_3d.origin,
                "orientation": part.part_3d.orientation,
                "scale": part.part_3d.scale,
                "vortices": part.grips.iter().map(|grip| json!({
                    "id": grip.id,
                    "vortexKind": if grip.grip_kind.is_empty() { grip.grip_2d.grip_kind.clone() } else { grip.grip_kind.clone() },
                    "position": grip.grip_3d.position,
                    "direction": grip.grip_3d.direction,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    let attractions: Vec<Value> = envelope.document.fasteners.iter().map(|fastener| json!({ "id": fastener.id, "attracting": fastener.source, "attracted": fastener.target })).collect();
    json!({
        "fixture": {
            "objects": objects,
            "attractions": attractions,
            "targetVolumes": [],
        },
        "kindCatalogs": engine_kind_catalogs_value(&envelope.document),
        "kindCompatibility": envelope.document.kind_compatibility.clone().unwrap_or(json!([])),
        "overlapBudget": envelope.runtime.overlap_budget,
        "seed": 1,
        "hostRules": {},
        "weights": {
            "objectWeights": envelope.runtime.object_kind_weights,
            "vortexWeights": envelope.runtime.vortex_kind_weights,
        },
    })
    .to_string()
}

/// 🔄️ Adopts an engine fixture while preserving flat aspects: existing parts keep `2d`, new parts get a synthesized flat aspect.
pub fn merge_engine_fixture(envelope: &Puzzle5dScene, fixture_json: &str) -> Option<Puzzle5dScene> {
    let parsed: Value = serde_json::from_str(fixture_json).ok()?;
    let objects = parsed.get("objects")?.as_array()?;
    let mut next = envelope.clone();
    let existing: HashMap<String, Puzzle5dPart> = envelope.document.parts.iter().map(|part| (part.id.clone(), part.clone())).collect();
    let mut new_ids: Vec<String> = Vec::new();
    next.document.parts = objects
        .iter()
        .filter_map(|object| {
            let id = object.get("id")?.as_str()?.to_string();
            let part_kind = object.get("objectKind").and_then(|value| value.as_str()).unwrap_or("Part").to_string();
            let origin: [f64; 3] = object.get("origin").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]);
            let orientation: Option<[f64; 4]> = object.get("orientation").and_then(|value| serde_json::from_value(value.clone()).ok());
            let mesh_url = object.get("meshUrl").and_then(|value| value.as_str()).map(str::to_string);
            let scale = object.get("scale").cloned().filter(|value| !value.is_null());
            if let Some(previous) = existing.get(&id) {
                let mut part = previous.clone();
                part.part_kind = part_kind;
                part.part_3d.origin = origin;
                part.part_3d.orientation = orientation.or(part.part_3d.orientation);
                part.part_3d.mesh_url = mesh_url.or(part.part_3d.mesh_url.clone());
                if scale.is_some() {
                    part.part_3d.scale = scale;
                }
                return Some(part);
            }
            let templates = grips_from_templates(&envelope.document, &part_kind);
            let grips: Vec<Puzzle5dGrip> = object
                .get("vortices")
                .and_then(|value| value.as_array())
                .into_iter()
                .flatten()
                .enumerate()
                .map(|(index, vortex)| {
                    let template = templates.get(index);
                    Puzzle5dGrip {
                        id: vortex.get("id").and_then(|value| value.as_str()).map_or_else(|| format!("v{index}"), str::to_string),
                        grip_kind: vortex.get("vortexKind").and_then(|value| value.as_str()).map(str::to_string).or_else(|| template.map(|t| t.grip_kind.clone())).unwrap_or_else(|| "grip".into()),
                        grip_2d: template.map(|t| t.grip_2d.clone()).unwrap_or_default(),
                        grip_3d: Puzzle5dGrip3d {
                            position: vortex.get("position").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]),
                            direction: vortex.get("direction").and_then(|value| serde_json::from_value(value.clone()).ok()),
                            radius: vortex.get("radius").and_then(|value| value.as_f64()).unwrap_or(0.36),
                            label: vortex.get("label").and_then(|value| value.as_str()).map(str::to_string),
                        },
                    }
                })
                .collect();
            let grips = if grips.is_empty() { templates } else { grips };
            new_ids.push(id.clone());
            Some(Puzzle5dPart {
                id,
                part_kind: part_kind.clone(),
                part_2d: Puzzle5dPart2d { x: 0.0, y: 0.0, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: part_kind, icon_kind: None, hidden: None, locked: None },
                part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: orientation.or(Some([0.0, 0.0, 0.0, 1.0])), scale, label: None },
                grips,
            })
        })
        .collect();
    let existing_kinds: HashMap<String, Option<String>> = envelope.document.fasteners.iter().map(|fastener| (fastener.id.clone(), fastener.fastener_kind.clone())).collect();
    let existing_transforms: HashMap<String, (f64, f64, f64, f64, f64, f64)> =
        envelope.document.fasteners.iter().map(|fastener| (fastener.id.clone(), (fastener.gap, fastener.shift, fastener.rise, fastener.rotation, fastener.turn, fastener.tilt))).collect();
    next.document.fasteners = parsed
        .get("attractions")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|attraction| {
            let id = attraction.get("id").and_then(|value| value.as_str()).unwrap_or("fastener").to_string();
            let (gap, shift, rise, rotation, turn, tilt) = existing_transforms.get(&id).copied().unwrap_or_default();
            Some(Puzzle5dFastener {
                fastener_kind: existing_kinds.get(&id).cloned().flatten().or_else(|| attraction.get("attractionKind").and_then(|value| value.as_str()).map(str::to_string)),
                source: attraction.get("attracting")?.as_str()?.to_string(),
                target: attraction.get("attracted")?.as_str()?.to_string(),
                id,
                gap,
                shift,
                rise,
                rotation,
                turn,
                tilt,
            })
        })
        .collect();
    synthesize_flat_for_new_parts(&mut next.document, &new_ids);
    Some(next)
}

/// 🌤️ Places flat centers for freshly-adopted parts next to their fastened neighbor, walking chains until every new part is placed.
fn synthesize_flat_for_new_parts(document: &mut Puzzle5dDocument, new_ids: &[String]) {
    let mut pending: HashSet<String> = new_ids.iter().cloned().collect();
    for _ in 0..=new_ids.len() {
        if pending.is_empty() {
            break;
        }
        let mut placed: Vec<(String, f64, f64)> = Vec::new();
        for fastener in &document.fasteners {
            for (own, other) in [(&fastener.source, &fastener.target), (&fastener.target, &fastener.source)] {
                let Some((own_part, _)) = find_part_by_grip_full_id(document, own) else {
                    continue;
                };
                if !pending.contains(&own_part.id) {
                    continue;
                }
                let Some((other_part, other_grip)) = find_part_by_grip_full_id(document, other) else {
                    continue;
                };
                if pending.contains(&other_part.id) {
                    continue;
                }
                let angle = other_grip.grip_2d.angle;
                let own_radius = if own_part.part_2d.radius > 0.0 { own_part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS };
                let other_radius = if other_part.part_2d.radius > 0.0 { other_part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS };
                let distance = own_radius + other_radius + PUZZLE5D_BOARD_PLACEMENT_GAP;
                placed.push((own_part.id.clone(), other_part.part_2d.x + angle.cos() * distance, other_part.part_2d.y + angle.sin() * distance));
            }
        }
        if placed.is_empty() {
            break;
        }
        for (id, x, y) in placed {
            set_part_2d_position(document, &id, Some(x), Some(y));
            pending.remove(&id);
        }
    }
    for (column, id) in pending.into_iter().enumerate() {
        set_part_2d_position(document, &id, Some(120.0 + column as f64 * 56.0), Some(120.0));
    }
}
//#endregion 🔖️Engine

//#region 🔖️Brush
pub fn puzzle5d_brush_target_grip(envelope: &Puzzle5dScene) -> Option<String> {
    envelope.runtime.selection.grip_ids.first().map(str::to_string).or_else(|| {
        let part_id = envelope.runtime.hovered_part_id.as_deref().or_else(|| envelope.runtime.selection.part_ids.first())?;
        let part = envelope.document.parts.iter().find(|part| part.id == part_id)?;
        let grip = part.grips.first()?;
        Some(puzzle5d_grip_full_id(&part.id, &grip.id))
    })
}

pub fn parse_brush_candidates_free(raw: &str) -> Vec<Value> {
    let parsed: Value = serde_json::from_str(raw).unwrap_or(Value::Null);
    parsed.get("free").and_then(|value| value.as_array()).cloned().unwrap_or_default()
}
//#endregion 🔖️Brush

//#region 🔖️Distribution
pub fn puzzle5d_kind_ids(document: &Puzzle5dDocument, slice: &str) -> Vec<String> {
    let mut ids: Vec<String> =
        document.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get(slice)).and_then(|value| value.as_array()).into_iter().flatten().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect();
    if ids.is_empty() {
        let mut inferred: Vec<String> = match slice {
            "parts" => document.parts.iter().map(|part| part.part_kind.clone()).collect(),
            "grips" => document.parts.iter().flat_map(|part| part.grips.iter().map(|grip| grip.grip_kind.clone())).collect(),
            _ => Vec::new(),
        };
        inferred.sort();
        inferred.dedup();
        ids = inferred;
    }
    ids
}

pub fn puzzle5d_uniform_kind_weights(ids: &[String]) -> HashMap<String, f64> {
    if ids.is_empty() {
        return HashMap::new();
    }
    let weight = 1.0 / ids.len() as f64;
    ids.iter().map(|id| (id.clone(), weight)).collect()
}

pub fn puzzle5d_normalize_kind_weight_group(weights: &HashMap<String, f64>, kind_ids: &[String], changed_id: &str, new_value: f64) -> HashMap<String, f64> {
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

pub fn puzzle5d_ensure_catalog_kind_weights(weights: &mut HashMap<String, f64>, kind_ids: &[String]) {
    if kind_ids.is_empty() {
        return;
    }
    if weights.is_empty() || kind_ids.iter().any(|id| !weights.contains_key(id)) {
        *weights = puzzle5d_uniform_kind_weights(kind_ids);
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

pub fn puzzle5d_kind_weight_sum(weights: &HashMap<String, f64>, kind_ids: &[String]) -> f64 {
    kind_ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum()
}
//#endregion 🔖️Distribution

//#region 🔖️Trees
/// 📊️ `label` is always genuine runtime document content here (a part/grip/fastener/catalog name),
/// never `app_labels!` chrome text — wrapped via `Label::data` accordingly. Shared by the document
/// and catalogue panels.
pub fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, icon_id: Option<&str>, action: ActionDescriptor) -> UiTreeItemNode {
    let mut item = UiTreeItemNode::base(id, Label::data(label));
    item.icon_id = icon_id.map(IconName::from);
    item.action = Some(action);
    item
}

/// 📊️ See `tree_item_with_action`'s doc comment — same `Label::data` rationale.
pub fn tree_info_item(id: impl Into<String>, label: impl Into<String>, description: Option<String>) -> UiTreeItemNode {
    let mut item = UiTreeItemNode::base(id, Label::data(label));
    item.description = description;
    item
}
//#endregion 🔖️Trees

//#region 🔖️CopyPaste
/// 🧩️ The part id a `"part_id:grip_id"` full grip reference belongs to.
fn owning_part_id_local(grip_ref: &str) -> &str {
    grip_ref.split(':').next().unwrap_or(grip_ref)
}

fn rewrite_grip_ref_local(grip_ref: &str, id_map: &HashMap<String, String>) -> String {
    match grip_ref.split_once(':') {
        Some((part_id, grip_id)) => match id_map.get(part_id) {
            Some(fresh_part_id) => format!("{fresh_part_id}:{grip_id}"),
            None => grip_ref.to_string(),
        },
        None => grip_ref.to_string(),
    }
}

/// 🧮️ Closure-selects a copy fragment: expands the part set to include every selected fastener's
/// endpoint parts, then expands the fastener set to include every fastener whose BOTH endpoints are
/// now in the part set — the untyped structural-twin twin of
/// `crate::artifacts::puzzle5d::engine::copy_selection`.
fn copy_selection_local(document: &Puzzle5dDocument, part_ids: &[String], fastener_ids: &[String]) -> (Vec<Puzzle5dPart>, Vec<Puzzle5dFastener>) {
    let mut part_set: HashSet<String> = part_ids.iter().cloned().collect();
    for fastener in &document.fasteners {
        if fastener_ids.contains(&fastener.id) {
            part_set.insert(owning_part_id_local(&fastener.source).to_string());
            part_set.insert(owning_part_id_local(&fastener.target).to_string());
        }
    }
    let mut fastener_set: HashSet<String> = fastener_ids.iter().cloned().collect();
    if !part_set.is_empty() {
        for fastener in &document.fasteners {
            let source_part = owning_part_id_local(&fastener.source);
            let target_part = owning_part_id_local(&fastener.target);
            if part_set.contains(source_part) && part_set.contains(target_part) {
                fastener_set.insert(fastener.id.clone());
            }
        }
    }
    let parts = document.parts.iter().filter(|part| part_set.contains(&part.id)).cloned().collect();
    let fasteners = document.fasteners.iter().filter(|fastener| fastener_set.contains(&fastener.id)).cloned().collect();
    (parts, fasteners)
}

fn centroid_2d_local(parts: &[Puzzle5dPart]) -> Option<(f64, f64)> {
    if parts.is_empty() {
        return None;
    }
    let (mut sum_x, mut sum_y) = (0.0, 0.0);
    for part in parts {
        sum_x += part.part_2d.x;
        sum_y += part.part_2d.y;
    }
    let count = parts.len() as f64;
    Some((sum_x / count, sum_y / count))
}

/// 🧮️ Resolves the 2D paste offset from `placement`: `Original` uses the (optional) position
/// override verbatim; every other anchor uses the target-minus-source centroid delta plus the
/// (optional) position override — mirrors semio_compose_rs's `__pasteCoordinateOffset`
/// (`semio_compose_rs/dev/algorithm/js/index.ts:358`).
fn paste_delta_2d(fragment_parts: &[Puzzle5dPart], target_parts: &[Puzzle5dPart], placement: &PastePlacement) -> (f64, f64) {
    let (offset_x, offset_y) = placement.position.map_or((0.0, 0.0), |position| (position[0], position[1]));
    if matches!(placement.anchor, PasteAnchor::Original) {
        return (offset_x, offset_y);
    }
    match (centroid_2d_local(fragment_parts), centroid_2d_local(target_parts)) {
        (Some(source), Some(target)) => (target.0 - source.0 + offset_x, target.1 - source.1 + offset_y),
        _ => (offset_x, offset_y),
    }
}

/// 🧮️ Materializes a copied fragment at 2D delta `delta` (applied verbatim to the 3D origin's x/y
/// too) — fresh ids (via `next_part_id`/`next_fastener_id`) dodge collisions with the live document,
/// and fastener endpoints are remapped onto the fresh part ids.
fn paste_selection_local(fragment_parts: &[Puzzle5dPart], fragment_fasteners: &[Puzzle5dFastener], delta: (f64, f64)) -> (Vec<Puzzle5dPart>, Vec<Puzzle5dFastener>) {
    let mut id_map: HashMap<String, String> = HashMap::new();
    let mut fresh_parts = Vec::with_capacity(fragment_parts.len());
    for part in fragment_parts {
        let fresh_id = next_part_id();
        id_map.insert(part.id.clone(), fresh_id.clone());
        let mut next = part.clone();
        next.id = fresh_id;
        next.part_2d.x += delta.0;
        next.part_2d.y += delta.1;
        next.part_3d.origin[0] += delta.0;
        next.part_3d.origin[1] += delta.1;
        fresh_parts.push(next);
    }
    let mut fresh_fasteners = Vec::with_capacity(fragment_fasteners.len());
    for fastener in fragment_fasteners {
        let mut next = fastener.clone();
        next.id = next_fastener_id();
        next.source = rewrite_grip_ref_local(&fastener.source, &id_map);
        next.target = rewrite_grip_ref_local(&fastener.target, &id_map);
        fresh_fasteners.push(next);
    }
    (fresh_parts, fresh_fasteners)
}
//#endregion 🔖️CopyPaste

//#region 🔖️KitIn
/// 🎞️ `kit:in` fragment row shapes (block3d's `puzzle3d_catalog_fragment`, camelCase) — local
/// deserialize-only mirrors of `objectKinds[]`/`objectKinds[].vortices[]`/`vortexKinds[]` entries, kept
/// separate from the typed catalog rows (whose field names/shape differ) so `import_media` can parse
/// the fragment once and then build the real typed catalog rows explicitly, field by field.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dKitInObjectKindFragment {
    id: String,
    name: String,
    label: String,
    #[serde(default)]
    mesh_url: Option<String>,
    #[serde(default)]
    vortices: Vec<Puzzle5dKitInVortexFragment>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dKitInVortexFragment {
    #[serde(default, rename = "id")]
    #[allow(dead_code)]
    _id: String,
    vortex_kind: String,
    position: [f64; 3],
    direction: [f64; 3],
    radius: f64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dKitInVortexKindFragment {
    id: String,
    name: String,
    label: String,
    color: String,
    #[serde(default)]
    default_cable_kind: String,
}

/// 🔌️ `kit:in` seam helper: keyed UPSERT of catalog PART rows (by `id`) — replaces any existing row
/// with the same id, else appends. Deterministic/order-independent in the resulting SET of ids (a
/// `multiplicity: Many` port may fan in from several producers across several `import_media` calls);
/// when two producers disagree on one id's content, the most-recently-applied wins.
fn puzzle5d_upsert_catalog_parts(existing: &mut Vec<crate::artifacts::puzzle5d::Puzzle5dCatalogPart>, incoming: Vec<crate::artifacts::puzzle5d::Puzzle5dCatalogPart>) {
    for row in incoming {
        match existing.iter().position(|entry| entry.id == row.id) {
            Some(index) => existing[index] = row,
            None => existing.push(row),
        }
    }
}

/// 🔌️ `kit:in` seam helper: keyed UPSERT of catalog GRIP-KIND rows (by `id`) — see
/// `puzzle5d_upsert_catalog_parts`'s doc for the upsert/idempotency contract.
fn puzzle5d_upsert_catalog_grips(existing: &mut Vec<crate::artifacts::puzzle5d::Puzzle5dCatalogGrip>, incoming: Vec<crate::artifacts::puzzle5d::Puzzle5dCatalogGrip>) {
    for row in incoming {
        match existing.iter().position(|entry| entry.id == row.id) {
            Some(index) => existing[index] = row,
            None => existing.push(row),
        }
    }
}

/// 🔌️ `kit:in` seam helper: keyed UPSERT of kind-compatibility rows by the `(source, target)` pair.
fn puzzle5d_upsert_kind_compatibility(existing: &mut Vec<crate::artifacts::puzzle5d::Puzzle5dKindCompatibility>, incoming: Vec<crate::artifacts::puzzle5d::Puzzle5dKindCompatibility>) {
    for row in incoming {
        match existing.iter().position(|entry| entry.source == row.source && entry.target == row.target) {
            Some(index) => existing[index] = row,
            None => existing.push(row),
        }
    }
}
//#endregion 🔖️KitIn

//#region 🔖️ContextMenu
/// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: `duplicateSelection`/`selectSameKindSelection`/
/// `zoomToSelection` stay top-level verbs; the hide/lock toggles (bespoke rows — their label/icon flip
/// on selection state, so they can't resolve from a single static `ActionDefinition`) fold into a
/// `settings` group; `deleteSelection` (bespoke label carrying the selection-count phrase) stays the
/// trailing destructive row. `organize_context_menu`, run automatically at the `VcsDocumentApp::context_menu`
/// funnel, handles taxonomy ordering/separator placement — this function only needs to emit the rows.
fn puzzle5d_context_menu_items(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels, is_de: bool, registry: &semio_framework_plugin::AppActionRegistry) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, ContextMenuItemSpec, Menu};
    if envelope.runtime.selection.part_ids.is_empty() {
        return Vec::new();
    }
    let selected: Vec<&Puzzle5dPart> = envelope.document.parts.iter().filter(|part| envelope.runtime.selection.part_ids.contains(&part.id)).collect();
    let all_hidden = !selected.is_empty() && selected.iter().all(|part| part.part_2d.hidden.unwrap_or(false));
    let all_locked = !selected.is_empty() && selected.iter().all(|part| part.part_2d.locked.unwrap_or(false));
    let phrase = selection_count_phrase(is_de, &[(envelope.runtime.selection.part_ids.len(), if is_de { "Teil" } else { "part" }, if is_de { "Teile" } else { "parts" })]);
    let bespoke = |id: &str, label: String, icon: &str, action: &str, args: Option<Value>, destructive: bool| ContextMenuItemSpec {
        id: id.into(),
        label: Some(label),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args: semio_framework_plugin::optional_json_to_dsl(args),
        destructive: destructive.then_some(true),
        ..Default::default()
    };
    Menu::of(registry)
        .action("duplicateSelection")
        .action("selectSameKindSelection")
        .action("zoomToSelection")
        .group("settings", |m| {
            m.item(bespoke("hide-show", if all_hidden { labels.show.into() } else { labels.hide.into() }, if all_hidden { "eye" } else { "eye-off" }, "setSelectionFlag", Some(json!({ "flag": "hidden", "value": !all_hidden })), false)).item(bespoke(
                "lock-unlock",
                if all_locked { labels.unlock.into() } else { labels.lock.into() },
                if all_locked { "lock-open" } else { "lock" },
                "setSelectionFlag",
                Some(json!({ "flag": "locked", "value": !all_locked })),
                false,
            ))
        })
        .item(bespoke("delete", format!("{} ({phrase})", labels.delete.as_str()), "trash", "deleteSelection", None, true))
        .build()
}
//#endregion 🔖️ContextMenu

//#region 🔖️Puzzle5dCommand
/// @emoji 🎯️ B1: `Puzzle5dPlayApp::Command` — the SOLE dispatch surface, one variant per declared
/// action (mirrors every `.operation(...)`/`.view_action(...)` id `create_puzzle5d_app` registers,
/// plus the framework-injected `SET_ACTIVE_UTILITY_ACTION_ID`). Each variant carries `window_id` (was
/// host-pushed `view_state.window_id`) plus `args` (the action's original `{...}` JSON payload,
/// unchanged) — `handle` reconstructs the exact `(action, args, window_id)` triple every
/// `🎮️commands/*` arm expects, so each arm's internal `args.get("field")` extraction stays
/// byte-for-byte identical to the pre-migration implementation.
///
/// ⚠️ `OpBinary` is a plain JSON-bytes bridge (NOT `#[derive(dsl::DslOps)]`, and NOT the framework's
/// `app_commands!` macro): a generic `args: Value` field is not representable in the DSL grammar those
/// target, so adopting them would silently rewrite this app's wire format. Keep this macro's variant
/// list, its order and its action-id literals byte-for-byte stable.
macro_rules! puzzle5d_command_variants {
    ($($Variant:ident = $id:tt),* $(,)?) => {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Puzzle5dCommand {
            $($Variant { window_id: Option<String>, args: Option<Value> }),*
        }

        impl Puzzle5dCommand {
            /// 🏷️ The action id this variant was declared under — used both for `command_id()`
            /// (command-log labeling / registry kind-discipline) and to reconstruct the exact
            /// `action: &str` `handle_action_impl` dispatches on.
            fn action_id(&self) -> &'static str {
                match self {
                    $(Puzzle5dCommand::$Variant { .. } => $id),*
                }
            }

            fn window_id(&self) -> Option<&str> {
                match self {
                    $(Puzzle5dCommand::$Variant { window_id, .. } => window_id.as_deref()),*
                }
            }

            fn args(&self) -> Option<&Value> {
                match self {
                    $(Puzzle5dCommand::$Variant { args, .. } => args.as_ref()),*
                }
            }

            /// 🧪️ Test-only reverse of `action_id()` — builds the variant for a given action id, for
            /// the testkit's `dispatch(...)` helper. Panics on an unknown action id (a test bug, not
            /// a runtime path).
            #[cfg(test)]
            fn from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Self {
                match action {
                    $($id => Puzzle5dCommand::$Variant { window_id, args }),*,
                    other => panic!("unknown puzzle5d action id in test: {other}"),
                }
            }
        }
    };
}

puzzle5d_command_variants! {
    SetFixtureJson = "setFixtureJson",
    SetActiveExample = "setActiveExample",
    AddNode = "addNode",
    AddPartKind = "addPartKind",
    AddBrushPart = "addBrushPart",
    AddBrushObject = "addBrushObject",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    SetSelectionFlag = "setSelectionFlag",
    ZoomToSelection = "zoomToSelection",
    FocusSelection = "focusSelection",
    EngagementSubmit = "engagementSubmit",
    SetFillCount = "setFillCount",
    PatchPart = "patchPart",
    PatchGrip = "patchGrip",
    PatchFastener = "patchFastener",
    ImportComposeKit = "importComposeKit",
    TranslateSelection = "translateSelection",
    RotateSelection = "rotateSelection",
    ScaleSelection = "scaleSelection",
    WorldRelocate = "worldRelocate",
    ApplyBoardEvents = "applyBoardEvents",
    SetCamera = "setCamera",
    SetCamera2d = "setCamera2d",
    SetCamera3d = "setCamera3d",
    SetSelection = "setSelection",
    DocumentSelect = "documentSelect",
    ClearSelection = "clearSelection",
    SelectAll = "selectAll",
    SelectSameKindSelection = "selectSameKindSelection",
    SelectSameKind = "selectSameKind",
    ToggleSun = "toggleSun",
    SetSunAzimuth = "setSunAzimuth",
    SetSunElevation = "setSunElevation",
    SetSunIntensity = "setSunIntensity",
    EngagementInput = "engagementInput",
    EngagementAbort = "engagementAbort",
    EngagementControlSelect = "engagementControlSelect",
    CycleBrushCandidate = "cycleBrushCandidate",
    RegisterBrushMesh = "registerBrushMesh",
    SetBrushPlacementOverlapBudget = "setBrushPlacementOverlapBudget",
    SetObjectKindWeight = "setObjectKindWeight",
    SetVortexKindWeight = "setVortexKindWeight",
    WorldSelect = "worldSelect",
    WorldPick = "worldPick",
    WorldHover = "worldHover",
    SetHover = "setHover",
    WorldVortexHover = "worldVortexHover",
    WorldVortexSelect = "worldVortexSelect",
    SetSelectionMethod = "setSelectionMethod",
    SetLodMode = "setLodMode",
    SetSuggestionOffset = "setSuggestionOffset",
    SetGridSnapEnabled = "setGridSnapEnabled",
    SetGridFactor = "setGridFactor",
    WorldPointerDown = "worldPointerDown",
    CanvasPointerDown = "canvasPointerDown",
    SetActiveUtility = SET_ACTIVE_UTILITY_ACTION_ID,
}

impl protocol::OpBinary for Puzzle5dCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}
//#endregion 🔖️Puzzle5dCommand

//#region 🔖️ActionContext
/// 🎬️ Everything one `🎮️commands/*` arm may read or write. The prologue/epilogue around the dispatch
/// match (scene materialization, delta computation, host-effect emission, config snapshotting) stays
/// in [`Puzzle5dPlayApp::handle_action_impl`]; an arm only mutates this bundle.
pub struct Puzzle5dActionCtx<'a> {
    /// 🧠️ The app's long-lived precompute session and mesh cache — every arm reaching them goes
    /// through `borrow_mut()`.
    pub app: &'a Puzzle5dPlayApp,
    pub scene: &'a mut Puzzle5dScene,
    /// 🪟️ The window this action targets (already defaulted to the 3D window).
    pub window_id: &'a str,
    /// 🛑️ Set by an arm that must skip the whole epilogue (delta, effects, config snapshot) — the
    /// direct replacement for the pre-migration `return Emit::default()` early exits.
    pub abort: bool,
}
//#endregion 🔖️ActionContext

//#region 🔖️PlayApp
/// 🧩️ B1: Puzzle-5d play app. Owns the precompute engine and the registered-mesh cache — both
/// per-call scratch, never VCS-tracked; the persisted document (bare `Puzzle5dDocument` json) lives in
/// the wrapping `VcsDocumentApp`'s document store, and the ephemeral view state lives in the wrapping
/// store's real, VCS-tracked `Puzzle5dConfig` artifact (see `🦀️config.rs`) — every read comes from
/// `cfg.projection`, every write flows out as a `Puzzle5dConfigOperation` in the returned `Emit`.
/// Each action mutates a transient {@link Puzzle5dScene}, then emits the granular operation delta.
/// Undo/redo/checkpoints are handled by the wrapper.
pub #[derive(Default, Clone, Copy)]
struct Puzzle5dPlayApp;

impl Default for Puzzle5dPlayApp
}

impl Puzzle5dPlayApp
        for url in collect_mesh_urls(&envelope.document) {
            if !(std::cell::RefCell::new(std::collections::HashSet::<String>::new())).borrow_mut().contains(&url) && !(std::cell::RefCell::new(Puzzle5dPrecomputeSession::default())).borrow_mut().has_mesh(&url) {
                let fallback = semio_framework_plugin::mesh_from_kind(PUZZLE5D_FALLBACK_MESH_KIND);
                (std::cell::RefCell::new(Puzzle5dPrecomputeSession::default())).borrow_mut().register_mesh(&url, &fallback.positions, &fallback.indices);
            }
        }
        let _ = (std::cell::RefCell::new(Puzzle5dPrecomputeSession::default())).borrow_mut().precompute_step(8);
    }

    pub fn apply_engine_brush_placement(&self, envelope: &Puzzle5dScene, payload: &Value) -> Option<Puzzle5dScene> {
        let brush_payload = serde_json::from_value::<BrushPlacePayload>(payload.clone()).ok()?;
        let fixture_json = (std::cell::RefCell::new(Puzzle5dPrecomputeSession::default())).borrow_mut().apply_brush_placement_rust(&serde_json::to_string(&brush_payload).ok()?).ok()?;
        merge_engine_fixture(envelope, &fixture_json)
    }

    /// 🖌️ Paired placement for a board `brushPlace` event: the engine picks the volume pose for the flat payload's kind, both aspects land in one part.
    pub fn apply_board_brush_place(&self, envelope: &mut Puzzle5dScene, payload: &Value) {
        self.drive_precompute(envelope);
        let node_kind = payload.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("Part").to_string();
        let source_grip = payload.get("sourceHandleId").and_then(|value| value.as_str()).map(str::to_string).or_else(|| puzzle5d_brush_target_grip(envelope));
        if let Some(source_grip) = source_grip.as_ref() {
            let candidates = parse_brush_candidates_free(&(std::cell::RefCell::new(Puzzle5dPrecomputeSession::default())).borrow().brush_candidates(source_grip));
            let candidate_index =
                candidates.iter().position(|candidate| candidate.get("objectKindId").or_else(|| candidate.get("objectKind")).and_then(|value| value.as_str()) == Some(node_kind.as_str())).unwrap_or(envelope.runtime.brush_candidate_index);
            let engine_payload = json!({ "objectKindId": node_kind, "targetVortexFullId": source_grip, "candidateIndex": candidate_index });
            if let Some(mut next) = self.apply_engine_brush_placement(envelope, &engine_payload) {
                let previous_ids: HashSet<String> = envelope.document.parts.iter().map(|part| part.id.clone()).collect();
                let new_id = next.document.parts.iter().map(|part| part.id.clone()).find(|id| !previous_ids.contains(id));
                if let Some(new_id) = new_id {
                    let x = payload.get("x").and_then(|value| value.as_f64());
                    let y = payload.get("y").and_then(|value| value.as_f64());
                    set_part_2d_position(&mut next.document, &new_id, x, y);
                    next.runtime.selection = Puzzle5dSelection { part_ids: SelectionSet::from_ids(vec![new_id]), grip_ids: SelectionSet::default(), fastener_ids: SelectionSet::default() };
                }
                *envelope = next;
                return;
            }
        }
        let id = payload.get("nodeId").and_then(|value| value.as_str()).map_or_else(next_part_id, str::to_string);
        let x = payload.get("x").and_then(|value| value.as_f64()).unwrap_or(120.0);
        let y = payload.get("y").and_then(|value| value.as_f64()).unwrap_or(120.0);
        let mesh_url = resolve_part_kind_mesh_url(&node_kind, envelope.document.kind_catalogs.as_ref());
        let grips = grips_from_templates(&envelope.document, &node_kind);
        let source_world = source_grip.as_ref().and_then(|full_id| find_part_by_grip_full_id(&envelope.document, full_id).map(|(part, grip)| (world_grip_position(part, grip), world_grip_direction(part, grip))));
        let origin = source_world.map_or([0.0, 0.0, 0.0], |(position, direction)| [position[0] + direction[0], position[1] + direction[1], position[2] + direction[2]]);
        envelope.document.parts.push(Puzzle5dPart {
            id: id.clone(),
            part_kind: node_kind.clone(),
            part_2d: Puzzle5dPart2d { x, y, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: node_kind, icon_kind: None, hidden: None, locked: None },
            part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
            grips,
        });
        if let (Some(source), Some(part)) = (source_grip, envelope.document.parts.last()) {
            if let Some(grip) = part.grips.first() {
                let target = puzzle5d_grip_full_id(&part.id, &grip.id);
                envelope.document.fasteners.push(Puzzle5dFastener {
                    id: payload.get("edgeId").and_then(|value| value.as_str()).map_or_else(next_fastener_id, str::to_string),
                    source,
                    target,
                    fastener_kind: None,
                    gap: 0.0,
                    shift: 0.0,
                    rise: 0.0,
                    rotation: 0.0,
                    turn: 0.0,
                    tilt: 0.0,
                });
            }
        }
        envelope.runtime.selection = Puzzle5dSelection { part_ids: SelectionSet::from_ids(vec![id]), grip_ids: SelectionSet::default(), fastener_ids: SelectionSet::default() };
    }

    pub fn apply_board_events_from_json(&self, events_json: &str, envelope: &mut Puzzle5dScene) {
        let Ok(events) = serde_json::from_str::<Vec<Value>>(events_json) else {
            return;
        };
        for event in events {
            let Some(name) = event.get("name").and_then(|value| value.as_str()) else {
                continue;
            };
            let payload = event.get("payload").cloned().unwrap_or(Value::Null);
            match name {
                "camera" => {
                    if let Ok(camera) = serde_json::from_value::<Puzzle5dCamera2d>(payload) {
                        envelope.runtime.camera2d = camera;
                    }
                }
                "select" => {
                    if let Some(ids) = payload.get("ids").and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                        envelope.runtime.selection = classify_selection(&envelope.document, &ids);
                    }
                }
                "nodeDragEnd" => {
                    for entry in payload.get("moves").and_then(|value| value.as_array()).into_iter().flatten() {
                        if let Some(id) = entry.get("id").and_then(|value| value.as_str()) {
                            set_part_2d_position(&mut envelope.document, id, entry.get("x").and_then(|value| value.as_f64()), entry.get("y").and_then(|value| value.as_f64()));
                        }
                    }
                }
                "nodeMove" => {
                    if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                        set_part_2d_position(&mut envelope.document, id, payload.get("x").and_then(|value| value.as_f64()), payload.get("y").and_then(|value| value.as_f64()));
                    }
                }
                "brushPlace" => {
                    self.apply_board_brush_place(envelope, &payload);
                }
                "edgeCreate" => {
                    let source = payload.get("source").and_then(|value| value.as_str()).unwrap_or("").to_string();
                    let target = payload.get("target").and_then(|value| value.as_str()).unwrap_or("").to_string();
                    if !source.is_empty() && !target.is_empty() && !envelope.document.fasteners.iter().any(|entry| entry.source == source && entry.target == target || entry.source == target && entry.target == source) {
                        envelope.document.fasteners.push(Puzzle5dFastener {
                            id: payload.get("id").and_then(|value| value.as_str()).map_or_else(next_fastener_id, str::to_string),
                            source,
                            target,
                            fastener_kind: payload.get("edgeKind").and_then(|value| value.as_str()).map(str::to_string),
                            gap: 0.0,
                            shift: 0.0,
                            rise: 0.0,
                            rotation: 0.0,
                            turn: 0.0,
                            tilt: 0.0,
                        });
                    }
                }
                "nodeDelete" => {
                    if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                        remove_parts(&mut envelope.document, &[id.to_string()]);
                        envelope.runtime.selection = Puzzle5dSelection::default();
                    }
                }
                "edgeDelete" => {
                    if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                        envelope.document.fasteners.retain(|fastener| fastener.id != id);
                    }
                }
                _ => {}
            }
        }
    }

    /// @emoji 🧩️ B1: the pure per-action core, dispatched into by `DocumentApp::handle` with
    /// `action`/`args`/`window_id` reconstructed 1:1 from the typed `Puzzle5dCommand`. Everything past
    /// this adapter boundary reads/writes the passed-in `Puzzle5dConfig` snapshot and returns a real
    /// `Emit` (document + config operations) instead of mutating `self`.
    fn handle_action_impl(&self, action: &str, args: Option<&Value>, window_id: Option<&str>, doc: &DocumentView<'_, Puzzle5dPlayProjection>, config: &Puzzle5dConfig) -> Emit<Puzzle5dOperation, Puzzle5dConfigOperation> {
        let before = doc.projection.0.clone();
        let active_utility_initial = puzzle5d_scene_active_utility(config, window_id);
        let wid = window_id.map_or_else(|| world3d::WINDOW_KIND_ID.to_string(), str::to_string);
        let mut scene = scene_from_projection(&before, config.clone(), &active_utility_initial);
        let mut ctx = Puzzle5dActionCtx { app: self, scene: &mut scene, window_id: &wid, abort: false };
        dispatch_puzzle5d_action(&mut ctx, action, args);
        if ctx.abort {
            return Emit::default();
        }
        let next_active_utility = scene.active_utility.clone();
        let operations = puzzle5d_operations_from_document_change(&before, &scene.document);
        // 🌀️ Coalesce each gumball drag tick into one undoable edit (compact per-part records, not full meshes).
        let coalesce_key = match action {
            "translateSelection" => Some("gumball-translate".to_string()),
            "rotateSelection" => Some("gumball-rotate".to_string()),
            "scaleSelection" => Some("gumball-scale".to_string()),
            _ => None,
        };
        // 🧰️ B1: a DIRECT `SetActiveUtility` command already told the host what it needs to know — never
        // re-emit the same switch as a `HostEffect` (the pre-B1 code only had to guard this for the
        // INDIRECT paths below, since the host itself pushed the direct switch before dispatching; now
        // the command IS the direct switch, so this arm must self-exclude). Programmatic utility
        // switches (engagement submit/abort, fill) still push the active utility back into the host
        // session for both windows.
        let is_direct_utility_switch = action == SET_ACTIVE_UTILITY_ACTION_ID;
        let effects = if !is_direct_utility_switch && next_active_utility != active_utility_initial {
            PUZZLE5D_PLAY_WINDOWS.iter().map(|window| HostEffect::SetActiveUtility { window_id: (*window).into(), utility_id: next_active_utility.clone() }).collect()
        } else {
            Vec::new()
        };
        // 🧮️ B1: only a REAL config change becomes a `Puzzle5dConfigOperation` — `PartialEq` (derived)
        // makes this cheap, and keeps a pure read-only action from creating a no-op undo entry.
        let config_operations = if &scene.runtime != config { vec![Puzzle5dConfigOperation::Snapshot { config: scene.runtime }] } else { Vec::new() };
        Emit { document_operations: operations, config_operations, coalesce_key, effects, ..Default::default() }
    }
}

/// 🎬️ Dispatch only: every arm's behaviour lives in its `🎮️commands/<group>/🦀️component.rs` free
/// function. No behaviour lives in this match.
fn dispatch_puzzle5d_action(ctx: &mut Puzzle5dActionCtx<'_>, action: &str, args: Option<&Value>) {
    match action {
        "setFixtureJson" => example::set_fixture_json(ctx, args),
        "setActiveExample" => example::set_active_example(ctx, args),
        "importComposeKit" => example::import_compose_kit(ctx, args),
        "setSelection" | "documentSelect" => selection_commands::set_selection(ctx, args),
        "clearSelection" => selection_commands::clear_selection(ctx),
        "selectAll" => selection_commands::select_all(ctx),
        "selectSameKindSelection" | "selectSameKind" => selection_commands::select_same_kind(ctx),
        "setSelectionMethod" => selection_commands::set_selection_method(ctx, args),
        "worldSelect" => selection_commands::world_select(ctx, args),
        "worldPick" => selection_commands::world_pick(ctx, args),
        "addNode" => part::add_node(ctx, args),
        "addPartKind" => part::add_part_kind(ctx, args),
        "deleteSelection" => part::delete_selection(ctx),
        "duplicateSelection" => part::duplicate_selection(ctx),
        "setSelectionFlag" => part::set_selection_flag(ctx, args),
        "patchPart" => patch::patch_part(ctx, args),
        "patchGrip" => patch::patch_grip(ctx, args),
        "patchFastener" => patch::patch_fastener(ctx, args),
        "worldHover" => hover::world_hover(ctx, args),
        "setHover" => hover::set_hover(ctx, args),
        "worldVortexHover" => hover::world_vortex_hover(ctx, args),
        "worldVortexSelect" => hover::world_vortex_select(ctx, args),
        "setCamera" => camera::set_camera(ctx, args),
        "setCamera2d" => camera::set_camera_2d(ctx, args),
        "setCamera3d" => camera::set_camera_3d(ctx, args),
        "zoomToSelection" | "focusSelection" => camera::zoom_to_selection(ctx),
        "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => sun::apply(ctx, action, args),
        "setLodMode" => lod::set_lod_mode(ctx, args),
        "setGridSnapEnabled" => grid::set_grid_snap_enabled(ctx, args),
        "setGridFactor" => grid::set_grid_factor(ctx, args),
        "addBrushPart" | "addBrushObject" => brush::add_brush_part(ctx, args),
        "cycleBrushCandidate" => brush::cycle_brush_candidate(ctx),
        "registerBrushMesh" => brush::register_brush_mesh(ctx, args),
        "setBrushPlacementOverlapBudget" => brush::set_brush_placement_overlap_budget(ctx, args),
        "setObjectKindWeight" | "setVortexKindWeight" => brush::set_kind_weight(ctx, action, args),
        "engagementControlSelect" => brush::engagement_control_select(ctx, args),
        "setSuggestionOffset" => brush::set_suggestion_offset(ctx, args),
        "setFillCount" => fill::set_fill_count(ctx, args),
        "engagementInput" => engagement::engagement_input(ctx, args),
        "engagementSubmit" => engagement::engagement_submit(ctx, args),
        "engagementAbort" => engagement::engagement_abort(ctx, args),
        "translateSelection" => transform::translate_selection(ctx, args),
        "rotateSelection" => transform::rotate_selection(ctx, args),
        "scaleSelection" => transform::scale_selection(ctx, args),
        "worldRelocate" => transform::world_relocate(ctx, args),
        "applyBoardEvents" => board::apply_board_events(ctx, args),
        SET_ACTIVE_UTILITY_ACTION_ID => utility::set_active(ctx, args),
        // 🛑️ Pure pointer-down notifications: no scene mutation, no operations, no config snapshot —
        // the pre-migration code returned `Emit::default()` here, which `abort` reproduces exactly.
        "worldPointerDown" | "canvasPointerDown" => ctx.abort = true,
        _ => {}
    }
}

impl DocumentApp for Puzzle5dPlayApp

    fn clipboard_media_type() -> Option<MediaType> {
        Some(MediaType { class: MediaClass::Kit, form: MediaForm::Design })
    }

    fn copy_fragment(doc: &DocumentView<'_, Puzzle5dPlayProjection>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> Result<ClipboardFragment, ClipboardError> {
        let document: Puzzle5dDocument = serde_json::from_value(doc.projection.0.clone()).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let selection = &cfg.projection.selection;
        let (parts, fasteners) = copy_selection_local(&document, selection.part_ids.as_slice(), selection.fastener_ids.as_slice());
        if parts.is_empty() {
            return Err(ClipboardError::EmptySelection);
        }
        let fragment_value = json!({ "schema": PUZZLE5D_SCHEMA, "parts": parts, "fasteners": fasteners });
        Ok(ClipboardFragment {
            schema: PUZZLE5D_SCHEMA.to_string(),
            media_type: self.clipboard_media_type().expect("declared above"),
            dsl_text: serde_json::to_string_pretty(&fragment_value).unwrap_or_default(),
            pack_bytes: None,
            source_app: PUZZLE5D_PLAY_APP_ID.to_string(),
            label: format!("{} part(s)", parts.len()),
        })
    }

    /// @emoji ✂️ B1: `DocumentApp::cut_operations`'s signature carries no config output channel (it
    /// returns a bare `Vec<Self::Operation>`, not an `Emit`), so this can only emit the document
    /// removal; clearing the selection is left to the framework's own post-cut selection reconciliation
    /// (the cut parts/fasteners are gone from the document either way, so a stale selection referencing
    /// them is inert until the next real selection action overwrites it).
    fn cut_operations(doc: &DocumentView<'_, Puzzle5dPlayProjection>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> Vec<Puzzle5dOperation> {
        let before = doc.projection.0.clone();
        let Ok(document) = serde_json::from_value::<Puzzle5dDocument>(before.clone()) else {
            return Vec::new();
        };
        let selection = &cfg.projection.selection;
        let (parts, fasteners) = copy_selection_local(&document, selection.part_ids.as_slice(), selection.fastener_ids.as_slice());
        if parts.is_empty() {
            return Vec::new();
        }
        let remove_part_ids: HashSet<&str> = parts.iter().map(|part| part.id.as_str()).collect();
        let remove_fastener_ids: HashSet<&str> = fasteners.iter().map(|fastener| fastener.id.as_str()).collect();
        let mut after = document;
        after.parts.retain(|part| !remove_part_ids.contains(part.id.as_str()));
        after.fasteners.retain(|fastener| !remove_fastener_ids.contains(fastener.id.as_str()));
        puzzle5d_operations_from_document_change(&before, &after)
    }

    /// @emoji 📋️ B1: `DocumentApp::paste_operations` carries no `ConfigView` at all (only `doc`/
    /// `fragment`/`placement`), so the new selection can't be threaded through this call; a following
    /// `setSelection` command (which the host already issues after a paste in practice) is what
    /// actually selects the pasted parts now.
    fn paste_operations(doc: &DocumentView<'_, Puzzle5dPlayProjection>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle5dOperation>, ClipboardError> {
        let expected = self.clipboard_media_type().unwrap_or(MediaType { class: MediaClass::Kit, form: MediaForm::Design });
        if fragment.media_type != expected {
            return Err(ClipboardError::IncompatibleMediaType(fragment.media_type));
        }
        let fragment_value: Value = serde_json::from_str(&fragment.dsl_text).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_parts: Vec<Puzzle5dPart> = serde_json::from_value(fragment_value.get("parts").cloned().unwrap_or_else(|| json!([]))).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_fasteners: Vec<Puzzle5dFastener> = serde_json::from_value(fragment_value.get("fasteners").cloned().unwrap_or_else(|| json!([]))).unwrap_or_default();
        let before = doc.projection.0.clone();
        let document: Puzzle5dDocument = serde_json::from_value(before.clone()).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let delta = paste_delta_2d(&fragment_parts, &document.parts, placement);
        let (fresh_parts, fresh_fasteners) = paste_selection_local(&fragment_parts, &fragment_fasteners, delta);
        let mut after = document;
        after.parts.extend(fresh_parts);
        after.fasteners.extend(fresh_fasteners);
        Ok(puzzle5d_operations_from_document_change(&before, &after))
    }

    /// 🏷️ Maps each `Puzzle5dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle5dCommand) -> &'static str {
        command.action_id()
    }

    /// @emoji 🧩️ Thin typed-command adapter — reconstructs the exact `(action, args, window_id)`
    /// triple `handle_action_impl` expects from the typed `Puzzle5dCommand`.
    fn handle(command: &Puzzle5dCommand, doc: &DocumentView<'_, Puzzle5dPlayProjection>, cfg: &ConfigView<'_, Puzzle5dConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Puzzle5dOperation, Puzzle5dConfigOperation, Self::DraftOperation>, Fault> {
        Ok(self.handle_action_impl(command.action_id(), command.args(), command.window_id(), doc, cfg.projection))
    }

    /// 🔌️ Declares puzzle5d's typed media I/O surface: the implicit document ports (from
    /// `.document([...])`/`.artifact_kind(...)` in `create_puzzle5d_app`) plus `kit:in` (accepting a
    /// `kit.catalog` fragment shaped like block3d's `puzzle3d_catalog_fragment`, fanning IN from
    /// potentially many producers) and `design:out` (this app's own `5d.puzzle` design artifact, fanning
    /// OUT to potentially many consumers).
    fn io() -> Option<AppIo> {
        Some(
            AppIo::from_document("puzzle.5d", MediaType { class: MediaClass::Kit, form: MediaForm::Design }, ArtifactPresentation { id: "5d.puzzle".into(), name: "5D Puzzle".into(), dimension: "5d".into(), component_kind: "puzzle5d".into() })
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
                        label: "5D Puzzle Design".into(),
                        direction: MediaPortDirection::Out,
                        // 🔁️ Reuses the exact `id`/`media_type` already declared on the artifact's own
                        // `artifact_kind()` — the same design artifact this app's document already
                        // publishes, just exposed as an explicit workflow output port.
                        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design },
                        kind_id: Some("5d.puzzle".into()),
                        required: false,
                        multiplicity: PortMultiplicity::Many,
                    },
                ]),
        )
    }

    /// 🎞️ `kit:in` seam: normalizes an incoming `kit.catalog` fragment (block3d's
    /// `puzzle3d_catalog_fragment` shape — `objectKinds`/`vortexKinds`/`cableKinds`/`attractionKinds`/
    /// `kindCompatibility`) into puzzle5d's own typed `kindCatalogs`/`kindCompatibility` vocabulary and
    /// upserts it (keyed by row `id`, deterministic/order-independent — safe for `multiplicity: Many`
    /// fan-in from several producers), then bridges the before/after document through
    /// `puzzle5d_operations_from_document_change` exactly like every other document-mutating action —
    /// this never mutates anything directly, only real, undoable operations.
    fn import_media(port: &str, media: &Media, doc: &DocumentView<'_, Puzzle5dPlayProjection>) -> Result<Emit<Puzzle5dOperation, Puzzle5dConfigOperation, Self::DraftOperation>, MediaError> {
        if port != "kit:in" {
            return Err(MediaError::NotImplemented);
        }
        let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "kit:in only accepts a Structured (JSON) payload".into()));
        };
        let fragment: Value = serde_json::from_str(json.as_str()).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let mut document: Puzzle5dDocument = serde_json::from_value(doc.projection.0.clone()).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;

        let mut catalogs: crate::artifacts::puzzle5d::Puzzle5dKindCatalogs = document.kind_catalogs.as_ref().and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();

        if let Some(incoming_parts) = fragment.get("objectKinds").and_then(Value::as_array) {
            let parsed: Vec<crate::artifacts::puzzle5d::Puzzle5dCatalogPart> = incoming_parts
                .iter()
                .filter_map(|row| {
                    let parsed_row: Puzzle5dKitInObjectKindFragment = serde_json::from_value(row.clone()).ok()?;
                    Some(crate::artifacts::puzzle5d::Puzzle5dCatalogPart {
                        id: parsed_row.id,
                        name: parsed_row.name,
                        label: parsed_row.label,
                        mesh_url: parsed_row.mesh_url,
                        grips: parsed_row
                            .vortices
                            .into_iter()
                            .map(|vortex| crate::artifacts::puzzle5d::Puzzle5dCatalogGripTemplate {
                                grip_kind: vortex.vortex_kind,
                                grip_2d: None,
                                grip_3d: Some(crate::artifacts::puzzle5d::Puzzle5dCatalogGripTemplate3d { position: vortex.position, direction: vortex.direction, radius: vortex.radius }),
                            })
                            .collect(),
                    })
                })
                .collect();
            puzzle5d_upsert_catalog_parts(&mut catalogs.parts, parsed);
        }
        if let Some(incoming_grips) = fragment.get("vortexKinds").and_then(Value::as_array) {
            let parsed: Vec<crate::artifacts::puzzle5d::Puzzle5dCatalogGrip> = incoming_grips
                .iter()
                .filter_map(|row| {
                    let parsed_row: Puzzle5dKitInVortexKindFragment = serde_json::from_value(row.clone()).ok()?;
                    Some(crate::artifacts::puzzle5d::Puzzle5dCatalogGrip { id: parsed_row.id, name: parsed_row.name, label: parsed_row.label, color: parsed_row.color, default_rope_kind: parsed_row.default_cable_kind })
                })
                .collect();
            puzzle5d_upsert_catalog_grips(&mut catalogs.grips, parsed);
        }
        // 🚫️ `cableKinds`/`attractionKinds` are deliberately left unmapped: puzzle5d's `Puzzle5dKindCatalogs`
        // has no genuine 1:1 counterpart for either in THIS fragment shape — `ropes`/`fasteners` exist as
        // catalog sections, but `cableKinds`/`attractionKinds` describe 3D cable-tension/attraction-constraint
        // kinds, a different domain concept than puzzle5d's fastener/rope kinds. Forcing a mapping here would
        // be a fabricated guess, not a verified equivalence — left for a follow-up ticket with real fixtures.
        document.kind_catalogs = Some(serde_json::to_value(&catalogs).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?);

        if let Some(incoming_compat) = fragment.get("kindCompatibility").and_then(Value::as_array) {
            let mut compatibility: Vec<crate::artifacts::puzzle5d::Puzzle5dKindCompatibility> = document.kind_compatibility.as_ref().and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
            let parsed: Vec<crate::artifacts::puzzle5d::Puzzle5dKindCompatibility> = incoming_compat.iter().filter_map(|row| serde_json::from_value(row.clone()).ok()).collect();
            puzzle5d_upsert_kind_compatibility(&mut compatibility, parsed);
            document.kind_compatibility = Some(serde_json::to_value(&compatibility).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?);
        }

        let operations = puzzle5d_operations_from_document_change(&doc.projection.0, &document);
        Ok(Emit::operations(operations))
    }

    fn render(body_key: &str, doc: &DocumentView<'_, Puzzle5dPlayProjection>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> UiNode {
        let config = cfg.projection;
        let window_for_body = if body_key == board2d::BODY_KEY { board2d::WINDOW_KIND_ID } else { world3d::WINDOW_KIND_ID };
        let active_utility = puzzle5d_scene_active_utility(config, Some(window_for_body));
        let envelope = scene_from_projection(&doc.projection.0, config.clone(), &active_utility);
        let labels = puzzle5d_labels(config);
        match body_key {
            board2d::BODY_KEY => board2d::render(&envelope),
            world3d::BODY_KEY => world3d::render(&envelope, &(std::cell::RefCell::new(Puzzle5dPrecomputeSession::default())).borrow()),
            document_panel::BODY_KEY => document_panel::render(&envelope, labels),
            catalogue::BODY_KEY => catalogue::render(&envelope, labels),
            inspection::BODY_KEY => inspection::render(&envelope, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_engagements(doc: &DocumentView<'_, Puzzle5dPlayProjection>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> HashMap<String, WindowEngagement> {
        let config = cfg.projection;
        let labels = puzzle5d_labels(config);
        // 🪟️ One entry per live window INSTANCE of each of the 2D/3D window kinds — see
        // `window_instance_ids`'s doc comment for why puzzle5d needs none of puzzle3d's genuine
        // multi-instance-per-kind machinery here (each kind is always its own sole instance).
        PUZZLE5D_PLAY_WINDOWS
            .iter()
            .flat_map(|window| {
                window_instance_ids(window).into_iter().map(|wid| {
                    let active_utility = puzzle5d_scene_active_utility(config, Some(&wid));
                    let envelope = scene_from_projection(&doc.projection.0, config.clone(), &active_utility);
                    (wid, edit::puzzle5d_engagement(&envelope, window, labels))
                })
            })
            .collect()
    }

    fn window_measures(doc: &DocumentView<'_, Puzzle5dPlayProjection>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        let labels = puzzle5d_labels(config);
        PUZZLE5D_PLAY_WINDOWS
            .iter()
            .flat_map(|window| {
                window_instance_ids(window).into_iter().map(|wid| {
                    let active_utility = puzzle5d_scene_active_utility(config, Some(&wid));
                    let envelope = scene_from_projection(&doc.projection.0, config.clone(), &active_utility);
                    let measures = if *window == board2d::WINDOW_KIND_ID {
                        board2d::window_measures(&envelope, &(std::cell::RefCell::new(Puzzle5dPrecomputeSession::default())).borrow(), labels)
                    } else {
                        world3d::window_measures(&envelope, &(std::cell::RefCell::new(Puzzle5dPrecomputeSession::default())).borrow(), labels)
                    };
                    (wid, measures)
                })
            })
            .collect()
    }

    fn context_menu(
        &self,
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &DocumentView<'_, Puzzle5dPlayProjection>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let config = cfg.projection;
        let labels = puzzle5d_labels(config);
        let is_de = puzzle5d_is_de_locale(config);
        let active_utility = puzzle5d_scene_active_utility(config, Some(world3d::WINDOW_KIND_ID));
        let mut envelope = scene_from_projection(&doc.projection.0, config.clone(), &active_utility);
        if let Some(surface) = request.surface.as_ref() {
            let part_ids: Vec<String> = surface.selection.iter().filter(|g| g.domain == "object" || g.domain == "node" || g.domain == "part").flat_map(|g| g.ids.iter().cloned()).collect();
            if !part_ids.is_empty() {
                envelope.runtime.selection.part_ids = part_ids.into();
            }
        }
        puzzle5d_context_menu_items(&envelope, labels, is_de, registry)
    }
}
//#endregion 🔖️PlayApp

//#region 🔖️Manifest
pub fn create_puzzle5d_app() -> App {
    let envelope = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: PUZZLE5D_DEFAULT_UTILITY.into() };
    let precompute = Puzzle5dPrecomputeSession::new();
    let manifest_labels = puzzle5d_labels(&Puzzle5dConfig::default());
    App::from_builder(
        App::builder(PUZZLE5D_PLAY_APP_ID, LocalizedLabel::native("Puzzle 5D", "Puzzle 5D"))
            .document(["semio", "puzzle", "5d"])
            .artifact_kind(crate::artifacts::puzzle5d::artifact_kind())
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "puzzle", "5d"])
            .mode_def(edit::definition())
            .default_mode_id(edit::PUZZLE5D_PLAY_MODE_EDIT)
            .window_kind_def(board2d::definition(&envelope, &precompute, manifest_labels))
            .window_kind_def(world3d::definition(&envelope, &precompute, manifest_labels))
            // 🏗️ 3D-first 60/40 split — mirrors semio_compose_rs's design app (scene 60% / diagram 40%,
            // `semio_compose_rs/client/lib/sketchpad/js/index.ts:15367-15378`), the assembly-editing use case
            // this app replaces.
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            // 🔧️ Document-mutating operations (emit VCS operations through the before/after document delta).
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Operation) })
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .operation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .operation("addPartKind", LocalizedLabel::native("Add Part", "Teil hinzufügen"))
            .operation("addBrushPart", LocalizedLabel::native("Add Brush Part", "Pinselteil hinzufügen"))
            .operation("addBrushObject", LocalizedLabel::native("Add Brush Object", "Pinselobjekt hinzufügen"))
            .action_with(ActionDefinition::new_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Operation).with_category("selection"))
            .action_with(ActionDefinition::new_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Operation).with_category("create"))
            .action_with(ActionDefinition::new_catalog("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Operation).with_category("settings"))
            .action_with(ActionDefinition::new_catalog("zoomToSelection", LocalizedLabel::native("Zoom To Selection", "Auf Auswahl zoomen"), ActionKind::Operation).with_category("view"))
            .operation("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"))
            .operation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .operation("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"))
            .operation("patchPart", LocalizedLabel::native("Patch Part", "Teil aktualisieren"))
            .operation("patchGrip", LocalizedLabel::native("Patch Grip", "Griff aktualisieren"))
            .operation("patchFastener", LocalizedLabel::native("Patch Fastener", "Verbinder aktualisieren"))
            .operation("importComposeKit", LocalizedLabel::native("Import Compose Kit", "Compose-Kit importieren"))
            .operation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .operation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .operation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .operation("worldRelocate", LocalizedLabel::native("Relocate Part", "Teil verlagern"))
            .operation("applyBoardEvents", LocalizedLabel::native("Apply Board Events", "Board-Ereignisse anwenden"))
            // 👁️ Ephemeral view state — selection, hover, utility parameters, brush cycling, camera pose.
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("setCamera2d", LocalizedLabel::native("Set Camera 2D", "Kamera 2D festlegen"))
            .view_action("setCamera3d", LocalizedLabel::native("Set Camera 3D", "Kamera 3D festlegen"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("documentSelect", LocalizedLabel::native("Document Select", "Dokument auswählen"))
            .view_action("clearSelection", LocalizedLabel::native("Clear Selection", "Auswahl aufheben"))
            .view_action("selectAll", LocalizedLabel::native("Select All", "Alles auswählen"))
            .action_with(ActionDefinition::new_catalog("selectSameKindSelection", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).with_category("selection"))
            .view_action("selectSameKind", LocalizedLabel::native("Select Same Kind (alias)", "Gleiche Art auswählen (Alias)"))
            .view_action("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"))
            .view_action("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"))
            .view_action("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"))
            .view_action("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"))
            .view_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"))
            .view_action("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"))
            .view_action("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"))
            .view_action("cycleBrushCandidate", LocalizedLabel::native("Cycle Brush Candidate", "Pinselkandidat wechseln"))
            .view_action("registerBrushMesh", LocalizedLabel::native("Register Brush Mesh", "Pinsel-Mesh registrieren"))
            .view_action("setBrushPlacementOverlapBudget", LocalizedLabel::native("Set Brush Placement Overlap Budget", "Pinsel-Überlappungsbudget festlegen"))
            .view_action("setObjectKindWeight", LocalizedLabel::native("Set Object Kind Weight", "Objektart-Gewicht festlegen"))
            .view_action("setVortexKindWeight", LocalizedLabel::native("Set Vortex Kind Weight", "Vortexart-Gewicht festlegen"))
            .view_action("worldSelect", LocalizedLabel::native("World Select", "Welt auswählen"))
            .view_action("worldPick", LocalizedLabel::native("World Pick", "Welt-Auswahl (Pick)"))
            .view_action("worldHover", LocalizedLabel::native("World Hover", "Überfahren (Welt)"))
            .view_action("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"))
            .view_action("worldVortexHover", LocalizedLabel::native("World Vortex Hover", "Welt-Vortex-Hover"))
            .view_action("worldVortexSelect", LocalizedLabel::native("World Vortex Select", "Welt-Vortex auswählen"))
            .view_action("setSelectionMethod", LocalizedLabel::native("Set Selection Method", "Auswahlmethode festlegen"))
            .view_action("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"))
            .view_action("setSuggestionOffset", LocalizedLabel::native("Set Suggestion Offset", "Vorschlagsversatz festlegen"))
            .view_action("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"))
            .view_action("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"))
            .view_action("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"))
            .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"))
            // 📝️ Staged argument forms for the brush create actions (P1).
            .action_args("addPartKind", vec![
                ActionArgDef::select("partKind", puzzle5d_localized(|l| l.kind), vec![ActionArgOption::new("Part", puzzle5d_localized(|l| l.part))]).default_value("Part"),
            ])
            .action_args("addBrushPart", vec![
                ActionArgDef::select("partKind", puzzle5d_localized(|l| l.kind), vec![ActionArgOption::new("Part", puzzle5d_localized(|l| l.part))]).default_value("Part"),
            ])
            .action_args("addBrushObject", vec![
                ActionArgDef::select("partKind", puzzle5d_localized(|l| l.kind), vec![ActionArgOption::new("Part", puzzle5d_localized(|l| l.part))]).default_value("Part"),
            ])
            // 🧰️ Flat per-window set of utilities; `select` is the default. Each `🪛️utilities/*` node
            // owns its own id/definition; a utility bound by BOTH windows is declared once (under the
            // 2D window) and referenced by the 3D window's `definition()`.
            .utility(board2d::utilities::select::definition(puzzle5d_localized(|l| l.select)))
            .utility(world3d::utilities::transform::move_definition())
            .utility(world3d::utilities::transform::rotate_definition())
            .utility(world3d::utilities::transform::scale_definition())
            .utility(board2d::utilities::brush::definition(puzzle5d_localized(|l| l.brush)))
            .utility(board2d::utilities::fill::definition(puzzle5d_localized(|l| l.fill)))
            .utility(world3d::utilities::world_relocate::definition()),
    )
    .example(PUZZLE5D_EXAMPLE_CONCRETE_FOREST, puzzle5d_localized(|l| l.example_concrete_forest), CONCRETE_FOREST_EXAMPLE_JSON.clone(), "list-tree")
    .example(PUZZLE5D_EXAMPLE_NAKAGIN, LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin-Kapselturm"), NAKAGIN_EXAMPLE_JSON.clone(), "building")
    .workflow("puzzle5d", "Puzzle 5D", "model")
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
/// 📥️ Tier C DWG mesh import — always returns the empty puzzle-5d document; never errors on a structurally valid mesh.
fn puzzle5d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<Value, String> {
    serde_json::to_value(empty_document()).map_err(|error| error.to_string())
}

/// 🗂️ Registers `Puzzle5dPlayProjection`'s pack<->dsl codec under its real `document_schema()` string
/// so `framework/sync`'s `FolderEndpoint::Pack` can print/parse puzzle-5d play documents without
/// depending on this crate's concrete `Projection`/`Operation` types, plus the 5d mesh export/import
/// handlers. Called by the plugin `setup:` hook (`crate::artifacts::puzzle2d::engine::register`).
pub fn register_puzzle5d_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Puzzle5dPlayApp>(PUZZLE5D_SCHEMA);
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    {
        semio_framework_os::register_mesh_exporter("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::ObjExporter));
        semio_framework_os::register_mesh_exporter("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::GlbExporter));
        semio_framework_os::register_mesh_exporter("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::StlExporter));
        semio_framework_os::register_mesh_importer("5d.puzzle", puzzle5d_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
        semio_framework_os::register_mesh_importer("5d.puzzle", puzzle5d_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
        semio_framework_os::register_mesh_importer("5d.puzzle", puzzle5d_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
        semio_framework_os::register_mesh_dwg_export_handler("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")));
        semio_framework_os::register_mesh_dwg_import_handler("5d.puzzle", puzzle5d_document_from_mesh);
    }
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ The one puzzle5d-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
/// instead of re-deriving a store/dispatch/render scaffold of its own.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::{testkit, ActionMeta, InvocationResult, PluginApp, VcsDocumentApp, ViewState};

    pub type Puzzle5dApp = VcsDocumentApp<Puzzle5dPlayApp>;

    pub fn meta(actor: &str) -> ActionMeta {
        testkit::meta(actor)
    }

    pub fn app() -> Puzzle5dApp {
        testkit::new_app::<Puzzle5dPlayApp>()
    }

    /// 🧰️ A registry-backed app so kind discipline (View actions must emit no operations) and the
    /// utility contract are enforced exactly as in production.
    pub fn app_with_registry() -> Puzzle5dApp {
        testkit::new_app_with_registry::<Puzzle5dPlayApp>(create_puzzle5d_app)
    }

    /// 🧪️ B1: test-only replacement for the deleted `VcsDocumentApp::handle_action` app-dispatch path
    /// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
    /// `Self::Command` channel). Reconstructs the `Puzzle5dCommand` from the same
    /// `(action, args, window_id)` triple every pre-migration test already passed.
    pub fn dispatch(app: &mut Puzzle5dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
        // 🕰️ Framework-reserved verbs (undo/redo/checkpoint/…) stay on `handle_action`.
        if matches!(action, "undo" | "redo" | "checkpoint" | "alternative" | "revertToCommand" | "historyFilter" | "noteShellCommand" | "copy" | "cut" | "paste") {
            return app.handle_action(action, args, &meta("local"));
        }
        app.dispatch_typed(Puzzle5dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), &meta("local"))
    }

    /// 🖼️ The rendered body, as a JSON string — every panel/window assertion greps this value.
    pub fn render_body(app: &mut Puzzle5dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewState::default()).expect("render")).expect("serialize rendered node")
    }

    pub fn projection_of(app: &Puzzle5dApp) -> Value {
        app.projection().expect("projection").0
    }

    pub fn part_count(app: &Puzzle5dApp) -> usize {
        projection_of(app).get("parts").and_then(|value| value.as_array()).map_or(0, Vec::len)
    }

    pub fn first_part_id(app: &Puzzle5dApp) -> String {
        projection_of(app).get("parts").and_then(Value::as_array).and_then(|parts| parts.first()).and_then(|part| part.get("id")).and_then(Value::as_str).expect("first part id").to_string()
    }

    /// 🎯️ Top-level utility tag of a `WindowMeasure::Group` by id, or `None` when the group is absent.
    pub fn measure_group_tag(measures: &[WindowMeasure], group_id: &str) -> Option<Option<String>> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Group { id, active_utility_id, .. } if id == group_id => Some(active_utility_id.clone()),
            _ => None,
        })
    }

    /// 🔍️ Depth-first search for a `WindowMeasure::Slider`'s presence by id, descending into groups.
    pub fn has_measure_slider(measures: &[WindowMeasure], slider_id: &str) -> bool {
        measures.iter().any(|measure| match measure {
            WindowMeasure::Slider { id, .. } => id == slider_id,
            WindowMeasure::Group { children, .. } => has_measure_slider(children, slider_id),
            _ => false,
        })
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::testkit::*;
    use super::*;
    use protocol::OperationDiff;
    use semio_framework_plugin::{ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, PluginApp, UiMenuRef};

    //#region 🔖️Rendering
    #[test]
    fn renders_paired_board_and_world_scenes() {
        let mut app = app();
        assert!(render_body(&mut app, board2d::BODY_KEY).contains("board-2d"));
        assert!(render_body(&mut app, world3d::BODY_KEY).contains("world-3d"));
    }

    #[test]
    fn initial_projection_is_the_concrete_forest_document() {
        let app = app();
        assert_eq!(projection_of(&app).get("schema").and_then(|value| value.as_str()), Some(PUZZLE5D_SCHEMA));
        assert!(part_count(&app) > 0, "the concrete-forest default document ships with parts");
    }

    #[test]
    fn document_panel_renders() {
        let mut app = app();
        assert!(!render_body(&mut app, document_panel::BODY_KEY).is_empty());
    }
    //#endregion 🔖️Rendering

    //#region 🔖️ContextMenu
    /// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: the selection context menu stays a shallow,
    /// disclosed list (top-level verbs + a handful of taxonomy groups) rather than a flat wall of rows,
    /// and the known destructive `deleteSelection` action stays the trailing group's last item.
    #[test]
    fn context_menu_is_grouped_and_keeps_delete_selection_last() {
        let mut app = app_with_registry();
        let part_id = first_part_id(&app);
        dispatch(&mut app, "setSelection", Some(&json!({ "partIds": [part_id] })), None).expect("select part");
        let request = ContextMenuRequest {
            menu: UiMenuRef { id: "world3d".into(), args: None },
            surface: Some(ContextMenuSurfaceTarget { surface_id: world3d::WINDOW_KIND_ID.into(), kind: "world3d".into(), hits: vec![], selection: vec![ContextMenuSelectionGroup { domain: "part".into(), ids: vec![part_id] }], text: None }),
            window_instance_id: None,
            point: None,
        };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level context menu should stay progressively disclosed: {menu:?}");
        let last = menu.last().expect("selection context menu should not be empty");
        let last_is_destructive_leaf = last.action.as_deref() == Some("deleteSelection") && last.destructive == Some(true);
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.action.as_deref() == Some("deleteSelection") && child.destructive == Some(true));
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must stay last: {menu:?}");
    }
    //#endregion 🔖️ContextMenu

    //#region 🔖️Pack
    /// 📦️ `Puzzle5dPlayProjection`'s pack encoding round-trips through the same `(RecordSpec,
    /// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
    /// `serde_json::Value` bridge impls), reusing the default concrete-forest fixture.
    #[test]
    fn puzzle5d_play_projection_pack_round_trips() {
        let app = app();
        store::test_support::assert_dsl_pack_equivalence(&app.projection().expect("projection"));
    }
    //#endregion 🔖️Pack

    //#region 🔖️Operations
    #[test]
    fn set_active_example_swaps_the_document_and_undo_restores_it() {
        let mut app = app();
        let loaded = part_count(&app);
        assert!(loaded > 0);
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).expect("empty");
        assert_eq!(part_count(&app), 0, "empty example clears the parts");
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(part_count(&app), loaded, "undo restores the concrete-forest parts");
        app.handle_action("redo", None, &meta("local")).expect("redo");
        assert_eq!(part_count(&app), 0);
    }

    #[test]
    fn patch_fastener_updates_transform_offsets_and_undoes() {
        let mut app = app();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin (has fasteners)");
        let projection = projection_of(&app);
        let fastener_id = projection["fasteners"][0]["id"].as_str().expect("seeded fastener").to_string();
        dispatch(&mut app, "patchFastener", Some(&json!({ "fastenerId": fastener_id, "field": "gap", "value": 2.5 })), None).expect("patch gap");
        let after = projection_of(&app);
        let fastener = after["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
        assert_eq!(fastener["gap"], 2.5);
        assert_eq!(fastener["shift"], 0.0);
        dispatch(&mut app, "patchFastener", Some(&json!({ "fastenerId": fastener_id, "field": "rotation", "value": 30.0 })), None).expect("patch rotation");
        let after2 = projection_of(&app);
        let fastener2 = after2["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
        assert_eq!(fastener2["gap"], 2.5, "earlier gap edit must survive a later rotation edit");
        assert_eq!(fastener2["rotation"], 30.0);
        app.handle_action("undo", None, &meta("local")).expect("undo");
        let undone = projection_of(&app);
        let fastener3 = undone["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
        assert_eq!(fastener3["rotation"], 0.0, "undo restores the pre-rotation-edit value");
        assert_eq!(fastener3["gap"], 2.5, "undo of rotation edit must not also revert the earlier gap edit");
    }

    #[test]
    fn import_compose_kit_replaces_parts_and_fasteners_and_undoes_as_one_edit() {
        let mut app = app();
        let before_count = part_count(&app);
        let compose_design = json!({
            "id": "design-1",
            "name": "Imported Tower",
            "pieces": { "items": [
                { "id": "piece-a", "type": { "id": "type-x" }, "pose": { "center": { "u": 1.0, "v": 2.0 }, "plane": { "origin": { "x": 0.0, "y": 0.0, "z": 0.0 }, "xAxis": { "x": 1.0, "y": 0.0, "z": 0.0 }, "yAxis": { "x": 0.0, "y": 1.0, "z": 0.0 } } } },
                { "id": "piece-b", "type": { "id": "type-x" }, "pose": { "center": { "u": 3.0, "v": 4.0 }, "plane": { "origin": { "x": 1.0, "y": 1.0, "z": 1.0 }, "xAxis": { "x": 1.0, "y": 0.0, "z": 0.0 }, "yAxis": { "x": 0.0, "y": 1.0, "z": 0.0 } } } },
            ] },
            "connections": { "items": [
                { "id": "conn-1", "parent": { "piece": { "id": "piece-a" }, "connector": { "id": "c1" } }, "child": { "piece": { "id": "piece-b" }, "connector": { "id": "c2" } }, "gap": 0.5, "shift": 0.0, "rise": 0.0, "rotation": 0.0, "turn": 0.0, "tilt": 0.0 },
            ] },
        });
        dispatch(&mut app, "importComposeKit", Some(&json!({ "design": compose_design })), None).expect("import");
        assert_eq!(part_count(&app), 2);
        let projection = projection_of(&app);
        assert_eq!(projection["label"], "Imported Tower");
        assert_eq!(projection["fasteners"].as_array().unwrap().len(), 1);
        assert_eq!(projection["fasteners"][0]["gap"], 0.5);
        assert_eq!(projection["fasteners"][0]["source"], "piece-a:c1");
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(part_count(&app), before_count, "one undo restores the pre-import document");
    }
    //#endregion 🔖️Operations

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Puzzle5dOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s. Deliberately
    /// dispatches through a standalone typed `Puzzle5dStore` — NOT through `Puzzle5dPlayApp`/
    /// `Puzzle5dPlayProjection` (the `🔖️ValueBridge` `serde_json::Value` wrapper this app's real
    /// `DocumentApp` still uses) — since `Puzzle5dOperation`'s canonical `Operation<Puzzle5dProjection>`
    /// impl (not its `Operation<Value>` bridge impl) is what the CW7 law is about.
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::puzzle5d::spr::Puzzle5dStore;
        use crate::artifacts::puzzle5d::{Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, PUZZLE_5D_SCHEMA};
        use protocol::{DocumentId, Edit, SchemaId};
        use store::create_document_envelope;

        let mut store = Puzzle5dStore::new(create_document_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", Puzzle5dProjection::default(), None));
        let part = Puzzle5dPart { id: "p1".into(), part_kind: None, part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() };
        store.dispatch(store::DocumentCommand::Apply { operations: vec![Puzzle5dOperation::SetPart { index: 0, part }], description: None }).expect("apply");
        let edit: &Edit<Puzzle5dOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<Puzzle5dProjection, Puzzle5dOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests

    //#region 🔖️Clipboard
    #[test]
    fn copy_emits_clipboard_fragment_for_the_closed_selection() {
        let mut app = app();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
        let first_part_id = first_part_id(&app);
        dispatch(&mut app, "setSelection", Some(&json!({ "partIds": [first_part_id] })), None).expect("select");
        let result = app.handle_action("copy", None, &meta("local")).expect("copy");
        assert!(result.operations.is_empty(), "copy must not record an undo entry");
        assert_eq!(result.requested_effects.len(), 1);
        let HostEffect::ClipboardWrite { fragment } = &result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
        assert_eq!(fragment.source_app, PUZZLE5D_PLAY_APP_ID);
        let fragment_value: Value = serde_json::from_str(&fragment.dsl_text).expect("fragment dsl_text is JSON");
        assert_eq!(fragment_value["parts"].as_array().expect("parts").len(), 1);
    }

    #[test]
    fn copy_with_no_selection_is_a_benign_no_operation() {
        let mut app = app();
        let result = app.handle_action("copy", None, &meta("local")).expect("copy");
        assert!(result.operations.is_empty());
        assert!(result.requested_effects.is_empty());
    }

    #[test]
    fn cut_removes_selected_part_and_undo_restores_it() {
        let mut app = app();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
        let before_count = part_count(&app);
        let first_part_id = first_part_id(&app);
        dispatch(&mut app, "setSelection", Some(&json!({ "partIds": [first_part_id] })), None).expect("select");
        let result = app.handle_action("cut", None, &meta("local")).expect("cut");
        assert_eq!(result.requested_effects.len(), 1, "cut must also copy to the clipboard");
        assert_eq!(part_count(&app), before_count - 1);
        let after = projection_of(&app);
        assert!(!after["parts"].as_array().unwrap().iter().any(|part| part["id"] == first_part_id));
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(part_count(&app), before_count, "one undo restores the cut part as a single edit");
    }

    #[test]
    fn paste_materializes_fragment_parts_at_original_anchor_with_fresh_ids() {
        let mut app = app();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
        let projection = projection_of(&app);
        let first_part_id = first_part_id(&app);
        dispatch(&mut app, "setSelection", Some(&json!({ "partIds": [first_part_id] })), None).expect("select");
        let copy_result = app.handle_action("copy", None, &meta("local")).expect("copy");
        let HostEffect::ClipboardWrite { fragment } = &copy_result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
        let before_count = part_count(&app);
        let before_ids: HashSet<String> = projection["parts"].as_array().unwrap().iter().map(|part| part["id"].as_str().unwrap_or_default().to_string()).collect();
        let paste_args = json!({ "fragment": fragment, "anchor": "original", "position": [10.0, 0.0, 0.0] });
        app.handle_action("paste", Some(&paste_args), &meta("local")).expect("paste");
        assert_eq!(part_count(&app), before_count + 1);
        let after = projection_of(&app);
        let pasted_parts: Vec<&Value> = after["parts"].as_array().unwrap().iter().filter(|part| !before_ids.contains(part["id"].as_str().unwrap_or_default())).collect();
        assert_eq!(pasted_parts.len(), 1);
        // "original" anchor uses the raw position override verbatim as the 2D delta.
        let original_x = projection["parts"][0]["2d"]["x"].as_f64().unwrap_or(0.0);
        assert_eq!(pasted_parts[0]["2d"]["x"].as_f64().unwrap(), original_x + 10.0);
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(part_count(&app), before_count, "one undo removes the whole pasted fragment");
    }

    #[test]
    fn paste_with_no_fragment_arg_is_a_benign_no_operation() {
        let mut app = app();
        let before_count = part_count(&app);
        let result = app.handle_action("paste", None, &meta("local")).expect("paste");
        assert!(result.operations.is_empty());
        assert_eq!(part_count(&app), before_count);
    }
    //#endregion 🔖️Clipboard

    //#region 🔖️Manifest
    #[test]
    fn app_definition_has_the_paired_windows() {
        let app = create_puzzle5d_app();
        let ids: Vec<&str> = app.definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
        assert!(ids.contains(&board2d::WINDOW_KIND_ID) && ids.contains(&world3d::WINDOW_KIND_ID));
    }

    #[test]
    fn window_kind_actions_scope_transform_to_3d_only() {
        let definition = create_puzzle5d_app().definition;
        let resolve = |window_id: &str| -> Vec<String> {
            let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
            semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
        };
        let board = resolve(board2d::WINDOW_KIND_ID);
        let world = resolve(world3d::WINDOW_KIND_ID);
        for transform_operation in ["translateSelection", "rotateSelection", "scaleSelection", "worldRelocate", "setCamera3d"] {
            assert!(world.contains(&transform_operation.to_string()), "3D must expose {transform_operation}");
            assert!(!board.contains(&transform_operation.to_string()), "2D must NOT expose {transform_operation}");
        }
        assert!(board.contains(&"applyBoardEvents".to_string()), "2D must expose applyBoardEvents");
        assert!(!world.contains(&"applyBoardEvents".to_string()), "3D must NOT expose applyBoardEvents");
        for shared in ["addBrushPart", "deleteSelection"] {
            assert!(board.contains(&shared.to_string()) && world.contains(&shared.to_string()), "{shared} stays on both windows");
        }
    }

    /// 📑️ The three declared panel tabs must survive the `panel_tab_def` stitch. Asserts PRESENCE
    /// only — the framework injects tabs of its own, so a total count would be brittle.
    #[test]
    fn app_definition_declares_its_three_panel_tabs() {
        let definition = create_puzzle5d_app().definition;
        let body_keys: Vec<&str> = definition.panel_tabs.iter().filter_map(|tab| tab.body_key.as_deref()).collect();
        for body_key in [document_panel::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY] {
            assert!(body_keys.contains(&body_key), "panel tab {body_key} must be declared, got {body_keys:?}");
        }
    }

    #[test]
    fn window_engagements_cover_both_windows() {
        let mut app = app();
        let engagements = app.window_engagements();
        assert!(engagements.contains_key(board2d::WINDOW_KIND_ID));
        assert!(engagements.contains_key(world3d::WINDOW_KIND_ID));
    }

    /// 🎯️ Every action id `dispatch_puzzle5d_action` matches on must have a `Puzzle5dCommand` variant
    /// under the SAME literal — the two lists are the whole app's dispatch contract and drift between
    /// them is silent. (Deliberately NOT the framework's own
    /// `assert_declared_actions_bridge_to_commands`, which probes `command_from_action`, the
    /// string-dispatch path this app does not implement — its commands carry an opaque `args: Value`,
    /// see the `🔖️Puzzle5dCommand` macro.)
    #[test]
    fn every_dispatched_action_bridges_to_a_command() {
        for action in [
            "setFixtureJson",
            "setActiveExample",
            "importComposeKit",
            "setSelection",
            "documentSelect",
            "clearSelection",
            "selectAll",
            "selectSameKindSelection",
            "selectSameKind",
            "setSelectionMethod",
            "worldSelect",
            "worldPick",
            "addNode",
            "addPartKind",
            "deleteSelection",
            "duplicateSelection",
            "setSelectionFlag",
            "patchPart",
            "patchGrip",
            "patchFastener",
            "worldHover",
            "setHover",
            "worldVortexHover",
            "worldVortexSelect",
            "setCamera",
            "setCamera2d",
            "setCamera3d",
            "zoomToSelection",
            "focusSelection",
            "toggleSun",
            "setSunAzimuth",
            "setSunElevation",
            "setSunIntensity",
            "setLodMode",
            "setGridSnapEnabled",
            "setGridFactor",
            "addBrushPart",
            "addBrushObject",
            "cycleBrushCandidate",
            "registerBrushMesh",
            "setBrushPlacementOverlapBudget",
            "setObjectKindWeight",
            "setVortexKindWeight",
            "engagementControlSelect",
            "setSuggestionOffset",
            "setFillCount",
            "engagementInput",
            "engagementSubmit",
            "engagementAbort",
            "translateSelection",
            "rotateSelection",
            "scaleSelection",
            "worldRelocate",
            "applyBoardEvents",
            "worldPointerDown",
            "canvasPointerDown",
            SET_ACTIVE_UTILITY_ACTION_ID,
        ] {
            assert_eq!(Puzzle5dCommand::from_action(action, None, None).action_id(), action, "dispatched action {action} must have a Puzzle5dCommand variant");
        }
    }
    //#endregion 🔖️Manifest

    //#region 🧰️ Window Actions & Utilities contract
    #[test]
    fn add_part_kind_materializes_the_declared_kind_default() {
        // 📝️ P1 arg form: addPartKind with no args materializes the declared `partKind` default and adds a part.
        let mut app = app_with_registry();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).expect("empty");
        let before = part_count(&app);
        let result = dispatch(&mut app, "addPartKind", None, None).expect("addPartKind");
        assert!(!result.operations.is_empty(), "addPartKind is an Operation that emits operations");
        assert_eq!(part_count(&app), before + 1, "the materialized default kind adds exactly one part");
        let projection = projection_of(&app);
        let kind = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.last()).and_then(|part| part.get("partKind")).and_then(Value::as_str);
        assert_eq!(kind, Some("Part"), "the declared partKind default was materialized host-side");
    }

    #[test]
    fn set_active_utility_emits_no_ops_and_no_history_entry() {
        // 🧰️ Switching utilities is the framework View action: no document operations, no undo entry, no re-emitted effect.
        let mut app = app_with_registry();
        let before = projection_of(&app);
        let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "brush" })), None).expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document operations");
        assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
        assert_eq!(projection_of(&app), before, "utility switching does not mutate the document");
    }

    #[test]
    fn set_camera_actions_write_runtime_and_emit_no_operations() {
        // 📷️ Camera pose is session-only view state (`ActionKind::View`): `setCamera2d`/`setCamera3d`
        // must mutate the app's runtime (visible via the rendered scene) without ever touching the
        // VCS-tracked document or emitting an operation.
        let mut app = app();
        let before = projection_of(&app);
        let camera2d_result = dispatch(&mut app, "setCamera2d", Some(&json!({ "camera": { "x": 12.5, "y": -6.5, "zoom": 3.5 } })), None).expect("setCamera2d");
        assert!(camera2d_result.operations.is_empty(), "setCamera2d is a View action and must never emit a document operation");
        assert_eq!(projection_of(&app), before, "setCamera2d must not mutate the document");
        let board = render_body(&mut app, board2d::BODY_KEY);
        assert!(board.contains("12.5") && board.contains("-6.5"), "the new 2D camera pose must be reflected in the rendered runtime state");
        let camera3d_result = dispatch(&mut app, "setCamera3d", Some(&json!({ "camera": { "position": [42.5, 7.5, 3.5], "target": [1.5, 2.5, 3.5], "zoom": 5.5 } })), None).expect("setCamera3d");
        assert!(camera3d_result.operations.is_empty(), "setCamera3d is a View action and must never emit a document operation");
        assert_eq!(projection_of(&app), before, "setCamera3d must not mutate the document");
        let world = render_body(&mut app, world3d::BODY_KEY);
        assert!(world.contains("42.5") && world.contains("7.5") && world.contains("1.5"), "the new 3D camera pose must be reflected in the rendered runtime state");
    }

    #[test]
    fn engagements_expose_no_utility_switch_options_for_either_window() {
        // 🧰️ select/brush/fill switching lives only on the framework utility bar; neither the 2D nor the 3D
        // engagement HUD may duplicate it as options.
        let mut app = app();
        let engagements = app.window_engagements();
        for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
            assert!(engagements.get(window).expect("engagement").options.is_none(), "the {window} engagement must not re-expose utility switching as options");
        }
    }

    /// 🎯️ D-3 follow-up: the fill-count slider and brush placement picker are tagged `WindowMeasure::Group`s
    /// in each window's `window_measures` (surfaced by `partition_window_measures` only for their active
    /// utility), never `WindowEngagementControl`s on the HUD — for both the 2D and 3D windows.
    #[test]
    fn fill_and_brush_params_are_tagged_utility_options_not_engagement_controls() {
        let labels = puzzle5d_labels(&Puzzle5dConfig::default());
        let session = Puzzle5dPrecomputeSession::new();
        // 🪣️ Fill utility: the fill-count slider lives in a "fill"-tagged Utility Options group (per window),
        // NOT the engagement HUD.
        let fill_runtime = Puzzle5dRuntime { fill_count: 3, ..Default::default() };
        let fill_scene = Puzzle5dScene { document: default_document(), runtime: fill_runtime, active_utility: "fill".into() };
        for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
            let measures = if window == board2d::WINDOW_KIND_ID { board2d::window_measures(&fill_scene, &session, labels) } else { world3d::window_measures(&fill_scene, &session, labels) };
            assert_eq!(measure_group_tag(&measures, "puzzle5d-play-utility-options-fill"), Some(Some("fill".into())), "{window} fill Utility Options must be tagged for the fill utility");
            assert!(has_measure_slider(&measures, "puzzle5d-fill-count"), "{window} fill Utility Options must carry the fill-count slider");
            let fill_hud = edit::puzzle5d_engagement(&fill_scene, window, labels);
            assert!(fill_hud.control.is_none() && fill_hud.controls.is_none(), "{window} fill engagement HUD must no longer carry the relocated control");
        }
        // 🖌️ Brush utility: with no candidates to place, the "brush"-tagged group still surfaces (matching the
        // old gate), and the engagement HUD is likewise bare.
        let brush_scene = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: "brush".into() };
        for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
            let measures = if window == board2d::WINDOW_KIND_ID { board2d::window_measures(&brush_scene, &session, labels) } else { world3d::window_measures(&brush_scene, &session, labels) };
            assert_eq!(measure_group_tag(&measures, "puzzle5d-play-utility-options-brush"), Some(Some("brush".into())), "{window} brush Utility Options surfaces even without candidates");
            let brush_hud = edit::puzzle5d_engagement(&brush_scene, window, labels);
            assert!(brush_hud.control.is_none() && brush_hud.controls.is_none(), "{window} brush engagement HUD must no longer carry the relocated control");
        }
    }

    #[test]
    fn engagement_submit_switches_utility_via_host_effect_for_both_windows() {
        // 🧰️ Reconciled dual entry point: the engagement token drives the same host-owned utility switch, once per window.
        let mut app = app();
        let result = dispatch(&mut app, "engagementSubmit", Some(&json!({ "window": world3d::WINDOW_KIND_ID, "value": "brush" })), None).expect("submit");
        let windows: Vec<&str> = result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                HostEffect::SetActiveUtility { window_id, utility_id } if utility_id == "brush" => Some(window_id.as_str()),
                _ => None,
            })
            .collect();
        assert!(windows.contains(&board2d::WINDOW_KIND_ID) && windows.contains(&world3d::WINDOW_KIND_ID), "brush switch is pushed to both windows, got {windows:?}");
    }

    #[test]
    fn gumball_translate_drag_coalesces_into_one_edit() {
        // 🌀️ Coalescing regression: three translate ticks with the same key are ONE undoable edit.
        let mut app = app();
        let part_id = first_part_id(&app);
        let origin_x = |app: &Puzzle5dApp| -> f64 {
            projection_of(app)
                .get("parts")
                .and_then(Value::as_array)
                .and_then(|parts| parts.iter().find(|part| part.get("id").and_then(Value::as_str) == Some(part_id.as_str())).cloned())
                .and_then(|part| part.pointer("/3d/origin/0").and_then(Value::as_f64))
                .unwrap_or(0.0)
        };
        let start = origin_x(&app);
        for dx in [1.0, 2.0, 3.0] {
            dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [part_id], "dx": dx, "dy": 0.0, "dz": 0.0 })), None).expect("drag tick");
        }
        assert!((origin_x(&app) - start - 6.0).abs() < 1e-9, "three ticks accumulate 1+2+3 on x");
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert!((origin_x(&app) - start).abs() < 1e-9, "one undo restores the whole coalesced gumball drag");
    }
    //#endregion 🧰️ Window Actions & Utilities contract

    //#region 🔖️KitInPort
    /// 🔌️ The flagship `kit:in` seam: feeding a `kit.catalog` fragment shaped exactly like
    /// block3d's `puzzle3d_catalog_fragment` (`objectKinds`/`vortexKinds`, camelCase) through
    /// `Puzzle5dPlayApp::import_media` must normalize `objectKinds` into the typed
    /// `kindCatalogs.parts` (with each per-object `vortices[]` entry becoming a grip template) and
    /// `vortexKinds` into `kindCatalogs.grips`, and land both after applying the returned operations.
    #[test]
    fn kit_in_import_media_upserts_part_and_grip_kinds_into_kind_catalogs() {
        let app = Puzzle5dPlayApp::default();
        let projection = app.initial_projection();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &projection, history: &history };

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

        let emit = app.import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
        assert!(!emit.document_operations.is_empty(), "importing a non-empty fragment must emit real operations");

        let mut next_projection = projection.0.clone();
        for operation in &emit.document_operations {
            next_projection = protocol::Operation::<Value>::diff(operation, &next_projection).apply(&next_projection);
        }

        let parts = next_projection.pointer("/kindCatalogs/parts").and_then(Value::as_array).expect("parts catalog present");
        let capsule = parts.iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).expect("the imported part kind must appear in kindCatalogs.parts");
        assert_eq!(capsule.get("meshUrl").and_then(Value::as_str), Some("/mesh/capsule.glb"));
        assert_eq!(capsule.pointer("/grips/0/gripKind").and_then(Value::as_str), Some("door"), "the per-part grip template keeps its gripKind after normalization");
        assert_eq!(capsule.pointer("/grips/0/3d/position").and_then(Value::as_array), Some(&vec![json!(0.0), json!(0.0), json!(0.0)]));
        assert_eq!(capsule.pointer("/grips/0/3d/direction").and_then(Value::as_array), Some(&vec![json!(0.0), json!(1.0), json!(0.0)]));
        assert_eq!(capsule.pointer("/grips/0/3d/radius").and_then(Value::as_f64), Some(0.3));

        let grips = next_projection.pointer("/kindCatalogs/grips").and_then(Value::as_array).expect("grips catalog present");
        let door = grips.iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some("door")).expect("the imported grip kind must appear in kindCatalogs.grips");
        assert_eq!(door.get("defaultRopeKind").and_then(Value::as_str), Some(""), "defaultCableKind maps onto defaultRopeKind (a naming judgment call — see import_media's doc comment)");

        let compatibility = next_projection.pointer("/kindCompatibility").and_then(Value::as_array).expect("kind compatibility present");
        assert!(compatibility.iter().any(|entry| entry.get("source").and_then(Value::as_str) == Some("door") && entry.get("target").and_then(Value::as_str) == Some("door")));
    }

    /// 🔁️ Re-importing the SAME fragment (simulating a second producer edge, or a redelivered
    /// message on a `multiplicity: Many` port) must upsert idempotently — no duplicate rows.
    #[test]
    fn kit_in_import_media_is_idempotent_on_repeated_delivery() {
        let app = Puzzle5dPlayApp::default();
        let projection = app.initial_projection();
        let history = semio_framework_plugin::HistoryView::empty();
        let mut current = projection.0;

        let fragment = json!({
            "objectKinds": [{ "id": "capsule", "name": "capsule", "label": "Capsule", "meshUrl": "/mesh/capsule.glb", "vortices": [] }],
            "vortexKinds": [],
            "cableKinds": [],
            "attractionKinds": [],
            "kindCompatibility": [],
        });
        let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

        for _ in 0..2 {
            let doc_projection = Puzzle5dPlayProjection(current.clone());
            let doc = DocumentView { projection: &doc_projection, history: &history };
            let emit = app.import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
            for operation in &emit.document_operations {
                current = protocol::Operation::<Value>::diff(operation, &current).apply(&current);
            }
        }

        let parts = current.pointer("/kindCatalogs/parts").and_then(Value::as_array).expect("parts catalog present");
        assert_eq!(parts.iter().filter(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).count(), 1, "repeated delivery of the same fragment must upsert, never duplicate");
    }

    #[test]
    fn kit_in_port_is_declared_on_the_app_io() {
        let app = Puzzle5dPlayApp::default();
        let io = app.io().expect("puzzle5d declares an AppIo");
        let kit_in = io.ports.iter().find(|port| port.id == "kit:in").expect("kit:in port declared");
        assert_eq!(kit_in.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(kit_in.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
        assert!(matches!(kit_in.multiplicity, PortMultiplicity::Many));
        let design_out = io.ports.iter().find(|port| port.id == "design:out").expect("design:out port declared");
        assert_eq!(design_out.kind_id.as_deref(), Some("5d.puzzle"));
        assert_eq!(design_out.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Design });
        assert!(matches!(design_out.multiplicity, PortMultiplicity::Many));
    }
    //#endregion 🔖️KitInPort
}
//#endregion 🧪️Tests
