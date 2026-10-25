//! 🎬️ Scene actions and node conversions: the text editor views, the node graph, text editor, board, ink canvas and tiled map action modules and the node and control conversions. See [`super`].

use super::*;

// #region ui
//#region 🔖️UiNode
// 🎬️ `NodeGraphScene::base` and `TextEditorScene::base` moved into `semio-framework-ui-scene`
// itself (inherent impls on a foreign type are an orphan-rule error, not just a style choice —
// see R9/E116). `json_view`/`code_input` stay here as free functions: both embed a `ui_wgpu`-only
// `ActionDescriptor` into `settings_json` via `serde_json`, so they legitimately belong on this
// side of the boundary the same way `TableCell`/`table_row_json` do.

/** 📖️ Builds a read-only JSON viewer text-editor scene: a pretty-printed JSON buffer,
     * `"json"` language, and `settingsJson` set to `{"readOnly":true}`. */
pub fn text_editor_json_view(json_pretty: String) -> TextEditorScene {
    let mut scene = TextEditorScene::base(json_pretty, Some("json".into()), None);
    scene.settings_json = Some(serde_json::json!({ "readOnly": true }).to_string());
    scene
}

/** ⌨️ Builds an editable code-input text-editor scene wired to a host settings-change
     * action: `settingsJson` carries `{"readOnly":false,"onEditSettings":<ActionDescriptor>}`, fired
     * by the renderer when the user edits editor settings (font size, tab width, ...) via its own
     * chrome. */
pub fn text_editor_code_input(buffer: String, language: &str, on_edit_settings: &ActionDescriptor) -> TextEditorScene {
    let mut scene = TextEditorScene::base(buffer, Some(language.into()), None);
    scene.settings_json = Some(serde_json::json!({ "readOnly": false, "onEditSettings": on_edit_settings }).to_string());
    scene
}

//#region 🔖️SceneActions
/** 🎮️ Renderer-to-plugin action names for node-graph surfaces. */
pub mod node_graph_actions {
    pub const SELECT: &str = "nodeGraphSelect";
    pub const HOVER: &str = "nodeGraphHover";
    pub const EDIT: &str = "nodeGraphEdit";
    pub const VIEWPORT: &str = "nodeGraphViewport";
    pub const SPOTLIGHT_COMMIT: &str = "spotlightCommit";
}

/** ✍️ Renderer-to-plugin action names for text-editor surfaces. */
pub mod text_editor_actions {
    pub const EDIT: &str = "textEdit";
    pub const SELECT: &str = "textSelect";
    pub const HOVER: &str = "textHover";
    pub const REQUEST_COMPLETIONS: &str = "requestCompletions";
    pub const COMMIT_RENAME: &str = "commitRename";
    pub const FORMAT_DOCUMENT: &str = "formatDocument";
}

/** 🧩️ Renderer-to-plugin action names for 2D board surfaces. */
pub mod board2d_actions {
    pub const APPLY_BOARD_EVENTS: &str = "applyBoardEvents";
}

/** 🖊️ Renderer-to-plugin action names for ink canvas surfaces. */
pub mod ink_canvas_actions {
    pub const APPLY_EVENTS: &str = "inkApplyEvents";
}

/** 🗺️ Renderer-to-plugin action names for tiled map surfaces. */
pub mod tiled_map_actions {
    pub const SET_CAMERA: &str = "setCamera";
    pub const SET_FEATURE_SELECTION: &str = "setFeatureSelection";
    pub const SET_HOVER: &str = "setHover";
    pub const SET_SELECTION_METHOD: &str = "setSelectionMethod";
    pub const SET_SELECTION_MODE: &str = "setSelectionMode";
    pub const CLEAR_SELECTION: &str = "clearSelection";
    pub const SELECT_ALL: &str = "selectAll";
    pub const DESELECT: &str = "deselect";
    pub const FOCUS_FEATURE: &str = "focusFeature";
    pub const OPEN_SOURCE: &str = "openSource";
    pub const SET_LAYER_STROKE_SCALE: &str = "setLayerStrokeScale";
    pub const FIT_WORLD: &str = "fitWorld";
}
//#endregion 🔖️SceneActions

