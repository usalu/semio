//! 🪟️ Editor and viewer surfaces: `ArtifactEditor`, `ArtifactViewer`, their app wrappers, builders, declarations and the artifact codec table.
#[cfg(test)] use crate::document_ingress::ArtifactStoreConstructorPhase;
#[cfg(test)] use crate::document_ingress::ArtifactStoreConstructorKind;
#[cfg(test)] use crate::__semio_dispatch_PluginApp;
use crate::MountedOwnerPhaseV1;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::MountedOwnerTurnV1;
use super::transient_publication::bounded_transient_store_disposer;
use super::window_config::WindowConfigOwnerRegistry;
#[cfg(any(test, feature = "artifact-app-testing"))]
use super::window_config::WindowConfigPack;
use super::window_transient::WindowTransientOwnerRegistry;
use semio_framework_value::DslValue;
use protocol::{OpBinary, OpText};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::kernel::FixedCommandPage;
use semio_framework::kernel::UiDirtyScope;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::manifest::{ActionInvocation as ManifestActionInvocation, CommandInvocation as ManifestCommandInvocation};
/// 🪪️ Declarative app manifest shared by plugin builders and surface registrations.
use semio_framework::AppDefinition;
use semio_framework::{kernel::{ActorId, ClipboardError, ClipboardFragment, Effect, PastePlacement}, ActionArgDef, ActionDefinition, ActionKind, ActionRef, AppIo, CapabilityAudience, CommandDefinition, CommandGrammar, ConfigSpec, DialogDefinition, Fault, FaultCode, FaultFrom, FaultOrigin, IconName, InteractionDefinition, InteractionRef, InteractionVerb, IntroductionDefinition, MediaForm, MediaPortSpec, ModeDefinition, PanelGroup, PanelTabDefinition, ToolDefinition, ToolRef, TutorialDefinition, UtilityDefinition, UtilityRef, ViewModel, WindowKindDefinition};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::{kernel::{AppEvent, HistoryPatch, InvocationResult}, ToolRunTraceCursor};
use semio_framework_2d::compute::EngineHandles;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use semio_framework_ui_runtime::ComponentTree;
use std::collections::HashMap;
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: `AsyncTask`'s boxed `run` closure/future.
use std::future::Future;
/// 🧵️ Used only by `artifact_app_laws`' spawned-task shape, so a plain `cargo check` reports it
/// unused while `--all-targets` needs it — do not remove on the strength of that warning.
#[allow(unused_imports)]
use std::pin::Pin;
use pack::snapshot::ArtifactPack;
use store::{os_store::document_materialize::create_document_envelope, os_store::store_schema::ArtifactEnvelope, ArtifactStore, ConfigStore, MemberFactory, Mutation, MutationDiff, NoMembers, SpaceMember};
use semio_framework_ui_locale::LocalizedLabel;
use ui_wgpu::wgpu::ContextMenuItemSpec;
use ui_wgpu::wgpu::ContextMenuRequest;
use ui_wgpu::wgpu::NamedLayout;
use ui_wgpu::wgpu::WindowEngagement;
use ui_wgpu::wgpu::WindowLayout;
use ui_wgpu::wgpu::WindowMeasure;
/// 🗂️ `OsMediaCapability`/`ArtifactKindSpec` now live in `semio-framework-core` (both this crate and
/// `semio-framework-os` already depend on it) the same way the legacy format enum used to — re-exported
/// here verbatim instead of duplicated, so `AppBuilder::artifact_kind(...)` and
/// `semio_framework_os`'s artifact catalog registry share one definition.
use semio_framework::ArtifactKindSpec;
/// 🎞️ The `Media`/`MediaPayload`/`MediaFingerprint`/`MediaError` value vocabulary backing
/// `ArtifactApp::{media_ports, export_media, import_media, media_fingerprint}` — re-exported so
/// implementers never need a direct `semio-framework-core` dependency just to satisfy this trait.
use semio_framework::{Media, MediaError, MediaFingerprint, MediaPayload};
/// 🧬️ `MediaClass`/`MediaType` also live in `semio-framework-core` — re-exported so callers can build
/// `ArtifactKindSpec.media_type` and `AppBuilder::media_input(...)`/`media_output(...)` port specs
/// without a direct `semio-framework-core` dependency.
use semio_framework::{MediaClass, MediaType};
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use semio_framework_artifact_reference::{ArtifactDialect, Dialect};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_artifact_reference::SubsetId;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
use semio_framework::{surface_app_id, AppRole};
#[cfg(test)]
use crate::app_views::NoConfig;
#[cfg(test)]
use crate::app_views::NoConfigMutation;
#[cfg(test)]
use crate::app_views::NoPresence;
#[cfg(test)]
use crate::app_views::NoPresenceMutation;
#[cfg(test)]
use crate::plugin_program::Plugin;
#[cfg(test)]
use crate::window_surface::built_text_to_component_tree;
#[cfg(test)]
use crate::app_contract::MediaConsumption;
#[cfg(test)]
use crate::app_contract::MediaArtifactError;
#[cfg(test)]
use crate::app_contract::MediaArtifact;
#[cfg(test)]
use crate::media_contract::ArtifactMediaExportResult;
#[cfg(test)]
use crate::media_contract::ArtifactMediaExportPoll;
#[cfg(test)]
use crate::media_contract::ArtifactMediaExportHandle;
#[cfg(test)]
use crate::peer_roster::PresenceRosterOutcome;
#[cfg(test)]
use crate::peer_roster::PresenceRosterAdmission;
#[cfg(test)]
use crate::app_contract::EphemeralSnapshot;
#[cfg(test)]
use crate::app_contract::TransactionPrepareOutcome;
#[cfg(test)]
use crate::tool_jobs::TransactionProposalDraft;
#[cfg(test)]
use crate::tool_jobs::EmitWire;
#[cfg(test)]
use crate::reserved_jobs::FrameworkReservedCommitProgress;
#[cfg(test)]
use crate::app_emission::ActionMeta;
#[cfg(test)]
use crate::store_publication::ComposedGestureResult;
#[cfg(test)]
use crate::tool_jobs::TypedOperationCompletion;
#[cfg(test)]
use crate::tool_jobs::TypedOperationUiProgress;
#[cfg(test)]
use crate::tool_jobs::TypedOperationResultToken;
#[cfg(test)]
use crate::tool_jobs::TypedOperationResultPage;
#[cfg(test)]
use crate::tool_contract::ArtifactToolPublicContract;
#[cfg(test)]
use crate::tool_contract::ToolOwnerWitness;
use crate::app_builder::AppBuilder;
use crate::app_builder::ExampleSource;
use crate::app_builder::PanelTabSpec;
use crate::window_kits::AppActionRegistry;
use crate::app_contract::ArtifactApp;
use crate::app_contract::NATURAL_FILE_PORT;
use crate::app_contract::NaturalFileCodec;
use crate::app_contract::NaturalFileDecodeOwner;
use crate::app_contract::document_load_emit;
use crate::app_contract::natural_file_input_bytes;
use crate::app_emission::Emit;
use crate::app_emission::EphemeralEmit;
use crate::app_emission::HostEvent;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::app_testing::artifact_app_laws;
use crate::app_views::AppRenderOperationContext;
use crate::app_views::ArtifactView;
use crate::app_views::ConfigView;
use crate::app_views::DraftView;
use crate::app_views::InteractionView;
use crate::app_views::NoDraft;
use crate::app_views::NoDraftMutation;
use crate::app_views::PresenceView;
use crate::window_transient::TransientView;
use crate::artifact_io::ArtifactBuilder;
use crate::bounded_ownership::PluginLifecycleStep;
use crate::media_contract::ArtifactMediaExportJobRequest;
use crate::tool_contract::ArtifactReservedToolJob;
use crate::sdk_fault::plugin_sdk_fault;
use crate::store_lifecycle::ArtifactDisposal;
use crate::store_lifecycle::ArtifactInitializationAdmission;
use crate::app_emission::ArtifactMutationOutcome;
use crate::store_lifecycle::ArtifactSnapshotDisposer;
use crate::store_lifecycle::bounded_config_store_disposer;
use crate::store_lifecycle::bounded_document_store_disposer;
use crate::store_lifecycle::bounded_document_store_initialization_job;
use crate::tool_contract::ArtifactBoundedFirstStepProof;
use crate::tool_contract::ArtifactInstanceOperationOwner;
use crate::tool_contract::ArtifactInstanceOperationOwnerHandle;
use crate::tool_contract::ArtifactOwnedToolJobRequest;
use crate::tool_contract::ArtifactReservedToolInput;
use crate::tool_contract::ArtifactReservedToolJobRequest;
use crate::tool_contract::ArtifactToolCompletionValue;
use crate::tool_contract::ArtifactToolFactoryRegistry;
use crate::tool_contract::EmptyArtifactInstanceOperationOwner;
use crate::natural_file_import::NaturalFileImportJob;
use crate::tool_run_contract::ToolRunJob;
use crate::tool_run_contract::ToolRunJobRequest;
use crate::tool_run_contract::ToolRunRetargetableJob;
use crate::window_kits::merge_ui_values;
use crate::window_surface::UiAssemblyResult;

