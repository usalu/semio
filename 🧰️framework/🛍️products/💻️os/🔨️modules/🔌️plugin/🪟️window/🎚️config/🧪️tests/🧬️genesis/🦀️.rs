//! 🧪️ Original Window genesis and catalog laws run as their own test target so they do not count against the library source budget.
extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as pack;

pub use semio_plugin_window_config::*;
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin, ViewModel};
use std::any::Any;
use semio_framework_value::{FactoryBoxedValue,FactoryBoxedPublication,retirement::controlled::ControlledRetirement};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_plugin_window_config_mutation as window_mutation;

#[path="../../🗄️store-owners/🌱️initialization/🎟️birth/🧪️tests/🦀️.rs"]
mod original_genesis_tests;
#[path="../../🏠️catalog/🧪️tests/🦀️.rs"]
mod original_catalog_tests;
