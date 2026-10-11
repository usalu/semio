//! 👯️ Puzzle 5d play app — the plugin's unified 2d+3d play app: its `ArtifactApp` impl
//! (dispatch-only), the structural-twin document model its command/panel/window nodes mutate and
//! render, the shared scene/engine/brush helpers those nodes reach for, and the manifest that
//! stitches them together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/{◻2d,🧊️3d}`. This file dispatches and stitches.
//!
//! 🌉️ `ArtifactApp::Snapshot` is the `Puzzle5dPlaySnapshot` newtype over a bare
//! `Value` document (see `crate::standards::v1::subsets::any::io::text::mutations`'s `🔖️ValueBridge`), not the
//! typed `Puzzle5dSnapshot` — the `Puzzle5dDocument` model below is this app's own structural twin
//! of it, and each action emits the concrete typed mutation kinds of the gesture it carries
//! (`🎮️commands/<group>`), never a delta of two documents.

#[path="📸️snapshot/🦀️.rs"]
pub mod snapshot;


use crate::editor::puzzle5d::snapshot::Puzzle5dPlaySnapshot;

use crate::standards::v1::subsets::any::schema::mutations::{Puzzle5dMutation};

use crate::Puzzle5dSnapshot;
use crate::editor::puzzle5d::commands::{
    apply_sun, cycle_brush_candidate, delete_selection, duplicate_selection, engagement_abort, engagement_control_select, engagement_input,
    engagement_repeat_last, engagement_submit, register_brush_mesh, rotate_selection, scale_selection, select_same_kind, set_brush_placement_contact_tolerance,
    set_camera, set_camera_2d, set_camera_3d, set_fill_count, set_grid_factor, set_grid_snap_enabled, set_kind_weight, set_lod_mode, set_selection_flag, set_suggestion_offset, target_brush_suggestions, translate_selection, world_relocate,
};
use crate::editor::puzzle5d::commands::focus_selection;
use crate::editor::puzzle5d::commands::{
    set_grid_spacing, set_grid_visible, set_grip_direction, set_grip_show, set_lod_automatic, set_lod_depth_variable, set_lod_manual, set_projection, set_selectable_kind, set_transform_gumball_flag,
};
use crate::editor::puzzle5d::commands::{add_target_volume, delete_target_volume, relocate_target_volume, set_target_volume_flag, set_voxel_dims};
use crate::editor::puzzle5d::commands::{set_chunk_size, set_proximity_radius};
use crate::editor::puzzle5d::commands::{accept_suggestion, close_vortex_suggestions, hover_suggestion, open_vortex_suggestions};
use crate::editor::puzzle5d::commands::{export_snapshot, import_snapshot, open_add_part_dialog, open_import_snapshot};
use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dConfig, Puzzle5dConfigMutation, Puzzle5dRuntime, Puzzle5dConfigSetObjectKindWeights, Puzzle5dConfigSetVortexKindWeights};
use crate::editor::puzzle5d::modes::edit;
use crate::editor::puzzle5d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle5d::modes::edit::windows::{board2d, world3d};
use crate::editor::puzzle5d::panels::{catalogue, artifact as artifact_panel, inspection, settings as settings_panel};
use semio_s_artifact_puzzle_3d::editor::puzzle3d::Puzzle3dInstanceOperationOwner;
use crate::editor::puzzle5d::presence::{Puzzle5dPresence, Puzzle5dPresenceMutation};
use crate::editor::puzzle5d::terminology::{puzzle5d_labels, puzzle5d_localized, Puzzle5dLabels};
use crate::editor::puzzle5d::window as window_ownership;
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PasteAnchor, PastePlacement, UiDirtyScope};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactPresentation;
use semio_framework_plugin::ArtifactReservedJob;
use semio_framework_plugin::ArtifactReservedToolInput;
use semio_framework_plugin::ArtifactReservedToolJob;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::ArtifactToolCompletion;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::DialogDefinition;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::InteractionWrite;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPortDirection;
use semio_framework_plugin::MediaPortSpec;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PortMultiplicity;
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
// 🕹️ `InteractionView` — see 🧊️3d/🦀️.rs's identical import comment (missing top-level
// re-export from `semio_framework_plugin`, flagged to the coordinator, not fixed here).
use semio_framework_job::{InteractiveJob, Operation, StepContext};
use semio_framework_plugin::app::{ArtifactToolCompletionRejection, InteractionView};
use semio_framework_value::{FromValue, ToValue};
use semio_framework_pack_json::{parse, Value};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Constants
pub const PUZZLE5D_PLAY_APP_ID: &str = "puzzle5d-play";
pub const PUZZLE5D_PLAY_CONTROLLER_ID: &str = "puzzle5d-play";
pub const PUZZLE5D_PLAY_WINDOWS: [&str; 2] = [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID];
pub const PUZZLE5D_SCHEMA: &str = "puzzle.5d";
pub const PUZZLE5D_BOARD_SNAPSHOT_SCHEMA: &str = "board.ports.directed.v1";
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
    ActionDescriptor { controller_id: PUZZLE5D_PLAY_CONTROLLER_ID.into(), action: action.into(), args: args.map(|value| semio_framework_pack_json::to_dsl_value(&value)) }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: builds a framework `interactionSelect`
/// action targeting one `(granularity, id)` pair in the `vortex` domain — replaces the deleted
/// `setSelection` action builders every document tree row used to construct by hand.
pub fn puzzle5d_interaction_select(granularity: &str, id: &str) -> ActionDescriptor {
    let targets = semio_framework_pack_json::to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    puzzle5d_action(INTERACTION_SELECT_ACTION_ID, Some(semio_framework_pack_json::json!({ "domainId": PUZZLE5D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })))
}

#[derive(Clone, Debug, Default, semio_framework_value::RetireOwned)]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGrip2d {
    #[value(default)]
    pub angle: f64,
    #[value(default, rename = "gripKind")]
    pub grip_kind: String,
    #[value(default)]
    pub radius: f64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGrip3d {
    #[value(default)]
    pub position: [f64; 3],
    #[value(default)]
    pub direction: Option<[f64; 3]>,
    #[value(default)]
    pub radius: f64,
    #[value(default)]
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGrip {
    pub id: String,
    #[value(default, rename = "gripKind")]
    pub grip_kind: String,
    #[value(default, rename = "2d")]
    pub grip_2d: Puzzle5dGrip2d,
    #[value(default, rename = "3d")]
    pub grip_3d: Puzzle5dGrip3d,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "lowercase")]
pub enum Puzzle5dPartAnchor {
    #[default]
    Fixed,
    Derived,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dFastener {
    pub id: String,
    pub source: String,
    pub target: String,
    #[value(default, rename = "fastenerKind", skip_serializing_if = "Option::is_none")]
    pub fastener_kind: Option<String>,
    #[value(default)]
    pub gap: f64,
    #[value(default)]
    pub shift: f64,
    #[value(default)]
    pub rise: f64,
    #[value(default)]
    pub rotation: f64,
    #[value(default)]
    pub turn: f64,
    #[value(default)]
    pub tilt: f64,
    #[value(default)]
    pub x: f64,
    #[value(default)]
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart2d {
    #[value(default)]
    pub x: f64,
    #[value(default)]
    pub y: f64,
    #[value(default)]
    pub shape: String,
    #[value(default)]
    pub radius: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(default)]
    pub text: String,
    #[value(default, rename = "iconKind", skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart3d {
    #[value(default)]
    pub origin: [f64; 3],
    #[value(default, rename = "meshUrl")]
    pub mesh_url: Option<String>,
    #[value(default)]
    pub orientation: Option<[f64; 4]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Value>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart {
    pub id: String,
    #[value(rename = "partKind")]
    pub part_kind: String,
    #[value(default)]
    pub anchor: Puzzle5dPartAnchor,
    #[value(default, rename = "2d")]
    pub part_2d: Puzzle5dPart2d,
    #[value(default, rename = "3d")]
    pub part_3d: Puzzle5dPart3d,
    #[value(default)]
    pub grips: Vec<Puzzle5dGrip>,
}

/// 🧊️ One oriented box constraining where the fill planner may place, in the document's 3D pose
/// space. The Volume Brush paints grid-snapped axis-aligned instances sized by the world window's
/// voxel dims; the transform gumball edits arbitrary oriented boxes through `relocateTargetVolume`.
/// The board pane paints the flat rectangle this box projects to, never a second persisted pose.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dTargetVolume {
    pub id: String,
    #[value(default)]
    pub origin: [f64; 3],
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Value>,
    #[value(default)]
    pub hidden: bool,
    #[value(default)]
    pub locked: bool,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dDocument {
    pub schema: String,
    #[value(default)]
    pub domain: String,
    #[value(default)]
    pub parts: Vec<Puzzle5dPart>,
    #[value(default)]
    pub fasteners: Vec<Puzzle5dFastener>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub target_volumes: Vec<Puzzle5dTargetVolume>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    #[value(default, rename = "kindCatalogs", skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<Value>,
    #[value(default, rename = "kindCompatibility", skip_serializing_if = "Option::is_none")]
    pub kind_compatibility: Option<Value>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

pub fn empty_document() -> Puzzle5dDocument {
    Puzzle5dDocument { schema: PUZZLE5D_SCHEMA.into(), domain: "architecture".into(), parts: Vec::new(), fasteners: Vec::new(), target_volumes: Vec::new(), meta: None, kind_catalogs: None, kind_compatibility: None, label: None }
}

/// 📥️ Decodes a SHIPPED example's JSON into the editor twin. A failure here is a build defect in the
/// example asset, never user input, so it is loud: the silent `empty_document()` fallback this used to
/// carry is what let the 2026-09-17 example regression ship a zero-part Nakagin and Capsule Dream.
/// User-supplied JSON arrives through `📥️import-snapshot`, which refuses with a notice instead.
pub fn document_from_json(json_text: &str) -> Puzzle5dDocument {
    semio_framework_pack_json::from_json_str::<Puzzle5dDocument>(json_text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("puzzle5d example document decodes: {error}"))
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

/// 🔎️ Reads a fixed-length `[f64; 3]` coordinate triple straight off a dsl `Value::Array` —
/// the direct replacement for the old `puzzle5d_record_from_projection::<[f64; 3]>(value.clone())` round
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

/// 🌉️ Bridges `Puzzle5dPlaySnapshot::value()` — the lazily materialized legacy `Value`
/// projection of the typed snapshot — into this file's own first-party `Value` via `DslValue` without ever
/// naming the foreign crate: `T`'s only real caller is `snapshot.value(): &Value`, resolved
/// structurally through the `DslValue: From<T>` bound — mirrors `🧊️3d/…/✏️editor/🦀️.rs`'s
/// `puzzle3d_projection_value`.
fn puzzle5d_projection_value<T:std::borrow::Borrow<semio_framework_value::DslValue>>(value:T)->Value{
    semio_framework_pack_json::from_dsl_value(value.borrow())
}

fn puzzle5d_editor_projection(snapshot: &Puzzle5dPlaySnapshot) -> Value {
    value_from_document(&puzzle5d_document_from_snapshot(snapshot.typed()).expect("admitted typed snapshot has a host projection"))
}

/// 🌱️ Admits a native host projection through the owned record contract.
fn puzzle5d_record_from_projection<T: FromValue>(value: Value) -> Result<T, semio_framework_value::ValueError> {
    T::from_value(semio_framework_pack_json::to_dsl_value(&value))
}

fn puzzle5d_paste_placement(args: &Value) -> Result<PastePlacement, semio_framework_value::ValueError> {
    let refusal = || semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "puzzle5d paste placement is malformed");
    let anchor = match args.get("anchor").map(|value| value.as_str().ok_or_else(refusal)).transpose()?.unwrap_or("original") {
        "original" => PasteAnchor::Original, "middle" => PasteAnchor::Middle, "centroid" => PasteAnchor::Centroid,
        "bottomLeft" => PasteAnchor::BottomLeft, "bottomRight" => PasteAnchor::BottomRight,
        "topLeft" => PasteAnchor::TopLeft, "topRight" => PasteAnchor::TopRight,
        _ => return Err(refusal()),
    };
    let position = args.get("position").filter(|value| !value.is_null()).map(|value| {
        let values = value.as_array().ok_or_else(refusal)?;
        if values.len() != 3 { return Err(refusal()); }
        Ok([values[0].as_f64().ok_or_else(refusal)?, values[1].as_f64().ok_or_else(refusal)?, values[2].as_f64().ok_or_else(refusal)?])
    }).transpose()?;
    Ok(PastePlacement { anchor, position })
}

/// 🧬️ Admits each host entity and splits catalogue custody into the typed child reference.
pub fn puzzle5d_snapshot_from_document(document: &Puzzle5dDocument) -> Result<Puzzle5dSnapshot, semio_framework_value::ValueError> {
    if document.schema != PUZZLE5D_SCHEMA {
        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "puzzle5d document schema is not admitted"));
    }
    let catalogs = document.kind_catalogs.as_ref().map(|value| crate::Puzzle5dKindCatalogs::from_value(value.to_value())).transpose()?;
    let (kind_catalogs, kind_catalogs_extra) = crate::split_and_seed_kind_catalogs(catalogs);
    Ok(Puzzle5dSnapshot {
        schema: document.schema.clone(), domain: document.domain.clone(), label: document.label.clone(),
        meta: document.meta.as_ref().map(|value| crate::Puzzle5dMeta::from_value(value.to_value())).transpose()?.unwrap_or_default(),
        kind_catalogs, kind_catalogs_extra,
        kind_compatibility: document.kind_compatibility.as_ref().map(|value| Vec::<crate::Puzzle5dKindCompatibility>::from_value(value.to_value())).transpose()?.unwrap_or_default(),
        parts: document.parts.iter().map(|part| crate::Puzzle5dPart::from_value(part.to_value())).collect::<Result<_, _>>()?,
        fasteners: document.fasteners.iter().map(|fastener| crate::Puzzle5dFastener::from_value(fastener.to_value())).collect::<Result<_, _>>()?,
        target_volumes: document.target_volumes.iter().map(|volume| crate::Puzzle5dTargetVolume::from_value(volume.to_value())).collect::<Result<_, _>>()?,
    })
}

/// 👁️ Projects one admitted snapshot for the native editor without decoding an external representation.
pub fn puzzle5d_document_from_snapshot(snapshot: &Puzzle5dSnapshot) -> Result<Puzzle5dDocument, semio_framework_value::ValueError> {
    let mut value = semio_framework_pack_json::from_dsl_value(&snapshot.to_value());
    if let Some(object) = value.as_object_mut() {
        match crate::kind_catalogs_of(&snapshot.kind_catalogs, &snapshot.kind_catalogs_extra) {
            Some(catalogs) => { object.insert("kindCatalogs", semio_framework_pack_json::from_dsl_value(&catalogs.to_value())); }
            None => { object.remove("kindCatalogs"); }
        }
    }
    puzzle5d_record_from_projection(value)
}

fn value_from_document(document: &Puzzle5dDocument) -> Value {
    semio_framework_pack_json::from_dsl_value(&document.to_value())
}

/// 📋️ The concrete kinds one materialized paste consists of: a `create-part` per fresh part, then a `connect-grips`
/// per fresh fastener (whose endpoints already name the fresh parts).
pub fn puzzle5d_paste_mutations(parts: Vec<Puzzle5dPart>, fasteners: Vec<Puzzle5dFastener>) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    let mut mutations = Vec::with_capacity(parts.len() + fasteners.len());
    for part in parts {
        let typed = <crate::Puzzle5dPart as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(&part))?;
        mutations.push(crate::standards::v1::subsets::any::schema::mutations::create_part(typed, None));
    }
    for fastener in fasteners {
        mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(fastener.id, fastener.source, fastener.target, fastener.fastener_kind, fastener.gap, fastener.shift, fastener.rise, fastener.rotation, fastener.turn, fastener.tilt, fastener.x, fastener.y, None));
    }
    Ok(mutations)
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

/// 🧲️ The motion an inspector stepper nudge states — `patchPart{field: x|y|origin.x|y|z, delta}` without an
/// absolute `value` — the inspector edits that are gestures, so they commit the transform tool's
/// `drag-selection2d` (board `x`/`y`) or `drag-selection3d` (world origin) instead of absolute poses. `None` for
/// every other edit.
pub(crate) fn puzzle5d_inspector_nudge(args: Option<&Value>) -> Option<world3d::utilities::transform::Puzzle5dSelectionMotion> {
    use world3d::utilities::transform::Puzzle5dSelectionMotion;
    let args = args?;
    if args.get("value").is_some_and(|value| !value.is_null()) {
        return None;
    }
    let delta = args.get("delta").and_then(Value::as_f64).filter(|delta| delta.is_finite())?;
    match args.get("field").and_then(Value::as_str)? {
        "x" => Some(Puzzle5dSelectionMotion::Board { dx: delta, dy: 0.0 }),
        "y" => Some(Puzzle5dSelectionMotion::Board { dx: 0.0, dy: delta }),
        field => {
            let mut offset = [0.0; 3];
            offset[puzzle5d_axis_index(field, "origin")?] = delta;
            Some(Puzzle5dSelectionMotion::World(semio_s_artifact_puzzle_3d::editor::puzzle3d::modes::edit::windows::main::utilities::transform::Puzzle3dSelectionMotion::Drag { offset }))
        }
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
            let grip_2d: Puzzle5dGrip2d = template.get("2d").and_then(|v| puzzle5d_record_from_projection(v.clone()).ok()).unwrap_or_default();
            let grip_3d: Puzzle5dGrip3d = template.get("3d").and_then(|v| puzzle5d_record_from_projection(v.clone()).ok()).unwrap_or_default();
            Puzzle5dGrip { id: format!("v{index}"), grip_kind, grip_2d, grip_3d }
        })
        .collect()
}

pub use crate::standards::v1::subsets::any::schema::mutations::{quat_from_axis_angle,quat_mul};


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
    args.and_then(|value| value.get("ids")).and_then(|value| <Vec<String> as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value)).ok()).filter(|ids| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
}

pub fn part_scale_json(part: &Puzzle5dPart) -> [f64; 3] {
    match &part.part_3d.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        Some(Value::Number(value)) => {
            let factor = value.as_f64();
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
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        Some(Value::Number(value)) => {
            let factor = value.as_f64();
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

pub use crate::standards::v1::subsets::any::schema::mutations::PUZZLE5D_FLAT_TO_WORLD;

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
    pub referenced: Vec<String>,
}

impl Puzzle5dInteractionSnapshot {
    /// 🕹️ Reads the live `vortex` domain: its selection, its `"pointer"`-channel hover and the ids the open time-travel
    /// draft references (what both panes paint highlighted while a history edit is open).
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
        Self { granularity, selected, hovered: hover.ids.clone(), referenced: interaction.draft_references(PUZZLE5D_INTERACTION_DOMAIN).to_vec() }
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
        Self { granularity, selected, hovered, referenced: Vec::new() }
    }

    /// 🔗️ The ids the board paints highlighted while a time-travel draft references them, as the scene's JSON id array.
    pub fn referenced_json(&self) -> String {
        semio_framework_pack_json::to_json_string(&self.referenced)
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
        semio_framework_pack_json::to_json_string(&self.selected)
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
    let document = <Puzzle5dDocument as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(projection)).unwrap_or_else(|_| empty_document());
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

/// 🧭️ The select/brush/fill/volume-brush interaction mode the world engine reads, derived from the flat
/// active utility (the transform gumball and `worldRelocate` both present as `select`) — puzzle 3d's
/// `main::scene_mode` twin.
pub fn puzzle5d_scene_mode(active_utility: &str) -> &str {
    match active_utility {
        "brush" => "brush",
        "fill" => "fill",
        "volumeBrush" => "volumeBrush",
        _ => "select",
    }
}

/// 🎚️ The gumball handle the world engine draws when the transform utility is active.
pub fn puzzle5d_transform_handle(active_utility: &str) -> Option<&'static str> {
    (active_utility == world3d::utilities::transform::UTILITY_ID).then_some("transform")
}

/// 🧭️ Whether the active utility is the transform gumball.
pub fn puzzle5d_transform_utility_active(active_utility: &str) -> bool {
    puzzle5d_transform_handle(active_utility).is_some()
}

/// 🕹️ Whether the world gumball should render: the transform utility is active, at least one handle
/// flag is on (`setTransformGumballFlag` — an all-off gumball would draw nothing to grab), and the
/// live `vortex` selection holds at least one part or target volume to move. Selection comes from the
/// framework-owned domain via [`Puzzle5dInteractionSnapshot`], never from stored app state.
pub fn puzzle5d_gumball_active(runtime: &Puzzle5dRuntime, active_utility: &str, interaction: &Puzzle5dInteractionSnapshot) -> bool {
    puzzle5d_transform_utility_active(active_utility) && (runtime.transform_move || runtime.transform_rotate) && !(interaction.selected_part_ids().is_empty() && interaction.selected_target_volume_ids().is_empty())
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
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .find(|entry| entry.get("id").and_then(Value::as_str) == Some(kind_id))
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

/// 🪧️ What a history-edit reference chip reads for one entity of the typed document (design §16.4) — the outliner's
/// names over the document itself: a part by its volume label, else its flat text, else its kind id (the catalogue rides
/// in a child store the chip does not open), a grip as `<part> · <grip kind>`, a fastener as `<part> → <part>`. Target
/// volumes carry no authored name, so they keep the framework's `<Kind> <short id>`; so does an id outside `kinds`.
pub fn puzzle5d_entity_label(snapshot: &Puzzle5dSnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
    let wants = |kind: &str| kinds.is_empty() || kinds.iter().any(|declared| declared == kind);
    let text = |value: &Option<String>| value.clone().filter(|text| !text.is_empty());
    let part_label = |part: &crate::Puzzle5dPart| text(&part.part_3d.label).or_else(|| text(&part.part_2d.text)).or_else(|| text(&part.part_kind)).unwrap_or_else(|| part.id.clone());
    let port = |full_id: &str| snapshot.parts.iter().find_map(|part| part.grips.iter().find(|grip| puzzle5d_grip_full_id(&part.id, &grip.id) == full_id).map(|grip| (part, grip)));
    let label = if let Some(part) = wants(PUZZLE5D_GRANULARITY_PART).then(|| snapshot.parts.iter().find(|part| part.id == id)).flatten() {
        Some(part_label(part))
    } else if let Some((part, grip)) = wants(PUZZLE5D_GRANULARITY_GRIP).then(|| port(id)).flatten() {
        Some(format!("{} \u{b7} {}", part_label(part), text(&grip.grip_kind).or_else(|| text(&grip.grip_2d.grip_kind)).unwrap_or_else(|| grip.id.clone())))
    } else if let Some(fastener) = wants(PUZZLE5D_GRANULARITY_FASTENER).then(|| snapshot.fasteners.iter().find(|fastener| fastener.id == id)).flatten() {
        let end = |full_id: &str| port(full_id).map_or_else(|| full_id.to_string(), |(part, _)| part_label(part));
        Some(format!("{} \u{2192} {}", end(&fastener.source), end(&fastener.target)))
    } else {
        None
    };
    label.filter(|label| label != id).map(|label| LocalizedLabel::data(&label))
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
pub fn puzzle5d_inferred_part_kind_rows(document: &Puzzle5dDocument) -> Vec<Value> {
    let mut rows: Vec<Value> = Vec::new();
    for part in &document.parts {
        if part.part_kind.is_empty() || rows.iter().any(|row| row.get("id").and_then(Value::as_str) == Some(part.part_kind.as_str())) {
            continue;
        }
        let rectangle = part.part_2d.shape == "rectangle";
        let size = if rectangle {
            part.part_2d.width.unwrap_or(PUZZLE5D_DEFAULT_PART_RADIUS * 2.0).max(part.part_2d.height.unwrap_or(PUZZLE5D_DEFAULT_PART_RADIUS * 2.0))
        } else {
            (if part.part_2d.radius > 0.0 { part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS }) * 2.0
        };
        let grips: Vec<Value> = part
            .grips
            .iter()
            .map(|grip| {
                semio_framework_pack_json::json!({
                    "gripKind": engine_grip_kind(grip),
                    "angle": grip.grip_2d.angle,
                    "radius": grip.grip_3d.radius,
                    "position": grip.grip_3d.position,
                    "direction": grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]),
                })
            })
            .collect();
        let mut row = semio_framework_pack_json::json!({
            "id": part.part_kind,
            "name": part.part_kind,
            "shape": if rectangle { "rectangle" } else { "circle" },
            "radius": size * 0.5,
            "width": size,
            "height": size,
            "grips": grips,
        });
        if let Some(object) = row.as_object_mut() {
            if let Some(icon) = part.part_2d.icon_kind.as_deref().filter(|icon| !icon.is_empty()) {
                object.insert("iconKind", semio_framework_pack_json::json!(icon));
            }
            if let Some(url) = part.part_3d.mesh_url.as_deref().filter(|url| !url.is_empty()) {
                object.insert("meshUrl", semio_framework_pack_json::json!(url));
            }
        }
        rows.push(row);
    }
    rows
}

/// 🧬️ The read-only kind rows of the grip/fastener/rope slices a catalogue-less document implies —
/// one `{id, name}` per distinct kind the document already names. `ropes` has no document carrier, so
/// it stays empty and its section keeps its placeholder.
pub fn puzzle5d_inferred_kind_rows(document: &Puzzle5dDocument, slice: &str) -> Vec<Value> {
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
    ids.into_iter().map(|id| semio_framework_pack_json::json!({ "id": id, "name": id })).collect()
}
//#endregion 🧬️InferredKinds

//#region 🙈️SelectionFlags
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

fn puzzle5d_job_fault(detail: impl AsRef<str>) -> crate::puzzle_job::JobTurn {
    let bytes = detail.as_ref().as_bytes();
    crate::puzzle_job::JobTurn::Fault(bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)].to_vec())
}