pub trait ArtifactEditor: Default + Send + 'static {
    /// 📬️ Borrows this editor's original typed completion retirement birth.
    fn completion_retirement_birth_bytes(value:&ArtifactToolCompletionValue<EditorApp<Self>>)->Option<usize>{super::completion_retirement::derived_completion_birth_bytes(value)}
    /// 🎟️ Retains the editor completion on every unsupported or unfunded cleanup admission.
    fn admit_completion_retirement(value:&mut Option<ArtifactToolCompletionValue<EditorApp<Self>>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{super::completion_retirement::derived_admit_completion_retirement(value,grant)}
    /// 📐️ Declares the physical authority frame needed to retain this editor's original mutation batch.
    fn owned_mutation_batch_birth_bytes() -> Option<usize> { None }

    /// 🫱️ Admits the original editor batch only when its first-party retirement authority is funded.
    fn admit_owned_mutation_batch(_values: &mut Option<Vec<Self::Mutation>>, _grant: RetainedCloneGrant) -> Result<Option<(store::MemberStoreOwnedBatch, RetainedCloneProgress)>, ValueError> { Ok(None) }

    /// 🎭️ Always `Editor` in practice — defaulted so implementors never restate it.
    const ROLE: AppRole = AppRole::Editor;
    /// 🧩️ The closed member roster every surface of this editor runs over — the ONE place a bundle
    /// reads it from, so `genesis_member_schema::<Self::Members>` resolves every dialect
    /// `genesis_child_pack` derives no matter which plugin registers the surface. `NoMembers` (the
    /// default) is right for an editor that composes no children; an editor that does state its own.
    type Members: SpaceMember + MemberFactory + Send + 'static = NoMembers;
    const DIALECT: Dialect;
    /// 🧬️ Supplies the editor's bounded projection of real parent child fields.
    fn child_restore_projection(_snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        Err(plugin_sdk_fault("editor did not declare a loaded-parent child projection"))
    }
    /// 🌱️ The editor's derivable-child genesis — see `ArtifactApp::genesis_child_pack`.
    fn genesis_child_pack(_snapshot: &Self::Snapshot, _slot: &str, _child_id: &str) -> Result<Option<Vec<u8>>, ValueError> {
        Ok((|| None)())
    }
    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = false;
    /// 📜️ Stable document schema id — prefer this over `artifact_schema(&self)`.
    const DOCUMENT_SCHEMA: &'static str;
    /// 📚️ Examples authored for this editor's dialect. `PluginBuilder::editor` stamps this
    /// catalogue onto the plugin manifest; the navbar dropdown is `examples_for_app` over that
    /// manifest, so a plugin never passes the list itself.
    fn examples() -> Vec<ExampleSource> {
        Vec::new()
    }
    type Snapshot: Clone + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + 'static;
    type Mutation: protocol::SemanticMutation<Self::Snapshot> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Config: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + store::ConfigRecord + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    type ConfigMutation: ::protocol::Mutation<Self::Config> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::replication::OpBinaryReceiving + 'static;
    type Draft: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    type DraftMutation: ::protocol::Mutation<Self::Draft> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Presence: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + store::ArtifactPresenceSnapshot + 'static;
    type PresenceMutation: ::protocol::Mutation<Self::Presence> + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Transient: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    type TransientMutation: ::protocol::Mutation<Self::Transient> + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Command: ::protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static;

    /// 📜️ Exact owner-local bounded reducer proofs forwarded by `EditorApp<Self>`.
    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        Vec::new()
    }

    fn mounted_job_maintenance_demands(_instance_id: u32, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn mounted_job_maintenance_phase(_instance_id: u32) -> Option<MountedOwnerPhaseV1> { None }

    fn mounted_job_maintenance_step(_instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,_grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,crate::bounded_ownership::PluginLifecycleFailure> {
        Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()))
    }

    fn mounted_job_close_demands(_instance_id: u32, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn mounted_job_close_step(_instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,_grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,crate::bounded_ownership::PluginLifecycleFailure> {
        Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()))
    }

    fn mounted_jobs_terminal_is_empty(_instance_id: u32) -> bool {
        true
    }

    fn mounted_job_prepare_snapshot_read(_operation: AppRenderOperationContext, _snapshot: &Self::Snapshot) -> bool {
        false
    }

    /// 🧩️ Author-owned forwarding seam for factories whose concrete runtime owner is the
    /// framework's `EditorApp<Self>` adapter.
    fn register_tool_job_factories(_registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        Ok(())
    }
    /// ⏯️ Author-owned run/revalidate job builder whose runtime owner is `EditorApp<Self>`.
    fn build_tool_run_job(_request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        Ok(None)
    }
    /// 🎯️ Author-owned retargetable run/revalidate job builder whose runtime owner is `EditorApp<Self>`.
    fn build_retargetable_tool_run_job(_request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<Box<dyn ToolRunRetargetableJob<Self::Config>>>, Fault> {
        Ok(None)
    }
    /// ⌨️ This editor's typing algebra — forwarded verbatim to `ArtifactApp::typing_fold` by `EditorApp<Self>`, where the
    /// contract lives (default: a single buffer, the latest delivery replaces the net).
    fn typing_fold(_net: &[Self::Mutation], next: &[Self::Mutation]) -> semio_framework_tool_machine::TypingFold<Self::Mutation> {
        semio_framework_tool_machine::TypingFold::Net(next.to_vec())
    }
    /// 🔰️ This editor's entity label for a history-edit reference chip — forwarded verbatim to
    /// `ArtifactApp::entity_label` by `EditorApp<Self>`, where the contract lives.
    fn entity_label(_snapshot: &Self::Snapshot, _kinds: &[String], _id: &str) -> Option<LocalizedLabel> {
        None
    }
    /// 🧲️ This editor's selection-row → entity-id map for a history edit's "Use selection" — forwarded verbatim to
    /// `ArtifactApp::selection_reference_id` by `EditorApp<Self>`, where the contract lives.
    fn selection_reference_id(_kinds: &[String], row: &str) -> Option<String> {
        Some(row.to_string())
    }
    /// 🧭️ This editor's intent leaf kinds per tool — forwarded verbatim to `ArtifactApp::tool_intent_kinds` by
    /// `EditorApp<Self>`, where the contract lives (default: none, the first operation labels a transaction row).
    fn tool_intent_kinds(_tool: &str) -> &'static [&'static str] {
        &[]
    }
    /// 📣️ This editor's fault notices — forwarded verbatim to `ArtifactApp::fault_notices` by `EditorApp<Self>`, where the
    /// contract lives (default: none).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        &[]
    }
    fn register_window_transient_owners(_registry: &mut WindowTransientOwnerRegistry) -> Result<(), Fault> {
        Ok(())
    }
    fn register_window_config_owners(_registry: &mut WindowConfigOwnerRegistry) -> Result<(), Fault> {
        Ok(())
    }

    fn retained_window_transient_target(_command: &Self::Command) -> Option<(&str, &'static str)> {
        None
    }

    /// 🐢️ This editor's declared interaction refresh scope — forwarded verbatim to
    /// `ArtifactApp::interaction_scope` by `EditorApp<Self>`, where the contract lives.
    fn interaction_scope(_verb: InteractionVerb, _domains: &[&str]) -> Option<UiDirtyScope> {
        None
    }

    /// ⏳️ This editor's operation-progress refresh scope — forwarded verbatim to
    /// `ArtifactApp::operation_progress_scope` by `EditorApp<Self>`, where the contract lives.
    fn operation_progress_scope() -> UiDirtyScope {
        UiDirtyScope::None
    }

    /// 📨️ This editor's answer to a [`HostEvent`] — forwarded verbatim to `ArtifactApp::host_event` by
    /// `EditorApp<Self>`, where the contract lives.
    fn host_event(_event: &HostEvent) -> Option<Self::Command> {
        None
    }

    fn build_instance_operation_owner() -> Box<dyn ArtifactInstanceOperationOwner> {
        Box::new(EmptyArtifactInstanceOperationOwner)
    }

    /// 🏗️ Author-owned payload builder paired with `register_tool_job_factories`.
    fn build_tool_job(_request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        Ok(None)
    }

    /// 🧭️ Supplies the framework-owned bounded natural-file importer while leaving every other
    /// reserved route to the editor. Editors with a richer media route may override this method.
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        let natural = matches!(&request.input, ArtifactReservedToolInput::Media { port, .. } if port == NATURAL_FILE_PORT) && Self::natural_file_codec().is_some();
        if natural && !request.raw_wire.is_empty() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("media.natural-import-wire"), "natural-file import admits an owned intrinsic byte value, never a second wire payload"));
        }
        if !natural {
            return Ok(None);
        }
        Ok(Some(ArtifactReservedToolJob::new(NaturalFileImportJob::<Self>::new(request)?)))
    }

    /// 🎞️ Author-owned resumable media-output producer for `EditorApp<Self>`.
    fn build_media_export_job(_request: ArtifactMediaExportJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        Ok(None)
    }

    fn build_snapshot_disposer() -> Option<Box<dyn ArtifactSnapshotDisposer<Self::Snapshot>>> {
        None
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        None
    }

    /// 🔐️ Framework-owned owner catalogs, paired with this trait's default disposers below — the
    /// adapter forwards these answers to `ArtifactApp`, so the defaults live on both traits.
    fn document_store_owners_source_demands()->Result<RetirementDemand,ValueError>{store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<Self::Snapshot,Self::Mutation>()}
    fn build_document_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<Self::Snapshot, Self::Mutation>(grant))
    }

    fn config_store_owners_source_demands()->Result<RetirementDemand,ValueError>{store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<Self::Config,Self::ConfigMutation>()}
    fn build_config_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Config, Self::ConfigMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Config, Self::ConfigMutation>>> {
        Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<Self::Config, Self::ConfigMutation>(grant))
    }

    /// 📝️ Grants exact draft retirement ownership to this editor adapter.
    fn draft_store_owners_source_demands()->Result<RetirementDemand,ValueError>{store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<Self::Draft,Self::DraftMutation>()}
    fn build_draft_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Draft, Self::DraftMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Draft, Self::DraftMutation>>> {
        Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<Self::Draft, Self::DraftMutation>(grant))
    }

    /// 📬️ Grants retained one-item document preparation to this editor adapter.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        None
    }

    /// 📬️ Grants retained one-item config preparation to this editor adapter.
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        None
    }

    /// 📬️ Grants retained one-item draft preparation to this editor adapter.
    fn build_draft_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Draft, Self::DraftMutation>>> {
        None
    }

    /// 📬️ Grants retained shared-live presence preparation to this editor adapter.
    fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        None
    }

    /// 📬️ Grants retained local-live transient preparation to this editor adapter.
    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        None
    }

    /// 🧹️ Grants bounded displaced-presence-root retirement to this editor adapter.
    /// Framework-owned by default — `EditorApp<E>` forwards THIS answer, so a `None` here
    /// would strip the `ArtifactApp` default and break registered-fixture close.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::bounded_presence_root_retirement_factory::<Self::Presence>())
    }

    /// 👥️ Grants exact typed retirement for peer presence snapshots to this editor adapter.
    /// Framework-owned by default, for the same reason as `ArtifactApp`'s twin: `EditorApp<E>`
    /// forwards THIS answer, so a `None` here would take the framework's default back out.
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::bounded_presence_root_retirement_factory::<Self::Presence>())
    }

    /// 🧹️ Grants bounded displaced-transient-root retirement to this editor adapter.
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        None
    }

    #[expect(clippy::result_large_err, reason = "Unavailable initialization returns the original completed envelope; boxing the refusal would change its retained ownership boundary.")]
    fn build_document_store_initialization_job(
        envelope: ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: ActorId,
    ) -> ArtifactInitializationAdmission<Self::Snapshot, Self::Mutation> {
        Ok(bounded_document_store_initialization_job(envelope, Self::DOCUMENT_SCHEMA, operation, generation, actor))
    }

    fn validate_document_store_publication(_operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        if Self::REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY {
            Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.publication-authority-missing"), "registered editor did not supply atomic publication authority"))
        } else {
            Ok(())
        }
    }

    /// 🧹️ The bounded close lanes are FRAMEWORK-owned by default — see `ArtifactApp`'s own
    /// defaults for the measurement. The adapter (`EditorApp`/`ViewerApp`) FORWARDS this trait's
    /// answer to `ArtifactApp`, so a `None` here overrides the framework default one layer up
    /// and reinstates `interactive-job.close-owned-disposer-missing`; the defaults therefore
    /// live on both traits (ticket 26/09/18 slice TC4 §3).
    fn build_document_store_disposer() -> ArtifactDisposal<ArtifactStore<Self::Snapshot, Self::Mutation>> {
        Some(bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> ArtifactDisposal<ConfigStore<Self::Config, Self::ConfigMutation>> {
        Some(bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> ArtifactDisposal<store::DraftStore<Self::Draft, Self::DraftMutation>> {
        Some(bounded_document_store_disposer::<Self::Draft, Self::DraftMutation>())
    }

    fn build_presence_store_disposer() -> ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        Some(crate::bounded_presence_store_disposer::<Self::Presence, Self::PresenceMutation>())
    }

    fn build_transient_store_disposer() -> ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        Some(bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    /// 👥️🫧️ The two EPHEMERAL lanes this command touches. Returns a plain tuple rather than
    /// `EphemeralEmit<Self>` — that struct is bound to the RUNTIME `ArtifactApp` trait, which
    /// `Self` here does not implement; `EditorApp<E>::ephemeral` wraps this tuple into a real
    /// `EphemeralEmit` at the adapter boundary, where the bound is satisfied.
    fn ephemeral(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _presence: &PresenceView<'_, Self::Presence>,
        _transient: &TransientView<'_, Self::Transient>,
    ) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>) {
        (Vec::new(), Vec::new())
    }
    fn config_schema() -> &'static str {
        "config.empty"
    }
    fn initial_snapshot() -> Self::Snapshot;
    fn initial_config() -> Self::Config {
        Self::Config::default()
    }
    /// 🧭️ Resolves a host-owned control-plane action to one configuration mutation.
    /// `EditorApp<E>` forwards this exact owner hook to the runtime adapter.
    fn host_configuration_mutation(_action: &str, _args: Option<&DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok(None)
    }
    fn initial_draft() -> Self::Draft {
        Self::Draft::default()
    }
    /// 🧩️ The pure heart of an editor — identical shape to `ArtifactApp::handle`.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        interaction: &InteractionView<'_>,
        view_state: Option<&ViewModel>,
        draft: &DraftView<'_, Self::Draft>,
        engines: &EngineHandles,
    ) -> ArtifactMutationOutcome<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>;
    fn command_id(_command: &Self::Command) -> &'static str {
        "typed-command"
    }
    fn command_from_action(action: &str, _args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        Err(Fault::new(
            FaultOrigin::App,
            FaultCode::new("app.command.unsupported"),
            format!("action '{action}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"),
        ))
    }
    /// 🔐️ `ArtifactApp::agent_target_revision`'s sibling for a hand-authored `ArtifactEditor`; `EditorApp<E>` delegates here.
    fn agent_target_revision(_action: &str, _args: &DslValue, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
        Ok(None)
    }

    fn config_spec() -> ConfigSpec {
        ConfigSpec::default()
    }
    fn clipboard_media_type() -> Option<MediaType> {
        None
    }
    fn clipboard_accepts() -> Vec<MediaType> {
        Self::clipboard_media_type().into_iter().collect()
    }
    fn copy_fragment(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        Err(ClipboardError::EmptySelection)
    }
    fn cut_operations(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>) -> Vec<Self::Mutation> {
        Vec::new()
    }
    fn interaction_topology(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, ValueError> {
        Ok((|| protocol::InteractionTopology::default())())
    }
    fn paste_operations(_doc: &ArtifactView<'_, Self::Snapshot>, _fragment: &ClipboardFragment, _placement: &PastePlacement) -> Result<Vec<Self::Mutation>, ClipboardError> {
        Ok(Vec::new())
    }
    fn pending_effects(_owner: &ArtifactInstanceOperationOwnerHandle, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view: Option<&ViewModel>) -> Vec<Effect> {
        Vec::new()
    }
    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> UiAssemblyResult<ComponentTree>;
    /// 🎭️ Renders against the same instance-retained operation owner used by typed jobs.
    fn render_with_instance_operation_owner(owner: &ArtifactInstanceOperationOwnerHandle, body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        let _ = owner;
        Self::render(body_key, doc, cfg, view_state)
    }
    /// 🧮️ Renders from the same immutable transient snapshot retained by app-owned jobs.
    ///
    /// 🕹️ `interaction` reads the framework-owned interaction mechanism (hover + selection) so a
    /// window can paint its own hovered/selected state instead of ever storing it again itself —
    /// see `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`.
    fn render_with_request_context(
        owner: &ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> crate::OriginalRenderResult<ComponentTree> {
        let rendered:crate::UiAssemblyResult<crate::ComponentTree>=(||{
        let _ = (transient, interaction);
        Self::render_with_instance_operation_owner(owner, body_key, doc, cfg, view_state)

        })();
        rendered.map(Some).map_err(Into::into)
        }
    fn window_engagements(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        HashMap::new()
    }
    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let _ = (transient, interaction);
        Self::window_engagements(doc, cfg, view_state)
    }
    fn window_measures(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::new()
    }
    /// 🕹️ See `ArtifactApp::window_measures_with_request_context` — same additive default.
    fn window_measures_with_request_context(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel, interaction: &InteractionView<'_>) -> HashMap<String, Vec<WindowMeasure>> {
        let _ = interaction;
        Self::window_measures(doc, cfg, view_state)
    }
    fn tool_measures(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::new()
    }
    /// 🛍️ See `ArtifactApp::app_catalogue_json` — the app-static catalogue published once per app
    /// instance on the reserved `framework.section.catalogue` surface, never per scene payload.
    fn app_catalogue_json() -> String {
        String::from("{}")
    }
    fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel, _registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        Vec::new()
    }
    /// 🕹️ See `ArtifactApp::context_menu_with_request_context` — same additive default.
    fn context_menu_with_request_context(
        request: &ContextMenuRequest,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        interaction: &InteractionView<'_>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        let _ = interaction;
        Self::context_menu(request, doc, cfg, view_state, registry)
    }
    fn genesis() -> Vec<Self::Mutation> {
        Vec::new()
    }
    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        None
    }
    fn io() -> Option<AppIo> {
        None
    }
    /// 📄️ Declares the exact representation only when this editor mounts both directions.
    fn natural_file_codec() -> Option<NaturalFileCodec> {
        None
    }
    /// 💾️ Encodes genuine natural-format bytes.
    fn encode_natural_file(_snapshot: &Self::Snapshot) -> Result<Vec<u8>, MediaError> {
        Err(MediaError::NotImplemented)
    }
    /// 📂️ Decodes genuine natural-format bytes.
    fn decode_natural_file(_bytes: &[u8]) -> Result<Self::Snapshot, MediaError> {
        Err(MediaError::NotImplemented)
    }
    /// 🧭️ Transfers owned natural bytes into this format's decoder. The explicit
    /// synchronous owner is the temporary small-input refusal path, never a bounded-parser claim.
    fn build_natural_file_decode_owner(bytes: Vec<u8>, _contract: semio_framework::ToolExecutionContract) -> Result<NaturalFileDecodeOwner<Self::Snapshot>, MediaError> {
        Ok(NaturalFileDecodeOwner::Synchronous(bytes))
    }
    async fn media_ports() -> Vec<MediaPortSpec> {
        match Self::io() {
            Some(io) => io.all_ports().await,
            None => Vec::new(),
        }
    }
    /// 🌀️ `io-async-signatures`: async signature, behaviourally unchanged body.
    fn export_media(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Media, MediaError> {
        if port != "artifact:out" {
            return Err(MediaError::NotImplemented);
        }
        let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
        let bytes = doc.snapshot.encode_pack();
        Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
    }
    fn export_media_with_request_context(owner: &ArtifactInstanceOperationOwnerHandle, port: &str, doc: &ArtifactView<'_, Self::Snapshot>, transient: &TransientView<'_, Self::Transient>) -> Result<Media, MediaError> {
        let _ = (owner, transient);
        Self::export_media(port, doc)
    }
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> ArtifactMutationOutcome<Self::Mutation, Self::ConfigMutation, Self::DraftMutation, MediaError> {
        if port == NATURAL_FILE_PORT {
            let codec = Self::natural_file_codec().ok_or(MediaError::NotImplemented)?;
            let bytes = natural_file_input_bytes(port, codec, media)?;
            let snapshot = Self::decode_natural_file(bytes)?;
            return Ok(document_load_emit(&snapshot, Self::DOCUMENT_SCHEMA));
        }
        if port != "artifact:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
        };
        let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let snapshot = <Self::Snapshot as ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        Ok(document_load_emit(&snapshot, Self::DOCUMENT_SCHEMA))
    }
    /// 🌀️ `io-async-signatures`: async signature (delegates to the now-async `export_media`).
    fn media_fingerprint(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<MediaFingerprint, MediaError> {
        match Self::export_media(port, doc) {
            Ok(media) => Ok(MediaFingerprint::of(&media)),
            Err(error) => Err(error),
        }
    }
}

