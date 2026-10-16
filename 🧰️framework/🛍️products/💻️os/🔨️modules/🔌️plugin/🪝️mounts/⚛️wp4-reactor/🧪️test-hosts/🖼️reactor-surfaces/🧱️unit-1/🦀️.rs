//! 🧪️ Unit 1 of the facade test host of `reactor-surfaces`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use crate::plugin_runtime;
use semio_framework::FaultFrom;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

#[cfg(test)]
#[path = "../../../../../⚛️reactor/📸️checkpoint/🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
