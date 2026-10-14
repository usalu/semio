//! 📸️ Native editor root with typed authority and a derived intrinsic host view.
#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use {semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot, semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation};
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use protocol::{DiffAlgebra,Mutation,MutationDiff};
use semio_framework_value::DslValue;

//#region 🔖️PlaySnapshot
/// 🌱️ Immutable typed puzzle authority and its deferred first-party intrinsic view.
#[derive(Debug)]
pub struct Puzzle3dPlaySnapshot {
    typed: std::sync::Arc<Puzzle3dSnapshot>,
    value: std::sync::OnceLock<std::sync::Arc<DslValue>>,
}

impl Puzzle3dPlaySnapshot {
    /// 🎯️ Builds an immutable root from an admitted typed snapshot.
    pub fn new(typed: Puzzle3dSnapshot) -> Self {
        Self::from_typed(typed)
    }

    /// 🧬️ Retains typed mutation authority and defers its native host view.
    pub fn from_typed(typed: Puzzle3dSnapshot) -> Self {
        Self { typed: std::sync::Arc::new(typed), value: std::sync::OnceLock::new() }
    }

    /// 👁️ Materializes the first-party host view once per immutable root.
    pub fn value(&self) -> &DslValue {
        self.value.get_or_init(|| std::sync::Arc::new(semio_framework_value::ToValue::to_value(self.typed.as_ref()))).as_ref()
    }

    /// 🧬️ Borrows the immutable typed authority.
    pub fn typed(&self) -> &Puzzle3dSnapshot {
        self.typed.as_ref()
    }

    /// 🧬️ Shares the immutable typed authority — what a tool request holds without copying the document.
    pub fn typed_arc(&self) -> std::sync::Arc<Puzzle3dSnapshot> {
        std::sync::Arc::clone(&self.typed)
    }
}

/// 🌱️ Encodes the admitted typed authority through the native value port.
impl semio_framework_value::ToValue for Puzzle3dPlaySnapshot {
    fn to_value(&self) -> DslValue { <Puzzle3dSnapshot as semio_framework_value::ToValue>::to_value(self.typed()) }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, semio_framework_value::ValueError> { <Puzzle3dSnapshot as semio_framework_value::ToValue>::to_value_controlled(self.typed(), control) }
}

impl semio_framework_value::FromValue for Puzzle3dPlaySnapshot {
    fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> { <Puzzle3dSnapshot as semio_framework_value::FromValue>::from_value(value).map(Self::from_typed) }
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> { <Puzzle3dSnapshot as semio_framework_value::FromValue>::from_value_controlled(value, control).map(Self::from_typed) }
}

impl Clone for Puzzle3dPlaySnapshot {
    fn clone(&self) -> Self {
        let value = std::sync::OnceLock::new();
        if let Some(projected) = self.value.get() {
            let _ = value.set(std::sync::Arc::clone(projected));
        }
        Self { typed: std::sync::Arc::clone(&self.typed), value }
    }
}

impl PartialEq for Puzzle3dPlaySnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.typed == other.typed
    }
}

/// 🧒️ Composition view of the play snapshot: a puzzle document owns no child artifacts, so the
/// typed snapshot's own (empty) composition is the whole answer.
impl semio_framework_schema_composition::ArtifactCompositionFields for Puzzle3dPlaySnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
        semio_framework_schema_composition::ArtifactCompositionFields::visit_child_refs(self.typed.as_ref(), visitor)
    }
    fn child_slots() -> &'static [semio_framework_schema_composition::ChildSlotSpec] {
        <Puzzle3dSnapshot as semio_framework_schema_composition::ArtifactCompositionFields>::child_slots()
    }
    fn link_slots() -> &'static [semio_framework_schema_composition::LinkSlotSpec] {
        <Puzzle3dSnapshot as semio_framework_schema_composition::ArtifactCompositionFields>::link_slots()
    }
}

impl MutationDiff<Puzzle3dPlaySnapshot> for Puzzle3dDiff {
    fn apply(&self, projection: &Puzzle3dPlaySnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle3dPlaySnapshot> {
        MutationDiff::<Puzzle3dSnapshot>::apply(self, projection.typed(), capability).map(Puzzle3dPlaySnapshot::from_typed)
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle3dSnapshot>::absorb(self, other);
    }
}

impl DiffAlgebra<Puzzle3dPlaySnapshot> for Puzzle3dDiff {
    fn inverse(&self, base: &Puzzle3dPlaySnapshot) -> Self {
        DiffAlgebra::<Puzzle3dSnapshot>::inverse(self, base.typed())
    }
    fn is_empty(&self) -> bool {
        DiffAlgebra::<Puzzle3dSnapshot>::is_empty(self)
    }
}

impl Mutation<Puzzle3dPlaySnapshot> for Puzzle3dMutation {
    type Diff = Puzzle3dDiff;

    /// 🧷️ Reuses the typed mutation metadata.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle3dSnapshot>>::DESCRIPTORS;
    const INPUT_SCHEMAS: &'static [&'static str] = <Self as Mutation<Puzzle3dSnapshot>>::INPUT_SCHEMAS;
    const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = <Self as Mutation<Puzzle3dSnapshot>>::INPUT_SCHEMA_DOCUMENTS;

    fn input_schema(&self) -> Option<&'static str> {
        Mutation::<Puzzle3dSnapshot>::input_schema(self)
    }

    fn payload_value(&self) -> DslValue {
        Mutation::<Puzzle3dSnapshot>::payload_value(self)
    }

    fn with_payload_value(&self, value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Mutation::<Puzzle3dSnapshot>::with_payload_value(self, value)
    }

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        Mutation::<Puzzle3dSnapshot>::descriptor(self)
    }

    fn inverse_rows(&self) -> usize {
        Mutation::<Puzzle3dSnapshot>::inverse_rows(self)
    }

    fn diff(&self, projection: &Puzzle3dPlaySnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        Mutation::<Puzzle3dSnapshot>::diff(self, projection.typed.as_ref())
    }

    fn inverse(&self, projection: &Puzzle3dPlaySnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    Ok({
        Mutation::<Puzzle3dSnapshot>::inverse(self, projection.typed.as_ref())?
    
    })
}
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle3dSnapshot>::may_emit_foreign_steps(self)
    }
    fn from_payload_value(kind: &str, value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Self as Mutation<Puzzle3dSnapshot>>::from_payload_value(kind, value)
    }
    fn conflict_target(&self) -> Vec<String> {
        Mutation::<Puzzle3dSnapshot>::conflict_target(self)
    }
}

/// 🪪️ `kinds`/`semantics`/`label`/`target` are projection-independent (the derive-generated
/// `SemanticMutation<Puzzle3dSnapshot>` impl above never actually reads `Puzzle3dSnapshot` data in
/// any of the four), so this bridges the same vocabulary onto `Puzzle3dPlaySnapshot` by forwarding
/// straight through — the `SemanticMutation` twin of the `Mutation<Puzzle3dPlaySnapshot>` bridge
/// immediately above, needed so `.editor_mutation_roster::<Puzzle3dPlayApp>()` can register this
/// dialect's real semantic vocabulary against the play app's own `Snapshot` type.
impl protocol::SemanticMutation<Puzzle3dPlaySnapshot> for Puzzle3dMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::kinds()
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::semantics(self)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::label(self)
    }
    fn target(&self) -> Vec<String> {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::target(self)
    }
}
//#endregion 🔖️PlaySnapshot

/// ♻️ The typed authority retires through its own owner, then the deferred first-party host view, each as its own bounded cursor turn.
impl semio_framework_value::retirement::RetireOwned for Puzzle3dPlaySnapshot {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self { typed, value } = self;
        semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(typed), semio_framework_value::retirement::deferred(value.into_inner())])
    }

    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.typed), semio_framework_value::retirement::deferred_birth_bytes::<Option<std::sync::Arc<DslValue>>>()])
    }

    fn controlled_retirement_supported() -> bool {
        true
    }
}

use semio_s_artifact_puzzle_3d_editor_config::Puzzle3dRuntime;
use semio_s_artifact_puzzle_3d_editor_terminology::Puzzle3dLabels;
use {semio_framework_plugin::kernel::ClipboardError, semio_framework_plugin::kernel::ClipboardFragment, semio_framework_plugin::kernel::PastePlacement};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::InteractionTarget;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
use semio_framework_plugin::SET_ACTIVE_TOOL_ACTION_ID;
use semio_framework_value::FromValue;
use {semio_framework_pack_json::from_json_str, semio_framework_pack_json::json, semio_framework_pack_json::object, semio_framework_pack_json::parse, semio_framework_pack_json::to_json_string, semio_framework_pack_json::to_string, semio_framework_pack_json::Object, semio_framework_pack_json::Value};
use semio_framework_plugin::app::InteractionView;
use {std::collections::HashMap, std::collections::HashSet};
use {std::sync::atomic::AtomicU32, std::sync::atomic::Ordering};
use std::sync::LazyLock;
use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;
use crate as snapshot;

