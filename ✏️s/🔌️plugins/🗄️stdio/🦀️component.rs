//! 🔌️ Schema-owned stdio library plugin assembly.

use semio_framework_plugin::{Plugin, PluginAssemblyError};

/// 🧾️ Builds all stdio definitions before the typed library assembly boundary.
pub fn plugin() -> Result<Plugin, PluginAssemblyError> {
    let mut builder = Plugin::builder("stdio").label("Stdio").version("0.1.0");
    for assembly in crate::registry::artifact_assemblies()? {
        builder = match assembly {
            crate::registry::ArtifactAssembly::Definition(definition) => builder.artifact_definition(definition),
            crate::registry::ArtifactAssembly::Runtime(declaration) => builder.artifact(declaration),
        };
    }
    builder.try_library()
}
