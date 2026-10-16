//! 🎯️ Test host of `app_emission`: runs `🔬️app-app-commands` in the facade, where the names it uses are in scope.
//! 🧪️ Minimal serde-free end-to-end proof for `app_commands!`. The fake payloads deliberately do not
//! implement `serde::Serialize`/`serde::Deserialize`, so restoring those derives on the generated enum
//! is a compile-time regression. The fixture exercises enum construction, `command_id()`, `dispatch()`,
//! and the `dsl::DslOps` wire round trip (`OpText`/`OpBinary`) via the same `store::test_support` helper every
//! real `*_protocol` crate's own Command enum tests use (see e.g. `flow_protocol`'s
//! `flow_command_text_binary_round_trips_document_mutating_variants`).

use super::child_emit_preparation::{ChildEmitPreparation, ChildEmitPreparationStep, OwnedChildEmit};
use super::window_config::WindowConfigMutation;
use super::window_transient::WindowTransientMutation;
use semio_framework::kernel::UiDirtyScope;
use semio_framework::{kernel::{AppEvent, Effect, SchemaId}, Fault, FaultCode, FaultOrigin, ViewModel};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::Serialize;
use std::future::Future;
#[allow(unused_imports)]
use std::pin::Pin;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::ManuallyDrop;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use crate::app_contract::ArtifactApp;
use crate::app_views::NoConfigMutation;
use crate::app_views::NoDraftMutation;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
#[allow(unused_imports)]
use crate::app_emission::*;
include!("../../../../🧪️tests/🔬️app-app-commands/🦀️.rs");
