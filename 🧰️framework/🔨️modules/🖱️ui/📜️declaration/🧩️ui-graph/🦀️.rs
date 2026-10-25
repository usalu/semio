//! 🧩️ Ui graph module: the node, control, tree, inspector, surface, scene and status declarations with the shared imports of the former inline module. See [`super`].

// #region ui
//! 🕸️ Declarative UI graph types shared by kernel, plugins, and renderers.

use crate::IconName;
use semio_framework_ui_locale::Label;
use semio_framework_value::DslValue;
use semio_framework_value::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖Action
pub use super::layout::{build_shell_context_menu_specs, context_menu_group_label, organize_context_menu, ribbon_parent_label, ShellMenuAction, CONTEXT_MENU_GROUP_ID_PREFIX, CONTEXT_MENU_OVERFLOW_CATEGORY, RIBBON_PARENT_CATEGORIES};
pub use super::layout::{ActionDescriptor, StyleSpec, UiPeerMark, UiPresence, UiState, UiStatus};
pub use super::layout::{ContextMenuHit, ContextMenuItemSpec, ContextMenuPoint, ContextMenuRequest, ContextMenuResponse, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, ContextMenuTextContext, UiMenuRef};
//#endregion 🔖Action

#[path = "📚️layout-and-text-nodes/🦀️.rs"]
mod layout_and_text_nodes;
pub use layout_and_text_nodes::{
    UiButtonNode, UiDropOverlaySpec, UiImageNode, UiSeparatorNode, UiStackNode, UiTextNode,
};

#[path = "⌨️input-select-toggle-and-group-nodes/🦀️.rs"]
mod input_select_toggle_and_group_nodes;
pub use input_select_toggle_and_group_nodes::{
    UiGroupNode, UiInputNode, UiKeyValueEntry, UiKeyValueNode, UiSelectItem, UiSelectNode,
    UiToggleNode,
};

#[path = "🎚️slider-stepper-and-control-nodes/🦀️.rs"]
mod slider_stepper_and_control_nodes;
pub use slider_stepper_and_control_nodes::{
    UiControlNode, UiIconSelectNode, UiNumberStepperNode, UiProgressNode, UiRingNode, UiSliderNode,
};

#[path = "🌲️field-section-and-tree-nodes/🦀️.rs"]
mod field_section_and_tree_nodes;
pub use field_section_and_tree_nodes::{
    ui_tree_stamp_presence, UiFieldNode, UiSectionNode, UiTreeActionPlacement, UiTreeItemAction,
    UiTreeItemNode, UiTreeNode, UiTreeSectionNode, UiTreeWindow, UiTreeWindowRowExtent,
};

#[path = "🔍️inspector-helpers/🦀️.rs"]
mod inspector_helpers;
pub use inspector_helpers::{
    ui_declarative_sections_to_tree, ui_inspector_all_equal, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_mixed_select, ui_inspector_mixed_slider,
    ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_inspector_readonly_field,
    ui_inspector_stepper_field, ui_inspector_toggle_field, ui_inspector_vec3_group,
    UiInspectorFieldGroup, UiInspectorMixedNumber, UiInspectorMixedText, UiInspectorMixedToggle,
    UI_INSPECTOR_MIXED_PLACEHOLDER,
};

#[path = "🏔️surface-kind-and-world-records/🦀️.rs"]
mod surface_kind_and_world_records;
pub use surface_kind_and_world_records::{
    world3d_camera_json, world3d_chunking_json, world3d_default_lod_json, world3d_fit_json,
    SurfaceKind, WorldChunkingRecord, WorldLodRecord, WorldMeshLodEntry,
};

#[path = "🍱️table-cells-and-component-scene-nodes/🦀️.rs"]
mod table_cells_and_component_scene_nodes;
pub use table_cells_and_component_scene_nodes::{
    BlockPaletteEntry, TableCell, UiComponentSceneNode, UiExternalSlotNode,
};

#[path = "🧬️ui-node-enum/🦀️.rs"]
mod ui_node_enum;
pub use ui_node_enum::UiNode;

