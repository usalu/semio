//! 🔖️ Constants, examples and scene types: the controller, granularity and hover constants, the shipped example json, the vortex, reference, object, target volume, attraction and scene types and the engagement verbs. See [`super`].

use super::*;

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