fn puzzle5d_job_checkpoint(stage: u8, cursor: usize, progress: u64) -> crate::puzzle_job::JobTurn {
    let mut state = [0; 17];
    state[0] = stage;
    state[1..9].copy_from_slice(&(cursor as u64).to_le_bytes());
    state[9..17].copy_from_slice(&progress.to_le_bytes());
    crate::puzzle_job::JobTurn::Checkpoint { applied_progress: progress, state: state.to_vec() }
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

fn puzzle5d_import_checkpoint(stage: u8, cursor: usize, nested_cursor: usize, decoded_items: usize, progress: u64) -> crate::puzzle_job::JobTurn {
    crate::puzzle_job::JobTurn::Checkpoint { applied_progress: progress, state: puzzle5d_import_checkpoint_bytes(stage, cursor, nested_cursor, decoded_items, progress).to_vec() }
}

fn puzzle5d_step_envelope(raw: &[u8], cursor: &mut usize, page: &mut [u8; PUZZLE5D_RESERVED_PAGE_BYTES], page_len: &mut usize, progress: &mut u64, cx: &mut StepContext<'_>) -> Option<crate::puzzle_job::JobTurn> {
    if *cursor >= raw.len() {
        *page_len = 0;
        return None;
    }
    let units = raw.len().saturating_sub(*cursor).min(page.len()).min(cx.fuel_remaining() as usize);
    if units == 0 {
        return Some(crate::puzzle_job::JobTurn::Yield);
    }
    let end = cursor.checked_add(units).filter(|end| *end <= raw.len()).expect("Puzzle5d fixed-page ingress preflights the source range before copy");
    page[..units].copy_from_slice(&raw[*cursor..end]);
    *page_len = units;
    *cursor = end;
    *progress = progress.saturating_add(units as u64);
    cx.consume_fuel(units as u64);
    Some(puzzle5d_job_checkpoint(0, *cursor, *progress))
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

/// ♻️ The scan's retained owners, staged whole when its job closes.
#[derive(Default, semio_framework_value::RetireOwned)]
struct Puzzle5dSelectionScanOwners {
    snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    projection: Option<Value>,
    part_ids: HashSet<String>,
    explicit_fastener_ids: HashSet<String>,
    parts: Vec<Puzzle5dPart>,
    fasteners: Vec<Puzzle5dFastener>,
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
    fn into_owners(&mut self) -> Puzzle5dSelectionScanOwners {
        Puzzle5dSelectionScanOwners { snapshot: self.snapshot.take(), projection: self.projection.take(), part_ids: std::mem::take(&mut self.part_ids), explicit_fastener_ids: std::mem::take(&mut self.explicit_fastener_ids), parts: std::mem::take(&mut self.parts), fasteners: std::mem::take(&mut self.fasteners) }
    }

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
            let projection = self.snapshot.as_ref().map(|snapshot| puzzle5d_editor_projection(snapshot))?;
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
                        self.fasteners.push(puzzle5d_record_from_projection(row).map_err(|error| error.to_string())?);
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
                        self.parts.push(puzzle5d_record_from_projection(row).map_err(|error| error.to_string())?);
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
}

enum Puzzle5dClipboardWorkStep {
    Outcome(crate::puzzle_job::JobTurn),
    Pending,
    Complete,
}

impl Puzzle5dClipboardWork {
    fn new(raw: Vec<u8>, snapshot: std::sync::Arc<Puzzle5dPlaySnapshot>, interaction: &semio_framework::InteractionState) -> Self {
        Self {
            raw,
            raw_cursor: 0,
            raw_page: [0; PUZZLE5D_RESERVED_PAGE_BYTES],
            raw_page_len: 0,
            progress: 0,
            stage: Puzzle5dClipboardStage::Envelope,
            scan: Puzzle5dSelectionScan::new(snapshot, interaction),
            encode_cursor: 0,
            dsl_text: String::new(),
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
                self.dsl_text = format!("{{\"schema\":{},\"parts\":[", semio_framework_pack_json::to_json_string(&PUZZLE5D_SCHEMA));
                self.stage = Puzzle5dClipboardStage::EncodeParts;
                self.encode_cursor = 0;
            }
            Puzzle5dClipboardStage::EncodeParts => {
                if let Some(part) = self.scan.parts.get(self.encode_cursor) {
                    if self.encode_cursor != 0 {
                        self.dsl_text.push(',');
                    }
                    self.dsl_text.push_str(&semio_framework_pack_json::to_json_string(part));
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
                    self.dsl_text.push_str(&semio_framework_pack_json::to_json_string(fastener));
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

    fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle5d_job_checkpoint(self.stage as u8, self.encode_cursor.max(self.scan.cursor), self.progress)
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
}

/// 📋️ Which clipboard verb one [`Puzzle5dClipboardJob`] answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dClipboardVerb {
    Copy,
    Cut,
}

/// ♻️ The owners one `Puzzle5dClipboardJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dClipboardOwners {
    raw: Vec<u8>,
    dsl_text: String,
    scan: Puzzle5dSelectionScanOwners,
    completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
}

struct Puzzle5dClipboardJob {
    verb: Puzzle5dClipboardVerb,
    work: Puzzle5dClipboardWork,
    completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
    completed: bool,
    outbox: crate::puzzle_job::JobOutbox,
    closing: bool,
    owners: crate::puzzle_job::WorkClosing<Puzzle5dClipboardOwners>,
}

impl Puzzle5dClipboardJob {
    fn new(verb: Puzzle5dClipboardVerb, request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, interaction: &semio_framework::InteractionState) -> Self {
        Self { verb, work: Puzzle5dClipboardWork::new(request.raw_wire, request.snapshot, interaction), completion: Some(request.completion), rejection: None, completed: false, outbox: Default::default(), closing: false, owners: Default::default() }
    }

    fn label(&self) -> &'static str {
        match self.verb {
            Puzzle5dClipboardVerb::Copy => "puzzle5d copy",
            Puzzle5dClipboardVerb::Cut => "puzzle5d cut",
        }
    }

    fn emit(&self) -> Emit<Puzzle5dMutation, Puzzle5dConfigMutation> {
        let effects = self.work.fragment().map(|fragment| vec![Effect::ClipboardWrite { fragment }]).unwrap_or_default();
        match self.verb {
            Puzzle5dClipboardVerb::Copy => Emit { effects, ..Default::default() },
            Puzzle5dClipboardVerb::Cut => {
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
                Emit { artifact_mutations: mutations, effects, ..Default::default() }
            }
        }
    }

    fn turn(&mut self, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if self.rejection.is_some() {
            return puzzle5d_job_fault(format!("{} completion remains rejected", self.label()));
        }
        if self.completed {
            return crate::puzzle_job::JobTurn::Complete;
        }
        match self.work.step_work(cx) {
            Ok(Puzzle5dClipboardWorkStep::Outcome(outcome)) => return outcome,
            Ok(Puzzle5dClipboardWorkStep::Pending) => return self.work.checkpoint(),
            Err(error) => return puzzle5d_job_fault(error),
            Ok(Puzzle5dClipboardWorkStep::Complete) => {}
        }
        if !self.work.raw.is_empty() {
            return crate::puzzle_job::JobTurn::Prepare(std::mem::take(&mut self.work.raw));
        }
        let emit = self.emit();
        let Some(completion) = self.completion.as_ref() else { return puzzle5d_job_fault(format!("{} lost its completion authority", self.label())) };
        if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
            let message = rejected.fault.message.clone();
            self.rejection = Some(rejected);
            return puzzle5d_job_fault(message);
        }
        self.completed = true;
        crate::puzzle_job::JobTurn::Complete
    }
}

impl InteractiveJob for Puzzle5dClipboardJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = if cx.fuel_exhausted() || cx.deadline_exceeded() { crate::puzzle_job::JobTurn::Yield } else { self.turn(cx) };
        self.outbox.settle(turn, cx)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle5dClipboardOwners {
            raw: std::mem::take(&mut self.work.raw),
            dsl_text: std::mem::take(&mut self.work.dsl_text),
            scan: self.work.scan.into_owners(),
            completion: self.completion.take(),
            rejection: self.rejection.take(),
        });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}

impl ArtifactReservedJob for Puzzle5dClipboardJob {}

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

/// ♻️ The owners one `Puzzle5dPasteJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dPasteOwners {
    raw: Vec<u8>,
    snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    args: Option<Value>,
    fragment_value: Option<Value>,
    fragment_parts: Vec<Puzzle5dPart>,
    id_map: HashMap<String, String>,
    fresh_ids: Puzzle5dFreshIds,
    mutations: Vec<Puzzle5dMutation>,
    completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
}

struct Puzzle5dPasteJob {
    raw: Vec<u8>,
    raw_cursor: usize,
    raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES],
    raw_page_len: usize,
    progress: u64,
    stage: Puzzle5dPasteStage,
    snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    args: Option<Value>,
    fragment_value: Option<Value>,
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
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
    completed: bool,
    outbox: crate::puzzle_job::JobOutbox,
    closing: bool,
    owners: crate::puzzle_job::WorkClosing<Puzzle5dPasteOwners>,
}

impl Puzzle5dPasteJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, args: Option<Value>) -> Self {
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
            completed: false,
            outbox: Default::default(),
            closing: false,
            owners: Default::default(),
        }
    }

    fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle5d_job_checkpoint(self.stage as u8, self.cursor, self.progress)
    }
}

impl Puzzle5dPasteJob {
    fn turn(&mut self, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle5d_job_fault("puzzle5d paste completion remains rejected");
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
                    return self.checkpoint();
                };
                let Some(fragment_value) = args.get("fragment").cloned() else {
                    self.stage = Puzzle5dPasteStage::Complete;
                    return self.checkpoint();
                };
                let fragment: ClipboardFragment = match puzzle5d_record_from_projection(fragment_value) {
                    Ok(fragment) => fragment,
                    Err(error) => return puzzle5d_job_fault(error.to_string()),
                };
                if fragment.media_type != (MediaType { class: MediaClass::Kit, form: MediaForm::Design }) {
                    return puzzle5d_job_fault("puzzle5d paste received an incompatible media type");
                }
                if fragment.dsl_text.len() > PUZZLE5D_RESERVED_RAW_BYTES {
                    return puzzle5d_job_fault("puzzle5d paste fragment exceeds its predecode cap");
                }
                self.fragment_value = match parse(&fragment.dsl_text, semio_framework_pack_json::JsonMemberPolicy::Reject) {
                    Ok(value) => Some(value),
                    Err(error) => return puzzle5d_job_fault(error.to_string()),
                };
                self.placement = match puzzle5d_paste_placement(args) {
                    Ok(placement) => placement,
                    Err(error) => return puzzle5d_job_fault(error.to_string()),
                };
                self.stage = Puzzle5dPasteStage::FragmentParts;
                self.cursor = 0;
            }
            Puzzle5dPasteStage::FragmentParts => {
                let rows = self.fragment_value.as_ref().and_then(|value| value.get("parts")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let part: Puzzle5dPart = match puzzle5d_record_from_projection(row) {
                        Ok(part) => part,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
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
                let rows = self.snapshot.as_ref().map(|snapshot| &snapshot.typed().parts).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor) {
                    self.cursor += 1;
                    self.fresh_ids.observe_part(&row.id);
                    self.target_sum.0 += row.part_2d.x;
                    self.target_sum.1 += row.part_2d.y;
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
                let rows = self.snapshot.as_ref().map(|snapshot| &snapshot.typed().fasteners).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor) {
                    self.cursor += 1;
                    self.fresh_ids.observe_fastener(&row.id);
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
                    let typed = match <crate::Puzzle5dPart as semio_framework_value::FromValue>::from_value(next.to_value()) {
                        Ok(typed) => typed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::create_part(typed, None));
                } else {
                    self.stage = Puzzle5dPasteStage::MaterializeFasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::MaterializeFasteners => {
                let rows = self.fragment_value.as_ref().and_then(|value| value.get("fasteners")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let fastener: Puzzle5dFastener = match puzzle5d_record_from_projection(row) {
                        Ok(fastener) => fastener,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
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
                        fastener.y + self.delta.1, None,
                    ));
                } else {
                    self.stage = Puzzle5dPasteStage::Complete;
                }
            }
            Puzzle5dPasteStage::Complete => {
                if !self.raw.is_empty() {
                    return crate::puzzle_job::JobTurn::Prepare(std::mem::take(&mut self.raw));
                }
                if !self.completed {
                    // 🕹️ The pasted parts BECOME the selection: a paste whose result is invisible until the
                    // user hunts for it is indistinguishable from one that landed nowhere, and the fresh ids
                    // are the only handle on it. Sorted, so one fragment always re-selects in one order.
                    let mut fresh: Vec<String> = self.id_map.values().cloned().collect();
                    fresh.sort();
                    let emit = Emit { artifact_mutations: std::mem::take(&mut self.mutations), interaction_writes: vec![InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, fresh)], ..Default::default() };
                    let Some(completion) = self.completion.as_ref() else { return puzzle5d_job_fault("puzzle5d paste lost its completion authority") };
                    if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(rejected);
                        return puzzle5d_job_fault(message);
                    }
                    self.completed = true;
                }
                return crate::puzzle_job::JobTurn::Complete;
            }
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        self.checkpoint()
    }
}

impl InteractiveJob for Puzzle5dPasteJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = if cx.fuel_exhausted() || cx.deadline_exceeded() { crate::puzzle_job::JobTurn::Yield } else { self.turn(cx) };
        self.outbox.settle(turn, cx)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle5dPasteOwners {
            raw: std::mem::take(&mut self.raw),
            snapshot: self.snapshot.take(),
            args: self.args.take(),
            fragment_value: self.fragment_value.take(),
            fragment_parts: std::mem::take(&mut self.fragment_parts),
            id_map: std::mem::take(&mut self.id_map),
            fresh_ids: std::mem::take(&mut self.fresh_ids),
            mutations: std::mem::take(&mut self.mutations),
            completion: self.completion.take(),
            pending_completion_rejection: self.pending_completion_rejection.take(),
        });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}

impl ArtifactReservedJob for Puzzle5dPasteJob {}
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