#[path = "🎬️scene-actions-and-node-conversions/🦀️.rs"]
mod scene_actions_and_node_conversions;
pub use scene_actions_and_node_conversions::{
    board2d_actions, ink_canvas_actions, node_graph_actions, text_editor_actions,
    text_editor_code_input, text_editor_json_view, tiled_map_actions, ui_control_to_node,
    ui_external_slot, ui_image, ui_node_to_control, ui_stack_vertical, ui_text,
};

#[path = "🏗️scene-builders/🦀️.rs"]
mod scene_builders;
pub use scene_builders::{
    build_block_list_scene, build_board2d_scene, build_canvas_2d_scene, build_diff_view_scene,
    build_event_feed_scene, build_graph_timeline_scene, build_icon_render_scene,
    build_ink_canvas_scene, build_node_graph_scene, build_paint_2d_scene, build_table_scene,
    build_text_editor_scene, build_tiled_map_scene, build_virtual_file_system_scene,
    build_world_3d_scene, text_identifier_bounds_at, text_identifier_occurrences_json,
};

#[path = "🧯️status-builders/🦀️.rs"]
mod status_builders;
pub use status_builders::{ui_empty_state, ui_error_state, ui_import_drop_zone, ui_recovery_panel};

//#region 🔖️ComponentScenes

// 🎬️ The 15 product scene structs (`Canvas2dScene`, `World3dScene`, `NodeGraphScene` + its
// nested port/node/edge/viewport/operator records, `TextEditorScene`, `TableScene`,
// `Paint2dScene`, `IconRenderScene`, `VirtualFileSystemScene`, `TiledMapScene`, `Board2dScene`,
// `InkCanvasScene`, `GraphTimelineScene`, `DiffViewScene`, `EventFeedScene`, `BlockListScene`)
// relocated to `semio-framework-ui-scene` — ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
// RUNTIME packet `scene-surface`. Re-exported here (not a compatibility shim: this crate
// genuinely still builds `UiNode`s from them, see `build_table_scene` etc. below) so every
// Scene payload ownership belongs to the independent scene contract.
// `TableScene::drop_action_json` and `NodeGraphOperatorChannelRecord::default_json` are now
// opaque JSON strings, not live `ActionDescriptor`/`serde_json::Value` — see the scene crate's
// own `🦀️scenes.rs` header for why (that crate is wasm-safe and depends on nothing beyond
// `ui_contract`/`serde`, so it cannot carry either type).
pub use ui_scene::{
    canvas2d_snapshot_abort_write, canvas2d_snapshot_abort_write_step, canvas2d_snapshot_admit_page, canvas2d_snapshot_begin, canvas2d_snapshot_begin_close, canvas2d_snapshot_close_step, canvas2d_snapshot_seal,
    canvas2d_snapshot_terminal_is_empty, canvas2d_snapshot_with_page, canvas2d_snapshot_write_terminal_is_empty, decode as decode_surface_doc, encode as encode_surface_doc, world3d_snapshot_abort_write, world3d_snapshot_abort_write_step,
    world3d_snapshot_admit_page, world3d_snapshot_begin, world3d_snapshot_begin_close, world3d_snapshot_claim_draw_permit, world3d_snapshot_close_step, world3d_snapshot_seal, world3d_snapshot_terminal_is_empty, world3d_snapshot_with_page,
    world3d_snapshot_write_terminal_is_empty, BlockListSelectionTarget, BlockListBlock, BlockListStep, BlockListPaletteEntry, BlockListScene, Board2dScene, Canvas2dFraming, Canvas2dRejectedSnapshotPage, Canvas2dScene, Canvas2dSnapshotDescriptor, Canvas2dSnapshotFault, Canvas2dSnapshotLease, Canvas2dSnapshotPage,
    Canvas2dSnapshotWriteToken, DiffViewScene, EventFeedScene, GraphTimelineScene, IconRenderScene, InkCanvasInteractionDomain, InkCanvasScene, NodeGraphEdgeRecord, NodeGraphFindItem, NodeGraphHover, NodeGraphInteractionDomain,
    NodeGraphNodeRecord, NodeGraphOperatorChannelRecord, NodeGraphOperatorRecord, NodeGraphOperatorVariadicRecord, NodeGraphPortRecord, NodeGraphScene, Paint2dScene, SceneDoc, TableScene, TextEditorScene, TextSplice, TextSpliceComposition, LocatedTextSplice, AppliedTextSplice, DraftChange, DRAFT_SPLICES_ARGUMENT, compose_draft_step, write_draft_changes_json_into, DraftChangesJsonCursor, TEXT_EDITOR_TYPING_BUFFER_ARG, TEXT_EDITOR_TYPING_COMMIT_ARG, TEXT_EDITOR_TYPING_IDLE_MS, TEXT_SPLICE_CONTEXT_SCALARS, TiledMapScene, VirtualFileSystemScene,
    scene_lane_hash, world3d_modelling_lane_text, world3d_modelling_strings, world3d_sub_element_paint, project_world3d_annotations, format_world3d_legend_value, World3dAnnotation, World3dAnnotationLayer, World3dColorRamp, World3dHighlight, World3dLabelAlign,
    World3dLegendLines, World3dMarkerShape, World3dModellingOptions, World3dModellingStrings, World3dPickGranularity, World3dProjectedAnnotation, World3dScalarDomain, World3dScalarField, World3dSection, World3dSubElementDefaults, World3dText, World3dTone,
    WORLD3D_ANNOTATION_ARROW_PX, WORLD3D_ANNOTATION_LABEL_GAP_PX, WORLD3D_ANNOTATION_MARKER_PX, WORLD3D_ANNOTATION_STROKE_PX, WORLD3D_SCALAR_NO_DATA_RGB, World3dProjectedArc,
    World3dPresentation, World3dPresentationClear, World3dRejectedSnapshotPage, World3dScene, World3dSnapshotDescriptor, World3dSnapshotDrawPermit, World3dSnapshotFault, World3dSnapshotItem, World3dSnapshotLease, World3dSnapshotPage,
    World3dSnapshotPageKind, World3dSnapshotSpan, World3dSnapshotWriteToken, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY,
};

