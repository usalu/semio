use super::*;
use semio_framework_ui_declaration::ui::{UiButtonNode, UiComponentSceneNode, UiInputNode, UiNumberStepperNode, UiPresence, UiSelectItem, UiSelectNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiTextNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
use semio_framework_ui_retained_tree::tree::{Node, NodeKey, WidgetSpec};
use semio_framework_ui_declaration::IconName;
use semio_framework_ui_locale::Label;
use semio_framework_test_fixtures::fixture_text;

include!("🎯️event-fixtures-hit-test-capture-and-focus-laws/🦀️.rs");
include!("🪟️overlay-drag-drop-and-scroll-laws/🦀️.rs");
include!("✍️edit-state-and-hover-reveal-laws/🦀️.rs");
include!("🖱️interactivity-and-scene-command-laws/🦀️.rs");
include!("📍️anchored-placement-and-tooltip-dwell-laws/🦀️.rs");
include!("⌨️widget-keyboard-laws/🦀️.rs");
include!("🧭️flow-direction-focus-visible-and-search-moment-laws/🦀️.rs");
include!("🪞️mirrored-number-and-text-keyboard-laws/🦀️.rs");
