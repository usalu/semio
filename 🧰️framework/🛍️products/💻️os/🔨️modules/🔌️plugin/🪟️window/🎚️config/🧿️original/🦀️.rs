//! 🧿️ Original Window authority capture and the shared preview epoch, with the retained load, preview and capture laws that need the whole window-config surface.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as pack;

pub use semio_framework_plugin_window_config::*;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin, ViewModel};
use std::any::Any;
use semio_framework_value::{FactoryBoxedValue,FactoryBoxedPublication,retirement::controlled::ControlledRetirement};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_plugin_window_config_mutation as window_mutation;
use semio_framework_plugin_window_config_registry as registry;
use registry::WindowRegistry;

#[path="../🎟️capture/🦀️.rs"]
mod original_capture;
pub use original_capture::{OriginalWindowConfigCapture,OriginalWindowAddressCopy};

#[path="../🔒️epoch/🦀️.rs"]
mod preview_epoch;
pub use preview_epoch::OriginalWindowPreviewEpoch;

#[cfg(test)]
#[global_allocator]
static ORIGINAL_WINDOW_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[cfg(test)]
#[path="../🗄️store-owners/🌱️initialization/🎟️birth/🧪️tests/🧸️fixture/🦀️.rs"]
mod fixture;
#[cfg(test)]
mod original_genesis_tests{pub use super::fixture::{Owner,State,Mutation,grant,fixture};}
#[cfg(test)]
#[path="../🧪️tests/📷️camera-owner/🦀️.rs"]
mod camera_owner;
#[cfg(test)]
#[path="🧬️preview/🦀️.rs"]
mod semantic_preview;
#[cfg(test)]
#[path="📥️retained/🦀️.rs"]
mod retained;
