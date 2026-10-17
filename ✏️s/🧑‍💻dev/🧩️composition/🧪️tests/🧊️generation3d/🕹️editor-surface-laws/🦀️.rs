//! 🕹️ Editor surface laws: the command-surface and window-and-menu law groups plus the selection and transform command wrappers.
#![allow(dead_code, unused_imports)]

include!("../🦀️.rs");

#[path = ".."]
mod editor_domain {
include!("../📥️editor-imports/🦀️.rs");
#[path = "🕹️editor-surface-laws/📚️editor-laws/🦀️.rs"]
pub(crate) mod editor_laws;
}

#[path = ".."]
mod editor_generation3d_commands_scale_selection_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_scale_selection::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_model::gumball_widget_json;
use semio_s_artifact_procedural_3d_model::with_host;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📏️scale-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_cycle_lod_mode_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::cycle_lod_mode::*;
use semio_s_artifact_procedural_3d_editor_config::next_lod_mode;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔁️cycle-lod-mode/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_delete_selection_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_delete_selection::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::commit_host_snapshot;
use semio_s_artifact_procedural_3d_model::with_host;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❌️delete-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_rotate_selection_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_rotate_selection::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_model::gumball_widget_json;
use semio_s_artifact_procedural_3d_model::with_host;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️rotate-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_set_lod_mode_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::set_lod_mode::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔬️set-lod-mode/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_knife_mesh_selection_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::knife_mesh_selection::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_editor_selection::component_group;
use semio_s_artifact_procedural_3d_editor_selection::DOMAIN;
use semio_s_artifact_procedural_3d_model::commit_host_snapshot;
use semio_s_artifact_procedural_3d_model::with_host;
use semio_s_artifact_procedural_3d_mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔪️knife-mesh-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_toggle_sun_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::toggle_sun::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌞️toggle-sun/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_cycle_show_mode_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::cycle_show_mode::*;
use semio_s_artifact_procedural_3d_editor_config::next_show_mode;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔁️cycle-show-mode/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_translate_selection_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_commands::translate_selection::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_io_text::mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_model::gumball_widget_json;
use semio_s_artifact_procedural_3d_model::with_host;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️translate-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_selection_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_editor_selection::*;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}
