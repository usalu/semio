//! 🧪️ Unit 1 of the facade test host of `builder`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework_plugin_decl_registry::ArtifactInferenceServiceMetadata;
use semio_framework_plugin_decl_registry::FlowExtensionDeclaration;
use semio_framework_plugin_decl_registry::HostMediaHandlerDeclaration;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_decl_app::ArtifactApp;

#[cfg(test)]
#[path = "../../../../../🏗️builder/🧪️tests/🔗️dependency-contribution/🦀️.rs"]
mod dependency_fixture;
#[cfg(test)]
#[path = "../../../../../🏗️builder/🧪️tests/🔬️plugin-builder-dependency/🦀️.rs"]
mod plugin_builder_dependency_tests;
#[cfg(test)]
#[path = "../../../../../🏗️builder/🧪️tests/🔬️schema-stamping/🦀️.rs"]
mod schema_stamping_tests;
#[cfg(test)]
#[path = "../../../../../🏗️builder/🧪️tests/🧾️document-authority/🦀️.rs"]
mod schema_document_authority_tests;
