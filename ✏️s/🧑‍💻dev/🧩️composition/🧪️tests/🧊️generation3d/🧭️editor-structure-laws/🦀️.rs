//! 🧭️ Editor structure laws: fold, interaction scope, generate interactions, example switch, tick addressing, work capacity, mode panels and slider values.
#![allow(dead_code, unused_imports)]

include!("../🦀️.rs");

#[path = ".."]
mod editor_domain {
include!("../📥️editor-imports/🦀️.rs");
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod editor_laws;
}

#[path = ".."]
mod editor_generation3d_fold_contract_laws {
include!("../📥️editor-imports/🦀️.rs");
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️fold-contract/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_interaction_scope_laws {
include!("../📥️editor-imports/🦀️.rs");
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🕹️interaction-scope/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_generate_interactions_laws {
include!("../📥️editor-imports/🦀️.rs");
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️generate-interactions/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_example_switch_laws {
include!("../📥️editor-imports/🦀️.rs");
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️example-switch/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_tick_addressing_laws {
include!("../📥️editor-imports/🦀️.rs");
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️tick-addressing/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_work_capacity_laws {
include!("../📥️editor-imports/🦀️.rs");
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_mode_panels_laws {
include!("../📥️editor-imports/🦀️.rs");
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mode-panels/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_slider_values_laws {
include!("../📥️editor-imports/🦀️.rs");
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🎚️slider-values/🦀️.rs"]
mod laws;
}

#[path = ".."]
mod editor_generation3d_commands_set_widget_input_tests_laws {
use super::*;
use semio_s_artifact_procedural_3d_set_widget_input::*;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_model::commit_host_snapshot;
use semio_s_artifact_procedural_3d_model::with_host;
use semio_s_artifact_procedural_3d_mutations::text::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧪️tests/🔬️unit/🦀️.rs"]
mod laws;
}
