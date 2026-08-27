//! 👯️ Puzzle 5d play app — the plugin's unified 2d+3d play app: its `ArtifactApp` impl
//! (dispatch-only), the structural-twin document model its command/panel/window nodes mutate and
//! render, the shared scene/engine/brush helpers those nodes reach for, and the manifest that
//! stitches them together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️component.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/{◻2d,🧊️3d}`. This file dispatches and stitches.
//!
//! 🌉️ `ArtifactApp::Snapshot` is the `Puzzle5dPlaySnapshot` newtype over a bare
//! `serde_json::Value` document (see `crate::artifacts::puzzle5d::op`'s `🔖️ValueBridge`), not the
//! typed `Puzzle5dSnapshot` — the `Puzzle5dDocument` model below is this app's own structural twin
//! of it, and each action emits the granular typed operation delta
//! (`puzzle5d_operations_from_document_change`) turning the old document into the new one.

use crate::artifacts::puzzle5d::op::{puzzle5d_document_delta_operations, Puzzle5dMutation, Puzzle5dPlaySnapshot};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use crate::editor::puzzle5d::commands::{
    add_brush_part, add_node, add_part_kind, apply_board_events, apply_sun, create_fastener, cycle_brush_candidate, delete_fastener, delete_selection, duplicate_selection, edit_fastener, engagement_abort, engagement_control_select, engagement_input,
    engagement_submit, patch_fastener, patch_grip, patch_part, proximity_connect, register_brush_mesh, retarget_fastener, rotate_selection, scale_selection, select_same_kind, set_active, set_active_example, set_brush_placement_overlap_budget,
    set_camera, set_camera_2d, set_camera_3d, set_fill_count, set_fixture_json, set_grid_factor, set_grid_snap_enabled, set_kind_weight, set_lod_mode, set_selection_flag, set_suggestion_offset, translate_selection, world_relocate, zoom_to_selection,
};
use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dCamera3d, Puzzle5dConfig, Puzzle5dConfigMutation, Puzzle5dRuntime};
use crate::editor::puzzle5d::modes::edit;
use crate::editor::puzzle5d::modes::edit::windows::{board2d, world3d};
use crate::editor::puzzle5d::panels::{catalogue, document as document_panel, inspection};
use crate::editor::puzzle5d::precompute::{BrushPlacePayload, Puzzle5dPrecomputeSession};
use crate::editor::puzzle5d::presence::{Puzzle5dPresence, Puzzle5dPresenceMutation};
use crate::editor::puzzle5d::terminology::{puzzle5d_is_de_locale, puzzle5d_labels, puzzle5d_localized, Puzzle5dLabels};
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PasteAnchor, PastePlacement, UiDirtyScope};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppIo, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactPresentation, ArtifactReservedJob, ArtifactReservedToolInput, ArtifactReservedToolJob,
    ArtifactReservedToolJobRequest, ArtifactToolCompletion, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, DraftView, Editor, EditorApp, Emit, EphemeralEmit, Fault,
    GranularityDefinition, HierarchyProvider, HoverSpec, IconName,
    InteractionDefinition, InteractionRef, InteractionTarget, InteractiveJobClassification, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, MergeMode, NoDraft, NoDraftMutation,
    PluginCloseStep, PortMultiplicity, SelectionMethod, SelectionMode, SelectionSpec, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, UiNode, UiTreeItemNode, WindowEngagement, WindowMeasure, INTERACTION_SELECT_ACTION_ID,
    SET_ACTIVE_UTILITY_ACTION_ID,
};
// 🕹️ `InteractionView` — see 🧊️3d/🦀️component.rs's identical import comment (missing top-level
// re-export from `semio_framework_plugin`, flagged to the coordinator, not fixed here).
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, JobFault, JobPayloadAdmissionFault, JobPayloadCloseStep, JobPayloadStream, Operation, RetainedJobPayload, RetainedJobPayloadWriter, StepContext, StepOutcome};
use semio_framework_plugin::app::InteractionView;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::cell::RefCell;
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
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the one interaction domain this app
/// declares — the previously-separate `Puzzle5dSelection` bags (part/grip/fastener) collapse into
/// one framework-owned domain, distinguished by `DomainSelection.granularity`.
pub const PUZZLE5D_INTERACTION_DOMAIN: &str = "vortex";
pub const PUZZLE5D_GRANULARITY_PART: &str = "part";
pub const PUZZLE5D_GRANULARITY_GRIP: &str = "grip";
pub const PUZZLE5D_GRANULARITY_FASTENER: &str = "fastener";

/// 🌉️ This app's own scratch fixture stays a local structural-twin mirror (`Puzzle5dDocument`) of
/// `crate::artifacts::puzzle5d::Puzzle5dSnapshot` — see that artifact's `🔖️ValueBridge` region — so
/// the DSL-text example fixtures are parsed once into the typed projection and re-serialized to the
/// JSON string this module's `document_from_json`/`.example(...)` call sites expect.
pub static CONCRETE_FOREST_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| parse_example_dsl(crate::artifacts::puzzle5d::dsl::PUZZLE5D_CONCRETE_FOREST_EXAMPLE_TEXT, "concrete-forest"));
pub static NAKAGIN_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| parse_example_dsl(crate::artifacts::puzzle5d::dsl::PUZZLE5D_NAKAGIN_EXAMPLE_TEXT, "nakagin"));
pub static CAPSULE_DREAM_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| parse_example_dsl(crate::artifacts::puzzle5d::dsl::PUZZLE5D_CAPSULE_DREAM_EXAMPLE_TEXT, "capsule-dream"));
static CONCRETE_FOREST_EXAMPLE_DOCUMENT: LazyLock<Puzzle5dDocument> = LazyLock::new(|| document_from_json(CONCRETE_FOREST_EXAMPLE_JSON.as_str()));
static NAKAGIN_EXAMPLE_DOCUMENT: LazyLock<Puzzle5dDocument> = LazyLock::new(|| document_from_json(NAKAGIN_EXAMPLE_JSON.as_str()));
static CAPSULE_DREAM_EXAMPLE_DOCUMENT: LazyLock<Puzzle5dDocument> = LazyLock::new(|| document_from_json(CAPSULE_DREAM_EXAMPLE_JSON.as_str()));
static EMPTY_EXAMPLE_DOCUMENT: LazyLock<Puzzle5dDocument> = LazyLock::new(empty_document);

fn parse_example_dsl(dsl_text: &str, label: &str) -> String {
    let projection = <Puzzle5dSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).unwrap_or_else(|error| panic!("{label} example fixture parses as dsl: {error}"));
    serde_json::to_string(&projection).unwrap_or_else(|error| panic!("serialize {label} example fixture: {error}"))
}

const PUZZLE5D_RESERVED_RAW_BYTES: usize = 65_536;
const PUZZLE5D_RESERVED_ITEMS: usize = 4_096;
const PUZZLE5D_RESERVED_OUTPUT_BYTES: usize = 1_048_576;
const PUZZLE5D_RESERVED_PAGE_BYTES: usize = 4_096;
const PUZZLE5D_IMPORT_MEDIA_BYTES: usize = semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
const PUZZLE5D_IMPORT_SEMANTIC_ITEMS: usize = 32;
const PUZZLE5D_IMPORT_DECODED_ITEMS: usize = PUZZLE5D_IMPORT_SEMANTIC_ITEMS * PUZZLE5D_IMPORT_SEMANTIC_ITEMS + PUZZLE5D_IMPORT_SEMANTIC_ITEMS * 5;
const PUZZLE5D_IMPORT_MUTATION_ITEMS: usize = PUZZLE5D_IMPORT_SEMANTIC_ITEMS * 2 + 1;
const PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE: usize = semio_framework_job::JOB_PAYLOAD_PAGE_BYTES / std::mem::size_of::<Puzzle5dMutation>();
const PUZZLE5D_IMPORT_MUTATION_PAGES: usize = (PUZZLE5D_IMPORT_MUTATION_ITEMS + PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE - 1) / PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE;

macro_rules! puzzle5d_reserved_publication {
    ("copy") => {
        &[ArtifactToolPublicationContract { tool_id: "copy", lanes: &[ArtifactToolPublicationLane::HostOnly] }]
    };
    ("cut") => {
        &[ArtifactToolPublicationContract { tool_id: "cut", lanes: &[ArtifactToolPublicationLane::Artifact] }]
    };
    ("paste") => {
        &[ArtifactToolPublicationContract { tool_id: "paste", lanes: &[ArtifactToolPublicationLane::Artifact] }]
    };
    ("import-media") => {
        &[ArtifactToolPublicationContract { tool_id: "import-media", lanes: &[ArtifactToolPublicationLane::Artifact] }]
    };
}

macro_rules! puzzle5d_reserved_factory {
    ($factory:ident, $tool:tt, $schema:literal) => {
        struct $factory {
            keys: [ToolFactoryKey; 1],
        }

        impl $factory {
            fn new(controller_id: &str) -> Self {
                Self { keys: [ToolFactoryKey::new(controller_id, $tool)] }
            }
        }

        impl ToolJobFactory for $factory {
            type Payload = ArtifactReservedToolJob;
            type Job = ArtifactReservedToolJob;

            fn keys(&self) -> &[ToolFactoryKey] {
                &self.keys
            }

            fn payload_schema_id(&self) -> &str {
                $schema
            }

            fn classification(&self) -> semio_framework::InteractiveJobClassification {
                semio_framework::InteractiveJobClassification::Migrated
            }

            fn execution_contract(&self) -> ToolExecutionContract {
                ToolExecutionContract::resumable(PUZZLE5D_RESERVED_RAW_BYTES, PUZZLE5D_RESERVED_ITEMS, 4_096, PUZZLE5D_RESERVED_OUTPUT_BYTES, 7_500, 1, 1)
            }

            fn create_job(&mut self, operation: Operation, mut payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
                payload.bind_operation(operation)?;
                Ok(payload)
            }
        }

        impl ArtifactOwnedToolJobFactory for $factory {
            type Owner = EditorApp<Puzzle5dPlayApp>;
            const TOOL_IDS: &'static [&'static str] = &[$tool];
            const DOCUMENT_SCHEMA: &'static str = PUZZLE5D_SCHEMA;
            const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = puzzle5d_reserved_publication!($tool);
        }
    };
}

puzzle5d_reserved_factory!(Puzzle5dCopyJobFactory, "copy", "puzzle.5d.reserved.copy.v1");
puzzle5d_reserved_factory!(Puzzle5dCutJobFactory, "cut", "puzzle.5d.reserved.cut.v1");
puzzle5d_reserved_factory!(Puzzle5dPasteJobFactory, "paste", "puzzle.5d.reserved.paste.v1");
puzzle5d_reserved_factory!(Puzzle5dImportJobFactory, "import-media", "puzzle.5d.reserved.import-media.v1");

pub fn puzzle5d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PUZZLE5D_PLAY_CONTROLLER_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: builds a framework `interactionSelect`
/// action targeting one `(granularity, id)` pair in the `vortex` domain — replaces the deleted
/// `setSelection` action builders every document tree row used to construct by hand.
pub fn puzzle5d_interaction_select(granularity: &str, id: &str) -> ActionDescriptor {
    let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
    puzzle5d_action(INTERACTION_SELECT_ACTION_ID, Some(json!({ "domainId": PUZZLE5D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })))
}

#[derive(Clone, Debug, Default)]
pub struct Puzzle5dFreshIds {
    occupied_parts: HashSet<String>,
    occupied_fasteners: HashSet<String>,
    part_cursor: u64,
    fastener_cursor: u64,
}

impl Puzzle5dFreshIds {
    pub fn from_document(document: &Puzzle5dDocument) -> Self {
        Self {
            occupied_parts: document.parts.iter().map(|part| part.id.clone()).collect(),
            occupied_fasteners: document.fasteners.iter().map(|fastener| fastener.id.clone()).collect(),
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
    #[serde(default)]
    pub anchor: Puzzle5dPartAnchor,
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

struct Puzzle5dExampleOperations {
    before: Value,
    after: Puzzle5dDocument,
    operations: Vec<Puzzle5dMutation>,
}

static PUZZLE5D_EXAMPLE_OPERATIONS: LazyLock<Vec<Puzzle5dExampleOperations>> = LazyLock::new(|| {
    let documents = vec![empty_document(), concrete_forest_example_document(), nakagin_example_document(), capsule_dream_example_document()];
    let values: Vec<Value> = documents.iter().filter_map(|document| serde_json::to_value(document).ok()).collect();
    let mut entries = Vec::new();
    for before in &values {
        for (after, after_value) in documents.iter().zip(&values) {
            entries.push(Puzzle5dExampleOperations { operations: puzzle5d_document_delta_operations(before, after_value), before: before.clone(), after: after.clone() });
        }
    }
    entries
});

/// 🧮️ Document operations for a document mutation through the typed semantic delta vocabulary.
pub fn puzzle5d_operations_from_document_change(before: &Value, after_document: &Puzzle5dDocument) -> Vec<Puzzle5dMutation> {
    if let Some(entry) = PUZZLE5D_EXAMPLE_OPERATIONS.iter().find(|entry| &entry.before == before && &entry.after == after_document) {
        return entry.operations.clone();
    }
    let after = serde_json::to_value(after_document).unwrap_or_else(|_| before.clone());
    puzzle5d_document_delta_operations(before, &after)
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
                operations.push(crate::artifacts::puzzle5d::mutations::change_fastener_kind::mutation::change_fastener_kind(fastener.id.clone(), fastener.fastener_kind.clone()));
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
                operations.push(crate::artifacts::puzzle5d::mutations::replace_fastener_geometry::mutation::replace_fastener_geometry(
                    fastener.id.clone(),
                    fastener.gap,
                    fastener.shift,
                    fastener.rise,
                    fastener.rotation,
                    fastener.turn,
                    fastener.tilt,
                    fastener.x,
                    fastener.y,
                ));
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
    let origin = envelope
        .document
        .parts
        .first()
        .map_or([x * flat_to_world, -y * flat_to_world, 0.0], |peer| [peer.part_3d.origin[0] + (x - peer.part_2d.x) * flat_to_world, peer.part_3d.origin[1] - (y - peer.part_2d.y) * flat_to_world, peer.part_3d.origin[2]]);
    let id = Puzzle5dFreshIds::from_document(&envelope.document).next_part();
    let mesh_url = resolve_part_kind_mesh_url(part_kind, envelope.document.kind_catalogs.as_ref());
    let grips = grips_from_templates(&envelope.document, part_kind);
    envelope.document.parts.push(Puzzle5dPart {
        id: id.clone(),
        anchor: Default::default(),
        part_kind: part_kind.into(),
        part_2d: Puzzle5dPart2d { x, y, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: part_kind.into(), icon_kind: None, hidden: None, locked: None },
        part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
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
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: `render` never gained an
/// `InteractionView` parameter — see `puzzle3d`'s `gumball_active` doc comment for the identical
/// framework-level gap. Defaults to "never render an unattached gumball".
pub fn puzzle5d_gumball_active(_runtime: &Puzzle5dRuntime, _active_utility: &str) -> bool {
    false
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
                anchor: Default::default(),
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
                x: attraction.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0),
                y: attraction.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0),
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
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to fall back to
/// `runtime.selection.grip_ids`/`selection.part_ids`/`hovered_part_id`, all dissolved into the
/// framework-owned `vortex` interaction domain — see `puzzle3d_brush_target_vortex`'s doc comment for
/// the identical framework-level gap (`ArtifactApp::render` never gained an `InteractionView`).
/// Callers holding a `Puzzle5dActionCtx` should prefer `ctx.selected_grip_ids()?.first()` before
/// reaching for this.
pub fn puzzle5d_brush_target_grip(_envelope: &Puzzle5dScene) -> Option<String> {
    None
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
/// `crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::transfer::copy_selection`.
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
        Self { snapshot: Some(snapshot), part_ids, explicit_fastener_ids, stage: Puzzle5dSelectionStage::Endpoints, cursor: 0, parts: Vec::new(), fasteners: Vec::new() }
    }

    fn rows(&self, key: &str) -> &[Value] {
        self.snapshot.as_ref().and_then(|snapshot| snapshot.0.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
    }

    fn step(&mut self) -> Result<bool, String> {
        match self.stage {
            Puzzle5dSelectionStage::Endpoints => {
                if let Some(row) = self.rows("fasteners").get(self.cursor).cloned() {
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
                if let Some(row) = self.rows("fasteners").get(self.cursor).cloned() {
                    self.cursor += 1;
                    let source = row.get("source").and_then(Value::as_str).map(owning_part_id_local);
                    let target = row.get("target").and_then(Value::as_str).map(owning_part_id_local);
                    let selected = row.get("id").and_then(Value::as_str).is_some_and(|id| self.explicit_fastener_ids.contains(id))
                        || source.zip(target).is_some_and(|(source, target)| !self.part_ids.is_empty() && self.part_ids.contains(source) && self.part_ids.contains(target));
                    if selected {
                        self.fasteners.push(serde_json::from_value(row).map_err(|error| error.to_string())?);
                    }
                } else {
                    self.stage = Puzzle5dSelectionStage::Parts;
                    self.cursor = 0;
                }
            }
            Puzzle5dSelectionStage::Parts => {
                if let Some(row) = self.rows("parts").get(self.cursor).cloned() {
                    self.cursor += 1;
                    if row.get("id").and_then(Value::as_str).is_some_and(|id| self.part_ids.contains(id)) {
                        self.parts.push(serde_json::from_value(row).map_err(|error| error.to_string())?);
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
    if !owners.is_empty() || owners.capacity() == 0 {
        return Ok(None);
    }
    let bytes = owners.capacity().saturating_mul(std::mem::size_of::<T>());
    if bytes > maximum_bytes {
        return Err(Fault::from("puzzle5d vector backing exceeds its bounded disposal byte slice"));
    }
    *owners = Vec::new();
    Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }))
}

fn puzzle5d_retire_json_step(value: &mut Value, key: &mut [u8; PUZZLE5D_JSON_RETIREMENT_KEY_BYTES], maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    match value {
        Value::Null => Ok(None),
        Value::Bool(_) | Value::Number(_) => {
            *value = Value::Null;
            Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }))
        }
        Value::String(text) => {
            let bytes = text.capacity();
            if bytes > maximum_bytes {
                return Err(Fault::from("puzzle5d recursive string exceeds its bounded disposal byte slice"));
            }
            *value = Value::Null;
            Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }))
        }
        Value::Array(values) => {
            if let Some(last) = values.last_mut() {
                if last.is_null() {
                    values.pop();
                    return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
                }
                return puzzle5d_retire_json_step(last, key, maximum_bytes);
            }
            let bytes = values.capacity().saturating_mul(std::mem::size_of::<Value>());
            if bytes > maximum_bytes {
                return Err(Fault::from("puzzle5d recursive array backing exceeds its bounded disposal byte slice"));
            }
            *value = Value::Null;
            Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }))
        }
        Value::Object(values) => {
            let next = values.iter().next().map(|(name, child)| (name.len(), child.is_null()));
            let Some((name_len, child_empty)) = next else {
                *value = Value::Null;
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
mod puzzle5d_retained_retirement_laws {
    use super::*;

    fn strict_import_source(source: &str) -> bool {
        [
            "Puzzle5dImportStage::CensusParts",
            "Puzzle5dImportStage::ReserveCatalogParts",
            "Puzzle5dImportStage::ReserveMutations",
            "Puzzle5dImportStage::LoadCatalogParts",
            "Puzzle5dImportStage::PartReserve",
            "Puzzle5dImportStage::PartVortices",
            "Puzzle5dImportStage::PartPublish",
            "Puzzle5dImportStage::CatalogMutation",
            "PUZZLE5D_IMPORT_SEMANTIC_ITEMS",
            "part_index: Vec<(String, usize)>",
            "compatibility_index: Vec<((String, String), usize)>",
            "puzzle5d_import_checkpoint(self.stage as u8, self.cursor, self.nested_cursor, self.decoded_items, self.progress, cx)",
            "puzzle5d_retire_part_kind_step",
            "puzzle5d_retire_catalogs_step",
            "puzzle5d_retire_import_mutation_step",
            "puzzle5d_decode_import_fragment",
        ]
        .into_iter()
        .all(|marker| source.contains(marker))
            && source.matches("Puzzle5dImportStage::PartVortices").count() == 2
            && source.matches("Puzzle5dImportStage::CatalogMutation").count() == 2
            && source.matches("puzzle5d_retire_part_kind_step").count() == 4
            && !source.contains("self.initial_catalogs")
            && !source.contains("self.compatibility_mutations")
            && !source.contains("part_index: HashMap")
            && !source.contains("compatibility_index: HashMap")
            && !source.contains("self.rows(\"objectKinds\").get(self.cursor).cloned()")
            && !source.contains("pop_owner!(self.catalogs.parts)")
    }

    #[test]
    fn recursive_json_zero_and_key_max_plus_one_preserve_exact_owner_before_incremental_close() {
        let oversized_key = "k".repeat(PUZZLE5D_JSON_RETIREMENT_KEY_BYTES + 1);
        let mut value = serde_json::json!({ oversized_key.clone(): { "nested": ["payload"] } });
        let before = value.clone();
        let mut key = [0; PUZZLE5D_JSON_RETIREMENT_KEY_BYTES];
        assert!(puzzle5d_retire_json_step(&mut value, &mut key, 0).is_err());
        assert_eq!(value, before);
        let nested = value.as_object_mut().and_then(|object| object.get_mut(&oversized_key)).and_then(Value::as_object_mut).and_then(|object| object.get_mut("nested")).and_then(Value::as_array_mut).and_then(|array| array.last_mut());
        if let Some(nested) = nested {
            *nested = Value::Null;
        }
        assert!(puzzle5d_retire_json_step(&mut value, &mut key, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).is_err());
        assert!(value.get(&oversized_key).is_some());
    }

    #[test]
    fn empty_vector_backing_requires_exact_byte_credit_and_retires_once() {
        let mut owners = Vec::<u64>::with_capacity(17);
        let admitted = owners.capacity();
        let bytes = admitted * std::mem::size_of::<u64>();
        assert!(puzzle5d_retire_vec_backing(&mut owners, bytes - 1).is_err());
        assert_eq!(owners.capacity(), admitted);
        assert!(matches!(puzzle5d_retire_vec_backing(&mut owners, bytes), Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes })) if released_bytes == bytes));
        assert_eq!(owners.capacity(), 0);
        assert!(matches!(puzzle5d_retire_vec_backing(&mut owners, bytes), Ok(None)));
    }

    #[test]
    fn reserved_wire_exact_max_and_plus_one_preflight_return_the_original_owner() {
        let maximum = vec![7; PUZZLE5D_RESERVED_RAW_BYTES];
        let maximum_identity = maximum.as_ptr();
        let admitted = puzzle5d_preflight_reserved_wire(maximum, PUZZLE5D_RESERVED_RAW_BYTES).expect("exact maximum is admitted");
        assert_eq!(admitted.as_ptr(), maximum_identity, "exact maximum preserves the original fixed-page source owner");

        let plus_one = vec![9; PUZZLE5D_RESERVED_RAW_BYTES + 1];
        let plus_one_identity = plus_one.as_ptr();
        let (_, rejected) = puzzle5d_preflight_reserved_wire(plus_one, PUZZLE5D_RESERVED_RAW_BYTES).expect_err("maximum plus one is rejected before copy");
        assert_eq!(rejected.as_ptr(), plus_one_identity, "maximum plus one returns the exact rejected wire owner");
    }

    #[test]
    fn import_media_exact_parse_cap_and_plus_one_are_preflighted_under_one_turn_budget() {
        let prefix = r#"{"objectKinds":[],"vortexKinds":[{"id":"grip","name":"Grip","label":""#;
        let suffix = r##"","color":"#fff","defaultCableKind":""}],"kindCompatibility":[]}"##;
        let label = "x".repeat(PUZZLE5D_IMPORT_MEDIA_BYTES.checked_sub(prefix.len() + suffix.len()).expect("fixture shell fits import cap"));
        let maximum = format!("{prefix}{label}{suffix}");
        assert_eq!(maximum.len(), PUZZLE5D_IMPORT_MEDIA_BYTES);
        let started = std::time::Instant::now();
        let parsed = puzzle5d_decode_import_fragment(&maximum).expect("exact import-media cap parses");
        let elapsed = started.elapsed();
        assert!(elapsed < std::time::Duration::from_micros(7_500), "bounded import-media serde turn exceeded 7.5 ms: {elapsed:?}");
        assert!(puzzle5d_import_keys_are(&parsed, &["objectKinds", "vortexKinds", "kindCompatibility"]));
        let maximum_plus_one = format!("{maximum} ");
        assert_eq!(maximum_plus_one.len(), PUZZLE5D_IMPORT_MEDIA_BYTES + 1);
        assert!(puzzle5d_decode_import_fragment(&maximum_plus_one).is_err());

        let canonical = serde_json::json!({
            "schema": "manifest",
            "objectKinds": [],
            "vortexKinds": [],
            "cableKinds": [],
            "attractionKinds": [],
            "kindCompatibility": [],
        });
        assert!(puzzle5d_import_keys_are(&canonical, &["schema", "objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"]));
        let hostile = serde_json::json!({ "objectKinds": [], "legacyRows": [] });
        assert!(!puzzle5d_import_keys_are(&hostile, &["schema", "objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"]));
    }

