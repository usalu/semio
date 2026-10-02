//! 🧊️ Real extension sessions drive the public Generation3d app law corpus.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
use semio_s_artifact_procedural_generation3d::*;
use semio_framework_plugin::*;
use semio_framework_os_flow::*;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, FlowGraphStep, Widget};
use semio_framework_os_flow::forms_bridge::flow_host_snapshot_to_form_spec;
use serde_json::Value;

/// 📸️ A concrete law fixture owns and retires each projection read.
pub(crate) struct SnapshotRead(Option<Generation3dSnapshot>);

impl SnapshotRead {
    pub(crate) fn new(snapshot: Generation3dSnapshot) -> Self { Self(Some(snapshot)) }
    pub(crate) fn into_inner(mut self) -> Generation3dSnapshot { self.0.take().expect("projection read owns its snapshot") }
}

impl std::ops::Deref for SnapshotRead {
    type Target = Generation3dSnapshot;
    fn deref(&self) -> &Self::Target { self.0.as_ref().expect("projection read owns its snapshot") }
}

impl std::ops::DerefMut for SnapshotRead {
    fn deref_mut(&mut self) -> &mut Self::Target { self.0.as_mut().expect("projection read owns its snapshot") }
}

impl Drop for SnapshotRead {
    fn drop(&mut self) { if let Some(snapshot) = self.0.take() { snapshot.retire_cold(); } }
}

impl PartialEq for SnapshotRead {
    fn eq(&self, other: &Self) -> bool { **self == **other }
}

impl PartialEq<Generation3dSnapshot> for SnapshotRead {
    fn eq(&self, other: &Generation3dSnapshot) -> bool { **self == *other }
}

impl std::fmt::Debug for SnapshotRead {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(&**self, formatter) }
}

/// 🧭️ One coordinate-literal parameter (`{x, y, z}`) of a graph operator, or `default` when the operator or the
/// parameter is absent — what a gumball law reads off the transform operator its gesture composed into.
pub(crate) fn gumball_param_vector(snapshot: &FlowHostSnapshot, id: &str, key: &str, default: [f64; 3]) -> [f64; 3] {
    let widget = snapshot.widgets.iter().find(|widget| widget_id(widget) == id).map(dsl::ToValue::to_value);
    let literal = widget.as_ref().and_then(|widget| widget.get("params")).and_then(|params| params.get(key));
    std::array::from_fn(|axis| literal.and_then(|value| value.get(["x", "y", "z"][axis])).and_then(dsl::DslValue::as_f64).unwrap_or(default[axis]))
}

/// 🔢️ One number-literal parameter (`{value}`) of a graph operator, or `default` when the operator or the parameter is
/// absent.
pub(crate) fn gumball_param_number(snapshot: &FlowHostSnapshot, id: &str, key: &str, default: f64) -> f64 {
    let widget = snapshot.widgets.iter().find(|widget| widget_id(widget) == id).map(dsl::ToValue::to_value);
    widget.as_ref().and_then(|widget| widget.get("params")).and_then(|params| params.get(key)).and_then(|literal| literal.get("value")).and_then(dsl::DslValue::as_f64).unwrap_or(default)
}

#[path = "🧪️tests/🔒️serial-lock-discipline/🦀️.rs"]
pub(crate) mod serial_lock_discipline;

#[path = "🧪️tests/🔬️brep-extension/🦀️.rs"]
pub(crate) mod brep_extension;

#[path = "🧪️tests/🔬️publication-authority/🦀️.rs"]
pub(crate) mod publication_authority;

#[path = "🧪️tests/🔬️serial/🦀️.rs"]
pub(crate) mod test_serial;

#[path = "."]
mod preview_eval_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::preview_eval::*;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod viewer_generation3d_eval_chain_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::*;
use semio_s_artifact_procedural_generation3d::preview_eval;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::commands::{export_document, flow_eval_release, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use semio_s_artifact_procedural_generation3d::viewer::generation3d::modes::view;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::modes::view::windows::preview;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::presence::{Generation3dViewPresence, Generation3dViewPresenceMutation};
use semio_s_artifact_procedural_generation3d::viewer::generation3d::transient::{Generation3dViewTransient, Generation3dViewTransientMutation};
use semio_s_artifact_procedural_generation3d::{Generation3dMutation, Generation3dSnapshot, GENERATION3D_DIALECT, GENERATION_3D_SCHEMA};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️eval-chain/🦀️.rs"]
mod laws;
}

