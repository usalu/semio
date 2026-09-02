//! 🎬️ The 15 product scene payloads — one per [`ui_contract::SurfaceKind`] wire tag — relocated
//! verbatim from `ui_wgpu`'s `🎯️targets/🧊️wgpu/🦀️component.rs` (ticket
//! 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME packet `scene-surface`). `ui_wgpu` now
//! re-exports these types instead of defining them, so every existing `ui_wgpu::wgpu::WorldXScene`
//! reference keeps compiling unchanged.
//!
//! Every field here is plain wasm-safe data (`String`/`f64`/`bool`/`Option`/`Vec`/nested plain
//! records) — this crate depends on nothing beyond `ui_contract` and `serde`. Two fields could not
//! move byte-identical for that reason: [`TableScene::drop_action_json`] (was
//! `Option<ActionDescriptor>`, a `ui_wgpu`-only type pulling `dsl::DslValue`/`Label`/`IconName`) and
//! [`NodeGraphOperatorChannelRecord::default_json`] (was `Option<serde_json::Value>`, which would
//! have been this crate's first dependency beyond `ui_contract`/`serde`). Every sibling field on
//! these 15 structs already uses the `_json: String` opaque-payload convention for exactly this
//! class of renderer-specific/arbitrary-shaped data (`selection_json`, `sort_json`, `camera_json`,
//! ...) — both renamed fields just apply that same established convention to their one outlier,
//! rather than inventing a shim or a new dependency.
//!
//! 🚫️async: E6 sync payload construction — the `.base()` constructors below are plain sync `fn` by
//! decree (ticket ruling E6), not the `pub async fn` they arrived as in `ui_wgpu`.

use protocol::value::{DslValue, FromValue, ToValue, ValueError};
use serde::{Deserialize, Serialize};

//#region 🔖️SceneDoc
/// 🆔️ The version axis every renderer/decoder gates on: `T::SCHEMA` is stamped into
/// `SurfaceProps::doc_schema` by [`crate::pack::encode`] and checked back by
/// [`crate::pack::decode`], which refuses to decode bytes stamped with any other schema.
pub trait SceneDoc: Clone + Serialize + serde::de::DeserializeOwned {
    /// 🏷️ `"<surface-kind-wire-tag>@<version>"` — the wire tag half agrees verbatim with the
    /// matching [`ui_contract::SurfaceKind`] variant's own `#[serde(rename = ...)]` (including the
    /// one deliberate inconsistency, `virtualFileSystem`, preserved for the same reason
    /// `ui_contract`'s own `SurfaceKind` doc gives: a rename is a breaking wire change for a later
    /// packet to make on purpose, not a silent side effect of this move).
    const SCHEMA: &'static str;

    /// 🧳️ Encodes the stable opaque scene payload independently of its human-readable JSON shape.
    fn encode_pack(&self) -> Result<Vec<u8>, crate::pack::PackError> {
        crate::pack::to_bytes(self)
    }

    /// 🧳️ Decodes the stable opaque scene payload independently of its human-readable JSON shape.
    fn decode_pack(bytes: &[u8]) -> Result<Self, crate::pack::PackError> {
        crate::pack::from_bytes(bytes)
    }
}

macro_rules! scene_pack_wire {
    ($wire:ident, $scene:ident { $($field:ident: $ty:ty),+ $(,)? }) => {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct $wire {
            $($field: $ty),+
        }

        impl From<&$scene> for $wire {
            fn from(scene: &$scene) -> Self {
                Self { $($field: scene.$field.clone()),+ }
            }
        }

        impl From<$wire> for $scene {
            fn from(wire: $wire) -> Self {
                Self { $($field: wire.$field),+ }
            }
        }
    };
}
//#endregion 🔖️SceneDoc

//#region 🔖️ValueCodecHelpers
/// 🧰️ Shared object-entry primitives every hand-written `ToValue`/`FromValue` impl below builds on
/// — `#[derive(ToValue, FromValue)]` hardcodes `::semio_framework_os_kernel::…` paths (see this
/// crate's `Cargo.toml` docstring), which this wasm-safe, os-kernel-free crate must never depend on,
/// so every scene/record type is encoded/decoded by hand against `protocol::value::` directly.
/// Mirrors serde semantics exactly: an `Option<T>` field decodes to `None` when its key is absent
/// (matching serde's implicit optionality for `Option` fields, independent of whether
/// `#[serde(default)]` is spelled out), a field with an explicit `#[serde(default = ...)]` falls
/// back to that default when absent, and every other field is required.
fn value_field(entries: &[(String, DslValue)], key: &str) -> Option<DslValue> {
    entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
}

fn value_required(entries: &[(String, DslValue)], key: &str) -> Result<DslValue, ValueError> {
    value_field(entries, key).ok_or_else(|| ValueError::new(format!("missing field `{key}`")))
}

/// 🌱️ Decodes a required (non-defaulted, non-`Option`) field.
fn value_decode<T: FromValue>(entries: &[(String, DslValue)], key: &str) -> Result<T, ValueError> {
    T::from_value(value_required(entries, key)?).map_err(|error| error.under(key))
}

/// 🌱️ Decodes an `Option<T>` field — a missing key or an explicit `null` both decode to `None`.
fn value_decode_option<T: FromValue>(entries: &[(String, DslValue)], key: &str) -> Result<Option<T>, ValueError> {
    match value_field(entries, key) {
        None | Some(DslValue::Null) => Ok(None),
        Some(value) => T::from_value(value).map(Some).map_err(|error| error.under(key)),
    }
}

/// 🌱️ Decodes a field with a `#[serde(default = ...)]` fallback — a missing key calls `default`.
fn value_decode_default<T: FromValue>(entries: &[(String, DslValue)], key: &str, default: impl FnOnce() -> T) -> Result<T, ValueError> {
    match value_field(entries, key) {
        None => Ok(default()),
        Some(value) => T::from_value(value).map_err(|error| error.under(key)),
    }
}

/// 🌱️ Pushes a field unconditionally — the twin of a plain (non-`skip_serializing_if`) struct field.
fn value_push<T: ToValue>(entries: &mut Vec<(String, DslValue)>, key: &str, value: &T) {
    entries.push((key.to_string(), value.to_value()));
}

/// 🌱️ Pushes an `Option<T>` field only when `Some` — mirrors `#[serde(skip_serializing_if =
/// "Option::is_none")]`.
fn value_push_option<T: ToValue>(entries: &mut Vec<(String, DslValue)>, key: &str, value: &Option<T>) {
    if let Some(value) = value {
        entries.push((key.to_string(), value.to_value()));
    }
}

/// 🌱️ Pushes a `Vec<T>` field only when non-empty — mirrors `#[serde(skip_serializing_if =
/// "Vec::is_empty")]`.
fn value_push_if_nonempty<T: ToValue>(entries: &mut Vec<(String, DslValue)>, key: &str, value: &[T]) {
    if !value.is_empty() {
        entries.push((key.to_string(), DslValue::Array(value.iter().map(ToValue::to_value).collect())));
    }
}
//#endregion 🔖️ValueCodecHelpers

//#region 🔖️Canvas2dScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canvas2dScene {
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub layers_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<crate::Canvas2dSnapshotLease>,
}

impl SceneDoc for Canvas2dScene {
    const SCHEMA: &'static str = "canvas-2d@1";
}

impl ToValue for Canvas2dScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "cameraX", &self.camera_x);
        value_push(&mut entries, "cameraY", &self.camera_y);
        value_push(&mut entries, "zoom", &self.zoom);
        value_push(&mut entries, "layersJson", &self.layers_json);
        value_push_option(&mut entries, "snapshot", &self.snapshot);
        DslValue::Object(entries)
    }
}

impl FromValue for Canvas2dScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            camera_x: value_decode(&entries, "cameraX")?,
            camera_y: value_decode(&entries, "cameraY")?,
            zoom: value_decode(&entries, "zoom")?,
            layers_json: value_decode(&entries, "layersJson")?,
            snapshot: value_decode_option(&entries, "snapshot")?,
        })
    }
}
//#endregion 🔖️Canvas2dScene

