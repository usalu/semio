//! 🧪️ Unit 3 of the facade test host of `extension`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework_plugin_plugin_program::ARTIFACT_INFERENCE_WIRE_VERSION;
use semio_framework_plugin_decl_registry::ArtifactInferenceExecution;
use semio_framework_plugin_decl_registry::ArtifactInferenceExecutionError;
use semio_framework_plugin_decl_registry::ArtifactInferenceExecutionRequest;
use semio_framework_plugin_decl_registry::artifact_inference::ArtifactInferenceService;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_plugin_decl_registry::WireArtifactInferenceBudget;
use semio_framework_plugin_decl_registry::WireArtifactInferenceCacheMode;
use semio_framework_plugin_plugin_program::WireArtifactInferenceRequest;
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_pack_json::JsonWriteSource;
use semio_framework_value::retirement::RetireOwned;

#[cfg(test)]
#[path="../../../../../💡️inference/🚪️gateway/🧪️tests/🦀️.rs"]
mod tests;