#[path = "."]
mod viewer_domain {
use super::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::*;
use semio_s_artifact_procedural_generation3d::preview_eval;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::commands::{export_document, flow_eval_release, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use semio_s_artifact_procedural_generation3d::viewer::generation3d::modes::view;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::modes::view::windows::preview;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::presence::{Generation3dViewPresence, Generation3dViewPresenceMutation};
use semio_s_artifact_procedural_generation3d::viewer::generation3d::transient::{Generation3dViewTransient, Generation3dViewTransientMutation};
use semio_s_artifact_procedural_generation3d::{Generation3dMutation, Generation3dSnapshot, GENERATION3D_DIALECT, GENERATION_3D_SCHEMA};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod viewer_laws;
}

#[path = "."]
mod viewer_generation3d_status_contract_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::*;
use semio_s_artifact_procedural_generation3d::preview_eval;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::commands::{export_document, flow_eval_release, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use semio_s_artifact_procedural_generation3d::viewer::generation3d::modes::view;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::modes::view::windows::preview;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::presence::{Generation3dViewPresence, Generation3dViewPresenceMutation};
use semio_s_artifact_procedural_generation3d::viewer::generation3d::transient::{Generation3dViewTransient, Generation3dViewTransientMutation};
use semio_s_artifact_procedural_generation3d::{Generation3dMutation, Generation3dSnapshot, GENERATION3D_DIALECT, GENERATION_3D_SCHEMA};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️status-contract/🦀️.rs"]
mod laws;
}

#[path = "."]
mod viewer_generation3d_modes_view_windows_preview_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::modes::view::windows::preview::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config::Generation3dViewConfig;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
use semio_s_artifact_procedural_generation3d::preview_eval::{self, PreviewChannelItem, PreviewInlineGeometry};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod viewer_generation3d_commands_set_sun_azimuth_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::commands::set_sun_azimuth::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🧭️set-sun-azimuth/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod viewer_generation3d_commands_set_sun_elevation_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::commands::set_sun_elevation::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🌄️set-sun-elevation/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod viewer_generation3d_commands_set_sun_intensity_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::commands::set_sun_intensity::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🔆️set-sun-intensity/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod viewer_generation3d_commands_set_camera_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::commands::set_camera::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation, Generation3dViewCamera};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/📷️set-camera/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod viewer_generation3d_commands_set_active_example_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::commands::set_active_example::*;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::is_generation3d_example_id;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod viewer_generation3d_commands_set_show_mode_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::commands::set_show_mode::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/👁️set-show-mode/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod viewer_generation3d_commands_set_lod_mode_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::commands::set_lod_mode::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🔬️set-lod-mode/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod viewer_generation3d_commands_toggle_sun_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::commands::toggle_sun::*;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🌞️toggle-sun/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_panels_artifact_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::panels::artifact::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::Generation3dConfig;
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::Generation3dLabels;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{dag_host_snapshot_to_workflow, with_host};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_panels_inspection_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::panels::inspection::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::Generation3dLabels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use semio_s_artifact_procedural_generation3d::widget_id;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_panels_catalogue_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::panels::catalogue::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::Generation3dLabels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_fold_contract_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::navigate_graph::{activate_selection, select_downstream_node, select_next_node, select_previous_node, select_upstream_node};
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::{
    add_generation, add_widget, cycle_lod_mode, cycle_show_mode, delete_selection, export_document, flow_eval_release, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, import_document, import_document_request, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_generation, remove_widget, rename_generation, reorganize, rotate_selection,
    scale_selection, select_generation, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun, translate_selection,
    update_generation_values, set_widget_input,
};
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::generate::windows::{form, generations, preview as generate_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::{edit, generate};
use semio_s_artifact_procedural_generation3d::editor::generation3d::panels::{catalogue as catalogue_panel, artifact as artifact_panel, inspection as inspection_panel};
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::generation3d_labels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::transient::{Generation3dTransient, Generation3dTransientMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::{artifact_kind, Generation3dSnapshot, GENERATION_3D_SCHEMA};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️fold-contract/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_interaction_scope_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::navigate_graph::{activate_selection, select_downstream_node, select_next_node, select_previous_node, select_upstream_node};
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::{
    add_generation, add_widget, cycle_lod_mode, cycle_show_mode, delete_selection, export_document, flow_eval_release, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, import_document, import_document_request, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_generation, remove_widget, rename_generation, reorganize, rotate_selection,
    scale_selection, select_generation, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun, translate_selection,
    update_generation_values, set_widget_input,
};
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::generate::windows::{form, generations, preview as generate_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::{edit, generate};
use semio_s_artifact_procedural_generation3d::editor::generation3d::panels::{catalogue as catalogue_panel, artifact as artifact_panel, inspection as inspection_panel};
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::generation3d_labels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::transient::{Generation3dTransient, Generation3dTransientMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::{artifact_kind, Generation3dSnapshot, GENERATION_3D_SCHEMA};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🕹️interaction-scope/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_generate_interactions_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::navigate_graph::{activate_selection, select_downstream_node, select_next_node, select_previous_node, select_upstream_node};
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::{
    add_generation, add_widget, cycle_lod_mode, cycle_show_mode, delete_selection, export_document, flow_eval_release, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, import_document, import_document_request, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_generation, remove_widget, rename_generation, reorganize, rotate_selection,
    scale_selection, select_generation, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun, translate_selection,
    update_generation_values, set_widget_input,
};
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::generate::windows::{form, generations, preview as generate_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::{edit, generate};
use semio_s_artifact_procedural_generation3d::editor::generation3d::panels::{catalogue as catalogue_panel, artifact as artifact_panel, inspection as inspection_panel};
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::generation3d_labels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::transient::{Generation3dTransient, Generation3dTransientMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::{artifact_kind, Generation3dSnapshot, GENERATION_3D_SCHEMA};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️generate-interactions/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_example_switch_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::navigate_graph::{activate_selection, select_downstream_node, select_next_node, select_previous_node, select_upstream_node};
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::{
    add_generation, add_widget, cycle_lod_mode, cycle_show_mode, delete_selection, export_document, flow_eval_release, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, import_document, import_document_request, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_generation, remove_widget, rename_generation, reorganize, rotate_selection,
    scale_selection, select_generation, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun, translate_selection,
    update_generation_values, set_widget_input,
};
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::generate::windows::{form, generations, preview as generate_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::{edit, generate};
use semio_s_artifact_procedural_generation3d::editor::generation3d::panels::{catalogue as catalogue_panel, artifact as artifact_panel, inspection as inspection_panel};
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::generation3d_labels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::transient::{Generation3dTransient, Generation3dTransientMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::{artifact_kind, Generation3dSnapshot, GENERATION_3D_SCHEMA};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️example-switch/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_domain {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::navigate_graph::{activate_selection, select_downstream_node, select_next_node, select_previous_node, select_upstream_node};
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::{
    add_generation, add_widget, cycle_lod_mode, cycle_show_mode, delete_selection, export_document, flow_eval_release, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, import_document, import_document_request, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_generation, remove_widget, rename_generation, reorganize, rotate_selection,
    scale_selection, select_generation, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun, translate_selection,
    update_generation_values, set_widget_input,
};
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::generate::windows::{form, generations, preview as generate_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::{edit, generate};
use semio_s_artifact_procedural_generation3d::editor::generation3d::panels::{catalogue as catalogue_panel, artifact as artifact_panel, inspection as inspection_panel};
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::generation3d_labels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::transient::{Generation3dTransient, Generation3dTransientMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::{artifact_kind, Generation3dSnapshot, GENERATION_3D_SCHEMA};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod editor_laws;
}

#[path = "."]
mod editor_generation3d_tick_addressing_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::navigate_graph::{activate_selection, select_downstream_node, select_next_node, select_previous_node, select_upstream_node};
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::{
    add_generation, add_widget, cycle_lod_mode, cycle_show_mode, delete_selection, export_document, flow_eval_release, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, import_document, import_document_request, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_generation, remove_widget, rename_generation, reorganize, rotate_selection,
    scale_selection, select_generation, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun, translate_selection,
    update_generation_values, set_widget_input,
};
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::generate::windows::{form, generations, preview as generate_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::{edit, generate};
use semio_s_artifact_procedural_generation3d::editor::generation3d::panels::{catalogue as catalogue_panel, artifact as artifact_panel, inspection as inspection_panel};
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::generation3d_labels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::transient::{Generation3dTransient, Generation3dTransientMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::{artifact_kind, Generation3dSnapshot, GENERATION_3D_SCHEMA};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️tick-addressing/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_work_capacity_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::navigate_graph::{activate_selection, select_downstream_node, select_next_node, select_previous_node, select_upstream_node};
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::{
    add_generation, add_widget, cycle_lod_mode, cycle_show_mode, delete_selection, export_document, flow_eval_release, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, import_document, import_document_request, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_generation, remove_widget, rename_generation, reorganize, rotate_selection,
    scale_selection, select_generation, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun, translate_selection,
    update_generation_values, set_widget_input,
};
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::generate::windows::{form, generations, preview as generate_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::{edit, generate};
use semio_s_artifact_procedural_generation3d::editor::generation3d::panels::{catalogue as catalogue_panel, artifact as artifact_panel, inspection as inspection_panel};
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::generation3d_labels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::transient::{Generation3dTransient, Generation3dTransientMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::{artifact_kind, Generation3dSnapshot, GENERATION_3D_SCHEMA};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_mode_panels_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::navigate_graph::{activate_selection, select_downstream_node, select_next_node, select_previous_node, select_upstream_node};
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::{
    add_generation, add_widget, cycle_lod_mode, cycle_show_mode, delete_selection, export_document, flow_eval_release, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, import_document, import_document_request, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_generation, remove_widget, rename_generation, reorganize, rotate_selection,
    scale_selection, select_generation, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun, translate_selection,
    update_generation_values, set_widget_input,
};
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::generate::windows::{form, generations, preview as generate_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::{edit, generate};
use semio_s_artifact_procedural_generation3d::editor::generation3d::panels::{catalogue as catalogue_panel, artifact as artifact_panel, inspection as inspection_panel};
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::generation3d_labels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::transient::{Generation3dTransient, Generation3dTransientMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::{artifact_kind, Generation3dSnapshot, GENERATION_3D_SCHEMA};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mode-panels/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_slider_values_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::navigate_graph::{activate_selection, select_downstream_node, select_next_node, select_previous_node, select_upstream_node};
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::{
    add_generation, add_widget, cycle_lod_mode, cycle_show_mode, delete_selection, export_document, flow_eval_release, flow_eval_resolve, flow_eval_tick, flow_tessellate_cancel_resolve, flow_tessellate_resolve, import_document, import_document_request, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_generation, remove_widget, rename_generation, reorganize, rotate_selection,
    scale_selection, select_generation, set_active_example, set_camera, set_contributions, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun, translate_selection,
    update_generation_values, set_widget_input,
};
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::generate::windows::{form, generations, preview as generate_preview};
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::{edit, generate};
use semio_s_artifact_procedural_generation3d::editor::generation3d::panels::{catalogue as catalogue_panel, artifact as artifact_panel, inspection as inspection_panel};
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::generation3d_labels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::transient::{Generation3dTransient, Generation3dTransientMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::{artifact_kind, Generation3dSnapshot, GENERATION_3D_SCHEMA};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🎚️slider-values/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_modes_generate_windows_preview_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::generate::windows::preview::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::Generation3dConfig;
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::edit::windows::preview::show_mode_measure;
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::Generation3dLabels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use semio_s_artifact_procedural_generation3d::editor::generation3d::{preview_camera_json, preview_payload, preview_selection_json, preview_status_json, preview_window_status_json, PreviewInteractionMarks, PreviewPayload, PreviewStatusDebug, GENERATION_3D_INTERACTION_DOMAIN, GENERATION_3D_INTERACTION_GRANULARITY};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{generation_by_id, generation_host_snapshot_for};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_modes_generate_windows_form_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::generate::windows::form::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::Generation3dLabels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_modes_generate_windows_generations_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::generate::windows::generations::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_modes_edit_windows_preview_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::edit::windows::preview::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::Generation3dConfig;
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::Generation3dLabels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use semio_s_artifact_procedural_generation3d::editor::generation3d::{preview_camera_json, preview_payload, preview_selection_json, preview_status_json, preview_window_status_json, PreviewInteractionMarks, PreviewStatusDebug, GENERATION_3D_INTERACTION_DOMAIN, GENERATION_3D_INTERACTION_GRANULARITY};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_modes_edit_windows_flow_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::modes::edit::windows::flow::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::Generation3dConfig;
use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::Generation3dLabels;
use semio_s_artifact_procedural_generation3d::editor::generation3d::PreviewInteractionMarks;
use semio_s_artifact_procedural_generation3d::editor::generation3d::GENERATION_3D_INTERACTION_CHANNEL;
use semio_s_artifact_procedural_generation3d::editor::generation3d::GENERATION_3D_INTERACTION_DOMAIN;
use semio_s_artifact_procedural_generation3d::editor::generation3d::GENERATION_3D_INTERACTION_GRANULARITY;
use semio_s_artifact_procedural_generation3d::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{dag_host_snapshot_to_workflow, with_host};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_flow_eval_tick_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::flow_eval_tick::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::preview_eval;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_scale_selection_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::scale_selection::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{gumball_widget_json, with_host};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📏️scale-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_node_graph_edit_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::node_graph_edit::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{commit_host_snapshot, with_host};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_set_widget_input_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::set_widget_input::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{commit_host_snapshot, with_host, mutations::text::Generation3dMutation};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_flow_tessellate_cancel_resolve_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::flow_tessellate_cancel_resolve::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::preview_eval;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧯️flow-tessellate-cancel-resolve/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_flow_eval_resolve_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::flow_eval_resolve::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::flow_eval_tick;
use semio_s_artifact_procedural_generation3d::preview_eval;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️flow-eval-resolve/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_flow_eval_resolve_budget_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::flow_eval_resolve::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::flow_eval_tick;
use semio_s_artifact_procedural_generation3d::preview_eval;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️flow-eval-resolve/🧪️tests/🔬️budget/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_cycle_lod_mode_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::cycle_lod_mode::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{next_lod_mode, Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔁️cycle-lod-mode/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_delete_selection_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::delete_selection::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{commit_host_snapshot, with_host};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❌️delete-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_rotate_selection_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::rotate_selection::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{gumball_widget_json, with_host};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️rotate-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_remove_widget_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::remove_widget::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{commit_host_snapshot, with_host};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-widget/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_add_widget_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::add_widget::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{commit_host_snapshot, with_host};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️add-widget/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_set_contributions_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::set_contributions::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_flow_tessellate_resolve_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::flow_tessellate_resolve::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::flow_eval_tick;
use semio_s_artifact_procedural_generation3d::preview_eval;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔺️flow-tessellate-resolve/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_reorganize_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::reorganize::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{commit_host_snapshot, with_host};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️reorganize/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_flow_eval_release_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::flow_eval_release::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::preview_eval;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔓️flow-eval-release/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_set_active_example_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::set_active_example::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::{generation3d_host_snapshot_operations, generation_mutation_to_generation3d, Generation3dMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{empty_generation3d_snapshot, example_snapshot, is_generation3d_example_id};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_patch_flow_widgets_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::patch_flow_widgets::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::with_host;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::{generation3d_host_snapshot_operations, Generation3dMutation};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-flow-widgets/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_add_generation_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::add_generation::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::generation::generation_command_result;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-generation/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_navigate_graph_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::navigate_graph::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️navigate-graph/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_set_lod_mode_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::set_lod_mode::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔬️set-lod-mode/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_knife_mesh_selection_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::knife_mesh_selection::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::{config::{Generation3dConfig, Generation3dConfigMutation}, selection::{component_group, DOMAIN}};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{commit_host_snapshot, with_host, mutations::text::Generation3dMutation};
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔪️knife-mesh-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_toggle_sun_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::toggle_sun::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌞️toggle-sun/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_cycle_show_mode_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::cycle_show_mode::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{next_show_mode, Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔁️cycle-show-mode/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_commands_translate_selection_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::commands::translate_selection::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_generation3d::Generation3dSnapshot;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{gumball_widget_json, with_host};
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️translate-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "."]
mod editor_generation3d_selection_tests_laws {
use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::selection::*;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = "🧪️tests/🔬️flow-operators/🦀️.rs"]
pub(crate) mod flow_operators;
