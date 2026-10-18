//! 📜️ Artifact declarations: the declaration builder, document codecs, capability sources, host media handlers and flow extensions.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[doc(hidden)]
#[path = "📇️registration/🦀️.rs"]
pub mod artifact_registration;
#[doc(hidden)]
#[path = "🌳️tree/🦀️.rs"]
pub mod declarations;
#[doc(hidden)]
#[path = "🛬️lifted/💡️inference/🦀️.rs"]
pub mod artifact_inference;
#[allow(unused_imports)]
pub use artifact_inference::*;
#[doc(hidden)]
#[path = "🛬️lifted/🔌️program/🦀️.rs"]
pub mod plugin_program;
use semio_framework::kernel::CapabilityRequirement;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
/// 🚪️ `io`'s `ArtifactRef`/`ArtifactKindId` vocabulary is not glob-re-exported at the
/// `semio-framework-os-kernel` crate root (deliberate — see that crate's own glue.rs comment on
/// the `io` mount), so it is named through the `store::io::` path everywhere in this file,
/// exactly like the sibling `🎞️gif` migration leaf (`semio_framework_artifact_reference::ArtifactDialect`) already does.
use semio_framework_artifact_reference::ArtifactKindId;
use pack::snapshot::ArtifactPack;
/// 🗂️ `OsMediaCapability`/`ArtifactKindSpec` now live in `semio-framework-core` (both this crate and
/// `semio-framework-os` already depend on it) the same way the legacy format enum used to — re-exported
/// here verbatim instead of duplicated, so `AppBuilder::artifact_kind(...)` and
/// `semio_framework_os`'s artifact catalog registry share one definition.
use semio_framework::ArtifactKindSpec;
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use {semio_framework_artifact_reference::{ArtifactDialect, Dialect}, semio_framework_os_kernel::io::{ComposerEntry, SubsetValidatorEntry}};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_identity::ArtifactCapabilityKind;
use semio_framework_plugin_identity::ArtifactDefinition;
use semio_framework_plugin_ownership::sdk_fault::ArtifactDefinitionError;
use semio_framework_plugin_identity::ArtifactDefinitionRegistry;
use semio_framework_plugin_identity::ArtifactIdentity;
use semio_framework_plugin_identity::ArtifactIdentityClaim;
use semio_framework_plugin_identity::ArtifactIdentityNamespace;
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
use semio_framework_plugin_identity::preflight_artifact_identity;
use semio_framework_plugin_identity::require_declared_capability_or_record;
use crate::artifact_inference::ArtifactInferenceService;

#[path = "📋️declaration-and-builder-states/🦀️.rs"]
mod declaration_and_builder_states;
pub use declaration_and_builder_states::*;

#[path = "🏗️ready-declaration-builder/🦀️.rs"]
mod ready_declaration_builder;

#[path = "🧮️codec-spec-and-capability-sources/🦀️.rs"]
mod codec_spec_and_capability_sources;
pub use codec_spec_and_capability_sources::*;

#[path = "🎬️host-media-handlers/🦀️.rs"]
mod host_media_handlers;
pub use host_media_handlers::*;

#[path = "🌊️flow-extensions/🦀️.rs"]
mod flow_extensions;
pub use flow_extensions::*;
