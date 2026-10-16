//! 🪪️ Test host of `artifact_registration`: runs `🪪️codec-binding` in the facade, where the names it uses are in scope.

use semio_framework::{ActionArgDef, PluginManifest};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_artifact_reference::ArtifactKindId;
use pack::snapshot::ArtifactPack;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_os_kernel::io::{ComposerEntry, SubsetValidatorEntry};
#[cfg(test)]
use crate::app_contract::ArtifactApp;
#[cfg(test)]
use crate::declarations;
use crate::artifact_declaration::ArtifactDeclaration;
use crate::artifact_declaration::DocumentCodecSpec;
use crate::artifact_declaration::FlowExtensionDeclaration;
use crate::artifact_declaration::FlowExtensionRegistry;
use crate::artifact_declaration::HostMediaHandlerDeclaration;
use crate::artifact_declaration::HostMediaHandlerRegistry;
use crate::artifact_identity::ArtifactDefinitionRegistry;
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
use crate::artifact_inference::ArtifactInferenceService;
use crate::artifact_inference::ArtifactInferenceServiceKey;
use crate::artifact_inference::ArtifactInferenceServiceMetadata;
use crate::artifact_inference::ArtifactInferenceServiceRegistry;
use semio_framework_plugin_ownership::store_lifecycle::OwnerMutationRoster;
#[cfg(test)]
use crate::artifact_identity::ArtifactDefinition;
#[cfg(test)]
use crate::artifact_identity::ArtifactIdentity;
#[cfg(test)]
use crate::surface_contract::EditorApp;
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};
#[allow(unused_imports)]
use crate::artifact_registration::*;
include!("../../../../🧪️tests/🪪️codec-binding/🦀️.rs");
