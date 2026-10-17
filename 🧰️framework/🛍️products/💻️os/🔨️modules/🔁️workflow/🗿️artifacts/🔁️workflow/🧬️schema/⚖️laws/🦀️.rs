//! ⚖️ Workflow laws: the schema, snapshot codec and mutation leaf suites over the whole workflow artifact.

use semio_framework_test_fixtures::fixture_path;

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use semio_framework_artifact_workflow_workflow_schema::*;
#[cfg(test)]
use semio_framework_artifact_workflow_workflow_graph::*;
#[cfg(test)]
use semio_framework_artifact_workflow_workflow_snapshot::*;
#[cfg(test)] use semio_framework::{MediaClass, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, MediaWireFormat, PortMultiplicity};
#[cfg(test)] use semio_framework_ui_locale::Locale;
#[cfg(test)] use semio_framework_ui_locale::Terminology;
#[cfg(test)] use std::collections::HashSet;

/// 🧾️ Reads one file below the workflow node at test time instead of compiling it into the suite; the text lives for the whole test process.
#[cfg(test)]
fn read_node(path: &str) -> &'static str {
    let file = fixture_path!("../../../..").join(path);
    Box::leak(std::fs::read_to_string(&file).unwrap_or_else(|error| panic!("{}: {error}", file.display())).into_boxed_str())
}

#[cfg(test)]
#[path = "../../🧪️tests/🔁️workflow/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_tests;
#[cfg(test)]
#[path = "../🧬️mutations/↔️move-node/🧪️tests/🔬️unit/🦀️.rs"]
mod move_node_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/⛔️unbind-output/🧪️tests/🔬️unit/🦀️.rs"]
mod unbind_output_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/✂️disconnect-edge/🧪️tests/🔬️unit/🦀️.rs"]
mod disconnect_edge_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/✏️rename-node/🧪️tests/🔬️unit/🦀️.rs"]
mod rename_node_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/➕️add-node/🧪️tests/🔬️unit/🦀️.rs"]
mod add_node_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/📍️set-node-positions/🧪️tests/🔬️unit/🦀️.rs"]
mod set_node_positions_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/📤bind-output/🧪️tests/🔬️unit/🦀️.rs"]
mod bind_output_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/📥add-input/🧪️tests/🔬️unit/🦀️.rs"]
mod add_input_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/🔄update-node-ports/🧪️tests/🔬️unit/🦀️.rs"]
mod update_node_ports_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/🔌bind-input/🧪️tests/🔬️unit/🦀️.rs"]
mod bind_input_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/🔒bind-parameter-field/🧪️tests/🔬️unit/🦀️.rs"]
mod bind_parameter_field_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/🔓unbind-parameter-field/🧪️tests/🔬️unit/🦀️.rs"]
mod unbind_parameter_field_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/🔗connect-ports/🧪️tests/🔬️unit/🦀️.rs"]
mod connect_ports_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/🗑️remove-node/🧪️tests/🔬️unit/🦀️.rs"]
mod remove_node_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/🚚️move-nodes/🧪️tests/🔬️unit/🦀️.rs"]
mod move_nodes_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/🚪unbind-input/🧪️tests/🔬️unit/🦀️.rs"]
mod unbind_input_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/🚮remove-input/🧪️tests/🔬️unit/🦀️.rs"]
mod remove_input_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/🧩add-parameter/🧪️tests/🔬️unit/🦀️.rs"]
mod add_parameter_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/🧹remove-parameter/🧪️tests/🔬️unit/🦀️.rs"]
mod remove_parameter_leaf_tests;
#[cfg(test)]
#[path = "../🧬️mutations/🩹change-parameter/🧪️tests/🔬️unit/🦀️.rs"]
mod change_parameter_leaf_tests;