//#region 🔖️World3dScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World3dScene {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<crate::World3dSnapshotLease>,
    pub camera_json: String,
    #[serde(default = "world3d_default_meshes_json")]
    pub meshes_json: String,
    pub instances_json: String,
    #[serde(default = "world3d_default_selection_json")]
    pub selection_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vortices_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attractions_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_volumes_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub references_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brush_preview_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engagement_preview_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lod_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunking_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fit_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terrain_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub points_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_json: Option<String>,
    /// 🪟️ The framework [`InteractionRef`]/`InteractionDefinition` id this world window is bound to,
    /// serialized as `domainId`. `None` leaves the window on the OS's own shared `world` board
    /// domain and plain plugin-private actions (`setHover`/`worldPick`/`worldSelect`). When set, a
    /// renderer routes its own instance pick/hover through the framework verbs
    /// `interactionSelect`/`interactionHover` on this domain instead — see
    /// `world3d_scene_extended`'s own doc comment for the constructor side of this contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// 🎯️ `domain_id`'s bound domain granularity id for a plain (non-component) instance pick/hover
    /// hit — `None` when `domain_id` is `None` or the domain has no plain-hit granularity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_granularity_id: Option<String>,
}

scene_pack_wire!(World3dScenePack, World3dScene {
    snapshot: Option<crate::World3dSnapshotLease>,
    camera_json: String,
    meshes_json: String,
    instances_json: String,
    selection_json: String,
    vortices_json: Option<String>,
    attractions_json: Option<String>,
    target_volumes_json: Option<String>,
    references_json: Option<String>,
    brush_preview_json: Option<String>,
    interaction_json: Option<String>,
    engagement_preview_json: Option<String>,
    lod_json: Option<String>,
    chunking_json: Option<String>,
    environment_json: Option<String>,
    frame_json: Option<String>,
    fit_json: Option<String>,
    terrain_json: Option<String>,
    points_json: Option<String>,
    status_json: Option<String>,
    domain_id: Option<String>,
    domain_granularity_id: Option<String>,
});

impl SceneDoc for World3dScene {
    const SCHEMA: &'static str = "world-3d@1";

    fn encode_pack(&self) -> Result<Vec<u8>, crate::pack::PackError> {
        crate::pack::to_bytes(&World3dScenePack::from(self))
    }

    fn decode_pack(bytes: &[u8]) -> Result<Self, crate::pack::PackError> {
        crate::pack::from_bytes::<World3dScenePack>(bytes).map(Into::into)
    }
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn world3d_default_meshes_json() -> String {
    "[]".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn world3d_default_selection_json() -> String {
    r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#.into()
}

impl World3dScene {
    /** @emoji 🌐️ Builds a world-3d scene with optional extensions unset. */
    pub fn base(camera_json: String, meshes_json: String, instances_json: String, selection_json: String) -> Self {
        Self {
            snapshot: None,
            camera_json,
            meshes_json,
            instances_json,
            selection_json,
            vortices_json: None,
            attractions_json: None,
            target_volumes_json: None,
            references_json: None,
            brush_preview_json: None,
            interaction_json: None,
            engagement_preview_json: None,
            lod_json: None,
            chunking_json: None,
            environment_json: None,
            frame_json: None,
            fit_json: None,
            terrain_json: None,
            points_json: None,
            status_json: None,
            domain_id: None,
            domain_granularity_id: None,
        }
    }
}

impl ToValue for World3dScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push_option(&mut entries, "snapshot", &self.snapshot);
        value_push(&mut entries, "cameraJson", &self.camera_json);
        value_push(&mut entries, "meshesJson", &self.meshes_json);
        value_push(&mut entries, "instancesJson", &self.instances_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        value_push_option(&mut entries, "vorticesJson", &self.vortices_json);
        value_push_option(&mut entries, "attractionsJson", &self.attractions_json);
        value_push_option(&mut entries, "targetVolumesJson", &self.target_volumes_json);
        value_push_option(&mut entries, "referencesJson", &self.references_json);
        value_push_option(&mut entries, "brushPreviewJson", &self.brush_preview_json);
        value_push_option(&mut entries, "interactionJson", &self.interaction_json);
        value_push_option(&mut entries, "engagementPreviewJson", &self.engagement_preview_json);
        value_push_option(&mut entries, "lodJson", &self.lod_json);
        value_push_option(&mut entries, "chunkingJson", &self.chunking_json);
        value_push_option(&mut entries, "environmentJson", &self.environment_json);
        value_push_option(&mut entries, "frameJson", &self.frame_json);
        value_push_option(&mut entries, "fitJson", &self.fit_json);
        value_push_option(&mut entries, "terrainJson", &self.terrain_json);
        value_push_option(&mut entries, "pointsJson", &self.points_json);
        value_push_option(&mut entries, "statusJson", &self.status_json);
        value_push_option(&mut entries, "domainId", &self.domain_id);
        value_push_option(&mut entries, "domainGranularityId", &self.domain_granularity_id);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            snapshot: value_decode_option(&entries, "snapshot")?,
            camera_json: value_decode(&entries, "cameraJson")?,
            meshes_json: value_decode_default(&entries, "meshesJson", world3d_default_meshes_json)?,
            instances_json: value_decode(&entries, "instancesJson")?,
            selection_json: value_decode_default(&entries, "selectionJson", world3d_default_selection_json)?,
            vortices_json: value_decode_option(&entries, "vorticesJson")?,
            attractions_json: value_decode_option(&entries, "attractionsJson")?,
            target_volumes_json: value_decode_option(&entries, "targetVolumesJson")?,
            references_json: value_decode_option(&entries, "referencesJson")?,
            brush_preview_json: value_decode_option(&entries, "brushPreviewJson")?,
            interaction_json: value_decode_option(&entries, "interactionJson")?,
            engagement_preview_json: value_decode_option(&entries, "engagementPreviewJson")?,
            lod_json: value_decode_option(&entries, "lodJson")?,
            chunking_json: value_decode_option(&entries, "chunkingJson")?,
            environment_json: value_decode_option(&entries, "environmentJson")?,
            frame_json: value_decode_option(&entries, "frameJson")?,
            fit_json: value_decode_option(&entries, "fitJson")?,
            terrain_json: value_decode_option(&entries, "terrainJson")?,
            points_json: value_decode_option(&entries, "pointsJson")?,
            status_json: value_decode_option(&entries, "statusJson")?,
            domain_id: value_decode_option(&entries, "domainId")?,
            domain_granularity_id: value_decode_option(&entries, "domainGranularityId")?,
        })
    }
}
//#endregion 🔖️World3dScene

//#region 🔖️NodeGraphRecords
/// 🔌️ One port on a node-graph node: identity + display label. Direction is implied by whether the
/// record lives in the owning node's `inputs` or `outputs` list, not carried as a field.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphPortRecord {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceKind")]
    pub artifact_kind: Option<String>,
}

/// 🕸️ One node-graph node: identity, label, layout rect, typed input/output ports.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphNodeRecord {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    #[serde(default)]
    pub inputs: Vec<NodeGraphPortRecord>,
    #[serde(default)]
    pub outputs: Vec<NodeGraphPortRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

/// 🕸️ One node-graph edge between two node/port endpoints.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphEdgeRecord {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 📷️ Node-graph camera: pan position + zoom factor.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphViewport {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "node_graph_default_zoom")]
    pub zoom: f64,
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
fn node_graph_default_zoom() -> f64 {
    1.0
}

/// 🔎️ One spotlight/find result row for a node-graph surface.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphFindItem {
    pub id: String,
    pub label: String,
    pub category: String,
}

/// 🖱️ Hovered node id, if any.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphHover {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 🔌️ When set, the graph paints the hovered channel (port) on `node_id`, matching the
    /// `"{nodeId}@{portId}"` pick id (see `nodeGraphPickChannel`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port_id: Option<String>,
}

/// ➕️ Variadic input/output slot on an operator catalogue entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphOperatorVariadicRecord {
    pub slot_key: String,
    pub min: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<usize>,
}

/// 🔌️ Declared operator channel (input or output).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphOperatorChannelRecord {
    pub code: String,
    pub abbreviation: String,
    pub name: String,
    pub full_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operators: Vec<String>,
    /// 🕳️ Opaque JSON-encoded default value — was `Option<serde_json::Value>`; this crate depends
    /// on nothing beyond `ui_contract`/`serde`, so the arbitrary-shaped default rides as a JSON
    /// string like every sibling `_json` field on these scene structs already does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    pub cardinality: String,
}

/// 🧠️ One operator catalogue entry offered to a flow-backed node-graph's spotlight/palette.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphOperatorRecord {
    pub id: String,
    pub extension: String,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
    #[serde(default)]
    pub inputs: Vec<NodeGraphOperatorChannelRecord>,
    #[serde(default)]
    pub outputs: Vec<NodeGraphOperatorChannelRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variadic_input: Option<NodeGraphOperatorVariadicRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variadic_output: Option<NodeGraphOperatorVariadicRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<String>,
}

