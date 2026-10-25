//! ⚖️ Flow VCS laws: the retained VCS operation, canonical history and lifecycle suite over the whole VCS producer.

use semio_framework_test_fixtures::fixture_text;

pub use semio_framework_os_flow_vcs::*;

#[cfg(test)] use neural_engine as neural;
#[cfg(test)] use neural::{Neuron, Synapse};
#[cfg(test)] use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
#[cfg(test)]
use semio_framework_artifact_flow_flow::*;
#[cfg(test)] use semio_framework_os_flow_vcs::history_identity;
#[cfg(test)] use semio_framework_os_flow_vcs::original_retirement::admit_turn;

/// 🧾️ Reads one file below the VCS node at test time instead of compiling it into the suite; the text lives for the whole test process.
#[cfg(test)]
fn read_node(path: &str) -> &'static str {
    fixture_text!(format!("../../../{path}"))
}

// #region 🔖️Laws
#[cfg(test)]
#[path = "../🧪️tests/🔬️flow-vcs/🦀️.rs"]
mod flow_vcs_tests;
// #endregion 🔖️Laws
