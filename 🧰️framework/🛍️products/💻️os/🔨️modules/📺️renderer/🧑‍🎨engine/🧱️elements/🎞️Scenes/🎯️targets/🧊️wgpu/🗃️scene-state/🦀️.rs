//! 🗃️ Scene runtime state: admitted surface map, viewport, drag and ink-edit state, host temporal presentation, accessibility controls, surface retirement and canvas gestures.

use super::*;

#[path = "📐️viewport-drag-and-accessibility-controls/🦀️.rs"]
mod viewport_drag_and_accessibility_controls;
pub use viewport_drag_and_accessibility_controls::{
    BlockListAccessibilityControl, EventFeedAccessibilityControl, GraphTimelineAccessibilityControl,
    HostTemporalPresentation, InkEditKind, InkEditState, SceneAccessibilityPresentation, SceneDrag,
    SceneDragMode, VfsAccessibilityControl, VfsAccessibilityControlKind, Viewport,
    World3dAccessibilityControl,
};

#[path = "🧯️surface-state-and-retirement/🦀️.rs"]
mod surface_state_and_retirement;
pub use surface_state_and_retirement::{close_retired_scene_surface_one, SceneSurfaceState};
pub(crate) use surface_state_and_retirement::{SceneSurfaceRetirement, SceneValueRetirement};

#[path = "🖱️canvas-gestures-input-kit-and-chrome/🦀️.rs"]
mod canvas_gestures_input_kit_and_chrome;
pub use canvas_gestures_input_kit_and_chrome::{
    canvas_active_utility, render_placeholder, request_canvas_pointer_gesture_cancel_for_host,
    retire_tiled_map_scene_identity, write_scene_action_batch, BlockListChromeLabels,
    CanvasCatalogueHover, CanvasClick, CanvasGesture, CanvasGestureKind, CanvasGestureSlots,
    IconRenderChromeLabels, SceneChromeLabels, SceneEngineHosts, SceneListHit,
    SceneListTransferRelease, SceneListTransferStart, SceneModifiers, TableButtonAccessibilityCell,
    TableEditableTextAccessibilityCell, TableEditableTextFocusTarget, TableStepperAccessibilityCell,
    TableStepperFocusTarget, VirtualFileSystemChromeLabels, BLOCK_LIST_PALETTE_DRAG_MIME,
    MAP_MARQUEE_THRESHOLD_PX,
};

#[path = "✍️editor-ink-gumball-and-canvas-layer-wire/🦀️.rs"]
mod editor_ink_gumball_and_canvas_layer_wire;
pub use editor_ink_gumball_and_canvas_layer_wire::{
    draw_ink_rect_outline, pivot_screen, request_cancel, text_editor_ui, total, CanvasFillJson,
    CanvasGradientStopJson, CanvasLayer, GumballConfig, GumballGesture, GumballHandle,
    GumballMotion, GumballSlot, GumballView, InkBoundsF, ModelToLayer, TextEditorRenameDraft,
    TextEditorUiState, VfsAccessibilityFocusTarget, MOVE_EPSILON, TURN_EPSILON,
};
pub(crate) use editor_ink_gumball_and_canvas_layer_wire::{
    CanvasImageFieldJson, CanvasStrokeJson, CanvasTextFieldJson,
};