//#region 🔖️NodeGraphRecordsFromValue
/// 🌱️ Hand-written `FromValue` (decode direction only — nothing here needs `ToValue`, see the
/// Cargo.toml docstring on the `protocol` dependency for why this is hand-written rather than
/// `#[derive(ToValue, FromValue)]`) for the plugin-side JSON-round-trip shims (`space`'s
/// `json_array_to_node_graph_*` in `🕸️compiled-dag`/`🔄️workflow` window files) that decode a
/// first-party `pack::from_json_str::<Vec<NodeGraphXRecord>>(...)` off text a framework producer
/// still emits via `serde_json`. Mirrors each struct's own `#[serde(rename_all = "camelCase",
/// default, skip_serializing_if = ...)]` attributes field-by-field: a missing key decodes to the
/// same default a missing/omitted JSON key would under `serde`.
impl FromValue for NodeGraphPortRecord {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(format!("missing field `{key}`")));
        let opt = |key: &str| get(key).map(Option::<String>::from_value).transpose().map(Option::flatten);
        Ok(Self {
            id: String::from_value(field("id")?)?,
            label: opt("label")?,
            code: opt("code")?,
            abbreviation: opt("abbreviation")?,
            full_name: opt("fullName")?,
            artifact_kind: opt("resourceKind")?,
        })
    }
}

impl FromValue for NodeGraphNodeRecord {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(format!("missing field `{key}`")));
        let opt = |key: &str| get(key).map(Option::<String>::from_value).transpose().map(Option::flatten);
        Ok(Self {
            id: String::from_value(field("id")?)?,
            label: opt("label")?,
            x: f64::from_value(field("x")?)?,
            y: f64::from_value(field("y")?)?,
            width: f64::from_value(field("width")?)?,
            height: f64::from_value(field("height")?)?,
            inputs: get("inputs").map(Vec::from_value).transpose()?.unwrap_or_default(),
            outputs: get("outputs").map(Vec::from_value).transpose()?.unwrap_or_default(),
            instance_id: opt("instanceId")?,
            plugin_id: opt("pluginId")?,
            app_id: opt("appId")?,
            icon: opt("icon")?,
        })
    }
}

impl FromValue for NodeGraphEdgeRecord {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(format!("missing field `{key}`")));
        let opt = |key: &str| get(key).map(Option::<String>::from_value).transpose().map(Option::flatten);
        Ok(Self {
            id: String::from_value(field("id")?)?,
            source_node_id: String::from_value(field("sourceNodeId")?)?,
            source_port_id: String::from_value(field("sourcePortId")?)?,
            target_node_id: String::from_value(field("targetNodeId")?)?,
            target_port_id: String::from_value(field("targetPortId")?)?,
            label: opt("label")?,
        })
    }
}

impl FromValue for NodeGraphFindItem {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(format!("missing field `{key}`")));
        Ok(Self {
            id: String::from_value(field("id")?)?,
            label: String::from_value(field("label")?)?,
            category: String::from_value(field("category")?)?,
        })
    }
}

impl FromValue for NodeGraphOperatorVariadicRecord {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(format!("missing field `{key}`")));
        Ok(Self {
            slot_key: String::from_value(field("slotKey")?)?,
            min: usize::from_value(field("min")?)?,
            max: get("max").map(Option::<usize>::from_value).transpose()?.flatten(),
        })
    }
}

impl FromValue for NodeGraphOperatorChannelRecord {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(format!("missing field `{key}`")));
        let opt = |key: &str| get(key).map(Option::<String>::from_value).transpose().map(Option::flatten);
        Ok(Self {
            code: String::from_value(field("code")?)?,
            abbreviation: String::from_value(field("abbreviation")?)?,
            name: String::from_value(field("name")?)?,
            full_name: String::from_value(field("fullName")?)?,
            operators: get("operators").map(Vec::from_value).transpose()?.unwrap_or_default(),
            default_json: opt("defaultJson")?,
            label: opt("label")?,
            cardinality: get("cardinality").map(String::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

impl FromValue for NodeGraphOperatorRecord {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(format!("missing field `{key}`")));
        Ok(Self {
            id: String::from_value(field("id")?)?,
            extension: String::from_value(field("extension")?)?,
            name: String::from_value(field("name")?)?,
            abbreviation: String::from_value(field("abbreviation")?)?,
            icon: String::from_value(field("icon")?)?,
            summary: String::from_value(field("summary")?)?,
            inputs: get("inputs").map(Vec::from_value).transpose()?.unwrap_or_default(),
            outputs: get("outputs").map(Vec::from_value).transpose()?.unwrap_or_default(),
            variadic_input: get("variadicInput").map(Option::<NodeGraphOperatorVariadicRecord>::from_value).transpose()?.flatten(),
            variadic_output: get("variadicOutput").map(Option::<NodeGraphOperatorVariadicRecord>::from_value).transpose()?.flatten(),
            group: get("group").map(Vec::from_value).transpose()?.unwrap_or_default(),
        })
    }
}
//#endregion 🔖️NodeGraphRecordsFromValue

//#region 🔖️NodeGraphRecordsToValue
/// 🌱️ Hand-written `ToValue` (the encode direction — see `🔖️NodeGraphRecordsFromValue` above for
/// why this is hand-written rather than derived) for the same record types, plus both directions
/// for `NodeGraphViewport`/`NodeGraphHover`, which had neither before this pass.
impl ToValue for NodeGraphPortRecord {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "id", &self.id);
        value_push_option(&mut entries, "label", &self.label);
        value_push_option(&mut entries, "code", &self.code);
        value_push_option(&mut entries, "abbreviation", &self.abbreviation);
        value_push_option(&mut entries, "fullName", &self.full_name);
        value_push_option(&mut entries, "resourceKind", &self.artifact_kind);
        DslValue::Object(entries)
    }
}

impl ToValue for NodeGraphNodeRecord {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "id", &self.id);
        value_push_option(&mut entries, "label", &self.label);
        value_push(&mut entries, "x", &self.x);
        value_push(&mut entries, "y", &self.y);
        value_push(&mut entries, "width", &self.width);
        value_push(&mut entries, "height", &self.height);
        value_push(&mut entries, "inputs", &self.inputs);
        value_push(&mut entries, "outputs", &self.outputs);
        value_push_option(&mut entries, "instanceId", &self.instance_id);
        value_push_option(&mut entries, "pluginId", &self.plugin_id);
        value_push_option(&mut entries, "appId", &self.app_id);
        value_push_option(&mut entries, "icon", &self.icon);
        DslValue::Object(entries)
    }
}

impl ToValue for NodeGraphEdgeRecord {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "id", &self.id);
        value_push(&mut entries, "sourceNodeId", &self.source_node_id);
        value_push(&mut entries, "sourcePortId", &self.source_port_id);
        value_push(&mut entries, "targetNodeId", &self.target_node_id);
        value_push(&mut entries, "targetPortId", &self.target_port_id);
        value_push_option(&mut entries, "label", &self.label);
        DslValue::Object(entries)
    }
}

impl ToValue for NodeGraphViewport {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "x", &self.x);
        value_push(&mut entries, "y", &self.y);
        value_push(&mut entries, "zoom", &self.zoom);
        DslValue::Object(entries)
    }
}

impl FromValue for NodeGraphViewport {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            x: value_decode_default(&entries, "x", Default::default)?,
            y: value_decode_default(&entries, "y", Default::default)?,
            zoom: value_decode_default(&entries, "zoom", node_graph_default_zoom)?,
        })
    }
}

impl ToValue for NodeGraphFindItem {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "id", &self.id);
        value_push(&mut entries, "label", &self.label);
        value_push(&mut entries, "category", &self.category);
        DslValue::Object(entries)
    }
}

impl ToValue for NodeGraphHover {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push_option(&mut entries, "nodeId", &self.node_id);
        value_push_option(&mut entries, "portId", &self.port_id);
        DslValue::Object(entries)
    }
}

impl FromValue for NodeGraphHover {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self { node_id: value_decode_option(&entries, "nodeId")?, port_id: value_decode_option(&entries, "portId")? })
    }
}

impl ToValue for NodeGraphOperatorVariadicRecord {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "slotKey", &self.slot_key);
        value_push(&mut entries, "min", &self.min);
        value_push_option(&mut entries, "max", &self.max);
        DslValue::Object(entries)
    }
}

impl ToValue for NodeGraphOperatorChannelRecord {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "code", &self.code);
        value_push(&mut entries, "abbreviation", &self.abbreviation);
        value_push(&mut entries, "name", &self.name);
        value_push(&mut entries, "fullName", &self.full_name);
        value_push_if_nonempty(&mut entries, "operators", &self.operators);
        value_push_option(&mut entries, "defaultJson", &self.default_json);
        value_push_option(&mut entries, "label", &self.label);
        value_push(&mut entries, "cardinality", &self.cardinality);
        DslValue::Object(entries)
    }
}

