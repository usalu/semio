//! 🍌️ Companion test crate tests-05 of the plugin SDK partition: hosts the test roots `fixture_projection_retirement_tests`, `reactor_turn_test_hosts`, `plugin_runtime_plugin_builder_contract_support_1`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

#![recursion_limit = "256"]

extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as pack;
extern crate semio_os_kernel as dsl;

#[cfg(test)]

#[path = "../../🧪️tests/🌲️fixture-projection/🦀️.rs"]
pub mod fixture_projection_retirement_tests;
#[path = "../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"]
pub mod plugin_runtime_plugin_builder_contract_support_1;

use semio_os_kernel::os_store::{self as semio_os_store_operation_wire, codec_registry as semio_os_store_codec_registry};
use semio_plugin_store_lifecycle::store_lifecycle::ArtifactOwnedDisposer;
use semio_plugin_window_config::{WindowConfigOwner, WindowConfigApplyEdit, WindowConfigApplyMutation, WindowConfigNativePairReceiving, admit_window_config_native_pair, window_config_native_pair_demands, bounded_window_config_preparation_factory, bounded_window_config_preparation_source_birth, bounded_window_config_store_disposer, bounded_window_config_store_disposer_birth, bounded_window_config_store_owners};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
use std::sync::Arc;

#[path = "../../🪟️window/🎚️config/🗄️store-owners/🌱️initialization/🎟️birth/🧪️tests/🧸️fixture/🦀️.rs"]
pub mod publication_window_owner;

#[path = "../../🧪️tests/🧬️mutation-fixtures-job-close/🦀️.rs"]
pub(crate) mod job_close_fixture;