pub const PUZZLE3D_PLAY_APP_ID: &str = "puzzle3d-play";

pub const PUZZLE3D_PLAY_CONTROLLER_ID: &str = "puzzle3d-play";

pub const PUZZLE3D_SCENE_SNAPSHOT_SCHEMA: &str = "puzzle.3d.scene.v1";

pub const PUZZLE3D_FALLBACK_MESH_KIND: &str = "box";

/// 🧰️ Host-owned active utility when none has been pressed yet — none. The transform gumball must be
/// pressed explicitly; an unset/cleared utility must not fall back to `transform` or the gumball
/// appears without an active transform tool.
pub const PUZZLE3D_DEFAULT_UTILITY: &str = "";

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

/// 🌉️ This app's own `Puzzle3dScene.scene_snapshot: Puzzle3dSceneSnapshot` (and `ArtifactApp::Snapshot`) stays a
/// local structural-twin mirror of `crate::Puzzle3dSnapshot`, so the DSL-text
/// example fixtures are parsed once into the typed projection and re-serialized to the JSON string
/// this module's `from_json_str::<Puzzle3dSceneSnapshot>`/`.example(...)` call sites expect.
pub static CONCRETE_FOREST_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| semio_s_artifact_puzzle_3d_examples::concrete_forest::SOURCE.document_json().to_string());

pub static NAKAGIN_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| semio_s_artifact_puzzle_3d_examples::nakagin_capsule_tower::SOURCE.document_json().to_string());

pub static CONCRETE_FOREST_EXAMPLE_SNAPSHOT: LazyLock<Puzzle3dSceneSnapshot> = LazyLock::new(|| from_json_str(CONCRETE_FOREST_EXAMPLE_JSON.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| empty_scene_snapshot()));

pub static NAKAGIN_EXAMPLE_SNAPSHOT: LazyLock<Puzzle3dSceneSnapshot> = LazyLock::new(|| from_json_str(NAKAGIN_EXAMPLE_JSON.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| empty_scene_snapshot()));

pub static EMPTY_EXAMPLE_SNAPSHOT: LazyLock<Puzzle3dSceneSnapshot> = LazyLock::new(empty_scene_snapshot);

fn parse_example_dsl(dsl_text: &str, label: &str) -> String {
    let projection = <Puzzle3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(dsl_text).unwrap_or_else(|error| panic!("{label} example scene_snapshot parses as dsl: {error}"));
    to_json_string(&projection)
}

pub fn puzzle3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PUZZLE3D_PLAY_CONTROLLER_ID.into(), action: action.into(), args: args.map(|value| semio_framework_pack_json::to_dsl_value(&value)) }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: builds a framework `interactionSelect`
/// action targeting one `(granularity, id)` pair in the `vortex` domain — replaces the deleted
/// `setSelection` action builders every document/catalogue tree row used to construct by hand.
pub fn puzzle3d_interaction_select(granularity: &str, id: &str) -> ActionDescriptor {
    let targets = to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    puzzle3d_action(INTERACTION_SELECT_ACTION_ID, Some(json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })))
}

#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
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

#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dReferenceSource {
    #[value(default)]
    pub url: String,
    #[value(default, rename = "mediaKind")]
    pub media_kind: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
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
    pub scale: Option<semio_framework_value::DslValue>,
    #[value(default, rename = "meshUrl")]
    pub mesh_url: Option<String>,
    #[value(default)]
    pub vortices: Vec<Puzzle3dVortex>,
    #[value(default)]
    pub hidden: bool,
    #[value(default)]
    pub locked: bool,
}

/// 🗂️ The editor-side, untyped twin of the persisted `Puzzle3dMeta`: both catalog members stay
/// opaque `DslValue`s here because the panels read them row-wise.
///
/// 🕳️ An ABSENT member must project as absent, never as `Null`: the persisted twin types
/// `kindCompatibility` as a real `Vec` and refuses a `Null` for it, and
/// `Puzzle3dPlaySnapshot::new`'s decode of the whole document is all-or-nothing — so one `null`
/// member used to replace the ENTIRE typed authority with an empty document while the projection kept
/// every object. Nothing reading the projection noticed (the world, the panels and the scene_snapshot delta
/// all read `value()`), but `ArtifactApp::interaction_topology` reads the typed half, and
/// `protocol::validate_state` prunes every selected id an empty topology does not contain — which is
/// the first-pick-invisible Inspection wave B6 measured in the browser
/// (26/09/02/PUZZLE-3D-END-TO-END wave B9 lane 2).
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dSceneMetadata {
    #[value(default, rename = "kindCatalogs", skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<semio_framework_value::DslValue>,
    #[value(default, rename = "kindCompatibility", skip_serializing_if = "Option::is_none")]
    pub kind_compatibility: Option<semio_framework_value::DslValue>,
}

/// 🧊️ Persisted oriented box constraining fill placement. Volume Brush creates axis-aligned
/// voxel-sized instances; the Transform gumball edits arbitrary oriented boxes.
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dTargetVolume {
    pub id: String,
    #[value(default)]
    pub origin: [f64; 3],
    #[value(default)]
    pub orientation: Option<[f64; 4]>,
    #[value(default)]
    pub scale: Option<semio_framework_value::DslValue>,
    #[value(default)]
    pub hidden: bool,
    #[value(default)]
    pub locked: bool,
}

#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dSceneSnapshot {
    pub schema: String,
    #[value(default)]
    pub domain: String,
    #[value(default)]
    pub meta: Puzzle3dSceneMetadata,
    #[value(default)]
    pub objects: Vec<Puzzle3dObject>,
    #[value(default)]
    pub attractions: Vec<Puzzle3dAttraction>,
    #[value(default, rename = "targetVolumes")]
    pub target_volumes: Vec<Puzzle3dTargetVolume>,
    #[value(default)]
    pub references: Vec<Puzzle3dReference>,
}

/// 🧾️ Transient render/mutation bundle pairing the persisted projection (the bare `Puzzle3dSceneSnapshot`
/// json) with the app's ephemeral view state. Never persisted — the `VcsArtifactApp` store owns the
/// scene_snapshot and `Puzzle3dConfig` owns the runtime — but rebuilt per call so the panel/world/engagement
/// helpers keep one `&Puzzle3dScene` signature.
#[derive(Clone, semio_framework_value::RetireOwned)]
pub struct Puzzle3dScene {
    pub scene_snapshot: Puzzle3dSceneSnapshot,
    pub runtime: Puzzle3dRuntime,
    /// 🧰️ The effective interaction id for this render/mutation — transient, never persisted.
    pub active_utility: String,
}

pub fn empty_scene_snapshot() -> Puzzle3dSceneSnapshot {
    Puzzle3dSceneSnapshot { schema: PUZZLE3D_SCENE_SNAPSHOT_SCHEMA.into(), domain: "architecture".into(), meta: Puzzle3dSceneMetadata::default(), objects: Vec::new(), attractions: Vec::new(), target_volumes: Vec::new(), references: Vec::new() }
}

const PUZZLE3D_CLIPBOARD_SCHEMA: &str = "puzzle.3d.clipboard.v1";

fn puzzle3d_scene_snapshot_from_view(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>) -> Puzzle3dSceneSnapshot {
    puzzle3d_scene_snapshot_from_play_snapshot(doc.snapshot)
}

pub fn puzzle3d_scene_snapshot_from_play_snapshot(snapshot: &Puzzle3dPlaySnapshot) -> Puzzle3dSceneSnapshot {
    puzzle3d_scene_snapshot_from_document(snapshot.typed())
}

fn puzzle3d_selected_objects(scene_snapshot: &Puzzle3dSceneSnapshot, interaction: &InteractionView<'_>) -> Vec<Puzzle3dObject> {
    puzzle3d_selected_objects_from(&Puzzle3dInteractionSnapshot::from_interaction(interaction), scene_snapshot)
}

