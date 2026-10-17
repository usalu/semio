//! 🪪️ Test host of `artifact_registration`: runs `🪪️codec-binding` in the facade, where the names it uses are in scope.

use semio_framework_plugin_decl_registry::artifact_registration::ArtifactRegistrationPlan;
use semio_framework_plugin_decl_registry::artifact_registration::PluginRuntimeRegistry;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeSet;
use pack::snapshot::ArtifactPack;
#[cfg(test)]
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_registry::ArtifactDeclaration;
use semio_framework_plugin_decl_registry::DocumentCodecSpec;
use semio_framework_plugin_identity::ArtifactDefinitionRegistry;
#[cfg(test)]
use semio_framework_plugin_identity::ArtifactDefinition;
#[cfg(test)]
use semio_framework_plugin_identity::ArtifactIdentity;
#[cfg(test)]
use semio_framework_plugin_decl_surface::EditorApp;
use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
#[allow(unused_imports)]
use semio_framework_plugin_decl_registry::artifact_registration::*;
include!("../../../../🧪️tests/🪪️codec-binding/🦀️.rs");