pub fn ui_stack_vertical(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { menu: None, direction: "vertical".into(), gap: Some("standard".into()), padding: None, id: None, presence: UiPresence::default(), activate: None, children, drop_action: None, drop_overlay: None })
}

/** 🖼️ Builds an image node rendering a source URL or path. */
pub fn ui_image(id: impl Into<String>, src: impl Into<String>, alt: Option<Label>) -> UiNode {
    UiNode::Image(UiImageNode { menu: None, id: id.into(), src: src.into(), alt, presence: UiPresence::default() })
}

/** 🎛️ Extracts the control payload of a {@link UiNode} when it is a control variant. */
pub fn ui_node_to_control(node: &UiNode) -> Option<UiControlNode> {
    match node {
        UiNode::Input(input) => Some(UiControlNode::Input(input.clone())),
        UiNode::Select(select) => Some(UiControlNode::Select(select.clone())),
        UiNode::Toggle(toggle) => Some(UiControlNode::Toggle(toggle.clone())),
        UiNode::Button(button) => Some(UiControlNode::Button(button.clone())),
        UiNode::KeyValue(key_value) => Some(UiControlNode::KeyValue(key_value.clone())),
        UiNode::Slider(slider) => Some(UiControlNode::Slider(slider.clone())),
        UiNode::NumberStepper(stepper) => Some(UiControlNode::NumberStepper(stepper.clone())),
        UiNode::Ring(ring) => Some(UiControlNode::Ring(ring.clone())),
        UiNode::IconSelect(icon) => Some(UiControlNode::IconSelect(icon.clone())),
        _ => None,
    }
}

/** ↩️ Wraps a {@link UiControlNode} back into its matching {@link UiNode} control variant (inverse of {@link ui_node_to_control}). */
pub fn ui_control_to_node(control: UiControlNode) -> UiNode {
    match control {
        UiControlNode::Input(input) => UiNode::Input(input),
        UiControlNode::Select(select) => UiNode::Select(select),
        UiControlNode::Toggle(toggle) => UiNode::Toggle(toggle),
        UiControlNode::Button(button) => UiNode::Button(button),
        UiControlNode::KeyValue(key_value) => UiNode::KeyValue(key_value),
        UiControlNode::Slider(slider) => UiNode::Slider(slider),
        UiControlNode::NumberStepper(stepper) => UiNode::NumberStepper(stepper),
        UiControlNode::Ring(ring) => UiNode::Ring(ring),
        UiControlNode::IconSelect(icon) => UiNode::IconSelect(icon),
    }
}

impl Default for UiNode {
    fn default() -> Self {
        UiNode::Stack(UiStackNode { menu: None, direction: "vertical".into(), gap: Some("standard".into()), padding: None, id: None, presence: UiPresence::default(), activate: None, children: vec![], drop_action: None, drop_overlay: None })
    }
}

pub fn ui_text(value: impl Into<Label>) -> UiNode {
    UiNode::Text(UiTextNode { menu: None, value: value.into(), emphasize: None, data_attributes: None, presence: UiPresence::default() })
}

/** 🔌️ Renders a contributing program body inline at this tree position. */
pub fn ui_external_slot(plugin_id: impl Into<String>, app_id: impl Into<String>, body_key: impl Into<String>, params_json: impl Into<String>) -> UiNode {
    UiNode::ExternalSlot(UiExternalSlotNode { menu: None, plugin_id: plugin_id.into(), app_id: app_id.into(), body_key: body_key.into(), params_json: params_json.into(), host_status: None, presence: UiPresence::default() })
}
//#endregion 🔖️UiNode
// #endregion ui
