//! 🧊️ Puzzle 3d play app — the plugin's 3d editor surface: its `ArtifactEditor` impl (dispatch-only), the
//! structural-twin fixture document model its command/panel/window nodes mutate and render, the
//! attraction resolver that keeps every attracted object's pose derived from its attracting root,
//! and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/🧊️main`. This file dispatches and stitches.
//!
//! 🌉️ `ArtifactApp::Snapshot` is the `Puzzle3dPlaySnapshot` newtype over a bare
//! `serde_json::Value` fixture (see `crate::standards::v1::subsets::any::schema::mutations::text`'s `🔖️ValueBridge`), not the typed
//! `Puzzle3dSnapshot` — the `Puzzle3dFixture` model below is this app's own structural twin of it,
//! and each action emits the granular typed operation delta
//! (`puzzle3d_operations_from_fixture_change`) turning the old fixture into the new one.

use crate::editor::puzzle3d::commands::{
    accept_suggestion, add_brush_object, add_object_kind, add_target_volume, apply_sun, close_vortex_suggestions, create_attraction, cycle_candidate, delete_attraction, delete_selection, delete_target_volume, duplicate_selection, engagement_abort, export_fixture,
    engagement_control_select, engagement_input, engagement_repeat_last, engagement_submit, fill_build_tick, focus_selection, hover_suggestion, import_fixture, open_import_fixture, open_vortex_suggestions, patch_inspector, register_brush_mesh, relocate_target_volume, rotate_selection,
    scale_selection, select_same_kind, set_active_example, set_automatic, set_brush_placement_overlap_budget, set_camera, set_chunk_size, set_depth_variable, set_fill_count, set_kind_weight, set_manual, set_projection,
    set_proximity_radius, set_selectable_kind, set_selection_flag, set_snap_enabled, set_spacing, set_target_volume_flag, set_transform_gumball_flag, set_panel_page, set_visible, set_vortex_direction, set_vortex_show, set_voxel_dims, suggestions_tick,
    translate_selection, world_relocate,
};
use crate::editor::puzzle3d::config::{Puzzle3dConfig, Puzzle3dConfigMutation, Puzzle3dRuntime};
use crate::editor::puzzle3d::modes::edit;
use crate::editor::puzzle3d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle3d::modes::edit::windows::main;
use crate::editor::puzzle3d::modes::edit::windows::main::utilities;
use crate::editor::puzzle3d::panels::{catalogue, document, inspection, settings as settings_panel};
use crate::editor::puzzle3d::precompute::{Puzzle3dCollisionSession, Puzzle3dFillSession, Puzzle3dPrecomputeSession};
use crate::editor::puzzle3d::presence;
use crate::editor::puzzle3d::presence::{Puzzle3dPresence, Puzzle3dPresenceMutation};
use crate::editor::puzzle3d::terminology::{puzzle3d_labels, puzzle3d_localized, puzzle3d_localized_phrase, Puzzle3dLabels};
use crate::editor::puzzle3d::window as window_ownership;
use crate::standards::v1::subsets::any::schema::mutations::text::{puzzle3d_document_delta_operations, Puzzle3dMutation, Puzzle3dPlaySnapshot};
use crate::standards::v1::subsets::any::schema::mutations::puzzle3d_snapshot_mutations;
use crate::standards::v1::subsets::any::schema::Puzzle3dEngineCommand;
use crate::Puzzle3dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PastePlacement};
use semio_framework_job::{CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::{
    mesh_from_kind, panel_tab_element_id, panel_tab_first_draggable_element_id, window_element_id, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, ActionRef, AppIo, ArtifactEditor, ArtifactOwnedToolJobFactory,
    ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, BuiltNode, ConfigView, Dialect, DialogDefinition, DraftView, Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider,
    ArtifactReservedJob, ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, HoverSpec, InteractionDefinition, InteractionRef, InteractionTarget, InteractionVerb, InteractionWrite, IntroductionDefinition, IntroductionInteraction, IntroductionPlacement, IntroductionStepDefinition, Label, LocalizedLabel, Media, MediaClass, MediaError, PluginCloseStep,
    MediaForm, MediaPortDirection, MediaPortSpec, MediaType, MergeMode, NoDraft, NoDraftMutation, PortMultiplicity, SelectionMethod, SelectionMode, SelectionSpec, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolRef, WindowEngagement,
    WindowMeasure, FRAMEWORK_HISTORY_BODY_KEY, INTERACTION_SELECT_ACTION_ID, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID,
};
use store::EngineHandles;
// 🎭️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET (contract §2.1): `ArtifactEditor`
// replaces `ArtifactApp` as the authoring trait; `EditorApp<E>` is the runtime `ArtifactApp`
// adapter, needed by this file's own test harness (`VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>`).
// 🕹️ `InteractionView`'s canonical home is `semio_framework_plugin::app`; it is now also re-exported
// at that crate's root alongside `InteractionWrite`, and this path keeps the module-qualified spelling
// the other reference implementations (`process3d`, `generation3d`) use.
use dsl::json;
use dsl::FromValue;
use dsl::os_pack::json::{from_json_str, object, parse, to_json_string, to_string, Object, Value};
use semio_framework_plugin::app::{EphemeralEmit, InteractionView};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{LazyLock, Mutex, OnceLock};

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
/// 🖱️ Viewport marquee method: a plain click picks, a drag sweeps an axis-aligned rectangle. The
/// three values below are exactly `protocol::SelectionMethod`'s wire spellings, which is what
/// `World3dHost` reads out of the scene's `selection.method` to choose the marquee shape and its
/// coverage rule.
pub const PUZZLE3D_SELECTION_METHOD_PICK: &str = "pick";
/// 🖱️ Viewport marquee method: rectangle sweep.
pub const PUZZLE3D_SELECTION_METHOD_RECTANGLE: &str = "rectangle";
/// 🖱️ Viewport marquee method: free-hand lasso polygon.
pub const PUZZLE3D_SELECTION_METHOD_LASSO: &str = "lasso";
/// 🗣️ The code a notice carries when the host names a locale×terminology axis this app never
/// authored — the same fail-closed code `render_body` raises, never an English sentence, because this
/// UI has no default language.
pub const PUZZLE3D_LOCALIZATION_UNSUPPORTED: &str = "ui.localization.unsupported";
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
/// 🐁️ The one hover channel this domain declares (`puzzle3d_interaction_definition`), matching what
/// `World3dHost`'s `world3dHoverActionArgs` sends for a pointer move.
pub const PUZZLE3D_HOVER_CHANNEL: &str = "pointer";

/// 🔢️ Monotone serial behind every app-minted object / attraction / target-volume id.
pub static PUZZLE3D_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// 🌉️ This app's own `Puzzle3dScene.fixture: Puzzle3dFixture` (and `ArtifactApp::Snapshot`) stays a
/// local structural-twin mirror of `crate::Puzzle3dSnapshot`, so the DSL-text
/// example fixtures are parsed once into the typed projection and re-serialized to the JSON string
/// this module's `from_json_str::<Puzzle3dFixture>`/`.example(...)` call sites expect.
pub static CONCRETE_FOREST_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| parse_example_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT, "concrete-forest"));
pub static NAKAGIN_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| parse_example_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::PUZZLE3D_NAKAGIN_EXAMPLE_TEXT, "nakagin"));
static CONCRETE_FOREST_EXAMPLE_FIXTURE: LazyLock<Puzzle3dFixture> = LazyLock::new(|| from_json_str(CONCRETE_FOREST_EXAMPLE_JSON.as_str()).unwrap_or_else(|_| empty_fixture()));
static NAKAGIN_EXAMPLE_FIXTURE: LazyLock<Puzzle3dFixture> = LazyLock::new(|| from_json_str(NAKAGIN_EXAMPLE_JSON.as_str()).unwrap_or_else(|_| empty_fixture()));
static EMPTY_EXAMPLE_FIXTURE: LazyLock<Puzzle3dFixture> = LazyLock::new(empty_fixture);

fn parse_example_dsl(dsl_text: &str, label: &str) -> String {
    let projection = <Puzzle3dSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).unwrap_or_else(|error| panic!("{label} example fixture parses as dsl: {error}"));
    to_json_string(&projection)
}

pub fn puzzle3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PUZZLE3D_PLAY_CONTROLLER_ID.into(), action: action.into(), args: args.map(|value| json::to_dsl_value(&value)) }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: builds a framework `interactionSelect`
/// action targeting one `(granularity, id)` pair in the `vortex` domain — replaces the deleted
/// `setSelection` action builders every document/catalogue tree row used to construct by hand.
pub fn puzzle3d_interaction_select(granularity: &str, id: &str) -> ActionDescriptor {
    let targets = to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    puzzle3d_action(INTERACTION_SELECT_ACTION_ID, Some(json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })))
}
//#endregion 🔖️Constants

//#region 🔖️Document
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dVortex {
    pub id: String,
    #[value(default, rename = "vortexKind")]
    pub vortex_kind: Option<String>,
    #[value(default)]
    pub position: [f64; 3],
    #[value(default)]
    pub direction: Option<[f64; 3]>,
    #[value(default)]
    pub radius: Option<f64>,
    #[value(default)]
    pub hidden: bool,
    #[value(default)]
    pub locked: bool,
}

#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dReferenceSource {
    #[value(default)]
    pub url: String,
    #[value(default, rename = "mediaKind")]
    pub media_kind: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dReference {
    pub id: String,
    #[value(default)]
    pub source: Puzzle3dReferenceSource,
    #[value(default)]
    pub origin: [f64; 3],
    #[value(default, rename = "widthWorld")]
    pub width_world: f64,
    #[value(default)]
    pub locked: bool,
    #[value(default)]
    pub hidden: bool,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dObject {
    pub id: String,
    #[value(default)]
    pub label: Option<String>,
    #[value(default, rename = "objectKind")]
    pub object_kind: Option<String>,
    #[value(default)]
    pub origin: [f64; 3],
    #[value(default)]
    pub orientation: Option<[f64; 4]>,
    #[value(default)]
    pub scale: Option<dsl::DslValue>,
    #[value(default, rename = "meshUrl")]
    pub mesh_url: Option<String>,
    #[value(default)]
    pub vortices: Vec<Puzzle3dVortex>,
    #[value(default)]
    pub hidden: bool,
    #[value(default)]
    pub locked: bool,
    /// 🪣️ Live-viewport-only tag from `compose_fill_display` — this object's 0-based position in the
    /// fill plan's sequence, never persisted to the committed document.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reveal_index: Option<usize>,
}

/// 🗂️ The editor-side, untyped twin of the persisted `Puzzle3dMeta`: both catalog members stay
/// opaque `DslValue`s here because the panels read them row-wise.
///
/// 🕳️ An ABSENT member must project as absent, never as `Null`: the persisted twin types
/// `kindCompatibility` as a real `Vec` and refuses a `Null` for it, and
/// `Puzzle3dPlaySnapshot::new`'s decode of the whole document is all-or-nothing — so one `null`
/// member used to replace the ENTIRE typed authority with an empty document while the projection kept
/// every object. Nothing reading the projection noticed (the world, the panels and the fixture delta
/// all read `value()`), but `ArtifactApp::interaction_topology` reads the typed half, and
/// `protocol::validate_state` prunes every selected id an empty topology does not contain — which is
/// the first-pick-invisible Inspection wave B6 measured in the browser
/// (26/09/02/PUZZLE-3D-END-TO-END wave B9 lane 2).
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dFixtureMeta {
    #[value(default, rename = "kindCatalogs", skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<dsl::DslValue>,
    #[value(default, rename = "kindCompatibility", skip_serializing_if = "Option::is_none")]
    pub kind_compatibility: Option<dsl::DslValue>,
}

/// 🧊️ Persisted oriented box constraining fill placement. Volume Brush creates axis-aligned
/// voxel-sized instances; the Transform gumball edits arbitrary oriented boxes.
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dTargetVolume {
    pub id: String,
    #[value(default)]
    pub origin: [f64; 3],
    #[value(default)]
    pub orientation: Option<[f64; 4]>,
    #[value(default)]
    pub scale: Option<dsl::DslValue>,
    #[value(default)]
    pub hidden: bool,
    #[value(default)]
    pub locked: bool,
}

#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dAttraction {
    #[value(default)]
    pub id: String,
    pub attracting: String,
    pub attracted: String,
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
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dFixture {
    pub schema: String,
    #[value(default)]
    pub domain: String,
    #[value(default)]
    pub meta: Puzzle3dFixtureMeta,
    #[value(default)]
    pub objects: Vec<Puzzle3dObject>,
    #[value(default)]
    pub attractions: Vec<Puzzle3dAttraction>,
    #[value(default, rename = "targetVolumes")]
    pub target_volumes: Vec<Puzzle3dTargetVolume>,
    #[value(default)]
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

const PUZZLE3D_CLIPBOARD_SCHEMA: &str = "puzzle.3d.clipboard.v1";

fn puzzle3d_fixture_from_doc(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>) -> Puzzle3dFixture {
    puzzle3d_fixture_from_play_snapshot(doc.snapshot)
}

fn puzzle3d_fixture_from_play_snapshot(snapshot: &Puzzle3dPlaySnapshot) -> Puzzle3dFixture {
    puzzle3d_fixture_from_snapshot(snapshot.typed())
}

/// 🔭️ The ONE read of a persisted projection every viewport-visible answer is derived from — the
/// render's fixture, the semantic document delta's `before`, and the interaction topology the picks
/// are validated against. Deliberately the untyped `Puzzle3dFixture` decode rather than the
/// document's typed authority: the two are separate decodings of one document, and a disagreement
/// between them shows up as a pick that paints and then silently vanishes rather than as an error.
pub fn puzzle3d_fixture_from_projection(projection: &Value) -> Puzzle3dFixture {
    dsl::FromValue::from_value(json::to_dsl_value(projection)).unwrap_or_else(|_| empty_fixture())
}

fn puzzle3d_mutations_between(before: &Puzzle3dFixture, after: &Puzzle3dFixture) -> Vec<Puzzle3dMutation> {
    let before_snapshot = puzzle3d_snapshot_from_fixture(before);
    let after_snapshot = puzzle3d_snapshot_from_fixture(after);
    if before_snapshot == after_snapshot {
        return Vec::new();
    }
    puzzle3d_snapshot_mutations(&before_snapshot, &after_snapshot)
}

fn puzzle3d_selected_objects(fixture: &Puzzle3dFixture, interaction: &InteractionView<'_>) -> Vec<Puzzle3dObject> {
    puzzle3d_selected_objects_from(&Puzzle3dInteractionSnapshot::from_interaction(interaction), fixture)
}

fn puzzle3d_selected_objects_from(snapshot: &Puzzle3dInteractionSnapshot, fixture: &Puzzle3dFixture) -> Vec<Puzzle3dObject> {
    let ids = snapshot.selected_object_ids();
    let mut objects: Vec<Puzzle3dObject> = fixture.objects.iter().filter(|object| ids.contains(&object.id)).cloned().collect();
    if objects.is_empty() {
        objects = fixture.objects.iter().filter(|object| snapshot.selected.iter().any(|id| id == &object.id)).cloned().collect();
    }
    if objects.is_empty() {
        objects = fixture
            .objects
            .iter()
            .filter(|object| {
                object.vortices.iter().any(|vortex| snapshot.selected.iter().any(|id| id == &vortex.id || id == &puzzle3d_vortex_full_id(&object.id, &vortex.id)))
            })
            .cloned()
            .collect();
    }
    objects
}

/// 📋️ Copies the selected objects as a fixture-shaped fragment.
fn puzzle3d_copy_fragment(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
    let fixture = puzzle3d_fixture_from_doc(doc);
    puzzle3d_copy_fragment_from(&fixture, puzzle3d_selected_objects(&fixture, interaction))
}

fn puzzle3d_copy_fragment_from(fixture: &Puzzle3dFixture, objects: Vec<Puzzle3dObject>) -> Result<ClipboardFragment, ClipboardError> {
    let _ = fixture;
    if objects.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let mut clip = empty_fixture();
    clip.schema = PUZZLE3D_CLIPBOARD_SCHEMA.into();
    clip.objects = objects;
    Ok(ClipboardFragment {
        schema: PUZZLE3D_CLIPBOARD_SCHEMA.into(),
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Design },
        dsl_text: to_json_string(&puzzle3d_projection_value(dsl::ToValue::to_value(&clip))),
        pack_bytes: None,
        source_app: PUZZLE3D_PLAY_APP_ID.into(),
        label: format!("{} objects", clip.objects.len()),
    })
}

/// ✂️ Copies then deletes the selected objects as one mutation list.
fn puzzle3d_cut_operations(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, interaction: &InteractionView<'_>) -> Result<Vec<Puzzle3dMutation>, ClipboardError> {
    let before = puzzle3d_fixture_from_doc(doc);
    puzzle3d_cut_operations_from(&before, &Puzzle3dInteractionSnapshot::from_interaction(interaction))
}

fn puzzle3d_cut_operations_from(before: &Puzzle3dFixture, marks: &Puzzle3dInteractionSnapshot) -> Result<Vec<Puzzle3dMutation>, ClipboardError> {
    let objects = puzzle3d_selected_objects_from(marks, before);
    if objects.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let ids: HashSet<String> = objects.into_iter().map(|object| object.id).collect();
    let mut after = before.clone();
    after.objects.retain(|object| !ids.contains(&object.id));
    after.attractions.retain(|attraction| !ids.iter().any(|id| attraction.attracting.starts_with(&format!("{id}:")) || attraction.attracted.starts_with(&format!("{id}:"))));
    Ok(puzzle3d_mutations_between(before, &after))
}

/// 📋 Clones a copied fixture fragment with fresh ids.
fn puzzle3d_paste_operations(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle3dMutation>, ClipboardError> {
    puzzle3d_paste_operations_on(&puzzle3d_fixture_from_doc(doc), fragment, placement)
}

fn puzzle3d_paste_operations_on(before: &Puzzle3dFixture, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle3dMutation>, ClipboardError> {
    let expected = MediaType { class: MediaClass::ThreeD, form: MediaForm::Design };
    if fragment.media_type != expected {
        return Err(ClipboardError::IncompatibleMediaType(fragment.media_type.clone()));
    }
    let value = parse(&fragment.dsl_text).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    let clip = Puzzle3dFixture::from_value(json::to_dsl_value(&value)).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    if clip.objects.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let mut after = before.clone();
    let offset = placement.position.unwrap_or([0.5, 0.5, 0.0]);
    for object in clip.objects {
        let mut clone = object;
        clone.id = next_object_id();
        clone.origin[0] += offset[0];
        clone.origin[1] += offset[1];
        clone.origin[2] += offset[2];
        after.objects.push(clone);
    }
    Ok(puzzle3d_mutations_between(before, &after))
}


pub fn default_fixture() -> Puzzle3dFixture {
    CONCRETE_FOREST_EXAMPLE_FIXTURE.clone()
}

pub fn nakagin_fixture() -> Puzzle3dFixture {
    NAKAGIN_EXAMPLE_FIXTURE.clone()
}

/// 🌉️ `Puzzle3dPlaySnapshot::value()` (owned by `🧬️mutations/🦀️.rs`, out of this ticket's scope)
/// still projects the bare fixture document as `serde_json::Value` — bridges it into this file's
/// own first-party `Value` via `DslValue` without ever naming the foreign crate: `T`'s only caller
/// is `snapshot.value(): &serde_json::Value`, resolved structurally through the `DslValue: From<T>`
/// bound rather than a spelled-out type.
pub(crate) fn puzzle3d_projection_value<T>(value: T) -> Value
where
    dsl::DslValue: From<T>,
{
    json::from_dsl_value(&dsl::DslValue::from(value))
}

/// 🌉️ `puzzle3d_document_delta_operations` (owned by `🧬️mutations/🦀️.rs`, out of this ticket's
/// scope) is typed against `serde_json::Value`; bridges this file's own first-party `Value` through
/// `DslValue` at that one boundary, inferring the foreign return type from the callee's own
/// signature via `Into` rather than naming it here.
fn puzzle3d_operations_from_values(before: &Value, after: &Value) -> Vec<Puzzle3dMutation> {
    let before_dsl = json::to_dsl_value(before);
    let after_dsl = json::to_dsl_value(after);
    puzzle3d_document_delta_operations(&(&before_dsl).into(), &(&after_dsl).into())
}

/// 🧾️ Materializes the transient scene from the persisted projection (bare fixture json) and the
/// app's current view state; an unparseable projection degrades to an empty board.
pub fn scene_from_projection(projection: &Value, runtime: Puzzle3dRuntime, active_utility: &str) -> Puzzle3dScene {
    let fixture = dsl::FromValue::from_value(json::to_dsl_value(projection)).unwrap_or_else(|_| empty_fixture());
    Puzzle3dScene { fixture, runtime, active_utility: active_utility.to_string() }
}

/// 🧾️ Materializes the transient scene straight off the snapshot's own TYPED authority, with no
/// `Value` anywhere on the path. `Puzzle3dPlaySnapshot` keeps the persisted document as a typed
/// `Puzzle3dSnapshot` and materializes its `serde_json` projection only for readers that ask; going
/// through that projection cost a measured 4.4 ms per action on the 180-object Nakagin document
/// (1.6 ms `serde_json::Value` → `DslValue` → `Value`, 2.8 ms back to `DslValue` → `Puzzle3dFixture`)
/// for a translation between two structural twins. Ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3;
/// `puzzle3d_typed_fixture_matches_the_projection_bridge_for_every_example` is the differential law.
pub fn scene_from_snapshot(document: &Puzzle3dSnapshot, runtime: Puzzle3dRuntime, active_utility: &str) -> Puzzle3dScene {
    Puzzle3dScene { fixture: puzzle3d_fixture_from_snapshot(document), runtime, active_utility: active_utility.to_string() }
}

/// 🌉️ The persisted `Puzzle3dSnapshot` as this app's own structural-twin `Puzzle3dFixture`. The
/// twin deliberately does not carry `anchor`, a vortex `label` or an attraction's diagram `x`/`y`, so
/// those are dropped here exactly as the `Value` bridge dropped them (an unknown key is ignored by
/// `FromValue`), and the two untyped catalog members are serialized once instead of per object.
pub fn puzzle3d_fixture_from_snapshot(document: &Puzzle3dSnapshot) -> Puzzle3dFixture {
    Puzzle3dFixture {
        schema: document.schema.clone(),
        domain: document.domain.clone(),
        meta: Puzzle3dFixtureMeta { kind_catalogs: document.meta.kind_catalogs.as_ref().map(dsl::ToValue::to_value), kind_compatibility: Some(dsl::ToValue::to_value(&document.meta.kind_compatibility)) },
        objects: document.objects.iter().map(fixture_object_from_snapshot).collect(),
        attractions: document.attractions.iter().map(fixture_attraction_from_snapshot).collect(),
        target_volumes: document.target_volumes.iter().map(fixture_target_volume_from_snapshot).collect(),
        references: document.references.iter().map(fixture_reference_from_snapshot).collect(),
    }
}

/// 🧬️ Inverse of [`puzzle3d_fixture_from_snapshot`] — the document delta must compare typed snapshots,
/// not a camelCase fixture `ToValue` re-parsed as `Puzzle3dSnapshot` (that FromValue fails closed to empty).
pub fn puzzle3d_snapshot_from_fixture(fixture: &Puzzle3dFixture) -> Puzzle3dSnapshot {
    Puzzle3dSnapshot {
        schema: fixture.schema.clone(),
        domain: fixture.domain.clone(),
        meta: crate::Puzzle3dMeta {
            kind_catalogs: fixture.meta.kind_catalogs.clone().and_then(|value| dsl::FromValue::from_value(value).ok()),
            kind_compatibility: fixture.meta.kind_compatibility.clone().and_then(|value| dsl::FromValue::from_value(value).ok()).unwrap_or_default(),
        },
        objects: fixture.objects.iter().map(snapshot_object_from_fixture).collect(),
        attractions: fixture.attractions.iter().map(snapshot_attraction_from_fixture).collect(),
        target_volumes: fixture.target_volumes.iter().map(snapshot_target_volume_from_fixture).collect(),
        references: fixture.references.iter().map(snapshot_reference_from_fixture).collect(),
    }
}

fn snapshot_object_from_fixture(object: &Puzzle3dObject) -> crate::Puzzle3dObject {
    crate::Puzzle3dObject {
        id: object.id.clone(),
        label: object.label.clone(),
        object_kind: object.object_kind.clone(),
        anchor: crate::Puzzle3dObjectAnchor::default(),
        origin: object.origin,
        orientation: object.orientation,
        scale: object.scale.clone().and_then(|value| dsl::FromValue::from_value(value).ok()),
        mesh_url: object.mesh_url.clone(),
        vortices: object.vortices.iter().map(snapshot_vortex_from_fixture).collect(),
        hidden: object.hidden,
        locked: object.locked,
    }
}

fn snapshot_vortex_from_fixture(vortex: &Puzzle3dVortex) -> crate::Puzzle3dVortex {
    crate::Puzzle3dVortex { id: vortex.id.clone(), vortex_kind: vortex.vortex_kind.clone(), label: None, position: vortex.position, direction: vortex.direction, radius: vortex.radius, hidden: vortex.hidden, locked: vortex.locked }
}

fn snapshot_attraction_from_fixture(attraction: &Puzzle3dAttraction) -> crate::Puzzle3dAttraction {
    crate::Puzzle3dAttraction {
        id: attraction.id.clone(),
        attracting: attraction.attracting.clone(),
        attracted: attraction.attracted.clone(),
        gap: attraction.gap,
        shift: attraction.shift,
        rise: attraction.rise,
        rotation: attraction.rotation,
        turn: attraction.turn,
        tilt: attraction.tilt,
        x: 0.0,
        y: 0.0,
    }
}

fn snapshot_target_volume_from_fixture(volume: &Puzzle3dTargetVolume) -> crate::Puzzle3dTargetVolume {
    crate::Puzzle3dTargetVolume { id: volume.id.clone(), origin: volume.origin, orientation: volume.orientation, scale: volume.scale.clone().and_then(|value| dsl::FromValue::from_value(value).ok()), hidden: volume.hidden, locked: volume.locked }
}

fn snapshot_reference_from_fixture(reference: &Puzzle3dReference) -> crate::Puzzle3dReference {
    crate::Puzzle3dReference {
        id: reference.id.clone(),
        source: crate::Puzzle3dReferenceSource { url: reference.source.url.clone(), media_kind: reference.source.media_kind.clone() },
        origin: reference.origin,
        width_world: reference.width_world,
        locked: reference.locked,
        hidden: reference.hidden,
    }
}

fn fixture_object_from_snapshot(object: &crate::Puzzle3dObject) -> Puzzle3dObject {
    Puzzle3dObject {
        id: object.id.clone(),
        label: object.label.clone(),
        object_kind: object.object_kind.clone(),
        origin: object.origin,
        orientation: object.orientation,
        scale: object.scale.as_ref().map(dsl::ToValue::to_value),
        mesh_url: object.mesh_url.clone(),
        vortices: object.vortices.iter().map(fixture_vortex_from_snapshot).collect(),
        hidden: object.hidden,
        locked: object.locked,
        reveal_index: None,
    }
}

fn fixture_vortex_from_snapshot(vortex: &crate::Puzzle3dVortex) -> Puzzle3dVortex {
    Puzzle3dVortex { id: vortex.id.clone(), vortex_kind: vortex.vortex_kind.clone(), position: vortex.position, direction: vortex.direction, radius: vortex.radius, hidden: vortex.hidden, locked: vortex.locked }
}

fn fixture_attraction_from_snapshot(attraction: &crate::Puzzle3dAttraction) -> Puzzle3dAttraction {
    Puzzle3dAttraction {
        id: attraction.id.clone(),
        attracting: attraction.attracting.clone(),
        attracted: attraction.attracted.clone(),
        gap: attraction.gap,
        shift: attraction.shift,
        rise: attraction.rise,
        rotation: attraction.rotation,
        turn: attraction.turn,
        tilt: attraction.tilt,
    }
}

fn fixture_target_volume_from_snapshot(volume: &crate::Puzzle3dTargetVolume) -> Puzzle3dTargetVolume {
    Puzzle3dTargetVolume { id: volume.id.clone(), origin: volume.origin, orientation: volume.orientation, scale: volume.scale.as_ref().map(dsl::ToValue::to_value), hidden: volume.hidden, locked: volume.locked }
}

fn fixture_reference_from_snapshot(reference: &crate::Puzzle3dReference) -> Puzzle3dReference {
    Puzzle3dReference {
        id: reference.id.clone(),
        source: Puzzle3dReferenceSource { url: reference.source.url.clone(), media_kind: reference.source.media_kind.clone() },
        origin: reference.origin,
        width_world: reference.width_world,
        locked: reference.locked,
        hidden: reference.hidden,
    }
}

/// 🧮️ Document operations for a fixture mutation through the typed semantic delta vocabulary.
pub fn puzzle3d_operations_from_fixture_change(before: &Value, after_fixture: &Puzzle3dFixture) -> Vec<Puzzle3dMutation> {
    let before_fixture: Puzzle3dFixture = dsl::FromValue::from_value(json::to_dsl_value(before)).unwrap_or_else(|_| empty_fixture());
    let before_snapshot = puzzle3d_snapshot_from_fixture(&before_fixture);
    let after_snapshot = puzzle3d_snapshot_from_fixture(after_fixture);
    if before_snapshot == after_snapshot {
        return Vec::new();
    }
    let ops = puzzle3d_snapshot_mutations(&before_snapshot, &after_snapshot);
    ops
}

/// 🔌️ `kit:in` seam helper: keyed UPSERT of `incoming` rows (each shaped `{"id": "...", ...}`) into
/// `catalogs[section]` (creating the section as an empty array if absent) — replaces any existing row
/// with the same `"id"`, else appends. Deterministic/order-independent in the resulting SET of ids (a
/// `multiplicity: Many` port may fan in from several producers across several `import_media` calls);
/// when two producers disagree on one id's content, the most-recently-applied wins.
/// 🌉️ Rebuilds `row` into a fresh `Object` rather than mutating in place — this crate's own
/// `Object` (unlike `serde_json::Map`) has no `remove`, only `get`/`get_mut`/`insert`/`iter`, so
/// dropping the `meshUrl` key means copying every other key across instead.
fn puzzle3d_normalize_object_kind_row(row: Value) -> Value {
    let mesh_url = row.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).map(str::to_string);
    let has_rep = row.get("representations").and_then(Value::as_array).is_some_and(|rows| rows.iter().any(|rep| rep.get("url").and_then(Value::as_str).filter(|url| !url.is_empty()).is_some()));
    let id = row.get("id").and_then(Value::as_str).unwrap_or("kind").to_string();
    let Some(url) = mesh_url else {
        return row;
    };
    let Some(object) = row.as_object() else {
        return row;
    };
    let mut next = Object::new();
    for (key, value) in object.iter() {
        if key == "meshUrl" {
            continue;
        }
        next.insert(key.to_string(), value.clone());
    }
    if !has_rep {
        next.insert("representations".to_string(), json!([{ "id": format!("{id}:rep0"), "name": "default", "url": url, "mime": "", "description": "", "tags": [] }]));
    }
    Value::Object(next)
}

/// 🌉️ `catalogs` is stored as `Puzzle3dFixtureMeta.kind_catalogs: Option<dsl::DslValue>` (a
/// `#[derive(value_derive::ToValue, FromValue)]` field, so it cannot hold this crate's own
/// `Value` — see this file's own `dsl::DslValue` field-type note), while `incoming` is the
/// still-`Value`-shaped `import_media` fragment; `DslValue`'s `Object`/`Array` variants are bare
/// `Vec`s with no `entry`/`as_*_mut` sugar, so the section lookup below is index-based instead of
/// `serde_json::Value`'s `entry(...).or_insert_with(...)`.
fn puzzle3d_upsert_catalog_rows(catalogs: &mut dsl::DslValue, section: &str, incoming: Option<&Value>) {
    let Some(incoming_rows) = incoming.and_then(Value::as_array) else {
        return;
    };
    if incoming_rows.is_empty() {
        return;
    }
    let dsl::DslValue::Object(catalog_entries) = catalogs else {
        return;
    };
    let section_index = match catalog_entries.iter().position(|(key, _)| key == section) {
        Some(index) => index,
        None => {
            catalog_entries.push((section.to_string(), dsl::DslValue::Array(Vec::new())));
            catalog_entries.len() - 1
        }
    };
    if !matches!(catalog_entries[section_index].1, dsl::DslValue::Array(_)) {
        catalog_entries[section_index].1 = dsl::DslValue::Array(Vec::new());
    }
    let dsl::DslValue::Array(existing) = &mut catalog_entries[section_index].1 else {
        return;
    };
    for row in incoming_rows {
        let row = if section == "objects" { puzzle3d_normalize_object_kind_row(row.clone()) } else { row.clone() };
        let Some(id) = row.get("id").and_then(Value::as_str) else {
            continue;
        };
        let row_dsl = json::to_dsl_value(&row);
        match existing.iter().position(|entry| entry.get("id").and_then(dsl::DslValue::as_str) == Some(id)) {
            Some(index) => existing[index] = row_dsl,
            None => existing.push(row_dsl),
        }
    }
}

pub fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| dsl::FromValue::from_value(json::to_dsl_value(value)).ok()).filter(|ids: &Vec<String>| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
}

/** @emoji 🧭️ Whether `handle` may emit VCS operations from a fixture before/after delta — view-only actions skip the document snapshot entirely. */
fn puzzle3d_action_document_intent(action: &str) -> bool {
    matches!(
        action,
        "setActiveExample"
            | "addObjectKind"
            | "deleteSelection"
            | "duplicateSelection"
            | "translateSelection"
            | "rotateSelection"
            | "scaleSelection"
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
            | "importFixture"
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
fn scale_value_mul(scale: &Option<dsl::DslValue>, sx: f64, sy: f64, sz: f64) -> dsl::DslValue {
    let triple = |x: f64, y: f64, z: f64| dsl::DslValue::Array(vec![dsl::DslValue::float(x), dsl::DslValue::float(y), dsl::DslValue::float(z)]);
    match scale.as_ref().and_then(dsl::DslValue::as_array) {
        Some(values) if values.len() >= 3 => triple(values[0].as_f64().unwrap_or(1.0) * sx, values[1].as_f64().unwrap_or(1.0) * sy, values[2].as_f64().unwrap_or(1.0) * sz),
        _ => match scale.as_ref().and_then(dsl::DslValue::as_f64) {
            Some(factor) => triple(factor * sx, factor * sy, factor * sz),
            None => triple(sx, sy, sz),
        },
    }
}

/// 🗂️ One document catalog's `objectKind id → meshUrl` rows, indexed in a single pass over the
/// untyped catalog array. Resolving one object's mesh straight out of that array is a linear scan, so
/// a caller that resolves a whole document pays O(objects × kinds) — a measured 7.4 ms of every
/// `sync_precompute_session` turn on the 180-object Nakagin document (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END W-P3). Every whole-document caller builds this once instead.
pub struct Puzzle3dKindMeshIndex<'a> {
    by_kind: HashMap<&'a str, &'a str>,
}

impl<'a> Puzzle3dKindMeshIndex<'a> {
    pub fn of(meta: &'a Puzzle3dFixtureMeta) -> Self {
        let rows = meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("objects")).and_then(dsl::DslValue::as_array).unwrap_or_default();
        let mut by_kind = HashMap::with_capacity(rows.len());
        for entry in rows {
            let (Some(id), Some(url)) = (entry.get("id").and_then(dsl::DslValue::as_str), entry.get("meshUrl").and_then(dsl::DslValue::as_str)) else {
                continue;
            };
            by_kind.insert(id, url);
        }
        Self { by_kind }
    }

    /// 🥽️ The mesh identity one object renders and collides with — its own `meshUrl` when it carries
    /// a non-empty one, else its kind's catalog row.
    pub fn resolve<'b>(&'b self, object: &'b Puzzle3dObject) -> Option<&'b str> {
        if let Some(url) = object.mesh_url.as_deref().filter(|url| !url.is_empty()) {
            return Some(url);
        }
        self.by_kind.get(object.object_kind.as_deref()?).copied()
    }

    fn catalog_urls(&self) -> impl Iterator<Item = &'a str> + '_ {
        self.by_kind.values().copied()
    }
}

pub fn resolve_object_mesh_url(object: &Puzzle3dObject, meta: &Puzzle3dFixtureMeta) -> Option<String> {
    Puzzle3dKindMeshIndex::of(meta).resolve(object).map(str::to_string)
}

pub fn collect_mesh_urls(fixture: &Puzzle3dFixture) -> Vec<String> {
    let index = Puzzle3dKindMeshIndex::of(&fixture.meta);
    let mut urls: HashSet<&str> = HashSet::with_capacity(fixture.objects.len());
    for object in &fixture.objects {
        if let Some(url) = index.resolve(object) {
            urls.insert(url);
        }
    }
    urls.extend(index.catalog_urls());
    urls.into_iter().map(str::to_string).collect()
}

fn scale_array_to_dsl_value(scale: [f64; 3]) -> dsl::DslValue {
    dsl::DslValue::Array(scale.iter().map(|component| dsl::DslValue::float(*component)).collect())
}

pub fn object_scale_json(object: &Puzzle3dObject) -> [f64; 3] {
    match object.scale.as_ref().and_then(dsl::DslValue::as_array) {
        Some(values) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        _ => [1.0, 1.0, 1.0],
    }
}

pub fn target_volume_scale_json(volume: &Puzzle3dTargetVolume) -> [f64; 3] {
    match volume.scale.as_ref().and_then(dsl::DslValue::as_array) {
        Some(values) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
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

pub fn puzzle3d_catalog_entries<'a>(fixture: &'a Puzzle3dFixture, section: &str) -> &'a [dsl::DslValue] {
    fixture.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get(section)).and_then(|entries| entries.as_array()).unwrap_or(&[])
}

pub fn puzzle3d_kind_ids(fixture: &Puzzle3dFixture, section: &str) -> Vec<String> {
    puzzle3d_catalog_entries(fixture, section).iter().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

pub fn next_object_id() -> String {
    let next = PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("object-{next}")
}

/// 🧊️ Seeds real vortices for a freshly placed object from its kind catalog's `vortices` templates, so it is immediately brushable instead of connector-less.
pub fn puzzle3d_vortices_from_kind_template(catalog_entry: &dsl::DslValue) -> Vec<Puzzle3dVortex> {
    catalog_entry
        .get("vortices")
        .and_then(|value| value.as_array())
        .map(|templates| {
            templates
                .iter()
                .enumerate()
                .map(|(index, template)| {
                    let position = template.get("position").and_then(|value| dsl::FromValue::from_value(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]);
                    let direction = template.get("direction").and_then(|value| dsl::FromValue::from_value(value.clone()).ok());
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
pub fn puzzle3d_fill_tool_active(config: &Puzzle3dRuntime) -> bool {
    config.active_tool_id.as_deref() == Some(fill_tool::TOOL_ID)
}

/// 🪟️ The window INSTANCE one render/dispatch is addressed at, in the order the host actually
/// carries it: an explicit `<body>:<instance>` body key, then `ViewModel::window_id` — the field
/// `ViewModel::for_window_instance` stamps and the ONLY per-call carrier of the instance identity —
/// then `ViewModel::focused_window_id`, then the caller's own fallback (a command's `windowId`
/// argument, which may still spell a bare window KIND), then the live roster's first pane.
///
/// 🎯️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B15: the focused-pane hop is what an APP-LEVEL PANEL
/// resolves through. A panel body is never rendered FOR a window (`ViewModel::for_panel` clears
/// `window_id` by design), so before it this function fell straight to the roster's first entry —
/// which is the base window KIND, a pane nobody is looking at — and every Settings stepper baked THAT
/// into its args (B12 §5.1 measured `setGridSpacing window=Some("puzzle3d-main")` live).
///
/// 🕹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B9: `ViewModel::active_utility_by_window_id`,
/// the window-owned config partitions and the per-window options are ALL keyed by instance
/// (`puzzle3d-main-perspective`), so resolving the roster's first pane before the host's own
/// `window_id` published the FIRST pane's armed utility into every other pane's world lane.
///
/// 🎯️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B18: a kind-valued `window_id` (`puzzle3d-main`)
/// is not an addressing hit when an instance candidate exists — `suggestionsTick` otherwise looks
/// the utility map up under the kind (`map_hit=false`) and never warms the brush preview cache.
pub fn puzzle3d_window_id_is_kind(id: &str) -> bool {
    id == main::WINDOW_KIND_ID
}

fn puzzle3d_first_instance_id<'a>(ids: impl IntoIterator<Item = Option<&'a str>>) -> Option<&'a str> {
    ids.into_iter().flatten().find(|id| !id.is_empty() && !puzzle3d_window_id_is_kind(id))
}

fn puzzle3d_utility_map_hit<'a>(view: &'a semio_framework_plugin::ViewModel, wid: &str) -> Option<&'a String> {
    view.active_utility_by_window_id.get(wid).or_else(|| {
        if !puzzle3d_window_id_is_kind(wid) {
            return None;
        }
        view.focused_window_id
            .as_deref()
            .and_then(|focused| view.active_utility_by_window_id.get(focused))
            .or_else(|| {
                view.active_utility_by_window_id.iter().find_map(|(key, utility)| {
                    (key.starts_with(wid) && key.as_str() != wid && !utility.is_empty()).then_some(utility)
                })
            })
    })
}

pub fn puzzle3d_addressed_window_id<'a>(view_state: Option<&'a semio_framework_plugin::ViewModel>, keyed: Option<&'a str>, fallback: Option<&'a str>, roster: &'a [String]) -> &'a str {
    puzzle3d_first_instance_id([
        keyed,
        view_state.and_then(|view| view.window_id.as_deref()),
        view_state.and_then(|view| view.focused_window_id.as_deref()),
        fallback,
        roster.iter().map(String::as_str).find(|id| !id.is_empty() && !puzzle3d_window_id_is_kind(id)),
    ])
    .or_else(|| keyed.filter(|id| !id.is_empty()))
    .or_else(|| view_state.and_then(|view| view.window_id.as_deref()).filter(|id| !id.is_empty()))
    .or_else(|| view_state.and_then(|view| view.focused_window_id.as_deref()).filter(|id| !id.is_empty()))
    .or(fallback.filter(|id| !id.is_empty()))
    .or_else(|| roster.first().map(String::as_str).filter(|id| !id.is_empty()))
    .unwrap_or(main::WINDOW_KIND_ID)
}

/// 🛠️ The effective interaction id threaded through `Puzzle3dScene.active_utility`: the host-owned
/// per-window utility from `ViewModel`, unless the mode-level fill tool is active
/// (`active_tool_id`), in which case fill wins. Fill keeps its viewport interaction even though it is
/// declared as a windowless tool, not a `WindowKindDefinition` utility.
///
/// 🎯️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B18: `SET_ACTIVE_UTILITY` keys the map by window
/// INSTANCE. A kind-valued lookup (`puzzle3d-main`) — `suggestionsTick` / `registerBrushMesh` when
/// `view.window_id` is the kind — is a map miss (`map_hit=false`, `utility=` empty default), so the
/// tick never arms `brush_live_target` and render publishes `preview=0`. Kind lookups fall through
/// to the focused pane, then the first instance-prefixed map key.
pub fn puzzle3d_scene_active_utility(config: &Puzzle3dRuntime, view_state: Option<&semio_framework_plugin::ViewModel>, window_id: Option<&str>) -> String {
    if puzzle3d_fill_tool_active(config) {
        return fill_tool::TOOL_ID.to_string();
    }
    window_id
        .and_then(|wid| view_state.and_then(|view| puzzle3d_utility_map_hit(view, wid)))
        .or_else(|| view_state.and_then(|view| view.active_utility_id.as_ref()))
        .filter(|utility| !utility.is_empty())
        .cloned()
        .unwrap_or_else(|| PUZZLE3D_DEFAULT_UTILITY.to_string())
}

pub fn puzzle3d_brush_target_vortex(envelope: &Puzzle3dScene, interaction: &Puzzle3dInteractionSnapshot) -> Option<String> {
    if let Some(id) = interaction.selected_vortex_ids().first() {
        return Some(id.clone());
    }
    if let Some(id) = interaction.hovered_vortex_full_id(&envelope.fixture) {
        return Some(id.to_string());
    }
    let hovered_object = interaction.hovered_object_id(&envelope.fixture)?;
    let object = envelope.fixture.objects.iter().find(|object| object.id == hovered_object)?;
    object.vortices.first().map(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id))
}
//#endregion 🔖️SceneState

//#region 🔖️InteractionSnapshot
/// 🕹️ One render pass' read of the framework-owned `vortex` interaction domain (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): the selected ids at the granularity they were
/// picked at, plus the pointer-channel hover. `DomainSelection` carries exactly one live granularity,
/// so this mirrors [`Puzzle3dActionCtx::selected_object_ids`] and friends verbatim — render and
/// reducer read the same shape, never two divergent projections.
///
/// 🐁️ Hover ids arrive WITHOUT a granularity (`protocol::DomainHover` has none), so classifying one
/// needs the document: `hovered_object_id`/`hovered_vortex_full_id` resolve against the live fixture
/// rather than guessing from the id's spelling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Puzzle3dInteractionSnapshot {
    pub granularity: String,
    pub selected: Vec<String>,
    pub hovered: Vec<String>,
}

impl Puzzle3dInteractionSnapshot {
    /// 🕹️ Reads the live `vortex` domain: its selection plus its `"pointer"`-channel hover.
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        let selection = interaction.selection(PUZZLE3D_INTERACTION_DOMAIN);
        let hover = interaction.hover(PUZZLE3D_INTERACTION_DOMAIN, PUZZLE3D_HOVER_CHANNEL);
        let leftover_ids = interaction.leftover_selected_ids();
        let selected = if selection.ids.is_empty() { leftover_ids } else { selection.ids.clone() };
        let granularity = if !selection.granularity.is_empty() { selection.granularity.clone() } else if !selected.is_empty() { PUZZLE3D_GRANULARITY_OBJECT.to_string() } else { selection.granularity.clone() };
        Self { granularity, selected, hovered: hover.ids.clone() }
    }

    /// 🕹️ The retained-reducer twin — the same read from the raw `InteractionState`/hover map a
    /// retained tool job is handed, so a command reusing a render helper sees the same selection and
    /// hover the viewport painted.
    pub fn from_state(state: &protocol::InteractionState, hover: &semio_framework_plugin::app::InteractionHoverState) -> Self {
        let selection = state.selection.get(PUZZLE3D_INTERACTION_DOMAIN);
        let hovered = hover.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|hover| hover.channel == PUZZLE3D_HOVER_CHANNEL).map(|hover| hover.ids.clone()).unwrap_or_default();
        let leftover_ids: Vec<String> = state.selection.values().flat_map(|selection| selection.ids.iter().cloned()).collect();
        let selected = selection.filter(|selection| !selection.ids.is_empty()).map(|selection| selection.ids.clone()).unwrap_or(leftover_ids);
        let granularity = selection.map(|selection| selection.granularity.clone()).filter(|granularity| !granularity.is_empty()).unwrap_or_else(|| if selected.is_empty() { String::new() } else { PUZZLE3D_GRANULARITY_OBJECT.to_string() });
        Self { granularity, selected, hovered }
    }

    pub fn selected_ids(&self, granularity: &str) -> &[String] {
        if self.granularity == granularity {
            &self.selected
        } else {
            &[]
        }
    }
    pub fn selected_object_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE3D_GRANULARITY_OBJECT)
    }
    pub fn selected_vortex_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE3D_GRANULARITY_VORTEX)
    }
    pub fn selected_attraction_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE3D_GRANULARITY_ATTRACTION)
    }
    pub fn selected_target_volume_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE3D_GRANULARITY_TARGET_VOLUME)
    }
    pub fn selected_reference_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE3D_GRANULARITY_REFERENCE)
    }

    /// 🐁️ The hovered object id, resolved against the fixture's own object ids.
    pub fn hovered_object_id(&self, fixture: &Puzzle3dFixture) -> Option<&str> {
        self.hovered.iter().find(|id| fixture.objects.iter().any(|object| &object.id == *id)).map(String::as_str)
    }

    /// 🐁️ The hovered vortex full id (`{objectId}:{vortexId}`), resolved against the fixture.
    pub fn hovered_vortex_full_id(&self, fixture: &Puzzle3dFixture) -> Option<&str> {
        self.hovered.iter().find(|id| fixture.objects.iter().any(|object| object.vortices.iter().any(|vortex| &puzzle3d_vortex_full_id(&object.id, &vortex.id) == *id))).map(String::as_str)
    }

    /// 🐁️ The hovered reference id, resolved against the fixture.
    pub fn hovered_reference_id(&self, fixture: &Puzzle3dFixture) -> Option<&str> {
        self.hovered.iter().find(|id| fixture.references.iter().any(|reference| &reference.id == *id)).map(String::as_str)
    }

    /// 👁️ Whether `object_id` is itself selected/hovered, or owns a selected/hovered vortex — the
    /// predicate the `PUZZLE3D_VORTEX_SHOW_SELECTED` marker mode is gated on.
    pub fn touches_object(&self, object: &Puzzle3dObject) -> bool {
        let mut marks = self.selected.iter().chain(self.hovered.iter());
        marks.any(|id| id == &object.id || object.vortices.iter().any(|vortex| &puzzle3d_vortex_full_id(&object.id, &vortex.id) == id))
    }

    /// 🕹️ Whether anything at all is marked — the cheap "is there a selection" test the gumball and
    /// the inspection panel's empty state both need.
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty() && self.hovered.is_empty()
    }
}
//#endregion 🔖️InteractionSnapshot

//#region 🔖️FixtureEdits
/// 🧲️ Applies one absolute gumball translate (total delta from drag-start) onto a fixture snapshot.
pub fn puzzle3d_apply_translate(fixture: &mut Puzzle3dFixture, object_ids: &[String], volume_ids: &[String], dx: f64, dy: f64, dz: f64) {
    for object in &mut fixture.objects {
        if object_ids.contains(&object.id) && !object.locked {
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
                                object.scale = Some(scale_array_to_dsl_value(scale));
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
                                volume.scale = Some(scale_array_to_dsl_value(scale));
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
/// writes `envelope.runtime.camera` (session-only per-window state), never the shared `fixture`. An
/// EMPTY `selected_object_ids` frames the whole document — same subject rule as
/// `Puzzle3dFocusSelectionWork::frames`, so the retained lane and this one can never disagree.
pub fn apply_puzzle3d_focus_selection(envelope: &mut Puzzle3dScene, selected_object_ids: &[String]) {
    let selected_origins: Vec<[f64; 3]> = envelope.fixture.objects.iter().filter(|object| selected_object_ids.is_empty() || selected_object_ids.contains(&object.id)).map(|object| object.origin).collect();
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
/// translation between two independently-evolved Rust types, not a wasm-bindgen boundary. Built
/// TYPED, field by field: routing the whole document through a `dsl::DslValue` tree and back cost a
/// measured 3.5 ms per sync on the 180-object Nakagin document (0.5 ms to build the tree, 3.0 ms to
/// re-parse it), four times this artifact's own per-step budget, for a translation whose every field
/// is a move or a copy. Only the two genuinely untyped members of `Puzzle3dFixtureMeta`
/// (`kindCatalogs`/`kindCompatibility`, which this app's document model keeps as raw `DslValue`)
/// still decode through `FromValue`, and a malformed one refuses the whole scene exactly as the
/// all-`FromValue` shape did. `#[cfg(test)] scene_config_value` below is the differential oracle this
/// is checked against.
fn scene_config(envelope: &Puzzle3dScene) -> Option<crate::standards::v1::subsets::any::schema::SceneConfig> {
    let meta = &envelope.fixture.meta;
    let kind_catalogs = match meta.kind_catalogs.as_ref() {
        None | Some(dsl::DslValue::Null) => None,
        Some(catalogs) => Some(dsl::FromValue::from_value(catalogs.clone()).ok()?),
    };
    let kind_compatibility = match meta.kind_compatibility.as_ref() {
        None => Vec::new(),
        Some(entries) => dsl::FromValue::from_value(entries.clone()).ok()?,
    };
    Some(crate::standards::v1::subsets::any::schema::SceneConfig {
        fixture: crate::standards::v1::subsets::any::schema::Fixture {
            objects: envelope.fixture.objects.iter().map(engine_fixture_object).collect(),
            attractions: envelope.fixture.attractions.iter().map(engine_attraction_props).collect(),
            target_volumes: envelope.fixture.target_volumes.iter().map(engine_world_volume_props).collect(),
        },
        kind_catalogs,
        kind_compatibility,
        overlap_budget: envelope.runtime.overlap_budget,
        seed: 1,
        host_rules: crate::standards::v1::subsets::any::schema::BrushHostRules::default(),
        weights: crate::standards::v1::subsets::any::schema::BrushKindWeights {
            object_weights: envelope.runtime.object_kind_weights.iter().map(|(kind, weight)| (kind.clone(), *weight)).collect(),
            vortex_weights: envelope.runtime.vortex_kind_weights.iter().map(|(kind, weight)| (kind.clone(), *weight)).collect(),
        },
    })
}

/// 🧱️ One document object as the engine sees it. `anchor` has no counterpart in this app's own
/// document model, so it takes the same default the JSON bridge gave it.
fn engine_fixture_object(object: &Puzzle3dObject) -> crate::standards::v1::subsets::any::schema::FixtureObject {
    crate::standards::v1::subsets::any::schema::FixtureObject {
        id: object.id.clone(),
        object_kind: object.object_kind.clone(),
        anchor: crate::Puzzle3dObjectAnchor::default(),
        mesh_url: object.mesh_url.clone(),
        origin: object.origin,
        orientation: object.orientation,
        scale: object.scale.clone(),
        vortices: object.vortices.iter().map(engine_vortex_props).collect(),
        reveal_index: object.reveal_index,
    }
}

fn engine_vortex_props(vortex: &Puzzle3dVortex) -> crate::standards::v1::subsets::any::schema::VortexProps {
    crate::standards::v1::subsets::any::schema::VortexProps { id: vortex.id.clone(), vortex_kind: vortex.vortex_kind.clone(), position: vortex.position, direction: vortex.direction }
}

/// 🔗️ One attraction as the engine sees it. `x`/`y` are diagram-only coordinates this app's own
/// document model does not carry, so they take the same default the JSON bridge gave them.
fn engine_attraction_props(attraction: &Puzzle3dAttraction) -> crate::standards::v1::subsets::any::schema::AttractionProps {
    crate::standards::v1::subsets::any::schema::AttractionProps {
        id: attraction.id.clone(),
        attracting: attraction.attracting.clone(),
        attracted: attraction.attracted.clone(),
        gap: attraction.gap,
        shift: attraction.shift,
        rise: attraction.rise,
        rotation: attraction.rotation,
        turn: attraction.turn,
        tilt: attraction.tilt,
        x: 0.0,
        y: 0.0,
    }
}

fn engine_world_volume_props(volume: &Puzzle3dTargetVolume) -> crate::standards::v1::subsets::any::schema::WorldVolumeProps {
    crate::standards::v1::subsets::any::schema::WorldVolumeProps { id: volume.id.clone(), origin: volume.origin, orientation: volume.orientation, scale: volume.scale.clone() }
}

/// 🌉️ The all-`DslValue` bridge `scene_config` replaced, retained as its differential oracle: the
/// derived `ToValue`/`FromValue` machinery is an independent implementation of the same translation,
/// so `puzzle3d_typed_scene_config_matches_the_value_bridge_for_every_example` can prove the typed
/// construction agrees with it on every shipped document.
#[cfg(test)]
pub(crate) fn scene_config_value(envelope: &Puzzle3dScene) -> dsl::DslValue {
    dsl::DslValue::object([
        (
            "fixture".to_string(),
            dsl::DslValue::object([
                ("objects".to_string(), dsl::ToValue::to_value(&envelope.fixture.objects)),
                ("attractions".to_string(), dsl::ToValue::to_value(&envelope.fixture.attractions)),
                ("targetVolumes".to_string(), dsl::ToValue::to_value(&envelope.fixture.target_volumes)),
            ]),
        ),
        ("kindCatalogs".to_string(), envelope.fixture.meta.kind_catalogs.clone().unwrap_or(dsl::DslValue::Null)),
        ("kindCompatibility".to_string(), envelope.fixture.meta.kind_compatibility.clone().unwrap_or_else(|| dsl::DslValue::Array(Vec::new()))),
        ("overlapBudget".to_string(), dsl::DslValue::float(envelope.runtime.overlap_budget)),
        ("seed".to_string(), dsl::DslValue::uint(1)),
        ("weights".to_string(), dsl::DslValue::object([("objectWeights".to_string(), dsl::ToValue::to_value(&envelope.runtime.object_kind_weights)), ("vortexWeights".to_string(), dsl::ToValue::to_value(&envelope.runtime.vortex_kind_weights))])),
    ])
}

/// 🧊️ Scales the unit box fallback (`mesh_from_kind` extent 1.0) past the collision engine's minimum
/// brush mesh extent (2.0), otherwise its registration is a silent no-operation and brush candidates
/// never populate before a real GLB arrives.
const PUZZLE3D_FALLBACK_MESH_SCALE: f32 = 4.0;

fn scaled_mesh_positions(positions: &[f32], scale: f32) -> Vec<f32> {
    positions.iter().map(|value| value * scale).collect()
}

/// 🧊️ Pushes the current scene into the precompute session. The engine compares the decoded scene
/// structurally and no-operates on a resync, so this is the one call a caller makes per scene — and it
/// is deliberately the LAST half of a sync: installing a mesh identity rebuilds the fill preparation
/// and the brush queue against whatever scene is installed, so seeding first and pushing once costs one
/// rebuild instead of one per mesh (a measured 0.6–4.4 ms each, twelve of them on Nakagin).
pub fn sync_precompute_scene(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    if let Some(scene) = scene_config(envelope) {
        push_precompute_scene(session, scene);
    }
}

/// 🧊️ Installs an already-built engine scene. Split from `scene_config` so a staged caller pays the
/// document-shaped translation (a measured 1.3 ms on Nakagin) and the engine's own scene install (0.8 ms,
/// it rebuilds the fill preparation and re-arms the brush broad phase) in two separate bounded turns.
pub fn push_precompute_scene(session: &mut Puzzle3dPrecomputeSession, scene: crate::standards::v1::subsets::any::schema::SceneConfig) {
    let _ = session.dispatch(Puzzle3dEngineCommand::SetScene { scene });
}

/// 🥽️ Seeds the scaled box fallback for ONE mesh identity the session holds no geometry for yet, so
/// a real GLB registered earlier via `registerBrushMesh` survives every resync. Answers whether it
/// registered one, i.e. whether the caller should come back for the next; one registration costs a
/// measured 0.6 ms and the 180-object Nakagin document carries twelve of them, so a staged caller
/// spends one bounded turn on each instead of 8 ms on all of them at once.
pub fn seed_one_precompute_mesh_fallback(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) -> bool {
    let fallback = mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND);
    let fallback_positions = scaled_mesh_positions(&fallback.positions, PUZZLE3D_FALLBACK_MESH_SCALE);
    if !session.has_mesh(PUZZLE3D_FALLBACK_MESH_KIND) {
        session.register_mesh_fallback(PUZZLE3D_FALLBACK_MESH_KIND, &fallback_positions, &fallback.indices);
        return true;
    }
    let Some(url) = collect_mesh_urls(&envelope.fixture).into_iter().find(|url| !session.has_mesh(url)) else {
        return false;
    };
    session.register_mesh_fallback(&url, &fallback_positions, &fallback.indices);
    true
}

/// 🧊️ The whole sync, for the callers that are not themselves step-bounded (render, the restored
/// session, the brush lane driver): every owed mesh fallback, then the scene.
pub fn sync_precompute_session(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    while seed_one_precompute_mesh_fallback(session, envelope) {}
    sync_precompute_scene(session, envelope);
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
    session.precompute_step_lane(crate::standards::v1::subsets::any::schema::PrecomputeLane::Brush, 8);
}

/// 🎯️ `dispatch`'s `Fixture` outcome is the precompute schema's own typed fixture shape, distinct from
/// this app's `Puzzle3dFixture` document model — bridged through one JSON round trip (schema translation
/// between two independently-evolved Rust types) exactly like `scene_config_json` bridges the other
/// direction.
pub fn fixture_from_engine_fixture(envelope: &Puzzle3dScene, fixture: &crate::standards::v1::subsets::any::schema::Fixture) -> Option<Puzzle3dScene> {
    let parsed = dsl::ToValue::to_value(fixture);
    let mut next = envelope.clone();
    next.fixture.objects = dsl::FromValue::from_value(parsed.get("objects")?.clone()).ok()?;
    next.fixture.attractions = parsed.get("attractions").and_then(|v| dsl::FromValue::from_value(v.clone()).ok()).unwrap_or_default();
    next.fixture.target_volumes = parsed.get("targetVolumes").and_then(|v| dsl::FromValue::from_value(v.clone()).ok()).unwrap_or_default();
    Some(next)
}

#[derive(Clone, value_derive::FromValue)]
struct Puzzle3dFillDisplayPayload {
    #[value(default)]
    objects: Vec<Puzzle3dObject>,
    #[value(default)]
    attractions: Vec<Puzzle3dAttraction>,
}

#[derive(Clone)]
struct FillDisplayMemo {
    plan_count: u32,
    available_count: u32,
    applied_count: u32,
    payload: Puzzle3dFillDisplayPayload,
}

fn fill_display_payload_from_fixture(fixture: &crate::standards::v1::subsets::any::schema::Fixture) -> Option<Puzzle3dFillDisplayPayload> {
    dsl::FromValue::from_value(dsl::ToValue::to_value(fixture)).ok()
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
                on_change: puzzle3d_action("setVortexKindWeight", Some(json!({ "kindId": vortex_kind_id.as_str(), "objectKindId": object_kind_id }))),
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
                on_change: Some(puzzle3d_action("setObjectKindWeight", Some(json!({ "kindId": object_kind_id.as_str() })))),
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

/// 🪟️ One per-window option repaints the world body it configures, the measures rail that binds every
/// one of its controls, and the Settings panel — which mirrors four of the same fields and must never
/// disagree with the rail about them. Nothing else: not the outliner, not the catalogue, not the
/// inspector, not the history, not the labels.
///
/// 🐢️ These verbs fell through the table to [`Puzzle3dScopeClass::Chrome`], i.e. `UiDirtyScope::Full`,
/// so ONE grid toggle re-rendered every window body (180 Nakagin instances), every panel body and the
/// whole label overlay before the rail's own checkbox could show the new value. Measured on the live
/// `:6013` shell (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12): 0.7 s idle, seconds under load — long
/// enough that a second click lands on a control still rendering the value the user just left.
pub fn puzzle3d_window_option_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: vec![main::BODY_KEY.to_string()],
        panel_bodies: vec![settings_panel::BODY_KEY.to_string()],
        utilities: false,
        tools: false,
        engagements: false,
        measures: true,
        labels: false,
    }
}

/// 🔍️ The panel bodies a SELECTION change invalidates: the field inspector (it renders the selected
/// entity's own fields, `panels::inspection::render`), the artifact outliner (it marks the selected
/// rows) and the framework history panel (the framework records one command row per
/// `interactionSelect`). The catalogue lists object KINDS and the settings panel reads window options
/// — neither can move when only the selection moves.
pub fn puzzle3d_selection_panel_bodies() -> Vec<String> {
    vec![inspection::BODY_KEY.to_string(), document::BODY_KEY.to_string(), FRAMEWORK_HISTORY_BODY_KEY.to_string()]
}

/// 📝️ The panel bodies a DOCUMENT edit invalidates: everything a selection change does, plus the
/// catalogue (object/vortex kind rosters are document data).
pub fn puzzle3d_document_panel_bodies() -> Vec<String> {
    let mut bodies = puzzle3d_selection_panel_bodies();
    bodies.push(catalogue::BODY_KEY.to_string());
    bodies
}

/// 🕹️ The scope a SELECTION change invalidates: the world body every window renders the marks into,
/// [`puzzle3d_selection_panel_bodies`] (inspector + outliner rows + history) and the window measures
/// whose Select controls bind the active mode/granularity. Never the catalogue, never the settings
/// panel, never the utilities/tools/engagements rails, never the label overlay.
pub fn puzzle3d_selection_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: puzzle3d_selection_panel_bodies(), utilities: false, tools: false, engagements: false, measures: true, labels: false }
}

/// 🪜️ Switching a domain's selection MODE or GRANULARITY moves no selection and no document — it moves
/// the window's own Select chrome (the measures rail binds both controls) and what the world body
/// highlights at the new level of detail. No panel renders either field.
pub fn puzzle3d_interaction_chrome_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: true, labels: false }
}

/// @emoji 🐢️ What class of shell state one declared command invalidates — the ONE place this app
/// decides a `UiDirtyScope`, next to the command catalogue whose ids it keys on.
///
/// 🧯️ Why this is a table and not a default: `dispatch_step` used to start every arm at
/// [`UiDirtyScope::Full`] and let four viewport verbs opt out, so the only alternatives an author had
/// were "repaint the entire shell" or "hand-write a Partial" — and every hand-written Partial in this
/// app carried `panel_bodies: Vec::new()`, which is correct for a fill tick and silently wrong for
/// anything that moves the selection or the document. The class says WHAT changed; the scope is
/// derived from it exactly once, so a command can never name a window body and forget its panels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle3dScopeClass {
    /// 🈳️ Paints nothing (a host-only upload, an idle tick, a refused command).
    Quiet,
    /// 🎥️ Camera/projection only — the world body, no panels, no chrome.
    Viewport,
    /// 🪣️ Background fill planning: world body + fill-tool measures.
    FillBuild,
    /// ⚖️ Fill/distribution slider gestures: world body + fill-tool measures + window measures.
    FillOptions,
    /// ⏱️ Suggestion collision ticking: world body only.
    SuggestionsTick,
    /// 🪣️ Committing a planned fill: a DOCUMENT edit that also moves the fill-count slider range, so
    /// it is [`Puzzle3dScopeClass::Document`] plus the fill tool's own measures.
    FillApply,
    /// 🪟️ One per-window option from the measures rail or the Settings panel (grid, LOD, vortex, sun,
    /// selectable kinds, projection, the four steppers). It publishes on the per-window `WindowConfig`
    /// lane ONLY: the world body re-derives from it, the rail's own controls are bound to it, and the
    /// Settings panel mirrors four of its fields — nothing else in the shell can move.
    WindowOption,
    /// 🕹️ Selection changed: world body + [`puzzle3d_selection_panel_bodies`] + window measures.
    Selection,
    /// 🎮️ One of the framework's six interaction verbs ([`InteractionVerb`]). These never reach
    /// `dispatch_step` — the framework's `dispatch_interaction_action` owns them — so this app declares
    /// them back through `ArtifactEditor::interaction_scope` instead, out of the SAME table.
    Interaction(InteractionVerb),
    /// 📝️ Document edited: world body + [`puzzle3d_document_panel_bodies`] + window measures.
    Document,
    /// 🏛️ Anything that can move the shell chrome itself — utilities, tools, engagements, labels, or
    /// a whole-document switch. The honest answer for a command whose blast radius is not narrower.
    Chrome,
}

/// 🐢️ The exact `UiDirtyScope` one [`Puzzle3dScopeClass`] stands for.
pub fn puzzle3d_scope(class: Puzzle3dScopeClass) -> UiDirtyScope {
    match class {
        Puzzle3dScopeClass::Quiet => UiDirtyScope::None,
        Puzzle3dScopeClass::Viewport => puzzle3d_viewport_scope(),
        Puzzle3dScopeClass::FillBuild => puzzle3d_fill_build_scope(),
        Puzzle3dScopeClass::FillOptions => puzzle3d_fill_options_scope(),
        Puzzle3dScopeClass::SuggestionsTick => puzzle3d_suggestions_tick_scope(),
        Puzzle3dScopeClass::WindowOption => puzzle3d_window_option_scope(),
        Puzzle3dScopeClass::Selection => puzzle3d_selection_scope(),
        Puzzle3dScopeClass::Interaction(InteractionVerb::Hover) => puzzle3d_viewport_scope(),
        Puzzle3dScopeClass::Interaction(InteractionVerb::Select | InteractionVerb::ClearSelection | InteractionVerb::SelectAll) => puzzle3d_selection_scope(),
        Puzzle3dScopeClass::Interaction(InteractionVerb::SetSelectionMode | InteractionVerb::SetGranularity) => puzzle3d_interaction_chrome_scope(),
        Puzzle3dScopeClass::Document => {
            UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: puzzle3d_document_panel_bodies(), utilities: false, tools: false, engagements: false, measures: true, labels: false }
        }
        Puzzle3dScopeClass::FillApply => {
            UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: puzzle3d_document_panel_bodies(), utilities: false, tools: true, engagements: false, measures: true, labels: false }
        }
        Puzzle3dScopeClass::Chrome => UiDirtyScope::Full,
    }
}

/// @emoji 🐢️ The scope table: one class per declared action id. An id absent from the table is
/// [`Puzzle3dScopeClass::Chrome`] — the widest, always-correct answer — so a newly declared command
/// is slow before it is wrong, and `command_scope_classes_name_the_panels_they_change` in
/// `🧪️tests/🔬️unit/🦀️.rs` is what forces every MUTATING id to earn a narrower class.
pub fn puzzle3d_command_scope_class(action: &str) -> Puzzle3dScopeClass {
    match action {
        "setCamera" | "focusSelection" => Puzzle3dScopeClass::Viewport,
        "setGridVisible" | "setGridSnapEnabled" | "setGridSpacing" | "setLodAutomatic" | "setLodDepthVariable" | "setLodManual" | "setVortexShow" | "setVortexDirection" | "toggleSun" | "setSunAzimuth" | "setSunElevation"
        | "setSunIntensity" | "setSelectableKind" | "setTransformGumballFlag" | "setProximityRadius" | "setChunkSize" | "setVoxelDims" | "setProjection" | "setProjectionParam" => Puzzle3dScopeClass::WindowOption,
        "fillBuildTick" | "cancelFillBuild" => Puzzle3dScopeClass::FillBuild,
        "setObjectKindWeight" | "setVortexKindWeight" => Puzzle3dScopeClass::FillOptions,
        "suggestionsTick" => Puzzle3dScopeClass::SuggestionsTick,
        "registerBrushMesh" | "exportFixture" | "openImportFixture" => Puzzle3dScopeClass::Quiet,
        "setActiveUtility" | "setActiveTool" => Puzzle3dScopeClass::Viewport,
        "selectSameKindSelection" => Puzzle3dScopeClass::Selection,
        "duplicateSelection" | "deleteSelection" | "translateSelection" | "rotateSelection" | "scaleSelection" | "patchInspector" | "setSelectionFlag" | "setTargetVolumeFlag" | "deleteAttraction" | "deleteTargetVolume" | "createAttraction"
        | "worldRelocate" | "relocateTargetVolume" | "addObjectKind" | "addTargetVolume" | "importFixture" => Puzzle3dScopeClass::Document,
        "setFillCount" => Puzzle3dScopeClass::FillApply,
        "setActiveExample" => Puzzle3dScopeClass::Chrome,
        _ => Puzzle3dScopeClass::Chrome,
    }
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
        #[derive(Clone, Debug, PartialEq)]
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

            /// 🪶️ Hand-written JSON bridge (see this macro's own doc comment on why `OpBinary` here
            /// is a plain JSON-bytes bridge, not a derive): reproduces serde's default externally
            /// tagged struct-variant shape (`{"VariantName": {"window_id": ..., "args": ...}}`) so
            /// `encode_op`/`decode_op` stay byte-for-byte compatible with the pre-migration wire.
            fn to_json(&self) -> Value {
                match self {
                    $(Puzzle3dCommand::$Variant { window_id, args } => object([(
                        stringify!($Variant).to_string(),
                        object([("window_id".to_string(), Value::from(window_id.clone())), ("args".to_string(), args.clone().unwrap_or(Value::Null))]),
                    )])),*
                }
            }

            fn from_json(value: &Value) -> Option<Self> {
                let entries = value.as_object()?;
                if entries.len() != 1 {
                    return None;
                }
                let (tag, payload) = entries.iter().next()?;
                let window_id = payload.get("window_id").and_then(Value::as_str).map(str::to_string);
                let args = payload.get("args").cloned().filter(|value| !value.is_null());
                match tag {
                    $(stringify!($Variant) => Some(Puzzle3dCommand::$Variant { window_id, args }),)*
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
    SetActiveExample = "setActiveExample",
    SetActiveTool = SET_ACTIVE_TOOL_ACTION_ID,
    AddObjectKind = "addObjectKind",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    ExportFixture = "exportFixture",
    ImportFixture = "importFixture",
    OpenImportFixture = "openImportFixture",
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
    SetPanelPage = "setPanelPage",
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
    CancelFillBuild = "cancelFillBuild",
    RegisterBrushMesh = "registerBrushMesh",
    WorldPointerDown = "worldPointerDown",
    // 🗣️ B1: locale/terminology used to be host-pushed `ViewState` fields with no app-level action of
    // their own; now that `ViewState` is gone from the app-facing surface, they need a real Command.
}

impl protocol::OpBinary for Puzzle3dCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &[
        "openAddObjectDialog",
        "transformBegin",
        "transformEnd",
        "translateSelection",
        "rotateSelection",
        "scaleSelection",
        "setActiveExample",
        "addObjectKind",
        "deleteSelection",
        "duplicateSelection",
        "exportFixture",
        "importFixture",
        "openImportFixture",
        "selectSameKindSelection",
        "setCamera",
        "setProjection",
        "setProjectionParam",
        "setVortexShow",
        "setVortexDirection",
        "relocateTargetVolume",
        "worldRelocate",
        "toggleSun",
        "setSunAzimuth",
        "setSunElevation",
        "setSunIntensity",
        "setLodAutomatic",
        "setLodDepthVariable",
        "setGridVisible",
        "setPanelPage",
        "setLodManual",
        "setGridSnapEnabled",
        "setGridSpacing",
        "setProximityRadius",
        "setChunkSize",
        "setSelectableKind",
        "setSelectionFlag",
        "patchInspector",
        "focusSelection",
        "engagementInput",
        "engagementSubmit",
        "engagementRepeatLast",
        "engagementAbort",
        "createAttraction",
        "deleteAttraction",
        "setTransformGumballFlag",
        "setVoxelDims",
        "addTargetVolume",
        "deleteTargetVolume",
        "setTargetVolumeFlag",
        "engagementControlSelect",
        "addBrushObject",
        "setFillCount",
        "setBrushPlacementOverlapBudget",
        "setObjectKindWeight",
        "setVortexKindWeight",
        "cycleBrushCandidate",
        "cycleBrushCandidateBack",
        "openVortexSuggestions",
        "closeVortexSuggestions",
        "hoverSuggestion",
        "acceptSuggestion",
        "suggestionsTick",
        "fillBuildTick",
        "cancelFillBuild",
        "registerBrushMesh",
        "worldPointerDown",
        // 🧭️ The two framework-injected host-configuration verbs. They are `interactiveJob: "migrated"`
        // in the manifest and `host_configuration_mutation` resolves each to one Config mutation, so
        // they need a generated tool id here for `validate_tool_job_rows` to admit
        // `Puzzle3dHostConfigurationProofs`' generic bounded proofs — without which `dispatch_action`
        // fails closed with `interactive-job.missing-factory` before the hook is ever consulted.
        "setActiveTool",
        "setActiveUtility",
    ];

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(to_string(&self.to_json()).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        let value = parse(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        Self::from_json(&value).ok_or_else(|| protocol::ProtocolError::Pack(store::PackError::Schema("unrecognized Puzzle3dCommand tag".to_string())))
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
    pub config: &'a Puzzle3dRuntime,
    pub view_state: Option<&'a semio_framework_plugin::ViewModel>,
    /// 🕹️ Read-only view of the framework-owned `vortex` interaction domain (current selection plus
    /// `"pointer"` hover — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). Retained
    /// selection-acting verbs (delete/duplicate/focus/rotate/scale/translate-selection,
    /// select-same-kind, set-selection-flag, engagement-control-select) read it through the
    /// `selected_*_ids` helpers below instead of the deleted `Puzzle3dConfig` selection fields.
    pub interaction: &'a Puzzle3dInteractionSnapshot,
    pub ui_scope: &'a mut UiDirtyScope,
    pub effects: &'a mut Vec<Effect>,
    /// 🕹️ App-initiated selection writes this arm wants applied once its own mutations land — the
    /// single sanctioned way a reducer selects what it just created or widened to
    /// (`semio_framework_plugin::InteractionWrite`, applied by `VcsArtifactApp` through the same
    /// state machine the reserved `interactionSelect` verb uses). Empty for every arm that does not
    /// move the selection.
    pub interaction_writes: &'a mut Vec<InteractionWrite>,
    /// 🛑️ Set by an arm that must skip the whole epilogue (window save, delta, config snapshot).
    pub abort: bool,
}

impl<'a> Puzzle3dActionCtx<'a> {
    pub fn selected_object_ids(&self) -> Vec<String> {
        self.interaction.selected_object_ids().to_vec()
    }
    pub fn selected_vortex_ids(&self) -> Vec<String> {
        self.interaction.selected_vortex_ids().to_vec()
    }
    pub fn selected_attraction_ids(&self) -> Vec<String> {
        self.interaction.selected_attraction_ids().to_vec()
    }
    pub fn selected_target_volume_ids(&self) -> Vec<String> {
        self.interaction.selected_target_volume_ids().to_vec()
    }
    pub fn selected_reference_ids(&self) -> Vec<String> {
        self.interaction.selected_reference_ids().to_vec()
    }

    /// 🕹️ Replaces this app's whole `vortex` selection with `ids` at `granularity` once the action's
    /// own mutations have landed — "select the thing I just created/widened to".
    pub fn replace_selection(&mut self, granularity: &str, ids: impl IntoIterator<Item = String>) {
        let write = InteractionWrite::replace(PUZZLE3D_INTERACTION_DOMAIN, granularity, ids);
        if !write.targets.is_empty() {
            self.interaction_writes.push(write);
        }
    }

    /// 🧹️ Empties this app's whole `vortex` selection through the same sanctioned reducer channel
    /// [`Self::replace_selection`] uses. It is expressed as a `Subtractive` write naming exactly what
    /// is selected right now, NOT an empty `Replace`: the framework's state machine returns the
    /// current selection unchanged when a write names no target at all (`protocol::next_selection`),
    /// so an empty `Replace` is a silent no-op. Nothing selected means nothing to clear.
    pub fn clear_selection(&mut self) {
        let targets: Vec<InteractionTarget> = [
            (PUZZLE3D_GRANULARITY_OBJECT, self.selected_object_ids()),
            (PUZZLE3D_GRANULARITY_VORTEX, self.selected_vortex_ids()),
            (PUZZLE3D_GRANULARITY_ATTRACTION, self.selected_attraction_ids()),
            (PUZZLE3D_GRANULARITY_TARGET_VOLUME, self.selected_target_volume_ids()),
            (PUZZLE3D_GRANULARITY_REFERENCE, self.selected_reference_ids()),
        ]
        .into_iter()
        .flat_map(|(granularity, ids)| ids.into_iter().map(move |id| InteractionTarget { granularity: granularity.to_string(), id }))
        .collect();
        if targets.is_empty() {
            return;
        }
        self.interaction_writes.push(InteractionWrite { domain: PUZZLE3D_INTERACTION_DOMAIN.into(), targets, merge: MergeMode::Subtractive });
    }

    /// 🧯️ Surfaces ONE bounded, localized notice through the shell's transient-notice channel
    /// (`Effect::Notify` → `ShellHost`'s `showTransientNotice`) — how an arm whose engine work was
    /// REJECTED tells the user, instead of the `if let Ok(Fixture(_))` silence
    /// `addBrushObject`/`acceptSuggestion` used to fall through (`📓️2026-09-09-user-feature-
    /// checklist.md` §9/§13). At most one notice per action: a second call replaces nothing and adds
    /// nothing, so the effect list stays fixed-width. An unauthored locale×terminology axis carries
    /// the app's own `ui.localization.unsupported` code rather than an English sentence — this UI has
    /// no default language, and a silent drop would restore exactly the defect being fixed.
    /// 🎯️ Refuses a selection-scoped command that has nothing to act on: ONE localized notice, then the
    /// same `abort` `select_same_kind` already uses, so no empty document edit and no empty interaction
    /// write is emitted. Answers whether it refused, so an arm reads
    /// `if ctx.refuse_without_selection(&ids) { return }`.
    ///
    /// 🕹️ Why this exists: every one of these arms used to run to completion over an EMPTY id list —
    /// `duplicate_selection` cloned nothing and then called `replace_selection` with no targets,
    /// `translate/rotate/scale_selection` moved nothing, `delete_selection` retained everything. The
    /// emitted delta was empty, so the framework logged no command row and the outliner never changed:
    /// in the browser "Duplicate Selection" was indistinguishable from a dead menu row (measured
    /// 2026-09-09 21:05, no notice of any kind). A refusal must be visible.
    pub fn refuse_without_selection(&mut self, ids: &[String]) -> bool {
        if !ids.is_empty() {
            return false;
        }
        self.notice(|labels| labels.nothing_selected.as_str());
        self.abort = true;
        true
    }

    pub fn refuse_when_locked(&mut self) -> bool {
        self.notice(|labels| labels.selection_locked.as_str());
        self.abort = true;
        true
    }

    pub fn notice(&mut self, message: impl Fn(&Puzzle3dLabels) -> &'static str) {
        if self.effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })) {
            return;
        }
        let text = self.view_state.and_then(puzzle3d_labels).map_or_else(|| PUZZLE3D_LOCALIZATION_UNSUPPORTED.to_string(), |labels| message(labels).to_string());
        self.effects.push(Effect::Notify { message: text });
    }
}
/// 🏷️ Admits dynamic puzzle labels into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref().to_string()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d label admission failed"))
}

/// 🌳️ Admits fallibly assembled puzzle nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<BuiltNode>> {
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
        args: args.map(|value| json::to_dsl_value(&value)),
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
    fn bucket(&mut self, domain: &str) -> Option<&mut Vec<String>> {
        match domain {
            "node" | PUZZLE3D_GRANULARITY_OBJECT => Some(&mut self.object_ids),
            PUZZLE3D_GRANULARITY_VORTEX => Some(&mut self.vortex_ids),
            PUZZLE3D_GRANULARITY_ATTRACTION => Some(&mut self.attraction_ids),
            PUZZLE3D_GRANULARITY_TARGET_VOLUME => Some(&mut self.target_volume_ids),
            PUZZLE3D_GRANULARITY_REFERENCE => Some(&mut self.reference_ids),
            _ => None,
        }
    }

    /// 🎯️ `surface.hits` is the entity the pointer is actually over, `surface.selection` the entities the
    /// document already holds selected. The hit WINS when it names a granularity the selection does not
    /// carry, so a right-click on an unselected object opens that object's menu instead of the empty menu
    /// that made the shell fallback take over the viewport (`📓️2026-09-11-wave-B1-battery-extension.md`
    /// §5 defect 8). A hit inside the current selection changes nothing — the whole selection stays the
    /// menu's subject, so "Delete (3 objects)" never silently narrows to the one row under the cursor.
    fn from_surface(surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Self {
        let mut out = Self::default();
        let Some(surface) = surface else {
            return out;
        };
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

    /// 🕹️ Fills in the granularities the CLIENT surface never sends. `World3dHost` only puts its
    /// painted object ids (domain `"object"`) and component ids (`"feature"`) into
    /// `ContextMenuSurfaceTarget.selection`, so a selected vortex / attraction / target volume /
    /// reference reaches a menu only through the framework-owned domain read. Per-granularity
    /// additive: whatever the surface DID supply keeps priority (a right-click on an unselected
    /// entity still targets what was clicked).
    fn fill_from_interaction(&mut self, interaction: &Puzzle3dInteractionSnapshot) {
        for (bucket, ids) in [
            (&mut self.object_ids, interaction.selected_object_ids()),
            (&mut self.vortex_ids, interaction.selected_vortex_ids()),
            (&mut self.attraction_ids, interaction.selected_attraction_ids()),
            (&mut self.target_volume_ids, interaction.selected_target_volume_ids()),
            (&mut self.reference_ids, interaction.selected_reference_ids()),
        ] {
            if bucket.is_empty() {
                bucket.extend(ids.iter().cloned());
            }
        }
    }
}

fn puzzle3d_context_menu_items(envelope: &Puzzle3dScene, selection: &Puzzle3dContextSelection, labels: &Puzzle3dLabels, registry: &semio_framework_plugin::AppActionRegistry) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::Menu;
    if !selection.object_ids.is_empty() {
        let all_hidden = envelope.fixture.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).all(|object| object.hidden);
        let all_locked = envelope.fixture.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).all(|object| object.locked);
        let count = selection.object_ids.len();
        let phrase = if count == 1 { format!("1 {}", labels.object.as_str()) } else { format!("{count} {}", labels.objects.as_str()) };
        return {
            Menu::of(registry)
                .item(puzzle3d_context_menu_row("duplicate", labels.duplicate, "copy", "duplicateSelection", None, false))
                .item(puzzle3d_context_menu_row("select-same-kind", labels.select_same_kind, "layers", "selectSameKindSelection", None, false))
                .item(puzzle3d_context_menu_row("zoom", labels.zoom_to_selection, "crosshair", "focusSelection", None, false))
                .group("hand", |m| {
                    m.item(puzzle3d_context_menu_row("hide-show", if all_hidden { labels.show } else { labels.hide }, if all_hidden { "eye" } else { "eye-off" }, "setSelectionFlag", Some(json!({ "flag": "hidden", "value": !all_hidden })), false))
                        .item(puzzle3d_context_menu_row(
                            "lock-unlock",
                            if all_locked { labels.unlock } else { labels.lock },
                            if all_locked { "lock-open" } else { "lock" },
                            "setSelectionFlag",
                            Some(json!({ "flag": "locked", "value": !all_locked })),
                            false,
                        ))
                })
                .item(puzzle3d_context_menu_row("delete", format!("{} ({phrase})", labels.delete.as_str()), "trash", "deleteSelection", None, true))
                .build()
        };
    }
    if !selection.vortex_ids.is_empty() {
        let mut menu = Menu::of(registry);
        if let [only] = selection.vortex_ids.as_slice() {
            menu = menu.item(puzzle3d_context_menu_row("suggest", labels.suggest_objects, "sparkles", "openVortexSuggestions", Some(json!({ "fullId": only.as_str() })), false));
        }
        return menu.item(puzzle3d_context_menu_row("zoom", labels.zoom_to_selection, "crosshair", "focusSelection", None, false)).item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteSelection", None, true)).build();
    }
    if let Some(id) = selection.attraction_ids.first() {
        return Menu::of(registry).item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteAttraction", Some(json!({ "id": id.as_str() })), true)).build();
    }
    if let Some(id) = selection.target_volume_ids.first() {
        let target_volume = envelope.fixture.target_volumes.iter().find(|volume| &volume.id == id);
        let hidden = target_volume.is_some_and(|volume| volume.hidden);
        let locked = target_volume.is_some_and(|volume| volume.locked);
        return {
            Menu::of(registry)
                .group("targets", |m| {
                    m.item(puzzle3d_context_menu_row(
                        "hide-show",
                        if hidden { labels.show } else { labels.hide },
                        if hidden { "eye" } else { "eye-off" },
                        "setTargetVolumeFlag",
                        Some(json!({ "id": id.as_str(), "flag": "hidden", "value": !hidden })),
                        false,
                    ))
                    .item(puzzle3d_context_menu_row(
                        "lock-unlock",
                        if locked { labels.unlock } else { labels.lock },
                        if locked { "lock-open" } else { "lock" },
                        "setTargetVolumeFlag",
                        Some(json!({ "id": id.as_str(), "flag": "locked", "value": !locked })),
                        false,
                    ))
                })
                .item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteTargetVolume", Some(json!({ "id": id.as_str() })), true))
                .build()
        };
    }
    if !selection.reference_ids.is_empty() {
        return {
            Menu::of(registry).item(puzzle3d_context_menu_row("zoom", labels.zoom_to_selection, "crosshair", "focusSelection", None, false)).item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteSelection", None, true)).build()
        };
    }
    Vec::new()
}
//#endregion 🔖️ContextMenu

//#region 🔖️PlayApp
// 🧩️ Puzzle-3d play app. Owns the precompute engine; the persisted document (bare `Puzzle3dFixture`
// json) lives in the wrapping `VcsArtifactApp`'s operation store and the view state in
// `Puzzle3dConfig`. Each action rehydrates the engine from the projection, mutates a transient
// [`Puzzle3dScene`], then emits the granular operation delta.
//
// 🧲️ Gumball drags carry NO app-side session: `World3dHost` tracks the drag host-locally and
// dispatches exactly ONE absolute start→end delta (`translateSelection`/`rotateSelection`/
// `scaleSelection`) on drag end, which commits straight to the document like any other mutation.
// `transformBegin`/`transformEnd` are host-only brackets around that single tick — declared so the
// host may dispatch them, deliberately completing empty.
//#region 🎟️SessionRegistry
/// 🎟️ Document instances that may hold a live cache slot at once. Sized for real desktop use — a dozen
/// open documents across split panes, with headroom — and deliberately unrelated to
/// `FIXED_OWNER_SLOTS`, which bounds one fill-planner retirement batch and nothing else.
const PUZZLE3D_SESSION_SLOTS: usize = 64;
/// 🎯️ Slots probed for one instance id before the registry gives up or evicts, mirroring
/// `FillEnvelopeRegistry::begin_measurement`'s bounded candidate walk.
const PUZZLE3D_SESSION_PROBES: usize = 4;
/// ⚖️ Process-wide ceiling on cached session bytes. A check-in whose census would cross it is dropped
/// rather than admitted, so the next call for that instance pays exactly one cold rebuild — the
/// pre-session baseline, never a correctness change.
const PUZZLE3D_SESSION_PROCESS_BYTES: usize = 96 * 1024 * 1024;

/// 🧠 Everything one document instance keeps between two dispatches — the outliner tree included,
/// now that `BuiltNode::credited_clone` gives a memoized tree an owned read. It is boxed (like the
/// whole state), so the fixed slot row itself never carries the multi-kilobyte payload inline.
#[derive(Default)]
struct Puzzle3dSessionState {
    geometry: Option<(u64, String, String)>,
    fill_display: Option<FillDisplayMemo>,
    document_tree: Option<(Puzzle3dDocumentTreeKey, Box<BuiltNode>)>,
    collision: Option<Puzzle3dCollisionSession>,
    fill: Option<Puzzle3dFillSession>,
    brush_live_target: Option<String>,
}

impl Puzzle3dSessionState {
    fn bytes(&self) -> usize {
        self.geometry
            .as_ref()
            .map_or(0, |(_, instances, meshes)| instances.len().saturating_add(meshes.len()))
            .saturating_add(self.fill_display.as_ref().map_or(0, |_| size_of::<FillDisplayMemo>()))
            .saturating_add(self.document_tree.as_ref().map_or(0, |_| size_of::<BuiltNode>()))
            .saturating_add(self.collision.as_ref().map_or(0, Puzzle3dCollisionSession::bytes))
            .saturating_add(self.fill.as_ref().map_or(0, Puzzle3dFillSession::bytes))
            .saturating_add(self.brush_live_target.as_ref().map_or(0, String::len))
    }
}

/// 🔑️ What the outliner memo is keyed on: the fixture's geometry fingerprint AND the identity of the
/// resolved label set, because the same fixture renders different row text per locale×terminology.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Puzzle3dDocumentTreeKey {
    fingerprint: u64,
    label_set: usize,
    pages: u64,
}

impl Puzzle3dDocumentTreeKey {
    fn of(fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels, pages: &HashMap<String, u32>) -> Self {
        let mut pages_digest = 0u64;
        let mut keys: Vec<&String> = pages.keys().collect();
        keys.sort();
        for key in keys {
            for byte in key.as_bytes() {
                pages_digest = pages_digest.wrapping_mul(16777619) ^ u64::from(*byte);
            }
            pages_digest = pages_digest.wrapping_mul(16777619) ^ u64::from(pages[key]);
        }
        Self { fingerprint: main::fixture_geometry_fingerprint(fixture), label_set: std::ptr::from_ref(labels).addr(), pages: pages_digest }
    }
}

/// 🎟️ One slot of the fixed row. The cached state lives behind a pointer on purpose: the row is a
/// `[Option<Puzzle3dSessionSlot>; PUZZLE3D_SESSION_SLOTS]` built by value, so an inline payload would be
/// multiplied by every slot on the stack of whichever call first touches the registry.
struct Puzzle3dSessionSlot {
    app_instance_id: u32,
    document_id: Option<String>,
    bytes: usize,
    touched: u64,
    state: Option<Box<Puzzle3dSessionState>>,
}

/// 🎫 What a caller holds while a slot's state is checked out. The slot's own generation plus the
/// instance id make it ABA-proof: a slot that was retired, evicted or re-keyed rejects the lease
/// instead of adopting a cache built against a different document.
#[derive(Clone, Copy)]
struct Puzzle3dSessionLease {
    slot: usize,
    app_instance_id: u32,
    generation: u64,
}

struct Puzzle3dSessionRegistry {
    slots: [Option<Puzzle3dSessionSlot>; PUZZLE3D_SESSION_SLOTS],
    generations: [u64; PUZZLE3D_SESSION_SLOTS],
    aggregate_bytes: usize,
    sequence: u64,
}

impl Default for Puzzle3dSessionRegistry {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), generations: [0; PUZZLE3D_SESSION_SLOTS], aggregate_bytes: 0, sequence: 0 }
    }
}

fn puzzle3d_session_registry() -> &'static Mutex<Puzzle3dSessionRegistry> {
    static REGISTRY: OnceLock<Mutex<Puzzle3dSessionRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Puzzle3dSessionRegistry::default()))
}

impl Puzzle3dSessionRegistry {
    fn candidates(app_instance_id: u32) -> impl Iterator<Item = usize> {
        let base = usize::try_from(app_instance_id).unwrap_or(0) % PUZZLE3D_SESSION_SLOTS;
        (0..PUZZLE3D_SESSION_PROBES).map(move |offset| (base + offset) % PUZZLE3D_SESSION_SLOTS)
    }

    fn owns(&self, slot: usize, app_instance_id: u32) -> bool {
        self.slots[slot].as_ref().is_some_and(|entry| entry.app_instance_id == app_instance_id)
    }

    /// 🚪️ Drops the state a slot held and bumps its generation, so every lease taken against it fails to
    /// check in. Driven in production the moment one instance id is observed carrying a different
    /// parent document — the framework reuses instance ids, and a reused id is a new document.
    fn retire(&mut self, slot: usize) {
        if let Some(entry) = self.slots[slot].take() {
            self.aggregate_bytes = self.aggregate_bytes.saturating_sub(entry.bytes);
        }
        self.generations[slot] = self.generations[slot].saturating_add(1);
        crate::editor::puzzle3d::precompute::retire_abandoned_brush_mesh_uploads();
    }

    fn resolve_slot(&mut self, app_instance_id: u32, document_id: Option<&str>) -> Option<usize> {
        if let Some(slot) = Self::candidates(app_instance_id).find(|slot| self.owns(*slot, app_instance_id)) {
            let rekeyed = self.slots[slot].as_ref().is_some_and(|entry| entry.document_id.as_deref().zip(document_id).is_some_and(|(held, current)| held != current));
            if rekeyed {
                self.retire(slot);
            }
            return Some(slot);
        }
        if let Some(slot) = Self::candidates(app_instance_id).find(|slot| self.slots[*slot].is_none()) {
            return Some(slot);
        }
        let evicted = Self::candidates(app_instance_id).filter(|slot| self.slots[*slot].as_ref().is_some_and(|entry| entry.state.is_some())).min_by_key(|slot| self.slots[*slot].as_ref().map_or(u64::MAX, |entry| entry.touched))?;
        self.retire(evicted);
        Some(evicted)
    }

    fn check_out(&mut self, app_instance_id: u32, document_id: Option<&str>) -> Option<(Puzzle3dSessionLease, Puzzle3dSessionState)> {
        let slot = self.resolve_slot(app_instance_id, document_id)?;
        self.sequence = self.sequence.saturating_add(1);
        let previous = self.slots[slot].take();
        if let Some(entry) = &previous {
            self.aggregate_bytes = self.aggregate_bytes.saturating_sub(entry.bytes);
        }
        let held = previous.and_then(|entry| entry.state);
        let document_id = document_id.map(str::to_string);
        self.slots[slot] = Some(Puzzle3dSessionSlot { app_instance_id, document_id, bytes: 0, touched: self.sequence, state: None });
        Some((Puzzle3dSessionLease { slot, app_instance_id, generation: self.generations[slot] }, held.map_or_else(Puzzle3dSessionState::default, |state| *state)))
    }

    fn check_in(&mut self, lease: Puzzle3dSessionLease, state: Puzzle3dSessionState) {
        if self.generations[lease.slot] != lease.generation {
            return;
        }
        let bytes = state.bytes();
        if self.aggregate_bytes.checked_add(bytes).is_none_or(|total| total > PUZZLE3D_SESSION_PROCESS_BYTES) {
            return;
        }
        let Some(entry) = self.slots[lease.slot].as_mut().filter(|entry| entry.app_instance_id == lease.app_instance_id) else {
            return;
        };
        entry.bytes = bytes;
        entry.state = Some(Box::new(state));
        self.aggregate_bytes = self.aggregate_bytes.saturating_add(bytes);
    }
}

/// 🎟️ Adopts one instance's session onto a freshly built app. A contended registry, an exhausted slot
/// row or a first-ever call all resolve to "no cached state", which is exactly the pre-session
/// behaviour — this path can never make a call wrong, only cold.
fn puzzle3d_session_check_out(app_instance_id: u32, document_id: Option<&str>, app: &Puzzle3dPlayApp) -> Option<Puzzle3dSessionLease> {
    let (lease, state) = puzzle3d_session_registry().try_lock().ok()?.check_out(app_instance_id, document_id)?;
    *app.geometry_cache.lock().expect("geometry cache") = state.geometry;
    *app.fill_display_memo.lock().expect("fill display memo") = state.fill_display;
    *app.document_tree_cache.lock().expect("document cache") = state.document_tree;
    {
        let mut session = app.precompute.borrow_mut();
        if let Some(collision) = state.collision {
            session.install_collision_session(collision);
        }
        if let Some(fill) = state.fill {
            session.install_fill_session(fill);
        }
        session.set_brush_live_target(state.brush_live_target);
    }
    Some(lease)
}

/// 🎟️ Returns the app's caches to their slot. A stale lease (the slot was retired or re-keyed mid-call)
/// is rejected and the state simply dropped.
fn puzzle3d_session_check_in(lease: Puzzle3dSessionLease, app: &Puzzle3dPlayApp) {
    let (collision, fill, brush_live_target) = {
        let mut session = app.precompute.borrow_mut();
        (Some(session.take_collision_session()), Some(session.take_fill_session()), session.brush_live_target().map(str::to_string))
    };
    let state = Puzzle3dSessionState {
        geometry: app.geometry_cache.lock().expect("geometry cache").take(),
        fill_display: app.fill_display_memo.lock().expect("fill display memo").take(),
        document_tree: app.document_tree_cache.lock().expect("document cache").take(),
        collision,
        fill,
        brush_live_target,
    };
    if let Ok(mut registry) = puzzle3d_session_registry().try_lock() {
        registry.check_in(lease, state);
    }
}

/// 🪪️ The document-instance identity the framework already threads into every view — the session key.
/// A command operation carries the parent document id too, which is what lets a reused instance id
/// retire the previous document's cache instead of adopting it.
fn puzzle3d_view_session_key(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>) -> Option<(u32, Option<String>)> {
    if let Some(operation) = doc.operation_optional() {
        return Some((operation.app_instance_id, Some(operation.parent_document_id.clone())));
    }
    doc.render_operation().map(|operation| (operation.app_instance_id, None))
}
//#endregion 🎟️SessionRegistry

/// 🧠 One document instance's play session. `ArtifactApp` methods are associated fns (no `&self`), so
/// every dispatch still builds a fresh app object — but its caches are now checked out of, and back
/// into, a process-global slot keyed by `app_instance_id`, so `geometry_cache`/`fill_display_memo`/
/// registered brush meshes/the brush broad-phase index survive a call and a worker hop. The fill
/// envelope stays inside the retained session and is never copied into app or window configuration.
fn with_puzzle3d_app_for<R>(session: Option<(u32, Option<String>)>, config: &Puzzle3dRuntime, f: impl FnOnce(&Puzzle3dPlayApp) -> R) -> R {
    let app = Puzzle3dPlayApp::default();
    let lease = session.and_then(|(app_instance_id, document_id)| puzzle3d_session_check_out(app_instance_id, document_id.as_deref(), &app));
    let _ = config;
    let result = f(&app);
    if let Some(lease) = lease {
        puzzle3d_session_check_in(lease, &app);
    }
    result
}

#[cfg(test)]
pub(crate) fn with_puzzle3d_app<R>(f: impl FnOnce(&Puzzle3dPlayApp) -> R) -> R {
    let app = Puzzle3dPlayApp::default();
    f(&app)
}

#[cfg(test)]
pub(crate) fn with_puzzle3d_app_mut<R>(f: impl FnOnce(&mut Puzzle3dPlayApp) -> R) -> R {
    let mut app = Puzzle3dPlayApp::default();
    f(&mut app)
}

#[cfg(test)]
thread_local! {
    /// 🔬️ How many times `geometry_jsons` genuinely re-serialized a fixture on THIS thread. The session
    /// registry exists so that a second call on the same document does not, and the tests read this
    /// counter instead of inferring cache behaviour from timings. Thread-local so one test's measurement
    /// cannot be perturbed by another test running concurrently in the same process.
    pub(crate) static PUZZLE3D_GEOMETRY_SERIALIZATIONS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    /// 🔬️ How many times `document_tree_cached` genuinely rebuilt the outliner tree on THIS thread —
    /// the counter the memo's laws read instead of inferring cache behaviour from timings.
    pub(crate) static PUZZLE3D_DOCUMENT_TREE_BUILDS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

pub struct Puzzle3dPlayApp {
    pub(crate) precompute: std::cell::RefCell<Box<Puzzle3dPrecomputeSession>>,
    fill_display_memo: Mutex<Option<FillDisplayMemo>>,
    geometry_cache: Mutex<Option<(u64, String, String)>>,
    /// 🌳️ Boxed on purpose: `BuiltNode` is ~36 KiB by value, and this app object is built on the stack on
    /// every dispatch and every render — inline it dominated `size_of::<Puzzle3dPlayApp>()` and was the
    /// difference between a comfortable and an overflowing worker stack.
    document_tree_cache: Mutex<Option<(Puzzle3dDocumentTreeKey, Box<BuiltNode>)>>,
}

impl Default for Puzzle3dPlayApp {
    fn default() -> Self {
        Self { precompute: std::cell::RefCell::new(Box::new(Puzzle3dPrecomputeSession::new())), fill_display_memo: Mutex::new(None), geometry_cache: Mutex::new(None), document_tree_cache: Mutex::new(None) }
    }
}

impl Puzzle3dPlayApp {
    fn geometry_jsons(&self, fixture: &Puzzle3dFixture) -> (String, String) {
        let fingerprint = main::fixture_geometry_fingerprint(fixture);
        let mut cache = self.geometry_cache.lock().expect("geometry cache");
        if cache.as_ref().is_none_or(|(fp, _, _)| *fp != fingerprint) {
            #[cfg(test)]
            PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(|counter| counter.set(counter.get().saturating_add(1)));
            *cache = Some((fingerprint, main::world_instances_geometry_json(fixture), main::world_meshes_json(fixture)));
            *self.document_tree_cache.lock().expect("document cache") = None;
        }
        let (_, instances, meshes) = cache.as_ref().expect("geometry cache populated");
        (instances.clone(), meshes.clone())
    }

    /// 🌳️ The outliner body for one fixture, memoized on [`Puzzle3dDocumentTreeKey`] and carried by the
    /// session slot, so a second dispatch against an unchanged document and label set pays one
    /// `BuiltNode::credited_clone` instead of a full tree rebuild. A refused alias credit (the retirement
    /// pool or the value arena is saturated) is never fatal: the memo is simply dropped and the caller
    /// gets the freshly built tree — the pre-memo baseline.
    pub(crate) fn document_tree_cached(&self, fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
        self.document_tree_cached_from(fixture, labels, &HashMap::new())
    }

    pub(crate) fn document_tree_cached_from(&self, fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels, pages: &HashMap<String, u32>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
        let key = Puzzle3dDocumentTreeKey::of(fixture, labels, pages);
        let mut cache = self.document_tree_cache.lock().expect("document cache");
        if let Some(retained) = cache.as_ref().filter(|(cached, _)| *cached == key).and_then(|(_, node)| node.credited_clone()) {
            return Ok(retained);
        }
        #[cfg(test)]
        PUZZLE3D_DOCUMENT_TREE_BUILDS.with(|counter| counter.set(counter.get().saturating_add(1)));
        cache.take();
        let node = document::render_from(fixture, labels, pages)?;
        *cache = node.credited_clone().map(|retained| (key, Box::new(retained)));
        Ok(node)
    }

    /// 🖼️ Fixture used for world render — decoded straight from the persisted projection: a gumball
    /// drag is tracked host-locally and reaches the app as one already-absolute committed delta, so
    /// there is no in-flight app-side pose for the render to prefer.
    fn render_fixture(&self, projection: &Value) -> Puzzle3dFixture {
        puzzle3d_fixture_from_projection(projection)
    }

    /// 🧾️ Rebuilds the transient render bundle for one `(projection, config, window)` triple, with the
    /// window instance's own view-local options materialized onto the runtime.
    fn scene_for(&self, projection: &Value, config: &Puzzle3dRuntime, view_state: Option<&semio_framework_plugin::ViewModel>, window_id: &str) -> Puzzle3dScene {
        let active_utility = puzzle3d_scene_active_utility(config, view_state, Some(window_id));
        let mut scene = scene_from_projection(projection, config.clone(), &active_utility);
        main::frame_unset_camera(&mut scene, window_id);
        scene
    }

    /// @emoji 🧩️ B1: the pure per-action core, dispatched into by `ArtifactApp::handle` with
    /// `action`/`args`/`window_id` reconstructed 1:1 from the typed `Puzzle3dCommand`. Everything past
    /// this adapter boundary reads/writes the passed-in `Puzzle3dConfig` snapshot and returns a real
    /// `Emit` (document + config operations) instead of mutating `self`.
    ///
    /// ⏱️ The three halves are [`Puzzle3dActionPrologue`]'s own steps, so a staged retained work can
    /// spend one bounded turn on each instead of paying all of them in its single publish turn — see
    /// that type. This one-call shape is what the unstaged call sites (render, tool measures, the
    /// non-retained dispatch adapter) use, and is literally the three steps in a row.
    fn handle_action_impl(
        &self,
        command: &Puzzle3dCommand,
        window_id: Option<&str>,
        snapshot: &Puzzle3dPlaySnapshot,
        config: &Puzzle3dRuntime,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        interaction: &Puzzle3dInteractionSnapshot,
    ) -> Puzzle3dActionEmission {
        let action = command.action_id();
        if action == "openImportFixture" {
            eprintln!("[DEBUG] puzzle3d.openImport.enter action={action} window={window_id:?} payload=shell-only");
        }
        if let Some(shell) = puzzle3d_shell_only_emit(action) {
            if action == "openImportFixture" {
                eprintln!("[DEBUG] puzzle3d.openImport.exit action={action} path=shell-only effects=1");
            }
            return shell;
        }
        let mut prologue = Puzzle3dActionPrologue::default();
        prologue.scene_step(action, snapshot, config, view_state, window_id);
        let mut sync_turns = 0_u32;
        while prologue.sync_step(self, action, config, view_state, window_id) {
            sync_turns += 1;
            eprintln!("[DEBUG] puzzle3d.prologue.sync action={action} turn={sync_turns} stage={:?}", prologue.sync_stage);
            if action == "openImportFixture" {
                eprintln!("[DEBUG] puzzle3d.openImport.step action={action} turn={sync_turns} stage={:?}", prologue.sync_stage);
            }
            if sync_turns >= 8 {
                eprintln!("[DEBUG] puzzle3d.prologue.sync action={action} hang-point=sync-budget turns={sync_turns}");
                if action == "openImportFixture" {
                    eprintln!("[DEBUG] puzzle3d.openImport.exit action={action} hang-point=sync-budget turns={sync_turns}");
                }
                break;
            }
        }
        let emission = prologue.dispatch_step(self, command, window_id, config, view_state, interaction);
        if action == "openImportFixture" {
            eprintln!("[DEBUG] puzzle3d.openImport.exit action={action} path=dispatch sync_turns={sync_turns} effects={}", emission.0.effects.len());
        }
        emission
    }
}

/// 📬️ Persisted and ephemeral effects produced by one Puzzle 3D action.
type Puzzle3dActionEmission = (Emit<Puzzle3dMutation, Puzzle3dConfigMutation>, EphemeralEmit<EditorApp<Puzzle3dPlayApp>>);

//#region 🧾️ActionPrologue
/// 🧊️ The session sync's own three phases, in the order that costs ONE engine rebuild instead of
/// one per mesh: seed every owed collision-mesh fallback, build the engine scene, push it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Puzzle3dPrologueSyncStage {
    #[default]
    Meshes,
    Build,
    Push,
    Done,
}

/// 🧾️ The prologue every puzzle3d action shares, split into the three halves a staged retained work
/// spends bounded turns on: `scene_step` materializes the transient scene (and, only for a
/// document-intent action, the `before` projection the semantic delta is taken against), `sync_step`
/// pushes that scene into the precompute session across its own phases, and `dispatch_step` runs the
/// action arm and assembles the `Emit`.
///
/// ⏱️ Ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3. Run whole, this prologue measured 17.6 ms on the
/// 180-object Nakagin document — twice the framework's 8 000 µs interactive step ceiling, inside the
/// one turn every precompute-gated command publishes from. Splitting it is what keeps each half inside
/// that ceiling; making each half typed (`scene_from_snapshot`, `scene_config`,
/// `Puzzle3dKindMeshIndex`) is what keeps every one of them under this artifact's own 2 000 µs budget.
///
/// 🔄️ `runtime`/`active_utility` are refreshed from the caller's own `config` at the top of every step,
/// so a staged work that spends many turns here reads exactly the configuration the framework handed it
/// that turn — only the expensive document-shaped halves are carried across turns.
#[derive(Default)]
pub(crate) struct Puzzle3dActionPrologue {
    scene: Option<Puzzle3dScene>,
    before: Option<Value>,
    sync_stage: Puzzle3dPrologueSyncStage,
    built: Option<crate::standards::v1::subsets::any::schema::SceneConfig>,
}

impl Puzzle3dActionPrologue {
    /// 🧾️ Half one: the transient scene, straight off the snapshot's typed authority. The persisted
    /// projection `Value` is materialized ONLY for a document-intent action, which is its one reader
    /// (`puzzle3d_operations_from_fixture_change`'s `before`).
    pub(crate) fn scene_step(&mut self, action: &str, snapshot: &Puzzle3dPlaySnapshot, config: &Puzzle3dRuntime, view_state: Option<&semio_framework_plugin::ViewModel>, window_id: Option<&str>) {
        let active_utility = puzzle3d_scene_active_utility(config, view_state, window_id);
        let map_hit = window_id.and_then(|wid| view_state.and_then(|view| puzzle3d_utility_map_hit(view, wid))).is_some();
        eprintln!("[DEBUG] puzzle3d.utility.publish action={action} window={window_id:?} utility={active_utility} map_hit={map_hit}");
        self.before = puzzle3d_action_document_intent(action).then(|| puzzle3d_projection_value(snapshot.value()));
        self.scene = Some(scene_from_snapshot(snapshot.typed(), config.clone(), &active_utility));
    }

    /// 🧊️ Half two: the precompute session sync, for the actions that read it. The ~forty session-less
    /// actions never touch this half at all. RESUMABLE — exactly one owed collision-mesh fallback per
    /// call, then the engine scene build, then its push; answers whether another call is still owed, so a
    /// staged work spends one bounded turn on each instead of 15 ms on all of them at once.
    pub(crate) fn sync_step(&mut self, app: &Puzzle3dPlayApp, action: &str, config: &Puzzle3dRuntime, view_state: Option<&semio_framework_plugin::ViewModel>, window_id: Option<&str>) -> bool {
        if !puzzle3d_action_uses_precompute(action) {
            return false;
        }
        let applied_count = config.fill_count;
        let stage = match self.sync_stage {
            Puzzle3dPrologueSyncStage::Done => return false,
            Puzzle3dPrologueSyncStage::Meshes => {
                let Some(scene) = self.refreshed(config, view_state, window_id) else {
                    return false;
                };
                let owed = seed_one_precompute_mesh_fallback(&mut app.precompute.borrow_mut(), scene);
                if owed {
                    Puzzle3dPrologueSyncStage::Meshes
                } else {
                    Puzzle3dPrologueSyncStage::Build
                }
            }
            Puzzle3dPrologueSyncStage::Build => {
                self.built = self.refreshed(config, view_state, window_id).and_then(scene_config);
                Puzzle3dPrologueSyncStage::Push
            }
            Puzzle3dPrologueSyncStage::Push => {
                let mut precompute = app.precompute.borrow_mut();
                if let Some(built) = self.built.take() {
                    push_precompute_scene(&mut precompute, built);
                }
                precompute.set_fill_applied_count(applied_count);
                Puzzle3dPrologueSyncStage::Done
            }
        };
        self.sync_stage = stage;
        stage != Puzzle3dPrologueSyncStage::Done
    }

    /// 🎬️ Half three: the action arm itself plus the whole `Emit` assembly — the only half that reads
    /// `args`, mutates the scene and diffs the document.
    pub(crate) fn dispatch_step(
        &mut self,
        app: &Puzzle3dPlayApp,
        command: &Puzzle3dCommand,
        window_id: Option<&str>,
        config: &Puzzle3dRuntime,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        interaction: &Puzzle3dInteractionSnapshot,
    ) -> Puzzle3dActionEmission {
        let action = command.action_id();
        let args = command.args();
        let active_utility_initial = puzzle3d_scene_active_utility(config, view_state, window_id);
        if self.refreshed(config, view_state, window_id).is_none() {
            return (Emit::default(), EphemeralEmit::default());
        }
        let before = self.before.take();
        let Some(mut scene) = self.scene.take() else {
            return (Emit::default(), EphemeralEmit::default());
        };
        let shared_before = window_ownership::shared(config);
        let window_before = window_ownership::Puzzle3dWindowConfig::from_runtime(config);
        let transient_before = window_ownership::transient(config, view_state);
        // 🪟️ This action targets the one exact window owner already composed into `config`.
        let wid = window_id.map_or_else(|| main::WINDOW_KIND_ID.into(), str::to_string);
        let mut ui_scope = puzzle3d_scope(puzzle3d_command_scope_class(action));
        let mut effects = Vec::new();
        let mut interaction_writes = Vec::new();
        let mut ctx = Puzzle3dActionCtx { app, scene: &mut scene, window_id: &wid, config, view_state, interaction, ui_scope: &mut ui_scope, effects: &mut effects, interaction_writes: &mut interaction_writes, abort: false };
        dispatch_puzzle3d_action(&mut ctx, action, args);
        let aborted = ctx.abort;
        if aborted {
            // 🧯️ An aborted arm emits no document/config delta — but the refusal NOTICE it pushed is the
            // user-visible half of that refusal and must survive, or "refused" and "silently did nothing"
            // look identical (exactly how `duplicateSelection` presented in the browser). Effects travel
            // on their own lane, so keeping them costs no mutation and no undo entry; the scope stays
            // `None` because nothing was painted.
            return (Emit { effects, ui_scope: UiDirtyScope::None, ..Default::default() }, EphemeralEmit::default());
        }
        let next_active_utility = scene.active_utility.clone();
        let operations = if let Some(before) = before.as_ref() {
            puzzle3d_operations_from_fixture_change(before, &scene.fixture)
        } else {
            debug_assert!(!puzzle3d_action_document_intent(action));
            Vec::new()
        };
        if action == "importFixture" {
            eprintln!("[DEBUG] puzzle3d.import.apply ops={} after_objects={}", operations.len(), scene.fixture.objects.len());
        }
        let coalesce_key = match action {
            "translateSelection" => Some("gumball-translate".to_string()),
            "rotateSelection" => Some("gumball-rotate".to_string()),
            "scaleSelection" => Some("gumball-scale".to_string()),
            "setFillCount" => Some("fill-count".to_string()),
            _ => None,
        };
        // 🧰️🛠️ Programmatic utility/tool switches push the host session. `setActiveTool` itself never
        // re-emits. Entering fill emits `SetActiveTool { fill }` only. Leaving fill is exclusively a
        // host `setActiveTool ""` — an empty tool effect here bounce-disarms a just-armed fill (Escape
        // `engagementAbort`, mid-flow utility-field rewrite). A real utility change still emits
        // `SetActiveUtility` (host mutual exclusion clears the tool when the utility is non-empty).
        let is_direct_tool_switch = action == SET_ACTIVE_TOOL_ACTION_ID;
        let is_direct_utility_switch = action == SET_ACTIVE_UTILITY_ACTION_ID;
        let initial_is_fill_tool = active_utility_initial == fill_tool::TOOL_ID;
        let next_is_fill_tool = next_active_utility == fill_tool::TOOL_ID;
        if !is_direct_tool_switch && next_is_fill_tool && !initial_is_fill_tool {
            effects.push(Effect::SetActiveTool { tool_id: fill_tool::TOOL_ID.into() });
        }
        if !is_direct_utility_switch && !is_direct_tool_switch && !next_is_fill_tool && next_active_utility != active_utility_initial {
            effects.push(Effect::SetActiveUtility { window_id: wid.clone(), utility_id: next_active_utility });
        }
        // 🧮️ B1: only a REAL config change becomes a `Puzzle3dConfigMutation` — `PartialEq` (derived)
        // makes this cheap, and keeps a pure read-only action (e.g. a re-materialize/re-save of an
        // already-idle window's options) from creating a no-op undo entry.
        let shared_after = window_ownership::shared(&scene.runtime);
        let config_mutations = if shared_after != shared_before { vec![Puzzle3dConfigMutation::Snapshot { config: shared_after }] } else { Vec::new() };
        let window_after = window_ownership::Puzzle3dWindowConfig::from_runtime(&scene.runtime);
        let window_config_mutations = if window_after != window_before { vec![window_ownership::addressed_config_for(&wid, window_after)] } else { Vec::new() };
        let transient_after = window_ownership::transient(&scene.runtime, view_state);
        let window_transient = if transient_after != transient_before { vec![window_ownership::addressed_transient_for(&wid, transient_after)] } else { Vec::new() };
        (Emit { artifact_mutations: operations, config_mutations, window_config_mutations, coalesce_key, effects, ui_scope, interaction_writes, ..Default::default() }, EphemeralEmit { window_transient, ..Default::default() })
    }

    /// 🔄️ Re-materializes the carried scene's view-local halves against the configuration of the turn
    /// that is about to read them.
    fn refreshed(&mut self, config: &Puzzle3dRuntime, view_state: Option<&semio_framework_plugin::ViewModel>, window_id: Option<&str>) -> Option<&Puzzle3dScene> {
        let scene = self.scene.as_mut()?;
        scene.runtime = config.clone();
        scene.active_utility = puzzle3d_scene_active_utility(config, view_state, window_id);
        Some(scene)
    }

    /// 🧹️ One retained owner per bounded grant, for a staged work's own close cursor.
    pub(crate) fn close_one(&mut self) -> bool {
        self.scene.take().is_some() || self.before.take().is_some() || self.built.take().is_some()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.scene.is_none() && self.before.is_none() && self.built.is_none()
    }

    /// ⏱️ Turns one staged run of this prologue may take: one for the scene, one per mesh identity the
    /// document seeds a collision fallback for, one that finds none owed, one to build the engine scene,
    /// one to push it, and one to dispatch. A document past this refuses on its work's own capacity
    /// guard rather than running long.
    pub(crate) const WORK_ITEMS: usize = crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS + 5;
}

/// 🗨️ The three actions with no document interaction at all, hence no scene/before/after scaffolding:
/// `openAddObjectDialog` opens the declared dialog over a glass veil, and the host-only gumball
/// brackets are pure `World3dHost` state (it owns the live drag and dispatches ONE absolute delta on
/// drag end), so there is nothing for the app to open or close.
fn puzzle3d_shell_only_emit(action: &str) -> Option<Puzzle3dActionEmission> {
    match action {
        "openAddObjectDialog" => Some((Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(120), dialog_id: "addObject".into(), args: None }), EphemeralEmit::default())),
        "openImportFixture" => Some((
            Emit {
                effects: vec![Effect::RequestFileOpen {
                    req: semio_framework_plugin::RequestId(121),
                    accept: "application/json,.json".into(),
                    read_as: Some("text".into()),
                    import_action: "importFixture".into(),
                    multiple: false,
                }],
                ui_scope: UiDirtyScope::None,
                ..Default::default()
            },
            EphemeralEmit::default(),
        )),
        "transformBegin" | "transformEnd" => Some((Emit::default(), EphemeralEmit::default())),
        _ => None,
    }
}
//#endregion 🧾️ActionPrologue

/// 🎬️ Dispatch only: every arm's behaviour lives in its `🎮️commands/<group>/🦀️.rs` free
/// function. No behaviour lives in this match.
fn dispatch_puzzle3d_action(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    match action {
        "setActiveExample" => set_active_example::set_active_example(ctx, args),
        "selectSameKindSelection" => select_same_kind::select_same_kind(ctx),
        "setSelectableKind" => set_selectable_kind::set_selectable_kind(ctx, args),
        "addObjectKind" => add_object_kind::add_object_kind(ctx, args),
        "deleteSelection" => delete_selection::delete_selection(ctx),
        "duplicateSelection" => duplicate_selection::duplicate_selection(ctx),
        "exportFixture" => export_fixture::export_fixture(ctx),
        "importFixture" => import_fixture::import_fixture(ctx, args),
        "openImportFixture" => open_import_fixture::open_import_fixture(ctx),
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
        "setPanelPage" => set_panel_page::set_panel_page(ctx, args),
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
        "cancelFillBuild" => fill_build_tick::cancel_fill_build(ctx, args),
        "setObjectKindWeight" | "setVortexKindWeight" => set_kind_weight::set_kind_weight(ctx, action, args),
        "engagementInput" => engagement_input::engagement_input(ctx, args),
        "engagementSubmit" => engagement_submit::engagement_submit(ctx, args),
        "engagementRepeatLast" => engagement_repeat_last::engagement_repeat_last(ctx),
        "engagementAbort" => engagement_abort::engagement_abort(ctx),
        "worldPointerDown" => {}
        _ => {}
    }
}

/// 🧊️ Whether an action's arm READS the precompute session's synced view of the scene, and therefore
/// owes [`Puzzle3dCommandPrologue::sync_step`]'s three stages — one owed mesh fallback per turn, the
/// engine scene build, its push — before it runs.
///
/// 🥽️ `registerBrushMesh` is deliberately NOT one of them, even though it is the action that touches
/// the session most directly: it only ever WRITES geometry into it. Syncing first was not merely dead
/// work, it was work the same command then threw away — the sync seeds a scaled-box fallback for every
/// mesh id the session holds no geometry for (exactly the ids a `registerBrushMesh` run is about to
/// supply) and each seeding clears the brush cache and rebuilds the queue, then the arm's own install
/// clears them again. Seven serialized announcements each paid a whole document scene build plus that
/// rebuild, which is where the 0.65 s → 2.85 s per call measured on 2026-09-09 lived — the arm itself
/// is two `HashMap` lookups. The next scene-reading action re-syncs, so nothing downstream is stale.
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
            | "setFillCount"
            | "fillBuildTick"
            | "cancelFillBuild"
            | "setObjectKindWeight"
            | "setVortexKindWeight"
            | "engagementRepeatLast"
    )
}

//#region 🧵️RetainedCommands
/// 🧯️ ONE bounded, localized notice as a terminal `Emit` — how a retained placement work whose gesture
/// produced nothing tells the user WHY, instead of the bare `Emit::default()` every refusal path used
/// to complete with (`📓️2026-09-09-user-feature-checklist.md` §9/§13: "a real placement error produces
/// no visible feedback even though the menu correctly closes"). The message is resolved against the
/// host's declared locale×terminology axes; an axis this app never authored carries
/// [`PUZZLE3D_LOCALIZATION_UNSUPPORTED`] rather than an English sentence, because this UI has no
/// default language and a silent drop would restore the very defect being fixed.
///
/// 🐢️ The scope is [`UiDirtyScope::None`], not `Emit::effect`'s `Default` (which is `Full`): a work
/// that refused published nothing, so there is nothing to repaint — the notice itself travels on the
/// effect lane, which the host applies before it ever consults the scope.
fn puzzle3d_notice_emit(view_state: Option<&semio_framework_plugin::ViewModel>, message: impl Fn(&Puzzle3dLabels) -> &'static str) -> Emit<Puzzle3dMutation, Puzzle3dConfigMutation> {
    let text = view_state.and_then(puzzle3d_labels).map_or_else(|| PUZZLE3D_LOCALIZATION_UNSUPPORTED.to_string(), |labels| message(labels).to_string());
    Emit { effects: vec![Effect::Notify { message: text }], ui_scope: UiDirtyScope::None, ..Default::default() }
}

pub(crate) const PUZZLE3D_RETAINED_TOOL_IDS: &[&str] = &[
    "openAddObjectDialog",
    "worldPointerDown",
    "transformBegin",
    "transformEnd",
    "setActiveExample",
    "setFillCount",
    "addTargetVolume",
    "acceptSuggestion",
    "addBrushObject",
    "addObjectKind",
    "createAttraction",
    "deleteAttraction",
    "deleteSelection",
    "deleteTargetVolume",
    "duplicateSelection",
    "exportFixture",
    "importFixture",
    "openImportFixture",
    "patchInspector",
    "rotateSelection",
    "scaleSelection",
    "setSelectionFlag",
    "setTargetVolumeFlag",
    "translateSelection",
    "worldRelocate",
    "closeVortexSuggestions",
    "cycleBrushCandidate",
    "cycleBrushCandidateBack",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementRepeatLast",
    "engagementSubmit",
    "cancelFillBuild",
    "fillBuildTick",
    "focusSelection",
    "hoverSuggestion",
    "openVortexSuggestions",
    "registerBrushMesh",
    "relocateTargetVolume",
    "selectSameKindSelection",
    "setBrushPlacementOverlapBudget",
    "setCamera",
    "setChunkSize",
    "setGridSnapEnabled",
    "setGridSpacing",
    "setGridVisible",
    "setPanelPage",
    "setLodAutomatic",
    "setLodDepthVariable",
    "setLodManual",
    "setObjectKindWeight",
    "setProjection",
    "setProjectionParam",
    "setProximityRadius",
    "setSelectableKind",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setTransformGumballFlag",
    "setVortexDirection",
    "setVortexKindWeight",
    "setVortexShow",
    "setVoxelDims",
    "suggestionsTick",
    "toggleSun",
];
const PUZZLE3D_RETAINED_PAYLOAD_SCHEMA: &str = "puzzle.3d.fixture.tool-command.v1";

fn puzzle3d_retained_extent(command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
    if matches!(command.action_id(), "addTargetVolume" | "openAddObjectDialog" | "worldPointerDown" | "transformBegin" | "transformEnd") {
        return Some(1);
    }
    let selection = interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map_or(0, |selection| selection.ids.len());
    let document = snapshot.typed();
    let document_items = match command.action_id() {
        "focusSelection" | "patchInspector" | "translateSelection" | "rotateSelection" | "scaleSelection" => document.objects.len().checked_add(document.target_volumes.len())?,
        "createAttraction" | "worldRelocate" => document.objects.len().checked_add(document.attractions.len())?,
        "addObjectKind" | "setObjectKindWeight" | "setVortexKindWeight" => document.meta.kind_catalogs.as_ref().map_or(0, |catalogs| catalogs.objects.len().saturating_add(catalogs.vortices.len())),
        _ => 1,
    };
    selection.checked_add(document_items).filter(|items| *items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
}

/// 🧵️ Session-less entry point — the shape `PuzzleCommandReducer` demands of a plain fn pointer, for
/// the retained actions that only rewrite the document and gain nothing from a warm cache.
#[expect(clippy::unnecessary_wraps, reason = "Implements the fallible PuzzleCommandReducer callback signature.")]
fn puzzle3d_retained_reduce(
    command: &Puzzle3dCommand,
    snapshot: &Puzzle3dPlaySnapshot,
    config: &Puzzle3dConfig,
    interaction: &protocol::InteractionState,
    hover: &semio_framework_plugin::app::InteractionHoverState,
    view_state: Option<&semio_framework_plugin::ViewModel>,
) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation>, Fault> {
    let runtime = window_ownership::runtime(config, &window_ownership::Puzzle3dWindowConfig::default(), &window_ownership::Puzzle3dWindowTransient::default(), view_state);
    if command.action_id() == "openAddObjectDialog" {
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(120), dialog_id: "addObject".into(), args: None }));
    }
    if command.action_id() == "worldPointerDown" {
        return Ok(Emit::default());
    }
    if command.action_id() == "addTargetVolume" {
        let Some(origin) = command.args().and_then(|args| args.get("origin")).and_then(value_as_vec3) else { return Ok(Emit::default()) };
        let grid_spacing = runtime.grid_spacing.max(0.1);
        let voxel_dims = runtime.voxel_dims;
        let snapped = [(origin[0] / grid_spacing).round() * grid_spacing, (origin[1] / grid_spacing).round() * grid_spacing, (origin[2] / grid_spacing).round() * grid_spacing];
        let scale = crate::Puzzle3dScale::Vec3([voxel_dims[0] as f64 * grid_spacing, voxel_dims[1] as f64 * grid_spacing, voxel_dims[2] as f64 * grid_spacing]);
        let id = format!("target-volume-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
        let volume = crate::Puzzle3dTargetVolume { id, origin: snapped, orientation: None, scale: Some(scale), hidden: false, locked: false };
        return Ok(Emit { artifact_mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_target_volume(volume, None)], ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("addTargetVolume")), ..Default::default() });
    }
    let snapshot_interaction = Puzzle3dInteractionSnapshot::from_state(interaction, hover);
    let window_id = puzzle3d_addressed_window_id(view_state, None, command.window_id(), &runtime.window_ids);
    Ok(with_puzzle3d_app_for(None, &runtime, |app| app.handle_action_impl(command, Some(window_id), snapshot, &runtime, view_state, &snapshot_interaction).0))
}

/// 🧾️ The shared action prologue's three halves as this work's own stages, plus the one half that is
/// NOT the prologue's: [`Puzzle3dWindowCommandStage::Warm`], where a just-opened suggestion popup
/// resolves the candidates it is going to show.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dWindowCommandStage {
    Scene,
    Sync,
    Dispatch,
    Warm,
    Complete,
    Closing,
}

/// 🖌️ Brush-lane units one [`Puzzle3dWindowCommandStage::Warm`] turn asks for. The lane bounds ITSELF
/// on `PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US` (500 µs), so this is a unit ceiling, not a time one, and one
/// warm turn can never approach the framework's 8 ms interactive step ceiling
/// (`📓️2026-09-09-wave-P3-command-prologue.md`).
const PUZZLE3D_SUGGESTION_WARM_UNITS: u32 = 8;

/// 🖌️ How many such turns a popup ALWAYS spends before it opens — a fixed count, never an
/// "until resolved" loop: `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin`
/// asserts every cold run of the same document takes the SAME bounded turns, and a wall-clock-terminated
/// warm would make that turn count a machine-load reading. A turn whose lane has nothing left to do
/// pops an empty queue and costs nothing. `openVortexSuggestions`
/// used to warm its target in exactly ONE 500 µs slice inside the dispatch turn
/// (`🎮️commands/🔓️open-vortex-suggestions/🦀️.rs`'s `refresh_brush_candidates`), which on a debug build
/// resolves zero narrow-phase candidates — the popup then rendered empty, because `render` only SYNCS
/// the precompute session and never drives its brush lane. Warming here instead keeps the work inside
/// the app's own retained stages, where every turn is separately bounded.
const PUZZLE3D_SUGGESTION_WARM_TURNS: usize = 8;

/// 🪟️ The one-action-per-window route: every tool id whose whole semantic work IS
/// [`Puzzle3dActionPrologue`] — no document scan of its own, no extra cursor. It used to run that
/// prologue whole in ONE turn, which on the 180-object Nakagin document measured 17.6 ms, twice the
/// framework's interactive step ceiling (ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3); it now spends one
/// bounded turn on the scene, one per owed mesh fallback, and one on the dispatch.
struct Puzzle3dWindowCommandWork {
    tool_id: &'static str,
    stage: Puzzle3dWindowCommandStage,
    turns: usize,
    prologue: Puzzle3dActionPrologue,
    view_state: Option<semio_framework_plugin::ViewModel>,
    window_config: Option<semio_framework_plugin::WindowConfigSnapshot>,
    window_transient: Option<semio_framework_plugin::WindowTransientSnapshot>,
    session: Option<(u32, Option<String>)>,
    ephemeral: Option<EphemeralEmit<EditorApp<Puzzle3dPlayApp>>>,
    /// 🖌️ The vortex a just-opened suggestion popup will show candidates for, and the turns already
    /// spent resolving them.
    warm_target: Option<String>,
    warm_turns: usize,
    /// 📬️ The dispatch turn's own emission, held while the popup warms so the whole gesture still
    /// publishes exactly once, on the terminal step.
    emit: Option<Emit<Puzzle3dMutation, Puzzle3dConfigMutation>>,
}

impl Puzzle3dWindowCommandWork {
    fn new(tool_id: &'static str) -> Self {
        Self { tool_id, stage: Puzzle3dWindowCommandStage::Scene, turns: 0, prologue: Puzzle3dActionPrologue::default(), view_state: None, window_config: None, window_transient: None, session: None, ephemeral: None, warm_target: None, warm_turns: 0, emit: None }
    }

    /// 🖌️ The vortex whose candidates this command's popup is about to render, taken from the command's
    /// own recorded target — the ONE action that opens a picker the user reads immediately.
    fn warm_target_of(command: &Puzzle3dCommand) -> Option<String> {
        (command.action_id() == "openVortexSuggestions").then(|| command.args().and_then(|args| args.get("fullId")).and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string))?
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dWindowCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }
    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }
    fn bind_instance(&mut self, app_instance_id: u32, document_id: &str) {
        self.session = Some((app_instance_id, Some(document_id.to_string())));
    }
    fn bind_window_owners(&mut self, config: Option<semio_framework_plugin::WindowConfigSnapshot>, transient: Option<semio_framework_plugin::WindowTransientSnapshot>) {
        self.window_config = config;
        self.window_transient = transient;
    }
    fn take_ephemeral(&mut self) -> EphemeralEmit<EditorApp<Puzzle3dPlayApp>> {
        self.ephemeral.take().unwrap_or_default()
    }
    fn extent(&self, _command: &Puzzle3dCommand, _snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        Some(1)
    }
    fn step(
        &mut self,
        command: &Puzzle3dCommand,
        snapshot: &Puzzle3dPlaySnapshot,
        config: &Puzzle3dConfig,
        interaction: &protocol::InteractionState,
        hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>>, Fault> {
        if self.turns >= Puzzle3dActionPrologue::WORK_ITEMS + PUZZLE3D_SUGGESTION_WARM_TURNS {
            return Err(Fault::from("puzzle3d-window-work-capacity"));
        }
        self.turns += 1;
        let resolved_window_id = {
            let view = self.view_state.as_ref().ok_or_else(|| Fault::from("puzzle3d-window-context-required"))?;
            let roster: Vec<String> = view.window_instances.iter().map(|instance| instance.id.clone()).collect();
            puzzle3d_addressed_window_id(Some(view), None, command.window_id(), &roster).to_string()
        };
        if let Some(view) = self.view_state.as_mut() {
            if view.window_id.as_deref() != Some(resolved_window_id.as_str()) {
                *view = view.for_window_instance(&resolved_window_id).unwrap_or_else(|| {
                    let mut next = view.clone();
                    next.window_id = Some(resolved_window_id.clone());
                    next
                });
            }
        }
        let view = self.view_state.as_ref().ok_or_else(|| Fault::from("puzzle3d-window-context-required"))?;
        let window_id = resolved_window_id.as_str();
        let window_config = window_ownership::config_from_snapshot(self.window_config.as_ref());
        let window_transient = window_ownership::transient_from_snapshot(self.window_transient.as_ref());
        let runtime = window_ownership::runtime(config, &window_config, &window_transient, Some(view));
        match self.stage {
            Puzzle3dWindowCommandStage::Scene => {
                if let Some(shell) = puzzle3d_shell_only_emit(command.action_id()) {
                    self.stage = Puzzle3dWindowCommandStage::Complete;
                    self.ephemeral = Some(shell.1);
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(shell.0));
                }
                // 📤️ `exportFixture` reads the document and publishes a download; it owns no mutation, no
                // precompute session and no placement scene, so it resolves here, from the snapshot, and
                // picks its lane by payload size: one inline effect under the guest's contiguous-request
                // ceiling, the framework's segmented-download lane above it (a 145 714 B Nakagin export
                // reached no file as an inline effect — B36 §5, B38 §2).
                if command.action_id() == "exportFixture" {
                    self.stage = Puzzle3dWindowCommandStage::Complete;
                    self.ephemeral = Some(EphemeralEmit::default());
                    return match export_fixture::puzzle3d_export_publication(&puzzle3d_fixture_from_snapshot(snapshot.typed()), &runtime.active_example_id)? {
                        export_fixture::Puzzle3dExportPublication::Inline(effect) => Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::effect(effect))),
                        export_fixture::Puzzle3dExportPublication::Segmented(download) => Ok(crate::retained_command::PuzzleCommandWorkStep::Download(download)),
                    };
                }
                self.prologue.scene_step(command.action_id(), snapshot, &runtime, Some(view), Some(window_id));
                self.stage = Puzzle3dWindowCommandStage::Sync;
                Ok(Self::progress("puzzle3d-action-scene", "Reading the document", "Dokument wird gelesen"))
            }
            Puzzle3dWindowCommandStage::Sync => {
                let owed = with_puzzle3d_app_for(self.session.clone(), &runtime, |app| self.prologue.sync_step(app, command.action_id(), &runtime, Some(view), Some(window_id)));
                if !owed {
                    self.stage = Puzzle3dWindowCommandStage::Dispatch;
                }
                Ok(Self::progress("puzzle3d-action-sync", "Preparing the placement session", "Platzierungssitzung wird vorbereitet"))
            }
            Puzzle3dWindowCommandStage::Dispatch => {
                let snapshot_interaction = Puzzle3dInteractionSnapshot::from_state(interaction, hover);
                let (emit, ephemeral) = with_puzzle3d_app_for(self.session.clone(), &runtime, |app| self.prologue.dispatch_step(app, command, Some(window_id), &runtime, Some(view), &snapshot_interaction));
                self.ephemeral = Some(ephemeral);
                self.warm_target = Self::warm_target_of(command);
                if self.warm_target.is_none() {
                    self.stage = Puzzle3dWindowCommandStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(emit));
                }
                self.emit = Some(emit);
                self.stage = Puzzle3dWindowCommandStage::Warm;
                Ok(Self::progress("puzzle3d-suggestion-warm", "Resolving placement suggestions", "Platzierungsvorschläge werden ermittelt"))
            }
            Puzzle3dWindowCommandStage::Warm => {
                if self.warm_target.is_none() {
                    return Err(Fault::from("puzzle3d-window-work-warm-target"));
                }
                with_puzzle3d_app_for(self.session.clone(), &runtime, |app| app.precompute.borrow_mut().precompute_step_lane(crate::standards::v1::subsets::any::schema::PrecomputeLane::Brush, PUZZLE3D_SUGGESTION_WARM_UNITS));
                self.warm_turns += 1;
                if self.warm_turns < PUZZLE3D_SUGGESTION_WARM_TURNS {
                    return Ok(Self::progress("puzzle3d-suggestion-warm", "Resolving placement suggestions", "Platzierungsvorschläge werden ermittelt"));
                }
                self.warm_target = None;
                self.stage = Puzzle3dWindowCommandStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(self.emit.take().unwrap_or_default()))
            }
            Puzzle3dWindowCommandStage::Complete => Err(Fault::from("puzzle3d-window-work-repeated")),
            Puzzle3dWindowCommandStage::Closing => Err(Fault::from("puzzle3d-window-work-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dWindowCommandStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.prologue.close_one() || self.view_state.take().is_some() || self.window_config.take().is_some() || self.window_transient.take().is_some() || self.session.take().is_some() || self.ephemeral.take().is_some() || self.warm_target.take().is_some() || self.emit.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle3dWindowCommandStage::Closing
            && self.prologue.is_empty()
            && self.view_state.is_none()
            && self.window_config.is_none()
            && self.window_transient.is_none()
            && self.session.is_none()
            && self.ephemeral.is_none()
            && self.warm_target.is_none()
            && self.emit.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dKindWeightStage {
    Catalog,
    Validate,
    SumOthers,
    Changed,
    Build,
    Publish,
    Complete,
    Closing,
}

struct Puzzle3dKindWeightWork {
    tool_id: &'static str,
    stage: Puzzle3dKindWeightStage,
    cursor: usize,
    ids: Vec<String>,
    result: HashMap<String, f64>,
    missing: bool,
    base_sum: f64,
    other_sum: f64,
    other_count: usize,
    changed_id: Option<String>,
    requested: f64,
    ignored: bool,
}

impl Puzzle3dKindWeightWork {
    fn new(tool_id: &'static str) -> Self {
        Self {
            tool_id,
            stage: Puzzle3dKindWeightStage::Catalog,
            cursor: 0,
            ids: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            result: HashMap::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            missing: false,
            base_sum: 0.0,
            other_sum: 0.0,
            other_count: 0,
            changed_id: None,
            requested: 1.0,
            ignored: false,
        }
    }

    fn weights<'a>(&self, config: &'a Puzzle3dConfig) -> &'a HashMap<String, f64> {
        if self.tool_id == "setObjectKindWeight" {
            &config.object_kind_weights
        } else {
            &config.vortex_kind_weights
        }
    }

    fn base_weight(&self, config: &Puzzle3dConfig, id: &str) -> f64 {
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

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dKindWeightWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let catalogs = snapshot.typed().meta.kind_catalogs.as_ref()?;
        let count = if self.tool_id == "setObjectKindWeight" { catalogs.objects.len() } else { catalogs.vortices.len() };
        let items = count.checked_mul(4)?.checked_add(3)?;
        (count <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle3dCommand,
        snapshot: &Puzzle3dPlaySnapshot,
        config: &Puzzle3dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>>, Fault> {
        match self.stage {
            Puzzle3dKindWeightStage::Catalog => {
                let catalogs = snapshot.typed().meta.kind_catalogs.as_ref().ok_or_else(|| Fault::from("puzzle3d-kind-weight-catalog-owner"))?;
                let id = if self.tool_id == "setObjectKindWeight" { catalogs.objects.get(self.cursor).map(|entry| entry.id.as_str()) } else { catalogs.vortices.get(self.cursor).map(|entry| entry.id.as_str()) };
                if let Some(id) = id {
                    if self.ids.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle3d-kind-weight-catalog-capacity"));
                    }
                    self.ids.push(id.to_string());
                    self.cursor += 1;
                    return Ok(Self::progress("puzzle3d-kind-weight-catalog", "Reading kind owner", "Artinhaber wird gelesen"));
                }
                self.changed_id = Some(command.args().and_then(|args| args.get("kindId")).and_then(Value::as_str).unwrap_or("").to_string());
                let requested = command.args().and_then(|args| args.get("value")).and_then(Value::as_f64).unwrap_or(1.0).clamp(0.0, 1.0);
                self.requested = if self.tool_id == "setVortexKindWeight" {
                    if let Some(object_kind_id) = command.args().and_then(|args| args.get("objectKindId")).and_then(Value::as_str) {
                        let object_weight = config.object_kind_weights.get(object_kind_id).copied().unwrap_or(0.0);
                        if object_weight <= f64::EPSILON {
                            self.ignored = true;
                            self.stage = Puzzle3dKindWeightStage::Publish;
                            return Ok(Self::progress("puzzle3d-kind-weight-publish", "Ignoring zero-weight child", "Kind mit Nullgewicht wird ignoriert"));
                        }
                        (requested / object_weight).clamp(0.0, 1.0)
                    } else {
                        requested
                    }
                } else {
                    requested
                };
                self.cursor = 0;
                self.stage = Puzzle3dKindWeightStage::Validate;
                Ok(Self::progress("puzzle3d-kind-weight-validate", "Validating current weights", "Aktuelle Gewichte werden geprüft"))
            }
            Puzzle3dKindWeightStage::Validate => {
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle3dKindWeightStage::SumOthers;
                    return Ok(Self::progress("puzzle3d-kind-weight-sum", "Measuring sibling weights", "Geschwistergewichte werden gemessen"));
                };
                let weights = self.weights(config);
                self.missing |= !weights.contains_key(id);
                self.base_sum += weights.get(id).copied().unwrap_or(0.0);
                self.cursor += 1;
                Ok(Self::progress("puzzle3d-kind-weight-validate", "Validating kind weight", "Artgewicht wird geprüft"))
            }
            Puzzle3dKindWeightStage::SumOthers => {
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle3dKindWeightStage::Changed;
                    return Ok(Self::progress("puzzle3d-kind-weight-changed", "Preparing changed weight", "Geändertes Gewicht wird vorbereitet"));
                };
                if self.changed_id.as_deref() != Some(id.as_str()) {
                    self.other_sum += self.base_weight(config, id);
                    self.other_count += 1;
                }
                self.cursor += 1;
                Ok(Self::progress("puzzle3d-kind-weight-sum", "Measuring sibling weight", "Geschwistergewicht wird gemessen"))
            }
            Puzzle3dKindWeightStage::Changed => {
                if self.ids.len() >= 2 {
                    let changed_id = self.changed_id.clone().ok_or_else(|| Fault::from("puzzle3d-kind-weight-changed-owner"))?;
                    self.result.insert(changed_id, self.requested);
                }
                self.stage = Puzzle3dKindWeightStage::Build;
                Ok(Self::progress("puzzle3d-kind-weight-build", "Building normalized weights", "Normalisierte Gewichte werden aufgebaut"))
            }
            Puzzle3dKindWeightStage::Build => {
                let Some(id) = self.ids.get(self.cursor).cloned() else {
                    self.stage = Puzzle3dKindWeightStage::Publish;
                    return Ok(Self::progress("puzzle3d-kind-weight-publish", "Preparing weight publication", "Gewichtsveröffentlichung wird vorbereitet"));
                };
                self.cursor += 1;
                let value = if self.ids.len() == 1 {
                    1.0
                } else if self.changed_id.as_deref() == Some(id.as_str()) {
                    return Ok(Self::progress("puzzle3d-kind-weight-build", "Keeping changed weight", "Geändertes Gewicht wird beibehalten"));
                } else {
                    let remainder = (1.0 - self.requested).max(0.0);
                    if self.other_sum <= f64::EPSILON {
                        remainder / self.other_count.max(1) as f64
                    } else {
                        self.base_weight(config, &id) / self.other_sum * remainder
                    }
                };
                self.result.insert(id, value);
                Ok(Self::progress("puzzle3d-kind-weight-build", "Building kind weight", "Artgewicht wird aufgebaut"))
            }
            Puzzle3dKindWeightStage::Publish => {
                self.stage = Puzzle3dKindWeightStage::Complete;
                if self.ignored {
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                }
                let mutation = if self.tool_id == "setObjectKindWeight" {
                    Puzzle3dConfigMutation::SetObjectKindWeights { value: std::mem::take(&mut self.result) }
                } else {
                    Puzzle3dConfigMutation::SetVortexKindWeights { value: std::mem::take(&mut self.result) }
                };
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { config_mutations: vec![mutation], ui_scope: puzzle3d_scope(puzzle3d_command_scope_class(self.tool_id)), ..Default::default() }))
            }
            Puzzle3dKindWeightStage::Complete => Err(Fault::from("puzzle3d-kind-weight-complete-repolled")),
            Puzzle3dKindWeightStage::Closing => Err(Fault::from("puzzle3d-kind-weight-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dKindWeightStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.ids.pop().is_some() || self.changed_id.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let removed = {
            let mut values = self.result.extract_if(|_, _| true);
            values.next()
        };
        if removed.is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle3dKindWeightStage::Closing && self.ids.is_empty() && self.result.is_empty() && self.changed_id.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dAddObjectKindStage {
    Decode,
    Catalog,
    Kind,
    Representation,
    Vortex,
    Publish,
    Complete,
    Closing,
}

struct Puzzle3dAddObjectKindPayload {
    kind_id: String,
    origin: [f64; 3],
}

struct Puzzle3dAddObjectKindWork {
    stage: Puzzle3dAddObjectKindStage,
    kind_cursor: usize,
    representation_cursor: usize,
    vortex_cursor: usize,
    kind_index: Option<usize>,
    payload: Option<Puzzle3dAddObjectKindPayload>,
    object_id: Option<String>,
    mesh_url: Option<String>,
    vortices: Vec<crate::Puzzle3dVortex>,
    catalog_mutation: Option<Puzzle3dMutation>,
    mutation: Option<Puzzle3dMutation>,
}

impl Default for Puzzle3dAddObjectKindWork {
    fn default() -> Self {
        Self {
            stage: Puzzle3dAddObjectKindStage::Decode,
            kind_cursor: 0,
            representation_cursor: 0,
            vortex_cursor: 0,
            kind_index: None,
            payload: None,
            object_id: None,
            mesh_url: None,
            vortices: Vec::with_capacity(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT),
            catalog_mutation: None,
            mutation: None,
        }
    }
}

impl Puzzle3dAddObjectKindWork {
    fn decode(command: &Puzzle3dCommand) -> Puzzle3dAddObjectKindPayload {
        let args = command.args();
        Puzzle3dAddObjectKindPayload {
            kind_id: args.and_then(|args| args.get("objectKind")).and_then(Value::as_str).unwrap_or("Object").to_string(),
            origin: args.and_then(|args| args.get("origin")).and_then(value_as_vec3).unwrap_or([0.0, 0.0, 0.0]),
        }
    }

    fn object_id(snapshot: &Puzzle3dPlaySnapshot, payload: &Puzzle3dAddObjectKindPayload) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        snapshot.typed().objects.len().hash(&mut hasher);
        payload.kind_id.hash(&mut hasher);
        for value in payload.origin {
            value.to_bits().hash(&mut hasher);
        }
        format!("puzzle3d.object.{:016x}", hasher.finish())
    }

    /// 🧱️ The declared default object kind an uncatalogued document materializes on its first
    /// `addObjectKind`: one catalog row carrying the command's own `objectKind` default id, with no
    /// representation and no vortex template, so the object this gesture creates references a real
    /// catalogued kind instead of a dangling one.
    fn declared_default_kind(kind_id: &str) -> crate::Puzzle3dCatalogObjectKind {
        crate::Puzzle3dCatalogObjectKind {
            id: kind_id.to_string(),
            name: kind_id.to_string(),
            label: kind_id.to_string(),
            description: String::new(),
            icon: String::new(),
            image: String::new(),
            unit: String::new(),
            is_abstract: false,
            base_kinds: Vec::new(),
            representations: Vec::new(),
            vortices: Vec::new(),
            attributes: Vec::new(),
            authors: Vec::new(),
        }
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dAddObjectKindWork {
    fn tool_id(&self) -> &'static str {
        "addObjectKind"
    }

    /// 📏️ A document with no kind catalogs is THREE turns, not a refusal and not a no-op: decode,
    /// materialize the declared default kind catalog, publish. Returning `None` here instead
    /// conflated "nothing to do" with "over capacity", and the preflight reported *"puzzle command
    /// exceeds fixed semantic work capacity"* for every `addObjectKind` after `setActiveExample ""`
    /// cleared the catalogs.
    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let Some(catalogs) = snapshot.typed().meta.kind_catalogs.as_ref() else { return Some(3) };
        let items = catalogs.objects.len().checked_add(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT.checked_mul(2)?)?.checked_add(3)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle3dCommand,
        snapshot: &Puzzle3dPlaySnapshot,
        _config: &Puzzle3dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>>, Fault> {
        let catalogs = snapshot.typed().meta.kind_catalogs.as_ref();
        match self.stage {
            Puzzle3dAddObjectKindStage::Decode => {
                let payload = Self::decode(command);
                self.object_id = Some(Self::object_id(snapshot, &payload));
                self.payload = Some(payload);
                self.stage = if catalogs.is_some() { Puzzle3dAddObjectKindStage::Kind } else { Puzzle3dAddObjectKindStage::Catalog };
                Ok(Self::progress("puzzle3d-add-kind-scan", "Finding object kind", "Objektart wird gesucht"))
            }
            Puzzle3dAddObjectKindStage::Catalog => {
                let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-add-kind-payload-owner"))?;
                let catalogs = crate::Puzzle3dKindCatalogs { objects: vec![Self::declared_default_kind(&payload.kind_id)], ..Default::default() };
                self.catalog_mutation = Some(crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs(Some(catalogs)));
                self.stage = Puzzle3dAddObjectKindStage::Publish;
                Ok(Self::progress("puzzle3d-add-kind-catalog", "Creating the default object kind", "Standard-Objektart wird angelegt"))
            }
            Puzzle3dAddObjectKindStage::Kind => {
                let catalogs = catalogs.ok_or_else(|| Fault::from("puzzle3d-add-kind-catalog-owner"))?;
                let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-add-kind-payload-owner"))?;
                let Some(kind) = catalogs.objects.get(self.kind_cursor) else {
                    self.stage = Puzzle3dAddObjectKindStage::Publish;
                    return Ok(Self::progress("puzzle3d-add-kind-publish", "Preparing default object", "Standardobjekt wird vorbereitet"));
                };
                if kind.id == payload.kind_id {
                    if kind.representations.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT || kind.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                        return Err(Fault::from("puzzle3d-add-kind-catalog-capacity"));
                    }
                    self.kind_index = Some(self.kind_cursor);
                    self.stage = Puzzle3dAddObjectKindStage::Representation;
                } else {
                    self.kind_cursor += 1;
                }
                Ok(Self::progress("puzzle3d-add-kind-scan", "Scanning object kind", "Objektart wird geprüft"))
            }
            Puzzle3dAddObjectKindStage::Representation => {
                let catalogs = catalogs.ok_or_else(|| Fault::from("puzzle3d-add-kind-catalog-owner"))?;
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-add-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-add-kind-cursor"))?;
                let Some(representation) = kind.representations.get(self.representation_cursor) else {
                    self.stage = Puzzle3dAddObjectKindStage::Vortex;
                    return Ok(Self::progress("puzzle3d-add-kind-vortex", "Preparing object vortices", "Objekt-Vortices werden vorbereitet"));
                };
                self.representation_cursor += 1;
                if self.mesh_url.is_none() && !representation.url.is_empty() {
                    self.mesh_url = Some(representation.url.clone());
                }
                Ok(Self::progress("puzzle3d-add-kind-representation", "Reading object representation", "Objektdarstellung wird gelesen"))
            }
            Puzzle3dAddObjectKindStage::Vortex => {
                let catalogs = catalogs.ok_or_else(|| Fault::from("puzzle3d-add-kind-catalog-owner"))?;
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-add-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-add-kind-cursor"))?;
                let Some(template) = kind.vortices.get(self.vortex_cursor) else {
                    self.stage = Puzzle3dAddObjectKindStage::Publish;
                    return Ok(Self::progress("puzzle3d-add-kind-publish", "Preparing object publication", "Objektveröffentlichung wird vorbereitet"));
                };
                let index = self.vortex_cursor;
                self.vortex_cursor += 1;
                self.vortices.push(crate::Puzzle3dVortex {
                    id: if template.id.is_empty() { format!("v{index}") } else { template.id.clone() },
                    label: (!template.label.is_empty()).then(|| template.label.clone()),
                    vortex_kind: template.vortex_kind.clone(),
                    position: template.point,
                    direction: Some(template.direction),
                    radius: template.radius,
                    hidden: false,
                    locked: false,
                });
                Ok(Self::progress("puzzle3d-add-kind-vortex", "Building one object vortex", "Ein Objekt-Vortex wird aufgebaut"))
            }
            Puzzle3dAddObjectKindStage::Publish => {
                let payload = self.payload.take().ok_or_else(|| Fault::from("puzzle3d-add-kind-payload-owner"))?;
                let object_id = self.object_id.take().ok_or_else(|| Fault::from("puzzle3d-add-kind-object-owner"))?;
                let object = crate::Puzzle3dObject {
                    id: object_id.clone(),
                    label: Some(payload.kind_id.clone()),
                    object_kind: Some(payload.kind_id),
                    anchor: Default::default(),
                    origin: payload.origin,
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    mesh_url: self.mesh_url.take(),
                    vortices: std::mem::take(&mut self.vortices),
                    hidden: false,
                    locked: false,
                };
                self.mutation = Some(crate::standards::v1::subsets::any::schema::mutations::create_object(object, None));
                self.stage = Puzzle3dAddObjectKindStage::Complete;
                let artifact_mutations = self.catalog_mutation.take().into_iter().chain(self.mutation.take()).collect();
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit {
                    artifact_mutations,
                    ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("addObjectKind")),
                    interaction_writes: vec![InteractionWrite::replace(PUZZLE3D_INTERACTION_DOMAIN, PUZZLE3D_GRANULARITY_OBJECT, [object_id])],
                    ..Default::default()
                }))
            }
            Puzzle3dAddObjectKindStage::Complete => Err(Fault::from("puzzle3d-add-kind-complete-repolled")),
            Puzzle3dAddObjectKindStage::Closing => Err(Fault::from("puzzle3d-add-kind-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dAddObjectKindStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.catalog_mutation.take().is_some() || self.mutation.take().is_some() || self.vortices.pop().is_some() || self.mesh_url.take().is_some() || self.object_id.take().is_some() || self.payload.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle3dAddObjectKindStage::Closing && self.catalog_mutation.is_none() && self.mutation.is_none() && self.vortices.is_empty() && self.mesh_url.is_none() && self.object_id.is_none() && self.payload.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dScaleStage {
    ObjectSelection,
    VolumeSelection,
    Objects,
    Volumes,
    Complete,
    Closing,
}

struct Puzzle3dScaleWork {
    tool_id: &'static str,
    stage: Puzzle3dScaleStage,
    selection_cursor: usize,
    object_cursor: usize,
    volume_cursor: usize,
    objects: HashSet<String>,
    volumes: HashSet<String>,
    mutations: Vec<Puzzle3dMutation>,
    /// 🗣️ The host's declared locale×terminology axes, bound by `build_tool_job` exactly like every
    /// other retained work — this work resolves its own refusal notice and has no `Puzzle3dActionCtx`
    /// to borrow one from.
    view_state: Option<semio_framework_plugin::ViewModel>,
}

impl Default for Puzzle3dScaleWork {
    fn default() -> Self {
        Self::new("scaleSelection")
    }
}

impl Puzzle3dScaleWork {
    fn new(tool_id: &'static str) -> Self {
        Self {
            tool_id,
            stage: Puzzle3dScaleStage::ObjectSelection,
            selection_cursor: 0,
            object_cursor: 0,
            volume_cursor: 0,
            objects: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            volumes: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
            view_state: None,
        }
    }
    fn explicit_ids(command: &Puzzle3dCommand) -> Option<&Vec<Value>> {
        command.args().and_then(|args| args.get("ids")).and_then(Value::as_array).filter(|ids| !ids.is_empty())
    }

    fn selection<'a>(interaction: &'a protocol::InteractionState, granularity: &str) -> Option<&'a protocol::DomainSelection> {
        interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == granularity)
    }

    fn scale(command: &Puzzle3dCommand) -> [f64; 3] {
        let axis = |key: &str| command.args().and_then(|args| args.get(key)).and_then(Value::as_f64).unwrap_or(1.0);
        [axis("sx"), axis("sy"), axis("sz")]
    }

    fn scaled(scale: Option<crate::Puzzle3dScale>, factors: [f64; 3]) -> crate::Puzzle3dScale {
        let current = match scale {
            Some(crate::Puzzle3dScale::Uniform(value)) => [value; 3],
            Some(crate::Puzzle3dScale::Vec3(value)) => value,
            None => [1.0; 3],
        };
        crate::Puzzle3dScale::Vec3([current[0] * factors[0], current[1] * factors[1], current[2] * factors[2]])
    }

    fn translated(origin: [f64; 3], command: &Puzzle3dCommand) -> [f64; 3] {
        let axis = |key: &str| command.args().and_then(|args| args.get(key)).and_then(Value::as_f64).unwrap_or_default();
        [origin[0] + axis("dx"), origin[1] + axis("dy"), origin[2] + axis("dz")]
    }

    fn rotated(orientation: Option<[f64; 4]>, command: &Puzzle3dCommand) -> [f64; 4] {
        let axis = |key: &str| command.args().and_then(|args| args.get(key)).and_then(Value::as_f64).unwrap_or_default();
        let delta = quat_from_axis_angle(axis("ax"), axis("ay"), axis("az"), axis("angle"));
        quat_mul(delta, orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]))
    }

    fn object_mutation(&self, object: &crate::Puzzle3dObject, command: &Puzzle3dCommand) -> Puzzle3dMutation {
        match self.tool_id {
            "translateSelection" => crate::standards::v1::subsets::any::schema::mutations::move_object(object.id.clone(), Self::translated(object.origin, command)),
            "rotateSelection" => crate::standards::v1::subsets::any::schema::mutations::rotate_object(object.id.clone(), Some(Self::rotated(object.orientation, command))),
            _ => crate::standards::v1::subsets::any::schema::mutations::scale_object(object.id.clone(), Some(Self::scaled(object.scale, Self::scale(command)))),
        }
    }

    fn volume_mutation(&self, volume: &crate::Puzzle3dTargetVolume, command: &Puzzle3dCommand) -> Puzzle3dMutation {
        match self.tool_id {
            "translateSelection" => crate::standards::v1::subsets::any::schema::mutations::move_target_volume(volume.id.clone(), Self::translated(volume.origin, command)),
            "rotateSelection" => crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume(volume.id.clone(), Some(Self::rotated(volume.orientation, command))),
            _ => crate::standards::v1::subsets::any::schema::mutations::scale_target_volume(volume.id.clone(), Some(Self::scaled(volume.scale, Self::scale(command)))),
        }
    }

    fn coalesce_key(&self) -> &'static str {
        match self.tool_id {
            "translateSelection" => "gumball-translate",
            "rotateSelection" => "gumball-rotate",
            _ => "gumball-scale",
        }
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dScaleWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }

    fn extent(&self, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
        let object_selection = Self::explicit_ids(command).map_or_else(|| Self::selection(interaction, PUZZLE3D_GRANULARITY_OBJECT).map_or(0, |selection| selection.ids.len()), Vec::len);
        let volume_selection = Self::selection(interaction, PUZZLE3D_GRANULARITY_TARGET_VOLUME).map_or(0, |selection| selection.ids.len());
        let items = object_selection.checked_add(volume_selection)?.checked_add(snapshot.typed().objects.len())?.checked_add(snapshot.typed().target_volumes.len())?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle3dCommand,
        snapshot: &Puzzle3dPlaySnapshot,
        _config: &Puzzle3dConfig,
        interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>>, Fault> {
        match self.stage {
            Puzzle3dScaleStage::ObjectSelection => {
                let id = if let Some(ids) = Self::explicit_ids(command) {
                    ids.get(self.selection_cursor).and_then(Value::as_str)
                } else {
                    Self::selection(interaction, PUZZLE3D_GRANULARITY_OBJECT).and_then(|selection| selection.ids.get(self.selection_cursor)).map(String::as_str)
                };
                if let Some(id) = id {
                    if self.objects.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle3d-scale-object-selection-capacity"));
                    }
                    self.objects.insert(id.to_string());
                    self.selection_cursor += 1;
                    return Ok(Self::progress("puzzle3d-scale-object-selection", "Reading selected object", "Ausgewähltes Objekt wird gelesen"));
                }
                self.selection_cursor = 0;
                self.stage = Puzzle3dScaleStage::VolumeSelection;
                Ok(Self::progress("puzzle3d-scale-volume-selection", "Reading selected volume", "Ausgewähltes Volumen wird gelesen"))
            }
            Puzzle3dScaleStage::VolumeSelection => {
                let id = Self::selection(interaction, PUZZLE3D_GRANULARITY_TARGET_VOLUME).and_then(|selection| selection.ids.get(self.selection_cursor));
                if let Some(id) = id {
                    if self.volumes.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle3d-scale-volume-selection-capacity"));
                    }
                    self.volumes.insert(id.clone());
                    self.selection_cursor += 1;
                    return Ok(Self::progress("puzzle3d-scale-volume-selection", "Reading selected volume", "Ausgewähltes Volumen wird gelesen"));
                }
                // 🧲️ Both selection cursors are exhausted, so this is the exact set the gesture will act
                // on. Acting on NOTHING is a refusal, and a refusal is visible: one localized notice, no
                // mutation, no coalesce key (a refusal must never join the gesture's latest-wins group)
                // and nothing repainted. Measured 2026-09-09 21:05 in the browser as a dead "Translate
                // Selection" row; the `refuse_without_selection` guard the `🎮️commands/*` arms carry
                // never runs for these three verbs, because `build_tool_job` routes them here instead of
                // through `dispatch_step`.
                if self.objects.is_empty() && self.volumes.is_empty() {
                    self.stage = Puzzle3dScaleStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.nothing_selected.as_str())));
                }
                self.stage = Puzzle3dScaleStage::Objects;
                Ok(Self::progress("puzzle3d-scale-object", "Scaling selected object", "Ausgewähltes Objekt wird skaliert"))
            }
            Puzzle3dScaleStage::Objects => {
                let Some(object) = snapshot.typed().objects.get(self.object_cursor) else {
                    self.stage = Puzzle3dScaleStage::Volumes;
                    return Ok(Self::progress("puzzle3d-scale-volume", "Scaling selected volume", "Ausgewähltes Volumen wird skaliert"));
                };
                self.object_cursor += 1;
                if self.objects.contains(&object.id) && !object.locked {
                    self.mutations.push(self.object_mutation(object, command));
                }
                Ok(Self::progress("puzzle3d-scale-object", "Scaling selected object", "Ausgewähltes Objekt wird skaliert"))
            }
            Puzzle3dScaleStage::Volumes => {
                let Some(volume) = snapshot.typed().target_volumes.get(self.volume_cursor) else {
                    self.stage = Puzzle3dScaleStage::Complete;
                    let mutations = std::mem::take(&mut self.mutations);
                    if mutations.is_empty() && (!self.objects.is_empty() || !self.volumes.is_empty()) {
                        return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.selection_locked.as_str())));
                    }
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: mutations, coalesce_key: Some(self.coalesce_key().to_string()), ui_scope: puzzle3d_scope(puzzle3d_command_scope_class(self.tool_id)), ..Default::default() }));
                };
                self.volume_cursor += 1;
                if self.volumes.contains(&volume.id) && !volume.locked {
                    self.mutations.push(self.volume_mutation(volume, command));
                }
                Ok(Self::progress("puzzle3d-scale-volume", "Scaling selected volume", "Ausgewähltes Volumen wird skaliert"))
            }
            Puzzle3dScaleStage::Complete => Err(Fault::from("puzzle3d-scale-complete-repolled")),
            Puzzle3dScaleStage::Closing => Err(Fault::from("puzzle3d-scale-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dScaleStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.view_state.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let object = {
            let mut objects = self.objects.extract_if(|_| true);
            objects.next()
        };
        if object.is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let volume = {
            let mut volumes = self.volumes.extract_if(|_| true);
            volumes.next()
        };
        if volume.is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle3dScaleStage::Closing && self.view_state.is_none() && self.mutations.is_empty() && self.objects.is_empty() && self.volumes.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dPatchInspectorStage {
    Selection,
    Objects,
    Vortices,
    Attractions,
    AttractionReconnect,
    References,
    Volumes,
    Complete,
    Closing,
}

struct Puzzle3dPatchInspectorWork {
    stage: Puzzle3dPatchInspectorStage,
    selection_cursor: usize,
    item_cursor: usize,
    child_cursor: usize,
    selected: HashSet<String>,
    pending_attraction: Option<crate::Puzzle3dAttraction>,
    mutations: Vec<Puzzle3dMutation>,
}

impl Default for Puzzle3dPatchInspectorWork {
    fn default() -> Self {
        Self {
            stage: Puzzle3dPatchInspectorStage::Selection,
            selection_cursor: 0,
            item_cursor: 0,
            child_cursor: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            pending_attraction: None,
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
        }
    }
}

impl Puzzle3dPatchInspectorWork {
    fn entity(command: &Puzzle3dCommand) -> &str {
        command.args().and_then(|args| args.get("entity")).and_then(Value::as_str).unwrap_or("")
    }

    fn field(command: &Puzzle3dCommand) -> &str {
        command.args().and_then(|args| args.get("field")).and_then(Value::as_str).unwrap_or("")
    }

    fn granularity(entity: &str) -> &str {
        match entity {
            "object" => PUZZLE3D_GRANULARITY_OBJECT,
            "vortex" => PUZZLE3D_GRANULARITY_VORTEX,
            "attraction" => PUZZLE3D_GRANULARITY_ATTRACTION,
            "reference" => PUZZLE3D_GRANULARITY_REFERENCE,
            "targetVolume" => PUZZLE3D_GRANULARITY_TARGET_VOLUME,
            _ => "",
        }
    }

    fn source_id<'a>(command: &'a Puzzle3dCommand, interaction: &'a protocol::InteractionState, index: usize) -> Option<&'a str> {
        if let Some(ids) = command.args().and_then(|args| args.get("ids")).and_then(Value::as_array).filter(|ids| !ids.is_empty()) {
            return ids.get(index).and_then(Value::as_str);
        }
        let granularity = Self::granularity(Self::entity(command));
        interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == granularity).and_then(|selection| selection.ids.get(index)).map(String::as_str)
    }

    fn source_len(command: &Puzzle3dCommand, interaction: &protocol::InteractionState) -> usize {
        command.args().and_then(|args| args.get("ids")).and_then(Value::as_array).filter(|ids| !ids.is_empty()).map_or_else(
            || {
                let granularity = Self::granularity(Self::entity(command));
                interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == granularity).map_or(0, |selection| selection.ids.len())
            },
            Vec::len,
        )
    }

    fn first_stage(entity: &str) -> Puzzle3dPatchInspectorStage {
        match entity {
            "object" => Puzzle3dPatchInspectorStage::Objects,
            "vortex" => Puzzle3dPatchInspectorStage::Vortices,
            "attraction" => Puzzle3dPatchInspectorStage::Attractions,
            "reference" => Puzzle3dPatchInspectorStage::References,
            "targetVolume" => Puzzle3dPatchInspectorStage::Volumes,
            _ => Puzzle3dPatchInspectorStage::Complete,
        }
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn push(&mut self, mutation: Puzzle3dMutation) -> Result<(), Fault> {
        if self.mutations.len() >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle3d-patch-inspector-output-capacity"));
        }
        self.mutations.push(mutation);
        Ok(())
    }

    fn complete(&mut self) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        self.stage = Puzzle3dPatchInspectorStage::Complete;
        crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("patchInspector")), ..Default::default() })
    }

    fn scale(value: Option<crate::Puzzle3dScale>) -> [f64; 3] {
        match value {
            Some(crate::Puzzle3dScale::Uniform(value)) => [value; 3],
            Some(crate::Puzzle3dScale::Vec3(value)) => value,
            None => [1.0; 3],
        }
    }

    fn object_mutation(command: &Puzzle3dCommand, object: &crate::Puzzle3dObject) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        match field {
            "hidden" => value.and_then(Value::as_bool).map(|value| crate::standards::v1::subsets::any::schema::mutations::change_object_hidden(object.id.clone(), value)),
            "locked" => value.and_then(Value::as_bool).map(|value| crate::standards::v1::subsets::any::schema::mutations::change_object_locked(object.id.clone(), value)),
            "label" => Some(crate::standards::v1::subsets::any::schema::mutations::edit_object_label(object.id.clone(), value.and_then(Value::as_str).map(str::to_string))),
            "objectKind" => Some(crate::standards::v1::subsets::any::schema::mutations::change_object_kind(object.id.clone(), value.and_then(Value::as_str).map(str::to_string))),
            "meshUrl" => Some(crate::standards::v1::subsets::any::schema::mutations::change_object_mesh(object.id.clone(), value.and_then(Value::as_str).map(str::to_string))),
            "origin" => value.and_then(value_as_vec3).map(|origin| crate::standards::v1::subsets::any::schema::mutations::move_object(object.id.clone(), origin)),
            _ => {
                if let Some(axis) = puzzle3d_axis_index(field, "origin") {
                    let mut origin = object.origin;
                    origin[axis] = puzzle3d_resolve_number_edit(origin[axis], value, delta)?;
                    return Some(crate::standards::v1::subsets::any::schema::mutations::move_object(object.id.clone(), origin));
                }
                if let Some(axis) = puzzle3d_axis_index(field, "scale") {
                    let mut scale = Self::scale(object.scale);
                    scale[axis] = puzzle3d_resolve_number_edit(scale[axis], value, delta)?;
                    return Some(crate::standards::v1::subsets::any::schema::mutations::scale_object(object.id.clone(), Some(crate::Puzzle3dScale::Vec3(scale))));
                }
                let axis = puzzle3d_axis_index(field, "orientation")?;
                let mut orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                orientation[axis] = puzzle3d_resolve_number_edit(orientation[axis], value, delta)?;
                Some(crate::standards::v1::subsets::any::schema::mutations::rotate_object(object.id.clone(), Some(quat_normalize(orientation))))
            }
        }
    }

    fn vortex_mutation(command: &Puzzle3dCommand, object_id: &str, vortex: &crate::Puzzle3dVortex) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        let mut next = vortex.clone();
        match field {
            "hidden" => next.hidden = value.and_then(Value::as_bool)?,
            "locked" => next.locked = value.and_then(Value::as_bool)?,
            "vortexKind" => next.vortex_kind = value.and_then(Value::as_str).map(str::to_string),
            "position" => next.position = value.and_then(value_as_vec3)?,
            "direction" => next.direction = Some(value.and_then(value_as_vec3)?),
            "radius" => next.radius = Some(puzzle3d_resolve_number_edit(next.radius.unwrap_or(0.35), value, delta)?),
            _ => {
                if let Some(axis) = puzzle3d_axis_index(field, "position") {
                    next.position[axis] = puzzle3d_resolve_number_edit(next.position[axis], value, delta)?;
                } else {
                    let axis = puzzle3d_axis_index(field, "direction")?;
                    let mut direction = next.direction.unwrap_or([0.0, 0.0, 1.0]);
                    direction[axis] = puzzle3d_resolve_number_edit(direction[axis], value, delta)?;
                    next.direction = Some(direction);
                }
            }
        }
        Some(crate::standards::v1::subsets::any::schema::mutations::replace_object_vortex(object_id.to_string(), vortex.id.clone(), next))
    }

    fn attraction_geometry(command: &Puzzle3dCommand, attraction: &crate::Puzzle3dAttraction) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        let mut geometry = [attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt, attraction.x, attraction.y];
        let index = match field {
            "gap" => 0,
            "shift" => 1,
            "rise" => 2,
            "rotation" => 3,
            "turn" => 4,
            "tilt" => 5,
            _ => return None,
        };
        geometry[index] = puzzle3d_resolve_number_edit(geometry[index], value, delta)?;
        Some(crate::standards::v1::subsets::any::schema::mutations::replace_attraction_geometry(crate::standards::v1::subsets::any::schema::mutations::ReplaceAttractionGeometry {
            id: attraction.id.clone(),
            new_gap: geometry[0],
            new_shift: geometry[1],
            new_rise: geometry[2],
            new_rotation: geometry[3],
            new_turn: geometry[4],
            new_tilt: geometry[5],
            new_x: geometry[6],
            new_y: geometry[7],
        }))
    }

    fn reference_mutation(command: &Puzzle3dCommand, reference: &crate::Puzzle3dReference) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        match field {
            "hidden" => value.and_then(Value::as_bool).map(|value| crate::standards::v1::subsets::any::schema::mutations::change_reference_hidden(reference.id.clone(), value)),
            "locked" => value.and_then(Value::as_bool).map(|value| crate::standards::v1::subsets::any::schema::mutations::change_reference_locked(reference.id.clone(), value)),
            "sourceUrl" | "mediaKind" => {
                let mut source = reference.source.clone();
                if field == "sourceUrl" {
                    source.url = value.and_then(Value::as_str)?.to_string();
                } else {
                    source.media_kind = value.and_then(Value::as_str).map(str::to_string);
                }
                Some(crate::standards::v1::subsets::any::schema::mutations::replace_reference_source(reference.id.clone(), source))
            }
            "origin" => value.and_then(value_as_vec3).map(|origin| crate::standards::v1::subsets::any::schema::mutations::move_reference(reference.id.clone(), origin)),
            "widthWorld" => puzzle3d_resolve_number_edit(reference.width_world, value, delta).map(|width| crate::standards::v1::subsets::any::schema::mutations::resize_reference(reference.id.clone(), width)),
            _ => {
                let axis = puzzle3d_axis_index(field, "origin")?;
                let mut origin = reference.origin;
                origin[axis] = puzzle3d_resolve_number_edit(origin[axis], value, delta)?;
                Some(crate::standards::v1::subsets::any::schema::mutations::move_reference(reference.id.clone(), origin))
            }
        }
    }

    fn volume_mutation(command: &Puzzle3dCommand, volume: &crate::Puzzle3dTargetVolume) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        match field {
            "hidden" => value.and_then(Value::as_bool).map(|value| crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden(volume.id.clone(), value)),
            "locked" => value.and_then(Value::as_bool).map(|value| crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked(volume.id.clone(), value)),
            "origin" => value.and_then(value_as_vec3).map(|origin| crate::standards::v1::subsets::any::schema::mutations::move_target_volume(volume.id.clone(), origin)),
            _ => {
                if let Some(axis) = puzzle3d_axis_index(field, "origin") {
                    let mut origin = volume.origin;
                    origin[axis] = puzzle3d_resolve_number_edit(origin[axis], value, delta)?;
                    return Some(crate::standards::v1::subsets::any::schema::mutations::move_target_volume(volume.id.clone(), origin));
                }
                if let Some(axis) = puzzle3d_axis_index(field, "scale") {
                    let mut scale = Self::scale(volume.scale);
                    scale[axis] = puzzle3d_resolve_number_edit(scale[axis], value, delta)?;
                    return Some(crate::standards::v1::subsets::any::schema::mutations::scale_target_volume(volume.id.clone(), Some(crate::Puzzle3dScale::Vec3(scale))));
                }
                let axis = puzzle3d_axis_index(field, "orientation")?;
                let mut orientation = volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                orientation[axis] = puzzle3d_resolve_number_edit(orientation[axis], value, delta)?;
                Some(crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume(volume.id.clone(), Some(quat_normalize(orientation))))
            }
        }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dPatchInspectorWork {
    fn tool_id(&self) -> &'static str {
        "patchInspector"
    }

    fn extent(&self, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
        let source = Self::source_len(command, interaction);
        let document = snapshot.typed();
        let scan = match Self::entity(command) {
            "object" => document.objects.len().checked_add(1)?,
            "vortex" => {
                let mut vortices = 0usize;
                for object in &document.objects {
                    vortices = vortices.checked_add(object.vortices.len())?;
                }
                document.objects.len().checked_add(1)?.checked_add(vortices)?
            }
            "attraction" => document.attractions.len().checked_mul(2)?.checked_add(1)?,
            "reference" => document.references.len().checked_add(1)?,
            "targetVolume" => document.target_volumes.len().checked_add(1)?,
            _ => 0,
        };
        let items = source.checked_add(scan)?.checked_add(1)?;
        (source <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle3dCommand,
        snapshot: &Puzzle3dPlaySnapshot,
        _config: &Puzzle3dConfig,
        interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>>, Fault> {
        match self.stage {
            Puzzle3dPatchInspectorStage::Selection => {
                if let Some(id) = Self::source_id(command, interaction, self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle3d-patch-inspector-selection-capacity"));
                    }
                    self.selected.insert(id.to_string());
                    self.selection_cursor += 1;
                    return Ok(Self::progress("puzzle3d-patch-inspector-selection", "Reading inspector target", "Inspektionsziel wird gelesen"));
                }
                self.stage = Self::first_stage(Self::entity(command));
                Ok(Self::progress("puzzle3d-patch-inspector-scan", "Finding inspector target", "Inspektionsziel wird gesucht"))
            }
            Puzzle3dPatchInspectorStage::Objects => {
                let Some(object) = snapshot.typed().objects.get(self.item_cursor) else { return Ok(self.complete()) };
                self.item_cursor += 1;
                if self.selected.contains(&object.id) {
                    if let Some(mutation) = Self::object_mutation(command, object) {
                        self.push(mutation)?;
                    }
                }
                Ok(Self::progress("puzzle3d-patch-inspector-object", "Patching object", "Objekt wird geändert"))
            }
            Puzzle3dPatchInspectorStage::Vortices => {
                let Some(object) = snapshot.typed().objects.get(self.item_cursor) else { return Ok(self.complete()) };
                let Some(vortex) = object.vortices.get(self.child_cursor) else {
                    self.item_cursor += 1;
                    self.child_cursor = 0;
                    return Ok(Self::progress("puzzle3d-patch-inspector-vortex-owner", "Advancing vortex owner", "Vortex-Eigentümer wird gewechselt"));
                };
                self.child_cursor += 1;
                if self.selected.contains(&puzzle3d_vortex_full_id(&object.id, &vortex.id)) {
                    if let Some(mutation) = Self::vortex_mutation(command, &object.id, vortex) {
                        self.push(mutation)?;
                    }
                }
                Ok(Self::progress("puzzle3d-patch-inspector-vortex", "Patching vortex", "Vortex wird geändert"))
            }
            Puzzle3dPatchInspectorStage::Attractions => {
                let Some(attraction) = snapshot.typed().attractions.get(self.item_cursor) else { return Ok(self.complete()) };
                self.item_cursor += 1;
                if !self.selected.contains(&attraction.id) {
                    return Ok(Self::progress("puzzle3d-patch-inspector-attraction", "Scanning attraction", "Anziehung wird geprüft"));
                }
                let field = Self::field(command);
                if matches!(field, "attracting" | "attracted") {
                    let Some(value) = command.args().and_then(|args| args.get("value")).and_then(Value::as_str) else {
                        return Ok(Self::progress("puzzle3d-patch-inspector-attraction", "Skipping malformed attraction", "Fehlerhafte Anziehung wird übersprungen"));
                    };
                    let mut next = attraction.clone();
                    if field == "attracting" {
                        next.attracting = value.to_string();
                    } else {
                        next.attracted = value.to_string();
                    }
                    self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_vortices(attraction.id.clone()))?;
                    self.pending_attraction = Some(next);
                    self.stage = Puzzle3dPatchInspectorStage::AttractionReconnect;
                } else if let Some(mutation) = Self::attraction_geometry(command, attraction) {
                    self.push(mutation)?;
                }
                Ok(Self::progress("puzzle3d-patch-inspector-attraction", "Patching attraction", "Anziehung wird geändert"))
            }
            Puzzle3dPatchInspectorStage::AttractionReconnect => {
                let attraction = self.pending_attraction.take().ok_or_else(|| Fault::from("puzzle3d-patch-inspector-attraction-owner"))?;
                self.push(crate::standards::v1::subsets::any::schema::mutations::connect_vortices(
                    attraction.id,
                    attraction.attracting,
                    attraction.attracted,
                    attraction.gap,
                    attraction.shift,
                    attraction.rise,
                    attraction.rotation,
                    attraction.turn,
                    attraction.tilt,
                    attraction.x,
                    attraction.y,
                ))?;
                self.stage = Puzzle3dPatchInspectorStage::Attractions;
                Ok(Self::progress("puzzle3d-patch-inspector-attraction-reconnect", "Reconnecting attraction", "Anziehung wird neu verbunden"))
            }
            Puzzle3dPatchInspectorStage::References => {
                let Some(reference) = snapshot.typed().references.get(self.item_cursor) else { return Ok(self.complete()) };
                self.item_cursor += 1;
                if self.selected.contains(&reference.id) {
                    if let Some(mutation) = Self::reference_mutation(command, reference) {
                        self.push(mutation)?;
                    }
                }
                Ok(Self::progress("puzzle3d-patch-inspector-reference", "Patching reference", "Referenz wird geändert"))
            }
            Puzzle3dPatchInspectorStage::Volumes => {
                let Some(volume) = snapshot.typed().target_volumes.get(self.item_cursor) else { return Ok(self.complete()) };
                self.item_cursor += 1;
                if self.selected.contains(&volume.id) {
                    if let Some(mutation) = Self::volume_mutation(command, volume) {
                        self.push(mutation)?;
                    }
                }
                Ok(Self::progress("puzzle3d-patch-inspector-volume", "Patching target volume", "Zielvolumen wird geändert"))
            }
            Puzzle3dPatchInspectorStage::Complete => Err(Fault::from("puzzle3d-patch-inspector-complete-repolled")),
            Puzzle3dPatchInspectorStage::Closing => Err(Fault::from("puzzle3d-patch-inspector-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dPatchInspectorStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() || self.pending_attraction.take().is_some() {
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
        self.stage == Puzzle3dPatchInspectorStage::Closing && self.selected.is_empty() && self.mutations.is_empty() && self.pending_attraction.is_none()
    }
}

const PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dWorldRelocateStage {
    Object,
    ExistingAttractions,
    CandidateObject,
    CandidateVortex,
    PublishAttraction,
    Complete,
    Closing,
}

struct Puzzle3dWorldRelocateSource {
    object_id: String,
    vortex_id: String,
    local_position: [f64; 3],
    local_direction: [f64; 3],
    world_position: [f64; 3],
    object_position: [f64; 3],
    object_orientation: [f64; 4],
}

struct Puzzle3dWorldRelocateCandidate {
    vortex_id: String,
    local_position: [f64; 3],
    local_direction: [f64; 3],
    object_position: [f64; 3],
    object_orientation: [f64; 4],
}

struct Puzzle3dWorldRelocateWork {
    stage: Puzzle3dWorldRelocateStage,
    object_cursor: usize,
    vortex_cursor: usize,
    attraction_cursor: usize,
    source: Option<Puzzle3dWorldRelocateSource>,
    candidate: Option<Puzzle3dWorldRelocateCandidate>,
    existing: HashSet<String>,
    mutations: Vec<Puzzle3dMutation>,
    window_config: Option<semio_framework_plugin::WindowConfigSnapshot>,
    view_state: Option<semio_framework_plugin::ViewModel>,
}

impl Default for Puzzle3dWorldRelocateWork {
    fn default() -> Self {
        Self {
            stage: Puzzle3dWorldRelocateStage::Object,
            object_cursor: 0,
            vortex_cursor: 0,
            attraction_cursor: 0,
            source: None,
            candidate: None,
            existing: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
            window_config: None,
            view_state: None,
        }
    }
}

impl Puzzle3dWorldRelocateWork {
    fn position(command: &Puzzle3dCommand) -> Option<[f64; 3]> {
        command.args().and_then(|args| args.get("position")).and_then(value_as_vec3)
    }

    fn edge(first: &str, second: &str) -> String {
        if first <= second {
            format!("{first}\0{second}")
        } else {
            format!("{second}\0{first}")
        }
    }

    fn world_position(origin: [f64; 3], orientation: [f64; 4], local: [f64; 3]) -> [f64; 3] {
        let rotated = quat_rotate_vector(orientation, local);
        [origin[0] + rotated[0], origin[1] + rotated[1], origin[2] + rotated[2]]
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn complete(&mut self) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        self.stage = Puzzle3dWorldRelocateStage::Complete;
        crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("worldRelocate")), ..Default::default() })
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dWorldRelocateWork {
    fn tool_id(&self) -> &'static str {
        "worldRelocate"
    }

    fn bind_window_owners(&mut self, config: Option<semio_framework_plugin::WindowConfigSnapshot>, _transient: Option<semio_framework_plugin::WindowTransientSnapshot>) {
        self.window_config = config;
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let document = snapshot.typed();
        let mut object_vortices = 0usize;
        for object in &document.objects {
            object_vortices = object_vortices.checked_add(object.vortices.len())?;
        }
        let object_stage = document.objects.len().checked_add(1)?;
        let existing_attraction_stage = document.attractions.len().checked_add(1)?;
        let candidate_dispatch_stage = document.objects.len().checked_add(1)?;
        let candidate_scan_stage = object_vortices.checked_mul(2)?.checked_add(document.objects.len())?;
        let items = object_stage.checked_add(existing_attraction_stage)?.checked_add(candidate_dispatch_stage)?.checked_add(candidate_scan_stage)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle3dCommand,
        snapshot: &Puzzle3dPlaySnapshot,
        _config: &Puzzle3dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>>, Fault> {
        match self.stage {
            Puzzle3dWorldRelocateStage::Object => {
                let requested = command.args().and_then(|args| args.get("objectId")).and_then(Value::as_str).unwrap_or("");
                let Some(position) = Self::position(command) else { return Ok(self.complete()) };
                let Some(object) = snapshot.typed().objects.get(self.object_cursor) else { return Ok(self.complete()) };
                self.object_cursor += 1;
                // 🔒️ A locked/hidden grab must ANSWER, exactly like `translateSelection`'s own refusal:
                // dropping out silently left the host's relocate ghost snapping back with nothing said.
                if object.id == requested && (object.locked || object.hidden) {
                    self.stage = Puzzle3dWorldRelocateStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.selection_locked.as_str())));
                }
                if object.id == requested && !object.locked && !object.hidden {
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::move_object(object.id.clone(), position));
                    if let Some(vortex) = object.vortices.first() {
                        let orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                        self.source = Some(Puzzle3dWorldRelocateSource {
                            object_id: object.id.clone(),
                            vortex_id: puzzle3d_vortex_full_id(&object.id, &vortex.id),
                            local_position: vortex.position,
                            local_direction: vortex.direction.unwrap_or([0.0, 0.0, -1.0]),
                            world_position: Self::world_position(position, orientation, vortex.position),
                            object_position: position,
                            object_orientation: orientation,
                        });
                    }
                    self.attraction_cursor = 0;
                    self.stage = Puzzle3dWorldRelocateStage::ExistingAttractions;
                }
                Ok(Self::progress("puzzle3d-world-relocate-object", "Finding moved object", "Verschobenes Objekt wird gesucht"))
            }
            Puzzle3dWorldRelocateStage::ExistingAttractions => {
                let Some(attraction) = snapshot.typed().attractions.get(self.attraction_cursor) else {
                    self.object_cursor = 0;
                    self.stage = Puzzle3dWorldRelocateStage::CandidateObject;
                    return Ok(Self::progress("puzzle3d-world-relocate-candidate-object", "Finding nearby object", "Nahes Objekt wird gesucht"));
                };
                if self.existing.len() >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
                    return Err(Fault::from("puzzle3d-world-relocate-attraction-capacity"));
                }
                self.existing.insert(Self::edge(&attraction.attracting, &attraction.attracted));
                self.attraction_cursor += 1;
                Ok(Self::progress("puzzle3d-world-relocate-existing-attraction", "Reading existing attraction", "Bestehende Anziehung wird gelesen"))
            }
            Puzzle3dWorldRelocateStage::CandidateObject => {
                let Some(source) = self.source.as_ref() else { return Ok(self.complete()) };
                let Some(object) = snapshot.typed().objects.get(self.object_cursor) else { return Ok(self.complete()) };
                if object.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                    return Err(Fault::from("puzzle3d-world-relocate-vortex-capacity"));
                }
                self.vortex_cursor = 0;
                if object.id == source.object_id {
                    self.object_cursor += 1;
                } else {
                    self.stage = Puzzle3dWorldRelocateStage::CandidateVortex;
                }
                Ok(Self::progress("puzzle3d-world-relocate-candidate-object", "Scanning nearby object", "Nahes Objekt wird geprüft"))
            }
            Puzzle3dWorldRelocateStage::CandidateVortex => {
                let source = self.source.as_ref().ok_or_else(|| Fault::from("puzzle3d-world-relocate-source-owner"))?;
                let object = snapshot.typed().objects.get(self.object_cursor).ok_or_else(|| Fault::from("puzzle3d-world-relocate-object-cursor"))?;
                let Some(vortex) = object.vortices.get(self.vortex_cursor) else {
                    self.object_cursor += 1;
                    self.stage = Puzzle3dWorldRelocateStage::CandidateObject;
                    return Ok(Self::progress("puzzle3d-world-relocate-candidate-object", "Advancing nearby object", "Nächstes nahes Objekt wird geprüft"));
                };
                self.vortex_cursor += 1;
                let vortex_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                let edge = Self::edge(&source.vortex_id, &vortex_id);
                if vortex_id == source.vortex_id || self.existing.contains(&edge) {
                    return Ok(Self::progress("puzzle3d-world-relocate-candidate-vortex", "Skipping connected vortex", "Verbundener Vortex wird übersprungen"));
                }
                let orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                let world = Self::world_position(object.origin, orientation, vortex.position);
                let delta = [source.world_position[0] - world[0], source.world_position[1] - world[1], source.world_position[2] - world[2]];
                let distance = (delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt();
                let proximity_radius = window_ownership::config_from_snapshot(self.window_config.as_ref()).proximity_radius;
                if distance <= proximity_radius {
                    self.candidate = Some(Puzzle3dWorldRelocateCandidate { vortex_id, local_position: vortex.position, local_direction: vortex.direction.unwrap_or([0.0, 0.0, -1.0]), object_position: object.origin, object_orientation: orientation });
                    self.stage = Puzzle3dWorldRelocateStage::PublishAttraction;
                }
                Ok(Self::progress("puzzle3d-world-relocate-candidate-vortex", "Measuring nearby vortex", "Naher Vortex wird gemessen"))
            }
            Puzzle3dWorldRelocateStage::PublishAttraction => {
                let source = self.source.as_ref().ok_or_else(|| Fault::from("puzzle3d-world-relocate-source-owner"))?;
                let candidate = self.candidate.take().ok_or_else(|| Fault::from("puzzle3d-world-relocate-candidate-owner"))?;
                let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(
                    candidate.object_position,
                    candidate.object_orientation,
                    candidate.local_position,
                    candidate.local_direction,
                    source.local_position,
                    source.local_direction,
                    source.object_position,
                    source.object_orientation,
                );
                let id = format!("attraction-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_vortices(id, candidate.vortex_id.clone(), source.vortex_id.clone(), gap, shift, rise, rotation, turn, tilt, 0.0, 0.0));
                self.existing.insert(Self::edge(&source.vortex_id, &candidate.vortex_id));
                self.stage = Puzzle3dWorldRelocateStage::CandidateVortex;
                Ok(Self::progress("puzzle3d-world-relocate-publish-attraction", "Connecting nearby vortex", "Naher Vortex wird verbunden"))
            }
            Puzzle3dWorldRelocateStage::Complete => Err(Fault::from("puzzle3d-world-relocate-complete-repolled")),
            Puzzle3dWorldRelocateStage::Closing => Err(Fault::from("puzzle3d-world-relocate-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dWorldRelocateStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() || self.candidate.take().is_some() || self.source.take().is_some() || self.window_config.take().is_some() || self.view_state.take().is_some() {
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
        self.stage == Puzzle3dWorldRelocateStage::Closing && self.source.is_none() && self.candidate.is_none() && self.existing.is_empty() && self.mutations.is_empty() && self.window_config.is_none() && self.view_state.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dCreateAttractionStage {
    Existing,
    Attracting,
    Attracted,
    Compatibility,
    Publish,
    Complete,
    Closing,
}

struct Puzzle3dAttractionEndpoint {
    vortex_id: String,
    vortex_kind: Option<String>,
    local_position: [f64; 3],
    local_direction: [f64; 3],
    object_position: [f64; 3],
    object_orientation: [f64; 4],
}

struct Puzzle3dCreateAttractionWork {
    stage: Puzzle3dCreateAttractionStage,
    item_cursor: usize,
    child_cursor: usize,
    compatibility_cursor: usize,
    attracting: Option<Puzzle3dAttractionEndpoint>,
    attracted: Option<Puzzle3dAttractionEndpoint>,
    compatible: bool,
}

impl Default for Puzzle3dCreateAttractionWork {
    fn default() -> Self {
        Self { stage: Puzzle3dCreateAttractionStage::Existing, item_cursor: 0, child_cursor: 0, compatibility_cursor: 0, attracting: None, attracted: None, compatible: false }
    }
}

impl Puzzle3dCreateAttractionWork {
    fn ids(command: &Puzzle3dCommand) -> (&str, &str) {
        let args = command.args();
        (args.and_then(|args| args.get("attracting")).and_then(Value::as_str).unwrap_or(""), args.and_then(|args| args.get("attracted")).and_then(Value::as_str).unwrap_or(""))
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn endpoint(object: &crate::Puzzle3dObject, vortex: &crate::Puzzle3dVortex) -> Puzzle3dAttractionEndpoint {
        Puzzle3dAttractionEndpoint {
            vortex_id: puzzle3d_vortex_full_id(&object.id, &vortex.id),
            vortex_kind: vortex.vortex_kind.clone(),
            local_position: vortex.position,
            local_direction: vortex.direction.unwrap_or([0.0, 0.0, -1.0]),
            object_position: object.origin,
            object_orientation: object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
        }
    }

    fn scan_endpoint(&mut self, snapshot: &Puzzle3dPlaySnapshot, requested: &str) -> Result<Option<Puzzle3dAttractionEndpoint>, Fault> {
        let Some(object) = snapshot.typed().objects.get(self.item_cursor) else {
            return Ok(None);
        };
        if object.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
            return Err(Fault::from("puzzle3d-create-attraction-vortex-capacity"));
        }
        let Some(vortex) = object.vortices.get(self.child_cursor) else {
            self.item_cursor += 1;
            self.child_cursor = 0;
            return Ok(None);
        };
        self.child_cursor += 1;
        Ok((puzzle3d_vortex_full_id(&object.id, &vortex.id) == requested).then(|| Self::endpoint(object, vortex)))
    }

    fn begin_endpoint_scan(&mut self, stage: Puzzle3dCreateAttractionStage) {
        self.item_cursor = 0;
        self.child_cursor = 0;
        self.stage = stage;
    }

    fn complete(&mut self) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        self.stage = Puzzle3dCreateAttractionStage::Complete;
        crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default())
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dCreateAttractionWork {
    fn tool_id(&self) -> &'static str {
        "createAttraction"
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let document = snapshot.typed();
        let mut object_vortices = 0usize;
        for object in &document.objects {
            object_vortices = object_vortices.checked_add(object.vortices.len())?;
        }
        let endpoint_scan_stage = object_vortices.checked_add(document.objects.len())?.checked_add(1)?;
        let items = document.attractions.len().checked_add(1)?.checked_add(endpoint_scan_stage)?.checked_add(endpoint_scan_stage)?.checked_add(document.meta.kind_compatibility.len())?.checked_add(2)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle3dCommand,
        snapshot: &Puzzle3dPlaySnapshot,
        _config: &Puzzle3dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>>, Fault> {
        let (attracting_id, attracted_id) = Self::ids(command);
        if attracting_id.is_empty() || attracted_id.is_empty() || attracting_id == attracted_id {
            return Ok(self.complete());
        }
        match self.stage {
            Puzzle3dCreateAttractionStage::Existing => {
                let Some(attraction) = snapshot.typed().attractions.get(self.item_cursor) else {
                    self.begin_endpoint_scan(Puzzle3dCreateAttractionStage::Attracting);
                    return Ok(Self::progress("puzzle3d-create-attraction-attracting", "Finding attracting vortex", "Anziehender Vortex wird gesucht"));
                };
                self.item_cursor += 1;
                if (attraction.attracting == attracting_id && attraction.attracted == attracted_id) || (attraction.attracting == attracted_id && attraction.attracted == attracting_id) {
                    return Ok(self.complete());
                }
                Ok(Self::progress("puzzle3d-create-attraction-existing", "Checking existing attraction", "Bestehende Anziehung wird geprüft"))
            }
            Puzzle3dCreateAttractionStage::Attracting => {
                if self.item_cursor >= snapshot.typed().objects.len() {
                    return Ok(self.complete());
                }
                if let Some(endpoint) = self.scan_endpoint(snapshot, attracting_id)? {
                    self.attracting = Some(endpoint);
                    self.begin_endpoint_scan(Puzzle3dCreateAttractionStage::Attracted);
                }
                Ok(Self::progress("puzzle3d-create-attraction-attracting", "Scanning attracting vortex", "Anziehender Vortex wird geprüft"))
            }
            Puzzle3dCreateAttractionStage::Attracted => {
                if self.item_cursor >= snapshot.typed().objects.len() {
                    return Ok(self.complete());
                }
                if let Some(endpoint) = self.scan_endpoint(snapshot, attracted_id)? {
                    self.attracted = Some(endpoint);
                    self.compatibility_cursor = 0;
                    self.compatible = snapshot.typed().meta.kind_compatibility.is_empty();
                    self.stage = Puzzle3dCreateAttractionStage::Compatibility;
                }
                Ok(Self::progress("puzzle3d-create-attraction-attracted", "Scanning attracted vortex", "Angezogener Vortex wird geprüft"))
            }
            Puzzle3dCreateAttractionStage::Compatibility => {
                let attracting_kind = self.attracting.as_ref().and_then(|endpoint| endpoint.vortex_kind.as_deref());
                let attracted_kind = self.attracted.as_ref().and_then(|endpoint| endpoint.vortex_kind.as_deref());
                let (Some(attracting_kind), Some(attracted_kind)) = (attracting_kind, attracted_kind) else {
                    return Ok(self.complete());
                };
                let Some(row) = snapshot.typed().meta.kind_compatibility.get(self.compatibility_cursor) else {
                    if self.compatible {
                        self.stage = Puzzle3dCreateAttractionStage::Publish;
                        return Ok(Self::progress("puzzle3d-create-attraction-publish", "Preparing attraction", "Anziehung wird vorbereitet"));
                    }
                    return Ok(self.complete());
                };
                self.compatibility_cursor += 1;
                self.compatible |= (row.source == attracting_kind && row.target == attracted_kind) || (row.bidirectional && row.source == attracted_kind && row.target == attracting_kind);
                Ok(Self::progress("puzzle3d-create-attraction-compatibility", "Checking vortex compatibility", "Vortex-Kompatibilität wird geprüft"))
            }
            Puzzle3dCreateAttractionStage::Publish => {
                let attracting = self.attracting.take().ok_or_else(|| Fault::from("puzzle3d-create-attraction-attracting-owner"))?;
                let attracted = self.attracted.take().ok_or_else(|| Fault::from("puzzle3d-create-attraction-attracted-owner"))?;
                let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(
                    attracting.object_position,
                    attracting.object_orientation,
                    attracting.local_position,
                    attracting.local_direction,
                    attracted.local_position,
                    attracted.local_direction,
                    attracted.object_position,
                    attracted.object_orientation,
                );
                let id = format!("attraction-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                self.stage = Puzzle3dCreateAttractionStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit {
                    artifact_mutations: vec![crate::standards::v1::subsets::any::schema::mutations::connect_vortices(id, attracting.vortex_id, attracted.vortex_id, gap, shift, rise, rotation, turn, tilt, 0.0, 0.0)],
                    ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("createAttraction")),
                    ..Default::default()
                }))
            }
            Puzzle3dCreateAttractionStage::Complete => Err(Fault::from("puzzle3d-create-attraction-complete-repolled")),
            Puzzle3dCreateAttractionStage::Closing => Err(Fault::from("puzzle3d-create-attraction-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dCreateAttractionStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.attracting.take().is_some() || self.attracted.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle3dCreateAttractionStage::Closing && self.attracting.is_none() && self.attracted.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dSetActiveExampleStage {
    DeleteAttractions,
    DeleteObjects,
    DeleteVolumes,
    DeleteReferences,
    DeleteCompatibility,
    Domain,
    Catalogs,
    CreateObjects,
    CreateAttractions,
    CreateVolumes,
    CreateReferences,
    CreateCompatibility,
    Publish,
    Complete,
    Closing,
}

/// 🔢️ Steps `Puzzle3dSetActiveExampleWork` spends on stage TRANSITIONS rather than on document items:
/// each of the five delete stages and each of the five create stages answers its own exhaustion with one
/// `Progress`, `Domain` and `Catalogs` are one step each, and `Publish` is the final `Complete` call —
/// 5 + 1 + 1 + 5 + 1 = 13. The extent is an upper bound on `step()` calls (the sibling `patchInspector`
/// law asserts `iterations <= extent`), so it has to carry all of them; it declared `3`, which is short
/// by ten and made a real example load overrun its own declared envelope.
const PUZZLE3D_SET_ACTIVE_EXAMPLE_FIXED_STEPS: usize = 13;
const PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK: usize = 8;
const PUZZLE3D_SET_ACTIVE_EXAMPLE_COALESCE_KEY: &str = "set-active-example";
const PUZZLE3D_SET_ACTIVE_EXAMPLE_DESCRIPTION: &str = "Set Active Example";

struct Puzzle3dSetActiveExampleWork {
    stage: Puzzle3dSetActiveExampleStage,
    cursor: usize,
    mutations: Vec<Puzzle3dMutation>,
}

impl Default for Puzzle3dSetActiveExampleWork {
    fn default() -> Self {
        Self { stage: Puzzle3dSetActiveExampleStage::DeleteAttractions, cursor: 0, mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS) }
    }
}

impl Puzzle3dSetActiveExampleWork {
    fn target(command: &Puzzle3dCommand) -> Option<&'static Puzzle3dFixture> {
        match command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or("") {
            "" => Some(&EMPTY_EXAMPLE_FIXTURE),
            PUZZLE3D_EXAMPLE_CONCRETE_FOREST | "concrete" => Some(&CONCRETE_FOREST_EXAMPLE_FIXTURE),
            PUZZLE3D_EXAMPLE_NAKAGIN | "nakagin" => Some(&NAKAGIN_EXAMPLE_FIXTURE),
            _ => None,
        }
    }

    /// 🏷️ The CANONICAL example id this command loads, for the alias the picker may have sent
    /// (`concrete`/`nakagin`). Persisted on the shared config so `export_fixture` can name its
    /// download after the example instead of one constant filename.
    fn canonical_example_id(command: &Puzzle3dCommand) -> Option<&'static str> {
        match command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or("") {
            "" => Some(""),
            PUZZLE3D_EXAMPLE_CONCRETE_FOREST | "concrete" => Some(PUZZLE3D_EXAMPLE_CONCRETE_FOREST),
            PUZZLE3D_EXAMPLE_NAKAGIN | "nakagin" => Some(PUZZLE3D_EXAMPLE_NAKAGIN),
            _ => None,
        }
    }

    fn compatibility_rows(target: &Puzzle3dFixture) -> &[dsl::DslValue] {
        target.meta.kind_compatibility.as_ref().and_then(dsl::DslValue::as_array).unwrap_or_default()
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn push(&mut self, mutation: Puzzle3dMutation) -> Result<(), Fault> {
        if self.mutations.len() >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle3d-set-active-example-output-capacity"));
        }
        self.mutations.push(mutation);
        Ok(())
    }

    fn take_chunk(cursor: &mut usize, len: usize) -> std::ops::Range<usize> {
        if *cursor >= len {
            return *cursor..*cursor;
        }
        let end = cursor.saturating_add(PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK).min(len);
        let range = *cursor..end;
        *cursor = end;
        range
    }

    fn advance(&mut self, stage: Puzzle3dSetActiveExampleStage) {
        self.cursor = 0;
        self.stage = stage;
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dSetActiveExampleWork {
    fn tool_id(&self) -> &'static str {
        "setActiveExample"
    }

    fn extent(&self, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let target = Self::target(command)?;
        let document = snapshot.typed();
        let items = document
            .attractions
            .len()
            .checked_add(document.objects.len())?
            .checked_add(document.target_volumes.len())?
            .checked_add(document.references.len())?
            .checked_add(document.meta.kind_compatibility.len())?
            .checked_add(target.attractions.len())?
            .checked_add(target.objects.len())?
            .checked_add(target.target_volumes.len())?
            .checked_add(target.references.len())?
            .checked_add(Self::compatibility_rows(target).len())?
            .checked_add(PUZZLE3D_SET_ACTIVE_EXAMPLE_FIXED_STEPS)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle3dCommand,
        snapshot: &Puzzle3dPlaySnapshot,
        config: &Puzzle3dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>>, Fault> {
        let Some(target) = Self::target(command) else { return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default())) };
        match self.stage {
            Puzzle3dSetActiveExampleStage::DeleteAttractions => {
                let items = &snapshot.typed().attractions;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for attraction in &items[range] {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_vortices(attraction.id.clone()))?;
                    }
                    return Ok(Self::progress("puzzle3d-example-delete-attraction", "Removing old attraction", "Alte Anziehung wird entfernt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::DeleteObjects);
                Ok(Self::progress("puzzle3d-example-delete-object", "Removing old object", "Altes Objekt wird entfernt"))
            }
            Puzzle3dSetActiveExampleStage::DeleteObjects => {
                let items = &snapshot.typed().objects;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for object in &items[range] {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::delete_object(object.id.clone()))?;
                    }
                    return Ok(Self::progress("puzzle3d-example-delete-object", "Removing old object", "Altes Objekt wird entfernt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::DeleteVolumes);
                Ok(Self::progress("puzzle3d-example-delete-volume", "Removing old target volume", "Altes Zielvolumen wird entfernt"))
            }
            Puzzle3dSetActiveExampleStage::DeleteVolumes => {
                let items = &snapshot.typed().target_volumes;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for volume in &items[range] {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::delete_target_volume(volume.id.clone()))?;
                    }
                    return Ok(Self::progress("puzzle3d-example-delete-volume", "Removing old target volume", "Altes Zielvolumen wird entfernt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::DeleteReferences);
                Ok(Self::progress("puzzle3d-example-delete-reference", "Removing old reference", "Alte Referenz wird entfernt"))
            }
            Puzzle3dSetActiveExampleStage::DeleteReferences => {
                let items = &snapshot.typed().references;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for reference in &items[range] {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::delete_reference(reference.id.clone()))?;
                    }
                    return Ok(Self::progress("puzzle3d-example-delete-reference", "Removing old reference", "Alte Referenz wird entfernt"))
                }
                self.advance(Puzzle3dSetActiveExampleStage::DeleteCompatibility);
                Ok(Self::progress("puzzle3d-example-delete-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"))
            }
            Puzzle3dSetActiveExampleStage::DeleteCompatibility => {
                let items = &snapshot.typed().meta.kind_compatibility;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for row in &items[range] {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility(row.source.clone(), row.target.clone()))?;
                    }
                    return Ok(Self::progress("puzzle3d-example-delete-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::Domain);
                Ok(Self::progress("puzzle3d-example-domain", "Updating document domain", "Dokumentdomäne wird aktualisiert"))
            }
            Puzzle3dSetActiveExampleStage::Domain => {
                self.push(crate::standards::v1::subsets::any::schema::mutations::change_domain(target.domain.clone()))?;
                self.advance(Puzzle3dSetActiveExampleStage::Catalogs);
                Ok(Self::progress("puzzle3d-example-catalogs", "Updating kind catalogs", "Artenkataloge werden aktualisiert"))
            }
            Puzzle3dSetActiveExampleStage::Catalogs => {
                let catalogs = target.meta.kind_catalogs.as_ref().map(|catalogs| dsl::FromValue::from_value(catalogs.clone())).transpose().map_err(|_| Fault::from("puzzle3d-set-active-example-catalogs-malformed"))?;
                self.push(crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs(catalogs))?;
                self.advance(Puzzle3dSetActiveExampleStage::CreateObjects);
                Ok(Self::progress("puzzle3d-example-create-object", "Adding example object", "Beispielobjekt wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateObjects => {
                let items = &target.objects;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for object in &items[range] {
                        let value = dsl::ToValue::to_value(object);
                        let object = dsl::FromValue::from_value(value).map_err(|_| Fault::from("puzzle3d-set-active-example-object-malformed"))?;
                        self.push(crate::standards::v1::subsets::any::schema::mutations::create_object(object, None))?;
                    }
                    return Ok(Self::progress("puzzle3d-example-create-object", "Adding example object", "Beispielobjekt wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::CreateAttractions);
                Ok(Self::progress("puzzle3d-example-create-attraction", "Adding example attraction", "Beispielanziehung wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateAttractions => {
                let items = &target.attractions;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for attraction in &items[range] {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::connect_vortices(
                            attraction.id.clone(),
                            attraction.attracting.clone(),
                            attraction.attracted.clone(),
                            attraction.gap,
                            attraction.shift,
                            attraction.rise,
                            attraction.rotation,
                            attraction.turn,
                            attraction.tilt,
                            0.0,
                            0.0,
                        ))?;
                    }
                    return Ok(Self::progress("puzzle3d-example-create-attraction", "Adding example attraction", "Beispielanziehung wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::CreateVolumes);
                Ok(Self::progress("puzzle3d-example-create-volume", "Adding example target volume", "Beispielzielvolumen wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateVolumes => {
                let items = &target.target_volumes;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for volume in &items[range] {
                        let value = dsl::ToValue::to_value(volume);
                        let volume = dsl::FromValue::from_value(value).map_err(|_| Fault::from("puzzle3d-set-active-example-volume-malformed"))?;
                        self.push(crate::standards::v1::subsets::any::schema::mutations::create_target_volume(volume, None))?;
                    }
                    return Ok(Self::progress("puzzle3d-example-create-volume", "Adding example target volume", "Beispielzielvolumen wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::CreateReferences);
                Ok(Self::progress("puzzle3d-example-create-reference", "Adding example reference", "Beispielreferenz wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateReferences => {
                let items = &target.references;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for reference in &items[range] {
                        let value = dsl::ToValue::to_value(reference);
                        let reference = dsl::FromValue::from_value(value).map_err(|_| Fault::from("puzzle3d-set-active-example-reference-malformed"))?;
                        self.push(crate::standards::v1::subsets::any::schema::mutations::create_reference(reference, None))?;
                    }
                    return Ok(Self::progress("puzzle3d-example-create-reference", "Adding example reference", "Beispielreferenz wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::CreateCompatibility);
                Ok(Self::progress("puzzle3d-example-create-compatibility", "Adding example compatibility", "Beispielkompatibilität wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateCompatibility => {
                let items = Self::compatibility_rows(target);
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for row in items[range].iter().cloned() {
                        let row: crate::Puzzle3dKindCompatibility = dsl::FromValue::from_value(row).map_err(|_| Fault::from("puzzle3d-set-active-example-compatibility-malformed"))?;
                        self.push(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(row.source, row.target, row.bidirectional, row.important, row.specificity))?;
                    }
                    return Ok(Self::progress("puzzle3d-example-create-compatibility", "Adding example compatibility", "Beispielkompatibilität wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::Publish);
                Ok(Self::progress("puzzle3d-example-publish", "Publishing example", "Beispiel wird veröffentlicht"))
            }
            Puzzle3dSetActiveExampleStage::Publish => {
                self.stage = Puzzle3dSetActiveExampleStage::Complete;
                let example_id = Self::canonical_example_id(command).unwrap_or_default();
                let config_mutations = if config.active_example_id == example_id {
                    Vec::new()
                } else {
                    vec![Puzzle3dConfigMutation::Snapshot { config: Puzzle3dConfig { active_example_id: example_id.to_string(), ..config.clone() } }]
                };
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit {
                    artifact_mutations: std::mem::take(&mut self.mutations),
                    config_mutations,
                    ui_scope: puzzle3d_scope(Puzzle3dScopeClass::Chrome),
                    description: Some(PUZZLE3D_SET_ACTIVE_EXAMPLE_DESCRIPTION.into()),
                    coalesce_key: Some(PUZZLE3D_SET_ACTIVE_EXAMPLE_COALESCE_KEY.into()),
                    ..Default::default()
                }))
            }
            Puzzle3dSetActiveExampleStage::Complete => Err(Fault::from("puzzle3d-set-active-example-complete-repolled")),
            Puzzle3dSetActiveExampleStage::Closing => Err(Fault::from("puzzle3d-set-active-example-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dSetActiveExampleStage::Closing;
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
        self.stage == Puzzle3dSetActiveExampleStage::Closing && self.mutations.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dAddBrushObjectStage {
    Decode,
    Kind,
    Representation,
    Vortices,
    ExistingAttractions,
    PublishObject,
    PublishAttraction,
    Complete,
    Closing,
}

struct Puzzle3dBrushPayloadOwner {
    target_vortex_id: String,
    object_kind_id: String,
    source_vortex_index: usize,
    origin: [f64; 3],
    orientation: [f64; 4],
    scale: Option<crate::Puzzle3dScale>,
}

struct Puzzle3dAddBrushObjectWork {
    stage: Puzzle3dAddBrushObjectStage,
    /// 🗣️ Bound once per admission — the locale×terminology axes a refusal notice is phrased against.
    view_state: Option<semio_framework_plugin::ViewModel>,
    kind_cursor: usize,
    representation_cursor: usize,
    vortex_cursor: usize,
    attraction_cursor: usize,
    kind_index: Option<usize>,
    payload: Option<Puzzle3dBrushPayloadOwner>,
    object_id: Option<String>,
    mesh_url: Option<String>,
    vortices: Vec<crate::Puzzle3dVortex>,
    mutations: Vec<Puzzle3dMutation>,
}

impl Default for Puzzle3dAddBrushObjectWork {
    fn default() -> Self {
        Self {
            stage: Puzzle3dAddBrushObjectStage::Decode,
            view_state: None,
            kind_cursor: 0,
            representation_cursor: 0,
            vortex_cursor: 0,
            attraction_cursor: 0,
            kind_index: None,
            payload: None,
            object_id: None,
            mesh_url: None,
            vortices: Vec::with_capacity(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT),
            mutations: Vec::with_capacity(2),
        }
    }
}

impl Puzzle3dAddBrushObjectWork {
    fn vector3(value: Option<&Value>) -> Option<[f64; 3]> {
        let values = value.and_then(Value::as_array)?;
        Some([values.first().and_then(Value::as_f64)?, values.get(1).and_then(Value::as_f64)?, values.get(2).and_then(Value::as_f64)?])
    }

    fn quaternion(value: Option<&Value>) -> Option<[f64; 4]> {
        let values = value.and_then(Value::as_array)?;
        Some([values.first().and_then(Value::as_f64)?, values.get(1).and_then(Value::as_f64)?, values.get(2).and_then(Value::as_f64)?, values.get(3).and_then(Value::as_f64)?])
    }

    fn scale(value: Option<&Value>) -> Option<crate::Puzzle3dScale> {
        match value {
            Some(Value::Number(value)) => Some(crate::Puzzle3dScale::Uniform(value.as_f64())),
            Some(Value::Array(values)) if values.len() >= 3 => Some(crate::Puzzle3dScale::Vec3([values.first().and_then(Value::as_f64)?, values.get(1).and_then(Value::as_f64)?, values.get(2).and_then(Value::as_f64)?])),
            _ => None,
        }
    }

    fn decode(command: &Puzzle3dCommand) -> Option<Puzzle3dBrushPayloadOwner> {
        let args = command.args()?;
        let target_vortex_id = args.get("targetVortexFullId").and_then(Value::as_str)?.to_string();
        let object_kind_id = args.get("objectKindId").and_then(Value::as_str)?.to_string();
        let source_vortex_index = args.get("sourceVortexIndex").and_then(Value::as_u64)? as usize;
        Some(Puzzle3dBrushPayloadOwner { target_vortex_id, object_kind_id, source_vortex_index, origin: Self::vector3(args.get("origin"))?, orientation: Self::quaternion(args.get("orientation"))?, scale: Self::scale(args.get("scale")) })
    }

    fn object_id(snapshot: &Puzzle3dPlaySnapshot, payload: &Puzzle3dBrushPayloadOwner) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        snapshot.typed().objects.len().hash(&mut hasher);
        payload.target_vortex_id.hash(&mut hasher);
        payload.object_kind_id.hash(&mut hasher);
        payload.source_vortex_index.hash(&mut hasher);
        for value in payload.origin.into_iter().chain(payload.orientation) {
            value.to_bits().hash(&mut hasher);
        }
        format!("puzzle3d.brush.{:016x}", hasher.finish())
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dAddBrushObjectWork {
    fn tool_id(&self) -> &'static str {
        "addBrushObject"
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }


    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let catalogs = snapshot.typed().meta.kind_catalogs.as_ref()?;
        let items = catalogs.objects.len().checked_add(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT.checked_mul(2)?)?.checked_add(snapshot.typed().attractions.len())?.checked_add(4)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle3dCommand,
        snapshot: &Puzzle3dPlaySnapshot,
        _config: &Puzzle3dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>>, Fault> {
        // 🧯️ Every early terminal below is a click that placed nothing. Each one now says so — an
        // absent/mismatched kind, a kind with no mesh or no source vortex, and a target vortex that is
        // already attracted are all invisible refusals otherwise.
        let Some(catalogs) = snapshot.typed().meta.kind_catalogs.as_ref() else {
            self.stage = Puzzle3dAddBrushObjectStage::Complete;
            return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.placement_unavailable.as_str())));
        };
        match self.stage {
            Puzzle3dAddBrushObjectStage::Decode => {
                let Some(payload) = Self::decode(command) else {
                    self.stage = Puzzle3dAddBrushObjectStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.placement_rejected.as_str())));
                };
                self.object_id = Some(Self::object_id(snapshot, &payload));
                self.payload = Some(payload);
                self.stage = Puzzle3dAddBrushObjectStage::Kind;
                Ok(Self::progress("puzzle3d-brush-kind", "Finding brush object kind", "Pinselobjektart wird gesucht"))
            }
            Puzzle3dAddBrushObjectStage::Kind => {
                let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                let Some(kind) = catalogs.objects.get(self.kind_cursor) else {
                    self.stage = Puzzle3dAddBrushObjectStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.placement_unavailable.as_str())));
                };
                if kind.id == payload.object_kind_id {
                    if kind.representations.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT || kind.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                        return Err(Fault::from("puzzle3d-brush-catalog-capacity"));
                    }
                    self.kind_index = Some(self.kind_cursor);
                    self.stage = Puzzle3dAddBrushObjectStage::Representation;
                } else {
                    self.kind_cursor += 1;
                }
                Ok(Self::progress("puzzle3d-brush-kind", "Scanning brush object kind", "Pinselobjektart wird geprüft"))
            }
            Puzzle3dAddBrushObjectStage::Representation => {
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-brush-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-brush-kind-cursor"))?;
                let Some(representation) = kind.representations.get(self.representation_cursor) else {
                    self.stage = Puzzle3dAddBrushObjectStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.placement_unavailable.as_str())));
                };
                self.representation_cursor += 1;
                if !representation.url.is_empty() {
                    self.mesh_url = Some(representation.url.clone());
                    self.stage = Puzzle3dAddBrushObjectStage::Vortices;
                }
                Ok(Self::progress("puzzle3d-brush-representation", "Finding brush mesh", "Pinsel-Mesh wird gesucht"))
            }
            Puzzle3dAddBrushObjectStage::Vortices => {
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-brush-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-brush-kind-cursor"))?;
                let Some(template) = kind.vortices.get(self.vortex_cursor) else {
                    let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                    if payload.source_vortex_index >= self.vortices.len() {
                        self.stage = Puzzle3dAddBrushObjectStage::Complete;
                        return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.placement_rejected.as_str())));
                    }
                    self.stage = Puzzle3dAddBrushObjectStage::ExistingAttractions;
                    return Ok(Self::progress("puzzle3d-brush-existing-attraction", "Checking brush target", "Pinselziel wird geprüft"));
                };
                let object_id = self.object_id.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-object-owner"))?;
                let index = self.vortex_cursor;
                self.vortex_cursor += 1;
                self.vortices.push(crate::Puzzle3dVortex {
                    id: format!("{object_id}:v{index}"),
                    label: None,
                    vortex_kind: template.vortex_kind.clone(),
                    position: template.point,
                    direction: Some(template.direction),
                    radius: template.radius,
                    hidden: false,
                    locked: false,
                });
                Ok(Self::progress("puzzle3d-brush-vortex", "Building brush vortex", "Pinsel-Vortex wird aufgebaut"))
            }
            Puzzle3dAddBrushObjectStage::ExistingAttractions => {
                let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                let source = self.vortices.get(payload.source_vortex_index).ok_or_else(|| Fault::from("puzzle3d-brush-source-vortex-owner"))?;
                let Some(attraction) = snapshot.typed().attractions.get(self.attraction_cursor) else {
                    self.stage = Puzzle3dAddBrushObjectStage::PublishObject;
                    return Ok(Self::progress("puzzle3d-brush-publish-object", "Preparing brush object", "Pinselobjekt wird vorbereitet"));
                };
                self.attraction_cursor += 1;
                if attraction.attracting == payload.target_vortex_id || attraction.attracted == source.id {
                    self.stage = Puzzle3dAddBrushObjectStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.placement_occupied.as_str())));
                }
                Ok(Self::progress("puzzle3d-brush-existing-attraction", "Scanning brush target", "Pinselziel wird geprüft"))
            }
            Puzzle3dAddBrushObjectStage::PublishObject => {
                let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                let object = crate::Puzzle3dObject {
                    id: self.object_id.clone().ok_or_else(|| Fault::from("puzzle3d-brush-object-owner"))?,
                    label: None,
                    object_kind: Some(payload.object_kind_id.clone()),
                    anchor: Default::default(),
                    origin: payload.origin,
                    orientation: Some(payload.orientation),
                    scale: payload.scale,
                    mesh_url: self.mesh_url.clone(),
                    vortices: std::mem::take(&mut self.vortices),
                    hidden: false,
                    locked: false,
                };
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::create_object(object, None));
                self.stage = Puzzle3dAddBrushObjectStage::PublishAttraction;
                Ok(Self::progress("puzzle3d-brush-publish-object", "Publishing brush object", "Pinselobjekt wird veröffentlicht"))
            }
            Puzzle3dAddBrushObjectStage::PublishAttraction => {
                let payload = self.payload.take().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                let object_id = self.object_id.take().ok_or_else(|| Fault::from("puzzle3d-brush-object-owner"))?;
                let attracted = format!("{object_id}:v{}", payload.source_vortex_index);
                let attraction_id = format!("attraction-{}-{attracted}", payload.target_vortex_id);
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_vortices(attraction_id, payload.target_vortex_id, attracted, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
                self.stage = Puzzle3dAddBrushObjectStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("addBrushObject")), ..Default::default() }))
            }
            Puzzle3dAddBrushObjectStage::Complete => Err(Fault::from("puzzle3d-brush-complete-repolled")),
            Puzzle3dAddBrushObjectStage::Closing => Err(Fault::from("puzzle3d-brush-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dAddBrushObjectStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.mutations.pop().is_some() || self.vortices.pop().is_some() || self.payload.take().is_some() || self.object_id.take().is_some() || self.mesh_url.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle3dAddBrushObjectStage::Closing && self.payload.is_none() && self.object_id.is_none() && self.mesh_url.is_none() && self.vortices.is_empty() && self.mutations.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dFocusSelectionStage {
    Selection,
    SumObjects,
    DistanceObjects,
    Publish,
    Complete,
    Closing,
}

struct Puzzle3dFocusSelectionWork {
    stage: Puzzle3dFocusSelectionStage,
    selection_cursor: usize,
    object_cursor: usize,
    selected: HashSet<String>,
    center: [f64; 3],
    matched: usize,
    maximum_distance: f64,
    view_state: Option<semio_framework_plugin::ViewModel>,
    window_config: Option<semio_framework_plugin::WindowConfigSnapshot>,
}

impl Default for Puzzle3dFocusSelectionWork {
    fn default() -> Self {
        Self {
            stage: Puzzle3dFocusSelectionStage::Selection,
            selection_cursor: 0,
            object_cursor: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            center: [0.0; 3],
            matched: 0,
            maximum_distance: 1.0,
            view_state: None,
            window_config: None,
        }
    }
}

impl Puzzle3dFocusSelectionWork {
    fn selection(interaction: &protocol::InteractionState) -> &[String] {
        interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE3D_GRANULARITY_OBJECT).map(|selection| selection.ids.as_slice()).unwrap_or_default()
    }

    /// 🎥️ Which objects this focus frames. An EMPTY selection frames the whole document rather than
    /// completing with `Emit::default()` — a camera verb with nothing selected has an obvious subject
    /// (everything), and the silent no-op made `f` a dead key on every boot before the user had picked
    /// anything (`📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 14). Costs no extra capacity: the
    /// two object scans this work already declares in `extent` run either way.
    fn frames(&self, object_id: &str) -> bool {
        self.selected.is_empty() || self.selected.contains(object_id)
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dFocusSelectionWork {
    fn tool_id(&self) -> &'static str {
        "focusSelection"
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }

    fn bind_window_owners(&mut self, config: Option<semio_framework_plugin::WindowConfigSnapshot>, _transient: Option<semio_framework_plugin::WindowTransientSnapshot>) {
        self.window_config = config;
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
        let selected = Self::selection(interaction).len();
        let objects = snapshot.typed().objects.len();
        let items = selected.checked_add(objects.checked_mul(2)?)?.checked_add(1)?;
        (selected <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        _command: &Puzzle3dCommand,
        snapshot: &Puzzle3dPlaySnapshot,
        _config: &Puzzle3dConfig,
        interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>>, Fault> {
        match self.stage {
            Puzzle3dFocusSelectionStage::Selection => {
                if let Some(id) = Self::selection(interaction).get(self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle3d-focus-selection-capacity"));
                    }
                    self.selected.insert(id.clone());
                    self.selection_cursor += 1;
                    return Ok(Self::progress("puzzle3d-focus-selection-owner", "Reading selected object", "Ausgewähltes Objekt wird gelesen"));
                }
                self.stage = Puzzle3dFocusSelectionStage::SumObjects;
                Ok(Self::progress("puzzle3d-focus-selection-center", "Measuring selection center", "Auswahlzentrum wird gemessen"))
            }
            Puzzle3dFocusSelectionStage::SumObjects => {
                let Some(object) = snapshot.typed().objects.get(self.object_cursor) else {
                    self.object_cursor = 0;
                    if self.matched > 0 {
                        let divisor = self.matched as f64;
                        self.center = [self.center[0] / divisor, self.center[1] / divisor, self.center[2] / divisor];
                    }
                    self.stage = Puzzle3dFocusSelectionStage::DistanceObjects;
                    return Ok(Self::progress("puzzle3d-focus-selection-distance", "Measuring selection radius", "Auswahlradius wird gemessen"));
                };
                self.object_cursor += 1;
                if self.frames(&object.id) {
                    self.center[0] += object.origin[0];
                    self.center[1] += object.origin[1];
                    self.center[2] += object.origin[2];
                    self.matched += 1;
                }
                Ok(Self::progress("puzzle3d-focus-selection-center", "Scanning selected object", "Ausgewähltes Objekt wird geprüft"))
            }
            Puzzle3dFocusSelectionStage::DistanceObjects => {
                let Some(object) = snapshot.typed().objects.get(self.object_cursor) else {
                    self.stage = Puzzle3dFocusSelectionStage::Publish;
                    return Ok(Self::progress("puzzle3d-focus-selection-publish", "Preparing camera focus", "Kamerafokus wird vorbereitet"));
                };
                self.object_cursor += 1;
                if self.frames(&object.id) {
                    let delta = [object.origin[0] - self.center[0], object.origin[1] - self.center[1], object.origin[2] - self.center[2]];
                    self.maximum_distance = self.maximum_distance.max((delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt());
                }
                Ok(Self::progress("puzzle3d-focus-selection-distance", "Scanning selection radius", "Auswahlradius wird geprüft"))
            }
            Puzzle3dFocusSelectionStage::Publish => {
                self.stage = Puzzle3dFocusSelectionStage::Complete;
                if self.matched == 0 {
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default()));
                }
                let distance = self.maximum_distance * 3.0 + 2.0;
                let mut next = window_ownership::config_from_snapshot(self.window_config.as_ref());
                next.camera.position = [self.center[0] + distance * 0.6, self.center[1] - distance * 0.6, self.center[2] + distance * 0.5];
                next.camera.target = self.center;
                let view = self.view_state.as_ref().ok_or_else(|| Fault::from("puzzle3d-focus-window-context-required"))?;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { window_config_mutations: vec![window_ownership::addressed_config(view, next)?], ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("focusSelection")), ..Default::default() }))
            }
            Puzzle3dFocusSelectionStage::Complete => Err(Fault::from("puzzle3d-focus-selection-complete-repolled")),
            Puzzle3dFocusSelectionStage::Closing => Err(Fault::from("puzzle3d-focus-selection-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dFocusSelectionStage::Closing;
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
        self.stage == Puzzle3dFocusSelectionStage::Closing && self.selected.is_empty() && self.view_state.is_none() && self.window_config.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dRelocateVolumeStage {
    Search,
    Origin,
    Orientation,
    Scale,
    Complete,
    Closing,
}

struct Puzzle3dRelocateVolumeWork {
    stage: Puzzle3dRelocateVolumeStage,
    cursor: usize,
    volume_id: Option<String>,
    mutations: Vec<Puzzle3dMutation>,
}

impl Default for Puzzle3dRelocateVolumeWork {
    fn default() -> Self {
        Self { stage: Puzzle3dRelocateVolumeStage::Search, cursor: 0, volume_id: None, mutations: Vec::with_capacity(3) }
    }
}

impl Puzzle3dRelocateVolumeWork {
    fn complete(&mut self) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        self.stage = Puzzle3dRelocateVolumeStage::Complete;
        crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("relocateTargetVolume")), ..Default::default() })
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dRelocateVolumeWork {
    fn tool_id(&self) -> &'static str {
        "relocateTargetVolume"
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let items = snapshot.typed().target_volumes.len().checked_add(4)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle3dCommand,
        snapshot: &Puzzle3dPlaySnapshot,
        _config: &Puzzle3dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>>, Fault> {
        let requested_id = command.args().and_then(|args| args.get("volumeId")).and_then(Value::as_str).unwrap_or("");
        let after = command.args().and_then(|args| args.get("after"));
        match self.stage {
            Puzzle3dRelocateVolumeStage::Search => {
                let Some(volume) = snapshot.typed().target_volumes.get(self.cursor) else { return Ok(self.complete()) };
                self.cursor += 1;
                if volume.id == requested_id && !volume.locked && after.is_some() {
                    self.volume_id = Some(volume.id.clone());
                    self.stage = Puzzle3dRelocateVolumeStage::Origin;
                }
                Ok(Self::progress("puzzle3d-relocate-volume-search", "Finding target volume", "Zielvolumen wird gesucht"))
            }
            Puzzle3dRelocateVolumeStage::Origin => {
                if let Some(origin) = after.and_then(|after| after.get("position")).and_then(value_as_vec3) {
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::move_target_volume(self.volume_id.clone().ok_or_else(|| Fault::from("puzzle3d-relocate-volume-owner-lost"))?, origin));
                }
                self.stage = Puzzle3dRelocateVolumeStage::Orientation;
                Ok(Self::progress("puzzle3d-relocate-volume-orientation", "Preparing volume rotation", "Volumendrehung wird vorbereitet"))
            }
            Puzzle3dRelocateVolumeStage::Orientation => {
                if let Some(values) = after.and_then(|after| after.get("quaternion")).and_then(Value::as_array).filter(|values| values.len() >= 4) {
                    let orientation =
                        [values.first().and_then(Value::as_f64).unwrap_or(0.0), values.get(1).and_then(Value::as_f64).unwrap_or(0.0), values.get(2).and_then(Value::as_f64).unwrap_or(0.0), values.get(3).and_then(Value::as_f64).unwrap_or(1.0)];
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume(self.volume_id.clone().ok_or_else(|| Fault::from("puzzle3d-relocate-volume-owner-lost"))?, Some(orientation)));
                }
                self.stage = Puzzle3dRelocateVolumeStage::Scale;
                Ok(Self::progress("puzzle3d-relocate-volume-scale", "Preparing volume scale", "Volumenskalierung wird vorbereitet"))
            }
            Puzzle3dRelocateVolumeStage::Scale => {
                if let Some(values) = after.and_then(|after| after.get("scale")).and_then(Value::as_array).filter(|values| values.len() >= 3) {
                    let scale = crate::Puzzle3dScale::Vec3([values.first().and_then(Value::as_f64).unwrap_or(1.0), values.get(1).and_then(Value::as_f64).unwrap_or(1.0), values.get(2).and_then(Value::as_f64).unwrap_or(1.0)]);
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::scale_target_volume(self.volume_id.clone().ok_or_else(|| Fault::from("puzzle3d-relocate-volume-owner-lost"))?, Some(scale)));
                }
                Ok(self.complete())
            }
            Puzzle3dRelocateVolumeStage::Complete => Err(Fault::from("puzzle3d-relocate-volume-complete-repolled")),
            Puzzle3dRelocateVolumeStage::Closing => Err(Fault::from("puzzle3d-relocate-volume-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dRelocateVolumeStage::Closing;
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
        self.stage == Puzzle3dRelocateVolumeStage::Closing && self.mutations.is_empty() && self.volume_id.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dAcceptSuggestionStage {
    Target,
    Candidate,
    Representation,
    Vortices,
    ExistingAttractions,
    PublishObject,
    PublishAttraction,
    PublishResult,
    Complete,
    Closing,
}

struct Puzzle3dAcceptSuggestionWork {
    stage: Puzzle3dAcceptSuggestionStage,
    object_cursor: usize,
    vortex_cursor: usize,
    representation_cursor: usize,
    attraction_cursor: usize,
    target_id: Option<String>,
    target_position: Option<[f64; 3]>,
    kind_index: Option<usize>,
    object_id: Option<String>,
    mesh_url: Option<String>,
    vortices: Vec<crate::Puzzle3dVortex>,
    mutations: Vec<Puzzle3dMutation>,
    window_transient: Option<semio_framework_plugin::WindowTransientSnapshot>,
    view_state: Option<semio_framework_plugin::ViewModel>,
}

impl Default for Puzzle3dAcceptSuggestionWork {
    fn default() -> Self {
        Self {
            stage: Puzzle3dAcceptSuggestionStage::Target,
            object_cursor: 0,
            vortex_cursor: 0,
            representation_cursor: 0,
            attraction_cursor: 0,
            target_id: None,
            target_position: None,
            kind_index: None,
            object_id: None,
            mesh_url: None,
            vortices: Vec::with_capacity(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT),
            mutations: Vec::with_capacity(2),
            window_transient: None,
            view_state: None,
        }
    }
}

impl Puzzle3dAcceptSuggestionWork {
    fn requested_target(command: &Puzzle3dCommand, transient: &window_ownership::Puzzle3dWindowTransient, interaction: &protocol::InteractionState) -> Option<String> {
        command
            .args()
            .and_then(|args| args.get("fullId"))
            .and_then(Value::as_str)
            .map(str::to_string)
            .or_else(|| transient.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.clone()).filter(|id| !id.is_empty()))
            .or_else(|| interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE3D_GRANULARITY_VORTEX).and_then(|selection| selection.ids.first().cloned()))
    }

    /// 🔕️ The popup is ONE-SHOT window-transient state: whichever way this gesture terminates —
    /// placed, refused, target not found, catalogs absent — the accept it answers also retires it.
    /// Without this the menu the user just answered stays painted forever, because nothing else on
    /// the accept route writes the window transient lane.
    fn dismissal(&self) -> EphemeralEmit<EditorApp<Puzzle3dPlayApp>> {
        let transient = window_ownership::transient_from_snapshot(self.window_transient.as_ref());
        if transient.suggestion_menu.is_none() {
            return EphemeralEmit::default();
        }
        let dismissed = window_ownership::Puzzle3dWindowTransient { suggestion_menu: None, ..transient };
        let window_transient = self.view_state.as_ref().and_then(|view| window_ownership::addressed_transient(view, dismissed).ok()).into_iter().collect();
        EphemeralEmit { window_transient, ..Default::default() }
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }
}

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dAcceptSuggestionWork {
    fn tool_id(&self) -> &'static str {
        "acceptSuggestion"
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }

    fn bind_window_owners(&mut self, _config: Option<semio_framework_plugin::WindowConfigSnapshot>, transient: Option<semio_framework_plugin::WindowTransientSnapshot>) {
        self.window_transient = transient;
    }

    fn take_ephemeral(&mut self) -> EphemeralEmit<EditorApp<Puzzle3dPlayApp>> {
        self.dismissal()
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let document = snapshot.typed();
        document.meta.kind_catalogs.as_ref()?;
        let mut object_vortices = 0usize;
        for object in &document.objects {
            object_vortices = object_vortices.checked_add(object.vortices.len())?;
        }
        let target_scan_stage = object_vortices.checked_add(document.objects.len())?.checked_add(1)?;
        let kind_scan_stage = PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT.checked_mul(2)?.checked_add(2)?;
        let items = target_scan_stage.checked_add(1)?.checked_add(kind_scan_stage)?.checked_add(document.attractions.len())?.checked_add(1)?.checked_add(3)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle3dCommand,
        snapshot: &Puzzle3dPlaySnapshot,
        _config: &Puzzle3dConfig,
        interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>>, Fault> {
        let document = snapshot.typed();
        let transient = window_ownership::transient_from_snapshot(self.window_transient.as_ref());
        // 🧯️ The popup always retires (`dismissal`), but a dismissed popup that placed nothing is
        // indistinguishable from a successful one unless the refusal itself speaks — `accept_suggestion_
        // closes_menu_even_when_placement_fails` is literally named after that silence.
        let Some(catalogs) = document.meta.kind_catalogs.as_ref() else {
            self.stage = Puzzle3dAcceptSuggestionStage::Complete;
            return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.placement_unavailable.as_str())));
        };
        match self.stage {
            Puzzle3dAcceptSuggestionStage::Target => {
                let Some(requested) = Self::requested_target(command, &transient, interaction) else {
                    self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.placement_unavailable.as_str())));
                };
                let Some(object) = document.objects.get(self.object_cursor) else {
                    self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.placement_unavailable.as_str())));
                };
                if object.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                    return Err(Fault::from("puzzle3d-accept-target-vortex-capacity"));
                }
                let Some(vortex) = object.vortices.get(self.vortex_cursor) else {
                    self.object_cursor += 1;
                    self.vortex_cursor = 0;
                    return Ok(Self::progress("puzzle3d-accept-target-object", "Scanning target object", "Zielobjekt wird geprüft"));
                };
                self.vortex_cursor += 1;
                if puzzle3d_vortex_full_id(&object.id, &vortex.id) == requested {
                    let rotated = quat_rotate_vector(object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), vortex.position);
                    self.target_id = Some(requested);
                    self.target_position = Some([object.origin[0] + rotated[0], object.origin[1] + rotated[1], object.origin[2] + rotated[2]]);
                    self.stage = Puzzle3dAcceptSuggestionStage::Candidate;
                }
                Ok(Self::progress("puzzle3d-accept-target-vortex", "Scanning target vortex", "Ziel-Vortex wird geprüft"))
            }
            Puzzle3dAcceptSuggestionStage::Candidate => {
                if catalogs.objects.is_empty() {
                    self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.placement_unavailable.as_str())));
                }
                let requested = command.args().and_then(|args| args.get("index")).and_then(Value::as_u64).unwrap_or(transient.brush_candidate_index as u64) as usize;
                let index = requested % catalogs.objects.len();
                let kind = catalogs.objects.get(index).ok_or_else(|| Fault::from("puzzle3d-accept-candidate-cursor"))?;
                if kind.representations.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT || kind.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                    return Err(Fault::from("puzzle3d-accept-candidate-capacity"));
                }
                use std::hash::{Hash, Hasher};
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                self.target_id.hash(&mut hasher);
                kind.id.hash(&mut hasher);
                requested.hash(&mut hasher);
                document.objects.len().hash(&mut hasher);
                self.object_id = Some(format!("puzzle3d.suggestion.{:016x}", hasher.finish()));
                self.kind_index = Some(index);
                self.stage = Puzzle3dAcceptSuggestionStage::Representation;
                Ok(Self::progress("puzzle3d-accept-candidate", "Selecting suggestion candidate", "Vorschlagskandidat wird gewählt"))
            }
            Puzzle3dAcceptSuggestionStage::Representation => {
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-accept-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-accept-kind-cursor"))?;
                let Some(representation) = kind.representations.get(self.representation_cursor) else {
                    self.stage = Puzzle3dAcceptSuggestionStage::Vortices;
                    return Ok(Self::progress("puzzle3d-accept-vortex", "Building suggested vortices", "Vorschlags-Vortices werden aufgebaut"));
                };
                self.representation_cursor += 1;
                if self.mesh_url.is_none() && !representation.url.is_empty() {
                    self.mesh_url = Some(representation.url.clone());
                }
                Ok(Self::progress("puzzle3d-accept-representation", "Scanning candidate mesh", "Kandidaten-Mesh wird geprüft"))
            }
            Puzzle3dAcceptSuggestionStage::Vortices => {
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-accept-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-accept-kind-cursor"))?;
                let Some(template) = kind.vortices.get(self.vortices.len()) else {
                    if self.vortices.is_empty() {
                        self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                        return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.placement_unavailable.as_str())));
                    }
                    self.stage = Puzzle3dAcceptSuggestionStage::ExistingAttractions;
                    return Ok(Self::progress("puzzle3d-accept-existing", "Checking target ownership", "Zielinhaberschaft wird geprüft"));
                };
                let object_id = self.object_id.as_ref().ok_or_else(|| Fault::from("puzzle3d-accept-object-owner"))?;
                let index = self.vortices.len();
                self.vortices.push(crate::Puzzle3dVortex {
                    id: format!("{object_id}:v{index}"),
                    label: None,
                    vortex_kind: template.vortex_kind.clone(),
                    position: template.point,
                    direction: Some(template.direction),
                    radius: template.radius,
                    hidden: false,
                    locked: false,
                });
                Ok(Self::progress("puzzle3d-accept-vortex", "Building one suggested vortex", "Ein Vorschlags-Vortex wird aufgebaut"))
            }
            Puzzle3dAcceptSuggestionStage::ExistingAttractions => {
                let target = self.target_id.as_ref().ok_or_else(|| Fault::from("puzzle3d-accept-target-owner"))?;
                let Some(attraction) = document.attractions.get(self.attraction_cursor) else {
                    self.stage = Puzzle3dAcceptSuggestionStage::PublishObject;
                    return Ok(Self::progress("puzzle3d-accept-publish-object", "Preparing suggested object", "Vorschlagsobjekt wird vorbereitet"));
                };
                self.attraction_cursor += 1;
                if attraction.attracting == *target || attraction.attracted == *target {
                    self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle3d_notice_emit(self.view_state.as_ref(), |labels| labels.placement_occupied.as_str())));
                }
                Ok(Self::progress("puzzle3d-accept-existing", "Scanning existing attraction", "Bestehende Anziehung wird geprüft"))
            }
            Puzzle3dAcceptSuggestionStage::PublishObject => {
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-accept-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-accept-kind-cursor"))?;
                let object = crate::Puzzle3dObject {
                    id: self.object_id.clone().ok_or_else(|| Fault::from("puzzle3d-accept-object-owner"))?,
                    label: None,
                    object_kind: Some(kind.id.clone()),
                    anchor: Default::default(),
                    origin: self.target_position.ok_or_else(|| Fault::from("puzzle3d-accept-position-owner"))?,
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    mesh_url: self.mesh_url.clone(),
                    vortices: std::mem::take(&mut self.vortices),
                    hidden: false,
                    locked: false,
                };
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::create_object(object, None));
                self.stage = Puzzle3dAcceptSuggestionStage::PublishAttraction;
                Ok(Self::progress("puzzle3d-accept-publish-object", "Transferring suggested object", "Vorschlagsobjekt wird übertragen"))
            }
            Puzzle3dAcceptSuggestionStage::PublishAttraction => {
                let target = self.target_id.take().ok_or_else(|| Fault::from("puzzle3d-accept-target-owner"))?;
                let object_id = self.object_id.take().ok_or_else(|| Fault::from("puzzle3d-accept-object-owner"))?;
                let source = format!("{object_id}:v0");
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_vortices(format!("attraction-{target}-{source}"), target, source, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
                self.stage = Puzzle3dAcceptSuggestionStage::PublishResult;
                Ok(Self::progress("puzzle3d-accept-publish-attraction", "Transferring suggested attraction", "Vorschlagsanziehung wird übertragen"))
            }
            Puzzle3dAcceptSuggestionStage::PublishResult => {
                self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), config_mutations: Vec::new(), ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("acceptSuggestion")), ..Default::default() }))
            }
            Puzzle3dAcceptSuggestionStage::Complete => Err(Fault::from("puzzle3d-accept-complete-repolled")),
            Puzzle3dAcceptSuggestionStage::Closing => Err(Fault::from("puzzle3d-accept-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dAcceptSuggestionStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.vortices.pop().is_some()
            || self.mutations.pop().is_some()
            || self.target_id.take().is_some()
            || self.target_position.take().is_some()
            || self.object_id.take().is_some()
            || self.mesh_url.take().is_some()
            || self.window_transient.take().is_some()
            || self.view_state.take().is_some()
        {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle3dAcceptSuggestionStage::Closing
            && self.vortices.is_empty()
            && self.mutations.is_empty()
            && self.target_id.is_none()
            && self.target_position.is_none()
            && self.object_id.is_none()
            && self.mesh_url.is_none()
            && self.window_transient.is_none()
            && self.view_state.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dPrecomputeCommandStage {
    Decode,
    Objects,
    Vortices,
    Attractions,
    CatalogObjects,
    CatalogVortices,
    Positions,
    Indices,
    FillPrepare,
    FillPlan,
    FillApply,
    PrologueScene,
    PrologueSync,
    Publish,
    Complete,
    Closing,
}

struct Puzzle3dPrecomputeCommandWork {
    tool_id: &'static str,
    stage: Puzzle3dPrecomputeCommandStage,
    object_cursor: usize,
    child_cursor: usize,
    attraction_cursor: usize,
    catalog_object_cursor: usize,
    catalog_vortex_cursor: usize,
    payload_cursor: usize,
    requested_count: u32,
    delta: isize,
    candidate_count: usize,
    processed_units: usize,
    /// 🪪️ Session key of the admission that built this work — bound by `build_tool_job` on every
    /// admission and every worker-hop resume, so the precompute-gated actions reach the same slot.
    session: Option<(u32, String)>,
    view_state: Option<semio_framework_plugin::ViewModel>,
    window_config: Option<semio_framework_plugin::WindowConfigSnapshot>,
    window_transient: Option<semio_framework_plugin::WindowTransientSnapshot>,
    ephemeral: Option<EphemeralEmit<EditorApp<Puzzle3dPlayApp>>>,
    fill_precompute: Option<Puzzle3dPrecomputeSession>,
    fill_mutations: Vec<Puzzle3dMutation>,
    prologue: Puzzle3dActionPrologue,
}

impl Puzzle3dPrecomputeCommandWork {
    fn session(&self) -> Option<(u32, Option<String>)> {
        self.session.as_ref().map(|(app_instance_id, document_id)| (*app_instance_id, Some(document_id.clone())))
    }

    fn new(tool_id: &'static str) -> Self {
        Self {
            tool_id,
            stage: Puzzle3dPrecomputeCommandStage::Decode,
            object_cursor: 0,
            child_cursor: 0,
            attraction_cursor: 0,
            catalog_object_cursor: 0,
            catalog_vortex_cursor: 0,
            payload_cursor: 0,
            requested_count: 0,
            delta: if tool_id == "cycleBrushCandidateBack" { -1 } else { 1 },
            candidate_count: 0,
            processed_units: 0,
            session: None,
            view_state: None,
            window_config: None,
            window_transient: None,
            ephemeral: None,
            fill_precompute: None,
            fill_mutations: Vec::new(),
            prologue: Puzzle3dActionPrologue::default(),
        }
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    /// 🔤️ Validates one bounded slice of a `registerBrushMesh` page's base64 payload — the alphabet and,
    /// once the stream ends, its four-character grouping. `payload_cursor` returns to zero exactly when
    /// the stream is fully scanned, which is what advances the stage.
    fn scan_mesh_page(&mut self, command: &Puzzle3dCommand, key: &str) -> bool {
        let payload = command.args().and_then(|args| args.get(key)).and_then(Value::as_str).unwrap_or_default().as_bytes();
        let start = self.payload_cursor.min(payload.len());
        let end = payload.len().min(start.saturating_add(PUZZLE3D_MESH_PAGE_SCAN_CHARS));
        let admissible = payload[start..end].iter().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'/' | b'='));
        self.payload_cursor = if end >= payload.len() { 0 } else { end };
        admissible && payload.len().is_multiple_of(4)
    }
}

/// 🔤️ Base64 characters one interactive step of a `registerBrushMesh` page validates. A whole page is
/// at most [`crate::editor::puzzle3d::precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS`] characters, so a
/// page costs at most eleven steps per stream and no step approaches the lane's own budget.
const PUZZLE3D_MESH_PAGE_SCAN_CHARS: usize = 512;

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dPrecomputeCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn bind_instance(&mut self, app_instance_id: u32, parent_document_id: &str) {
        self.session = Some((app_instance_id, parent_document_id.to_string()));
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }

    fn bind_window_owners(&mut self, config: Option<semio_framework_plugin::WindowConfigSnapshot>, transient: Option<semio_framework_plugin::WindowTransientSnapshot>) {
        self.window_config = config;
        self.window_transient = transient;
    }

    fn take_ephemeral(&mut self) -> EphemeralEmit<EditorApp<Puzzle3dPlayApp>> {
        self.ephemeral.take().unwrap_or_default()
    }

    /// 📏️ `registerBrushMesh` carries one page of a mesh upload run as two base64 payload strings, never
    /// as JSON number arrays — a whole document-scale mesh is 64 KB of them and the shared retained wire
    /// admits 8 192 bytes. The extent a page claims is the characters it must validate, so a page that
    /// declares more than [`crate::editor::puzzle3d::precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS`] per
    /// stream is refused before a single character is read.
    fn extent(&self, command: &Puzzle3dCommand, _snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        let payload = |key: &str| command.args().and_then(|args| args.get(key)).and_then(Value::as_str).map_or(0, str::len);
        let (positions, indices) = (payload("positionsB64"), payload("indicesB64"));
        let maximum = crate::editor::puzzle3d::precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS;
        let items = positions.max(indices).div_ceil(PUZZLE3D_MESH_PAGE_SCAN_CHARS).checked_mul(2)?.checked_add(1)?;
        (positions <= maximum && indices <= maximum && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &Puzzle3dCommand,
        snapshot: &Puzzle3dPlaySnapshot,
        config: &Puzzle3dConfig,
        interaction: &protocol::InteractionState,
        hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle3dPlayApp>>, Fault> {
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle3d-precompute-work-capacity"));
        }
        self.processed_units += 1;
        let document = snapshot.typed();
        match self.stage {
            Puzzle3dPrecomputeCommandStage::Decode => {
                self.requested_count = set_fill_count::parse_count(command.args());
                self.delta = command.args().and_then(|args| args.get("delta")).and_then(Value::as_i64).map_or(self.delta, |value| value.clamp(isize::MIN as i64, isize::MAX as i64) as isize);
                self.stage = match self.tool_id {
                    "registerBrushMesh" => Puzzle3dPrecomputeCommandStage::Positions,
                    "cancelFillBuild" => Puzzle3dPrecomputeCommandStage::PrologueScene,
                    _ => Puzzle3dPrecomputeCommandStage::Objects,
                };
                Ok(Self::progress("puzzle3d-precompute-decode", "Reading precompute command", "Vorberechnungsbefehl wird gelesen"))
            }
            Puzzle3dPrecomputeCommandStage::Objects => {
                let Some(object) = document.objects.get(self.object_cursor) else {
                    self.stage = Puzzle3dPrecomputeCommandStage::Attractions;
                    return Ok(Self::progress("puzzle3d-precompute-attractions", "Scanning attraction owner", "Anziehungsinhaber wird geprüft"));
                };
                if object.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                    return Err(Fault::from("puzzle3d-precompute-vortex-capacity"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::Vortices;
                Ok(Self::progress("puzzle3d-precompute-object", "Scanning object owner", "Objektinhaber wird geprüft"))
            }
            Puzzle3dPrecomputeCommandStage::Vortices => {
                let object = document.objects.get(self.object_cursor).ok_or_else(|| Fault::from("puzzle3d-precompute-object-cursor"))?;
                if object.vortices.get(self.child_cursor).is_some() {
                    self.child_cursor += 1;
                    return Ok(Self::progress("puzzle3d-precompute-vortex", "Scanning one vortex owner", "Ein Vortexinhaber wird geprüft"));
                }
                self.object_cursor += 1;
                self.child_cursor = 0;
                self.stage = Puzzle3dPrecomputeCommandStage::Objects;
                Ok(Self::progress("puzzle3d-precompute-object", "Advancing object cursor", "Objektzeiger wird fortgesetzt"))
            }
            Puzzle3dPrecomputeCommandStage::Attractions => {
                if document.attractions.get(self.attraction_cursor).is_some() {
                    self.attraction_cursor += 1;
                    return Ok(Self::progress("puzzle3d-precompute-attraction", "Scanning one attraction owner", "Ein Anziehungsinhaber wird geprüft"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::CatalogObjects;
                Ok(Self::progress("puzzle3d-precompute-catalog-object", "Scanning object kind owner", "Objektartinhaber wird geprüft"))
            }
            Puzzle3dPrecomputeCommandStage::CatalogObjects => {
                let entries = document.meta.kind_catalogs.as_ref().map(|catalogs| catalogs.objects.as_slice()).unwrap_or_default();
                if entries.get(self.catalog_object_cursor).is_some() {
                    self.catalog_object_cursor += 1;
                    self.candidate_count += 1;
                    return Ok(Self::progress("puzzle3d-precompute-catalog-object", "Scanning one object kind", "Eine Objektart wird geprüft"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::CatalogVortices;
                Ok(Self::progress("puzzle3d-precompute-catalog-vortex", "Scanning vortex kind owner", "Vortexartinhaber wird geprüft"))
            }
            Puzzle3dPrecomputeCommandStage::CatalogVortices => {
                let entries = document.meta.kind_catalogs.as_ref().map(|catalogs| catalogs.vortices.as_slice()).unwrap_or_default();
                if entries.get(self.catalog_vortex_cursor).is_some() {
                    self.catalog_vortex_cursor += 1;
                    return Ok(Self::progress("puzzle3d-precompute-catalog-vortex", "Scanning one vortex kind", "Eine Vortexart wird geprüft"));
                }
                self.stage = if self.tool_id == "setFillCount" { Puzzle3dPrecomputeCommandStage::FillPrepare } else { Puzzle3dPrecomputeCommandStage::PrologueScene };
                Ok(Self::progress("puzzle3d-precompute-transfer", "Transferring precompute census", "Vorberechnungszensus wird übertragen"))
            }
            Puzzle3dPrecomputeCommandStage::Positions => {
                if !self.scan_mesh_page(command, "positionsB64") {
                    return Err(Fault::from("puzzle3d-register-mesh-position-malformed"));
                }
                if self.payload_cursor > 0 {
                    return Ok(Self::progress("puzzle3d-register-mesh-position", "Reading one mesh position page", "Eine Mesh-Positionsseite wird gelesen"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::Indices;
                Ok(Self::progress("puzzle3d-register-mesh-index", "Reading mesh indices", "Mesh-Indizes werden gelesen"))
            }
            Puzzle3dPrecomputeCommandStage::Indices => {
                if !self.scan_mesh_page(command, "indicesB64") {
                    return Err(Fault::from("puzzle3d-register-mesh-index-malformed"));
                }
                if self.payload_cursor > 0 {
                    return Ok(Self::progress("puzzle3d-register-mesh-index", "Reading one mesh index page", "Eine Mesh-Indexseite wird gelesen"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::PrologueScene;
                Ok(Self::progress("puzzle3d-register-mesh-transfer", "Transferring validated mesh owner", "Geprüfter Mesh-Inhaber wird übertragen"))
            }
            Puzzle3dPrecomputeCommandStage::FillPrepare => {
                let view = self.view_state.as_ref().ok_or_else(|| Fault::from("puzzle3d-fill-window-context-required"))?;
                let window_config = window_ownership::config_from_snapshot(self.window_config.as_ref());
                let window_transient = window_ownership::transient_from_snapshot(self.window_transient.as_ref());
                let runtime = window_ownership::runtime(config, &window_config, &window_transient, Some(view));
                let window_id = puzzle3d_addressed_window_id(Some(view), None, command.window_id(), &runtime.window_ids);
                let active_utility = puzzle3d_scene_active_utility(&runtime, Some(view), Some(window_id));
                let scene = scene_from_projection(&puzzle3d_projection_value(snapshot.value()), runtime, &active_utility);
                let mut precompute = Puzzle3dPrecomputeSession::new();
                sync_precompute_session(&mut precompute, &scene);
                precompute.set_fill_applied_count(config.fill_count);
                self.fill_precompute = Some(precompute);
                self.stage = Puzzle3dPrecomputeCommandStage::FillPlan;
                Ok(Self::progress("puzzle3d-fill-plan", "Preparing retained fill plan", "Beibehaltener Füllplan wird vorbereitet"))
            }
            Puzzle3dPrecomputeCommandStage::FillPlan => {
                let precompute = self.fill_precompute.as_mut().ok_or_else(|| Fault::from("puzzle3d-fill-plan-owner"))?;
                let available = precompute.fill_available_count();
                if available > 0 || precompute.fill_is_done() {
                    self.requested_count = self.requested_count.min(available);
                    self.stage = Puzzle3dPrecomputeCommandStage::FillApply;
                } else {
                    precompute.precompute_step_lane(crate::standards::v1::subsets::any::schema::PrecomputeLane::Fill, 1);
                    let required = self.requested_count.max(config.fill_count);
                    if precompute.fill_available_count() >= required || precompute.fill_is_done() {
                        self.requested_count = self.requested_count.min(precompute.fill_available_count());
                        self.stage = Puzzle3dPrecomputeCommandStage::FillApply;
                    }
                }
                Ok(Self::progress("puzzle3d-fill-plan", "Advancing retained fill plan", "Beibehaltener Füllplan wird fortgesetzt"))
            }
            Puzzle3dPrecomputeCommandStage::FillApply => {
                let precompute = self.fill_precompute.as_mut().ok_or_else(|| Fault::from("puzzle3d-fill-apply-owner"))?;
                let (applied, mutations) = set_fill_count::apply_chunk(precompute, self.requested_count).ok_or_else(|| Fault::from("puzzle3d-fill-apply-unavailable"))?;
                let progressed = !mutations.is_empty();
                self.fill_mutations.extend(mutations);
                if applied == self.requested_count || !progressed {
                    self.stage = Puzzle3dPrecomputeCommandStage::Publish;
                }
                Ok(Self::progress("puzzle3d-fill-apply", "Applying one retained fill placement", "Eine beibehaltene Füllplatzierung wird angewendet"))
            }
            Puzzle3dPrecomputeCommandStage::PrologueScene => {
                let view = self.view_state.as_ref().ok_or_else(|| Fault::from("puzzle3d-precompute-window-context-required"))?;
                let window_config = window_ownership::config_from_snapshot(self.window_config.as_ref());
                let window_transient = window_ownership::transient_from_snapshot(self.window_transient.as_ref());
                let runtime = window_ownership::runtime(config, &window_config, &window_transient, Some(view));
                let window_id = puzzle3d_addressed_window_id(Some(view), None, command.window_id(), &runtime.window_ids);
                self.prologue.scene_step(command.action_id(), snapshot, &runtime, Some(view), Some(window_id));
                self.stage = Puzzle3dPrecomputeCommandStage::PrologueSync;
                Ok(Self::progress("puzzle3d-action-scene", "Reading the document", "Dokument wird gelesen"))
            }
            Puzzle3dPrecomputeCommandStage::PrologueSync => {
                let view = self.view_state.as_ref().ok_or_else(|| Fault::from("puzzle3d-precompute-window-context-required"))?;
                let window_config = window_ownership::config_from_snapshot(self.window_config.as_ref());
                let window_transient = window_ownership::transient_from_snapshot(self.window_transient.as_ref());
                let runtime = window_ownership::runtime(config, &window_config, &window_transient, Some(view));
                let window_id = puzzle3d_addressed_window_id(Some(view), None, command.window_id(), &runtime.window_ids);
                let session = self.session();
                let owed = with_puzzle3d_app_for(session, &runtime, |app| self.prologue.sync_step(app, command.action_id(), &runtime, Some(view), Some(window_id)));
                if !owed {
                    self.stage = Puzzle3dPrecomputeCommandStage::Publish;
                }
                Ok(Self::progress("puzzle3d-action-sync", "Preparing the placement session", "Platzierungssitzung wird vorbereitet"))
            }
            Puzzle3dPrecomputeCommandStage::Publish => {
                if self.tool_id == "setFillCount" {
                    let config_mutations = (self.requested_count != config.fill_count).then_some(Puzzle3dConfigMutation::SetFillCount { count: self.requested_count }).into_iter().collect();
                    self.stage = Puzzle3dPrecomputeCommandStage::Complete;
                    return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit {
                        artifact_mutations: std::mem::take(&mut self.fill_mutations),
                        config_mutations,
                        coalesce_key: Some("fill-count".into()),
                        ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("setFillCount")),
                        ..Default::default()
                    }));
                }
                let view = self.view_state.as_ref().ok_or_else(|| Fault::from("puzzle3d-precompute-window-context-required"))?;
                let window_config = window_ownership::config_from_snapshot(self.window_config.as_ref());
                let window_transient = window_ownership::transient_from_snapshot(self.window_transient.as_ref());
                let runtime = window_ownership::runtime(config, &window_config, &window_transient, Some(view));
                let snapshot_interaction = Puzzle3dInteractionSnapshot::from_state(interaction, hover);
                let window_id = puzzle3d_addressed_window_id(Some(view), None, command.window_id(), &runtime.window_ids);
                let session = self.session();
                let (emit, ephemeral) = with_puzzle3d_app_for(session, &runtime, |app| self.prologue.dispatch_step(app, command, Some(window_id), &runtime, Some(view), &snapshot_interaction));
                self.ephemeral = Some(ephemeral);
                self.stage = Puzzle3dPrecomputeCommandStage::Complete;
                Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(emit))
            }
            Puzzle3dPrecomputeCommandStage::Complete => Err(Fault::from("puzzle3d-precompute-complete-repolled")),
            Puzzle3dPrecomputeCommandStage::Closing => Err(Fault::from("puzzle3d-precompute-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dPrecomputeCommandStage::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.view_state.take().is_some()
            || self.window_config.take().is_some()
            || self.window_transient.take().is_some()
            || self.ephemeral.take().is_some()
            || self.session.take().is_some()
            || self.fill_precompute.take().is_some()
            || self.fill_mutations.pop().is_some()
            || self.prologue.close_one()
        {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle3dPrecomputeCommandStage::Closing
            && self.view_state.is_none()
            && self.window_config.is_none()
            && self.window_transient.is_none()
            && self.ephemeral.is_none()
            && self.session.is_none()
            && self.fill_precompute.is_none()
            && self.fill_mutations.is_empty()
            && self.prologue.is_empty()
    }
}

const PUZZLE3D_IMPORT_RAW_BYTES: usize = 262_144;
const PUZZLE3D_IMPORT_DECODED_ITEMS: usize = 16_384;

struct Puzzle3dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
    contract: semio_framework::ToolExecutionContract,
}

impl Puzzle3dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self {
            keys: PUZZLE3D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect(),
            contract: semio_framework::ToolExecutionContract::resumable(PUZZLE3D_IMPORT_RAW_BYTES, PUZZLE3D_IMPORT_DECODED_ITEMS, 1, crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES, crate::retained_command::PUZZLE_COMMAND_STEP_MICROS, 1, 1),
        }
    }
}

impl ToolJobFactory for Puzzle3dRetainedCommandJobFactory {
    type Payload = crate::retained_command::RetainedPuzzleCommandPayload<EditorApp<Puzzle3dPlayApp>>;
    type Job = crate::retained_command::RetainedPuzzleCommandJob<EditorApp<Puzzle3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PUZZLE3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        self.contract
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
        if input.declared_bytes() > self.contract.max_raw_wire_bytes {
            return Err((ToolJobFactoryError::new("Puzzle 3d retained command rejects an oversized wire owner"), input, checkpoint));
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

impl ArtifactOwnedToolJobFactory for Puzzle3dRetainedCommandJobFactory {
    type Owner = EditorApp<Puzzle3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PUZZLE3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE3D_FIXTURE_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "openAddObjectDialog", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "transformBegin", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "transformEnd", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setFillCount", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "addTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "acceptSuggestion", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "addBrushObject", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "addObjectKind", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "createAttraction", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteAttraction", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "deleteTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "duplicateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "exportFixture", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importFixture", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "openImportFixture", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "patchInspector", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetVolumeFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "worldRelocate", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "closeVortexSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidate", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidateBack", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementAbort", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementControlSelect", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementRepeatLast", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "fillBuildTick", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "cancelFillBuild", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "focusSelection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "hoverSuggestion", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "openVortexSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "registerBrushMesh", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "relocateTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "selectSameKindSelection", lanes: &[ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "setBrushPlacementOverlapBudget", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setChunkSize", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridSnapEnabled", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridSpacing", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridVisible", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setPanelPage", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodAutomatic", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodDepthVariable", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodManual", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setObjectKindWeight", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setProjection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setProjectionParam", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setProximityRadius", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSelectableKind", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setTransformGumballFlag", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setVortexDirection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setVortexKindWeight", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setVortexShow", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setVoxelDims", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "suggestionsTick", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
const PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES: usize = 32_768;

struct Puzzle3dConfigStorePreparation {
    base: Option<store::SnapshotRead<Puzzle3dConfig>>,
    mutation: Option<Puzzle3dConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(Puzzle3dConfig, Vec<Puzzle3dConfigMutation>, Puzzle3dConfigMutation, usize)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Puzzle3dConfig, Puzzle3dConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    phase: u8,
    cancelled: bool,
    closing: bool,
}

struct Puzzle3dConfigStorePreparationFactory;

/// 🌉️ `serde_json::to_writer` streamed into a byte-counting `Write` sink so an oversize config could
/// abort mid-encode without materializing it; the in-house `dsl::json::to_json_string` has no
/// streaming writer, so this measures the fully-encoded string's byte length against the same
/// `PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES` bound instead — same bound, same error, one buffer instead
/// of zero (`Puzzle3dConfig` is a small, fixed-shape record, never large enough for this to matter).
fn puzzle3d_config_store_bounded_bytes(value: &Puzzle3dConfig) -> Result<usize, String> {
    let encoded = to_json_string(value);
    if encoded.len() > PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Puzzle3d Config Store root exceeds its fixed envelope".to_string());
    }
    Ok(encoded.len())
}

/// 📏️ The exact retained byte cost of one Config mutation, derived from its own encoded payload and
/// bounded by the single `PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES` envelope every Config lane publication
/// obeys. Every `Puzzle3dConfigMutation` variant is admissible: an allowlist here silently killed the
/// whole `Puzzle3dScalarConfigWork` family (camera, projection, sun, LOD, grid, selectable kinds,
/// proximity radius, chunk size, voxel dims, gumball flags, vortex show/direction, overlap budget,
/// suggestion menu, engagement input) plus both kind-weight routes, because
/// `Puzzle3dConfigStorePreparation::advance` rejects whatever this returns `None` for and the
/// operation's publication lease is cancelled behind it. Only an oversize payload is refused now, and
/// `Snapshot`'s cost stays the whole-config cost it always was — its payload *is* the config.
fn puzzle3d_config_store_mutation_bytes(mutation: &Puzzle3dConfigMutation) -> Option<usize> {
    let encoded = to_json_string(mutation).len();
    (encoded <= PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES).then_some(encoded)
}

fn puzzle3d_config_store_edit(forward: Puzzle3dConfigMutation, inverse: Vec<Puzzle3dConfigMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<Puzzle3dConfigMutation> {
    let id = format!("puzzle3d-config-retained-{}", authority.next_sequence_number());
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

impl store::ArtifactStoreOneItemPreparation<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparation {
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
                let base = self.base.as_ref().ok_or_else(|| "Puzzle3d Config preparation lost its exact base root".to_string())?;
                let mutation = self.mutation.take().ok_or_else(|| "Puzzle3d Config preparation lost its mutation owner".to_string())?;
                if puzzle3d_config_store_mutation_bytes(&mutation).is_none() {
                    return Err("Puzzle3d Config preparation rejected its exact mutation envelope".into());
                }
                let completed_bytes = puzzle3d_config_store_bounded_bytes(base.get())?;
                let inverse = mutation.inverse(base.get());
                let post = mutation.diff(base.get()).into_parts().0.apply(base.get()).map_err(|_| "Puzzle3d Config mutation could not produce its post root".to_string())?;
                self.candidate = Some((post, inverse, mutation, completed_bytes));
                self.phase = 1;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: completed_bytes as u64, digest: [0; 32] };
                Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            1 => {
                let (post, inverse, mutation, completed_bytes) = self.candidate.take().ok_or_else(|| "Puzzle3d Config preparation lost its semantic candidate".to_string())?;
                let authority = self.authority.as_ref().ok_or_else(|| "Puzzle3d Config preparation lost its Store authority".to_string())?;
                let prepared = authority.prepare_one_item(puzzle3d_config_store_edit(mutation, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
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

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Puzzle3dConfig, Puzzle3dConfigMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Puzzle3dConfig, Puzzle3dConfigMutation>> {
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
                return Err("Puzzle3d Config preparation could not return its exact base root".into());
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

impl store::ArtifactStoreOneItemPreparationFactory<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparationFactory {
    fn preflight(&self, mutation: &Puzzle3dConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Puzzle3d Config preparation rejected its lane or description".into());
        }
        let retained_bytes = puzzle3d_config_store_mutation_bytes(mutation).ok_or_else(|| "Puzzle3d Config preparation rejected its exact mutation".to_string())?;
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Puzzle3dConfig, Puzzle3dConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Puzzle3dConfig, Puzzle3dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Puzzle3dConfig, Puzzle3dConfigMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Puzzle3dConfigStorePreparation {
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

// 🧩️ The Artifact-lane (`Puzzle3dPlaySnapshot`/`Puzzle3dMutation`) sibling of the preparation
// factory above — mirrors `Puzzle5dStorePreparationFactory`
// (`🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`), the one other puzzle-family
// app whose retained commands publish to the Artifact lane through this same trait. Unlike the
// Config preparation above, an arbitrary `Puzzle3dMutation` variant is admitted generically via
// `protocol::Mutation`/`protocol::MutationDiff` rather than an allowlist match, because
// `setActiveExample`'s work loop (`Puzzle3dSetActiveExampleWork`) emits many different mutation
// kinds (delete/create object, attraction, target volume, reference, compatibility, domain,
// catalogs). Each kind is batched to `PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK` and the Complete emit
// is one coalesced document-replacement gesture (`set-active-example`), not one store commit per item.
struct Puzzle3dArtifactStorePreparationFactory;

struct Puzzle3dArtifactStorePreparation {
    base: Option<store::SnapshotRead<Puzzle3dPlaySnapshot>>,
    mutation: Option<Puzzle3dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(Puzzle3dPlaySnapshot, Vec<Puzzle3dMutation>, Puzzle3dMutation)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Puzzle3dPlaySnapshot, Puzzle3dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    phase: u8,
    cancelled: bool,
    closing: bool,
}

fn puzzle3d_artifact_store_edit(forward: Puzzle3dMutation, inverse: Vec<Puzzle3dMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<Puzzle3dMutation> {
    let id = format!("puzzle3d-artifact-retained-{}", authority.next_sequence_number());
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

impl store::ArtifactStoreOneItemPreparationFactory<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparationFactory {
    fn preflight(&self, _mutation: &Puzzle3dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Puzzle3d Artifact preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Puzzle3dPlaySnapshot, Puzzle3dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Puzzle3dPlaySnapshot, Puzzle3dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Puzzle3dPlaySnapshot, Puzzle3dMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Puzzle3dArtifactStorePreparation {
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

impl store::ArtifactStoreOneItemPreparation<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparation {
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
                let base = self.base.as_ref().ok_or_else(|| "Puzzle3d Artifact preparation lost its exact base root".to_string())?;
                let mutation = self.mutation.take().ok_or_else(|| "Puzzle3d Artifact preparation lost its mutation owner".to_string())?;
                let inverse = mutation.inverse(base.get());
                let post = mutation.diff(base.get()).into_parts().0.apply(base.get()).map_err(|_| "Puzzle3d Artifact mutation could not produce its post root".to_string())?;
                self.candidate = Some((post, inverse, mutation));
                self.phase = 1;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: [0; 32] };
                Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            1 => {
                let (post, inverse, mutation) = self.candidate.take().ok_or_else(|| "Puzzle3d Artifact preparation lost its semantic candidate".to_string())?;
                let authority = self.authority.as_ref().ok_or_else(|| "Puzzle3d Artifact preparation lost its Store authority".to_string())?;
                let prepared = authority.prepare_one_item(puzzle3d_artifact_store_edit(mutation, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
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

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Puzzle3dPlaySnapshot, Puzzle3dMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Puzzle3dPlaySnapshot, Puzzle3dMutation>> {
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
                return Err("Puzzle3d Artifact preparation could not return its exact base root".into());
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

//#region 📜️ToolProofs
/// 📜️ The retained command catalog's own bounded first-step proofs — one per
/// `PUZZLE3D_RETAINED_TOOL_IDS` entry, all joined to the single concrete
/// `Puzzle3dRetainedCommandJobFactory`.
struct Puzzle3dRetainedCommandProofs;

impl Puzzle3dRetainedCommandProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle3d@1/*#editor",
        document_schema: "puzzle.3d.fixture",
        factory: "Puzzle3dRetainedCommandJobFactory",
        factory_type: Puzzle3dRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(262_144, 16_384, 1, 262_144, 7_500, 1, 1),
        tools: [
            "openAddObjectDialog", "worldPointerDown", "transformBegin", "transformEnd", "setActiveExample", "setFillCount",
            "addTargetVolume",
            "acceptSuggestion", "addBrushObject", "addObjectKind", "createAttraction", "deleteAttraction", "deleteSelection", "deleteTargetVolume", "duplicateSelection", "exportFixture", "importFixture", "openImportFixture", "patchInspector", "rotateSelection", "scaleSelection", "setSelectionFlag", "setTargetVolumeFlag", "translateSelection", "worldRelocate", "relocateTargetVolume",
            "closeVortexSuggestions", "cycleBrushCandidate", "cycleBrushCandidateBack", "engagementAbort", "engagementControlSelect", "engagementInput", "engagementRepeatLast", "engagementSubmit", "cancelFillBuild", "fillBuildTick", "focusSelection", "hoverSuggestion", "openVortexSuggestions", "registerBrushMesh", "selectSameKindSelection", "setBrushPlacementOverlapBudget", "setCamera", "setChunkSize", "setGridSnapEnabled", "setGridSpacing", "setGridVisible", "setPanelPage", "setLodAutomatic", "setLodDepthVariable", "setLodManual", "setObjectKindWeight", "setProjection", "setProjectionParam", "setProximityRadius", "setSelectableKind", "setSunAzimuth", "setSunElevation", "setSunIntensity", "setTransformGumballFlag", "setVortexDirection", "setVortexKindWeight", "setVortexShow", "setVoxelDims", "suggestionsTick", "toggleSun",
        ]
    }
}

/// 📜️ The two framework-injected host-configuration verbs. Deliberately GENERIC proofs (no
/// `factory_type`): they are not app-owned retained tools — `Puzzle3dRetainedCommandJobFactory` never
/// claims them and `retained_command_catalog_excludes_framework_owned_shared_actions` pins that — they
/// only need the wire admission and output budget `dispatch_action`'s host-configuration branch asks
/// for before it applies `host_configuration_mutation`'s single Config mutation.
struct Puzzle3dHostConfigurationProofs;

impl Puzzle3dHostConfigurationProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle3d@1/*#editor",
        document_schema: "puzzle.3d.fixture",
        factory: "BoundedFirstStepCommandJobFactory",
        contract: semio_framework::ToolExecutionContract::resumable(8_192, 8, 1, 8_192, 7_500, 1, 1),
        tools: ["setActiveTool", "setActiveUtility"]
    }
}
//#endregion 📜️ToolProofs


/// 📋️ One-step reserved copy/cut/paste job — the framework route is an empty stub unless the app owns this producer.
struct Puzzle3dClipboardJob {
    tool_id: String,
    snapshot: std::sync::Arc<Puzzle3dPlaySnapshot>,
    raw_wire: Vec<u8>,
    input: Option<ArtifactReservedToolInput>,
    completion: Option<semio_framework_plugin::app::ArtifactToolCompletion<EditorApp<Puzzle3dPlayApp>>>,
    closing: bool,
}

impl Puzzle3dClipboardJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle3dPlayApp>>) -> Self {
        Self { tool_id: request.tool_id, snapshot: request.snapshot, raw_wire: request.raw_wire, input: Some(request.input), completion: Some(request.completion), closing: false }
    }

    fn emit(&mut self) -> Emit<Puzzle3dMutation, Puzzle3dConfigMutation, NoDraftMutation> {
        let ArtifactReservedToolInput::Action { args, interaction, hover } = self.input.take().expect("puzzle3d clipboard input") else {
            return Emit::default();
        };
        let fixture = puzzle3d_fixture_from_play_snapshot(self.snapshot.as_ref());
        let marks = Puzzle3dInteractionSnapshot::from_state(&interaction, &hover);
        match self.tool_id.as_str() {
            "copy" => match puzzle3d_copy_fragment_from(&fixture, puzzle3d_selected_objects_from(&marks, &fixture)) {
                Ok(fragment) => Emit { effects: vec![Effect::ClipboardWrite { fragment }], ..Default::default() },
                Err(_) => Emit::default(),
            },
            "cut" => {
                let objects = puzzle3d_selected_objects_from(&marks, &fixture);
                let Ok(fragment) = puzzle3d_copy_fragment_from(&fixture, objects) else { return Emit::default() };
                let mutations = puzzle3d_cut_operations_from(&fixture, &marks).unwrap_or_default();
                Emit { artifact_mutations: mutations, effects: vec![Effect::ClipboardWrite { fragment }], ..Default::default() }
            },
            "paste" => {
                let Some(args) = args else { return Emit::default() };
                let Some(fragment) = args.get("fragment").and_then(|value| dsl::FromValue::from_value(value.clone()).ok()) else { return Emit::default() };
                let placement = PastePlacement::default();
                match puzzle3d_paste_operations_on(&fixture, &fragment, &placement) {
                    Ok(mutations) => Emit { artifact_mutations: mutations, ..Default::default() },
                    Err(_) => Emit::default(),
                }
            }
            _ => Emit::default(),
        }
    }
}

impl InteractiveJob for Puzzle3dClipboardJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        let emit = self.emit();
        let Some(completion) = self.completion.as_ref() else { return StepOutcome::Fault(JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }) };
        if completion.complete(Ok(emit), EphemeralEmit::default()).is_err() {
            return StepOutcome::Fault(JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
        }
        let output = cx.payload_from_bytes(JobPayloadStream::CommitOutput, &self.raw_wire).unwrap_or_else(|rejected| {
            drop(rejected.into_source());
            RetainedJobPayload::empty(JobPayloadStream::CommitOutput)
        });
        StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.closing = true;
        if !self.raw_wire.is_empty() {
            if maximum_items == 0 || maximum_bytes == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            let released_bytes = self.raw_wire.len().min(maximum_bytes);
            self.raw_wire.truncate(self.raw_wire.len() - released_bytes);
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
        }
        if self.input.take().is_some() || self.completion.take().is_some() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.raw_wire.is_empty() && self.input.is_none() && self.completion.is_none()
    }
}

impl ArtifactReservedJob for Puzzle3dClipboardJob {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(match InteractiveJob::close_step(self, maximum_items, maximum_bytes) {
            InteractiveJobCloseStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released_bytes },
            InteractiveJobCloseStep::Blocked => PluginCloseStep::Blocked { reason: "puzzle3d clipboard route close is blocked" },
            InteractiveJobCloseStep::Complete => PluginCloseStep::Complete,
        })
    }

    fn terminal_is_empty(&self) -> bool {
        InteractiveJob::terminal_is_empty(self)
    }
}

impl ArtifactEditor for Puzzle3dPlayApp {
    const DIALECT: Dialect = crate::PUZZLE3D_DIALECT;
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

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        window_ownership::register_config(registry)
    }

    /// 🎮️ The six framework interaction verbs, answered out of this app's OWN scope table
    /// ([`Puzzle3dScopeClass::Interaction`]) instead of the framework's blanket `UiDirtyScope::Full`: a
    /// hover repaints the world body alone, a pick adds the inspector + outliner rows + history panel,
    /// a mode/granularity switch only the window's Select chrome. A verb that touched any domain this
    /// app does not declare — or none at all — answers `None`, and the framework keeps its widest scope.
    fn interaction_scope(verb: InteractionVerb, domains: &[&str]) -> Option<UiDirtyScope> {
        let declared = !domains.is_empty() && domains.iter().all(|domain| *domain == PUZZLE3D_INTERACTION_DOMAIN);
        declared.then(|| puzzle3d_scope(Puzzle3dScopeClass::Interaction(verb)))
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        window_ownership::register_transient(registry)
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Puzzle3dConfigStorePreparationFactory))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Puzzle3dArtifactStorePreparationFactory))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<NoDraft, NoDraftMutation>())
    }

    /// 🧽️ puzzle3d owns no draft lane, but `VcsArtifactApp::close_step` still walks it: every one of
    /// its seven owned lanes is mandatory, and the `interaction-store` lane behind this one never runs
    /// until the draft/presence/transient disposers exist.
    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<NoDraft, NoDraftMutation>())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(presence::Puzzle3dPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(presence::Puzzle3dPresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(presence::puzzle3d_presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    /// 🧭️ The two framework-injected host-configuration verbs, resolved to one event-sourced Config
    /// mutation each instead of a retained tool operation. The shell owns their session state and then
    /// forwards the resolved value here so the app can clear/prepare its scratch
    /// (`🏛️ShellHost/🟦️.tsx`'s `SET_ACTIVE_TOOL_ACTION_ID`/`SET_ACTIVE_UTILITY_ACTION_ID` branches);
    /// without this hook both verbs fell through to `admit_command_json` and failed closed with
    /// `interactive-job.missing-factory`, so the fill tool could not even be selected.
    ///
    /// 🧰️ `setActiveUtility` needs no variant of its own: puzzle3d's active utility IS host view state
    /// (`puzzle3d_scene_active_utility` reads `ViewModel.active_utility_by_window_id`), and the only
    /// app-owned state a utility switch invalidates is that window's engagement input.
    fn host_configuration_mutation(_action: &str, _args: Option<&dsl::DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok(None)
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Puzzle3dRetainedCommandProofs::bounded_first_step_tool_proofs();
        proofs.extend(Puzzle3dHostConfigurationProofs::bounded_first_step_tool_proofs());
        proofs
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Puzzle3dRetainedCommandJobFactory::new(&controller))?;
        Ok(())
    }

    fn build_tool_job(request: semio_framework_plugin::app::ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !PUZZLE3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(Fault::from("puzzle3d-command-tool-mismatch"));
        }
        let tool_id = request.command.action_id();
        let mut work: Box<dyn crate::retained_command::PuzzleCommandWork<EditorApp<Self>>> = match tool_id {
            "translateSelection" | "rotateSelection" | "scaleSelection" => Box::new(Puzzle3dScaleWork::new(tool_id)),
            "patchInspector" => Box::new(Puzzle3dPatchInspectorWork::default()),
            "worldRelocate" => Box::new(Puzzle3dWorldRelocateWork::default()),
            "createAttraction" => Box::new(Puzzle3dCreateAttractionWork::default()),
            "setActiveExample" => Box::new(Puzzle3dSetActiveExampleWork::default()),
            "addBrushObject" => Box::new(Puzzle3dAddBrushObjectWork::default()),
            "addObjectKind" => Box::new(Puzzle3dAddObjectKindWork::default()),
            "engagementAbort" => Box::new(Puzzle3dWindowCommandWork::new(tool_id)),
            "engagementRepeatLast" => Box::new(Puzzle3dWindowCommandWork::new(tool_id)),
            "engagementSubmit" => Box::new(Puzzle3dWindowCommandWork::new(tool_id)),
            "setObjectKindWeight" | "setVortexKindWeight" => Box::new(Puzzle3dKindWeightWork::new(tool_id)),
            "acceptSuggestion" => Box::new(Puzzle3dAcceptSuggestionWork::default()),
            "cycleBrushCandidate" | "cycleBrushCandidateBack" | "cancelFillBuild" | "fillBuildTick" | "registerBrushMesh" | "setFillCount" | "suggestionsTick" => Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id)),
            "focusSelection" => Box::new(Puzzle3dFocusSelectionWork::default()),
            "relocateTargetVolume" => Box::new(Puzzle3dRelocateVolumeWork::default()),
            "setCamera"
            | "setProjection"
            | "setProjectionParam"
            | "toggleSun"
            | "setSunAzimuth"
            | "setSunElevation"
            | "setSunIntensity"
            | "setLodAutomatic"
            | "setLodDepthVariable"
            | "setLodManual"
            | "setGridVisible"
            | "setPanelPage"
            | "setGridSnapEnabled"
            | "setGridSpacing"
            | "setSelectableKind"
            | "setProximityRadius"
            | "setChunkSize"
            | "setVoxelDims"
            | "setTransformGumballFlag"
            | "setVortexShow"
            | "setVortexDirection"
            | "setBrushPlacementOverlapBudget"
            | "openVortexSuggestions"
            | "closeVortexSuggestions"
            | "hoverSuggestion"
            | "engagementControlSelect"
            | "engagementInput" => Box::new(Puzzle3dWindowCommandWork::new(tool_id)),
            "exportFixture" => Box::new(Puzzle3dWindowCommandWork::new(tool_id)),
            "openImportFixture" => Box::new(Puzzle3dWindowCommandWork::new(tool_id)),
            "importFixture" => Box::new(Puzzle3dWindowCommandWork::new(tool_id)),
            "addTargetVolume" => Box::new(Puzzle3dWindowCommandWork::new(tool_id)),
            "worldPointerDown" | "transformBegin" | "transformEnd" => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(tool_id)),
            _ => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent)),
        };
        work.bind_view_state(request.context.view_state.clone());
        work.bind_instance(request.app_instance_id, &request.parent_document_id);
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
            command_id: Puzzle3dCommand::action_id,
            work,
        };
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 📎 Ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d: replaces the old
    /// `crate::editor::puzzle3d::config::schema::register_app_schema()` self-registering call, which
    /// puzzle's plugin root used to reach `.setup()` for — `register_document_app`/`document_app`
    /// now call this automatically the moment `Puzzle3dPlayApp` is bound to a plugin, exactly like
    /// `🗒️note`'s own `app_schema` override.
    fn app_schema() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::puzzle3d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Puzzle3dPlaySnapshot {
        LazyLock::force(&NAKAGIN_EXAMPLE_FIXTURE);
        let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&default_fixture())).into());
        let config = Puzzle3dRuntime::default();
        let active_utility = puzzle3d_scene_active_utility(&config, None, None);
        let scene = scene_from_projection(&puzzle3d_projection_value(snapshot.value()), config, &active_utility);
        let app = Puzzle3dPlayApp::default();
        sync_precompute_session(&mut app.precompute.borrow_mut(), &scene);
        snapshot
    }

    fn clipboard_media_type() -> Option<MediaType> {
        Some(MediaType { class: MediaClass::ThreeD, form: MediaForm::Design })
    }

    fn copy_fragment(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle3dConfig>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        puzzle3d_copy_fragment(doc, interaction)
    }

    fn cut_operations(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle3dConfig>, interaction: &InteractionView<'_>) -> Vec<Puzzle3dMutation> {
        puzzle3d_cut_operations(doc, interaction).unwrap_or_default()
    }

    fn paste_operations(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle3dMutation>, ClipboardError> {
        puzzle3d_paste_operations(doc, fragment, placement)
    }

    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if !matches!(request.tool_id.as_str(), "copy" | "cut" | "paste") {
            return Ok(None);
        }
        Ok(Some(ArtifactReservedToolJob::new(Puzzle3dClipboardJob::new(request))))
    }

    /// 🏷️ Maps each `Puzzle3dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle3dCommand) -> &'static str {
        command.action_id()
    }

    /// 🎯️ Maps the host's transitional `{action,args}` wire onto Puzzle 3D's closed command
    /// enum until React and wgpu send `OpBinary` command bytes directly.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let window_id = args.and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(dsl::DslValue::as_str).map(str::to_string);
        let args = args.map(json::from_dsl_value);
        Puzzle3dCommand::from_action(action, args, window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 3D action '{action}'")))
    }

    /// @emoji 🧩️ Thin typed-command adapter — reconstructs the exact `(action, args, window_id)`
    /// triple `handle_action_impl` expects from the typed `Puzzle3dCommand`.
    fn handle(
        command: &Puzzle3dCommand,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation, Self::DraftMutation>, Fault> {
        let interaction = Puzzle3dInteractionSnapshot::from_interaction(interaction);
        let window = window_ownership::config_from_view(cfg);
        let runtime = window_ownership::runtime(cfg.snapshot, &window, &window_ownership::Puzzle3dWindowTransient::default(), view_state);
        let window_id = puzzle3d_addressed_window_id(view_state, None, command.window_id(), &runtime.window_ids);
        Ok(with_puzzle3d_app_for(puzzle3d_view_session_key(doc), &runtime, |app| app.handle_action_impl(command, Some(window_id), doc.snapshot, &runtime, view_state, &interaction).0))
    }

    /// 🕹️ `vortex` domain topology (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
    /// objects and every root-level entity kind (vortex marker, attraction, target volume, reference,
    /// catalogue kind) as one flat forest, except object-owned vortex markers whose parent is the
    /// object they mark — the one real nesting relationship this app's document carries, replacing
    /// what `hoveredVortexFullId`'s ad hoc highlighting used to do by hand.
    /// 🔭️ Read off the SAME projection every render publishes (`puzzle3d_fixture_from_projection`),
    /// never a second decoding of the document: `protocol::validate_state` prunes every selected id
    /// this topology does not contain, so a universe narrower than what the viewport paints makes a
    /// pick land on the leftover and then vanish — with no error anywhere. One authority is what
    /// makes "the user can see it" and "the user can pick it" the same statement
    /// (26/09/02/PUZZLE-3D-END-TO-END wave B9 lane 2).
    fn interaction_topology(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle3dConfig>) -> semio_framework_plugin::InteractionTopology {
        let snapshot = puzzle3d_fixture_from_projection(&puzzle3d_projection_value(doc.snapshot.value()));
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
        for kind in puzzle3d_kind_ids(&snapshot, "objects") {
            ordered.push(semio_framework_plugin::TopologyNode { id: kind, granularity: PUZZLE3D_GRANULARITY_KIND.into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(PUZZLE3D_INTERACTION_DOMAIN.to_string(), semio_framework_plugin::DomainTopology { ordered });
        semio_framework_plugin::InteractionTopology { domains }
    }

    /// 🔌️ Declares puzzle3d's typed media I/O surface — the implicit document ports plus the flagship
    /// `kit:in` seam: an input port accepting `Kit×Type` media tagged `kit.catalog`, fanning IN from
    /// potentially many producers (`multiplicity: Many`).
    fn io() -> Option<AppIo> {
        Some(puzzle3d_io())
    }

    /// 🎞️ `kit:in` seam: normalizes an incoming `kit.catalog` fragment (`objectKinds`/`vortexKinds`/
    /// `cableKinds`/`attractionKinds`/`kindCompatibility`) into puzzle3d's own `meta.kind_catalogs`
    /// vocabulary (`objects`/`vortices`/`cables`/`attractions`) and upserts it (keyed by row `id`,
    /// deterministic/order-independent — safe for `multiplicity: Many` fan-in) via the same
    /// `puzzle3d_operations_from_fixture_change` delta bridge every other fixture-mutating action
    /// already uses, so this never mutates anything directly — only real, undoable operations.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "kit:in" {
            return Err(MediaError::NotImplemented);
        }
        let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "kit:in only accepts a Structured (JSON) payload".into()));
        };
        let fragment: Value = parse(json.as_str()).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let mut fixture: Puzzle3dFixture = dsl::FromValue::from_value(dsl::DslValue::from(doc.snapshot.value())).map_err(|error: dsl::ValueError| MediaError::Payload(port.to_string(), error.to_string()))?;

        let mut catalogs: dsl::DslValue = fixture.meta.kind_catalogs.clone().unwrap_or_else(|| {
            dsl::DslValue::object([
                ("objects".to_string(), dsl::DslValue::Array(Vec::new())),
                ("vortices".to_string(), dsl::DslValue::Array(Vec::new())),
                ("cables".to_string(), dsl::DslValue::Array(Vec::new())),
                ("attractions".to_string(), dsl::DslValue::Array(Vec::new())),
            ])
        });
        puzzle3d_upsert_catalog_rows(&mut catalogs, "objects", fragment.get("objectKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "vortices", fragment.get("vortexKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "cables", fragment.get("cableKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "attractions", fragment.get("attractionKinds"));
        fixture.meta.kind_catalogs = Some(catalogs);

        if let Some(incoming_compat) = fragment.get("kindCompatibility").and_then(Value::as_array) {
            let mut compat: Vec<dsl::DslValue> = fixture.meta.kind_compatibility.as_ref().and_then(dsl::DslValue::as_array).map(<[dsl::DslValue]>::to_vec).unwrap_or_default();
            for row in incoming_compat {
                let source = row.get("source").and_then(Value::as_str).unwrap_or_default();
                let target = row.get("target").and_then(Value::as_str).unwrap_or_default();
                let row_dsl = json::to_dsl_value(row);
                match compat.iter().position(|entry| entry.get("source").and_then(dsl::DslValue::as_str) == Some(source) && entry.get("target").and_then(dsl::DslValue::as_str) == Some(target)) {
                    Some(index) => compat[index] = row_dsl,
                    None => compat.push(row_dsl),
                }
            }
            fixture.meta.kind_compatibility = Some(dsl::DslValue::Array(compat));
        }

        let operations = puzzle3d_operations_from_fixture_change(&puzzle3d_projection_value(doc.snapshot.value()), &fixture);
        Ok(Emit::mutations(operations))
    }

    /// 🕹️ Plain `render` is the no-interaction entry point the framework only reaches through
    /// `render_with_request_context`'s default body — this app overrides that, so every real render
    /// carries a live selection. Kept as a thin delegate against an empty snapshot rather than a
    /// second body, so there is exactly one render implementation
    /// ([`Puzzle3dPlayApp::render_body`]).
    fn render(body_key: &str, doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, cfg, view_state, &window_ownership::Puzzle3dWindowTransient::default(), &Puzzle3dInteractionSnapshot::default())
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): resolves the live `vortex` domain
    /// (selection + `"pointer"` hover) once per render and threads it into the whole body — the world
    /// scene's selection/gumball/vortex-marker paint, the Brush ghost, and the inspection panel's
    /// per-entity field groups all read it instead of ever storing selection in this app again.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let window_transient = window_ownership::transient_from_view(transient);
        Self::render_body(body_key, doc, cfg, view_state, &window_transient, &Puzzle3dInteractionSnapshot::from_interaction(interaction))
    }

    fn window_measures_with_request_context(
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
    ) -> HashMap<String, Vec<WindowMeasure>> {
        Self::window_measures_body(doc, cfg, view_state, &Puzzle3dInteractionSnapshot::from_interaction(interaction))
    }

    /// 🕹️ The context menu reads the AUTHORITATIVE framework-owned selection, not only the
    /// client-supplied `request.surface.selection`: `World3dHost` only ever puts its painted object
    /// ids (and component ids) into that field, so a selected vortex / target volume / reference would
    /// otherwise never reach a menu row. The surface's own hits/selection still win when present, so a
    /// right-click on an unselected entity still targets what was clicked.
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &Puzzle3dInteractionSnapshot::from_interaction(interaction), registry)
    }

    fn window_engagements(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle3dWindowTransient::default(), Some(view_state));
        with_puzzle3d_app_for(puzzle3d_view_session_key(doc), &runtime, |app| {
            let Some(labels) = puzzle3d_labels(view_state) else { return HashMap::new() };
            let envelope = app.scene_for(&puzzle3d_projection_value(doc.snapshot.value()), &runtime, Some(view_state), window_id);
            HashMap::from([(window_id.to_string(), main::engagement(&envelope, labels))])
        })
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
    ) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::transient_from_view(transient), Some(view_state));
        with_puzzle3d_app_for(puzzle3d_view_session_key(doc), &runtime, |app| {
            let Some(labels) = puzzle3d_labels(view_state) else { return HashMap::new() };
            let envelope = app.scene_for(&puzzle3d_projection_value(doc.snapshot.value()), &runtime, Some(view_state), window_id);
            HashMap::from([(window_id.to_string(), main::engagement(&envelope, labels))])
        })
    }

    /// 🕹️ See `render`: the interaction-aware `window_measures_with_request_context` is what the
    /// runtime actually calls; this delegate keeps one chrome body against an empty selection.
    fn window_measures(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        Self::window_measures_body(doc, cfg, view_state, &Puzzle3dInteractionSnapshot::default())
    }

    fn tool_measures(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle3dWindowTransient::default(), Some(view_state));
        with_puzzle3d_app_for(puzzle3d_view_session_key(doc), &runtime, |app| {
            let wid = view_state.window_id.as_deref().unwrap_or(main::WINDOW_KIND_ID);
            let Some(labels) = puzzle3d_labels(view_state) else { return HashMap::new() };
            let envelope = app.scene_for(&puzzle3d_projection_value(doc.snapshot.value()), &runtime, Some(view_state), wid);
            // 🧠️ The app's OWN live precompute session, exactly as `render` reads it — never a session
            // minted per call. Tool chrome reports what the lanes have actually resolved
            // (fill readiness, brush candidates); a session minted here starts empty every call, so no
            // amount of warming could ever reach the user's controls.
            {
                let mut precompute = app.precompute.borrow_mut();
                sync_precompute_session(&mut precompute, &envelope);
                precompute.set_fill_applied_count(runtime.fill_count);
            }
            let precompute = app.precompute.borrow();
            HashMap::from([(fill_tool::TOOL_ID.to_string(), fill_tool::measures(&envelope, &precompute, labels))])
        })
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &Puzzle3dInteractionSnapshot::default(), registry)
    }
}

impl Puzzle3dPlayApp {
    /// 🖱️ The ONE context-menu implementation — `ArtifactEditor::context_menu` and
    /// `context_menu_with_request_context` funnel here.
    fn context_menu_body(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &Puzzle3dInteractionSnapshot,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let config = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle3dWindowTransient::default(), Some(view_state));
        let Some(labels) = puzzle3d_labels(view_state) else { return Vec::new() };
        let wid = view_state.window_id.as_deref().unwrap_or(main::WINDOW_KIND_ID);
        let active_utility = puzzle3d_scene_active_utility(&config, Some(view_state), Some(wid));
        let envelope = scene_from_projection(&puzzle3d_projection_value(doc.snapshot.value()), config, &active_utility);
        let mut selection = Puzzle3dContextSelection::from_surface(request.surface.as_ref());
        selection.fill_from_interaction(interaction);
        puzzle3d_context_menu_items(&envelope, &selection, labels, registry)
    }

    /// 🖼️ The ONE render implementation — both `ArtifactEditor::render` and
    /// `render_with_request_context` funnel here, differing only in whether `interaction` carries a
    /// live `vortex`-domain read or the empty default.
    fn render_body(
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &window_ownership::Puzzle3dWindowTransient,
        interaction: &Puzzle3dInteractionSnapshot,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), transient, Some(view_state));
        let node = with_puzzle3d_app_for(puzzle3d_view_session_key(doc), &runtime, |app| -> semio_framework_plugin::UiAssemblyResult<_> {
            let (base_body_key, window_id_from_key) = body_key.split_once(':').map_or((body_key, None), |(b, w)| (b, Some(w)));
            let config = &runtime;
            let wid = puzzle3d_addressed_window_id(Some(view_state), window_id_from_key, None, &config.window_ids);
            let active_utility = puzzle3d_scene_active_utility(config, Some(view_state), Some(wid));
            let precompute_scene = app.scene_for(&puzzle3d_projection_value(doc.snapshot.value()), config, Some(view_state), wid);
            {
                let mut precompute = app.precompute.borrow_mut();
                sync_precompute_session(&mut precompute, &precompute_scene);
                precompute.set_fill_applied_count(config.fill_count);
            }
            let precompute = app.precompute.borrow();
            // 🪣️ Additive-only: appends just the not-yet-committed fill-plan tail onto the live fixture —
            // safe even during a live gumball scratch drag, since it never touches/replaces any
            // already-present object (the dragged one included).
            let fill_available = precompute.fill_available_count();
            let fixture = puzzle3d_fixture_with_fill_display_memo(app.render_fixture(&puzzle3d_projection_value(doc.snapshot.value())), &precompute, config.fill_count, fill_available, &app.fill_display_memo);
            let mut envelope = Puzzle3dScene { fixture, runtime: config.clone(), active_utility };
            main::frame_unset_camera(&mut envelope, wid);
            let labels = puzzle3d_labels(view_state).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.localization.unsupported", "puzzle3d has no authored label set for the host's locale/terminology axes"))?;
            match base_body_key {
                main::BODY_KEY => {
                    let (instances_json, meshes_json) = app.geometry_jsons(&envelope.fixture);
                    main::render(&envelope, &precompute, labels, instances_json, meshes_json, interaction)
                }
                document::BODY_KEY => app.document_tree_cached_from(&envelope.fixture, labels, &envelope.runtime.panel_pages),
                catalogue::BODY_KEY => catalogue::render(&envelope, labels),
                inspection::BODY_KEY => inspection::render(&envelope, interaction, labels),
                settings_panel::BODY_KEY => settings_panel::render(&envelope, labels, wid),
                _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d unknown-body label admission failed")),
            }
        })?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    /// 🪟️ The ONE per-window-instance chrome implementation — `ArtifactEditor::window_measures` and
    /// `window_measures_with_request_context` funnel here.
    fn window_measures_body(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel, interaction: &Puzzle3dInteractionSnapshot) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let config = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle3dWindowTransient::default(), Some(view_state));
        with_puzzle3d_app_for(puzzle3d_view_session_key(doc), &config, |app| {
            let Some(labels) = puzzle3d_labels(view_state) else { return HashMap::new() };
            let envelope = app.scene_for(&puzzle3d_projection_value(doc.snapshot.value()), &config, Some(view_state), window_id);
            // 🧠️ See `tool_measures`: window chrome reads the app's OWN live precompute session, the same
            // one `render` reads, so the brush placement picker offers the candidates the brush lane has
            // actually resolved for the live target instead of a cold session's empty list.
            {
                let mut precompute = app.precompute.borrow_mut();
                sync_precompute_session(&mut precompute, &envelope);
                precompute.set_fill_applied_count(config.fill_count);
            }
            let precompute = app.precompute.borrow();
            HashMap::from([(window_id.to_string(), main::window_measures(&envelope, &precompute, labels, interaction))])
        })
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
        hover: HoverSpec { enabled: true, transitive: false, channels: vec![PUZZLE3D_HOVER_CHANNEL.into()], broadcast: true },
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}

/// 🗂️ Fixed ceiling on the object-kind rows the `addObjectKind` arg form and the "Add Object" dialog
/// offer — the manifest is minted once per process, so this select is built eagerly and must be
/// bounded independently of how wide a catalog a future example declares.
pub const PUZZLE3D_OBJECT_KIND_OPTIONS_MAX: usize = 64;

/// 🗂️ The object kinds the "Add Object" dialog (and the standalone `addObjectKind` arg form) offers —
/// read from the declared examples' own `meta.kindCatalogs`, exactly the rows
/// `📌️panels/🛍️catalogue` renders, deduplicated across examples in catalog order. `AppDefinition`
/// is built once per process and never sees the live document, and `setActiveExample` is this
/// editor's only document source (see `📓️2026-09-09-user-feature-checklist.md` §24: there is no
/// import), so the union of the declared example catalogs IS the reachable kind set — not the single
/// literal `"Object"` option this select used to hardcode, which could not add a single real kind of
/// either example.
fn puzzle3d_object_kind_options() -> Vec<ActionArgOption> {
    let mut options: Vec<ActionArgOption> = Vec::with_capacity(PUZZLE3D_OBJECT_KIND_OPTIONS_MAX);
    for fixture in [&*CONCRETE_FOREST_EXAMPLE_FIXTURE, &*NAKAGIN_EXAMPLE_FIXTURE] {
        for entry in puzzle3d_catalog_entries(fixture, "objects") {
            if options.len() >= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX {
                return options;
            }
            let Some(id) = entry.get("id").and_then(dsl::DslValue::as_str) else {
                continue;
            };
            if options.iter().any(|option| option.value == id) {
                continue;
            }
            options.push(ActionArgOption::new(id, LocalizedLabel::data(catalogue::catalog_entry_label(entry))));
        }
    }
    options
}

/// 🗂️ The kind the `objectKind` select stages when nothing is picked — the first catalog row, never a
/// literal id no catalog declares.
fn puzzle3d_default_object_kind(options: &[ActionArgOption]) -> String {
    options.first().map(|option| option.value.clone()).unwrap_or_default()
}

/// 🗂️ The one `objectKind` select both the standalone `addObjectKind` arg form and the "Add Object"
/// dialog declare — built twice from the same catalog so the two forms can never drift apart.
fn puzzle3d_object_kind_arg() -> ActionArgDef {
    let options = puzzle3d_object_kind_options();
    let default = puzzle3d_default_object_kind(&options);
    ActionArgDef::select("objectKind", puzzle3d_localized(|l| l.kind), options).default_value(&default)
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
    Editor::builder(crate::PUZZLE3D_DIALECT)
            .document(["semio", "puzzle", "3d"])
            .artifact_kind(crate::artifact_kind())
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
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            // 🗨️ Shell-only effect (no document mutation): opens the "addObject" dialog. Declared HERE, in
            // front of every other create verb, because the shell fallback menu and the ribbon keep only
            // the first `CONTEXT_MENU_PRIMARY_BUDGET` leaves at top level and fold the rest into "More ›" —
            // the user-facing "Add Object…" row has to be the one that opens the declared dialog.
            .action_with(ActionDefinition::bounded_catalog("openAddObjectDialog", puzzle3d_localized_phrase(|l| l.object, |w| format!("Add {w}…"), |w| format!("{w} hinzufügen…")), ActionKind::Shell).category("create"))
            // 🌱️ The dialog's (and the catalogue row's / the viewport drop's) parametrized verb, never a
            // user-facing menu row of its own: its `objectKind` select IS the dialog, so offering it in the
            // palette/menu published a SECOND row carrying the identical "Add Object…" label that opened a
            // bare action pane instead (`📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 7).
            .action_with(ActionDefinition::bounded_catalog("addObjectKind", puzzle3d_localized_phrase(|l| l.object, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")), ActionKind::Mutation).category("create").in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).category("selection"))
            .action_with(ActionDefinition::bounded_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).category("create"))
            .action_with(ActionDefinition::bounded_catalog("exportFixture", LocalizedLabel::native("Export", "Exportieren"), ActionKind::Shell).category("file"))
            .action_with(ActionDefinition::bounded_catalog("importFixture", LocalizedLabel::native("Import", "Importieren"), ActionKind::Mutation).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("openImportFixture", LocalizedLabel::native("Import", "Importieren"), ActionKind::Shell).category("file"))
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .mutation("worldRelocate", puzzle3d_localized_phrase(|l| l.object, |w| format!("Relocate {w}"), |w| format!("{w} verlagern")))
            .action_with(ActionDefinition::bounded_catalog("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).category("hand"))
            .mutation("patchInspector", LocalizedLabel::native("Patch Inspector", "Inspektor aktualisieren"))
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .mutation("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"))
            .mutation("createAttraction", puzzle3d_localized_phrase(|l| l.attraction, |w| format!("Create {w}"), |w| format!("{w} erstellen")))
            // 🎯️ Entity-scoped and therefore NOT palette/shell-menu vocabulary: `delete_attraction` acts
            // on the `id` its own context-menu row carries, and the action declares no args, so a generic
            // surface can only ever dispatch it empty. Offering it anyway put "Delete Attraction" /
            // "Delete Target Volume" in the shell fallback menu of a plain OBJECT selection (measured
            // 2026-09-09 21:05) where they name nothing and do nothing. The attraction's own row in
            // `puzzle3d_context_menu_items` is unaffected — it supplies the id.
            .action_with(ActionDefinition::bounded_catalog("deleteAttraction", puzzle3d_localized_phrase(|l| l.attraction, |w| format!("Delete {w}"), |w| format!("{w} löschen")), ActionKind::Mutation).category("targets").in_palette(false))
            .mutation("addTargetVolume", puzzle3d_localized_phrase(|l| l.target_volume, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")))
            .action_with(ActionDefinition::bounded_catalog("deleteTargetVolume", LocalizedLabel::native("Delete Target Volume", "Zielvolumen löschen"), ActionKind::Mutation).category("targets").in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("setTargetVolumeFlag", LocalizedLabel::native("Set Target Volume Flag", "Zielvolumenmarkierung festlegen"), ActionKind::Mutation).category("targets").in_palette(false))
            .mutation("addBrushObject", puzzle3d_localized_phrase(|l| l.object, |w| format!("Add Brush {w}"), |w| format!("Pinsel-{w} hinzufügen")))
            .mutation("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"))
            .mutation("acceptSuggestion", LocalizedLabel::native("Accept Suggestion", "Vorschlag annehmen"))
            // 👁️ Ephemeral view state — selection, hover, camera scratch, utility-parameter runtime.
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("setProjection", LocalizedLabel::native("Set Projection", "Projektion festlegen"), ActionKind::View, "scan"))
            .action_with(ActionDefinition::new("setProjectionParam", LocalizedLabel::native("Set Projection Parameter", "Projektionsparameter festlegen"), ActionKind::View, "scan"))
            .view_action("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"))
            .action_with(ActionDefinition::bounded_catalog("selectSameKindSelection", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).category("selection"))
            .view_action("setVortexShow", puzzle3d_localized_phrase(|l| l.vortex_show, |w| format!("Set {w}"), |w| format!("{w} festlegen")))
            .view_action("setVortexDirection", puzzle3d_localized_phrase(|l| l.vortex_direction, |w| format!("Set {w}"), |w| format!("{w} festlegen")))
            .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun"))
            .view_action("setLodAutomatic", LocalizedLabel::native("Set Lod Automatic", "Detailstufe automatisch"))
            .view_action("setLodDepthVariable", LocalizedLabel::native("Set Lod Depth Variable", "Detailstufen-Tiefe festlegen"))
            .view_action("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster anzeigen"))
            .view_action("setPanelPage", LocalizedLabel::native("Set Panel Page", "Panel-Seite festlegen"))
            .view_action("setLodManual", LocalizedLabel::native("Set Lod Manual", "Detailstufe manuell"))
            .action_with(ActionDefinition::new("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View, "grid-3x3"))
            .view_action("setGridSpacing", LocalizedLabel::native("Set Grid Spacing", "Rasterabstand festlegen"))
            .view_action("setProximityRadius", LocalizedLabel::native("Set Proximity Radius", "Näheradius festlegen"))
            .view_action("setChunkSize", LocalizedLabel::native("Set Chunk Size", "Blockgröße festlegen"))
            .view_action("setSelectableKind", LocalizedLabel::native("Set Selectable Kind", "Auswählbare Art festlegen"))
            .action_with(ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand"))
            .action_with(ActionDefinition::new("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View, "hand"))
            .action_with(ActionDefinition::new("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"), ActionKind::View, "hand"))
            .view_action("setTransformGumballFlag", LocalizedLabel::native("Set Transform Gumball Flag", "Transformieren-Griff festlegen"))
            .action_with(ActionDefinition::new("transformBegin", LocalizedLabel::native("Transform Begin", "Transformieren beginnen"), ActionKind::View, "move"))
            .view_action("transformEnd", LocalizedLabel::native("Transform End", "Transformieren beenden"))
            .view_action("setVoxelDims", LocalizedLabel::native("Set Voxel Dims", "Voxel-Abmessungen festlegen"))
            .mutation("relocateTargetVolume", LocalizedLabel::native("Relocate Target Volume", "Zielvolumen verlagern"))
            .view_action("setBrushPlacementOverlapBudget", LocalizedLabel::native("Set Brush Placement Overlap Budget", "Pinsel-Überlappungsbudget festlegen"))
            .view_action("setObjectKindWeight", puzzle3d_localized_phrase(|l| l.object, |w| format!("Set {w} Kind Weight"), |w| format!("{w}-Art-Gewicht festlegen")))
            .view_action("setVortexKindWeight", puzzle3d_localized_phrase(|l| l.vortex, |w| format!("Set {w} Kind Weight"), |w| format!("{w}-Art-Gewicht festlegen")))
            .view_action("cycleBrushCandidate", LocalizedLabel::native("Cycle Brush Candidate", "Pinselkandidat wechseln"))
            .view_action("cycleBrushCandidateBack", LocalizedLabel::native("Cycle Brush Candidate Back", "Pinselkandidat rückwärts wechseln"))
            .action_with(ActionDefinition::bounded_catalog("openVortexSuggestions", puzzle3d_localized_phrase(|l| l.vortex, |w| format!("Open {w} Suggestions"), |w| format!("{w}-Vorschläge öffnen")), ActionKind::View).category("tools"))
            .view_action("closeVortexSuggestions", puzzle3d_localized_phrase(|l| l.vortex, |w| format!("Close {w} Suggestions"), |w| format!("{w}-Vorschläge schließen")))
            .view_action("hoverSuggestion", LocalizedLabel::native("Hover Suggestion", "Vorschlag überfahren"))
            .view_action("suggestionsTick", LocalizedLabel::native("Suggestions Tick", "Vorschläge-Takt"))
            .view_action("fillBuildTick", LocalizedLabel::native("Fill Build Tick", "Füllaufbau-Takt"))
            .view_action("cancelFillBuild", puzzle3d_localized(|l| l.fill_cancel))
            .view_action("registerBrushMesh", LocalizedLabel::native("Register Brush Mesh", "Pinsel-Mesh registrieren"))
            .action_with(ActionDefinition::new("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            // 📝️ Staged argument forms for the panel-visible create/query actions (P1).
// 🛑 The fill job's own identity travels with every cancel, so a cancel dispatched against a
            // superseded run is rejected instead of killing the plan the user is looking at — the same
            // guard `🔋️energy`'s `request_identity_args` puts on `cancel-energy-simulation`.
            .action_args("cancelFillBuild", vec![
                ActionArgDef::number("job", LocalizedLabel::native("Job", "Auftrag")).required(),
                ActionArgDef::number("operation", LocalizedLabel::native("Operation", "Vorgang")).required(),
                ActionArgDef::number("generation", LocalizedLabel::native("Generation", "Generation")).required(),
            ])
            .action_args("addObjectKind", vec![puzzle3d_object_kind_arg()])
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
                    .args(vec![puzzle3d_object_kind_arg().required()])
                    .submit_label(LocalizedLabel::native("Add", "Hinzufügen")),
            )
            .action_interactive_job("acceptSuggestion", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addBrushObject", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addObjectKind", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addTargetVolume", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("closeVortexSuggestions", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("createAttraction", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidate", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidateBack", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteAttraction", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteTargetVolume", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("exportFixture", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("importFixture", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("openImportFixture", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementAbort", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementControlSelect", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementRepeatLast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("fillBuildTick", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("cancelFillBuild", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("focusSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("hoverSuggestion", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("openAddObjectDialog", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("openVortexSuggestions", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("patchInspector", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("registerBrushMesh", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("relocateTargetVolume", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("selectSameKindSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushPlacementOverlapBudget", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setChunkSize", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setFillCount", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSnapEnabled", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSpacing", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridVisible", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setPanelPage", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodAutomatic", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodDepthVariable", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodManual", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setObjectKindWeight", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjectionParam", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setProximityRadius", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectableKind", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionFlag", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetVolumeFlag", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setTransformGumballFlag", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setVortexDirection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setVortexKindWeight", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setVortexShow", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setVoxelDims", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("suggestionsTick", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("transformBegin", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("transformEnd", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerDown", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("worldRelocate", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .build_definition()
}

//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ The one puzzle3d-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
/// instead of re-deriving a store/dispatch/render scaffold of its own.
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️example-switch/🦀️.rs"]
mod example_switch;
//#endregion 🧪️Tests