    #[test]
    fn import_media_typed_close_retires_nested_owners_one_bounded_unit_per_turn() {
        let mut owner = crate::artifacts::puzzle5d::Puzzle5dCatalogPartKind {
            id: "part-ä".repeat(64),
            name: "Part".into(),
            label: "Teil".into(),
            representations: vec![crate::artifacts::puzzle5d::Puzzle5dRepresentation { id: "mesh".into(), name: "Mesh".into(), url: "mesh.glb".into(), mime: "model/gltf-binary".into(), tags: vec!["tag-ß".repeat(64)], ..Default::default() }],
            grips: vec![crate::artifacts::puzzle5d::Puzzle5dGripTemplate { id: "g0".into(), name: "socket".into(), label: "Socket".into(), grip_kind: Some("socket".into()), ..Default::default() }],
            ..Default::default()
        };
        let mut turns = 0usize;
        loop {
            match puzzle5d_retire_part_kind_step(&mut owner, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).expect("bounded typed close") {
                Some(PluginCloseStep::Pending { released_items, released_bytes }) => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                    turns += 1;
                }
                Some(PluginCloseStep::Blocked { reason }) => panic!("typed close blocked: {reason}"),
                Some(PluginCloseStep::Complete) => panic!("nested owner helper cannot publish outer completion"),
                None => break,
            }
            assert!(turns < 100_000, "typed close did not converge");
        }
        assert!(turns > 8, "nested close was collapsed into an unbounded row drop");
        assert!(owner.id.is_empty() && owner.id.capacity() == 0);
        assert!(owner.representations.is_empty() && owner.representations.capacity() == 0);
        assert!(owner.grips.is_empty() && owner.grips.capacity() == 0);

