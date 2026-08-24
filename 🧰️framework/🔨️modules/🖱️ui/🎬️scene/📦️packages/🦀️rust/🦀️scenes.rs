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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
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

impl SceneDoc for TextEditorScene {
    const SCHEMA: &'static str = "text-editor@1";
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
//#endregion 🔖️BlockListScene