/// 🪪️ Accepts only one server-minted nonzero lowercase artifact identity.
fn is_server_minted_artifact_document_id_v1(document_id: &str) -> bool {
    let Some(suffix) = document_id.strip_prefix("artifact-") else { return false };
    suffix.len() == 32 && suffix.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) && suffix.bytes().any(|byte| byte != b'0')
}

/// 🪡️ Builds one app-owned initial pair at a server-minted identity while preserving an exact
/// empty history and cursor. This is the single producer behind `world actor`'s `codec.genesis`
/// export, so every plugin gets creation authority from the one implementation rather than from
/// a per-package native factory the host has to link (ticket 26/09/18 slice TC3b).
pub async fn artifact_app_genesis_pair<A: ArtifactApp>(document_id: &str) -> Result<store::os_store::serialization::ArtifactPackFiles, store::VcsError> {
    let dialect: ArtifactDialect = A::DIALECT.into();
    if !is_server_minted_artifact_document_id_v1(document_id) {
        return Err(store::VcsError::ValidationFailed("artifact genesis identity is not a server-minted artifact id".into()));
    }
    let mut envelope = create_document_envelope::<A::Snapshot, A::Mutation>(A::DOCUMENT_SCHEMA, document_id, A::initial_snapshot().await, None).into_owners();
    envelope.dialect = Some(dialect.clone());
    let zero_cursor = envelope.cursor.as_ref().is_some_and(|cursor| cursor.applied_edit_ids.is_empty() && cursor.redo_edit_ids.is_empty() && cursor.checkpoint_id.is_none());
    if envelope.id != document_id
        || envelope.dialect.as_ref() != Some(&dialect)
        || !envelope.vcs.edits.is_empty()
        || !envelope.vcs.changes.is_empty()
        || !envelope.vcs.checkpoints.is_empty()
        || !envelope.vcs.alternatives.is_empty()
        || !zero_cursor
    {
        ArtifactEnvelope::from_owners(envelope).retire_unadopted();
        return Err(store::VcsError::ValidationFailed("artifact genesis produced nonzero history".into()));
    }
    let printed = store::os_store::text_format::print_document_pack(&envelope).await;
    ArtifactEnvelope::from_owners(envelope).retire_unadopted();
    printed
}

/// 🧩️ Applies one `os_spr::encode_ops_vec` batch to an app-owned pair — the guest half of
/// `world actor`'s `codec.apply-ops` export, structurally the same reduction
/// `store::os_store::serialization::ArtifactCodec::apply_ops_binary` performs for a natively linked codec.
/// 🔭️ The LIVE snapshot of one artifact `pack`/`spr` pair, without loading it into any
/// instance and without leaving a history shell behind. This is how a READER — an inference
/// computing over the artifact its caller named, never an editor — gets the document it was
/// asked about: the pair is the authoritative one, `parse_document_pack` replays it exactly as
/// a document archive load folds it, and `into_snapshot` retires the envelope through the store's own
/// bounded path instead of dropping it (a bare drop aborts the process — every terminal shell
/// under `ArtifactEnvelope` asserts its owners were retired first).
pub async fn artifact_pair_snapshot<P, Mutation>(pack: &[u8], spr: &[u8]) -> Result<P, store::VcsError>
where
    P: Clone + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static,
    Mutation: OpText + OpBinary + store::Mutation<P> + 'static,
{
    if pack.is_empty() || spr.is_empty() {
        return Err(store::VcsError::Deserialize("artifact pair carries no pack+spr baseline".into()));
    }
    let parsed: store::os_store::text_format::ParsedDocumentText<P, Mutation> = store::os_store::text_format::parse_document_pack(pack, spr).await.map_err(|error| store::VcsError::Deserialize(error.to_string()))?;
    Ok(parsed.into_snapshot())
}

/// 🎟️ Refuses borrowed codec sources before decode until an original retained reduction recipient exists.
pub async fn artifact_app_apply_ops<A: ArtifactApp>(_pack: &[u8], _spr: &[u8], _ops: &[u8], _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<store::os_store::serialization::ArtifactPackFiles, store::VcsError> {
    Err(store::VcsError::NativeEncoding(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"codec requires an original retained reduction operation")))
}

/// 📜️ Preserves the typed blocking history fault at the codec boundary.
pub fn replay_envelopes_fault(error: store::VcsError) -> Fault {
    match error {
        store::VcsError::Rejected { messages, .. } => {
            let blocking: Vec<String> = messages.iter().filter(|message| protocol::MergePolicy::Normal.rejects(message.level)).take(8).map(|message| format!("{}: {}", message.code.0, message.message)).collect();
            Fault::new(FaultOrigin::Framework, FaultCode::new("history.ledger-not-replayable"), format!("the ledger does not replay under the check-in policy ({} blocking message(s)): {}", messages.len(), blocking.join("; ")))
        }
        other => other.into_fault(),
    }
}

/// 🎟️ Refuses the borrowed ledger before parsing or an unfunded catalog construction.
pub async fn artifact_app_replay_envelopes<A: ArtifactApp>(_pack: &[u8], _spr: &[u8], _envelopes: &[u8]) -> Result<store::os_store::serialization::ArtifactPackFiles, store::VcsError> {
    Err(store::VcsError::NativeEncoding(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"codec requires an original retained reduction operation")))
}

/// 🧬️ The future one `codec` answer of an [`ArtifactCodecTableV1`] resolves to.
pub type ArtifactCodecFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, Fault>> + 'a>>;

