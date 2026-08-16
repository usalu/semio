//! 🛂️ Schema-derived stdio manifest assembly.

use semio_framework_plugin::io::FormatDescriptor;
use semio_framework_plugin::{ArtifactDefinition, PluginAssemblyError};

/// 🧾️ Returns the one schema-owned definition for every stdio artifact.
pub fn stdio_artifact_definitions() -> Result<Vec<ArtifactDefinition>, PluginAssemblyError> {
    crate::registry::artifact_definitions()
}

/// 🗂️ Returns runtime descriptors derived from schema-owned representations.
pub fn stdio_format_descriptors() -> Result<Vec<FormatDescriptor>, PluginAssemblyError> {
    crate::registry::format_descriptors()
}
