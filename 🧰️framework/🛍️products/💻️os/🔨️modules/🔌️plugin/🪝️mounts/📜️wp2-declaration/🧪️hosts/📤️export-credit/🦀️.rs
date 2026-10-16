//! 📤️ Test host of `media_contract`: runs `🔬️app-artifact-media-export-credit` in the facade, where the names it uses are in scope.

use semio_framework::Fault;
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use semio_framework::Media;
use semio_framework::MediaError;
use semio_framework::MediaPayload;
use semio_framework::MediaType;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use crate::app_contract::ArtifactApp;
use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedQueue;
use crate::history_view::HistoryView;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactSnapshotCloseLease;
#[allow(unused_imports)]
use crate::media_contract::*;
include!("../../../../🧪️tests/🔬️app-artifact-media-export-credit/🦀️.rs");