// 🎬️ `NodeGraphRecords` (`NodeGraphPortRecord`/`NodeGraphNodeRecord`/`NodeGraphEdgeRecord`/
// `NodeGraphFindItem`/`NodeGraphHover`/`NodeGraphOperator*Record`) and
// `NodeGraphScene` itself moved to `semio-framework-ui-scene` together (see the re-export above)
// — the latter's fields are typed directly against the former, so they moved as one unit.

// 🎬️ `TextEditorScene` and `TableScene` (+ its `base()`) moved to `semio-framework-ui-scene`
// (see the re-export above). `TableScene.drop_action` is `drop_action_json: Option<String>` on
// the moved type — it named `ActionDescriptor` (this crate's own type, defined just below via
// `UiTreeItemAction`'s sibling), which the wasm-safe scene crate cannot depend on.
//
// `TableCell` stays here; `TableScene` receives its rows through the scene wire string.

//#endregion 🔖️ComponentScenes

//#region 🔖️WireFormatGoldenTests
/** 🧊️ Golden wire-format tests: freeze exact JSON for every UiNode/scene/SurfaceKind
    before these types move into ui_wgpu, so the move can be proven byte-identical. */
#[cfg(test)]
include!("../../🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs");
//#endregion 🔖️WireFormatGoldenTests

//#region 🔖️ValueRoundTripTests
/// 🌱️ `ToValue`/`FromValue` round-trip tests for ticket
/// 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS's additive pass over
/// `pub mod ui`'s standalone node types, plus (since `semio-framework-ui-scene` landed hand-
/// written `ToValue`/`FromValue` for all 15 embedded scene payload types) the previously-
/// blocked recursive-through-`UiNode` family: `UiStackNode`/`UiGroupNode`/`UiFieldNode`/
/// `UiSectionNode`/`UiInspectorFieldGroup`/`UiComponentSceneNode`/`UiNode` itself.
#[cfg(test)]
include!("../../🧪️tests/🔬️targets-wgpu-component-ui-value-round-trip/🦀️.rs");
//#endregion 🔖️ValueRoundTripTests
// #endregion ui