/// ♻️ The owners one `Puzzle5dImportJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dImportOwners {
    raw: Vec<u8>,
    port: String,
    media_json: Option<String>,
    snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    fragment: Option<Value>,
    catalogs: crate::Puzzle5dKindCatalogs,
    compatibility: Vec<crate::Puzzle5dKindCompatibility>,
    part_index: Vec<(String, usize)>,
    grip_index: Vec<(String, usize)>,
    compatibility_index: Vec<((String, String), usize)>,
    mutation_pages: [Vec<Puzzle5dMutation>; PUZZLE5D_IMPORT_MUTATION_PAGES],
    current_part: Option<crate::Puzzle5dCatalogPartKind>,
    completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
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
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
    completed: bool,
    outbox: crate::puzzle_job::JobOutbox,
    closing: bool,
    owners: crate::puzzle_job::WorkClosing<Puzzle5dImportOwners>,
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
    parse(media_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

impl Puzzle5dImportJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, port: String, media: Media) -> Self {
        let media_json = match media.payload {
            semio_framework_plugin::MediaPayload::Structured { json, .. } => Some(json),
            semio_framework_plugin::MediaPayload::Intrinsic { value, .. } => Some(semio_framework_pack_json::to_json_string(&value)),
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
            completed: false,
            outbox: Default::default(),
            closing: false,
            owners: Default::default(),
        }
    }

    fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle5d_import_checkpoint(self.stage as u8, self.cursor, self.nested_cursor, self.decoded_items, self.progress)
    }

    fn rows(&self, key: &str) -> &[Value] {
        self.fragment.as_ref().and_then(|value| value.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
    }

    fn snapshot_rows(&self, parent: &str, key: &str) -> Vec<Value> {
        self.snapshot.as_ref().map(|snapshot| puzzle5d_editor_projection(snapshot)).and_then(|projection| projection.get(parent).and_then(|value| value.get(key)).and_then(Value::as_array).cloned()).unwrap_or_default()
    }

    fn snapshot_kind_compatibility_rows(&self) -> Vec<Value> {
        self.snapshot.as_ref().map(|snapshot| puzzle5d_editor_projection(snapshot)).and_then(|projection| projection.get("kindCompatibility").and_then(Value::as_array).cloned()).unwrap_or_default()
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

impl Puzzle5dImportJob {
    fn turn(&mut self, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle5d_job_fault("puzzle5d import completion remains rejected");
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
                    return puzzle5d_job_fault("puzzle5d import only implements kit:in");
                }
                let Some(media_json) = self.media_json.as_ref() else {
                    return puzzle5d_job_fault("puzzle5d kit:in requires a structured payload");
                };
                let fragment = match puzzle5d_decode_import_fragment(media_json) {
                    Ok(fragment) => fragment,
                    Err(error) => return puzzle5d_job_fault(error),
                };
                let fragment_items = ["objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"].into_iter().try_fold(0usize, |total, key| total.checked_add(fragment.get(key).and_then(Value::as_array).map_or(0, Vec::len)));
                let snapshot_items = ["parts", "grips", "fasteners", "ropes"]
                    .into_iter()
                    .try_fold(0usize, |total, key| total.checked_add(self.snapshot_rows("kindCatalogs", key).len()))
                    .and_then(|total| total.checked_add(self.snapshot_kind_compatibility_rows().len()));
                self.decoded_items = match fragment_items.and_then(|fragment_items| snapshot_items.and_then(|snapshot_items| fragment_items.checked_add(snapshot_items))) {
                    Some(items) if items <= PUZZLE5D_IMPORT_DECODED_ITEMS => items,
                    _ => return puzzle5d_job_fault("puzzle5d kit:in decoded item limit exceeded"),
                };
                if fragment.as_object().is_none() {
                    return puzzle5d_job_fault("puzzle5d kit:in root must be an object");
                }
                if !puzzle5d_import_keys_are(&fragment, &["schema", "objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"]) {
                    return puzzle5d_job_fault("puzzle5d kit:in root contains an unknown field");
                }
                if fragment.get("schema").is_some_and(|value| value.as_str() != Some("manifest")) {
                    return puzzle5d_job_fault("puzzle5d kit:in schema must be manifest when present");
                }
                if ["objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"].into_iter().any(|key| fragment.get(key).is_some_and(|value| value.as_array().is_none())) {
                    return puzzle5d_job_fault("puzzle5d kit:in collection must be an array");
                }
                if ["objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"].into_iter().any(|key| fragment.get(key).and_then(Value::as_array).is_some_and(|rows| rows.len() > PUZZLE5D_IMPORT_SEMANTIC_ITEMS)) {
                    return puzzle5d_job_fault("puzzle5d kit:in collection exceeds its fixed-page descriptor cap");
                }
                if ["cableKinds", "attractionKinds"].into_iter().any(|key| fragment.get(key).and_then(Value::as_array).is_some_and(|rows| !rows.is_empty())) {
                    return puzzle5d_job_fault("puzzle5d kit:in cannot silently discard unmapped cable or attraction kinds");
                }
                self.had_catalogs = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.typed().kind_catalogs.is_some());
                self.catalog_changed = !self.had_catalogs;
                self.fragment = Some(fragment);
                self.stage = Puzzle5dImportStage::CensusParts;
                self.cursor = 0;
            }
            Puzzle5dImportStage::CensusParts => {
                if let Some(row) = self.rows("objectKinds").get(self.cursor) {
                    if !puzzle5d_import_keys_are(row, &["id", "name", "label", "meshUrl", "vortices"]) {
                        return puzzle5d_job_fault("puzzle5d kit:in object kind contains an unknown field");
                    }
                    if row.get("vortices").is_some_and(|value| value.as_array().is_none()) {
                        return puzzle5d_job_fault("puzzle5d kit:in object-kind vortices must be an array");
                    }
                    let vortices = row.get("vortices").and_then(Value::as_array).map_or(0, Vec::len);
                    if vortices > PUZZLE5D_IMPORT_SEMANTIC_ITEMS {
                        return puzzle5d_job_fault("puzzle5d kit:in vortex collection exceeds its fixed-page descriptor cap");
                    }
                    self.decoded_items = match self.decoded_items.checked_add(vortices) {
                        Some(items) if items <= PUZZLE5D_IMPORT_DECODED_ITEMS => items,
                        _ => return puzzle5d_job_fault("puzzle5d kit:in nested vortex item limit exceeded"),
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
                    return puzzle5d_job_fault("puzzle5d kit:in part catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCatalogGrips;
            }
            Puzzle5dImportStage::ReserveCatalogGrips => {
                let capacity = self.snapshot_rows("kindCatalogs", "grips").len().saturating_add(self.rows("vortexKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.grips.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in grip catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCatalogFasteners;
            }
            Puzzle5dImportStage::ReserveCatalogFasteners => {
                let capacity = self.snapshot_rows("kindCatalogs", "fasteners").len();
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.fasteners.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in fastener catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCatalogRopes;
            }
            Puzzle5dImportStage::ReserveCatalogRopes => {
                let capacity = self.snapshot_rows("kindCatalogs", "ropes").len();
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.ropes.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in rope catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCompatibility;
            }
            Puzzle5dImportStage::ReserveCompatibility => {
                let capacity = self.snapshot_kind_compatibility_rows().len().saturating_add(self.rows("kindCompatibility").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.compatibility.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in compatibility reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReservePartIndex;
            }
            Puzzle5dImportStage::ReservePartIndex => {
                let capacity = self.snapshot_rows("kindCatalogs", "parts").len().saturating_add(self.rows("objectKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.part_index.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in part index reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveGripIndex;
            }
            Puzzle5dImportStage::ReserveGripIndex => {
                let capacity = self.snapshot_rows("kindCatalogs", "grips").len().saturating_add(self.rows("vortexKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.grip_index.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in grip index reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCompatibilityIndex;
            }
            Puzzle5dImportStage::ReserveCompatibilityIndex => {
                let capacity = self.snapshot_kind_compatibility_rows().len().saturating_add(self.rows("kindCompatibility").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.compatibility_index.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in compatibility index reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveMutations;
                self.nested_cursor = 0;
            }
            Puzzle5dImportStage::ReserveMutations => {
                let capacity = self.rows("kindCompatibility").len().saturating_mul(2).saturating_add(1);
                if capacity > PUZZLE5D_IMPORT_MUTATION_ITEMS {
                    return puzzle5d_job_fault("puzzle5d kit:in mutation reserve rejected");
                }
                let page_start = self.nested_cursor.saturating_mul(PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);
                if page_start < capacity {
                    let page_items = capacity.saturating_sub(page_start).min(PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);
                    let Some(page) = self.mutation_pages.get_mut(self.nested_cursor) else {
                        return puzzle5d_job_fault("puzzle5d kit:in mutation page reserve rejected");
                    };
                    if page.try_reserve_exact(page_items).is_err() {
                        return puzzle5d_job_fault("puzzle5d kit:in mutation page reserve rejected");
                    }
                    self.nested_cursor += 1;
                    return self.checkpoint();
                }
                self.stage = Puzzle5dImportStage::LoadCatalogParts;
                self.cursor = 0;
                self.nested_cursor = 0;
            }
            Puzzle5dImportStage::LoadCatalogParts => {
                if let Some(row) = self.snapshot_rows("kindCatalogs", "parts").get(self.cursor) {
                    let parsed = match <crate::Puzzle5dCatalogPartKind as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
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
                    let parsed = match <crate::Puzzle5dCatalogGripKind as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
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
                    let parsed = match <crate::Puzzle5dCatalogFastenerKind as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
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
                    let parsed = match <crate::Puzzle5dCatalogRopeKind as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
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
                    let parsed = match <crate::Puzzle5dKindCompatibility as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
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
                        None => return puzzle5d_job_fault("puzzle5d kit:in object kind lacks id"),
                    };
                    let name = match row.get("name").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in object kind lacks name"),
                    };
                    let label = match row.get("label").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in object kind lacks label"),
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
                    return puzzle5d_job_fault("puzzle5d kit:in lost its part before nested reserve");
                };
                if grip_count > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || part.grips.try_reserve_exact(grip_count).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in nested grip reserve rejected");
                }
                self.stage = Puzzle5dImportStage::PartVortices;
            }
            Puzzle5dImportStage::PartVortices => {
                let vortex = self.rows("objectKinds").get(self.cursor).and_then(|row| row.get("vortices")).and_then(Value::as_array).and_then(|rows| rows.get(self.nested_cursor));
                if let Some(vortex) = vortex {
                    if !puzzle5d_import_keys_are(vortex, &["id", "vortexKind", "position", "direction", "radius"]) {
                        return puzzle5d_job_fault("puzzle5d kit:in vortex contains an unknown field");
                    }
                    let vortex_kind = match vortex.get("vortexKind").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in vortex lacks kind"),
                    };
                    let point = match puzzle5d_import_vec3(vortex.get("position")) {
                        Ok(value) => value,
                        Err(error) => return puzzle5d_job_fault(error),
                    };
                    let direction = match puzzle5d_import_vec3(vortex.get("direction")) {
                        Ok(value) => value,
                        Err(error) => return puzzle5d_job_fault(error),
                    };
                    let radius = match vortex.get("radius").and_then(Value::as_f64).filter(|value| value.is_finite()) {
                        Some(value) => value,
                        None => return puzzle5d_job_fault("puzzle5d kit:in vortex lacks finite radius"),
                    };
                    let Some(part) = self.current_part.as_mut() else {
                        return puzzle5d_job_fault("puzzle5d kit:in lost its current part owner");
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
                    return puzzle5d_job_fault("puzzle5d kit:in lost its completed part owner");
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
                        return puzzle5d_job_fault("puzzle5d kit:in vortex kind contains an unknown field");
                    }
                    let id = match row.get("id").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in vortex kind lacks id"),
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
                        return puzzle5d_job_fault("puzzle5d kit:in compatibility contains an unknown field");
                    }
                    let parsed = match <crate::Puzzle5dKindCompatibility as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    let key = (parsed.source.clone(), parsed.target.clone());
                    match self.compatibility_index.iter().find_map(|(candidate, index)| (candidate == &key).then_some(*index)) {
                        Some(index) if self.compatibility[index] == parsed => {}
                        Some(index) => {
                            if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility(parsed.source.clone(), parsed.target.clone())) {
                                return puzzle5d_job_fault(error);
                            }
                            if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(parsed.source.clone(), parsed.target.clone(), parsed.bidirectional, parsed.important, parsed.specificity, None)) {
                                return puzzle5d_job_fault(error);
                            }
                            self.compatibility[index] = parsed;
                        }
                        None => {
                            if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(parsed.source.clone(), parsed.target.clone(), parsed.bidirectional, parsed.important, parsed.specificity, None)) {
                                return puzzle5d_job_fault(error);
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
                        return puzzle5d_job_fault(error);
                    }
                    self.catalog_changed = false;
                }
                self.stage = Puzzle5dImportStage::Complete;
            }
            Puzzle5dImportStage::Complete => {
                if !self.raw.is_empty() {
                    return crate::puzzle_job::JobTurn::Prepare(std::mem::take(&mut self.raw));
                }
                if !self.completed {
                    let mut mutation_pages = std::mem::take(&mut self.mutation_pages);
                    let mut mutations = std::mem::take(&mut mutation_pages[0]);
                    for page in mutation_pages.iter_mut().skip(1) {
                        mutations.append(page);
                    }
                    let Some(completion) = self.completion.as_ref() else { return puzzle5d_job_fault("puzzle5d import lost its completion authority") };
                    if let Err(rejected) = completion.complete(Ok(Emit::mutations(mutations)), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(rejected);
                        return puzzle5d_job_fault(message);
                    }
                    self.completed = true;
                }
                return crate::puzzle_job::JobTurn::Complete;
            }
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        self.checkpoint()
    }
}

impl InteractiveJob for Puzzle5dImportJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = if cx.fuel_exhausted() || cx.deadline_exceeded() { crate::puzzle_job::JobTurn::Yield } else { self.turn(cx) };
        self.outbox.settle(turn, cx)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle5dImportOwners {
            raw: std::mem::take(&mut self.raw),
            port: std::mem::take(&mut self.port),
            media_json: self.media_json.take(),
            snapshot: self.snapshot.take(),
            fragment: self.fragment.take(),
            catalogs: std::mem::take(&mut self.catalogs),
            compatibility: std::mem::take(&mut self.compatibility),
            part_index: std::mem::take(&mut self.part_index),
            grip_index: std::mem::take(&mut self.grip_index),
            compatibility_index: std::mem::take(&mut self.compatibility_index),
            mutation_pages: std::mem::take(&mut self.mutation_pages),
            current_part: self.current_part.take(),
            completion: self.completion.take(),
            pending_completion_rejection: self.pending_completion_rejection.take(),
        });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}

impl ArtifactReservedJob for Puzzle5dImportJob {}
//#endregion 🧵️ReservedJobs

//#region 🔖️ContextMenu
/// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: `duplicateSelection`/`selectSameKindSelection`/
/// `focusSelection` stay top-level verbs; the hide/lock toggles (bespoke rows — their label/icon flip
/// on selection state, so they can't resolve from a single static `ActionDefinition`) fold into a
/// `settings` group; `deleteSelection` (bespoke label carrying the selection-count phrase) stays the
/// trailing destructive row. `organize_context_menu`, run automatically at the `VcsArtifactApp::context_menu`
/// funnel, handles taxonomy ordering/separator placement — this function only needs to emit the rows.
/// 🕹️ The per-granularity ids one context-menu request carries. `surface.selection` is what the
/// document holds selected; `surface.hits` is the entity the pointer is actually over. A hit OUTSIDE
/// the selection is the menu's whole subject — a right-click on the grip of a selected part opens that
/// grip's menu, never the part's — while a hit inside the selection keeps the whole selection, so
/// "Delete (3 parts)" never silently narrows to the one row under the cursor.
#[derive(Default)]
pub struct Puzzle5dContextSelection {
    pub part_ids: Vec<String>,
    pub grip_ids: Vec<String>,
    pub fastener_ids: Vec<String>,
    /// 🎯️ The pointer's hit alone is the subject, so no selection read may widen it again.
    hit_subject: bool,
}

impl Puzzle5dContextSelection {
    /// 🪣️ The bucket one surface domain name belongs to — the board host's `node`/`handle`/`edge` and the world
    /// host's `object`/`vortex`/`attraction` pick domains for what this app calls a part, grip and fastener.
    fn bucket(&mut self, domain: &str) -> Option<&mut Vec<String>> {
        match domain {
            "node" | "object" | PUZZLE5D_GRANULARITY_PART => Some(&mut self.part_ids),
            "handle" | "vortex" | PUZZLE5D_GRANULARITY_GRIP => Some(&mut self.grip_ids),
            "edge" | "attraction" | PUZZLE5D_GRANULARITY_FASTENER => Some(&mut self.fastener_ids),
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
        // 🎯️ Hosts list every target under the pointer, most specific first (a grip before its part), and only
        // that one is the menu's subject.
        let mut hits = Self::default();
        if let Some(hit) = surface.hits.iter().find(|hit| hits.bucket(hit.domain.as_str()).is_some()) {
            hits.bucket(hit.domain.as_str()).into_iter().for_each(|bucket| bucket.push(hit.id.clone()));
        }
        let hit_selected = |selected: &[String], hit: &[String]| hit.iter().all(|id| selected.contains(id));
        let inside = hit_selected(&out.part_ids, &hits.part_ids) && hit_selected(&out.grip_ids, &hits.grip_ids) && hit_selected(&out.fastener_ids, &hits.fastener_ids);
        if hits.is_empty() || inside {
            return out;
        }
        hits.hit_subject = true;
        hits
    }

    /// 🕹️ Fills in the granularities the CLIENT surface never sends — puzzle 3d's twin. `World3dHost` only puts
    /// its painted part ids into `ContextMenuSurfaceTarget.selection`, so a selected grip or fastener reaches a
    /// menu only through the framework-owned domain read. Per-granularity additive: whatever the surface DID
    /// supply keeps priority (a right-click on an unselected entity still targets what was clicked).
    pub fn fill_from_interaction(&mut self, interaction: &Puzzle5dInteractionSnapshot) {
        if self.hit_subject {
            return;
        }
        for (bucket, ids) in [(&mut self.part_ids, interaction.selected_part_ids()), (&mut self.grip_ids, interaction.selected_grip_ids()), (&mut self.fastener_ids, interaction.selected_fastener_ids())] {
            if bucket.is_empty() {
                bucket.extend(ids.iter().cloned());
            }
        }
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
    view_state: &semio_framework_plugin::ViewModel,
    registry: &semio_framework_plugin::AppActionRegistry,
) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, ContextMenuItemSpec, Menu, SelectionKind};
    let bespoke = |id: &str, label: String, icon: &str, action: &str, args: Option<Value>, destructive: bool| ContextMenuItemSpec {
        id: id.into(),
        label: Some(label),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args: args.map(|value| semio_framework_pack_json::to_dsl_value(&value)),
        destructive: destructive.then_some(true),
        ..Default::default()
    };
    if selection.is_empty() {
        return Menu::of(registry, view_state)
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
        let phrase = selection_count_phrase(view_state.locale, &[(part_ids.len(), SelectionKind::Part)]).unwrap_or_default();
        return Menu::of(registry, view_state)
            .action("duplicateSelection")
            .item(bespoke("copy", labels.copy.into(), "copy", "copy", None, false))
            .item(bespoke("cut", labels.cut.into(), "scissors", "cut", None, false))
            .action("selectSameKindSelection")
            .action("focusSelection")
            .group("settings", |m| {
                m.item(bespoke("hide-show", if all_hidden { labels.show.into() } else { labels.hide.into() }, if all_hidden { "eye" } else { "eye-off" }, "setSelectionFlag", Some(semio_framework_pack_json::json!({ "flag": "hidden", "value": !all_hidden })), false))
                    .item(bespoke("lock-unlock", if all_locked { labels.unlock.into() } else { labels.lock.into() }, if all_locked { "lock-open" } else { "lock" }, "setSelectionFlag", Some(semio_framework_pack_json::json!({ "flag": "locked", "value": !all_locked })), false))
            })
            .item(bespoke("delete", format!("{} ({phrase})", labels.delete.as_str()), "trash", "deleteSelection", None, true))
            .build();
    }
    if !selection.grip_ids.is_empty() {
        let mut menu = Menu::of(registry, view_state);
        // 🎣️ Only for EXACTLY one grip: the suggestion search points at a single grip. The row carries the
        // world host's `openVortexSuggestions` verb, which the host turns into a live submenu: it starts the
        // search the moment the menu opens and lists the free parts as they are found.
        if let [only] = selection.grip_ids.as_slice() {
            menu = menu.item(bespoke("suggest", labels.suggest_parts.into(), "sparkles", "openVortexSuggestions", Some(semio_framework_pack_json::json!({ "fullId": only.as_str() })), false));
        }
        return menu.action("focusSelection").item(bespoke("delete", labels.delete.into(), "trash", "deleteSelection", None, true)).build();
    }
    // 🔗️ A fastener row carries its own id. `retargetFastener` is deliberately NOT offered here: it
    // needs a REPLACEMENT grip a context menu cannot name, and dispatching it with an id alone is an
    // early return — a visibly dead row. Retargeting lives in the inspector, which has both grips.
    let Some(id) = selection.fastener_ids.first() else { return Vec::new() };
    Menu::of(registry, view_state).item(bespoke("delete", labels.delete.into(), "trash", "deleteFastener", Some(semio_framework_pack_json::json!({ "id": id.as_str() })), true)).build()
}
//#endregion 🔖️ContextMenu

//#region 🔖️Puzzle5dCommand
/// 🎯️ B1: `Puzzle5dPlayApp::Command` — the SOLE dispatch surface, one variant per declared
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
        #[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
        pub enum Puzzle5dCommand {
            $($Variant { window_id: Option<String>, args: Option<semio_framework_pack_json::Value> }),*
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

            fn args(&self) -> Option<&semio_framework_pack_json::Value> {
                match self {
                    $(Puzzle5dCommand::$Variant { args, .. } => args.as_ref()),*
                }
            }

            fn try_from_action(action: &str, args: Option<semio_framework_pack_json::Value>, window_id: Option<String>) -> Option<Self> {
                match action {
                    $($id => Some(Puzzle5dCommand::$Variant { window_id, args })),*,
                    _ => None,
                }
            }

            #[cfg(test)]
            fn from_action(action: &str, args: Option<semio_framework_pack_json::Value>, window_id: Option<String>) -> Self {
                Self::try_from_action(action, args, window_id)
                    .unwrap_or_else(|| panic!("unknown puzzle5d action id in test: {action}"))
            }

            /// 🪶️ Hand-written JSON bridge (see this macro's own doc comment on why `OpBinary` here
            /// is a plain JSON-bytes bridge, not a derive): reproduces serde's default externally
            /// tagged struct-variant shape (`{"VariantName": {"window_id": ..., "args": ...}}`) so
            /// `encode_op`/`decode_op` stay byte-for-byte compatible with the pre-migration wire.
            fn to_json(&self) -> semio_framework_pack_json::Value {
                match self {
                    $(Puzzle5dCommand::$Variant { window_id, args } => semio_framework_pack_json::object([(
                        stringify!($Variant).to_string(),
                        semio_framework_pack_json::object([("window_id".to_string(), semio_framework_pack_json::Value::from(window_id.clone())), ("args".to_string(), args.clone().unwrap_or(semio_framework_pack_json::Value::Null))]),
                    )])),*
                }
            }

            fn from_json(value: &semio_framework_pack_json::Value) -> Option<Self> {
                let entries = value.as_object()?;
                if entries.len() != 1 {
                    return None;
                }
                let (tag, payload) = entries.iter().next()?;
                let window_id = payload.get("window_id").and_then(semio_framework_pack_json::Value::as_str).map(str::to_string);
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
    ExportSnapshot = "exportSnapshot",
    ImportSnapshot = "importSnapshot",
    OpenImportSnapshot = "openImportSnapshot",
    OpenAddPartDialog = "openAddPartDialog",
    SetActiveExample = "setActiveExample",
    AddNode = "addNode",
    AddPartKind = "addPartKind",
    AddBrushPart = "addBrushPart",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    SetSelectionFlag = "setSelectionFlag",
    SetSelectionHidden = "setSelectionHidden",
    SetSelectionLocked = "setSelectionLocked",
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
    OpenVortexSuggestions = "openVortexSuggestions",
    CloseVortexSuggestions = "closeVortexSuggestions",
    HoverSuggestion = "hoverSuggestion",
    AcceptSuggestion = "acceptSuggestion",
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
    SetTargetVolumeHidden = "setTargetVolumeHidden",
    SetTargetVolumeLocked = "setTargetVolumeLocked",
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
        Ok(semio_framework_pack_json::to_string(&self.to_json()).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        let value = parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error.into_value_error())))?;
        Self::from_json(&value).ok_or_else(|| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unrecognized Puzzle5dCommand tag"))))
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
    /// 🌱️ The admission's authoring seed every tool transaction this action commits is minted from; empty for
    /// a reduction no admission seeded, whose transaction then stays unstamped.
    pub authoring_seed: &'a str,
    /// 🛠️ The parametric mutations the transform tool yielded for this action, published ahead of the scene delta.
    pub artifact_mutations: Vec<Puzzle5dMutation>,
    /// 🛠️ The tool transaction this action committed — stamped on the ONE edit it publishes.
    pub transaction: Option<protocol::TransactionRef>,
}

impl<'a> Puzzle5dActionCtx<'a> {
    /// 🧲️ One gumball pose delta (`translateSelection`/`rotateSelection`/`scaleSelection`) as ONE tool
    /// transaction over the gesture's own part ids (else the selected parts) and the selected target volumes.
    pub fn commit_gumball(&mut self, verb: &str, args: Option<&Value>) {
        let targets = [mesh_selection_ids(args, &self.selected_part_ids()), self.selected_ids(PUZZLE5D_GRANULARITY_TARGET_VOLUME)].concat();
        if let Some(record) = semio_s_artifact_puzzle_3d::editor::puzzle3d::modes::edit::windows::main::utilities::transform::Puzzle3dSelectionRecord::from_gumball(verb, args, targets) {
            self.commit_selection(verb, vec![world3d::utilities::transform::Puzzle5dSelectionRecord::world(record)]);
        }
    }

    /// 🛠️ Commits `records` through the transform tool machine as ONE tool transaction of this action — the
    /// parametric selection leaves plus the fasteners their drops land, yielded as `verb`. Nothing named, or
    /// nothing named that exists, is the `nothing_selected` refusal; a request whose every target is locked is
    /// the `selection_locked` one; a request with a movable target is yielded whole, and its leaf reports the
    /// locked rest as `mutation.partial`. A motionless request leaves zero trace.
    pub fn commit_selection(&mut self, verb: &str, records: Vec<world3d::utilities::transform::Puzzle5dSelectionRecord>) {
        let base = self.snapshot.typed_arc();
        if records.iter().all(|record| !record.names_any(&base)) {
            self.refuse_without_selection(&[]);
            return;
        }
        if records.iter().any(|record| record.refused_as_locked(&base)) && !records.iter().any(|record| record.applies_to(&base)) {
            self.notice(|labels| labels.selection_locked.as_str());
            self.abort = true;
            return;
        }
        let request = world3d::utilities::transform::TransformToolRequest { base, records };
        if let Some((transaction, mutations)) = world3d::utilities::transform::puzzle5d_transform_tool_commit(verb, self.authoring_seed, request) {
            self.transaction = (!self.authoring_seed.is_empty()).then_some(transaction);
            self.artifact_mutations.extend(mutations);
        }
    }

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
    let document: Puzzle5dDocument = puzzle5d_document_from_snapshot(snapshot.typed()).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    let (parts, fasteners) = copy_selection_local(&document, part_ids, fastener_ids);
    if parts.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let fragment_value = semio_framework_pack_json::json!({ "schema": PUZZLE5D_SCHEMA, "parts": parts, "fasteners": fasteners });
    Ok(ClipboardFragment {
        schema: PUZZLE5D_SCHEMA.to_string(),
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design },
        dsl_text: semio_framework_pack_json::to_string_pretty(&fragment_value),
        pack_bytes: None,
        source_app: PUZZLE5D_PLAY_APP_ID.to_string(),
        label: format!("{} part(s)", parts.len()),
    })
}

/// ✂️ The document removal of the selected parts and fasteners.
pub fn puzzle5d_cut_operations(snapshot: &Puzzle5dPlaySnapshot, part_ids: &[String], fastener_ids: &[String]) -> Vec<Puzzle5dMutation> {
    let before = puzzle5d_editor_projection(snapshot);
    let Ok(document) = <Puzzle5dDocument as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&before)) else {
        return Vec::new();
    };
    let (parts, fasteners) = copy_selection_local(&document, part_ids, fastener_ids);
    if parts.is_empty() {
        return Vec::new();
    }
    // 🔒️ A LOCKED part is copied but never removed — a lock exists precisely to refuse a destructive
    // gesture — and the fasteners of a surviving part survive with it, or the document is left half-cut.
    // `Puzzle5dClipboardJob` applies the identical rule on the retained route; the two must never disagree.
    let locked: HashSet<&str> = parts.iter().filter(|part| part.part_2d.locked.unwrap_or(false)).map(|part| part.id.as_str()).collect();
    fasteners
        .iter()
        .filter(|fastener| !locked.contains(owning_part_id_local(&fastener.source)) && !locked.contains(owning_part_id_local(&fastener.target)))
        .map(|fastener| crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(fastener.id.clone()))
        .chain(parts.iter().filter(|part| !locked.contains(part.id.as_str())).map(|part| crate::standards::v1::subsets::any::schema::mutations::delete_part(part.id.clone())))
        .collect()
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
    /// 🧩️ B1: the pure per-action core, dispatched into by `ArtifactApp::handle` with
    /// `action`/`args`/`window_id` reconstructed 1:1 from the typed `Puzzle5dCommand`. Everything past
    /// this adapter boundary reads/writes the passed-in `Puzzle5dConfig` snapshot and returns a real
    /// `Emit` (document + config operations) instead of mutating `self`.
    fn handle_action_impl(
        &self,
        authoring_seed: &str,
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
        let projection = puzzle5d_editor_projection(snapshot);
        let shared_before = window_ownership::shared(config);
        let window_before = window_ownership::config_from_runtime(config);
        let transient_before = window_ownership::transient_from_runtime(config, window_id.unwrap_or(world3d::WINDOW_KIND_ID));
        let active_utility_initial = puzzle5d_scene_active_utility(view_state, window_id);
        let wid = window_id.map_or_else(|| world3d::WINDOW_KIND_ID.to_string(), str::to_string);
        let window_kind = view_state.and_then(window_ownership::kind_for_view).unwrap_or(world3d::WINDOW_KIND_ID);
        let mut scene = scene_from_projection(&projection, config.clone(), &active_utility_initial);
        let mut ctx = Puzzle5dActionCtx {
            scene: &mut scene,
            snapshot,
            instance_owner,
            window_id: &wid,
            window_kind,
            selection,
            view_state,
            tool_run,
            effects: Vec::new(),
            interaction_writes: Vec::new(),
            abort: false,
            authoring_seed,
            artifact_mutations: Vec::new(),
            transaction: None,
        };
        dispatch_puzzle5d_action(&mut ctx, action, args);
        let aborted = ctx.abort;
        let mut arm_effects = std::mem::take(&mut ctx.effects);
        let interaction_writes = std::mem::take(&mut ctx.interaction_writes);
        let tool_mutations = std::mem::take(&mut ctx.artifact_mutations);
        let transaction = ctx.transaction.take();
        if aborted {
            // 🧯️ An aborted arm emits no document/config delta — but the refusal NOTICE it pushed is the
            // user-visible half of that refusal and must survive, or "refused" and "silently did nothing"
            // look identical. Effects travel on their own lane, so keeping them costs no mutation and no
            // undo entry; the scope stays `None` because nothing was painted.
            return (Emit { effects: arm_effects, ui_scope: UiDirtyScope::None, ..Default::default() }, EphemeralEmit::default());
        }
        let next_active_utility = scene.active_utility.clone();
        let operations: Vec<Puzzle5dMutation> = tool_mutations;
        // 🛠️ A committed transform-tool transaction stamps every op of this ONE edit.
        let transaction = transaction.filter(|_| !operations.is_empty());
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
        let config_mutations = shared_before.mutations_to(&shared_after);
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
        (Emit { artifact_mutations: operations, config_mutations, window_config_mutations, transaction, effects, interaction_writes, ..Default::default() }, EphemeralEmit { window_transient, ..Default::default() })
    }
}

impl Puzzle5dPlayApp {
    /// 🖱️ The ONE context-menu implementation — `ArtifactEditor::context_menu` and
    /// `context_menu_with_request_context` funnel here, differing only in whether `interaction` carries a live
    /// `vortex`-domain read or the empty default.
    fn context_menu_body(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &Puzzle5dInteractionSnapshot,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let projection = puzzle5d_editor_projection(&doc.snapshot);
        let Some(labels) = puzzle5d_labels(view_state) else { return Vec::new() };
        let window_id = view_state.window_id.as_deref().unwrap_or(world3d::WINDOW_KIND_ID);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        let mut selection = Puzzle5dContextSelection::from_surface(request.surface.as_ref());
        selection.fill_from_interaction(interaction);
        puzzle5d_context_menu_items(&envelope, &selection, labels, view_state, registry)
    }
}

