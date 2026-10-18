use super::*;
use semio_framework_value::DslValue;
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{
    ui_node_to_control, SurfaceKind, UiButtonNode, UiComponentSceneNode, UiControlNode, UiExternalSlotNode, UiFieldNode, UiGroupNode, UiIconSelectNode, UiImageNode, UiInputNode, UiKeyValueEntry, UiKeyValueNode, UiNumberStepperNode, UiPresence,
    UiProgressNode, UiRingNode, UiSectionNode, UiSelectItem, UiSelectNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiState, UiTextNode, UiToggleNode, UiTreeActionPlacement, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
};
use crate::wgpu::events::{AccessibilityUiEvent, PointerButton};
use crate::wgpu::geometry::Rect;
use crate::wgpu::input::{HitKind, InputState};
use crate::wgpu::scene_slots::SceneSlot;
use crate::wgpu::widgets::{
    draw_text_on, draw_text_overlay_on, measure_widget, render_scroll_region, render_widget, wrap_text, ControlNode, InputMeta, KeyValueEntry, RingMeta, SelectItem, SliderMeta, StepperMeta, TreeItem, TreeItemAction, TreeSection, WidgetContext,
    WidgetInteractionMaps, WidgetNode,
};
use crate::wgpu::Label;
use crate::wgpu::TOOLTIP_DWELL_SECONDS;
use std::collections::HashMap as StdHashMap;
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiNodeId, UiNodeRecord, UiRevision};

include!("🧱️engine-fixtures-and-scene-lifetime-laws/🦀️.rs");
include!("💬️tooltip-and-accessibility-fixtures-laws/🦀️.rs");
include!("⚙️settings-close-and-accessibility-commit-laws/🦀️.rs");
include!("🖼️facade-frame-storm-and-mounted-layout-laws/🦀️.rs");
include!("🎭️scene-host-overlay-body-and-frame-laws/🦀️.rs");
include!("🏆️golden-widget-conversions-and-paint-goldens/🦀️.rs");
include!("🔬️golden-tree-gaps-introspection-and-widget-measure-laws/🦀️.rs");
include!("🖱️widget-interaction-and-render-laws/🦀️.rs");
include!("📐️surface-content-height-and-worker-close-laws/🦀️.rs");