impl ToValue for NodeGraphOperatorRecord {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "id", &self.id);
        value_push(&mut entries, "extension", &self.extension);
        value_push(&mut entries, "name", &self.name);
        value_push(&mut entries, "abbreviation", &self.abbreviation);
        value_push(&mut entries, "icon", &self.icon);
        value_push(&mut entries, "summary", &self.summary);
        value_push(&mut entries, "inputs", &self.inputs);
        value_push(&mut entries, "outputs", &self.outputs);
        value_push_option(&mut entries, "variadicInput", &self.variadic_input);
        value_push_option(&mut entries, "variadicOutput", &self.variadic_output);
        value_push_if_nonempty(&mut entries, "group", &self.group);
        DslValue::Object(entries)
    }
}
//#endregion 🔖️NodeGraphRecordsToValue
//#endregion 🔖️NodeGraphRecords

//#region 🔖️NodeGraphScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphScene {
    #[serde(default)]
    pub nodes: Vec<NodeGraphNodeRecord>,
    #[serde(default)]
    pub edges: Vec<NodeGraphEdgeRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewport: Option<NodeGraphViewport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operators: Vec<NodeGraphOperatorRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub find_items: Vec<NodeGraphFindItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selection: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hover: Option<NodeGraphHover>,
    /// ✨️ Ids — nodes, edges, or `"{nodeId}@{portId}"` ports — the plugin wants highlighted, e.g.
    /// because they are transitively hovered from another window.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlighted: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_off_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lod_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogue_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computing_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixture_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_peers_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_json: Option<String>,
}

impl SceneDoc for NodeGraphScene {
    const SCHEMA: &'static str = "node-graph@1";
}

impl NodeGraphScene {
    /** @emoji 🕸️ Builds a node-graph scene with optional extensions unset. */
    pub fn base(nodes: Vec<NodeGraphNodeRecord>, edges: Vec<NodeGraphEdgeRecord>, viewport: NodeGraphViewport) -> Self {
        Self {
            nodes,
            edges,
            viewport: Some(viewport),
            editable: None,
            operators: Vec::new(),
            find_items: Vec::new(),
            selection: Vec::new(),
            hover: None,
            highlighted: Vec::new(),
            preview_off_json: None,
            lod_json: None,
            catalogue_json: None,
            controls_json: None,
            clusters_json: None,
            computing_json: None,
            status_json: None,
            capabilities_json: None,
            fixture_json: None,
            presence_peers_json: None,
            eval_json: None,
        }
    }
}

impl ToValue for NodeGraphScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "nodes", &self.nodes);
        value_push(&mut entries, "edges", &self.edges);
        value_push_option(&mut entries, "viewport", &self.viewport);
        value_push_option(&mut entries, "editable", &self.editable);
        value_push_if_nonempty(&mut entries, "operators", &self.operators);
        value_push_if_nonempty(&mut entries, "findItems", &self.find_items);
        value_push_if_nonempty(&mut entries, "selection", &self.selection);
        value_push_option(&mut entries, "hover", &self.hover);
        value_push_if_nonempty(&mut entries, "highlighted", &self.highlighted);
        value_push_option(&mut entries, "previewOffJson", &self.preview_off_json);
        value_push_option(&mut entries, "lodJson", &self.lod_json);
        value_push_option(&mut entries, "catalogueJson", &self.catalogue_json);
        value_push_option(&mut entries, "controlsJson", &self.controls_json);
        value_push_option(&mut entries, "clustersJson", &self.clusters_json);
        value_push_option(&mut entries, "computingJson", &self.computing_json);
        value_push_option(&mut entries, "statusJson", &self.status_json);
        value_push_option(&mut entries, "capabilitiesJson", &self.capabilities_json);
        value_push_option(&mut entries, "fixtureJson", &self.fixture_json);
        value_push_option(&mut entries, "presencePeersJson", &self.presence_peers_json);
        value_push_option(&mut entries, "evalJson", &self.eval_json);
        DslValue::Object(entries)
    }
}

impl FromValue for NodeGraphScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            nodes: value_decode_default(&entries, "nodes", Vec::new)?,
            edges: value_decode_default(&entries, "edges", Vec::new)?,
            viewport: value_decode_option(&entries, "viewport")?,
            editable: value_decode_option(&entries, "editable")?,
            operators: value_decode_default(&entries, "operators", Vec::new)?,
            find_items: value_decode_default(&entries, "findItems", Vec::new)?,
            selection: value_decode_default(&entries, "selection", Vec::new)?,
            hover: value_decode_option(&entries, "hover")?,
            highlighted: value_decode_default(&entries, "highlighted", Vec::new)?,
            preview_off_json: value_decode_option(&entries, "previewOffJson")?,
            lod_json: value_decode_option(&entries, "lodJson")?,
            catalogue_json: value_decode_option(&entries, "catalogueJson")?,
            controls_json: value_decode_option(&entries, "controlsJson")?,
            clusters_json: value_decode_option(&entries, "clustersJson")?,
            computing_json: value_decode_option(&entries, "computingJson")?,
            status_json: value_decode_option(&entries, "statusJson")?,
            capabilities_json: value_decode_option(&entries, "capabilitiesJson")?,
            fixture_json: value_decode_option(&entries, "fixtureJson")?,
            presence_peers_json: value_decode_option(&entries, "presencePeersJson")?,
            eval_json: value_decode_option(&entries, "evalJson")?,
        })
    }
}
//#endregion 🔖️NodeGraphScene

//#region 🔖️TextEditorScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEditorScene {
    pub buffer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completions_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlays_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrences_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholders_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_carets_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_spans_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newline_gates_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_json: Option<String>,
}

scene_pack_wire!(TextEditorScenePack, TextEditorScene {
    buffer: String,
    language: Option<String>,
    selection_json: Option<String>,
    tokens_json: Option<String>,
    diagnostics_json: Option<String>,
    completions_json: Option<String>,
    overlays_json: Option<String>,
    occurrences_json: Option<String>,
    placeholders_json: Option<String>,
    extra_carets_json: Option<String>,
    selectable_spans_json: Option<String>,
    settings_json: Option<String>,
    camera_json: Option<String>,
    hover_json: Option<String>,
    newline_gates_json: Option<String>,
    rename_json: Option<String>,
});

impl SceneDoc for TextEditorScene {
    const SCHEMA: &'static str = "text-editor@1";

    fn encode_pack(&self) -> Result<Vec<u8>, crate::pack::PackError> {
        crate::pack::to_bytes(&TextEditorScenePack::from(self))
    }

    fn decode_pack(bytes: &[u8]) -> Result<Self, crate::pack::PackError> {
        crate::pack::from_bytes::<TextEditorScenePack>(bytes).map(Into::into)
    }
}

impl TextEditorScene {
    /** @emoji ✍️ Builds a text-editor scene with optional extensions unset. */
    pub fn base(buffer: String, language: Option<String>, selection_json: Option<String>) -> Self {
        Self {
            buffer,
            language,
            selection_json,
            tokens_json: None,
            diagnostics_json: None,
            completions_json: None,
            overlays_json: None,
            occurrences_json: None,
            placeholders_json: None,
            extra_carets_json: None,
            selectable_spans_json: None,
            settings_json: None,
            camera_json: None,
            hover_json: None,
            newline_gates_json: None,
            rename_json: None,
        }
    }
}

impl ToValue for TextEditorScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "buffer", &self.buffer);
        value_push_option(&mut entries, "language", &self.language);
        value_push_option(&mut entries, "selectionJson", &self.selection_json);
        value_push_option(&mut entries, "tokensJson", &self.tokens_json);
        value_push_option(&mut entries, "diagnosticsJson", &self.diagnostics_json);
        value_push_option(&mut entries, "completionsJson", &self.completions_json);
        value_push_option(&mut entries, "overlaysJson", &self.overlays_json);
        value_push_option(&mut entries, "occurrencesJson", &self.occurrences_json);
        value_push_option(&mut entries, "placeholdersJson", &self.placeholders_json);
        value_push_option(&mut entries, "extraCaretsJson", &self.extra_carets_json);
        value_push_option(&mut entries, "selectableSpansJson", &self.selectable_spans_json);
        value_push_option(&mut entries, "settingsJson", &self.settings_json);
        value_push_option(&mut entries, "cameraJson", &self.camera_json);
        value_push_option(&mut entries, "hoverJson", &self.hover_json);
        value_push_option(&mut entries, "newlineGatesJson", &self.newline_gates_json);
        value_push_option(&mut entries, "renameJson", &self.rename_json);
        DslValue::Object(entries)
    }
}

