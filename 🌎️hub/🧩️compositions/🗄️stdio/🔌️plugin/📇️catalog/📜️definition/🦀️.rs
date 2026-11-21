//! 📜️ The Stdio artifact catalog commitment derived from the composition's selected schemas.

use semio_plugin::PluginAssemblyError;
pub use semio_hub_stdio_catalog_projection::{native_artifact_catalog_dependency, validate_native_artifact_catalog_dependency};

/// 🧮️ Projects the exact Stdio catalog commitment without loading artifact implementations.
pub fn native_artifact_catalog_contribution() -> Result<semio_framework::TopicContribution, PluginAssemblyError> {
    static CATALOG: std::sync::OnceLock<Result<semio_framework::TopicContribution, PluginAssemblyError>> = std::sync::OnceLock::new();
    CATALOG.get_or_init(|| semio_hub_stdio_catalog_projection::artifact_catalog_contribution_from_bundle(include_bytes!("../../../🤖️generated/📜️catalog/🗜️.bin"))).clone()
}
