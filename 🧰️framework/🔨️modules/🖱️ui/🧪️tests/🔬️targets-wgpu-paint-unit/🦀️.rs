use super::*;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_declaration::ui::{UiFieldNode, UiNumberStepperNode, UiSectionNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiTreeItemAction, UiTreeSectionNode};
use semio_framework_ui_draw_contract::{KIND_GLYPH, KIND_LOADING_BORDER, KIND_ROUNDED, KIND_SOLID, KIND_WAITING_BORDER};
use semio_framework_ui_wgpu_controls::select::arm_retained_select_scroll_at;
use semio_framework_ui_select_behavior::{select_scroll_step, select_scroll_viewport_height, SELECT_COLLISION_PADDING};
use semio_framework_ui_retained_tree::tree::EditState;
use semio_framework_test_fixtures::fixture_text;

include!("🧱️paint-fixtures-and-loading-border-laws/🦀️.rs");
include!("🎚️number-stepper-slider-and-field-paint-laws/🦀️.rs");
include!("🔽️select-popup-drop-and-drag-paint-laws/🦀️.rs");
include!("✍️input-caret-and-focus-ring-paint-laws/🦀️.rs");
include!("🧵️retained-paint-grant-laws/🦀️.rs");
include!("📶️progress-and-image-paint-laws/🦀️.rs");
include!("🦴️skeleton-celebrate-and-chrome-paint-laws/🦀️.rs");