impl FromValue for TextEditorScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            buffer: value_decode(&entries, "buffer")?,
            language: value_decode_option(&entries, "language")?,
            selection_json: value_decode_option(&entries, "selectionJson")?,
            tokens_json: value_decode_option(&entries, "tokensJson")?,
            diagnostics_json: value_decode_option(&entries, "diagnosticsJson")?,
            completions_json: value_decode_option(&entries, "completionsJson")?,
            overlays_json: value_decode_option(&entries, "overlaysJson")?,
            occurrences_json: value_decode_option(&entries, "occurrencesJson")?,
            placeholders_json: value_decode_option(&entries, "placeholdersJson")?,
            extra_carets_json: value_decode_option(&entries, "extraCaretsJson")?,
            selectable_spans_json: value_decode_option(&entries, "selectableSpansJson")?,
            settings_json: value_decode_option(&entries, "settingsJson")?,
            camera_json: value_decode_option(&entries, "cameraJson")?,
            hover_json: value_decode_option(&entries, "hoverJson")?,
            newline_gates_json: value_decode_option(&entries, "newlineGatesJson")?,
            rename_json: value_decode_option(&entries, "renameJson")?,
        })
    }
}
//#endregion 🔖️TextEditorScene

//#region 🔖️TableScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableScene {
    pub columns_json: String,
    pub rows_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_drag_mime: Option<String>,
    /// 🪟️ Opaque pack-encoded `ActionDescriptor` JSON — was a live `ui_wgpu::wgpu::ActionDescriptor`
    /// before this move; see this module's own header for why that type couldn't come along.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop_action_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
}

impl SceneDoc for TableScene {
    const SCHEMA: &'static str = "table@1";
}

impl TableScene {
    /** @emoji 📋️ Builds a table scene with optional extensions (selection/drag/sort/domain) unset. */
    pub fn base(columns_json: impl Into<String>, rows_json: impl Into<String>) -> Self {
        Self { columns_json: columns_json.into(), rows_json: rows_json.into(), selection_json: None, row_drag_mime: None, drop_action_json: None, sort_json: None, domain_id: None }
    }
}

impl ToValue for TableScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "columnsJson", &self.columns_json);
        value_push(&mut entries, "rowsJson", &self.rows_json);
        value_push_option(&mut entries, "selectionJson", &self.selection_json);
        value_push_option(&mut entries, "rowDragMime", &self.row_drag_mime);
        value_push_option(&mut entries, "dropActionJson", &self.drop_action_json);
        value_push_option(&mut entries, "sortJson", &self.sort_json);
        value_push_option(&mut entries, "domainId", &self.domain_id);
        DslValue::Object(entries)
    }
}

impl FromValue for TableScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            columns_json: value_decode(&entries, "columnsJson")?,
            rows_json: value_decode(&entries, "rowsJson")?,
            selection_json: value_decode_option(&entries, "selectionJson")?,
            row_drag_mime: value_decode_option(&entries, "rowDragMime")?,
            drop_action_json: value_decode_option(&entries, "dropActionJson")?,
            sort_json: value_decode_option(&entries, "sortJson")?,
            domain_id: value_decode_option(&entries, "domainId")?,
        })
    }
}
//#endregion 🔖️TableScene

//#region 🔖️Paint2dScene
/** @emoji 🖼️ Paint-2d scene: WASM `RasterSession` sync channels for the composite/navigator windows. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paint2dScene {
    pub document_sync_json: String,
    pub assets_json: String,
    pub camera_json: String,
    pub selection_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_id: Option<String>,
    pub active_utility: String,
    pub brush_size: f64,
    pub brush_opacity: f64,
    pub view_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_viewport_json: Option<String>,
}

impl SceneDoc for Paint2dScene {
    const SCHEMA: &'static str = "paint-2d@1";
}

impl ToValue for Paint2dScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "documentSyncJson", &self.document_sync_json);
        value_push(&mut entries, "assetsJson", &self.assets_json);
        value_push(&mut entries, "cameraJson", &self.camera_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        value_push_option(&mut entries, "hoveredId", &self.hovered_id);
        value_push(&mut entries, "activeUtility", &self.active_utility);
        value_push(&mut entries, "brushSize", &self.brush_size);
        value_push(&mut entries, "brushOpacity", &self.brush_opacity);
        value_push(&mut entries, "viewMode", &self.view_mode);
        value_push_option(&mut entries, "compositeViewportJson", &self.composite_viewport_json);
        DslValue::Object(entries)
    }
}

impl FromValue for Paint2dScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            document_sync_json: value_decode(&entries, "documentSyncJson")?,
            assets_json: value_decode(&entries, "assetsJson")?,
            camera_json: value_decode(&entries, "cameraJson")?,
            selection_json: value_decode(&entries, "selectionJson")?,
            hovered_id: value_decode_option(&entries, "hoveredId")?,
            active_utility: value_decode(&entries, "activeUtility")?,
            brush_size: value_decode(&entries, "brushSize")?,
            brush_opacity: value_decode(&entries, "brushOpacity")?,
            view_mode: value_decode(&entries, "viewMode")?,
            composite_viewport_json: value_decode_option(&entries, "compositeViewportJson")?,
        })
    }
}
//#endregion 🔖️Paint2dScene

//#region 🔖️IconRenderScene
/** @emoji 🖼️ Icon-render scene: client-side render request for a shot preview. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconRenderScene {
    pub request_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_json: Option<String>,
}

impl SceneDoc for IconRenderScene {
    const SCHEMA: &'static str = "icon-render@1";
}

impl ToValue for IconRenderScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "requestJson", &self.request_json);
        value_push_option(&mut entries, "footer", &self.footer);
        value_push_option(&mut entries, "frameJson", &self.frame_json);
        DslValue::Object(entries)
    }
}

impl FromValue for IconRenderScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            request_json: value_decode(&entries, "requestJson")?,
            footer: value_decode_option(&entries, "footer")?,
            frame_json: value_decode_option(&entries, "frameJson")?,
        })
    }
}
//#endregion 🔖️IconRenderScene

//#region 🔖️VirtualFileSystemScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualFileSystemScene {
    pub schema_json: String,
    pub rows_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_row_ids_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_row_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_drop_enabled: Option<bool>,
}

impl SceneDoc for VirtualFileSystemScene {
    // 🧭️ `ui_contract`'s `SurfaceKind::VirtualFileSystem` wire tag was renamed `"virtualFileSystem"`
    // (camelCase) → `"virtual-file-system"` (kebab-case, matching every sibling) by packet
    // `ui-w4-core` — see that crate's own `🦀️surface.rs` header. This schema string tracks the rename.
    const SCHEMA: &'static str = "virtual-file-system@1";
}

impl ToValue for VirtualFileSystemScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "schemaJson", &self.schema_json);
        value_push(&mut entries, "rowsJson", &self.rows_json);
        value_push_option(&mut entries, "selectedRowIdsJson", &self.selected_row_ids_json);
        value_push_option(&mut entries, "hoveredRowId", &self.hovered_row_id);
        value_push_option(&mut entries, "emptyMessage", &self.empty_message);
        value_push_option(&mut entries, "dragDropEnabled", &self.drag_drop_enabled);
        DslValue::Object(entries)
    }
}

impl FromValue for VirtualFileSystemScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            schema_json: value_decode(&entries, "schemaJson")?,
            rows_json: value_decode(&entries, "rowsJson")?,
            selected_row_ids_json: value_decode_option(&entries, "selectedRowIdsJson")?,
            hovered_row_id: value_decode_option(&entries, "hoveredRowId")?,
            empty_message: value_decode_option(&entries, "emptyMessage")?,
            drag_drop_enabled: value_decode_option(&entries, "dragDropEnabled")?,
        })
    }
}
//#endregion 🔖️VirtualFileSystemScene

//#region 🔖️TiledMapScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiledMapScene {
    pub map_fixture_json: String,
    pub camera_json: String,
    #[serde(default = "tiled_map_default_render_mode")]
    pub render_mode: String,
    #[serde(default = "tiled_map_default_vector_style")]
    pub vector_style: String,
    #[serde(default = "tiled_map_default_lod_mode")]
    pub lod_mode: String,
    #[serde(default = "tiled_map_default_tile_url_template")]
    pub tile_url_template: String,
    #[serde(default = "tiled_map_default_vector_tile_url_template")]
    pub vector_tile_url_template: String,
    #[serde(default = "tiled_map_default_layer_visibility_json")]
    pub layer_visibility_json: String,
    #[serde(default = "tiled_map_default_layer_stroke_scale_json")]
    pub layer_stroke_scale_json: String,
    #[serde(default = "tiled_map_default_selection_json")]
    pub selection_json: String,
    #[serde(default = "tiled_map_default_hover_json")]
    pub hover_json: String,
    #[serde(default = "tiled_map_default_selection_method")]
    pub selection_method: String,
    #[serde(default = "tiled_map_default_selection_mode")]
    pub selection_mode: String,
}

impl SceneDoc for TiledMapScene {
    const SCHEMA: &'static str = "tiled-map@1";
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_render_mode() -> String {
    "combined".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_vector_style() -> String {
    "colored".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_lod_mode() -> String {
    "automatic".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_tile_url_template() -> String {
    "/osm/{z}/{x}/{y}.png".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_vector_tile_url_template() -> String {
    "/vt/{z}/{x}/{y}.pbf".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_layer_visibility_json() -> String {
    "{}".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_layer_stroke_scale_json() -> String {
    "{}".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_selection_json() -> String {
    "{}".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_hover_json() -> String {
    "null".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_selection_method() -> String {
    "rectangle".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_selection_mode() -> String {
    "default".into()
}

impl TiledMapScene {
    /** @emoji 🗺️ Builds a tiled map scene with optional extensions unset. */
    pub fn base(map_fixture_json: String, camera_json: String) -> Self {
        Self {
            map_fixture_json,
            camera_json,
            render_mode: tiled_map_default_render_mode(),
            vector_style: tiled_map_default_vector_style(),
            lod_mode: tiled_map_default_lod_mode(),
            tile_url_template: tiled_map_default_tile_url_template(),
            vector_tile_url_template: tiled_map_default_vector_tile_url_template(),
            layer_visibility_json: tiled_map_default_layer_visibility_json(),
            layer_stroke_scale_json: tiled_map_default_layer_stroke_scale_json(),
            selection_json: tiled_map_default_selection_json(),
            hover_json: tiled_map_default_hover_json(),
            selection_method: tiled_map_default_selection_method(),
            selection_mode: tiled_map_default_selection_mode(),
        }
    }
}

