//! 🎮️ Editor command laws: one law wrapper per flow, widget and graph editor command.
#![allow(dead_code, unused_imports)]

include!("../🦀️.rs");

#[path = ".."]
mod editor_domain {
include!("../📥️editor-imports/🦀️.rs");
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod editor_laws;
}

#[path = ".."]
mod editor_generation3d_commands_flow_eval_tick_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_flow_eval_tick::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_preview_eval as preview_eval;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_node_graph_edit_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_node_graph_edit::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::commit_host_snapshot;
use semio_s_artifact_procedural_3d_model::with_host;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_flow_tessellate_cancel_resolve_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::flow_tessellate_cancel_resolve::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_preview_eval as preview_eval;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧯️flow-tessellate-cancel-resolve/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_flow_eval_resolve_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::flow_eval_resolve::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_flow_eval_tick as flow_eval_tick;
use semio_s_artifact_procedural_3d_preview_eval as preview_eval;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️flow-eval-resolve/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_flow_eval_resolve_budget_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::flow_eval_resolve::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_flow_eval_tick as flow_eval_tick;
use semio_s_artifact_procedural_3d_preview_eval as preview_eval;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️flow-eval-resolve/🧪️tests/🔬️budget/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_remove_widget_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::remove_widget::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::commit_host_snapshot;
use semio_s_artifact_procedural_3d_model::with_host;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-widget/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_add_widget_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_add_widget::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::commit_host_snapshot;
use semio_s_artifact_procedural_3d_model::with_host;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️add-widget/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_set_contributions_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_set_contributions::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_flow_tessellate_resolve_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::flow_tessellate_resolve::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_flow_eval_tick as flow_eval_tick;
use semio_s_artifact_procedural_3d_preview_eval as preview_eval;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔺️flow-tessellate-resolve/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_reorganize_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::reorganize::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::commit_host_snapshot;
use semio_s_artifact_procedural_3d_model::with_host;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️reorganize/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_flow_eval_release_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::flow_eval_release::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_preview_eval as preview_eval;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔓️flow-eval-release/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_set_active_example_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::set_active_example::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::generation3d_host_snapshot_operations;
use semio_s_artifact_procedural_3d_io_text::mutations::generation_mutation_to_generation3d;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::empty_generation3d_snapshot;
use semio_s_artifact_procedural_3d_model::example_snapshot;
use semio_s_artifact_procedural_3d_model::is_generation3d_example_id;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_patch_flow_widgets_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::patch_flow_widgets::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_model::with_host;
use semio_s_artifact_procedural_3d_io_text::mutations::generation3d_host_snapshot_operations;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-flow-widgets/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_add_generation_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::add_generation::*;
use semio_s_artifact_procedural_3d_editor_commands::generation::generation_command_result;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-generation/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_navigate_graph_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_navigate_graph::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️navigate-graph/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}
