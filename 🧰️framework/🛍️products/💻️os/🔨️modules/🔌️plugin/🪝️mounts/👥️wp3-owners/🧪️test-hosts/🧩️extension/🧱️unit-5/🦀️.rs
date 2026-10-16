//! 🧪️ Unit 5 of the facade test host of `extension`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework_plugin_extension::extension_runtime_lift::ExtensionBundle;
use semio_framework::Fault;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::MetadataOwner;

#[cfg(test)]
#[path="../../../../../🧩️extension/🧪️tests/📨️invoke/🦀️.rs"]
mod original_invocation_tests;