impl ToValue for TiledMapScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "mapFixtureJson", &self.map_fixture_json);
        value_push(&mut entries, "cameraJson", &self.camera_json);
        value_push(&mut entries, "renderMode", &self.render_mode);
        value_push(&mut entries, "vectorStyle", &self.vector_style);
        value_push(&mut entries, "lodMode", &self.lod_mode);
        value_push(&mut entries, "tileUrlTemplate", &self.tile_url_template);
        value_push(&mut entries, "vectorTileUrlTemplate", &self.vector_tile_url_template);
        value_push(&mut entries, "layerVisibilityJson", &self.layer_visibility_json);
        value_push(&mut entries, "layerStrokeScaleJson", &self.layer_stroke_scale_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        value_push(&mut entries, "hoverJson", &self.hover_json);
        value_push(&mut entries, "selectionMethod", &self.selection_method);
        value_push(&mut entries, "selectionMode", &self.selection_mode);
        DslValue::Object(entries)
    }
}

impl FromValue for TiledMapScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            map_fixture_json: value_decode(&entries, "mapFixtureJson")?,
            camera_json: value_decode(&entries, "cameraJson")?,
            render_mode: value_decode_default(&entries, "renderMode", tiled_map_default_render_mode)?,
            vector_style: value_decode_default(&entries, "vectorStyle", tiled_map_default_vector_style)?,
            lod_mode: value_decode_default(&entries, "lodMode", tiled_map_default_lod_mode)?,
            tile_url_template: value_decode_default(&entries, "tileUrlTemplate", tiled_map_default_tile_url_template)?,
            vector_tile_url_template: value_decode_default(&entries, "vectorTileUrlTemplate", tiled_map_default_vector_tile_url_template)?,
            layer_visibility_json: value_decode_default(&entries, "layerVisibilityJson", tiled_map_default_layer_visibility_json)?,
            layer_stroke_scale_json: value_decode_default(&entries, "layerStrokeScaleJson", tiled_map_default_layer_stroke_scale_json)?,
            selection_json: value_decode_default(&entries, "selectionJson", tiled_map_default_selection_json)?,
            hover_json: value_decode_default(&entries, "hoverJson", tiled_map_default_hover_json)?,
            selection_method: value_decode_default(&entries, "selectionMethod", tiled_map_default_selection_method)?,
            selection_mode: value_decode_default(&entries, "selectionMode", tiled_map_default_selection_mode)?,
        })
    }
}
//#endregion 🔖️TiledMapScene

//#region 🔖️Board2dScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board2dScene {
    pub fixture_json: String,
    pub camera_json: String,
    #[serde(default = "board2d_default_glyph_catalogs_json")]
    pub glyph_catalogs_json: String,
    #[serde(default = "board2d_default_selection_json")]
    pub selection_json: String,
    #[serde(default)]
    pub interactive: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hovered_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_utility: Option<String>,
    #[serde(default = "board2d_default_selection_method")]
    pub selection_method: String,
    #[serde(default)]
    pub grid_snap_enabled: bool,
    #[serde(default = "board2d_default_grid_factor")]
    pub grid_factor: f64,
    #[serde(default)]
    pub suggestion_offset: f64,
    #[serde(default = "board2d_default_brush_weights_json")]
    pub brush_weights_json: String,
    #[serde(default = "board2d_default_placement_compatibility_json")]
    pub placement_compatibility_json: String,
    #[serde(default = "board2d_default_lod_mode")]
    pub lod_mode: String,
}

scene_pack_wire!(Board2dScenePack, Board2dScene {
    fixture_json: String,
    camera_json: String,
    glyph_catalogs_json: String,
    selection_json: String,
    interactive: bool,
    hovered_id: Option<String>,
    active_utility: Option<String>,
    selection_method: String,
    grid_snap_enabled: bool,
    grid_factor: f64,
    suggestion_offset: f64,
    brush_weights_json: String,
    placement_compatibility_json: String,
    lod_mode: String,
});

impl SceneDoc for Board2dScene {
    const SCHEMA: &'static str = "board-2d@1";

    fn encode_pack(&self) -> Result<Vec<u8>, crate::pack::PackError> {
        crate::pack::to_bytes(&Board2dScenePack::from(self))
    }

    fn decode_pack(bytes: &[u8]) -> Result<Self, crate::pack::PackError> {
        crate::pack::from_bytes::<Board2dScenePack>(bytes).map(Into::into)
    }
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_glyph_catalogs_json() -> String {
    "{}".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_selection_json() -> String {
    "[]".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_selection_method() -> String {
    "rectangle".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_grid_factor() -> f64 {
    1.0
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_brush_weights_json() -> String {
    "{}".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_placement_compatibility_json() -> String {
    "[]".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_lod_mode() -> String {
    "automatic".into()
}

impl Board2dScene {
    /** @emoji 🧩️ Builds a 2D board scene with optional extensions unset. */
    pub fn base(fixture_json: String, camera_json: String, interactive: bool) -> Self {
        Self {
            fixture_json,
            camera_json,
            glyph_catalogs_json: board2d_default_glyph_catalogs_json(),
            selection_json: board2d_default_selection_json(),
            interactive,
            hovered_id: None,
            active_utility: None,
            selection_method: board2d_default_selection_method(),
            grid_snap_enabled: false,
            grid_factor: board2d_default_grid_factor(),
            suggestion_offset: 0.0,
            brush_weights_json: board2d_default_brush_weights_json(),
            placement_compatibility_json: board2d_default_placement_compatibility_json(),
            lod_mode: board2d_default_lod_mode(),
        }
    }
}

impl ToValue for Board2dScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "fixtureJson", &self.fixture_json);
        value_push(&mut entries, "cameraJson", &self.camera_json);
        value_push(&mut entries, "glyphCatalogsJson", &self.glyph_catalogs_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        value_push(&mut entries, "interactive", &self.interactive);
        value_push_option(&mut entries, "hoveredId", &self.hovered_id);
        value_push_option(&mut entries, "activeUtility", &self.active_utility);
        value_push(&mut entries, "selectionMethod", &self.selection_method);
        value_push(&mut entries, "gridSnapEnabled", &self.grid_snap_enabled);
        value_push(&mut entries, "gridFactor", &self.grid_factor);
        value_push(&mut entries, "suggestionOffset", &self.suggestion_offset);
        value_push(&mut entries, "brushWeightsJson", &self.brush_weights_json);
        value_push(&mut entries, "placementCompatibilityJson", &self.placement_compatibility_json);
        value_push(&mut entries, "lodMode", &self.lod_mode);
        DslValue::Object(entries)
    }
}

impl FromValue for Board2dScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            fixture_json: value_decode(&entries, "fixtureJson")?,
            camera_json: value_decode(&entries, "cameraJson")?,
            glyph_catalogs_json: value_decode_default(&entries, "glyphCatalogsJson", board2d_default_glyph_catalogs_json)?,
            selection_json: value_decode_default(&entries, "selectionJson", board2d_default_selection_json)?,
            interactive: value_decode_default(&entries, "interactive", Default::default)?,
            hovered_id: value_decode_option(&entries, "hoveredId")?,
            active_utility: value_decode_option(&entries, "activeUtility")?,
            selection_method: value_decode_default(&entries, "selectionMethod", board2d_default_selection_method)?,
            grid_snap_enabled: value_decode_default(&entries, "gridSnapEnabled", Default::default)?,
            grid_factor: value_decode_default(&entries, "gridFactor", board2d_default_grid_factor)?,
            suggestion_offset: value_decode_default(&entries, "suggestionOffset", Default::default)?,
            brush_weights_json: value_decode_default(&entries, "brushWeightsJson", board2d_default_brush_weights_json)?,
            placement_compatibility_json: value_decode_default(&entries, "placementCompatibilityJson", board2d_default_placement_compatibility_json)?,
            lod_mode: value_decode_default(&entries, "lodMode", board2d_default_lod_mode)?,
        })
    }
}
//#endregion 🔖️Board2dScene

//#region 🔖️InkCanvasScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkCanvasScene {
    pub document_json: String,
    #[serde(default = "ink_canvas_default_selection_json")]
    pub selection_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hovered_id: Option<String>,
    pub active_utility: String,
    pub view_mode: String,
    #[serde(default)]
    pub interactive: bool,
}

