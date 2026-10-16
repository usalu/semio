//! 📡️ Test host of `plugin_runtime`: runs `🔬️plugin-runtime-contributed-mutation-wire` in the facade, where the names it uses are in scope.

use crate::app_emission::ActionMeta;
use crate::plugin_program::{AppInstance, Plugin, PluginProgram};
use crate::media_contract::ArtifactMediaExportHandle;
use crate::media_contract::ArtifactMediaExportPoll;
use crate::media_contract::ArtifactMediaExportResult;
use crate::tool_jobs::{EmitWire, TransactionProposalDraft, TypedOperationLeftover, TypedOperationResultPage, TypedOperationResultToken};
use crate::app_contract::{EphemeralSnapshot, MediaArtifact, MediaArtifactDescriptor, MediaArtifactError, MediaConsumption};
use crate::plugin_app::PluginApp;
use semio_framework::MediaError;
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
use crate::peer_roster::PresenceRosterAdmission;
use crate::{ArtifactApp, WindowConfigPack, MountedOwnerPolicyV1, MountedOwnerTurnV1, MountedOwnerIdentityV1, MountedOwnerPhaseV1};
use semio_framework_value::{DslValue,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework::manifest::{ActionInvocation as ManifestActionInvocation, CommandAddress as ManifestCommandAddress, CommandInvocation as ManifestCommandInvocation, CommandOwnerAddress as ManifestCommandOwnerAddress};
use semio_framework::{
    kernel::{ActivationEvent, CapabilityRequest, CapabilityRequirement, Effect, InvocationResult, MessageEndpoint, QuotaSchema},
    AssetDeclaration, ExecutionMode, ExtensionPointDeclaration, Fault, FaultCode, FaultFrom, FaultOrigin, PluginManifest, TopicContribution, ViewModel,
};
use semio_framework_ui_contract as ui_contract;
use semio_framework_ui_runtime::ComponentTree;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::{Cell, RefCell};
use std::mem::{size_of_val};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, Ordering};
use semio_framework_ui_layout::layout::{ContextMenuPoint, ContextMenuRequest, ContextMenuResponse, ContextMenuSurfaceTarget, UiMenuRef};
use crate::reserved_job_outcomes::CleanupTurn;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
#[allow(unused_imports)]
use crate::plugin_runtime::*;
include!("../../../../🧪️tests/🔬️plugin-runtime-contributed-mutation-wire/🦀️.rs");
