//! 🧭️ Layout contract module: the action and presence, context menu, window layout, measure and engagement declarations with the shared imports of the former inline module. See [`super`].

// #region layout
//! 📐️ Window layouts, panel tab constants, and engagement rails.

use crate::IconName;
use semio_framework_value::DslValue;
use semio_framework_value::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use semio_framework_ui_contract::UiFixedList;

#[path = "🎬️action-and-presence/🦀️.rs"]
mod action_and_presence;
pub use action_and_presence::{
    ActionDescriptor, StyleSpec, UiPeerMark, UiPresence, UiState, UiStatus,
};
#[cfg(test)]
use action_and_presence::is_default;

#[path = "🗂️context-menu-organizer/🦀️.rs"]
mod context_menu_organizer;
pub use context_menu_organizer::{
    build_shell_context_menu_specs, context_menu_group_label, organize_context_menu,
    ribbon_parent_label, ContextMenuItemSpec, ShellMenuAction, UiMenuRef,
    CONTEXT_MENU_GROUP_ID_PREFIX, CONTEXT_MENU_OVERFLOW_CATEGORY, RIBBON_PARENT_CATEGORIES,
};

#[path = "📋️context-menu-requests-and-panel-tabs/🦀️.rs"]
mod context_menu_requests_and_panel_tabs;
pub use context_menu_requests_and_panel_tabs::{
    framework_panel_tab_label, ContextMenuHit, ContextMenuPoint, ContextMenuRequest,
    ContextMenuResponse, ContextMenuSelectionGroup, ContextMenuSurfaceTarget,
    ContextMenuTextContext, FRAMEWORK_HISTORY_BODY_KEY, FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID,
    FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HISTORY_ICON_ID,
    FRAMEWORK_PANEL_TAB_HISTORY_ID, FRAMEWORK_PANEL_TAB_HISTORY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
    FRAMEWORK_PANEL_TAB_PARAMETERS_ID, FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
    FRAMEWORK_PANEL_TAB_TOOL_RUN_ICON_ID, FRAMEWORK_PANEL_TAB_TOOL_RUN_ID,
    FRAMEWORK_PANEL_TAB_TOOL_RUN_LABEL,
};

#[path = "🪟️window-layout-tree/🦀️.rs"]
mod window_layout_tree;
pub use window_layout_tree::{
    collect_window_kind_ids_from_layout, create_default_layout, create_named_layout,
    create_stack_layout, create_tab_stack_layout, create_window_layout, even_window_layout,
    merge_named_layouts, NamedLayout, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild,
    WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, WindowStackCorner,
};

#[path = "📏️window-measure/🦀️.rs"]
mod window_measure;
pub use window_measure::{
    partition_window_measures, MeasureSelectItem, WindowMeasure, WindowMeasurePartition,
    WINDOW_MEASURE_PARTITION_CAPACITY,
};

#[path = "🤝️window-engagement/🦀️.rs"]
mod window_engagement;
pub use window_engagement::{
    default_viewport_engagement, WindowEngagement, WindowEngagementControl, WindowEngagementInput,
    WindowEngagementOption, WindowEngagementPossible, WindowEngagementRingOption,
    WindowEngagementSelectItem, WindowEngagementSlot, WindowEngagementStatus,
    WindowEngagementToggleGroupOption, WindowOptions,
};

//#region 🔖️WireFormatGoldenTests
/** 🧊️ Golden wire-format tests: freeze exact JSON for layout/action/engagement types
    before these move into ui_wgpu, so the move can be proven byte-identical. */
#[cfg(test)]
include!("../../../🧪️tests/🔬️targets-wgpu-component-layout-layout-wire-format/🦀️.rs");
//#endregion 🔖️WireFormatGoldenTests

//#region 🔖️ValueRoundTripTests
/// 🌱️ `ToValue`/`FromValue` round-trip tests for the keystone types this pass gave the traits
/// to — `FromValue(ToValue(x)) == x` for each, matching the requirement in ticket
/// 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[cfg(test)]
include!("../../../🧪️tests/🔬️targets-wgpu-component-layout-value-round-trip/🦀️.rs");
//#endregion 🔖️ValueRoundTripTests
// #endregion layout
