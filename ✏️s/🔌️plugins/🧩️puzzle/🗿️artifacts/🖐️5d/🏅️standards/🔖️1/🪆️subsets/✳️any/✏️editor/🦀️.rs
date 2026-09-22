//! 👯️ Puzzle 5d play app — the plugin's unified 2d+3d play app: its `ArtifactApp` impl
//! (dispatch-only), the structural-twin document model its command/panel/window nodes mutate and
//! render, the shared scene/engine/brush helpers those nodes reach for, and the manifest that
//! stitches them together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/{◻2d,🧊️3d}`. This file dispatches and stitches.
//!
//! 🌉️ `ArtifactApp::Snapshot` is the `Puzzle5dPlaySnapshot` newtype over a bare
//! `serde_json::Value` document (see `crate::standards::v1::subsets::any::schema::mutations::text`'s `🔖️ValueBridge`), not the
//! typed `Puzzle5dSnapshot` — the `Puzzle5dDocument` model below is this app's own structural twin
//! of it, and each action emits the granular typed operation delta
//! (`puzzle5d_operations_from_document_change`) turning the old document into the new one.

use crate::standards::v1::subsets::any::schema::mutations::text::{puzzle5d_document_delta_operations, Puzzle5dMutation, Puzzle5dPlaySnapshot};
use crate::Puzzle5dSnapshot;
use crate::editor::puzzle5d::commands::{
    add_brush_part, add_node, add_part_kind, apply_board_events, apply_sun, create_fastener, cycle_brush_candidate, delete_fastener, delete_selection, duplicate_selection, edit_fastener, engagement_abort, engagement_control_select, engagement_input,
    engagement_repeat_last, engagement_submit, patch_fastener, patch_grip, patch_part, proximity_connect, register_brush_mesh, retarget_fastener, rotate_selection, scale_selection, select_same_kind, set_active_example, set_brush_placement_contact_tolerance,
    set_camera, set_camera_2d, set_camera_3d, set_fill_count, set_grid_factor, set_grid_snap_enabled, set_kind_weight, set_lod_mode, set_selection_flag, set_suggestion_offset, target_brush_suggestions, translate_selection, world_relocate,
};
use crate::editor::puzzle5d::commands::focus_selection;
use crate::editor::puzzle5d::commands::{
    set_grid_spacing, set_grid_visible, set_grip_direction, set_grip_show, set_lod_automatic, set_lod_depth_variable, set_lod_manual, set_projection, set_selectable_kind, set_transform_gumball_flag,
};
use crate::editor::puzzle5d::commands::{add_target_volume, delete_target_volume, relocate_target_volume, set_target_volume_flag, set_voxel_dims};
use crate::editor::puzzle5d::commands::{set_chunk_size, set_proximity_radius};
use crate::editor::puzzle5d::commands::{export_fixture, import_fixture, open_add_part_dialog, open_import_fixture};
use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dConfig, Puzzle5dConfigMutation, Puzzle5dRuntime};
use crate::editor::puzzle5d::modes::edit;
use crate::editor::puzzle5d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle5d::modes::edit::windows::{board2d, world3d};
use crate::editor::puzzle5d::panels::{catalogue, artifact as artifact_panel, inspection, settings as settings_panel};
use semio_s_artifact_puzzle_3d::editor::puzzle3d::Puzzle3dInstanceOperationOwner;
use crate::editor::puzzle5d::presence::{Puzzle5dPresence, Puzzle5dPresenceMutation};
use crate::editor::puzzle5d::terminology::{puzzle5d_is_de_locale, puzzle5d_labels, puzzle5d_localized, Puzzle5dLabels};
use crate::editor::puzzle5d::window as window_ownership;
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PasteAnchor, PastePlacement, UiDirtyScope};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppIo, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactPresentation, ArtifactReservedJob, ArtifactReservedToolInput, ArtifactReservedToolJob,
    ArtifactReservedToolJobRequest, ArtifactToolCompletion, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, DraftView, Editor, EditorApp, Emit, EphemeralEmit, Fault,
    DialogDefinition, GranularityDefinition, HierarchyProvider, HoverSpec,
    InteractionDefinition, InteractionRef, InteractionTarget, InteractionWrite, InteractiveJobClassification, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, MergeMode, NoDraft, NoDraftMutation,
    PluginCloseStep, PortMultiplicity, ToolRef, SelectionMethod, SelectionMode, SelectionSpec, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, WindowEngagement, WindowMeasure, INTERACTION_SELECT_ACTION_ID,
};
// 🕹️ `InteractionView` — see 🧊️3d/🦀️.rs's identical import comment (missing top-level
// re-export from `semio_framework_plugin`, flagged to the coordinator, not fixed here).
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, JobFault, JobPayloadAdmissionFault, JobPayloadCloseStep, JobPayloadStream, Operation, RetainedJobPayload, RetainedJobPayloadWriter, StepContext, StepOutcome};
use semio_framework_plugin::app::{ArtifactToolCompletionRejection, InteractionView};
use serde::{Deserialize, Serialize};
use dsl::os_pack::json::{parse, Value};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use store::EngineHandles;

//#region 🔖️Constants
pub const PUZZLE5D_PLAY_APP_ID: &str = "puzzle5d-play";
pub const PUZZLE5D_PLAY_CONTROLLER_ID: &str = "puzzle5d-play";
pub const PUZZLE5D_PLAY_WINDOWS: [&str; 2] = [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID];
pub const PUZZLE5D_SCHEMA: &str = "puzzle.5d";
pub const PUZZLE5D_BOARD_FIXTURE_SCHEMA: &str = "puzzle.2d.fixture";
pub const PUZZLE5D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";
pub const PUZZLE5D_EXAMPLE_NAKAGIN: &str = "nakagin-capsule-tower";
pub const PUZZLE5D_EXAMPLE_CAPSULE_DREAM: &str = "capsule-dream";

pub const PUZZLE5D_FALLBACK_MESH_KIND: &str = "box";
/// 🧰️ Active utility fallback when the host `ViewModel` has not selected one yet.
pub const PUZZLE5D_DEFAULT_UTILITY: &str = "select";
/// 🎯️ The fill count a fresh 5d document offers. There is no ceiling to pair it with: the wrapped 3d
/// planner plans toward whatever the operator types and reports a stall reason when the document
/// cannot hold that many.
pub const PUZZLE5D_DEFAULT_FILL_COUNT: u32 = 100;
pub const PUZZLE5D_LOD_MODE_AUTOMATIC: &str = "automatic";
pub const PUZZLE5D_SUGGESTION_OFFSET_MIN: f64 = 0.0;
pub const PUZZLE5D_SUGGESTION_OFFSET_MAX: f64 = 160.0;
pub const PUZZLE5D_SUGGESTION_OFFSET_STEP: f64 = 4.0;
pub const PUZZLE5D_DEFAULT_SUGGESTION_OFFSET: f64 = 80.0;
pub const PUZZLE5D_DEFAULT_PART_RADIUS: f64 = 20.0;
pub const PUZZLE5D_BOARD_PLACEMENT_GAP: f64 = 16.0;
pub const PUZZLE5D_PROXIMITY_RADIUS: f64 = 0.75;
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the one interaction domain this app
/// declares — the previously-separate `Puzzle5dSelection` bags (part/grip/fastener) collapse into
/// one framework-owned domain, distinguished by `DomainSelection.granularity`.
pub const PUZZLE5D_INTERACTION_DOMAIN: &str = "vortex";
pub const PUZZLE5D_GRANULARITY_PART: &str = "part";
pub const PUZZLE5D_GRANULARITY_GRIP: &str = "grip";
pub const PUZZLE5D_GRANULARITY_FASTENER: &str = "fastener";
/// 🧊️ The granularity a target-volume row picks with — puzzle 3d's own `targetVolume` spelling, so the
/// two artifacts' outliners, inspectors and hosts read one vocabulary.
pub const PUZZLE5D_GRANULARITY_TARGET_VOLUME: &str = "targetVolume";
/// 🐁️ The pointer hover channel of the `vortex` domain — the one channel both panes paint from.
pub const PUZZLE5D_HOVER_CHANNEL: &str = "pointer";

/// 🤏️ Grip marker emission modes (`setGripShow`).
pub const PUZZLE5D_GRIP_SHOW_ALWAYS: &str = "always";
pub const PUZZLE5D_GRIP_SHOW_SELECTED: &str = "selected";
/// 🧭️ Grip direction arrow modes (`setGripDirection`).
pub const PUZZLE5D_GRIP_DIRECTION_OUTWARDS: &str = "outwards";
pub const PUZZLE5D_GRIP_DIRECTION_INWARDS: &str = "inwards";
/// 🔭️ The band the world pane's manual LOD slider — and `setLodManual` — clamps against.
pub const PUZZLE5D_LOD_SLIDER_MIN: f64 = 0.0;
pub const PUZZLE5D_LOD_SLIDER_MAX: f64 = 1000.0;
/// 🌐️ The band `setGridSpacing` clamps the world pane's grid pitch (m) into.
pub const PUZZLE5D_GRID_SPACING_MIN: f64 = 0.5;
pub const PUZZLE5D_GRID_SPACING_MAX: f64 = 50.0;
/// 🧊️ The band `setVoxelDims` clamps each Volume-Brush voxel axis (in grid-spacing units) into, and
/// the extent one Alt+click paints before the operator moves a slider.
pub const PUZZLE5D_VOXEL_DIM_MIN: f64 = 1.0;
pub const PUZZLE5D_VOXEL_DIM_MAX: f64 = 64.0;
pub const PUZZLE5D_DEFAULT_VOXEL_DIMS: [u32; 3] = [4, 4, 4];
/// 🎨️ The wire colour the world pane paints a target volume in — the same pink puzzle 3d uses, so an
/// operator reads one constraint the same way in either artifact.
pub const PUZZLE5D_TARGET_VOLUME_COLOR: &str = "#f472b6";
/// 🆔️ The id stem `addTargetVolume` mints from, shared with the part/fastener id counter.
pub const PUZZLE5D_TARGET_VOLUME_ID_STEM: &str = "target-volume";

/// 📐️ The band `setGridFactor` clamps the board pane's snap factor into.
pub const PUZZLE5D_GRID_FACTOR_MIN: f64 = 0.25;
pub const PUZZLE5D_GRID_FACTOR_MAX: f64 = 16.0;
/// 📡️ The ceiling `setProximityRadius` clamps the auto-connect distance (m) to — past this every part
/// in a normal document would be a candidate neighbour and a drop would connect the wrong grip.
pub const PUZZLE5D_PROXIMITY_RADIUS_MAX: f64 = 25.0;
/// 🧱️ The band `setChunkSize` clamps the broad-phase chunk edge (m) into. The floor is a real minimum:
/// a zero edge buckets the whole document into one cell and the placement search degenerates.
pub const PUZZLE5D_CHUNK_SIZE_MIN: f64 = 0.25;
pub const PUZZLE5D_CHUNK_SIZE_MAX: f64 = 512.0;

/// 🌉️ This app's own scratch fixture stays a local structural-twin mirror (`Puzzle5dDocument`) of
/// `crate::Puzzle5dSnapshot` — see that artifact's `🔖️ValueBridge` region — so
/// the DSL-text example fixtures are parsed once into the typed projection and re-serialized to the
/// JSON string this module's `document_from_json`/`.example(...)` call sites expect.
pub static CONCRETE_FOREST_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| crate::examples::puzzle5d::concrete_forest::SOURCE.document_json().to_string());
pub static NAKAGIN_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| crate::examples::puzzle5d::nakagin_capsule_tower::SOURCE.document_json().to_string());
pub static CAPSULE_DREAM_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| crate::examples::puzzle5d::capsule_dream::SOURCE.document_json().to_string());
static CONCRETE_FOREST_EXAMPLE_DOCUMENT: LazyLock<Puzzle5dDocument> = LazyLock::new(|| document_from_json(CONCRETE_FOREST_EXAMPLE_JSON.as_str()));
static NAKAGIN_EXAMPLE_DOCUMENT: LazyLock<Puzzle5dDocument> = LazyLock::new(|| document_from_json(NAKAGIN_EXAMPLE_JSON.as_str()));
static CAPSULE_DREAM_EXAMPLE_DOCUMENT: LazyLock<Puzzle5dDocument> = LazyLock::new(|| document_from_json(CAPSULE_DREAM_EXAMPLE_JSON.as_str()));
static EMPTY_EXAMPLE_DOCUMENT: LazyLock<Puzzle5dDocument> = LazyLock::new(empty_document);


const PUZZLE5D_RESERVED_RAW_BYTES: usize = 65_536;
const PUZZLE5D_RESERVED_OUTPUT_BYTES: usize = 1_048_576;
const PUZZLE5D_RESERVED_PAGE_BYTES: usize = 4_096;
const PUZZLE5D_IMPORT_MEDIA_BYTES: usize = semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
const PUZZLE5D_IMPORT_SEMANTIC_ITEMS: usize = 32;
const PUZZLE5D_IMPORT_DECODED_ITEMS: usize = PUZZLE5D_IMPORT_SEMANTIC_ITEMS * PUZZLE5D_IMPORT_SEMANTIC_ITEMS + PUZZLE5D_IMPORT_SEMANTIC_ITEMS * 5;
const PUZZLE5D_IMPORT_MUTATION_ITEMS: usize = PUZZLE5D_IMPORT_SEMANTIC_ITEMS * 2 + 1;
const PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE: usize = semio_framework_job::JOB_PAYLOAD_PAGE_BYTES / size_of::<Puzzle5dMutation>();
const PUZZLE5D_IMPORT_MUTATION_PAGES: usize = PUZZLE5D_IMPORT_MUTATION_ITEMS.div_ceil(PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);

pub fn puzzle5d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PUZZLE5D_PLAY_CONTROLLER_ID.into(), action: action.into(), args: args.map(|value| dsl::os_pack::json::to_dsl_value(&value)) }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: builds a framework `interactionSelect`
/// action targeting one `(granularity, id)` pair in the `vortex` domain — replaces the deleted
/// `setSelection` action builders every document tree row used to construct by hand.
pub fn puzzle5d_interaction_select(granularity: &str, id: &str) -> ActionDescriptor {
    let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
    puzzle5d_action(INTERACTION_SELECT_ACTION_ID, Some(dsl::json!({ "domainId": PUZZLE5D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })))
}

#[derive(Clone, Debug, Default)]
pub struct Puzzle5dFreshIds {
    occupied_parts: HashSet<String>,
    occupied_fasteners: HashSet<String>,
    occupied_target_volumes: HashSet<String>,
    part_cursor: u64,
    fastener_cursor: u64,
    target_volume_cursor: u64,
}

impl Puzzle5dFreshIds {
    pub fn from_document(document: &Puzzle5dDocument) -> Self {
        Self {
            occupied_parts: document.parts.iter().map(|part| part.id.clone()).collect(),
            occupied_fasteners: document.fasteners.iter().map(|fastener| fastener.id.clone()).collect(),
            occupied_target_volumes: document.target_volumes.iter().map(|volume| volume.id.clone()).collect(),
            ..Self::default()
        }
    }

    pub fn observe_part(&mut self, id: &str) {
        self.occupied_parts.insert(id.to_string());
    }

    pub fn observe_fastener(&mut self, id: &str) {
        self.occupied_fasteners.insert(id.to_string());
    }

    pub fn next_part(&mut self) -> String {
        next_scoped_id("part", &mut self.part_cursor, &mut self.occupied_parts)
    }

    pub fn next_fastener(&mut self) -> String {
        next_scoped_id("fastener", &mut self.fastener_cursor, &mut self.occupied_fasteners)
    }

    /// 🧊️ The next free `target-volume-<n>` id — minted off the document so a repainted volume never
    /// shadows one the operator already placed.
    pub fn next_target_volume(&mut self) -> String {
        next_scoped_id(PUZZLE5D_TARGET_VOLUME_ID_STEM, &mut self.target_volume_cursor, &mut self.occupied_target_volumes)
    }
}

fn next_scoped_id(prefix: &str, cursor: &mut u64, occupied: &mut HashSet<String>) -> String {
    loop {
        *cursor = cursor.saturating_add(1);
        let candidate = format!("{prefix}-{cursor}");
        if occupied.insert(candidate.clone()) {
            return candidate;
        }
    }
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Puzzle5dPartAnchor {
    #[default]
    Fixed,
    Derived,
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
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
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
    pub scale: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dPart {
    pub id: String,
    #[serde(rename = "partKind")]
    pub part_kind: String,
    #[serde(default)]
    pub anchor: Puzzle5dPartAnchor,
    #[serde(default, rename = "2d")]
    pub part_2d: Puzzle5dPart2d,
    #[serde(default, rename = "3d")]
    pub part_3d: Puzzle5dPart3d,
    #[serde(default)]
    pub grips: Vec<Puzzle5dGrip>,
}

/// 🧊️ One oriented box constraining where the fill planner may place, in the document's 3D pose
/// space. The Volume Brush paints grid-snapped axis-aligned instances sized by the world window's
/// voxel dims; the transform gumball edits arbitrary oriented boxes through `relocateTargetVolume`.
/// The board pane paints the flat rectangle this box projects to, never a second persisted pose.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dTargetVolume {
    pub id: String,
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<serde_json::Value>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub locked: bool,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_volumes: Vec<Puzzle5dTargetVolume>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    #[serde(default, rename = "kindCatalogs", skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<serde_json::Value>,
    #[serde(default, rename = "kindCompatibility", skip_serializing_if = "Option::is_none")]
    pub kind_compatibility: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🌉️ Hand-written, not derived: `value_derive::FromValue` cannot expand over this struct's
/// `Option<serde_json::Value>` fields (`meta`/`kindCatalogs`/`kindCompatibility` stay the raw
/// document JSON, matching `Puzzle5dPlaySnapshot`'s own still-`serde_json::Value` boundary), so this
/// routes through the framework's pre-existing `DslValue -> serde_json::Value` bridge and the
/// struct's own unconditional `Deserialize` instead. Needed because
/// `🎮️commands/🧪️set-fixture-json` round-trips this type through `dsl::os_pack::json::from_json_str`.
impl dsl::FromValue for Puzzle5dDocument {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        serde_json::from_value(serde_json::Value::from(&value)).map_err(|error| dsl::ValueError::new(error.to_string()))
    }
}

pub fn empty_document() -> Puzzle5dDocument {
    Puzzle5dDocument { schema: PUZZLE5D_SCHEMA.into(), domain: "architecture".into(), parts: Vec::new(), fasteners: Vec::new(), target_volumes: Vec::new(), meta: None, kind_catalogs: None, kind_compatibility: None, label: None }
}

/// 📥️ Decodes a SHIPPED example's JSON into the editor twin. A failure here is a build defect in the
/// example asset, never user input, so it is loud: the silent `empty_document()` fallback this used to
/// carry is what let the 2026-09-17 example regression ship a zero-part Nakagin and Capsule Dream.
/// User-supplied JSON arrives through `📥️import-fixture`, which refuses with a notice instead.
pub fn document_from_json(json_text: &str) -> Puzzle5dDocument {
    serde_json::from_str::<Puzzle5dDocument>(json_text).unwrap_or_else(|error| panic!("puzzle5d example document decodes: {error}"))
}

pub fn concrete_forest_example_document() -> Puzzle5dDocument {
    CONCRETE_FOREST_EXAMPLE_DOCUMENT.clone()
}

pub fn nakagin_example_document() -> Puzzle5dDocument {
    NAKAGIN_EXAMPLE_DOCUMENT.clone()
}

pub fn capsule_dream_example_document() -> Puzzle5dDocument {
    CAPSULE_DREAM_EXAMPLE_DOCUMENT.clone()
}

pub fn default_document() -> Puzzle5dDocument {
    concrete_forest_example_document()
}

/// 🌉️ `Puzzle5dPlaySnapshot`'s inner `.0` (owned by `🧬️mutations/🦀️.rs`, out of this ticket's
/// scope, and 5d has no `.typed()`/`.value()` accessor there the way 3d's sibling does — an
/// asymmetry worth closing in that file, not this one) still projects the bare document as
/// `serde_json::Value`; bridges it into this file's own first-party `Value` via `DslValue` without
/// ever naming the foreign crate: `T`'s only real caller is `&snapshot.0: &serde_json::Value`,
/// resolved structurally through the `DslValue: From<T>` bound rather than a spelled-out type —
/// mirrors `🧊️3d/…/✏️editor/🦀️.rs`'s `puzzle3d_projection_value`.
/// 🔎️ Reads a fixed-length `[f64; 3]` coordinate triple straight off a dsl `Value::Array` —
/// the direct replacement for the old `serde_json::from_value::<[f64; 3]>(value.clone())` round
/// trip, which this file's own `Value` no longer supports (no `Deserialize`).
pub(crate) fn puzzle5d_value_as_f64_3(value: &Value) -> Option<[f64; 3]> {
    let values = value.as_array()?;
    Some([values.first()?.as_f64()?, values.get(1)?.as_f64()?, values.get(2)?.as_f64()?])
}

/// 🔎️ The `[f64; 4]` sibling of [`puzzle5d_value_as_f64_3`] (quaternion orientation reads).
fn puzzle5d_value_as_f64_4(value: &Value) -> Option<[f64; 4]> {
    let values = value.as_array()?;
    Some([values.first()?.as_f64()?, values.get(1)?.as_f64()?, values.get(2)?.as_f64()?, values.get(3)?.as_f64()?])
}

fn puzzle5d_projection_value<T>(value: T) -> Value
where
    dsl::DslValue: From<T>,
{
    dsl::os_pack::json::from_dsl_value(&dsl::DslValue::from(value))
}

/// 🌉️ `puzzle5d_document_delta_operations` (owned by `🧬️mutations/🦀️.rs`, out of this ticket's
/// scope) is typed against `serde_json::Value`; bridges this file's own first-party `Value` through
/// `DslValue` at that one boundary, inferring the foreign return type from the callee's own
/// signature via `Into` rather than naming it here.
fn puzzle5d_operations_from_values(before: &Value, after: &Value) -> Vec<Puzzle5dMutation> {
    let before_dsl = dsl::os_pack::json::to_dsl_value(before);
    let after_dsl = dsl::os_pack::json::to_dsl_value(after);
    puzzle5d_document_delta_operations(&(&before_dsl).into(), &(&after_dsl).into())
}

/// 🌉️ `Puzzle5dDocument` stays hand-written `Serialize`/`Deserialize` (see its own doc comment
/// above), so a document becomes this file's first-party `Value` through `serde_json::to_value`
/// then the framework's `DslValue` bridge, matching `Puzzle5dDocument::from_value`'s own reverse
/// route.
fn value_from_document(document: &Puzzle5dDocument) -> Value {
    let serde_value = serde_json::to_value(document).unwrap_or(serde_json::Value::Null);
    dsl::os_pack::json::from_dsl_value(&dsl::DslValue::from(&serde_value))
}

/// 🧮️ Document operations for a document mutation through the typed semantic delta vocabulary.
///
/// 🐛️ This used to consult `PUZZLE5D_EXAMPLE_OPERATIONS`, a `LazyLock` 4×4 matrix of the PAIRWISE
/// deltas between all four shipped example documents, and fall back to computing the delta only on a
/// miss. Every entry of that matrix held a full `before: Value` AND a full `after: Puzzle5dDocument`
/// clone, and `capsule-dream` is 2 880 parts / ~3.5 MB of JSON — so the FIRST document-changing
/// command of a process paid: four example documents decoded (3 035 200 B of DSL for capsule-dream
/// alone), four `Value` projections, sixteen semantic diffs (several of them empty ↔ 2 880 parts),
/// and thirty-two deep clones. Then EVERY later call linearly scanned those sixteen entries
/// comparing `before` by DEEP `Value` equality against a multi-megabyte value.
///
/// It bought nothing: `entry.operations` was computed by exactly this `puzzle5d_operations_from_values`
/// call on exactly these two values, so the memo could only ever return what the direct computation
/// returns. Removing it is bit-identical in output and turns an O(examples²) startup cliff plus an
/// O(document) per-call scan into one O(document) delta. It is what made every app-creating puzzle5d
/// law run past 60 s and the whole test binary die to the 30-minute watchdog, and it is the same
/// cliff on the live `setActiveExample` path.
pub fn puzzle5d_operations_from_document_change(before: &Value, after_document: &Puzzle5dDocument) -> Vec<Puzzle5dMutation> {
    let after = value_from_document(after_document);
    puzzle5d_operations_from_values(before, &after)
}

fn puzzle5d_patch_fastener_operations(before: &Value, after_document: &Puzzle5dDocument, args: Option<&Value>) -> Vec<Puzzle5dMutation> {
    let field = args.and_then(|value| value.get("field")).and_then(Value::as_str).unwrap_or("");
    let mut ids = HashSet::new();
    for id in args.and_then(|value| value.get("fastenerIds")).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str) {
        ids.insert(id);
    }
    if let Some(id) = args.and_then(|value| value.get("fastenerId")).and_then(Value::as_str) {
        ids.insert(id);
    }
    let before_fasteners = before.get("fasteners").and_then(Value::as_array);
    let mut operations = Vec::new();
    for fastener in after_document.fasteners.iter().filter(|fastener| ids.contains(fastener.id.as_str())) {
        let previous = before_fasteners.and_then(|entries| entries.iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some(fastener.id.as_str())));
        if field == "fastenerKind" {
            let old = previous.and_then(|entry| entry.get("fastenerKind")).and_then(Value::as_str);
            if old != fastener.fastener_kind.as_deref() {
                operations.push(crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind::change_fastener_kind(fastener.id.clone(), fastener.fastener_kind.clone()));
            }
        } else if matches!(field, "gap" | "shift" | "rise" | "rotation" | "turn" | "tilt" | "x" | "y") {
            let old = previous.and_then(|entry| entry.get(field)).and_then(Value::as_f64).unwrap_or(0.0);
            let new = match field {
                "gap" => fastener.gap,
                "shift" => fastener.shift,
                "rise" => fastener.rise,
                "rotation" => fastener.rotation,
                "turn" => fastener.turn,
                "tilt" => fastener.tilt,
                "x" => fastener.x,
                "y" => fastener.y,
                _ => old,
            };
            if old != new {
                operations.push(crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::replace_fastener_geometry(crate::standards::v1::subsets::any::schema::mutations::ReplaceFastenerGeometry { id: fastener.id.clone(), new_gap: fastener.gap, new_shift: fastener.shift, new_rise: fastener.rise, new_rotation: fastener.rotation, new_turn: fastener.turn, new_tilt: fastener.tilt, new_x: fastener.x, new_y: fastener.y }));
            }
        }
    }
    operations
}

/// 🪟️ B1: puzzle5d has exactly two window KINDS (2D and 3D), each single-instance — unlike puzzle3d's
/// split top/perspective panes (two INSTANCES of one kind), puzzle5d's own dispatch never distinguishes
/// a window instance id from its kind id (every action matches the literal kind id via
/// `PUZZLE5D_PLAY_WINDOWS.contains(&window)`), so this needs none of `Puzzle3dConfig`'s self-maintained
/// `window_ids`/`load_window`/`save_window` machinery — each kind's sole instance id is the kind id
/// itself. Kept as a named helper (rather than inlining `vec![kind_id.to_string()]`) purely so
/// `window_engagements`/`window_measures` read the same "one entry per live window instance" shape
/// `ArtifactApp`'s doc comment describes, and so a future genuine multi-instance need has one seam to extend.
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

/// 📏️ Settings counterpart to [`puzzle5d_resolve_number_edit`]: reads `value`/`delta` straight out of
/// an action's `args`, for the single global/window settings whose slider or stepper dispatches to its
/// own dedicated action (puzzle 3d's `puzzle3d_absolute_or_delta`).
pub fn puzzle5d_absolute_or_delta(args: Option<&Value>, current: f64) -> Option<f64> {
    puzzle5d_resolve_number_edit(current, args.and_then(|value| value.get("value")), args.and_then(|value| value.get("delta")))
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

pub fn resolve_part_mesh_url(part: &Puzzle5dPart, kind_catalogs: Option<&serde_json::Value>) -> Option<String> {
    if let Some(url) = part.part_3d.mesh_url.as_ref().filter(|url| !url.is_empty()) {
        return Some(url.clone());
    }
    resolve_part_kind_mesh_url(&part.part_kind, kind_catalogs)
}

pub fn resolve_part_kind_mesh_url(part_kind: &str, kind_catalogs: Option<&serde_json::Value>) -> Option<String> {
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

fn part_kind_grip_templates(document: &Puzzle5dDocument, part_kind: &str) -> Vec<serde_json::Value> {
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

pub fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| <Vec<String> as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(value)).ok()).filter(|ids| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
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
        Some(serde_json::Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        Some(serde_json::Value::Number(value)) => {
            let factor = value.as_f64().unwrap_or(1.0);
            [factor, factor, factor]
        }
        _ => [1.0, 1.0, 1.0],
    }
}

/// 🧊️ One target volume's world extent as an `[x, y, z]` triple — the `Puzzle5dScale` union read the
/// same way `part_scale_json` reads a part's, so a volume painted uniformly and one scaled per axis
/// both reach the world host as three numbers.
pub fn target_volume_scale_json(volume: &Puzzle5dTargetVolume) -> [f64; 3] {
    match &volume.scale {
        Some(serde_json::Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        Some(serde_json::Value::Number(value)) => {
            let factor = value.as_f64().unwrap_or(1.0);
            [factor, factor, factor]
        }
        _ => [1.0, 1.0, 1.0],
    }
}

/// 📐️ The flat rectangle a target volume constrains on the board — `[x, y, width, height]` in board
/// units, centred on the volume's own centre. The projection is the SAME linear board↔world map
/// `add_palette_part` and `🧬️schema/💡️inferences/🎛️flat-position` place paired parts with
/// ([`PUZZLE5D_FLAT_TO_WORLD`], world XY ground plane, board Y pointing the other way), so a part the
/// planner placed inside the box lands inside the painted rectangle — never a second persisted pose.
pub fn target_volume_flat_rect(volume: &Puzzle5dTargetVolume) -> [f64; 4] {
    let extent = target_volume_scale_json(volume);
    let to_flat = 1.0 / PUZZLE5D_FLAT_TO_WORLD;
    [volume.origin[0] * to_flat, -volume.origin[1] * to_flat, extent[0].abs() * to_flat, extent[1].abs() * to_flat]
}

/// 🎛️ The ONE board↔world scale this artifact places and moves paired parts with — the linear inverse of
/// `🧬️schema/💡️inferences/🎛️flat-position`'s plan projection, so a flat point and a world origin stay one
/// consistent pair whichever pane the gesture came from. A flat unit is one board pixel; 48 of them make
/// one world metre (the board's own default part box, `part_2d.width`/`height`).
pub const PUZZLE5D_FLAT_TO_WORLD: f64 = 1.0 / 48.0;

/// 🎨️ Palette drop: creates a free paired part at the flat drop point, deriving the volume origin from the nearest peer part's offset.
pub fn add_palette_part(envelope: &mut Puzzle5dScene, part_kind: &str, x: f64, y: f64) {
    let flat_to_world = PUZZLE5D_FLAT_TO_WORLD;
    let origin = envelope
        .document
        .parts
        .first()
        .map_or([x * flat_to_world, -y * flat_to_world, 0.0], |peer| [peer.part_3d.origin[0] + (x - peer.part_2d.x) * flat_to_world, peer.part_3d.origin[1] - (y - peer.part_2d.y) * flat_to_world, peer.part_3d.origin[2]]);
    let id = Puzzle5dFreshIds::from_document(&envelope.document).next_part();
    let mesh_url = resolve_part_kind_mesh_url(part_kind, envelope.document.kind_catalogs.as_ref());
    let grips = grips_from_templates(&envelope.document, part_kind);
    // 🏷️ Stamped at creation, not derived at paint: the outliner, the 3D instance and a later export
    // must all read the SAME authored name, and only this moment knows how many peers of this kind
    // the document already holds.
    let label = puzzle5d_next_part_label(&envelope.document.parts, &envelope.document, part_kind);
    envelope.document.parts.push(Puzzle5dPart {
        id: id,
        anchor: Default::default(),
        part_kind: part_kind.into(),
        part_2d: Puzzle5dPart2d { x, y, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: part_kind.into(), icon_kind: None, hidden: None, locked: None },
        part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: Some(label) },
        grips,
    });
    let _ = id;
}
//#endregion 🔖️Document

//#region 🔖️Scene
/// 🧾️ Transient render/mutation bundle pairing the persisted projection (the bare `Puzzle5dDocument`
/// json) with the app's view state. Never persisted — the `VcsArtifactApp` store owns the document
/// and the wrapping store owns the VCS-tracked `Puzzle5dConfig` — but rebuilt per call so the
/// board/world/engagement helpers keep their `&scene` signatures.
#[derive(Clone)]
pub struct Puzzle5dScene {
    pub document: Puzzle5dDocument,
    pub runtime: Puzzle5dRuntime,
    /// 🧰️ The active utility for this window — transient, never persisted.
    pub active_utility: String,
    /// 🕹️ The framework-owned `vortex` selection AND hover this render/mutation reads. ONE domain for
    /// BOTH panes: a pick in the board pane and a pick in the world pane write the same domain, so
    /// each pane paints what the other selected or hovered without any cross-pane bus.
    pub interaction: Puzzle5dInteractionSnapshot,
}

/// 🕹️ One render pass' read of the framework-owned `vortex` interaction domain (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): the selected ids at the granularity they were
/// picked at plus the pointer-channel hover — the 5d twin of `Puzzle3dInteractionSnapshot` /
/// `Puzzle2dInteractionSnapshot`, in this artifact's part/grip/fastener vocabulary.
///
/// 🐁️ Hover ids arrive WITHOUT a granularity (`protocol::DomainHover` has none), so classifying one
/// needs the document: `hovered_part_id`/`hovered_grip_full_id` resolve against the live document
/// rather than guessing from the id's spelling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Puzzle5dInteractionSnapshot {
    pub granularity: String,
    pub selected: Vec<String>,
    pub hovered: Vec<String>,
}

impl Puzzle5dInteractionSnapshot {
    /// 🕹️ Reads the live `vortex` domain: its selection plus its `"pointer"`-channel hover.
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        let selection = interaction.selection(PUZZLE5D_INTERACTION_DOMAIN);
        let hover = interaction.hover(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_HOVER_CHANNEL);
        let leftover_ids = interaction.leftover_selected_ids();
        let selected = if selection.ids.is_empty() { leftover_ids } else { selection.ids.clone() };
        let granularity = if !selection.granularity.is_empty() {
            selection.granularity.clone()
        } else if selected.is_empty() {
            String::new()
        } else {
            PUZZLE5D_GRANULARITY_PART.to_string()
        };
        Self { granularity, selected, hovered: hover.ids.clone() }
    }

    /// 🕹️ The retained-reducer twin — the same read from the raw `InteractionState`/hover map a
    /// retained tool job is handed, so a command reusing a render helper sees the selection and hover
    /// both panes painted.
    pub fn from_state(state: &protocol::InteractionState, hover: &semio_framework_plugin::app::InteractionHoverState) -> Self {
        let selection = state.selection.get(PUZZLE5D_INTERACTION_DOMAIN);
        let hovered = hover.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|hover| hover.channel == PUZZLE5D_HOVER_CHANNEL).map(|hover| hover.ids.clone()).unwrap_or_default();
        let leftover_ids: Vec<String> = state.selection.values().flat_map(|selection| selection.ids.iter().cloned()).collect();
        let selected = selection.filter(|selection| !selection.ids.is_empty()).map(|selection| selection.ids.clone()).unwrap_or(leftover_ids);
        let granularity = selection.map(|selection| selection.granularity.clone()).filter(|granularity| !granularity.is_empty()).unwrap_or_else(|| if selected.is_empty() { String::new() } else { PUZZLE5D_GRANULARITY_PART.to_string() });
        Self { granularity, selected, hovered }
    }

    pub fn selected_ids(&self, granularity: &str) -> &[String] {
        if self.granularity == granularity {
            &self.selected
        } else {
            &[]
        }
    }
    pub fn selected_part_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE5D_GRANULARITY_PART)
    }
    pub fn selected_grip_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE5D_GRANULARITY_GRIP)
    }
    pub fn selected_fastener_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE5D_GRANULARITY_FASTENER)
    }
    pub fn selected_target_volume_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE5D_GRANULARITY_TARGET_VOLUME)
    }

    /// 🎨️ Every marked id regardless of granularity — what the board pane paints as its selection and
    /// the world pane matches instances against.
    pub fn selection_json(&self) -> String {
        serde_json::to_string(&self.selected).unwrap_or_else(|_| "[]".into())
    }

    /// 🐁️ The hovered part id, resolved against the document's own part ids.
    pub fn hovered_part_id(&self, document: &Puzzle5dDocument) -> Option<&str> {
        self.hovered.iter().find(|id| document.parts.iter().any(|part| &part.id == *id)).map(String::as_str)
    }

    /// 🐁️ The hovered grip full id (`{partId}:{gripId}`), resolved against the document.
    pub fn hovered_grip_full_id(&self, document: &Puzzle5dDocument) -> Option<&str> {
        self.hovered.iter().find(|id| document.parts.iter().any(|part| part.grips.iter().any(|grip| &puzzle5d_grip_full_id(&part.id, &grip.id) == *id))).map(String::as_str)
    }

    /// 🐁️ The first hovered id of any granularity — what `Board2dScene::hovered_id` paints.
    pub fn hovered_id(&self) -> Option<&str> {
        self.hovered.first().map(String::as_str)
    }

    /// 👁️ Whether `part` is itself selected/hovered or owns a selected/hovered grip — the predicate
    /// `PUZZLE5D_GRIP_SHOW_SELECTED` gates grip-marker emission on.
    pub fn touches_part(&self, part: &Puzzle5dPart) -> bool {
        let mut marks = self.selected.iter().chain(self.hovered.iter());
        marks.any(|id| id == &part.id || part.grips.iter().any(|grip| &puzzle5d_grip_full_id(&part.id, &grip.id) == id))
    }

    /// 🕹️ Whether anything at all is marked.
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty() && self.hovered.is_empty()
    }
}

/// 🧾️ Materializes the transient scene from the persisted projection (bare document json) and the
/// app's current view state; an unparseable projection degrades to an empty document. The interaction
/// read stays empty — call [`scene_from_projection_with_interaction`] from any path that holds one.
pub fn scene_from_projection(projection: &Value, runtime: Puzzle5dRuntime, active_utility: &str) -> Puzzle5dScene {
    scene_from_projection_with_interaction(projection, runtime, active_utility, Puzzle5dInteractionSnapshot::default())
}

/// 🧾️ The interaction-aware twin: the render paths that are handed an `InteractionView` build the
/// scene through this one so both panes project the SAME live selection and hover.
pub fn scene_from_projection_with_interaction(projection: &Value, runtime: Puzzle5dRuntime, active_utility: &str, interaction: Puzzle5dInteractionSnapshot) -> Puzzle5dScene {
    let document = <Puzzle5dDocument as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(projection)).unwrap_or_else(|_| empty_document());
    Puzzle5dScene { document, runtime, active_utility: active_utility.to_string(), interaction }
}

/// 🛠️ Whether the host has the mode-level fill TOOL armed (`ViewModel::active_tool_id`), which outranks every
/// window utility: fill keeps its viewport interaction in both panes while it is the armed tool.
pub fn puzzle5d_fill_tool_active(view_state: Option<&semio_framework_plugin::ViewModel>) -> bool {
    view_state.and_then(|state| state.active_tool_id.as_deref()) == Some(fill_tool::TOOL_ID)
}

/// 🧰️ Resolves the host-owned armed gesture for a concrete window: the mode-level tool first (a tool and a
/// utility are mutually exclusive in the shell, and fill wins), then that window's utility, then the surface's.
pub fn puzzle5d_scene_active_utility(view_state: Option<&semio_framework_plugin::ViewModel>, window_id: Option<&str>) -> String {
    if puzzle5d_fill_tool_active(view_state) {
        return fill_tool::TOOL_ID.to_string();
    }
    window_id
        .and_then(|window_id| view_state.and_then(|state| state.active_utility_by_window_id.get(window_id)))
        .or_else(|| view_state.and_then(|state| state.active_utility_id.as_ref()))
        .filter(|utility| !utility.is_empty())
        .cloned()
        .unwrap_or_else(|| PUZZLE5D_DEFAULT_UTILITY.to_string())
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

/// 🕹️ Whether the world gumball should render: a transform utility is active, at least one handle
/// flag is on (`setTransformGumballFlag` — an all-off gumball would draw nothing to grab), and the
/// live `vortex` selection holds at least one part. Selection comes from the framework-owned domain
/// via [`Puzzle5dInteractionSnapshot`], never from stored app state.
pub fn puzzle5d_gumball_active(runtime: &Puzzle5dRuntime, active_utility: &str, interaction: &Puzzle5dInteractionSnapshot) -> bool {
    puzzle5d_transform_utility_active(active_utility) && (runtime.transform_move || runtime.transform_rotate) && !interaction.selected_part_ids().is_empty()
}

pub fn gumball_target_world(envelope: &Puzzle5dScene, selected_part_ids: &[String]) -> Option<[f64; 3]> {
    let selected: Vec<&Puzzle5dPart> = envelope.document.parts.iter().filter(|part| selected_part_ids.contains(&part.id)).collect();
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
/// 🔘️ The kind a grip gates attraction compatibility with: its own kind, else its flat aspect's kind.
pub fn engine_grip_kind(grip: &Puzzle5dGrip) -> String {
    if grip.grip_kind.is_empty() { grip.grip_2d.grip_kind.clone() } else { grip.grip_kind.clone() }
}
//#endregion 🔖️Engine

//#region 🏷️Labels
/// 🏷️ The catalogue display name one part kind carries (`label` → `name` → its own id), else the kind
/// id itself — the 5d twin of `puzzle3d_kind_catalog_label`.
pub fn puzzle5d_kind_catalog_label(document: &Puzzle5dDocument, kind_id: &str) -> String {
    document
        .kind_catalogs
        .as_ref()
        .and_then(|catalogs| catalogs.get("parts"))
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .find(|entry| entry.get("id").and_then(serde_json::Value::as_str) == Some(kind_id))
        .map(catalogue::catalog_kind_label)
        .unwrap_or_else(|| kind_id.to_string())
}

fn puzzle5d_label_root(label: &str) -> String {
    let Some((base, suffix)) = label.rsplit_once(' ') else {
        return label.to_string();
    };
    if suffix.parse::<u32>().is_ok() { base.to_string() } else { label.to_string() }
}

fn puzzle5d_label_number(label: &str, root: &str) -> Option<u32> {
    if label == root {
        return Some(1);
    }
    label.strip_prefix(root)?.strip_prefix(' ')?.parse().ok()
}

/// 🏷️ The authored label of one part — its volume label, else its flat text, else the kind's catalogue
/// display name, else its id. Same precedence as `puzzle3d_object_display_label`, with 5d's second
/// authored carrier (the flat aspect's `text`) between the label and the catalogue name.
pub fn puzzle5d_part_display_label(part: &Puzzle5dPart, document: &Puzzle5dDocument) -> String {
    if let Some(label) = part.part_3d.label.as_deref().filter(|value| !value.is_empty()) {
        return label.to_string();
    }
    if !part.part_2d.text.is_empty() {
        return part.part_2d.text.clone();
    }
    if part.part_kind.is_empty() {
        return part.id.clone();
    }
    puzzle5d_kind_catalog_label(document, &part.part_kind)
}

/// 🔢️ The next distinct part label for one kind — the first instance takes the catalogue name, each
/// further one appends ` 2`, ` 3`, … to the root its peers already carry.
pub fn puzzle5d_next_part_label(parts: &[Puzzle5dPart], document: &Puzzle5dDocument, kind_id: &str) -> String {
    let catalog_base = puzzle5d_kind_catalog_label(document, kind_id);
    let peers: Vec<&Puzzle5dPart> = parts.iter().filter(|part| part.part_kind == kind_id).collect();
    if peers.is_empty() {
        return catalog_base;
    }
    let root = peers.iter().find_map(|part| part.part_3d.label.as_deref().filter(|value| !value.is_empty())).map(puzzle5d_label_root).unwrap_or(catalog_base);
    let mut max = 0u32;
    let mut unlabeled = 0u32;
    for part in peers {
        match part.part_3d.label.as_deref().filter(|value| !value.is_empty()) {
            Some(label) => {
                if let Some(number) = puzzle5d_label_number(label, &root) {
                    max = max.max(number);
                }
            }
            None => unlabeled += 1,
        }
    }
    max = max.max(unlabeled);
    if max == 0 { root } else { format!("{root} {}", max + 1) }
}

/// 🔢️ `puzzle5d_next_part_label` read off the JSON projection instead of the typed document — the
/// numbering a RETAINED creation Work needs, because those works only ever hold `Puzzle5dPlaySnapshot`'s
/// projection. Same precedence and the same `puzzle5d_label_root`/`puzzle5d_label_number` arithmetic, so
/// a part created by the palette and a part created by `addNode` land on the same series.
pub fn puzzle5d_next_part_label_from_projection(projection: &Value, kind_id: &str) -> String {
    let catalog_base = projection
        .get("kindCatalogs")
        .and_then(|catalogs| catalogs.get("parts"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .find(|entry| entry.get("id").and_then(Value::as_str) == Some(kind_id))
        .and_then(|entry| ["label", "name", "id"].into_iter().find_map(|key| entry.get(key).and_then(Value::as_str).filter(|value| !value.is_empty())))
        .unwrap_or(kind_id)
        .to_string();
    let peer_label = |part: &Value| part.get("3d").and_then(|pose| pose.get("label")).and_then(Value::as_str).filter(|value| !value.is_empty()).map(str::to_string);
    let peers: Vec<String> = projection
        .get("parts")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|part| part.get("partKind").and_then(Value::as_str) == Some(kind_id))
        .map(|part| peer_label(part).unwrap_or_default())
        .collect();
    if peers.is_empty() {
        return catalog_base;
    }
    let root = peers.iter().find(|label| !label.is_empty()).map(|label| puzzle5d_label_root(label)).unwrap_or(catalog_base);
    let mut max = 0u32;
    let mut unlabeled = 0u32;
    for label in &peers {
        if label.is_empty() {
            unlabeled += 1;
        } else if let Some(number) = puzzle5d_label_number(label, &root) {
            max = max.max(number);
        }
    }
    max = max.max(unlabeled);
    if max == 0 { root } else { format!("{root} {}", max + 1) }
}
//#endregion 🏷️Labels

//#region 🧬️InferredKinds
/// 🧬️ The part-kind rows a catalogue-less document IMPLIES: one row per distinct `partKind`, shaped
/// like the first part carrying it — flat shape and size, icon, mesh and that part's grips as
/// templates. None of the three 5d examples authors `kindCatalogs`, so without this every catalogue
/// section and every catalogue drop would be empty; a document whose `kindCatalogs` names parts never
/// reaches this (mirrors `inferred_node_kind_rows` in puzzle 2d).
pub fn puzzle5d_inferred_part_kind_rows(document: &Puzzle5dDocument) -> Vec<serde_json::Value> {
    let mut rows: Vec<serde_json::Value> = Vec::new();
    for part in &document.parts {
        if part.part_kind.is_empty() || rows.iter().any(|row| row.get("id").and_then(serde_json::Value::as_str) == Some(part.part_kind.as_str())) {
            continue;
        }
        let rectangle = part.part_2d.shape == "rectangle";
        let size = if rectangle {
            part.part_2d.width.unwrap_or(PUZZLE5D_DEFAULT_PART_RADIUS * 2.0).max(part.part_2d.height.unwrap_or(PUZZLE5D_DEFAULT_PART_RADIUS * 2.0))
        } else {
            (if part.part_2d.radius > 0.0 { part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS }) * 2.0
        };
        let grips: Vec<serde_json::Value> = part
            .grips
            .iter()
            .map(|grip| {
                serde_json::json!({
                    "gripKind": engine_grip_kind(grip),
                    "angle": grip.grip_2d.angle,
                    "radius": grip.grip_3d.radius,
                    "position": grip.grip_3d.position,
                    "direction": grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]),
                })
            })
            .collect();
        let mut row = serde_json::json!({
            "id": part.part_kind,
            "name": part.part_kind,
            "shape": if rectangle { "rectangle" } else { "circle" },
            "radius": size * 0.5,
            "width": size,
            "height": size,
            "grips": grips,
        });
        if let Some(icon) = part.part_2d.icon_kind.as_deref().filter(|icon| !icon.is_empty()) {
            row["iconKind"] = serde_json::json!(icon);
        }
        if let Some(url) = part.part_3d.mesh_url.as_deref().filter(|url| !url.is_empty()) {
            row["meshUrl"] = serde_json::json!(url);
        }
        rows.push(row);
    }
    rows
}

/// 🧬️ The read-only kind rows of the grip/fastener/rope slices a catalogue-less document implies —
/// one `{id, name}` per distinct kind the document already names. `ropes` has no document carrier, so
/// it stays empty and its section keeps its placeholder.
pub fn puzzle5d_inferred_kind_rows(document: &Puzzle5dDocument, slice: &str) -> Vec<serde_json::Value> {
    if slice == "parts" {
        return puzzle5d_inferred_part_kind_rows(document);
    }
    let mut ids: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    match slice {
        "grips" => {
            for part in &document.parts {
                ids.extend(part.grips.iter().map(engine_grip_kind).filter(|kind| !kind.is_empty()));
            }
        }
        "fasteners" => ids.extend(document.fasteners.iter().filter_map(|fastener| fastener.fastener_kind.clone()).filter(|kind| !kind.is_empty())),
        _ => {}
    }
    ids.into_iter().map(|id| serde_json::json!({ "id": id, "name": id })).collect()
}
//#endregion 🧬️InferredKinds

//#region 🙈️SelectionFlags
/// 🙈️ Sets `hidden`/`locked` on exactly the named entities. Only a part carries these flags in 5d
/// (a grip and a fastener have no such field), so any other `entity` is a no-op rather than a fault.
pub fn apply_puzzle5d_selection_flag(document: &mut Puzzle5dDocument, entity: &str, ids: &[String], flag: &str, value: bool) {
    if entity != PUZZLE5D_GRANULARITY_PART {
        return;
    }
    for part in &mut document.parts {
        if !ids.iter().any(|id| id == &part.id) {
            continue;
        }
        match flag {
            "hidden" => part.part_2d.hidden = Some(value),
            "locked" => part.part_2d.locked = Some(value),
            _ => {}
        }
    }
}
//#endregion 🙈️SelectionFlags

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
/// `crate::standards::v1::subsets::any::schema::transfer::copy_selection`.
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
/// too) — document-scoped fresh ids dodge collisions with the live document,
/// and fastener endpoints are remapped onto the fresh part ids.
fn paste_selection_local(document: &Puzzle5dDocument, fragment_parts: &[Puzzle5dPart], fragment_fasteners: &[Puzzle5dFastener], delta: (f64, f64)) -> (Vec<Puzzle5dPart>, Vec<Puzzle5dFastener>) {
    let mut fresh_ids = Puzzle5dFreshIds::from_document(document);
    let mut id_map: HashMap<String, String> = HashMap::new();
    let mut fresh_parts = Vec::with_capacity(fragment_parts.len());
    for part in fragment_parts {
        let fresh_id = fresh_ids.next_part();
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
        next.id = fresh_ids.next_fastener();
        next.source = rewrite_grip_ref_local(&fastener.source, &id_map);
        next.target = rewrite_grip_ref_local(&fastener.target, &id_map);
        fresh_fasteners.push(next);
    }
    (fresh_parts, fresh_fasteners)
}
//#endregion 🔖️CopyPaste

//#region 🧵️ReservedJobs
fn puzzle5d_preflight_reserved_wire(raw: Vec<u8>, maximum_bytes: usize) -> Result<Vec<u8>, (Fault, Vec<u8>)> {
    if raw.len() > maximum_bytes {
        return Err((Fault::from("puzzle5d reserved wire exceeds its exact route cap before fixed-page copy"), raw));
    }
    Ok(raw)
}

fn puzzle5d_payload(cx: &mut StepContext<'_>, stream: JobPayloadStream, bytes: &[u8]) -> RetainedJobPayload {
    match cx.payload_from_bytes(stream, bytes) {
        Ok(payload) => payload,
        Err(rejected) => {
            drop(rejected.into_source());
            RetainedJobPayload::empty(stream)
        }
    }
}

fn puzzle5d_job_fault(cx: &mut StepContext<'_>, detail: impl AsRef<str>) -> StepOutcome {
    let bytes = detail.as_ref().as_bytes();
    let bounded = &bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)];
    StepOutcome::Fault(JobFault { detail: puzzle5d_payload(cx, JobPayloadStream::Fault, bounded) })
}

fn puzzle5d_job_checkpoint(stage: u8, cursor: usize, progress: u64, cx: &mut StepContext<'_>) -> StepOutcome {
    let mut state = [0; 17];
    state[0] = stage;
    state[1..9].copy_from_slice(&(cursor as u64).to_le_bytes());
    state[9..17].copy_from_slice(&progress.to_le_bytes());
    StepOutcome::CheckpointReady(Checkpoint { state: puzzle5d_payload(cx, JobPayloadStream::CheckpointState, &state), applied_progress: progress })
}

fn puzzle5d_import_checkpoint_bytes(stage: u8, cursor: usize, nested_cursor: usize, decoded_items: usize, progress: u64) -> [u8; 33] {
    let mut state = [0; 33];
    state[0] = stage;
    state[1..9].copy_from_slice(&(cursor as u64).to_le_bytes());
    state[9..17].copy_from_slice(&(nested_cursor as u64).to_le_bytes());
    state[17..25].copy_from_slice(&(decoded_items as u64).to_le_bytes());
    state[25..33].copy_from_slice(&progress.to_le_bytes());
    state
}

fn puzzle5d_import_checkpoint(stage: u8, cursor: usize, nested_cursor: usize, decoded_items: usize, progress: u64, cx: &mut StepContext<'_>) -> StepOutcome {
    let state = puzzle5d_import_checkpoint_bytes(stage, cursor, nested_cursor, decoded_items, progress);
    StepOutcome::CheckpointReady(Checkpoint { state: puzzle5d_payload(cx, JobPayloadStream::CheckpointState, &state), applied_progress: progress })
}

struct Puzzle5dCommitEnvelope {
    writer: std::mem::ManuallyDrop<Option<RetainedJobPayloadWriter>>,
    output: std::mem::ManuallyDrop<Option<RetainedJobPayload>>,
    cursor: usize,
    closing: bool,
}

impl Puzzle5dCommitEnvelope {
    fn new() -> Self {
        Self { writer: std::mem::ManuallyDrop::new(Some(RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput))), output: std::mem::ManuallyDrop::new(None), cursor: 0, closing: false }
    }

    fn prepare(&mut self, raw: &[u8], cx: &mut StepContext<'_>) -> Result<bool, &'static str> {
        if self.output.is_some() {
            return Ok(true);
        }
        let writer = self.writer.as_mut().ok_or("puzzle5d commit envelope lost its writer authority")?;
        match writer.write_slice_page(cx, raw, &mut self.cursor) {
            Ok(false) | Err(JobPayloadAdmissionFault::OpportunityExhausted) => Ok(false),
            Err(_) => Err("puzzle5d commit envelope rejected its fixed output page"),
            Ok(true) => {
                let writer = self.writer.take().ok_or("puzzle5d commit envelope lost its completed writer")?;
                match writer.finish() {
                    Ok(output) => {
                        *self.output = Some(output);
                        Ok(true)
                    }
                    Err(writer) => {
                        *self.writer = Some(writer);
                        Err("puzzle5d commit envelope retained a rejected output page")
                    }
                }
            }
        }
    }

    fn take_output(&mut self) -> Option<RetainedJobPayload> {
        self.output.take()
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(writer) = self.writer.as_mut() {
            writer.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
        self.begin_close();
        if self.output.is_some() {
            let step = self.output.as_mut().expect("checked Puzzle5d commit output").close_step(maximum_items, maximum_bytes);
            return match step {
                JobPayloadCloseStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released_bytes },
                JobPayloadCloseStep::Complete => {
                    drop(self.output.take());
                    PluginCloseStep::Pending { released_items: usize::from(maximum_items > 0), released_bytes: 0 }
                }
            };
        }
        if self.writer.is_some() {
            let (step, terminal) = {
                let writer = self.writer.as_mut().expect("checked Puzzle5d commit writer");
                let step = writer.close_step(maximum_items, maximum_bytes);
                (step, writer.terminal_is_empty())
            };
            return match step {
                JobPayloadCloseStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released_bytes },
                JobPayloadCloseStep::Complete if terminal => {
                    drop(self.writer.take());
                    PluginCloseStep::Pending { released_items: usize::from(maximum_items > 0), released_bytes: 0 }
                }
                JobPayloadCloseStep::Complete => PluginCloseStep::Blocked { reason: "puzzle5d commit envelope writer returned a false terminal witness" },
            };
        }
        PluginCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.writer.is_none() && self.output.is_none()
    }
}

impl Drop for Puzzle5dCommitEnvelope {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            unsafe {
                std::mem::ManuallyDrop::drop(&mut self.writer);
                std::mem::ManuallyDrop::drop(&mut self.output);
            }
        } else {
            debug_assert!(false, "Puzzle5d commit envelope requires exact output close before Drop");
        }
    }
}

fn puzzle5d_step_envelope(raw: &[u8], cursor: &mut usize, page: &mut [u8; PUZZLE5D_RESERVED_PAGE_BYTES], page_len: &mut usize, progress: &mut u64, cx: &mut StepContext<'_>) -> Option<StepOutcome> {
    if *cursor >= raw.len() {
        *page_len = 0;
        return None;
    }
    let units = raw.len().saturating_sub(*cursor).min(page.len()).min(cx.fuel_remaining() as usize);
    if units == 0 {
        return Some(StepOutcome::Yield);
    }
    let end = cursor.checked_add(units).filter(|end| *end <= raw.len()).expect("Puzzle5d fixed-page ingress preflights the source range before copy");
    page[..units].copy_from_slice(&raw[*cursor..end]);
    *page_len = units;
    *cursor = end;
    *progress = progress.saturating_add(units as u64);
    cx.consume_fuel(units as u64);
    Some(puzzle5d_job_checkpoint(0, *cursor, *progress, cx))
}

fn puzzle5d_selection_ids(interaction: &semio_framework::InteractionState) -> (HashSet<String>, HashSet<String>) {
    match interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN) {
        Some(selection) if selection.granularity == PUZZLE5D_GRANULARITY_PART => (selection.ids.iter().cloned().collect(), HashSet::new()),
        Some(selection) if selection.granularity == PUZZLE5D_GRANULARITY_FASTENER => (HashSet::new(), selection.ids.iter().cloned().collect()),
        _ => (HashSet::new(), HashSet::new()),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dSelectionStage {
    Endpoints,
    Fasteners,
    Parts,
    Complete,
}

struct Puzzle5dSelectionScan {
    snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    projection: Option<Value>,
    part_ids: HashSet<String>,
    explicit_fastener_ids: HashSet<String>,
    stage: Puzzle5dSelectionStage,
    cursor: usize,
    parts: Vec<Puzzle5dPart>,
    fasteners: Vec<Puzzle5dFastener>,
}

impl Puzzle5dSelectionScan {
    fn new(snapshot: std::sync::Arc<Puzzle5dPlaySnapshot>, interaction: &semio_framework::InteractionState) -> Self {
        let (part_ids, explicit_fastener_ids) = puzzle5d_selection_ids(interaction);
        Self { snapshot: Some(snapshot), projection: None, part_ids, explicit_fastener_ids, stage: Puzzle5dSelectionStage::Endpoints, cursor: 0, parts: Vec::new(), fasteners: Vec::new() }
    }

    /// 🗂️ One cursored row of the scanned projection, which is derived ONCE and then cached. Re-deriving
    /// it per step — and cloning the whole row array with it — cost O(document) on every one of a
    /// selection's ~2N+M cursor turns and blew the interactive ceiling on a real document:
    /// `framework route 'copy' overran the 8 ms step ceiling for 4 consecutive steps, worst 70521us`
    /// on the 180-part Nakagin, which is what made `copy`/`cut` fault the moment the shipped examples
    /// stopped loading empty. The cache is retired on its own rung of the close ladder.
    fn row(&mut self, key: &str, index: usize) -> Option<Value> {
        if self.projection.is_none() {
            let projection = self.snapshot.as_ref().map(|snapshot| puzzle5d_projection_value(&snapshot.0))?;
            self.projection = Some(projection);
        }
        self.projection.as_ref()?.get(key).and_then(Value::as_array).and_then(|rows| rows.get(index)).cloned()
    }

    fn step(&mut self) -> Result<bool, String> {
        match self.stage {
            Puzzle5dSelectionStage::Endpoints => {
                let cursor = self.cursor;
                if let Some(row) = self.row("fasteners", cursor) {
                    self.cursor += 1;
                    if row.get("id").and_then(Value::as_str).is_some_and(|id| self.explicit_fastener_ids.contains(id)) {
                        if let Some(source) = row.get("source").and_then(Value::as_str) {
                            self.part_ids.insert(owning_part_id_local(source).to_string());
                        }
                        if let Some(target) = row.get("target").and_then(Value::as_str) {
                            self.part_ids.insert(owning_part_id_local(target).to_string());
                        }
                    }
                } else {
                    self.stage = Puzzle5dSelectionStage::Fasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dSelectionStage::Fasteners => {
                let cursor = self.cursor;
                if let Some(row) = self.row("fasteners", cursor) {
                    self.cursor += 1;
                    let source = row.get("source").and_then(Value::as_str).map(owning_part_id_local);
                    let target = row.get("target").and_then(Value::as_str).map(owning_part_id_local);
                    let selected = row.get("id").and_then(Value::as_str).is_some_and(|id| self.explicit_fastener_ids.contains(id))
                        || source.zip(target).is_some_and(|(source, target)| !self.part_ids.is_empty() && self.part_ids.contains(source) && self.part_ids.contains(target));
                    if selected {
                        self.fasteners.push(serde_json::from_value(serde_json::Value::from(&dsl::os_pack::json::to_dsl_value(&row))).map_err(|error| error.to_string())?);
                    }
                } else {
                    self.stage = Puzzle5dSelectionStage::Parts;
                    self.cursor = 0;
                }
            }
            Puzzle5dSelectionStage::Parts => {
                let cursor = self.cursor;
                if let Some(row) = self.row("parts", cursor) {
                    self.cursor += 1;
                    if row.get("id").and_then(Value::as_str).is_some_and(|id| self.part_ids.contains(id)) {
                        self.parts.push(serde_json::from_value(serde_json::Value::from(&dsl::os_pack::json::to_dsl_value(&row))).map_err(|error| error.to_string())?);
                    }
                } else {
                    self.stage = Puzzle5dSelectionStage::Complete;
                }
            }
            Puzzle5dSelectionStage::Complete => return Ok(true),
        }
        Ok(self.stage == Puzzle5dSelectionStage::Complete)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dClipboardStage {
    Envelope,
    Select,
    EncodeParts,
    EncodeFasteners,
    Complete,
}

const PUZZLE5D_JSON_RETIREMENT_KEY_BYTES: usize = 4_096;

fn puzzle5d_retire_vec_backing<T>(owners: &mut Vec<T>, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    // 🐛️ A `Vec` of a ZERO-SIZED element never allocates, and `Vec::capacity` reports `usize::MAX` for
    // it by definition — so `capacity() == 0` is false forever and this returned
    // `Pending { released_items: 1 }` on every call for a lane that has no backing at all. `Emit`'s
    // `draft_mutations` is exactly that: `NoDraftMutation = NoConfigMutation` is the uninhabited
    // `pub enum NoConfigMutation {}`. That made `puzzle5d_retire_completion_emit_step` answer `Some`
    // forever, `Puzzle5dPendingCompletionRejection::close_step` never set `emit_closed`, and all four
    // `*_completion_rejection_*` laws spun 100 000 bounded turns without converging. Captured
    // 2026-09-22 from the law's own dump: `emit(mutations=0 cap=0 effects=0 cap=0 events=0 children=0
    // cap=0 …)` with `emit_closed=false`.
    if !owners.is_empty() || owners.capacity() == 0 || size_of::<T>() == 0 {
        return Ok(None);
    }
    let bytes = owners.capacity().saturating_mul(size_of::<T>());
    if bytes > maximum_bytes {
        return Err(Fault::from("puzzle5d vector backing exceeds its bounded disposal byte slice"));
    }
    *owners = Vec::new();
    Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }))
}

fn puzzle5d_retire_json_step(value: &mut serde_json::Value, key: &mut [u8; PUZZLE5D_JSON_RETIREMENT_KEY_BYTES], maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::Bool(_) | serde_json::Value::Number(_) => {
            *value = serde_json::Value::Null;
            Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }))
        }
        serde_json::Value::String(text) => {
            let bytes = text.capacity();
            if bytes > maximum_bytes {
                return Err(Fault::from("puzzle5d recursive string exceeds its bounded disposal byte slice"));
            }
            *value = serde_json::Value::Null;
            Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }))
        }
        serde_json::Value::Array(values) => {
            if let Some(last) = values.last_mut() {
                if last.is_null() {
                    values.pop();
                    return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
                }
                return puzzle5d_retire_json_step(last, key, maximum_bytes);
            }
            let bytes = values.capacity().saturating_mul(size_of::<serde_json::Value>());
            if bytes > maximum_bytes {
                return Err(Fault::from("puzzle5d recursive array backing exceeds its bounded disposal byte slice"));
            }
            *value = serde_json::Value::Null;
            Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }))
        }
        serde_json::Value::Object(values) => {
            let next = values.iter().next().map(|(name, child)| (name.len(), child.is_null()));
            let Some((name_len, child_empty)) = next else {
                *value = serde_json::Value::Null;
                return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
            };
            if name_len > key.len() || name_len > maximum_bytes {
                return Err(Fault::from("puzzle5d recursive object key exceeds its fixed disposal key slice"));
            }
            if !child_empty {
                let child = values.iter_mut().next().map(|(_, child)| child).ok_or_else(|| Fault::from("puzzle5d recursive object changed during retirement"))?;
                return puzzle5d_retire_json_step(child, key, maximum_bytes);
            }
            let name = values.iter().next().map(|(name, _)| name.as_bytes()).ok_or_else(|| Fault::from("puzzle5d recursive object changed during key retirement"))?;
            key[..name_len].copy_from_slice(name);
            let name = std::str::from_utf8(&key[..name_len]).map_err(|error| Fault::from(error.to_string()))?;
            let removed = values.remove_entry(name).ok_or_else(|| Fault::from("puzzle5d recursive object lost its admitted key"))?;
            let bytes = removed.0.capacity();
            drop(removed);
            Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }))
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️puzzle5d-retained-retirement-laws/🦀️.rs"]
mod puzzle5d_retained_retirement_laws;

struct Puzzle5dClipboardWork {
    raw: Vec<u8>,
    raw_cursor: usize,
    raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES],
    raw_page_len: usize,
    progress: u64,
    stage: Puzzle5dClipboardStage,
    scan: Puzzle5dSelectionScan,
    encode_cursor: usize,
    dsl_text: String,
    completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    commit: Puzzle5dCommitEnvelope,
    closing: bool,
}

enum Puzzle5dClipboardWorkStep {
    Outcome(StepOutcome),
    Pending,
    Complete,
}

impl Puzzle5dClipboardWork {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, interaction: semio_framework::InteractionState) -> Self {
        Self {
            raw: request.raw_wire,
            raw_cursor: 0,
            raw_page: [0; PUZZLE5D_RESERVED_PAGE_BYTES],
            raw_page_len: 0,
            progress: 0,
            stage: Puzzle5dClipboardStage::Envelope,
            scan: Puzzle5dSelectionScan::new(request.snapshot, &interaction),
            encode_cursor: 0,
            dsl_text: String::new(),
            completion: Some(request.completion),
            commit: Puzzle5dCommitEnvelope::new(),
            closing: false,
        }
    }

    fn step_work(&mut self, cx: &mut StepContext<'_>) -> Result<Puzzle5dClipboardWorkStep, String> {
        match self.stage {
            Puzzle5dClipboardStage::Envelope => {
                if let Some(outcome) = puzzle5d_step_envelope(&self.raw, &mut self.raw_cursor, &mut self.raw_page, &mut self.raw_page_len, &mut self.progress, cx) {
                    return Ok(Puzzle5dClipboardWorkStep::Outcome(outcome));
                }
                self.stage = Puzzle5dClipboardStage::Select;
            }
            Puzzle5dClipboardStage::Select => {
                if !self.scan.step()? {
                    self.progress = self.progress.saturating_add(1);
                    cx.consume_fuel(1);
                    return Ok(Puzzle5dClipboardWorkStep::Pending);
                }
                self.dsl_text = format!("{{\"schema\":{},\"parts\":[", serde_json::to_string(PUZZLE5D_SCHEMA).map_err(|error| error.to_string())?);
                self.stage = Puzzle5dClipboardStage::EncodeParts;
                self.encode_cursor = 0;
            }
            Puzzle5dClipboardStage::EncodeParts => {
                if let Some(part) = self.scan.parts.get(self.encode_cursor) {
                    if self.encode_cursor != 0 {
                        self.dsl_text.push(',');
                    }
                    self.dsl_text.push_str(&serde_json::to_string(part).map_err(|error| error.to_string())?);
                    self.encode_cursor += 1;
                    self.progress = self.progress.saturating_add(1);
                    cx.consume_fuel(1);
                    return Ok(Puzzle5dClipboardWorkStep::Pending);
                }
                self.dsl_text.push_str("],\"fasteners\":[");
                self.stage = Puzzle5dClipboardStage::EncodeFasteners;
                self.encode_cursor = 0;
            }
            Puzzle5dClipboardStage::EncodeFasteners => {
                if let Some(fastener) = self.scan.fasteners.get(self.encode_cursor) {
                    if self.encode_cursor != 0 {
                        self.dsl_text.push(',');
                    }
                    self.dsl_text.push_str(&serde_json::to_string(fastener).map_err(|error| error.to_string())?);
                    self.encode_cursor += 1;
                    self.progress = self.progress.saturating_add(1);
                    cx.consume_fuel(1);
                    return Ok(Puzzle5dClipboardWorkStep::Pending);
                }
                self.dsl_text.push_str("]}");
                if self.dsl_text.len() > PUZZLE5D_RESERVED_OUTPUT_BYTES {
                    return Err("puzzle5d clipboard fragment exceeds output cap".into());
                }
                self.stage = Puzzle5dClipboardStage::Complete;
            }
            Puzzle5dClipboardStage::Complete => return Ok(Puzzle5dClipboardWorkStep::Complete),
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        Ok(if self.stage == Puzzle5dClipboardStage::Complete { Puzzle5dClipboardWorkStep::Complete } else { Puzzle5dClipboardWorkStep::Pending })
    }

    fn checkpoint(&self, cx: &mut StepContext<'_>) -> StepOutcome {
        puzzle5d_job_checkpoint(self.stage as u8, self.encode_cursor.max(self.scan.cursor), self.progress, cx)
    }

    fn fragment(&self) -> Option<ClipboardFragment> {
        (!self.scan.parts.is_empty()).then(|| ClipboardFragment {
            schema: PUZZLE5D_SCHEMA.to_string(),
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design },
            dsl_text: self.dsl_text.clone(),
            pack_bytes: None,
            source_app: PUZZLE5D_PLAY_APP_ID.to_string(),
            label: format!("{} part(s)", self.scan.parts.len()),
        })
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        self.closing = true;
        if maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.commit.close_step(maximum_items, maximum_bytes) {
            PluginCloseStep::Complete => {}
            step => return Ok(step),
        }
        if let Some(part) = self.scan.parts.last_mut() {
            if let Some(grip) = part.grips.pop() {
                let bytes = grip.id.len().saturating_add(grip.grip_kind.len()).saturating_add(grip.grip_2d.grip_kind.len()).saturating_add(grip.grip_3d.label.as_ref().map_or(0, String::len));
                if bytes > maximum_bytes {
                    part.grips.push(grip);
                    return Err(Fault::from("puzzle5d clipboard grip exceeds its bounded disposal byte slice"));
                }
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
            }
            if let Some(step) = puzzle5d_retire_vec_backing(&mut part.grips, maximum_bytes)? {
                return Ok(step);
            }
            if let Some(step) = puzzle5d_retire_vec_backing(&mut part.grips, maximum_bytes)? {
                return Ok(step);
            }
            if matches!(part.part_3d.scale, Some(serde_json::Value::Array(_)) | Some(serde_json::Value::Object(_))) {
                return Err(Fault::from("puzzle5d clipboard part retains an unproved recursive scale value"));
            }
            let part = self.scan.parts.pop().expect("last part exists");
            let bytes = part
                .id
                .len()
                .saturating_add(part.part_kind.len())
                .saturating_add(part.part_2d.shape.len())
                .saturating_add(part.part_2d.text.len())
                .saturating_add(part.part_2d.icon_kind.as_ref().map_or(0, String::len))
                .saturating_add(part.part_3d.mesh_url.as_ref().map_or(0, String::len))
                .saturating_add(part.part_3d.label.as_ref().map_or(0, String::len));
            if bytes > maximum_bytes {
                self.scan.parts.push(part);
                return Err(Fault::from("puzzle5d clipboard part exceeds its bounded disposal byte slice"));
            }
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(fastener) = self.scan.fasteners.pop() {
            let bytes = fastener.id.len().saturating_add(fastener.source.len()).saturating_add(fastener.target.len()).saturating_add(fastener.fastener_kind.as_ref().map_or(0, String::len));
            if bytes > maximum_bytes {
                self.scan.fasteners.push(fastener);
                return Err(Fault::from("puzzle5d clipboard fastener exceeds its bounded disposal byte slice"));
            }
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(step) = puzzle5d_retire_vec_backing(&mut self.scan.parts, maximum_bytes)? {
            return Ok(step);
        }
        if let Some(step) = puzzle5d_retire_vec_backing(&mut self.scan.fasteners, maximum_bytes)? {
            return Ok(step);
        }
        let part_id = {
            let mut ids = self.scan.part_ids.extract_if(|_| true);
            ids.next()
        };
        if let Some(key) = part_id {
            if key.capacity() > maximum_bytes {
                self.scan.part_ids.insert(key);
                return Err(Fault::from("puzzle5d clipboard selection id exceeds its bounded disposal byte slice"));
            }
            let bytes = key.capacity();
            drop(key);
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        let fastener_id = {
            let mut ids = self.scan.explicit_fastener_ids.extract_if(|_| true);
            ids.next()
        };
        if let Some(key) = fastener_id {
            if key.capacity() > maximum_bytes {
                self.scan.explicit_fastener_ids.insert(key);
                return Err(Fault::from("puzzle5d clipboard fastener id exceeds its bounded disposal byte slice"));
            }
            let bytes = key.capacity();
            drop(key);
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if self.scan.part_ids.is_empty() && self.scan.part_ids.capacity() != 0 {
            let bytes = self.scan.part_ids.capacity().saturating_mul(size_of::<String>());
            if bytes > maximum_bytes {
                return Err(Fault::from("puzzle5d clipboard selection backing exceeds its bounded disposal byte slice"));
            }
            self.scan.part_ids.shrink_to_fit();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if self.scan.explicit_fastener_ids.is_empty() && self.scan.explicit_fastener_ids.capacity() != 0 {
            let bytes = self.scan.explicit_fastener_ids.capacity().saturating_mul(size_of::<String>());
            if bytes > maximum_bytes {
                return Err(Fault::from("puzzle5d clipboard fastener selection backing exceeds its bounded disposal byte slice"));
            }
            self.scan.explicit_fastener_ids.shrink_to_fit();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(character) = self.dsl_text.pop() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: character.len_utf8() });
        }
        if self.dsl_text.capacity() != 0 {
            let bytes = self.dsl_text.capacity();
            if bytes > maximum_bytes {
                return Err(Fault::from("puzzle5d clipboard text backing exceeds its bounded disposal byte slice"));
            }
            self.dsl_text = String::new();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if !self.raw.is_empty() && maximum_bytes == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.raw.pop().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 1 });
        }
        if self.raw.capacity() != 0 {
            let bytes = self.raw.capacity();
            if bytes > maximum_bytes {
                return Err(Fault::from("puzzle5d clipboard wire backing exceeds its bounded disposal byte slice"));
            }
            self.raw = Vec::new();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if self.scan.projection.take().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.scan.snapshot.as_ref().is_some_and(|snapshot| std::sync::Arc::strong_count(snapshot) == 1) {
            return Ok(PluginCloseStep::Blocked { reason: "puzzle5d clipboard snapshot has no mounted retained authority" });
        }
        if self.scan.snapshot.take().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.completion.as_ref().is_some_and(|completion| !completion.has_mounted_consumer()) {
            return Ok(PluginCloseStep::Blocked { reason: "puzzle5d clipboard completion has no mounted consumer authority" });
        }
        if self.completion.take().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.raw.is_empty()
            && self.raw.capacity() == 0
            && self.scan.snapshot.is_none()
            && self.scan.projection.is_none()
            && self.scan.part_ids.is_empty()
            && self.scan.part_ids.capacity() == 0
            && self.scan.explicit_fastener_ids.is_empty()
            && self.scan.explicit_fastener_ids.capacity() == 0
            && self.scan.parts.is_empty()
            && self.scan.parts.capacity() == 0
            && self.scan.fasteners.is_empty()
            && self.scan.fasteners.capacity() == 0
            && self.dsl_text.is_empty()
            && self.dsl_text.capacity() == 0
            && self.completion.is_none()
            && self.commit.terminal_is_empty()
    }
}

struct Puzzle5dCopyJob {
    work: Puzzle5dClipboardWork,
    pending_completion_rejection: Option<Puzzle5dPendingCompletionRejection>,
    completed: bool,
}

impl InteractiveJob for Puzzle5dCopyJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle5d_job_fault(cx, "puzzle5d copy completion remains rejected");
        }
        if !self.completed {
            match self.work.step_work(cx) {
                Ok(Puzzle5dClipboardWorkStep::Outcome(outcome)) => return outcome,
                Ok(Puzzle5dClipboardWorkStep::Pending) => return self.work.checkpoint(cx),
                Err(error) => return puzzle5d_job_fault(cx, error),
                Ok(Puzzle5dClipboardWorkStep::Complete) => {
                    match self.work.commit.prepare(&self.work.raw, cx) {
                        Ok(false) => return StepOutcome::Yield,
                        Err(error) => return puzzle5d_job_fault(cx, error),
                        Ok(true) => {}
                    }
                    let emit = match self.work.fragment() {
                        Some(fragment) => Emit { effects: vec![Effect::ClipboardWrite { fragment }], ..Default::default() },
                        None => Emit::default(),
                    };
                    let Some(completion) = self.work.completion.as_ref() else { return puzzle5d_job_fault(cx, "puzzle5d copy lost its completion authority") };
                    if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(Puzzle5dPendingCompletionRejection::new(Puzzle5dCompletionOwnerKind::Copy, rejected));
                        return puzzle5d_job_fault(cx, message);
                    }
                    self.completed = true;
                }
            }
        }
        let Some(output) = self.work.commit.take_output() else { return puzzle5d_job_fault(cx, "puzzle5d copy lost its exact admitted envelope") };
        StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output })
    }

    fn begin_close(&mut self) {
        self.work.closing = true;
        self.work.commit.begin_close();
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

impl ArtifactReservedJob for Puzzle5dCopyJob {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            let step = rejected.close_step(maximum_items, maximum_bytes)?;
            if step == PluginCloseStep::Complete {
                self.pending_completion_rejection = None;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        self.work.close_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.pending_completion_rejection.is_none() && self.work.terminal_is_empty()
    }
}

struct Puzzle5dCutJob {
    work: Puzzle5dClipboardWork,
    pending_completion_rejection: Option<Puzzle5dPendingCompletionRejection>,
    completed: bool,
}

impl InteractiveJob for Puzzle5dCutJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle5d_job_fault(cx, "puzzle5d cut completion remains rejected");
        }
        if !self.completed {
            match self.work.step_work(cx) {
                Ok(Puzzle5dClipboardWorkStep::Outcome(outcome)) => return outcome,
                Ok(Puzzle5dClipboardWorkStep::Pending) => return self.work.checkpoint(cx),
                Err(error) => return puzzle5d_job_fault(cx, error),
                Ok(Puzzle5dClipboardWorkStep::Complete) => {
                    match self.work.commit.prepare(&self.work.raw, cx) {
                        Ok(false) => return StepOutcome::Yield,
                        Err(error) => return puzzle5d_job_fault(cx, error),
                        Ok(true) => {}
                    }
                    // 🔒️ A LOCKED part is copied but never removed: a lock exists precisely to refuse a
                    // destructive gesture, and a cut that deleted it anyway would be the one clipboard
                    // path that ignores it. Its fasteners stay too — disconnecting an edge whose part
                    // survives would leave the document half-cut.
                    let locked: HashSet<&str> = self.work.scan.parts.iter().filter(|part| part.part_2d.locked.unwrap_or(false)).map(|part| part.id.as_str()).collect();
                    let mut mutations = self
                        .work
                        .scan
                        .fasteners
                        .iter()
                        .filter(|fastener| !locked.contains(owning_part_id_local(&fastener.source)) && !locked.contains(owning_part_id_local(&fastener.target)))
                        .map(|fastener| crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(fastener.id.clone()))
                        .collect::<Vec<_>>();
                    mutations.extend(self.work.scan.parts.iter().filter(|part| !locked.contains(part.id.as_str())).map(|part| crate::standards::v1::subsets::any::schema::mutations::delete_part(part.id.clone())));
                    let effects = self.work.fragment().map(|fragment| vec![Effect::ClipboardWrite { fragment }]).unwrap_or_default();
                    let emit = Emit { artifact_mutations: mutations, effects, ..Default::default() };
                    let Some(completion) = self.work.completion.as_ref() else { return puzzle5d_job_fault(cx, "puzzle5d cut lost its completion authority") };
                    if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(Puzzle5dPendingCompletionRejection::new(Puzzle5dCompletionOwnerKind::Cut, rejected));
                        return puzzle5d_job_fault(cx, message);
                    }
                    self.completed = true;
                }
            }
        }
        let Some(output) = self.work.commit.take_output() else { return puzzle5d_job_fault(cx, "puzzle5d cut lost its exact admitted envelope") };
        StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output })
    }

    fn begin_close(&mut self) {
        self.work.closing = true;
        self.work.commit.begin_close();
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

impl ArtifactReservedJob for Puzzle5dCutJob {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            let step = rejected.close_step(maximum_items, maximum_bytes)?;
            if step == PluginCloseStep::Complete {
                self.pending_completion_rejection = None;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        self.work.close_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.pending_completion_rejection.is_none() && self.work.terminal_is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dPasteStage {
    Envelope,
    Decode,
    FragmentParts,
    TargetParts,
    TargetFasteners,
    MaterializeParts,
    MaterializeFasteners,
    Complete,
}

struct Puzzle5dPasteJob {
    raw: Vec<u8>,
    raw_cursor: usize,
    raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES],
    raw_page_len: usize,
    progress: u64,
    stage: Puzzle5dPasteStage,
    snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    args: Option<serde_json::Value>,
    fragment_value: Option<serde_json::Value>,
    fragment_parts: Vec<Puzzle5dPart>,
    cursor: usize,
    source_sum: (f64, f64),
    target_sum: (f64, f64),
    target_count: usize,
    placement: PastePlacement,
    delta: (f64, f64),
    id_map: HashMap<String, String>,
    fresh_ids: Puzzle5dFreshIds,
    mutations: Vec<Puzzle5dMutation>,
    completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    pending_completion_rejection: Option<Puzzle5dPendingCompletionRejection>,
    commit: Puzzle5dCommitEnvelope,
    completed: bool,
    retirement_key: [u8; PUZZLE5D_JSON_RETIREMENT_KEY_BYTES],
    closing: bool,
}

impl Puzzle5dPasteJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, args: Option<serde_json::Value>) -> Self {
        Self {
            raw: request.raw_wire,
            raw_cursor: 0,
            raw_page: [0; PUZZLE5D_RESERVED_PAGE_BYTES],
            raw_page_len: 0,
            progress: 0,
            stage: Puzzle5dPasteStage::Envelope,
            snapshot: Some(request.snapshot),
            args,
            fragment_value: None,
            fragment_parts: Vec::new(),
            cursor: 0,
            source_sum: (0.0, 0.0),
            target_sum: (0.0, 0.0),
            target_count: 0,
            placement: PastePlacement::default(),
            delta: (0.0, 0.0),
            id_map: HashMap::new(),
            fresh_ids: Puzzle5dFreshIds::default(),
            mutations: Vec::new(),
            completion: Some(request.completion),
            pending_completion_rejection: None,
            commit: Puzzle5dCommitEnvelope::new(),
            completed: false,
            retirement_key: [0; PUZZLE5D_JSON_RETIREMENT_KEY_BYTES],
            closing: false,
        }
    }

    fn checkpoint(&self, cx: &mut StepContext<'_>) -> StepOutcome {
        puzzle5d_job_checkpoint(self.stage as u8, self.cursor, self.progress, cx)
    }
}

impl InteractiveJob for Puzzle5dPasteJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle5d_job_fault(cx, "puzzle5d paste completion remains rejected");
        }
        match self.stage {
            Puzzle5dPasteStage::Envelope => {
                if let Some(outcome) = puzzle5d_step_envelope(&self.raw, &mut self.raw_cursor, &mut self.raw_page, &mut self.raw_page_len, &mut self.progress, cx) {
                    return outcome;
                }
                self.stage = Puzzle5dPasteStage::Decode;
            }
            Puzzle5dPasteStage::Decode => {
                let Some(args) = self.args.as_ref() else {
                    self.stage = Puzzle5dPasteStage::Complete;
                    return self.checkpoint(cx);
                };
                let Some(fragment_value) = args.get("fragment").cloned() else {
                    self.stage = Puzzle5dPasteStage::Complete;
                    return self.checkpoint(cx);
                };
                let fragment: ClipboardFragment = match serde_json::from_value(fragment_value) {
                    Ok(fragment) => fragment,
                    Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                };
                if fragment.media_type != (MediaType { class: MediaClass::Kit, form: MediaForm::Design }) {
                    return puzzle5d_job_fault(cx, "puzzle5d paste received an incompatible media type");
                }
                if fragment.dsl_text.len() > PUZZLE5D_RESERVED_RAW_BYTES {
                    return puzzle5d_job_fault(cx, "puzzle5d paste fragment exceeds its predecode cap");
                }
                self.fragment_value = match serde_json::from_str(&fragment.dsl_text) {
                    Ok(value) => Some(value),
                    Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                };
                self.placement = serde_json::from_value(serde_json::json!({
                    "anchor": args.get("anchor").cloned().unwrap_or_else(|| serde_json::json!("original")),
                    "position": args.get("position").cloned()
                }))
                .unwrap_or_default();
                self.stage = Puzzle5dPasteStage::FragmentParts;
                self.cursor = 0;
            }
            Puzzle5dPasteStage::FragmentParts => {
                let rows = self.fragment_value.as_ref().and_then(|value| value.get("parts")).and_then(serde_json::Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let part: Puzzle5dPart = match serde_json::from_value(row) {
                        Ok(part) => part,
                        Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                    };
                    self.source_sum.0 += part.part_2d.x;
                    self.source_sum.1 += part.part_2d.y;
                    self.fragment_parts.push(part);
                } else {
                    self.stage = Puzzle5dPasteStage::TargetParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::TargetParts => {
                let rows = self.snapshot.as_ref().and_then(|snapshot| snapshot.0.get("parts")).and_then(serde_json::Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor) {
                    self.cursor += 1;
                    if let Some(id) = row.get("id").and_then(serde_json::Value::as_str) {
                        self.fresh_ids.observe_part(id);
                    }
                    self.target_sum.0 += row.get("2d").and_then(|value| value.get("x")).and_then(serde_json::Value::as_f64).unwrap_or_default();
                    self.target_sum.1 += row.get("2d").and_then(|value| value.get("y")).and_then(serde_json::Value::as_f64).unwrap_or_default();
                    self.target_count += 1;
                } else {
                    let offset = self.placement.position.map_or((0.0, 0.0), |position| (position[0], position[1]));
                    self.delta = if matches!(self.placement.anchor, PasteAnchor::Original) || self.fragment_parts.is_empty() || self.target_count == 0 {
                        offset
                    } else {
                        (self.target_sum.0 / self.target_count as f64 - self.source_sum.0 / self.fragment_parts.len() as f64 + offset.0, self.target_sum.1 / self.target_count as f64 - self.source_sum.1 / self.fragment_parts.len() as f64 + offset.1)
                    };
                    self.stage = Puzzle5dPasteStage::TargetFasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::TargetFasteners => {
                let rows = self.snapshot.as_ref().and_then(|snapshot| snapshot.0.get("fasteners")).and_then(serde_json::Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor) {
                    self.cursor += 1;
                    if let Some(id) = row.get("id").and_then(serde_json::Value::as_str) {
                        self.fresh_ids.observe_fastener(id);
                    }
                } else {
                    self.stage = Puzzle5dPasteStage::MaterializeParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::MaterializeParts => {
                if let Some(part) = self.fragment_parts.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let fresh_id = self.fresh_ids.next_part();
                    self.id_map.insert(part.id.clone(), fresh_id.clone());
                    let mut next = part;
                    next.id = fresh_id;
                    next.part_2d.x += self.delta.0;
                    next.part_2d.y += self.delta.1;
                    next.part_3d.origin[0] += self.delta.0;
                    next.part_3d.origin[1] += self.delta.1;
                    let typed = match <crate::Puzzle5dPart as dsl::FromValue>::from_value(dsl::DslValue::from(&serde_json::to_value(&next).unwrap_or(serde_json::Value::Null))) {
                        Ok(typed) => typed,
                        Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                    };
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::create_part(typed, None));
                } else {
                    self.stage = Puzzle5dPasteStage::MaterializeFasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::MaterializeFasteners => {
                let rows = self.fragment_value.as_ref().and_then(|value| value.get("fasteners")).and_then(serde_json::Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let fastener: Puzzle5dFastener = match serde_json::from_value(row) {
                        Ok(fastener) => fastener,
                        Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                    };
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(
                        self.fresh_ids.next_fastener(),
                        rewrite_grip_ref_local(&fastener.source, &self.id_map),
                        rewrite_grip_ref_local(&fastener.target, &self.id_map),
                        fastener.fastener_kind,
                        fastener.gap,
                        fastener.shift,
                        fastener.rise,
                        fastener.rotation,
                        fastener.turn,
                        fastener.tilt,
                        fastener.x + self.delta.0,
                        fastener.y + self.delta.1,
                    ));
                } else {
                    self.stage = Puzzle5dPasteStage::Complete;
                }
            }
            Puzzle5dPasteStage::Complete => {
                match self.commit.prepare(&self.raw, cx) {
                    Ok(false) => return StepOutcome::Yield,
                    Err(error) => return puzzle5d_job_fault(cx, error),
                    Ok(true) => {}
                }
                if !self.completed {
                    // 🕹️ The pasted parts BECOME the selection: a paste whose result is invisible until the
                    // user hunts for it is indistinguishable from one that landed nowhere, and the fresh ids
                    // are the only handle on it. Sorted, so one fragment always re-selects in one order.
                    let mut fresh: Vec<String> = self.id_map.values().cloned().collect();
                    fresh.sort();
                    let emit = Emit { artifact_mutations: std::mem::take(&mut self.mutations), interaction_writes: vec![InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, fresh)], ..Default::default() };
                    let Some(completion) = self.completion.as_ref() else { return puzzle5d_job_fault(cx, "puzzle5d paste lost its completion authority") };
                    if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(Puzzle5dPendingCompletionRejection::new(Puzzle5dCompletionOwnerKind::Paste, rejected));
                        return puzzle5d_job_fault(cx, message);
                    }
                    self.completed = true;
                }
                let Some(output) = self.commit.take_output() else { return puzzle5d_job_fault(cx, "puzzle5d paste lost its exact admitted envelope") };
                return StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output });
            }
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        self.checkpoint(cx)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.commit.begin_close();
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

impl ArtifactReservedJob for Puzzle5dPasteJob {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        self.closing = true;
        if maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            let step = rejected.close_step(maximum_items, maximum_bytes)?;
            if step == PluginCloseStep::Complete {
                self.pending_completion_rejection = None;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        match self.commit.close_step(maximum_items, maximum_bytes) {
            PluginCloseStep::Complete => {}
            step => return Ok(step),
        }
        if let Some(value) = self.args.as_mut() {
            if let Some(step) = puzzle5d_retire_json_step(value, &mut self.retirement_key, maximum_bytes)? {
                return Ok(step);
            }
            self.args = None;
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(value) = self.fragment_value.as_mut() {
            if let Some(step) = puzzle5d_retire_json_step(value, &mut self.retirement_key, maximum_bytes)? {
                return Ok(step);
            }
            self.fragment_value = None;
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(part) = self.fragment_parts.last_mut() {
            if let Some(scale) = part.part_3d.scale.take() {
                // 🌉️ `scale` is always bounded (a bare number or a short per-axis array), so this
                // disposes it with the same flat capacity-credit idiom as the sibling grip/part
                // disposal arms below rather than the shared recursive `puzzle5d_retire_json_step`.
                let bytes = match &scale {
                    serde_json::Value::Array(values) => values.capacity().saturating_mul(size_of::<serde_json::Value>()),
                    _ => size_of::<serde_json::Value>(),
                };
                if bytes > maximum_bytes {
                    part.part_3d.scale = Some(scale);
                    return Err(Fault::from("puzzle5d paste part scale exceeds its bounded disposal byte slice"));
                }
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
            }
            if let Some(grip) = part.grips.pop() {
                let bytes = grip.id.capacity().saturating_add(grip.grip_kind.capacity()).saturating_add(grip.grip_2d.grip_kind.capacity()).saturating_add(grip.grip_3d.label.as_ref().map_or(0, String::capacity));
                if bytes > maximum_bytes {
                    part.grips.push(grip);
                    return Err(Fault::from("puzzle5d paste grip exceeds its bounded disposal byte slice"));
                }
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
            }
            let part = self.fragment_parts.pop().ok_or_else(|| Fault::from("puzzle5d paste part changed during retirement"))?;
            let bytes = part.id.capacity().saturating_add(part.part_kind.capacity()).saturating_add(part.part_2d.shape.capacity()).saturating_add(part.part_2d.text.capacity());
            if bytes > maximum_bytes {
                self.fragment_parts.push(part);
                return Err(Fault::from("puzzle5d paste part exceeds its bounded disposal byte slice"));
            }
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        let mapping = {
            let mut mappings = self.id_map.extract_if(|_, _| true);
            mappings.next()
        };
        if let Some((source, target)) = mapping {
            let bytes = source.capacity().saturating_add(target.capacity());
            if bytes > maximum_bytes {
                self.id_map.insert(source, target);
                return Err(Fault::from("puzzle5d paste id mapping exceeds its bounded disposal byte slice"));
            }
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if self.id_map.is_empty() && self.id_map.capacity() != 0 {
            let bytes = self.id_map.capacity().saturating_mul(size_of::<(String, String)>());
            if bytes > maximum_bytes {
                return Err(Fault::from("puzzle5d paste id map backing exceeds its bounded disposal byte slice"));
            }
            self.id_map.shrink_to_fit();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(mutation) = self.mutations.last() {
            let bytes = size_of_val(mutation);
            if bytes > maximum_bytes {
                return Err(Fault::from("puzzle5d paste mutation exceeds its bounded disposal byte slice"));
            }
            self.mutations.pop();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(step) = puzzle5d_retire_vec_backing(&mut self.fragment_parts, maximum_bytes)? {
            return Ok(step);
        }
        if let Some(step) = puzzle5d_retire_vec_backing(&mut self.mutations, maximum_bytes)? {
            return Ok(step);
        }
        if !self.raw.is_empty() && maximum_bytes == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.raw.pop().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 1 });
        }
        if let Some(step) = puzzle5d_retire_vec_backing(&mut self.raw, maximum_bytes)? {
            return Ok(step);
        }
        if self.snapshot.as_ref().is_some_and(|snapshot| std::sync::Arc::strong_count(snapshot) == 1) {
            return Ok(PluginCloseStep::Blocked { reason: "puzzle5d paste snapshot has no mounted retained authority" });
        }
        if self.snapshot.take().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.completion.as_ref().is_some_and(|completion| !completion.has_mounted_consumer()) {
            return Ok(PluginCloseStep::Blocked { reason: "puzzle5d paste completion has no mounted consumer authority" });
        }
        if self.completion.take().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.raw.is_empty()
            && self.raw.capacity() == 0
            && self.snapshot.is_none()
            && self.args.is_none()
            && self.fragment_value.is_none()
            && self.fragment_parts.is_empty()
            && self.fragment_parts.capacity() == 0
            && self.id_map.is_empty()
            && self.id_map.capacity() == 0
            && self.mutations.is_empty()
            && self.mutations.capacity() == 0
            && self.pending_completion_rejection.is_none()
            && self.completion.is_none()
            && self.commit.terminal_is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dImportStage {
    Envelope,
    Decode,
    CensusParts,
    ReserveCatalogParts,
    ReserveCatalogGrips,
    ReserveCatalogFasteners,
    ReserveCatalogRopes,
    ReserveCompatibility,
    ReservePartIndex,
    ReserveGripIndex,
    ReserveCompatibilityIndex,
    ReserveMutations,
    LoadCatalogParts,
    LoadCatalogGrips,
    LoadCatalogFasteners,
    LoadCatalogRopes,
    LoadCompatibility,
    IndexParts,
    IndexGrips,
    IndexCompatibility,
    Parts,
    PartReserve,
    PartVortices,
    PartPublish,
    Grips,
    Compatibility,
    CatalogMutation,
    Complete,
}

struct Puzzle5dImportJob {
    raw: Vec<u8>,
    raw_cursor: usize,
    raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES],
    raw_page_len: usize,
    progress: u64,
    stage: Puzzle5dImportStage,
    port: String,
    media_json: Option<String>,
    snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    fragment: Option<Value>,
    catalogs: crate::Puzzle5dKindCatalogs,
    had_catalogs: bool,
    catalog_changed: bool,
    compatibility: Vec<crate::Puzzle5dKindCompatibility>,
    part_index: Vec<(String, usize)>,
    grip_index: Vec<(String, usize)>,
    compatibility_index: Vec<((String, String), usize)>,
    mutation_pages: [Vec<Puzzle5dMutation>; PUZZLE5D_IMPORT_MUTATION_PAGES],
    cursor: usize,
    nested_cursor: usize,
    decoded_items: usize,
    current_part: Option<crate::Puzzle5dCatalogPartKind>,
    completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    pending_completion_rejection: Option<Puzzle5dPendingCompletionRejection>,
    commit: Puzzle5dCommitEnvelope,
    completed: bool,
    retiring_index_primary: Option<String>,
    retiring_index_secondary: Option<String>,
    closing: bool,
}

fn puzzle5d_import_vec3(value: Option<&Value>) -> Result<[f64; 3], &'static str> {
    let values = value.and_then(Value::as_array).ok_or("puzzle5d kit:in vector is not an array")?;
    if values.len() != 3 {
        return Err("puzzle5d kit:in vector must contain exactly three coordinates");
    }
    let mut result = [0.0; 3];
    for (index, target) in result.iter_mut().enumerate() {
        *target = values.get(index).and_then(Value::as_f64).filter(|value| value.is_finite()).ok_or("puzzle5d kit:in vector contains a non-finite coordinate")?;
    }
    Ok(result)
}

fn puzzle5d_import_keys_are(value: &Value, allowed: &[&str]) -> bool {
    value.as_object().is_some_and(|object| object.iter().all(|(key, _)| allowed.contains(&key)))
}

fn puzzle5d_decode_import_fragment(media_json: &str) -> Result<Value, String> {
    if media_json.len() > PUZZLE5D_IMPORT_MEDIA_BYTES {
        return Err("puzzle5d kit:in payload exceeds its predecode cap".into());
    }
    parse(media_json).map_err(|error| error.to_string())
}

fn puzzle5d_retire_string_step(owner: &mut String, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    if let Some(bytes) = owner.chars().next_back().map(char::len_utf8) {
        if bytes > maximum_bytes {
            return Ok(Some(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }));
        }
        owner.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if owner.capacity() == 0 {
        return Ok(None);
    }
    let bytes = owner.capacity();
    if bytes > maximum_bytes {
        return Err(Fault::from("puzzle5d import string backing exceeds its bounded disposal byte slice"));
    }
    *owner = String::new();
    Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }))
}

fn puzzle5d_retire_optional_string_step(owner: &mut Option<String>, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    let Some(value) = owner.as_mut() else { return Ok(None) };
    if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
        return Ok(Some(step));
    }
    *owner = None;
    Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }))
}

fn puzzle5d_retire_string_vec_step(owners: &mut Vec<String>, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    if let Some(owner) = owners.last_mut() {
        if let Some(step) = puzzle5d_retire_string_step(owner, maximum_bytes)? {
            return Ok(Some(step));
        }
        owners.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    puzzle5d_retire_vec_backing(owners, maximum_bytes)
}

fn puzzle5d_retire_representation_step(owner: &mut crate::Puzzle5dRepresentation, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    if let Some(step) = puzzle5d_retire_string_vec_step(&mut owner.tags, maximum_bytes)? {
        return Ok(Some(step));
    }
    for value in [&mut owner.id, &mut owner.name, &mut owner.url, &mut owner.mime, &mut owner.description] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    puzzle5d_retire_optional_string_step(&mut owner.lod, maximum_bytes)
}

fn puzzle5d_retire_grip_template_step(owner: &mut crate::Puzzle5dGripTemplate, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    for value in [&mut owner.id, &mut owner.name, &mut owner.label, &mut owner.description, &mut owner.icon] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    puzzle5d_retire_optional_string_step(&mut owner.grip_kind, maximum_bytes)
}

fn puzzle5d_retire_attribute_step(owner: &mut crate::Puzzle5dAttribute, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    for value in [&mut owner.id, &mut owner.key, &mut owner.value] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    puzzle5d_retire_optional_string_step(&mut owner.definition, maximum_bytes)
}

fn puzzle5d_retire_author_step(owner: &mut crate::Puzzle5dAuthor, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    for value in [&mut owner.id, &mut owner.name, &mut owner.email] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    puzzle5d_retire_optional_string_step(&mut owner.role, maximum_bytes)
}

fn puzzle5d_retire_part_kind_step(owner: &mut crate::Puzzle5dCatalogPartKind, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    if let Some(step) = puzzle5d_retire_string_vec_step(&mut owner.base_kinds, maximum_bytes)? {
        return Ok(Some(step));
    }
    if let Some(value) = owner.representations.last_mut() {
        if let Some(step) = puzzle5d_retire_representation_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
        owner.representations.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.representations, maximum_bytes)? {
        return Ok(Some(step));
    }
    if let Some(value) = owner.grips.last_mut() {
        if let Some(step) = puzzle5d_retire_grip_template_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
        owner.grips.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.grips, maximum_bytes)? {
        return Ok(Some(step));
    }
    if let Some(value) = owner.attributes.last_mut() {
        if let Some(step) = puzzle5d_retire_attribute_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
        owner.attributes.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.attributes, maximum_bytes)? {
        return Ok(Some(step));
    }
    if let Some(value) = owner.authors.last_mut() {
        if let Some(step) = puzzle5d_retire_author_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
        owner.authors.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.authors, maximum_bytes)? {
        return Ok(Some(step));
    }
    for value in [&mut owner.id, &mut owner.name, &mut owner.label, &mut owner.description, &mut owner.icon, &mut owner.image, &mut owner.unit] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    Ok(None)
}

fn puzzle5d_retire_grip_kind_step(owner: &mut crate::Puzzle5dCatalogGripKind, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    if let Some(step) = puzzle5d_retire_string_vec_step(&mut owner.compatible_with, maximum_bytes)? {
        return Ok(Some(step));
    }
    for value in [&mut owner.id, &mut owner.description, &mut owner.icon, &mut owner.color, &mut owner.default_rope_kind] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    if let Some(step) = puzzle5d_retire_optional_string_step(&mut owner.code, maximum_bytes)? {
        return Ok(Some(step));
    }
    puzzle5d_retire_optional_string_step(&mut owner.label, maximum_bytes)
}

fn puzzle5d_retire_fastener_kind_step(owner: &mut crate::Puzzle5dCatalogFastenerKind, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    for value in [&mut owner.id, &mut owner.name] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    puzzle5d_retire_optional_string_step(&mut owner.label, maximum_bytes)
}

fn puzzle5d_retire_rope_kind_step(owner: &mut crate::Puzzle5dCatalogRopeKind, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    for value in [&mut owner.id, &mut owner.name, &mut owner.label, &mut owner.default_fastener_kind] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    Ok(None)
}

fn puzzle5d_retire_compatibility_step(owner: &mut crate::Puzzle5dKindCompatibility, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    for value in [&mut owner.source, &mut owner.target] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    Ok(None)
}

fn puzzle5d_retire_catalogs_step(owner: &mut crate::Puzzle5dKindCatalogs, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    if let Some(value) = owner.parts.last_mut() {
        if let Some(step) = puzzle5d_retire_part_kind_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
        owner.parts.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if let Some(value) = owner.grips.last_mut() {
        if let Some(step) = puzzle5d_retire_grip_kind_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
        owner.grips.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if let Some(value) = owner.fasteners.last_mut() {
        if let Some(step) = puzzle5d_retire_fastener_kind_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
        owner.fasteners.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if let Some(value) = owner.ropes.last_mut() {
        if let Some(step) = puzzle5d_retire_rope_kind_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
        owner.ropes.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.parts, maximum_bytes)? {
        return Ok(Some(step));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.grips, maximum_bytes)? {
        return Ok(Some(step));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.fasteners, maximum_bytes)? {
        return Ok(Some(step));
    }
    puzzle5d_retire_vec_backing(&mut owner.ropes, maximum_bytes)
}

fn puzzle5d_retire_import_mutation_step(owner: &mut Puzzle5dMutation, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    match owner {
        Puzzle5dMutation::ConnectKindCompatibility(value) => {
            if let Some(step) = puzzle5d_retire_string_step(&mut value.source, maximum_bytes)? {
                return Ok(Some(step));
            }
            puzzle5d_retire_string_step(&mut value.target, maximum_bytes)
        }
        Puzzle5dMutation::DisconnectKindCompatibility(value) => {
            if let Some(step) = puzzle5d_retire_string_step(&mut value.source, maximum_bytes)? {
                return Ok(Some(step));
            }
            puzzle5d_retire_string_step(&mut value.target, maximum_bytes)
        }
        Puzzle5dMutation::ReplaceKindCatalogs(value) => {
            let Some(catalogs) = value.new_catalogs.as_mut() else { return Ok(None) };
            if let Some(step) = puzzle5d_retire_catalogs_step(catalogs, maximum_bytes)? {
                return Ok(Some(step));
            }
            value.new_catalogs = None;
            Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }))
        }
        _ => Err(Fault::from("puzzle5d import retained an unexpected mutation owner before publication")),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dCompletionOwnerKind {
    Copy,
    Cut,
    Paste,
    Import,
}

fn puzzle5d_retire_typed_part_step(owner: &mut crate::Puzzle5dPart, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    if let Some(grip) = owner.grips.last_mut() {
        if let Some(step) = puzzle5d_retire_string_step(&mut grip.id, maximum_bytes)? {
            return Ok(Some(step));
        }
        if let Some(step) = puzzle5d_retire_optional_string_step(&mut grip.grip_kind, maximum_bytes)? {
            return Ok(Some(step));
        }
        if let Some(step) = puzzle5d_retire_optional_string_step(&mut grip.grip_2d.grip_kind, maximum_bytes)? {
            return Ok(Some(step));
        }
        if let Some(step) = puzzle5d_retire_optional_string_step(&mut grip.grip_3d.label, maximum_bytes)? {
            return Ok(Some(step));
        }
        owner.grips.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.grips, maximum_bytes)? {
        return Ok(Some(step));
    }
    if let Some(step) = puzzle5d_retire_string_step(&mut owner.id, maximum_bytes)? {
        return Ok(Some(step));
    }
    if let Some(step) = puzzle5d_retire_optional_string_step(&mut owner.part_kind, maximum_bytes)? {
        return Ok(Some(step));
    }
    for value in [&mut owner.part_2d.shape, &mut owner.part_2d.text, &mut owner.part_2d.icon_kind, &mut owner.part_3d.mesh_url, &mut owner.part_3d.label] {
        if let Some(step) = puzzle5d_retire_optional_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    Ok(None)
}

fn puzzle5d_retire_completion_mutation_step(kind: Puzzle5dCompletionOwnerKind, owner: &mut Puzzle5dMutation, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    match (kind, owner) {
        (Puzzle5dCompletionOwnerKind::Cut, Puzzle5dMutation::DisconnectGrips(value)) => puzzle5d_retire_string_step(&mut value.id, maximum_bytes),
        (Puzzle5dCompletionOwnerKind::Cut, Puzzle5dMutation::DeletePart(value)) => puzzle5d_retire_string_step(&mut value.id, maximum_bytes),
        (Puzzle5dCompletionOwnerKind::Paste, Puzzle5dMutation::CreatePart(value)) => puzzle5d_retire_typed_part_step(&mut value.part, maximum_bytes),
        (Puzzle5dCompletionOwnerKind::Paste, Puzzle5dMutation::ConnectGrips(value)) => {
            for text in [&mut value.id, &mut value.source, &mut value.target] {
                if let Some(step) = puzzle5d_retire_string_step(text, maximum_bytes)? {
                    return Ok(Some(step));
                }
            }
            puzzle5d_retire_optional_string_step(&mut value.fastener_kind, maximum_bytes)
        }
        (Puzzle5dCompletionOwnerKind::Import, owner) => puzzle5d_retire_import_mutation_step(owner, maximum_bytes),
        _ => Err(Fault::from("puzzle5d completion rejection retained an unexpected mutation owner")),
    }
}

fn puzzle5d_retire_clipboard_fragment_step(owner: &mut ClipboardFragment, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    for text in [&mut owner.schema, &mut owner.dsl_text, &mut owner.source_app, &mut owner.label] {
        if let Some(step) = puzzle5d_retire_string_step(text, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    let Some(bytes) = owner.pack_bytes.as_mut() else { return Ok(None) };
    if bytes.pop().is_some() {
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 1 }));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(bytes, maximum_bytes)? {
        return Ok(Some(step));
    }
    owner.pack_bytes = None;
    Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }))
}

fn puzzle5d_retire_fault_step(owner: &mut Fault, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    if let Some(cause) = owner.causes.last_mut() {
        if let Some(step) = puzzle5d_retire_string_step(&mut cause.message, maximum_bytes)? {
            return Ok(Some(step));
        }
        if let Some(code) = cause.code.as_mut() {
            if let Some(step) = puzzle5d_retire_string_step(&mut code.0, maximum_bytes)? {
                return Ok(Some(step));
            }
            cause.code = None;
            return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
        }
        owner.causes.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.causes, maximum_bytes)? {
        return Ok(Some(step));
    }
    for field in [
        &mut owner.scope.plugin_id,
        &mut owner.scope.app_id,
        &mut owner.scope.instance_id,
        &mut owner.scope.module,
        &mut owner.scope.body_key,
    ] {
        if let Some(step) = puzzle5d_retire_optional_string_step(field, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    if let Some(step) = puzzle5d_retire_string_step(&mut owner.message, maximum_bytes)? {
        return Ok(Some(step));
    }
    if let Some(step) = puzzle5d_retire_string_step(&mut owner.code.0, maximum_bytes)? {
        return Ok(Some(step));
    }
    if owner.span.take().is_some() {
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    Ok(None)
}

fn puzzle5d_retire_completion_effect_step(owner: &mut Effect, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    match owner {
        Effect::ClipboardWrite { fragment } => puzzle5d_retire_clipboard_fragment_step(fragment, maximum_bytes),
        _ => Err(Fault::from("puzzle5d completion rejection retained an unexpected effect owner")),
    }
}

fn puzzle5d_retire_completion_emit_step(
    kind: Puzzle5dCompletionOwnerKind,
    owner: &mut Emit<Puzzle5dMutation, Puzzle5dConfigMutation, NoDraftMutation>,
    maximum_items: usize,
    maximum_bytes: usize,
) -> Result<Option<PluginCloseStep>, Fault> {
    if let Some(step) = owner.close_child_one(maximum_items, maximum_bytes) {
        return Ok(Some(step));
    }
    if let Some(mutation) = owner.artifact_mutations.last_mut() {
        if let Some(step) = puzzle5d_retire_completion_mutation_step(kind, mutation, maximum_bytes)? {
            return Ok(Some(step));
        }
        owner.artifact_mutations.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.artifact_mutations, maximum_bytes)? {
        return Ok(Some(step));
    }
    if !owner.config_mutations.is_empty() || !owner.draft_mutations.is_empty() {
        return Err(Fault::from("puzzle5d completion rejection retained an impossible non-document mutation owner"));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.config_mutations, maximum_bytes)? {
        return Ok(Some(step));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.draft_mutations, maximum_bytes)? {
        return Ok(Some(step));
    }
    if let Some(step) = puzzle5d_retire_optional_string_step(&mut owner.description, maximum_bytes)? {
        return Ok(Some(step));
    }
    if let Some(step) = puzzle5d_retire_optional_string_step(&mut owner.coalesce_key, maximum_bytes)? {
        return Ok(Some(step));
    }
    if let Some(effect) = owner.effects.last_mut() {
        if let Some(step) = puzzle5d_retire_completion_effect_step(effect, maximum_bytes)? {
            return Ok(Some(step));
        }
        owner.effects.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.effects, maximum_bytes)? {
        return Ok(Some(step));
    }
    if !owner.events.is_empty() {
        return Err(Fault::from("puzzle5d completion rejection retained an unexpected event owner"));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.events, maximum_bytes)? {
        return Ok(Some(step));
    }
    puzzle5d_retire_vec_backing(&mut owner.child_emits, maximum_bytes)
}

fn puzzle5d_retire_completion_ephemeral_step(owner: &mut EphemeralEmit<EditorApp<Puzzle5dPlayApp>>, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    if matches!(owner.presence.last(), Some(Puzzle5dPresenceMutation::Snapshot { .. })) {
        owner.presence.pop();
        return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
    }
    if let Some(step) = puzzle5d_retire_vec_backing(&mut owner.presence, maximum_bytes)? {
        return Ok(Some(step));
    }
    if !owner.transient.is_empty() {
        return Err(Fault::from("puzzle5d completion rejection retained an impossible transient owner"));
    }
    puzzle5d_retire_vec_backing(&mut owner.transient, maximum_bytes)
}

struct Puzzle5dPendingCompletionRejection {
    owner: Option<ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
    kind: Puzzle5dCompletionOwnerKind,
    emit_closed: bool,
    ephemeral_closed: bool,
    fault_closed: bool,
}

impl Puzzle5dPendingCompletionRejection {
    fn new(kind: Puzzle5dCompletionOwnerKind, owner: ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>) -> Self {
        Self { owner: Some(owner), kind, emit_closed: false, ephemeral_closed: false, fault_closed: false }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(owner) = self.owner.as_mut() else { return Ok(PluginCloseStep::Complete) };
        if !self.emit_closed {
            let step = match owner.emit.as_mut() {
                Ok(emit) => puzzle5d_retire_completion_emit_step(self.kind, emit, maximum_items, maximum_bytes)?,
                Err(fault) => puzzle5d_retire_fault_step(fault, maximum_bytes)?,
            };
            if let Some(step) = step {
                return Ok(step);
            }
            self.emit_closed = true;
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if !self.ephemeral_closed {
            if let Some(step) = puzzle5d_retire_completion_ephemeral_step(&mut owner.ephemeral, maximum_bytes)? {
                return Ok(step);
            }
            self.ephemeral_closed = true;
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if !self.fault_closed {
            if let Some(step) = puzzle5d_retire_fault_step(&mut owner.fault, maximum_bytes)? {
                return Ok(step);
            }
            self.fault_closed = true;
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.owner = None;
        Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
    }
}

impl Puzzle5dImportJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, port: String, media: Media) -> Self {
        let media_json = match media.payload {
            semio_framework_plugin::MediaPayload::Structured { json, .. } => Some(json),
            semio_framework_plugin::MediaPayload::Binary { .. } => None,
        };
        Self {
            raw: request.raw_wire,
            raw_cursor: 0,
            raw_page: [0; PUZZLE5D_RESERVED_PAGE_BYTES],
            raw_page_len: 0,
            progress: 0,
            stage: Puzzle5dImportStage::Envelope,
            port,
            media_json,
            snapshot: Some(request.snapshot),
            fragment: None,
            catalogs: Default::default(),
            had_catalogs: false,
            catalog_changed: false,
            compatibility: Vec::new(),
            part_index: Vec::new(),
            grip_index: Vec::new(),
            compatibility_index: Vec::new(),
            mutation_pages: std::array::from_fn(|_| Vec::new()),
            cursor: 0,
            nested_cursor: 0,
            decoded_items: 0,
            current_part: None,
            completion: Some(request.completion),
            pending_completion_rejection: None,
            commit: Puzzle5dCommitEnvelope::new(),
            completed: false,
            retiring_index_primary: None,
            retiring_index_secondary: None,
            closing: false,
        }
    }

    fn checkpoint(&self, cx: &mut StepContext<'_>) -> StepOutcome {
        puzzle5d_import_checkpoint(self.stage as u8, self.cursor, self.nested_cursor, self.decoded_items, self.progress, cx)
    }

    fn rows(&self, key: &str) -> &[Value] {
        self.fragment.as_ref().and_then(|value| value.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
    }

    fn snapshot_rows(&self, parent: &str, key: &str) -> Vec<Value> {
        self.snapshot.as_ref().map(|snapshot| puzzle5d_projection_value(&snapshot.0)).and_then(|projection| projection.get(parent).and_then(|value| value.get(key)).and_then(Value::as_array).cloned()).unwrap_or_default()
    }

    fn snapshot_kind_compatibility_rows(&self) -> Vec<Value> {
        self.snapshot.as_ref().map(|snapshot| puzzle5d_projection_value(&snapshot.0)).and_then(|projection| projection.get("kindCompatibility").and_then(Value::as_array).cloned()).unwrap_or_default()
    }

    fn push_mutation(&mut self, mutation: Puzzle5dMutation) -> Result<(), &'static str> {
        let mutation_count = self.mutation_pages.iter().map(Vec::len).sum::<usize>();
        if mutation_count >= PUZZLE5D_IMPORT_MUTATION_ITEMS {
            return Err("puzzle5d kit:in mutation limit exceeded");
        }
        let page_index = mutation_count / PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE;
        let Some(page) = self.mutation_pages.get_mut(page_index) else {
            return Err("puzzle5d kit:in mutation page limit exceeded");
        };
        if page.len() == page.capacity() {
            return Err("puzzle5d kit:in mutation page reserve exhausted");
        }
        page.push(mutation);
        Ok(())
    }
}

impl InteractiveJob for Puzzle5dImportJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle5d_job_fault(cx, "puzzle5d import completion remains rejected");
        }
        match self.stage {
            Puzzle5dImportStage::Envelope => {
                if let Some(outcome) = puzzle5d_step_envelope(&self.raw, &mut self.raw_cursor, &mut self.raw_page, &mut self.raw_page_len, &mut self.progress, cx) {
                    return outcome;
                }
                self.stage = Puzzle5dImportStage::Decode;
            }
            Puzzle5dImportStage::Decode => {
                if self.port != "kit:in" {
                    return puzzle5d_job_fault(cx, "puzzle5d import only implements kit:in");
                }
                let Some(media_json) = self.media_json.as_ref() else {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in requires a structured payload");
                };
                let fragment = match puzzle5d_decode_import_fragment(media_json) {
                    Ok(fragment) => fragment,
                    Err(error) => return puzzle5d_job_fault(cx, error),
                };
                let fragment_items = ["objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"].into_iter().try_fold(0usize, |total, key| total.checked_add(fragment.get(key).and_then(Value::as_array).map_or(0, Vec::len)));
                let snapshot_items = ["parts", "grips", "fasteners", "ropes"]
                    .into_iter()
                    .try_fold(0usize, |total, key| total.checked_add(self.snapshot_rows("kindCatalogs", key).len()))
                    .and_then(|total| total.checked_add(self.snapshot_kind_compatibility_rows().len()));
                self.decoded_items = match fragment_items.and_then(|fragment_items| snapshot_items.and_then(|snapshot_items| fragment_items.checked_add(snapshot_items))) {
                    Some(items) if items <= PUZZLE5D_IMPORT_DECODED_ITEMS => items,
                    _ => return puzzle5d_job_fault(cx, "puzzle5d kit:in decoded item limit exceeded"),
                };
                if fragment.as_object().is_none() {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in root must be an object");
                }
                if !puzzle5d_import_keys_are(&fragment, &["schema", "objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"]) {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in root contains an unknown field");
                }
                if fragment.get("schema").is_some_and(|value| value.as_str() != Some("manifest")) {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in schema must be manifest when present");
                }
                if ["objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"].into_iter().any(|key| fragment.get(key).is_some_and(|value| value.as_array().is_none())) {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in collection must be an array");
                }
                if ["objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"].into_iter().any(|key| fragment.get(key).and_then(Value::as_array).is_some_and(|rows| rows.len() > PUZZLE5D_IMPORT_SEMANTIC_ITEMS)) {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in collection exceeds its fixed-page descriptor cap");
                }
                if ["cableKinds", "attractionKinds"].into_iter().any(|key| fragment.get(key).and_then(Value::as_array).is_some_and(|rows| !rows.is_empty())) {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in cannot silently discard unmapped cable or attraction kinds");
                }
                self.had_catalogs = self.snapshot.as_ref().and_then(|snapshot| snapshot.0.get("kindCatalogs")).is_some_and(|value| !value.is_null());
                self.catalog_changed = !self.had_catalogs;
                self.fragment = Some(fragment);
                self.stage = Puzzle5dImportStage::CensusParts;
                self.cursor = 0;
            }
            Puzzle5dImportStage::CensusParts => {
                if let Some(row) = self.rows("objectKinds").get(self.cursor) {
                    if !puzzle5d_import_keys_are(row, &["id", "name", "label", "meshUrl", "vortices"]) {
                        return puzzle5d_job_fault(cx, "puzzle5d kit:in object kind contains an unknown field");
                    }
                    if row.get("vortices").is_some_and(|value| value.as_array().is_none()) {
                        return puzzle5d_job_fault(cx, "puzzle5d kit:in object-kind vortices must be an array");
                    }
                    let vortices = row.get("vortices").and_then(Value::as_array).map_or(0, Vec::len);
                    if vortices > PUZZLE5D_IMPORT_SEMANTIC_ITEMS {
                        return puzzle5d_job_fault(cx, "puzzle5d kit:in vortex collection exceeds its fixed-page descriptor cap");
                    }
                    self.decoded_items = match self.decoded_items.checked_add(vortices) {
                        Some(items) if items <= PUZZLE5D_IMPORT_DECODED_ITEMS => items,
                        _ => return puzzle5d_job_fault(cx, "puzzle5d kit:in nested vortex item limit exceeded"),
                    };
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::ReserveCatalogParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::ReserveCatalogParts => {
                let capacity = self.snapshot_rows("kindCatalogs", "parts").len().saturating_add(self.rows("objectKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.parts.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in part catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCatalogGrips;
            }
            Puzzle5dImportStage::ReserveCatalogGrips => {
                let capacity = self.snapshot_rows("kindCatalogs", "grips").len().saturating_add(self.rows("vortexKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.grips.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in grip catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCatalogFasteners;
            }
            Puzzle5dImportStage::ReserveCatalogFasteners => {
                let capacity = self.snapshot_rows("kindCatalogs", "fasteners").len();
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.fasteners.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in fastener catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCatalogRopes;
            }
            Puzzle5dImportStage::ReserveCatalogRopes => {
                let capacity = self.snapshot_rows("kindCatalogs", "ropes").len();
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.ropes.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in rope catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCompatibility;
            }
            Puzzle5dImportStage::ReserveCompatibility => {
                let capacity = self.snapshot_kind_compatibility_rows().len().saturating_add(self.rows("kindCompatibility").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.compatibility.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in compatibility reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReservePartIndex;
            }
            Puzzle5dImportStage::ReservePartIndex => {
                let capacity = self.snapshot_rows("kindCatalogs", "parts").len().saturating_add(self.rows("objectKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.part_index.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in part index reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveGripIndex;
            }
            Puzzle5dImportStage::ReserveGripIndex => {
                let capacity = self.snapshot_rows("kindCatalogs", "grips").len().saturating_add(self.rows("vortexKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.grip_index.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in grip index reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCompatibilityIndex;
            }
            Puzzle5dImportStage::ReserveCompatibilityIndex => {
                let capacity = self.snapshot_kind_compatibility_rows().len().saturating_add(self.rows("kindCompatibility").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.compatibility_index.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in compatibility index reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveMutations;
                self.nested_cursor = 0;
            }
            Puzzle5dImportStage::ReserveMutations => {
                let capacity = self.rows("kindCompatibility").len().saturating_mul(2).saturating_add(1);
                if capacity > PUZZLE5D_IMPORT_MUTATION_ITEMS {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in mutation reserve rejected");
                }
                let page_start = self.nested_cursor.saturating_mul(PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);
                if page_start < capacity {
                    let page_items = capacity.saturating_sub(page_start).min(PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);
                    let Some(page) = self.mutation_pages.get_mut(self.nested_cursor) else {
                        return puzzle5d_job_fault(cx, "puzzle5d kit:in mutation page reserve rejected");
                    };
                    if page.try_reserve_exact(page_items).is_err() {
                        return puzzle5d_job_fault(cx, "puzzle5d kit:in mutation page reserve rejected");
                    }
                    self.nested_cursor += 1;
                    return self.checkpoint(cx);
                }
                self.stage = Puzzle5dImportStage::LoadCatalogParts;
                self.cursor = 0;
                self.nested_cursor = 0;
            }
            Puzzle5dImportStage::LoadCatalogParts => {
                if let Some(row) = self.snapshot_rows("kindCatalogs", "parts").get(self.cursor) {
                    let parsed = match <crate::Puzzle5dCatalogPartKind as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                    };
                    self.catalogs.parts.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::LoadCatalogGrips;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::LoadCatalogGrips => {
                if let Some(row) = self.snapshot_rows("kindCatalogs", "grips").get(self.cursor) {
                    let parsed = match <crate::Puzzle5dCatalogGripKind as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                    };
                    self.catalogs.grips.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::LoadCatalogFasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::LoadCatalogFasteners => {
                if let Some(row) = self.snapshot_rows("kindCatalogs", "fasteners").get(self.cursor) {
                    let parsed = match <crate::Puzzle5dCatalogFastenerKind as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                    };
                    self.catalogs.fasteners.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::LoadCatalogRopes;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::LoadCatalogRopes => {
                if let Some(row) = self.snapshot_rows("kindCatalogs", "ropes").get(self.cursor) {
                    let parsed = match <crate::Puzzle5dCatalogRopeKind as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                    };
                    self.catalogs.ropes.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::LoadCompatibility;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::LoadCompatibility => {
                let rows = self.snapshot_kind_compatibility_rows();
                if let Some(row) = rows.get(self.cursor) {
                    let parsed = match <crate::Puzzle5dKindCompatibility as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                    };
                    self.compatibility.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::IndexParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::IndexParts => {
                if let Some(row) = self.catalogs.parts.get(self.cursor) {
                    self.part_index.push((row.id.clone(), self.cursor));
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::IndexGrips;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::IndexGrips => {
                if let Some(row) = self.catalogs.grips.get(self.cursor) {
                    self.grip_index.push((row.id.clone(), self.cursor));
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::IndexCompatibility;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::IndexCompatibility => {
                if let Some(row) = self.compatibility.get(self.cursor) {
                    self.compatibility_index.push(((row.source.clone(), row.target.clone()), self.cursor));
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::Parts;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::Parts => {
                if let Some(row) = self.rows("objectKinds").get(self.cursor) {
                    let id = match row.get("id").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault(cx, "puzzle5d kit:in object kind lacks id"),
                    };
                    let name = match row.get("name").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault(cx, "puzzle5d kit:in object kind lacks name"),
                    };
                    let label = match row.get("label").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault(cx, "puzzle5d kit:in object kind lacks label"),
                    };
                    let mesh_url = row.get("meshUrl").and_then(Value::as_str).map(str::to_string);
                    self.current_part = Some(crate::Puzzle5dCatalogPartKind {
                        id,
                        name,
                        label,
                        representations: mesh_url.map(|url| vec![crate::Puzzle5dRepresentation { id: "mesh".into(), name: "mesh".into(), url, mime: "model/gltf-binary".into(), ..Default::default() }]).unwrap_or_default(),
                        grips: Vec::new(),
                        ..Default::default()
                    });
                    self.nested_cursor = 0;
                    self.stage = Puzzle5dImportStage::PartReserve;
                } else {
                    self.stage = Puzzle5dImportStage::Grips;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::PartReserve => {
                let grip_count = self.rows("objectKinds").get(self.cursor).and_then(|row| row.get("vortices")).and_then(Value::as_array).map_or(0, Vec::len);
                let Some(part) = self.current_part.as_mut() else {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in lost its part before nested reserve");
                };
                if grip_count > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || part.grips.try_reserve_exact(grip_count).is_err() {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in nested grip reserve rejected");
                }
                self.stage = Puzzle5dImportStage::PartVortices;
            }
            Puzzle5dImportStage::PartVortices => {
                let vortex = self.rows("objectKinds").get(self.cursor).and_then(|row| row.get("vortices")).and_then(Value::as_array).and_then(|rows| rows.get(self.nested_cursor));
                if let Some(vortex) = vortex {
                    if !puzzle5d_import_keys_are(vortex, &["id", "vortexKind", "position", "direction", "radius"]) {
                        return puzzle5d_job_fault(cx, "puzzle5d kit:in vortex contains an unknown field");
                    }
                    let vortex_kind = match vortex.get("vortexKind").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault(cx, "puzzle5d kit:in vortex lacks kind"),
                    };
                    let point = match puzzle5d_import_vec3(vortex.get("position")) {
                        Ok(value) => value,
                        Err(error) => return puzzle5d_job_fault(cx, error),
                    };
                    let direction = match puzzle5d_import_vec3(vortex.get("direction")) {
                        Ok(value) => value,
                        Err(error) => return puzzle5d_job_fault(cx, error),
                    };
                    let radius = match vortex.get("radius").and_then(Value::as_f64).filter(|value| value.is_finite()) {
                        Some(value) => value,
                        None => return puzzle5d_job_fault(cx, "puzzle5d kit:in vortex lacks finite radius"),
                    };
                    let Some(part) = self.current_part.as_mut() else {
                        return puzzle5d_job_fault(cx, "puzzle5d kit:in lost its current part owner");
                    };
                    part.grips.push(crate::Puzzle5dGripTemplate {
                        id: format!("g{}", self.nested_cursor),
                        name: vortex_kind.clone(),
                        label: vortex_kind.clone(),
                        grip_kind: Some(vortex_kind),
                        point,
                        direction,
                        radius: Some(radius),
                        ..Default::default()
                    });
                    self.nested_cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::PartPublish;
                }
            }
            Puzzle5dImportStage::PartPublish => {
                let Some(next) = self.current_part.take() else {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in lost its completed part owner");
                };
                let id = next.id.clone();
                match self.part_index.iter().find_map(|(candidate, index)| (candidate == &id).then_some(*index)) {
                    Some(index) => self.catalogs.parts[index] = next,
                    None => {
                        self.part_index.push((id, self.catalogs.parts.len()));
                        self.catalogs.parts.push(next);
                    }
                }
                self.catalog_changed = true;
                self.cursor += 1;
                self.stage = Puzzle5dImportStage::Parts;
            }
            Puzzle5dImportStage::Grips => {
                if let Some(row) = self.rows("vortexKinds").get(self.cursor) {
                    if !puzzle5d_import_keys_are(row, &["id", "name", "label", "color", "defaultCableKind"]) {
                        return puzzle5d_job_fault(cx, "puzzle5d kit:in vortex kind contains an unknown field");
                    }
                    let id = match row.get("id").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault(cx, "puzzle5d kit:in vortex kind lacks id"),
                    };
                    let next = crate::Puzzle5dCatalogGripKind {
                        id: id.clone(),
                        code: row.get("name").and_then(Value::as_str).map(str::to_string),
                        label: row.get("label").and_then(Value::as_str).map(str::to_string),
                        color: row.get("color").and_then(Value::as_str).unwrap_or_default().to_string(),
                        default_rope_kind: row.get("defaultCableKind").and_then(Value::as_str).unwrap_or_default().to_string(),
                        ..Default::default()
                    };
                    match self.grip_index.iter().find_map(|(candidate, index)| (candidate == &id).then_some(*index)) {
                        Some(index) => self.catalogs.grips[index] = next,
                        None => {
                            self.grip_index.push((id, self.catalogs.grips.len()));
                            self.catalogs.grips.push(next);
                        }
                    }
                    self.catalog_changed = true;
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::Compatibility;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::Compatibility => {
                if let Some(row) = self.rows("kindCompatibility").get(self.cursor) {
                    if !puzzle5d_import_keys_are(row, &["source", "target", "bidirectional", "important", "specificity"]) {
                        return puzzle5d_job_fault(cx, "puzzle5d kit:in compatibility contains an unknown field");
                    }
                    let parsed = match <crate::Puzzle5dKindCompatibility as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                    };
                    let key = (parsed.source.clone(), parsed.target.clone());
                    match self.compatibility_index.iter().find_map(|(candidate, index)| (candidate == &key).then_some(*index)) {
                        Some(index) if self.compatibility[index] == parsed => {}
                        Some(index) => {
                            if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility(parsed.source.clone(), parsed.target.clone())) {
                                return puzzle5d_job_fault(cx, error);
                            }
                            if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(parsed.source.clone(), parsed.target.clone(), parsed.bidirectional, parsed.important, parsed.specificity)) {
                                return puzzle5d_job_fault(cx, error);
                            }
                            self.compatibility[index] = parsed;
                        }
                        None => {
                            if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(parsed.source.clone(), parsed.target.clone(), parsed.bidirectional, parsed.important, parsed.specificity)) {
                                return puzzle5d_job_fault(cx, error);
                            }
                            self.compatibility_index.push((key, self.compatibility.len()));
                            self.compatibility.push(parsed);
                        }
                    }
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::CatalogMutation;
                }
            }
            Puzzle5dImportStage::CatalogMutation => {
                if self.catalog_changed {
                    let mutation = crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs(Some(std::mem::take(&mut self.catalogs)));
                    if let Err(error) = self.push_mutation(mutation) {
                        return puzzle5d_job_fault(cx, error);
                    }
                    self.catalog_changed = false;
                }
                self.stage = Puzzle5dImportStage::Complete;
            }
            Puzzle5dImportStage::Complete => {
                match self.commit.prepare(&self.raw, cx) {
                    Ok(false) => return StepOutcome::Yield,
                    Err(error) => return puzzle5d_job_fault(cx, error),
                    Ok(true) => {}
                }
                if !self.completed {
                    let mut mutation_pages = std::mem::take(&mut self.mutation_pages);
                    let mut mutations = std::mem::take(&mut mutation_pages[0]);
                    for page in mutation_pages.iter_mut().skip(1) {
                        mutations.append(page);
                    }
                    let Some(completion) = self.completion.as_ref() else { return puzzle5d_job_fault(cx, "puzzle5d import lost its completion authority") };
                    if let Err(rejected) = completion.complete(Ok(Emit::mutations(mutations)), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(Puzzle5dPendingCompletionRejection::new(Puzzle5dCompletionOwnerKind::Import, rejected));
                        return puzzle5d_job_fault(cx, message);
                    }
                    self.completed = true;
                }
                let Some(output) = self.commit.take_output() else { return puzzle5d_job_fault(cx, "puzzle5d import lost its exact admitted envelope") };
                return StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output });
            }
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        self.checkpoint(cx)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.commit.begin_close();
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

impl ArtifactReservedJob for Puzzle5dImportJob {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        self.closing = true;
        if maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            let step = rejected.close_step(maximum_items, maximum_bytes)?;
            if step == PluginCloseStep::Complete {
                self.pending_completion_rejection = None;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        match self.commit.close_step(maximum_items, maximum_bytes) {
            PluginCloseStep::Complete => {}
            step => return Ok(step),
        }
        if let Some(fragment) = self.fragment.take() {
            // 🌉️ `fragment` is this file's own first-party `Value` (built by `parse`, not
            // `serde_json::from_str` — see `puzzle5d_decode_import_fragment`), so the shared
            // `puzzle5d_retire_json_step` (pinned to `serde_json::Value` by its own
            // `serde_json::json!`-built test, an untouched `json!` site) cannot walk it. dsl's
            // `Object` (unlike `serde_json::Map`) exposes no `remove`/`iter_mut`, so it cannot
            // support the same incremental key-at-a-time descent either — a real gap, not
            // papered over here: this disposes the whole (already admission-capped, at
            // `PUZZLE5D_IMPORT_MEDIA_BYTES`/`PUZZLE5D_IMPORT_SEMANTIC_ITEMS`) fragment in one
            // step instead of the sibling fields' byte-exact recursive walk.
            let bytes = match &fragment {
                Value::Object(object) => object.iter().count().saturating_mul(64).max(size_of::<Value>()),
                Value::Array(values) => values.len().saturating_mul(size_of::<Value>()),
                _ => size_of::<Value>(),
            };
            if bytes > maximum_bytes {
                self.fragment = Some(fragment);
                return Err(Fault::from("puzzle5d kit:in fragment exceeds its bounded disposal byte slice"));
            }
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(owner) = self.current_part.as_mut() {
            if let Some(step) = puzzle5d_retire_part_kind_step(owner, maximum_bytes)? {
                return Ok(step);
            }
            self.current_part = None;
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(page_index) = self.mutation_pages.iter().rposition(|page| !page.is_empty()) {
            let owner = self.mutation_pages[page_index].last_mut().ok_or_else(|| Fault::from("puzzle5d import mutation page changed during retirement"))?;
            if let Some(step) = puzzle5d_retire_import_mutation_step(owner, maximum_bytes)? {
                return Ok(step);
            }
            self.mutation_pages[page_index].pop();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(owner) = self.compatibility.last_mut() {
            if let Some(step) = puzzle5d_retire_compatibility_step(owner, maximum_bytes)? {
                return Ok(step);
            }
            self.compatibility.pop();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(owner) = self.catalogs.parts.last_mut() {
            if let Some(step) = puzzle5d_retire_part_kind_step(owner, maximum_bytes)? {
                return Ok(step);
            }
            self.catalogs.parts.pop();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(owner) = self.catalogs.grips.last_mut() {
            if let Some(step) = puzzle5d_retire_grip_kind_step(owner, maximum_bytes)? {
                return Ok(step);
            }
            self.catalogs.grips.pop();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(owner) = self.catalogs.fasteners.last_mut() {
            if let Some(step) = puzzle5d_retire_fastener_kind_step(owner, maximum_bytes)? {
                return Ok(step);
            }
            self.catalogs.fasteners.pop();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(owner) = self.catalogs.ropes.last_mut() {
            if let Some(step) = puzzle5d_retire_rope_kind_step(owner, maximum_bytes)? {
                return Ok(step);
            }
            self.catalogs.ropes.pop();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        macro_rules! retire_backing {
            ($owners:expr) => {
                if let Some(step) = puzzle5d_retire_vec_backing(&mut $owners, maximum_bytes)? {
                    return Ok(step);
                }
            };
        }
        for page in &mut self.mutation_pages {
            if let Some(step) = puzzle5d_retire_vec_backing(page, maximum_bytes)? {
                return Ok(step);
            }
        }
        retire_backing!(self.compatibility);
        retire_backing!(self.catalogs.parts);
        retire_backing!(self.catalogs.grips);
        retire_backing!(self.catalogs.fasteners);
        retire_backing!(self.catalogs.ropes);
        if let Some(key) = self.retiring_index_primary.as_mut() {
            if let Some(step) = puzzle5d_retire_string_step(key, maximum_bytes)? {
                return Ok(step);
            }
            self.retiring_index_primary = None;
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(key) = self.retiring_index_secondary.as_mut() {
            if let Some(step) = puzzle5d_retire_string_step(key, maximum_bytes)? {
                return Ok(step);
            }
            self.retiring_index_secondary = None;
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some((key, _)) = self.part_index.pop() {
            self.retiring_index_primary = Some(key);
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some((key, _)) = self.grip_index.pop() {
            self.retiring_index_primary = Some(key);
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(((source, target), _)) = self.compatibility_index.pop() {
            self.retiring_index_primary = Some(source);
            self.retiring_index_secondary = Some(target);
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(step) = puzzle5d_retire_vec_backing(&mut self.part_index, maximum_bytes)? {
            return Ok(step);
        }
        if let Some(step) = puzzle5d_retire_vec_backing(&mut self.grip_index, maximum_bytes)? {
            return Ok(step);
        }
        if let Some(step) = puzzle5d_retire_vec_backing(&mut self.compatibility_index, maximum_bytes)? {
            return Ok(step);
        }
        if let Some(text) = self.media_json.as_mut() {
            if let Some(step) = puzzle5d_retire_string_step(text, maximum_bytes)? {
                return Ok(step);
            }
            self.media_json = None;
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(step) = puzzle5d_retire_string_step(&mut self.port, maximum_bytes)? {
            return Ok(step);
        }
        if !self.raw.is_empty() && maximum_bytes == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.raw.pop().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(step) = puzzle5d_retire_vec_backing(&mut self.raw, maximum_bytes)? {
            return Ok(step);
        }
        if self.snapshot.as_ref().is_some_and(|snapshot| std::sync::Arc::strong_count(snapshot) == 1) {
            return Ok(PluginCloseStep::Blocked { reason: "puzzle5d import snapshot has no mounted retained authority" });
        }
        if self.snapshot.take().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.completion.as_ref().is_some_and(|completion| !completion.has_mounted_consumer()) {
            return Ok(PluginCloseStep::Blocked { reason: "puzzle5d import completion has no mounted consumer authority" });
        }
        if self.completion.take().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.raw.is_empty()
            && self.raw.capacity() == 0
            && self.port.is_empty()
            && self.port.capacity() == 0
            && self.media_json.is_none()
            && self.snapshot.is_none()
            && self.fragment.is_none()
            && self.current_part.is_none()
            && self.catalogs.parts.is_empty()
            && self.catalogs.parts.capacity() == 0
            && self.catalogs.grips.is_empty()
            && self.catalogs.grips.capacity() == 0
            && self.catalogs.fasteners.is_empty()
            && self.catalogs.fasteners.capacity() == 0
            && self.catalogs.ropes.is_empty()
            && self.catalogs.ropes.capacity() == 0
            && self.compatibility.is_empty()
            && self.compatibility.capacity() == 0
            && self.part_index.is_empty()
            && self.part_index.capacity() == 0
            && self.grip_index.is_empty()
            && self.grip_index.capacity() == 0
            && self.compatibility_index.is_empty()
            && self.compatibility_index.capacity() == 0
            && self.mutation_pages.iter().all(|page| page.is_empty() && page.capacity() == 0)
            && self.retiring_index_primary.is_none()
            && self.retiring_index_secondary.is_none()
            && self.pending_completion_rejection.is_none()
            && self.completion.is_none()
            && self.commit.terminal_is_empty()
    }
}
//#endregion 🧵️ReservedJobs

//#region 🔖️ContextMenu
/// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: `duplicateSelection`/`selectSameKindSelection`/
/// `focusSelection` stay top-level verbs; the hide/lock toggles (bespoke rows — their label/icon flip
/// on selection state, so they can't resolve from a single static `ActionDefinition`) fold into a
/// `settings` group; `deleteSelection` (bespoke label carrying the selection-count phrase) stays the
/// trailing destructive row. `organize_context_menu`, run automatically at the `VcsArtifactApp::context_menu`
/// funnel, handles taxonomy ordering/separator placement — this function only needs to emit the rows.
/// 🕹️ The per-granularity ids one context-menu request carries. `surface.selection` is what the
/// document holds selected; `surface.hits` is the entity the pointer is actually over and WINS only
/// for a granularity the selection does not carry, so a right-click on an unselected grip opens that
/// grip's menu while a right-click inside a part selection keeps the whole selection as the subject.
#[derive(Default)]
pub struct Puzzle5dContextSelection {
    pub part_ids: Vec<String>,
    pub grip_ids: Vec<String>,
    pub fastener_ids: Vec<String>,
}

impl Puzzle5dContextSelection {
    /// 🪣️ The bucket one surface domain name belongs to. `"object"`/`"node"` are the board and world
    /// hosts' own painted-entity domain names for what this app calls a PART.
    fn bucket(&mut self, domain: &str) -> Option<&mut Vec<String>> {
        match domain {
            "node" | "object" | PUZZLE5D_GRANULARITY_PART => Some(&mut self.part_ids),
            "vortex" | PUZZLE5D_GRANULARITY_GRIP => Some(&mut self.grip_ids),
            "attraction" | PUZZLE5D_GRANULARITY_FASTENER => Some(&mut self.fastener_ids),
            _ => None,
        }
    }

    pub fn from_surface(surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Self {
        let mut out = Self::default();
        let Some(surface) = surface else { return out };
        for group in &surface.selection {
            let ids = group.ids.clone();
            if let Some(bucket) = out.bucket(group.domain.as_str()) {
                bucket.extend(ids);
            }
        }
        for hit in &surface.hits {
            let id = hit.id.clone();
            if let Some(bucket) = out.bucket(hit.domain.as_str()).filter(|bucket| bucket.is_empty()) {
                bucket.push(id);
            }
        }
        out
    }

    fn is_empty(&self) -> bool {
        self.part_ids.is_empty() && self.grip_ids.is_empty() && self.fastener_ids.is_empty()
    }
}

/// 🗂️ Every action id a context-menu row may carry that is NOT one of this app's declared actions:
/// the framework-reserved clipboard and interaction verbs, which live on `handle_action`'s reserved
/// branch and therefore never appear in the `AppActionRegistry` a `Menu` resolves against. Named here
/// so the context-menu law can hold every OTHER row to a declared, `Migrated` app action.
pub const PUZZLE5D_CONTEXT_MENU_RESERVED_ACTIONS: [&str; 4] = ["copy", "cut", "paste", "selectAll"];

/// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS, one branch per selection granularity (puzzle 3d's
/// own shape): an EMPTY surface offers what can be done with no subject (select all, paste, add a
/// part), a PART selection the full selection vocabulary with the hide/lock toggles folded into a
/// `settings` group, a GRIP the brush-suggestions entry point, a FASTENER its own delete. The toggles
/// and the count-carrying delete are bespoke rows because their label/icon flip on state, which a
/// single static `ActionDefinition` cannot resolve; every other row resolves through `Menu::action`
/// against the registry, so a typo'd id is a construction-time panic rather than a dead row.
/// `organize_context_menu`, run at the `VcsArtifactApp::context_menu` funnel, handles taxonomy
/// ordering and separator placement — this function only emits rows.
fn puzzle5d_context_menu_items(
    envelope: &Puzzle5dScene,
    selection: &Puzzle5dContextSelection,
    labels: &Puzzle5dLabels,
    is_de: bool,
    registry: &semio_framework_plugin::AppActionRegistry,
) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, ContextMenuItemSpec, Menu};
    let bespoke = |id: &str, label: String, icon: &str, action: &str, args: Option<Value>, destructive: bool| ContextMenuItemSpec {
        id: id.into(),
        label: Some(label),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args: args.map(|value| dsl::os_pack::json::to_dsl_value(&value)),
        destructive: destructive.then_some(true),
        ..Default::default()
    };
    if selection.is_empty() {
        return Menu::of(registry)
            .item(bespoke("select-all", labels.select_all.into(), "box-select", "selectAll", None, false))
            .item(bespoke("paste", labels.paste.into(), "clipboard-paste", "paste", None, false))
            .action("openAddPartDialog")
            .build();
    }
    if !selection.part_ids.is_empty() {
        let part_ids = &selection.part_ids;
        let selected: Vec<&Puzzle5dPart> = envelope.document.parts.iter().filter(|part| part_ids.contains(&part.id)).collect();
        let all_hidden = !selected.is_empty() && selected.iter().all(|part| part.part_2d.hidden.unwrap_or(false));
        let all_locked = !selected.is_empty() && selected.iter().all(|part| part.part_2d.locked.unwrap_or(false));
        let phrase = selection_count_phrase(is_de, &[(part_ids.len(), if is_de { "Teil" } else { "part" }, if is_de { "Teile" } else { "parts" })]);
        return Menu::of(registry)
            .action("duplicateSelection")
            .item(bespoke("copy", labels.copy.into(), "copy", "copy", None, false))
            .item(bespoke("cut", labels.cut.into(), "scissors", "cut", None, false))
            .action("selectSameKindSelection")
            .action("focusSelection")
            .group("settings", |m| {
                m.item(bespoke("hide-show", if all_hidden { labels.show.into() } else { labels.hide.into() }, if all_hidden { "eye" } else { "eye-off" }, "setSelectionFlag", Some(dsl::json!({ "flag": "hidden", "value": !all_hidden })), false))
                    .item(bespoke("lock-unlock", if all_locked { labels.unlock.into() } else { labels.lock.into() }, if all_locked { "lock-open" } else { "lock" }, "setSelectionFlag", Some(dsl::json!({ "flag": "locked", "value": !all_locked })), false))
            })
            .item(bespoke("delete", format!("{} ({phrase})", labels.delete.as_str()), "trash", "deleteSelection", None, true))
            .build();
    }
    if !selection.grip_ids.is_empty() {
        let mut menu = Menu::of(registry);
        // 🎣️ Only for EXACTLY one grip: the brush suggestions link points at a single grip, so a
        // multi-grip row would silently keep whichever one the link last held.
        if let [only] = selection.grip_ids.as_slice() {
            menu = menu.item(bespoke("suggest", labels.suggest_parts.into(), "sparkles", "targetBrushSuggestions", Some(dsl::json!({ "fullId": only.as_str() })), false));
        }
        return menu.action("focusSelection").item(bespoke("delete", labels.delete.into(), "trash", "deleteSelection", None, true)).build();
    }
    // 🔗️ A fastener row carries its own id. `retargetFastener` is deliberately NOT offered here: it
    // needs a REPLACEMENT grip a context menu cannot name, and dispatching it with an id alone is an
    // early return — a visibly dead row. Retargeting lives in the inspector, which has both grips.
    let Some(id) = selection.fastener_ids.first() else { return Vec::new() };
    Menu::of(registry).item(bespoke("delete", labels.delete.into(), "trash", "deleteFastener", Some(dsl::json!({ "id": id.as_str() })), true)).build()
}
//#endregion 🔖️ContextMenu

//#region 🔖️Puzzle5dCommand
/// @emoji 🎯️ B1: `Puzzle5dPlayApp::Command` — the SOLE dispatch surface, one variant per declared
/// action (mirrors every `.mutation(...)`/`.view_action(...)` id `create_puzzle5d_app` registers,
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
        #[derive(Clone, Debug, PartialEq)]
        pub enum Puzzle5dCommand {
            $($Variant { window_id: Option<String>, args: Option<dsl::os_pack::json::Value> }),*
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

            fn args(&self) -> Option<&dsl::os_pack::json::Value> {
                match self {
                    $(Puzzle5dCommand::$Variant { args, .. } => args.as_ref()),*
                }
            }

            fn try_from_action(action: &str, args: Option<dsl::os_pack::json::Value>, window_id: Option<String>) -> Option<Self> {
                match action {
                    $($id => Some(Puzzle5dCommand::$Variant { window_id, args })),*,
                    _ => None,
                }
            }

            #[cfg(test)]
            fn from_action(action: &str, args: Option<dsl::os_pack::json::Value>, window_id: Option<String>) -> Self {
                Self::try_from_action(action, args, window_id)
                    .unwrap_or_else(|| panic!("unknown puzzle5d action id in test: {action}"))
            }

            /// 🪶️ Hand-written JSON bridge (see this macro's own doc comment on why `OpBinary` here
            /// is a plain JSON-bytes bridge, not a derive): reproduces serde's default externally
            /// tagged struct-variant shape (`{"VariantName": {"window_id": ..., "args": ...}}`) so
            /// `encode_op`/`decode_op` stay byte-for-byte compatible with the pre-migration wire.
            fn to_json(&self) -> dsl::os_pack::json::Value {
                match self {
                    $(Puzzle5dCommand::$Variant { window_id, args } => dsl::os_pack::json::object([(
                        stringify!($Variant).to_string(),
                        dsl::os_pack::json::object([("window_id".to_string(), dsl::os_pack::json::Value::from(window_id.clone())), ("args".to_string(), args.clone().unwrap_or(dsl::os_pack::json::Value::Null))]),
                    )])),*
                }
            }

            fn from_json(value: &dsl::os_pack::json::Value) -> Option<Self> {
                let entries = value.as_object()?;
                if entries.len() != 1 {
                    return None;
                }
                let (tag, payload) = entries.iter().next()?;
                let window_id = payload.get("window_id").and_then(dsl::os_pack::json::Value::as_str).map(str::to_string);
                let args = payload.get("args").cloned().filter(|value| !value.is_null());
                match tag {
                    $(stringify!($Variant) => Some(Puzzle5dCommand::$Variant { window_id, args }),)*
                    _ => None,
                }
            }
        }
    };
}

puzzle5d_command_variants! {
    ExportFixture = "exportFixture",
    ImportFixture = "importFixture",
    OpenImportFixture = "openImportFixture",
    OpenAddPartDialog = "openAddPartDialog",
    SetActiveExample = "setActiveExample",
    AddNode = "addNode",
    AddPartKind = "addPartKind",
    AddBrushPart = "addBrushPart",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    SetSelectionFlag = "setSelectionFlag",
    FocusSelection = "focusSelection",
    EngagementSubmit = "engagementSubmit",
    EngagementRepeatLast = "engagementRepeatLast",
    SetFillCount = "setFillCount",
    PatchPart = "patchPart",
    PatchGrip = "patchGrip",
    PatchFastener = "patchFastener",
    CreateFastener = "createFastener",
    DeleteFastener = "deleteFastener",
    RetargetFastener = "retargetFastener",
    EditFastener = "editFastener",
    ProximityConnect = "proximityConnect",
    TranslateSelection = "translateSelection",
    RotateSelection = "rotateSelection",
    ScaleSelection = "scaleSelection",
    WorldRelocate = "worldRelocate",
    ApplyBoardEvents = "applyBoardEvents",
    SetCamera = "setCamera",
    SetCamera2d = "setCamera2d",
    SetCamera3d = "setCamera3d",
    SelectSameKindSelection = "selectSameKindSelection",
    ToggleSun = "toggleSun",
    SetSunAzimuth = "setSunAzimuth",
    SetSunElevation = "setSunElevation",
    SetSunIntensity = "setSunIntensity",
    EngagementInput = "engagementInput",
    EngagementAbort = "engagementAbort",
    EngagementControlSelect = "engagementControlSelect",
    CycleBrushCandidate = "cycleBrushCandidate",
    CycleBrushCandidateBack = "cycleBrushCandidateBack",
    TargetBrushSuggestions = "targetBrushSuggestions",
    RegisterBrushMesh = "registerBrushMesh",
    SetBrushPlacementContactTolerance = "setBrushPlacementContactTolerance",
    SetProximityRadius = "setProximityRadius",
    SetChunkSize = "setChunkSize",
    SetPartKindWeight = "setPartKindWeight",
    SetGripKindWeight = "setGripKindWeight",
    SetLodMode = "setLodMode",
    SetSuggestionOffset = "setSuggestionOffset",
    SetGridSnapEnabled = "setGridSnapEnabled",
    SetGridFactor = "setGridFactor",
    SetGridVisible = "setGridVisible",
    SetGridSpacing = "setGridSpacing",
    SetProjection = "setProjection",
    SetProjectionParam = "setProjectionParam",
    SetGripShow = "setGripShow",
    SetGripDirection = "setGripDirection",
    SetSelectableKind = "setSelectableKind",
    SetLodAutomatic = "setLodAutomatic",
    SetLodDepthVariable = "setLodDepthVariable",
    SetLodManual = "setLodManual",
    SetTransformGumballFlag = "setTransformGumballFlag",
    WorldPointerDown = "worldPointerDown",
    CanvasPointerDown = "canvasPointerDown",
    AddTargetVolume = "addTargetVolume",
    DeleteTargetVolume = "deleteTargetVolume",
    RelocateTargetVolume = "relocateTargetVolume",
    SetTargetVolumeFlag = "setTargetVolumeFlag",
    SetVoxelDims = "setVoxelDims",
}

/// 🧭️ Every generated tool id this app's wire may address: the retained command catalog plus the two
/// framework-injected host-configuration verbs. They are not app-owned retained tools, but
/// `validate_tool_job_rows` only admits `Puzzle5dHostConfigurationProofs`' generic bounded proofs for a
/// generated id — without them `dispatch_action` fails closed with `interactive-job.missing-factory` before
/// the host-configuration branch is ever consulted, and the fill TOOL cannot be armed.
const PUZZLE5D_TOOL_JOB_IDS: [&str; PUZZLE5D_RETAINED_TOOL_IDS.len() + 2] = {
    let mut ids = [""; PUZZLE5D_RETAINED_TOOL_IDS.len() + 2];
    let mut index = 0;
    while index < PUZZLE5D_RETAINED_TOOL_IDS.len() {
        ids[index] = PUZZLE5D_RETAINED_TOOL_IDS[index];
        index += 1;
    }
    ids[index] = semio_framework_plugin::SET_ACTIVE_TOOL_ACTION_ID;
    ids[index + 1] = semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID;
    ids
};

impl protocol::OpBinary for Puzzle5dCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &PUZZLE5D_TOOL_JOB_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(dsl::os_pack::json::to_string(&self.to_json()).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        let value = parse(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        Self::from_json(&value).ok_or_else(|| protocol::ProtocolError::Pack(store::PackError::Schema("unrecognized Puzzle5dCommand tag".to_string())))
    }
}
//#endregion 🔖️Puzzle5dCommand

//#region 🔖️ActionContext
/// 🎬️ Everything one `🎮️commands/*` arm may read or write. The prologue/epilogue around the dispatch
/// match (scene materialization, delta computation, host-effect emission, config snapshotting) stays
/// in [`Puzzle5dPlayApp::handle_action_impl`]; an arm only mutates this bundle.
pub struct Puzzle5dActionCtx<'a> {
    pub scene: &'a mut Puzzle5dScene,
    /// 📸️ The committed play snapshot the scene was materialized from (authored kind catalogs included).
    pub snapshot: &'a Puzzle5dPlaySnapshot,
    /// 🧠️ The document instance's retained operation owner (the brush suggestions link), when the route bound one.
    pub instance_owner: Option<&'a semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    /// 🪟️ The window this action targets (already defaulted to the 3D window).
    pub window_id: &'a str,
    /// 🧭️ The registered kind of the exact target window.
    pub window_kind: &'a str,
    /// 🕹️ Read-only view of the framework-owned `vortex` interaction domain (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — retained selection-acting verbs read
    /// `.selected_part_ids()?`/etc. here instead of the deleted `Puzzle5dConfig` selection fields.
    pub selection: &'a protocol::DomainSelection,
    /// 🌍️ The host's projected locale×terminology axes — the only source a notice may resolve its
    /// sentence from (`Puzzle5dActionCtx::notice`).
    pub view_state: Option<&'a semio_framework_plugin::ViewModel>,
    /// ⏯️ The instance's tool run as of command admission (`ArtifactOwnedToolJobContext::tool_run`) — the
    /// identity Escape's `toolRunAbort` is bound to while a fill run is live.
    pub tool_run: Option<&'a semio_framework_plugin::ToolRunView>,
    /// 🧯️ At most ONE bounded, localized transient notice this arm wants shown, plus whatever host
    /// effects it raised. An arm that REFUSES pushes a notice here instead of completing silently.
    pub effects: Vec<Effect>,
    /// 🕹️ App-initiated selection writes this arm wants applied once its own mutations land — the
    /// single sanctioned way a reducer selects what it just created, or widens/clears the selection
    /// (`semio_framework_plugin::InteractionWrite`, applied by `VcsArtifactApp` through the same state
    /// machine the reserved `interactionSelect` verb uses). Empty for every arm that does not move it.
    pub interaction_writes: Vec<InteractionWrite>,
    /// 🛑️ Set by an arm that must skip the whole epilogue (delta, effects, config snapshot) — the
    /// direct replacement for the pre-migration `return Emit::default()` early exits.
    pub abort: bool,
}

impl<'a> Puzzle5dActionCtx<'a> {
    /// 🔗️ Runs `apply` on this instance's brush suggestions link; `None` without an instance owner or while the
    /// owner is busy.
    pub fn brush_suggestions<R>(&self, apply: impl FnOnce(&mut semio_s_artifact_puzzle_3d::editor::puzzle3d::precompute::brush::BrushSuggestionsLink) -> R) -> Option<R> {
        self.instance_owner?.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(apply(&mut owner.brush_suggestions))).ok()
    }

    fn selected_ids(&self, granularity_id: &str) -> Vec<String> {
        if self.selection.granularity == granularity_id {
            self.selection.ids.clone()
        } else {
            Vec::new()
        }
    }
    pub fn selected_part_ids(&self) -> Vec<String> {
        self.selected_ids(PUZZLE5D_GRANULARITY_PART)
    }
    pub fn selected_grip_ids(&self) -> Vec<String> {
        self.selected_ids(PUZZLE5D_GRANULARITY_GRIP)
    }
    pub fn selected_fastener_ids(&self) -> Vec<String> {
        self.selected_ids(PUZZLE5D_GRANULARITY_FASTENER)
    }

    /// 🕹️ Replaces this app's whole `vortex` selection with `ids` at `granularity` once the action's own
    /// mutations have landed — "select the thing I just created/widened to".
    pub fn replace_selection(&mut self, granularity: &str, ids: impl IntoIterator<Item = String>) {
        let write = InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, granularity, ids);
        if !write.targets.is_empty() {
            self.interaction_writes.push(write);
        }
    }

    /// 🧹️ Empties this app's whole `vortex` selection through the same sanctioned reducer channel
    /// [`Self::replace_selection`] uses. It is a `Subtractive` write naming exactly what is selected right
    /// now, NOT an empty `Replace`: the framework's state machine returns the current selection unchanged
    /// when a write names no target at all (`protocol::next_selection`), so an empty `Replace` is a silent
    /// no-op. Nothing selected means nothing to clear.
    pub fn clear_selection(&mut self) {
        let targets: Vec<InteractionTarget> = [
            (PUZZLE5D_GRANULARITY_PART, self.selected_part_ids()),
            (PUZZLE5D_GRANULARITY_GRIP, self.selected_grip_ids()),
            (PUZZLE5D_GRANULARITY_FASTENER, self.selected_fastener_ids()),
        ]
        .into_iter()
        .flat_map(|(granularity, ids)| ids.into_iter().map(move |id| InteractionTarget { granularity: granularity.to_string(), id }))
        .collect();
        if targets.is_empty() {
            return;
        }
        self.interaction_writes.push(InteractionWrite { domain: PUZZLE5D_INTERACTION_DOMAIN.into(), targets, merge: MergeMode::Subtractive });
    }

    /// 🧯️ Surfaces ONE bounded, localized notice through the shell's transient-notice channel
    /// (`Effect::Notify` → `ShellHost`'s `showTransientNotice`) — how an arm whose work was REFUSED tells
    /// the user, instead of the `Ok(Fixture(_))`-only silence every 5d placement arm fell through with. At
    /// most one notice per action, so the effect list stays fixed-width. An unauthored locale×terminology
    /// axis carries this app's own `ui.localization.unsupported` code rather than an English sentence.
    pub fn notice(&mut self, message: impl Fn(&Puzzle5dLabels) -> &'static str) {
        if self.effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })) {
            return;
        }
        let text = self.view_state.and_then(puzzle5d_labels).map_or_else(|| PUZZLE5D_LOCALIZATION_UNSUPPORTED.to_string(), |labels| message(labels).to_string());
        self.effects.push(Effect::Notify { message: text });
    }

    /// 🎯️ Refuses a selection-scoped command that has nothing to act on: ONE localized notice, then the
    /// same `abort` the early `return` used, so neither an empty document edit nor an empty interaction
    /// write is emitted. Answers whether it refused.
    pub fn refuse_without_selection(&mut self, ids: &[String]) -> bool {
        if !ids.is_empty() {
            return false;
        }
        self.notice(|labels| labels.nothing_selected.as_str());
        self.abort = true;
        true
    }

    /// 🔒️ Refuses a gesture whose every addressed part carries `part_2d.locked` — a locked part must not
    /// move, and a silent no-op is indistinguishable from a dead control.
    pub fn refuse_when_locked(&mut self, ids: &[String]) -> bool {
        if ids.is_empty() || ids.iter().any(|id| self.scene.document.parts.iter().any(|part| &part.id == id && !part.part_2d.locked.unwrap_or(false))) {
            return false;
        }
        self.notice(|labels| labels.selection_locked.as_str());
        self.abort = true;
        true
    }
}

/// 🌍️ The code an unauthored locale×terminology axis carries instead of an English sentence — this UI has
/// no default language, so a hard-coded fallback would be a silent localization regression.
pub const PUZZLE5D_LOCALIZATION_UNSUPPORTED: &str = "ui.localization.unsupported";

/// 🧯️ ONE bounded, localized notice as a terminal `Emit` — how a retained work whose gesture produced
/// nothing tells the user WHY, instead of the bare `Emit::default()` every refusal path used to complete
/// with. The scope is [`UiDirtyScope::None`]: a work that refused published nothing, so there is nothing to
/// repaint — the notice itself travels on the effect lane, which the host applies before it consults scope.
fn puzzle5d_notice_emit(view_state: Option<&semio_framework_plugin::ViewModel>, message: impl Fn(&Puzzle5dLabels) -> &'static str) -> Emit<Puzzle5dMutation, Puzzle5dConfigMutation> {
    let text = view_state.and_then(puzzle5d_labels).map_or_else(|| PUZZLE5D_LOCALIZATION_UNSUPPORTED.to_string(), |labels| message(labels).to_string());
    Emit { effects: vec![Effect::Notify { message: text }], ui_scope: UiDirtyScope::None, ..Default::default() }
}

/// 📋️ The clipboard fragment of the selected parts and fasteners (the fasteners among them only).
pub fn puzzle5d_copy_fragment(snapshot: &Puzzle5dPlaySnapshot, part_ids: &[String], fastener_ids: &[String]) -> Result<ClipboardFragment, ClipboardError> {
    let document: Puzzle5dDocument = serde_json::from_value(snapshot.0.clone()).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    let (parts, fasteners) = copy_selection_local(&document, part_ids, fastener_ids);
    if parts.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let fragment_value = serde_json::json!({ "schema": PUZZLE5D_SCHEMA, "parts": parts, "fasteners": fasteners });
    Ok(ClipboardFragment {
        schema: PUZZLE5D_SCHEMA.to_string(),
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design },
        dsl_text: serde_json::to_string_pretty(&fragment_value).unwrap_or_default(),
        pack_bytes: None,
        source_app: PUZZLE5D_PLAY_APP_ID.to_string(),
        label: format!("{} part(s)", parts.len()),
    })
}

/// ✂️ The document removal of the selected parts and fasteners.
pub fn puzzle5d_cut_operations(snapshot: &Puzzle5dPlaySnapshot, part_ids: &[String], fastener_ids: &[String]) -> Vec<Puzzle5dMutation> {
    let before = puzzle5d_projection_value(&snapshot.0);
    let Ok(document) = <Puzzle5dDocument as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(&before)) else {
        return Vec::new();
    };
    let (parts, fasteners) = copy_selection_local(&document, part_ids, fastener_ids);
    if parts.is_empty() {
        return Vec::new();
    }
    // 🔒️ A LOCKED part is copied but never removed — a lock exists precisely to refuse a destructive
    // gesture — and the fasteners of a surviving part survive with it, or the document is left half-cut.
    // `Puzzle5dCutJob` applies the identical rule on the retained route; the two must never disagree.
    let locked: HashSet<&str> = parts.iter().filter(|part| part.part_2d.locked.unwrap_or(false)).map(|part| part.id.as_str()).collect();
    let remove_part_ids: HashSet<&str> = parts.iter().map(|part| part.id.as_str()).filter(|id| !locked.contains(id)).collect();
    let remove_fastener_ids: HashSet<&str> =
        fasteners.iter().filter(|fastener| !locked.contains(owning_part_id_local(&fastener.source)) && !locked.contains(owning_part_id_local(&fastener.target))).map(|fastener| fastener.id.as_str()).collect();
    let mut after = document;
    after.parts.retain(|part| !remove_part_ids.contains(part.id.as_str()));
    after.fasteners.retain(|fastener| !remove_fastener_ids.contains(fastener.id.as_str()));
    puzzle5d_operations_from_document_change(&before, &after)
}

/// 🕹️ `copy_fragment`/`cut_operations` have no `Puzzle5dActionCtx` (only `doc`/`cfg`/`interaction`
/// per `ArtifactApp`'s signature) — a free-function twin of `Puzzle5dActionCtx::selected_part_ids`/
/// `selected_fastener_ids` for those two call sites.
fn puzzle5d_interaction_part_and_fastener_ids(interaction: &InteractionView<'_>) -> (Vec<String>, Vec<String>) {
    let selection = interaction.selection(PUZZLE5D_INTERACTION_DOMAIN);
    match selection.granularity.as_str() {
        PUZZLE5D_GRANULARITY_PART => (selection.ids.clone(), Vec::new()),
        PUZZLE5D_GRANULARITY_FASTENER => (Vec::new(), selection.ids.clone()),
        _ => (Vec::new(), Vec::new()),
    }
}
/// 🏷️ Admits dynamic puzzle labels into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref().to_string()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d label admission failed"))
}
//#endregion 🔖️ActionContext

//#region 🔖️PlayApp
// 🧩️ B1: Puzzle-5d play app. Stateless: the persisted document (bare `Puzzle5dDocument` json) lives in
// the wrapping `VcsArtifactApp`'s document store, the ephemeral view state in the wrapping store's real,
// VCS-tracked `Puzzle5dConfig` artifact (see `🦀️config.rs`) — every read comes from `cfg.snapshot`, every write
// flows out as a `Puzzle5dConfigMutation` in the returned `Emit` — and what an instance retains across turns
// (the brush suggestions link) in its instance operation owner.
// Each action mutates a transient {@link Puzzle5dScene}, then emits the granular operation delta.
// Undo/redo/checkpoints are handled by the wrapper.
fn with_puzzle5d_app<R>(f: impl FnOnce(&Puzzle5dPlayApp) -> R) -> R {
    let app = Puzzle5dPlayApp;
    f(&app)
}

#[derive(Default)]
pub struct Puzzle5dPlayApp;

impl Puzzle5dPlayApp {
    /// @emoji 🧩️ B1: the pure per-action core, dispatched into by `ArtifactApp::handle` with
    /// `action`/`args`/`window_id` reconstructed 1:1 from the typed `Puzzle5dCommand`. Everything past
    /// this adapter boundary reads/writes the passed-in `Puzzle5dConfig` snapshot and returns a real
    /// `Emit` (document + config operations) instead of mutating `self`.
    fn handle_action_impl(
        &self,
        action: &str,
        args: Option<&Value>,
        window_id: Option<&str>,
        snapshot: &Puzzle5dPlaySnapshot,
        config: &Puzzle5dRuntime,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        selection: &protocol::DomainSelection,
        instance_owner: Option<&semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
        tool_run: Option<&semio_framework_plugin::ToolRunView>,
    ) -> (Emit<Puzzle5dMutation, Puzzle5dConfigMutation>, EphemeralEmit<EditorApp<Puzzle5dPlayApp>>) {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let before = projection;
        let shared_before = window_ownership::shared(config);
        let window_before = window_ownership::config_from_runtime(config);
        let transient_before = window_ownership::transient_from_runtime(config, window_id.unwrap_or(world3d::WINDOW_KIND_ID));
        let active_utility_initial = puzzle5d_scene_active_utility(view_state, window_id);
        let wid = window_id.map_or_else(|| world3d::WINDOW_KIND_ID.to_string(), str::to_string);
        let window_kind = view_state.and_then(window_ownership::kind_for_view).unwrap_or(world3d::WINDOW_KIND_ID);
        let mut scene = scene_from_projection(&before, config.clone(), &active_utility_initial);
        let mut ctx =
            Puzzle5dActionCtx { scene: &mut scene, snapshot, instance_owner, window_id: &wid, window_kind, selection, view_state, tool_run, effects: Vec::new(), interaction_writes: Vec::new(), abort: false };
        dispatch_puzzle5d_action(&mut ctx, action, args);
        let aborted = ctx.abort;
        let mut arm_effects = std::mem::take(&mut ctx.effects);
        let interaction_writes = std::mem::take(&mut ctx.interaction_writes);
        if aborted {
            // 🧯️ An aborted arm emits no document/config delta — but the refusal NOTICE it pushed is the
            // user-visible half of that refusal and must survive, or "refused" and "silently did nothing"
            // look identical. Effects travel on their own lane, so keeping them costs no mutation and no
            // undo entry; the scope stays `None` because nothing was painted.
            return (Emit { effects: arm_effects, ui_scope: UiDirtyScope::None, ..Default::default() }, EphemeralEmit::default());
        }
        let next_active_utility = scene.active_utility.clone();
        let operations = if action == "patchFastener" { puzzle5d_patch_fastener_operations(&before, &scene.document, args) } else { puzzle5d_operations_from_document_change(&before, &scene.document) };
        // 🌀️ Coalesce each gumball drag tick into one undoable edit (compact per-part records, not full meshes).
        let coalesce_key = match action {
            "translateSelection" => Some("gumball-translate".to_string()),
            "rotateSelection" => Some("gumball-rotate".to_string()),
            "scaleSelection" => Some("gumball-scale".to_string()),
            _ => None,
        };
        // 🧰️🛠️ Programmatic tool/utility switches push the host session. Arming fill emits `SetActiveTool { fill }`
        // only — an empty tool effect here would bounce-disarm a just-armed run, so LEAVING fill is exclusively the
        // host's own `setActiveTool ""` (which `engagementAbort` raises itself). A real utility change still emits
        // `SetActiveUtility`; the host's mutual exclusion clears the tool when the utility is non-empty.
        let initial_is_fill_tool = active_utility_initial == fill_tool::TOOL_ID;
        let next_is_fill_tool = next_active_utility == fill_tool::TOOL_ID;
        if next_is_fill_tool && !initial_is_fill_tool {
            arm_effects.push(Effect::SetActiveTool { tool_id: fill_tool::TOOL_ID.into() });
        }
        if !next_is_fill_tool && next_active_utility != active_utility_initial {
            arm_effects.push(Effect::SetActiveUtility { window_id: wid.clone(), utility_id: next_active_utility.clone() });
        }
        let effects = arm_effects;
        let shared_after = window_ownership::shared(&scene.runtime);
        let config_mutations = if shared_after != shared_before { vec![Puzzle5dConfigMutation::Snapshot { config: shared_after }] } else { Vec::new() };
        let window_after = window_ownership::config_from_runtime(&scene.runtime);
        let window_config_mutations = if window_after != window_before {
            view_state.and_then(|view| window_ownership::addressed_config(view, window_after).ok()).into_iter().collect()
        } else {
            Vec::new()
        };
        let transient_after = window_ownership::transient_from_runtime(&scene.runtime, window_id.unwrap_or(world3d::WINDOW_KIND_ID));
        let window_transient = if transient_after != transient_before {
            view_state.and_then(|view| window_ownership::addressed_transient(view, transient_after).ok()).into_iter().collect()
        } else {
            Vec::new()
        };
        (Emit { artifact_mutations: operations, config_mutations, window_config_mutations, coalesce_key, effects, interaction_writes, ..Default::default() }, EphemeralEmit { window_transient, ..Default::default() })
    }
}

/// 🎬️ Dispatch only: every arm's behaviour lives in its `🎮️commands/<group>/🦀️.rs` free
/// function. No behaviour lives in this match.
fn dispatch_puzzle5d_action(ctx: &mut Puzzle5dActionCtx<'_>, action: &str, args: Option<&Value>) {
    match action {
        "importFixture" => import_fixture::import_fixture(ctx, args),
        "openImportFixture" => open_import_fixture::open_import_fixture(ctx),
        "openAddPartDialog" => open_add_part_dialog::open_add_part_dialog(ctx),
        "setActiveExample" => set_active_example::set_active_example(ctx, args),
        "selectSameKindSelection" => select_same_kind::select_same_kind(ctx),
        "addNode" => add_node::add_node(ctx, args),
        "addPartKind" => add_part_kind::add_part_kind(ctx, args),
        "deleteSelection" => delete_selection::delete_selection(ctx),
        "duplicateSelection" => duplicate_selection::duplicate_selection(ctx),
        "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),
        "patchPart" => patch_part::patch_part(ctx, args),
        "patchGrip" => patch_grip::patch_grip(ctx, args),
        "patchFastener" => patch_fastener::patch_fastener(ctx, args),
        "createFastener" => create_fastener::create_fastener(ctx, args),
        "deleteFastener" => delete_fastener::delete_fastener(ctx, args),
        "retargetFastener" => retarget_fastener::retarget_fastener(ctx, args),
        "editFastener" => edit_fastener::edit_fastener(ctx, args),
        "proximityConnect" => proximity_connect::proximity_connect(ctx, args),
        "setCamera" => set_camera::set_camera(ctx, args),
        "setCamera2d" => set_camera_2d::set_camera_2d(ctx, args),
        "setCamera3d" => set_camera_3d::set_camera_3d(ctx, args),
        "focusSelection" => focus_selection::focus_selection(ctx),
        "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => apply_sun::apply(ctx, action, args),
        "setLodMode" => set_lod_mode::set_lod_mode(ctx, args),
        "setGridSnapEnabled" => set_grid_snap_enabled::set_grid_snap_enabled(ctx, args),
        "setGridFactor" => set_grid_factor::set_grid_factor(ctx, args),
        "setGridVisible" => set_grid_visible::set_grid_visible(ctx, args),
        "setGridSpacing" => set_grid_spacing::set_grid_spacing(ctx, args),
        "setProjection" | "setProjectionParam" => set_projection::set_projection(ctx, action, args),
        "setGripShow" => set_grip_show::set_grip_show(ctx, args),
        "setGripDirection" => set_grip_direction::set_grip_direction(ctx, args),
        "setSelectableKind" => set_selectable_kind::set_selectable_kind(ctx, args),
        "setLodAutomatic" => set_lod_automatic::set_lod_automatic(ctx, args),
        "setLodDepthVariable" => set_lod_depth_variable::set_lod_depth_variable(ctx, args),
        "setLodManual" => set_lod_manual::set_lod_manual(ctx, args),
        "setTransformGumballFlag" => set_transform_gumball_flag::set_transform_gumball_flag(ctx, args),
        "addBrushPart" => add_brush_part::add_brush_part(ctx, args),
        "cycleBrushCandidate" => cycle_brush_candidate::cycle_brush_candidate(ctx),
        "cycleBrushCandidateBack" => cycle_brush_candidate::cycle_brush_candidate_back(ctx),
        "targetBrushSuggestions" => target_brush_suggestions::target_brush_suggestions(ctx, args),
        "registerBrushMesh" => register_brush_mesh::register_brush_mesh(ctx, args),
        "setBrushPlacementContactTolerance" => set_brush_placement_contact_tolerance::set_brush_placement_contact_tolerance(ctx, args),
        "setProximityRadius" => set_proximity_radius::set_proximity_radius(ctx, args),
        "setChunkSize" => set_chunk_size::set_chunk_size(ctx, args),
        "setPartKindWeight" | "setGripKindWeight" => set_kind_weight::set_kind_weight(ctx, action, args),
        "engagementControlSelect" => engagement_control_select::engagement_control_select(ctx, args),
        "setSuggestionOffset" => set_suggestion_offset::set_suggestion_offset(ctx, args),
        "setFillCount" => set_fill_count::set_fill_count(ctx, args),
        "engagementInput" => engagement_input::engagement_input(ctx, args),
        "engagementSubmit" => engagement_submit::engagement_submit(ctx, args),
        "engagementRepeatLast" => engagement_repeat_last::engagement_repeat_last(ctx),
        "engagementAbort" => engagement_abort::engagement_abort(ctx, args),
        "translateSelection" => translate_selection::translate_selection(ctx, args),
        "rotateSelection" => rotate_selection::rotate_selection(ctx, args),
        "scaleSelection" => scale_selection::scale_selection(ctx, args),
        "worldRelocate" => world_relocate::world_relocate(ctx, args),
        "addTargetVolume" => add_target_volume::add_target_volume(ctx, args),
        "deleteTargetVolume" => delete_target_volume::delete_target_volume(ctx, args),
        "relocateTargetVolume" => relocate_target_volume::relocate_target_volume(ctx, args),
        "setTargetVolumeFlag" => set_target_volume_flag::set_target_volume_flag(ctx, args),
        "setVoxelDims" => set_voxel_dims::set_voxel_dims(ctx, args),
        "applyBoardEvents" => apply_board_events::apply_board_events(ctx, args),
        // 🛑️ Pure pointer-down notifications: no scene mutation, no operations, no config snapshot —
        // the pre-migration code returned `Emit::default()` here, which `abort` reproduces exactly.
        "worldPointerDown" | "canvasPointerDown" => ctx.abort = true,
        _ => {}
    }
}

//#region 🧵️RetainedCommands
pub(crate) const PUZZLE5D_RETAINED_TOOL_IDS: &[&str] = &[
    "addTargetVolume",
    "deleteTargetVolume",
    "relocateTargetVolume",
    "setTargetVolumeFlag",
    "setVoxelDims",
    "canvasPointerDown",
    "cycleBrushCandidate",
    "cycleBrushCandidateBack",
    "worldPointerDown",
    "deleteSelection",
    "duplicateSelection",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementRepeatLast",
    "engagementSubmit",
    "exportFixture",
    "importFixture",
    "openAddPartDialog",
    "openImportFixture",
    "selectSameKindSelection",
    "setFillCount",
    "setSelectionFlag",
    "targetBrushSuggestions",
    "focusSelection",
    "addBrushPart",
    "addNode",
    "addPartKind",
    "applyBoardEvents",
    "createFastener",
    "deleteFastener",
    "editFastener",
    "patchFastener",
    "patchGrip",
    "patchPart",
    "proximityConnect",
    "registerBrushMesh",
    "retargetFastener",
    "rotateSelection",
    "scaleSelection",
    "setActiveExample",
    "translateSelection",
    "worldRelocate",
    "setCamera",
    "setCamera2d",
    "setCamera3d",
    "setGridFactor",
    "setGridSnapEnabled",
    "setLodMode",
    "setSuggestionOffset",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setBrushPlacementContactTolerance",
    "setProximityRadius",
    "setChunkSize",
    "setPartKindWeight",
    "setGripKindWeight",
    "setGridVisible",
    "setGridSpacing",
    "setProjection",
    "setProjectionParam",
    "setGripShow",
    "setGripDirection",
    "setSelectableKind",
    "setLodAutomatic",
    "setLodDepthVariable",
    "setLodManual",
    "setTransformGumballFlag",
];
/// 🪟️ Every tool id whose whole semantic work IS one `handle_action_impl` turn against the addressed
/// window's runtime — routed through [`Puzzle5dWindowCommandWork`]. Camera, grid, LOD, sun and the
/// suggestion offset publish the addressed pane's `WindowConfig`; the contact tolerance publishes the
/// shared app `Config` through the same one-turn route (puzzle 3d joins the identical set to its own
/// `Puzzle3dWindowCommandWork` arm).
const PUZZLE5D_WINDOW_TOOL_IDS: &[&str] = &[
    "addTargetVolume",
    "setVoxelDims",
    "cycleBrushCandidate",
    "cycleBrushCandidateBack",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementRepeatLast",
    "engagementSubmit",
    "setFillCount",
    "targetBrushSuggestions",
    "setCamera",
    "setCamera2d",
    "setCamera3d",
    "setGridFactor",
    "setGridSnapEnabled",
    "setLodMode",
    "setSuggestionOffset",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setBrushPlacementContactTolerance",
    "setProximityRadius",
    "setChunkSize",
    "setGridVisible",
    "setGridSpacing",
    "setProjection",
    "setProjectionParam",
    "setGripShow",
    "setGripDirection",
    "setSelectableKind",
    "setLodAutomatic",
    "setLodDepthVariable",
    "setLodManual",
    "setTransformGumballFlag",
];
const PUZZLE5D_RETAINED_PAYLOAD_SCHEMA: &str = "puzzle.5d.tool-command.v1";

fn puzzle5d_retained_extent(_command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
    let projection = puzzle5d_projection_value(&snapshot.0);
    let selection = interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).map_or(0, |selection| selection.ids.len());
    let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
    let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
    let target_volumes = projection.get("targetVolumes").and_then(Value::as_array).map_or(0, Vec::len);
    let items = selection.checked_add(parts)?.checked_add(fasteners)?.checked_add(target_volumes)?;
    (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
}

fn puzzle5d_retained_reduce(
    command: &Puzzle5dCommand,
    snapshot: &Puzzle5dPlaySnapshot,
    config: &Puzzle5dConfig,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    view_state: Option<&semio_framework_plugin::ViewModel>,
) -> Result<Emit<Puzzle5dMutation, Puzzle5dConfigMutation>, Fault> {
    let empty_selection = protocol::DomainSelection::default();
    let selection = interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).unwrap_or(&empty_selection);
    let window_id = view_state.and_then(|view| view.window_id.as_deref()).or_else(|| command.window_id()).unwrap_or(world3d::WINDOW_KIND_ID);
    let runtime = window_ownership::runtime(config, &window_ownership::Puzzle5dWindowConfig::default(), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
    Ok(with_puzzle5d_app(|app| app.handle_action_impl(command.action_id(), command.args(), command.window_id(), snapshot, &runtime, view_state, selection, None, None).0))
}

struct Puzzle5dWindowCommandWork {
    tool_id: &'static str,
    consumed: bool,
    instance_owner: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    view_state: Option<semio_framework_plugin::ViewModel>,
    window_config: Option<semio_framework_plugin::WindowConfigSnapshot>,
    window_transient: Option<semio_framework_plugin::WindowTransientSnapshot>,
    tool_run: Option<semio_framework_plugin::ToolRunView>,
    ephemeral: Option<EphemeralEmit<EditorApp<Puzzle5dPlayApp>>>,
}

impl Puzzle5dWindowCommandWork {
    fn new(tool_id: &'static str) -> Self {
        Self { tool_id, consumed: false, instance_owner: None, view_state: None, window_config: None, window_transient: None, tool_run: None, ephemeral: None }
    }

    /// ⏯️ Binds the instance's tool run as of admission — the identity Escape's `toolRunAbort` needs.
    fn with_tool_run(mut self, tool_run: Option<semio_framework_plugin::ToolRunView>) -> Self {
        self.tool_run = tool_run;
        self
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dWindowCommandWork {
    fn tool_id(&self) -> &'static str { self.tool_id }
    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) { self.view_state = view_state; }
    fn bind_instance_owner(&mut self, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) { self.instance_owner = Some(owner); }
    fn bind_window_owners(&mut self, config: Option<semio_framework_plugin::WindowConfigSnapshot>, transient: Option<semio_framework_plugin::WindowTransientSnapshot>) {
        self.window_config = config;
        self.window_transient = transient;
    }
    fn take_ephemeral(&mut self) -> EphemeralEmit<EditorApp<Puzzle5dPlayApp>> { self.ephemeral.take().unwrap_or_default() }
    fn extent(&self, _command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> { Some(1) }
    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        config: &Puzzle5dConfig,
        interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        if self.consumed { return Err(Fault::from("puzzle5d-window-work-repeated")); }
        let view = self.view_state.as_ref().ok_or_else(|| Fault::from("puzzle5d-window-context-required"))?;
        let window_id = view.window_id.as_deref().or_else(|| command.window_id()).ok_or_else(|| Fault::from("puzzle5d-window-id-required"))?;
        let window_config = window_ownership::config_from_snapshot(self.window_config.as_ref());
        let window_transient = window_ownership::transient_from_snapshot(self.window_transient.as_ref());
        let runtime = window_ownership::runtime(config, &window_config, &window_transient, window_id);
        let empty_selection = protocol::DomainSelection::default();
        let selection = interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).unwrap_or(&empty_selection);
        let (emit, ephemeral) = with_puzzle5d_app(|app| app.handle_action_impl(command.action_id(), command.args(), Some(window_id), snapshot, &runtime, Some(view), selection, self.instance_owner.as_ref(), self.tool_run.as_ref()));
        self.consumed = true;
        self.ephemeral = Some(ephemeral);
        Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(emit))
    }
}

/// 📤 `exportFixture` reads the document and publishes a download; it owns no mutation, so it resolves
/// straight from the snapshot and picks its lane by payload size: one inline effect under the guest's
/// contiguous request ceiling, the framework's segmented-download lane above it, a localized notice
/// above what one segmented download may carry. It is NOT a `dispatch_puzzle5d_action` arm because a
/// segmented download is a `PuzzleCommandWorkStep`, not an `Emit` — `◻️2d`'s `Puzzle2dExportWork` is
/// the same shape.
#[derive(Default)]
struct Puzzle5dExportWork {
    consumed: bool,
    view_state: Option<semio_framework_plugin::ViewModel>,
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dExportWork {
    fn tool_id(&self) -> &'static str {
        "exportFixture"
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }

    fn extent(&self, _command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        Some(1)
    }

    fn step(
        &mut self,
        _command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        if self.consumed {
            return Err(Fault::from("puzzle5d-export-work-repeated"));
        }
        self.consumed = true;
        let document: Puzzle5dDocument = serde_json::from_value(snapshot.0.clone()).map_err(|_| Fault::from("puzzle5d-export-document-malformed"))?;
        Ok(match export_fixture::puzzle5d_export_publication(&document)? {
            export_fixture::Puzzle5dExportPublication::Inline(effect) => {
                crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { effects: vec![effect], ui_scope: UiDirtyScope::None, ..Default::default() })
            }
            export_fixture::Puzzle5dExportPublication::Segmented(download) => crate::retained_command::PuzzleCommandWorkStep::Download(download),
            export_fixture::Puzzle5dExportPublication::Refused(_) => crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle5d_notice_emit(self.view_state.as_ref(), |labels| labels.export_too_large.as_str())),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dTransformStage {
    Selection,
    Parts,
    Volumes,
    Complete,
    Closing,
}

struct Puzzle5dTransformWork {
    tool_id: &'static str,
    stage: Puzzle5dTransformStage,
    selection_cursor: usize,
    part_cursor: usize,
    volume_cursor: usize,
    selected: HashSet<String>,
    mutations: Vec<Puzzle5dMutation>,
    locked: usize,
    moved: usize,
    view_state: Option<semio_framework_plugin::ViewModel>,
}

impl Puzzle5dTransformWork {
    fn new(tool_id: &'static str) -> Self {
        Self {
            tool_id,
            stage: Puzzle5dTransformStage::Selection,
            selection_cursor: 0,
            part_cursor: 0,
            volume_cursor: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
            locked: 0,
            moved: 0,
            view_state: None,
        }
    }

    fn source_len(command: &Puzzle5dCommand, interaction: &protocol::InteractionState) -> usize {
        command
            .args()
            .and_then(|args| args.get("ids"))
            .and_then(Value::as_array)
            .filter(|ids| !ids.is_empty())
            .map_or_else(|| interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE5D_GRANULARITY_PART).map_or(0, |selection| selection.ids.len()), Vec::len)
    }

    fn source_id<'a>(command: &'a Puzzle5dCommand, interaction: &'a protocol::InteractionState, index: usize) -> Option<&'a str> {
        if let Some(ids) = command.args().and_then(|args| args.get("ids")).and_then(Value::as_array).filter(|ids| !ids.is_empty()) {
            return ids.get(index).and_then(Value::as_str);
        }
        interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE5D_GRANULARITY_PART).and_then(|selection| selection.ids.get(index)).map(String::as_str)
    }

    fn axis(command: &Puzzle5dCommand, key: &str, fallback: f64) -> f64 {
        command.args().and_then(|args| args.get(key)).and_then(Value::as_f64).unwrap_or(fallback)
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    /// 📏️ The `[x, y, z]` extent a `scale` row carries in either arm of the `Puzzle5dScale` union — a bare
    /// number is uniform, an array is per-axis, absent is unscaled.
    fn scale_row(scale: Option<&Value>) -> [f64; 3] {
        match scale {
            Some(Value::Number(value)) => [value.as_f64(); 3],
            Some(Value::Array(values)) => [values.first().and_then(Value::as_f64).unwrap_or(1.0), values.get(1).and_then(Value::as_f64).unwrap_or(1.0), values.get(2).and_then(Value::as_f64).unwrap_or(1.0)],
            _ => [1.0; 3],
        }
    }

    /// 🏁️ The ONE terminal step every transform arm ends on: one history edit per gesture under the
    /// gumball's own coalesce key, or a single visible refusal when everything addressed was locked (an
    /// empty delta is indistinguishable from a dead gumball).
    fn complete(&mut self) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        self.stage = Puzzle5dTransformStage::Complete;
        let coalesce_key = match self.tool_id {
            "translateSelection" => "gumball-translate",
            "rotateSelection" => "gumball-rotate",
            "scaleSelection" => "gumball-scale",
            _ => return Err(Fault::from("puzzle5d-transform-tool-mismatch")),
        };
        if self.moved == 0 && self.locked > 0 {
            self.mutations.clear();
            return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle5d_notice_emit(self.view_state.as_ref(), |labels| labels.selection_locked.as_str())));
        }
        let mutations = std::mem::take(&mut self.mutations);
        Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: mutations, coalesce_key: Some(coalesce_key.to_string()), ui_scope: UiDirtyScope::Full, ..Default::default() }))
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dTransformWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let items = Self::source_len(command, interaction)
            .checked_add(projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?
            .checked_add(projection.get("targetVolumes").and_then(Value::as_array).map_or(0, Vec::len))?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        match self.stage {
            Puzzle5dTransformStage::Selection => {
                if let Some(id) = Self::source_id(command, interaction, self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-transform-selection-capacity"));
                    }
                    self.selected.insert(id.to_string());
                    self.selection_cursor += 1;
                    return Ok(Self::progress("puzzle5d-transform-selection", "Reading selected part", "Ausgewähltes Teil wird gelesen"));
                }
                if self.selected.is_empty() {
                    self.stage = Puzzle5dTransformStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle5d_notice_emit(self.view_state.as_ref(), |labels| labels.nothing_selected.as_str())));
                }
                self.stage = Puzzle5dTransformStage::Parts;
                Ok(Self::progress("puzzle5d-transform-part", "Transforming selected part", "Ausgewähltes Teil wird transformiert"))
            }
            Puzzle5dTransformStage::Parts => {
                let Some(row) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    if self.tool_id != "scaleSelection" {
                        return self.complete();
                    }
                    self.stage = Puzzle5dTransformStage::Volumes;
                    return Ok(Self::progress("puzzle5d-transform-volume", "Scaling target volume", "Zielvolumen wird skaliert"));
                };
                self.part_cursor += 1;
                let Some(id) = row.get("id").and_then(Value::as_str) else {
                    return Ok(Self::progress("puzzle5d-transform-part", "Skipping malformed part", "Fehlerhaftes Teil wird übersprungen"));
                };
                if !self.selected.contains(id) {
                    return Ok(Self::progress("puzzle5d-transform-part", "Scanning part", "Teil wird geprüft"));
                }
                if row.get("2d").and_then(|part| part.get("locked")).and_then(Value::as_bool).unwrap_or(false) {
                    self.locked += 1;
                    return Ok(Self::progress("puzzle5d-transform-part", "Skipping locked part", "Gesperrtes Teil wird übersprungen"));
                }
                self.moved += 1;
                let part_3d = row.get("3d").and_then(Value::as_object);
                let mutation = match self.tool_id {
                    "translateSelection" => {
                        let origin = part_3d.and_then(|part| part.get("origin")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default();
                        let delta = [Self::axis(command, "dx", 0.0), Self::axis(command, "dy", 0.0), Self::axis(command, "dz", 0.0)];
                        // 🎛️ A 5d part carries BOTH poses. A world drag that moved only `3d` would leave the
                        // board pin behind, so the same delta is projected back onto the flat pose through the
                        // one board↔world scale this artifact already places parts with
                        // ([`PUZZLE5D_FLAT_TO_WORLD`], the inverse of `🧬️schema/💡️inferences/🎛️flat-position`'s
                        // own plan projection). Rotation and scale leave the flat pose alone: neither moves a
                        // part's own origin.
                        let flat_x = row.get("2d").and_then(|part| part.get("x")).and_then(Value::as_f64).unwrap_or_default();
                        let flat_y = row.get("2d").and_then(|part| part.get("y")).and_then(Value::as_f64).unwrap_or_default();
                        self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::move_part_2d(id.to_string(), flat_x + delta[0] / PUZZLE5D_FLAT_TO_WORLD, flat_y - delta[1] / PUZZLE5D_FLAT_TO_WORLD));
                        crate::standards::v1::subsets::any::schema::mutations::move_part_3d(id.to_string(), [origin[0] + delta[0], origin[1] + delta[1], origin[2] + delta[2]])
                    }
                    "rotateSelection" => {
                        let orientation = part_3d.and_then(|part| part.get("orientation")).and_then(puzzle5d_value_as_f64_4).unwrap_or([0.0, 0.0, 0.0, 1.0]);
                        let delta = quat_from_axis_angle(Self::axis(command, "ax", 0.0), Self::axis(command, "ay", 0.0), Self::axis(command, "az", 0.0), Self::axis(command, "angle", 0.0));
                        crate::standards::v1::subsets::any::schema::mutations::rotate_part_3d(id.to_string(), Some(quat_mul(delta, orientation)))
                    }
                    "scaleSelection" => {
                        let current = Self::scale_row(part_3d.and_then(|part| part.get("scale")));
                        crate::standards::v1::subsets::any::schema::mutations::scale_part_3d(
                            id.to_string(),
                            Some(crate::Puzzle5dScale::Vec3([current[0] * Self::axis(command, "sx", 1.0), current[1] * Self::axis(command, "sy", 1.0), current[2] * Self::axis(command, "sz", 1.0)])),
                        )
                    }
                    _ => return Err(Fault::from("puzzle5d-transform-tool-mismatch")),
                };
                self.mutations.push(mutation);
                Ok(Self::progress("puzzle5d-transform-part", "Transforming selected part", "Ausgewähltes Teil wird transformiert"))
            }
            Puzzle5dTransformStage::Volumes => {
                let Some(row) = projection.get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.get(self.volume_cursor)) else {
                    return self.complete();
                };
                self.volume_cursor += 1;
                let Some(id) = row.get("id").and_then(Value::as_str) else {
                    return Ok(Self::progress("puzzle5d-transform-volume", "Skipping malformed target volume", "Fehlerhaftes Zielvolumen wird übersprungen"));
                };
                if !self.selected.contains(id) {
                    return Ok(Self::progress("puzzle5d-transform-volume", "Scanning target volume", "Zielvolumen wird geprüft"));
                }
                if row.get("locked").and_then(Value::as_bool).unwrap_or(false) {
                    self.locked += 1;
                    return Ok(Self::progress("puzzle5d-transform-volume", "Skipping locked target volume", "Gesperrtes Zielvolumen wird übersprungen"));
                }
                self.moved += 1;
                let current = Self::scale_row(row.get("scale"));
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::scale_target_volume(
                    id.to_string(),
                    Some(crate::Puzzle5dScale::Vec3([current[0] * Self::axis(command, "sx", 1.0), current[1] * Self::axis(command, "sy", 1.0), current[2] * Self::axis(command, "sz", 1.0)])),
                ));
                Ok(Self::progress("puzzle5d-transform-volume", "Scaling target volume", "Zielvolumen wird skaliert"))
            }
            Puzzle5dTransformStage::Complete => Err(Fault::from("puzzle5d-transform-complete-repolled")),
            Puzzle5dTransformStage::Closing => Err(Fault::from("puzzle5d-transform-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dTransformStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() || self.view_state.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let selected = {
            let mut selected = self.selected.extract_if(|_| true);
            selected.next()
        };
        if selected.is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dTransformStage::Closing && self.mutations.is_empty() && self.selected.is_empty() && self.view_state.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dKindWeightStage {
    Catalog,
    InferParts,
    InferGrips,
    Validate,
    SumOthers,
    Changed,
    Build,
    Publish,
    Complete,
    Closing,
}

struct Puzzle5dKindWeightWork {
    tool_id: &'static str,
    stage: Puzzle5dKindWeightStage,
    cursor: usize,
    part_cursor: usize,
    grip_cursor: usize,
    ids: Vec<String>,
    seen: HashSet<String>,
    result: HashMap<String, f64>,
    missing: bool,
    base_sum: f64,
    other_sum: f64,
    other_count: usize,
    changed_id: Option<String>,
    requested: f64,
}

impl Puzzle5dKindWeightWork {
    fn new(tool_id: &'static str) -> Self {
        Self {
            tool_id,
            stage: Puzzle5dKindWeightStage::Catalog,
            cursor: 0,
            part_cursor: 0,
            grip_cursor: 0,
            ids: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            seen: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            result: HashMap::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            missing: false,
            base_sum: 0.0,
            other_sum: 0.0,
            other_count: 0,
            changed_id: None,
            requested: 1.0,
        }
    }

    fn section(&self) -> &'static str {
        if self.tool_id == "setPartKindWeight" {
            "parts"
        } else {
            "grips"
        }
    }

    fn weights<'a>(&self, config: &'a Puzzle5dConfig) -> &'a HashMap<String, f64> {
        if self.tool_id == "setPartKindWeight" {
            &config.object_kind_weights
        } else {
            &config.vortex_kind_weights
        }
    }

    fn catalog(&self, snapshot: &Puzzle5dPlaySnapshot) -> Vec<Value> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        projection.get("kindCatalogs").and_then(|value| value.get(self.section())).and_then(Value::as_array).cloned().unwrap_or_default()
    }

    fn push_id(&mut self, id: &str, inferred: bool) -> Result<(), Fault> {
        if self.ids.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
            return Err(Fault::from("puzzle5d-kind-weight-catalog-capacity"));
        }
        if !inferred || self.seen.insert(id.to_string()) {
            self.ids.push(id.to_string());
        }
        Ok(())
    }

    fn base_weight(&self, config: &Puzzle5dConfig, id: &str) -> f64 {
        if self.ids.is_empty() {
            return 0.0;
        }
        if self.missing || self.weights(config).is_empty() {
            return 1.0 / self.ids.len() as f64;
        }
        let value = self.weights(config).get(id).copied().unwrap_or(0.0);
        if (self.base_sum - 1.0).abs() > 0.001 && self.base_sum.abs() > f64::EPSILON {
            value / self.base_sum
        } else {
            value
        }
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dKindWeightWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        Some(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        match self.stage {
            Puzzle5dKindWeightStage::Catalog => {
                let entries = self.catalog(snapshot);
                if let Some(entry) = entries.get(self.cursor) {
                    if let Some(id) = entry.get("id").and_then(Value::as_str) {
                        self.push_id(id, false)?;
                    }
                    self.cursor += 1;
                    return Ok(Self::progress("puzzle5d-kind-weight-catalog", "Reading kind owner", "Artinhaber wird gelesen"));
                }
                self.stage = if entries.is_empty() {
                    if self.tool_id == "setPartKindWeight" {
                        Puzzle5dKindWeightStage::InferParts
                    } else {
                        Puzzle5dKindWeightStage::InferGrips
                    }
                } else {
                    Puzzle5dKindWeightStage::Validate
                };
                self.cursor = 0;
                Ok(Self::progress("puzzle5d-kind-weight-infer", "Preparing kind validation", "Artprüfung wird vorbereitet"))
            }
            Puzzle5dKindWeightStage::InferParts => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dKindWeightStage::Validate;
                    return Ok(Self::progress("puzzle5d-kind-weight-validate", "Validating current weights", "Aktuelle Gewichte werden geprüft"));
                };
                self.part_cursor += 1;
                if let Some(id) = part.get("partKind").and_then(Value::as_str) {
                    self.push_id(id, true)?;
                }
                Ok(Self::progress("puzzle5d-kind-weight-part", "Reading inferred part kind", "Abgeleitete Teileart wird gelesen"))
            }
            Puzzle5dKindWeightStage::InferGrips => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dKindWeightStage::Validate;
                    return Ok(Self::progress("puzzle5d-kind-weight-validate", "Validating current weights", "Aktuelle Gewichte werden geprüft"));
                };
                let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
                    self.part_cursor += 1;
                    self.grip_cursor = 0;
                    return Ok(Self::progress("puzzle5d-kind-weight-part", "Advancing grip owner", "Griffinhaber wird gewechselt"));
                };
                self.grip_cursor += 1;
                if let Some(id) = grip.get("gripKind").and_then(Value::as_str) {
                    self.push_id(id, true)?;
                }
                Ok(Self::progress("puzzle5d-kind-weight-grip", "Reading inferred grip kind", "Abgeleitete Griffart wird gelesen"))
            }
            Puzzle5dKindWeightStage::Validate => {
                if self.changed_id.is_none() {
                    self.changed_id = Some(command.args().and_then(|args| args.get("kindId")).and_then(Value::as_str).unwrap_or("").to_string());
                    self.requested = command.args().and_then(|args| args.get("value")).and_then(Value::as_f64).unwrap_or(1.0).clamp(0.0, 1.0);
                }
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle5dKindWeightStage::SumOthers;
                    return Ok(Self::progress("puzzle5d-kind-weight-sum", "Measuring sibling weights", "Geschwistergewichte werden gemessen"));
                };
                let weights = self.weights(config);
                self.missing |= !weights.contains_key(id);
                self.base_sum += weights.get(id).copied().unwrap_or(0.0);
                self.cursor += 1;
                Ok(Self::progress("puzzle5d-kind-weight-validate", "Validating kind weight", "Artgewicht wird geprüft"))
            }
            Puzzle5dKindWeightStage::SumOthers => {
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle5dKindWeightStage::Changed;
                    return Ok(Self::progress("puzzle5d-kind-weight-changed", "Preparing changed weight", "Geändertes Gewicht wird vorbereitet"));
                };
                if self.changed_id.as_deref() != Some(id.as_str()) {
                    self.other_sum += self.base_weight(config, id);
                    self.other_count += 1;
                }
                self.cursor += 1;
                Ok(Self::progress("puzzle5d-kind-weight-sum", "Measuring sibling weight", "Geschwistergewicht wird gemessen"))
            }
            Puzzle5dKindWeightStage::Changed => {
                if self.ids.len() >= 2 {
                    self.result.insert(self.changed_id.clone().ok_or_else(|| Fault::from("puzzle5d-kind-weight-changed-owner"))?, self.requested);
                }
                self.stage = Puzzle5dKindWeightStage::Build;
                Ok(Self::progress("puzzle5d-kind-weight-build", "Building normalized weights", "Normalisierte Gewichte werden aufgebaut"))
            }
            Puzzle5dKindWeightStage::Build => {
                let Some(id) = self.ids.get(self.cursor).cloned() else {
                    self.stage = Puzzle5dKindWeightStage::Publish;
                    return Ok(Self::progress("puzzle5d-kind-weight-publish", "Preparing weight publication", "Gewichtsveröffentlichung wird vorbereitet"));
                };
                self.cursor += 1;
                let value = if self.ids.len() == 1 {
                    1.0
                } else if self.changed_id.as_deref() == Some(id.as_str()) {
                    return Ok(Self::progress("puzzle5d-kind-weight-build", "Keeping changed weight", "Geändertes Gewicht wird beibehalten"));
                } else {
                    let remainder = (1.0 - self.requested).max(0.0);
                    if self.other_sum <= f64::EPSILON {
                        remainder / self.other_count.max(1) as f64
                    } else {
                        self.base_weight(config, &id) / self.other_sum * remainder
                    }
                };
                self.result.insert(id, value);
                Ok(Self::progress("puzzle5d-kind-weight-build", "Building kind weight", "Artgewicht wird aufgebaut"))
            }
            Puzzle5dKindWeightStage::Publish => {
                self.stage = Puzzle5dKindWeightStage::Complete;
                let mutation = if self.tool_id == "setPartKindWeight" {
                    Puzzle5dConfigMutation::SetObjectKindWeights { value: std::mem::take(&mut self.result) }
                } else {
                    Puzzle5dConfigMutation::SetVortexKindWeights { value: std::mem::take(&mut self.result) }
                };
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { config_mutations: vec![mutation], ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dKindWeightStage::Complete => Err(Fault::from("puzzle5d-kind-weight-complete-repolled")),
            Puzzle5dKindWeightStage::Closing => Err(Fault::from("puzzle5d-kind-weight-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dKindWeightStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.ids.pop().is_some() || self.changed_id.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let seen = {
            let mut values = self.seen.extract_if(|_| true);
            values.next()
        };
        if seen.is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let result = {
            let mut values = self.result.extract_if(|_, _| true);
            values.next()
        };
        if result.is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dKindWeightStage::Closing && self.ids.is_empty() && self.seen.is_empty() && self.result.is_empty() && self.changed_id.is_none()
    }
}

/// 🚚️ The four halves of a gumball volume relocate — one addressed volume, then its origin, its
/// orientation and its extent, each a separate bounded step so a three-arm pose push never builds an
/// unbounded mutation list in one turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dRelocateVolumeStage {
    Search,
    Origin,
    Orientation,
    Scale,
    Complete,
    Closing,
}

struct Puzzle5dRelocateVolumeWork {
    stage: Puzzle5dRelocateVolumeStage,
    cursor: usize,
    volume_id: Option<String>,
    mutations: Vec<Puzzle5dMutation>,
}

impl Default for Puzzle5dRelocateVolumeWork {
    fn default() -> Self {
        Self { stage: Puzzle5dRelocateVolumeStage::Search, cursor: 0, volume_id: None, mutations: Vec::with_capacity(3) }
    }
}

impl Puzzle5dRelocateVolumeWork {
    fn complete(&mut self) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        self.stage = Puzzle5dRelocateVolumeStage::Complete;
        crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() })
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn volumes(snapshot: &Puzzle5dPlaySnapshot) -> Vec<Value> {
        puzzle5d_projection_value(&snapshot.0).get("targetVolumes").and_then(Value::as_array).cloned().unwrap_or_default()
    }

    fn owner(&self) -> Result<String, Fault> {
        self.volume_id.clone().ok_or_else(|| Fault::from("puzzle5d-relocate-volume-owner-lost"))
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dRelocateVolumeWork {
    fn tool_id(&self) -> &'static str {
        "relocateTargetVolume"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let items = Self::volumes(snapshot).len().checked_add(4)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let requested_id = command.args().and_then(|args| args.get("volumeId")).and_then(Value::as_str).unwrap_or("").to_string();
        let after = command.args().and_then(|args| args.get("after")).cloned();
        match self.stage {
            Puzzle5dRelocateVolumeStage::Search => {
                let volumes = Self::volumes(snapshot);
                let Some(volume) = volumes.get(self.cursor) else { return Ok(self.complete()) };
                self.cursor += 1;
                let locked = volume.get("locked").and_then(Value::as_bool).unwrap_or(false);
                if volume.get("id").and_then(Value::as_str) == Some(requested_id.as_str()) && !locked && after.is_some() {
                    self.volume_id = Some(requested_id);
                    self.stage = Puzzle5dRelocateVolumeStage::Origin;
                }
                Ok(Self::progress("puzzle5d-relocate-volume-search", "Finding target volume", "Zielvolumen wird gesucht"))
            }
            Puzzle5dRelocateVolumeStage::Origin => {
                if let Some(origin) = after.as_ref().and_then(|after| after.get("position")).and_then(puzzle5d_value_as_f64_3) {
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::move_target_volume(self.owner()?, origin));
                }
                self.stage = Puzzle5dRelocateVolumeStage::Orientation;
                Ok(Self::progress("puzzle5d-relocate-volume-orientation", "Preparing volume rotation", "Volumendrehung wird vorbereitet"))
            }
            Puzzle5dRelocateVolumeStage::Orientation => {
                if let Some(orientation) = after.as_ref().and_then(|after| after.get("quaternion")).and_then(puzzle5d_value_as_f64_4) {
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume(self.owner()?, Some(orientation)));
                }
                self.stage = Puzzle5dRelocateVolumeStage::Scale;
                Ok(Self::progress("puzzle5d-relocate-volume-scale", "Preparing volume scale", "Volumenskalierung wird vorbereitet"))
            }
            Puzzle5dRelocateVolumeStage::Scale => {
                if let Some(scale) = after.as_ref().and_then(|after| after.get("scale")).and_then(puzzle5d_value_as_f64_3) {
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::scale_target_volume(self.owner()?, Some(crate::Puzzle5dScale::Vec3(scale))));
                }
                Ok(self.complete())
            }
            Puzzle5dRelocateVolumeStage::Complete => Err(Fault::from("puzzle5d-relocate-volume-complete-repolled")),
            Puzzle5dRelocateVolumeStage::Closing => Err(Fault::from("puzzle5d-relocate-volume-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dRelocateVolumeStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() || self.volume_id.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dRelocateVolumeStage::Closing && self.mutations.is_empty() && self.volume_id.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dFocusSelectionStage {
    Selection,
    Parts,
    Publish,
    Complete,
    Closing,
}

struct Puzzle5dFocusSelectionWork {
    stage: Puzzle5dFocusSelectionStage,
    selection_cursor: usize,
    part_cursor: usize,
    selected: HashSet<String>,
    sum_2d: [f64; 2],
    sum_3d: [f64; 3],
    matched: usize,
    minimum_3d: [f64; 3],
    maximum_3d: [f64; 3],
    view_state: Option<semio_framework_plugin::ViewModel>,
    window_config: Option<semio_framework_plugin::WindowConfigSnapshot>,
}

impl Default for Puzzle5dFocusSelectionWork {
    fn default() -> Self {
        Self {
            stage: Puzzle5dFocusSelectionStage::Selection,
            selection_cursor: 0,
            part_cursor: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            sum_2d: [0.0; 2],
            sum_3d: [0.0; 3],
            matched: 0,
            minimum_3d: [f64::MAX; 3],
            maximum_3d: [f64::MIN; 3],
            view_state: None,
            window_config: None,
        }
    }
}

impl Puzzle5dFocusSelectionWork {
    fn source(interaction: &protocol::InteractionState) -> &[String] {
        interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE5D_GRANULARITY_PART).map(|selection| selection.ids.as_slice()).unwrap_or_default()
    }

    /// 🎥️ Which parts this focus frames. An EMPTY selection frames the WHOLE document rather than
    /// completing with `Emit::default()` — a camera verb with nothing selected has an obvious subject
    /// (everything), and the silent no-op made the focus keybinding dead on every boot before the user
    /// had picked anything (puzzle 3d's `Puzzle3dFocusSelectionWork::frames`). Costs no extra capacity:
    /// the part scan this work already declares in `extent` runs either way.
    fn frames(&self, part_id: &str) -> bool {
        self.selected.is_empty() || self.selected.contains(part_id)
    }

    fn axis(row: &Value, section: &str, field: &str, index: usize) -> f64 {
        row.get(section).and_then(|section| section.get(field)).and_then(Value::as_array).and_then(|values| values.get(index)).and_then(Value::as_f64).unwrap_or(0.0)
    }

    fn scalar(row: &Value, section: &str, field: &str) -> f64 {
        row.get(section).and_then(|section| section.get(field)).and_then(Value::as_f64).unwrap_or(0.0)
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dFocusSelectionWork {
    fn tool_id(&self) -> &'static str {
        "focusSelection"
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }

    fn bind_window_owners(&mut self, config: Option<semio_framework_plugin::WindowConfigSnapshot>, _transient: Option<semio_framework_plugin::WindowTransientSnapshot>) {
        self.window_config = config;
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let selected = Self::source(interaction).len();
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let items = selected.checked_add(parts)?.checked_add(1)?;
        (selected <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        _command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        match self.stage {
            Puzzle5dFocusSelectionStage::Selection => {
                if let Some(id) = Self::source(interaction).get(self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-focus-selection-capacity"));
                    }
                    self.selected.insert(id.clone());
                    self.selection_cursor += 1;
                    return Ok(Self::progress("puzzle5d-focus-selection-owner", "Reading selected part", "Ausgewähltes Teil wird gelesen"));
                }
                self.stage = Puzzle5dFocusSelectionStage::Parts;
                Ok(Self::progress("puzzle5d-focus-selection-part", "Finding selected part", "Ausgewähltes Teil wird gesucht"))
            }
            Puzzle5dFocusSelectionStage::Parts => {
                let Some(row) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dFocusSelectionStage::Publish;
                    return Ok(Self::progress("puzzle5d-focus-selection-publish", "Preparing camera focus", "Kamerafokus wird vorbereitet"));
                };
                self.part_cursor += 1;
                if row.get("id").and_then(Value::as_str).is_some_and(|id| self.frames(id)) {
                    self.sum_2d[0] += Self::scalar(row, "2d", "x");
                    self.sum_2d[1] += Self::scalar(row, "2d", "y");
                    self.sum_3d[0] += Self::axis(row, "3d", "origin", 0);
                    self.sum_3d[1] += Self::axis(row, "3d", "origin", 1);
                    self.sum_3d[2] += Self::axis(row, "3d", "origin", 2);
                    for axis in 0..3 {
                        let value = Self::axis(row, "3d", "origin", axis);
                        self.minimum_3d[axis] = self.minimum_3d[axis].min(value);
                        self.maximum_3d[axis] = self.maximum_3d[axis].max(value);
                    }
                    self.matched += 1;
                }
                Ok(Self::progress("puzzle5d-focus-selection-part", "Scanning selected part", "Ausgewähltes Teil wird geprüft"))
            }
            Puzzle5dFocusSelectionStage::Publish => {
                self.stage = Puzzle5dFocusSelectionStage::Complete;
                if self.matched == 0 {
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle5d_notice_emit(self.view_state.as_ref(), |labels| labels.nothing_selected.as_str())));
                }
                let divisor = self.matched as f64;
                let centre = [self.sum_3d[0] / divisor, self.sum_3d[1] / divisor, self.sum_3d[2] / divisor];
                // 📏️ Half the diagonal of the framed parts' own axis-aligned bound, accumulated in the ONE
                // part scan `extent` already declares — no second pass, no unbounded growth.
                let span = [self.maximum_3d[0] - self.minimum_3d[0], self.maximum_3d[1] - self.minimum_3d[1], self.maximum_3d[2] - self.minimum_3d[2]];
                let radius = (span[0] * span[0] + span[1] * span[1] + span[2] * span[2]).sqrt() / 2.0;
                let distance = radius * 3.0 + 2.0;
                // 🪟️ BOTH poses are written into the one window-config value; `addressed_config` projects
                // exactly the half the addressed pane owns (board → `camera2d`, world → `camera3d`), so a
                // focus in either pane keeps that pane's own camera coherent with the 5d dual pose.
                let mut next = window_ownership::config_from_snapshot(self.window_config.as_ref());
                let offset = [next.camera3d.position[0] - next.camera3d.target[0], next.camera3d.position[1] - next.camera3d.target[1], next.camera3d.position[2] - next.camera3d.target[2]];
                let orbit = (offset[0] * offset[0] + offset[1] * offset[1] + offset[2] * offset[2]).sqrt();
                let scale = if orbit > f64::EPSILON { distance / orbit } else { 1.0 };
                next.camera3d.target = centre;
                next.camera3d.position = [centre[0] + offset[0] * scale, centre[1] + offset[1] * scale, centre[2] + offset[2] * scale];
                next.camera2d.x = self.sum_2d[0] / divisor;
                next.camera2d.y = self.sum_2d[1] / divisor;
                let view = self.view_state.as_ref().ok_or_else(|| Fault::from("puzzle5d-focus-window-context-required"))?;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { window_config_mutations: vec![window_ownership::addressed_config(view, next)?], ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dFocusSelectionStage::Complete => Err(Fault::from("puzzle5d-focus-selection-complete-repolled")),
            Puzzle5dFocusSelectionStage::Closing => Err(Fault::from("puzzle5d-focus-selection-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dFocusSelectionStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let selected = {
            let mut selected = self.selected.extract_if(|_| true);
            selected.next()
        };
        if selected.is_some() || self.view_state.take().is_some() || self.window_config.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dFocusSelectionStage::Closing && self.selected.is_empty() && self.view_state.is_none() && self.window_config.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dPatchPartStage {
    Selection,
    Parts,
    Complete,
    Closing,
}

struct Puzzle5dPatchPartWork {
    stage: Puzzle5dPatchPartStage,
    selection_cursor: usize,
    part_cursor: usize,
    selected: HashSet<String>,
    mutations: Vec<Puzzle5dMutation>,
    view_state: Option<semio_framework_plugin::ViewModel>,
}

impl Default for Puzzle5dPatchPartWork {
    fn default() -> Self {
        Self {
            stage: Puzzle5dPatchPartStage::Selection,
            selection_cursor: 0,
            part_cursor: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
            view_state: None,
        }
    }
}

impl Puzzle5dPatchPartWork {
    fn source_len(command: &Puzzle5dCommand) -> usize {
        let args = command.args();
        args.and_then(|args| args.get("partIds")).and_then(Value::as_array).map_or(0, Vec::len) + usize::from(args.and_then(|args| args.get("partId")).and_then(Value::as_str).is_some())
    }

    fn source_id(command: &Puzzle5dCommand, index: usize) -> Option<&str> {
        let args = command.args()?;
        let ids = args.get("partIds").and_then(Value::as_array);
        if let Some(id) = ids.and_then(|ids| ids.get(index)).and_then(Value::as_str) {
            return (!id.is_empty()).then_some(id);
        }
        (index == ids.map_or(0, Vec::len)).then(|| args.get("partId").and_then(Value::as_str)).flatten().filter(|id| !id.is_empty())
    }

    fn mutation(command: &Puzzle5dCommand, part: &Puzzle5dPart) -> Option<Puzzle5dMutation> {
        let args = command.args()?;
        let field = args.get("field").and_then(Value::as_str).unwrap_or("");
        let value = args.get("value");
        let delta = args.get("delta");
        let text = value.and_then(Value::as_str);
        match field {
            "partKind" => text.map(|text| crate::standards::v1::subsets::any::schema::mutations::change_part_kind(part.id.clone(), Some(text.to_string()))),
            "anchor" => text.map(|text| {
                let anchor = match text.to_ascii_lowercase().as_str() {
                    "derived" | "connected" => crate::Puzzle5dPartAnchor::Derived,
                    _ => crate::Puzzle5dPartAnchor::Fixed,
                };
                crate::standards::v1::subsets::any::schema::mutations::change_part_anchor(part.id.clone(), anchor)
            }),
            "text" => text.map(|text| crate::standards::v1::subsets::any::schema::mutations::edit_part_2d_text(part.id.clone(), Some(text.to_string()))),
            "label" => Some(crate::standards::v1::subsets::any::schema::mutations::edit_part_3d_label(part.id.clone(), text.filter(|text| !text.is_empty()).map(str::to_string))),
            "meshUrl" => Some(crate::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh(part.id.clone(), text.filter(|text| !text.is_empty()).map(str::to_string))),
            "x" => puzzle5d_resolve_number_edit(part.part_2d.x, value, delta).map(|updated| crate::standards::v1::subsets::any::schema::mutations::move_part_2d(part.id.clone(), updated, part.part_2d.y)),
            "y" => puzzle5d_resolve_number_edit(part.part_2d.y, value, delta).map(|updated| crate::standards::v1::subsets::any::schema::mutations::move_part_2d(part.id.clone(), part.part_2d.x, updated)),
            _ => {
                let axis = puzzle5d_axis_index(field, "origin")?;
                let updated = puzzle5d_resolve_number_edit(part.part_3d.origin[axis], value, delta)?;
                let mut origin = part.part_3d.origin;
                origin[axis] = updated;
                Some(crate::standards::v1::subsets::any::schema::mutations::move_part_3d(part.id.clone(), origin))
            }
        }
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dPatchPartWork {
    fn tool_id(&self) -> &'static str {
        "patchPart"
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let items = Self::source_len(command).checked_add(projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        match self.stage {
            Puzzle5dPatchPartStage::Selection => {
                if let Some(id) = Self::source_id(command, self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-patch-part-selection-capacity"));
                    }
                    self.selected.insert(id.to_string());
                    self.selection_cursor += 1;
                    return Ok(Self::progress("puzzle5d-patch-part-selection", "Reading part target", "Teilziel wird gelesen"));
                }
                self.stage = Puzzle5dPatchPartStage::Parts;
                Ok(Self::progress("puzzle5d-patch-part", "Patching part", "Teil wird geändert"))
            }
            Puzzle5dPatchPartStage::Parts => {
                let Some(row) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)).cloned() else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    // 🧯️ An unaddressed id or a field this inspector cannot write produced no mutation at all;
                    // the pre-migration arm fell through silently and the panel looked dead.
                    if self.mutations.is_empty() {
                        return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle5d_notice_emit(self.view_state.as_ref(), |labels| labels.edit_not_applicable.as_str())));
                    }
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                self.part_cursor += 1;
                let part: Puzzle5dPart = serde_json::from_value(serde_json::Value::from(&dsl::os_pack::json::to_dsl_value(&row))).map_err(|_| Fault::from("puzzle5d-patch-part-malformed"))?;
                if self.selected.contains(&part.id) {
                    if let Some(mutation) = Self::mutation(command, &part) {
                        self.mutations.push(mutation);
                    }
                }
                Ok(Self::progress("puzzle5d-patch-part", "Patching part", "Teil wird geändert"))
            }
            Puzzle5dPatchPartStage::Complete => Err(Fault::from("puzzle5d-patch-part-complete-repolled")),
            Puzzle5dPatchPartStage::Closing => Err(Fault::from("puzzle5d-patch-part-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dPatchPartStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() || self.view_state.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let selected = {
            let mut selected = self.selected.extract_if(|_| true);
            selected.next()
        };
        if selected.is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dPatchPartStage::Closing && self.mutations.is_empty() && self.selected.is_empty() && self.view_state.is_none()
    }
}

struct Puzzle5dPatchFastenerWork {
    stage: Puzzle5dPatchPartStage,
    selection_cursor: usize,
    fastener_cursor: usize,
    selected: HashSet<String>,
    mutations: Vec<Puzzle5dMutation>,
}

impl Default for Puzzle5dPatchFastenerWork {
    fn default() -> Self {
        Self {
            stage: Puzzle5dPatchPartStage::Selection,
            selection_cursor: 0,
            fastener_cursor: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
        }
    }
}

impl Puzzle5dPatchFastenerWork {
    fn source_len(command: &Puzzle5dCommand) -> usize {
        let args = command.args();
        args.and_then(|args| args.get("fastenerIds")).and_then(Value::as_array).map_or(0, Vec::len) + usize::from(args.and_then(|args| args.get("fastenerId")).and_then(Value::as_str).is_some())
    }

    fn source_id(command: &Puzzle5dCommand, index: usize) -> Option<&str> {
        let args = command.args()?;
        let ids = args.get("fastenerIds").and_then(Value::as_array);
        if let Some(id) = ids.and_then(|ids| ids.get(index)).and_then(Value::as_str) {
            return (!id.is_empty()).then_some(id);
        }
        (index == ids.map_or(0, Vec::len)).then(|| args.get("fastenerId").and_then(Value::as_str)).flatten().filter(|id| !id.is_empty())
    }

    fn mutation(command: &Puzzle5dCommand, fastener: &Puzzle5dFastener) -> Option<Puzzle5dMutation> {
        let args = command.args()?;
        let field = args.get("field").and_then(Value::as_str).unwrap_or("");
        let value = args.get("value");
        let delta = args.get("delta");
        if field == "fastenerKind" {
            return Some(crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind(fastener.id.clone(), value.and_then(Value::as_str).filter(|text| !text.is_empty()).map(str::to_string)));
        }
        let mut geometry = [fastener.gap, fastener.shift, fastener.rise, fastener.rotation, fastener.turn, fastener.tilt, fastener.x, fastener.y];
        let index = match field {
            "gap" => 0,
            "shift" => 1,
            "rise" => 2,
            "rotation" => 3,
            "turn" => 4,
            "tilt" => 5,
            "x" => 6,
            "y" => 7,
            _ => return None,
        };
        geometry[index] = puzzle5d_resolve_number_edit(geometry[index], value, delta)?;
        Some(crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry(crate::standards::v1::subsets::any::schema::mutations::ReplaceFastenerGeometry { id: fastener.id.clone(), new_gap: geometry[0], new_shift: geometry[1], new_rise: geometry[2], new_rotation: geometry[3], new_turn: geometry[4], new_tilt: geometry[5], new_x: geometry[6], new_y: geometry[7] }))
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dPatchFastenerWork {
    fn tool_id(&self) -> &'static str {
        "patchFastener"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let items = Self::source_len(command).checked_add(projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len))?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        match self.stage {
            Puzzle5dPatchPartStage::Selection => {
                if let Some(id) = Self::source_id(command, self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-patch-fastener-selection-capacity"));
                    }
                    self.selected.insert(id.to_string());
                    self.selection_cursor += 1;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-patch-fastener-selection", "Reading fastener target", "Verbindungsziel wird gelesen"));
                }
                self.stage = Puzzle5dPatchPartStage::Parts;
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-patch-fastener", "Patching fastener", "Verbindung wird geändert"))
            }
            Puzzle5dPatchPartStage::Parts => {
                let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)).cloned() else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                self.fastener_cursor += 1;
                let fastener: Puzzle5dFastener = serde_json::from_value(serde_json::Value::from(&dsl::os_pack::json::to_dsl_value(&row))).map_err(|_| Fault::from("puzzle5d-patch-fastener-malformed"))?;
                if self.selected.contains(&fastener.id) {
                    if let Some(mutation) = Self::mutation(command, &fastener) {
                        self.mutations.push(mutation);
                    }
                }
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-patch-fastener", "Patching fastener", "Verbindung wird geändert"))
            }
            Puzzle5dPatchPartStage::Complete => Err(Fault::from("puzzle5d-patch-fastener-complete-repolled")),
            Puzzle5dPatchPartStage::Closing => Err(Fault::from("puzzle5d-patch-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dPatchPartStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let selected = {
            let mut selected = self.selected.extract_if(|_| true);
            selected.next()
        };
        if selected.is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dPatchPartStage::Closing && self.mutations.is_empty() && self.selected.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dEditFastenerStage {
    Scan,
    Kind,
    Geometry,
    Complete,
    Closing,
}

struct Puzzle5dEditFastenerWork {
    stage: Puzzle5dEditFastenerStage,
    cursor: usize,
    fastener: Option<Puzzle5dFastener>,
    mutations: Vec<Puzzle5dMutation>,
}

impl Default for Puzzle5dEditFastenerWork {
    fn default() -> Self {
        Self { stage: Puzzle5dEditFastenerStage::Scan, cursor: 0, fastener: None, mutations: Vec::with_capacity(2) }
    }
}

impl Puzzle5dEditFastenerWork {
    fn id(command: &Puzzle5dCommand) -> &str {
        command.args().and_then(|args| args.get("id").or_else(|| args.get("fastenerId"))).and_then(Value::as_str).filter(|id| !id.is_empty()).unwrap_or("")
    }

    fn updated_kind(command: &Puzzle5dCommand, current: &Puzzle5dFastener) -> Option<Option<String>> {
        let args = command.args()?;
        let mut update = args.get("fastenerKind").or_else(|| args.get("edgeKind")).and_then(Value::as_str).filter(|text| !text.is_empty()).map(|text| Some(text.to_string()));
        if matches!(args.get("field").and_then(Value::as_str), Some("fastenerKind" | "edgeKind")) {
            update = Some(args.get("value").and_then(Value::as_str).filter(|text| !text.is_empty()).map(str::to_string));
        }
        update.filter(|updated| updated != &current.fastener_kind)
    }

    fn updated_geometry(command: &Puzzle5dCommand, current: &Puzzle5dFastener) -> Option<[f64; 8]> {
        let args = command.args()?;
        let mut geometry = [current.gap, current.shift, current.rise, current.rotation, current.turn, current.tilt, current.x, current.y];
        let keys = ["gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y"];
        let mut changed = false;
        for (index, key) in keys.iter().enumerate() {
            if let Some(value) = args.get(key) {
                if let Some(updated) = puzzle5d_resolve_number_edit(geometry[index], Some(value), None) {
                    changed |= updated != geometry[index];
                    geometry[index] = updated;
                }
            }
        }
        if let Some(index) = keys.iter().position(|key| args.get("field").and_then(Value::as_str) == Some(*key)) {
            if let Some(updated) = puzzle5d_resolve_number_edit(geometry[index], args.get("value"), args.get("delta")) {
                changed |= updated != geometry[index];
                geometry[index] = updated;
            }
        }
        changed.then_some(geometry)
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dEditFastenerWork {
    fn tool_id(&self) -> &'static str {
        "editFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len).checked_add(2).filter(|items| *items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        match self.stage {
            Puzzle5dEditFastenerStage::Scan => {
                let target = Self::id(command);
                if target.is_empty() {
                    self.stage = Puzzle5dEditFastenerStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                }
                let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.cursor)).cloned() else {
                    self.stage = Puzzle5dEditFastenerStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                };
                self.cursor += 1;
                if row.get("id").and_then(Value::as_str) == Some(target) {
                    self.fastener = Some(serde_json::from_value(serde_json::Value::from(&dsl::os_pack::json::to_dsl_value(&row))).map_err(|_| Fault::from("puzzle5d-edit-fastener-malformed"))?);
                    self.stage = Puzzle5dEditFastenerStage::Kind;
                }
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-edit-fastener-scan", "Finding fastener", "Verbindung wird gesucht"))
            }
            Puzzle5dEditFastenerStage::Kind => {
                let Some(fastener) = self.fastener.as_ref() else { return Err(Fault::from("puzzle5d-edit-fastener-owner")) };
                if let Some(kind) = Self::updated_kind(command, fastener) {
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind(fastener.id.clone(), kind));
                }
                self.stage = Puzzle5dEditFastenerStage::Geometry;
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-edit-fastener-kind", "Updating fastener kind", "Verbindungsart wird aktualisiert"))
            }
            Puzzle5dEditFastenerStage::Geometry => {
                let Some(fastener) = self.fastener.as_ref() else { return Err(Fault::from("puzzle5d-edit-fastener-owner")) };
                if let Some(geometry) = Self::updated_geometry(command, fastener) {
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry(crate::standards::v1::subsets::any::schema::mutations::ReplaceFastenerGeometry { id: fastener.id.clone(), new_gap: geometry[0], new_shift: geometry[1], new_rise: geometry[2], new_rotation: geometry[3], new_turn: geometry[4], new_tilt: geometry[5], new_x: geometry[6], new_y: geometry[7] }));
                }
                self.stage = Puzzle5dEditFastenerStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dEditFastenerStage::Complete => Err(Fault::from("puzzle5d-edit-fastener-complete-repolled")),
            Puzzle5dEditFastenerStage::Closing => Err(Fault::from("puzzle5d-edit-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dEditFastenerStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() || self.fastener.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dEditFastenerStage::Closing && self.mutations.is_empty() && self.fastener.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dRetargetFastenerStage {
    Fastener,
    SourceGrip,
    TargetGrip,
    Duplicate,
    Compatibility,
    Disconnect,
    Connect,
    Complete,
    Closing,
}

struct Puzzle5dRetargetFastenerWork {
    stage: Puzzle5dRetargetFastenerStage,
    part_cursor: usize,
    grip_cursor: usize,
    fastener_cursor: usize,
    compatibility_cursor: usize,
    processed_units: usize,
    fastener: Option<Puzzle5dFastener>,
    source: Option<String>,
    target: Option<String>,
    source_kind: Option<String>,
    target_kind: Option<String>,
    mutations: Vec<Puzzle5dMutation>,
}

impl Default for Puzzle5dRetargetFastenerWork {
    fn default() -> Self {
        Self {
            stage: Puzzle5dRetargetFastenerStage::Fastener,
            part_cursor: 0,
            grip_cursor: 0,
            fastener_cursor: 0,
            compatibility_cursor: 0,
            processed_units: 0,
            fastener: None,
            source: None,
            target: None,
            source_kind: None,
            target_kind: None,
            mutations: Vec::with_capacity(2),
        }
    }
}

impl Puzzle5dRetargetFastenerWork {
    fn argument<'a>(command: &'a Puzzle5dCommand, primary: &str, alias: &str) -> Option<&'a str> {
        command.args().and_then(|args| args.get(primary).or_else(|| args.get(alias))).and_then(Value::as_str).filter(|value| !value.is_empty())
    }

    fn scan_grip(&mut self, snapshot: &Puzzle5dPlaySnapshot, target: &str) -> Puzzle5dGripScan {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
            return Puzzle5dGripScan::Exhausted;
        };
        let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
            self.part_cursor += 1;
            self.grip_cursor = 0;
            return Puzzle5dGripScan::Progress;
        };
        self.grip_cursor += 1;
        let Some(part_id) = part.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        if puzzle5d_grip_full_id(part_id, grip_id) != target {
            return Puzzle5dGripScan::Progress;
        }
        let kind = grip.get("gripKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).or_else(|| grip.get("2d").and_then(|value| value.get("gripKind")).and_then(Value::as_str).filter(|kind| !kind.is_empty())).map(str::to_string);
        Puzzle5dGripScan::Found(kind)
    }

    fn complete_empty(&mut self) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        self.stage = Puzzle5dRetargetFastenerStage::Complete;
        crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default())
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dRetargetFastenerWork {
    fn tool_id(&self) -> &'static str {
        "retargetFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
        let items = parts.checked_mul(2)?.checked_add(fasteners.checked_mul(2)?)?.checked_add(compatibility)?.checked_add(2)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-retarget-fastener-work-capacity"));
        }
        self.processed_units += 1;
        match self.stage {
            Puzzle5dRetargetFastenerStage::Fastener => {
                let Some(id) = Self::argument(command, "id", "fastenerId") else { return Ok(self.complete_empty()) };
                let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)).cloned() else {
                    return Ok(self.complete_empty());
                };
                self.fastener_cursor += 1;
                if row.get("id").and_then(Value::as_str) == Some(id) {
                    let fastener: Puzzle5dFastener = serde_json::from_value(serde_json::Value::from(&dsl::os_pack::json::to_dsl_value(&row))).map_err(|_| Fault::from("puzzle5d-retarget-fastener-malformed"))?;
                    self.source = Some(Self::argument(command, "source", "attracting").map_or_else(|| fastener.source.clone(), str::to_string));
                    self.target = Some(Self::argument(command, "target", "attracted").map_or_else(|| fastener.target.clone(), str::to_string));
                    if self.source.as_deref().is_none_or(str::is_empty) || self.target.as_deref().is_none_or(str::is_empty) || self.source == self.target {
                        return Ok(self.complete_empty());
                    }
                    self.fastener = Some(fastener);
                    self.part_cursor = 0;
                    self.grip_cursor = 0;
                    self.stage = Puzzle5dRetargetFastenerStage::SourceGrip;
                }
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-retarget-fastener", "Finding fastener", "Verbindung wird gesucht"))
            }
            Puzzle5dRetargetFastenerStage::SourceGrip => {
                let source = self.source.as_deref().unwrap_or("").to_string();
                match self.scan_grip(snapshot, &source) {
                    Puzzle5dGripScan::Progress => Ok(Puzzle5dPatchPartWork::progress("puzzle5d-retarget-source", "Finding source grip", "Quellgriff wird gesucht")),
                    Puzzle5dGripScan::Found(kind) => {
                        self.source_kind = kind;
                        self.part_cursor = 0;
                        self.grip_cursor = 0;
                        self.stage = Puzzle5dRetargetFastenerStage::TargetGrip;
                        Ok(Puzzle5dPatchPartWork::progress("puzzle5d-retarget-target", "Finding target grip", "Zielgriff wird gesucht"))
                    }
                    Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
                }
            }
            Puzzle5dRetargetFastenerStage::TargetGrip => {
                let target = self.target.as_deref().unwrap_or("").to_string();
                match self.scan_grip(snapshot, &target) {
                    Puzzle5dGripScan::Progress => Ok(Puzzle5dPatchPartWork::progress("puzzle5d-retarget-target", "Finding target grip", "Zielgriff wird gesucht")),
                    Puzzle5dGripScan::Found(kind) => {
                        self.target_kind = kind;
                        self.fastener_cursor = 0;
                        self.stage = Puzzle5dRetargetFastenerStage::Duplicate;
                        Ok(Puzzle5dPatchPartWork::progress("puzzle5d-retarget-duplicate", "Checking duplicate fastener", "Doppelte Verbindung wird geprüft"))
                    }
                    Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
                }
            }
            Puzzle5dRetargetFastenerStage::Duplicate => {
                if let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let id = row.get("id").and_then(Value::as_str).unwrap_or("");
                    let source = row.get("source").and_then(Value::as_str).unwrap_or("");
                    let target = row.get("target").and_then(Value::as_str).unwrap_or("");
                    let own_id = self.fastener.as_ref().map_or("", |fastener| fastener.id.as_str());
                    let next_source = self.source.as_deref().unwrap_or("");
                    let next_target = self.target.as_deref().unwrap_or("");
                    if id != own_id && ((source == next_source && target == next_target) || (source == next_target && target == next_source)) {
                        return Ok(self.complete_empty());
                    }
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-retarget-duplicate", "Checking duplicate fastener", "Doppelte Verbindung wird geprüft"));
                }
                self.stage = Puzzle5dRetargetFastenerStage::Compatibility;
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-retarget-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dRetargetFastenerStage::Compatibility => {
                let rows = projection.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if rows.is_empty() || self.source_kind.is_none() || self.target_kind.is_none() {
                    self.stage = Puzzle5dRetargetFastenerStage::Disconnect;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-retarget-disconnect", "Disconnecting old fastener", "Alte Verbindung wird getrennt"));
                }
                let Some(row) = rows.get(self.compatibility_cursor) else { return Ok(self.complete_empty()) };
                self.compatibility_cursor += 1;
                let source = row.get("source").and_then(Value::as_str).unwrap_or("");
                let target = row.get("target").and_then(Value::as_str).unwrap_or("");
                let bidirectional = row.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
                let source_kind = self.source_kind.as_deref().unwrap_or("");
                let target_kind = self.target_kind.as_deref().unwrap_or("");
                if (source == source_kind && target == target_kind) || (bidirectional && source == target_kind && target == source_kind) {
                    self.stage = Puzzle5dRetargetFastenerStage::Disconnect;
                }
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-retarget-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dRetargetFastenerStage::Disconnect => {
                let id = self.fastener.as_ref().map(|fastener| fastener.id.clone()).ok_or_else(|| Fault::from("puzzle5d-retarget-fastener-owner"))?;
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id));
                self.stage = Puzzle5dRetargetFastenerStage::Connect;
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-retarget-connect", "Connecting retargeted fastener", "Neu ausgerichtete Verbindung wird erstellt"))
            }
            Puzzle5dRetargetFastenerStage::Connect => {
                let fastener = self.fastener.as_ref().ok_or_else(|| Fault::from("puzzle5d-retarget-fastener-owner"))?;
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(
                    fastener.id.clone(),
                    self.source.as_ref().cloned().ok_or_else(|| Fault::from("puzzle5d-retarget-source-owner"))?,
                    self.target.as_ref().cloned().ok_or_else(|| Fault::from("puzzle5d-retarget-target-owner"))?,
                    fastener.fastener_kind.clone(),
                    fastener.gap,
                    fastener.shift,
                    fastener.rise,
                    fastener.rotation,
                    fastener.turn,
                    fastener.tilt,
                    fastener.x,
                    fastener.y,
                ));
                self.stage = Puzzle5dRetargetFastenerStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dRetargetFastenerStage::Complete => Err(Fault::from("puzzle5d-retarget-fastener-complete-repolled")),
            Puzzle5dRetargetFastenerStage::Closing => Err(Fault::from("puzzle5d-retarget-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dRetargetFastenerStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() || self.fastener.take().is_some() || self.source.take().is_some() || self.target.take().is_some() || self.source_kind.take().is_some() || self.target_kind.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dRetargetFastenerStage::Closing && self.mutations.is_empty() && self.fastener.is_none() && self.source.is_none() && self.target.is_none() && self.source_kind.is_none() && self.target_kind.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dProximityConnectStage {
    Moved,
    Candidate,
    Existing,
    Compatibility,
    Emit,
    Complete,
    Closing,
}

struct Puzzle5dProximityConnectWork {
    stage: Puzzle5dProximityConnectStage,
    part_cursor: usize,
    grip_cursor: usize,
    fastener_cursor: usize,
    compatibility_cursor: usize,
    processed_units: usize,
    moved_id: Option<String>,
    moved_kind: Option<String>,
    moved_position: Option<[f64; 3]>,
    candidate_id: Option<String>,
    candidate_kind: Option<String>,
    mutations: Vec<Puzzle5dMutation>,
    operation_nonce: u64,
    fresh_cursor: u64,
}

impl Default for Puzzle5dProximityConnectWork {
    fn default() -> Self {
        Self {
            stage: Puzzle5dProximityConnectStage::Moved,
            part_cursor: 0,
            grip_cursor: 0,
            fastener_cursor: 0,
            compatibility_cursor: 0,
            processed_units: 0,
            moved_id: None,
            moved_kind: None,
            moved_position: None,
            candidate_id: None,
            candidate_kind: None,
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
            operation_nonce: 0,
            fresh_cursor: 0,
        }
    }
}

impl Puzzle5dProximityConnectWork {
    fn argument<'a>(command: &'a Puzzle5dCommand, key: &str) -> Option<&'a str> {
        command.args().and_then(|args| args.get(key)).and_then(Value::as_str).filter(|value| !value.is_empty())
    }

    fn grip_kind(grip: &Value) -> Option<String> {
        grip.get("gripKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).or_else(|| grip.get("2d").and_then(|value| value.get("gripKind")).and_then(Value::as_str).filter(|kind| !kind.is_empty())).map(str::to_string)
    }

    fn world_position(part: &Value, grip: &Value) -> [f64; 3] {
        let origin = part.get("3d").and_then(|part| part.get("origin")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default();
        let orientation = part.get("3d").and_then(|part| part.get("orientation")).and_then(puzzle5d_value_as_f64_4).unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let position = grip.get("3d").and_then(|grip| grip.get("position")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default();
        let rotated = quat_rotate_vector(orientation, position);
        [origin[0] + rotated[0], origin[1] + rotated[1], origin[2] + rotated[2]]
    }

    fn clear_candidate(&mut self) {
        self.candidate_id = None;
        self.candidate_kind = None;
        self.fastener_cursor = 0;
        self.compatibility_cursor = 0;
        self.stage = Puzzle5dProximityConnectStage::Candidate;
    }

    fn complete(&mut self) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        self.stage = Puzzle5dProximityConnectStage::Complete;
        crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() })
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dProximityConnectWork {
    fn tool_id(&self) -> &'static str {
        "proximityConnect"
    }

    fn bind_operation(&mut self, operation: Operation) {
        self.operation_nonce = operation.operation.0 ^ operation.generation.0.rotate_left(17) ^ operation.seed.rotate_left(31);
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
        let items = parts.checked_add(fasteners.checked_mul(parts)?)?.checked_add(compatibility.checked_mul(parts)?)?.checked_add(parts)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-proximity-connect-work-capacity"));
        }
        self.processed_units += 1;
        let part_id = Self::argument(command, "partId").or_else(|| Self::argument(command, "objectId")).unwrap_or("");
        if part_id.is_empty() {
            return Ok(self.complete());
        }
        match self.stage {
            Puzzle5dProximityConnectStage::Moved => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    return Ok(self.complete());
                };
                self.part_cursor += 1;
                if part.get("id").and_then(Value::as_str) == Some(part_id) {
                    let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.first()) else { return Ok(self.complete()) };
                    let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Ok(self.complete()) };
                    self.moved_id = Some(puzzle5d_grip_full_id(part_id, grip_id));
                    self.moved_kind = Self::grip_kind(grip);
                    self.moved_position = Some(Self::world_position(part, grip));
                    self.part_cursor = 0;
                    self.grip_cursor = 0;
                    self.stage = Puzzle5dProximityConnectStage::Candidate;
                }
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-moved", "Finding moved grip", "Verschobener Griff wird gesucht"))
            }
            Puzzle5dProximityConnectStage::Candidate => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    return Ok(self.complete());
                };
                let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
                    self.part_cursor += 1;
                    self.grip_cursor = 0;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                };
                self.grip_cursor += 1;
                if part.get("id").and_then(Value::as_str) == Some(part_id) {
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                }
                let Some(peer_part_id) = part.get("id").and_then(Value::as_str) else { return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-candidate", "Skipping malformed part", "Fehlerhaftes Teil wird übersprungen")) };
                let Some(peer_grip_id) = grip.get("id").and_then(Value::as_str) else { return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-candidate", "Skipping malformed grip", "Fehlerhafter Griff wird übersprungen")) };
                let peer_id = puzzle5d_grip_full_id(peer_part_id, peer_grip_id);
                if self.moved_id.as_deref() == Some(peer_id.as_str()) {
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                }
                let moved = self.moved_position.ok_or_else(|| Fault::from("puzzle5d-proximity-position-owner"))?;
                let peer = Self::world_position(part, grip);
                let radius = command.args().and_then(|args| args.get("radius")).and_then(Value::as_f64).unwrap_or(PUZZLE5D_PROXIMITY_RADIUS).max(0.0);
                let dx = moved[0] - peer[0];
                let dy = moved[1] - peer[1];
                let dz = moved[2] - peer[2];
                if (dx * dx + dy * dy + dz * dz).sqrt() > radius {
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                }
                self.candidate_id = Some(peer_id);
                self.candidate_kind = Self::grip_kind(grip);
                self.fastener_cursor = 0;
                self.stage = Puzzle5dProximityConnectStage::Existing;
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Existing => {
                if let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let source = row.get("source").and_then(Value::as_str).unwrap_or("");
                    let target = row.get("target").and_then(Value::as_str).unwrap_or("");
                    let peer = self.candidate_id.as_deref().unwrap_or("");
                    let moved = self.moved_id.as_deref().unwrap_or("");
                    if (source == peer && target == moved) || (source == moved && target == peer) {
                        self.clear_candidate();
                    }
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"));
                }
                self.stage = Puzzle5dProximityConnectStage::Compatibility;
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Compatibility => {
                let rows = projection.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if rows.is_empty() || self.candidate_kind.is_none() || self.moved_kind.is_none() {
                    self.stage = Puzzle5dProximityConnectStage::Emit;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-emit", "Connecting nearby grip", "Naher Griff wird verbunden"));
                }
                let Some(row) = rows.get(self.compatibility_cursor) else {
                    self.clear_candidate();
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                };
                self.compatibility_cursor += 1;
                let source = row.get("source").and_then(Value::as_str).unwrap_or("");
                let target = row.get("target").and_then(Value::as_str).unwrap_or("");
                let bidirectional = row.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
                let source_kind = self.candidate_kind.as_deref().unwrap_or("");
                let target_kind = self.moved_kind.as_deref().unwrap_or("");
                if (source == source_kind && target == target_kind) || (bidirectional && source == target_kind && target == source_kind) {
                    self.stage = Puzzle5dProximityConnectStage::Emit;
                }
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Emit => {
                if self.mutations.len() >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
                    return Err(Fault::from("puzzle5d-proximity-connect-output-capacity"));
                }
                let id = format!("fastener-{:016x}-{}", self.operation_nonce, self.fresh_cursor);
                self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                let source = self.candidate_id.as_ref().cloned().ok_or_else(|| Fault::from("puzzle5d-proximity-source-owner"))?;
                let target = self.moved_id.as_ref().cloned().ok_or_else(|| Fault::from("puzzle5d-proximity-target-owner"))?;
                let arg = |key: &str| command.args().and_then(|args| args.get(key)).and_then(Value::as_f64).unwrap_or(0.0);
                let kind = command.args().and_then(|args| args.get("fastenerKind").or_else(|| args.get("edgeKind"))).and_then(Value::as_str).filter(|kind| !kind.is_empty()).map(str::to_string);
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(id, source, target, kind, arg("gap"), arg("shift"), arg("rise"), arg("rotation"), arg("turn"), arg("tilt"), arg("x"), arg("y")));
                self.clear_candidate();
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Complete => Err(Fault::from("puzzle5d-proximity-connect-complete-repolled")),
            Puzzle5dProximityConnectStage::Closing => Err(Fault::from("puzzle5d-proximity-connect-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dProximityConnectStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() || self.moved_id.take().is_some() || self.moved_kind.take().is_some() || self.moved_position.take().is_some() || self.candidate_id.take().is_some() || self.candidate_kind.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dProximityConnectStage::Closing && self.mutations.is_empty() && self.moved_id.is_none() && self.moved_kind.is_none() && self.moved_position.is_none() && self.candidate_id.is_none() && self.candidate_kind.is_none()
    }
}

struct Puzzle5dPatchGripWork {
    stage: Puzzle5dPatchPartStage,
    selection_cursor: usize,
    part_cursor: usize,
    grip_cursor: usize,
    processed_grips: usize,
    selected: HashSet<String>,
    mutations: Vec<Puzzle5dMutation>,
}

impl Default for Puzzle5dPatchGripWork {
    fn default() -> Self {
        Self {
            stage: Puzzle5dPatchPartStage::Selection,
            selection_cursor: 0,
            part_cursor: 0,
            grip_cursor: 0,
            processed_grips: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
        }
    }
}

impl Puzzle5dPatchGripWork {
    fn source_len(command: &Puzzle5dCommand) -> usize {
        let args = command.args();
        args.and_then(|args| args.get("gripFullIds")).and_then(Value::as_array).map_or(0, Vec::len) + usize::from(args.and_then(|args| args.get("gripFullId")).and_then(Value::as_str).is_some())
    }

    fn source_id(command: &Puzzle5dCommand, index: usize) -> Option<&str> {
        let args = command.args()?;
        let ids = args.get("gripFullIds").and_then(Value::as_array);
        if let Some(id) = ids.and_then(|ids| ids.get(index)).and_then(Value::as_str) {
            return (!id.is_empty()).then_some(id);
        }
        (index == ids.map_or(0, Vec::len)).then(|| args.get("gripFullId").and_then(Value::as_str)).flatten().filter(|id| !id.is_empty())
    }

    fn patch(command: &Puzzle5dCommand, grip: &mut crate::Puzzle5dGrip) -> bool {
        let Some(args) = command.args() else { return false };
        let field = args.get("field").and_then(Value::as_str).unwrap_or("");
        let value = args.get("value");
        let delta = args.get("delta");
        let text = value.and_then(Value::as_str);
        match field {
            "gripKind" => {
                let Some(text) = text else { return false };
                grip.grip_kind = Some(text.to_string());
                grip.grip_2d.grip_kind = Some(text.to_string());
            }
            "angle" => {
                let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_2d.angle, value, delta) else { return false };
                grip.grip_2d.angle = updated;
            }
            "radius" => {
                let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_3d.radius.unwrap_or(0.0), value, delta) else { return false };
                grip.grip_2d.radius = Some(updated);
                grip.grip_3d.radius = Some(updated);
            }
            "label" => grip.grip_3d.label = text.filter(|text| !text.is_empty()).map(str::to_string),
            _ => {
                if let Some(axis) = puzzle5d_axis_index(field, "position") {
                    let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_3d.position[axis], value, delta) else { return false };
                    grip.grip_3d.position[axis] = updated;
                } else if let Some(axis) = puzzle5d_axis_index(field, "direction") {
                    let mut direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
                    let Some(updated) = puzzle5d_resolve_number_edit(direction[axis], value, delta) else { return false };
                    direction[axis] = updated;
                    grip.grip_3d.direction = Some(direction);
                } else {
                    return false;
                }
            }
        }
        true
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dPatchGripWork {
    fn tool_id(&self) -> &'static str {
        "patchGrip"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let items = Self::source_len(command).checked_add(projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        match self.stage {
            Puzzle5dPatchPartStage::Selection => {
                if let Some(id) = Self::source_id(command, self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-patch-grip-selection-capacity"));
                    }
                    self.selected.insert(id.to_string());
                    self.selection_cursor += 1;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-patch-grip-selection", "Reading grip target", "Griffziel wird gelesen"));
                }
                self.stage = Puzzle5dPatchPartStage::Parts;
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-patch-grip", "Patching grip", "Griff wird geändert"))
            }
            Puzzle5dPatchPartStage::Parts => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                let Some(grip_value) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)).cloned() else {
                    self.part_cursor += 1;
                    self.grip_cursor = 0;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-patch-grip-part", "Advancing grip owner", "Griffinhaber wird gewechselt"));
                };
                if self.processed_grips >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
                    return Err(Fault::from("puzzle5d-patch-grip-work-capacity"));
                }
                self.processed_grips += 1;
                self.grip_cursor += 1;
                let part_id = part.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle5d-patch-grip-part-id-malformed"))?;
                let mut grip: crate::Puzzle5dGrip = <crate::Puzzle5dGrip as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(&grip_value)).map_err(|_| Fault::from("puzzle5d-patch-grip-malformed"))?;
                let full_id = puzzle5d_grip_full_id(part_id, &grip.id);
                if self.selected.contains(&full_id) && Self::patch(command, &mut grip) {
                    let grip_id = grip.id.clone();
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::replace_part_grip(part_id.to_string(), grip_id, grip));
                }
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-patch-grip", "Patching grip", "Griff wird geändert"))
            }
            Puzzle5dPatchPartStage::Complete => Err(Fault::from("puzzle5d-patch-grip-complete-repolled")),
            Puzzle5dPatchPartStage::Closing => Err(Fault::from("puzzle5d-patch-grip-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dPatchPartStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let selected = {
            let mut selected = self.selected.extract_if(|_| true);
            selected.next()
        };
        if selected.is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dPatchPartStage::Closing && self.mutations.is_empty() && self.selected.is_empty()
    }
}

struct Puzzle5dDeleteFastenerWork {
    cursor: usize,
    closing: bool,
    mutations: Vec<Puzzle5dMutation>,
}

impl Default for Puzzle5dDeleteFastenerWork {
    fn default() -> Self {
        Self { cursor: 0, closing: false, mutations: Vec::with_capacity(1) }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dDeleteFastenerWork {
    fn tool_id(&self) -> &'static str {
        "deleteFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        projection.get("fasteners").and_then(Value::as_array).map_or(Some(1), |fasteners| fasteners.len().checked_add(1)).filter(|items| *items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let target = command.args().and_then(|args| args.get("id").or_else(|| args.get("fastenerId"))).and_then(Value::as_str).filter(|id| !id.is_empty());
        let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.cursor)) else {
            return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
        };
        self.cursor += 1;
        if target == row.get("id").and_then(Value::as_str) {
            if let Some(id) = target {
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id.to_string()));
            }
        }
        Ok(Puzzle5dPatchPartWork::progress("puzzle5d-delete-fastener", "Scanning fastener", "Verbindung wird geprüft"))
    }

    fn begin_close(&mut self) {
        self.closing = true;
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
        self.closing && self.mutations.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dAddNodeStage {
    Catalog,
    Grips,
    Complete,
    Closing,
}

struct Puzzle5dAddNodeWork {
    stage: Puzzle5dAddNodeStage,
    catalog_cursor: usize,
    grip_cursor: usize,
    catalog_index: Option<usize>,
    mesh_url: Option<String>,
    grips: Vec<crate::Puzzle5dGrip>,
    mutation: Option<Puzzle5dMutation>,
    operation_nonce: u64,
}

impl Default for Puzzle5dAddNodeWork {
    fn default() -> Self {
        Self { stage: Puzzle5dAddNodeStage::Catalog, catalog_cursor: 0, grip_cursor: 0, catalog_index: None, mesh_url: None, grips: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS), mutation: None, operation_nonce: 0 }
    }
}

impl Puzzle5dAddNodeWork {
    fn part_kind(command: &Puzzle5dCommand) -> &str {
        command.args().and_then(|args| args.get("kind")).and_then(Value::as_str).unwrap_or("Part")
    }

    fn catalogs(snapshot: &Puzzle5dPlaySnapshot) -> Vec<Value> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        projection.get("kindCatalogs").and_then(|catalogs| catalogs.get("parts")).and_then(Value::as_array).cloned().unwrap_or_default()
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dAddNodeWork {
    fn tool_id(&self) -> &'static str {
        "addNode"
    }

    fn bind_operation(&mut self, operation: Operation) {
        self.operation_nonce = operation.operation.0 ^ operation.generation.0.rotate_left(17) ^ operation.seed.rotate_left(31);
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let items = Self::catalogs(snapshot).len().checked_add(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let catalogs = Self::catalogs(snapshot);
        match self.stage {
            Puzzle5dAddNodeStage::Catalog => {
                let Some(entry) = catalogs.get(self.catalog_cursor) else {
                    self.stage = Puzzle5dAddNodeStage::Grips;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-add-node-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                };
                let index = self.catalog_cursor;
                self.catalog_cursor += 1;
                if entry.get("id").and_then(Value::as_str) == Some(Self::part_kind(command)) {
                    self.catalog_index = Some(index);
                    self.mesh_url = entry.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).map(str::to_string);
                    self.stage = Puzzle5dAddNodeStage::Grips;
                }
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-add-node-catalog", "Reading part catalog", "Teilekatalog wird gelesen"))
            }
            Puzzle5dAddNodeStage::Grips => {
                let templates = self.catalog_index.and_then(|index| catalogs.get(index)).and_then(|entry| entry.get("grips")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if let Some(template) = templates.get(self.grip_cursor) {
                    if self.grips.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-add-node-grip-capacity"));
                    }
                    let grip_kind = template.get("gripKind").and_then(Value::as_str).unwrap_or("grip").to_string();
                    let grip_2d: crate::Puzzle5dGrip2d = match template.get("2d") {
                        Some(value) => <crate::Puzzle5dGrip2d as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(value)).map_err(|_| Fault::from("puzzle5d-add-node-grip2d-malformed"))?,
                        None => Default::default(),
                    };
                    let grip_3d: crate::Puzzle5dGrip3d = match template.get("3d") {
                        Some(value) => <crate::Puzzle5dGrip3d as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(value)).map_err(|_| Fault::from("puzzle5d-add-node-grip3d-malformed"))?,
                        None => Default::default(),
                    };
                    self.grips.push(crate::Puzzle5dGrip { id: format!("v{}", self.grip_cursor), grip_kind: Some(grip_kind), grip_2d, grip_3d });
                    self.grip_cursor += 1;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-add-node-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                }
                let x = command.args().and_then(|args| args.get("x")).and_then(Value::as_f64).unwrap_or(120.0);
                let y = command.args().and_then(|args| args.get("y")).and_then(Value::as_f64).unwrap_or(120.0);
                let flat_to_world = PUZZLE5D_FLAT_TO_WORLD;
                let origin = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.first()).map_or([x * flat_to_world, -y * flat_to_world, 0.0], |peer| {
                    let peer_2d = peer.get("2d");
                    let peer_3d = peer.get("3d");
                    let peer_x = peer_2d.and_then(|part| part.get("x")).and_then(Value::as_f64).unwrap_or_default();
                    let peer_y = peer_2d.and_then(|part| part.get("y")).and_then(Value::as_f64).unwrap_or_default();
                    let peer_origin = peer_3d.and_then(|part| part.get("origin")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default();
                    [peer_origin[0] + (x - peer_x) * flat_to_world, peer_origin[1] - (y - peer_y) * flat_to_world, peer_origin[2]]
                });
                let part_kind = Self::part_kind(command).to_string();
                let created_id = format!("part-{:016x}-0", self.operation_nonce);
                let part = crate::Puzzle5dPart {
                    id: created_id.clone(),
                    part_kind: Some(part_kind.clone()),
                    anchor: Default::default(),
                    part_2d: crate::Puzzle5dPart2d { x, y, shape: Some("circle".to_string()), radius: Some(PUZZLE5D_DEFAULT_PART_RADIUS), text: Some(part_kind.clone()), ..Default::default() },
                    part_3d: crate::Puzzle5dPart3d { origin, mesh_url: self.mesh_url.take(), orientation: Some([0.0, 0.0, 0.0, 1.0]), label: Some(puzzle5d_next_part_label_from_projection(&projection, &part_kind)), ..Default::default() },
                    grips: std::mem::take(&mut self.grips),
                };
                self.mutation = Some(crate::standards::v1::subsets::any::schema::mutations::create_part(part, None));
                self.stage = Puzzle5dAddNodeStage::Complete;
                // 🕹️ Re-select what this gesture just created — a palette drop that leaves the old selection
                // standing makes every follow-up verb (patch, transform, delete) act on the wrong part.
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit {
                    artifact_mutations: self.mutation.take().into_iter().collect(),
                    interaction_writes: vec![InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, [created_id])],
                    ui_scope: UiDirtyScope::Full,
                    ..Default::default()
                }))
            }
            Puzzle5dAddNodeStage::Complete => Err(Fault::from("puzzle5d-add-node-complete-repolled")),
            Puzzle5dAddNodeStage::Closing => Err(Fault::from("puzzle5d-add-node-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dAddNodeStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutation.take().is_some() || self.grips.pop().is_some() || self.mesh_url.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dAddNodeStage::Closing && self.mutation.is_none() && self.grips.is_empty() && self.mesh_url.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dAddBrushPartStage {
    Catalog,
    Grips,
    Target,
    Create,
    Connect,
    Complete,
    Closing,
}

struct Puzzle5dAddBrushPartWork {
    tool_id: &'static str,
    payload: Option<Value>,
    stage: Puzzle5dAddBrushPartStage,
    catalog_cursor: usize,
    grip_cursor: usize,
    part_cursor: usize,
    target_grip_cursor: usize,
    processed_units: usize,
    catalog_index: Option<usize>,
    mesh_url: Option<String>,
    target_id: Option<String>,
    target_position: Option<[f64; 3]>,
    target_direction: Option<[f64; 3]>,
    created_id: Option<String>,
    created_grip_id: Option<String>,
    grips: Vec<crate::Puzzle5dGrip>,
    mutations: Vec<Puzzle5dMutation>,
    operation_nonce: u64,
    fresh_cursor: u64,
}

impl Puzzle5dAddBrushPartWork {
    fn new(tool_id: &'static str) -> Self {
        Self {
            tool_id,
            payload: None,
            stage: Puzzle5dAddBrushPartStage::Catalog,
            catalog_cursor: 0,
            grip_cursor: 0,
            part_cursor: 0,
            target_grip_cursor: 0,
            processed_units: 0,
            catalog_index: None,
            mesh_url: None,
            target_id: None,
            target_position: None,
            target_direction: None,
            created_id: None,
            created_grip_id: None,
            grips: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            mutations: Vec::with_capacity(2),
            operation_nonce: 0,
            fresh_cursor: 0,
        }
    }

    fn from_board_payload(payload: Value) -> Self {
        let mut work = Self::new("addBrushPart");
        work.payload = Some(payload);
        work
    }

    /// 🌉️ `self.payload` (materialized once via `take_payload`/`parse`) and `command.args()` are
    /// both this file's own first-party `Value` now, so no bridge is needed — an in-progress board
    /// payload wins, falling back to the dispatch args only when there is none.
    fn args(&self, command: &Puzzle5dCommand) -> Option<Value> {
        self.payload.clone().or_else(|| command.args().cloned())
    }

    /// 🗂️ The kind this run adds: the caller's own `partKind`, else THIS tool's own declared select
    /// default — never a literal.
    ///
    /// 🐛️ It used to fall back to the string `"Part"`, the very literal
    /// `PUZZLE5D_SHIPPED_PART_KINDS` replaced in the select, so a bare `addPartKind` (the arg form
    /// dispatched with no args, which is what an agent sends) added a part of a kind no catalog
    /// declares. Measured 2026-09-22 (slice PZ2): `add_part_kind_materializes_the_declared_kind_default`
    /// read back `"Part"` where the declared default is `"Hexagonal Cut Concrete Forest Left"`.
    /// `addBrushPart` declares a one-row `"Part"` select of its own, so reading each tool's OWN
    /// declared default keeps that verb byte-identical while fixing this one.
    fn owned_part_kind(&self, command: &Puzzle5dCommand) -> String {
        self.args(command)
            .and_then(|args| args.get("partKind").or_else(|| args.get("objectKindId")).or_else(|| args.get("nodeKind")).and_then(Value::as_str).map(str::to_string))
            .filter(|kind| !kind.is_empty())
            .unwrap_or_else(|| if self.tool_id == "addPartKind" { puzzle5d_default_part_kind(&puzzle5d_part_kind_options()) } else { "Part".to_string() })
    }

    fn catalogs(snapshot: &Puzzle5dPlaySnapshot) -> Vec<Value> {
        Puzzle5dAddNodeWork::catalogs(snapshot)
    }

    fn target(&self, command: &Puzzle5dCommand, interaction: &protocol::InteractionState) -> Option<String> {
        self.args(command)
            .and_then(|args| args.get("targetVortexFullId").or_else(|| args.get("targetGripFullId")).and_then(Value::as_str).map(str::to_string))
            .filter(|id| !id.is_empty())
            .or_else(|| interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE5D_GRANULARITY_GRIP).and_then(|selection| selection.ids.first().cloned()))
    }

    fn world_direction(part: &Value, grip: &Value) -> [f64; 3] {
        let orientation = part.get("3d").and_then(|part| part.get("orientation")).and_then(puzzle5d_value_as_f64_4).unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let direction = grip.get("3d").and_then(|grip| grip.get("direction")).and_then(puzzle5d_value_as_f64_3).unwrap_or([0.0, 0.0, -1.0]);
        quat_rotate_vector(orientation, direction)
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dAddBrushPartWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn bind_operation(&mut self, operation: Operation) {
        self.operation_nonce = operation.operation.0 ^ operation.generation.0.rotate_left(17) ^ operation.seed.rotate_left(31);
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let items = Self::catalogs(snapshot).len().checked_add(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS)?.checked_add(projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?.checked_add(2)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-add-brush-part-work-capacity"));
        }
        self.processed_units += 1;
        let catalogs = Self::catalogs(snapshot);
        match self.stage {
            Puzzle5dAddBrushPartStage::Catalog => {
                let Some(entry) = catalogs.get(self.catalog_cursor) else {
                    self.stage = Puzzle5dAddBrushPartStage::Grips;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-brush-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                };
                let index = self.catalog_cursor;
                self.catalog_cursor += 1;
                if entry.get("id").and_then(Value::as_str) == Some(self.owned_part_kind(command).as_str()) {
                    self.catalog_index = Some(index);
                    self.mesh_url = entry.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).map(str::to_string);
                    self.stage = Puzzle5dAddBrushPartStage::Grips;
                }
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-brush-catalog", "Reading part catalog", "Teilekatalog wird gelesen"))
            }
            Puzzle5dAddBrushPartStage::Grips => {
                let templates = self.catalog_index.and_then(|index| catalogs.get(index)).and_then(|entry| entry.get("grips")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if let Some(template) = templates.get(self.grip_cursor) {
                    if self.grips.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-add-brush-part-grip-capacity"));
                    }
                    let grip_kind = template.get("gripKind").and_then(Value::as_str).unwrap_or("grip").to_string();
                    let grip_2d = template.get("2d").map(|value| <crate::Puzzle5dGrip2d as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(value))).transpose().map_err(|_| Fault::from("puzzle5d-add-brush-part-grip2d-malformed"))?.unwrap_or_default();
                    let grip_3d = template.get("3d").map(|value| <crate::Puzzle5dGrip3d as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(value))).transpose().map_err(|_| Fault::from("puzzle5d-add-brush-part-grip3d-malformed"))?.unwrap_or_default();
                    let id = format!("v{}", self.grip_cursor);
                    if self.created_grip_id.is_none() {
                        self.created_grip_id = Some(id.clone());
                    }
                    self.grips.push(crate::Puzzle5dGrip { id, grip_kind: Some(grip_kind), grip_2d, grip_3d });
                    self.grip_cursor += 1;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-brush-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                }
                self.target_id = self.target(command, interaction);
                self.stage = if self.target_id.is_some() { Puzzle5dAddBrushPartStage::Target } else { Puzzle5dAddBrushPartStage::Create };
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht"))
            }
            Puzzle5dAddBrushPartStage::Target => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.target_id = None;
                    self.stage = Puzzle5dAddBrushPartStage::Create;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-brush-create", "Creating brush part", "Pinselteil wird erstellt"));
                };
                let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.target_grip_cursor)) else {
                    self.part_cursor += 1;
                    self.target_grip_cursor = 0;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht"));
                };
                self.target_grip_cursor += 1;
                let Some(part_id) = part.get("id").and_then(Value::as_str) else { return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht")) };
                let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht")) };
                if self.target_id.as_deref() == Some(puzzle5d_grip_full_id(part_id, grip_id).as_str()) {
                    self.target_position = Some(Puzzle5dProximityConnectWork::world_position(part, grip));
                    self.target_direction = Some(Self::world_direction(part, grip));
                    self.stage = Puzzle5dAddBrushPartStage::Create;
                }
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht"))
            }
            Puzzle5dAddBrushPartStage::Create => {
                let x = self.args(command).and_then(|args| args.get("x").and_then(Value::as_f64)).unwrap_or(120.0);
                let y = self.args(command).and_then(|args| args.get("y").and_then(Value::as_f64)).unwrap_or(120.0);
                let origin = match (self.target_position, self.target_direction) {
                    (Some(position), Some(direction)) => [position[0] + direction[0], position[1] + direction[1], position[2] + direction[2]],
                    _ => [x * PUZZLE5D_FLAT_TO_WORLD, -y * PUZZLE5D_FLAT_TO_WORLD, 0.0],
                };
                let part_kind = self.owned_part_kind(command);
                let id = self.args(command).and_then(|args| args.get("nodeId").or_else(|| args.get("partId")).or_else(|| args.get("objectId")).and_then(Value::as_str).map(str::to_string)).filter(|id| !id.is_empty()).unwrap_or_else(|| {
                    let id = format!("part-{:016x}-{}", self.operation_nonce, self.fresh_cursor);
                    self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                    id
                });
                let part = crate::Puzzle5dPart {
                    id: id.clone(),
                    part_kind: Some(part_kind.clone()),
                    anchor: Default::default(),
                    part_2d: crate::Puzzle5dPart2d { x, y, shape: Some("circle".to_string()), radius: Some(PUZZLE5D_DEFAULT_PART_RADIUS), text: Some(part_kind.clone()), ..Default::default() },
                    part_3d: crate::Puzzle5dPart3d { origin, mesh_url: self.mesh_url.take(), orientation: Some([0.0, 0.0, 0.0, 1.0]), label: Some(puzzle5d_next_part_label_from_projection(&projection, &part_kind)), ..Default::default() },
                    grips: std::mem::take(&mut self.grips),
                };
                self.created_id = Some(id);
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::create_part(part, None));
                self.stage = Puzzle5dAddBrushPartStage::Connect;
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-brush-create", "Creating brush part", "Pinselteil wird erstellt"))
            }
            Puzzle5dAddBrushPartStage::Connect => {
                if let (Some(source), Some(part), Some(grip)) = (self.target_id.as_ref(), self.created_id.as_ref(), self.created_grip_id.as_ref()) {
                    let id = self.args(command).and_then(|args| args.get("edgeId").and_then(Value::as_str).map(str::to_string)).filter(|id| !id.is_empty()).unwrap_or_else(|| {
                        let id = format!("fastener-{:016x}-{}", self.operation_nonce, self.fresh_cursor);
                        self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                        id
                    });
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(id, source.clone(), puzzle5d_grip_full_id(part, grip), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
                }
                self.stage = Puzzle5dAddBrushPartStage::Complete;
                // 🕹️ Re-select the placed part so the brush's next gesture chains off it, exactly as puzzle
                // 3d's `addBrushObject` does.
                let interaction_writes = self.created_id.take().map(|id| InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, [id])).into_iter().collect();
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), interaction_writes, ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dAddBrushPartStage::Complete => Err(Fault::from("puzzle5d-add-brush-part-complete-repolled")),
            Puzzle5dAddBrushPartStage::Closing => Err(Fault::from("puzzle5d-add-brush-part-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dAddBrushPartStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some()
            || self.payload.take().is_some()
            || self.grips.pop().is_some()
            || self.mesh_url.take().is_some()
            || self.target_id.take().is_some()
            || self.target_position.take().is_some()
            || self.target_direction.take().is_some()
            || self.created_id.take().is_some()
            || self.created_grip_id.take().is_some()
        {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dAddBrushPartStage::Closing
            && self.mutations.is_empty()
            && self.payload.is_none()
            && self.grips.is_empty()
            && self.mesh_url.is_none()
            && self.target_id.is_none()
            && self.target_position.is_none()
            && self.target_direction.is_none()
            && self.created_id.is_none()
            && self.created_grip_id.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dBoardEventsStage {
    Open,
    Scan,
    Decode,
    Dispatch,
    DragMove,
    FindMovePart,
    ScanEdge,
    ScanDeleteEdges,
    Brush,
    DrainBrush,
    CloseBrush,
    Complete,
    Closing,
}

struct Puzzle5dBoardEventsWork {
    stage: Puzzle5dBoardEventsStage,
    byte_cursor: usize,
    event_start: Option<usize>,
    event_end: usize,
    depth: usize,
    in_string: bool,
    escape: bool,
    event: Option<Value>,
    drag_moves: Option<Value>,
    drag_cursor: usize,
    pending_move_id: Option<String>,
    pending_move_x: Option<f64>,
    pending_move_y: Option<f64>,
    part_cursor: usize,
    pending_source: Option<String>,
    pending_target: Option<String>,
    pending_edge_id: Option<String>,
    pending_edge_kind: Option<String>,
    fastener_cursor: usize,
    pending_delete_id: Option<String>,
    brush: Option<Puzzle5dAddBrushPartWork>,
    brush_first: Option<Puzzle5dMutation>,
    brush_second: Option<Puzzle5dMutation>,
    camera2d: Option<Puzzle5dCamera2d>,
    mutations: Vec<Puzzle5dMutation>,
    operation_nonce: u64,
    fresh_cursor: u64,
    /// 🕹️ The LAST `select` row of the batch is the engine's whole selection set (replace semantics), so
    /// later rows supersede earlier ones inside one batch.
    select_ids: Option<Vec<String>>,
    /// 🧹️ Ids this batch removed from the document — a delete must not leave a dangling selection, so each
    /// one is subtracted from the live selection through the sanctioned reducer channel.
    removed_ids: Vec<String>,
    /// 🖌️ The part a `brushPlace` row created, re-selected so the next brush gesture chains off it.
    placed_id: Option<String>,
    locked_refused: bool,
    view_state: Option<semio_framework_plugin::ViewModel>,
    window_config: Option<semio_framework_plugin::WindowConfigSnapshot>,
}

impl Default for Puzzle5dBoardEventsWork {
    fn default() -> Self {
        Self {
            stage: Puzzle5dBoardEventsStage::Open,
            byte_cursor: 0,
            event_start: None,
            event_end: 0,
            depth: 0,
            in_string: false,
            escape: false,
            event: None,
            drag_moves: None,
            drag_cursor: 0,
            pending_move_id: None,
            pending_move_x: None,
            pending_move_y: None,
            part_cursor: 0,
            pending_source: None,
            pending_target: None,
            pending_edge_id: None,
            pending_edge_kind: None,
            fastener_cursor: 0,
            pending_delete_id: None,
            brush: None,
            brush_first: None,
            brush_second: None,
            camera2d: None,
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            operation_nonce: 0,
            fresh_cursor: 0,
            select_ids: None,
            removed_ids: Vec::new(),
            placed_id: None,
            locked_refused: false,
            view_state: None,
            window_config: None,
        }
    }
}

impl Puzzle5dBoardEventsWork {
    fn source(command: &Puzzle5dCommand) -> Result<&str, Fault> {
        command.args().and_then(|args| args.get("eventsJson")).and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle5d-board-events-input-missing"))
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn push(&mut self, mutation: Puzzle5dMutation) -> Result<(), Fault> {
        if self.mutations.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
            return Err(Fault::from("puzzle5d-board-events-mutation-capacity"));
        }
        self.mutations.push(mutation);
        Ok(())
    }

    fn next_event(&mut self) {
        self.event = None;
        self.event_start = None;
        self.event_end = 0;
        self.depth = 0;
        self.in_string = false;
        self.escape = false;
        self.stage = Puzzle5dBoardEventsStage::Scan;
    }

    fn scan_one(&mut self, source: &str) -> Result<(), Fault> {
        let bytes = source.as_bytes();
        let Some(byte) = bytes.get(self.byte_cursor).copied() else {
            return Err(Fault::from("puzzle5d-board-events-array-unterminated"));
        };
        match self.stage {
            Puzzle5dBoardEventsStage::Open => {
                self.byte_cursor += 1;
                if byte.is_ascii_whitespace() {
                    return Ok(());
                }
                if byte != b'[' {
                    return Err(Fault::from("puzzle5d-board-events-array-malformed"));
                }
                self.stage = Puzzle5dBoardEventsStage::Scan;
            }
            Puzzle5dBoardEventsStage::Scan if self.event_start.is_none() => {
                self.byte_cursor += 1;
                if byte.is_ascii_whitespace() || byte == b',' {
                    return Ok(());
                }
                if byte == b']' {
                    self.stage = Puzzle5dBoardEventsStage::Complete;
                    return Ok(());
                }
                if byte != b'{' {
                    return Err(Fault::from("puzzle5d-board-events-event-malformed"));
                }
                self.event_start = Some(self.byte_cursor - 1);
                self.depth = 1;
            }
            Puzzle5dBoardEventsStage::Scan => {
                self.byte_cursor += 1;
                if self.in_string {
                    if self.escape {
                        self.escape = false;
                    } else if byte == b'\\' {
                        self.escape = true;
                    } else if byte == b'"' {
                        self.in_string = false;
                    }
                    return Ok(());
                }
                if byte == b'"' {
                    self.in_string = true;
                } else if byte == b'{' || byte == b'[' {
                    self.depth = self.depth.checked_add(1).ok_or_else(|| Fault::from("puzzle5d-board-events-depth-capacity"))?;
                } else if byte == b'}' || byte == b']' {
                    self.depth = self.depth.checked_sub(1).ok_or_else(|| Fault::from("puzzle5d-board-events-depth-malformed"))?;
                    if self.depth == 0 {
                        self.event_end = self.byte_cursor;
                        self.stage = Puzzle5dBoardEventsStage::Decode;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn take_payload(&mut self) -> Value {
        self.event.as_mut().and_then(Value::as_object_mut).and_then(|event| event.get_mut("payload")).map_or(Value::Null, |value| std::mem::replace(value, Value::Null))
    }

    fn schedule_move(&mut self, payload: &Value) {
        self.pending_move_id = payload.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
        self.pending_move_x = payload.get("x").and_then(Value::as_f64);
        self.pending_move_y = payload.get("y").and_then(Value::as_f64);
        self.part_cursor = 0;
        self.stage = Puzzle5dBoardEventsStage::FindMovePart;
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dBoardEventsWork {
    fn tool_id(&self) -> &'static str {
        "applyBoardEvents"
    }

    fn bind_operation(&mut self, operation: Operation) {
        self.operation_nonce = operation.operation.0 ^ operation.generation.0.rotate_left(17) ^ operation.seed.rotate_left(31);
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }

    fn bind_window_owners(&mut self, config: Option<semio_framework_plugin::WindowConfigSnapshot>, _transient: Option<semio_framework_plugin::WindowTransientSnapshot>) {
        self.window_config = config;
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let bytes = Self::source(command).ok()?.len();
        let document_items = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len).checked_add(projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len))?.checked_add(2)?;
        let items = bytes.checked_mul(document_items)?;
        (bytes <= crate::retained_command::PUZZLE_COMMAND_RAW_BYTES && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items.max(1))
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        config: &Puzzle5dConfig,
        interaction: &protocol::InteractionState,
        hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let source = Self::source(command)?;
        match self.stage {
            Puzzle5dBoardEventsStage::Open | Puzzle5dBoardEventsStage::Scan => {
                self.scan_one(source)?;
                Ok(Self::progress("puzzle5d-board-event-scan", "Reading board event", "Board-Ereignis wird gelesen"))
            }
            Puzzle5dBoardEventsStage::Decode => {
                let start = self.event_start.ok_or_else(|| Fault::from("puzzle5d-board-events-event-owner-missing"))?;
                self.event = Some(parse(source.get(start..self.event_end).ok_or_else(|| Fault::from("puzzle5d-board-events-event-range"))?).map_err(|_| Fault::from("puzzle5d-board-events-event-malformed"))?);
                self.stage = Puzzle5dBoardEventsStage::Dispatch;
                Ok(Self::progress("puzzle5d-board-event-decode", "Decoding board event", "Board-Ereignis wird dekodiert"))
            }
            Puzzle5dBoardEventsStage::Dispatch => {
                let name = self.event.as_ref().and_then(|event| event.get("name")).and_then(Value::as_str).map(str::to_string);
                let payload = self.take_payload();
                match name.as_deref() {
                    Some("camera") => {
                        self.camera2d = Some(<Puzzle5dCamera2d as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(&payload)).map_err(|_| Fault::from("puzzle5d-board-events-camera-malformed"))?);
                        self.next_event();
                    }
                    Some("nodeMove") => self.schedule_move(&payload),
                    Some("nodeDragEnd") => {
                        let mut payload = payload;
                        self.drag_moves = Some(payload.as_object_mut().and_then(|payload| payload.get_mut("moves")).map_or(Value::Array(Vec::new()), |value| std::mem::replace(value, Value::Null)));
                        self.drag_cursor = 0;
                        self.stage = Puzzle5dBoardEventsStage::DragMove;
                    }
                    Some("edgeCreate") => {
                        self.pending_source = payload.get("source").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                        self.pending_target = payload.get("target").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                        self.pending_edge_id = payload.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                        if self.pending_edge_id.is_none() {
                            self.pending_edge_id = Some(format!("fastener-{:016x}-{}", self.operation_nonce, self.fresh_cursor));
                            self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                        }
                        self.pending_edge_kind = payload.get("edgeKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).map(str::to_string);
                        self.fastener_cursor = 0;
                        self.stage = Puzzle5dBoardEventsStage::ScanEdge;
                    }
                    Some("edgeDelete") => {
                        if let Some(id) = payload.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()) {
                            self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id.to_string()))?;
                            self.removed_ids.push(id.to_string());
                        }
                        self.next_event();
                    }
                    Some("select") => {
                        self.select_ids = Some(payload.get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default());
                        self.next_event();
                    }
                    Some("nodeDelete") => {
                        self.pending_delete_id = payload.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                        self.fastener_cursor = 0;
                        self.stage = Puzzle5dBoardEventsStage::ScanDeleteEdges;
                    }
                    Some("brushPlace") => {
                        let mut brush = Puzzle5dAddBrushPartWork::from_board_payload(payload);
                        brush.operation_nonce = self.operation_nonce;
                        brush.fresh_cursor = self.fresh_cursor;
                        self.fresh_cursor = self.fresh_cursor.saturating_add(2);
                        self.brush = Some(brush);
                        self.stage = Puzzle5dBoardEventsStage::Brush;
                    }
                    _ => self.next_event(),
                }
                Ok(Self::progress("puzzle5d-board-event-dispatch", "Applying board event", "Board-Ereignis wird angewendet"))
            }
            Puzzle5dBoardEventsStage::DragMove => {
                let Some(move_payload) = self.drag_moves.as_ref().and_then(Value::as_array).and_then(|moves| moves.get(self.drag_cursor)).cloned() else {
                    self.drag_moves = None;
                    self.next_event();
                    return Ok(Self::progress("puzzle5d-board-event-scan", "Reading board event", "Board-Ereignis wird gelesen"));
                };
                self.drag_cursor += 1;
                self.schedule_move(&move_payload);
                Ok(Self::progress("puzzle5d-board-drag", "Moving board node", "Board-Knoten wird verschoben"))
            }
            Puzzle5dBoardEventsStage::FindMovePart => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.pending_move_id = None;
                    self.stage = if self.drag_moves.is_some() { Puzzle5dBoardEventsStage::DragMove } else { Puzzle5dBoardEventsStage::Scan };
                    return Ok(Self::progress("puzzle5d-board-move", "Finding board node", "Board-Knoten wird gesucht"));
                };
                self.part_cursor += 1;
                if part.get("id").and_then(Value::as_str) == self.pending_move_id.as_deref() {
                    let current = part.get("2d");
                    // 🔒️ A locked part refuses the drag. The board drag moves ONLY the flat pose — the world
                    // origin stays where the 3d pane put it, which is what makes a board drag a plan edit.
                    if current.and_then(|value| value.get("locked")).and_then(Value::as_bool).unwrap_or(false) {
                        self.locked_refused = true;
                        self.pending_move_id = None;
                        self.stage = if self.drag_moves.is_some() { Puzzle5dBoardEventsStage::DragMove } else { Puzzle5dBoardEventsStage::Scan };
                        return Ok(Self::progress("puzzle5d-board-move", "Skipping locked node", "Gesperrter Knoten wird übersprungen"));
                    }
                    let x = self.pending_move_x.unwrap_or_else(|| current.and_then(|value| value.get("x")).and_then(Value::as_f64).unwrap_or_default());
                    let y = self.pending_move_y.unwrap_or_else(|| current.and_then(|value| value.get("y")).and_then(Value::as_f64).unwrap_or_default());
                    let id = self.pending_move_id.take().expect("matched move id");
                    self.push(crate::standards::v1::subsets::any::schema::mutations::move_part_2d(id, x, y))?;
                    self.stage = if self.drag_moves.is_some() { Puzzle5dBoardEventsStage::DragMove } else { Puzzle5dBoardEventsStage::Scan };
                }
                Ok(Self::progress("puzzle5d-board-move", "Finding board node", "Board-Knoten wird gesucht"))
            }
            Puzzle5dBoardEventsStage::ScanEdge => {
                if self.pending_source.is_none() || self.pending_target.is_none() {
                    self.next_event();
                    return Ok(Self::progress("puzzle5d-board-edge", "Checking board edge", "Board-Kante wird geprüft"));
                }
                if let Some(fastener) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let source = fastener.get("source").and_then(Value::as_str);
                    let target = fastener.get("target").and_then(Value::as_str);
                    if (source == self.pending_source.as_deref() && target == self.pending_target.as_deref()) || (source == self.pending_target.as_deref() && target == self.pending_source.as_deref()) {
                        self.pending_source = None;
                        self.pending_target = None;
                    }
                    return Ok(Self::progress("puzzle5d-board-edge", "Checking board edge", "Board-Kante wird geprüft"));
                }
                let id = self.pending_edge_id.take().expect("preflighted edge id");
                let source = self.pending_source.take().expect("preflighted edge source");
                let target = self.pending_target.take().expect("preflighted edge target");
                let kind = self.pending_edge_kind.take();
                self.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(id, source, target, kind, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0))?;
                self.next_event();
                Ok(Self::progress("puzzle5d-board-edge", "Creating board edge", "Board-Kante wird erstellt"))
            }
            Puzzle5dBoardEventsStage::ScanDeleteEdges => {
                let Some(id) = self.pending_delete_id.as_deref() else {
                    self.next_event();
                    return Ok(Self::progress("puzzle5d-board-delete", "Deleting board node", "Board-Knoten wird gelöscht"));
                };
                if let Some(fastener) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let incident = fastener.get("source").and_then(Value::as_str).is_some_and(|grip| grip.split_once(':').is_some_and(|(part_id, _)| part_id == id))
                        || fastener.get("target").and_then(Value::as_str).is_some_and(|grip| grip.split_once(':').is_some_and(|(part_id, _)| part_id == id));
                    if incident {
                        if let Some(fastener_id) = fastener.get("id").and_then(Value::as_str) {
                            self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(fastener_id.to_string()))?;
                        }
                    }
                    return Ok(Self::progress("puzzle5d-board-delete-edge", "Removing attached edge", "Verbundene Kante wird entfernt"));
                }
                let id = self.pending_delete_id.take().expect("preflighted deleted part");
                self.removed_ids.push(id.clone());
                self.push(crate::standards::v1::subsets::any::schema::mutations::delete_part(id))?;
                self.next_event();
                Ok(Self::progress("puzzle5d-board-delete", "Deleting board node", "Board-Knoten wird gelöscht"))
            }
            Puzzle5dBoardEventsStage::Brush => {
                let brush = self.brush.as_mut().ok_or_else(|| Fault::from("puzzle5d-board-brush-owner-missing"))?;
                match <Puzzle5dAddBrushPartWork as crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>>>::step(brush, command, snapshot, config, interaction, hover)? {
                    crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de } => Ok(crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }),
                    crate::retained_command::PuzzleCommandWorkStep::Complete(emit) => {
                        self.placed_id = emit.interaction_writes.iter().flat_map(|write| write.targets.iter()).map(|target| target.id.clone()).next();
                        let mut mutations = emit.artifact_mutations.into_iter();
                        self.brush_first = mutations.next();
                        self.brush_second = mutations.next();
                        if mutations.next().is_some() {
                            return Err(Fault::from("puzzle5d-board-brush-output-capacity"));
                        }
                        self.stage = Puzzle5dBoardEventsStage::DrainBrush;
                        Ok(Self::progress("puzzle5d-board-brush-transfer", "Publishing brush mutation", "Pinselmutation wird veröffentlicht"))
                    }
                    crate::retained_command::PuzzleCommandWorkStep::Download(_) => Err(Fault::from("puzzle5d-board-brush-download-unsupported")),
                }
            }
            Puzzle5dBoardEventsStage::DrainBrush => {
                if let Some(mutation) = self.brush_first.take() {
                    self.push(mutation)?;
                    return Ok(Self::progress("puzzle5d-board-brush-transfer", "Publishing brush mutation", "Pinselmutation wird veröffentlicht"));
                }
                if let Some(mutation) = self.brush_second.take() {
                    self.push(mutation)?;
                    return Ok(Self::progress("puzzle5d-board-brush-transfer", "Publishing brush mutation", "Pinselmutation wird veröffentlicht"));
                }
                if let Some(brush) = self.brush.as_mut() {
                    <Puzzle5dAddBrushPartWork as crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>>>::begin_close(brush);
                }
                self.stage = Puzzle5dBoardEventsStage::CloseBrush;
                Ok(Self::progress("puzzle5d-board-brush-close", "Releasing brush owners", "Pinseleigentümer werden freigegeben"))
            }
            Puzzle5dBoardEventsStage::CloseBrush => {
                let Some(brush) = self.brush.as_mut() else {
                    self.next_event();
                    return Ok(Self::progress("puzzle5d-board-event-scan", "Reading board event", "Board-Ereignis wird gelesen"));
                };
                let step = <Puzzle5dAddBrushPartWork as crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>>>::close_step(brush, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                if matches!(step, semio_framework_job::InteractiveJobCloseStep::Complete) && <Puzzle5dAddBrushPartWork as crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>>>::terminal_is_empty(brush) {
                    self.brush.take();
                    self.next_event();
                }
                Ok(Self::progress("puzzle5d-board-brush-close", "Releasing brush owners", "Pinseleigentümer werden freigegeben"))
            }
            Puzzle5dBoardEventsStage::Complete => {
                self.stage = Puzzle5dBoardEventsStage::Closing;
                // 🪟️ A board `camera` row is the pane's own persisted pose. Dropping it (which this work did
                // until 5A2) made pan/zoom in the board window revert on every refresh.
                let camera2d = self.camera2d.take();
                let window_config_mutations = match (camera2d, self.view_state.as_ref()) {
                    (Some(camera2d), Some(view)) => {
                        let mut next = window_ownership::config_from_snapshot(self.window_config.as_ref());
                        next.camera2d = camera2d;
                        vec![window_ownership::addressed_config(view, next)?]
                    }
                    _ => Vec::new(),
                };
                let mut interaction_writes = Vec::new();
                match (self.select_ids.take(), self.placed_id.take()) {
                    (Some(ids), _) => interaction_writes.push(InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, ids)),
                    (None, Some(id)) => interaction_writes.push(InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, [id])),
                    (None, None) => {}
                }
                let removed = std::mem::take(&mut self.removed_ids);
                if !removed.is_empty() {
                    // 🧹️ A delete clears what it removed through a SUBTRACTIVE write naming exactly those ids;
                    // an empty `Replace` is a silent no-op in `protocol::next_selection`.
                    let targets: Vec<InteractionTarget> = removed
                        .into_iter()
                        .flat_map(|id| [InteractionTarget { granularity: PUZZLE5D_GRANULARITY_PART.to_string(), id: id.clone() }, InteractionTarget { granularity: PUZZLE5D_GRANULARITY_FASTENER.to_string(), id }])
                        .collect();
                    interaction_writes.push(InteractionWrite { domain: PUZZLE5D_INTERACTION_DOMAIN.into(), targets, merge: MergeMode::Subtractive });
                }
                let effects = if self.locked_refused {
                    self.locked_refused = false;
                    puzzle5d_notice_emit(self.view_state.as_ref(), |labels| labels.selection_locked.as_str()).effects
                } else {
                    Vec::new()
                };
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit {
                    artifact_mutations: std::mem::take(&mut self.mutations),
                    window_config_mutations,
                    interaction_writes,
                    effects,
                    ui_scope: UiDirtyScope::Full,
                    ..Default::default()
                }))
            }
            Puzzle5dBoardEventsStage::Closing => Err(Fault::from("puzzle5d-board-events-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dBoardEventsStage::Closing;
        if let Some(brush) = self.brush.as_mut() {
            <Puzzle5dAddBrushPartWork as crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>>>::begin_close(brush);
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(brush) = self.brush.as_mut() {
            let step = <Puzzle5dAddBrushPartWork as crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>>>::close_step(brush, maximum_items.min(1), maximum_bytes);
            if matches!(step, semio_framework_job::InteractiveJobCloseStep::Complete) && <Puzzle5dAddBrushPartWork as crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>>>::terminal_is_empty(brush) {
                self.brush.take();
            }
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.mutations.pop().is_some()
            || self.brush_first.take().is_some()
            || self.brush_second.take().is_some()
            || self.event.take().is_some()
            || self.drag_moves.take().is_some()
            || self.pending_move_id.take().is_some()
            || self.pending_source.take().is_some()
            || self.pending_target.take().is_some()
            || self.pending_edge_id.take().is_some()
            || self.pending_edge_kind.take().is_some()
            || self.pending_delete_id.take().is_some()
            || self.camera2d.take().is_some()
            || self.select_ids.take().is_some()
            || self.removed_ids.pop().is_some()
            || self.placed_id.take().is_some()
            || self.view_state.take().is_some()
            || self.window_config.take().is_some()
        {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dBoardEventsStage::Closing
            && self.brush.is_none()
            && self.mutations.is_empty()
            && self.brush_first.is_none()
            && self.brush_second.is_none()
            && self.event.is_none()
            && self.drag_moves.is_none()
            && self.pending_move_id.is_none()
            && self.pending_source.is_none()
            && self.pending_target.is_none()
            && self.pending_edge_id.is_none()
            && self.pending_edge_kind.is_none()
            && self.pending_delete_id.is_none()
            && self.camera2d.is_none()
            && self.select_ids.is_none()
            && self.removed_ids.is_empty()
            && self.placed_id.is_none()
            && self.view_state.is_none()
            && self.window_config.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dCreateFastenerStage {
    Source,
    Target,
    Existing,
    Compatibility,
    Emit,
    Complete,
    Closing,
}

enum Puzzle5dGripScan {
    Progress,
    Found(Option<String>),
    Exhausted,
}

struct Puzzle5dCreateFastenerWork {
    stage: Puzzle5dCreateFastenerStage,
    part_cursor: usize,
    grip_cursor: usize,
    fastener_cursor: usize,
    compatibility_cursor: usize,
    processed_units: usize,
    source_kind: Option<String>,
    target_kind: Option<String>,
    mutation: Option<Puzzle5dMutation>,
    operation_nonce: u64,
}

impl Default for Puzzle5dCreateFastenerWork {
    fn default() -> Self {
        Self { stage: Puzzle5dCreateFastenerStage::Source, part_cursor: 0, grip_cursor: 0, fastener_cursor: 0, compatibility_cursor: 0, processed_units: 0, source_kind: None, target_kind: None, mutation: None, operation_nonce: 0 }
    }
}

impl Puzzle5dCreateFastenerWork {
    fn endpoint<'a>(command: &'a Puzzle5dCommand, primary: &str, alias: &str) -> &'a str {
        command.args().and_then(|args| args.get(primary).or_else(|| args.get(alias))).and_then(Value::as_str).filter(|id| !id.is_empty()).unwrap_or("")
    }

    fn scan_grip(&mut self, snapshot: &Puzzle5dPlaySnapshot, target: &str) -> Puzzle5dGripScan {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
            return Puzzle5dGripScan::Exhausted;
        };
        let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
            self.part_cursor += 1;
            self.grip_cursor = 0;
            return Puzzle5dGripScan::Progress;
        };
        self.grip_cursor += 1;
        let Some(part_id) = part.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        if puzzle5d_grip_full_id(part_id, grip_id) != target {
            return Puzzle5dGripScan::Progress;
        }
        let kind = grip.get("gripKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).or_else(|| grip.get("2d").and_then(|value| value.get("gripKind")).and_then(Value::as_str).filter(|kind| !kind.is_empty())).map(str::to_string);
        Puzzle5dGripScan::Found(kind)
    }

    fn complete_empty(&mut self) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        self.stage = Puzzle5dCreateFastenerStage::Complete;
        crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default())
    }

    fn arg_f64(command: &Puzzle5dCommand, key: &str) -> f64 {
        command.args().and_then(|args| args.get(key)).and_then(Value::as_f64).unwrap_or(0.0)
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dCreateFastenerWork {
    fn tool_id(&self) -> &'static str {
        "createFastener"
    }

    fn bind_operation(&mut self, operation: Operation) {
        self.operation_nonce = operation.operation.0 ^ operation.generation.0.rotate_left(17) ^ operation.seed.rotate_left(31);
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
        let items = parts.checked_mul(2)?.checked_add(fasteners)?.checked_add(compatibility)?.checked_add(1)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-create-fastener-work-capacity"));
        }
        self.processed_units += 1;
        let source = Self::endpoint(command, "source", "attracting");
        let target = Self::endpoint(command, "target", "attracted");
        if source.is_empty() || target.is_empty() || source == target {
            return Ok(self.complete_empty());
        }
        match self.stage {
            Puzzle5dCreateFastenerStage::Source => match self.scan_grip(snapshot, source) {
                Puzzle5dGripScan::Progress => Ok(Puzzle5dPatchPartWork::progress("puzzle5d-create-fastener-source", "Finding source grip", "Quellgriff wird gesucht")),
                Puzzle5dGripScan::Found(kind) => {
                    self.source_kind = kind;
                    self.part_cursor = 0;
                    self.grip_cursor = 0;
                    self.stage = Puzzle5dCreateFastenerStage::Target;
                    Ok(Puzzle5dPatchPartWork::progress("puzzle5d-create-fastener-target", "Finding target grip", "Zielgriff wird gesucht"))
                }
                Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
            },
            Puzzle5dCreateFastenerStage::Target => match self.scan_grip(snapshot, target) {
                Puzzle5dGripScan::Progress => Ok(Puzzle5dPatchPartWork::progress("puzzle5d-create-fastener-target", "Finding target grip", "Zielgriff wird gesucht")),
                Puzzle5dGripScan::Found(kind) => {
                    self.target_kind = kind;
                    self.stage = Puzzle5dCreateFastenerStage::Existing;
                    Ok(Puzzle5dPatchPartWork::progress("puzzle5d-create-fastener-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"))
                }
                Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
            },
            Puzzle5dCreateFastenerStage::Existing => {
                if let Some(fastener) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let existing_source = fastener.get("source").and_then(Value::as_str).unwrap_or("");
                    let existing_target = fastener.get("target").and_then(Value::as_str).unwrap_or("");
                    if (existing_source == source && existing_target == target) || (existing_source == target && existing_target == source) {
                        return Ok(self.complete_empty());
                    }
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-create-fastener-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"));
                }
                self.stage = Puzzle5dCreateFastenerStage::Compatibility;
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-create-fastener-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dCreateFastenerStage::Compatibility => {
                if self.source_kind.is_none() || self.target_kind.is_none() {
                    self.stage = Puzzle5dCreateFastenerStage::Emit;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-create-fastener-emit", "Creating fastener", "Verbindung wird erstellt"));
                }
                let rows = projection.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if rows.is_empty() {
                    self.stage = Puzzle5dCreateFastenerStage::Emit;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-create-fastener-emit", "Creating fastener", "Verbindung wird erstellt"));
                }
                let Some(row) = rows.get(self.compatibility_cursor) else { return Ok(self.complete_empty()) };
                self.compatibility_cursor += 1;
                let row_source = row.get("source").and_then(Value::as_str).unwrap_or("");
                let row_target = row.get("target").and_then(Value::as_str).unwrap_or("");
                let bidirectional = row.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
                let source_kind = self.source_kind.as_deref().unwrap_or("");
                let target_kind = self.target_kind.as_deref().unwrap_or("");
                if (row_source == source_kind && row_target == target_kind) || (bidirectional && row_source == target_kind && row_target == source_kind) {
                    self.stage = Puzzle5dCreateFastenerStage::Emit;
                }
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-create-fastener-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dCreateFastenerStage::Emit => {
                let id = command.args().and_then(|args| args.get("id").or_else(|| args.get("fastenerId"))).and_then(Value::as_str).filter(|id| !id.is_empty()).map_or_else(|| format!("fastener-{:016x}-0", self.operation_nonce), str::to_string);
                let fastener_kind = command.args().and_then(|args| args.get("fastenerKind").or_else(|| args.get("edgeKind"))).and_then(Value::as_str).filter(|kind| !kind.is_empty()).map(str::to_string);
                self.mutation = Some(crate::standards::v1::subsets::any::schema::mutations::connect_grips(
                    id,
                    source.to_string(),
                    target.to_string(),
                    fastener_kind,
                    Self::arg_f64(command, "gap"),
                    Self::arg_f64(command, "shift"),
                    Self::arg_f64(command, "rise"),
                    Self::arg_f64(command, "rotation"),
                    Self::arg_f64(command, "turn"),
                    Self::arg_f64(command, "tilt"),
                    Self::arg_f64(command, "x"),
                    Self::arg_f64(command, "y"),
                ));
                self.stage = Puzzle5dCreateFastenerStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: self.mutation.take().into_iter().collect(), ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dCreateFastenerStage::Complete => Err(Fault::from("puzzle5d-create-fastener-complete-repolled")),
            Puzzle5dCreateFastenerStage::Closing => Err(Fault::from("puzzle5d-create-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dCreateFastenerStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutation.take().is_some() || self.source_kind.take().is_some() || self.target_kind.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dCreateFastenerStage::Closing && self.mutation.is_none() && self.source_kind.is_none() && self.target_kind.is_none()
    }
}

const PUZZLE5D_RELOCATE_GRIPS_PER_PART: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dWorldRelocateStage {
    SourcePart,
    ExistingFasteners,
    CandidatePart,
    CandidateGrip,
    PublishFastener,
    Complete,
    Closing,
}

struct Puzzle5dWorldRelocateSource {
    part_id: String,
    grip_id: String,
    world_position: [f64; 3],
}

struct Puzzle5dWorldRelocateCandidate {
    grip_id: String,
}

struct Puzzle5dWorldRelocateWork {
    stage: Puzzle5dWorldRelocateStage,
    part_cursor: usize,
    grip_cursor: usize,
    fastener_cursor: usize,
    source: Option<Puzzle5dWorldRelocateSource>,
    candidate_part: Option<Puzzle5dPart>,
    candidate: Option<Puzzle5dWorldRelocateCandidate>,
    existing: HashSet<String>,
    mutations: Vec<Puzzle5dMutation>,
}

impl Default for Puzzle5dWorldRelocateWork {
    fn default() -> Self {
        Self {
            stage: Puzzle5dWorldRelocateStage::SourcePart,
            part_cursor: 0,
            grip_cursor: 0,
            fastener_cursor: 0,
            source: None,
            candidate_part: None,
            candidate: None,
            existing: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
        }
    }
}

impl Puzzle5dWorldRelocateWork {
    fn position(command: &Puzzle5dCommand) -> Option<[f64; 3]> {
        let values = command.args().and_then(|args| args.get("position")).and_then(Value::as_array)?;
        Some([values.first().and_then(Value::as_f64)?, values.get(1).and_then(Value::as_f64)?, values.get(2).and_then(Value::as_f64)?])
    }

    fn edge(first: &str, second: &str) -> String {
        if first <= second {
            format!("{first}\0{second}")
        } else {
            format!("{second}\0{first}")
        }
    }

    fn fastener_id(first: &str, second: &str) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        Self::edge(first, second).hash(&mut hasher);
        format!("puzzle5d.relocate.{:016x}", hasher.finish())
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn complete(&mut self) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        self.stage = Puzzle5dWorldRelocateStage::Complete;
        crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() })
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dWorldRelocateWork {
    fn tool_id(&self) -> &'static str {
        "worldRelocate"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let items = parts.checked_mul(PUZZLE5D_RELOCATE_GRIPS_PER_PART)?.checked_add(parts.checked_mul(2)?)?.checked_add(fasteners)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        match self.stage {
            Puzzle5dWorldRelocateStage::SourcePart => {
                let requested = command.args().and_then(|args| args.get("objectId")).and_then(Value::as_str).unwrap_or("");
                let Some(position) = Self::position(command) else { return Ok(self.complete()) };
                let Some(row) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)).cloned() else { return Ok(self.complete()) };
                self.part_cursor += 1;
                if row.get("id").and_then(Value::as_str) == Some(requested) {
                    let mut part: Puzzle5dPart = serde_json::from_value(serde_json::Value::from(&dsl::os_pack::json::to_dsl_value(&row))).map_err(|_| Fault::from("puzzle5d-world-relocate-source-malformed"))?;
                    if part.grips.len() > PUZZLE5D_RELOCATE_GRIPS_PER_PART {
                        return Err(Fault::from("puzzle5d-world-relocate-grip-capacity"));
                    }
                    part.part_3d.origin = position;
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::move_part_3d(part.id.clone(), position));
                    if let Some(grip) = part.grips.first() {
                        self.source = Some(Puzzle5dWorldRelocateSource { part_id: part.id.clone(), grip_id: puzzle5d_grip_full_id(&part.id, &grip.id), world_position: world_grip_position(&part, grip) });
                    }
                    self.fastener_cursor = 0;
                    self.stage = Puzzle5dWorldRelocateStage::ExistingFasteners;
                }
                Ok(Self::progress("puzzle5d-world-relocate-source", "Finding moved part", "Verschobenes Teil wird gesucht"))
            }
            Puzzle5dWorldRelocateStage::ExistingFasteners => {
                let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|rows| rows.get(self.fastener_cursor)) else {
                    self.part_cursor = 0;
                    self.stage = Puzzle5dWorldRelocateStage::CandidatePart;
                    return Ok(Self::progress("puzzle5d-world-relocate-candidate-part", "Finding nearby part", "Nahes Teil wird gesucht"));
                };
                let source = row.get("source").and_then(Value::as_str).unwrap_or("");
                let target = row.get("target").and_then(Value::as_str).unwrap_or("");
                if self.existing.len() >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
                    return Err(Fault::from("puzzle5d-world-relocate-fastener-capacity"));
                }
                self.existing.insert(Self::edge(source, target));
                self.fastener_cursor += 1;
                Ok(Self::progress("puzzle5d-world-relocate-existing-fastener", "Reading existing fastener", "Bestehende Verbindung wird gelesen"))
            }
            Puzzle5dWorldRelocateStage::CandidatePart => {
                let Some(source) = self.source.as_ref() else { return Ok(self.complete()) };
                let Some(row) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)).cloned() else { return Ok(self.complete()) };
                self.part_cursor += 1;
                if row.get("id").and_then(Value::as_str) == Some(source.part_id.as_str()) {
                    return Ok(Self::progress("puzzle5d-world-relocate-candidate-part", "Skipping moved part", "Verschobenes Teil wird übersprungen"));
                }
                let part: Puzzle5dPart = serde_json::from_value(serde_json::Value::from(&dsl::os_pack::json::to_dsl_value(&row))).map_err(|_| Fault::from("puzzle5d-world-relocate-candidate-malformed"))?;
                if part.grips.len() > PUZZLE5D_RELOCATE_GRIPS_PER_PART {
                    return Err(Fault::from("puzzle5d-world-relocate-grip-capacity"));
                }
                self.candidate_part = Some(part);
                self.grip_cursor = 0;
                self.stage = Puzzle5dWorldRelocateStage::CandidateGrip;
                Ok(Self::progress("puzzle5d-world-relocate-candidate-part", "Scanning nearby part", "Nahes Teil wird geprüft"))
            }
            Puzzle5dWorldRelocateStage::CandidateGrip => {
                let source = self.source.as_ref().ok_or_else(|| Fault::from("puzzle5d-world-relocate-source-owner"))?;
                let part = self.candidate_part.as_ref().ok_or_else(|| Fault::from("puzzle5d-world-relocate-part-owner"))?;
                let Some(grip) = part.grips.get(self.grip_cursor) else {
                    self.candidate_part.take();
                    self.stage = Puzzle5dWorldRelocateStage::CandidatePart;
                    return Ok(Self::progress("puzzle5d-world-relocate-candidate-part", "Advancing nearby part", "Nächstes nahes Teil wird geprüft"));
                };
                self.grip_cursor += 1;
                let grip_id = puzzle5d_grip_full_id(&part.id, &grip.id);
                let edge = Self::edge(&source.grip_id, &grip_id);
                if grip_id == source.grip_id || self.existing.contains(&edge) {
                    return Ok(Self::progress("puzzle5d-world-relocate-candidate-grip", "Skipping connected grip", "Verbundener Griff wird übersprungen"));
                }
                let world = world_grip_position(part, grip);
                let delta = [source.world_position[0] - world[0], source.world_position[1] - world[1], source.world_position[2] - world[2]];
                if (delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt() <= PUZZLE5D_PROXIMITY_RADIUS {
                    self.candidate = Some(Puzzle5dWorldRelocateCandidate { grip_id });
                    self.stage = Puzzle5dWorldRelocateStage::PublishFastener;
                }
                Ok(Self::progress("puzzle5d-world-relocate-candidate-grip", "Measuring nearby grip", "Naher Griff wird gemessen"))
            }
            Puzzle5dWorldRelocateStage::PublishFastener => {
                let source = self.source.as_ref().ok_or_else(|| Fault::from("puzzle5d-world-relocate-source-owner"))?;
                let candidate = self.candidate.take().ok_or_else(|| Fault::from("puzzle5d-world-relocate-candidate-owner"))?;
                let id = Self::fastener_id(&source.grip_id, &candidate.grip_id);
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(id, source.grip_id.clone(), candidate.grip_id.clone(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
                self.existing.insert(Self::edge(&source.grip_id, &candidate.grip_id));
                self.stage = Puzzle5dWorldRelocateStage::CandidateGrip;
                Ok(Self::progress("puzzle5d-world-relocate-publish", "Connecting nearby grip", "Naher Griff wird verbunden"))
            }
            Puzzle5dWorldRelocateStage::Complete => Err(Fault::from("puzzle5d-world-relocate-complete-repolled")),
            Puzzle5dWorldRelocateStage::Closing => Err(Fault::from("puzzle5d-world-relocate-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dWorldRelocateStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() || self.candidate.take().is_some() || self.candidate_part.take().is_some() || self.source.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let edge = {
            let mut existing = self.existing.extract_if(|_| true);
            existing.next()
        };
        if edge.is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dWorldRelocateStage::Closing && self.source.is_none() && self.candidate_part.is_none() && self.candidate.is_none() && self.existing.is_empty() && self.mutations.is_empty()
    }
}

/// 📦️ Rows one `setActiveExample` step replays. A declared unit is a CHUNK, not a row:
/// `🌙️capsule-dream` is 2 880 parts + 2 865 fasteners, so a row-per-unit extent (≈5 749, and ≈11 500 when
/// reloading capsule-dream over itself) overruns the shared `PUZZLE_COMMAND_WORK_ITEMS` band and the
/// switch could only ever refuse. Chunking makes the same replay fit without widening a constant three
/// fixtures pin. It is 8× puzzle 3d's `PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK` on purpose: 3d steps over a
/// TYPED snapshot, while every 5d work re-derives `puzzle5d_projection_value` per step, so a step's cost
/// is dominated by that one O(document) projection rather than by the rows it then replays — fewer,
/// fatter steps is strictly cheaper here, and capsule-dream's switch drops from ≈720 steps to ≈100.
const PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK: usize = 64;

/// 🔢️ Stage transitions and whole-document rows a switch always pays on top of its chunks (label, domain,
/// description, catalogs and one exhaustion step per cursored stage). 3d's own count, kept identical.
const PUZZLE5D_SET_ACTIVE_EXAMPLE_FIXED_STEPS: usize = 13;

/// 🧮️ Mutations one switch may accumulate: exactly what the declared extent can legitimately produce, so
/// `push` can never refuse work `extent` already admitted.
const PUZZLE5D_SET_ACTIVE_EXAMPLE_MUTATIONS: usize = crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS * PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dSetActiveExampleStage {
    ClearFasteners,
    ClearParts,
    Label,
    Domain,
    Description,
    ClearCompatibility,
    AddCompatibility,
    Catalogs,
    AddParts,
    AddFasteners,
    Complete,
    Closing,
}

struct Puzzle5dSetActiveExampleWork {
    stage: Puzzle5dSetActiveExampleStage,
    cursor: usize,
    admitted: bool,
    mutations: Vec<Puzzle5dMutation>,
    view_state: Option<semio_framework_plugin::ViewModel>,
    /// 🗂️ The BEFORE document's ids to clear — fastener ids, part ids and compatibility pairs —
    /// harvested ONCE and then merely indexed by the cursored clearing stages.
    ///
    /// 🐛️ `step` used to call `puzzle5d_projection_value(&snapshot.0)` on entry, i.e. re-derive the
    /// whole document (`serde_json::Value` → `DslValue` → os-pack `Value`) on each of the ~110 chunk
    /// steps one example switch takes, and then re-walk its arrays. The run's snapshot is an `Arc`
    /// the retained driver holds FIXED for the whole run (nothing publishes before `Complete`), so
    /// every one of those derivations produced the same bytes — and leaving `🌙️capsule-dream`
    /// (2 880 parts, ~3.5 MB of JSON) paid that whole projection ~110 times for one switch.
    before: Option<std::sync::Arc<Puzzle5dSetActiveExampleBefore>>,
}

/// 🗂️ Everything the clearing stages need from the BEFORE document, harvested in one pass.
#[derive(Default)]
struct Puzzle5dSetActiveExampleBefore {
    fastener_ids: Vec<String>,
    part_ids: Vec<String>,
    compatibility: Vec<(String, String)>,
}

impl Default for Puzzle5dSetActiveExampleWork {
    fn default() -> Self {
        Self { stage: Puzzle5dSetActiveExampleStage::ClearFasteners, cursor: 0, admitted: false, mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS), view_state: None, before: None }
    }
}

impl Puzzle5dSetActiveExampleWork {
    /// 📏️ How many semantic units switching to `target` from `snapshot` costs — every old fastener and
    /// part cleared, every old compatibility row dropped, then the target's own compatibility rows, parts
    /// and fasteners, each counted in `PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK`-sized steps, plus the fixed
    /// stage-transition and whole-document rows.
    fn units(command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot) -> Option<usize> {
        let projection = puzzle5d_projection_value(&snapshot.0);
        let target = Self::target(command)?;
        let rows = [
            snapshot.0.get("fasteners").and_then(serde_json::Value::as_array).map_or(0, Vec::len),
            projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len),
            projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len),
            Self::compatibility_rows(target).len(),
            target.parts.len(),
            target.fasteners.len(),
        ];
        rows.into_iter().try_fold(PUZZLE5D_SET_ACTIVE_EXAMPLE_FIXED_STEPS, |units, len| units.checked_add(len.div_ceil(PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK)))
    }

    /// ✂️ The next `PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK` rows of a cursored stage, advancing the cursor.
    fn take_chunk(cursor: &mut usize, len: usize) -> std::ops::Range<usize> {
        if *cursor >= len {
            return *cursor..*cursor;
        }
        let end = cursor.saturating_add(PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK).min(len);
        let range = *cursor..end;
        *cursor = end;
        range
    }

    fn target(command: &Puzzle5dCommand) -> Option<&'static Puzzle5dDocument> {
        let example_id = command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or("");
        match example_id {
            "" => Some(&EMPTY_EXAMPLE_DOCUMENT),
            PUZZLE5D_EXAMPLE_CONCRETE_FOREST | "concrete" => Some(&CONCRETE_FOREST_EXAMPLE_DOCUMENT),
            PUZZLE5D_EXAMPLE_NAKAGIN | "nakagin" => Some(&NAKAGIN_EXAMPLE_DOCUMENT),
            PUZZLE5D_EXAMPLE_CAPSULE_DREAM | "capsule" => Some(&CAPSULE_DREAM_EXAMPLE_DOCUMENT),
            _ => None,
        }
    }

    fn compatibility_rows(document: &Puzzle5dDocument) -> &[serde_json::Value] {
        document.kind_compatibility.as_ref().and_then(serde_json::Value::as_array).map(Vec::as_slice).unwrap_or_default()
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn push(&mut self, mutation: Puzzle5dMutation) -> Result<(), Fault> {
        if self.mutations.len() >= PUZZLE5D_SET_ACTIVE_EXAMPLE_MUTATIONS {
            return Err(Fault::from("puzzle5d-set-active-example-output-capacity"));
        }
        self.mutations.push(mutation);
        Ok(())
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dSetActiveExampleWork {
    fn tool_id(&self) -> &'static str {
        "setActiveExample"
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }

    /// 🧯️ An example whose switch costs more than one edit's fixed work capacity is REFUSED with a notice
    /// on the first step, not admitted and faulted: `🌙️capsule-dream` is 2,880 parts + 2,865 fasteners, so
    /// it exceeds `PUZZLE_COMMAND_WORK_ITEMS` (4,096) on its own, and a bare `None` here would surface as
    /// the framework's opaque "exceeds fixed semantic work capacity" fault instead of a sentence the user
    /// can read. The refusal path therefore declares one unit, and `step` completes with the notice.
    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let units = Self::units(command, snapshot)?;
        Some(if units <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS { units } else { 1 })
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        if self.before.is_none() {
            let projection = puzzle5d_projection_value(&snapshot.0);
            let strings = |rows: Option<&Vec<Value>>, key: &str| -> Vec<String> {
                rows.map(|rows| rows.iter().filter_map(|row| row.get(key)).filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default()
            };
            self.before = Some(std::sync::Arc::new(Puzzle5dSetActiveExampleBefore {
                fastener_ids: strings(projection.get("fasteners").and_then(Value::as_array), "id"),
                part_ids: strings(projection.get("parts").and_then(Value::as_array), "id"),
                compatibility: projection
                    .get("kindCompatibility")
                    .and_then(Value::as_array)
                    .map(|rows| rows.iter().map(|row| (row.get("source").and_then(Value::as_str).unwrap_or("").to_string(), row.get("target").and_then(Value::as_str).unwrap_or("").to_string())).collect())
                    .unwrap_or_default(),
            }));
        }
        if !self.admitted {
            self.admitted = true;
            if Self::target(command).is_none() || Self::units(command, snapshot).is_none_or(|units| units > crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS) {
                self.stage = Puzzle5dSetActiveExampleStage::Complete;
                return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle5d_notice_emit(self.view_state.as_ref(), |labels| labels.example_too_large.as_str())));
            }
        }
        // 🔗️ An `Arc` clone, so the cached rows stay readable while the arms below take `&mut self`.
        let before = self.before.clone().unwrap_or_default();
        let Some(target) = Self::target(command) else {
            self.stage = Puzzle5dSetActiveExampleStage::Complete;
            return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle5d_notice_emit(self.view_state.as_ref(), |labels| labels.example_too_large.as_str())));
        };
        match self.stage {
            Puzzle5dSetActiveExampleStage::ClearFasteners => {
                let range = Self::take_chunk(&mut self.cursor, before.fastener_ids.len());
                if !range.is_empty() {
                    for id in before.fastener_ids[range].to_vec() {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id))?;
                    }
                    return Ok(Self::progress("puzzle5d-example-clear-fastener", "Removing old fastener", "Alte Verbindung wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::ClearParts;
                Ok(Self::progress("puzzle5d-example-clear-part", "Removing old part", "Altes Teil wird entfernt"))
            }
            Puzzle5dSetActiveExampleStage::ClearParts => {
                let range = Self::take_chunk(&mut self.cursor, before.part_ids.len());
                if !range.is_empty() {
                    for id in before.part_ids[range].to_vec() {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::delete_part(id))?;
                    }
                    return Ok(Self::progress("puzzle5d-example-clear-part", "Removing old part", "Altes Teil wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::Label;
                Ok(Self::progress("puzzle5d-example-label", "Updating document label", "Dokumenttitel wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Label => {
                self.push(crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d(target.label.clone()))?;
                self.stage = Puzzle5dSetActiveExampleStage::Domain;
                Ok(Self::progress("puzzle5d-example-domain", "Updating document domain", "Dokumentdomäne wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Domain => {
                self.push(crate::standards::v1::subsets::any::schema::mutations::change_domain(target.domain.clone()))?;
                self.stage = Puzzle5dSetActiveExampleStage::Description;
                Ok(Self::progress("puzzle5d-example-description", "Updating description", "Beschreibung wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Description => {
                let description = target.meta.as_ref().and_then(|meta| meta.get("description")).and_then(serde_json::Value::as_str).unwrap_or("");
                self.push(crate::standards::v1::subsets::any::schema::mutations::change_description(description.to_string()))?;
                self.stage = Puzzle5dSetActiveExampleStage::ClearCompatibility;
                Ok(Self::progress("puzzle5d-example-clear-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"))
            }
            Puzzle5dSetActiveExampleStage::ClearCompatibility => {
                let range = Self::take_chunk(&mut self.cursor, before.compatibility.len());
                if !range.is_empty() {
                    for (source, target) in before.compatibility[range].to_vec() {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility(source, target))?;
                    }
                    return Ok(Self::progress("puzzle5d-example-clear-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::AddCompatibility;
                Ok(Self::progress("puzzle5d-example-add-compatibility", "Adding compatibility", "Kompatibilität wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddCompatibility => {
                let rows = Self::compatibility_rows(target);
                let range = Self::take_chunk(&mut self.cursor, rows.len());
                if !range.is_empty() {
                    for row in &rows[range] {
                        let row: crate::Puzzle5dKindCompatibility = <crate::Puzzle5dKindCompatibility as dsl::FromValue>::from_value(dsl::DslValue::from(row)).map_err(|_| Fault::from("puzzle5d-set-active-example-compatibility-malformed"))?;
                        self.push(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(row.source, row.target, row.bidirectional, row.important, row.specificity))?;
                    }
                    return Ok(Self::progress("puzzle5d-example-add-compatibility", "Adding compatibility", "Kompatibilität wird hinzugefügt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::Catalogs;
                Ok(Self::progress("puzzle5d-example-catalogs", "Updating kind catalogs", "Artenkataloge werden aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Catalogs => {
                let catalogs = target.kind_catalogs.as_ref().map(|catalogs| <crate::Puzzle5dKindCatalogs as dsl::FromValue>::from_value(dsl::DslValue::from(catalogs))).transpose().map_err(|_| Fault::from("puzzle5d-set-active-example-catalogs-malformed"))?;
                self.push(crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs(catalogs))?;
                self.stage = Puzzle5dSetActiveExampleStage::AddParts;
                Ok(Self::progress("puzzle5d-example-add-part", "Adding example part", "Beispielteil wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddParts => {
                let range = Self::take_chunk(&mut self.cursor, target.parts.len());
                if !range.is_empty() {
                    for part in &target.parts[range] {
                        let value = serde_json::to_value(part).map_err(|_| Fault::from("puzzle5d-set-active-example-part-malformed"))?;
                        let part = <crate::Puzzle5dPart as dsl::FromValue>::from_value(dsl::DslValue::from(&value)).map_err(|_| Fault::from("puzzle5d-set-active-example-part-malformed"))?;
                        self.push(crate::standards::v1::subsets::any::schema::mutations::create_part(part, None))?;
                    }
                    return Ok(Self::progress("puzzle5d-example-add-part", "Adding example part", "Beispielteil wird hinzugefügt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::AddFasteners;
                Ok(Self::progress("puzzle5d-example-add-fastener", "Adding example fastener", "Beispielverbindung wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddFasteners => {
                let range = Self::take_chunk(&mut self.cursor, target.fasteners.len());
                if !range.is_empty() {
                    for fastener in &target.fasteners[range] {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(
                            fastener.id.clone(),
                            fastener.source.clone(),
                            fastener.target.clone(),
                            fastener.fastener_kind.clone(),
                            fastener.gap,
                            fastener.shift,
                            fastener.rise,
                            fastener.rotation,
                            fastener.turn,
                            fastener.tilt,
                            fastener.x,
                            fastener.y,
                        ))?;
                    }
                    return Ok(Self::progress("puzzle5d-example-add-fastener", "Adding example fastener", "Beispielverbindung wird hinzugefügt"));
                }
                self.stage = Puzzle5dSetActiveExampleStage::Complete;
                // 🧹️ Every part the old document held is gone, so a surviving selection would address ids that
                // no longer exist; it is cleared with a SUBTRACTIVE write naming exactly what is selected now.
                let selection = _interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN);
                let targets: Vec<InteractionTarget> = selection
                    .map(|selection| selection.ids.iter().map(|id| InteractionTarget { granularity: selection.granularity.clone(), id: id.clone() }).collect())
                    .unwrap_or_default();
                let interaction_writes = if targets.is_empty() { Vec::new() } else { vec![InteractionWrite { domain: PUZZLE5D_INTERACTION_DOMAIN.into(), targets, merge: MergeMode::Subtractive }] };
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit {
                    artifact_mutations: std::mem::take(&mut self.mutations),
                    config_mutations: vec![Puzzle5dConfigMutation::Snapshot { config: Puzzle5dConfig::default() }],
                    interaction_writes,
                    ui_scope: UiDirtyScope::Full,
                    ..Default::default()
                }))
            }
            Puzzle5dSetActiveExampleStage::Complete => Err(Fault::from("puzzle5d-set-active-example-complete-repolled")),
            Puzzle5dSetActiveExampleStage::Closing => Err(Fault::from("puzzle5d-set-active-example-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dSetActiveExampleStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() || self.view_state.take().is_some() || self.before.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dSetActiveExampleStage::Closing && self.mutations.is_empty() && self.view_state.is_none() && self.before.is_none()
    }
}

/// 📏️ The retained command route's own wire-admission band, widened off the shared 8 KiB/512 puzzle
/// default to puzzle 3d's (`PUZZLE3D_IMPORT_RAW_BYTES`/`PUZZLE3D_IMPORT_DECODED_ITEMS`): a Nakagin-sized
/// document's camera/grid/sun publications and the import/fixture routes both carry wire owners that the
/// narrow band rejects before the job ever admits.
const PUZZLE5D_RETAINED_RAW_BYTES: usize = 262_144;
const PUZZLE5D_RETAINED_DECODED_ITEMS: usize = 16_384;

/// 🔢️ Mesh numbers one interactive step of a `registerBrushMesh` page transfers. A page is bounded by the
/// retained wire grant, so a whole stream costs a fixed, small number of steps and no step approaches the
/// lane's own time budget.
const PUZZLE5D_MESH_PAGE_VALUES: usize = 512;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dRegisterBrushMeshStage {
    Positions,
    Indices,
    Derive,
    Complete,
    Closing,
}

/// 📋️ `registerBrushMesh` as real retained Work: the browser's GLB round-trip arrives as two flat number
/// arrays, transferred here one bounded page per step into puzzle 3d's process-wide mesh store (which the
/// brush suggestions and fill runs read). HostOnly — that store is neither document nor config state — but
/// the transfer itself is real, cursored work, not the `Emit::default()` a `BoundedFirstStepCommandWork`
/// would have produced in one unbounded turn.
struct Puzzle5dRegisterBrushMeshWork {
    stage: Puzzle5dRegisterBrushMeshStage,
    position_cursor: usize,
    index_cursor: usize,
    positions: Vec<f32>,
    indices: Vec<u32>,
    view_state: Option<semio_framework_plugin::ViewModel>,
}

impl Default for Puzzle5dRegisterBrushMeshWork {
    fn default() -> Self {
        Self { stage: Puzzle5dRegisterBrushMeshStage::Positions, position_cursor: 0, index_cursor: 0, positions: Vec::new(), indices: Vec::new(), view_state: None }
    }
}

impl Puzzle5dRegisterBrushMeshWork {
    fn array<'a>(command: &'a Puzzle5dCommand, key: &str) -> &'a [Value] {
        command.args().and_then(|args| args.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default()
    }

    fn pages(command: &Puzzle5dCommand, key: &str) -> usize {
        Self::array(command, key).len().div_ceil(PUZZLE5D_MESH_PAGE_VALUES)
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dRegisterBrushMeshWork {
    fn tool_id(&self) -> &'static str {
        "registerBrushMesh"
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }

    fn extent(&self, command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let items = Self::pages(command, "positions").checked_add(Self::pages(command, "indices"))?.checked_add(3)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        _snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        match self.stage {
            Puzzle5dRegisterBrushMeshStage::Positions => {
                let page = Self::array(command, "positions");
                let end = page.len().min(self.position_cursor.saturating_add(PUZZLE5D_MESH_PAGE_VALUES));
                self.positions.extend(page[self.position_cursor..end].iter().filter_map(|value| value.as_f64().map(|number| number as f32)));
                self.position_cursor = end;
                if end >= page.len() {
                    self.stage = Puzzle5dRegisterBrushMeshStage::Indices;
                }
                Ok(crate::retained_command::PuzzleCommandWorkStep::Progress { stage: "puzzle5d-register-brush-mesh-position", en: "Reading mesh positions", de: "Mesh-Positionen werden gelesen" })
            }
            Puzzle5dRegisterBrushMeshStage::Indices => {
                let page = Self::array(command, "indices");
                let end = page.len().min(self.index_cursor.saturating_add(PUZZLE5D_MESH_PAGE_VALUES));
                self.indices.extend(page[self.index_cursor..end].iter().filter_map(|value| value.as_u64().and_then(|number| u32::try_from(number).ok())));
                self.index_cursor = end;
                if end >= page.len() {
                    self.stage = Puzzle5dRegisterBrushMeshStage::Derive;
                }
                Ok(crate::retained_command::PuzzleCommandWorkStep::Progress { stage: "puzzle5d-register-brush-mesh-index", en: "Reading mesh indices", de: "Mesh-Indizes werden gelesen" })
            }
            Puzzle5dRegisterBrushMeshStage::Derive => {
                self.stage = Puzzle5dRegisterBrushMeshStage::Complete;
                let url = command.args().and_then(|args| args.get("url")).and_then(Value::as_str).unwrap_or_default().to_string();
                let positions = std::mem::take(&mut self.positions);
                let indices = std::mem::take(&mut self.indices);
                if !register_brush_mesh::puzzle5d_install_brush_mesh(&url, &positions, &indices) {
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle5d_notice_emit(self.view_state.as_ref(), |labels| labels.brush_reason_pose_unavailable.as_str())));
                }
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { ui_scope: UiDirtyScope::None, ..Default::default() }))
            }
            Puzzle5dRegisterBrushMeshStage::Complete => Err(Fault::from("puzzle5d-register-brush-mesh-complete-repolled")),
            Puzzle5dRegisterBrushMeshStage::Closing => Err(Fault::from("puzzle5d-register-brush-mesh-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dRegisterBrushMeshStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.positions.pop().is_some() || self.indices.pop().is_some() || self.view_state.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dRegisterBrushMeshStage::Closing && self.positions.is_empty() && self.indices.is_empty() && self.view_state.is_none()
    }
}

struct Puzzle5dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
    contract: ToolExecutionContract,
}

impl Puzzle5dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self {
            keys: PUZZLE5D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect(),
            contract: ToolExecutionContract::resumable(PUZZLE5D_RETAINED_RAW_BYTES, PUZZLE5D_RETAINED_DECODED_ITEMS, 1, crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES, crate::retained_command::PUZZLE_COMMAND_STEP_MICROS, 1, 1),
        }
    }
}

impl ToolJobFactory for Puzzle5dRetainedCommandJobFactory {
    type Payload = crate::retained_command::RetainedPuzzleCommandPayload<EditorApp<Puzzle5dPlayApp>>;
    type Job = crate::retained_command::RetainedPuzzleCommandJob<EditorApp<Puzzle5dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PUZZLE5D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        self.contract
    }

    fn create_job(&mut self, operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(crate::retained_command::RetainedPuzzleCommandJob::new(operation, payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        operation: Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > self.contract.max_raw_wire_bytes {
            return Err((ToolJobFactoryError::new("Puzzle 5d retained command rejects an oversized wire owner"), input, checkpoint));
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

impl ArtifactOwnedToolJobFactory for Puzzle5dRetainedCommandJobFactory {
    type Owner = EditorApp<Puzzle5dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PUZZLE5D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE5D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidate", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidateBack", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "duplicateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "engagementAbort", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementControlSelect", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementRepeatLast", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "exportFixture", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importFixture", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "openAddPartDialog", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "openImportFixture", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "selectSameKindSelection", lanes: &[ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "setFillCount", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSelectionFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "targetBrushSuggestions", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "focusSelection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "addBrushPart", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "addPartKind", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "applyBoardEvents", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "createFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "editFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchGrip", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchPart", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "proximityConnect", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "registerBrushMesh", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "retargetFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "worldRelocate", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setCamera2d", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setCamera3d", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridFactor", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridSnapEnabled", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSuggestionOffset", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setBrushPlacementContactTolerance", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setProximityRadius", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setChunkSize", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setPartKindWeight", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setGripKindWeight", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setGridVisible", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridSpacing", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setProjection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setProjectionParam", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGripShow", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGripDirection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSelectableKind", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodAutomatic", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodDepthVariable", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodManual", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setTransformGumballFlag", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "addTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "relocateTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetVolumeFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setVoxelDims", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
struct Puzzle5dStorePreparationFactory;

struct Puzzle5dStorePreparation {
    base: Option<store::SnapshotRead<Puzzle5dPlaySnapshot>>,
    mutation: Option<Puzzle5dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(Puzzle5dPlaySnapshot, Vec<Puzzle5dMutation>, Puzzle5dMutation)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Puzzle5dPlaySnapshot, Puzzle5dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    phase: u8,
    cancelled: bool,
    closing: bool,
}

fn puzzle5d_store_edit(
    forward: Puzzle5dMutation,
    inverse: Vec<Puzzle5dMutation>,
    description: Option<String>,
    authority: &store::ArtifactStoreOneItemLiveAuthority,
) -> protocol::Edit<Puzzle5dMutation> {
    let id = format!("puzzle5d-retained-{}", authority.next_sequence_number());
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

impl store::ArtifactStoreOneItemPreparationFactory<Puzzle5dPlaySnapshot, Puzzle5dMutation> for Puzzle5dStorePreparationFactory {
    fn preflight(&self, _mutation: &Puzzle5dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Puzzle5d Store preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Puzzle5dPlaySnapshot, Puzzle5dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Puzzle5dPlaySnapshot, Puzzle5dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Puzzle5dPlaySnapshot, Puzzle5dMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Puzzle5dStorePreparation {
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

impl store::ArtifactStoreOneItemPreparation<Puzzle5dPlaySnapshot, Puzzle5dMutation> for Puzzle5dStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() || self.phase >= 2 {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        match self.phase {
            0 => {
                let base = self.base.as_ref().ok_or_else(|| "Puzzle5d preparation lost its exact base root".to_string())?;
                let mutation = self.mutation.take().ok_or_else(|| "Puzzle5d preparation lost its mutation owner".to_string())?;
                let inverse = mutation.inverse(base.get());
                let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|_| "Puzzle5d mutation could not produce its post root".to_string())?;
                self.candidate = Some((post, inverse, mutation));
                self.phase = 1;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: [0; 32] };
                Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            1 => {
                let (post, inverse, mutation) = self.candidate.take().ok_or_else(|| "Puzzle5d preparation lost its semantic candidate".to_string())?;
                let authority = self.authority.as_ref().ok_or_else(|| "Puzzle5d preparation lost its Store authority".to_string())?;
                let prepared = authority.prepare_one_item(puzzle5d_store_edit(mutation, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
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

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Puzzle5dPlaySnapshot, Puzzle5dMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Puzzle5dPlaySnapshot, Puzzle5dMutation>> {
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
                return Err("Puzzle5d preparation could not return its exact base root".into());
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

struct Puzzle5dConfigStorePreparationFactory;

struct Puzzle5dConfigStorePreparation {
    base: Option<store::SnapshotRead<Puzzle5dConfig>>,
    mutation: Option<Puzzle5dConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Puzzle5dConfig, Puzzle5dConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Puzzle5dConfig, Puzzle5dConfigMutation> for Puzzle5dConfigStorePreparationFactory {
    fn preflight(&self, _mutation: &Puzzle5dConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Puzzle5d config Store preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Puzzle5dConfig, Puzzle5dConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Puzzle5dConfig, Puzzle5dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Puzzle5dConfig, Puzzle5dConfigMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
        {
            return Err(request);
        }
        Ok(Box::new(Puzzle5dConfigStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Puzzle5dConfig, Puzzle5dConfigMutation> for Puzzle5dConfigStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Puzzle5d config preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Puzzle5d config preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|_| "Puzzle5d config mutation could not produce its post root".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Puzzle5d config preparation lost its Store authority".to_string())?;
        let id = format!("puzzle5d-config-retained-{}", authority.next_sequence_number());
        let edit = protocol::Edit {
            id: id.clone(),
            actor: Some(authority.actor().to_string()),
            forwards: vec![mutation],
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
            description: self.description.take(),
            coalesce_key: None,
            sequence_number: authority.next_sequence_number(),
            started_at: String::new(),
            finished_at: None,
        };
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Puzzle5dConfig, Puzzle5dConfigMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Puzzle5dConfig, Puzzle5dConfigMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("Puzzle5d config preparation could not return its exact base root".into()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️StorePreparation

//#region 📜️ToolProofs
/// 📜️ The retained command catalog's own bounded first-step proofs — one per
/// `PUZZLE5D_RETAINED_TOOL_IDS` entry, all joined to the single concrete
/// `Puzzle5dRetainedCommandJobFactory`.
struct Puzzle5dRetainedCommandProofs;

impl Puzzle5dRetainedCommandProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle5dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle5d@1/*#editor",
        artifact_schema: "puzzle.5d",
        factory: "Puzzle5dRetainedCommandJobFactory",
        factory_type: Puzzle5dRetainedCommandJobFactory,
        contract: ToolExecutionContract::resumable(PUZZLE5D_RETAINED_RAW_BYTES, PUZZLE5D_RETAINED_DECODED_ITEMS, 1, 262_144, 7_500, 1, 1),
        tools: [
            "canvasPointerDown",
            "cycleBrushCandidate",
            "cycleBrushCandidateBack",
            "worldPointerDown",
            "deleteSelection",
            "duplicateSelection",
            "engagementAbort",
            "engagementControlSelect",
            "engagementInput",
            "engagementRepeatLast",
            "engagementSubmit",
            "exportFixture",
            "importFixture",
            "openAddPartDialog",
            "openImportFixture",
            "selectSameKindSelection",
            "setFillCount",
            "setSelectionFlag",
            "targetBrushSuggestions",
            "focusSelection",
            "addBrushPart",
            "addNode",
            "addPartKind",
            "applyBoardEvents",
            "createFastener",
            "deleteFastener",
            "editFastener",
            "patchFastener",
            "patchGrip",
            "patchPart",
            "proximityConnect",
            "registerBrushMesh",
            "retargetFastener",
            "rotateSelection",
            "scaleSelection",
            "setActiveExample",
            "translateSelection",
            "worldRelocate",
            "setCamera",
            "setCamera2d",
            "setCamera3d",
            "setGridFactor",
            "setGridSnapEnabled",
            "setLodMode",
            "setSuggestionOffset",
            "toggleSun",
            "setSunAzimuth",
            "setSunElevation",
            "setSunIntensity",
            "setBrushPlacementContactTolerance",
            "setProximityRadius",
            "setChunkSize",
            "setPartKindWeight",
            "setGripKindWeight",
            "setGridVisible",
            "setGridSpacing",
            "setProjection",
            "setProjectionParam",
            "setGripShow",
            "setGripDirection",
            "setSelectableKind",
            "setLodAutomatic",
            "setLodDepthVariable",
            "setLodManual",
            "setTransformGumballFlag",
            "addTargetVolume",
            "deleteTargetVolume",
            "relocateTargetVolume",
            "setTargetVolumeFlag",
            "setVoxelDims"
        ]
    }
}

/// 📜️ The two framework-injected host-configuration verbs. Deliberately GENERIC proofs (no
/// `factory_type`): they are not app-owned retained tools — `Puzzle5dRetainedCommandJobFactory` never
/// claims them — they only need the wire admission and output budget `dispatch_action`'s
/// host-configuration branch asks for before it applies `host_configuration_mutation`. Without them the
/// shell's `setActiveTool`/`setActiveUtility` fall through to `admit_command_json` and fail closed with
/// `interactive-job.missing-factory`, so the fill TOOL could not even be armed.
struct Puzzle5dHostConfigurationProofs;

impl Puzzle5dHostConfigurationProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle5dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle5d@1/*#editor",
        artifact_schema: "puzzle.5d",
        factory: "BoundedFirstStepCommandJobFactory",
        contract: ToolExecutionContract::resumable(8_192, 8, 1, 8_192, 7_500, 1, 1),
        tools: ["setActiveTool", "setActiveUtility"]
    }
}
//#endregion 📜️ToolProofs

impl ArtifactEditor for Puzzle5dPlayApp {
    const DIALECT: semio_framework_plugin::app::Dialect = crate::PUZZLE5D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE5D_SCHEMA;
    type Snapshot = Puzzle5dPlaySnapshot;
    type Mutation = Puzzle5dMutation;
    type Config = Puzzle5dConfig;
    type ConfigMutation = Puzzle5dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = Puzzle5dPresence;
    type PresenceMutation = Puzzle5dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = Puzzle5dCommand;

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        window_ownership::register_config(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        window_ownership::register_transient(registry)
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<NoDraft, NoDraftMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    /// 👥️ Puzzle 5d presence is inline camera scalars, so the default root is its exact empty terminal.
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |_| true).expect("default Puzzle5d presence is the exact empty terminal")))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Puzzle5dStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Puzzle5dConfigStorePreparationFactory))
    }

    /// ⏯️ The fill utility's run and revalidate jobs and the brush utility's suggestions run: the puzzle 3d planners
    /// behind the 5d translation.
    fn build_tool_run_job(request: semio_framework_plugin::ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolRunJob>, Fault> {
        match request.tool_id {
            board2d::utilities::brush::UTILITY_ID => crate::editor::puzzle5d::precompute::brush::build_run_job(request),
            fill_tool::TOOL_ID => crate::editor::puzzle5d::precompute::fill::build_run_job(request),
            _ => Ok(None),
        }
    }

    /// 📜️ The retained command catalog's proofs plus the two host-configuration verbs' generic ones.
    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Puzzle5dRetainedCommandProofs::bounded_first_step_tool_proofs();
        proofs.extend(Puzzle5dHostConfigurationProofs::bounded_first_step_tool_proofs());
        proofs
    }

    /// 🧠️ What one document instance retains across turns: puzzle 3d's brush suggestions link, which the brush run
    /// job follows and publishes to.
    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(Puzzle3dInstanceOperationOwner::default())
    }

    /// 🚦️ Starts, wakes or aborts the brush suggestions run for the grip the link points at.
    fn pending_effects(owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>, _view: Option<&semio_framework_plugin::ViewModel>) -> Vec<Effect> {
        owner.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(semio_s_artifact_puzzle_3d::editor::puzzle3d::modes::edit::windows::main::utilities::brush::run_effects(&mut owner.brush_suggestions, doc.tool_run()))).unwrap_or_default()
    }


    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(Puzzle5dRetainedCommandJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: semio_framework_plugin::app::ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !PUZZLE5D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(Fault::from("puzzle5d-command-tool-mismatch"));
        }
        let tool_id = request.command.action_id();
        let mut work: Box<dyn crate::retained_command::PuzzleCommandWork<EditorApp<Self>>> = match tool_id {
            "engagementAbort" => Box::new(Puzzle5dWindowCommandWork::new(tool_id).with_tool_run(request.context.tool_run().cloned())),
            window if PUZZLE5D_WINDOW_TOOL_IDS.contains(&window) => Box::new(Puzzle5dWindowCommandWork::new(window)),
            "addBrushPart" | "addPartKind" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id)),
            "applyBoardEvents" => Box::new(Puzzle5dBoardEventsWork::default()),
            "translateSelection" | "rotateSelection" | "scaleSelection" => Box::new(Puzzle5dTransformWork::new(tool_id)),
            "focusSelection" => Box::new(Puzzle5dFocusSelectionWork::default()),
            "patchPart" => Box::new(Puzzle5dPatchPartWork::default()),
            "patchFastener" => Box::new(Puzzle5dPatchFastenerWork::default()),
            "editFastener" => Box::new(Puzzle5dEditFastenerWork::default()),
            "retargetFastener" => Box::new(Puzzle5dRetargetFastenerWork::default()),
            "proximityConnect" => Box::new(Puzzle5dProximityConnectWork::default()),
            "patchGrip" => Box::new(Puzzle5dPatchGripWork::default()),
            "deleteFastener" => Box::new(Puzzle5dDeleteFastenerWork::default()),
            "addNode" => Box::new(Puzzle5dAddNodeWork::default()),
            "createFastener" => Box::new(Puzzle5dCreateFastenerWork::default()),
            "exportFixture" => Box::new(Puzzle5dExportWork::default()),
            "setActiveExample" => Box::new(Puzzle5dSetActiveExampleWork::default()),
            "registerBrushMesh" => Box::new(Puzzle5dRegisterBrushMeshWork::default()),
            "worldRelocate" => Box::new(Puzzle5dWorldRelocateWork::default()),
            "relocateTargetVolume" => Box::new(Puzzle5dRelocateVolumeWork::default()),
            "setPartKindWeight" | "setGripKindWeight" => Box::new(Puzzle5dKindWeightWork::new(tool_id)),
            "worldPointerDown" | "canvasPointerDown" => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(tool_id)),
            _ => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent)),
        };
        work.bind_view_state(request.context.view_state.clone());
        work.bind_instance_owner(request.instance_operation_owner.clone());
        let payload = crate::retained_command::RetainedPuzzleCommandPayload {
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            interaction_state: request.interaction_state,
            interaction_hover: request.interaction_hover,
            window_config: request.window_config,
            window_transient: request.context.window_transient.clone(),
            context_identity: request.context.identity_digest(),
            completion: request.completion,
            command_id: Puzzle5dCommand::action_id,
            work,
        };
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_reserved_tool_job(mut request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if !["copy", "cut", "paste", "import-media"].contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        let raw = std::mem::take(&mut request.raw_wire);
        request.raw_wire = match puzzle5d_preflight_reserved_wire(raw, request.contract.max_raw_wire_bytes) {
            Ok(raw) => raw,
            Err((fault, rejected)) => {
                drop(rejected);
                return Err(fault);
            }
        };
        let job = match request.tool_id.as_str() {
            "copy" => {
                let interaction = match &request.input {
                    ArtifactReservedToolInput::Action { interaction, .. } => interaction.clone(),
                    _ => return Err(Fault::from("puzzle5d copy requires action input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dCopyJob { work: Puzzle5dClipboardWork::new(request, interaction), pending_completion_rejection: None, completed: false })
            }
            "cut" => {
                let interaction = match &request.input {
                    ArtifactReservedToolInput::Action { interaction, .. } => interaction.clone(),
                    _ => return Err(Fault::from("puzzle5d cut requires action input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dCutJob { work: Puzzle5dClipboardWork::new(request, interaction), pending_completion_rejection: None, completed: false })
            }
            "paste" => {
                let args = match &request.input {
                    ArtifactReservedToolInput::Action { args, .. } => args.clone(),
                    _ => return Err(Fault::from("puzzle5d paste requires action input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dPasteJob::new(request, args.map(|value| serde_json::Value::from(&value))))
            }
            "import-media" => {
                let (port, media) = match &request.input {
                    ArtifactReservedToolInput::Media { port, media } => (port.clone(), media.clone()),
                    _ => return Err(Fault::from("puzzle5d import-media requires media input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dImportJob::new(request, port, media))
            }
            _ => return Ok(None),
        };
        Ok(Some(job))
    }

    /// 📎 Ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d: replaces the old
    /// `crate::editor::puzzle5d::config::schema::register_app_schema()` self-registering call, which
    /// puzzle's plugin root used to reach `.setup()` for — `register_document_app`/`document_app`
    /// now call this automatically the moment `Puzzle5dPlayApp` is bound to a plugin, exactly like
    /// `🗒️note`'s own `app_schema` override.
    fn app_schema() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::puzzle5d::config::schema::app_schema_descriptor())
    }

    /// 🚀️ Boots on `default_document()` and on NOTHING else.
    ///
    /// 🐛️ This used to pre-warm three statics before returning: `NAKAGIN_EXAMPLE_DOCUMENT` (168 355 B
    /// of DSL), `CAPSULE_DREAM_EXAMPLE_DOCUMENT` (3 035 200 B of DSL → ~3.5 MB of JSON → a typed
    /// `Puzzle5dDocument` of 2 880 parts) and `PUZZLE5D_EXAMPLE_OPERATIONS` — the 4×4 matrix of
    /// PAIRWISE semantic diffs between all four example documents, i.e. sixteen diffs, several of them
    /// between a 2 880-part and a 180-part document. Every mount of this app paid all of it before its
    /// first frame, which is why a pane took seconds to boot and why this crate's own example-switch
    /// and retained-import laws ran for over a minute each (the 2026-09-21 test binary was killed by
    /// the 10-minute watchdog). None of it is needed to produce the boot document, and nothing is lost:
    /// each static is a `LazyLock`, and `puzzle5d_operations_from_document_change` already computes the
    /// ONE pair it needs. That matrix has since been deleted outright — see
    /// `puzzle5d_operations_from_document_change`.
    fn initial_snapshot() -> Puzzle5dPlaySnapshot {
        Puzzle5dPlaySnapshot(serde_json::to_value(default_document()).unwrap_or(serde_json::Value::Null))
    }

    fn clipboard_media_type() -> Option<MediaType> {
        Some(MediaType { class: MediaClass::Kit, form: MediaForm::Design })
    }

    fn copy_fragment(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        let (part_ids, fastener_ids) = puzzle5d_interaction_part_and_fastener_ids(interaction);
        puzzle5d_copy_fragment(doc.snapshot, &part_ids, &fastener_ids)
    }

    /// @emoji ✂️ B1: `ArtifactApp::cut_operations`'s signature carries no config output channel (it
    /// returns a bare `Vec<Self::Mutation>`, not an `Emit`), so this can only emit the document
    /// removal; clearing the selection is left to the framework's own post-cut selection reconciliation
    /// (the cut parts/fasteners are gone from the document either way, so a stale selection referencing
    /// them is inert until the next real selection action overwrites it).
    fn cut_operations(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>, interaction: &InteractionView<'_>) -> Vec<Puzzle5dMutation> {
        let (part_ids, fastener_ids) = puzzle5d_interaction_part_and_fastener_ids(interaction);
        puzzle5d_cut_operations(doc.snapshot, &part_ids, &fastener_ids)
    }

    /// @emoji 📋️ B1: `ArtifactApp::paste_operations` carries no `ConfigView` at all (only `doc`/
    /// `fragment`/`placement`), so the new selection can't be threaded through this call; a following
    /// `setSelection` command (which the host already issues after a paste in practice) is what
    /// actually selects the pasted parts now.
    fn paste_operations(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle5dMutation>, ClipboardError> {
        let expected = Self::clipboard_media_type().unwrap_or(MediaType { class: MediaClass::Kit, form: MediaForm::Design });
        if fragment.media_type != expected {
            return Err(ClipboardError::IncompatibleMediaType(fragment.media_type));
        }
        let fragment_value: serde_json::Value = serde_json::from_str(&fragment.dsl_text).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_parts: Vec<Puzzle5dPart> = serde_json::from_value(fragment_value.get("parts").cloned().unwrap_or_else(|| serde_json::json!([]))).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_fasteners: Vec<Puzzle5dFastener> = serde_json::from_value(fragment_value.get("fasteners").cloned().unwrap_or_else(|| serde_json::json!([]))).unwrap_or_default();
        let before = puzzle5d_projection_value(&doc.snapshot.0);
        let document: Puzzle5dDocument = <Puzzle5dDocument as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(&before)).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let delta = paste_delta_2d(&fragment_parts, &document.parts, placement);
        let (fresh_parts, fresh_fasteners) = paste_selection_local(&document, &fragment_parts, &fragment_fasteners, delta);
        let mut after = document;
        after.parts.extend(fresh_parts);
        after.fasteners.extend(fresh_fasteners);
        Ok(puzzle5d_operations_from_document_change(&before, &after))
    }

    /// 🏷️ Maps each `Puzzle5dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle5dCommand) -> &'static str {
        command.action_id()
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.map(dsl::os_pack::json::from_dsl_value);
        let window_id = args.as_ref().and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(Value::as_str).map(str::to_string);
        Puzzle5dCommand::try_from_action(action, args, window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 5D action '{action}'")))
    }

    /// @emoji 🧩️ Thin typed-command adapter — reconstructs the exact `(action, args, window_id)`
    /// triple `handle_action_impl` expects from the typed `Puzzle5dCommand`.
    fn handle(
        command: &Puzzle5dCommand,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Puzzle5dMutation, Puzzle5dConfigMutation, Self::DraftMutation>, Fault> {
        let selection = interaction.selection(PUZZLE5D_INTERACTION_DOMAIN);
        let window_id = view_state.and_then(|view| view.window_id.as_deref()).or_else(|| command.window_id()).unwrap_or(world3d::WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        with_puzzle5d_app(|app| Ok(app.handle_action_impl(command.action_id(), command.args(), command.window_id(), doc.snapshot, &runtime, view_state, selection, None, None).0))
    }

    /// 🕹️ `vortex` domain topology (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
    /// parts and fasteners as flat roots, grips nested under their owning part (mirrors puzzle3d's
    /// object→vortex-marker nesting).
    fn interaction_topology(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>) -> semio_framework_plugin::InteractionTopology {
        let document: Puzzle5dDocument = serde_json::from_value(doc.snapshot.0.clone()).unwrap_or_else(|_| empty_document());
        let mut ordered = Vec::new();
        for part in &document.parts {
            ordered.push(semio_framework_plugin::TopologyNode { id: part.id.clone(), granularity: PUZZLE5D_GRANULARITY_PART.into(), parent: None });
            for grip in &part.grips {
                ordered.push(semio_framework_plugin::TopologyNode { id: puzzle5d_grip_full_id(&part.id, &grip.id), granularity: PUZZLE5D_GRANULARITY_GRIP.into(), parent: Some(part.id.clone()) });
            }
        }
        for fastener in &document.fasteners {
            ordered.push(semio_framework_plugin::TopologyNode { id: fastener.id.clone(), granularity: PUZZLE5D_GRANULARITY_FASTENER.into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(PUZZLE5D_INTERACTION_DOMAIN.to_string(), semio_framework_plugin::DomainTopology { ordered });
        semio_framework_plugin::InteractionTopology { domains }
    }

    /// 🔌️ Declares puzzle5d's typed media I/O surface: the implicit document ports (from
    /// `.document([...])`/`.artifact_kind(...)` in `create_puzzle5d_app`) plus `kit:in` (accepting a
    /// `kit.catalog` fragment shaped like block3d's `puzzle3d_catalog_fragment`, fanning IN from
    /// potentially many producers) and `design:out` (this app's own `5d.puzzle` design artifact, fanning
    /// OUT to potentially many consumers).
    fn io() -> Option<AppIo> {
        let io = semio_framework::io::resolve_ready(AppIo::from_artifact(
            "puzzle.5d",
            MediaType { class: MediaClass::Kit, form: MediaForm::Design },
            ArtifactPresentation { id: "5d.puzzle".into(), name: "5D Puzzle".into(), dimension: "5d".into(), component_kind: "puzzle5d".into() },
        ));
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
        ])))
    }

    /// 🧵️ The synchronous editor callback is deliberately closed: production import enters only
    /// remains batch-only until an artifact-lane preparation owner can retire its publication roots.
    fn import_media(_port: &str, _media: &Media, _doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>) -> Result<Emit<Puzzle5dMutation, Puzzle5dConfigMutation, Self::DraftMutation>, MediaError> {
        Err(MediaError::NotImplemented)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let projection = puzzle5d_projection_value(&doc.snapshot.0);
        let window_for_body = if body_key == board2d::BODY_KEY { board2d::WINDOW_KIND_ID } else { world3d::WINDOW_KIND_ID };
        let window_id = view_state.window_id.as_deref().unwrap_or(window_for_body);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_for_body));
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        let labels = puzzle5d_labels(view_state).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.localization.unsupported", "puzzle5d has no authored label set for the host's locale/terminology axes"))?;
        // 🪟️ One `TreeWindows` per render, read off the host's `ViewModel::tree_windows` for exactly
        // the body being rendered — every panel container below shares its first-paint row budget.
        let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
        let node = match body_key {
            board2d::BODY_KEY => board2d::render(&envelope),
            world3d::BODY_KEY => world3d::render(&envelope, doc.tool_run(), &crate::editor::puzzle5d::precompute::puzzle5d_mesh_lane(doc.snapshot, &envelope.document)),
            artifact_panel::BODY_KEY => artifact_panel::render(&envelope, labels, &windows),
            catalogue::BODY_KEY => catalogue::render(&envelope, labels, &windows),
            inspection::BODY_KEY => inspection::render(&envelope, labels, &windows),
            settings_panel::BODY_KEY => settings_panel::render(&envelope, labels, settings_panel::panel_window_id(view_state)),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d unknown-body label admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let projection = puzzle5d_projection_value(&doc.snapshot.0);
        let window_kind = window_ownership::kind_for_view(view_state).unwrap_or_else(|| if body_key == board2d::BODY_KEY { board2d::WINDOW_KIND_ID } else { world3d::WINDOW_KIND_ID });
        let window_id = view_state.window_id.as_deref().unwrap_or(window_kind);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::transient_from_view(transient), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        // 🕹️ The ONE live read of the framework-owned `vortex` domain: both panes' paint, the gumball
        // and the inspection panel's per-entity field groups all render against this snapshot.
        let envelope = scene_from_projection_with_interaction(&projection, runtime, &active_utility, Puzzle5dInteractionSnapshot::from_interaction(interaction));
        let labels = puzzle5d_labels(view_state).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.localization.unsupported", "puzzle5d has no authored label set for the host's locale/terminology axes"))?;
        // 🪟️ One `TreeWindows` per render, read off the host's `ViewModel::tree_windows` for exactly
        // the body being rendered — every panel container below shares its first-paint row budget.
        let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
        let node = match body_key {
            board2d::BODY_KEY => board2d::render(&envelope),
            world3d::BODY_KEY => world3d::render(&envelope, doc.tool_run(), &crate::editor::puzzle5d::precompute::puzzle5d_mesh_lane(doc.snapshot, &envelope.document)),
            artifact_panel::BODY_KEY => artifact_panel::render(&envelope, labels, &windows),
            catalogue::BODY_KEY => catalogue::render(&envelope, labels, &windows),
            inspection::BODY_KEY => inspection::render(&envelope, labels, &windows),
            settings_panel::BODY_KEY => settings_panel::render(&envelope, labels, settings_panel::panel_window_id(view_state)),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d unknown-body label admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    fn window_engagements(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let projection = puzzle5d_projection_value(&doc.snapshot.0);
        let Some(labels) = puzzle5d_labels(view_state) else { return HashMap::new() };
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_kind = window_ownership::kind_for_view(view_state).unwrap_or(board2d::WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        HashMap::from([(window_id.to_string(), edit::puzzle5d_engagement(&envelope, window_kind, labels, doc.tool_run()))])
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let Some(labels) = puzzle5d_labels(view_state) else { return HashMap::new() };
        let window_kind = window_ownership::kind_for_view(view_state).unwrap_or(board2d::WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::transient_from_view(transient), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&puzzle5d_projection_value(&doc.snapshot.0), runtime, &active_utility);
        HashMap::from([(window_id.to_string(), edit::puzzle5d_engagement(&envelope, window_kind, labels, doc.tool_run()))])
    }

    fn window_measures(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let projection = puzzle5d_projection_value(&doc.snapshot.0);
        let Some(labels) = puzzle5d_labels(view_state) else { return HashMap::new() };
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_kind = window_ownership::kind_for_view(view_state).unwrap_or(board2d::WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        let measures = if window_kind == board2d::WINDOW_KIND_ID { board2d::window_measures(&envelope, labels) } else { world3d::window_measures(&envelope, labels) };
        HashMap::from([(window_id.to_string(), measures)])
    }

    /// 🛠️ The mode-level tool options rail: the fill tool's count entry and distribution trees, live against
    /// this instance's run so the count reads `loading` while the run is working.
    fn tool_measures(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(labels) = puzzle5d_labels(view_state) else { return HashMap::new() };
        let window_id = view_state.window_id.as_deref().unwrap_or(world3d::WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&puzzle5d_projection_value(&doc.snapshot.0), runtime, &active_utility);
        HashMap::from([(fill_tool::TOOL_ID.to_string(), fill_tool::measures(&envelope, labels, doc.tool_run()))])
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let projection = puzzle5d_projection_value(&doc.snapshot.0);
        let Some(labels) = puzzle5d_labels(view_state) else { return Vec::new() };
        let Some(is_de) = puzzle5d_is_de_locale(view_state) else { return Vec::new() };
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(world3d::WINDOW_KIND_ID));
        let window_id = view_state.window_id.as_deref().unwrap_or(world3d::WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        let selection = Puzzle5dContextSelection::from_surface(request.surface.as_ref());
        puzzle5d_context_menu_items(&envelope, &selection, labels, is_de, registry)
    }
}
//#endregion 🔖️PlayApp

//#region 🔖️Manifest
/// 🗨️ Fixed ceiling on the part-kind rows the `addPartKind` arg form and the "Add Part" dialog offer —
/// the manifest is minted once per process, so this select is built eagerly and must stay bounded
/// however wide a catalog a future example declares.
pub const PUZZLE5D_PART_KIND_OPTIONS_MAX: usize = 64;

/// 🗨️ The part kinds the "Add Part" dialog (and `addPartKind`'s own arg form) offers — read from the
/// shipped documents' own `kindCatalogs.parts`, exactly the rows `📌️panels/🛍️catalogue` renders,
/// deduplicated across documents in catalog order, with the same part-kind inference fallback the
/// catalogue panel applies to a document that declares no catalog. `AppDefinition` is built once per
/// process and never sees the live document, so the union of the shipped catalogs IS the reachable
/// kind set — not the single literal `"Part"` this select used to hardcode, which could not add a
/// single real kind of any example.
///
/// 🌙️ `capsule-dream` is deliberately NOT in that union, for two independent reasons measured on
/// 2026-09-21 (`📓️pz1-catalog-zero-diagnostics.md` §2.3):
/// 1. **It names no kind a human could pick.** Its own `kindCatalogs.parts` is empty — the catalog
///    is a composed child, absent from the standalone document — so the inference fallback runs over
///    its 2 880 parts, whose `part-kind` column holds the child's raw UUIDs
///    (`"0e240cd2-7f98-42b6-af39-34e7ee4fad35"`, …). `nakagin`'s and `concrete-forest`'s hold names
///    (`Base`, `Bridge`, `Capital`, `Tambour`, `Capsule With Balcony J`, …). A select that offers
///    UUIDs is a worse select, and with `PUZZLE5D_PART_KIND_OPTIONS_MAX = 64` they crowd out real
///    kinds.
/// 2. **It is what made this package undescribable.** Dereferencing
///    `CAPSULE_DREAM_EXAMPLE_DOCUMENT` here parses 3 035 200 B of DSL, re-serialises ~3.5 MB of
///    JSON and deserialises it into `Puzzle5dDocument` — inside the owned interpreter, while the
///    bundle is assembled, i.e. before `describe()` emits anything. `AppDefinition` is built on the
///    describe path, so this one call put `🧩️puzzle` over the 1 800 s guest epoch on its own.
///
/// 🖐️ Those rows are AUTHORED below rather than derived, in the two named examples' own catalog
/// order, and pinned to the documents by
/// `shipped_part_kinds_are_the_two_named_examples_own_catalog_rows`.
///
/// 🐛️ Deriving it dereferenced `CONCRETE_FOREST_EXAMPLE_DOCUMENT` and `NAKAGIN_EXAMPLE_DOCUMENT`,
/// i.e. parsed 171 591 B of authored DSL, re-serialised it to 205 896 B of JSON and deserialised
/// that into two typed `Puzzle5dDocument`s — on the `AppDefinition` path, which is the `describe()`
/// path AND every actor boot. Measured natively on 2026-09-22 (slice PZ2,
/// `🗑️generated/pz2-native-profile-*.txt`): `create_puzzle5d_app()` cost 338 ms cold and 1 ms with
/// those two statics already warm, so ALL of it was this one select. Thirteen authored pairs cost
/// nothing, and the law below is what keeps them true.
pub const PUZZLE5D_SHIPPED_PART_KINDS: &[(&str, &str)] = &[
    ("Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Left"),
    ("Base", "Base"),
    ("Bridge", "Bridge"),
    ("Capital", "Capital"),
    ("Capsule With Balcony Backslash", "Capsule With Balcony Backslash"),
    ("Capsule With Balcony J", "Capsule With Balcony J"),
    ("Capsule With Balcony L", "Capsule With Balcony L"),
    ("Capsule With Balcony P", "Capsule With Balcony P"),
    ("Capsule With Balcony S", "Capsule With Balcony S"),
    ("Capsule With Balcony Slash", "Capsule With Balcony Slash"),
    ("First Storey Tambour", "First Storey Tambour"),
    ("Last Storey Tambour", "Last Storey Tambour"),
    ("Tambour", "Tambour"),
];

/// 🗂️ The `partKind` select's options, mapped from [`PUZZLE5D_SHIPPED_PART_KINDS`].
fn puzzle5d_part_kind_options() -> Vec<ActionArgOption> {
    PUZZLE5D_SHIPPED_PART_KINDS.iter().take(PUZZLE5D_PART_KIND_OPTIONS_MAX).map(|(id, label)| ActionArgOption::new(*id, LocalizedLabel::data(*label))).collect()
}

/// 🗨️ The kind the `partKind` select stages when nothing is picked — the first catalog row, never a
/// literal id no catalog declares.
fn puzzle5d_default_part_kind(options: &[ActionArgOption]) -> String {
    options.first().map(|option| option.value.clone()).unwrap_or_default()
}

/// 🗨️ The one `partKind` select both the standalone `addPartKind` arg form and the "Add Part" dialog
/// declare — built twice from the same catalog so the two forms can never drift apart.
fn puzzle5d_part_kind_arg() -> ActionArgDef {
    let options = puzzle5d_part_kind_options();
    let default = puzzle5d_default_part_kind(&options);
    ActionArgDef::select("partKind", puzzle5d_localized(|l| l.kind), options).default_value(&default)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `vortex` domain declaration —
/// one granularity per previously-distinct `Puzzle5dSelection` bag (part/grip/fastener). `Topology`
/// hierarchy (see `Puzzle5dPlayApp::interaction_topology`) exposes the part→grip nesting.
fn puzzle5d_interaction_definition() -> InteractionDefinition {
    let granularity = |id: &str, label: LocalizedLabel, icon: &str| GranularityDefinition { id: id.into(), label, icon_id: icon.into() };
    InteractionDefinition {
        id: PUZZLE5D_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Vortex", "Vortex"),
        granularities: vec![
            granularity(PUZZLE5D_GRANULARITY_PART, puzzle5d_localized(|l| l.part), "box"),
            granularity(PUZZLE5D_GRANULARITY_GRIP, puzzle5d_localized(|l| l.grip), "circle-dot"),
            granularity(PUZZLE5D_GRANULARITY_FASTENER, LocalizedLabel::native("Fastener", "Verbinder"), "link"),
        ],
        hierarchy: HierarchyProvider::Topology,
        hover: HoverSpec { enabled: true, transitive: false, channels: vec![PUZZLE5D_HOVER_CHANNEL.into()], broadcast: true },
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}

/// 🚧️ SDK note (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4): `EditorBuilder`
/// has no `.example(...)`/`.workflow(...)` methods — `.editor::<E>(def: AppDefinition)` only takes the
/// bare definition, `App.examples` is discarded by the plugin-root builder. The three examples this
/// app used to register (`PUZZLE5D_EXAMPLE_CONCRETE_FOREST`/`_NAKAGIN`/`_CAPSULE_DREAM`) and the
/// `"puzzle5d"` workflow tag are DROPPED here, not silently ported — the constants/JSON statics stay
/// live for `setActiveExample`'s own dispatch path (`🎮️commands/🛍️set-active-example`), only the
/// manifest-level registration is gone.
pub fn create_puzzle5d_app() -> semio_framework_plugin::AppDefinition {
    let envelope = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: PUZZLE5D_DEFAULT_UTILITY.into(), interaction: Puzzle5dInteractionSnapshot::default() };
    let manifest_labels = puzzle5d_labels(&semio_framework_plugin::ViewModel::default()).expect("puzzle5d authored a label set for the host's own default axes");
    Editor::builder(Puzzle5dPlayApp::DIALECT)
            .document(["semio", "puzzle", "5d"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "puzzle", "5d"])
            .mode_def(edit::definition())
            .default_mode_id(edit::PUZZLE5D_PLAY_MODE_EDIT)
            .window_kind_def(board2d::definition(&envelope, manifest_labels))
            .window_kind_def(world3d::definition(&envelope, manifest_labels))
            .interaction(puzzle5d_interaction_definition())
            .window_kind_interactions(board2d::WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE5D_INTERACTION_DOMAIN)])
            .window_kind_interactions(world3d::WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE5D_INTERACTION_DOMAIN)])
            .window_kind_action_refs(board2d::WINDOW_KIND_ID, vec![board2d::actions::apply_board_events::reference(), board2d::actions::set_camera::reference()])
            .window_kind_action_refs(
                world3d::WINDOW_KIND_ID,
                vec![
                    world3d::actions::translate_selection::reference(),
                    world3d::actions::rotate_selection::reference(),
                    world3d::actions::scale_selection::reference(),
                    world3d::actions::world_relocate::reference(),
                    world3d::actions::set_camera::reference(),
                ],
            )
            // 🏗️ 3D-first 60/40 split — mirrors semio_compose_rs's design app (scene 60% / diagram 40%,
            // `semio_compose_rs/client/lib/sketchpad/js/index.ts:15367-15378`), the assembly-editing use case
            // this app replaces.
            .default_layout(edit::layout())
            .panel_tab_def(artifact_panel::definition())
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
            // 🔧️ Document-mutating operations (emit VCS operations through the before/after document delta).
            // 📤️📥️ Document IO. `exportFixture`/`openImportFixture` are SHELL verbs (a download, a file
            // picker — no document mutation of their own); `importFixture` is the mutation the picker
            // re-dispatches once per wire page and never a menu row of its own, so the user-facing
            // "Import" row is the one that actually opens a picker.
            .action_with(ActionDefinition::bounded_catalog("exportFixture", puzzle5d_localized(|l| l.export), ActionKind::Shell).with_category("file"))
            .action_with(ActionDefinition::bounded_catalog("openImportFixture", puzzle5d_localized(|l| l.import), ActionKind::Shell).with_category("file"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importFixture", puzzle5d_localized(|l| l.import), ActionKind::Mutation) })
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            // 🗨️ Shell-only effect: opens the declared "addPart" dialog, whose submit dispatches
            // `addPartKind`. The parametrized verb itself is NOT a palette row — its `partKind` select IS
            // the dialog, and offering both published two rows carrying the same "Add Part" label, the
            // second one opening a bare action pane (puzzle 3d's own measured defect).
            .action_with(ActionDefinition::bounded_catalog("openAddPartDialog", puzzle5d_localized(|l| l.add_part_prompt), ActionKind::Shell).with_category("create"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("addPartKind", puzzle5d_localized(|l| l.add_part), ActionKind::Mutation).with_category("create") })
            .mutation("addBrushPart", LocalizedLabel::native("Add Brush Part", "Pinselteil hinzufügen"))
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteSelection")
            .action_with(ActionDefinition::bounded_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).with_category("create"))
            .action_with(ActionDefinition::bounded_catalog("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).with_category("settings"))
            .action_with(ActionDefinition::bounded_catalog("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"), ActionKind::Mutation).with_category("view"))
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .mutation("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"))
            .mutation("patchPart", LocalizedLabel::native("Patch Part", "Teil aktualisieren"))
            .mutation("patchGrip", LocalizedLabel::native("Patch Grip", "Griff aktualisieren"))
            .mutation("patchFastener", LocalizedLabel::native("Patch Fastener", "Verbinder aktualisieren"))
            .mutation("createFastener", LocalizedLabel::native("Create Fastener", "Verbinder erstellen"))
            .mutation("deleteFastener", LocalizedLabel::native("Delete Fastener", "Verbinder löschen"))
            .action_destructive("deleteFastener")
            .mutation("retargetFastener", LocalizedLabel::native("Retarget Fastener", "Verbinder umhängen"))
            .mutation("editFastener", LocalizedLabel::native("Edit Fastener", "Verbinder bearbeiten"))
            .mutation("proximityConnect", LocalizedLabel::native("Proximity Connect", "Näherungsverbinden"))
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .mutation("worldRelocate", LocalizedLabel::native("Relocate Part", "Teil verlagern"))
            .mutation("addTargetVolume", LocalizedLabel::native("Add Target Volume", "Zielvolumen hinzufügen"))
            .action_with(ActionDefinition::bounded_catalog("deleteTargetVolume", LocalizedLabel::native("Delete Target Volume", "Zielvolumen löschen"), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteTargetVolume")
            .action_with(ActionDefinition::bounded_catalog("setTargetVolumeFlag", LocalizedLabel::native("Set Target Volume Flag", "Zielvolumen-Markierung festlegen"), ActionKind::Mutation).with_category("settings"))
            .mutation("relocateTargetVolume", LocalizedLabel::native("Relocate Target Volume", "Zielvolumen verlagern"))
            .mutation("applyBoardEvents", LocalizedLabel::native("Apply Board Events", "Board-Ereignisse anwenden"))
            // 👁️ Ephemeral view state — selection, hover, utility parameters, brush cycling, camera pose.
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("setCamera2d", LocalizedLabel::native("Set Camera 2D", "Kamera 2D festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("setCamera3d", LocalizedLabel::native("Set Camera 3D", "Kamera 3D festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::bounded_catalog("selectSameKindSelection", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).with_category("selection"))
            .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand"))
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition::new("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View, "hand"))
            .action_audience("engagementAbort", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition::new("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"), ActionKind::View, "hand"))
            .view_action("cycleBrushCandidate", LocalizedLabel::native("Cycle Brush Candidate", "Pinselkandidat wechseln"))
            .view_action("cycleBrushCandidateBack", LocalizedLabel::native("Cycle Brush Candidate Back", "Pinselkandidat rückwärts wechseln"))
            .view_action("targetBrushSuggestions", LocalizedLabel::native("Target Brush Suggestions", "Pinselvorschläge ausrichten"))
            .view_action("registerBrushMesh", LocalizedLabel::native("Register Brush Mesh", "Pinsel-Mesh registrieren"))
            .view_action("setBrushPlacementContactTolerance", LocalizedLabel::native("Set Brush Placement Contact Tolerance", "Pinsel-Kontakttoleranz festlegen"))
            .view_action("setProximityRadius", LocalizedLabel::native("Set Proximity Radius", "Näherungsradius festlegen"))
            .view_action("setChunkSize", LocalizedLabel::native("Set Chunk Size", "Blockgröße festlegen"))
            .view_action("setPartKindWeight", LocalizedLabel::native("Set Part Kind Weight", "Teileart-Gewicht festlegen"))
            .view_action("setGripKindWeight", LocalizedLabel::native("Set Grip Kind Weight", "Griffart-Gewicht festlegen"))
            .action_with(ActionDefinition::new("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"), ActionKind::View, "layers"))
            .view_action("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"))
            .view_action("setSuggestionOffset", LocalizedLabel::native("Set Suggestion Offset", "Vorschlagsversatz festlegen"))
            .action_with(ActionDefinition::new("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View, "grid-3x3"))
            .action_with(ActionDefinition::new("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"), ActionKind::View, "grid-3x3"))
            .action_with(ActionDefinition::new("setVoxelDims", LocalizedLabel::native("Set Voxel Dims", "Voxelmaße festlegen"), ActionKind::View, "box"))
            .action_with(ActionDefinition::new("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster anzeigen"), ActionKind::View, "layout-grid"))
            .action_with(ActionDefinition::new("setGridSpacing", LocalizedLabel::native("Set Grid Spacing", "Rasterabstand festlegen"), ActionKind::View, "grid-3x3"))
            .action_with(ActionDefinition::new("setProjection", LocalizedLabel::native("Set Projection", "Projektion festlegen"), ActionKind::View, "video"))
            .action_with(ActionDefinition::new("setProjectionParam", LocalizedLabel::native("Set Projection Parameter", "Projektionsparameter festlegen"), ActionKind::View, "video"))
            .action_with(ActionDefinition::new("setGripShow", LocalizedLabel::native("Set Grip Markers", "Griffmarken festlegen"), ActionKind::View, "circle-dot"))
            .action_with(ActionDefinition::new("setGripDirection", LocalizedLabel::native("Set Grip Direction", "Griffrichtung festlegen"), ActionKind::View, "compass"))
            .action_with(ActionDefinition::new("setSelectableKind", LocalizedLabel::native("Set Selectable Kind", "Auswählbare Art festlegen"), ActionKind::View, "mouse-pointer").with_category("selection"))
            .action_with(ActionDefinition::new("setLodAutomatic", LocalizedLabel::native("Set Lod Automatic", "LOD automatisch"), ActionKind::View, "zoom-in"))
            .action_with(ActionDefinition::new("setLodDepthVariable", LocalizedLabel::native("Set Lod Depth Variable", "LOD tiefenabhängig"), ActionKind::View, "layers"))
            .action_with(ActionDefinition::new("setLodManual", LocalizedLabel::native("Set Lod Manual", "LOD manuell festlegen"), ActionKind::View, "layers"))
            .action_with(ActionDefinition::new("setTransformGumballFlag", LocalizedLabel::native("Set Transform Gumball Flag", "Transformationsgriff umschalten"), ActionKind::View, "move-3d"))
            .action_with(ActionDefinition::new("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            .action_audience("worldPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            .action_audience("canvasPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("addBrushPart", InteractiveJobClassification::Migrated)
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("addPartKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("applyBoardEvents", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("createFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidate", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidateBack", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("editFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementAbort", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementControlSelect", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementRepeatLast", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("focusSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportFixture", InteractiveJobClassification::Migrated)
            .action_interactive_job("importFixture", InteractiveJobClassification::Migrated)
            .action_interactive_job("openAddPartDialog", InteractiveJobClassification::Migrated)
            .action_interactive_job("openImportFixture", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchGrip", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchPart", InteractiveJobClassification::Migrated)
            .action_interactive_job("proximityConnect", InteractiveJobClassification::Migrated)
            .action_interactive_job("registerBrushMesh", InteractiveJobClassification::Migrated)
            .action_interactive_job("retargetFastener", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectSameKindSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushPlacementContactTolerance", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProximityRadius", InteractiveJobClassification::Migrated)
            .action_interactive_job("setChunkSize", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera2d", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera3d", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFillCount", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridFactor", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSnapEnabled", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridVisible", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSpacing", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjectionParam", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGripShow", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGripDirection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectableKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodAutomatic", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodDepthVariable", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodManual", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTransformGumballFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("setPartKindWeight", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSuggestionOffset", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGripKindWeight", InteractiveJobClassification::Migrated)
            .action_interactive_job("targetBrushSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("addTargetVolume", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteTargetVolume", InteractiveJobClassification::Migrated)
            .action_interactive_job("relocateTargetVolume", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetVolumeFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setVoxelDims", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldRelocate", InteractiveJobClassification::Migrated)
            // 📝️ Staged argument forms for the brush create actions (P1).
            .action_args("addPartKind", vec![puzzle5d_part_kind_arg()])
            // 🗨️ The dialog `openAddPartDialog` opens, driving the SAME `addPartKind` operation and the
            // SAME live `partKind` select the arg form declares.
            .dialog(
                DialogDefinition::new(open_add_part_dialog::PUZZLE5D_ADD_PART_DIALOG, puzzle5d_localized(|l| l.add_part), semio_framework_plugin::ActionRef::new("addPartKind"))
                    .body(puzzle5d_localized(|l| l.add_part_body))
                    .args(vec![puzzle5d_part_kind_arg().required()])
                    .submit_label(puzzle5d_localized(|l| l.add)),
            )
            .action_args("addBrushPart", vec![
                ActionArgDef::select("partKind", puzzle5d_localized(|l| l.kind), vec![ActionArgOption::new("Part", puzzle5d_localized(|l| l.part))]).default_value(&"Part"),
            ])
            // 🧰️ Flat per-window set of utilities; `select` is the default. Each `🪛️utilities/*` node
            // owns its own id/definition; a utility bound by BOTH windows is declared once (under the
            // 2D window) and referenced by the 3D window's `definition()`.
            .utility(board2d::utilities::select::definition(puzzle5d_localized(|l| l.select)))
            .utility(world3d::utilities::transform::move_definition())
            .utility(world3d::utilities::transform::rotate_definition())
            .utility(world3d::utilities::transform::scale_definition())
            .utility(board2d::utilities::brush::definition(puzzle5d_localized(|l| l.brush)))
            .utility(world3d::utilities::volume_brush::definition(puzzle5d_localized(|l| l.volume_brush)))
            .utility(world3d::utilities::world_relocate::definition())
            // 🛠️ Fill is a mode-level TOOL (a whole-document generator over both projections), not a window
            // utility — it keeps its viewport interaction in both panes through the host's `active_tool_id`.
            .tool(fill_tool::definition(puzzle5d_localized(|l| l.fill)))
            .mode_tools(edit::PUZZLE5D_PLAY_MODE_EDIT, vec![semio_framework::io::resolve_ready(ToolRef::new(fill_tool::TOOL_ID))])
    .build_definition()
}

// 🗂️ `Puzzle5dPlaySnapshot`'s pack<->dsl codec (so `framework/sync`'s `FolderEndpoint::Pack` can
// print/parse puzzle-5d play documents without depending on this crate's concrete
// `Projection`/`Mutation` types) is now declared via `.document_codec::<Puzzle5dPlayApp>()` on
// `crate::declaration()` (ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`
// M1) — the old side-effecting `register_puzzle5d_exports()` wrapper (this app file's only caller of
// `register_document_codec_for_app`) is gone. The 5d mesh export/import OS-host registration
// (`register_mesh_io()`/`puzzle5d_document_from_mesh`) was never rewired to a real `.setup()` caller
// after the artifacts-only-plugin-architecture migration (`🧩️puzzle/🦀️.rs`'s `plugin()`
// builder chain has no `.setup()` call at all) and was deleted as dead code (ticket
// 26/08/17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS) — same fate as puzzle3d's
// sibling mesh bridge. Mesh export/import should be re-derived from `io_dispatch`'s real
// `ComposerEntry` chain if/when this bridge is needed again.
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ The one puzzle5d-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
/// instead of re-deriving a store/dispatch/render scaffold of its own.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;

/// 🧊️ The target-volume / Volume-Brush laws — a topic of its own so the one shared harness stays the
/// only scaffold and this family's laws are readable as one block.
#[cfg(test)]
#[path = "🧪️tests/🔬️target-volumes/🦀️.rs"]
mod target_volume_tests;
//#endregion 🧪️UnitTests