/// 🧬️ One registered app's `world actor` `codec` answers as plain functions of its type, recorded when the app
/// is registered. Every answer is a pure function of the app's TYPE — its snapshot record shape, its initial
/// snapshot and dialect, its owner catalogue — never of a constructed instance, so a `codec` call reads this
/// table and constructs no app: no store around an initial snapshot, no action registry, no window owners,
/// nothing to drain and close afterwards.
#[derive(Clone, Copy)]
pub struct ArtifactCodecTableV1 {
    /// 🧬️ Structural fingerprint of the app's snapshot record shape — the same 32 bytes
    /// `store::os_store::serialization::ArtifactCodec::pack_schema_hash` carries, read off `Snapshot::record_spec()`. `None` when the
    /// snapshot is a hand-written `ArtifactPack` with no `RecordSpec`, exactly as the native codec table reports
    /// `[0; 32]` for that case.
    /// 🚫️async: E4 fn-pointer slot
    pub pack_schema_hash: fn() -> Option<[u8; 32]>,
    /// 🌱️ The canonical empty document of the app's kind at `document_id`: its initial snapshot, its own
    /// dialect, and an exactly zero history (no edits, changes, checkpoints or alternatives, and an empty cursor).
    /// 🚫️async: E4 fn-pointer slot
    pub genesis: for<'a> fn(&'a str) -> ArtifactCodecFuture<'a, store::os_store::serialization::ArtifactPackFiles>,
    /// 📥️ `(pack, spr) -> (dsl, ops)` mirror of a pair of the app's kind — the pair-validation fence of a host
    /// whose codec it does not link.
    /// 🚫️async: E4 fn-pointer slot
    pub print_mirror: for<'a> fn(&'a [u8], &'a [u8]) -> ArtifactCodecFuture<'a, store::os_store::text_format::ArtifactTextFiles>,
    /// 🧩️ Applies one `os_spr::encode_ops_vec` batch to a pair of the app's kind and returns the next pair.
    /// 🚫️async: E4 fn-pointer slot
    pub apply_ops: for<'a, 'b> fn(&'a [u8], &'a [u8], &'a [u8], &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> ArtifactCodecFuture<'a, store::os_store::serialization::ArtifactPackFiles>,
    /// 📜️ Folds one `os_spr::encode_envelopes` ledger stream onto a pair of the app's kind through the replica
    /// merge gate (`store::replay_envelopes_onto_pair`).
    /// 🚫️async: E4 fn-pointer slot
    pub replay_envelopes: for<'a> fn(&'a [u8], &'a [u8], &'a [u8]) -> ArtifactCodecFuture<'a, store::os_store::serialization::ArtifactPackFiles>,
}

/// 🧬️ The [`ArtifactCodecTableV1`] of every app built on `A`.
pub fn artifact_codec_table<A: ArtifactApp>() -> ArtifactCodecTableV1 {
    fn pack_schema_hash<A: ArtifactApp>() -> Option<[u8; 32]> {
        <A::Snapshot as ArtifactPack>::record_spec().map(|spec| store::os_pack::schema_hash(&spec))
    }
    fn genesis<A: ArtifactApp>(document_id: &str) -> ArtifactCodecFuture<'_, store::os_store::serialization::ArtifactPackFiles> {
        Box::pin(async move { artifact_app_genesis_pair::<A>(document_id).await.map_err(|error| error.into_fault()) })
    }
    fn print_mirror<'a, A: ArtifactApp>(pack: &'a [u8], spr: &'a [u8]) -> ArtifactCodecFuture<'a, store::os_store::text_format::ArtifactTextFiles> {
        Box::pin(async move {
            let parsed: store::os_store::text_format::ParsedDocumentText<A::Snapshot, A::Mutation> = store::os_store::text_format::parse_document_pack(pack, spr).await.map_err(|error| error.into_fault())?;
            let envelope = parsed.into_envelope();
            let mirror = store::os_store::text_format::print_document_text(&envelope).await;
            envelope.retire_unadopted();
            mirror.map_err(|error| error.into_fault())
        })
    }
    fn apply_ops<'a, 'b, A: ArtifactApp>(pack: &'a [u8], spr: &'a [u8], ops: &'a [u8], identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> ArtifactCodecFuture<'a, store::os_store::serialization::ArtifactPackFiles> {
        Box::pin(async move { artifact_app_apply_ops::<A>(pack, spr, ops, identity).await.map_err(|error| error.into_fault()) })
    }
    fn replay_envelopes<'a, A: ArtifactApp>(pack: &'a [u8], spr: &'a [u8], envelopes: &'a [u8]) -> ArtifactCodecFuture<'a, store::os_store::serialization::ArtifactPackFiles> {
        Box::pin(async move { artifact_app_replay_envelopes::<A>(pack, spr, envelopes).await.map_err(replay_envelopes_fault) })
    }
    ArtifactCodecTableV1 { pack_schema_hash: pack_schema_hash::<A>, genesis: genesis::<A>, print_mirror: print_mirror::<A>, apply_ops: apply_ops::<A>, replay_envelopes: replay_envelopes::<A> }
}