impl SceneDoc for InkCanvasScene {
    const SCHEMA: &'static str = "ink-canvas@1";
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn ink_canvas_default_selection_json() -> String {
    "[]".into()
}

impl InkCanvasScene {
    /** @emoji 🖊️ Builds an ink canvas scene with the default empty selection. */
    pub fn base(document_json: String, active_utility: String, view_mode: String, interactive: bool) -> Self {
        Self { document_json, selection_json: ink_canvas_default_selection_json(), hovered_id: None, active_utility, view_mode, interactive }
    }
}

impl ToValue for InkCanvasScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "documentJson", &self.document_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        value_push_option(&mut entries, "hoveredId", &self.hovered_id);
        value_push(&mut entries, "activeUtility", &self.active_utility);
        value_push(&mut entries, "viewMode", &self.view_mode);
        value_push(&mut entries, "interactive", &self.interactive);
        DslValue::Object(entries)
    }
}

impl FromValue for InkCanvasScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            document_json: value_decode(&entries, "documentJson")?,
            selection_json: value_decode_default(&entries, "selectionJson", ink_canvas_default_selection_json)?,
            hovered_id: value_decode_option(&entries, "hoveredId")?,
            active_utility: value_decode(&entries, "activeUtility")?,
            view_mode: value_decode(&entries, "viewMode")?,
            interactive: value_decode_default(&entries, "interactive", Default::default)?,
        })
    }
}
//#endregion 🔖️InkCanvasScene

//#region 🔖️GraphTimelineScene
/** @emoji 🗄️ A checkpoint ancestor-graph history view. `columns_json` is a `HistoryColumn[]` array,
 * newest checkpoint first. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphTimelineScene {
    pub columns_json: String,
}

impl SceneDoc for GraphTimelineScene {
    const SCHEMA: &'static str = "graph-timeline@1";
}

impl ToValue for GraphTimelineScene {
    fn to_value(&self) -> DslValue {
        DslValue::object([("columnsJson".to_string(), self.columns_json.to_value())])
    }
}

impl FromValue for GraphTimelineScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self { columns_json: value_decode(&entries, "columnsJson")? })
    }
}
//#endregion 🔖️GraphTimelineScene

//#region 🔖️DiffViewScene
/** @emoji 🆚️ A before/after text comparison. `mode` picks the renderer's layout (`"unified"` inline
 * hunks or `"split"` side-by-side panes); `language` is an optional syntax-highlighting hint. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffViewScene {
    pub before: String,
    pub after: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
}

impl SceneDoc for DiffViewScene {
    const SCHEMA: &'static str = "diff-view@1";
}

impl ToValue for DiffViewScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "before", &self.before);
        value_push(&mut entries, "after", &self.after);
        value_push_option(&mut entries, "language", &self.language);
        value_push_option(&mut entries, "mode", &self.mode);
        value_push_option(&mut entries, "domainId", &self.domain_id);
        DslValue::Object(entries)
    }
}

impl FromValue for DiffViewScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            before: value_decode(&entries, "before")?,
            after: value_decode(&entries, "after")?,
            language: value_decode_option(&entries, "language")?,
            mode: value_decode_option(&entries, "mode")?,
            domain_id: value_decode_option(&entries, "domainId")?,
        })
    }
}
//#endregion 🔖️DiffViewScene

//#region 🔖️EventFeedScene
/** @emoji 📰️ A chronological feed of host-authored events. `entries_json` is a
 * `{id, timestampMs, iconId, title, detail?, tone?}[]` array; `activate_action` (if set) is the
 * action name fired with the clicked entry's `id` when an entry is activated. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventFeedScene {
    pub entries_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate_action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
}

impl SceneDoc for EventFeedScene {
    const SCHEMA: &'static str = "event-feed@1";
}

impl ToValue for EventFeedScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "entriesJson", &self.entries_json);
        value_push_option(&mut entries, "follow", &self.follow);
        value_push_option(&mut entries, "activateAction", &self.activate_action);
        value_push_option(&mut entries, "domainId", &self.domain_id);
        DslValue::Object(entries)
    }
}

impl FromValue for EventFeedScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            entries_json: value_decode(&entries, "entriesJson")?,
            follow: value_decode_option(&entries, "follow")?,
            activate_action: value_decode_option(&entries, "activateAction")?,
            domain_id: value_decode_option(&entries, "domainId")?,
        })
    }
}
//#endregion 🔖️EventFeedScene

//#region 🔖️BlockListScene
/** @emoji 🧩️ A strict, ordered list of steps/blocks for the Blockly-like list editor. `steps_json`
 * is a `PlaybookStep[]` array, `palette_json` is a `BlockPaletteEntry[]` array of the block kinds
 * available to insert (kept as opaque JSON here too — `BlockPaletteEntry.icon_id` was an `IconName`,
 * the same `ui_wgpu`-only-type problem as `TableScene::drop_action_json`, see this module's header). */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockListScene {
    pub steps_json: String,
    pub palette_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dragging_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
}

impl SceneDoc for BlockListScene {
    const SCHEMA: &'static str = "block-list@1";
}

impl ToValue for BlockListScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "stepsJson", &self.steps_json);
        value_push(&mut entries, "paletteJson", &self.palette_json);
        value_push_option(&mut entries, "selectedId", &self.selected_id);
        value_push_option(&mut entries, "draggingId", &self.dragging_id);
        value_push_option(&mut entries, "domainId", &self.domain_id);
        DslValue::Object(entries)
    }
}

impl FromValue for BlockListScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            steps_json: value_decode(&entries, "stepsJson")?,
            palette_json: value_decode(&entries, "paletteJson")?,
            selected_id: value_decode_option(&entries, "selectedId")?,
            dragging_id: value_decode_option(&entries, "draggingId")?,
            domain_id: value_decode_option(&entries, "domainId")?,
        })
    }
}
//#endregion 🔖️BlockListScene

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn world3d_scene_domain_id_round_trips_as_camel_case_and_omits_when_none() {
        let mut scene = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
        scene.domain_id = Some("cad".into());
        scene.domain_granularity_id = Some("handle".into());
        let value = serde_json::to_value(&scene).expect("serialize");
        assert_eq!(value.get("domainId").and_then(Value::as_str), Some("cad"));
        assert_eq!(value.get("domainGranularityId").and_then(Value::as_str), Some("handle"));
        let back: World3dScene = serde_json::from_value(value).expect("deserialize");
        assert_eq!(back, scene);

        let bare = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
        let bare_value = serde_json::to_value(&bare).expect("serialize");
        assert!(bare_value.get("domainId").is_none());
        assert!(bare_value.get("domainGranularityId").is_none());
    }

    #[test]
    fn node_graph_hover_port_id_round_trips_as_camel_case_and_omits_when_none() {
        let hover = NodeGraphHover { node_id: Some("combine".into()), port_id: Some("b".into()) };
        let value = serde_json::to_value(&hover).expect("serialize");
        assert_eq!(value.get("nodeId").and_then(Value::as_str), Some("combine"));
        assert_eq!(value.get("portId").and_then(Value::as_str), Some("b"));
        let back: NodeGraphHover = serde_json::from_value(value).expect("deserialize");
        assert_eq!(back, hover);

        let bare = NodeGraphHover { node_id: Some("combine".into()), port_id: None };
        let bare_value = serde_json::to_value(&bare).expect("serialize");
        assert!(bare_value.get("portId").is_none());
    }

    #[test]
    fn node_graph_scene_highlighted_round_trips_and_omits_when_empty() {
        let viewport = NodeGraphViewport { x: 0.0, y: 0.0, zoom: 1.0 };
        let mut scene = NodeGraphScene { highlighted: vec!["a".into(), "b@out".into()], ..NodeGraphScene::base(Vec::new(), Vec::new(), viewport.clone()) };
        let value = serde_json::to_value(&scene).expect("serialize");
        assert_eq!(value.get("highlighted").and_then(Value::as_array).map(Vec::len), Some(2));
        let back: NodeGraphScene = serde_json::from_value(value).expect("deserialize");
        assert_eq!(back, scene);

        scene.highlighted = Vec::new();
        let bare_value = serde_json::to_value(&scene).expect("serialize");
        assert!(bare_value.get("highlighted").is_none());
    }
}

