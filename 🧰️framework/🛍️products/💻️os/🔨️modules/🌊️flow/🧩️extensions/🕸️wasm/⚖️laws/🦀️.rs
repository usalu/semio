//! ⚖️ Flow extension laws: the SDK request, raw recipient and invocation wire suite over the whole extension evaluation producer.

use semio_framework_test_fixtures::fixture_text;

pub use semio_framework_os_flow_extensions::*;

#[cfg(test)] use neural_engine::{ColdOwner, Dictionary, Registry};
#[cfg(test)] use semio_framework_value::DslValue;
#[cfg(test)] use semio_framework_value::ValueError;
#[cfg(test)] use semio_framework_value::retirement::RetireOwned;
#[cfg(test)] use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(test)] use std::collections::BTreeMap;
#[cfg(test)] use semio_framework_os_flow_extensions::external_completion::EvaluationExternalCompletion;

/// 🧾️ Reads one file below the extension node at test time instead of compiling it into the suite; the text lives for the whole test process.
#[cfg(test)]

// #region 🔖️Tests
#[cfg(test)]
#[path = "../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