/// 👁️ Authoring trait for a read-only surface (contract §2.2) — the mutation-incapable twin of
/// `ArtifactEditor`. `Mutation` stays decode-only (the store's op log must still decode past
/// edits an `ArtifactEditor` made) but `handle` returns `ViewEmit`, which cannot structurally
/// carry an artifact or draft mutation. `ViewerApp<V>` below is the sole `ArtifactApp` implementor.
pub trait ArtifactViewer: Default + Send + 'static {
    /// 📬️ Borrows this viewer's original typed completion retirement birth.
    fn completion_retirement_birth_bytes(value:&ArtifactToolCompletionValue<ViewerApp<Self>>)->Option<usize>{super::completion_retirement::derived_completion_birth_bytes(value)}
    /// 🎟️ Retains the viewer completion on every unsupported or unfunded cleanup admission.
    fn admit_completion_retirement(value:&mut Option<ArtifactToolCompletionValue<ViewerApp<Self>>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{super::completion_retirement::derived_admit_completion_retirement(value,grant)}
    /// 🧮️ Declares the physical authority frame for this viewer's decoded editor-authored mutation batch.
    fn owned_mutation_batch_birth_bytes() -> Option<usize> { None }

    /// 📥️ Admits a decoded original batch through the viewer's explicit first-party owner.
    fn admit_owned_mutation_batch(_values: &mut Option<Vec<Self::Mutation>>, _grant: RetainedCloneGrant) -> Result<Option<(store::MemberStoreOwnedBatch, RetainedCloneProgress)>, ValueError> { Ok(None) }

    /// 🧩️ Read-only twin of `ArtifactEditor::Members` — the roster this viewer's composed children
    /// are opened through, carried by the app rather than by whoever registers it.
    type Members: SpaceMember + MemberFactory + Send + 'static = NoMembers;
    /// 📖️ Supplies domain-owned decoding for saved documents and their editor-authored history.
    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        None
    }

    /// 🏗️ Restores a saved document through the viewer's retained validation and replay authority — the
    /// bounded job by default, paired with the bounded default owners.
    #[expect(clippy::result_large_err, reason = "A refused initialization returns ownership of the original completed envelope.")]
    fn build_document_store_initialization_job(
        envelope: ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: ActorId,
    ) -> ArtifactInitializationAdmission<Self::Snapshot, Self::Mutation> {
        Ok(bounded_document_store_initialization_job(envelope, Self::DOCUMENT_SCHEMA, operation, generation, actor))
    }

    /// 🔐️ Framework-owned owner catalogs, paired with this trait's default disposers below — the
    /// adapter forwards these answers to `ArtifactApp`, so a `None` here beside a default disposer
    /// faulted every viewer close (the trusted codec probe's throwaway viewer included) with
    /// `artifact store has no owner-supplied bounded disposer`.
    fn document_store_owners_source_demands()->Result<RetirementDemand,ValueError>{store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<Self::Snapshot,Self::Mutation>()}
    fn build_document_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<Self::Snapshot, Self::Mutation>(grant))
    }

    fn config_store_owners_source_demands()->Result<RetirementDemand,ValueError>{store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<Self::Config,Self::ConfigMutation>()}
    fn build_config_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Config, Self::ConfigMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Config, Self::ConfigMutation>>> {
        Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<Self::Config, Self::ConfigMutation>(grant))
    }

    /// 🧹️ The bounded close lanes are FRAMEWORK-owned by default — see `ArtifactApp`'s own
    /// defaults for the measurement. The adapter (`EditorApp`/`ViewerApp`) FORWARDS this trait's
    /// answer to `ArtifactApp`, so a `None` here overrides the framework default one layer up
    /// and reinstates `interactive-job.close-owned-disposer-missing`; the defaults therefore
    /// live on both traits (ticket 26/09/18 slice TC4 §3).
    fn build_document_store_disposer() -> ArtifactDisposal<ArtifactStore<Self::Snapshot, Self::Mutation>> {
        Some(bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> ArtifactDisposal<ConfigStore<Self::Config, Self::ConfigMutation>> {
        Some(bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_presence_store_disposer() -> ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        Some(crate::bounded_presence_store_disposer::<Self::Presence, Self::PresenceMutation>())
    }

    fn build_transient_store_disposer() -> ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        Some(bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::bounded_presence_root_retirement_factory::<Self::Presence>())
    }

    /// 🎭️ Always `Viewer` in practice — defaulted so implementors never restate it.
    const ROLE: AppRole = AppRole::Viewer;
    const DIALECT: Dialect;
    /// 🪟️ Supplies the viewer's bounded projection of real parent child fields.
    fn child_restore_projection(_snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        Err(plugin_sdk_fault("viewer did not declare a loaded-parent child projection"))
    }
    /// 🌱️ The viewer's derivable-child genesis — see `ArtifactApp::genesis_child_pack`.
    fn genesis_child_pack(_snapshot: &Self::Snapshot, _slot: &str, _child_id: &str) -> Result<Option<Vec<u8>>, ValueError> {
        Ok((|| None)())
    }
    const DOCUMENT_SCHEMA: &'static str;
    type Snapshot: Clone + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + 'static;
    /// 📜️ Decode-only — never constructed by `handle`, but the store's op log must still decode
    /// past edits made by an `ArtifactEditor` sharing this dialect, and labels them in every locale like it.
    type Mutation: protocol::SemanticMutation<Self::Snapshot> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Config: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + store::ConfigRecord + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    type ConfigMutation: ::protocol::Mutation<Self::Config> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::replication::OpBinaryReceiving + 'static;
    type Presence: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + store::ArtifactPresenceSnapshot + 'static;
    type PresenceMutation: ::protocol::Mutation<Self::Presence> + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Transient: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    type TransientMutation: ::protocol::Mutation<Self::Transient> + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Command: ::protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static;

    fn mounted_job_maintenance_demands(_instance_id: u32, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn mounted_job_maintenance_phase(_instance_id: u32) -> Option<MountedOwnerPhaseV1> { None }

    fn mounted_job_maintenance_step(_instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,_grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,crate::bounded_ownership::PluginLifecycleFailure> {
        Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()))
    }

    fn mounted_job_close_demands(_instance_id: u32, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn mounted_job_close_step(_instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,_grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,crate::bounded_ownership::PluginLifecycleFailure> {
        Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()))
    }

    fn mounted_jobs_terminal_is_empty(_instance_id: u32) -> bool {
        true
    }

    fn mounted_job_prepare_snapshot_read(_operation: AppRenderOperationContext, _snapshot: &Self::Snapshot) -> bool {
        false
    }

    fn register_window_transient_owners(_registry: &mut WindowTransientOwnerRegistry) -> Result<(), Fault> {
        Ok(())
    }
    fn register_window_config_owners(_registry: &mut WindowConfigOwnerRegistry) -> Result<(), Fault> {
        Ok(())
    }

    /// 📜️ Exact owner-local bounded reducer proofs forwarded by `ViewerApp<Self>` — the read-only
    /// twin of `ArtifactEditor::bounded_first_step_tool_proofs`. A viewer that declares view-only
    /// actions (camera orbit, show-mode, sun) needs them for exactly the same reason an editor
    /// does: `InteractiveJobClassification::Migrated` is the only UI-dispatchable classification,
    /// and a `Migrated` verb without an exact app-owned reducer is rejected with
    /// `interactive-job.missing-owned-reducer`. The emitted mutations stay config-only by
    /// construction — a viewer's publication contracts may never name the artifact lane.
    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        Vec::new()
    }

    /// 🧩️ Author-owned forwarding seam for factories whose concrete runtime owner is the
    /// framework's `ViewerApp<Self>` adapter.
    fn register_tool_job_factories(_registry: &mut ArtifactToolFactoryRegistry<'_, ViewerApp<Self>>) -> Result<(), Fault> {
        Ok(())
    }

    /// ⏯️ Author-owned read-only run job builder whose runtime owner is `ViewerApp<Self>` (contract §3.7).
    fn build_tool_run_job(_request: ToolRunJobRequest<'_, ViewerApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        Ok(None)
    }

    /// 🎯️ Author-owned retargetable read-only run job builder whose runtime owner is `ViewerApp<Self>`.
    fn build_retargetable_tool_run_job(_request: ToolRunJobRequest<'_, ViewerApp<Self>>) -> Result<Option<Box<dyn ToolRunRetargetableJob<Self::Config>>>, Fault> {
        Ok(None)
    }

    fn retained_window_transient_target(_command: &Self::Command) -> Option<(&str, &'static str)> {
        None
    }

    /// 🐢️ This viewer's declared interaction refresh scope — forwarded verbatim to
    /// `ArtifactApp::interaction_scope` by `ViewerApp<Self>`, where the contract lives.
    fn interaction_scope(_verb: InteractionVerb, _domains: &[&str]) -> Option<UiDirtyScope> {
        None
    }

    /// ⏳️ This viewer's operation-progress refresh scope — forwarded verbatim to
    /// `ArtifactApp::operation_progress_scope` by `ViewerApp<Self>`, where the contract lives.
    fn operation_progress_scope() -> UiDirtyScope {
        UiDirtyScope::None
    }

    /// 📨️ This viewer's answer to a [`HostEvent`] — forwarded verbatim to `ArtifactApp::host_event` by
    /// `ViewerApp<Self>`, where the contract lives.
    fn host_event(_event: &HostEvent) -> Option<Self::Command> {
        None
    }

    fn build_instance_operation_owner() -> Box<dyn ArtifactInstanceOperationOwner> {
        Box::new(EmptyArtifactInstanceOperationOwner)
    }

    /// 🏗️ Author-owned payload builder paired with `register_tool_job_factories`.
    fn build_tool_job(_request: ArtifactOwnedToolJobRequest<ViewerApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        Ok(None)
    }

    /// 🧭️ Builds a persistent producer/import job for one explicitly registered viewer route.
    fn build_reserved_tool_job(_request: ArtifactReservedToolJobRequest<ViewerApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        Ok(None)
    }

    /// 🎞️ Builds an owned resumable producer for an explicitly registered viewer media-output port.
    fn build_media_export_job(_request: ArtifactMediaExportJobRequest<ViewerApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        Ok(None)
    }

    /// 🧹️ Supplies the exact bounded disposer for snapshots retained by viewer output jobs.
    fn build_snapshot_disposer() -> Option<Box<dyn ArtifactSnapshotDisposer<Self::Snapshot>>> {
        None
    }

    /// 📬️ Grants retained one-item config preparation to this viewer adapter.
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        None
    }

    /// 📬️ Grants retained shared-live presence preparation to this viewer adapter.
    fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        None
    }

    /// 📬️ Grants retained local-live transient preparation to this viewer adapter.
    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        None
    }

    /// 🧹️ Grants bounded displaced-presence-root retirement to this viewer adapter.
    /// Framework-owned by default — `ViewerApp<V>` forwards THIS answer, so a `None` here
    /// would strip the `ArtifactApp` default and break registered-fixture close.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::bounded_presence_root_retirement_factory::<Self::Presence>())
    }

    /// 🧹️ Grants bounded displaced-transient-root retirement to this viewer adapter.
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        None
    }

    /// 👥️🫧️ See `ArtifactEditor::ephemeral` for why this returns a tuple, not `EphemeralEmit<Self>`.
    fn ephemeral(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _presence: &PresenceView<'_, Self::Presence>,
        _transient: &TransientView<'_, Self::Transient>,
    ) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>) {
        (Vec::new(), Vec::new())
    }
    fn config_schema() -> &'static str {
        "config.empty"
    }
    /// 📣️ This viewer's fault notices — forwarded verbatim to `ArtifactApp::fault_notices` by `ViewerApp<Self>`, where the
    /// contract lives (default: none).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        &[]
    }
    fn initial_snapshot() -> Self::Snapshot;
    fn initial_config() -> Self::Config {
        Self::Config::default()
    }
    /// 👁️ The pure heart of a viewer — same shape as `ArtifactEditor::handle` minus `draft` (a
    /// viewer never has a draft lane), returning `ViewEmit` instead of `Emit`: structurally
    /// incapable of an artifact or draft mutation (contract §2.2).
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        interaction: &InteractionView<'_>,
        view_state: Option<&ViewModel>,
        engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault>;
    fn command_id(_command: &Self::Command) -> &'static str {
        "typed-command"
    }
    fn command_from_action(action: &str, _args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        Err(Fault::new(
            FaultOrigin::App,
            FaultCode::new("app.command.unsupported"),
            format!("action '{action}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"),
        ))
    }
    fn config_spec() -> ConfigSpec {
        ConfigSpec::default()
    }
    fn interaction_topology(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, ValueError> {
        Ok((|| protocol::InteractionTopology::default())())
    }
    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> UiAssemblyResult<ComponentTree>;
    /// 🕹️ `interaction` reads the framework-owned interaction mechanism (hover + selection) so a
    /// window can paint its own hovered/selected state instead of ever storing it again itself —
    /// see `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`.
    fn render_with_request_context(
        _owner: &ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> crate::OriginalRenderResult<ComponentTree> {
        let rendered:crate::UiAssemblyResult<crate::ComponentTree>=(||{
        let _ = (transient, interaction);
        Self::render(body_key, doc, cfg, view_state)

        })();
        rendered.map(Some).map_err(Into::into)
        }
    fn pending_effects(_owner: &ArtifactInstanceOperationOwnerHandle, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view: Option<&ViewModel>) -> Vec<Effect> {
        Vec::new()
    }
    fn window_engagements(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        HashMap::new()
    }
    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let _ = (transient, interaction);
        Self::window_engagements(doc, cfg, view_state)
    }
    fn window_measures(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::new()
    }
    /// 🕹️ See `ArtifactApp::window_measures_with_request_context` — same additive default.
    fn window_measures_with_request_context(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel, interaction: &InteractionView<'_>) -> HashMap<String, Vec<WindowMeasure>> {
        let _ = interaction;
        Self::window_measures(doc, cfg, view_state)
    }
    fn tool_measures(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::new()
    }
    /// 🛍️ See `ArtifactApp::app_catalogue_json` — the app-static catalogue published once per app
    /// instance on the reserved `framework.section.catalogue` surface, never per scene payload.
    fn app_catalogue_json() -> String {
        String::from("{}")
    }
    fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel, _registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        Vec::new()
    }
    /// 🕹️ See `ArtifactApp::context_menu_with_request_context` — same additive default.
    fn context_menu_with_request_context(
        request: &ContextMenuRequest,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        interaction: &InteractionView<'_>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        let _ = interaction;
        Self::context_menu(request, doc, cfg, view_state, registry)
    }
    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        None
    }
    fn io() -> Option<AppIo> {
        None
    }
    async fn media_ports() -> Vec<MediaPortSpec> {
        match Self::io() {
            Some(io) => io.all_ports().await,
            None => Vec::new(),
        }
    }
    /// 🌀️ `io-async-signatures`: async signature, behaviourally unchanged body.
    fn export_media(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Media, MediaError> {
        if port != "artifact:out" {
            return Err(MediaError::NotImplemented);
        }
        let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
        let bytes = doc.snapshot.encode_pack();
        Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
    }
    fn export_media_with_request_context(owner: &ArtifactInstanceOperationOwnerHandle, port: &str, doc: &ArtifactView<'_, Self::Snapshot>, transient: &TransientView<'_, Self::Transient>) -> Result<Media, MediaError> {
        let _ = (owner, transient);
        Self::export_media(port, doc)
    }
    /// 🌀️ `io-async-signatures`: async signature (delegates to the now-async `export_media`).
    fn media_fingerprint(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<MediaFingerprint, MediaError> {
        match Self::export_media(port, doc) {
            Ok(media) => Ok(MediaFingerprint::of(&media)),
            Err(error) => Err(error),
        }
    }
}

/// 👁️ What a viewer may emit — structurally cannot carry an artifact or draft mutation (contract
/// §2.2): the read-only guarantee is a type property, not a runtime check.
pub struct ViewEmit<ConfigMutation> {
    pub config_mutations: Vec<ConfigMutation>,
    pub effects: Vec<Effect>,
    pub ui_dirty: UiDirtyScope,
}

impl<ConfigMutation> Default for ViewEmit<ConfigMutation> {
    fn default() -> Self {
        Self { config_mutations: Vec::new(), effects: Vec::new(), ui_dirty: UiDirtyScope::default() }
    }
}

impl<ConfigMutation> ViewEmit<ConfigMutation> {
    pub fn new() -> Self {
        Self::default()
    }
    /// 🧮️ A config-operation emission carrying `config_mutations` and nothing else.
    pub fn config(config_mutations: Vec<ConfigMutation>) -> Self {
        Self { config_mutations, ..Default::default() }
    }
    /// 🐚️ A single host effect and no config operations.
    pub fn effect(effect: Effect) -> Self {
        Self { effects: vec![effect], ..Default::default() }
    }
    /// 🐢️ Narrows which rendered UI sections this emission invalidates.
    pub fn dirty(ui_dirty: UiDirtyScope) -> Self {
        Self { ui_dirty, ..Default::default() }
    }
}

/// ✏️ Binds the authored editor and its original derived surface identity.
pub struct EditorApp<E:ArtifactEditor>{id:String,_editor:std::marker::PhantomData<E>}

impl<E:ArtifactEditor> Default for EditorApp<E>{
    fn default()->Self{Self{id:surface_app_id(&E::DIALECT.into(),E::ROLE),_editor:std::marker::PhantomData}}
}

impl<E:ArtifactEditor> EditorApp<E>{
    /// 🪪️ Borrows the actual derived surface identity used by the owning stores.
    pub async fn surface_id(&self)->&str{&self.id}
}

impl<E: ArtifactEditor> ArtifactApp for EditorApp<E> {
    fn owned_mutation_batch_birth_bytes() -> Option<usize> { E::owned_mutation_batch_birth_bytes() }
    fn completion_retirement_birth_bytes(value:&ArtifactToolCompletionValue<Self>)->Option<usize>{E::completion_retirement_birth_bytes(value)}
    fn admit_completion_retirement(value:&mut Option<ArtifactToolCompletionValue<Self>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{E::admit_completion_retirement(value,grant)}

    fn admit_owned_mutation_batch(values: &mut Option<Vec<Self::Mutation>>, grant: RetainedCloneGrant) -> Result<Option<(store::MemberStoreOwnedBatch, RetainedCloneProgress)>, ValueError> { E::admit_owned_mutation_batch(values, grant) }

    const DIALECT: Dialect = E::DIALECT;
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        E::child_restore_projection(snapshot)
    }
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, ValueError> {
        E::genesis_child_pack(snapshot, slot, child_id)
    }
    const APP_ID: &'static str = "surface";
    const ROLE: AppRole = E::ROLE;
    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = E::REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY;
    const DOCUMENT_SCHEMA: &'static str = E::DOCUMENT_SCHEMA;

    /// 🪪️ Overrides the `APP_ID` placeholder default — every ownership check must see the real
    /// derived surface id (`self.id`, see struct doc), matching what `ShellHost` addresses.
    async fn instance_id(&self) -> &str {
        self.surface_id().await
    }
    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        E::bounded_first_step_tool_proofs()
    }
    fn mounted_job_maintenance_phase(instance_id: u32) -> Option<MountedOwnerPhaseV1> { E::mounted_job_maintenance_phase(instance_id) }
    fn mounted_job_maintenance_demands(instance_id: u32, body: usize) -> Result<RetirementDemand, ValueError> { E::mounted_job_maintenance_demands(instance_id, body) }
    fn mounted_job_close_demands(instance_id: u32, body: usize) -> Result<RetirementDemand, ValueError> { E::mounted_job_close_demands(instance_id, body) }
    fn mounted_job_maintenance_step(instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,crate::bounded_ownership::PluginLifecycleFailure> {
        E::mounted_job_maintenance_step(instance_id,cx,grant)
    }
    fn mounted_job_close_step(instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,crate::bounded_ownership::PluginLifecycleFailure> {
        E::mounted_job_close_step(instance_id,cx,grant)
    }
    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        E::mounted_jobs_terminal_is_empty(instance_id)
    }
    fn mounted_job_prepare_snapshot_read(operation: AppRenderOperationContext, snapshot: &Self::Snapshot) -> bool {
        E::mounted_job_prepare_snapshot_read(operation, snapshot)
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
        E::register_tool_job_factories(registry)
    }
    fn build_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<ToolRunJob>, Fault> {
        E::build_tool_run_job(request)
    }
    fn build_retargetable_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<Box<dyn ToolRunRetargetableJob<Self::Config>>>, Fault> {
        E::build_retargetable_tool_run_job(request)
    }
    fn register_window_transient_owners(registry: &mut WindowTransientOwnerRegistry) -> Result<(), Fault> {
        E::register_window_transient_owners(registry)
    }
    fn register_window_config_owners(registry: &mut WindowConfigOwnerRegistry) -> Result<(), Fault> {
        E::register_window_config_owners(registry)
    }
    fn retained_window_transient_target(command: &Self::Command) -> Option<(&str, &'static str)> {
        E::retained_window_transient_target(command)
    }
    fn interaction_scope(verb: InteractionVerb, domains: &[&str]) -> Option<UiDirtyScope> {
        E::interaction_scope(verb, domains)
    }
    fn operation_progress_scope() -> UiDirtyScope {
        E::operation_progress_scope()
    }
    fn host_event(event: &HostEvent) -> Option<Self::Command> {
        E::host_event(event)
    }
    fn build_instance_operation_owner() -> Box<dyn ArtifactInstanceOperationOwner> {
        E::build_instance_operation_owner()
    }
    async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        E::build_tool_job(request)
    }
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<Self>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        E::build_reserved_tool_job(request)
    }
    fn build_media_export_job(request: ArtifactMediaExportJobRequest<Self>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        E::build_media_export_job(request)
    }
    fn build_snapshot_disposer() -> Option<Box<dyn ArtifactSnapshotDisposer<Self::Snapshot>>> {
        E::build_snapshot_disposer()
    }
    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        E::build_envelope_decode_owner_bundle()
    }
    fn document_store_owners_source_demands()->Result<RetirementDemand,ValueError>{E::document_store_owners_source_demands()}
    fn build_document_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        E::build_document_store_owners(grant)
    }
    fn config_store_owners_source_demands()->Result<RetirementDemand,ValueError>{E::config_store_owners_source_demands()}
    fn build_config_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Config, Self::ConfigMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Config, Self::ConfigMutation>>> {
        E::build_config_store_owners(grant)
    }
    fn draft_store_owners_source_demands()->Result<RetirementDemand,ValueError>{E::draft_store_owners_source_demands()}
    fn build_draft_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Draft, Self::DraftMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Draft, Self::DraftMutation>>> {
        E::build_draft_store_owners(grant)
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        E::build_artifact_store_one_item_preparation_factory()
    }
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        E::build_config_store_one_item_preparation_factory()
    }
    fn build_draft_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Draft, Self::DraftMutation>>> {
        E::build_draft_store_one_item_preparation_factory()
    }
    fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        E::build_presence_store_one_item_preparation_factory()
    }
    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        E::build_transient_store_one_item_preparation_factory()
    }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        E::build_presence_local_root_retirement_factory()
    }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        E::build_presence_peer_retirement_factory()
    }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        E::build_transient_local_root_retirement_factory()
    }
    fn build_document_store_initialization_job(
        envelope: ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: ActorId,
    ) -> ArtifactInitializationAdmission<Self::Snapshot, Self::Mutation> {
        E::build_document_store_initialization_job(envelope, operation, generation, actor)
    }
    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        E::validate_document_store_publication(operation, generation, live_generation)
    }
    fn build_document_store_disposer() -> ArtifactDisposal<ArtifactStore<Self::Snapshot, Self::Mutation>> {
        E::build_document_store_disposer()
    }
    fn build_config_store_disposer() -> ArtifactDisposal<ConfigStore<Self::Config, Self::ConfigMutation>> {
        E::build_config_store_disposer()
    }
    fn build_draft_store_disposer() -> ArtifactDisposal<store::DraftStore<Self::Draft, Self::DraftMutation>> {
        E::build_draft_store_disposer()
    }
    fn build_presence_store_disposer() -> ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        E::build_presence_store_disposer()
    }
    fn build_transient_store_disposer() -> ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        E::build_transient_store_disposer()
    }
    type Snapshot = E::Snapshot;
    type Mutation = E::Mutation;
    type Config = E::Config;
    type ConfigMutation = E::ConfigMutation;
    type Draft = E::Draft;
    type DraftMutation = E::DraftMutation;
    type Presence = E::Presence;
    type PresenceMutation = E::PresenceMutation;
    type Transient = E::Transient;
    type TransientMutation = E::TransientMutation;
    type Command = E::Command;

    async fn ephemeral(command: &Self::Command, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, presence: &PresenceView<'_, Self::Presence>, transient: &TransientView<'_, Self::Transient>) -> EphemeralEmit<Self> {
        let (presence_mutations, transient_mutations) = E::ephemeral(command, doc, cfg, presence, transient);
        EphemeralEmit { presence: presence_mutations, transient: transient_mutations, window_transient: Vec::new() }
    }
    async fn config_schema() -> &'static str {
        E::config_schema()
    }
    fn catalogue_example_document(example_id: &str) -> Option<Result<String, Fault>> {
        let mut examples = E::examples();
        if examples.is_empty() {
            examples = subset_examples(&E::DIALECT);
        }
        if examples.is_empty() {
            return None;
        }
        if example_id.is_empty() {
            return Some(Ok(<E::Snapshot as pack::snapshot::ArtifactDsl>::print_dsl(&E::initial_snapshot())));
        }
        Some(match examples.into_iter().find(|source| source.id() == example_id) {
            Some(source) => Ok(source.document()),
            None => Err(Fault::new(FaultOrigin::App, FaultCode::new("app.example.unknown"), format!("no registered example '{example_id}'"))),
        })
    }
    async fn initial_snapshot() -> Self::Snapshot {
        E::initial_snapshot()
    }
    async fn initial_config() -> Self::Config {
        E::initial_config()
    }
    fn host_configuration_mutation(action: &str, args: Option<&DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        E::host_configuration_mutation(action, args)
    }
    fn typing_fold(net: &[Self::Mutation], next: &[Self::Mutation]) -> semio_framework_tool_machine::TypingFold<Self::Mutation> {
        E::typing_fold(net, next)
    }
    fn entity_label(snapshot: &Self::Snapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        E::entity_label(snapshot, kinds, id)
    }
    fn selection_reference_id(kinds: &[String], row: &str) -> Option<String> {
        E::selection_reference_id(kinds, row)
    }
    fn tool_intent_kinds(tool: &str) -> &'static [&'static str] {
        E::tool_intent_kinds(tool)
    }
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        E::fault_notices()
    }
    async fn initial_draft() -> Self::Draft {
        E::initial_draft()
    }
    async fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        interaction: &InteractionView<'_>,
        view_state: Option<&ViewModel>,
        draft: &DraftView<'_, Self::Draft>,
        engines: &EngineHandles,
    ) -> ArtifactMutationOutcome<Self::Mutation, Self::ConfigMutation, Self::DraftMutation> {
        E::handle(command, doc, cfg, interaction, view_state, draft, engines)
    }
    async fn command_id(command: &Self::Command) -> &'static str {
        E::command_id(command)
    }
    async fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        E::command_from_action(action, args, original_fault)
    }
    async fn agent_target_revision(action: &str, args: &DslValue, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
        E::agent_target_revision(action, args, doc)
    }

    async fn config_spec() -> ConfigSpec {
        E::config_spec()
    }
    async fn clipboard_media_type() -> Option<MediaType> {
        E::clipboard_media_type()
    }
    async fn clipboard_accepts() -> Vec<MediaType> {
        E::clipboard_accepts()
    }
    async fn copy_fragment(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        E::copy_fragment(doc, cfg, interaction)
    }
    async fn cut_operations(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, interaction: &InteractionView<'_>) -> Vec<Self::Mutation> {
        E::cut_operations(doc, cfg, interaction)
    }
    async fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, ValueError> {
        E::interaction_topology(doc, cfg)
    }
    async fn paste_operations(doc: &ArtifactView<'_, Self::Snapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Self::Mutation>, ClipboardError> {
        E::paste_operations(doc, fragment, placement)
    }
    async fn pending_effects(owner: &ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view: Option<&ViewModel>) -> Vec<Effect> {
        E::pending_effects(owner, doc, cfg, view)
    }
    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        E::render(body_key, doc, cfg, view_state)
    }
    async fn render_with_instance_operation_owner(
        owner: &ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
    ) -> UiAssemblyResult<ComponentTree> {
        E::render_with_instance_operation_owner(owner, body_key, doc, cfg, view_state)
    }
    async fn render_with_request_context(
        owner: &ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> crate::OriginalRenderResult<ComponentTree> {
        E::render_with_request_context(owner, body_key, doc, cfg, view_state, transient, interaction,cx)
    }
    async fn window_engagements(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        E::window_engagements(doc, cfg, view_state)
    }
    async fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        E::window_engagements_with_request_context(doc, cfg, view_state, transient, interaction)
    }
    async fn window_measures(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        E::window_measures(doc, cfg, view_state)
    }
    async fn window_measures_with_request_context(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel, interaction: &InteractionView<'_>) -> HashMap<String, Vec<WindowMeasure>> {
        E::window_measures_with_request_context(doc, cfg, view_state, interaction)
    }
    async fn tool_measures(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        E::tool_measures(doc, cfg, view_state)
    }
    async fn app_catalogue_json() -> String {
        E::app_catalogue_json()
    }
    async fn context_menu(request: &ContextMenuRequest, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        E::context_menu(request, doc, cfg, view_state, registry)
    }
    async fn context_menu_with_request_context(
        request: &ContextMenuRequest,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        interaction: &InteractionView<'_>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        E::context_menu_with_request_context(request, doc, cfg, view_state, interaction, registry)
    }
    async fn genesis() -> Vec<Self::Mutation> {
        E::genesis()
    }
    async fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        E::app_schema()
    }
    async fn io() -> Option<AppIo> {
        E::io()
    }
    async fn natural_file_codec() -> Option<NaturalFileCodec> {
        E::natural_file_codec()
    }
    async fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, MediaError> {
        E::encode_natural_file(snapshot)
    }
    async fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, MediaError> {
        E::decode_natural_file(bytes)
    }
    async fn media_ports() -> Vec<MediaPortSpec> {
        E::media_ports().await
    }
    async fn export_media(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Media, MediaError> {
        E::export_media(port, doc)
    }
    async fn export_media_with_request_context(owner: &ArtifactInstanceOperationOwnerHandle, port: &str, doc: &ArtifactView<'_, Self::Snapshot>, transient: &TransientView<'_, Self::Transient>) -> Result<Media, MediaError> {
        E::export_media_with_request_context(owner, port, doc, transient)
    }
    async fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Self::Snapshot>) -> ArtifactMutationOutcome<Self::Mutation, Self::ConfigMutation, Self::DraftMutation, MediaError> {
        E::import_media(port, media, doc)
    }
    async fn media_fingerprint(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<MediaFingerprint, MediaError> {
        E::media_fingerprint(port, doc)
    }
}