/// 🎬️ Dispatch only: every arm's behaviour lives in its `🎮️commands/<group>/🦀️.rs` free
/// function. No behaviour lives in this match.
fn dispatch_puzzle5d_action(ctx: &mut Puzzle5dActionCtx<'_>, action: &str, args: Option<&Value>) {
    match action {
        "importSnapshot" => import_snapshot::import_snapshot(ctx, args),
        "openImportSnapshot" => open_import_snapshot::open_import_snapshot(ctx),
        "openAddPartDialog" => open_add_part_dialog::open_add_part_dialog(ctx),
        "selectSameKindSelection" => select_same_kind::select_same_kind(ctx),
        "deleteSelection" => delete_selection::delete_selection(ctx),
        "duplicateSelection" => duplicate_selection::duplicate_selection(ctx),
        "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),
        "setSelectionHidden" => set_selection_flag::set_selection_flag_value(ctx, args, "hidden"),
        "setSelectionLocked" => set_selection_flag::set_selection_flag_value(ctx, args, "locked"),
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
        "cycleBrushCandidate" => cycle_brush_candidate::cycle_brush_candidate(ctx),
        "cycleBrushCandidateBack" => cycle_brush_candidate::cycle_brush_candidate_back(ctx),
        "targetBrushSuggestions" => target_brush_suggestions::target_brush_suggestions(ctx, args),
        "openVortexSuggestions" => open_vortex_suggestions::open_vortex_suggestions(ctx, args),
        "closeVortexSuggestions" => close_vortex_suggestions::close_vortex_suggestions(ctx, args),
        "hoverSuggestion" => hover_suggestion::hover_suggestion(ctx, args),
        "acceptSuggestion" => accept_suggestion::accept_suggestion(ctx, args),
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
        "setTargetVolumeHidden" => set_target_volume_flag::set_target_volume_flag_value(ctx, args, "hidden"),
        "setTargetVolumeLocked" => set_target_volume_flag::set_target_volume_flag_value(ctx, args, "locked"),
        "setVoxelDims" => set_voxel_dims::set_voxel_dims(ctx, args),
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
    "setTargetVolumeHidden",
    "setTargetVolumeLocked",
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
    "exportSnapshot",
    "importSnapshot",
    "openAddPartDialog",
    "openImportSnapshot",
    "selectSameKindSelection",
    "setFillCount",
    "setSelectionFlag",
    "setSelectionHidden",
    "setSelectionLocked",
    "targetBrushSuggestions",
    "openVortexSuggestions",
    "closeVortexSuggestions",
    "hoverSuggestion",
    "acceptSuggestion",
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
    "openVortexSuggestions",
    "closeVortexSuggestions",
    "hoverSuggestion",
    "acceptSuggestion",
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
    let projection = puzzle5d_editor_projection(snapshot);
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
    Ok(with_puzzle5d_app(|app| app.handle_action_impl("", command.action_id(), command.args(), command.window_id(), snapshot, &runtime, view_state, selection, None, None).0))
}

/// 🎲️ The nonce one operation's minted part and fastener ids derive from.
fn puzzle5d_operation_nonce(operation: &Operation) -> u64 {
    operation.operation.0 ^ operation.generation.0.rotate_left(17) ^ operation.seed.rotate_left(31)
}

/// ♻️ The owners one `Puzzle5dWindowCommandWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dWindowWorkOwners {
    instance_owner: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    tool_run: Option<semio_framework_plugin::ToolRunView>,
    authoring_seed: String,
}

struct Puzzle5dWindowCommandWork {
    tool_id: &'static str,
    consumed: bool,
    closing: bool,
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dWindowWorkOwners>,
    instance_owner: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    tool_run: Option<semio_framework_plugin::ToolRunView>,
    /// 🌱️ The admission's authoring seed a typed `move`/`rotate`/`scale` submit mints its tool transaction from.
    authoring_seed: String,
}

impl Puzzle5dWindowCommandWork {
    fn new(tool_id: &'static str, authoring_seed: String) -> Self {
        Self { tool_id, consumed: false, closing: false, close_owners: Default::default(), instance_owner: None, tool_run: None, authoring_seed }
    }

    /// 🪪️ Binds the admission's retained operation owner.
    fn bound(mut self, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        self.instance_owner = Some(owner);
        self
    }

    /// ⏯️ Binds the instance's tool run as of admission — the identity Escape's `toolRunAbort` needs.
    fn with_tool_run(mut self, tool_run: Option<semio_framework_plugin::ToolRunView>) -> Self {
        self.tool_run = tool_run;
        self
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dWindowCommandWork {
    fn tool_id(&self) -> &'static str { self.tool_id }
    fn extent(&self, _command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> { Some(1) }
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let window_transient_snapshot = context.and_then(|context| context.window_transient.as_ref());
        if self.consumed { return Err(Fault::from("puzzle5d-window-work-repeated")); }
        let view = view_state.ok_or_else(|| Fault::from("puzzle5d-window-context-required"))?;
        let window_id = view.window_id.as_deref().or_else(|| command.window_id()).ok_or_else(|| Fault::from("puzzle5d-window-id-required"))?;
        let window_config = window_ownership::config_from_snapshot(window_config_snapshot);
        let window_transient = window_ownership::transient_from_snapshot(window_transient_snapshot);
        let runtime = window_ownership::runtime(config, &window_config, &window_transient, window_id);
        let empty_selection = protocol::DomainSelection::default();
        let selection = interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).unwrap_or(&empty_selection);
        let (emit, ephemeral) = with_puzzle5d_app(|app| app.handle_action_impl(&self.authoring_seed, command.action_id(), command.args(), Some(window_id), snapshot, &runtime, Some(view), selection, self.instance_owner.as_ref(), self.tool_run.as_ref()));
        self.consumed = true;
        Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral })
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.close_owners.stage(Puzzle5dWindowWorkOwners { instance_owner: self.instance_owner.take(), tool_run: self.tool_run.take(), authoring_seed: std::mem::take(&mut self.authoring_seed) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

/// 📤 `exportSnapshot` reads the document and publishes a download; it owns no mutation, so it resolves
/// straight from the snapshot and picks its lane by payload size: one inline effect under the guest's
/// contiguous request ceiling, the framework's segmented-download lane above it, a localized notice
/// above what one segmented download may carry. It is NOT a `dispatch_puzzle5d_action` arm because a
/// segmented download is a `PuzzleCommandWorkStep`, not an `Emit` — `◻️2d`'s `Puzzle2dExportWork` is
/// the same shape.
#[derive(Default)]
struct Puzzle5dExportWork {
    consumed: bool,
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dExportWork {
    fn tool_id(&self) -> &'static str {
        "exportSnapshot"
    }

    fn extent(&self, _command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        Some(1)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config: _config, interaction: _interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        if self.consumed {
            return Err(Fault::from("puzzle5d-export-work-repeated"));
        }
        self.consumed = true;
        let document: Puzzle5dDocument = puzzle5d_document_from_snapshot(snapshot.typed()).map_err(|_| Fault::from("puzzle5d-export-document-malformed"))?;
        Ok(match export_snapshot::puzzle5d_export_publication(&document)? {
            export_snapshot::Puzzle5dExportPublication::Inline(effect) => {
                semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { effects: vec![effect], ui_scope: UiDirtyScope::None, ..Default::default() })
            }
            export_snapshot::Puzzle5dExportPublication::Segmented(download) => semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteDownload { download: download, ephemeral: Default::default() },
            export_snapshot::Puzzle5dExportPublication::Refused(_) => semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.export_too_large.as_str())),
        })
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dTransformStage {
    Read,
    Scan,
    Commit,
    Complete,
    Closing,
}

/// 🎞️ What `Read` states for one transform: the finished record, or the world drop whose proximity scan `Scan` pages.
enum Puzzle5dTransformGesture {
    Stated(world3d::utilities::transform::Puzzle5dSelectionRecord),
    Scanning(world3d::utilities::transform::Puzzle5dRelocateScan),
}

/// 🛠️ The ONE retained work of every selection transform — a gumball translate/rotate/scale, a target-volume
/// gumball relocate, a world drop, an inspector `x`/`y`/origin nudge. `Read` states the gesture as ONE
/// [`world3d::utilities::transform::Puzzle5dSelectionRecord`] (the gesture's own ids, else the live selection);
/// a world drop's proximity scan then runs page by page in `Scan` (one progress report per page, cancellable between
/// pages), and `Commit` runs the record through the transform tool: one `ToolTransaction` whose parametric leaves
/// publish as ONE edit stamped with the ref minted from the admission's `authoring_seed`. A gesture that moves nothing leaves
/// zero trace; nothing addressed, or everything addressed locked, is one localized refusal.
/// ♻️ The owners one `Puzzle5dTransformWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dTransformWorkOwners {
    record: Option<world3d::utilities::transform::Puzzle5dSelectionRecord>,
    scan: Option<world3d::utilities::transform::Puzzle5dRelocateScan>,
}

struct Puzzle5dTransformWork {
    tool_id: &'static str,
    authoring_seed: String,
    stage: Puzzle5dTransformStage,
    record: Option<world3d::utilities::transform::Puzzle5dSelectionRecord>,
    scan: Option<world3d::utilities::transform::Puzzle5dRelocateScan>,
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dTransformWorkOwners>,
}

impl Puzzle5dTransformWork {
    fn new(tool_id: &'static str, authoring_seed: String) -> Self {
        Self { close_owners: Default::default(), tool_id, authoring_seed, stage: Puzzle5dTransformStage::Read, record: None, scan: None }
    }

    /// 🕹️ The parts a selection-scoped verb addresses: the command's own `ids`, else the live part selection.
    fn addressed(command: &Puzzle5dCommand, interaction: &protocol::InteractionState) -> Vec<String> {
        let explicit: Vec<String> = command.args().and_then(|args| args.get("ids")).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
        if !explicit.is_empty() {
            return explicit;
        }
        Self::selected(interaction, PUZZLE5D_GRANULARITY_PART)
    }

    fn selected(interaction: &protocol::InteractionState, granularity: &str) -> Vec<String> {
        interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == granularity).map_or_else(Vec::new, |selection| selection.ids.clone())
    }

