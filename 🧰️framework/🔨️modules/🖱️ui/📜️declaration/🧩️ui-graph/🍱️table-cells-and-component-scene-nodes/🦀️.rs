//! 🍱️ Table cells and component scene nodes: the table cells, the block palette entries, the external slot node and the component scene node. See [`super`].

use super::*;

// #region ui
//#region 🔖️ComponentScenes
//#region 🔖️TableCells
/// 🧾️ A typed table cell value: plain text/number, or an interactive stepper/button group.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum TableCell {
    Text { value: String },
    Number { value: f64 },
    EditableText { value: String, action: ActionDescriptor },
    Stepper { value: f64, min: f64, max: f64, step: f64, action: ActionDescriptor },
    Buttons { buttons: Vec<UiTreeItemAction> },
}
//#endregion 🔖️TableCells

/* 🖼️ Paint-2d scene: WASM `RasterSession` sync channels for the composite/navigator windows, see framework/surface/paint/rs/lib.rs. */
// 🎬️ `Paint2dScene`, `IconRenderScene`, `VirtualFileSystemScene`, `TiledMapScene` (+ its default
// fns and `base()`), `Board2dScene` (+ its default fns and `base()`), `InkCanvasScene` (+ its
// default fn and `base()`), `GraphTimelineScene`, `DiffViewScene`, and `EventFeedScene` all moved
// to `semio-framework-ui-scene` (see the re-export above).

/** 🧩️ A palette entry for a block kind insertable into a [`BlockListScene`], contributed
     * either by the host app's own built-ins or by a `"playbook.blockKind"` topic contribution. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct BlockPaletteEntry {
    pub block_kind: String,
    pub label: String,
    pub icon_id: IconName,
}

// 🎬️ `BlockListScene` itself moved to `semio-framework-ui-scene` (see the re-export above);
// `BlockPaletteEntry` above is the host palette contribution projected into scene entries.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiExternalSlotNode {
    pub plugin_id: String,
    pub app_id: String,
    pub body_key: String,
    pub params_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub host_status: Option<String>,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiComponentSceneNode {
    /// 🪪️ Mounted component identity for native hosts; the document remains the action address.
    #[serde(skip)]
    #[value(skip)]
    pub host_id: String,
    pub surface_id: String,
    pub controller_id: String,
    pub component_kind: SurfaceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub pane_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub binding_id: Option<String>,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    /// 🖱️ Optional override of the implicit per-`component_kind` convention id (`"world3d"`,
    /// `"nodeGraph"`, `"tiledMap"`, ...) the host uses when resolving which surface answers a
    /// right-click — set only when a plugin needs a menu id other than the surface-kind default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub canvas_2d: Option<Canvas2dScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub world_3d: Option<World3dScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub node_graph: Option<NodeGraphScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub text_editor: Option<TextEditorScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub paint_2d: Option<Paint2dScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub virtual_file_system: Option<VirtualFileSystemScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub tiled_map: Option<TiledMapScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub board2d: Option<Board2dScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub icon_render: Option<IconRenderScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub ink_canvas: Option<InkCanvasScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub graph_timeline: Option<GraphTimelineScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub block_list: Option<BlockListScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub diff_view: Option<DiffViewScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub event_feed: Option<EventFeedScene>,
}
//#endregion 🔖️ComponentScenes
// #endregion ui
