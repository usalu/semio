// #region reconcile
//! 🔁️ Keyed single-pass reconciliation: applies an incoming declarative `UiNode` tree to a retained
//! `UiTree`, matching children by key, diffing matched nodes, and marking the minimal dirty flags.
//! `Stack`/`Section`/`Field` recurse into their own literal `UiNode` children; `Select`/`Tree` recurse
//! into *synthesized* retained children (see `🔖️CompositeExpansion` below) built from their
//! non-`UiNode` payload (`items`/`sections`) since there is no dedicated `UiNode` variant for "one
//! Select option row" or "one Tree row" to reuse verbatim — the remaining 14 variants have no nested
//! `UiNode`/composite payload at all, so a diffed leaf is already their complete, correct treatment.
//! KNOWN GAP (wiring request, not fixable from this region alone — see `tree::WidgetState`'s own doc
//! comment): `Select`'s synthesized option rows are always built, unconditionally, regardless of
//! open/closed — `tree::WidgetState` is currently a zero-field marker with nowhere to record "is this
//! Select open", so this region can't gate the *rows' existence* on it. `NodeFlags::HAS_POPUP` is set
//! on the `Select` node itself (whenever it has ≥1 item) so a later events/paint milestone can find
//! the always-ready rows once `WidgetState` grows an `open`-like field to gate *showing*/hit-testing
//! them — no further reconcile-side change should be needed at that point.

use semio_framework_ui_locale::Label;
use semio_framework_ui_declaration::ui::UiTreeActionPlacement;
use semio_framework_value::DslValue;
#[cfg(any(test, feature = "testkit"))]
use std::borrow::Cow;
use std::collections::HashMap;
#[cfg(any(test, feature = "testkit"))]
use std::collections::HashSet;

use semio_framework_ui_arena::NodeId;
use semio_framework_ui_declaration::layout::ActionDescriptor;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_ui_declaration::ui::ui_control_to_node;
use semio_framework_ui_declaration::ui::{
    SurfaceKind, UiButtonNode, UiComponentSceneNode, UiControlNode, UiDropOverlaySpec, UiFieldNode, UiGroupNode, UiIconSelectNode, UiImageNode, UiInputNode, UiKeyValueEntry, UiKeyValueNode, UiMenuRef, UiNode, UiNumberStepperNode, UiPresence,
    UiProgressNode, UiRingNode, UiSectionNode, UiSelectItem, UiSelectNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiState, UiStatus, UiTextNode, UiToggleNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, UiTreeWindow,
    UiTreeWindowRowExtent,
};
use crate::tree::{Node, NodeFlags, NodeKey, NodeKeyRef, UiDocumentPageRejection, UiDocumentTree, UiDocumentTreeFault, UiTree, WidgetSpec};
use crate::{IconName, UiIntentAddress, UiIntentBindings};
use ui_contract::{UiDocumentNodePage, UiNodeId, UiNodeRecord, UI_DOCUMENT_NODES};

#[path = "📄️page-reconcile-and-cursor/🦀️.rs"]
mod page_reconcile_and_cursor;
pub use page_reconcile_and_cursor::{
    UiComponentSceneWitness, UiDocumentReconcileCursor, UiDocumentReconcileFault,
    UiDocumentReconcileStep, UiRetiredComponentScene, UI_DOCUMENT_RECONCILE_DEPTH,
};
use page_reconcile_and_cursor::{PlannedNode, UiDocumentReconcilePhase};

#[path = "🧾️record-routing-and-actions/🦀️.rs"]
mod record_routing_and_actions;
use record_routing_and_actions::{
    contract_label, drop_overlay, icon_name, input_kind, layout_routing_flags, menu_ref,
    merge_scene_lanes, optional_contract_label, record_action, record_action_or_inert,
    record_consumes_subtree, record_intent_bindings, record_presence, row_activation,
    row_verb_action, stack_metrics, surface_kind, surface_plugin_id,
};
pub use record_routing_and_actions::component_scene_host_id;

#[path = "🖼️surface-scene-and-row-nodes/🦀️.rs"]
mod surface_scene_and_row_nodes;
use surface_scene_and_row_nodes::{
    data_attributes, drag_data, row_actions, surface_scene_node, table_section, tree_window,
};

#[path = "🌲️tree-item-nodes/🦀️.rs"]
mod tree_item_nodes;
use tree_item_nodes::{tree_item, tree_toolbar};
#[cfg(test)]
use tree_item_nodes::tree_row_tone;
pub use tree_item_nodes::{
    tree_content_label_lines, tree_content_lines, tree_row_content, tree_row_content_lines,
};

#[path = "🎛️widget-nodes/🦀️.rs"]
mod widget_nodes;
use widget_nodes::{
    button_node, icon_select_node, input_node, key_value_node, number_stepper_node, progress_node,
    ring_node, select_node, slider_node, toggle_node,
};

#[path = "🎞️media-transport-and-node-from-record/🦀️.rs"]
mod media_transport_and_node_from_record;
pub use media_transport_and_node_from_record::{
    media_transport_contract_valid, ui_node_from_record, MEDIA_TRANSPORT_EXTENSION_ID,
};

#[path = "🌳️ui-tree-reconcile/🦀️.rs"]
mod ui_tree_reconcile;

#[path = "🔽️composite-expansion-rows/🦀️.rs"]
mod composite_expansion_rows;
pub use composite_expansion_rows::select_item_row;
#[cfg(any(test,feature="testkit"))]
use composite_expansion_rows::{layout_affecting_change, own_fields_equal, tree_section_row};

#[path = "🧰️reconcile-testkit-helpers/🦀️.rs"]
mod reconcile_testkit_helpers;
#[cfg(any(test,feature="testkit"))]
use reconcile_testkit_helpers::node_key;

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-reconcile-unit/🦀️.rs"]
mod tests;

// #endregion reconcile