        let mut mutation = crate::artifacts::puzzle5d::mutations::replace_kind_catalogs(Some(crate::artifacts::puzzle5d::Puzzle5dKindCatalogs {
            parts: vec![crate::artifacts::puzzle5d::Puzzle5dCatalogPartKind { id: "cancelled-part".into(), ..Default::default() }],
            ..Default::default()
        }));
        let mut mutation_turns = 0usize;
        loop {
            match puzzle5d_retire_import_mutation_step(&mut mutation, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).expect("bounded preassembled mutation close") {
                Some(PluginCloseStep::Pending { released_items, released_bytes }) => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                    mutation_turns += 1;
                }
                Some(PluginCloseStep::Blocked { reason }) => panic!("preassembled mutation close blocked: {reason}"),
                Some(PluginCloseStep::Complete) => panic!("mutation helper cannot publish outer completion"),
                None => break,
            }
            assert!(mutation_turns < 100_000, "preassembled mutation close did not converge");
        }
        assert!(matches!(mutation, Puzzle5dMutation::ReplaceKindCatalogs(value) if value.new_catalogs.is_none()));
    }

    #[test]
    fn import_media_exact_semantic_backings_fit_one_native_or_wasm_close_page() {
        fn exact_backing_bytes<T>(items: usize) -> usize {
            let mut owner = Vec::<T>::new();
            owner.try_reserve_exact(items).expect("fixed-page descriptor reserve");
            owner.capacity().checked_mul(std::mem::size_of::<T>()).expect("fixed-page descriptor extent")
        }

        let page = semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
        assert!(exact_backing_bytes::<crate::artifacts::puzzle5d::Puzzle5dCatalogPartKind>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
        assert!(exact_backing_bytes::<crate::artifacts::puzzle5d::Puzzle5dCatalogGripKind>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
        assert!(exact_backing_bytes::<crate::artifacts::puzzle5d::Puzzle5dCatalogFastenerKind>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
        assert!(exact_backing_bytes::<crate::artifacts::puzzle5d::Puzzle5dCatalogRopeKind>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
        assert!(exact_backing_bytes::<crate::artifacts::puzzle5d::Puzzle5dKindCompatibility>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
        assert!(exact_backing_bytes::<crate::artifacts::puzzle5d::Puzzle5dGripTemplate>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
        assert!(exact_backing_bytes::<(String, usize)>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
        assert!(exact_backing_bytes::<((String, String), usize)>(PUZZLE5D_IMPORT_SEMANTIC_ITEMS) <= page);
        for page_index in 0..PUZZLE5D_IMPORT_MUTATION_PAGES {
            let remaining = PUZZLE5D_IMPORT_MUTATION_ITEMS.saturating_sub(page_index * PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);
            let items = remaining.min(PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);
            assert!(exact_backing_bytes::<Puzzle5dMutation>(items) <= page);
        }
        assert_eq!(PUZZLE5D_IMPORT_DECODED_ITEMS, 1_184);
        assert_eq!(PUZZLE5D_IMPORT_MUTATION_ITEMS, 65);
        assert_eq!(PUZZLE5D_IMPORT_MUTATION_PAGES, 2);
    }

    #[test]
    fn import_media_checkpoint_preserves_outer_nested_census_and_progress_cursors() {
        let state = puzzle5d_import_checkpoint_bytes(23, usize::MAX, usize::MAX - 1, usize::MAX - 2, u64::MAX - 3);
        assert_eq!(state.len(), 33);
        assert_eq!(state[0], 23);
        assert_eq!(u64::from_le_bytes(state[1..9].try_into().expect("outer cursor slice")), usize::MAX as u64);
        assert_eq!(u64::from_le_bytes(state[9..17].try_into().expect("nested cursor slice")), (usize::MAX - 1) as u64);
        assert_eq!(u64::from_le_bytes(state[17..25].try_into().expect("census slice")), (usize::MAX - 2) as u64);
        assert_eq!(u64::from_le_bytes(state[25..33].try_into().expect("progress slice")), u64::MAX - 3);
        assert_ne!(state, puzzle5d_import_checkpoint_bytes(23, usize::MAX, usize::MAX - 2, usize::MAX - 2, u64::MAX - 3));
    }

    #[test]
    fn import_media_source_requires_schema_census_reserve_nested_cursor_and_recursive_close() {
        let source = include_str!("🦀️component.rs");
        let import = source.rsplit_once("enum Puzzle5dImportStage").and_then(|(_, suffix)| suffix.split_once("//#endregion 🧵️ReservedJobs").map(|(import, _)| import)).expect("import source region");
        assert!(strict_import_source(import));
        assert!(!strict_import_source(&import.replacen("Puzzle5dImportStage::PartVortices", "Puzzle5dImportStage::Parts", 1)));
        assert!(!strict_import_source(&import.replacen("puzzle5d_retire_part_kind_step", "unbounded_drop", 1)));
        assert!(!strict_import_source(&import.replacen("Puzzle5dImportStage::CatalogMutation", "Puzzle5dImportStage::Complete", 1)));
        assert!(!strict_import_source(&import.replacen("self.nested_cursor, self.decoded_items", "0, 0", 1)));
    }
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
            if matches!(part.part_3d.scale, Some(Value::Array(_)) | Some(Value::Object(_))) {
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
            let bytes = self.scan.part_ids.capacity().saturating_mul(std::mem::size_of::<String>());
            if bytes > maximum_bytes {
                return Err(Fault::from("puzzle5d clipboard selection backing exceeds its bounded disposal byte slice"));
            }
            self.scan.part_ids.shrink_to_fit();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if self.scan.explicit_fastener_ids.is_empty() && self.scan.explicit_fastener_ids.capacity() != 0 {
            let bytes = self.scan.explicit_fastener_ids.capacity().saturating_mul(std::mem::size_of::<String>());
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
    completed: bool,
}

impl InteractiveJob for Puzzle5dCopyJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
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
                    if let Err(error) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                        return puzzle5d_job_fault(cx, error.message);
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
            Ok(PluginCloseStep::Blocked { .. }) | Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
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
        self.work.close_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.work.terminal_is_empty()
    }
}

struct Puzzle5dCutJob {
    work: Puzzle5dClipboardWork,
    completed: bool,
}

impl InteractiveJob for Puzzle5dCutJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
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
                    let mut mutations = self.work.scan.fasteners.iter().map(|fastener| crate::artifacts::puzzle5d::mutations::disconnect_grips(fastener.id.clone())).collect::<Vec<_>>();
                    mutations.extend(self.work.scan.parts.iter().map(|part| crate::artifacts::puzzle5d::mutations::delete_part(part.id.clone())));
                    let effects = self.work.fragment().map(|fragment| vec![Effect::ClipboardWrite { fragment }]).unwrap_or_default();
                    let emit = Emit { artifact_mutations: mutations, effects, ..Default::default() };
                    let Some(completion) = self.work.completion.as_ref() else { return puzzle5d_job_fault(cx, "puzzle5d cut lost its completion authority") };
                    if let Err(error) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                        return puzzle5d_job_fault(cx, error.message);
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
            Ok(PluginCloseStep::Blocked { .. }) | Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
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
        self.work.close_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.work.terminal_is_empty()
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
    commit: Puzzle5dCommitEnvelope,
    completed: bool,
    retirement_key: [u8; PUZZLE5D_JSON_RETIREMENT_KEY_BYTES],
    closing: bool,
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
                self.placement = serde_json::from_value(json!({
                    "anchor": args.get("anchor").cloned().unwrap_or_else(|| json!("original")),
                    "position": args.get("position").cloned()
                }))
                .unwrap_or_default();
                self.stage = Puzzle5dPasteStage::FragmentParts;
                self.cursor = 0;
            }
            Puzzle5dPasteStage::FragmentParts => {
                let rows = self.fragment_value.as_ref().and_then(|value| value.get("parts")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
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
                let rows = self.snapshot.as_ref().and_then(|snapshot| snapshot.0.get("parts")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor) {
                    self.cursor += 1;
                    if let Some(id) = row.get("id").and_then(Value::as_str) {
                        self.fresh_ids.observe_part(id);
                    }
                    self.target_sum.0 += row.get("2d").and_then(|value| value.get("x")).and_then(Value::as_f64).unwrap_or_default();
                    self.target_sum.1 += row.get("2d").and_then(|value| value.get("y")).and_then(Value::as_f64).unwrap_or_default();
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
                let rows = self.snapshot.as_ref().and_then(|snapshot| snapshot.0.get("fasteners")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor) {
                    self.cursor += 1;
                    if let Some(id) = row.get("id").and_then(Value::as_str) {
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
                    let typed = match serde_json::to_value(next).and_then(serde_json::from_value::<crate::artifacts::puzzle5d::Puzzle5dPart>) {
                        Ok(typed) => typed,
                        Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                    };
                    self.mutations.push(crate::artifacts::puzzle5d::mutations::create_part(typed, None));
                } else {
                    self.stage = Puzzle5dPasteStage::MaterializeFasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::MaterializeFasteners => {
                let rows = self.fragment_value.as_ref().and_then(|value| value.get("fasteners")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let fastener: Puzzle5dFastener = match serde_json::from_value(row) {
                        Ok(fastener) => fastener,
                        Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                    };
                    self.mutations.push(crate::artifacts::puzzle5d::mutations::connect_grips(
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
                    let emit = Emit::mutations(std::mem::take(&mut self.mutations));
                    let Some(completion) = self.completion.as_ref() else { return puzzle5d_job_fault(cx, "puzzle5d paste lost its completion authority") };
                    if let Err(error) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                        return puzzle5d_job_fault(cx, error.message);
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
            Ok(PluginCloseStep::Blocked { .. }) | Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
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
            if let Some(scale) = part.part_3d.scale.as_mut() {
                if let Some(step) = puzzle5d_retire_json_step(scale, &mut self.retirement_key, maximum_bytes)? {
                    return Ok(step);
                }
                part.part_3d.scale = None;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
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
            let bytes = self.id_map.capacity().saturating_mul(std::mem::size_of::<(String, String)>());
            if bytes > maximum_bytes {
                return Err(Fault::from("puzzle5d paste id map backing exceeds its bounded disposal byte slice"));
            }
            self.id_map.shrink_to_fit();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(mutation) = self.mutations.last() {
            let bytes = std::mem::size_of_val(mutation);
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
    catalogs: crate::artifacts::puzzle5d::Puzzle5dKindCatalogs,
    had_catalogs: bool,
    catalog_changed: bool,
    compatibility: Vec<crate::artifacts::puzzle5d::Puzzle5dKindCompatibility>,
    part_index: Vec<(String, usize)>,
    grip_index: Vec<(String, usize)>,
    compatibility_index: Vec<((String, String), usize)>,
    mutation_pages: [Vec<Puzzle5dMutation>; PUZZLE5D_IMPORT_MUTATION_PAGES],
    cursor: usize,
    nested_cursor: usize,
    decoded_items: usize,
    current_part: Option<crate::artifacts::puzzle5d::Puzzle5dCatalogPartKind>,
    completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    commit: Puzzle5dCommitEnvelope,
    completed: bool,
    retiring_index_primary: Option<String>,
    retiring_index_secondary: Option<String>,
    retirement_key: [u8; PUZZLE5D_JSON_RETIREMENT_KEY_BYTES],
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
    value.as_object().is_some_and(|object| object.keys().all(|key| allowed.contains(&key.as_str())))
}

fn puzzle5d_decode_import_fragment(media_json: &str) -> Result<Value, String> {
    if media_json.len() > PUZZLE5D_IMPORT_MEDIA_BYTES {
        return Err("puzzle5d kit:in payload exceeds its predecode cap".into());
    }
    serde_json::from_str(media_json).map_err(|error| error.to_string())
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

fn puzzle5d_retire_representation_step(owner: &mut crate::artifacts::puzzle5d::Puzzle5dRepresentation, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
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

fn puzzle5d_retire_grip_template_step(owner: &mut crate::artifacts::puzzle5d::Puzzle5dGripTemplate, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    for value in [&mut owner.id, &mut owner.name, &mut owner.label, &mut owner.description, &mut owner.icon] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    puzzle5d_retire_optional_string_step(&mut owner.grip_kind, maximum_bytes)
}

fn puzzle5d_retire_attribute_step(owner: &mut crate::artifacts::puzzle5d::Puzzle5dAttribute, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    for value in [&mut owner.id, &mut owner.key, &mut owner.value] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    puzzle5d_retire_optional_string_step(&mut owner.definition, maximum_bytes)
}

fn puzzle5d_retire_author_step(owner: &mut crate::artifacts::puzzle5d::Puzzle5dAuthor, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    for value in [&mut owner.id, &mut owner.name, &mut owner.email] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    puzzle5d_retire_optional_string_step(&mut owner.role, maximum_bytes)
}

fn puzzle5d_retire_part_kind_step(owner: &mut crate::artifacts::puzzle5d::Puzzle5dCatalogPartKind, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
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

fn puzzle5d_retire_grip_kind_step(owner: &mut crate::artifacts::puzzle5d::Puzzle5dCatalogGripKind, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
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

fn puzzle5d_retire_fastener_kind_step(owner: &mut crate::artifacts::puzzle5d::Puzzle5dCatalogFastenerKind, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    for value in [&mut owner.id, &mut owner.name] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    puzzle5d_retire_optional_string_step(&mut owner.label, maximum_bytes)
}

fn puzzle5d_retire_rope_kind_step(owner: &mut crate::artifacts::puzzle5d::Puzzle5dCatalogRopeKind, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    for value in [&mut owner.id, &mut owner.name, &mut owner.label, &mut owner.default_fastener_kind] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    Ok(None)
}

fn puzzle5d_retire_compatibility_step(owner: &mut crate::artifacts::puzzle5d::Puzzle5dKindCompatibility, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
    for value in [&mut owner.source, &mut owner.target] {
        if let Some(step) = puzzle5d_retire_string_step(value, maximum_bytes)? {
            return Ok(Some(step));
        }
    }
    Ok(None)
}

fn puzzle5d_retire_catalogs_step(owner: &mut crate::artifacts::puzzle5d::Puzzle5dKindCatalogs, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
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
            commit: Puzzle5dCommitEnvelope::new(),
            completed: false,
            retiring_index_primary: None,
            retiring_index_secondary: None,
            retirement_key: [0; PUZZLE5D_JSON_RETIREMENT_KEY_BYTES],
            closing: false,
        }
    }

    fn checkpoint(&self, cx: &mut StepContext<'_>) -> StepOutcome {
        puzzle5d_import_checkpoint(self.stage as u8, self.cursor, self.nested_cursor, self.decoded_items, self.progress, cx)
    }

    fn rows(&self, key: &str) -> &[Value] {
        self.fragment.as_ref().and_then(|value| value.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
    }

    fn snapshot_rows(&self, parent: &str, key: &str) -> &[Value] {
        self.snapshot.as_ref().and_then(|snapshot| snapshot.0.get(parent)).and_then(|value| value.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
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
                    .and_then(|total| total.checked_add(self.snapshot.as_ref().and_then(|snapshot| snapshot.0.get("kindCompatibility")).and_then(Value::as_array).map_or(0, Vec::len)));
                self.decoded_items = match fragment_items.and_then(|fragment_items| snapshot_items.and_then(|snapshot_items| fragment_items.checked_add(snapshot_items))) {
                    Some(items) if items <= PUZZLE5D_IMPORT_DECODED_ITEMS => items,
                    _ => return puzzle5d_job_fault(cx, "puzzle5d kit:in decoded item limit exceeded"),
                };
                if !fragment.is_object() {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in root must be an object");
                }
                if !puzzle5d_import_keys_are(&fragment, &["schema", "objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"]) {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in root contains an unknown field");
                }
                if fragment.get("schema").is_some_and(|value| value.as_str() != Some("manifest")) {
                    return puzzle5d_job_fault(cx, "puzzle5d kit:in schema must be manifest when present");
                }
                if ["objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"].into_iter().any(|key| fragment.get(key).is_some_and(|value| !value.is_array())) {
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
                    if row.get("vortices").is_some_and(|value| !value.is_array()) {
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
                let capacity = self.snapshot.as_ref().and_then(|snapshot| snapshot.0.get("kindCompatibility")).and_then(Value::as_array).map_or(0, Vec::len).saturating_add(self.rows("kindCompatibility").len());
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
                let capacity = self.snapshot.as_ref().and_then(|snapshot| snapshot.0.get("kindCompatibility")).and_then(Value::as_array).map_or(0, Vec::len).saturating_add(self.rows("kindCompatibility").len());
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
                    let parsed = match crate::artifacts::puzzle5d::Puzzle5dCatalogPartKind::deserialize(row) {
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
                    let parsed = match crate::artifacts::puzzle5d::Puzzle5dCatalogGripKind::deserialize(row) {
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
                    let parsed = match crate::artifacts::puzzle5d::Puzzle5dCatalogFastenerKind::deserialize(row) {
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
                    let parsed = match crate::artifacts::puzzle5d::Puzzle5dCatalogRopeKind::deserialize(row) {
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
                let rows = self.snapshot.as_ref().and_then(|snapshot| snapshot.0.get("kindCompatibility")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor) {
                    let parsed = match crate::artifacts::puzzle5d::Puzzle5dKindCompatibility::deserialize(row) {
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
                    self.current_part = Some(crate::artifacts::puzzle5d::Puzzle5dCatalogPartKind {
                        id,
                        name,
                        label,
                        representations: mesh_url.map(|url| vec![crate::artifacts::puzzle5d::Puzzle5dRepresentation { id: "mesh".into(), name: "mesh".into(), url, mime: "model/gltf-binary".into(), ..Default::default() }]).unwrap_or_default(),
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
                    part.grips.push(crate::artifacts::puzzle5d::Puzzle5dGripTemplate {
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
                    let next = crate::artifacts::puzzle5d::Puzzle5dCatalogGripKind {
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
                    let parsed = match crate::artifacts::puzzle5d::Puzzle5dKindCompatibility::deserialize(row) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(cx, error.to_string()),
                    };
                    let key = (parsed.source.clone(), parsed.target.clone());
                    match self.compatibility_index.iter().find_map(|(candidate, index)| (candidate == &key).then_some(*index)) {
                        Some(index) if self.compatibility[index] == parsed => {}
                        Some(index) => {
                            if let Err(error) = self.push_mutation(crate::artifacts::puzzle5d::mutations::disconnect_kind_compatibility(parsed.source.clone(), parsed.target.clone())) {
                                return puzzle5d_job_fault(cx, error);
                            }
                            if let Err(error) = self.push_mutation(crate::artifacts::puzzle5d::mutations::connect_kind_compatibility(parsed.source.clone(), parsed.target.clone(), parsed.bidirectional, parsed.important, parsed.specificity)) {
                                return puzzle5d_job_fault(cx, error);
                            }
                            self.compatibility[index] = parsed;
                        }
                        None => {
                            if let Err(error) = self.push_mutation(crate::artifacts::puzzle5d::mutations::connect_kind_compatibility(parsed.source.clone(), parsed.target.clone(), parsed.bidirectional, parsed.important, parsed.specificity)) {
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
                    let mutation = crate::artifacts::puzzle5d::mutations::replace_kind_catalogs(Some(std::mem::take(&mut self.catalogs)));
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
                    if let Err(error) = completion.complete(Ok(Emit::mutations(mutations)), EphemeralEmit::default()) {
                        return puzzle5d_job_fault(cx, error.message);
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
            Ok(PluginCloseStep::Blocked { .. }) | Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
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
        match self.commit.close_step(maximum_items, maximum_bytes) {
            PluginCloseStep::Complete => {}
            step => return Ok(step),
        }
        if let Some(fragment) = self.fragment.as_mut() {
            if let Some(step) = puzzle5d_retire_json_step(fragment, &mut self.retirement_key, maximum_bytes)? {
                return Ok(step);
            }
            self.fragment = None;
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
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
            && self.completion.is_none()
            && self.commit.terminal_is_empty()
    }
}
//#endregion 🧵️ReservedJobs

//#region 🔖️ContextMenu
/// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: `duplicateSelection`/`selectSameKindSelection`/
/// `zoomToSelection` stay top-level verbs; the hide/lock toggles (bespoke rows — their label/icon flip
/// on selection state, so they can't resolve from a single static `ActionDefinition`) fold into a
/// `settings` group; `deleteSelection` (bespoke label carrying the selection-count phrase) stays the
/// trailing destructive row. `organize_context_menu`, run automatically at the `VcsArtifactApp::context_menu`
/// funnel, handles taxonomy ordering/separator placement — this function only needs to emit the rows.
fn puzzle5d_context_menu_items(envelope: &Puzzle5dScene, part_ids: &[String], labels: &Puzzle5dLabels, is_de: bool, registry: &semio_framework_plugin::AppActionRegistry) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, ContextMenuItemSpec, Menu};
    if part_ids.is_empty() {
        return Vec::new();
    }
    let selected: Vec<&Puzzle5dPart> = envelope.document.parts.iter().filter(|part| part_ids.contains(&part.id)).collect();
    let all_hidden = !selected.is_empty() && selected.iter().all(|part| part.part_2d.hidden.unwrap_or(false));
    let all_locked = !selected.is_empty() && selected.iter().all(|part| part.part_2d.locked.unwrap_or(false));
    let phrase = semio_framework::io::resolve_ready(selection_count_phrase(is_de, &[(part_ids.len(), if is_de { "Teil" } else { "part" }, if is_de { "Teile" } else { "parts" })]));
    let bespoke = |id: &str, label: String, icon: &str, action: &str, args: Option<Value>, destructive: bool| ContextMenuItemSpec {
        id: id.into(),
        label: Some(label),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args: semio_framework_plugin::optional_json_to_dsl(args),
        destructive: destructive.then_some(true),
        ..Default::default()
    };
    semio_framework::io::resolve_ready(async {
        Menu::of(registry)
            .await
            .action("duplicateSelection")
            .await
            .action("selectSameKindSelection")
            .await
            .action("zoomToSelection")
            .await
            .group("settings", |m| async {
                m.item(bespoke("hide-show", if all_hidden { labels.show.into() } else { labels.hide.into() }, if all_hidden { "eye" } else { "eye-off" }, "setSelectionFlag", Some(json!({ "flag": "hidden", "value": !all_hidden })), false))
                    .await
                    .item(bespoke("lock-unlock", if all_locked { labels.unlock.into() } else { labels.lock.into() }, if all_locked { "lock-open" } else { "lock" }, "setSelectionFlag", Some(json!({ "flag": "locked", "value": !all_locked })), false))
                    .await
            })
            .await
            .item(bespoke("delete", format!("{} ({phrase})", labels.delete.as_str()), "trash", "deleteSelection", None, true))
            .await
            .build()
            .await
    })
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

            fn try_from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Option<Self> {
                match action {
                    $($id => Some(Puzzle5dCommand::$Variant { window_id, args })),*,
                    _ => None,
                }
            }

            #[cfg(test)]
            fn from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Self {
                Self::try_from_action(action, args, window_id)
                    .unwrap_or_else(|| panic!("unknown puzzle5d action id in test: {action}"))
            }
        }
    };
}

puzzle5d_command_variants! {
    SetFixtureJson = "setFixtureJson",
    SetActiveExample = "setActiveExample",
            ImportComposeKit = "importComposeKit",
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
    /// 🕹️ Read-only view of the framework-owned `vortex` interaction domain (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — retained selection-acting verbs read
    /// `.selected_part_ids()?`/etc. here instead of the deleted `Puzzle5dConfig` selection fields.
    pub selection: &'a protocol::DomainSelection,
    /// 🛑️ Set by an arm that must skip the whole epilogue (delta, effects, config snapshot) — the
    /// direct replacement for the pre-migration `return Emit::default()` early exits.
    pub abort: bool,
}

impl<'a> Puzzle5dActionCtx<'a> {
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

/// 🌳️ Admits fallibly assembled puzzle nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        nodes.try_push(value?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d node admission failed"))?;
    }
    Ok(nodes)
}
//#endregion 🔖️ActionContext

//#region 🔖️PlayApp
// 🧩️ B1: Puzzle-5d play app. Owns the precompute engine and the registered-mesh cache — both
// per-call scratch, never VCS-tracked; the persisted document (bare `Puzzle5dDocument` json) lives in
// the wrapping `VcsArtifactApp`'s document store, and the ephemeral view state lives in the wrapping
// store's real, VCS-tracked `Puzzle5dConfig` artifact (see `🦀️config.rs`) — every read comes from
// `cfg.snapshot`, every write flows out as a `Puzzle5dConfigMutation` in the returned `Emit`.
// Each action mutates a transient {@link Puzzle5dScene}, then emits the granular operation delta.
// Undo/redo/checkpoints are handled by the wrapper.
fn with_puzzle5d_app<R>(f: impl FnOnce(&Puzzle5dPlayApp) -> R) -> R {
    let app = Puzzle5dPlayApp::default();
    f(&app)
}

pub struct Puzzle5dPlayApp {
    pub precompute: RefCell<Puzzle5dPrecomputeSession>,
    pub registered_mesh_urls: RefCell<HashSet<String>>,
}

impl Default for Puzzle5dPlayApp {
    fn default() -> Self {
        Self { precompute: RefCell::new(Puzzle5dPrecomputeSession::new()), registered_mesh_urls: RefCell::new(HashSet::new()) }
    }
}

impl Puzzle5dPlayApp {
    pub fn drive_precompute(&self, envelope: &Puzzle5dScene) {
        let _ = self.precompute.borrow_mut().set_scene(&scene_config_json(envelope));
        // 🧊️ Guarded by `has_mesh` (mirrors the puzzle3d path): `register_mesh` now invalidates the
        // precompute cache, so re-registering the same fallback body on every drive would wipe
        // suggestion/fill progress every call and defeat `set_scene`'s idempotence above.
        if !self.precompute.borrow_mut().has_mesh(PUZZLE5D_FALLBACK_MESH_KIND) {
            let fallback = semio_framework_plugin::mesh_from_kind(PUZZLE5D_FALLBACK_MESH_KIND);
            self.precompute.borrow_mut().register_mesh(PUZZLE5D_FALLBACK_MESH_KIND, &fallback.positions, &fallback.indices);
        }
        for url in collect_mesh_urls(&envelope.document) {
            if !self.registered_mesh_urls.borrow_mut().contains(&url) && !self.precompute.borrow_mut().has_mesh(&url) {
                let fallback = semio_framework_plugin::mesh_from_kind(PUZZLE5D_FALLBACK_MESH_KIND);
                self.precompute.borrow_mut().register_mesh(&url, &fallback.positions, &fallback.indices);
            }
        }
        let _ = self.precompute.borrow_mut().precompute_step(8);
    }

    pub fn apply_engine_brush_placement(&self, envelope: &Puzzle5dScene, payload: &Value) -> Option<Puzzle5dScene> {
        let brush_payload = serde_json::from_value::<BrushPlacePayload>(payload.clone()).ok()?;
        let fixture_json = self.precompute.borrow_mut().apply_brush_placement_rust(&serde_json::to_string(&brush_payload).ok()?).ok()?;
        merge_engine_fixture(envelope, &fixture_json)
    }

    /// 🖌️ Paired placement for a board `brushPlace` event: the engine picks the volume pose for the flat payload's kind, both aspects land in one part.
    pub fn apply_board_brush_place(&self, envelope: &mut Puzzle5dScene, payload: &Value) {
        self.drive_precompute(envelope);
        let node_kind = payload.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("Part").to_string();
        let source_grip = payload.get("sourceHandleId").and_then(|value| value.as_str()).map(str::to_string).or_else(|| puzzle5d_brush_target_grip(envelope));
        if let Some(source_grip) = source_grip.as_ref() {
            let candidates = parse_brush_candidates_free(&self.precompute.borrow().brush_candidates(source_grip));
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
                }
                *envelope = next;
                return;
            }
        }
        let mut fresh_ids = Puzzle5dFreshIds::from_document(&envelope.document);
        let id = payload.get("nodeId").and_then(|value| value.as_str()).map_or_else(|| fresh_ids.next_part(), str::to_string);
        let x = payload.get("x").and_then(|value| value.as_f64()).unwrap_or(120.0);
        let y = payload.get("y").and_then(|value| value.as_f64()).unwrap_or(120.0);
        let mesh_url = resolve_part_kind_mesh_url(&node_kind, envelope.document.kind_catalogs.as_ref());
        let grips = grips_from_templates(&envelope.document, &node_kind);
        let source_world = source_grip.as_ref().and_then(|full_id| find_part_by_grip_full_id(&envelope.document, full_id).map(|(part, grip)| (world_grip_position(part, grip), world_grip_direction(part, grip))));
        let origin = source_world.map_or([0.0, 0.0, 0.0], |(position, direction)| [position[0] + direction[0], position[1] + direction[1], position[2] + direction[2]]);
        envelope.document.parts.push(Puzzle5dPart {
            id: id.clone(),
            anchor: Default::default(),
            part_kind: node_kind.clone(),
            part_2d: Puzzle5dPart2d { x, y, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: node_kind, icon_kind: None, hidden: None, locked: None },
            part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
            grips,
        });
        if let (Some(source), Some(part)) = (source_grip, envelope.document.parts.last()) {
            if let Some(grip) = part.grips.first() {
                let target = puzzle5d_grip_full_id(&part.id, &grip.id);
                envelope.document.fasteners.push(Puzzle5dFastener {
                    id: payload.get("edgeId").and_then(|value| value.as_str()).map_or_else(|| fresh_ids.next_fastener(), str::to_string),
                    source,
                    target,
                    fastener_kind: None,
                    gap: 0.0,
                    shift: 0.0,
                    rise: 0.0,
                    rotation: 0.0,
                    turn: 0.0,
                    tilt: 0.0,
                    x: 0.0,
                    y: 0.0,
                });
            }
        }
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
                // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: "select"
                // board events used to write `envelope.runtime.selection` directly; selection is
                // framework-owned now and `handle` has no channel to write it (see puzzle3d's
                // `select-same-kind` doc comment for the identical limitation) — dropped.
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
                        let id = payload
                            .get("id")
                            .and_then(|value| value.as_str())
                            .map(str::to_string)
                            .unwrap_or_else(|| Puzzle5dFreshIds::from_document(&envelope.document).next_fastener());
                        envelope.document.fasteners.push(Puzzle5dFastener {
                            id,
                            source,
                            target,
                            fastener_kind: payload.get("edgeKind").and_then(|value| value.as_str()).map(str::to_string),
                            gap: 0.0,
                            shift: 0.0,
                            rise: 0.0,
                            rotation: 0.0,
                            turn: 0.0,
                            tilt: 0.0,
                            x: 0.0,
                            y: 0.0,
                        });
                    }
                }
                "nodeDelete" => {
                    if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                        remove_parts(&mut envelope.document, &[id.to_string()]);
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

    /// @emoji 🧩️ B1: the pure per-action core, dispatched into by `ArtifactApp::handle` with
    /// `action`/`args`/`window_id` reconstructed 1:1 from the typed `Puzzle5dCommand`. Everything past
    /// this adapter boundary reads/writes the passed-in `Puzzle5dConfig` snapshot and returns a real
    /// `Emit` (document + config operations) instead of mutating `self`.
    fn handle_action_impl(&self, action: &str, args: Option<&Value>, window_id: Option<&str>, snapshot: &Puzzle5dPlaySnapshot, config: &Puzzle5dConfig, selection: &protocol::DomainSelection) -> Emit<Puzzle5dMutation, Puzzle5dConfigMutation> {
        let before = snapshot.0.clone();
        let active_utility_initial = puzzle5d_scene_active_utility(config, window_id);
        let wid = window_id.map_or_else(|| world3d::WINDOW_KIND_ID.to_string(), str::to_string);
        let mut scene = scene_from_projection(&before, config.clone(), &active_utility_initial);
        let mut ctx = Puzzle5dActionCtx { app: self, scene: &mut scene, window_id: &wid, selection, abort: false };
        dispatch_puzzle5d_action(&mut ctx, action, args);
        if ctx.abort {
            return Emit::default();
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
        // 🧰️ B1: a DIRECT `SetActiveUtility` command already told the host what it needs to know — never
        // re-emit the same switch as a `Effect` (the pre-B1 code only had to guard this for the
        // INDIRECT paths below, since the host itself pushed the direct switch before dispatching; now
        // the command IS the direct switch, so this arm must self-exclude). Programmatic utility
        // switches (engagement submit/abort, fill) still push the active utility back into the host
        // session for both windows.
        let is_direct_utility_switch = action == SET_ACTIVE_UTILITY_ACTION_ID;
        let effects = if !is_direct_utility_switch && next_active_utility != active_utility_initial {
            PUZZLE5D_PLAY_WINDOWS.iter().map(|window| Effect::SetActiveUtility { window_id: (*window).into(), utility_id: next_active_utility.clone() }).collect()
        } else {
            Vec::new()
        };
        // 🧮️ B1: only a REAL config change becomes a `Puzzle5dConfigMutation` — `PartialEq` (derived)
        // makes this cheap, and keeps a pure read-only action from creating a no-op undo entry.
        let config_mutations = if &scene.runtime != config { vec![Puzzle5dConfigMutation::Snapshot { config: scene.runtime }] } else { Vec::new() };
        Emit { artifact_mutations: operations, config_mutations, coalesce_key, effects, ..Default::default() }
    }
}

/// 🎬️ Dispatch only: every arm's behaviour lives in its `🎮️commands/<group>/🦀️component.rs` free
/// function. No behaviour lives in this match.
fn dispatch_puzzle5d_action(ctx: &mut Puzzle5dActionCtx<'_>, action: &str, args: Option<&Value>) {
    match action {
        "setFixtureJson" => set_fixture_json::set_fixture_json(ctx, args),
        "setActiveExample" => set_active_example::set_active_example(ctx, args),
        "selectSameKindSelection" | "selectSameKind" => select_same_kind::select_same_kind(ctx),
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
        "zoomToSelection" | "focusSelection" => zoom_to_selection::zoom_to_selection(ctx),
        "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => apply_sun::apply(ctx, action, args),
        "setLodMode" => set_lod_mode::set_lod_mode(ctx, args),
        "setGridSnapEnabled" => set_grid_snap_enabled::set_grid_snap_enabled(ctx, args),
        "setGridFactor" => set_grid_factor::set_grid_factor(ctx, args),
        "addBrushPart" | "addBrushObject" => add_brush_part::add_brush_part(ctx, args),
        "cycleBrushCandidate" => cycle_brush_candidate::cycle_brush_candidate(ctx),
        "registerBrushMesh" => register_brush_mesh::register_brush_mesh(ctx, args),
        "setBrushPlacementOverlapBudget" => set_brush_placement_overlap_budget::set_brush_placement_overlap_budget(ctx, args),
        "setObjectKindWeight" | "setVortexKindWeight" => set_kind_weight::set_kind_weight(ctx, action, args),
        "engagementControlSelect" => engagement_control_select::engagement_control_select(ctx, args),
        "setSuggestionOffset" => set_suggestion_offset::set_suggestion_offset(ctx, args),
        "setFillCount" => set_fill_count::set_fill_count(ctx, args),
        "engagementInput" => engagement_input::engagement_input(ctx, args),
        "engagementSubmit" => engagement_submit::engagement_submit(ctx, args),
        "engagementAbort" => engagement_abort::engagement_abort(ctx, args),
        "translateSelection" => translate_selection::translate_selection(ctx, args),
        "rotateSelection" => rotate_selection::rotate_selection(ctx, args),
        "scaleSelection" => scale_selection::scale_selection(ctx, args),
        "worldRelocate" => world_relocate::world_relocate(ctx, args),
        "applyBoardEvents" => apply_board_events::apply_board_events(ctx, args),
        SET_ACTIVE_UTILITY_ACTION_ID => set_active::set_active(ctx, args),
        // 🛑️ Pure pointer-down notifications: no scene mutation, no operations, no config snapshot —
        // the pre-migration code returned `Emit::default()` here, which `abort` reproduces exactly.
        "worldPointerDown" | "canvasPointerDown" => ctx.abort = true,
        _ => {}
    }
}

//#region 🧵️RetainedCommands
pub(crate) const PUZZLE5D_RETAINED_TOOL_IDS: &[&str] = &[
    "canvasPointerDown",
    "worldPointerDown",
    "deleteSelection",
    "duplicateSelection",
    "importComposeKit",
    "selectSameKindSelection",
    "setFixtureJson",
    "setSelectionFlag",
    "zoomToSelection",
];
const PUZZLE5D_RETAINED_PAYLOAD_SCHEMA: &str = "puzzle.5d.tool-command.v1";

fn puzzle5d_retained_extent(_command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
    let selection = interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).map_or(0, |selection| selection.ids.len());
    let parts = snapshot.0.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
    let fasteners = snapshot.0.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
    let items = selection.checked_add(parts)?.checked_add(fasteners)?;
    (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
}

fn puzzle5d_retained_reduce(
    command: &Puzzle5dCommand,
    snapshot: &Puzzle5dPlaySnapshot,
    config: &Puzzle5dConfig,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
) -> Result<Emit<Puzzle5dMutation, Puzzle5dConfigMutation>, Fault> {
    if command.action_id() == "selectSameKindSelection" {
        return Err(Fault::from("puzzle5d selectSameKindSelection is fail-closed: ArtifactEditor has no route-specific interaction-selection publication primitive"));
    }
    if command.action_id() == "importComposeKit" {
        return Err(Fault::from("puzzle5d importComposeKit is fail-closed: no owner-qualified Compose-kit media value is present on this command route; use import-media kit:in"));
    }
    let empty_selection = protocol::DomainSelection::default();
    let selection = interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).unwrap_or(&empty_selection);
    Ok(with_puzzle5d_app(|app| app.handle_action_impl(command.action_id(), command.args(), command.window_id(), snapshot, config, selection)))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dTransformStage {
    Selection,
    Parts,
    Complete,
    Closing,
}

struct Puzzle5dTransformWork {
    tool_id: &'static str,
    stage: Puzzle5dTransformStage,
    selection_cursor: usize,
    part_cursor: usize,
    selected: HashSet<String>,
    mutations: Vec<Puzzle5dMutation>,
}

impl Puzzle5dTransformWork {
    fn new(tool_id: &'static str) -> Self {
        Self {
            tool_id,
            stage: Puzzle5dTransformStage::Selection,
            selection_cursor: 0,
            part_cursor: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
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
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dTransformWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
        let items = Self::source_len(command, interaction).checked_add(snapshot.0.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?;
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
                self.stage = Puzzle5dTransformStage::Parts;
                Ok(Self::progress("puzzle5d-transform-part", "Transforming selected part", "Ausgewähltes Teil wird transformiert"))
            }
            Puzzle5dTransformStage::Parts => {
                let Some(row) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dTransformStage::Complete;
                    let coalesce_key = match self.tool_id {
                        "translateSelection" => "gumball-translate",
                        "rotateSelection" => "gumball-rotate",
                        "scaleSelection" => "gumball-scale",
                        _ => return Err(Fault::from("puzzle5d-transform-tool-mismatch")),
                    };
                    let mutations = std::mem::take(&mut self.mutations);
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: mutations, coalesce_key: Some(coalesce_key.to_string()), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                self.part_cursor += 1;
                let Some(id) = row.get("id").and_then(Value::as_str) else {
                    return Ok(Self::progress("puzzle5d-transform-part", "Skipping malformed part", "Fehlerhaftes Teil wird übersprungen"));
                };
                if !self.selected.contains(id) {
                    return Ok(Self::progress("puzzle5d-transform-part", "Scanning part", "Teil wird geprüft"));
                }
                let part_3d = row.get("3d").and_then(Value::as_object);
                let mutation = match self.tool_id {
                    "translateSelection" => {
                        let origin = part_3d.and_then(|part| part.get("origin")).and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok()).unwrap_or_default();
                        crate::artifacts::puzzle5d::mutations::move_part_3d(id.to_string(), [origin[0] + Self::axis(command, "dx", 0.0), origin[1] + Self::axis(command, "dy", 0.0), origin[2] + Self::axis(command, "dz", 0.0)])
                    }
                    "rotateSelection" => {
                        let orientation = part_3d.and_then(|part| part.get("orientation")).and_then(|value| serde_json::from_value::<[f64; 4]>(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0, 1.0]);
                        let delta = quat_from_axis_angle(Self::axis(command, "ax", 0.0), Self::axis(command, "ay", 0.0), Self::axis(command, "az", 0.0), Self::axis(command, "angle", 0.0));
                        crate::artifacts::puzzle5d::mutations::rotate_part_3d(id.to_string(), Some(quat_mul(delta, orientation)))
                    }
                    "scaleSelection" => {
                        let scale = part_3d.and_then(|part| part.get("scale"));
                        let current = match scale {
                            Some(Value::Number(value)) => [value.as_f64().unwrap_or(1.0); 3],
                            Some(Value::Array(values)) => [values.first().and_then(Value::as_f64).unwrap_or(1.0), values.get(1).and_then(Value::as_f64).unwrap_or(1.0), values.get(2).and_then(Value::as_f64).unwrap_or(1.0)],
                            _ => [1.0; 3],
                        };
                        crate::artifacts::puzzle5d::mutations::scale_part_3d(
                            id.to_string(),
                            Some(crate::artifacts::puzzle5d::Puzzle5dScale::Vec3([current[0] * Self::axis(command, "sx", 1.0), current[1] * Self::axis(command, "sy", 1.0), current[2] * Self::axis(command, "sz", 1.0)])),
                        )
                    }
                    _ => return Err(Fault::from("puzzle5d-transform-tool-mismatch")),
                };
                self.mutations.push(mutation);
                Ok(Self::progress("puzzle5d-transform-part", "Transforming selected part", "Ausgewähltes Teil wird transformiert"))
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
        self.stage == Puzzle5dTransformStage::Closing && self.mutations.is_empty() && self.selected.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dScalarConfigStage {
    Apply,
    Complete,
    Closing,
}

struct Puzzle5dScalarConfigWork {
    tool_id: &'static str,
    stage: Puzzle5dScalarConfigStage,
}

impl Puzzle5dScalarConfigWork {
    fn new(tool_id: &'static str) -> Self {
        Self { tool_id, stage: Puzzle5dScalarConfigStage::Apply }
    }

    fn vector(value: Option<&Value>) -> [f64; 3] {
        let values = value.and_then(Value::as_array);
        [values.and_then(|values| values.first()).and_then(Value::as_f64).unwrap_or(0.0), values.and_then(|values| values.get(1)).and_then(Value::as_f64).unwrap_or(0.0), values.and_then(|values| values.get(2)).and_then(Value::as_f64).unwrap_or(0.0)]
    }

    fn camera2d(value: &Value) -> Puzzle5dCamera2d {
        Puzzle5dCamera2d { x: value.get("x").and_then(Value::as_f64).unwrap_or(0.0), y: value.get("y").and_then(Value::as_f64).unwrap_or(0.0), zoom: value.get("zoom").and_then(Value::as_f64).unwrap_or(1.0) }
    }

    fn camera3d(value: &Value) -> Puzzle5dCamera3d {
        Puzzle5dCamera3d { position: Self::vector(value.get("position")), target: Self::vector(value.get("target")), zoom: value.get("zoom").and_then(Value::as_f64).unwrap_or(1.0) }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dScalarConfigWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        Some(1)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        _snapshot: &Puzzle5dPlaySnapshot,
        config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        if self.stage != Puzzle5dScalarConfigStage::Apply {
            return Err(Fault::from("puzzle5d-scalar-config-repolled"));
        }
        self.stage = Puzzle5dScalarConfigStage::Complete;
        let args = command.args();
        let mutation = match self.tool_id {
            "setCamera" | "setCamera2d" | "setCamera3d" => {
                let Some(camera) = args.and_then(|args| args.get("camera")) else {
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                };
                let is_2d = self.tool_id == "setCamera2d" || (self.tool_id == "setCamera" && (args.and_then(|args| args.get("surfaceId")).and_then(Value::as_str) == Some(board2d::SURFACE_ID) || camera.get("position").is_none()));
                if is_2d {
                    Puzzle5dConfigMutation::SetCamera2d { camera: Self::camera2d(camera) }
                } else {
                    Puzzle5dConfigMutation::SetCamera3d { camera: Self::camera3d(camera) }
                }
            }
            "setGridFactor" => {
                let Some(value) = args.and_then(|args| args.get("value")).and_then(Value::as_f64) else {
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                };
                Puzzle5dConfigMutation::SetGridFactor { value }
            }
            "setGridSnapEnabled" => Puzzle5dConfigMutation::SetGridSnapEnabled { enabled: args.and_then(|args| args.get("enabled")).and_then(Value::as_bool).unwrap_or(false) },
            "setLodMode" => {
                let Some(mode) = args.and_then(|args| args.get("value").or_else(|| args.get("mode"))).and_then(Value::as_str) else {
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                };
                Puzzle5dConfigMutation::SetLodMode { mode: mode.to_string() }
            }
            "setSuggestionOffset" => {
                let Some(distance) = args.and_then(|args| args.get("distance").or_else(|| args.get("value"))).and_then(Value::as_f64) else {
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                };
                Puzzle5dConfigMutation::SetSuggestionOffset { distance: distance.clamp(PUZZLE5D_SUGGESTION_OFFSET_MIN, PUZZLE5D_SUGGESTION_OFFSET_MAX) }
            }
            "setBrushPlacementOverlapBudget" => {
                let Some(value) = args.and_then(|args| args.get("value")).and_then(Value::as_f64) else {
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                };
                Puzzle5dConfigMutation::SetOverlapBudget { value: value.clamp(0.0, 1.0) }
            }
            "engagementControlSelect" => {
                let candidate_id = args.and_then(|args| args.get("id").or_else(|| args.get("value"))).and_then(Value::as_str).unwrap_or("");
                let Some(index) = candidate_id.strip_prefix("puzzle5d.brush.candidate.").and_then(|value| value.parse::<usize>().ok()) else {
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                };
                Puzzle5dConfigMutation::SetBrushCandidateIndex { index }
            }
            "engagementInput" => {
                let window_id = args.and_then(|args| args.get("window")).and_then(Value::as_str).unwrap_or(board2d::WINDOW_KIND_ID);
                if !PUZZLE5D_PLAY_WINDOWS.contains(&window_id) {
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                }
                let value = args.and_then(|args| args.get("value")).and_then(Value::as_str).unwrap_or("");
                Puzzle5dConfigMutation::SetEngagementInput { window_id: window_id.to_string(), value: value.to_string() }
            }
            "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => {
                let mut sun = config.sun.clone();
                semio_framework_plugin::apply_world3d_sun_action(&mut sun, self.tool_id, args);
                if sun == config.sun {
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                }
                Puzzle5dConfigMutation::SetSun { sun }
            }
            _ => return Err(Fault::from("puzzle5d-scalar-config-tool-mismatch")),
        };
        Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { config_mutations: vec![mutation], ui_scope: UiDirtyScope::Full, ..Default::default() }))
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dScalarConfigStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dScalarConfigStage::Closing
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
        if self.tool_id == "setObjectKindWeight" {
            "parts"
        } else {
            "grips"
        }
    }

    fn weights<'a>(&self, config: &'a Puzzle5dConfig) -> &'a HashMap<String, f64> {
        if self.tool_id == "setObjectKindWeight" {
            &config.object_kind_weights
        } else {
            &config.vortex_kind_weights
        }
    }

    fn catalog<'a>(&self, snapshot: &'a Puzzle5dPlaySnapshot) -> &'a [Value] {
        snapshot.0.get("kindCatalogs").and_then(|value| value.get(self.section())).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default()
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
                    if self.tool_id == "setObjectKindWeight" {
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
                let Some(part) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
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
                let Some(part) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
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
                let mutation = if self.tool_id == "setObjectKindWeight" {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dEngagementAbortStage {
    Input,
    BoardUtility,
    WorldUtility,
    Publish,
    Complete,
    Closing,
}

struct Puzzle5dEngagementAbortWork {
    stage: Puzzle5dEngagementAbortStage,
    input: Option<Puzzle5dConfigMutation>,
    effects: [Option<Effect>; 2],
}

impl Default for Puzzle5dEngagementAbortWork {
    fn default() -> Self {
        Self { stage: Puzzle5dEngagementAbortStage::Input, input: None, effects: [None, None] }
    }
}

impl Puzzle5dEngagementAbortWork {
    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dEngagementAbortWork {
    fn tool_id(&self) -> &'static str {
        "engagementAbort"
    }

    fn extent(&self, _command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        Some(4)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        _snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let args = command.args();
        let window_id = args.and_then(|args| args.get("window")).and_then(Value::as_str).unwrap_or(board2d::WINDOW_KIND_ID);
        let utility_id = if window_id == world3d::WINDOW_KIND_ID { "move" } else { "select" };
        match self.stage {
            Puzzle5dEngagementAbortStage::Input => {
                if PUZZLE5D_PLAY_WINDOWS.contains(&window_id) {
                    self.input = Some(Puzzle5dConfigMutation::SetEngagementInput { window_id: window_id.to_string(), value: String::new() });
                }
                self.stage = Puzzle5dEngagementAbortStage::BoardUtility;
                Ok(Self::progress("puzzle5d-engagement-abort-input", "Clearing engagement input", "Eingabe wird geleert"))
            }
            Puzzle5dEngagementAbortStage::BoardUtility => {
                self.effects[0] = Some(Effect::SetActiveUtility { window_id: board2d::WINDOW_KIND_ID.to_string(), utility_id: utility_id.to_string() });
                self.stage = Puzzle5dEngagementAbortStage::WorldUtility;
                Ok(Self::progress("puzzle5d-engagement-abort-board", "Preparing board utility", "Board-Werkzeug wird vorbereitet"))
            }
            Puzzle5dEngagementAbortStage::WorldUtility => {
                self.effects[1] = Some(Effect::SetActiveUtility { window_id: world3d::WINDOW_KIND_ID.to_string(), utility_id: utility_id.to_string() });
                self.stage = Puzzle5dEngagementAbortStage::Publish;
                Ok(Self::progress("puzzle5d-engagement-abort-world", "Preparing world utility", "Welt-Werkzeug wird vorbereitet"))
            }
            Puzzle5dEngagementAbortStage::Publish => {
                let mut config_mutations = Vec::with_capacity(1);
                if let Some(input) = self.input.take() {
                    config_mutations.push(input);
                }
                let mut effects = Vec::with_capacity(2);
                if let Some(effect) = self.effects[0].take() {
                    effects.push(effect);
                }
                if let Some(effect) = self.effects[1].take() {
                    effects.push(effect);
                }
                self.stage = Puzzle5dEngagementAbortStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { config_mutations, effects, ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dEngagementAbortStage::Complete => Err(Fault::from("puzzle5d-engagement-abort-complete-repolled")),
            Puzzle5dEngagementAbortStage::Closing => Err(Fault::from("puzzle5d-engagement-abort-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dEngagementAbortStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.input.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        for effect in &mut self.effects {
            if effect.take().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dEngagementAbortStage::Closing && self.input.is_none() && self.effects.iter().all(Option::is_none)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dEngagementSubmitStage {
    Parse,
    BoardConfig,
    WorldConfig,
    BoardEffect,
    WorldEffect,
    Input,
    Publish,
    Complete,
    Closing,
}

struct Puzzle5dEngagementSubmitWork {
    stage: Puzzle5dEngagementSubmitStage,
    emit: Option<Emit<Puzzle5dMutation, Puzzle5dConfigMutation>>,
    utility: Option<String>,
    window_id: Option<String>,
}

impl Default for Puzzle5dEngagementSubmitWork {
    fn default() -> Self {
        Self { stage: Puzzle5dEngagementSubmitStage::Parse, emit: Some(Emit::default()), utility: None, window_id: None }
    }
}

impl Puzzle5dEngagementSubmitWork {
    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn emit_mut(&mut self) -> Result<&mut Emit<Puzzle5dMutation, Puzzle5dConfigMutation>, Fault> {
        self.emit.as_mut().ok_or_else(|| Fault::from("puzzle5d-engagement-submit-emit-owner"))
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dEngagementSubmitWork {
    fn tool_id(&self) -> &'static str {
        "engagementSubmit"
    }

    fn extent(&self, _command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        Some(7)
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
            Puzzle5dEngagementSubmitStage::Parse => {
                let args = command.args();
                let window_id = args.and_then(|args| args.get("window")).and_then(Value::as_str).unwrap_or(board2d::WINDOW_KIND_ID).to_string();
                let token = args.and_then(|args| args.get("value")).and_then(Value::as_str).unwrap_or("").trim().to_lowercase();
                self.utility = match token.as_str() {
                    "select" if window_id == world3d::WINDOW_KIND_ID => Some("move".to_string()),
                    "select" | "brush" | "fill" => Some(token),
                    _ => None,
                };
                self.window_id = Some(window_id);
                self.stage = if self.utility.is_some() { Puzzle5dEngagementSubmitStage::BoardConfig } else { Puzzle5dEngagementSubmitStage::Input };
                Ok(Self::progress("puzzle5d-engagement-submit-parse", "Reading engagement command", "Eingabebefehl wird gelesen"))
            }
            Puzzle5dEngagementSubmitStage::BoardConfig => {
                let value = self.utility.clone();
                self.emit_mut()?.config_mutations.push(Puzzle5dConfigMutation::SetActiveUtility { window_id: board2d::WINDOW_KIND_ID.to_string(), value });
                self.stage = Puzzle5dEngagementSubmitStage::WorldConfig;
                Ok(Self::progress("puzzle5d-engagement-submit-board-config", "Preparing board utility", "Board-Werkzeug wird vorbereitet"))
            }
            Puzzle5dEngagementSubmitStage::WorldConfig => {
                let value = self.utility.clone();
                self.emit_mut()?.config_mutations.push(Puzzle5dConfigMutation::SetActiveUtility { window_id: world3d::WINDOW_KIND_ID.to_string(), value });
                self.stage = Puzzle5dEngagementSubmitStage::BoardEffect;
                Ok(Self::progress("puzzle5d-engagement-submit-world-config", "Preparing world utility", "Welt-Werkzeug wird vorbereitet"))
            }
            Puzzle5dEngagementSubmitStage::BoardEffect => {
                let utility_id = self.utility.clone().ok_or_else(|| Fault::from("puzzle5d-engagement-submit-utility-owner"))?;
                self.emit_mut()?.effects.push(Effect::SetActiveUtility { window_id: board2d::WINDOW_KIND_ID.to_string(), utility_id });
                self.stage = Puzzle5dEngagementSubmitStage::WorldEffect;
                Ok(Self::progress("puzzle5d-engagement-submit-board-effect", "Preparing board publication", "Board-Veröffentlichung wird vorbereitet"))
            }
            Puzzle5dEngagementSubmitStage::WorldEffect => {
                let utility_id = self.utility.clone().ok_or_else(|| Fault::from("puzzle5d-engagement-submit-utility-owner"))?;
                self.emit_mut()?.effects.push(Effect::SetActiveUtility { window_id: world3d::WINDOW_KIND_ID.to_string(), utility_id });
                self.stage = Puzzle5dEngagementSubmitStage::Input;
                Ok(Self::progress("puzzle5d-engagement-submit-world-effect", "Preparing world publication", "Welt-Veröffentlichung wird vorbereitet"))
            }
            Puzzle5dEngagementSubmitStage::Input => {
                let window_id = self.window_id.clone().ok_or_else(|| Fault::from("puzzle5d-engagement-submit-window-owner"))?;
                if PUZZLE5D_PLAY_WINDOWS.contains(&window_id.as_str()) {
                    self.emit_mut()?.config_mutations.push(Puzzle5dConfigMutation::SetEngagementInput { window_id, value: String::new() });
                }
                self.stage = Puzzle5dEngagementSubmitStage::Publish;
                Ok(Self::progress("puzzle5d-engagement-submit-input", "Clearing engagement input", "Eingabe wird geleert"))
            }
            Puzzle5dEngagementSubmitStage::Publish => {
                self.stage = Puzzle5dEngagementSubmitStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(self.emit.take().ok_or_else(|| Fault::from("puzzle5d-engagement-submit-publish-owner"))?))
            }
            Puzzle5dEngagementSubmitStage::Complete => Err(Fault::from("puzzle5d-engagement-submit-complete-repolled")),
            Puzzle5dEngagementSubmitStage::Closing => Err(Fault::from("puzzle5d-engagement-submit-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dEngagementSubmitStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.emit.take().is_some() || self.utility.take().is_some() || self.window_id.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dEngagementSubmitStage::Closing && self.emit.is_none() && self.utility.is_none() && self.window_id.is_none()
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
}

impl Default for Puzzle5dFocusSelectionWork {
    fn default() -> Self {
        Self { stage: Puzzle5dFocusSelectionStage::Selection, selection_cursor: 0, part_cursor: 0, selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS), sum_2d: [0.0; 2], sum_3d: [0.0; 3], matched: 0 }
    }
}

impl Puzzle5dFocusSelectionWork {
    fn source(interaction: &protocol::InteractionState) -> &[String] {
        interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE5D_GRANULARITY_PART).map(|selection| selection.ids.as_slice()).unwrap_or_default()
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

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
        let selected = Self::source(interaction).len();
        let parts = snapshot.0.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let items = selected.checked_add(parts)?.checked_add(1)?;
        (selected <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        _command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        config: &Puzzle5dConfig,
        interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
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
                let Some(row) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dFocusSelectionStage::Publish;
                    return Ok(Self::progress("puzzle5d-focus-selection-publish", "Preparing camera focus", "Kamerafokus wird vorbereitet"));
                };
                self.part_cursor += 1;
                if row.get("id").and_then(Value::as_str).is_some_and(|id| self.selected.contains(id)) {
                    self.sum_2d[0] += Self::scalar(row, "2d", "x");
                    self.sum_2d[1] += Self::scalar(row, "2d", "y");
                    self.sum_3d[0] += Self::axis(row, "3d", "origin", 0);
                    self.sum_3d[1] += Self::axis(row, "3d", "origin", 1);
                    self.sum_3d[2] += Self::axis(row, "3d", "origin", 2);
                    self.matched += 1;
                }
                Ok(Self::progress("puzzle5d-focus-selection-part", "Scanning selected part", "Ausgewähltes Teil wird geprüft"))
            }
            Puzzle5dFocusSelectionStage::Publish => {
                self.stage = Puzzle5dFocusSelectionStage::Complete;
                if self.matched == 0 {
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                }
                let divisor = self.matched as f64;
                let target = [self.sum_3d[0] / divisor, self.sum_3d[1] / divisor, self.sum_3d[2] / divisor];
                let offset = [config.camera3d.position[0] - config.camera3d.target[0], config.camera3d.position[1] - config.camera3d.target[1], config.camera3d.position[2] - config.camera3d.target[2]];
                let mut next = config.clone();
                next.camera2d.x = self.sum_2d[0] / divisor;
                next.camera2d.y = self.sum_2d[1] / divisor;
                next.camera3d.target = target;
                next.camera3d.position = [target[0] + offset[0], target[1] + offset[1], target[2] + offset[2]];
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { config_mutations: vec![Puzzle5dConfigMutation::Snapshot { config: next }], ui_scope: UiDirtyScope::Full, ..Default::default() }))
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
        if selected.is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dFocusSelectionStage::Closing && self.selected.is_empty()
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
}

impl Default for Puzzle5dPatchPartWork {
    fn default() -> Self {
        Self {
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
            "partKind" => text.map(|text| crate::artifacts::puzzle5d::mutations::change_part_kind(part.id.clone(), Some(text.to_string()))),
            "anchor" => text.map(|text| {
                let anchor = match text.to_ascii_lowercase().as_str() {
                    "derived" | "connected" => crate::artifacts::puzzle5d::Puzzle5dPartAnchor::Derived,
                    _ => crate::artifacts::puzzle5d::Puzzle5dPartAnchor::Fixed,
                };
                crate::artifacts::puzzle5d::mutations::change_part_anchor(part.id.clone(), anchor)
            }),
            "text" => text.map(|text| crate::artifacts::puzzle5d::mutations::edit_part_2d_text(part.id.clone(), Some(text.to_string()))),
            "label" => Some(crate::artifacts::puzzle5d::mutations::edit_part_3d_label(part.id.clone(), text.filter(|text| !text.is_empty()).map(str::to_string))),
            "meshUrl" => Some(crate::artifacts::puzzle5d::mutations::change_part_3d_mesh(part.id.clone(), text.filter(|text| !text.is_empty()).map(str::to_string))),
            "x" => puzzle5d_resolve_number_edit(part.part_2d.x, value, delta).map(|updated| crate::artifacts::puzzle5d::mutations::move_part_2d(part.id.clone(), updated, part.part_2d.y)),
            "y" => puzzle5d_resolve_number_edit(part.part_2d.y, value, delta).map(|updated| crate::artifacts::puzzle5d::mutations::move_part_2d(part.id.clone(), part.part_2d.x, updated)),
            _ => {
                let axis = puzzle5d_axis_index(field, "origin")?;
                let updated = puzzle5d_resolve_number_edit(part.part_3d.origin[axis], value, delta)?;
                let mut origin = part.part_3d.origin;
                origin[axis] = updated;
                Some(crate::artifacts::puzzle5d::mutations::move_part_3d(part.id.clone(), origin))
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

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let items = Self::source_len(command).checked_add(snapshot.0.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?;
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
                let Some(row) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)).cloned() else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                self.part_cursor += 1;
                let part: Puzzle5dPart = serde_json::from_value(row).map_err(|_| Fault::from("puzzle5d-patch-part-malformed"))?;
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
            return Some(crate::artifacts::puzzle5d::mutations::change_fastener_kind(fastener.id.clone(), value.and_then(Value::as_str).filter(|text| !text.is_empty()).map(str::to_string)));
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
        Some(crate::artifacts::puzzle5d::mutations::replace_fastener_geometry(fastener.id.clone(), geometry[0], geometry[1], geometry[2], geometry[3], geometry[4], geometry[5], geometry[6], geometry[7]))
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dPatchFastenerWork {
    fn tool_id(&self) -> &'static str {
        "patchFastener"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let items = Self::source_len(command).checked_add(snapshot.0.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len))?;
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
                let Some(row) = snapshot.0.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)).cloned() else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                self.fastener_cursor += 1;
                let fastener: Puzzle5dFastener = serde_json::from_value(row).map_err(|_| Fault::from("puzzle5d-patch-fastener-malformed"))?;
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
            if let Some(value) = args.get(*key) {
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
        snapshot.0.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len).checked_add(2).filter(|items| *items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        match self.stage {
            Puzzle5dEditFastenerStage::Scan => {
                let target = Self::id(command);
                if target.is_empty() {
                    self.stage = Puzzle5dEditFastenerStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                }
                let Some(row) = snapshot.0.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.cursor)).cloned() else {
                    self.stage = Puzzle5dEditFastenerStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                };
                self.cursor += 1;
                if row.get("id").and_then(Value::as_str) == Some(target) {
                    self.fastener = Some(serde_json::from_value(row).map_err(|_| Fault::from("puzzle5d-edit-fastener-malformed"))?);
                    self.stage = Puzzle5dEditFastenerStage::Kind;
                }
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-edit-fastener-scan", "Finding fastener", "Verbindung wird gesucht"))
            }
            Puzzle5dEditFastenerStage::Kind => {
                let Some(fastener) = self.fastener.as_ref() else { return Err(Fault::from("puzzle5d-edit-fastener-owner")) };
                if let Some(kind) = Self::updated_kind(command, fastener) {
                    self.mutations.push(crate::artifacts::puzzle5d::mutations::change_fastener_kind(fastener.id.clone(), kind));
                }
                self.stage = Puzzle5dEditFastenerStage::Geometry;
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-edit-fastener-kind", "Updating fastener kind", "Verbindungsart wird aktualisiert"))
            }
            Puzzle5dEditFastenerStage::Geometry => {
                let Some(fastener) = self.fastener.as_ref() else { return Err(Fault::from("puzzle5d-edit-fastener-owner")) };
                if let Some(geometry) = Self::updated_geometry(command, fastener) {
                    self.mutations.push(crate::artifacts::puzzle5d::mutations::replace_fastener_geometry(fastener.id.clone(), geometry[0], geometry[1], geometry[2], geometry[3], geometry[4], geometry[5], geometry[6], geometry[7]));
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
        let Some(part) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
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
        let parts = snapshot.0.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = snapshot.0.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = snapshot.0.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
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
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-retarget-fastener-work-capacity"));
        }
        self.processed_units += 1;
        match self.stage {
            Puzzle5dRetargetFastenerStage::Fastener => {
                let Some(id) = Self::argument(command, "id", "fastenerId") else { return Ok(self.complete_empty()) };
                let Some(row) = snapshot.0.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)).cloned() else {
                    return Ok(self.complete_empty());
                };
                self.fastener_cursor += 1;
                if row.get("id").and_then(Value::as_str) == Some(id) {
                    let fastener: Puzzle5dFastener = serde_json::from_value(row).map_err(|_| Fault::from("puzzle5d-retarget-fastener-malformed"))?;
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
                if let Some(row) = snapshot.0.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
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
                let rows = snapshot.0.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
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
                self.mutations.push(crate::artifacts::puzzle5d::mutations::disconnect_grips(id));
                self.stage = Puzzle5dRetargetFastenerStage::Connect;
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-retarget-connect", "Connecting retargeted fastener", "Neu ausgerichtete Verbindung wird erstellt"))
            }
            Puzzle5dRetargetFastenerStage::Connect => {
                let fastener = self.fastener.as_ref().ok_or_else(|| Fault::from("puzzle5d-retarget-fastener-owner"))?;
                self.mutations.push(crate::artifacts::puzzle5d::mutations::connect_grips(
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
        let origin = part.get("3d").and_then(|part| part.get("origin")).and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok()).unwrap_or_default();
        let orientation = part.get("3d").and_then(|part| part.get("orientation")).and_then(|value| serde_json::from_value::<[f64; 4]>(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let position = grip.get("3d").and_then(|grip| grip.get("position")).and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok()).unwrap_or_default();
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
        let parts = snapshot.0.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = snapshot.0.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = snapshot.0.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
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
                let Some(part) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
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
                let Some(part) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
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
                if let Some(row) = snapshot.0.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
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
                let rows = snapshot.0.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
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
                self.mutations.push(crate::artifacts::puzzle5d::mutations::connect_grips(id, source, target, kind, arg("gap"), arg("shift"), arg("rise"), arg("rotation"), arg("turn"), arg("tilt"), arg("x"), arg("y")));
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

    fn patch(command: &Puzzle5dCommand, grip: &mut crate::artifacts::puzzle5d::Puzzle5dGrip) -> bool {
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
        let items = Self::source_len(command).checked_add(snapshot.0.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?;
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
                let Some(part) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
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
                let mut grip: crate::artifacts::puzzle5d::Puzzle5dGrip = serde_json::from_value(grip_value).map_err(|_| Fault::from("puzzle5d-patch-grip-malformed"))?;
                let full_id = puzzle5d_grip_full_id(part_id, &grip.id);
                if self.selected.contains(&full_id) && Self::patch(command, &mut grip) {
                    let grip_id = grip.id.clone();
                    self.mutations.push(crate::artifacts::puzzle5d::mutations::replace_part_grip(part_id.to_string(), grip_id, grip));
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
        snapshot.0.get("fasteners").and_then(Value::as_array).map_or(Some(1), |fasteners| fasteners.len().checked_add(1)).filter(|items| *items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        _config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let target = command.args().and_then(|args| args.get("id").or_else(|| args.get("fastenerId"))).and_then(Value::as_str).filter(|id| !id.is_empty());
        let Some(row) = snapshot.0.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.cursor)) else {
            return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
        };
        self.cursor += 1;
        if target == row.get("id").and_then(Value::as_str) {
            if let Some(id) = target {
                self.mutations.push(crate::artifacts::puzzle5d::mutations::disconnect_grips(id.to_string()));
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
    grips: Vec<crate::artifacts::puzzle5d::Puzzle5dGrip>,
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

    fn catalogs(snapshot: &Puzzle5dPlaySnapshot) -> &[Value] {
        snapshot.0.get("kindCatalogs").and_then(|catalogs| catalogs.get("parts")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default()
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
                    let grip_2d: crate::artifacts::puzzle5d::Puzzle5dGrip2d = match template.get("2d").cloned() {
                        Some(value) => serde_json::from_value(value).map_err(|_| Fault::from("puzzle5d-add-node-grip2d-malformed"))?,
                        None => Default::default(),
                    };
                    let grip_3d: crate::artifacts::puzzle5d::Puzzle5dGrip3d = match template.get("3d").cloned() {
                        Some(value) => serde_json::from_value(value).map_err(|_| Fault::from("puzzle5d-add-node-grip3d-malformed"))?,
                        None => Default::default(),
                    };
                    self.grips.push(crate::artifacts::puzzle5d::Puzzle5dGrip { id: format!("v{}", self.grip_cursor), grip_kind: Some(grip_kind), grip_2d, grip_3d });
                    self.grip_cursor += 1;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-add-node-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                }
                let x = command.args().and_then(|args| args.get("x")).and_then(Value::as_f64).unwrap_or(120.0);
                let y = command.args().and_then(|args| args.get("y")).and_then(Value::as_f64).unwrap_or(120.0);
                let flat_to_world = 1.0 / 48.0;
                let origin = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.first()).map_or([x * flat_to_world, -y * flat_to_world, 0.0], |peer| {
                    let peer_2d = peer.get("2d");
                    let peer_3d = peer.get("3d");
                    let peer_x = peer_2d.and_then(|part| part.get("x")).and_then(Value::as_f64).unwrap_or_default();
                    let peer_y = peer_2d.and_then(|part| part.get("y")).and_then(Value::as_f64).unwrap_or_default();
                    let peer_origin = peer_3d.and_then(|part| part.get("origin")).and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok()).unwrap_or_default();
                    [peer_origin[0] + (x - peer_x) * flat_to_world, peer_origin[1] - (y - peer_y) * flat_to_world, peer_origin[2]]
                });
                let part_kind = Self::part_kind(command).to_string();
                let part = crate::artifacts::puzzle5d::Puzzle5dPart {
                    id: format!("part-{:016x}-0", self.operation_nonce),
                    part_kind: Some(part_kind.clone()),
                    anchor: Default::default(),
                    part_2d: crate::artifacts::puzzle5d::Puzzle5dPart2d { x, y, shape: Some("circle".to_string()), radius: Some(PUZZLE5D_DEFAULT_PART_RADIUS), text: Some(part_kind), ..Default::default() },
                    part_3d: crate::artifacts::puzzle5d::Puzzle5dPart3d { origin, mesh_url: self.mesh_url.take(), orientation: Some([0.0, 0.0, 0.0, 1.0]), ..Default::default() },
                    grips: std::mem::take(&mut self.grips),
                };
                self.mutation = Some(crate::artifacts::puzzle5d::mutations::create_part(part, None));
                self.stage = Puzzle5dAddNodeStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: self.mutation.take().into_iter().collect(), ui_scope: UiDirtyScope::Full, ..Default::default() }))
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
    grips: Vec<crate::artifacts::puzzle5d::Puzzle5dGrip>,
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

    fn args<'a>(&'a self, command: &'a Puzzle5dCommand) -> Option<&'a Value> {
        self.payload.as_ref().or_else(|| command.args())
    }

    fn owned_part_kind<'a>(&'a self, command: &'a Puzzle5dCommand) -> &'a str {
        self.args(command).and_then(|args| args.get("partKind").or_else(|| args.get("objectKindId")).or_else(|| args.get("nodeKind"))).and_then(Value::as_str).filter(|kind| !kind.is_empty()).unwrap_or("Part")
    }

    fn catalogs(snapshot: &Puzzle5dPlaySnapshot) -> &[Value] {
        Puzzle5dAddNodeWork::catalogs(snapshot)
    }

    fn target(&self, command: &Puzzle5dCommand, interaction: &protocol::InteractionState) -> Option<String> {
        self.args(command)
            .and_then(|args| args.get("targetVortexFullId").or_else(|| args.get("targetGripFullId")))
            .and_then(Value::as_str)
            .filter(|id| !id.is_empty())
            .map(str::to_string)
            .or_else(|| interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE5D_GRANULARITY_GRIP).and_then(|selection| selection.ids.first().cloned()))
    }

    fn world_direction(part: &Value, grip: &Value) -> [f64; 3] {
        let orientation = part.get("3d").and_then(|part| part.get("orientation")).and_then(|value| serde_json::from_value::<[f64; 4]>(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let direction = grip.get("3d").and_then(|grip| grip.get("direction")).and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok()).unwrap_or([0.0, 0.0, -1.0]);
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
        let items = Self::catalogs(snapshot).len().checked_add(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS)?.checked_add(snapshot.0.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?.checked_add(2)?;
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
                if entry.get("id").and_then(Value::as_str) == Some(self.owned_part_kind(command)) {
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
                    let grip_2d = template.get("2d").cloned().map(serde_json::from_value).transpose().map_err(|_| Fault::from("puzzle5d-add-brush-part-grip2d-malformed"))?.unwrap_or_default();
                    let grip_3d = template.get("3d").cloned().map(serde_json::from_value).transpose().map_err(|_| Fault::from("puzzle5d-add-brush-part-grip3d-malformed"))?.unwrap_or_default();
                    let id = format!("v{}", self.grip_cursor);
                    if self.created_grip_id.is_none() {
                        self.created_grip_id = Some(id.clone());
                    }
                    self.grips.push(crate::artifacts::puzzle5d::Puzzle5dGrip { id, grip_kind: Some(grip_kind), grip_2d, grip_3d });
                    self.grip_cursor += 1;
                    return Ok(Puzzle5dPatchPartWork::progress("puzzle5d-brush-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                }
                self.target_id = self.target(command, interaction);
                self.stage = if self.target_id.is_some() { Puzzle5dAddBrushPartStage::Target } else { Puzzle5dAddBrushPartStage::Create };
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht"))
            }
            Puzzle5dAddBrushPartStage::Target => {
                let Some(part) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
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
                let x = self.args(command).and_then(|args| args.get("x")).and_then(Value::as_f64).unwrap_or(120.0);
                let y = self.args(command).and_then(|args| args.get("y")).and_then(Value::as_f64).unwrap_or(120.0);
                let origin = match (self.target_position, self.target_direction) {
                    (Some(position), Some(direction)) => [position[0] + direction[0], position[1] + direction[1], position[2] + direction[2]],
                    _ => [x / 48.0, -y / 48.0, 0.0],
                };
                let part_kind = self.owned_part_kind(command).to_string();
                let id = self.args(command).and_then(|args| args.get("nodeId").or_else(|| args.get("partId")).or_else(|| args.get("objectId"))).and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string).unwrap_or_else(|| {
                    let id = format!("part-{:016x}-{}", self.operation_nonce, self.fresh_cursor);
                    self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                    id
                });
                let part = crate::artifacts::puzzle5d::Puzzle5dPart {
                    id: id.clone(),
                    part_kind: Some(part_kind.clone()),
                    anchor: Default::default(),
                    part_2d: crate::artifacts::puzzle5d::Puzzle5dPart2d { x, y, shape: Some("circle".to_string()), radius: Some(PUZZLE5D_DEFAULT_PART_RADIUS), text: Some(part_kind), ..Default::default() },
                    part_3d: crate::artifacts::puzzle5d::Puzzle5dPart3d { origin, mesh_url: self.mesh_url.take(), orientation: Some([0.0, 0.0, 0.0, 1.0]), ..Default::default() },
                    grips: std::mem::take(&mut self.grips),
                };
                self.created_id = Some(id);
                self.mutations.push(crate::artifacts::puzzle5d::mutations::create_part(part, None));
                self.stage = Puzzle5dAddBrushPartStage::Connect;
                Ok(Puzzle5dPatchPartWork::progress("puzzle5d-brush-create", "Creating brush part", "Pinselteil wird erstellt"))
            }
            Puzzle5dAddBrushPartStage::Connect => {
                if let (Some(source), Some(part), Some(grip)) = (self.target_id.as_ref(), self.created_id.as_ref(), self.created_grip_id.as_ref()) {
                    let id = self.args(command).and_then(|args| args.get("edgeId")).and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string).unwrap_or_else(|| {
                        let id = format!("fastener-{:016x}-{}", self.operation_nonce, self.fresh_cursor);
                        self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                        id
                    });
                    self.mutations.push(crate::artifacts::puzzle5d::mutations::connect_grips(id, source.clone(), puzzle5d_grip_full_id(part, grip), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
                }
                self.stage = Puzzle5dAddBrushPartStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }))
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
        }
    }
}

impl Puzzle5dBoardEventsWork {
    fn source<'a>(command: &'a Puzzle5dCommand) -> Result<&'a str, Fault> {
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
        self.event.as_mut().and_then(Value::as_object_mut).and_then(|event| event.get_mut("payload")).map(Value::take).unwrap_or(Value::Null)
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

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let bytes = Self::source(command).ok()?.len();
        let document_items = snapshot.0.get("parts").and_then(Value::as_array).map_or(0, Vec::len).checked_add(snapshot.0.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len))?.checked_add(2)?;
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
        let source = Self::source(command)?;
        match self.stage {
            Puzzle5dBoardEventsStage::Open | Puzzle5dBoardEventsStage::Scan => {
                self.scan_one(source)?;
                Ok(Self::progress("puzzle5d-board-event-scan", "Reading board event", "Board-Ereignis wird gelesen"))
            }
            Puzzle5dBoardEventsStage::Decode => {
                let start = self.event_start.ok_or_else(|| Fault::from("puzzle5d-board-events-event-owner-missing"))?;
                self.event = Some(serde_json::from_str(source.get(start..self.event_end).ok_or_else(|| Fault::from("puzzle5d-board-events-event-range"))?).map_err(|_| Fault::from("puzzle5d-board-events-event-malformed"))?);
                self.stage = Puzzle5dBoardEventsStage::Dispatch;
                Ok(Self::progress("puzzle5d-board-event-decode", "Decoding board event", "Board-Ereignis wird dekodiert"))
            }
            Puzzle5dBoardEventsStage::Dispatch => {
                let name = self.event.as_ref().and_then(|event| event.get("name")).and_then(Value::as_str).map(str::to_string);
                let payload = self.take_payload();
                match name.as_deref() {
                    Some("camera") => {
                        self.camera2d = Some(serde_json::from_value(payload).map_err(|_| Fault::from("puzzle5d-board-events-camera-malformed"))?);
                        self.next_event();
                    }
                    Some("nodeMove") => self.schedule_move(&payload),
                    Some("nodeDragEnd") => {
                        let mut payload = payload;
                        self.drag_moves = Some(payload.as_object_mut().and_then(|payload| payload.get_mut("moves")).map(Value::take).unwrap_or(Value::Array(Vec::new())));
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
                            self.push(crate::artifacts::puzzle5d::mutations::disconnect_grips(id.to_string()))?;
                        }
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
                let Some(part) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.pending_move_id = None;
                    self.stage = if self.drag_moves.is_some() { Puzzle5dBoardEventsStage::DragMove } else { Puzzle5dBoardEventsStage::Scan };
                    return Ok(Self::progress("puzzle5d-board-move", "Finding board node", "Board-Knoten wird gesucht"));
                };
                self.part_cursor += 1;
                if part.get("id").and_then(Value::as_str) == self.pending_move_id.as_deref() {
                    let current = part.get("2d");
                    let x = self.pending_move_x.unwrap_or_else(|| current.and_then(|value| value.get("x")).and_then(Value::as_f64).unwrap_or_default());
                    let y = self.pending_move_y.unwrap_or_else(|| current.and_then(|value| value.get("y")).and_then(Value::as_f64).unwrap_or_default());
                    let id = self.pending_move_id.take().expect("matched move id");
                    self.push(crate::artifacts::puzzle5d::mutations::move_part_2d(id, x, y))?;
                    self.stage = if self.drag_moves.is_some() { Puzzle5dBoardEventsStage::DragMove } else { Puzzle5dBoardEventsStage::Scan };
                }
                Ok(Self::progress("puzzle5d-board-move", "Finding board node", "Board-Knoten wird gesucht"))
            }
            Puzzle5dBoardEventsStage::ScanEdge => {
                if self.pending_source.is_none() || self.pending_target.is_none() {
                    self.next_event();
                    return Ok(Self::progress("puzzle5d-board-edge", "Checking board edge", "Board-Kante wird geprüft"));
                }
                if let Some(fastener) = snapshot.0.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
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
                self.push(crate::artifacts::puzzle5d::mutations::connect_grips(id, source, target, kind, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0))?;
                self.next_event();
                Ok(Self::progress("puzzle5d-board-edge", "Creating board edge", "Board-Kante wird erstellt"))
            }
            Puzzle5dBoardEventsStage::ScanDeleteEdges => {
                let Some(id) = self.pending_delete_id.as_deref() else {
                    self.next_event();
                    return Ok(Self::progress("puzzle5d-board-delete", "Deleting board node", "Board-Knoten wird gelöscht"));
                };
                if let Some(fastener) = snapshot.0.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let incident = fastener.get("source").and_then(Value::as_str).is_some_and(|grip| grip.split_once(':').is_some_and(|(part_id, _)| part_id == id))
                        || fastener.get("target").and_then(Value::as_str).is_some_and(|grip| grip.split_once(':').is_some_and(|(part_id, _)| part_id == id));
                    if incident {
                        if let Some(fastener_id) = fastener.get("id").and_then(Value::as_str) {
                            self.push(crate::artifacts::puzzle5d::mutations::disconnect_grips(fastener_id.to_string()))?;
                        }
                    }
                    return Ok(Self::progress("puzzle5d-board-delete-edge", "Removing attached edge", "Verbundene Kante wird entfernt"));
                }
                let id = self.pending_delete_id.take().expect("preflighted deleted part");
                self.push(crate::artifacts::puzzle5d::mutations::delete_part(id))?;
                self.next_event();
                Ok(Self::progress("puzzle5d-board-delete", "Deleting board node", "Board-Knoten wird gelöscht"))
            }
            Puzzle5dBoardEventsStage::Brush => {
                let brush = self.brush.as_mut().ok_or_else(|| Fault::from("puzzle5d-board-brush-owner-missing"))?;
                match <Puzzle5dAddBrushPartWork as crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>>>::step(brush, command, snapshot, config, interaction, hover)? {
                    crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de } => Ok(crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }),
                    crate::retained_command::PuzzleCommandWorkStep::Complete(emit) => {
                        let mut mutations = emit.artifact_mutations.into_iter();
                        self.brush_first = mutations.next();
                        self.brush_second = mutations.next();
                        if mutations.next().is_some() {
                            return Err(Fault::from("puzzle5d-board-brush-output-capacity"));
                        }
                        self.stage = Puzzle5dBoardEventsStage::DrainBrush;
                        Ok(Self::progress("puzzle5d-board-brush-transfer", "Publishing brush mutation", "Pinselmutation wird veröffentlicht"))
                    }
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
                let config_mutations = self
                    .camera2d
                    .take()
                    .map(|camera2d| {
                        let mut config = config.clone();
                        config.camera2d = camera2d;
                        vec![Puzzle5dConfigMutation::Snapshot { config }]
                    })
                    .unwrap_or_default();
                self.stage = Puzzle5dBoardEventsStage::Closing;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), config_mutations, ui_scope: UiDirtyScope::Full, ..Default::default() }))
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
        let Some(part) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
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
        let parts = snapshot.0.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = snapshot.0.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = snapshot.0.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
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
                if let Some(fastener) = snapshot.0.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
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
                let rows = snapshot.0.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
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
                let id = command.args().and_then(|args| args.get("id").or_else(|| args.get("fastenerId"))).and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string).unwrap_or_else(|| format!("fastener-{:016x}-0", self.operation_nonce));
                let fastener_kind = command.args().and_then(|args| args.get("fastenerKind").or_else(|| args.get("edgeKind"))).and_then(Value::as_str).filter(|kind| !kind.is_empty()).map(str::to_string);
                self.mutation = Some(crate::artifacts::puzzle5d::mutations::connect_grips(
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
        let parts = snapshot.0.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = snapshot.0.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
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
        match self.stage {
            Puzzle5dWorldRelocateStage::SourcePart => {
                let requested = command.args().and_then(|args| args.get("objectId")).and_then(Value::as_str).unwrap_or("");
                let Some(position) = Self::position(command) else { return Ok(self.complete()) };
                let Some(row) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)).cloned() else { return Ok(self.complete()) };
                self.part_cursor += 1;
                if row.get("id").and_then(Value::as_str) == Some(requested) {
                    let mut part: Puzzle5dPart = serde_json::from_value(row).map_err(|_| Fault::from("puzzle5d-world-relocate-source-malformed"))?;
                    if part.grips.len() > PUZZLE5D_RELOCATE_GRIPS_PER_PART {
                        return Err(Fault::from("puzzle5d-world-relocate-grip-capacity"));
                    }
                    part.part_3d.origin = position;
                    self.mutations.push(crate::artifacts::puzzle5d::mutations::move_part_3d(part.id.clone(), position));
                    if let Some(grip) = part.grips.first() {
                        self.source = Some(Puzzle5dWorldRelocateSource { part_id: part.id.clone(), grip_id: puzzle5d_grip_full_id(&part.id, &grip.id), world_position: world_grip_position(&part, grip) });
                    }
                    self.fastener_cursor = 0;
                    self.stage = Puzzle5dWorldRelocateStage::ExistingFasteners;
                }
                Ok(Self::progress("puzzle5d-world-relocate-source", "Finding moved part", "Verschobenes Teil wird gesucht"))
            }
            Puzzle5dWorldRelocateStage::ExistingFasteners => {
                let Some(row) = snapshot.0.get("fasteners").and_then(Value::as_array).and_then(|rows| rows.get(self.fastener_cursor)) else {
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
                let Some(row) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)).cloned() else { return Ok(self.complete()) };
                self.part_cursor += 1;
                if row.get("id").and_then(Value::as_str) == Some(source.part_id.as_str()) {
                    return Ok(Self::progress("puzzle5d-world-relocate-candidate-part", "Skipping moved part", "Verschobenes Teil wird übersprungen"));
                }
                let part: Puzzle5dPart = serde_json::from_value(row).map_err(|_| Fault::from("puzzle5d-world-relocate-candidate-malformed"))?;
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
                self.mutations.push(crate::artifacts::puzzle5d::mutations::connect_grips(id, source.grip_id.clone(), candidate.grip_id.clone(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
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
    mutations: Vec<Puzzle5dMutation>,
}

impl Default for Puzzle5dSetActiveExampleWork {
    fn default() -> Self {
        Self { stage: Puzzle5dSetActiveExampleStage::ClearFasteners, cursor: 0, mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS) }
    }
}

impl Puzzle5dSetActiveExampleWork {
    fn target(command: &Puzzle5dCommand) -> Option<&'static Puzzle5dDocument> {
        let example_id = command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or("");
        match example_id {
            "" => Some(&EMPTY_EXAMPLE_DOCUMENT),
            PUZZLE5D_EXAMPLE_CONCRETE_FOREST | "concrete" => Some(&CONCRETE_FOREST_EXAMPLE_DOCUMENT),
            PUZZLE5D_EXAMPLE_NAKAGIN | "nakagin" => Some(&NAKAGIN_EXAMPLE_DOCUMENT),
            PUZZLE5D_EXAMPLE_CAPSULE_DREAM | "capsule-dream" | "capsule" => Some(&CAPSULE_DREAM_EXAMPLE_DOCUMENT),
            _ => None,
        }
    }

    fn compatibility_rows(document: &Puzzle5dDocument) -> &[Value] {
        document.kind_compatibility.as_ref().and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default()
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn push(&mut self, mutation: Puzzle5dMutation) -> Result<(), Fault> {
        if self.mutations.len() >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
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

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let target = Self::target(command)?;
        let items = snapshot
            .0
            .get("fasteners")
            .and_then(Value::as_array)
            .map_or(0, Vec::len)
            .checked_add(snapshot.0.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?
            .checked_add(snapshot.0.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len))?
            .checked_add(Self::compatibility_rows(target).len())?
            .checked_add(target.parts.len())?
            .checked_add(target.fasteners.len())?
            .checked_add(4)?;
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
        let Some(target) = Self::target(command) else {
            self.stage = Puzzle5dSetActiveExampleStage::Complete;
            return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
        };
        match self.stage {
            Puzzle5dSetActiveExampleStage::ClearFasteners => {
                if let Some(id) = snapshot.0.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.cursor)).and_then(|fastener| fastener.get("id")).and_then(Value::as_str) {
                    self.cursor += 1;
                    self.push(crate::artifacts::puzzle5d::mutations::disconnect_grips(id.to_string()))?;
                    return Ok(Self::progress("puzzle5d-example-clear-fastener", "Removing old fastener", "Alte Verbindung wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::ClearParts;
                Ok(Self::progress("puzzle5d-example-clear-part", "Removing old part", "Altes Teil wird entfernt"))
            }
            Puzzle5dSetActiveExampleStage::ClearParts => {
                if let Some(id) = snapshot.0.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.cursor)).and_then(|part| part.get("id")).and_then(Value::as_str) {
                    self.cursor += 1;
                    self.push(crate::artifacts::puzzle5d::mutations::delete_part(id.to_string()))?;
                    return Ok(Self::progress("puzzle5d-example-clear-part", "Removing old part", "Altes Teil wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::Label;
                Ok(Self::progress("puzzle5d-example-label", "Updating document label", "Dokumenttitel wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Label => {
                self.push(crate::artifacts::puzzle5d::mutations::rename_puzzle5d(target.label.clone()))?;
                self.stage = Puzzle5dSetActiveExampleStage::Domain;
                Ok(Self::progress("puzzle5d-example-domain", "Updating document domain", "Dokumentdomäne wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Domain => {
                self.push(crate::artifacts::puzzle5d::mutations::change_domain(target.domain.clone()))?;
                self.stage = Puzzle5dSetActiveExampleStage::Description;
                Ok(Self::progress("puzzle5d-example-description", "Updating description", "Beschreibung wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Description => {
                let description = target.meta.as_ref().and_then(|meta| meta.get("description")).and_then(Value::as_str).unwrap_or("");
                self.push(crate::artifacts::puzzle5d::mutations::change_description(description.to_string()))?;
                self.stage = Puzzle5dSetActiveExampleStage::ClearCompatibility;
                Ok(Self::progress("puzzle5d-example-clear-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"))
            }
            Puzzle5dSetActiveExampleStage::ClearCompatibility => {
                if let Some(row) = snapshot.0.get("kindCompatibility").and_then(Value::as_array).and_then(|rows| rows.get(self.cursor)) {
                    self.cursor += 1;
                    let source = row.get("source").and_then(Value::as_str).unwrap_or("").to_string();
                    let target = row.get("target").and_then(Value::as_str).unwrap_or("").to_string();
                    self.push(crate::artifacts::puzzle5d::mutations::disconnect_kind_compatibility(source, target))?;
                    return Ok(Self::progress("puzzle5d-example-clear-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::AddCompatibility;
                Ok(Self::progress("puzzle5d-example-add-compatibility", "Adding compatibility", "Kompatibilität wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddCompatibility => {
                if let Some(row) = Self::compatibility_rows(target).get(self.cursor).cloned() {
                    self.cursor += 1;
                    let row: crate::artifacts::puzzle5d::Puzzle5dKindCompatibility = serde_json::from_value(row).map_err(|_| Fault::from("puzzle5d-set-active-example-compatibility-malformed"))?;
                    self.push(crate::artifacts::puzzle5d::mutations::connect_kind_compatibility(row.source, row.target, row.bidirectional, row.important, row.specificity))?;
                    return Ok(Self::progress("puzzle5d-example-add-compatibility", "Adding compatibility", "Kompatibilität wird hinzugefügt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::Catalogs;
                Ok(Self::progress("puzzle5d-example-catalogs", "Updating kind catalogs", "Artenkataloge werden aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Catalogs => {
                let catalogs = target.kind_catalogs.as_ref().map(|catalogs| serde_json::from_value(catalogs.clone())).transpose().map_err(|_| Fault::from("puzzle5d-set-active-example-catalogs-malformed"))?;
                self.push(crate::artifacts::puzzle5d::mutations::replace_kind_catalogs(catalogs))?;
                self.stage = Puzzle5dSetActiveExampleStage::AddParts;
                Ok(Self::progress("puzzle5d-example-add-part", "Adding example part", "Beispielteil wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddParts => {
                if let Some(part) = target.parts.get(self.cursor) {
                    self.cursor += 1;
                    let value = serde_json::to_value(part).map_err(|_| Fault::from("puzzle5d-set-active-example-part-malformed"))?;
                    let part = serde_json::from_value(value).map_err(|_| Fault::from("puzzle5d-set-active-example-part-malformed"))?;
                    self.push(crate::artifacts::puzzle5d::mutations::create_part(part, None))?;
                    return Ok(Self::progress("puzzle5d-example-add-part", "Adding example part", "Beispielteil wird hinzugefügt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::AddFasteners;
                Ok(Self::progress("puzzle5d-example-add-fastener", "Adding example fastener", "Beispielverbindung wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddFasteners => {
                if let Some(fastener) = target.fasteners.get(self.cursor) {
                    self.cursor += 1;
                    self.push(crate::artifacts::puzzle5d::mutations::connect_grips(
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
                    return Ok(Self::progress("puzzle5d-example-add-fastener", "Adding example fastener", "Beispielverbindung wird hinzugefügt"));
                }
                self.stage = Puzzle5dSetActiveExampleStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit {
                    artifact_mutations: std::mem::take(&mut self.mutations),
                    config_mutations: vec![Puzzle5dConfigMutation::Snapshot { config: Puzzle5dRuntime::default() }],
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
        if self.mutations.pop().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dSetActiveExampleStage::Closing && self.mutations.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dPrecomputeCommandStage {
    Decode,
    Parts,
    Grips,
    Fasteners,
    CatalogParts,
    CatalogGrips,
    Positions,
    Indices,
    FillCount,
    BoardUtility,
    WorldUtility,
    Publish,
    Complete,
    Closing,
}

struct Puzzle5dPrecomputeCommandWork {
    tool_id: &'static str,
    stage: Puzzle5dPrecomputeCommandStage,
    part_cursor: usize,
    grip_cursor: usize,
    fastener_cursor: usize,
    catalog_cursor: usize,
    payload_cursor: usize,
    candidate_count: usize,
    requested_count: u32,
    processed_units: usize,
    emit: Option<Emit<Puzzle5dMutation, Puzzle5dConfigMutation>>,
}

impl Puzzle5dPrecomputeCommandWork {
    fn new(tool_id: &'static str) -> Self {
        Self { tool_id, stage: Puzzle5dPrecomputeCommandStage::Decode, part_cursor: 0, grip_cursor: 0, fastener_cursor: 0, catalog_cursor: 0, payload_cursor: 0, candidate_count: 0, requested_count: 0, processed_units: 0, emit: Some(Emit::default()) }
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn emit_mut(&mut self) -> Result<&mut Emit<Puzzle5dMutation, Puzzle5dConfigMutation>, Fault> {
        self.emit.as_mut().ok_or_else(|| Fault::from("puzzle5d-precompute-emit-owner"))
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dPrecomputeCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let positions = command.args().and_then(|args| args.get("positions")).and_then(Value::as_array).map_or(0, Vec::len);
        let indices = command.args().and_then(|args| args.get("indices")).and_then(Value::as_array).map_or(0, Vec::len);
        (positions <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS && indices <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS).then_some(1)
    }

    fn step(
        &mut self,
        command: &Puzzle5dCommand,
        snapshot: &Puzzle5dPlaySnapshot,
        config: &Puzzle5dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-precompute-work-capacity"));
        }
        self.processed_units += 1;
        match self.stage {
            Puzzle5dPrecomputeCommandStage::Decode => {
                self.requested_count = command.args().and_then(|args| args.get("count").or_else(|| args.get("value"))).and_then(Value::as_f64).map_or(0, |value| value.round().max(0.0) as u32).min(PUZZLE5D_FILL_COUNT_MAX);
                self.stage = if self.tool_id == "registerBrushMesh" { Puzzle5dPrecomputeCommandStage::Positions } else { Puzzle5dPrecomputeCommandStage::Parts };
                Ok(Self::progress("puzzle5d-precompute-decode", "Reading precompute command", "Vorberechnungsbefehl wird gelesen"))
            }
            Puzzle5dPrecomputeCommandStage::Parts => {
                let parts = snapshot.0.get("parts").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                let Some(part) = parts.get(self.part_cursor) else {
                    self.stage = Puzzle5dPrecomputeCommandStage::Fasteners;
                    return Ok(Self::progress("puzzle5d-precompute-fastener", "Scanning fastener owner", "Verbindungsinhaber wird geprüft"));
                };
                let grip_count = part.get("grips").and_then(Value::as_array).map_or(0, Vec::len);
                if grip_count > crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                    return Err(Fault::from("puzzle5d-precompute-grip-capacity"));
                }
                self.stage = Puzzle5dPrecomputeCommandStage::Grips;
                Ok(Self::progress("puzzle5d-precompute-part", "Scanning part owner", "Teilinhaber wird geprüft"))
            }
            Puzzle5dPrecomputeCommandStage::Grips => {
                let parts = snapshot.0.get("parts").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                let part = parts.get(self.part_cursor).ok_or_else(|| Fault::from("puzzle5d-precompute-part-cursor"))?;
                let grips = part.get("grips").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if grips.get(self.grip_cursor).is_some() {
                    self.grip_cursor += 1;
                    return Ok(Self::progress("puzzle5d-precompute-grip", "Scanning one grip owner", "Ein Griffinhaber wird geprüft"));
                }
                self.part_cursor += 1;
                self.grip_cursor = 0;
                self.stage = Puzzle5dPrecomputeCommandStage::Parts;
                Ok(Self::progress("puzzle5d-precompute-part", "Advancing part cursor", "Teilzeiger wird fortgesetzt"))
            }
            Puzzle5dPrecomputeCommandStage::Fasteners => {
                let fasteners = snapshot.0.get("fasteners").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if fasteners.get(self.fastener_cursor).is_some() {
                    self.fastener_cursor += 1;
                    return Ok(Self::progress("puzzle5d-precompute-fastener", "Scanning one fastener owner", "Ein Verbindungsinhaber wird geprüft"));
                }
                self.stage = Puzzle5dPrecomputeCommandStage::CatalogParts;
                Ok(Self::progress("puzzle5d-precompute-catalog-part", "Scanning part kind owner", "Teilartinhaber wird geprüft"))
            }
            Puzzle5dPrecomputeCommandStage::CatalogParts => {
                let entries = snapshot.0.get("kindCatalogs").and_then(|catalogs| catalogs.get("parts")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if entries.get(self.catalog_cursor).is_some() {
                    self.catalog_cursor += 1;
                    self.candidate_count += 1;
                    return Ok(Self::progress("puzzle5d-precompute-catalog-part", "Scanning one part kind", "Eine Teilart wird geprüft"));
                }
                self.catalog_cursor = 0;
                self.stage = Puzzle5dPrecomputeCommandStage::CatalogGrips;
                Ok(Self::progress("puzzle5d-precompute-catalog-grip", "Scanning grip kind owner", "Griffartinhaber wird geprüft"))
            }
            Puzzle5dPrecomputeCommandStage::CatalogGrips => {
                let entries = snapshot.0.get("kindCatalogs").and_then(|catalogs| catalogs.get("grips")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if entries.get(self.catalog_cursor).is_some() {
                    self.catalog_cursor += 1;
                    return Ok(Self::progress("puzzle5d-precompute-catalog-grip", "Scanning one grip kind", "Eine Griffart wird geprüft"));
                }
                self.stage = if self.tool_id == "setFillCount" { Puzzle5dPrecomputeCommandStage::FillCount } else { Puzzle5dPrecomputeCommandStage::Publish };
                Ok(Self::progress("puzzle5d-precompute-transfer", "Transferring precompute census", "Vorberechnungszensus wird übertragen"))
            }
            Puzzle5dPrecomputeCommandStage::Positions => {
                let positions = command.args().and_then(|args| args.get("positions")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if let Some(value) = positions.get(self.payload_cursor) {
                    if value.as_f64().filter(|value| value.is_finite()).is_none() {
                        return Err(Fault::from("puzzle5d-register-mesh-position-malformed"));
                    }
                    self.payload_cursor += 1;
                    return Ok(Self::progress("puzzle5d-register-mesh-position", "Reading one mesh position", "Eine Mesh-Position wird gelesen"));
                }
                self.payload_cursor = 0;
                self.stage = Puzzle5dPrecomputeCommandStage::Indices;
                Ok(Self::progress("puzzle5d-register-mesh-index", "Reading mesh indices", "Mesh-Indizes werden gelesen"))
            }
            Puzzle5dPrecomputeCommandStage::Indices => {
                let indices = command.args().and_then(|args| args.get("indices")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if let Some(value) = indices.get(self.payload_cursor) {
                    if value.as_u64().filter(|value| *value <= u32::MAX as u64).is_none() {
                        return Err(Fault::from("puzzle5d-register-mesh-index-malformed"));
                    }
                    self.payload_cursor += 1;
                    return Ok(Self::progress("puzzle5d-register-mesh-index", "Reading one mesh index", "Ein Mesh-Index wird gelesen"));
                }
                self.stage = Puzzle5dPrecomputeCommandStage::Publish;
                Ok(Self::progress("puzzle5d-register-mesh-transfer", "Transferring validated mesh owner", "Geprüfter Mesh-Inhaber wird übertragen"))
            }
            Puzzle5dPrecomputeCommandStage::FillCount => {
                let count = self.requested_count;
                self.emit_mut()?.config_mutations.push(Puzzle5dConfigMutation::SetFillCount { count });
                self.stage = Puzzle5dPrecomputeCommandStage::BoardUtility;
                Ok(Self::progress("puzzle5d-fill-count-owner", "Transferring fill count", "Füllanzahl wird übertragen"))
            }
            Puzzle5dPrecomputeCommandStage::BoardUtility => {
                self.emit_mut()?.config_mutations.push(Puzzle5dConfigMutation::SetActiveUtility { window_id: board2d::WINDOW_KIND_ID.to_string(), value: Some("fill".to_string()) });
                self.stage = Puzzle5dPrecomputeCommandStage::WorldUtility;
                Ok(Self::progress("puzzle5d-fill-board-utility", "Transferring board fill utility", "Board-Füllwerkzeug wird übertragen"))
            }
            Puzzle5dPrecomputeCommandStage::WorldUtility => {
                self.emit_mut()?.config_mutations.push(Puzzle5dConfigMutation::SetActiveUtility { window_id: world3d::WINDOW_KIND_ID.to_string(), value: Some("fill".to_string()) });
                self.stage = Puzzle5dPrecomputeCommandStage::Publish;
                Ok(Self::progress("puzzle5d-fill-world-utility", "Transferring world fill utility", "Welt-Füllwerkzeug wird übertragen"))
            }
            Puzzle5dPrecomputeCommandStage::Publish => {
                match self.tool_id {
                    "cycleBrushCandidate" => {
                        let index = if self.candidate_count == 0 { config.brush_candidate_index.saturating_add(1) } else { (config.brush_candidate_index + 1) % self.candidate_count };
                        self.emit_mut()?.config_mutations.push(Puzzle5dConfigMutation::SetBrushCandidateIndex { index });
                        self.emit_mut()?.ui_scope = UiDirtyScope::Full;
                    }
                    "setFillCount" => self.emit_mut()?.ui_scope = UiDirtyScope::Full,
                    "registerBrushMesh" => self.emit_mut()?.ui_scope = UiDirtyScope::None,
                    _ => return Err(Fault::from("puzzle5d-precompute-tool-authority")),
                }
                self.stage = Puzzle5dPrecomputeCommandStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(self.emit.take().ok_or_else(|| Fault::from("puzzle5d-precompute-publish-owner"))?))
            }
            Puzzle5dPrecomputeCommandStage::Complete => Err(Fault::from("puzzle5d-precompute-complete-repolled")),
            Puzzle5dPrecomputeCommandStage::Closing => Err(Fault::from("puzzle5d-precompute-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dPrecomputeCommandStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.emit.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dPrecomputeCommandStage::Closing && self.emit.is_none()
    }
}

struct Puzzle5dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Puzzle5dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: PUZZLE5D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Puzzle5dRetainedCommandJobFactory {
    type Payload = crate::retained_command::RetainedPuzzleCommandPayload<EditorApp<Puzzle5dPlayApp>>;
    type Job = crate::retained_command::RetainedPuzzleCommandJob<EditorApp<Puzzle5dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PUZZLE5D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        crate::retained_command::puzzle_command_contract()
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
        if input.declared_bytes() > crate::retained_command::PUZZLE_COMMAND_RAW_BYTES {
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

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Puzzle5dRetainedCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<Puzzle5dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PUZZLE5D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE5D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "duplicateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "importComposeKit", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "selectSameKindSelection", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setFixtureJson", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "zoomToSelection", lanes: &[ArtifactToolPublicationLane::Config] },
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
        use protocol::{Mutation as _, MutationDiff as _};
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
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
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
        use protocol::{Mutation as _, MutationDiff as _};
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

impl ArtifactEditor for Puzzle5dPlayApp {
    const DIALECT: semio_framework_plugin::app::Dialect = crate::artifacts::puzzle5d::PUZZLE5D_DIALECT;
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

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Puzzle5dStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Puzzle5dConfigStorePreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<Puzzle5dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "s.puzzle.puzzle5d@1/*#editor",
        document_schema: "puzzle.5d",
        factory: "Puzzle5dRetainedCommandJobFactory",
        factory_type: Puzzle5dRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(8_192, 512, 1, 262_144, 7_500, 1, 1),
        tools: [
            "canvasPointerDown",
            "worldPointerDown",
            "deleteSelection",
            "duplicateSelection",
            "importComposeKit",
            "selectSameKindSelection",
            "setFixtureJson",
            "setSelectionFlag",
            "zoomToSelection"
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(Puzzle5dCopyJobFactory::new(&controller_id))?;
        registry.register(Puzzle5dCutJobFactory::new(&controller_id))?;
        registry.register(Puzzle5dPasteJobFactory::new(&controller_id))?;
        registry.register(Puzzle5dImportJobFactory::new(&controller_id))?;
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
        let work: Box<dyn crate::retained_command::PuzzleCommandWork<EditorApp<Self>>> = match tool_id {
            "addBrushPart" | "addBrushObject" | "addPartKind" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id)),
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
            "setActiveExample" => Box::new(Puzzle5dSetActiveExampleWork::default()),
            "worldRelocate" => Box::new(Puzzle5dWorldRelocateWork::default()),
            "engagementAbort" => Box::new(Puzzle5dEngagementAbortWork::default()),
            "engagementSubmit" => Box::new(Puzzle5dEngagementSubmitWork::default()),
            "setObjectKindWeight" | "setVortexKindWeight" => Box::new(Puzzle5dKindWeightWork::new(tool_id)),
            "cycleBrushCandidate" | "registerBrushMesh" | "setFillCount" => Box::new(Puzzle5dPrecomputeCommandWork::new(tool_id)),
            "setCamera"
            | "setCamera2d"
            | "setCamera3d"
            | "setGridFactor"
            | "setGridSnapEnabled"
            | "setLodMode"
            | "setSuggestionOffset"
            | "setBrushPlacementOverlapBudget"
            | "engagementControlSelect"
            | "engagementInput"
            | "toggleSun"
            | "setSunAzimuth"
            | "setSunElevation"
            | "setSunIntensity" => Box::new(Puzzle5dScalarConfigWork::new(tool_id)),
            "worldPointerDown" | "canvasPointerDown" | "selectSameKind" => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(tool_id)),
            _ => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent)),
        };
        let payload = crate::retained_command::RetainedPuzzleCommandPayload {
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            interaction_state: request.interaction_state,
            interaction_hover: request.interaction_hover,
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
                ArtifactReservedToolJob::new(Puzzle5dCopyJob { work: Puzzle5dClipboardWork::new(request, interaction), completed: false })
            }
            "cut" => {
                let interaction = match &request.input {
                    ArtifactReservedToolInput::Action { interaction, .. } => interaction.clone(),
                    _ => return Err(Fault::from("puzzle5d cut requires action input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dCutJob { work: Puzzle5dClipboardWork::new(request, interaction), completed: false })
            }
            "paste" => {
                let args = match &request.input {
                    ArtifactReservedToolInput::Action { args, .. } => args.clone(),
                    _ => return Err(Fault::from("puzzle5d paste requires action input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dPasteJob::new(request, args))
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
    fn app_schema() -> Option<artifact_schema::AppSchemaDescriptor> {
        Some(crate::editor::puzzle5d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Puzzle5dPlaySnapshot {
        LazyLock::force(&NAKAGIN_EXAMPLE_DOCUMENT);
        LazyLock::force(&CAPSULE_DREAM_EXAMPLE_DOCUMENT);
        LazyLock::force(&PUZZLE5D_EXAMPLE_OPERATIONS);
        Puzzle5dPlaySnapshot(serde_json::to_value(default_document()).unwrap_or(Value::Null))
    }

    fn clipboard_media_type() -> Option<MediaType> {
        Some(MediaType { class: MediaClass::Kit, form: MediaForm::Design })
    }

    fn copy_fragment(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        let document: Puzzle5dDocument = serde_json::from_value(doc.snapshot.0.clone()).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let (part_ids, fastener_ids) = puzzle5d_interaction_part_and_fastener_ids(interaction);
        let (parts, fasteners) = copy_selection_local(&document, &part_ids, &fastener_ids);
        if parts.is_empty() {
            return Err(ClipboardError::EmptySelection);
        }
        let fragment_value = json!({ "schema": PUZZLE5D_SCHEMA, "parts": parts, "fasteners": fasteners });
        Ok(ClipboardFragment {
            schema: PUZZLE5D_SCHEMA.to_string(),
            media_type: Self::clipboard_media_type().expect("declared above"),
            dsl_text: serde_json::to_string_pretty(&fragment_value).unwrap_or_default(),
            pack_bytes: None,
            source_app: PUZZLE5D_PLAY_APP_ID.to_string(),
            label: format!("{} part(s)", parts.len()),
        })
    }

    /// @emoji ✂️ B1: `ArtifactApp::cut_operations`'s signature carries no config output channel (it
    /// returns a bare `Vec<Self::Mutation>`, not an `Emit`), so this can only emit the document
    /// removal; clearing the selection is left to the framework's own post-cut selection reconciliation
    /// (the cut parts/fasteners are gone from the document either way, so a stale selection referencing
    /// them is inert until the next real selection action overwrites it).
    fn cut_operations(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>, interaction: &InteractionView<'_>) -> Vec<Puzzle5dMutation> {
        let before = doc.snapshot.0.clone();
        let Ok(document) = serde_json::from_value::<Puzzle5dDocument>(before.clone()) else {
            return Vec::new();
        };
        let (part_ids, fastener_ids) = puzzle5d_interaction_part_and_fastener_ids(interaction);
        let (parts, fasteners) = copy_selection_local(&document, &part_ids, &fastener_ids);
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

    /// @emoji 📋️ B1: `ArtifactApp::paste_operations` carries no `ConfigView` at all (only `doc`/
    /// `fragment`/`placement`), so the new selection can't be threaded through this call; a following
    /// `setSelection` command (which the host already issues after a paste in practice) is what
    /// actually selects the pasted parts now.
    fn paste_operations(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle5dMutation>, ClipboardError> {
        let expected = Self::clipboard_media_type().unwrap_or(MediaType { class: MediaClass::Kit, form: MediaForm::Design });
        if fragment.media_type != expected {
            return Err(ClipboardError::IncompatibleMediaType(fragment.media_type));
        }
        let fragment_value: Value = serde_json::from_str(&fragment.dsl_text).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_parts: Vec<Puzzle5dPart> = serde_json::from_value(fragment_value.get("parts").cloned().unwrap_or_else(|| json!([]))).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_fasteners: Vec<Puzzle5dFastener> = serde_json::from_value(fragment_value.get("fasteners").cloned().unwrap_or_else(|| json!([]))).unwrap_or_default();
        let before = doc.snapshot.0.clone();
        let document: Puzzle5dDocument = serde_json::from_value(before.clone()).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
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

    fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let window_id = args.and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(Value::as_str).map(str::to_string);
        Puzzle5dCommand::try_from_action(action, args.cloned(), window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 5D action '{action}'")))
    }

    /// @emoji 🧩️ Thin typed-command adapter — reconstructs the exact `(action, args, window_id)`
    /// triple `handle_action_impl` expects from the typed `Puzzle5dCommand`.
    fn handle(
        command: &Puzzle5dCommand,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Puzzle5dMutation, Puzzle5dConfigMutation, Self::DraftMutation>, Fault> {
        let selection = interaction.selection(PUZZLE5D_INTERACTION_DOMAIN);
        with_puzzle5d_app(|app| Ok(app.handle_action_impl(command.action_id(), command.args(), command.window_id(), doc.snapshot, &cfg.snapshot, &selection)))
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
        let io = semio_framework::io::resolve_ready(AppIo::from_document(
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

    fn render(body_key: &str, doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let node = with_puzzle5d_app(|app| -> semio_framework_plugin::UiAssemblyResult<_> {
            let config = cfg.snapshot;
            let window_for_body = if body_key == board2d::BODY_KEY { board2d::WINDOW_KIND_ID } else { world3d::WINDOW_KIND_ID };
            let active_utility = puzzle5d_scene_active_utility(config, Some(window_for_body));
            let envelope = scene_from_projection(&doc.snapshot.0, config.clone(), &active_utility);
            let labels = puzzle5d_labels(config).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.localization.unsupported", "puzzle5d locale or terminology is not recognized"))?;
            match body_key {
                board2d::BODY_KEY => board2d::render(&envelope),
                world3d::BODY_KEY => world3d::render(&envelope, &app.precompute.borrow(), labels),
                document_panel::BODY_KEY => document_panel::render(&envelope, labels),
                catalogue::BODY_KEY => catalogue::render(&envelope, labels),
                inspection::BODY_KEY => inspection::render(&envelope, labels),
                _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d unknown-body label admission failed")),
            }
        })?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    fn window_engagements(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> HashMap<String, WindowEngagement> {
        let config = cfg.snapshot;
        let Some(labels) = puzzle5d_labels(config) else {
            return HashMap::new();
        };
        // 🪟️ One entry per live window INSTANCE of each of the 2D/3D window kinds — see
        // `window_instance_ids`'s doc comment for why puzzle5d needs none of puzzle3d's genuine
        // multi-instance-per-kind machinery here (each kind is always its own sole instance).
        PUZZLE5D_PLAY_WINDOWS
            .iter()
            .flat_map(|window| {
                window_instance_ids(window).into_iter().map(|wid| {
                    let active_utility = puzzle5d_scene_active_utility(config, Some(&wid));
                    let envelope = scene_from_projection(&doc.snapshot.0, config.clone(), &active_utility);
                    (wid, edit::puzzle5d_engagement(&envelope, window, labels))
                })
            })
            .collect()
    }

    fn window_measures(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        with_puzzle5d_app(|app| {
            let config = cfg.snapshot;
            let Some(labels) = puzzle5d_labels(config) else {
                return HashMap::new();
            };
            PUZZLE5D_PLAY_WINDOWS
                .iter()
                .flat_map(|window| {
                    window_instance_ids(window).into_iter().map(|wid| {
                        let active_utility = puzzle5d_scene_active_utility(config, Some(&wid));
                        let envelope = scene_from_projection(&doc.snapshot.0, config.clone(), &active_utility);
                        let measures = if *window == board2d::WINDOW_KIND_ID { board2d::window_measures(&envelope, &app.precompute.borrow(), labels) } else { world3d::window_measures(&envelope, &app.precompute.borrow(), labels) };
                        (wid, measures)
                    })
                })
                .collect()
        })
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let config = cfg.snapshot;
        let Some(labels) = puzzle5d_labels(config) else {
            return Vec::new();
        };
        let Some(is_de) = puzzle5d_is_de_locale(config) else {
            return Vec::new();
        };
        let active_utility = puzzle5d_scene_active_utility(config, Some(world3d::WINDOW_KIND_ID));
        let envelope = scene_from_projection(&doc.snapshot.0, config.clone(), &active_utility);
        let part_ids: Vec<String> =
            request.surface.as_ref().map(|surface| surface.selection.iter().filter(|g| g.domain == "object" || g.domain == "node" || g.domain == PUZZLE5D_GRANULARITY_PART).flat_map(|g| g.ids.iter().cloned()).collect()).unwrap_or_default();
        puzzle5d_context_menu_items(&envelope, &part_ids, labels, is_de, registry)
    }
}
//#endregion 🔖️PlayApp

//#region 🔖️Manifest
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

/// 🚧️ SDK note (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4): `EditorBuilder`
/// has no `.example(...)`/`.workflow(...)` methods — `.editor::<E>(def: AppDefinition)` only takes the
/// bare definition, `App.examples` is discarded by the plugin-root builder. The three examples this
/// app used to register (`PUZZLE5D_EXAMPLE_CONCRETE_FOREST`/`_NAKAGIN`/`_CAPSULE_DREAM`) and the
/// `"puzzle5d"` workflow tag are DROPPED here, not silently ported — the constants/JSON statics stay
/// live for `setActiveExample`'s own dispatch path (`🎮️commands/🛍️set-active-example`), only the
/// manifest-level registration is gone.
pub fn create_puzzle5d_app() -> semio_framework_plugin::AppDefinition {
    let envelope = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: PUZZLE5D_DEFAULT_UTILITY.into() };
    let precompute = Puzzle5dPrecomputeSession::new();
    let manifest_labels = puzzle5d_labels(&Puzzle5dConfig::default()).expect("default puzzle5d axes are explicit and recognized");
    Editor::builder(Puzzle5dPlayApp::DIALECT)
            .document(["semio", "puzzle", "5d"])
            .artifact_kind(crate::artifacts::puzzle5d::artifact_kind())
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "puzzle", "5d"])
            .mode_def(edit::definition())
            .default_mode_id(edit::PUZZLE5D_PLAY_MODE_EDIT)
            .window_kind_def(board2d::definition(&envelope, &precompute, manifest_labels))
            .window_kind_def(world3d::definition(&envelope, &precompute, manifest_labels))
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
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            // 🔧️ Document-mutating operations (emit VCS operations through the before/after document delta).
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Mutation) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importComposeKit", LocalizedLabel::native("Import Compose Kit", "Compose-Baukasten importieren"), ActionKind::Mutation) })
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .mutation("addPartKind", LocalizedLabel::native("Add Part", "Teil hinzufügen"))
            .mutation("addBrushPart", LocalizedLabel::native("Add Brush Part", "Pinselteil hinzufügen"))
            .mutation("addBrushObject", LocalizedLabel::native("Add Brush Object", "Pinselobjekt hinzufügen"))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection")))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).with_category("create")))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).with_category("settings")))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("zoomToSelection", LocalizedLabel::native("Zoom To Selection", "Auf Auswahl zoomen"), ActionKind::Mutation).with_category("view")))
            .mutation("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"))
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .mutation("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"))
            .mutation("patchPart", LocalizedLabel::native("Patch Part", "Teil aktualisieren"))
            .mutation("patchGrip", LocalizedLabel::native("Patch Grip", "Griff aktualisieren"))
            .mutation("patchFastener", LocalizedLabel::native("Patch Fastener", "Verbinder aktualisieren"))
            .mutation("createFastener", LocalizedLabel::native("Create Fastener", "Verbinder erstellen"))
            .mutation("deleteFastener", LocalizedLabel::native("Delete Fastener", "Verbinder löschen"))
            .mutation("retargetFastener", LocalizedLabel::native("Retarget Fastener", "Verbinder umhängen"))
            .mutation("editFastener", LocalizedLabel::native("Edit Fastener", "Verbinder bearbeiten"))
            .mutation("proximityConnect", LocalizedLabel::native("Proximity Connect", "Näherungsverbinden"))
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .mutation("worldRelocate", LocalizedLabel::native("Relocate Part", "Teil verlagern"))
            .mutation("applyBoardEvents", LocalizedLabel::native("Apply Board Events", "Board-Ereignisse anwenden"))
            // 👁️ Ephemeral view state — selection, hover, utility parameters, brush cycling, camera pose.
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("setCamera2d", LocalizedLabel::native("Set Camera 2D", "Kamera 2D festlegen"))
            .view_action("setCamera3d", LocalizedLabel::native("Set Camera 3D", "Kamera 3D festlegen"))
            .action_with(semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog("selectSameKindSelection", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).with_category("selection")))
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
            .view_action("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"))
            .view_action("setSuggestionOffset", LocalizedLabel::native("Set Suggestion Offset", "Vorschlagsversatz festlegen"))
            .view_action("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"))
            .view_action("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"))
            .view_action("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"))
            .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"))
            .action_interactive_job("addBrushObject", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addBrushPart", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addNode", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addPartKind", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("applyBoardEvents", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("createFastener", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("cycleBrushCandidate", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("deleteFastener", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("editFastener", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("engagementAbort", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("engagementControlSelect", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("engagementInput", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("focusSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("importComposeKit", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchFastener", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("patchGrip", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("patchPart", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("proximityConnect", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("registerBrushMesh", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("retargetFastener", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("selectSameKind", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("selectSameKindSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setBrushPlacementOverlapBudget", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setCamera", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setCamera2d", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setCamera3d", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setFillCount", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setFixtureJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridFactor", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setGridSnapEnabled", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setSelectionFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setObjectKindWeight", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setSuggestionOffset", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setVortexKindWeight", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("toggleSun", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("translateSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("worldPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldRelocate", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("zoomToSelection", InteractiveJobClassification::Migrated)
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
            .utility(world3d::utilities::world_relocate::definition())
    .build_definition()
}

// 🗂️ `Puzzle5dPlaySnapshot`'s pack<->dsl codec (so `framework/sync`'s `FolderEndpoint::Pack` can
// print/parse puzzle-5d play documents without depending on this crate's concrete
// `Projection`/`Mutation` types) is now declared via `.document_codec::<Puzzle5dPlayApp>()` on
// `crate::artifacts::puzzle5d::declaration()` (ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`
// M1) — the old side-effecting `register_puzzle5d_exports()` wrapper (this app file's only caller of
// `register_document_codec_for_app`) is gone. The 5d mesh export/import OS-host registration
// (`register_mesh_io()`/`puzzle5d_document_from_mesh`) was never rewired to a real `.setup()` caller
// after the artifacts-only-plugin-architecture migration (`🧩️puzzle/🦀️component.rs`'s `plugin()`
// builder chain has no `.setup()` call at all) and was deleted as dead code (ticket
// 26/08/17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS) — same fate as puzzle3d's
// sibling mesh bridge. Mesh export/import should be re-derived from `io_dispatch`'s real
// `ComposerEntry` chain if/when this bridge is needed again.
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ The one puzzle5d-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
/// instead of re-deriving a store/dispatch/render scaffold of its own.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::{testkit, ActionMeta, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    /// ✏️ `Puzzle5dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<Puzzle5dPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<Puzzle5dPlayApp>` builds it.
    pub type Puzzle5dApp = VcsArtifactApp<EditorApp<Puzzle5dPlayApp>>;

    pub fn meta(actor: &str) -> ActionMeta {
        testkit::meta(actor)
    }

    pub fn app() -> Puzzle5dApp {
        semio_framework::io::resolve_ready(testkit::new_app::<EditorApp<Puzzle5dPlayApp>>())
    }

    /// ✏️ Adapts `create_puzzle5d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `testkit::new_app_with_registry` still expects — framework testkit gap, not
    /// modifiable here (`🧰️framework/**` is outside this packet's lease).
    pub fn puzzle5d_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_puzzle5d_app(), examples: Vec::new() }
    }

    /// 🧰️ A registry-backed app so kind discipline (View actions must emit no operations) and the
    /// utility contract are enforced exactly as in production.
    pub fn app_with_registry() -> Puzzle5dApp {
        semio_framework::io::resolve_ready(testkit::new_app_with_registry::<EditorApp<Puzzle5dPlayApp>>(puzzle5d_app_manifest_for_testkit))
    }

    /// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
    /// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
    /// `Self::Command` channel). Reconstructs the `Puzzle5dCommand` from the same
    /// `(action, args, window_id)` triple every pre-migration test already passed.
    pub fn dispatch(app: &mut Puzzle5dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
        // 🕰️ Framework-reserved verbs (undo/redo/checkpoint/…/the six interaction verbs) stay on
        // `handle_action` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM added
        // interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
        // setInteractionGranularity to this reserved set.
        if matches!(
            action,
            "undo"
                | "redo"
                | "checkpoint"
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
            return semio_framework::io::resolve_ready(app.handle_action(action, args, &meta("local")));
        }
        semio_framework::io::resolve_ready(app.dispatch_typed(Puzzle5dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), &meta("local")))
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionSelect`
    /// for one `(granularity, id)` pair in the `vortex` domain — the test-side replacement for the
    /// deleted `setSelection` action.
    pub fn select_id(app: &mut Puzzle5dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
        let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
        dispatch(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE5D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None)
    }

    /// 🖼️ The rendered body, as a JSON string — every panel/window assertion greps this value.
    pub fn render_body(app: &mut Puzzle5dApp, body_key: &str) -> String {
        let tree = semio_framework::io::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render");
        let mut stack = vec![&tree.root];
        while let Some(node) = stack.pop() {
            if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
                let scene = match surface.doc_schema.as_str() {
                    schema if schema == <semio_framework_ui_scene::Board2dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA => {
                        serde_json::to_value(semio_framework_ui_scene::decode::<semio_framework_ui_scene::Board2dScene>(surface).expect("decode board scene"))
                    }
                    schema if schema == <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA => {
                        serde_json::to_value(semio_framework_ui_scene::decode::<semio_framework_ui_scene::World3dScene>(surface).expect("decode world scene"))
                    }
                    _ => continue,
                }
                .expect("serialize scene");
                return json!({ "schema": surface.doc_schema, "scene": scene }).to_string();
            }
            stack.extend(node.children.iter());
        }
        serde_json::to_string(&tree.root).expect("serialize rendered node")
    }

    pub fn projection_of(app: &Puzzle5dApp) -> Value {
        app.snapshot().expect("projection").0
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
    use protocol::MutationDiff;
    use semio_framework_plugin::{ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, PluginApp, UiMenuRef};

    #[test]
    fn retained_publication_contracts_are_an_exact_nonempty_tool_bijection() {
        let exact = |contracts: &[ArtifactToolPublicationContract]| {
            let ids = contracts.iter().map(|contract| contract.tool_id).collect::<std::collections::BTreeSet<_>>();
            ids == PUZZLE5D_RETAINED_TOOL_IDS.iter().copied().collect()
                && ids.len() == contracts.len()
                && contracts.iter().all(|contract| !contract.lanes.is_empty() && (!contract.lanes.contains(&ArtifactToolPublicationLane::HostOnly) || contract.lanes.len() == 1))
        };
        let contracts = <Puzzle5dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
        assert!(exact(contracts));
        assert!(!exact(&contracts[..contracts.len() - 1]));
        let mut duplicate = contracts.to_vec();
        let copied = duplicate[1];
        duplicate[0] = copied;
        assert!(!exact(&duplicate));
        let reserved = [
            <Puzzle5dCopyJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS[0],
            <Puzzle5dCutJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS[0],
            <Puzzle5dPasteJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS[0],
            <Puzzle5dImportJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS[0],
        ];
        assert_eq!(reserved.iter().map(|contract| contract.tool_id).collect::<Vec<_>>(), vec!["copy", "cut", "paste", "import-media"]);
        assert_eq!(reserved[0].lanes, &[ArtifactToolPublicationLane::HostOnly]);
        assert!(reserved[1..].iter().all(|contract| contract.lanes == &[ArtifactToolPublicationLane::Artifact]));
    }

    #[test]
    fn retained_import_media_has_no_live_synchronous_fallback() {
        let source = include_str!("🦀️component.rs");
        let production = source.split_once("//#region 🧪️Testkit").map(|(production, _)| production).expect("production prefix");
        let fallback = production.split_once("fn import_media(_port:").and_then(|(_, suffix)| suffix.split_once("fn render(").map(|(fallback, _)| fallback)).expect("closed synchronous import callback");
        assert!(fallback.contains("Err(MediaError::NotImplemented)"));
        assert!(!fallback.contains("serde_json::from_str"));
        assert!(!fallback.contains("artifact_mutations"));
        let hostile = fallback.replace("Err(MediaError::NotImplemented)", "serde_json::from_str(\"{}\").map(|_| Emit::default()).map_err(|_| MediaError::NotImplemented)");
        assert!(hostile.contains("serde_json::from_str"));
    }

    fn precompute_routes_are_cursorized(source: &str) -> bool {
        [
            r#""cycleBrushCandidate" | "registerBrushMesh" | "setFillCount" => Box::new(Puzzle5dPrecomputeCommandWork::new(tool_id))"#,
            "Puzzle5dPrecomputeCommandStage::Parts",
            "Puzzle5dPrecomputeCommandStage::Grips",
            "Puzzle5dPrecomputeCommandStage::Fasteners",
            "Puzzle5dPrecomputeCommandStage::CatalogParts",
            "Puzzle5dPrecomputeCommandStage::CatalogGrips",
            "Puzzle5dPrecomputeCommandStage::Positions",
            "Puzzle5dPrecomputeCommandStage::Indices",
            "Puzzle5dPrecomputeCommandStage::FillCount",
            "Puzzle5dPrecomputeCommandStage::BoardUtility",
            "Puzzle5dPrecomputeCommandStage::WorldUtility",
            "Puzzle5dPrecomputeCommandStage::Publish",
        ]
        .into_iter()
        .all(|marker| source.contains(marker))
            && !source.contains(r#""cycleBrushCandidate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
            && !source.contains(r#""registerBrushMesh" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
            && !source.contains(r#""setFillCount" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
    }

    #[test]
    fn precompute_hostile_static_law_rejects_one_grant_reducers_and_missing_boundaries() {
        let source = include_str!("🦀️component.rs");
        assert!(precompute_routes_are_cursorized(source));
        let direct = source.replace(
            r#""cycleBrushCandidate" | "registerBrushMesh" | "setFillCount" => Box::new(Puzzle5dPrecomputeCommandWork::new(tool_id))"#,
            r#""cycleBrushCandidate" | "registerBrushMesh" | "setFillCount" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
        );
        assert!(!precompute_routes_are_cursorized(&direct));
        for marker in [
            "Puzzle5dPrecomputeCommandStage::Parts",
            "Puzzle5dPrecomputeCommandStage::Grips",
            "Puzzle5dPrecomputeCommandStage::Fasteners",
            "Puzzle5dPrecomputeCommandStage::CatalogParts",
            "Puzzle5dPrecomputeCommandStage::CatalogGrips",
            "Puzzle5dPrecomputeCommandStage::Positions",
            "Puzzle5dPrecomputeCommandStage::Indices",
            "Puzzle5dPrecomputeCommandStage::FillCount",
            "Puzzle5dPrecomputeCommandStage::BoardUtility",
            "Puzzle5dPrecomputeCommandStage::WorldUtility",
            "Puzzle5dPrecomputeCommandStage::Publish",
        ] {
            assert!(!precompute_routes_are_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing retained boundary was falsely accepted: {marker}");
        }
    }

    fn complex_retained_route_is_cursorized(source: &str) -> bool {
        source.contains("\"applyBoardEvents\" => Box::new(Puzzle5dBoardEventsWork::default())")
            && source.contains("struct Puzzle5dBoardEventsWork")
            && source.contains("self.scan_one(source)?")
            && source.contains("Puzzle5dBoardEventsStage::FindMovePart")
            && source.contains("Puzzle5dBoardEventsStage::ScanEdge")
            && source.contains("Puzzle5dBoardEventsStage::ScanDeleteEdges")
            && source.contains("Puzzle5dBoardEventsStage::Brush")
            && source.contains("Puzzle5dBoardEventsStage::CloseBrush")
            && !source.contains("\"applyBoardEvents\" => Box::new(crate::retained_command::BoundedFirstStepCommandWork")
    }

    #[test]
    fn apply_board_events_hostile_static_law_rejects_the_old_one_grant_reducer() {
        let source = include_str!("🦀️component.rs");
        assert!(complex_retained_route_is_cursorized(source));
        let direct = source
            .replace("\"applyBoardEvents\" => Box::new(Puzzle5dBoardEventsWork::default())", "\"applyBoardEvents\" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))");
        assert!(!complex_retained_route_is_cursorized(&direct), "hostile old-reducer replacement must fail closed");
        for marker in ["self.scan_one(source)?", "Puzzle5dBoardEventsStage::FindMovePart", "Puzzle5dBoardEventsStage::ScanEdge", "Puzzle5dBoardEventsStage::ScanDeleteEdges", "Puzzle5dBoardEventsStage::CloseBrush"] {
            assert!(!complex_retained_route_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing cursor marker was falsely accepted: {marker}");
        }
    }

    fn focus_selection_route_is_cursorized(source: &str) -> bool {
        source.contains(r#""focusSelection" => Box::new(Puzzle5dFocusSelectionWork::default())"#)
            && source.contains("Puzzle5dFocusSelectionStage::Selection")
            && source.contains("Puzzle5dFocusSelectionStage::Parts")
            && source.contains("Puzzle5dFocusSelectionStage::Publish")
            && source.contains("self.part_cursor += 1")
            && !source.contains(r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
    }

    #[test]
    fn focus_selection_hostile_static_law_rejects_whole_selection_reducers() {
        let source = include_str!("🦀️component.rs");
        assert!(focus_selection_route_is_cursorized(source));
        let direct = source
            .replace(r#""focusSelection" => Box::new(Puzzle5dFocusSelectionWork::default())"#, r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#);
        assert!(!focus_selection_route_is_cursorized(&direct));
        for marker in ["Puzzle5dFocusSelectionStage::Selection", "Puzzle5dFocusSelectionStage::Parts", "Puzzle5dFocusSelectionStage::Publish", "self.part_cursor += 1"] {
            assert!(!focus_selection_route_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing focus cursor marker was falsely accepted: {marker}");
        }
    }

    fn scalar_config_routes_are_direct(source: &str) -> bool {
        source.contains("struct Puzzle5dScalarConfigWork")
            && source.contains(
                r#""setCamera"
            | "setCamera2d"
            | "setCamera3d""#,
            )
            && source.contains(r#"| "setSunIntensity" => Box::new(Puzzle5dScalarConfigWork::new(tool_id))"#)
            && source.contains(
                r#"| "engagementInput"
            | "toggleSun""#,
            )
            && source.contains("Puzzle5dConfigMutation::SetCamera2d")
            && source.contains("Puzzle5dConfigMutation::SetBrushCandidateIndex")
            && source.contains("Puzzle5dConfigMutation::SetEngagementInput")
            && source.contains("Puzzle5dConfigMutation::SetGridFactor")
            && source.contains("Puzzle5dConfigMutation::SetOverlapBudget")
            && source.contains("Puzzle5dConfigMutation::SetSun")
            && !source.contains(r#""setCamera" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
            && !source.contains(r#""setSunIntensity" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
    }

    #[test]
    fn scalar_config_hostile_static_law_rejects_old_reducer_and_missing_exact_mutations() {
        let source = include_str!("🦀️component.rs");
        assert!(scalar_config_routes_are_direct(source));
        for marker in [
            "struct Puzzle5dScalarConfigWork",
            "Puzzle5dConfigMutation::SetCamera2d",
            "Puzzle5dConfigMutation::SetBrushCandidateIndex",
            "Puzzle5dConfigMutation::SetEngagementInput",
            "Puzzle5dConfigMutation::SetGridFactor",
            "Puzzle5dConfigMutation::SetOverlapBudget",
            "Puzzle5dConfigMutation::SetSun",
        ] {
            assert!(!scalar_config_routes_are_direct(&source.replacen(marker, "route-removed", 1)), "missing scalar route marker was falsely accepted: {marker}");
        }
        let direct = source.replace(
            r#"| "setSunIntensity" => Box::new(Puzzle5dScalarConfigWork::new(tool_id))"#,
            r#"| "setSunIntensity" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
        );
        assert!(!scalar_config_routes_are_direct(&direct), "hostile scalar old-reducer replacement must fail closed");
    }

    fn engagement_abort_route_is_cursorized(source: &str) -> bool {
        source.contains(r#""engagementAbort" => Box::new(Puzzle5dEngagementAbortWork::default())"#)
            && source.contains("Puzzle5dEngagementAbortStage::Input")
            && source.contains("Puzzle5dEngagementAbortStage::BoardUtility")
            && source.contains("Puzzle5dEngagementAbortStage::WorldUtility")
            && source.contains("Puzzle5dEngagementAbortStage::Publish")
            && source.contains("self.effects[0].take()")
            && source.contains("self.effects[1].take()")
            && !source.contains(r#""engagementAbort" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
    }

    #[test]
    fn engagement_abort_hostile_static_law_rejects_old_reducer_and_missing_transfer_boundaries() {
        let source = include_str!("🦀️component.rs");
        assert!(engagement_abort_route_is_cursorized(source));
        for marker in ["Puzzle5dEngagementAbortStage::Input", "Puzzle5dEngagementAbortStage::BoardUtility", "Puzzle5dEngagementAbortStage::WorldUtility", "Puzzle5dEngagementAbortStage::Publish", "self.effects[0].take()", "self.effects[1].take()"] {
            assert!(!engagement_abort_route_is_cursorized(&source.replacen(marker, "route-removed", 1)), "missing engagement abort marker was falsely accepted: {marker}");
        }
        let direct = source.replace(
            r#""engagementAbort" => Box::new(Puzzle5dEngagementAbortWork::default())"#,
            r#""engagementAbort" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
        );
        assert!(!engagement_abort_route_is_cursorized(&direct));
    }

    fn add_part_kind_route_is_cursorized(source: &str) -> bool {
        source.contains(r#""addBrushPart" | "addBrushObject" | "addPartKind" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id))"#)
            && source.contains("Puzzle5dAddBrushPartStage::Catalog")
            && source.contains("Puzzle5dAddBrushPartStage::Grips")
            && source.contains("Puzzle5dAddBrushPartStage::Target")
            && source.contains("Puzzle5dAddBrushPartStage::Create")
            && source.contains("Puzzle5dAddBrushPartStage::Connect")
            && !source.contains(r#""addPartKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
    }

    #[test]
    fn add_part_kind_hostile_static_law_rejects_old_brush_reducer_and_missing_cursors() {
        let source = include_str!("🦀️component.rs");
        assert!(add_part_kind_route_is_cursorized(source));
        let direct = source.replace(
            r#""addBrushPart" | "addBrushObject" | "addPartKind" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id))"#,
            r#""addBrushPart" | "addBrushObject" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id)),
            "addPartKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
        );
        assert!(!add_part_kind_route_is_cursorized(&direct));
    }

    fn kind_weight_route_is_cursorized(source: &str) -> bool {
        source.contains(r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(Puzzle5dKindWeightWork::new(tool_id))"#)
            && source.contains("Puzzle5dKindWeightStage::Catalog")
            && source.contains("Puzzle5dKindWeightStage::InferParts")
            && source.contains("Puzzle5dKindWeightStage::InferGrips")
            && source.contains("Puzzle5dKindWeightStage::Validate")
            && source.contains("Puzzle5dKindWeightStage::SumOthers")
            && source.contains("Puzzle5dKindWeightStage::Build")
            && source.contains("Puzzle5dConfigMutation::SetObjectKindWeights")
            && source.contains("Puzzle5dConfigMutation::SetVortexKindWeights")
            && !source.contains(r#""setObjectKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
    }

    #[test]
    fn kind_weight_hostile_static_law_rejects_whole_normalizer_and_missing_cursors() {
        let source = include_str!("🦀️component.rs");
        assert!(kind_weight_route_is_cursorized(source));
        let direct = source.replace(
            r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(Puzzle5dKindWeightWork::new(tool_id))"#,
            r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
        );
        assert!(!kind_weight_route_is_cursorized(&direct));
        assert!(!source.contains("puzzle5d_normalize_kind_weight_group(self.weights"));
    }

    fn engagement_submit_route_is_cursorized(source: &str) -> bool {
        source.contains(r#""engagementSubmit" => Box::new(Puzzle5dEngagementSubmitWork::default())"#)
            && source.contains("Puzzle5dEngagementSubmitStage::Parse")
            && source.contains("Puzzle5dEngagementSubmitStage::BoardConfig")
            && source.contains("Puzzle5dEngagementSubmitStage::WorldConfig")
            && source.contains("Puzzle5dEngagementSubmitStage::BoardEffect")
            && source.contains("Puzzle5dEngagementSubmitStage::WorldEffect")
            && source.contains("Puzzle5dEngagementSubmitStage::Input")
            && source.contains("Puzzle5dEngagementSubmitStage::Publish")
            && source.contains("Puzzle5dConfigMutation::SetActiveUtility")
            && source.contains("Puzzle5dConfigMutation::SetEngagementInput")
            && !source.contains(r#""engagementSubmit" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
    }

    #[test]
    fn engagement_submit_hostile_static_law_rejects_old_reducer_and_missing_transfers() {
        let source = include_str!("🦀️component.rs");
        assert!(engagement_submit_route_is_cursorized(source));
        for marker in [
            "Puzzle5dEngagementSubmitStage::Parse",
            "Puzzle5dEngagementSubmitStage::BoardConfig",
            "Puzzle5dEngagementSubmitStage::WorldConfig",
            "Puzzle5dEngagementSubmitStage::BoardEffect",
            "Puzzle5dEngagementSubmitStage::WorldEffect",
            "Puzzle5dEngagementSubmitStage::Input",
            "Puzzle5dEngagementSubmitStage::Publish",
        ] {
            assert!(!engagement_submit_route_is_cursorized(&source.replacen(marker, "route-removed", 1)), "missing engagement submit marker was falsely accepted: {marker}");
        }
        let direct = source.replace(
            r#""engagementSubmit" => Box::new(Puzzle5dEngagementSubmitWork::default())"#,
            r#""engagementSubmit" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
        );
        assert!(!engagement_submit_route_is_cursorized(&direct));
    }

    fn world_relocate_route_is_cursorized(source: &str) -> bool {
        source.contains(r#""worldRelocate" => Box::new(Puzzle5dWorldRelocateWork::default())"#)
            && source.contains("struct Puzzle5dWorldRelocateWork")
            && source.contains("Puzzle5dWorldRelocateStage::SourcePart")
            && source.contains("Puzzle5dWorldRelocateStage::ExistingFasteners")
            && source.contains("Puzzle5dWorldRelocateStage::CandidatePart")
            && source.contains("Puzzle5dWorldRelocateStage::CandidateGrip")
            && source.contains("Puzzle5dWorldRelocateStage::PublishFastener")
            && source.contains("PUZZLE5D_RELOCATE_GRIPS_PER_PART")
            && !source.contains(r#""worldRelocate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
    }

    #[test]
    fn world_relocate_hostile_static_law_rejects_whole_proximity_scans() {
        let source = include_str!("🦀️component.rs");
        assert!(world_relocate_route_is_cursorized(source));
        let direct = source
            .replace(r#""worldRelocate" => Box::new(Puzzle5dWorldRelocateWork::default())"#, r#""worldRelocate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#);
        assert!(!world_relocate_route_is_cursorized(&direct), "hostile old-reducer replacement must fail closed");
        for marker in ["Puzzle5dWorldRelocateStage::ExistingFasteners", "Puzzle5dWorldRelocateStage::CandidatePart", "Puzzle5dWorldRelocateStage::CandidateGrip", "Puzzle5dWorldRelocateStage::PublishFastener", "PUZZLE5D_RELOCATE_GRIPS_PER_PART"] {
            assert!(!world_relocate_route_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing world-relocate marker was falsely accepted: {marker}");
        }
    }

    //#region 🔖️Rendering
    #[semio_framework_async_macros::async_test]
    async fn renders_paired_board_and_world_scenes() {
        let mut app = app();
        assert!(render_body(&mut app, board2d::BODY_KEY).contains("board-2d"));
        assert!(render_body(&mut app, world3d::BODY_KEY).contains("world-3d"));
    }

    #[semio_framework_async_macros::async_test]
    async fn initial_snapshot_is_the_concrete_forest_document() {
        let app = app();
        assert_eq!(projection_of(&app).get("schema").and_then(|value| value.as_str()), Some(PUZZLE5D_SCHEMA));
        assert!(part_count(&app) > 0, "the concrete-forest default document ships with parts");
    }

    #[semio_framework_async_macros::async_test]
    async fn document_panel_renders() {
        let mut app = app();
        assert!(!render_body(&mut app, document_panel::BODY_KEY).is_empty());
    }
    //#endregion 🔖️Rendering

    //#region 🔖️ContextMenu
    /// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: the selection context menu stays a shallow,
    /// disclosed list (top-level verbs + a handful of taxonomy groups) rather than a flat wall of rows,
    /// and the known destructive `deleteSelection` action stays the trailing group's last item.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_is_grouped_and_keeps_delete_selection_last() {
        let mut app = app_with_registry();
        let part_id = first_part_id(&app);
        select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &part_id).expect("select part");
        let request = ContextMenuRequest {
            menu: UiMenuRef { id: "world3d".into(), args: None },
            surface: Some(ContextMenuSurfaceTarget { surface_id: world3d::WINDOW_KIND_ID.into(), kind: "world3d".into(), hits: vec![], selection: vec![ContextMenuSelectionGroup { domain: "part".into(), ids: vec![part_id] }], text: None }),
            window_instance_id: None,
            point: None,
        };
        let menu = semio_framework::io::resolve_ready(app.context_menu(&request));
        assert!(menu.len() <= 9, "top-level context menu should stay progressively disclosed: {menu:?}");
        let last = menu.last().expect("selection context menu should not be empty");
        let last_is_destructive_leaf = last.action.as_deref() == Some("deleteSelection") && last.destructive == Some(true);
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.action.as_deref() == Some("deleteSelection") && child.destructive == Some(true));
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must stay last: {menu:?}");
    }
    //#endregion 🔖️ContextMenu

    //#region 🔖️Pack
    /// 📦️ `Puzzle5dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
    /// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
    /// `serde_json::Value` bridge impls), reusing the default concrete-forest fixture.
    #[semio_framework_async_macros::async_test]
    async fn puzzle5d_play_projection_pack_round_trips() {
        let app = app();
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&app.snapshot().expect("projection"));
    }
    //#endregion 🔖️Pack

    //#region 🔖️Operations
    #[semio_framework_async_macros::async_test]
    async fn set_active_example_swaps_the_document_and_undo_restores_it() {
        let mut app = app();
        let loaded = part_count(&app);
        assert!(loaded > 0);
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).expect("empty");
        assert_eq!(part_count(&app), 0, "empty example clears the parts");
        semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
        assert_eq!(part_count(&app), loaded, "undo restores the concrete-forest parts");
        semio_framework::io::resolve_ready(app.handle_action("redo", None, &meta("local"))).expect("redo");
        assert_eq!(part_count(&app), 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_fastener_updates_transform_offsets_and_undoes() {
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
        semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
        let undone = projection_of(&app);
        let fastener3 = undone["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
        assert_eq!(fastener3["rotation"], 0.0, "undo restores the pre-rotation-edit value");
        assert_eq!(fastener3["gap"], 2.5, "undo of rotation edit must not also revert the earlier gap edit");
    }
    //#endregion 🔖️Operations

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Puzzle5dMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s. Deliberately
    /// dispatches through a standalone typed `Puzzle5dStore` — NOT through `Puzzle5dPlayApp`/
    /// `Puzzle5dPlaySnapshot` (the `🔖️ValueBridge` `serde_json::Value` wrapper this app's real
    /// `ArtifactApp` still uses) — since `Puzzle5dMutation`'s canonical `Mutation<Puzzle5dSnapshot>`
    /// impl (not its `Mutation<Value>` bridge impl) is what the CW7 law is about.
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::puzzle5d::spr::Puzzle5dStore;
        use crate::artifacts::puzzle5d::{Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, PUZZLE_5D_SCHEMA};
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, EngineHandles};

        let mut store = semio_framework::io::resolve_ready(Puzzle5dStore::new(create_document_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", Puzzle5dSnapshot::default(), None))).expect("store");
        let part = Puzzle5dPart { id: "p1".into(), part_kind: None, anchor: Default::default(), part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() };
        semio_framework::io::resolve_ready(store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::artifacts::puzzle5d::mutations::create_part(part, None)], description: None })).expect("apply");
        let envelope = store.envelope();
        let edit: &Edit<Puzzle5dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
        semio_framework::io::resolve_ready(semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle5dSnapshot, Puzzle5dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())));
    }
    //#endregion 🔖️CommandEnvelopeTests

    //#region 🔖️Clipboard
    #[semio_framework_async_macros::async_test]
    async fn copy_emits_clipboard_fragment_for_the_closed_selection() {
        let mut app = app_with_registry();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
        let first_part_id = first_part_id(&app);
        select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &first_part_id).expect("select");
        let result = semio_framework::io::resolve_ready(app.handle_action("copy", None, &meta("local"))).expect("copy");
        assert!(result.mutations.is_empty(), "copy must not record an undo entry");
        assert_eq!(result.requested_effects.len(), 1);
        let Effect::ClipboardWrite { fragment } = &result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
        assert_eq!(fragment.source_app, PUZZLE5D_PLAY_APP_ID);
        let fragment_value: Value = serde_json::from_str(&fragment.dsl_text).expect("fragment dsl_text is JSON");
        assert_eq!(fragment_value["parts"].as_array().expect("parts").len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn copy_with_no_selection_is_a_benign_no_operation() {
        let mut app = app();
        let result = semio_framework::io::resolve_ready(app.handle_action("copy", None, &meta("local"))).expect("copy");
        assert!(result.mutations.is_empty());
        assert!(result.requested_effects.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn cut_removes_selected_part_and_undo_restores_it() {
        let mut app = app_with_registry();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
        let before_count = part_count(&app);
        let first_part_id = first_part_id(&app);
        select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &first_part_id).expect("select");
        let result = semio_framework::io::resolve_ready(app.handle_action("cut", None, &meta("local"))).expect("cut");
        assert_eq!(result.requested_effects.len(), 1, "cut must also copy to the clipboard");
        assert_eq!(part_count(&app), before_count - 1);
        let after = projection_of(&app);
        assert!(!after["parts"].as_array().unwrap().iter().any(|part| part["id"] == first_part_id));
        semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
        assert_eq!(part_count(&app), before_count, "one undo restores the cut part as a single edit");
    }

    #[semio_framework_async_macros::async_test]
    async fn paste_materializes_fragment_parts_at_original_anchor_with_fresh_ids() {
        let mut app = app_with_registry();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
        let projection = projection_of(&app);
        let first_part_id = first_part_id(&app);
        select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &first_part_id).expect("select");
        let copy_result = semio_framework::io::resolve_ready(app.handle_action("copy", None, &meta("local"))).expect("copy");
        let Effect::ClipboardWrite { fragment } = &copy_result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
        let before_count = part_count(&app);
        let before_ids: HashSet<String> = projection["parts"].as_array().unwrap().iter().map(|part| part["id"].as_str().unwrap_or_default().to_string()).collect();
        let paste_args = json!({ "fragment": fragment, "anchor": "original", "position": [10.0, 0.0, 0.0] });
        semio_framework::io::resolve_ready(app.handle_action("paste", Some(&paste_args), &meta("local"))).expect("paste");
        assert_eq!(part_count(&app), before_count + 1);
        let after = projection_of(&app);
        let pasted_parts: Vec<&Value> = after["parts"].as_array().unwrap().iter().filter(|part| !before_ids.contains(part["id"].as_str().unwrap_or_default())).collect();
        assert_eq!(pasted_parts.len(), 1);
        // "original" anchor uses the raw position override verbatim as the 2D delta.
        let original_x = projection["parts"][0]["2d"]["x"].as_f64().unwrap_or(0.0);
        assert_eq!(pasted_parts[0]["2d"]["x"].as_f64().unwrap(), original_x + 10.0);
        semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
        assert_eq!(part_count(&app), before_count, "one undo removes the whole pasted fragment");
    }

    #[semio_framework_async_macros::async_test]
    async fn paste_with_no_fragment_arg_is_a_benign_no_operation() {
        let mut app = app();
        let before_count = part_count(&app);
        let result = semio_framework::io::resolve_ready(app.handle_action("paste", None, &meta("local"))).expect("paste");
        assert!(result.mutations.is_empty());
        assert_eq!(part_count(&app), before_count);
    }
    //#endregion 🔖️Clipboard

    //#region 🔖️Manifest
    #[semio_framework_async_macros::async_test]
    async fn app_definition_has_the_paired_windows() {
        let definition = create_puzzle5d_app();
        let ids: Vec<&str> = definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
        assert!(ids.contains(&board2d::WINDOW_KIND_ID) && ids.contains(&world3d::WINDOW_KIND_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn window_kind_actions_scope_transform_to_3d_only() {
        let definition = create_puzzle5d_app();
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
    #[semio_framework_async_macros::async_test]
    async fn app_definition_declares_its_three_panel_tabs() {
        let definition = create_puzzle5d_app();
        let body_keys: Vec<&str> = definition.panel_tabs.iter().filter_map(|tab| tab.body_key.as_deref()).collect();
        for body_key in [document_panel::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY] {
            assert!(body_keys.contains(&body_key), "panel tab {body_key} must be declared, got {body_keys:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn window_engagements_cover_both_windows() {
        let mut app = app();
        let engagements = semio_framework::io::resolve_ready(app.window_engagements());
        assert!(engagements.contains_key(board2d::WINDOW_KIND_ID));
        assert!(engagements.contains_key(world3d::WINDOW_KIND_ID));
    }

    /// 🎯️ Every action id `dispatch_puzzle5d_action` matches on must have a `Puzzle5dCommand` variant
    /// under the SAME literal — the two lists are the whole app's dispatch contract and drift between
    /// them is silent. (Deliberately NOT the framework's own
    /// `assert_declared_actions_bridge_to_commands`, which probes `command_from_action`, the
    /// string-dispatch path this app does not implement — its commands carry an opaque `args: Value`,
    /// see the `🔖️Puzzle5dCommand` macro.)
    #[semio_framework_async_macros::async_test]
    async fn every_dispatched_action_bridges_to_a_command() {
        for action in [
            "setFixtureJson",
            "setActiveExample",
            "importComposeKit",
            "selectSameKindSelection",
            "selectSameKind",
            "addNode",
            "addPartKind",
            "deleteSelection",
            "duplicateSelection",
            "setSelectionFlag",
            "patchPart",
            "patchGrip",
            "patchFastener",
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
    #[semio_framework_async_macros::async_test]
    async fn add_part_kind_materializes_the_declared_kind_default() {
        // 📝️ P1 arg form: addPartKind with no args materializes the declared `partKind` default and adds a part.
        let mut app = app_with_registry();
        dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).expect("empty");
        let before = part_count(&app);
        let result = dispatch(&mut app, "addPartKind", None, None).expect("addPartKind");
        assert!(!result.mutations.is_empty(), "addPartKind is a Mutation that emits mutations");
        assert_eq!(part_count(&app), before + 1, "the materialized default kind adds exactly one part");
        let projection = projection_of(&app);
        let kind = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.last()).and_then(|part| part.get("partKind")).and_then(Value::as_str);
        assert_eq!(kind, Some("Part"), "the declared partKind default was materialized host-side");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_utility_emits_no_ops_and_no_history_entry() {
        // 🧰️ Switching utilities is the framework View action: no document operations, no undo entry, no re-emitted effect.
        let mut app = app_with_registry();
        let before = projection_of(&app);
        let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "brush" })), None).expect("switch utility");
        assert!(result.mutations.is_empty(), "utility switching never emits document operations");
        assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
        assert_eq!(projection_of(&app), before, "utility switching does not mutate the document");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_camera_actions_write_runtime_and_emit_no_operations() {
        // 📷️ Camera pose is session-only view state (`ActionKind::View`): `setCamera2d`/`setCamera3d`
        // must mutate the app's runtime (visible via the rendered scene) without ever touching the
        // VCS-tracked document or emitting an operation.
        let mut app = app();
        let before = projection_of(&app);
        let camera2d_result = dispatch(&mut app, "setCamera2d", Some(&json!({ "camera": { "x": 12.5, "y": -6.5, "zoom": 3.5 } })), None).expect("setCamera2d");
        assert!(camera2d_result.mutations.is_empty(), "setCamera2d is a View action and must never emit a document operation");
        assert_eq!(projection_of(&app), before, "setCamera2d must not mutate the document");
        let board = render_body(&mut app, board2d::BODY_KEY);
        assert!(board.contains("12.5") && board.contains("-6.5"), "the new 2D camera pose must be reflected in the rendered runtime state");
        let camera3d_result = dispatch(&mut app, "setCamera3d", Some(&json!({ "camera": { "position": [42.5, 7.5, 3.5], "target": [1.5, 2.5, 3.5], "zoom": 5.5 } })), None).expect("setCamera3d");
        assert!(camera3d_result.mutations.is_empty(), "setCamera3d is a View action and must never emit a document operation");
        assert_eq!(projection_of(&app), before, "setCamera3d must not mutate the document");
        let world = render_body(&mut app, world3d::BODY_KEY);
        assert!(world.contains("42.5") && world.contains("7.5") && world.contains("1.5"), "the new 3D camera pose must be reflected in the rendered runtime state");
    }

    #[semio_framework_async_macros::async_test]
    async fn engagements_expose_no_utility_switch_options_for_either_window() {
        // 🧰️ select/brush/fill switching lives only on the framework utility bar; neither the 2D nor the 3D
        // engagement HUD may duplicate it as options.
        let mut app = app();
        let engagements = semio_framework::io::resolve_ready(app.window_engagements());
        for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
            assert!(engagements.get(window).expect("engagement").options.is_none(), "the {window} engagement must not re-expose utility switching as options");
        }
    }

    /// 🎯️ D-3 follow-up: the fill-count slider and brush placement picker are tagged `WindowMeasure::Group`s
    /// in each window's `window_measures` (surfaced by `partition_window_measures` only for their active
    /// utility), never `WindowEngagementControl`s on the HUD — for both the 2D and 3D windows.
    #[semio_framework_async_macros::async_test]
    async fn fill_and_brush_params_are_tagged_utility_options_not_engagement_controls() {
        let labels = puzzle5d_labels(&Puzzle5dConfig::default()).expect("default puzzle5d axes are explicit");
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

    #[semio_framework_async_macros::async_test]
    async fn engagement_submit_switches_utility_via_host_effect_for_both_windows() {
        // 🧰️ Reconciled dual entry point: the engagement token drives the same host-owned utility switch, once per window.
        let mut app = app();
        let result = dispatch(&mut app, "engagementSubmit", Some(&json!({ "window": world3d::WINDOW_KIND_ID, "value": "brush" })), None).expect("submit");
        let windows: Vec<&str> = result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::SetActiveUtility { window_id, utility_id } if utility_id == "brush" => Some(window_id.as_str()),
                _ => None,
            })
            .collect();
        assert!(windows.contains(&board2d::WINDOW_KIND_ID) && windows.contains(&world3d::WINDOW_KIND_ID), "brush switch is pushed to both windows, got {windows:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn gumball_translate_drag_coalesces_into_one_edit() {
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
        semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
        assert!((origin_x(&app) - start).abs() < 1e-9, "one undo restores the whole coalesced gumball drag");
    }
    //#endregion 🧰️ Window Actions & Utilities contract

    //#region 🔖️KitInPort
    #[semio_framework_async_macros::async_test]
    async fn kit_in_retained_import_media_dispatches_the_exact_factory_and_applies_canonical_output() {
        let mut app = app_with_registry();
        let before = projection_of(&app);
        let fragment = json!({
            "schema": "manifest",
            "objectKinds": [{
                "id": "retained-capsule",
                "name": "retained-capsule",
                "label": "Retained Capsule",
                "meshUrl": "/mesh/retained-capsule.glb",
                "vortices": [{ "id": "v0", "vortexKind": "retained-door", "position": [0.0, 0.0, 0.0], "direction": [0.0, 1.0, 0.0], "radius": 0.3 }],
            }],
            "vortexKinds": [{ "id": "retained-door", "name": "retained-door", "label": "Retained Door", "color": "#ff0000", "defaultCableKind": "" }],
            "cableKinds": [],
            "attractionKinds": [],
            "kindCompatibility": [{ "source": "retained-door", "target": "retained-door", "bidirectional": true }],
        });
        let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };
        let result = app.import_media("kit:in", media, &meta("local")).await.expect("retained kit:in dispatch");
        assert!(!result.mutations.is_empty(), "exact retained import must publish document mutations");
        let after = projection_of(&app);
        assert_ne!(after, before, "exact retained import must apply its completion output");
        let snapshot: Puzzle5dSnapshot = serde_json::from_value(after).expect("retained projection deserializes");
        let catalogs = crate::artifacts::puzzle5d::kind_catalogs_of(&snapshot.kind_catalogs, &snapshot.kind_catalogs_extra).expect("retained catalog replacement applied");
        let part = catalogs.parts.iter().find(|part| part.id == "retained-capsule").expect("retained part catalog row");
        assert_eq!(part.grips.first().and_then(|grip| grip.grip_kind.as_deref()), Some("retained-door"));
        assert!(catalogs.grips.iter().any(|grip| grip.id == "retained-door"));
    }

    #[semio_framework_async_macros::async_test]
    async fn kit_in_retained_import_media_enforces_exact_media_max_plus_one_before_decode() {
        let prefix = r#"{"objectKinds":[],"vortexKinds":[{"id":"grip","name":"Grip","label":""#;
        let suffix = r##"","color":"#fff","defaultCableKind":""}],"kindCompatibility":[]}"##;
        let label = "x".repeat(PUZZLE5D_IMPORT_MEDIA_BYTES.checked_sub(prefix.len() + suffix.len()).expect("retained max fixture shell"));
        let maximum = format!("{prefix}{label}{suffix}");
        assert_eq!(maximum.len(), PUZZLE5D_IMPORT_MEDIA_BYTES);
        let mut app = app_with_registry();
        let exact = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: maximum.clone() } };
        app.import_media("kit:in", exact, &meta("local")).await.expect("exact media maximum retained dispatch");
        let after_exact = projection_of(&app);
        let plus_one = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: format!("{maximum} ") } };
        let error = app.import_media("kit:in", plus_one, &meta("local")).await.expect_err("media maximum plus one must fail before Serde");
        assert!(error.message.contains("predecode cap"));
        assert_eq!(projection_of(&app), after_exact, "rejected plus-one media must not mutate the document");
    }

    /// 🔌️ The flagship `kit:in` seam: feeding a `kit.catalog` fragment shaped exactly like
    /// block3d's `puzzle3d_catalog_fragment` (`objectKinds`/`vortexKinds`, camelCase) through
    /// `Puzzle5dPlayApp::import_media` must normalize `objectKinds` into the typed
    /// `kindCatalogs.parts` (with each per-object `vortices[]` entry becoming a grip template) and
    /// `vortexKinds` into `kindCatalogs.grips`, and land both after applying the returned operations.
    #[semio_framework_async_macros::async_test]
    async fn kit_in_retained_import_media_upserts_part_and_grip_kinds_into_kind_catalogs() {
        let mut app = app_with_registry();
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

        let result = app.import_media("kit:in", media, &meta("local")).await.expect("retained kit:in import succeeds");
        assert!(!result.mutations.is_empty(), "importing a non-empty fragment must emit real operations");
        let next_projection = projection_of(&app);

        // 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W4d: `next_projection`'s raw
        // `kindCatalogs` key is now the composed `{childId,target}` handle, not the embedded
        // `{parts:[...],...}` shape a JSON pointer could probe directly — reassemble the full
        // `Puzzle5dKindCatalogs` through the typed snapshot + `kind_catalogs_of` accessor instead
        // (same pattern `sourcing`'s `stock_of` established for its own composed catalog field).
        let next_snapshot: Puzzle5dSnapshot = serde_json::from_value(next_projection.clone()).expect("next_projection deserializes as Puzzle5dSnapshot");
        let catalogs = crate::artifacts::puzzle5d::kind_catalogs_of(&next_snapshot.kind_catalogs, &next_snapshot.kind_catalogs_extra).expect("parts catalog present");
        let capsule = catalogs.parts.iter().find(|entry| entry.id == "capsule").expect("the imported part kind must appear in kindCatalogs.parts");
        assert_eq!(capsule.representations.first().map(|representation| representation.url.as_str()), Some("/mesh/capsule.glb"));
        assert_eq!(capsule.grips.first().and_then(|grip| grip.grip_kind.as_deref()), Some("door"), "the per-part grip template keeps its gripKind after normalization");
        assert_eq!(capsule.grips.first().map(|grip| grip.point), Some([0.0, 0.0, 0.0]));
        assert_eq!(capsule.grips.first().map(|grip| grip.direction), Some([0.0, 1.0, 0.0]));
        assert_eq!(capsule.grips.first().and_then(|grip| grip.radius), Some(0.3));

        let door = catalogs.grips.iter().find(|entry| entry.id == "door").expect("the imported grip kind must appear in kindCatalogs.grips");
        assert_eq!(door.default_rope_kind.as_str(), "", "defaultCableKind maps onto defaultRopeKind (a naming judgment call — see import_media's doc comment)");

        let compatibility = next_projection.pointer("/kindCompatibility").and_then(Value::as_array).expect("kind compatibility present");
        assert!(compatibility.iter().any(|entry| entry.get("source").and_then(Value::as_str) == Some("door") && entry.get("target").and_then(Value::as_str) == Some("door")));
    }

    /// 🔁️ Re-importing the SAME fragment (simulating a second producer edge, or a redelivered
    /// message on a `multiplicity: Many` port) must upsert idempotently — no duplicate rows.
    #[semio_framework_async_macros::async_test]
    async fn kit_in_retained_import_media_is_idempotent_on_repeated_delivery() {
        let mut app = app_with_registry();
        let fragment = json!({
            "objectKinds": [{ "id": "capsule", "name": "capsule", "label": "Capsule", "meshUrl": "/mesh/capsule.glb", "vortices": [] }],
            "vortexKinds": [],
            "cableKinds": [],
            "attractionKinds": [],
            "kindCompatibility": [],
        });
        let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

        for _ in 0..2 {
            app.import_media("kit:in", media.clone(), &meta("local")).await.expect("retained kit:in import succeeds");
        }

        let current = projection_of(&app);
        let current_snapshot: Puzzle5dSnapshot = serde_json::from_value(current.clone()).expect("current deserializes as Puzzle5dSnapshot");
        let catalogs = crate::artifacts::puzzle5d::kind_catalogs_of(&current_snapshot.kind_catalogs, &current_snapshot.kind_catalogs_extra).expect("parts catalog present");
        assert_eq!(catalogs.parts.iter().filter(|entry| entry.id == "capsule").count(), 1, "repeated delivery of the same fragment must upsert, never duplicate");
    }

    #[semio_framework_async_macros::async_test]
    async fn kit_in_port_is_declared_on_the_app_io() {
        let app = Puzzle5dPlayApp::default();
        let io = Puzzle5dPlayApp::io().expect("puzzle5d declares an AppIo");
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