    /// 🩹️ The parts an inspector edit names: `partIds` then `partId`, each once, empty ids dropped.
    fn patched(command: &Puzzle5dCommand) -> Vec<String> {
        let args = command.args();
        let listed = args.and_then(|args| args.get("partIds")).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str);
        let single = args.and_then(|args| args.get("partId")).and_then(Value::as_str);
        listed.chain(single).filter(|id| !id.is_empty()).map(str::to_string).collect()
    }

    /// 🎬️ The gesture this command states on `document`, `None` when it states none.
    fn read(&self, command: &Puzzle5dCommand, document: &Puzzle5dSnapshot, config: &Puzzle5dConfig, interaction: &protocol::InteractionState) -> Option<Puzzle5dTransformGesture> {
        use semio_s_artifact_puzzle_3d::editor::puzzle3d::modes::edit::windows::main::utilities::transform::Puzzle3dSelectionRecord;
        use world3d::utilities::transform::{Puzzle5dRelocateScan, Puzzle5dSelectionRecord};
        let args = command.args();
        match self.tool_id {
            "translateSelection" | "rotateSelection" | "scaleSelection" => {
                let targets = [Self::addressed(command, interaction), Self::selected(interaction, PUZZLE5D_GRANULARITY_TARGET_VOLUME)].concat();
                Puzzle3dSelectionRecord::from_gumball(self.tool_id, args, targets).map(|record| Puzzle5dTransformGesture::Stated(Puzzle5dSelectionRecord::world(record)))
            }
            "relocateTargetVolume" => Puzzle3dSelectionRecord::from_pose_delta(args).map(|record| Puzzle5dTransformGesture::Stated(Puzzle5dSelectionRecord::world(record))),
            "worldRelocate" => {
                let part_id = args.and_then(|args| args.get("objectId")).and_then(Value::as_str).unwrap_or("");
                let position = args.and_then(|args| args.get("position")).and_then(puzzle5d_value_as_f64_3)?;
                Puzzle5dRelocateScan::begin(document, part_id, position, config.proximity_radius).map(Puzzle5dTransformGesture::Scanning)
            }
            _ => puzzle5d_inspector_nudge(args).map(|motion| Puzzle5dTransformGesture::Stated(Puzzle5dSelectionRecord::new(Self::patched(command), motion))),
        }
    }

    /// 🏁️ The ONE terminal emit: the committed transaction, a refusal, or nothing at all.
    fn commit(&mut self, snapshot: &Puzzle5dPlaySnapshot, view_state: Option<&semio_framework_plugin::ViewModel>) -> Emit<Puzzle5dMutation, Puzzle5dConfigMutation> {
        self.stage = Puzzle5dTransformStage::Complete;
        let Some(record) = self.record.take() else { return Emit { ui_scope: UiDirtyScope::None, ..Default::default() } };
        let base = snapshot.typed_arc();
        if !record.names_any(&base) {
            return puzzle5d_notice_emit(view_state, |labels| labels.nothing_selected.as_str());
        }
        if record.refused_as_locked(&base) {
            return puzzle5d_notice_emit(view_state, |labels| labels.selection_locked.as_str());
        }
        let request = world3d::utilities::transform::TransformToolRequest { base, records: vec![record] };
        match world3d::utilities::transform::puzzle5d_transform_tool_commit(self.tool_id, &self.authoring_seed, request) {
            Some((transaction, mutations)) => Emit { artifact_mutations: mutations, transaction: (!self.authoring_seed.is_empty()).then_some(transaction), ui_scope: UiDirtyScope::Full, ..Default::default() },
            None => Emit { ui_scope: UiDirtyScope::None, ..Default::default() },
        }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dTransformWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    /// 🔢️ `Read` + `Commit`, plus one `Scan` step per page of parts a world drop measures.
    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let addressed = Self::addressed(command, interaction).len().checked_add(Self::selected(interaction, PUZZLE5D_GRANULARITY_TARGET_VOLUME).len())?.checked_add(Self::patched(command).len())?;
        let pages = if self.tool_id == "worldRelocate" { snapshot.typed().parts.len().div_ceil(world3d::utilities::transform::PUZZLE5D_RELOCATE_SCAN_PAGE).max(1) } else { 0 };
        (addressed <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS).then_some(2 + pages)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction, hover: _hover, context, .. } = *input;
        match self.stage {
            Puzzle5dTransformStage::Read => {
                match self.read(command, snapshot.typed(), config, interaction) {
                    Some(Puzzle5dTransformGesture::Stated(record)) => self.record = Some(record),
                    Some(Puzzle5dTransformGesture::Scanning(scan)) => self.scan = Some(scan),
                    None => {}
                }
                self.stage = if self.scan.is_some() { Puzzle5dTransformStage::Scan } else { Puzzle5dTransformStage::Commit };
                Ok(crate::puzzle_progress_step!("puzzle5d-transform-read", "Reading the gesture", "Geste wird gelesen"))
            }
            Puzzle5dTransformStage::Scan => {
                let scan = self.scan.as_mut().ok_or_else(|| Fault::from("puzzle5d-transform-scan-owner"))?;
                if scan.step(snapshot.typed(), world3d::utilities::transform::PUZZLE5D_RELOCATE_SCAN_PAGE) {
                    self.record = self.scan.take().map(world3d::utilities::transform::Puzzle5dRelocateScan::finish);
                    self.stage = Puzzle5dTransformStage::Commit;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-transform-scan", "Measuring nearby grips", "Nahe Griffe werden gemessen"))
            }
            Puzzle5dTransformStage::Commit => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(self.commit(snapshot, context.and_then(|context| context.view_state.as_ref())))),
            Puzzle5dTransformStage::Complete => Err(Fault::from("puzzle5d-transform-complete-repolled")),
            Puzzle5dTransformStage::Closing => Err(Fault::from("puzzle5d-transform-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dTransformStage::Closing;
        self.close_owners.stage(Puzzle5dTransformWorkOwners { record: std::mem::take(&mut self.record), scan: std::mem::take(&mut self.scan) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dTransformStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
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

/// ♻️ The owners one `Puzzle5dKindWeightWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dKindWeightWorkOwners {
    ids: Vec<String>,
    seen: HashSet<String>,
    result: HashMap<String, f64>,
    changed_id: Option<String>,
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
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dKindWeightWorkOwners>,
}

impl Puzzle5dKindWeightWork {
    fn new(tool_id: &'static str) -> Self {
        Self { close_owners: Default::default(),
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
        let projection = puzzle5d_editor_projection(snapshot);
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

}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dKindWeightWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        Some(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dKindWeightStage::Catalog => {
                let entries = self.catalog(snapshot);
                if let Some(entry) = entries.get(self.cursor) {
                    if let Some(id) = entry.get("id").and_then(Value::as_str) {
                        self.push_id(id, false)?;
                    }
                    self.cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-catalog", "Reading kind owner", "Artinhaber wird gelesen"));
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
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-infer", "Preparing kind validation", "Artprüfung wird vorbereitet"))
            }
            Puzzle5dKindWeightStage::InferParts => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dKindWeightStage::Validate;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-validate", "Validating current weights", "Aktuelle Gewichte werden geprüft"));
                };
                self.part_cursor += 1;
                if let Some(id) = part.get("partKind").and_then(Value::as_str) {
                    self.push_id(id, true)?;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-part", "Reading inferred part kind", "Abgeleitete Teileart wird gelesen"))
            }
            Puzzle5dKindWeightStage::InferGrips => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dKindWeightStage::Validate;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-validate", "Validating current weights", "Aktuelle Gewichte werden geprüft"));
                };
                let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
                    self.part_cursor += 1;
                    self.grip_cursor = 0;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-part", "Advancing grip owner", "Griffinhaber wird gewechselt"));
                };
                self.grip_cursor += 1;
                if let Some(id) = grip.get("gripKind").and_then(Value::as_str) {
                    self.push_id(id, true)?;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-grip", "Reading inferred grip kind", "Abgeleitete Griffart wird gelesen"))
            }
            Puzzle5dKindWeightStage::Validate => {
                if self.changed_id.is_none() {
                    self.changed_id = Some(command.args().and_then(|args| args.get("kindId")).and_then(Value::as_str).unwrap_or("").to_string());
                    self.requested = command.args().and_then(|args| args.get("value")).and_then(Value::as_f64).unwrap_or(1.0).clamp(0.0, 1.0);
                }
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle5dKindWeightStage::SumOthers;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-sum", "Measuring sibling weights", "Geschwistergewichte werden gemessen"));
                };
                let weights = self.weights(config);
                self.missing |= !weights.contains_key(id);
                self.base_sum += weights.get(id).copied().unwrap_or(0.0);
                self.cursor += 1;
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-validate", "Validating kind weight", "Artgewicht wird geprüft"))
            }
            Puzzle5dKindWeightStage::SumOthers => {
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle5dKindWeightStage::Changed;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-changed", "Preparing changed weight", "Geändertes Gewicht wird vorbereitet"));
                };
                if self.changed_id.as_deref() != Some(id.as_str()) {
                    self.other_sum += self.base_weight(config, id);
                    self.other_count += 1;
                }
                self.cursor += 1;
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-sum", "Measuring sibling weight", "Geschwistergewicht wird gemessen"))
            }
            Puzzle5dKindWeightStage::Changed => {
                if self.ids.len() >= 2 {
                    self.result.insert(self.changed_id.clone().ok_or_else(|| Fault::from("puzzle5d-kind-weight-changed-owner"))?, self.requested);
                }
                self.stage = Puzzle5dKindWeightStage::Build;
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-build", "Building normalized weights", "Normalisierte Gewichte werden aufgebaut"))
            }
            Puzzle5dKindWeightStage::Build => {
                let Some(id) = self.ids.get(self.cursor).cloned() else {
                    self.stage = Puzzle5dKindWeightStage::Publish;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-publish", "Preparing weight publication", "Gewichtsveröffentlichung wird vorbereitet"));
                };
                self.cursor += 1;
                let value = if self.ids.len() == 1 {
                    1.0
                } else if self.changed_id.as_deref() == Some(id.as_str()) {
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-build", "Keeping changed weight", "Geändertes Gewicht wird beibehalten"));
                } else {
                    let remainder = (1.0 - self.requested).max(0.0);
                    if self.other_sum <= f64::EPSILON {
                        remainder / self.other_count.max(1) as f64
                    } else {
                        self.base_weight(config, &id) / self.other_sum * remainder
                    }
                };
                self.result.insert(id, value);
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-build", "Building kind weight", "Artgewicht wird aufgebaut"))
            }
            Puzzle5dKindWeightStage::Publish => {
                self.stage = Puzzle5dKindWeightStage::Complete;
                let mutation = if self.tool_id == "setPartKindWeight" {
                    Puzzle5dConfigMutation::SetObjectKindWeights(Puzzle5dConfigSetObjectKindWeights{ value: std::mem::take(&mut self.result) })
                } else {
                    Puzzle5dConfigMutation::SetVortexKindWeights(Puzzle5dConfigSetVortexKindWeights{ value: std::mem::take(&mut self.result) })
                };
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { config_mutations: vec![mutation], ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dKindWeightStage::Complete => Err(Fault::from("puzzle5d-kind-weight-complete-repolled")),
            Puzzle5dKindWeightStage::Closing => Err(Fault::from("puzzle5d-kind-weight-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dKindWeightStage::Closing;
        self.close_owners.stage(Puzzle5dKindWeightWorkOwners { ids: std::mem::take(&mut self.ids), seen: std::mem::take(&mut self.seen), result: std::mem::take(&mut self.result), changed_id: std::mem::take(&mut self.changed_id) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dKindWeightStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
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

/// ♻️ The owners one `Puzzle5dFocusSelectionWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dFocusSelectionWorkOwners {
    selected: HashSet<String>,
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
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dFocusSelectionWorkOwners>,
}

impl Default for Puzzle5dFocusSelectionWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dFocusSelectionStage::Selection,
            selection_cursor: 0,
            part_cursor: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            sum_2d: [0.0; 2],
            sum_3d: [0.0; 3],
            matched: 0,
            minimum_3d: [f64::MAX; 3],
            maximum_3d: [f64::MIN; 3],
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

}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dFocusSelectionWork {
    fn tool_id(&self) -> &'static str {
        "focusSelection"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let selected = Self::source(interaction).len();
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let items = selected.checked_add(parts)?.checked_add(1)?;
        (selected <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config: _config, interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dFocusSelectionStage::Selection => {
                if let Some(id) = Self::source(interaction).get(self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-focus-selection-capacity"));
                    }
                    self.selected.insert(id.clone());
                    self.selection_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-focus-selection-owner", "Reading selected part", "Ausgewähltes Teil wird gelesen"));
                }
                self.stage = Puzzle5dFocusSelectionStage::Parts;
                Ok(crate::puzzle_progress_step!("puzzle5d-focus-selection-part", "Finding selected part", "Ausgewähltes Teil wird gesucht"))
            }
            Puzzle5dFocusSelectionStage::Parts => {
                let Some(row) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dFocusSelectionStage::Publish;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-focus-selection-publish", "Preparing camera focus", "Kamerafokus wird vorbereitet"));
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
                Ok(crate::puzzle_progress_step!("puzzle5d-focus-selection-part", "Scanning selected part", "Ausgewähltes Teil wird geprüft"))
            }
            Puzzle5dFocusSelectionStage::Publish => {
                self.stage = Puzzle5dFocusSelectionStage::Complete;
                if self.matched == 0 {
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.nothing_selected.as_str())));
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
                let mut next = window_ownership::config_from_snapshot(window_config_snapshot);
                let offset = [next.camera3d.position[0] - next.camera3d.target[0], next.camera3d.position[1] - next.camera3d.target[1], next.camera3d.position[2] - next.camera3d.target[2]];
                let orbit = (offset[0] * offset[0] + offset[1] * offset[1] + offset[2] * offset[2]).sqrt();
                let scale = if orbit > f64::EPSILON { distance / orbit } else { 1.0 };
                next.camera3d.target = centre;
                next.camera3d.position = [centre[0] + offset[0] * scale, centre[1] + offset[1] * scale, centre[2] + offset[2] * scale];
                next.camera2d.x = self.sum_2d[0] / divisor;
                next.camera2d.y = self.sum_2d[1] / divisor;
                let view = view_state.ok_or_else(|| Fault::from("puzzle5d-focus-window-context-required"))?;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: vec![window_ownership::addressed_config(view, next)?], ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dFocusSelectionStage::Complete => Err(Fault::from("puzzle5d-focus-selection-complete-repolled")),
            Puzzle5dFocusSelectionStage::Closing => Err(Fault::from("puzzle5d-focus-selection-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dFocusSelectionStage::Closing;
        self.close_owners.stage(Puzzle5dFocusSelectionWorkOwners { selected: std::mem::take(&mut self.selected) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dFocusSelectionStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dPatchPartStage {
    Selection,
    Parts,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dPatchPartWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dPatchPartWorkOwners {
    mutations: Vec<Puzzle5dMutation>,
    selected: HashSet<String>,
}

struct Puzzle5dPatchPartWork {
    stage: Puzzle5dPatchPartStage,
    selection_cursor: usize,
    part_cursor: usize,
    selected: HashSet<String>,
    mutations: Vec<Puzzle5dMutation>,
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dPatchPartWorkOwners>,
}

impl Default for Puzzle5dPatchPartWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dPatchPartStage::Selection,
            selection_cursor: 0,
            part_cursor: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
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

}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dPatchPartWork {
    fn tool_id(&self) -> &'static str {
        "patchPart"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let items = Self::source_len(command).checked_add(projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dPatchPartStage::Selection => {
                if let Some(id) = Self::source_id(command, self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-patch-part-selection-capacity"));
                    }
                    self.selected.insert(id.to_string());
                    self.selection_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-patch-part-selection", "Reading part target", "Teilziel wird gelesen"));
                }
                self.stage = Puzzle5dPatchPartStage::Parts;
                Ok(crate::puzzle_progress_step!("puzzle5d-patch-part", "Patching part", "Teil wird geändert"))
            }
            Puzzle5dPatchPartStage::Parts => {
                let Some(row) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)).cloned() else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    // 🧯️ An unaddressed id or a field this inspector cannot write produced no mutation at all;
                    // the pre-migration arm fell through silently and the panel looked dead.
                    if self.mutations.is_empty() {
                        return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.edit_not_applicable.as_str())));
                    }
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                self.part_cursor += 1;
                let part: Puzzle5dPart = puzzle5d_record_from_projection(row).map_err(|_| Fault::from("puzzle5d-patch-part-malformed"))?;
                if self.selected.contains(&part.id) {
                    if let Some(mutation) = Self::mutation(command, &part) {
                        self.mutations.push(mutation);
                    }
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-patch-part", "Patching part", "Teil wird geändert"))
            }
            Puzzle5dPatchPartStage::Complete => Err(Fault::from("puzzle5d-patch-part-complete-repolled")),
            Puzzle5dPatchPartStage::Closing => Err(Fault::from("puzzle5d-patch-part-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dPatchPartStage::Closing;
        self.close_owners.stage(Puzzle5dPatchPartWorkOwners { mutations: std::mem::take(&mut self.mutations), selected: std::mem::take(&mut self.selected) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dPatchPartStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

/// ♻️ The owners one `Puzzle5dPatchFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dPatchFastenerWorkOwners {
    mutations: Vec<Puzzle5dMutation>,
    selected: HashSet<String>,
}

struct Puzzle5dPatchFastenerWork {
    stage: Puzzle5dPatchPartStage,
    selection_cursor: usize,
    fastener_cursor: usize,
    selected: HashSet<String>,
    mutations: Vec<Puzzle5dMutation>,
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dPatchFastenerWorkOwners>,
}

impl Default for Puzzle5dPatchFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
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

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dPatchFastenerWork {
    fn tool_id(&self) -> &'static str {
        "patchFastener"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let items = Self::source_len(command).checked_add(projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len))?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dPatchPartStage::Selection => {
                if let Some(id) = Self::source_id(command, self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-patch-fastener-selection-capacity"));
                    }
                    self.selected.insert(id.to_string());
                    self.selection_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-patch-fastener-selection", "Reading fastener target", "Verbindungsziel wird gelesen"));
                }
                self.stage = Puzzle5dPatchPartStage::Parts;
                Ok(crate::puzzle_progress_step!("puzzle5d-patch-fastener", "Patching fastener", "Verbindung wird geändert"))
            }
            Puzzle5dPatchPartStage::Parts => {
                let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)).cloned() else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                self.fastener_cursor += 1;
                let fastener: Puzzle5dFastener = puzzle5d_record_from_projection(row).map_err(|_| Fault::from("puzzle5d-patch-fastener-malformed"))?;
                if self.selected.contains(&fastener.id) {
                    if let Some(mutation) = Self::mutation(command, &fastener) {
                        self.mutations.push(mutation);
                    }
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-patch-fastener", "Patching fastener", "Verbindung wird geändert"))
            }
            Puzzle5dPatchPartStage::Complete => Err(Fault::from("puzzle5d-patch-fastener-complete-repolled")),
            Puzzle5dPatchPartStage::Closing => Err(Fault::from("puzzle5d-patch-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dPatchPartStage::Closing;
        self.close_owners.stage(Puzzle5dPatchFastenerWorkOwners { mutations: std::mem::take(&mut self.mutations), selected: std::mem::take(&mut self.selected) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dPatchPartStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
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

/// ♻️ The owners one `Puzzle5dEditFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dEditFastenerWorkOwners {
    mutations: Vec<Puzzle5dMutation>,
    fastener: Option<Puzzle5dFastener>,
}

struct Puzzle5dEditFastenerWork {
    stage: Puzzle5dEditFastenerStage,
    cursor: usize,
    fastener: Option<Puzzle5dFastener>,
    mutations: Vec<Puzzle5dMutation>,
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dEditFastenerWorkOwners>,
}

impl Default for Puzzle5dEditFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dEditFastenerStage::Scan, cursor: 0, fastener: None, mutations: Vec::with_capacity(2) }
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

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dEditFastenerWork {
    fn tool_id(&self) -> &'static str {
        "editFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len).checked_add(2).filter(|items| *items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dEditFastenerStage::Scan => {
                let target = Self::id(command);
                if target.is_empty() {
                    self.stage = Puzzle5dEditFastenerStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default()));
                }
                let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.cursor)).cloned() else {
                    self.stage = Puzzle5dEditFastenerStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default()));
                };
                self.cursor += 1;
                if row.get("id").and_then(Value::as_str) == Some(target) {
                    self.fastener = Some(puzzle5d_record_from_projection(row).map_err(|_| Fault::from("puzzle5d-edit-fastener-malformed"))?);
                    self.stage = Puzzle5dEditFastenerStage::Kind;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-edit-fastener-scan", "Finding fastener", "Verbindung wird gesucht"))
            }
            Puzzle5dEditFastenerStage::Kind => {
                let Some(fastener) = self.fastener.as_ref() else { return Err(Fault::from("puzzle5d-edit-fastener-owner")) };
                if let Some(kind) = Self::updated_kind(command, fastener) {
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind(fastener.id.clone(), kind));
                }
                self.stage = Puzzle5dEditFastenerStage::Geometry;
                Ok(crate::puzzle_progress_step!("puzzle5d-edit-fastener-kind", "Updating fastener kind", "Verbindungsart wird aktualisiert"))
            }
            Puzzle5dEditFastenerStage::Geometry => {
                let Some(fastener) = self.fastener.as_ref() else { return Err(Fault::from("puzzle5d-edit-fastener-owner")) };
                if let Some(geometry) = Self::updated_geometry(command, fastener) {
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry(crate::standards::v1::subsets::any::schema::mutations::ReplaceFastenerGeometry { id: fastener.id.clone(), new_gap: geometry[0], new_shift: geometry[1], new_rise: geometry[2], new_rotation: geometry[3], new_turn: geometry[4], new_tilt: geometry[5], new_x: geometry[6], new_y: geometry[7] }));
                }
                self.stage = Puzzle5dEditFastenerStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dEditFastenerStage::Complete => Err(Fault::from("puzzle5d-edit-fastener-complete-repolled")),
            Puzzle5dEditFastenerStage::Closing => Err(Fault::from("puzzle5d-edit-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dEditFastenerStage::Closing;
        self.close_owners.stage(Puzzle5dEditFastenerWorkOwners { mutations: std::mem::take(&mut self.mutations), fastener: std::mem::take(&mut self.fastener) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dEditFastenerStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
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

/// ♻️ The owners one `Puzzle5dRetargetFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dRetargetFastenerWorkOwners {
    mutations: Vec<Puzzle5dMutation>,
    fastener: Option<Puzzle5dFastener>,
    source: Option<String>,
    target: Option<String>,
    source_kind: Option<String>,
    target_kind: Option<String>,
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
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dRetargetFastenerWorkOwners>,
}

impl Default for Puzzle5dRetargetFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
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
        let projection = puzzle5d_editor_projection(snapshot);
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

    fn complete_empty(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        self.stage = Puzzle5dRetargetFastenerStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default())
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dRetargetFastenerWork {
    fn tool_id(&self) -> &'static str {
        "retargetFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
        let items = parts.checked_mul(2)?.checked_add(fasteners.checked_mul(2)?)?.checked_add(compatibility)?.checked_add(2)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
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
                    let fastener: Puzzle5dFastener = puzzle5d_record_from_projection(row).map_err(|_| Fault::from("puzzle5d-retarget-fastener-malformed"))?;
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
                Ok(crate::puzzle_progress_step!("puzzle5d-retarget-fastener", "Finding fastener", "Verbindung wird gesucht"))
            }
            Puzzle5dRetargetFastenerStage::SourceGrip => {
                let source = self.source.as_deref().unwrap_or("").to_string();
                match self.scan_grip(snapshot, &source) {
                    Puzzle5dGripScan::Progress => Ok(crate::puzzle_progress_step!("puzzle5d-retarget-source", "Finding source grip", "Quellgriff wird gesucht")),
                    Puzzle5dGripScan::Found(kind) => {
                        self.source_kind = kind;
                        self.part_cursor = 0;
                        self.grip_cursor = 0;
                        self.stage = Puzzle5dRetargetFastenerStage::TargetGrip;
                        Ok(crate::puzzle_progress_step!("puzzle5d-retarget-target", "Finding target grip", "Zielgriff wird gesucht"))
                    }
                    Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
                }
            }
            Puzzle5dRetargetFastenerStage::TargetGrip => {
                let target = self.target.as_deref().unwrap_or("").to_string();
                match self.scan_grip(snapshot, &target) {
                    Puzzle5dGripScan::Progress => Ok(crate::puzzle_progress_step!("puzzle5d-retarget-target", "Finding target grip", "Zielgriff wird gesucht")),
                    Puzzle5dGripScan::Found(kind) => {
                        self.target_kind = kind;
                        self.fastener_cursor = 0;
                        self.stage = Puzzle5dRetargetFastenerStage::Duplicate;
                        Ok(crate::puzzle_progress_step!("puzzle5d-retarget-duplicate", "Checking duplicate fastener", "Doppelte Verbindung wird geprüft"))
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
                    return Ok(crate::puzzle_progress_step!("puzzle5d-retarget-duplicate", "Checking duplicate fastener", "Doppelte Verbindung wird geprüft"));
                }
                self.stage = Puzzle5dRetargetFastenerStage::Compatibility;
                Ok(crate::puzzle_progress_step!("puzzle5d-retarget-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dRetargetFastenerStage::Compatibility => {
                let rows = projection.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if rows.is_empty() || self.source_kind.is_none() || self.target_kind.is_none() {
                    self.stage = Puzzle5dRetargetFastenerStage::Disconnect;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-retarget-disconnect", "Disconnecting old fastener", "Alte Verbindung wird getrennt"));
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
                Ok(crate::puzzle_progress_step!("puzzle5d-retarget-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dRetargetFastenerStage::Disconnect => {
                let id = self.fastener.as_ref().map(|fastener| fastener.id.clone()).ok_or_else(|| Fault::from("puzzle5d-retarget-fastener-owner"))?;
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id));
                self.stage = Puzzle5dRetargetFastenerStage::Connect;
                Ok(crate::puzzle_progress_step!("puzzle5d-retarget-connect", "Connecting retargeted fastener", "Neu ausgerichtete Verbindung wird erstellt"))
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
                    fastener.y, None,
                ));
                self.stage = Puzzle5dRetargetFastenerStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dRetargetFastenerStage::Complete => Err(Fault::from("puzzle5d-retarget-fastener-complete-repolled")),
            Puzzle5dRetargetFastenerStage::Closing => Err(Fault::from("puzzle5d-retarget-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dRetargetFastenerStage::Closing;
        self.close_owners.stage(Puzzle5dRetargetFastenerWorkOwners { mutations: std::mem::take(&mut self.mutations), fastener: std::mem::take(&mut self.fastener), source: std::mem::take(&mut self.source), target: std::mem::take(&mut self.target), source_kind: std::mem::take(&mut self.source_kind), target_kind: std::mem::take(&mut self.target_kind) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dRetargetFastenerStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
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

/// ♻️ The owners one `Puzzle5dProximityConnectWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dProximityConnectWorkOwners {
    mutations: Vec<Puzzle5dMutation>,
    moved_id: Option<String>,
    moved_kind: Option<String>,
    moved_position: Option<[f64; 3]>,
    candidate_id: Option<String>,
    candidate_kind: Option<String>,
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
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dProximityConnectWorkOwners>,
}

impl Default for Puzzle5dProximityConnectWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
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

    fn complete(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        self.stage = Puzzle5dProximityConnectStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() })
    }
}

impl Puzzle5dProximityConnectWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dProximityConnectWork {
    fn tool_id(&self) -> &'static str {
        "proximityConnect"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
        let items = parts.checked_add(fasteners.checked_mul(parts)?)?.checked_add(compatibility.checked_mul(parts)?)?.checked_add(parts)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
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
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-moved", "Finding moved grip", "Verschobener Griff wird gesucht"))
            }
            Puzzle5dProximityConnectStage::Candidate => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    return Ok(self.complete());
                };
                let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
                    self.part_cursor += 1;
                    self.grip_cursor = 0;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                };
                self.grip_cursor += 1;
                if part.get("id").and_then(Value::as_str) == Some(part_id) {
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                }
                let Some(peer_part_id) = part.get("id").and_then(Value::as_str) else { return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Skipping malformed part", "Fehlerhaftes Teil wird übersprungen")) };
                let Some(peer_grip_id) = grip.get("id").and_then(Value::as_str) else { return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Skipping malformed grip", "Fehlerhafter Griff wird übersprungen")) };
                let peer_id = puzzle5d_grip_full_id(peer_part_id, peer_grip_id);
                if self.moved_id.as_deref() == Some(peer_id.as_str()) {
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                }
                let moved = self.moved_position.ok_or_else(|| Fault::from("puzzle5d-proximity-position-owner"))?;
                let peer = Self::world_position(part, grip);
                let radius = command.args().and_then(|args| args.get("radius")).and_then(Value::as_f64).unwrap_or(PUZZLE5D_PROXIMITY_RADIUS).max(0.0);
                let dx = moved[0] - peer[0];
                let dy = moved[1] - peer[1];
                let dz = moved[2] - peer[2];
                if (dx * dx + dy * dy + dz * dz).sqrt() > radius {
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                }
                self.candidate_id = Some(peer_id);
                self.candidate_kind = Self::grip_kind(grip);
                self.fastener_cursor = 0;
                self.stage = Puzzle5dProximityConnectStage::Existing;
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"))
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
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"));
                }
                self.stage = Puzzle5dProximityConnectStage::Compatibility;
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Compatibility => {
                let rows = projection.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if rows.is_empty() || self.candidate_kind.is_none() || self.moved_kind.is_none() {
                    self.stage = Puzzle5dProximityConnectStage::Emit;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-emit", "Connecting nearby grip", "Naher Griff wird verbunden"));
                }
                let Some(row) = rows.get(self.compatibility_cursor) else {
                    self.clear_candidate();
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
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
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
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
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(id, source, target, kind, arg("gap"), arg("shift"), arg("rise"), arg("rotation"), arg("turn"), arg("tilt"), arg("x"), arg("y"), None));
                self.clear_candidate();
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Complete => Err(Fault::from("puzzle5d-proximity-connect-complete-repolled")),
            Puzzle5dProximityConnectStage::Closing => Err(Fault::from("puzzle5d-proximity-connect-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dProximityConnectStage::Closing;
        self.close_owners.stage(Puzzle5dProximityConnectWorkOwners { mutations: std::mem::take(&mut self.mutations), moved_id: std::mem::take(&mut self.moved_id), moved_kind: std::mem::take(&mut self.moved_kind), moved_position: std::mem::take(&mut self.moved_position), candidate_id: std::mem::take(&mut self.candidate_id), candidate_kind: std::mem::take(&mut self.candidate_kind) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dProximityConnectStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

/// ♻️ The owners one `Puzzle5dPatchGripWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dPatchGripWorkOwners {
    mutations: Vec<Puzzle5dMutation>,
    selected: HashSet<String>,
}

struct Puzzle5dPatchGripWork {
    stage: Puzzle5dPatchPartStage,
    selection_cursor: usize,
    part_cursor: usize,
    grip_cursor: usize,
    processed_grips: usize,
    selected: HashSet<String>,
    mutations: Vec<Puzzle5dMutation>,
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dPatchGripWorkOwners>,
}

impl Default for Puzzle5dPatchGripWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
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

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dPatchGripWork {
    fn tool_id(&self) -> &'static str {
        "patchGrip"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let items = Self::source_len(command).checked_add(projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dPatchPartStage::Selection => {
                if let Some(id) = Self::source_id(command, self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-patch-grip-selection-capacity"));
                    }
                    self.selected.insert(id.to_string());
                    self.selection_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-patch-grip-selection", "Reading grip target", "Griffziel wird gelesen"));
                }
                self.stage = Puzzle5dPatchPartStage::Parts;
                Ok(crate::puzzle_progress_step!("puzzle5d-patch-grip", "Patching grip", "Griff wird geändert"))
            }
            Puzzle5dPatchPartStage::Parts => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                let Some(grip_value) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)).cloned() else {
                    self.part_cursor += 1;
                    self.grip_cursor = 0;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-patch-grip-part", "Advancing grip owner", "Griffinhaber wird gewechselt"));
                };
                if self.processed_grips >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
                    return Err(Fault::from("puzzle5d-patch-grip-work-capacity"));
                }
                self.processed_grips += 1;
                self.grip_cursor += 1;
                let part_id = part.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle5d-patch-grip-part-id-malformed"))?;
                let mut grip: crate::Puzzle5dGrip = <crate::Puzzle5dGrip as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&grip_value)).map_err(|_| Fault::from("puzzle5d-patch-grip-malformed"))?;
                let full_id = puzzle5d_grip_full_id(part_id, &grip.id);
                if self.selected.contains(&full_id) && Self::patch(command, &mut grip) {
                    let grip_id = grip.id.clone();
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::replace_part_grip(part_id.to_string(), grip_id, grip));
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-patch-grip", "Patching grip", "Griff wird geändert"))
            }
            Puzzle5dPatchPartStage::Complete => Err(Fault::from("puzzle5d-patch-grip-complete-repolled")),
            Puzzle5dPatchPartStage::Closing => Err(Fault::from("puzzle5d-patch-grip-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dPatchPartStage::Closing;
        self.close_owners.stage(Puzzle5dPatchGripWorkOwners { mutations: std::mem::take(&mut self.mutations), selected: std::mem::take(&mut self.selected) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dPatchPartStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

/// ♻️ The owners one `Puzzle5dDeleteFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dDeleteFastenerWorkOwners {
    mutations: Vec<Puzzle5dMutation>,
}

struct Puzzle5dDeleteFastenerWork {
    cursor: usize,
    closing: bool,
    mutations: Vec<Puzzle5dMutation>,
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dDeleteFastenerWorkOwners>,
}

impl Default for Puzzle5dDeleteFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), cursor: 0, closing: false, mutations: Vec::with_capacity(1) }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dDeleteFastenerWork {
    fn tool_id(&self) -> &'static str {
        "deleteFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        projection.get("fasteners").and_then(Value::as_array).map_or(Some(1), |fasteners| fasteners.len().checked_add(1)).filter(|items| *items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        let target = command.args().and_then(|args| args.get("id").or_else(|| args.get("fastenerId"))).and_then(Value::as_str).filter(|id| !id.is_empty());
        let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.cursor)) else {
            return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
        };
        self.cursor += 1;
        if target == row.get("id").and_then(Value::as_str) {
            if let Some(id) = target {
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id.to_string()));
            }
        }
        Ok(crate::puzzle_progress_step!("puzzle5d-delete-fastener", "Scanning fastener", "Verbindung wird geprüft"))
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.close_owners.stage(Puzzle5dDeleteFastenerWorkOwners { mutations: std::mem::take(&mut self.mutations) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dAddNodeStage {
    Catalog,
    Grips,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dAddNodeWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dAddNodeWorkOwners {
    mutation: Option<Puzzle5dMutation>,
    grips: Vec<crate::Puzzle5dGrip>,
    mesh_url: Option<String>,
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
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dAddNodeWorkOwners>,
}

impl Default for Puzzle5dAddNodeWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dAddNodeStage::Catalog, catalog_cursor: 0, grip_cursor: 0, catalog_index: None, mesh_url: None, grips: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS), mutation: None, operation_nonce: 0 }
    }
}

impl Puzzle5dAddNodeWork {
    fn part_kind(command: &Puzzle5dCommand) -> &str {
        command.args().and_then(|args| args.get("kind")).and_then(Value::as_str).unwrap_or("Part")
    }

    fn catalogs(snapshot: &Puzzle5dPlaySnapshot) -> Vec<Value> {
        let projection = puzzle5d_editor_projection(snapshot);
        projection.get("kindCatalogs").and_then(|catalogs| catalogs.get("parts")).and_then(Value::as_array).cloned().unwrap_or_default()
    }
}

impl Puzzle5dAddNodeWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dAddNodeWork {
    fn tool_id(&self) -> &'static str {
        "addNode"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let items = Self::catalogs(snapshot).len().checked_add(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        let catalogs = Self::catalogs(snapshot);
        match self.stage {
            Puzzle5dAddNodeStage::Catalog => {
                let Some(entry) = catalogs.get(self.catalog_cursor) else {
                    self.stage = Puzzle5dAddNodeStage::Grips;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-add-node-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                };
                let index = self.catalog_cursor;
                self.catalog_cursor += 1;
                if entry.get("id").and_then(Value::as_str) == Some(Self::part_kind(command)) {
                    self.catalog_index = Some(index);
                    self.mesh_url = entry.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).map(str::to_string);
                    self.stage = Puzzle5dAddNodeStage::Grips;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-add-node-catalog", "Reading part catalog", "Teilekatalog wird gelesen"))
            }
            Puzzle5dAddNodeStage::Grips => {
                let templates = self.catalog_index.and_then(|index| catalogs.get(index)).and_then(|entry| entry.get("grips")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if let Some(template) = templates.get(self.grip_cursor) {
                    if self.grips.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-add-node-grip-capacity"));
                    }
                    let grip_kind = template.get("gripKind").and_then(Value::as_str).unwrap_or("grip").to_string();
                    let grip_2d: crate::Puzzle5dGrip2d = match template.get("2d") {
                        Some(value) => <crate::Puzzle5dGrip2d as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value)).map_err(|_| Fault::from("puzzle5d-add-node-grip2d-malformed"))?,
                        None => Default::default(),
                    };
                    let grip_3d: crate::Puzzle5dGrip3d = match template.get("3d") {
                        Some(value) => <crate::Puzzle5dGrip3d as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value)).map_err(|_| Fault::from("puzzle5d-add-node-grip3d-malformed"))?,
                        None => Default::default(),
                    };
                    self.grips.push(crate::Puzzle5dGrip { id: format!("v{}", self.grip_cursor), grip_kind: Some(grip_kind), grip_2d, grip_3d });
                    self.grip_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-add-node-grip", "Reading grip template", "Griffvorlage wird gelesen"));
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
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
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
        self.close_owners.stage(Puzzle5dAddNodeWorkOwners { mutation: std::mem::take(&mut self.mutation), grips: std::mem::take(&mut self.grips), mesh_url: std::mem::take(&mut self.mesh_url) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dAddNodeStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
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

/// ♻️ The owners one `Puzzle5dAddBrushPartWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dAddBrushPartWorkOwners {
    mutations: Vec<Puzzle5dMutation>,
    payload: Option<Value>,
    grips: Vec<crate::Puzzle5dGrip>,
    mesh_url: Option<String>,
    target_id: Option<String>,
    target_position: Option<[f64; 3]>,
    target_direction: Option<[f64; 3]>,
    created_id: Option<String>,
    created_grip_id: Option<String>,
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
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dAddBrushPartWorkOwners>,
}

impl Puzzle5dAddBrushPartWork {
    fn is_closing(&self) -> bool {
        self.stage == Puzzle5dAddBrushPartStage::Closing
    }

    fn new(tool_id: &'static str) -> Self {
        Self { close_owners: Default::default(),
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

impl Puzzle5dAddBrushPartWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dAddBrushPartWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let items = Self::catalogs(snapshot).len().checked_add(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS)?.checked_add(projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?.checked_add(2)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-add-brush-part-work-capacity"));
        }
        self.processed_units += 1;
        let catalogs = Self::catalogs(snapshot);
        match self.stage {
            Puzzle5dAddBrushPartStage::Catalog => {
                let Some(entry) = catalogs.get(self.catalog_cursor) else {
                    self.stage = Puzzle5dAddBrushPartStage::Grips;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-brush-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                };
                let index = self.catalog_cursor;
                self.catalog_cursor += 1;
                if entry.get("id").and_then(Value::as_str) == Some(self.owned_part_kind(command).as_str()) {
                    self.catalog_index = Some(index);
                    self.mesh_url = entry.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).map(str::to_string);
                    self.stage = Puzzle5dAddBrushPartStage::Grips;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-brush-catalog", "Reading part catalog", "Teilekatalog wird gelesen"))
            }
            Puzzle5dAddBrushPartStage::Grips => {
                let templates = self.catalog_index.and_then(|index| catalogs.get(index)).and_then(|entry| entry.get("grips")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if let Some(template) = templates.get(self.grip_cursor) {
                    if self.grips.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-add-brush-part-grip-capacity"));
                    }
                    let grip_kind = template.get("gripKind").and_then(Value::as_str).unwrap_or("grip").to_string();
                    let grip_2d = template.get("2d").map(|value| <crate::Puzzle5dGrip2d as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value))).transpose().map_err(|_| Fault::from("puzzle5d-add-brush-part-grip2d-malformed"))?.unwrap_or_default();
                    let grip_3d = template.get("3d").map(|value| <crate::Puzzle5dGrip3d as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value))).transpose().map_err(|_| Fault::from("puzzle5d-add-brush-part-grip3d-malformed"))?.unwrap_or_default();
                    let id = format!("v{}", self.grip_cursor);
                    if self.created_grip_id.is_none() {
                        self.created_grip_id = Some(id.clone());
                    }
                    self.grips.push(crate::Puzzle5dGrip { id, grip_kind: Some(grip_kind), grip_2d, grip_3d });
                    self.grip_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-brush-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                }
                self.target_id = self.target(command, interaction);
                self.stage = if self.target_id.is_some() { Puzzle5dAddBrushPartStage::Target } else { Puzzle5dAddBrushPartStage::Create };
                Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht"))
            }
            Puzzle5dAddBrushPartStage::Target => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.target_id = None;
                    self.stage = Puzzle5dAddBrushPartStage::Create;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-brush-create", "Creating brush part", "Pinselteil wird erstellt"));
                };
                let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.target_grip_cursor)) else {
                    self.part_cursor += 1;
                    self.target_grip_cursor = 0;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht"));
                };
                self.target_grip_cursor += 1;
                let Some(part_id) = part.get("id").and_then(Value::as_str) else { return Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht")) };
                let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht")) };
                if self.target_id.as_deref() == Some(puzzle5d_grip_full_id(part_id, grip_id).as_str()) {
                    self.target_position = Some(Puzzle5dProximityConnectWork::world_position(part, grip));
                    self.target_direction = Some(Self::world_direction(part, grip));
                    self.stage = Puzzle5dAddBrushPartStage::Create;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht"))
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
                Ok(crate::puzzle_progress_step!("puzzle5d-brush-create", "Creating brush part", "Pinselteil wird erstellt"))
            }
            Puzzle5dAddBrushPartStage::Connect => {
                if let (Some(source), Some(part), Some(grip)) = (self.target_id.as_ref(), self.created_id.as_ref(), self.created_grip_id.as_ref()) {
                    let id = self.args(command).and_then(|args| args.get("edgeId").and_then(Value::as_str).map(str::to_string)).filter(|id| !id.is_empty()).unwrap_or_else(|| {
                        let id = format!("fastener-{:016x}-{}", self.operation_nonce, self.fresh_cursor);
                        self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                        id
                    });
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(id, source.clone(), puzzle5d_grip_full_id(part, grip), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None));
                }
                self.stage = Puzzle5dAddBrushPartStage::Complete;
                // 🕹️ Re-select the placed part so the brush's next gesture chains off it, exactly as puzzle
                // 3d's `addBrushObject` does.
                let interaction_writes = self.created_id.take().map(|id| InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, [id])).into_iter().collect();
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), interaction_writes, ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dAddBrushPartStage::Complete => Err(Fault::from("puzzle5d-add-brush-part-complete-repolled")),
            Puzzle5dAddBrushPartStage::Closing => Err(Fault::from("puzzle5d-add-brush-part-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dAddBrushPartStage::Closing;
        self.close_owners.stage(Puzzle5dAddBrushPartWorkOwners { mutations: std::mem::take(&mut self.mutations), payload: std::mem::take(&mut self.payload), grips: std::mem::take(&mut self.grips), mesh_url: std::mem::take(&mut self.mesh_url), target_id: std::mem::take(&mut self.target_id), target_position: std::mem::take(&mut self.target_position), target_direction: std::mem::take(&mut self.target_direction), created_id: std::mem::take(&mut self.created_id), created_grip_id: std::mem::take(&mut self.created_grip_id) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dAddBrushPartStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dBoardEventsStage {
    Open,
    Scan,
    Decode,
    Dispatch,
    ScanEdge,
    ScanDeleteEdges,
    Brush,
    DrainBrush,
    CloseBrush,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dBoardEventsWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dBoardEventsWorkOwners {
    mutations: Vec<Puzzle5dMutation>,
    brush_first: Option<Puzzle5dMutation>,
    brush_second: Option<Puzzle5dMutation>,
    event: Option<Value>,
    drags: Vec<world3d::utilities::transform::Puzzle5dSelectionRecord>,
    drag_at: Option<usize>,
    pending_source: Option<String>,
    pending_target: Option<String>,
    pending_edge_id: Option<String>,
    pending_edge_kind: Option<String>,
    pending_delete_id: Option<String>,
    camera2d: Option<Puzzle5dCamera2d>,
    select_ids: Option<Vec<String>>,
    removed_ids: Vec<String>,
    placed_id: Option<String>,
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
    /// 🎬️ Every `drag` gesture record of the batch, each one board drag the transform tool yields — the whole
    /// batch's drags are ONE tool transaction.
    drags: Vec<world3d::utilities::transform::Puzzle5dSelectionRecord>,
    /// 📍️ Where the first drag sits among the batch's mutations — the tool's leaves publish there, in event order.
    drag_at: Option<usize>,
    /// 🌱️ The admission's authoring seed the drags' tool transaction is minted from.
    authoring_seed: String,
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
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dBoardEventsWorkOwners>,
}

impl Puzzle5dBoardEventsWork {
    fn new(authoring_seed: String) -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dBoardEventsStage::Open,
            byte_cursor: 0,
            event_start: None,
            event_end: 0,
            depth: 0,
            in_string: false,
            escape: false,
            event: None,
            drags: Vec::new(),
            drag_at: None,
            authoring_seed,
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
        }
    }
}

impl Puzzle5dBoardEventsWork {
    fn source(command: &Puzzle5dCommand) -> Result<&str, Fault> {
        command.args().and_then(|args| args.get("eventsJson")).and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle5d-board-events-input-missing"))
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
}

impl Puzzle5dBoardEventsWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dBoardEventsWork {
    fn tool_id(&self) -> &'static str {
        "applyBoardEvents"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let bytes = Self::source(command).ok()?.len();
        let document_items = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len).checked_add(projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len))?.checked_add(2)?;
        let items = bytes.checked_mul(document_items)?;
        (bytes <= crate::retained_command::PUZZLE_COMMAND_RAW_BYTES && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items.max(1))
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction, hover: hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let projection = puzzle5d_editor_projection(snapshot);
        let source = Self::source(command)?;
        match self.stage {
            Puzzle5dBoardEventsStage::Open | Puzzle5dBoardEventsStage::Scan => {
                self.scan_one(source)?;
                Ok(crate::puzzle_progress_step!("puzzle5d-board-event-scan", "Reading board event", "Board-Ereignis wird gelesen"))
            }
            Puzzle5dBoardEventsStage::Decode => {
                let start = self.event_start.ok_or_else(|| Fault::from("puzzle5d-board-events-event-owner-missing"))?;
                self.event = Some(parse(source.get(start..self.event_end).ok_or_else(|| Fault::from("puzzle5d-board-events-event-range"))?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| Fault::from("puzzle5d-board-events-event-malformed"))?);
                self.stage = Puzzle5dBoardEventsStage::Dispatch;
                Ok(crate::puzzle_progress_step!("puzzle5d-board-event-decode", "Decoding board event", "Board-Ereignis wird dekodiert"))
            }
            Puzzle5dBoardEventsStage::Dispatch => {
                let name = self.event.as_ref().and_then(|event| event.get("name")).and_then(Value::as_str).map(str::to_string);
                let payload = self.take_payload();
                match name.as_deref() {
                    Some("camera") => {
                        self.camera2d = Some(<Puzzle5dCamera2d as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&payload)).map_err(|_| Fault::from("puzzle5d-board-events-camera-malformed"))?);
                        self.next_event();
                    }
                    // 🎬️ A board drag is ONE `drag` gesture record: its targets (each once) move by its offset as a
                    // `drag-selection2d` leaf the transform tool yields at completion — relative, so the drag replays
                    // on whatever flat poses its base holds. The board drag moves ONLY the flat pose: the world origin
                    // stays where the 3d pane put it, which is what makes a board drag a plan edit. Other gesture
                    // kinds move no part here.
                    Some("gesture") => {
                        let drag = payload.get("kind").and_then(Value::as_str) == Some("drag");
                        let offset = payload.get("dx").and_then(Value::as_f64).zip(payload.get("dy").and_then(Value::as_f64)).filter(|(dx, dy)| dx.is_finite() && dy.is_finite());
                        if let Some((dx, dy)) = offset.filter(|_| drag) {
                            let targets = payload.get("targets").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                            let record = world3d::utilities::transform::Puzzle5dSelectionRecord::new(targets, world3d::utilities::transform::Puzzle5dSelectionMotion::Board { dx, dy });
                            if self.drags.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                                return Err(Fault::from("puzzle5d-board-events-drag-capacity"));
                            }
                            self.drag_at.get_or_insert(self.mutations.len());
                            self.drags.push(record);
                        }
                        self.next_event();
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
                Ok(crate::puzzle_progress_step!("puzzle5d-board-event-dispatch", "Applying board event", "Board-Ereignis wird angewendet"))
            }
            Puzzle5dBoardEventsStage::ScanEdge => {
                if self.pending_source.is_none() || self.pending_target.is_none() {
                    self.next_event();
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-edge", "Checking board edge", "Board-Kante wird geprüft"));
                }
                if let Some(fastener) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let source = fastener.get("source").and_then(Value::as_str);
                    let target = fastener.get("target").and_then(Value::as_str);
                    if (source == self.pending_source.as_deref() && target == self.pending_target.as_deref()) || (source == self.pending_target.as_deref() && target == self.pending_source.as_deref()) {
                        self.pending_source = None;
                        self.pending_target = None;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-edge", "Checking board edge", "Board-Kante wird geprüft"));
                }
                let id = self.pending_edge_id.take().expect("preflighted edge id");
                let source = self.pending_source.take().expect("preflighted edge source");
                let target = self.pending_target.take().expect("preflighted edge target");
                let kind = self.pending_edge_kind.take();
                self.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(id, source, target, kind, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None))?;
                self.next_event();
                Ok(crate::puzzle_progress_step!("puzzle5d-board-edge", "Creating board edge", "Board-Kante wird erstellt"))
            }
            Puzzle5dBoardEventsStage::ScanDeleteEdges => {
                let Some(id) = self.pending_delete_id.as_deref() else {
                    self.next_event();
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-delete", "Deleting board node", "Board-Knoten wird gelöscht"));
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
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-delete-edge", "Removing attached edge", "Verbundene Kante wird entfernt"));
                }
                let id = self.pending_delete_id.take().expect("preflighted deleted part");
                self.removed_ids.push(id.clone());
                self.push(crate::standards::v1::subsets::any::schema::mutations::delete_part(id))?;
                self.next_event();
                Ok(crate::puzzle_progress_step!("puzzle5d-board-delete", "Deleting board node", "Board-Knoten wird gelöscht"))
            }
            Puzzle5dBoardEventsStage::Brush => {
                let brush = self.brush.as_mut().ok_or_else(|| Fault::from("puzzle5d-board-brush-owner-missing"))?;
                match <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::step(brush, input, cx)? {
                    semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Progress { stage, preview } => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Progress { stage, preview }),
                    semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Replay { stage, preview } => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Replay { stage, preview }),
                    semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(emit) => {
                        self.placed_id = emit.interaction_writes.iter().flat_map(|write| write.targets.iter()).map(|target| target.id.clone()).next();
                        let mut mutations = emit.artifact_mutations.into_iter();
                        self.brush_first = mutations.next();
                        self.brush_second = mutations.next();
                        if mutations.next().is_some() {
                            return Err(Fault::from("puzzle5d-board-brush-output-capacity"));
                        }
                        self.stage = Puzzle5dBoardEventsStage::DrainBrush;
                        Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-transfer", "Publishing brush mutation", "Pinselmutation wird veröffentlicht"))
                    }
                    semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteDownload { .. } | semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { .. } => Err(Fault::from("puzzle5d-board-brush-download-unsupported")),
                }
            }
            Puzzle5dBoardEventsStage::DrainBrush => {
                if let Some(mutation) = self.brush_first.take() {
                    self.push(mutation)?;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-transfer", "Publishing brush mutation", "Pinselmutation wird veröffentlicht"));
                }
                if let Some(mutation) = self.brush_second.take() {
                    self.push(mutation)?;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-transfer", "Publishing brush mutation", "Pinselmutation wird veröffentlicht"));
                }
                if let Some(brush) = self.brush.as_mut() {
                    <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::begin_close(brush);
                }
                self.stage = Puzzle5dBoardEventsStage::CloseBrush;
                Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-close", "Releasing brush owners", "Pinseleigentümer werden freigegeben"))
            }
            Puzzle5dBoardEventsStage::CloseBrush => {
                let Some(brush) = self.brush.as_mut() else {
                    self.next_event();
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-event-scan", "Reading board event", "Board-Ereignis wird gelesen"));
                };
                let progress = match <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::close_step(brush, cx.retained_grant()) {
                    semio_framework_job::InteractiveJobCloseStep::Pending { progress } | semio_framework_job::InteractiveJobCloseStep::Complete { progress } => progress,
                    semio_framework_job::InteractiveJobCloseStep::Blocked => return Err(Fault::from("puzzle5d-board-brush-close-blocked")),
                    semio_framework_job::InteractiveJobCloseStep::Refused { .. } => return Err(Fault::from("puzzle5d-board-brush-close-refused")),
                };
                cx.consume_retained(progress).map_err(|_| Fault::from("puzzle5d-board-brush-close-grant"))?;
                if <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::terminal_is_empty(brush) {
                    self.brush.take();
                    self.next_event();
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-close", "Releasing brush owners", "Pinseleigentümer werden freigegeben"))
            }
            Puzzle5dBoardEventsStage::Complete => {
                self.stage = Puzzle5dBoardEventsStage::Closing;
                // 🪟️ A board `camera` row is the pane's own persisted pose. Dropping it (which this work did
                // until 5A2) made pan/zoom in the board window revert on every refresh.
                let camera2d = self.camera2d.take();
                let window_config_mutations = match (camera2d, view_state) {
                    (Some(camera2d), Some(view)) => {
                        let mut next = window_ownership::config_from_snapshot(window_config_snapshot);
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
                let drags = std::mem::take(&mut self.drags);
                let drag_at = self.drag_at.take().unwrap_or(self.mutations.len()).min(self.mutations.len());
                let base = snapshot.typed_arc();
                // 🔒️ A locked part refuses the drag; the rest of the batch still lands, and the leaf names the
                // skipped part as `mutation.partial`.
                self.locked_refused |= drags.iter().any(|record| record.targets.iter().any(|id| base.parts.iter().any(|part| &part.id == id && part.part_2d.locked == Some(true))));
                let request = world3d::utilities::transform::TransformToolRequest { base, records: drags };
                let committed = (!request.records.is_empty()).then(|| world3d::utilities::transform::puzzle5d_transform_tool_commit("applyBoardEvents", &self.authoring_seed, request)).flatten();
                let mut artifact_mutations = std::mem::take(&mut self.mutations);
                let transaction = committed.map(|(transaction, leaves)| {
                    let later = artifact_mutations.split_off(drag_at);
                    artifact_mutations.extend(leaves);
                    artifact_mutations.extend(later);
                    transaction
                });
                let effects = if self.locked_refused {
                    self.locked_refused = false;
                    puzzle5d_notice_emit(view_state, |labels| labels.selection_locked.as_str()).effects
                } else {
                    Vec::new()
                };
                // 🛠️ A committed drag transaction stamps every op of this ONE edit.
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations,
                    window_config_mutations,
                    interaction_writes,
                    effects,
                    transaction: transaction.filter(|_| !self.authoring_seed.is_empty()),
                    ui_scope: UiDirtyScope::Full,
                    ..Default::default()
                }))
            }
            Puzzle5dBoardEventsStage::Closing => Err(Fault::from("puzzle5d-board-events-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dBoardEventsStage::Closing;
        if let Some(brush) = self.brush.as_mut().filter(|brush| !brush.is_closing()) {
            <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::begin_close(brush);
        }
        self.close_owners.stage(Puzzle5dBoardEventsWorkOwners { mutations: std::mem::take(&mut self.mutations), brush_first: std::mem::take(&mut self.brush_first), brush_second: std::mem::take(&mut self.brush_second), event: std::mem::take(&mut self.event), drags: std::mem::take(&mut self.drags), drag_at: std::mem::take(&mut self.drag_at), pending_source: std::mem::take(&mut self.pending_source), pending_target: std::mem::take(&mut self.pending_target), pending_edge_id: std::mem::take(&mut self.pending_edge_id), pending_edge_kind: std::mem::take(&mut self.pending_edge_kind), pending_delete_id: std::mem::take(&mut self.pending_delete_id), camera2d: std::mem::take(&mut self.camera2d), select_ids: std::mem::take(&mut self.select_ids), removed_ids: std::mem::take(&mut self.removed_ids), placed_id: std::mem::take(&mut self.placed_id) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        let Some(brush) = self.brush.as_mut() else { return self.close_owners.close_step(grant) };
        let step = <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::close_step(brush, grant);
        if <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::terminal_is_empty(brush) {
            self.brush.take();
        }
        match step {
            semio_framework_job::InteractiveJobCloseStep::Complete { progress } => semio_framework_job::InteractiveJobCloseStep::Pending { progress },
            other => other,
        }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(brush) = self.brush.as_ref() {
            return <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::next_close_copy_byte_demand(brush);
        }
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(brush) = self.brush.as_ref() {
            return <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::next_close_capacity_byte_demand(brush, maximum_copy_bytes);
        }
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(brush) = self.brush.as_ref() {
            return <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::next_close_release_byte_demand(brush);
        }
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(brush) = self.brush.as_ref() {
            return <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::next_close_depth_demand(brush);
        }
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dBoardEventsStage::Closing && self.brush.is_none() && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
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

/// ♻️ The owners one `Puzzle5dCreateFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dCreateFastenerWorkOwners {
    mutation: Option<Puzzle5dMutation>,
    source_kind: Option<String>,
    target_kind: Option<String>,
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
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dCreateFastenerWorkOwners>,
}

impl Default for Puzzle5dCreateFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dCreateFastenerStage::Source, part_cursor: 0, grip_cursor: 0, fastener_cursor: 0, compatibility_cursor: 0, processed_units: 0, source_kind: None, target_kind: None, mutation: None, operation_nonce: 0 }
    }
}

impl Puzzle5dCreateFastenerWork {
    fn endpoint<'a>(command: &'a Puzzle5dCommand, primary: &str, alias: &str) -> &'a str {
        command.args().and_then(|args| args.get(primary).or_else(|| args.get(alias))).and_then(Value::as_str).filter(|id| !id.is_empty()).unwrap_or("")
    }

    fn scan_grip(&mut self, snapshot: &Puzzle5dPlaySnapshot, target: &str) -> Puzzle5dGripScan {
        let projection = puzzle5d_editor_projection(snapshot);
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

    fn complete_empty(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        self.stage = Puzzle5dCreateFastenerStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default())
    }

    fn arg_f64(command: &Puzzle5dCommand, key: &str) -> f64 {
        command.args().and_then(|args| args.get(key)).and_then(Value::as_f64).unwrap_or(0.0)
    }
}

impl Puzzle5dCreateFastenerWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dCreateFastenerWork {
    fn tool_id(&self) -> &'static str {
        "createFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
        let items = parts.checked_mul(2)?.checked_add(fasteners)?.checked_add(compatibility)?.checked_add(1)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
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
                Puzzle5dGripScan::Progress => Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-source", "Finding source grip", "Quellgriff wird gesucht")),
                Puzzle5dGripScan::Found(kind) => {
                    self.source_kind = kind;
                    self.part_cursor = 0;
                    self.grip_cursor = 0;
                    self.stage = Puzzle5dCreateFastenerStage::Target;
                    Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-target", "Finding target grip", "Zielgriff wird gesucht"))
                }
                Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
            },
            Puzzle5dCreateFastenerStage::Target => match self.scan_grip(snapshot, target) {
                Puzzle5dGripScan::Progress => Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-target", "Finding target grip", "Zielgriff wird gesucht")),
                Puzzle5dGripScan::Found(kind) => {
                    self.target_kind = kind;
                    self.stage = Puzzle5dCreateFastenerStage::Existing;
                    Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"))
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
                    return Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"));
                }
                self.stage = Puzzle5dCreateFastenerStage::Compatibility;
                Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dCreateFastenerStage::Compatibility => {
                if self.source_kind.is_none() || self.target_kind.is_none() {
                    self.stage = Puzzle5dCreateFastenerStage::Emit;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-emit", "Creating fastener", "Verbindung wird erstellt"));
                }
                let rows = projection.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if rows.is_empty() {
                    self.stage = Puzzle5dCreateFastenerStage::Emit;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-emit", "Creating fastener", "Verbindung wird erstellt"));
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
                Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
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
                    Self::arg_f64(command, "y"), None,
                ));
                self.stage = Puzzle5dCreateFastenerStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: self.mutation.take().into_iter().collect(), ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dCreateFastenerStage::Complete => Err(Fault::from("puzzle5d-create-fastener-complete-repolled")),
            Puzzle5dCreateFastenerStage::Closing => Err(Fault::from("puzzle5d-create-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dCreateFastenerStage::Closing;
        self.close_owners.stage(Puzzle5dCreateFastenerWorkOwners { mutation: std::mem::take(&mut self.mutation), source_kind: std::mem::take(&mut self.source_kind), target_kind: std::mem::take(&mut self.target_kind) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dCreateFastenerStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
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

/// ♻️ The owners one `Puzzle5dSetActiveExampleWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dSetActiveExampleWorkOwners {
    mutations: Vec<Puzzle5dMutation>,
    before: Option<std::sync::Arc<Puzzle5dSetActiveExampleBefore>>,
}

struct Puzzle5dSetActiveExampleWork {
    stage: Puzzle5dSetActiveExampleStage,
    cursor: usize,
    admitted: bool,
    mutations: Vec<Puzzle5dMutation>,
    /// 🗂️ The BEFORE document's ids to clear — fastener ids, part ids and compatibility pairs —
    /// harvested ONCE and then merely indexed by the cursored clearing stages.
    ///
    /// 🐛️ `step` used to call `puzzle5d_projection_value` over the run's snapshot on entry, i.e. re-derive the
    /// whole document (`Value` → `DslValue` → os-pack `Value`) on each of the ~110 chunk
    /// steps one example switch takes, and then re-walk its arrays. The run's snapshot is an `Arc`
    /// the retained driver holds FIXED for the whole run (nothing publishes before `Complete`), so
    /// every one of those derivations produced the same bytes — and leaving `🌙️capsule-dream`
    /// (2 880 parts, ~3.5 MB of JSON) paid that whole projection ~110 times for one switch.
    before: Option<std::sync::Arc<Puzzle5dSetActiveExampleBefore>>,
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dSetActiveExampleWorkOwners>,
}

/// 🗂️ Everything the clearing stages need from the BEFORE document, harvested in one pass.
#[derive(Default, semio_framework_value::RetireOwned)]
struct Puzzle5dSetActiveExampleBefore {
    fastener_ids: Vec<String>,
    part_ids: Vec<String>,
    compatibility: Vec<(String, String)>,
}

impl Default for Puzzle5dSetActiveExampleWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dSetActiveExampleStage::ClearFasteners, cursor: 0, admitted: false, mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS), before: None }
    }
}

impl Puzzle5dSetActiveExampleWork {
    /// 📏️ How many semantic units switching to `target` from `snapshot` costs — every old fastener and
    /// part cleared, every old compatibility row dropped, then the target's own compatibility rows, parts
    /// and fasteners, each counted in `PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK`-sized steps, plus the fixed
    /// stage-transition and whole-document rows.
    fn units(command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let target = Self::target(command)?;
        let rows = [
            snapshot.typed().fasteners.len(),
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

    fn compatibility_rows(document: &Puzzle5dDocument) -> &[Value] {
        document.kind_compatibility.as_ref().and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default()
    }

    fn push(&mut self, mutation: Puzzle5dMutation) -> Result<(), Fault> {
        if self.mutations.len() >= PUZZLE5D_SET_ACTIVE_EXAMPLE_MUTATIONS {
            return Err(Fault::from("puzzle5d-set-active-example-output-capacity"));
        }
        self.mutations.push(mutation);
        Ok(())
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dSetActiveExampleWork {
    fn tool_id(&self) -> &'static str {
        "setActiveExample"
    }

    /// 🧯️ An example whose switch costs more than one edit's fixed work capacity is REFUSED with a notice
    /// on the first step, not admitted and faulted: `🌙️capsule-dream` is 2,880 parts + 2,865 fasteners, so
    /// it exceeds `PUZZLE_COMMAND_WORK_ITEMS` (4,096) on its own, and a bare `None` here would surface as
    /// the framework's opaque "exceeds fixed semantic work capacity" fault instead of a sentence the user
    /// can read. The refusal path therefore declares one unit, and `step` completes with the notice.
    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let units = Self::units(command, snapshot)?;
        Some(if units <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS { units } else { 1 })
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction: _interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        if self.before.is_none() {
            let projection = puzzle5d_editor_projection(snapshot);
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
                return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.example_too_large.as_str())));
            }
        }
        // 🔗️ An `Arc` clone, so the cached rows stay readable while the arms below take `&mut self`.
        let before = self.before.clone().unwrap_or_default();
        let Some(target) = Self::target(command) else {
            self.stage = Puzzle5dSetActiveExampleStage::Complete;
            return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.example_too_large.as_str())));
        };
        match self.stage {
            Puzzle5dSetActiveExampleStage::ClearFasteners => {
                let range = Self::take_chunk(&mut self.cursor, before.fastener_ids.len());
                if !range.is_empty() {
                    for id in before.fastener_ids[range].to_vec() {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id))?;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-example-clear-fastener", "Removing old fastener", "Alte Verbindung wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::ClearParts;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-clear-part", "Removing old part", "Altes Teil wird entfernt"))
            }
            Puzzle5dSetActiveExampleStage::ClearParts => {
                let range = Self::take_chunk(&mut self.cursor, before.part_ids.len());
                if !range.is_empty() {
                    for id in before.part_ids[range].to_vec() {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::delete_part(id))?;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-example-clear-part", "Removing old part", "Altes Teil wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::Label;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-label", "Updating document label", "Dokumenttitel wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Label => {
                self.push(crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d(target.label.clone()))?;
                self.stage = Puzzle5dSetActiveExampleStage::Domain;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-domain", "Updating document domain", "Dokumentdomäne wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Domain => {
                self.push(crate::standards::v1::subsets::any::schema::mutations::change_domain(target.domain.clone()))?;
                self.stage = Puzzle5dSetActiveExampleStage::Description;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-description", "Updating description", "Beschreibung wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Description => {
                let description = target.meta.as_ref().and_then(|meta| meta.get("description")).and_then(Value::as_str).unwrap_or("");
                self.push(crate::standards::v1::subsets::any::schema::mutations::change_description(description.to_string()))?;
                self.stage = Puzzle5dSetActiveExampleStage::ClearCompatibility;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-clear-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"))
            }
            Puzzle5dSetActiveExampleStage::ClearCompatibility => {
                let range = Self::take_chunk(&mut self.cursor, before.compatibility.len());
                if !range.is_empty() {
                    for (source, target) in before.compatibility[range].to_vec() {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility(source, target))?;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-example-clear-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::AddCompatibility;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-add-compatibility", "Adding compatibility", "Kompatibilität wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddCompatibility => {
                let rows = Self::compatibility_rows(target);
                let range = Self::take_chunk(&mut self.cursor, rows.len());
                if !range.is_empty() {
                    for row in &rows[range] {
                        let row: crate::Puzzle5dKindCompatibility = <crate::Puzzle5dKindCompatibility as semio_framework_value::FromValue>::from_value(row.to_value()).map_err(|_| Fault::from("puzzle5d-set-active-example-compatibility-malformed"))?;
                        self.push(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(row.source, row.target, row.bidirectional, row.important, row.specificity, None))?;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-example-add-compatibility", "Adding compatibility", "Kompatibilität wird hinzugefügt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::Catalogs;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-catalogs", "Updating kind catalogs", "Artenkataloge werden aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Catalogs => {
                let catalogs = target.kind_catalogs.as_ref().map(|catalogs| <crate::Puzzle5dKindCatalogs as semio_framework_value::FromValue>::from_value(catalogs.to_value())).transpose().map_err(|_| Fault::from("puzzle5d-set-active-example-catalogs-malformed"))?;
                self.push(crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs(catalogs))?;
                self.stage = Puzzle5dSetActiveExampleStage::AddParts;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-add-part", "Adding example part", "Beispielteil wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddParts => {
                let range = Self::take_chunk(&mut self.cursor, target.parts.len());
                if !range.is_empty() {
                    for part in &target.parts[range] {
                        let value = part.to_value();
                        let part = <crate::Puzzle5dPart as semio_framework_value::FromValue>::from_value(value.to_value()).map_err(|_| Fault::from("puzzle5d-set-active-example-part-malformed"))?;
                        self.push(crate::standards::v1::subsets::any::schema::mutations::create_part(part, None))?;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-example-add-part", "Adding example part", "Beispielteil wird hinzugefügt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::AddFasteners;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-add-fastener", "Adding example fastener", "Beispielverbindung wird hinzugefügt"))
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
                            fastener.y, None,
                        ))?;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-example-add-fastener", "Adding example fastener", "Beispielverbindung wird hinzugefügt"));
                }
                self.stage = Puzzle5dSetActiveExampleStage::Complete;
                // 🧹️ Every part the old document held is gone, so a surviving selection would address ids that
                // no longer exist; it is cleared with a SUBTRACTIVE write naming exactly what is selected now.
                let selection = _interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN);
                let targets: Vec<InteractionTarget> = selection
                    .map(|selection| selection.ids.iter().map(|id| InteractionTarget { granularity: selection.granularity.clone(), id: id.clone() }).collect())
                    .unwrap_or_default();
                let interaction_writes = if targets.is_empty() { Vec::new() } else { vec![InteractionWrite { domain: PUZZLE5D_INTERACTION_DOMAIN.into(), targets, merge: MergeMode::Subtractive }] };
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations: std::mem::take(&mut self.mutations),
                    config_mutations: config.mutations_to(&Puzzle5dConfig::default()),
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
        self.close_owners.stage(Puzzle5dSetActiveExampleWorkOwners { mutations: std::mem::take(&mut self.mutations), before: std::mem::take(&mut self.before) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dSetActiveExampleStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

/// 📏️ The retained command route's own wire-admission band, widened off the shared 8 KiB/512 puzzle
/// default to puzzle 3d's (`PUZZLE3D_IMPORT_RAW_BYTES`/`PUZZLE3D_IMPORT_DECODED_ITEMS`): one whole `importSnapshot`
/// file, escaped, plus its envelope (`PUZZLE_IMPORT_RAW_BYTES`) — and a Nakagin-sized document's camera/grid/sun
/// publications, which the narrow band rejects before the job ever admits.
const PUZZLE5D_RETAINED_RAW_BYTES: usize = crate::retained_command::PUZZLE_IMPORT_RAW_BYTES;
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
/// ♻️ The owners one `Puzzle5dRegisterBrushMeshWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dRegisterBrushMeshWorkOwners {
    positions: Vec<f32>,
    indices: Vec<u32>,
}

struct Puzzle5dRegisterBrushMeshWork {
    stage: Puzzle5dRegisterBrushMeshStage,
    position_cursor: usize,
    index_cursor: usize,
    positions: Vec<f32>,
    indices: Vec<u32>,
    close_owners: crate::puzzle_job::WorkClosing<Puzzle5dRegisterBrushMeshWorkOwners>,
}

impl Default for Puzzle5dRegisterBrushMeshWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dRegisterBrushMeshStage::Positions, position_cursor: 0, index_cursor: 0, positions: Vec::new(), indices: Vec::new() }
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

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dRegisterBrushMeshWork {
    fn tool_id(&self) -> &'static str {
        "registerBrushMesh"
    }

    fn extent(&self, command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let items = Self::pages(command, "positions").checked_add(Self::pages(command, "indices"))?.checked_add(3)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot: _snapshot, config: _config, interaction: _interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        match self.stage {
            Puzzle5dRegisterBrushMeshStage::Positions => {
                let page = Self::array(command, "positions");
                let end = page.len().min(self.position_cursor.saturating_add(PUZZLE5D_MESH_PAGE_VALUES));
                self.positions.extend(page[self.position_cursor..end].iter().filter_map(|value| value.as_f64().map(|number| number as f32)));
                self.position_cursor = end;
                if end >= page.len() {
                    self.stage = Puzzle5dRegisterBrushMeshStage::Indices;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-register-brush-mesh-position", "Reading mesh positions", "Mesh-Positionen werden gelesen"))
            }
            Puzzle5dRegisterBrushMeshStage::Indices => {
                let page = Self::array(command, "indices");
                let end = page.len().min(self.index_cursor.saturating_add(PUZZLE5D_MESH_PAGE_VALUES));
                self.indices.extend(page[self.index_cursor..end].iter().filter_map(|value| value.as_u64().and_then(|number| u32::try_from(number).ok())));
                self.index_cursor = end;
                if end >= page.len() {
                    self.stage = Puzzle5dRegisterBrushMeshStage::Derive;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-register-brush-mesh-index", "Reading mesh indices", "Mesh-Indizes werden gelesen"))
            }
            Puzzle5dRegisterBrushMeshStage::Derive => {
                self.stage = Puzzle5dRegisterBrushMeshStage::Complete;
                let url = command.args().and_then(|args| args.get("url")).and_then(Value::as_str).unwrap_or_default().to_string();
                let positions = std::mem::take(&mut self.positions);
                let indices = std::mem::take(&mut self.indices);
                if !register_brush_mesh::puzzle5d_install_brush_mesh(&url, &positions, &indices) {
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.brush_reason_pose_unavailable.as_str())));
                }
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { ui_scope: UiDirtyScope::None, ..Default::default() }))
            }
            Puzzle5dRegisterBrushMeshStage::Complete => Err(Fault::from("puzzle5d-register-brush-mesh-complete-repolled")),
            Puzzle5dRegisterBrushMeshStage::Closing => Err(Fault::from("puzzle5d-register-brush-mesh-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dRegisterBrushMeshStage::Closing;
        self.close_owners.stage(Puzzle5dRegisterBrushMeshWorkOwners { positions: std::mem::take(&mut self.positions), indices: std::mem::take(&mut self.indices) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dRegisterBrushMeshStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
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
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<Puzzle5dPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Puzzle5dPlayApp>>;

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

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > payload.maximum_raw_bytes || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Puzzle 5d retained command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
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
        ArtifactToolPublicationContract { tool_id: "exportSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importSnapshot", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "openAddPartDialog", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "openImportSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "selectSameKindSelection", lanes: &[ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "setFillCount", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSelectionFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionHidden", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionLocked", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "targetBrushSuggestions", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "openVortexSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "closeVortexSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "hoverSuggestion", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "acceptSuggestion", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
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
        ArtifactToolPublicationContract { tool_id: "setTargetVolumeHidden", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetVolumeLocked", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setVoxelDims", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Puzzle5dStorePreparationFactory;

struct Puzzle5dStorePreparation {
    owners: store::OneItemOwners<Puzzle5dPlaySnapshot, Puzzle5dMutation>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    completed_bytes: usize,
    phase: u8,
    cancelled: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Puzzle5dPlaySnapshot, Puzzle5dMutation> for Puzzle5dStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Puzzle5dMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Puzzle5dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    /// 🧾️ The forward row plus the inverse rows the leaf's payload schema declares (`x-semio-inverse-rows`): a selection
    /// leaf one setter per changed pose field of each target, a removal the record and the fasteners it severs.
    fn preflight(&self, mutation: &Puzzle5dMutation, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document {
            return Err("Puzzle5d Store preparation rejected its lane".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint::for_leaf::<Puzzle5dPlaySnapshot, _>(mutation, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &Puzzle5dMutation, _lane: store::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Puzzle5dStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Puzzle5dPlaySnapshot, Puzzle5dMutation, Puzzle5dMutation>,
        grant: store::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::ArtifactStoreOneItemPreparation<Puzzle5dPlaySnapshot, Puzzle5dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::ArtifactStoreOneItemPreparationRequest<Puzzle5dPlaySnapshot, Puzzle5dMutation, Puzzle5dMutation>)> {
        if request.lane != store::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Puzzle5d preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Puzzle5dStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), completed_bytes: 0, phase: 0, cancelled: false }), progress))
    }
}

impl store::ArtifactStoreOneItemPreparation<Puzzle5dPlaySnapshot, Puzzle5dMutation> for Puzzle5dStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::Mutation as _;
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle5d preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() || self.phase >= 2 {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        match self.phase {
            0 => {
                let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d preparation lost its exact base root"))?;
                let mutation = self.owners.mutation.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d preparation lost its mutation owner"))?;
                let inverse = mutation.inverse(base.get())?;
                let post = protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle5d mutation could not produce its post root"))?;
                let mutation = self.owners.mutation.take().expect("observed original mutation owner");
                *self.owners.candidate = Some((post, inverse, mutation));
                self.completed_bytes = 1;
                self.phase = 1;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.completed_bytes as u64, digest: [0; 32] };
                Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress::default()))
            }
            1 => {
                let Some(authority) = self.owners.authority.as_ref() else {
                    return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d preparation lost its Store authority"));
                };
                let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d preparation lost its semantic candidate"))?;
                let prepared = match authority.prepare_one_item(authority.next_edit(mutation, inverse), std::sync::Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => {
                        *self.owners.refused = Some((edit, post));
                        return Err(error);
                    }
                };
                self.phase = 2;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.completed_bytes as u64, digest: prepared.edit_digest() };
                *self.owners.prepared = Some(prepared);
                Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
            }
            _ => Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default())),
        }
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Puzzle5dPlaySnapshot, Puzzle5dMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Puzzle5dPlaySnapshot, Puzzle5dMutation>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Puzzle5dConfigStorePreparationFactory;

struct Puzzle5dConfigStorePreparation {
    owners: store::OneItemOwners<Puzzle5dConfig, Puzzle5dConfigMutation>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Puzzle5dConfig, Puzzle5dConfigMutation> for Puzzle5dConfigStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Puzzle5dConfigMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Puzzle5dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Puzzle5dConfigMutation, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document {
            return Err("Puzzle5d config Store preparation rejected its lane".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &Puzzle5dConfigMutation, _lane: store::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Puzzle5dConfigStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Puzzle5dConfig, Puzzle5dConfigMutation, Puzzle5dConfigMutation>,
        grant: store::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::ArtifactStoreOneItemPreparation<Puzzle5dConfig, Puzzle5dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::ArtifactStoreOneItemPreparationRequest<Puzzle5dConfig, Puzzle5dConfigMutation, Puzzle5dConfigMutation>)> {
        if request.lane != store::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Puzzle5d config preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Puzzle5dConfigStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false }), progress))
    }
}

impl store::ArtifactStoreOneItemPreparation<Puzzle5dConfig, Puzzle5dConfigMutation> for Puzzle5dConfigStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::Mutation as _;
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle5d config preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d config preparation lost its exact base root"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d config preparation lost its mutation owner"))?;
        let inverse = mutation.inverse(base.get())?;
        let post = protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle5d config mutation could not produce its post root"))?;
        let Some(authority) = self.owners.authority.as_ref() else {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d config preparation lost its Store authority"));
        };
        let forward = self.owners.mutation.take().expect("observed original mutation owner");
        let edit = authority.next_edit(forward, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Puzzle5dConfig, Puzzle5dConfigMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Puzzle5dConfig, Puzzle5dConfigMutation>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
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
            "exportSnapshot",
            "importSnapshot",
            "openAddPartDialog",
            "openImportSnapshot",
            "selectSameKindSelection",
            "setFillCount",
            "setSelectionFlag",
            "setSelectionHidden",
            "setSelectionLocked",
            "targetBrushSuggestions",
            "openVortexSuggestions",
            "closeVortexSuggestions",
            "hoverSuggestion",
            "acceptSuggestion",
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
            "setTargetVolumeHidden",
            "setTargetVolumeLocked",
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

    /// 🪧️ A history-edit reference chip names its board/world entity as the outliner does ([`puzzle5d_entity_label`]).
    fn entity_label(snapshot: &Puzzle5dPlaySnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        puzzle5d_entity_label(snapshot.typed(), kinds, id)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        window_ownership::register_transient(registry)
    }



    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
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
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "puzzle5d-command-tool-mismatch"));
        }
        let tool_id = request.command.action_id();
        let nonce = puzzle5d_operation_nonce(&request.operation);
        let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Self>>> = match tool_id {
            // ⏯️ The verbs that start, retarget or abort a run read the instance's live run as of admission.
            "engagementAbort" | "openVortexSuggestions" | "closeVortexSuggestions" => Box::new(Puzzle5dWindowCommandWork::new(tool_id, request.authoring_seed.clone()).bound(request.instance_operation_owner.clone()).with_tool_run(request.context.tool_run().cloned())),
            window if PUZZLE5D_WINDOW_TOOL_IDS.contains(&window) => Box::new(Puzzle5dWindowCommandWork::new(window, request.authoring_seed.clone()).bound(request.instance_operation_owner.clone())),
            "addBrushPart" | "addPartKind" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id).bound(nonce)),
            "applyBoardEvents" => Box::new(Puzzle5dBoardEventsWork::new(request.authoring_seed.clone()).bound(nonce)),
            "translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => Box::new(Puzzle5dTransformWork::new(tool_id, request.authoring_seed.clone())),
            "patchPart" if puzzle5d_inspector_nudge(request.command.args()).is_some() => Box::new(Puzzle5dTransformWork::new(tool_id, request.authoring_seed.clone())),
            "focusSelection" => Box::new(Puzzle5dFocusSelectionWork::default()),
            "patchPart" => Box::new(Puzzle5dPatchPartWork::default()),
            "patchFastener" => Box::new(Puzzle5dPatchFastenerWork::default()),
            "editFastener" => Box::new(Puzzle5dEditFastenerWork::default()),
            "retargetFastener" => Box::new(Puzzle5dRetargetFastenerWork::default()),
            "proximityConnect" => Box::new(Puzzle5dProximityConnectWork::default().bound(nonce)),
            "patchGrip" => Box::new(Puzzle5dPatchGripWork::default()),
            "deleteFastener" => Box::new(Puzzle5dDeleteFastenerWork::default()),
            "addNode" => Box::new(Puzzle5dAddNodeWork::default().bound(nonce)),
            "createFastener" => Box::new(Puzzle5dCreateFastenerWork::default().bound(nonce)),
            "exportSnapshot" => Box::new(Puzzle5dExportWork::default()),
            "setActiveExample" => Box::new(Puzzle5dSetActiveExampleWork::default()),
            "registerBrushMesh" => Box::new(Puzzle5dRegisterBrushMeshWork::default()),
            "setPartKindWeight" | "setGripKindWeight" => Box::new(Puzzle5dKindWeightWork::new(tool_id)),
            "worldPointerDown" | "canvasPointerDown" => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(tool_id)),
            _ => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent)),
        };
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            Puzzle5dCommand::action_id,
            PUZZLE5D_RETAINED_RAW_BYTES,
            crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS,
            work,
        );
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
                ArtifactReservedToolJob::new(Puzzle5dClipboardJob::new(Puzzle5dClipboardVerb::Copy, request, &interaction))
            }
            "cut" => {
                let interaction = match &request.input {
                    ArtifactReservedToolInput::Action { interaction, .. } => interaction.clone(),
                    _ => return Err(Fault::from("puzzle5d cut requires action input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dClipboardJob::new(Puzzle5dClipboardVerb::Cut, request, &interaction))
            }
            "paste" => {
                let args = match &request.input {
                    ArtifactReservedToolInput::Action { args, .. } => args.clone(),
                    _ => return Err(Fault::from("puzzle5d paste requires action input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dPasteJob::new(request, args.map(|value| semio_framework_pack_json::from_dsl_value(&value))))
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
    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
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
    /// each static is a `LazyLock`, forced only when its example is switched to. That matrix has since been
    /// deleted outright.
    fn initial_snapshot() -> Puzzle5dPlaySnapshot {
        Puzzle5dPlaySnapshot::new(puzzle5d_snapshot_from_document(&default_document()).expect("authored initial document admits"))
    }

    fn clipboard_media_type() -> Option<MediaType> {
        Some(MediaType { class: MediaClass::Kit, form: MediaForm::Design })
    }

    fn copy_fragment(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        let (part_ids, fastener_ids) = puzzle5d_interaction_part_and_fastener_ids(interaction);
        puzzle5d_copy_fragment(doc.snapshot, &part_ids, &fastener_ids)
    }

    /// ✂️ B1: `ArtifactApp::cut_operations`'s signature carries no config output channel (it
    /// returns a bare `Vec<Self::Mutation>`, not an `Emit`), so this can only emit the document
    /// removal; clearing the selection is left to the framework's own post-cut selection reconciliation
    /// (the cut parts/fasteners are gone from the document either way, so a stale selection referencing
    /// them is inert until the next real selection action overwrites it).
    fn cut_operations(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>, interaction: &InteractionView<'_>) -> Vec<Puzzle5dMutation> {
        let (part_ids, fastener_ids) = puzzle5d_interaction_part_and_fastener_ids(interaction);
        puzzle5d_cut_operations(doc.snapshot, &part_ids, &fastener_ids)
    }

    /// 📋️ B1: `ArtifactApp::paste_operations` carries no `ConfigView` at all (only `doc`/
    /// `fragment`/`placement`), so the new selection can't be threaded through this call; a following
    /// `setSelection` command (which the host already issues after a paste in practice) is what
    /// actually selects the pasted parts now.
    fn paste_operations(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle5dMutation>, ClipboardError> {
        let expected = Self::clipboard_media_type().unwrap_or(MediaType { class: MediaClass::Kit, form: MediaForm::Design });
        if fragment.media_type != expected {
            return Err(ClipboardError::IncompatibleMediaType(fragment.media_type));
        }
        let fragment_value: Value = parse(&fragment.dsl_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_parts: Vec<Puzzle5dPart> = puzzle5d_record_from_projection(fragment_value.get("parts").cloned().unwrap_or_else(|| semio_framework_pack_json::json!([]))).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_fasteners: Vec<Puzzle5dFastener> = puzzle5d_record_from_projection(fragment_value.get("fasteners").cloned().unwrap_or_else(|| semio_framework_pack_json::json!([]))).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let before = puzzle5d_editor_projection(&doc.snapshot);
        let document: Puzzle5dDocument = <Puzzle5dDocument as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&before)).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let delta = paste_delta_2d(&fragment_parts, &document.parts, placement);
        let (fresh_parts, fresh_fasteners) = paste_selection_local(&document, &fragment_parts, &fragment_fasteners, delta);
        puzzle5d_paste_mutations(fresh_parts, fresh_fasteners).map_err(|error| ClipboardError::ParseFailed(error.to_string()))
    }

    /// 🏷️ Maps each `Puzzle5dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle5dCommand) -> &'static str {
        command.action_id()
    }

    /// 📢️ The localized notices of this guest's refusal codes ([`puzzle5d_fault_notices`]).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        puzzle5d_fault_notices()
    }

    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        if let Some(flag) = puzzle5d_flag_value_argument(action) {
            args.and_then(|value| value.get(flag)).and_then(semio_framework_value::DslValue::as_bool).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("puzzle5d.action.flag-value-required"), format!("action '{action}' requires the boolean '{flag}' it sets")))?;
        }
        let args = args.map(semio_framework_pack_json::from_dsl_value);
        let window_id = args.as_ref().and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(Value::as_str).map(str::to_string);
        Puzzle5dCommand::try_from_action(action, args, window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 5D action '{action}'")))
    }

    /// 🧩️ Thin typed-command adapter — reconstructs the exact `(action, args, window_id)`
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
        let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
        with_puzzle5d_app(|app| Ok(app.handle_action_impl(authoring_seed, command.action_id(), command.args(), command.window_id(), doc.snapshot, &runtime, view_state, selection, None, None).0))
    }

    /// 🕹️ `vortex` domain topology (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
    /// parts and fasteners as flat roots, grips nested under their owning part (mirrors puzzle3d's
    /// object→vortex-marker nesting).
    fn interaction_topology(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>) -> Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let document: Puzzle5dDocument = puzzle5d_document_from_snapshot(doc.snapshot.typed()).expect("admitted snapshot projects to native editor document");
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
    
})())
}

    /// 🔌️ Declares puzzle5d's typed media I/O surface: the implicit document ports (from
    /// `.document([...])`/`.artifact_kind(...)` in `create_puzzle5d_app`) plus `kit:in` (accepting a
    /// `kit.catalog` fragment shaped like block3d's `puzzle3d_catalog_fragment`, fanning IN from
    /// potentially many producers) and `design:out` (this app's own `5d.puzzle` design artifact, fanning
    /// OUT to potentially many consumers).
    fn io() -> Option<AppIo> {
        let io = ::semio_framework_async::poll::resolve_ready(AppIo::from_artifact(
            "puzzle.5d",
            MediaType { class: MediaClass::Kit, form: MediaForm::Design },
            ArtifactPresentation { id: "5d.puzzle".into(), name: "5D Puzzle".into(), dimension: "5d".into(), component_kind: "puzzle5d".into() },
        ));
        Some(::semio_framework_async::poll::resolve_ready(io.with_ports(vec![
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
        let projection = puzzle5d_editor_projection(&doc.snapshot);
        let window_for_body = if body_key == board2d::BODY_KEY { board2d::WINDOW_KIND_ID } else { world3d::WINDOW_KIND_ID };
        let window_id = view_state.window_id.as_deref().unwrap_or(window_for_body);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        let labels = puzzle5d_labels(view_state).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.localization.unsupported", "puzzle5d has no authored label set for the host's locale/terminology axes"))?;
        // 🪟️ One `TreeWindows` per render, read off the host's `ViewModel::tree_windows` for exactly
        // the body being rendered — every panel container below shares its first-paint row budget.
        let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
        let node = match body_key {
            board2d::BODY_KEY => board2d::render(&envelope, labels, None),
            world3d::BODY_KEY => world3d::render(&envelope, labels, doc.tool_run(), &crate::editor::puzzle5d::precompute::puzzle5d_mesh_lane(doc.snapshot, &envelope.document), None),
            artifact_panel::BODY_KEY => artifact_panel::render(&envelope, labels, &windows),
            catalogue::BODY_KEY => catalogue::render(&envelope, labels, &windows),
            inspection::BODY_KEY => inspection::render(&envelope, labels, &windows),
            settings_panel::BODY_KEY => settings_panel::render(&envelope, labels, settings_panel::panel_window_id(view_state)),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d unknown-body label admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let projection = puzzle5d_editor_projection(&doc.snapshot);
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
        // 🎣️ What the brush suggestions run resolved for the open menu's grip — the instance owner holds it, and both
        // panes list it.
        let menu_target = envelope.runtime.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.as_str());
        let suggestions = menu_target.and_then(|target| owner.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(owner.brush_suggestions.found(target).cloned())).ok().flatten());
        let node = match body_key {
            board2d::BODY_KEY => board2d::render(&envelope, labels, suggestions.as_ref()),
            world3d::BODY_KEY => world3d::render(&envelope, labels, doc.tool_run(), &crate::editor::puzzle5d::precompute::puzzle5d_mesh_lane(doc.snapshot, &envelope.document), suggestions.as_ref()),
            artifact_panel::BODY_KEY => artifact_panel::render(&envelope, labels, &windows),
            catalogue::BODY_KEY => catalogue::render(&envelope, labels, &windows),
            inspection::BODY_KEY => inspection::render(&envelope, labels, &windows),
            settings_panel::BODY_KEY => settings_panel::render(&envelope, labels, settings_panel::panel_window_id(view_state)),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d unknown-body label admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    fn window_engagements(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let projection = puzzle5d_editor_projection(&doc.snapshot);
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
        let envelope = scene_from_projection(&puzzle5d_editor_projection(&doc.snapshot), runtime, &active_utility);
        HashMap::from([(window_id.to_string(), edit::puzzle5d_engagement(&envelope, window_kind, labels, doc.tool_run()))])
    }

    fn window_measures(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let projection = puzzle5d_editor_projection(&doc.snapshot);
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
        let envelope = scene_from_projection(&puzzle5d_editor_projection(&doc.snapshot), runtime, &active_utility);
        HashMap::from([(fill_tool::TOOL_ID.to_string(), fill_tool::measures(&envelope, labels, doc.tool_run()))])
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &Puzzle5dInteractionSnapshot::default(), registry)
    }

    /// 🕹️ The live-`vortex`-domain twin: a grip or fastener selected in EITHER pane reaches the menu even though
    /// the world host's surface only reports its painted part ids.
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &Puzzle5dInteractionSnapshot::from_interaction(interaction), registry)
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

/// 📢️ The localized notice of every refusal code the puzzle5d guest names (design §20.12): a set-verb's missing flag value.
fn puzzle5d_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: LazyLock<[(&str, LocalizedLabel); 1]> = LazyLock::new(|| {
        [
            ("puzzle5d.action.flag-value-required", LocalizedLabel::native("Choose on or off for this setting.", "Für diese Einstellung Ein oder Aus wählen.")),
        ]
    });
    &*NOTICES
}

/// 🗨️ The one `partKind` select both the standalone `addPartKind` arg form and the "Add Part" dialog
/// declare — built twice from the same catalog so the two forms can never drift apart.

/// 🙈️ The flag a set-verb sets to exactly the boolean its arguments carry (`setSelectionHidden{hidden}`,
/// `setTargetVolumeLocked{locked}`, …) — the row target's explicit next state, so a stale view sets a value and never flips one.
fn puzzle5d_flag_value_argument(action: &str) -> Option<&'static str> {
    match action {
        "setSelectionHidden" | "setTargetVolumeHidden" => Some("hidden"),
        "setSelectionLocked" | "setTargetVolumeLocked" => Some("locked"),
        _ => None,
    }
}

/// 🙈️ A set-verb's definition: the explicit identity a row names (or, left empty, the live selection) and the REQUIRED
/// boolean `flag` it sets — a missing value is refused, never defaulted.
fn puzzle5d_flag_value_action(id: &str, identity: Vec<ActionArgDef>, flag: &str, label: LocalizedLabel, value: LocalizedLabel) -> ActionDefinition {
    let mut args = identity;
    args.push(ActionArgDef::toggle(flag, value).required());
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, ActionKind::Mutation).with_category("settings").with_args(args) }
}

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
            granularity(PUZZLE5D_GRANULARITY_TARGET_VOLUME, LocalizedLabel::native("Target Volume", "Zielvolumen"), "box-select"),
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
    Editor::builder(Puzzle5dPlayApp::DIALECT)
            .document(["semio", "puzzle", "5d"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "puzzle", "5d"])
            .mode_def(edit::definition())
            .default_mode_id(edit::PUZZLE5D_PLAY_MODE_EDIT)
            .window_kind_def(board2d::definition())
            .window_kind_def(world3d::definition())
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
            // 🏗️ 2D-first 40/60 split — board pane left (diagram 40%), world pane right (scene 60%).
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
            // 📤️📥️ Document IO. `exportSnapshot`/`openImportSnapshot` are SHELL verbs (a download, a file
            // picker — no document mutation of their own); `importSnapshot` is the mutation the picker
            // re-dispatches once per wire page and never a menu row of its own, so the user-facing
            // "Import" row is the one that actually opens a picker.
            .action_with(ActionDefinition::bounded_catalog("exportSnapshot", puzzle5d_localized(|l| l.export), ActionKind::Shell).with_category("file"))
            .action_with(ActionDefinition::bounded_catalog("openImportSnapshot", puzzle5d_localized(|l| l.import), ActionKind::Shell).with_category("file"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importSnapshot", puzzle5d_localized(|l| l.import), ActionKind::Mutation) })
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
            .action_with(puzzle5d_flag_value_action("setSelectionHidden", vec![ActionArgDef::text("entity", LocalizedLabel::native("Entity", "Entität")), ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs"))], "hidden", LocalizedLabel::native("Set Hidden", "Verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle5d_flag_value_action("setSelectionLocked", vec![ActionArgDef::text("entity", LocalizedLabel::native("Entity", "Entität")), ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs"))], "locked", LocalizedLabel::native("Set Locked", "Gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
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
            .action_with(puzzle5d_flag_value_action("setTargetVolumeHidden", vec![ActionArgDef::text("id", LocalizedLabel::native("Id", "ID"))], "hidden", LocalizedLabel::native("Set Target Volume Hidden", "Zielvolumen verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle5d_flag_value_action("setTargetVolumeLocked", vec![ActionArgDef::text("id", LocalizedLabel::native("Id", "ID"))], "locked", LocalizedLabel::native("Set Target Volume Locked", "Zielvolumen gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
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
            .view_action("openVortexSuggestions", LocalizedLabel::native("Open Grip Suggestions", "Griff-Vorschläge öffnen"))
            .view_action("closeVortexSuggestions", LocalizedLabel::native("Close Grip Suggestions", "Griff-Vorschläge schließen"))
            .view_action("hoverSuggestion", LocalizedLabel::native("Hover Suggestion", "Vorschlag überfahren"))
            .action_audience("hoverSuggestion", semio_framework_plugin::CapabilityAudience::Input)
            .mutation("acceptSuggestion", LocalizedLabel::native("Accept Suggestion", "Vorschlag annehmen"))
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
            .action_audience("setTransformGumballFlag", semio_framework_plugin::CapabilityAudience::Chrome)
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
            .action_interactive_job("exportSnapshot", InteractiveJobClassification::Migrated)
            .action_destructive("exportSnapshot")
            .action_interactive_job("importSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("openAddPartDialog", InteractiveJobClassification::Migrated)
            .action_interactive_job("openImportSnapshot", InteractiveJobClassification::Migrated)
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
            .action_interactive_job("setSelectionHidden", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionLocked", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("setPartKindWeight", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSuggestionOffset", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGripKindWeight", InteractiveJobClassification::Migrated)
            .action_interactive_job("targetBrushSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("openVortexSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("closeVortexSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("hoverSuggestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("acceptSuggestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("addTargetVolume", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteTargetVolume", InteractiveJobClassification::Migrated)
            .action_interactive_job("relocateTargetVolume", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetVolumeFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetVolumeHidden", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetVolumeLocked", InteractiveJobClassification::Migrated)
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
            .utility(world3d::utilities::transform::definition())
            .utility(board2d::utilities::brush::definition(puzzle5d_localized(|l| l.brush)))
            .utility(world3d::utilities::volume_brush::definition(puzzle5d_localized(|l| l.volume_brush)))
            .utility(world3d::utilities::world_relocate::definition())
            // 🛠️ Fill is a mode-level TOOL (a whole-document generator over both projections), not a window
            // utility — it keeps its viewport interaction in both panes through the host's `active_tool_id`.
            .tool(fill_tool::definition(puzzle5d_localized(|l| l.fill)))
            .mode_tools(edit::PUZZLE5D_PLAY_MODE_EDIT, vec![::semio_framework_async::poll::resolve_ready(ToolRef::new(fill_tool::TOOL_ID))])
    .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole 5D puzzle with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte 5D-Puzzle durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
    .action_describe("deleteSelection", LocalizedLabel::native("Deletes every selected part from the 5D puzzle together with its connections.", "Löscht alle ausgewählten Teile samt ihrer Verbindungen aus dem 5D-Puzzle."))
    .action_describe("duplicateSelection", LocalizedLabel::native("Adds a copy of every selected part next to the original; nothing happens without a selection.", "Fügt neben jedem ausgewählten Teil eine Kopie hinzu; ohne Auswahl geschieht nichts."))
    .action_describe("focusSelection", LocalizedLabel::native("Frames the camera on the selected parts; only the view changes.", "Richtet die Kamera auf die ausgewählten Teile aus; nur die Ansicht ändert sich."))
    .action_describe("translateSelection", LocalizedLabel::native("Moves the selected parts by dx and dy.", "Verschiebt die ausgewählten Teile um dx und dy."))
    .action_describe("rotateSelection", LocalizedLabel::native("Rotates the selected parts by the given angle.", "Dreht die ausgewählten Teile um den angegebenen Winkel."))
    .action_describe("scaleSelection", LocalizedLabel::native("Scales the selected parts by the given factor.", "Skaliert die ausgewählten Teile um den angegebenen Faktor."))
    .action_describe("exportSnapshot", LocalizedLabel::native("Writes the whole 5D puzzle as JSON to a downloaded file named after the active example on the user's machine.", "Schreibt das gesamte 5D-Puzzle als JSON in eine heruntergeladene, nach dem aktiven Beispiel benannte Datei auf dem Rechner des Nutzers."))
    .action_describe("openImportSnapshot", LocalizedLabel::native("Opens the host's file picker for a 5D puzzle JSON file; the chosen file then replaces the whole puzzle.", "Öffnet die Dateiauswahl des Hosts für eine 5D-Puzzle-JSON-Datei; die gewählte Datei ersetzt dann das gesamte Puzzle."))
    .action_describe("importSnapshot", LocalizedLabel::native("Replaces the whole 5D puzzle with one read from an imported JSON file; the previous puzzle is discarded.", "Ersetzt das gesamte 5D-Puzzle durch eines aus einer importierten JSON-Datei; das bisherige Puzzle wird verworfen."))
    .action_describe("setSelectionFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given or selected parts.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen oder ausgewählten Teile."))
    .action_describe("setSelectionHidden", LocalizedLabel::native("Sets the given or selected parts hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt die angegebenen oder ausgewählten Teile oder zeigt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
    .action_describe("setSelectionLocked", LocalizedLabel::native("Sets the given or selected parts locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt die angegebenen oder ausgewählten Teile oder entsperrt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
    .action_describe("acceptSuggestion", LocalizedLabel::native("Places the suggested piece chosen from the suggestion list (by index, or the highlighted one) at its connection point.", "Setzt das aus der Vorschlagsliste gewählte Teil (per Index oder das hervorgehobene) an seinem Anschlusspunkt."))
    .action_describe("selectSameKindSelection", LocalizedLabel::native("Extends the selection to every piece of the same kind as the selected one.", "Erweitert die Auswahl auf alle Teile derselben Art wie das ausgewählte."))
    .action_describe("toggleSun", LocalizedLabel::native("Switches the 3D view's sun light on or off; only the view changes.", "Schaltet das Sonnenlicht der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
    .action_describe("setSunAzimuth", LocalizedLabel::native("Sets the compass direction the 3D view's sun shines from; only the view changes.", "Legt die Himmelsrichtung fest, aus der die Sonne der 3D-Ansicht scheint; nur die Ansicht ändert sich."))
    .action_describe("setSunElevation", LocalizedLabel::native("Sets how high the 3D view's sun stands above the horizon; only the view changes.", "Legt fest, wie hoch die Sonne der 3D-Ansicht über dem Horizont steht; nur die Ansicht ändert sich."))
    .action_describe("setSunIntensity", LocalizedLabel::native("Sets the brightness of the 3D view's sun; only the view changes.", "Legt die Helligkeit der Sonne der 3D-Ansicht fest; nur die Ansicht ändert sich."))
    .action_describe("setLodAutomatic", LocalizedLabel::native("Turns automatic level of detail in the 3D view on or off; only the view changes.", "Schaltet die automatische Detailstufe der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
    .action_describe("setLodDepthVariable", LocalizedLabel::native("Turns depth-dependent level of detail in the 3D view on or off; only the view changes.", "Schaltet die tiefenabhängige Detailstufe der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
    .action_describe("setLodManual", LocalizedLabel::native("Sets a fixed level of detail for the 3D view; only the view changes.", "Legt eine feste Detailstufe für die 3D-Ansicht fest; nur die Ansicht ändert sich."))
    .action_describe("setGridVisible", LocalizedLabel::native("Shows or hides the placement grid in the 3D view; only the view changes.", "Blendet das Platzierungsraster in der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
    .action_describe("setGridSnapEnabled", LocalizedLabel::native("Turns snapping to the placement grid on or off for moves in the 3D view.", "Schaltet das Fangen am Platzierungsraster für Verschiebungen in der 3D-Ansicht ein oder aus."))
    .action_describe("setGridSpacing", LocalizedLabel::native("Sets the spacing of the placement grid.", "Legt den Abstand des Platzierungsrasters fest."))
    .action_describe("setProximityRadius", LocalizedLabel::native("Sets how close two pieces must come for a move or Connect Nearby to join their connection points.", "Legt fest, wie nahe sich zwei Teile kommen müssen, damit ein Verschieben oder In der Nähe verbinden ihre Anschlusspunkte verbindet."))
    .action_describe("setChunkSize", LocalizedLabel::native("Sets the size of the spatial chunks the 3D view loads and draws pieces in; only the view changes.", "Legt die Größe der räumlichen Blöcke fest, in denen die 3D-Ansicht Teile lädt und zeichnet; nur die Ansicht ändert sich."))
    .action_describe("setSelectableKind", LocalizedLabel::native("Sets whether pieces, connection points or attractions can be picked in the 3D view.", "Legt fest, ob Teile, Anschlusspunkte oder Anziehungen in der 3D-Ansicht gewählt werden können."))
    .action_describe("setTransformGumballFlag", LocalizedLabel::native("Switches one option of the transform gumball (such as translate, rotate or scale handles) on or off.", "Schaltet eine Option des Transformationsgriffs (etwa Verschiebe-, Dreh- oder Skaliergriffe) ein oder aus."))
    .action_describe("setVoxelDims", LocalizedLabel::native("Sets one dimension (width, depth or height) of the voxel volume the fill tool packs pieces into.", "Legt eine Abmessung (Breite, Tiefe oder Höhe) des Voxelvolumens fest, in das das Füllwerkzeug Teile packt."))
    .action_describe("relocateTargetVolume", LocalizedLabel::native("Moves one target volume to a new position.", "Verschiebt ein Zielvolumen an eine neue Position."))
    .action_describe("setFillCount", LocalizedLabel::native("Sets how many pieces the fill tool places into the target volume.", "Legt fest, wie viele Teile das Füllwerkzeug in das Zielvolumen setzt."))
    .action_describe("setBrushPlacementContactTolerance", LocalizedLabel::native("Sets how much overlap (0 to 1) the placement brush tolerates between a suggested piece and existing ones.", "Legt fest, wie viel Überlappung (0 bis 1) der Platzierungspinsel zwischen einem vorgeschlagenen und vorhandenen Teilen zulässt."))
    .action_describe("cycleBrushCandidate", LocalizedLabel::native("Switches the placement brush to the next suggested piece.", "Schaltet den Platzierungspinsel zum nächsten vorgeschlagenen Teil."))
    .action_describe("cycleBrushCandidateBack", LocalizedLabel::native("Switches the placement brush to the previous suggested piece.", "Schaltet den Platzierungspinsel zum vorherigen vorgeschlagenen Teil."))
    .action_describe("openVortexSuggestions", LocalizedLabel::native("Opens the list of pieces that fit at one connection point.", "Öffnet die Liste der Teile, die an einen Anschlusspunkt passen."))
    .action_describe("closeVortexSuggestions", LocalizedLabel::native("Closes the list of suggested pieces.", "Schließt die Liste der vorgeschlagenen Teile."))
    .action_describe("addTargetVolume", LocalizedLabel::native("Adds a target volume, a box region the fill tool packs pieces into.", "Fügt ein Zielvolumen hinzu, einen Quaderbereich, in den das Füllwerkzeug Teile packt."))
    .action_describe("deleteTargetVolume", LocalizedLabel::native("Deletes one target volume by id.", "Löscht ein Zielvolumen anhand seiner Id."))
    .action_describe("setTargetVolumeFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given target volumes.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen Zielvolumen."))
    .action_describe("setTargetVolumeHidden", LocalizedLabel::native("Sets the given target volume hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt das angegebene Zielvolumen oder zeigt es, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
    .action_describe("setTargetVolumeLocked", LocalizedLabel::native("Sets the given target volume locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt das angegebene Zielvolumen oder entsperrt es, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
    .action_describe("addNode", LocalizedLabel::native("Adds a part of the given kind (a 5D block kind) to the puzzle at x, y.", "Fügt dem Puzzle an x, y ein Teil der angegebenen Art (einer 5D-Blockart) hinzu."))
    .action_describe("proximityConnect", LocalizedLabel::native("Fastens every pair of compatible grips that lie within the proximity radius of each other.", "Verbindet jedes Paar verträglicher Griffe, die innerhalb des Näheradius beieinander liegen, mit Verbindern."))
    .action_describe("worldRelocate", LocalizedLabel::native("Moves one part to a new position and fastens it to nearby compatible grips.", "Verschiebt ein Teil an eine neue Position und verbindet es mit verträglichen Griffen in der Nähe."))
    .action_describe("openAddPartDialog", LocalizedLabel::native("Opens the Add Part dialog to pick a part kind to place.", "Öffnet den Dialog Teil hinzufügen, um eine zu setzende Teileart zu wählen."))
    .action_describe("addPartKind", LocalizedLabel::native("Adds a part of the given kind to the puzzle.", "Fügt dem Puzzle ein Teil der angegebenen Art hinzu."))
    .action_describe("addBrushPart", LocalizedLabel::native("Places a part of the given kind with the placement brush where the brush suggests.", "Setzt mit dem Platzierungspinsel ein Teil der angegebenen Art dort, wo der Pinsel es vorschlägt."))
    .action_describe("patchPart", LocalizedLabel::native("Sets one field on the given or selected parts.", "Setzt ein Feld auf den angegebenen oder ausgewählten Teilen."))
    .action_describe("patchGrip", LocalizedLabel::native("Sets one field on the given or selected grips.", "Setzt ein Feld auf den angegebenen oder ausgewählten Griffen."))
    .action_describe("patchFastener", LocalizedLabel::native("Sets one field on the given or selected fasteners.", "Setzt ein Feld auf den angegebenen oder ausgewählten Verbindern."))
    .action_describe("createFastener", LocalizedLabel::native("Creates a fastener joining two compatible grips of different parts.", "Erstellt einen Verbinder, der zwei verträgliche Griffe verschiedener Teile verbindet."))
    .action_describe("deleteFastener", LocalizedLabel::native("Deletes one fastener by id, releasing its two grips.", "Löscht einen Verbinder anhand seiner Id und gibt seine zwei Griffe frei."))
    .action_describe("retargetFastener", LocalizedLabel::native("Moves one end of a fastener to another compatible grip.", "Hängt ein Ende eines Verbinders an einen anderen verträglichen Griff um."))
    .action_describe("editFastener", LocalizedLabel::native("Sets the gap, shift or rise of one fastener, absolutely or by a delta.", "Setzt Abstand, Versatz oder Anhebung eines Verbinders, absolut oder um eine Differenz."))
    .action_describe("setPartKindWeight", LocalizedLabel::native("Sets how often the fill tool and brush pick one part kind relative to the others.", "Legt fest, wie oft Füllwerkzeug und Pinsel eine Teileart im Verhältnis zu den anderen wählen."))
    .action_describe("setGripKindWeight", LocalizedLabel::native("Sets how strongly one grip kind is preferred when suggesting placements.", "Legt fest, wie stark eine Griffart bei Platzierungsvorschlägen bevorzugt wird."))
    .action_describe("setLodMode", LocalizedLabel::native("Sets the level-of-detail mode of the views; only the view changes.", "Legt den Detailstufenmodus der Ansichten fest; nur die Ansicht ändert sich."))
    .action_describe("setSuggestionOffset", LocalizedLabel::native("Sets how far suggested parts are offset from the grip they would attach to.", "Legt fest, wie weit vorgeschlagene Teile vom Griff, an den sie ansetzen würden, versetzt werden."))
    .action_describe("setGridFactor", LocalizedLabel::native("Sets the spacing factor of the 2D view's snap grid; only the view setting changes.", "Legt den Abstandsfaktor des Fangrasters der 2D-Ansicht fest; nur die Ansichtseinstellung ändert sich."))
    .action_describe("setGripShow", LocalizedLabel::native("Sets whether grip markers are shown always or only on selected parts; only the view changes.", "Legt fest, ob Griffmarken immer oder nur an ausgewählten Teilen gezeigt werden; nur die Ansicht ändert sich."))
    .action_describe("setGripDirection", LocalizedLabel::native("Sets how grip directions are drawn; only the view changes.", "Legt fest, wie Griffrichtungen gezeichnet werden; nur die Ansicht ändert sich."))
    .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("setProjection", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("setProjectionParam", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("engagementRepeatLast", semio_framework_plugin::CapabilityAudience::Input)
    .action_audience("engagementControlSelect", semio_framework_plugin::CapabilityAudience::Input)
    .action_audience("targetBrushSuggestions", semio_framework_plugin::CapabilityAudience::Input)
    .action_audience("registerBrushMesh", semio_framework_plugin::CapabilityAudience::Input)
    .action_audience("setCamera2d", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("setCamera3d", semio_framework_plugin::CapabilityAudience::Chrome)
    .action_audience("applyBoardEvents", semio_framework_plugin::CapabilityAudience::Input)
    .action_destructive("importSnapshot")
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

#[cfg(test)]
#[path = "🧪️tests/🤖️agent-lane/🦀️.rs"]
mod agent_lane_tests;

/// 🧊️ The target-volume / Volume-Brush laws — a topic of its own so the one shared harness stays the
/// only scaffold and this family's laws are readable as one block.
#[cfg(test)]
#[path = "🧪️tests/🔬️target-volumes/🦀️.rs"]
mod target_volume_tests;
//#endregion 🧪️UnitTests

#[cfg(test)]
#[path = "🧪️tests/🧵️retained-wiring/🦀️.rs"]
mod retained_wiring_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️host-admission/🦀️.rs"]
mod host_admission_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️gesture-kinds/🦀️.rs"]
mod gesture_kinds;