/// 🗄️👁️ Adapter binding an `ArtifactViewer` author into the runtime `ArtifactApp` contract
/// (contract §2.1/§2.2). Draft-lane and clipboard/genesis/import methods are left to
/// `ArtifactApp`'s own defaults (empty/no-op) since `ArtifactViewer` never declares them — a
/// viewer never originates or imports a document. `handle` is the one real seam: it converts
/// `V::handle`'s `ViewEmit` into an `Emit` whose `artifact_mutations`/`draft_mutations` are always
/// empty BY CONSTRUCTION, built solely from `ViewEmit`'s three fields (contract §2.2) — never from
/// a caller-supplied mutation vector, so a hand-written runtime impl cannot smuggle one in either.
pub struct ViewerApp<V: ArtifactViewer> {
    /// 🪪️ See `EditorApp::id` — same placeholder/derivation split applies here.
    id: String,
    _viewer: std::marker::PhantomData<V>,
}

impl<V: ArtifactViewer> Default for ViewerApp<V> {
    fn default() -> Self {
        Self { id: surface_app_id(&V::DIALECT.into(), V::ROLE), _viewer: std::marker::PhantomData }
    }
}

impl<V: ArtifactViewer> ViewerApp<V> {
    /// 🪪️ The real derived surface id — see `EditorApp::surface_id`.
    pub async fn surface_id(&self) -> &str {
        &self.id
    }
}

