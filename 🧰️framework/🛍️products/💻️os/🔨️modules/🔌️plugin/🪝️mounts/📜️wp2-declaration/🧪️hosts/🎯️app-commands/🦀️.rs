//! 🎯️ Test host of `app_emission`: runs `🔬️app-app-commands` in the facade, where the names it uses are in scope.
//! 🧪️ Minimal serde-free end-to-end proof for `app_commands!`. The fake payloads deliberately do not
//! implement `serde::Serialize`/`serde::Deserialize`, so restoring those derives on the generated enum
//! is a compile-time regression. The fixture exercises enum construction, `command_id()`, `dispatch()`,
//! and the `dsl::DslOps` wire round trip (`OpText`/`OpBinary`) via the same `store::test_support` helper every
//! real `*_protocol` crate's own Command enum tests use (see e.g. `flow_protocol`'s
//! `flow_command_text_binary_round_trips_document_mutating_variants`).

use semio_framework::Fault;
use semio_framework_value_derive::{FromValue, ToValue};
use std::future::Future;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_views::NoConfigMutation;
#[allow(unused_imports)]
use semio_framework_plugin_decl_emission::{Emit, ToolCommandCatalog};
include!("../../../../🧪️tests/🔬️app-app-commands/🦀️.rs");
