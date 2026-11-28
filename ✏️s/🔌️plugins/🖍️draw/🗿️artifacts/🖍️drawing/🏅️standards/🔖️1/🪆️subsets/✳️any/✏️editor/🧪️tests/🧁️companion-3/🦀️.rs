//! 🧁️ Companion test crate of `semio-s-artifact-draw-drawing-editor`: the unit laws over selection, nodes, commands, envelopes, pointers and the clipboard runtime outside the shared unit context. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]
#![recursion_limit = "256"]

use semio_framework_value_testing_allocator as _;

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

#[path = "../🔬️unit-2/🦀️.rs"]
pub mod unit_tests;
