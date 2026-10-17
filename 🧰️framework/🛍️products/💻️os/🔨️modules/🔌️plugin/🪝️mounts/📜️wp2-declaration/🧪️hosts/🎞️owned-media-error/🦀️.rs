//! 🎞️ Test host of `app_contract`: runs `🔬️app-owned-media-error` in the facade, where the names it uses are in scope.

use semio_framework_value::DslValue;
use semio_framework::MediaForm;
use semio_framework_ui_contract::*;
use semio_framework_value_derive::{FromValue, ToValue};
use pack::snapshot::ArtifactPack;
use semio_framework::MediaWireFormat;
use semio_framework::{Media, MediaError, MediaFingerprint, MediaPayload};
use semio_framework::{MediaClass, MediaType};
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[allow(unused_imports)]
use semio_framework_plugin_document_media::MediaArtifact;
#[allow(unused_imports)]
use semio_framework_plugin_decl_app::{MediaArtifactDescriptor, MediaArtifactError, media_type};
#[allow(unused_imports)]
#[cfg(test)]
use semio_framework_plugin_document_media::{receive_media_artifact_fault, receive_media_output};
include!("../../../../🧪️tests/🔬️app-owned-media-error/🦀️.rs");