/// 🌱️ `FromValue(ToValue(x)) == x` coverage for the hand-written `protocol::value::` codec added in
/// this pass — this crate can never take the `os-kernel` dependency `#[derive(ToValue, FromValue)]`
/// hardcodes (see `🔖️ValueCodecHelpers`'s docstring), so every one of these impls is hand-written and
/// needs its own round-trip proof, same as `serde`'s derive gets proven by the sibling `mod tests`
/// above. `UiNode` itself (the recursive declarative-tree enum these scene types unblock) lives in
/// the sibling `ui_wgpu` crate, owned by a different agent this session — not testable from here.
#[cfg(test)]
mod value_round_trip_tests {
    use super::*;

    #[test]
    fn canvas2d_scene_round_trips_with_and_without_snapshot() {
        let bare = Canvas2dScene { camera_x: 1.5, camera_y: -2.0, zoom: 1.0, layers_json: "[]".into(), snapshot: None };
        assert_eq!(Canvas2dScene::from_value(bare.to_value()), Ok(bare.clone()));
        assert!(matches!(bare.to_value(), DslValue::Object(entries) if !entries.iter().any(|(k, _)| k == "snapshot")));

        let leased = Canvas2dScene { snapshot: Some(crate::Canvas2dSnapshotLease { slot: 1, epoch: 2, revision: 3, generation: 4, page_count: 1, byte_count: 16 }), ..bare };
        assert_eq!(Canvas2dScene::from_value(leased.to_value()), Ok(leased));
    }

    #[test]
    fn world3d_scene_round_trips_dense_and_bare_and_keeps_integers_as_integers() {
        let bare = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
        assert_eq!(World3dScene::from_value(bare.to_value()), Ok(bare.clone()));

        let mut dense = bare.clone();
        dense.domain_id = Some("cad".into());
        dense.domain_granularity_id = Some("handle".into());
        dense.snapshot = Some(crate::World3dSnapshotLease { slot: 2, epoch: 3, revision: 4, generation: 5, page_count: 6, item_count: 7, byte_count: 8 });
        let encoded = dense.to_value();
        assert_eq!(World3dScene::from_value(encoded.clone()), Ok(dense));
        let DslValue::Object(entries) = &encoded else { panic!("expected an object") };
        let snapshot_entries = match entries.iter().find(|(k, _)| k == "snapshot").map(|(_, v)| v) {
            Some(DslValue::Object(entries)) => entries,
            other => panic!("expected the nested snapshot lease to encode as an object, found {other:?}"),
        };
        let slot = snapshot_entries.iter().find(|(k, _)| k == "slot").map(|(_, v)| v.clone());
        assert!(matches!(slot, Some(DslValue::Number(protocol::value::Number::UInt(2)))), "u8 field must stay an integer, found {slot:?}");
    }

    #[test]
    fn missing_required_field_reports_the_field_name() {
        let empty = DslValue::object([]);
        assert_eq!(TableScene::from_value(empty), Err(ValueError::new("missing field `columnsJson`")));
    }

    /// 🕸️ `NodeGraphScene` is the deepest nesting in this crate — it embeds `NodeGraphNodeRecord`
    /// (itself embedding `NodeGraphPortRecord`), `NodeGraphEdgeRecord`, `NodeGraphViewport`,
    /// `NodeGraphHover`, and `NodeGraphOperatorRecord` (itself embedding
    /// `NodeGraphOperatorChannelRecord`/`NodeGraphOperatorVariadicRecord`) — one round trip here
    /// exercises every nested `NodeGraph*Record` codec added in this pass at once.
    #[test]
    fn node_graph_scene_round_trips_through_every_nested_record_type() {
        let port = NodeGraphPortRecord { id: "a".into(), label: Some("A".into()), code: None, abbreviation: None, full_name: None, artifact_kind: None };
        let node = NodeGraphNodeRecord { id: "n1".into(), label: Some("Node".into()), x: 1.0, y: 2.0, width: 100.0, height: 50.0, inputs: vec![port], outputs: Vec::new(), instance_id: Some("i1".into()), plugin_id: None, app_id: None, icon: None };
        let edge = NodeGraphEdgeRecord { id: "e1".into(), source_node_id: "n1".into(), source_port_id: "a".into(), target_node_id: "n1".into(), target_port_id: "a".into(), label: None };
        let variadic = NodeGraphOperatorVariadicRecord { slot_key: "vs".into(), min: 1, max: Some(4) };
        let channel = NodeGraphOperatorChannelRecord { code: "c".into(), abbreviation: "C".into(), name: "Chan".into(), full_name: "Channel".into(), operators: vec!["op".into()], default_json: Some("null".into()), label: None, cardinality: "one".into() };
        let operator = NodeGraphOperatorRecord { id: "op1".into(), extension: "core".into(), name: "Op".into(), abbreviation: "O".into(), icon: "icon".into(), summary: "sums".into(), inputs: vec![channel.clone()], outputs: vec![channel], variadic_input: Some(variadic.clone()), variadic_output: Some(variadic), group: vec!["g".into()] };
        let mut scene = NodeGraphScene::base(vec![node], vec![edge], NodeGraphViewport { x: 1.0, y: 2.0, zoom: 1.5 });
        scene.hover = Some(NodeGraphHover { node_id: Some("n1".into()), port_id: Some("a".into()) });
        scene.operators = vec![operator];
        scene.find_items = vec![NodeGraphFindItem { id: "f1".into(), label: "Find".into(), category: "cat".into() }];
        scene.highlighted = vec!["n1".into()];

        assert_eq!(NodeGraphScene::from_value(scene.to_value()), Ok(scene));
    }

    #[test]
    fn node_graph_viewport_and_hover_round_trip_including_all_none() {
        let viewport = NodeGraphViewport { x: 0.0, y: 0.0, zoom: 1.0 };
        assert_eq!(NodeGraphViewport::from_value(viewport.to_value()), Ok(viewport));
        let hover = NodeGraphHover { node_id: None, port_id: None };
        assert_eq!(NodeGraphHover::from_value(hover.to_value()), Ok(hover.clone()));
        assert_eq!(hover.to_value(), DslValue::object([]));
    }

    #[test]
    fn table_scene_and_tiled_map_scene_round_trip() {
        let table = TableScene { columns_json: "[]".into(), rows_json: "[]".into(), selection_json: Some("{}".into()), row_drag_mime: None, drop_action_json: None, sort_json: None, domain_id: Some("d".into()) };
        assert_eq!(TableScene::from_value(table.to_value()), Ok(table));

        let tiled = TiledMapScene::base("{}".into(), "{}".into());
        assert_eq!(TiledMapScene::from_value(tiled.to_value()), Ok(tiled.clone()));
        // 🕳️ Every field but `mapFixtureJson`/`cameraJson` carries a `#[serde(default = ...)]`
        // fallback — omitting just the defaulted ones must reproduce `base`'s own defaults exactly,
        // matching `serde`'s behaviour for a missing key on a `#[serde(default = "fn")]` field.
        assert_eq!(TiledMapScene::from_value(DslValue::object([("mapFixtureJson".to_string(), DslValue::String("{}".into())), ("cameraJson".to_string(), DslValue::String("{}".into()))])), Ok(tiled));
    }

    #[test]
    fn board2d_scene_and_block_list_scene_round_trip() {
        let board = Board2dScene::base("{}".into(), "{}".into(), true);
        assert_eq!(Board2dScene::from_value(board.to_value()), Ok(board));

        let blocks = BlockListScene { steps_json: "[]".into(), palette_json: "[]".into(), selected_id: Some("s1".into()), dragging_id: None, domain_id: None };
        assert_eq!(BlockListScene::from_value(blocks.to_value()), Ok(blocks));
    }
}