pub fn puzzle3d_selected_objects_from(snapshot: &Puzzle3dInteractionSnapshot, scene_snapshot: &Puzzle3dSceneSnapshot) -> Vec<Puzzle3dObject> {
    let ids = snapshot.selected_object_ids();
    let mut objects: Vec<Puzzle3dObject> = scene_snapshot.objects.iter().filter(|object| ids.contains(&object.id)).cloned().collect();
    if objects.is_empty() {
        objects = scene_snapshot.objects.iter().filter(|object| snapshot.selected.iter().any(|id| id == &object.id)).cloned().collect();
    }
    if objects.is_empty() {
        objects = scene_snapshot
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

/// 📋️ Copies the selected objects as a scene_snapshot-shaped fragment.
pub fn puzzle3d_copy_fragment(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
    let scene_snapshot = puzzle3d_scene_snapshot_from_view(doc);
    puzzle3d_copy_fragment_from(&scene_snapshot, puzzle3d_selected_objects(&scene_snapshot, interaction))
}

pub fn puzzle3d_copy_fragment_from(scene_snapshot: &Puzzle3dSceneSnapshot, objects: Vec<Puzzle3dObject>) -> Result<ClipboardFragment, ClipboardError> {
    let _ = scene_snapshot;
    if objects.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let mut clip = empty_scene_snapshot();
    clip.schema = PUZZLE3D_CLIPBOARD_SCHEMA.into();
    clip.objects = objects;
    Ok(ClipboardFragment {
        schema: PUZZLE3D_CLIPBOARD_SCHEMA.into(),
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Design },
        dsl_text: to_json_string(&puzzle3d_projection_value(semio_framework_value::ToValue::to_value(&clip))),
        pack_bytes: None,
        source_app: PUZZLE3D_PLAY_APP_ID.into(),
        label: format!("{} objects", clip.objects.len()),
    })
}

/// 📋 Clones a copied scene_snapshot fragment with fresh ids, one `create-object` per clone.
pub fn puzzle3d_paste_operations(fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle3dMutation>, ClipboardError> {
    let expected = MediaType { class: MediaClass::ThreeD, form: MediaForm::Design };
    if fragment.media_type != expected {
        return Err(ClipboardError::IncompatibleMediaType(fragment.media_type.clone()));
    }
    let value = parse(&fragment.dsl_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    let clip = Puzzle3dSceneSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&value)).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    if clip.objects.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let offset = placement.position.unwrap_or([0.5, 0.5, 0.0]);
    clip.objects
        .into_iter()
        .map(|mut clone| {
            clone.id = next_object_id();
            clone.origin[0] += offset[0];
            clone.origin[1] += offset[1];
            clone.origin[2] += offset[2];
            snapshot_object_from_host_snapshot(&clone).map(|object| semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(object, None)).map_err(|error| ClipboardError::ParseFailed(error.to_string()))
        })
        .collect()
}

pub fn default_scene_snapshot() -> Puzzle3dSceneSnapshot {
    CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()
}

pub fn nakagin_scene_snapshot() -> Puzzle3dSceneSnapshot {
    NAKAGIN_EXAMPLE_SNAPSHOT.clone()
}

/// 👁️ Builds the native host view from the editor-owned intrinsic projection.
pub fn puzzle3d_projection_value<T:std::borrow::Borrow<semio_framework_value::DslValue>>(value:T)->Value{
    semio_framework_pack_json::from_dsl_value(value.borrow())
}

/// 🧾️ Builds the native scene directly from canonical typed authority.
pub fn scene_from_snapshot(document: &Puzzle3dSnapshot, runtime: Puzzle3dRuntime, active_utility: &str) -> Puzzle3dScene {
    Puzzle3dScene { scene_snapshot: puzzle3d_scene_snapshot_from_document(document), runtime, active_utility: active_utility.to_string() }
}

/// 🌉️ The persisted `Puzzle3dSnapshot` as this app's own structural-twin `Puzzle3dSceneSnapshot`. The
/// twin deliberately does not carry `anchor`, a vortex `label` or an attraction's diagram `x`/`y`, so
/// those are dropped here exactly as the `Value` bridge dropped them (an unknown key is ignored by
/// `FromValue`), and the two untyped catalog members are serialized once instead of per object.
pub fn puzzle3d_scene_snapshot_from_document(document: &Puzzle3dSnapshot) -> Puzzle3dSceneSnapshot {
    Puzzle3dSceneSnapshot {
        schema: document.schema.clone(),
        domain: document.domain.clone(),
        meta: Puzzle3dSceneMetadata { kind_catalogs: document.meta.kind_catalogs.as_ref().map(semio_framework_value::ToValue::to_value), kind_compatibility: Some(semio_framework_value::ToValue::to_value(&document.meta.kind_compatibility)) },
        objects: document.objects.iter().map(scene_object_from_snapshot).collect(),
        attractions: document.attractions.iter().map(scene_attraction_from_snapshot).collect(),
        target_volumes: document.target_volumes.iter().map(scene_target_volume_from_snapshot).collect(),
        references: document.references.iter().map(scene_reference_from_snapshot).collect(),
    }
}

/// 🧬️ Inverse of [`puzzle3d_scene_snapshot_from_document`] — the document delta must compare typed snapshots,
/// Native-only scale and catalog views must admit their typed fields or refuse the operation.
pub fn puzzle3d_snapshot_from_host_snapshot(scene_snapshot: &Puzzle3dSceneSnapshot) -> Result<Puzzle3dSnapshot,semio_framework_value::ValueError> {
    Ok(Puzzle3dSnapshot {
        schema: scene_snapshot.schema.clone(),
        domain: scene_snapshot.domain.clone(),
        meta: semio_s_artifact_puzzle_3d_model::Puzzle3dMeta {
            kind_catalogs: scene_snapshot.meta.kind_catalogs.clone().map(FromValue::from_value).transpose()?,
            kind_compatibility: match scene_snapshot.meta.kind_compatibility.clone(){Some(value)=>FromValue::from_value(value)?,None=>Default::default()},
        },
        objects: scene_snapshot.objects.iter().map(snapshot_object_from_host_snapshot).collect::<Result<_,_>>()?,
        attractions: scene_snapshot.attractions.iter().map(snapshot_attraction_from_host_snapshot).collect(),
        target_volumes: scene_snapshot.target_volumes.iter().map(snapshot_target_volume_from_host_snapshot).collect::<Result<_,_>>()?,
        references: scene_snapshot.references.iter().map(snapshot_reference_from_host_snapshot).collect(),
    })
}

fn snapshot_object_from_host_snapshot(object: &Puzzle3dObject) -> Result<semio_s_artifact_puzzle_3d_model::Puzzle3dObject,semio_framework_value::ValueError> {
    Ok(semio_s_artifact_puzzle_3d_model::Puzzle3dObject {
        id: object.id.clone(),
        label: object.label.clone(),
        object_kind: object.object_kind.clone(),
        anchor: semio_s_artifact_puzzle_3d_model::Puzzle3dObjectAnchor::default(),
        origin: object.origin,
        orientation: object.orientation,
        scale: object.scale.clone().map(FromValue::from_value).transpose()?,
        mesh_url: object.mesh_url.clone(),
        vortices: object.vortices.iter().map(snapshot_vortex_from_host_snapshot).collect(),
        hidden: object.hidden,
        locked: object.locked,
    })
}

fn snapshot_vortex_from_host_snapshot(vortex: &Puzzle3dVortex) -> semio_s_artifact_puzzle_3d_model::Puzzle3dVortex {
    semio_s_artifact_puzzle_3d_model::Puzzle3dVortex { id: vortex.id.clone(), vortex_kind: vortex.vortex_kind.clone(), label: None, position: vortex.position, direction: vortex.direction, radius: vortex.radius, hidden: vortex.hidden, locked: vortex.locked }
}

fn snapshot_attraction_from_host_snapshot(attraction: &Puzzle3dAttraction) -> semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction {
    semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction {
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

fn snapshot_target_volume_from_host_snapshot(volume: &Puzzle3dTargetVolume) -> Result<semio_s_artifact_puzzle_3d_model::Puzzle3dTargetVolume,semio_framework_value::ValueError> {
    Ok(semio_s_artifact_puzzle_3d_model::Puzzle3dTargetVolume { id: volume.id.clone(), origin: volume.origin, orientation: volume.orientation, scale: volume.scale.clone().map(FromValue::from_value).transpose()?, hidden: volume.hidden, locked: volume.locked })
}

fn snapshot_reference_from_host_snapshot(reference: &Puzzle3dReference) -> semio_s_artifact_puzzle_3d_model::Puzzle3dReference {
    semio_s_artifact_puzzle_3d_model::Puzzle3dReference {
        id: reference.id.clone(),
        source: semio_s_artifact_puzzle_3d_model::Puzzle3dReferenceSource { url: reference.source.url.clone(), media_kind: reference.source.media_kind.clone() },
        origin: reference.origin,
        width_world: reference.width_world,
        locked: reference.locked,
        hidden: reference.hidden,
    }
}

pub fn scene_object_from_snapshot(object: &semio_s_artifact_puzzle_3d_model::Puzzle3dObject) -> Puzzle3dObject {
    Puzzle3dObject {
        id: object.id.clone(),
        label: object.label.clone(),
        object_kind: object.object_kind.clone(),
        origin: object.origin,
        orientation: object.orientation,
        scale: object.scale.as_ref().map(semio_framework_value::ToValue::to_value),
        mesh_url: object.mesh_url.clone(),
        vortices: object.vortices.iter().map(scene_vortex_from_snapshot).collect(),
        hidden: object.hidden,
        locked: object.locked,
    }
}

fn scene_vortex_from_snapshot(vortex: &semio_s_artifact_puzzle_3d_model::Puzzle3dVortex) -> Puzzle3dVortex {
    Puzzle3dVortex { id: vortex.id.clone(), vortex_kind: vortex.vortex_kind.clone(), position: vortex.position, direction: vortex.direction, radius: vortex.radius, hidden: vortex.hidden, locked: vortex.locked }
}

fn scene_attraction_from_snapshot(attraction: &semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction) -> Puzzle3dAttraction {
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

fn scene_target_volume_from_snapshot(volume: &semio_s_artifact_puzzle_3d_model::Puzzle3dTargetVolume) -> Puzzle3dTargetVolume {
    Puzzle3dTargetVolume { id: volume.id.clone(), origin: volume.origin, orientation: volume.orientation, scale: volume.scale.as_ref().map(semio_framework_value::ToValue::to_value), hidden: volume.hidden, locked: volume.locked }
}

fn scene_reference_from_snapshot(reference: &semio_s_artifact_puzzle_3d_model::Puzzle3dReference) -> Puzzle3dReference {
    Puzzle3dReference {
        id: reference.id.clone(),
        source: Puzzle3dReferenceSource { url: reference.source.url.clone(), media_kind: reference.source.media_kind.clone() },
        origin: reference.origin,
        width_world: reference.width_world,
        locked: reference.locked,
        hidden: reference.hidden,
    }
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

/// 🌉️ `catalogs` is stored as `Puzzle3dSceneMetadata.kind_catalogs: Option<dsl::DslValue>` (a
/// `#[derive(value_derive::ToValue, FromValue)]` field, so it cannot hold this crate's own
/// `Value` — see this file's own `dsl::DslValue` field-type note), while `incoming` is the
/// still-`Value`-shaped `import_media` fragment; `DslValue`'s `Object`/`Array` variants are bare
/// `Vec`s with no `entry`/`as_*_mut` sugar, so the section lookup below is index-based instead of
/// `serde_json::Value`'s `entry(...).or_insert_with(...)`.
pub fn puzzle3d_upsert_catalog_rows(catalogs: &mut semio_framework_value::DslValue, section: &str, incoming: Option<&Value>) {
    let Some(incoming_rows) = incoming.and_then(Value::as_array) else {
        return;
    };
    if incoming_rows.is_empty() {
        return;
    }
    let semio_framework_value::DslValue::Object(catalog_entries) = catalogs else {
        return;
    };
    let section_index = match catalog_entries.iter().position(|(key, _)| key == section) {
        Some(index) => index,
        None => {
            catalog_entries.push((section.to_string(), semio_framework_value::DslValue::Array(Vec::new())));
            catalog_entries.len() - 1
        }
    };
    if !matches!(catalog_entries[section_index].1, semio_framework_value::DslValue::Array(_)) {
        catalog_entries[section_index].1 = semio_framework_value::DslValue::Array(Vec::new());
    }
    let semio_framework_value::DslValue::Array(existing) = &mut catalog_entries[section_index].1 else {
        return;
    };
    for row in incoming_rows {
        let row = if section == "objects" { puzzle3d_normalize_object_kind_row(row.clone()) } else { row.clone() };
        let Some(id) = row.get("id").and_then(Value::as_str) else {
            continue;
        };
        let row_dsl = semio_framework_pack_json::to_dsl_value(&row);
        match existing.iter().position(|entry| entry.get("id").and_then(semio_framework_value::DslValue::as_str) == Some(id)) {
            Some(index) => existing[index] = row_dsl,
            None => existing.push(row_dsl),
        }
    }
}

pub fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| FromValue::from_value(semio_framework_pack_json::to_dsl_value(value)).ok()).filter(|ids: &Vec<String>| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
}

/// 📢️ The localized notice of every refusal code the puzzle 3d guest names (design §20.12): a set-verb's missing flag value and
/// the fill tool's run refusals (`🛠️tools/🪣️fill`).
pub fn puzzle3d_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: LazyLock<[(&str, LocalizedLabel); 3]> = LazyLock::new(|| {
        [
            ("puzzle3d.action.flag-value-required", LocalizedLabel::native("Choose on or off for this setting.", "Für diese Einstellung Ein oder Aus wählen.")),
            ("puzzle3d.fill.scene-unavailable", LocalizedLabel::native("Fill cannot start: the document has nothing to place against.", "Füllen kann nicht starten: Das Dokument enthält nichts, woran platziert werden kann.")),
            ("puzzle3d.fill.provisional-unreadable", LocalizedLabel::native("The fill's unconfirmed placements could not be read back. Start the fill again.", "Die unbestätigten Platzierungen des Füllens konnten nicht gelesen werden. Füllen erneut starten.")),
        ]
    });
    &*NOTICES
}

/// 🙈️ The flag a set-verb sets to exactly the boolean its arguments carry (`setSelectionHidden{hidden}`,
/// `setSelectionLocked{locked}`) — the row target's explicit next state, so a stale view sets a value and never flips one.
pub fn puzzle3d_flag_value_argument(action: &str) -> Option<&'static str> {
    match action {
        "setSelectionHidden" => Some("hidden"),
        "setSelectionLocked" => Some("locked"),
        _ => None,
    }
}

/// 🙈️ A set-verb's definition: the explicit `{entity, ids}` a row names (or, left empty, the live selection) and the REQUIRED
/// boolean `flag` it sets — a missing value is refused, never defaulted.
pub fn puzzle3d_flag_value_action(id: &str, flag: &str, label: LocalizedLabel, value: LocalizedLabel) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, label, ActionKind::Mutation)
        .category("hand")
        .with_args([ActionArgDef::text("entity", LocalizedLabel::native("Entity", "Entität")), ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs")), ActionArgDef::toggle(flag, value).required()])
        .in_palette(false)
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
    pub fn of(meta: &'a Puzzle3dSceneMetadata) -> Self {
        let rows = meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("objects")).and_then(semio_framework_value::DslValue::as_array).unwrap_or_default();
        let mut by_kind = HashMap::with_capacity(rows.len());
        for entry in rows {
            // 🐛️ Imported rows are normalized to `representations` (`puzzle3d_normalize_object_kind_row` drops
            // `meshUrl`), so reading `meshUrl` alone dropped every kind no placed object renders: the fill's
            // mesh lane had no body for it and each of its candidates was rejected `mesh-unavailable`. Same
            // order as `resolve_object_kind_mesh_url`, so the lane key IS the key a candidate looks up.
            let url = entry.get("representations").and_then(semio_framework_value::DslValue::as_array).into_iter().flatten().filter_map(|rep| rep.get("url").and_then(semio_framework_value::DslValue::as_str)).map(str::trim).find(|url| !url.is_empty());
            let (Some(id), Some(url)) = (entry.get("id").and_then(semio_framework_value::DslValue::as_str), url.or_else(|| entry.get("meshUrl").and_then(semio_framework_value::DslValue::as_str))) else {
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

pub fn resolve_object_mesh_url(object: &Puzzle3dObject, meta: &Puzzle3dSceneMetadata) -> Option<String> {
    Puzzle3dKindMeshIndex::of(meta).resolve(object).map(str::to_string)
}

/// 🥽️ Every mesh identity the document renders or a catalog kind can place, sorted, so the world mesh lane
/// (and every tool run trace subject indexing it) keeps one stable order.
pub fn collect_mesh_urls(scene_snapshot: &Puzzle3dSceneSnapshot) -> Vec<String> {
    let index = Puzzle3dKindMeshIndex::of(&scene_snapshot.meta);
    let mut urls: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    for object in &scene_snapshot.objects {
        if let Some(url) = index.resolve(object) {
            urls.insert(url);
        }
    }
    urls.extend(index.catalog_urls());
    urls.into_iter().map(str::to_string).collect()
}

pub fn object_scale_json(object: &Puzzle3dObject) -> [f64; 3] {
    match object.scale.as_ref().and_then(semio_framework_value::DslValue::as_array) {
        Some(values) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        _ => [1.0, 1.0, 1.0],
    }
}

pub fn target_volume_scale_json(volume: &Puzzle3dTargetVolume) -> [f64; 3] {
    match volume.scale.as_ref().and_then(semio_framework_value::DslValue::as_array) {
        Some(values) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        _ => [1.0, 1.0, 1.0],
    }
}

pub fn world_vortex_position(object: &Puzzle3dObject, vortex: &Puzzle3dVortex) -> [f64; 3] {
    let orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let rotated = quat_rotate_vector(orientation, vortex.position);
    [object.origin.first().copied().unwrap_or(0.0) + rotated[0], object.origin.get(1).copied().unwrap_or(0.0) + rotated[1], object.origin.get(2).copied().unwrap_or(0.0) + rotated[2]]
}

pub fn resolve_vortex_world_position(scene_snapshot: &Puzzle3dSceneSnapshot, full_id: &str) -> Option<[f64; 3]> {
    for object in &scene_snapshot.objects {
        for vortex in &object.vortices {
            if puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id {
                return Some(world_vortex_position(object, vortex));
            }
        }
    }
    None
}

pub fn puzzle3d_catalog_entries<'a>(scene_snapshot: &'a Puzzle3dSceneSnapshot, section: &str) -> &'a [semio_framework_value::DslValue] {
    scene_snapshot.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get(section)).and_then(|entries| entries.as_array()).unwrap_or(&[])
}

pub fn puzzle3d_kind_ids(scene_snapshot: &Puzzle3dSceneSnapshot, section: &str) -> Vec<String> {
    puzzle3d_catalog_entries(scene_snapshot, section).iter().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

pub fn next_object_id() -> String {
    let next = PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("object-{next}")
}

/// 🏷️ One kind catalog row's display name — the same `label` / `name` / `id` precedence as the catalogue panel.
pub fn puzzle3d_kind_catalog_label(scene_snapshot: &Puzzle3dSceneSnapshot, kind_id: &str) -> String {
    puzzle3d_catalog_entries(scene_snapshot, "objects")
        .iter()
        .find(|entry| entry.get("id").and_then(semio_framework_value::DslValue::as_str) == Some(kind_id))
        .map(crate::catalog_entry_label)
        .unwrap_or_else(|| kind_id.to_string())
}

fn puzzle3d_label_root(label: &str) -> String {
    let Some((base, suffix)) = label.rsplit_once(' ') else {
        return label.to_string();
    };
    if suffix.parse::<u32>().is_ok() {
        base.to_string()
    } else {
        label.to_string()
    }
}

fn puzzle3d_label_number(label: &str, root: &str) -> Option<u32> {
    if label == root {
        return Some(1);
    }
    let rest = label.strip_prefix(root)?.strip_prefix(' ')?;
    rest.parse().ok()
}

/// 🏷️ What one object row and one 3D instance should read — authored label, else the kind's catalog name, else id.
pub fn puzzle3d_object_display_label(object: &Puzzle3dObject, scene_snapshot: &Puzzle3dSceneSnapshot) -> String {
    if let Some(label) = object.label.as_deref().filter(|value| !value.is_empty()) {
        return label.to_string();
    }
    object
        .object_kind
        .as_deref()
        .map(|kind| puzzle3d_kind_catalog_label(scene_snapshot, kind))
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| object.id.clone())
}

/// 🪧️ What a history-edit reference chip reads for one entity of the typed document (design §16.4) — the outliner's
/// names: an object as [`puzzle3d_object_display_label`] names it (authored label, else its kind's catalog label or
/// name, else the kind id), a vortex as `<object> · <vortex label or kind>`, an attraction as `<attracting object> →
/// <attracted object>`. Target volumes and references carry no authored name, so they keep the framework's
/// `<Kind> <short id>`; so does an id that is not one of `kinds`.
pub fn puzzle3d_entity_label(snapshot: &Puzzle3dSnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
    let wants = |kind: &str| kinds.is_empty() || kinds.iter().any(|declared| declared == kind);
    let text = |value: &Option<String>| value.clone().filter(|text| !text.is_empty());
    let object_label = |object: &semio_s_artifact_puzzle_3d_model::Puzzle3dObject| {
        text(&object.label)
            .or_else(|| {
                let kind = object.object_kind.as_deref()?;
                let entry = snapshot.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.objects.iter().find(|entry| entry.id == kind));
                Some(entry.and_then(|entry| [&entry.label, &entry.name].into_iter().find(|name| !name.is_empty()).cloned()).unwrap_or_else(|| kind.to_string()))
            })
            .unwrap_or_else(|| object.id.clone())
    };
    let port = |full_id: &str| snapshot.objects.iter().find_map(|object| object.vortices.iter().find(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id).map(|vortex| (object, vortex)));
    let label = if let Some(object) = wants(PUZZLE3D_GRANULARITY_OBJECT).then(|| snapshot.objects.iter().find(|object| object.id == id)).flatten() {
        Some(object_label(object))
    } else if let Some((object, vortex)) = wants(PUZZLE3D_GRANULARITY_VORTEX).then(|| port(id)).flatten() {
        Some(format!("{} \u{b7} {}", object_label(object), text(&vortex.label).or_else(|| text(&vortex.vortex_kind)).unwrap_or_else(|| vortex.id.clone())))
    } else if let Some(attraction) = wants(PUZZLE3D_GRANULARITY_ATTRACTION).then(|| snapshot.attractions.iter().find(|attraction| attraction.id == id)).flatten() {
        let end = |full_id: &str| port(full_id).map_or_else(|| full_id.to_string(), |(object, _)| object_label(object));
        Some(format!("{} \u{2192} {}", end(&attraction.attracting), end(&attraction.attracted)))
    } else {
        None
    };
    label.filter(|label| label != id).map(|label| LocalizedLabel::data(&label))
}

/// 🔢️ The next distinct object label for one kind — first instance takes the catalog name; further ones append ` 2`, ` 3`, … to the root taken from peers.
pub fn puzzle3d_next_object_label(objects: &[Puzzle3dObject], scene_snapshot: &Puzzle3dSceneSnapshot, kind_id: &str) -> String {
    let catalog_base = puzzle3d_kind_catalog_label(scene_snapshot, kind_id);
    let peers: Vec<&Puzzle3dObject> = objects.iter().filter(|object| object.object_kind.as_deref() == Some(kind_id)).collect();
    if peers.is_empty() {
        return catalog_base;
    }
    let root = peers
        .iter()
        .find_map(|object| object.label.as_deref().filter(|value| !value.is_empty()))
        .map(puzzle3d_label_root)
        .unwrap_or(catalog_base);
    let mut max = 0u32;
    let mut unlabeled = 0u32;
    for object in peers {
        if let Some(label) = object.label.as_deref().filter(|value| !value.is_empty()) {
            if let Some(number) = puzzle3d_label_number(label, &root) {
                max = max.max(number);
            }
        } else {
            unlabeled += 1;
        }
    }
    max = max.max(unlabeled);
    if max == 0 {
        root
    } else {
        format!("{root} {}", max + 1)
    }
}

/// 🧊️ Seeds real vortices for a freshly placed object from its kind catalog's `vortices` templates, so it is immediately brushable instead of connector-less.
///
/// 🐛️ A catalog TEMPLATE names its seat `point` (`Puzzle3dCatalogVortexTemplate`), a placed vortex
/// names it `position` (`Puzzle3dVortex`) — reading only `position` here silently stacked every
/// seeded vortex of every catalogued kind at the object origin, so a freshly added object offered one
/// degenerate seat instead of its real rim. `position` is kept as the fallback for the
/// already-placed-shaped rows `kit:in` imports carry.
pub fn puzzle3d_vortices_from_kind_template(catalog_entry: &semio_framework_value::DslValue) -> Vec<Puzzle3dVortex> {
    catalog_entry
        .get("vortices")
        .and_then(|value| value.as_array())
        .map(|templates| {
            templates
                .iter()
                .enumerate()
                .map(|(index, template)| {
                    let position = template.get("point").or_else(|| template.get("position")).and_then(|value| FromValue::from_value(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]);
                    let direction = template.get("direction").and_then(|value| FromValue::from_value(value.clone()).ok());
                    let radius = template.get("radius").and_then(|value| value.as_f64());
                    Puzzle3dVortex { id: format!("v{index}"), vortex_kind: template.get("vortexKind").and_then(|value| value.as_str()).map(str::to_string), position, direction, radius, hidden: false, locked: false }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// 🪣️ Whether the mode-level Fill tool currently authorizes fill planning and interaction.
pub fn puzzle3d_fill_tool_active(config: &Puzzle3dRuntime) -> bool {
    config.active_tool_id.as_deref() == Some(PUZZLE3D_FILL_TOOL_ID)
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
/// is not an addressing hit when an instance candidate exists — a window command otherwise looks the
/// utility map up under the kind (`map_hit=false`) and never sees the pane's armed utility.
pub fn puzzle3d_window_id_is_kind(id: &str) -> bool {
    id == semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID
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
    .unwrap_or(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)
}

/// 🛠️ The effective interaction id threaded through `Puzzle3dScene.active_utility`: the host-owned
/// per-window utility from `ViewModel`, unless the mode-level fill tool is active
/// (`active_tool_id`), in which case fill wins. Fill keeps its viewport interaction even though it is
/// declared as a windowless tool, not a `WindowKindDefinition` utility.
///
/// 🎯️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B18: `SET_ACTIVE_UTILITY` keys the map by window
/// INSTANCE. A kind-valued lookup (`puzzle3d-main`) — `registerBrushMesh` when `view.window_id` is the
/// kind — is a map miss (`map_hit=false`, `utility=` empty default). Kind lookups fall through to the
/// focused pane, then the first instance-prefixed map key.
pub fn puzzle3d_scene_active_utility(config: &Puzzle3dRuntime, view_state: Option<&semio_framework_plugin::ViewModel>, window_id: Option<&str>) -> String {
    if puzzle3d_fill_tool_active(config) {
        return PUZZLE3D_FILL_TOOL_ID.to_string();
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
    if let Some(id) = interaction.hovered_vortex_full_id(&envelope.scene_snapshot) {
        return Some(id.to_string());
    }
    let hovered_object = interaction.hovered_object_id(&envelope.scene_snapshot)?;
    let object = envelope.scene_snapshot.objects.iter().find(|object| object.id == hovered_object)?;
    object.vortices.first().map(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id))
}

/// 🕹️ One render pass' read of the framework-owned `vortex` interaction domain (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): the selected ids at the granularity they were
/// picked at, plus the pointer-channel hover. `DomainSelection` carries exactly one live granularity,
/// so this mirrors [`Puzzle3dActionCtx::selected_object_ids`] and friends verbatim — render and
/// reducer read the same shape, never two divergent projections.
///
/// 🐁️ Hover ids arrive WITHOUT a granularity (`protocol::DomainHover` has none), so classifying one
/// needs the document: `hovered_object_id`/`hovered_vortex_full_id` resolve against the live scene_snapshot
/// rather than guessing from the id's spelling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Puzzle3dInteractionSnapshot {
    pub granularity: String,
    pub selected: Vec<String>,
    pub hovered: Vec<String>,
    pub referenced: Vec<String>,
}

impl Puzzle3dInteractionSnapshot {
    /// 🕹️ Reads the live `vortex` domain: its selection, its `"pointer"`-channel hover and the ids the open time-travel
    /// draft references (what the world paints highlighted while a history edit is open).
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        let selection = interaction.selection(PUZZLE3D_INTERACTION_DOMAIN);
        let hover = interaction.hover(PUZZLE3D_INTERACTION_DOMAIN, PUZZLE3D_HOVER_CHANNEL);
        let leftover_ids = interaction.leftover_selected_ids();
        let selected = if selection.ids.is_empty() { leftover_ids } else { selection.ids.clone() };
        let granularity = if !selection.granularity.is_empty() { selection.granularity.clone() } else if !selected.is_empty() { PUZZLE3D_GRANULARITY_OBJECT.to_string() } else { selection.granularity.clone() };
        Self { granularity, selected, hovered: hover.ids.clone(), referenced: interaction.draft_references(PUZZLE3D_INTERACTION_DOMAIN).to_vec() }
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
        Self { granularity, selected, hovered, referenced: Vec::new() }
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

    /// 🐁️ The hovered object id, resolved against the scene_snapshot's own object ids.
    pub fn hovered_object_id(&self, scene_snapshot: &Puzzle3dSceneSnapshot) -> Option<&str> {
        self.hovered.iter().find(|id| scene_snapshot.objects.iter().any(|object| &object.id == *id)).map(String::as_str)
    }

    /// 🐁️ The hovered vortex full id (`{objectId}:{vortexId}`), resolved against the scene_snapshot.
    pub fn hovered_vortex_full_id(&self, scene_snapshot: &Puzzle3dSceneSnapshot) -> Option<&str> {
        self.hovered.iter().find(|id| scene_snapshot.objects.iter().any(|object| object.vortices.iter().any(|vortex| &puzzle3d_vortex_full_id(&object.id, &vortex.id) == *id))).map(String::as_str)
    }

    /// 🐁️ The hovered reference id, resolved against the scene_snapshot.
    pub fn hovered_reference_id(&self, scene_snapshot: &Puzzle3dSceneSnapshot) -> Option<&str> {
        self.hovered.iter().find(|id| scene_snapshot.references.iter().any(|reference| &reference.id == *id)).map(String::as_str)
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

pub fn value_as_vec3(value: &Value) -> Option<[f64; 3]> {
    let array = value.as_array()?;
    Some([array.first()?.as_f64()?, array.get(1)?.as_f64()?, array.get(2)?.as_f64()?])
}

/** 📐️ Resolves one numeric-field edit: an absolute `value` (typed entry) wins when present,
 * otherwise a `delta` (stepper nudge) is added to `current` — offset-preserving across a multi-select
 * where `current` differs per entity. `None` when neither parses. */
pub fn puzzle3d_resolve_number_edit(current: f64, value: Option<&Value>, delta: Option<&Value>) -> Option<f64> {
    if let Some(absolute) = value.and_then(Value::as_f64) {
        return Some(absolute);
    }
    delta.and_then(Value::as_f64).map(|delta| current + delta)
}

/** 📐️ Settings counterpart to `puzzle3d_resolve_number_edit`: reads `value`/`delta` directly
 * out of an action's `args`, for single global settings (not per-entity multi-select) whose stepper
 * dispatches straight to their own dedicated action. */
pub fn puzzle3d_absolute_or_delta(args: Option<&Value>, current: f64) -> Option<f64> {
    puzzle3d_resolve_number_edit(current, args.and_then(|value| value.get("value")), args.and_then(|value| value.get("delta")))
}

/** 📐️ Parses a nested stepper-group field id as `"<base>.<axis>"` (`x`/`y`/`z`/`w`), returning
 * the axis index when `field` names a component of `base` — the dot-path convention
 * `ui_inspector_vec3_group`/the inspector's quaternion group use for their per-axis actions. */
pub fn puzzle3d_axis_index(field: &str, base: &str) -> Option<usize> {
    match field.strip_prefix(base)?.strip_prefix('.')? {
        "x" => Some(0),
        "y" => Some(1),
        "z" => Some(2),
        "w" => Some(3),
        _ => None,
    }
}

/// 🧲️ The world offset an inspector stepper nudge states — `patchInspector{entity: object|targetVolume,
/// field: origin.x|y|z, delta}` without an absolute `value` — the one inspector edit that is a gesture, so it
/// commits the transform tool's `drag-selection` instead of an absolute origin. `None` for every other edit.
pub fn puzzle3d_inspector_origin_nudge(args: Option<&Value>) -> Option<[f64; 3]> {
    let args = args?;
    if !matches!(args.get("entity").and_then(Value::as_str), Some("object" | "targetVolume")) || args.get("value").is_some_and(|value| !value.is_null()) {
        return None;
    }
    let axis = puzzle3d_axis_index(args.get("field").and_then(Value::as_str)?, "origin").filter(|axis| *axis < 3)?;
    let delta = args.get("delta").and_then(Value::as_f64).filter(|delta| delta.is_finite())?;
    let mut offset = [0.0; 3];
    offset[axis] = delta;
    Some(offset)
}

/// 🎯️ Mirrors the host's client-side zoom-to-selection framing math so a keybinding/engagement-token
/// driven focus (which bypasses that host interception) still produces a sensible camera. Camera-only:
/// writes `envelope.runtime.camera` (session-only per-window state), never the shared `scene_snapshot`. An
/// EMPTY `selected_object_ids` frames the whole document — same subject rule as
/// `Puzzle3dFocusSelectionWork::frames`, so the retained lane and this one can never disagree.
pub fn apply_puzzle3d_focus_selection(envelope: &mut Puzzle3dScene, selected_object_ids: &[String]) {
    let selected_origins: Vec<[f64; 3]> = envelope.scene_snapshot.objects.iter().filter(|object| selected_object_ids.is_empty() || selected_object_ids.contains(&object.id)).map(|object| object.origin).collect();
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

/// 🎲️ Nested object/vortex distribution — one group per object kind (header slider = the sum of its
/// vortex joints), vortex children are absolute joint probabilities. The sum of every joint across all
/// objects is 1; `object_kind_weights` mirrors each object's joint sum for persistence and precompute.
pub const PUZZLE3D_JOINT_WEIGHT_SEP: &str = "::";

pub fn puzzle3d_joint_weight_key(object_kind_id: &str, vortex_kind_id: &str) -> String {
    format!("{object_kind_id}{PUZZLE3D_JOINT_WEIGHT_SEP}{vortex_kind_id}")
}

pub fn puzzle3d_joint_weights_use_composite_keys(joint_weights: &HashMap<String, f64>) -> bool {
    joint_weights.keys().any(|key| key.contains(PUZZLE3D_JOINT_WEIGHT_SEP))
}

pub fn puzzle3d_read_joint_weight(joint_weights: &HashMap<String, f64>, object_weights: &HashMap<String, f64>, object_kind_id: &str, vortex_kind_id: &str) -> f64 {
    let composite = puzzle3d_joint_weight_key(object_kind_id, vortex_kind_id);
    if let Some(weight) = joint_weights.get(&composite) {
        return *weight;
    }
    let object_weight = object_weights.get(object_kind_id).copied().unwrap_or(0.0);
    let vortex_weight = joint_weights.get(vortex_kind_id).copied().unwrap_or(0.0);
    object_weight * vortex_weight
}

pub fn puzzle3d_object_joint_total(joint_weights: &HashMap<String, f64>, object_weights: &HashMap<String, f64>, object_kind_id: &str, vortex_kind_ids: &[String]) -> f64 {
    vortex_kind_ids.iter().map(|vortex_kind_id| puzzle3d_read_joint_weight(joint_weights, object_weights, object_kind_id, vortex_kind_id)).sum()
}

pub fn puzzle3d_global_joint_total(joint_weights: &HashMap<String, f64>, object_weights: &HashMap<String, f64>, object_kind_ids: &[String], vortex_kind_ids: &[String]) -> f64 {
    object_kind_ids.iter().map(|object_kind_id| puzzle3d_object_joint_total(joint_weights, object_weights, object_kind_id, vortex_kind_ids)).sum()
}

pub fn puzzle3d_sync_object_kind_weights_from_joints(object_weights: &mut HashMap<String, f64>, joint_weights: &HashMap<String, f64>, object_kind_ids: &[String], vortex_kind_ids: &[String]) {
    for object_kind_id in object_kind_ids {
        let total = puzzle3d_object_joint_total(joint_weights, object_weights, object_kind_id, vortex_kind_ids);
        object_weights.insert(object_kind_id.clone(), total);
    }
}

pub fn puzzle3d_ensure_joint_distribution(object_weights: &mut HashMap<String, f64>, joint_weights: &mut HashMap<String, f64>, object_kind_ids: &[String], vortex_kind_ids: &[String]) {
    if object_kind_ids.is_empty() || vortex_kind_ids.is_empty() {
        object_weights.clear();
        joint_weights.clear();
        return;
    }
    if puzzle3d_joint_weights_use_composite_keys(joint_weights) {
        puzzle3d_sync_object_kind_weights_from_joints(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
        let total = puzzle3d_global_joint_total(joint_weights, object_weights, object_kind_ids, vortex_kind_ids);
        if total > f64::EPSILON && (total - 1.0).abs() > 0.001 {
            for object_kind_id in object_kind_ids {
                for vortex_kind_id in vortex_kind_ids {
                    let key = puzzle3d_joint_weight_key(object_kind_id, vortex_kind_id);
                    if let Some(weight) = joint_weights.get_mut(&key) {
                        *weight /= total;
                    }
                }
            }
            puzzle3d_sync_object_kind_weights_from_joints(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
        }
        return;
    }
    puzzle3d_ensure_catalog_kind_weights(object_weights, object_kind_ids);
    puzzle3d_ensure_catalog_kind_weights(joint_weights, vortex_kind_ids);
    let mut composite = HashMap::new();
    for object_kind_id in object_kind_ids {
        let object_weight = object_weights.get(object_kind_id).copied().unwrap_or(0.0);
        for vortex_kind_id in vortex_kind_ids {
            let vortex_weight = joint_weights.get(vortex_kind_id).copied().unwrap_or(0.0);
            composite.insert(puzzle3d_joint_weight_key(object_kind_id, vortex_kind_id), object_weight * vortex_weight);
        }
    }
    *joint_weights = composite;
    puzzle3d_sync_object_kind_weights_from_joints(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
}

pub fn puzzle3d_set_object_kind_distribution_weight(object_weights: &mut HashMap<String, f64>, joint_weights: &mut HashMap<String, f64>, object_kind_ids: &[String], vortex_kind_ids: &[String], changed_object_kind_id: &str, new_object_total: f64) {
    puzzle3d_ensure_joint_distribution(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
    let new_object_total = new_object_total.clamp(0.0, 1.0);
    let old_object_total = puzzle3d_object_joint_total(joint_weights, object_weights, changed_object_kind_id, vortex_kind_ids);
    let old_global = puzzle3d_global_joint_total(joint_weights, object_weights, object_kind_ids, vortex_kind_ids).max(f64::EPSILON);
    let old_other_total = (old_global - old_object_total).max(0.0);
    let new_other_total = (1.0 - new_object_total).max(0.0);
    if old_object_total > f64::EPSILON {
        let scale = new_object_total / old_object_total;
        for vortex_kind_id in vortex_kind_ids {
            let key = puzzle3d_joint_weight_key(changed_object_kind_id, vortex_kind_id);
            let joint = puzzle3d_read_joint_weight(joint_weights, object_weights, changed_object_kind_id, vortex_kind_id);
            joint_weights.insert(key, joint * scale);
        }
    } else if !vortex_kind_ids.is_empty() {
        let each = new_object_total / vortex_kind_ids.len() as f64;
        for vortex_kind_id in vortex_kind_ids {
            joint_weights.insert(puzzle3d_joint_weight_key(changed_object_kind_id, vortex_kind_id), each);
        }
    }
    if old_other_total > f64::EPSILON && new_other_total > f64::EPSILON {
        let scale = new_other_total / old_other_total;
        for object_kind_id in object_kind_ids {
            if object_kind_id == changed_object_kind_id {
                continue;
            }
            for vortex_kind_id in vortex_kind_ids {
                let key = puzzle3d_joint_weight_key(object_kind_id, vortex_kind_id);
                let joint = puzzle3d_read_joint_weight(joint_weights, object_weights, object_kind_id, vortex_kind_id);
                joint_weights.insert(key, joint * scale);
            }
        }
    } else if new_other_total <= f64::EPSILON {
        for object_kind_id in object_kind_ids {
            if object_kind_id == changed_object_kind_id {
                continue;
            }
            for vortex_kind_id in vortex_kind_ids {
                joint_weights.insert(puzzle3d_joint_weight_key(object_kind_id, vortex_kind_id), 0.0);
            }
        }
    }
    puzzle3d_sync_object_kind_weights_from_joints(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
}

pub fn puzzle3d_set_vortex_joint_weight(object_weights: &mut HashMap<String, f64>, joint_weights: &mut HashMap<String, f64>, object_kind_ids: &[String], vortex_kind_ids: &[String], object_kind_id: &str, vortex_kind_id: &str, new_joint: f64) {
    puzzle3d_ensure_joint_distribution(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
    let object_total = puzzle3d_object_joint_total(joint_weights, object_weights, object_kind_id, vortex_kind_ids);
    if object_total <= f64::EPSILON {
        return;
    }
    let new_joint = new_joint.clamp(0.0, object_total);
    let mut relative = HashMap::new();
    for id in vortex_kind_ids {
        relative.insert(id.clone(), puzzle3d_read_joint_weight(joint_weights, object_weights, object_kind_id, id) / object_total);
    }
    let share = (new_joint / object_total).clamp(0.0, 1.0);
    let normalized = puzzle3d_normalize_kind_weight_group(&relative, vortex_kind_ids, vortex_kind_id, share);
    for id in vortex_kind_ids {
        joint_weights.insert(puzzle3d_joint_weight_key(object_kind_id, id), normalized.get(id).copied().unwrap_or(0.0) * object_total);
    }
    puzzle3d_sync_object_kind_weights_from_joints(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
}

pub fn puzzle3d_apply_distribution_weight(object_weights: &mut HashMap<String, f64>, joint_weights: &mut HashMap<String, f64>, object_kind_ids: &[String], vortex_kind_ids: &[String], action: &str, kind_id: &str, object_kind_id: Option<&str>, value: f64) {
    if action == "setObjectKindWeight" {
        puzzle3d_set_object_kind_distribution_weight(object_weights, joint_weights, object_kind_ids, vortex_kind_ids, kind_id, value);
    } else if let Some(object_kind_id) = object_kind_id {
        puzzle3d_set_vortex_joint_weight(object_weights, joint_weights, object_kind_ids, vortex_kind_ids, object_kind_id, kind_id, value);
    } else {
        puzzle3d_ensure_joint_distribution(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
        let legacy = puzzle3d_normalize_kind_weight_group(joint_weights, vortex_kind_ids, kind_id, value);
        for vortex_kind_id in vortex_kind_ids {
            if let Some(weight) = legacy.get(vortex_kind_id) {
                for object_kind_id in object_kind_ids {
                    joint_weights.insert(puzzle3d_joint_weight_key(object_kind_id, vortex_kind_id), object_weights.get(object_kind_id).copied().unwrap_or(0.0) * *weight);
                }
            }
        }
        puzzle3d_sync_object_kind_weights_from_joints(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
    }
}

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

fn puzzle3d_object_kind_label(scene_snapshot: &Puzzle3dSceneSnapshot, object_kind_id: &str) -> String {
    puzzle3d_catalog_entries(scene_snapshot, "objects")
        .iter()
        .find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(object_kind_id))
        .and_then(|entry| entry.get("label").and_then(|value| value.as_str()).or_else(|| entry.get("name").and_then(|value| value.as_str())))
        .unwrap_or(object_kind_id)
        .to_string()
}

pub fn puzzle3d_joint_vortex_weight(object_weight: f64, vortex_weight: f64) -> f64 {
    object_weight * vortex_weight
}

/// 🎲️ Vortex-kind sliders under an object row — displayed value is the joint probability for that
/// object/vortex pair. Siblings under the same object redistribute when one moves; the object header
/// is the sum of its joints. Disabled when that sum is 0.
pub fn puzzle3d_joint_vortex_measures(object_kind_id: &str, object_weight: f64, vortex_kind_ids: &[String], object_weights: &HashMap<String, f64>, joint_weights: &HashMap<String, f64>) -> Vec<WindowMeasure> {
    let object_kind_zero = object_weight <= f64::EPSILON;
    vortex_kind_ids
        .iter()
        .map(|vortex_kind_id| {
            let joint = puzzle3d_read_joint_weight(joint_weights, object_weights, object_kind_id, vortex_kind_id);
            WindowMeasure::Slider {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-joint-vortex-{object_kind_id}-{vortex_kind_id}"),
                label: Some(vortex_kind_id.clone()),
                value: joint,
                min: 0.0,
                max: 1.0,
                step: Some(0.01),
                ready: None,
                loading: None,
                waiting: None,
                disabled: if object_kind_zero { Some(true) } else { None },
                on_change: puzzle3d_action("setVortexKindWeight", Some(json!({ "kindId": vortex_kind_id.as_str(), "objectKindId": object_kind_id }))),
            }
        })
        .collect()
}

pub fn puzzle3d_distribution_children(envelope: &Puzzle3dScene, default_open: Option<bool>) -> Vec<WindowMeasure> {
    let object_ids = puzzle3d_kind_ids(&envelope.scene_snapshot, "objects");
    let vortex_kind_ids = puzzle3d_kind_ids(&envelope.scene_snapshot, "vortices");
    let mut object_weights = envelope.runtime.object_kind_weights.clone();
    let mut joint_weights = envelope.runtime.vortex_kind_weights.clone();
    puzzle3d_ensure_joint_distribution(&mut object_weights, &mut joint_weights, &object_ids, &vortex_kind_ids);
    object_ids
        .iter()
        .map(|object_kind_id| {
            let object_weight = puzzle3d_object_joint_total(&joint_weights, &object_weights, object_kind_id, &vortex_kind_ids);
            let label = puzzle3d_object_kind_label(&envelope.scene_snapshot, object_kind_id);
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
                children: puzzle3d_joint_vortex_measures(object_kind_id, object_weight, &vortex_kind_ids, &object_weights, &joint_weights),
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

/// 🎯️ B1: `Puzzle3dPlayApp::Command` — the SOLE dispatch surface, one variant per declared
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
        #[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
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
            /// transitional `{action,args}` bridge and the test context dispatch helper.
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
    TranslateSelection = "translateSelection",
    RotateSelection = "rotateSelection",
    ScaleSelection = "scaleSelection",
    SetActiveExample = "setActiveExample",
    SetActiveTool = SET_ACTIVE_TOOL_ACTION_ID,
    AddObjectKind = "addObjectKind",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    ExportSnapshot = "exportSnapshot",
    ImportSnapshot = "importSnapshot",
    OpenImportSnapshot = "openImportSnapshot",
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
    SetSelectionHidden = "setSelectionHidden",
    SetSelectionLocked = "setSelectionLocked",
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
    SetBrushPlacementContactTolerance = "setBrushPlacementContactTolerance",
    SetObjectKindWeight = "setObjectKindWeight",
    SetVortexKindWeight = "setVortexKindWeight",
    CycleBrushCandidate = "cycleBrushCandidate",
    CycleBrushCandidateBack = "cycleBrushCandidateBack",
    OpenVortexSuggestions = "openVortexSuggestions",
    CloseVortexSuggestions = "closeVortexSuggestions",
    HoverSuggestion = "hoverSuggestion",
    AcceptSuggestion = "acceptSuggestion",
    TargetBrushSuggestions = "targetBrushSuggestions",
    RegisterBrushMesh = "registerBrushMesh",
    WorldPointerDown = "worldPointerDown",
    // 🗣️ B1: locale/terminology used to be host-pushed `ViewState` fields with no app-level action of
    // their own; now that `ViewState` is gone from the app-facing surface, they need a real Command.
}

impl protocol::OpBinary for Puzzle3dCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &[
        "openAddObjectDialog",
        "translateSelection",
        "rotateSelection",
        "scaleSelection",
        "setActiveExample",
        "addObjectKind",
        "deleteSelection",
        "duplicateSelection",
        "exportSnapshot",
        "importSnapshot",
        "openImportSnapshot",
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
        "setLodManual",
        "setGridSnapEnabled",
        "setGridSpacing",
        "setProximityRadius",
        "setChunkSize",
        "setSelectableKind",
        "setSelectionFlag",
        "setSelectionHidden",
        "setSelectionLocked",
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
        "setBrushPlacementContactTolerance",
        "setObjectKindWeight",
        "setVortexKindWeight",
        "cycleBrushCandidate",
        "cycleBrushCandidateBack",
        "openVortexSuggestions",
        "closeVortexSuggestions",
        "hoverSuggestion",
        "acceptSuggestion",
        "targetBrushSuggestions",
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
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        let value = parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error.into_value_error())))?;
        Self::from_json(&value).ok_or_else(|| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unrecognized Puzzle3dCommand tag"))))
    }
}

/// 📍️ The kind mesh vortex markers draw with; never a collision body.
pub const VORTEX_MARKER_MESH_KIND: &str = "vortex-marker";

/// 🗣️ Every sub-verb the engagement input accepts, in the order the window's placeholder advertises
/// them (`🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` `engagement`). The placeholder is derived from this
/// list, so an advertised verb that no arm below implements cannot exist.
pub const PUZZLE3D_ENGAGEMENT_VERBS: &[&str] = &["brush", "fill <n>", "zoom", "clear", "pick", "rectangle", "lasso"];

/// 🏷️ One catalog entry's display name — `label`, else `name`, else its `id`. Shared with the
/// manifest's `addObjectKind`/"Add Object" dialog option set (`✏️editor/🦀️.rs`), which must read
/// exactly the same catalog rows this panel renders.
pub fn catalog_entry_label(entry: &semio_framework_value::DslValue) -> String {
    entry.get("label").and_then(|value| value.as_str()).or_else(|| entry.get("name").and_then(|value| value.as_str())).or_else(|| entry.get("id").and_then(|value| value.as_str())).unwrap_or("kind").into()
}

pub const PUZZLE3D_FILL_TOOL_ID: &str = "fill";

//#region 🚪️Codecs
/// 🚪️ The play snapshot shares the typed authority's text, pack, store and SQLite representations: every codec reads and
/// writes through `Puzzle3dSnapshot`.
use store::sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteSnapshotControl};
use store::os_store::serialization::ArtifactSqliteSnapshotHost;
use semio_framework_value::ValueError;
use pack::snapshot::ArtifactSqliteSnapshot as _;

impl pack::snapshot::ArtifactDsl for Puzzle3dPlaySnapshot {
    const EXTENSION: &'static str = "puzzle3d-play";

    fn envelope_id() -> &'static str { <Puzzle3dSnapshot as pack::snapshot::ArtifactDsl>::envelope_id() }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { <Puzzle3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map(Self::from_typed) }
    fn print_dsl(&self) -> String { <Puzzle3dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(self.typed()) }
}

/// 📦️ Packs through the typed authority, so the play kind shares `Puzzle3dSnapshot`'s derived record
/// layout and pack-schema identity.
impl pack::snapshot::ArtifactPack for Puzzle3dPlaySnapshot {
    
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        self.typed().encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        <Puzzle3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack_with(bytes, options).map(Self::from_typed)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Puzzle3dSnapshot as pack::snapshot::ArtifactPack>::record_spec()
    }
}

impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Puzzle3dPlaySnapshot {
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {Some(<Self as store::os_store::serialization::ArtifactSqliteSnapshotHost>::sqlite_codec())}
}

impl ArtifactSqliteSnapshotHost for Puzzle3dPlaySnapshot {
    
    
    
    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, c: &mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        Puzzle3dSnapshot::decode_sqlite_snapshot_native(payload, c,native_control).map(Self::from_typed)
    }
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, c: &mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>) -> Result<store::io_schema::IoPayload, ValueError> {
        self.typed().encode_sqlite_snapshot_native(encoding, c,native_owner)
    }
    fn validate_sqlite_snapshot_subset(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
        self.typed().validate_sqlite_snapshot_subset(dialect, d, c)
    }
}

impl pack::snapshot::ArtifactSqliteSnapshot for Puzzle3dPlaySnapshot {
const SQLITE_SCHEMA: &'static str = Puzzle3dSnapshot::SQLITE_SCHEMA;
fn to_sqlite_database(&self, c: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        self.typed().to_sqlite_database(c)
    }
fn from_sqlite_database(d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        Puzzle3dSnapshot::from_sqlite_database(d, c).map(Self::from_typed)
    }
}
//#endregion 🚪️Codecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️play-pack/🦀️.rs"]
mod play_pack_tests;
//#endregion 🧪️Tests