impl<V: ArtifactViewer> ArtifactApp for ViewerApp<V> {
    fn owned_mutation_batch_birth_bytes() -> Option<usize> { V::owned_mutation_batch_birth_bytes() }
    fn completion_retirement_birth_bytes(value:&ArtifactToolCompletionValue<Self>)->Option<usize>{V::completion_retirement_birth_bytes(value)}
    fn admit_completion_retirement(value:&mut Option<ArtifactToolCompletionValue<Self>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{V::admit_completion_retirement(value,grant)}

    fn admit_owned_mutation_batch(values: &mut Option<Vec<Self::Mutation>>, grant: RetainedCloneGrant) -> Result<Option<(store::MemberStoreOwnedBatch, RetainedCloneProgress)>, ValueError> { V::admit_owned_mutation_batch(values, grant) }

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        V::build_envelope_decode_owner_bundle()
    }

    fn build_document_store_initialization_job(
        envelope: ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: ActorId,
    ) -> ArtifactInitializationAdmission<Self::Snapshot, Self::Mutation> {
        V::build_document_store_initialization_job(envelope, operation, generation, actor)
    }

    fn document_store_owners_source_demands()->Result<RetirementDemand,ValueError>{V::document_store_owners_source_demands()}
    fn build_document_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        V::build_document_store_owners(grant)
    }

    fn config_store_owners_source_demands()->Result<RetirementDemand,ValueError>{V::config_store_owners_source_demands()}
    fn build_config_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Config, Self::ConfigMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Config, Self::ConfigMutation>>> {
        V::build_config_store_owners(grant)
    }

    fn build_document_store_disposer() -> ArtifactDisposal<ArtifactStore<Self::Snapshot, Self::Mutation>> {
        V::build_document_store_disposer()
    }

    fn build_config_store_disposer() -> ArtifactDisposal<ConfigStore<Self::Config, Self::ConfigMutation>> {
        V::build_config_store_disposer()
    }

    fn build_presence_store_disposer() -> ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        V::build_presence_store_disposer()
    }

    fn build_transient_store_disposer() -> ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        V::build_transient_store_disposer()
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        V::build_presence_peer_retirement_factory()
    }

    fn draft_store_owners_source_demands()->Result<RetirementDemand,ValueError>{store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<Self::Draft,Self::DraftMutation>()}
    fn build_draft_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Draft, Self::DraftMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Draft, Self::DraftMutation>>> {
        Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<NoDraft, NoDraftMutation>(grant))
    }

    fn build_draft_store_disposer() -> ArtifactDisposal<store::DraftStore<Self::Draft, Self::DraftMutation>> {
        Some(bounded_document_store_disposer::<NoDraft, NoDraftMutation>())
    }

    const DIALECT: Dialect = V::DIALECT;
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        V::child_restore_projection(snapshot)
    }
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, ValueError> {
        V::genesis_child_pack(snapshot, slot, child_id)
    }
    const APP_ID: &'static str = "surface";
    const ROLE: AppRole = V::ROLE;
    const DOCUMENT_SCHEMA: &'static str = V::DOCUMENT_SCHEMA;

    /// 🪪️ See `EditorApp::instance_id` — same placeholder-override reasoning applies here.
    async fn instance_id(&self) -> &str {
        self.surface_id().await
    }
    fn mounted_job_maintenance_phase(instance_id: u32) -> Option<MountedOwnerPhaseV1> { V::mounted_job_maintenance_phase(instance_id) }
    fn mounted_job_maintenance_demands(instance_id: u32, body: usize) -> Result<RetirementDemand, ValueError> { V::mounted_job_maintenance_demands(instance_id, body) }
    fn mounted_job_close_demands(instance_id: u32, body: usize) -> Result<RetirementDemand, ValueError> { V::mounted_job_close_demands(instance_id, body) }
    fn mounted_job_maintenance_step(instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,crate::bounded_ownership::PluginLifecycleFailure> {
        V::mounted_job_maintenance_step(instance_id,cx,grant)
    }
    fn mounted_job_close_step(instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,crate::bounded_ownership::PluginLifecycleFailure> {
        V::mounted_job_close_step(instance_id,cx,grant)
    }
    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        V::mounted_jobs_terminal_is_empty(instance_id)
    }
    fn mounted_job_prepare_snapshot_read(operation: AppRenderOperationContext, snapshot: &Self::Snapshot) -> bool {
        V::mounted_job_prepare_snapshot_read(operation, snapshot)
    }
    fn register_window_transient_owners(registry: &mut WindowTransientOwnerRegistry) -> Result<(), Fault> {
        V::register_window_transient_owners(registry)
    }
    fn register_window_config_owners(registry: &mut WindowConfigOwnerRegistry) -> Result<(), Fault> {
        V::register_window_config_owners(registry)
    }
    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        V::bounded_first_step_tool_proofs()
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
        V::register_tool_job_factories(registry)
    }
    fn build_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<ToolRunJob>, Fault> {
        V::build_tool_run_job(request)
    }
    fn build_retargetable_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<Box<dyn ToolRunRetargetableJob<Self::Config>>>, Fault> {
        V::build_retargetable_tool_run_job(request)
    }
    fn retained_window_transient_target(command: &Self::Command) -> Option<(&str, &'static str)> {
        V::retained_window_transient_target(command)
    }
    fn interaction_scope(verb: InteractionVerb, domains: &[&str]) -> Option<UiDirtyScope> {
        V::interaction_scope(verb, domains)
    }
    fn operation_progress_scope() -> UiDirtyScope {
        V::operation_progress_scope()
    }
    fn host_event(event: &HostEvent) -> Option<Self::Command> {
        V::host_event(event)
    }
    fn build_instance_operation_owner() -> Box<dyn ArtifactInstanceOperationOwner> {
        V::build_instance_operation_owner()
    }
    async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        V::build_tool_job(request)
    }
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<Self>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        V::build_reserved_tool_job(request)
    }
    fn build_media_export_job(request: ArtifactMediaExportJobRequest<Self>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        V::build_media_export_job(request)
    }
    fn build_snapshot_disposer() -> Option<Box<dyn ArtifactSnapshotDisposer<Self::Snapshot>>> {
        V::build_snapshot_disposer()
    }
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        V::build_config_store_one_item_preparation_factory()
    }
    fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        V::build_presence_store_one_item_preparation_factory()
    }
    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        V::build_transient_store_one_item_preparation_factory()
    }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        V::build_presence_local_root_retirement_factory()
    }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        V::build_transient_local_root_retirement_factory()
    }
    type Snapshot = V::Snapshot;
    type Mutation = V::Mutation;
    type Config = V::Config;
    type ConfigMutation = V::ConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = V::Presence;
    type PresenceMutation = V::PresenceMutation;
    type Transient = V::Transient;
    type TransientMutation = V::TransientMutation;
    type Command = V::Command;

    async fn ephemeral(command: &Self::Command, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, presence: &PresenceView<'_, Self::Presence>, transient: &TransientView<'_, Self::Transient>) -> EphemeralEmit<Self> {
        let (presence_mutations, transient_mutations) = V::ephemeral(command, doc, cfg, presence, transient);
        EphemeralEmit { presence: presence_mutations, transient: transient_mutations, window_transient: Vec::new() }
    }
    async fn config_schema() -> &'static str {
        V::config_schema()
    }
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        V::fault_notices()
    }
    async fn initial_snapshot() -> Self::Snapshot {
        V::initial_snapshot()
    }
    async fn initial_config() -> Self::Config {
        V::initial_config()
    }
    async fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        interaction: &InteractionView<'_>,
        view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        engines: &EngineHandles,
    ) -> ArtifactMutationOutcome<Self::Mutation, Self::ConfigMutation, Self::DraftMutation> {
        let view_emit = V::handle(command, doc, cfg, interaction, view_state, engines)?;
        Ok(Emit { config_mutations: view_emit.config_mutations, effects: view_emit.effects, ui_scope: view_emit.ui_dirty, ..Default::default() })
    }
    async fn command_id(command: &Self::Command) -> &'static str {
        V::command_id(command)
    }
    async fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        V::command_from_action(action, args, original_fault)
    }
    async fn config_spec() -> ConfigSpec {
        V::config_spec()
    }
    async fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, ValueError> {
        V::interaction_topology(doc, cfg)
    }
    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        V::render(body_key, doc, cfg, view_state)
    }
    async fn render_with_request_context(
        owner: &ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> crate::OriginalRenderResult<ComponentTree> {
        V::render_with_request_context(owner, body_key, doc, cfg, view_state, transient, interaction,cx)
    }
    async fn pending_effects(owner: &ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view: Option<&ViewModel>) -> Vec<Effect> {
        V::pending_effects(owner, doc, cfg, view)
    }
    async fn window_engagements(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        V::window_engagements(doc, cfg, view_state)
    }
    async fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        V::window_engagements_with_request_context(doc, cfg, view_state, transient, interaction)
    }
    async fn window_measures(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        V::window_measures(doc, cfg, view_state)
    }
    async fn window_measures_with_request_context(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel, interaction: &InteractionView<'_>) -> HashMap<String, Vec<WindowMeasure>> {
        V::window_measures_with_request_context(doc, cfg, view_state, interaction)
    }
    async fn tool_measures(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        V::tool_measures(doc, cfg, view_state)
    }
    async fn app_catalogue_json() -> String {
        V::app_catalogue_json()
    }
    async fn context_menu(request: &ContextMenuRequest, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        V::context_menu(request, doc, cfg, view_state, registry)
    }
    async fn context_menu_with_request_context(
        request: &ContextMenuRequest,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        interaction: &InteractionView<'_>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        V::context_menu_with_request_context(request, doc, cfg, view_state, interaction, registry)
    }
    async fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        V::app_schema()
    }
    async fn io() -> Option<AppIo> {
        V::io()
    }
    async fn media_ports() -> Vec<MediaPortSpec> {
        V::media_ports().await
    }
    async fn export_media(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Media, MediaError> {
        V::export_media(port, doc)
    }
    async fn export_media_with_request_context(owner: &ArtifactInstanceOperationOwnerHandle, port: &str, doc: &ArtifactView<'_, Self::Snapshot>, transient: &TransientView<'_, Self::Transient>) -> Result<Media, MediaError> {
        V::export_media_with_request_context(owner, port, doc, transient)
    }
    async fn media_fingerprint(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<MediaFingerprint, MediaError> {
        V::media_fingerprint(port, doc)
    }
}

/// 🎭️👁️ Marker whose `builder` seeds a `ViewerBuilder` with the surface id/role/dialect already
/// derived (contract §2.4) — mirrors `App::builder`'s `(id, label) -> AppBuilder` shape but never
/// accepts a hand-written id.
pub struct Viewer;

/// 🎭️✏️ Editor twin of `Viewer`.
pub struct Editor;

impl Viewer {
    pub fn builder(dialect: Dialect) -> ViewerBuilder {
        let dialect: ArtifactDialect = dialect.into();
        let inner = Box::new(::semio_framework_async::poll::resolve_ready(AppBuilder::new(surface_app_id(&dialect, AppRole::Viewer), LocalizedLabel::native("Viewer", "Betrachter"))));
        ViewerBuilder { inner, dialect }
    }
}

impl Editor {
    pub fn builder(dialect: Dialect) -> EditorBuilder {
        let dialect: ArtifactDialect = dialect.into();
        let inner = Box::new(::semio_framework_async::poll::resolve_ready(AppBuilder::new(surface_app_id(&dialect, AppRole::Editor), LocalizedLabel::native("Editor", "Editor"))));
        EditorBuilder { inner, dialect }
    }
}

/// 👁️ Fluent viewer surface builder (contract §2.4) — every `AppBuilder` method except
/// `.mutation(...)`, which does not exist on this type: calling it is a compile error, not a
/// runtime rejection.
pub struct ViewerBuilder {
    inner: Box<AppBuilder>,
    dialect: ArtifactDialect,
}

/// ✏️ Fluent editor surface builder (contract §2.4) — every `AppBuilder` method, `.mutation(...)`
/// included.
/// 📦️ `inner` is BOXED on purpose. Every fluent method takes `self` by value and returns a new
/// surface builder, so an unoptimized build gives each step of a chain its own stack slot for the
/// whole returned value: a real app manifest chains ~250 steps, and an inline `AppBuilder` made
/// that 400 KiB of one frame — 20 % of a 2 MiB test thread spent before the app even exists, which
/// is why every whole-app law in `puzzle3d` ran at 70–95 % of its stack and the heaviest one
/// overflowed (ticket 26/09/02 wave F). Behind a `Box` a chain step costs a pointer, and the
/// `AppBuilder` value itself only ever lives in the frame of the one method that moves it.
pub struct EditorBuilder {
    inner: Box<AppBuilder>,
    dialect: ArtifactDialect,
}

impl ViewerBuilder {
    #[inline(never)]
    pub fn document<I, S>(mut self, document: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.inner = Box::new((*self.inner).document(document));
        self
    }
    #[inline(never)]
    pub fn terminology_document(mut self, id: impl Into<String>, document: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.inner = Box::new(::semio_framework_async::poll::resolve_ready((*self.inner).terminology_document(id, document)));
        self
    }
    pub fn build_definition(self) -> AppDefinition {
        let mut definition = (*self.inner).build_definition();
        definition.role = AppRole::Viewer;
        definition.dialect = self.dialect;
        definition
    }
}

impl EditorBuilder {
    #[inline(never)]
    pub fn document<I, S>(mut self, document: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.inner = Box::new((*self.inner).document(document));
        self
    }
    #[inline(never)]
    pub fn terminology_document(mut self, id: impl Into<String>, document: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.inner = Box::new(::semio_framework_async::poll::resolve_ready((*self.inner).terminology_document(id, document)));
        self
    }
    /// ✏️ Declares a document-mutating action — the one method `ViewerBuilder` doesn't have.
    #[inline(never)]
    pub fn mutation(mut self, id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        self.inner = Box::new(::semio_framework_async::poll::resolve_ready((*self.inner).mutation(id, label)));
        self
    }
    pub fn build_definition(self) -> AppDefinition {
        let mut definition = (*self.inner).build_definition();
        definition.role = AppRole::Editor;
        definition.dialect = self.dialect;
        definition
    }
}

/// 🏭️ Forwards the `AppBuilder` methods `ViewerBuilder`/`EditorBuilder` share verbatim (every
/// `AppBuilder` builder method except `new`/`build_definition`/`document`/`terminology_document`/
/// `mutation`, each handled above) — one macro invocation is the single source of truth for the
/// shared surface instead of two ~45-method hand-copied impls drifting apart.
macro_rules! surface_builder_forward {
    ( [ $($Type:ident),+ ] ; $methods:tt ) => {
        $(
            surface_builder_forward!(@type $Type; $methods);
        )+
    };
    ( @type $Type:ident; { $( $name:ident ( $( $arg:ident : $ty:ty ),* ) ),+ $(,)? } ) => {
        impl $Type {
            $(
                #[inline(never)]
                pub fn $name(mut self, $( $arg: $ty ),* ) -> Self {
                    self.inner = Box::new(::semio_framework_async::poll::resolve_ready((*self.inner).$name( $( $arg ),* )));
                    self
                }
            )+
        }
    };
}

surface_builder_forward!(
    [ViewerBuilder, EditorBuilder];
    {
    artifact_kind(spec: ArtifactKindSpec),
    config(spec: ConfigSpec),
    io(io: AppIo),
    command_grammar(grammar: CommandGrammar),
    media_input(spec: MediaPortSpec),
    media_output(spec: MediaPortSpec),
    terminology(id: impl Into<String>),
    introduction(introduction: IntroductionDefinition),
    tutorial(tutorial: TutorialDefinition),
    dialog(dialog: DialogDefinition),
    icon_id(icon_id: impl Into<IconName>),
    mode(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>),
    mode_commands(mode_id: impl AsRef<str>, commands: Vec<CommandDefinition>),
    mode_command(mode_id: impl AsRef<str>, command: CommandDefinition),
    mode_layout(mode_id: impl AsRef<str>, layout_id: impl Into<String>),
    mode_tools(mode_id: impl AsRef<str>, tool_ids: Vec<ToolRef>),
    default_mode_id(id: impl Into<String>),
    window_kind(id: impl Into<String>, label: impl Into<LocalizedLabel>, body_key: impl Into<String>, surface_kind: SurfaceKind, icon_id: impl Into<IconName>),
    window_kind_with_engagement(id: impl Into<String>, label: impl Into<LocalizedLabel>, body_key: impl Into<String>, surface_kind: SurfaceKind, engagement: WindowEngagement, icon_id: impl Into<IconName>),
    mode_def(def: ModeDefinition),
    window_kind_def(def: WindowKindDefinition),
    panel_tab_def(def: PanelTabDefinition),
    window_kind_measures(window_kind_id: impl AsRef<str>, measures: Vec<WindowMeasure>),
    window_kind_actions(window_kind_id: impl AsRef<str>, actions: Vec<ActionDefinition>),
    window_kind_action_refs(window_kind_id: impl AsRef<str>, action_refs: Vec<ActionRef>),
    window_kind_utilities(window_kind_id: impl AsRef<str>, utility_ids: Vec<UtilityRef>),
    window_kind_initial_utility(window_kind_id: impl AsRef<str>, utility_id: impl Into<UtilityRef>),
    window_kind_interactions(window_kind_id: impl AsRef<str>, interaction_ids: Vec<InteractionRef>),
    named_layout(layout: NamedLayout),
    default_layout(layout: WindowLayout),
    panel_tab(id: impl Into<String>, label: impl Into<LocalizedLabel>, group: PanelGroup, body_key: impl Into<String>),
    panel_tab_tree(tab: PanelTabSpec),
    panel_tab_framework(tab: PanelTabSpec),
    keybinding(keys: impl Into<String>, action: impl Into<String>),
    view_action(id: impl Into<String>, label: impl Into<LocalizedLabel>),
    shell_action(id: impl Into<String>, label: impl Into<LocalizedLabel>),
    action_with(action: ActionDefinition),
    action_args(action_id: impl AsRef<str>, args: Vec<ActionArgDef>),
    action_describe(action_id: impl AsRef<str>, description: impl Into<LocalizedLabel>),
    action_use_when(action_id: impl AsRef<str>, phrases: Vec<String>),
    action_audience(action_id: impl AsRef<str>, audience: CapabilityAudience),
    action_destructive(action_id: impl AsRef<str>),
    action_interactive_job(action_id: impl AsRef<str>, classification: semio_framework::InteractiveJobClassification),
    interactive_jobs(classification: semio_framework::InteractiveJobClassification),
    command(command: CommandDefinition),
    app_command(id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>, kind: ActionKind),
    command_args(command_id: impl AsRef<str>, args: Vec<ActionArgDef>),
    utility(utility: UtilityDefinition),
    interaction(interaction: InteractionDefinition),
    utility_simple(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>),
    tool(tool: ToolDefinition),
    tool_simple(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>),
    }
);

/// 🏗️ A generic [`ArtifactBuilder`] for a subset with no custom build logic — everything is one
/// snapshot value and the ordinary `Mutation`/`MutationDiff` algebra, so a trivial subset writes
/// zero builder boilerplate (Task 3): `type Construction = SnapshotBuilder<Snapshot, Mutation>;`
/// where old code hand-wrote its own `empty`/`from_snapshot`/`from_text`/`from_binary`/`mutate`/
/// `absorb`/`build`. `mutate` reuses `store::os_store::replay::apply_outcome` (this crate's own established
/// "apply once, fold a rejection into a Fatal message" idiom) rather than re-deriving it, matching
/// the trait's own doc: "the builder applies `outcome.diff()` once and returns that outcome intact".
pub struct SnapshotBuilder<S, M> {
    snapshot: S,
    _mutation: std::marker::PhantomData<fn() -> M>,
}

impl<S, M> SnapshotBuilder<S, M> {
    fn wrap(snapshot: S) -> Self {
        Self { snapshot, _mutation: std::marker::PhantomData }
    }
}

impl<S, M> ArtifactBuilder for SnapshotBuilder<S, M>
where
    S: Default + Clone + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot,
    M: Mutation<S>,
    M::Diff: MutationDiff<S>,
{
    type Snapshot = S;
    type Mutation = M;
    type Diff = M::Diff;

    fn empty() -> Self {
        Self::wrap(S::default())
    }
    fn from_snapshot(snapshot: S) -> Self {
        Self::wrap(snapshot)
    }
    fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Ok(Self::wrap(S::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::wrap(S::decode_pack(bytes)?))
    }
    fn mutate(self, mutation: M) -> (Self, protocol::MutationOutcome<M::Diff>) {
        let (snapshot, outcome) = store::os_store::replay::apply_outcome(&self.snapshot, mutation.diff(&self.snapshot));
        (Self::wrap(snapshot), outcome)
    }
    fn absorb(self, diff: M::Diff) -> protocol::MutationApplyResult<Self> {
        match protocol::apply_diff(&diff, &self.snapshot) {
            Ok(snapshot) => Ok(Self::wrap(snapshot)),
            Err(error) => Err(error),
        }
    }
    fn build(self) -> Result<S, Vec<semio_framework_diagnostic::Diagnostic>> {
        Ok(self.snapshot)
    }
}

fn subset_example_table() -> &'static std::sync::Mutex<HashMap<Dialect, Vec<ExampleSource>>> {
    static TABLE: std::sync::OnceLock<std::sync::Mutex<HashMap<Dialect, Vec<ExampleSource>>>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| std::sync::Mutex::new(HashMap::new()))
}

pub(crate) fn remember_subset_examples(dialect: Dialect, examples: &[ExampleSource]) {
    if examples.is_empty() {
        return;
    }
    subset_example_table().lock().expect("subset example catalogue").insert(dialect, examples.to_vec());
}

/// 📚️ Examples a subset declaration registered for `dialect`.
fn subset_examples(dialect: &Dialect) -> Vec<ExampleSource> {
    subset_example_table().lock().expect("subset example catalogue").get(dialect).cloned().unwrap_or_default()
}
