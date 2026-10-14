//! 🧩️ The app contract: `ArtifactApp`, media artifacts, `PluginApp` and the action registry.
use semio_framework::HISTORY_EDIT_ACTION_IDS;
use crate::window_kits::AppActionRegistry;
use crate::MountedOwnerPhaseV1;
use super::transient_publication::bounded_transient_store_disposer;
use super::window_config::WindowConfigOwnerRegistry;
use super::window_transient::WindowTransientOwnerRegistry;
use semio_framework_value::DslValue;
use semio_framework::kernel::UiDirtyScope;
/// 🪪️ Declarative app manifest shared by plugin builders and surface registrations.
use semio_framework::AppDefinition;
use semio_framework::{kernel::{ActorId, ClipboardError, ClipboardFragment, Effect, PastePlacement}, ActionArgDef, ActionDefinition, ActionKind, AppIo, ArgPresentation, CapabilityAudience, ConfigSpec, Fault, FaultCode, FaultOrigin, InteractionVerb, MediaForm, MediaPortSpec, ViewModel, REVERT_TO_COMMAND_ACTION_ID, OPEN_ARTIFACT_FILE_ACTION_ID, SAVE_ARTIFACT_FILE_ACTION_ID};
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
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::HashMap;
use pack::snapshot::ArtifactPack;
use store::{os_store::store_schema::ArtifactEnvelope, ArtifactStore, ConfigStore};
use semio_framework_ui_locale::LocalizedLabel;
use ui_wgpu::wgpu::ContextMenuItemSpec;
use ui_wgpu::wgpu::ContextMenuRequest;
use ui_wgpu::wgpu::WindowEngagement;
use ui_wgpu::wgpu::WindowMeasure;
/// 🎞️ `MediaWireFormat` backs `MediaArtifactDescriptor::wire` (see `🔖️DocumentContract`
/// below) — the plugin ABI's `consume-media`/`produce-media` payload framing, separate from
/// `MediaPayload` above (which pairs with `Media`'s per-port `MediaType` projection).
/// The legacy format enum was retired in ticket 26/08/11/
/// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6 — `MediaWireFormat::
/// Binary` now carries a plain format kind id string.
use semio_framework::MediaWireFormat;
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
use semio_framework_artifact_reference::Dialect;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
use semio_framework::AppRole;
use crate::app_emission::Emit;
use crate::app_emission::EphemeralEmit;
use crate::app_emission::HostEvent;
use crate::app_views::AppRenderOperationContext;
use crate::app_views::ArtifactView;
use crate::app_views::ConfigView;
use crate::app_views::DraftView;
use crate::app_views::InteractionView;
use crate::app_views::PresenceView;
use crate::window_transient::TransientView;
use crate::bounded_ownership::PluginLifecycleStep;
use crate::media_contract::ArtifactMediaExportJobRequest;
use crate::tool_contract::ArtifactReservedToolJob;
use crate::sdk_fault::plugin_sdk_fault;
use crate::store_lifecycle::ArtifactDisposal;
use crate::store_lifecycle::ArtifactInitializationAdmission;
use crate::app_emission::ArtifactMutationOutcome;
use crate::store_lifecycle::bounded_config_store_disposer;
use crate::store_lifecycle::bounded_document_store_disposer;
use crate::store_lifecycle::bounded_document_store_initialization_job;
use crate::tool_contract::ArtifactBoundedFirstStepProof;
use crate::tool_contract::ArtifactInstanceOperationOwner;
use crate::tool_contract::ArtifactInstanceOperationOwnerHandle;
use crate::tool_contract::ArtifactOwnedToolJobRequest;
use crate::tool_contract::ArtifactReservedToolJobRequest;
use crate::tool_contract::ArtifactToolCompletionValue;
use crate::tool_contract::ArtifactToolFactoryRegistry;
use crate::tool_contract::EmptyArtifactInstanceOperationOwner;
use crate::tool_run_contract::ToolRunJob;
use crate::tool_run_contract::ToolRunJobRequest;
use crate::tool_run_contract::ToolRunRetargetableJob;

use crate::window_kits::merge_ui_values;
use crate::window_surface::UiAssemblyResult;

/// 🧩️ Typed, per-app author surface. An app declares its `Snapshot` and `Mutation` (a
/// `store::Mutation<Snapshot>`), mutates nothing directly, and returns an {@link ActionEmit} whose
/// operations flow through a persistent `ArtifactStore` owned by {@link VcsArtifactApp}. Ephemeral
/// view state (selection/camera/active utility) lives in the app struct itself, not in the document.
///
/// # 🔖️ToolContract
/// The formalized actions-vs-utilities contract:
/// - **Actions** are non-interactive: they carry optional declared `ActionArgDef`s, stage in the
///   renderer, and execute once. `Mutation`-kind actions emit operations; `View`/`🐚️Shell`-kind actions must
///   emit **zero** operations ({@link VcsArtifactApp} enforces this — a View/Shell action returning operations is a
///   hard error).
/// - **Utilities** are interactive live-preview pointer modes. Exactly one utility is active per window kind;
///   the active utility arrives via `view_state.active_utility_id` and is **never** stored in the document
///   nor emitted as an operation. Switching utilities dispatches the framework `setActiveUtility` View action; on a
///   switch the app must clear any in-progress preview scratch.
/// - **Gestures are tool machines** (design §5, §15): a press-drag-release drives a persisted `ToolMachineRunner`
///   through the host's `phase` (`stream` / `commit` / `abort` + `reason`; none = one-shot), previews from a
///   local-only transient and yields its relative leaves as ONE transaction through
///   {@link ActionEmit::commit_transaction}; an abort leaves zero trace. No gesture bracket verb exists.
/// - The pointer vocabulary (`canvasPointerDown/Move/Up`, `worldPointerDown/Move/Up`) are internal action ids
///   driving the above.
pub trait ArtifactApp: Default + Send + 'static {
    /// 📬️ Measures the original typed completion's exact app-owned cleanup constructor.
    fn completion_retirement_birth_bytes(value:&ArtifactToolCompletionValue<Self>)->Option<usize>{super::completion_retirement::derived_completion_birth_bytes(value)}
    /// 🎟️ Admits the original completion payload without consuming it on an unfunded or unsupported route.
    fn admit_completion_retirement(value:&mut Option<ArtifactToolCompletionValue<Self>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{super::completion_retirement::derived_admit_completion_retirement(value,grant)}
    /// 🎟️ Declares the exact first-party scaffold birth for an applying parent mutation source.
    fn owned_mutation_batch_birth_bytes()->Option<usize>{None}
    /// 🪵️ Retains the original parent source on every refused or unfunded admission.
    fn admit_owned_mutation_batch(_values:&mut Option<Vec<Self::Mutation>>,_grant:RetainedCloneGrant)->Result<Option<(store::MemberStoreOwnedBatch,RetainedCloneProgress)>,ValueError>{Ok(None)}
    /// 🪪️ Exact author-owned coordinate stamped into every document envelope.
    const DIALECT: Dialect;
    /// 🧩️ Projects actual loaded child references; an undeclared owner grants no restore authority.
    fn child_restore_projection(_snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        Err(plugin_sdk_fault("app did not declare a loaded-parent child projection"))
    }
    /// 🌱️ Initial pack for one composed child the parent snapshot declares (`slot`/`child_id`) but no
    /// archive member carries — the content-addressed half a parent derives from its own state, such as
    /// sourcing's kit catalogue. `None` keeps the closure strict: a missing member the app cannot derive
    /// still rejects the load as `Incomplete`. Read at boot for the initial snapshot and by every
    /// document archive load before its member roster is reserved.
    fn genesis_child_pack(_snapshot: &Self::Snapshot, _slot: &str, _child_id: &str) -> Result<Option<Vec<u8>>, ValueError> {
        Ok((|| None)())
    }
    /// 🪪 Stable app id — prefer this over `app_id(&self)` on the path to receiverless ZSTs.
    /// For a hand-written direct `ArtifactApp` impl this IS the real canonical id. `EditorApp<E>`/
    /// `ViewerApp<V>` cannot follow suit — `surface_app_id` is only knowable at runtime from
    /// `E::DIALECT`/`E::ROLE` — so they keep this a fixed placeholder and override `instance_id`
    /// below instead; every ownership check MUST read `instance_id`, never this const, or it
    /// silently reduces to comparing every surface against the same placeholder (ticket
    /// 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-G).
    const APP_ID: &'static str;
    /// 🔐️ Marks domains whose atomic replacement must never use the unmanaged peer path.
    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = false;
    /// 🪪️ The real, runtime-resolved app id an ownership check must compare against. Defaults to
    /// `Self::APP_ID` (correct for every hand-written impl, where the const already IS the real
    /// id); `EditorApp<E>`/`ViewerApp<V>` override it to return their derived `surface_app_id`.
    async fn instance_id(&self) -> &str {
        Self::APP_ID
    }
    /// 📜️ Exact owner-local bounded reducer proofs. Empty grants no execution authority.
    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        Vec::new()
    }
    /// 🧹️ Advances one owner-local mounted background-job cleanup opportunity while the
    /// application remains live.
    fn mounted_job_maintenance_phase(_instance_id: u32) -> Option<MountedOwnerPhaseV1> { None }

    /// 📏️ Quotes the exact next [`Self::mounted_job_maintenance_step`] turn of the owner-local mounted jobs.
    fn mounted_job_maintenance_demands(_instance_id: u32, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn mounted_job_maintenance_step(_instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,_grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,crate::bounded_ownership::PluginLifecycleFailure> {
        Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()))
    }
    /// 📏️ Quotes the exact next [`Self::mounted_job_close_step`] turn of the owner-local mounted jobs.
    fn mounted_job_close_demands(_instance_id: u32, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }
    /// 🧹️ Advances one owner-local mounted background-job close opportunity.
    fn mounted_job_close_step(_instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,_grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,crate::bounded_ownership::PluginLifecycleFailure> {
        Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()))
    }
    /// 🧹️ Exact terminal witness paired with `mounted_job_close_step`. It is also what keeps an app
    /// with owner-local cleanup still owing OUT of `maintenance_step`'s idle fast path: that path
    /// leaves the round-robin stage cursor untouched, so an app answering `true` here while it still
    /// held work never reached its own stage 15 again — the incremental teardown would simply never
    /// run, which is when it is needed most (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B42).
    fn mounted_jobs_terminal_is_empty(_instance_id: u32) -> bool {
        true
    }
    /// 🔍️ Requests an opaque immutable snapshot lease only when the mounted owner has
    /// admitted genuinely new work. The default path issues no lease.
    fn mounted_job_prepare_snapshot_read(_operation: AppRenderOperationContext, _snapshot: &Self::Snapshot) -> bool {
        false
    }
    /// 🧩️ Registers only app-owned exact factories. The empty default contributes no execution
    /// authority; implementations must bind every factory's `Owner` associated type to `Self`.
    fn register_tool_job_factories(_registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
        Ok(())
    }
    /// ⏯️ Builds one run or revalidate job of a tool run (contract §3.7); `None` faults the run.
    fn build_tool_run_job(_request: ToolRunJobRequest<'_, Self>) -> Result<Option<ToolRunJob>, Fault> {
        Ok(None)
    }
    /// 🎯️ Builds a job that honours the in-place retarget hooks (`ToolRunRetargetableJob`); the driver asks
    /// this first and falls back to `build_tool_run_job` on `None`.
    fn build_retargetable_tool_run_job(_request: ToolRunJobRequest<'_, Self>) -> Result<Option<Box<dyn ToolRunRetargetableJob<Self::Config>>>, Fault> {
        Ok(None)
    }
    fn register_window_transient_owners(_registry: &mut WindowTransientOwnerRegistry) -> Result<(), Fault> {
        Ok(())
    }
    fn register_window_config_owners(_registry: &mut WindowConfigOwnerRegistry) -> Result<(), Fault> {
        Ok(())
    }
    /// 🎯️ Selects a concrete attached window as the retained command's window-transient owner.
    /// The runtime validates the id and expected kind against the command's trusted ViewModel
    /// roster before it captures mutation authority for the current app and document.
    fn retained_window_transient_target(_command: &Self::Command) -> Option<(&str, &'static str)> {
        None
    }
    /// 🐢️ What one framework-owned interaction verb (`InteractionVerb`) dirties in THIS app,
    /// for the domains it actually touched — the app half of `dispatch_interaction_action`'s
    /// refresh scope. `None` means "not declared", and the framework then DERIVES the scope from the
    /// app's own surface declarations (`semio_framework::interaction_declared_refresh_scope`: the
    /// bodies of the window kinds that declare a touched domain, plus every panel body for the three
    /// verbs that move a selection), keeping the widest, always-correct answer
    /// ([`UiDirtyScope::Full`]) only when not even that is declared.
    ///
    /// 🌪️ Why this is a hook and not a constant: `interactionHover` fires on pointer motion, so a
    /// blanket `Full` makes every mouse move repaint every window body, every panel body, the
    /// utilities/tools/engagements rails, labels and measures — measured at 24 interaction
    /// ingresses and 3 whole-shell completions per 80 s lane, which is what a document mutation's
    /// own refresh pass then queues behind (ticket 26/09/02 `📓️2026-09-12-wave-B32-…md` §3.2).
    /// Only the app knows which of its bodies render a hover or a selection, so only the app can
    /// answer; the framework owns the verbs and the default.
    fn interaction_scope(_verb: InteractionVerb, _domains: &[&str]) -> Option<UiDirtyScope> {
        None
    }
    /// ⏳️ What a typed operation's progress change or retirement dirties in THIS app: the bodies that render
    /// `ArtifactView::operations`. Only the app knows which of its bodies show them, so the default refreshes
    /// nothing — an operation's progress never widens the dirty scope its own result declared.
    fn operation_progress_scope() -> UiDirtyScope {
        UiDirtyScope::None
    }
    /// 📨️ The typed command this app answers a [`HostEvent`] with: the runtime dispatches it to the event's window
    /// through the ordinary typed pipeline, and a command that edits no store is never a history row. `None` (the
    /// default) ignores the event.
    fn host_event(_event: &HostEvent) -> Option<Self::Command> {
        None
    }
    /// 🧳️ Builds one operation owner retained by the concrete VCS app instance. The owner is
    /// never process-global and closes only after every mounted typed command has retired.
    fn build_instance_operation_owner() -> Box<dyn ArtifactInstanceOperationOwner> {
        Box::new(EmptyArtifactInstanceOperationOwner)
    }
    /// 🏗️ Builds one payload-owning operation for an explicitly registered app factory. Returning
    /// `None` is fail-closed; the framework never substitutes its bounded-first-step factory.
    async fn build_tool_job(_request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        Ok(None)
    }
    /// 🧭️ Builds a persistent producer/import job for one explicitly registered reserved route.
    /// The empty default grants no route authority and never calls legacy monolithic hooks.
    fn build_reserved_tool_job(_request: ArtifactReservedToolJobRequest<Self>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        Ok(None)
    }
    /// 🎞️ Builds an owned resumable producer for an explicitly registered media-output port.
    /// The completed `Media` crosses back to the host only after the job's fresh commit boundary.
    fn build_media_export_job(_request:&mut Option<ArtifactMediaExportJobRequest<Self>>,_body:&mut Option<Box<dyn crate::tool_contract::ArtifactReservedJob+Send>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"registered media output lacks its concrete source-preserving producer").into())
    }


    /// 🧬️ Installs the exact domain-owned catalog and nested retirement factories for
    /// retained envelope decode. `None` grants no decoder authority.
    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        None
    }

    /// 🔐️ Supplies the exact snapshot/mutation/store disposal catalog used by document
    /// replacement and close. Framework-owned by default, paired with
    /// [`Self::build_document_store_disposer`]'s default: that disposer drives the store's
    /// installed owner catalog, so a default disposer over no catalog faulted every close with
    /// `artifact store has no owner-supplied bounded disposer`. `None` keeps both routes fail closed.
    fn document_store_owners_source_demands()->Result<RetirementDemand,ValueError>{store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<Self::Snapshot,Self::Mutation>()}
    fn build_document_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<Self::Snapshot, Self::Mutation>(grant))
    }

    /// 🎛️ Supplies exact config snapshot/mutation/store retirement authority, framework-owned by
    /// default beside [`Self::build_config_store_disposer`].
    fn config_store_owners_source_demands()->Result<RetirementDemand,ValueError>{store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<Self::Config,Self::ConfigMutation>()}
    fn build_config_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Config, Self::ConfigMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Config, Self::ConfigMutation>>> {
        Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<Self::Config, Self::ConfigMutation>(grant))
    }

    /// 📝️ Supplies exact draft snapshot/mutation/store retirement authority, framework-owned by
    /// default beside [`Self::build_draft_store_disposer`].
    fn draft_store_owners_source_demands()->Result<RetirementDemand,ValueError>{store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<Self::Draft,Self::DraftMutation>()}
    fn build_draft_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Draft, Self::DraftMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Draft, Self::DraftMutation>>> {
        Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<Self::Draft, Self::DraftMutation>(grant))
    }

    /// 📬️ Supplies the app-owned retained semantic preparation authority for one
    /// document mutation. `None` is an explicit fail-closed publication denial.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        None
    }

    /// 📬️ Supplies the app-owned retained semantic preparation authority for one
    /// config mutation. Generic config mutation replay is never substituted.
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        None
    }

    /// 📬️ Supplies the app-owned retained semantic preparation authority for one
    /// draft mutation. Generic draft mutation replay is never substituted.
    fn build_draft_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Draft, Self::DraftMutation>>> {
        None
    }

    /// 📬️ Supplies retained shared-live presence preparation. Absence denies the lane.
    fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        None
    }

    /// 📬️ Supplies retained local-only transient preparation. Absence denies the lane.
    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        None
    }

    /// 🧹️ Supplies exact bounded retirement for a displaced local presence root. The framework
    /// owns this lane by default (`bounded_presence_root_retirement_factory`) — an app that wants
    /// its own paging overrides it, but none has to declare one to be allowed to READ its own
    /// presence.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::bounded_presence_root_retirement_factory::<Self::Presence>())
    }

    /// 🧹️ Supplies exact bounded retirement for a displaced transient root.
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        None
    }

    /// 🏗️ Begins the app's retained semantic-validation/replay/store-construction job — framework-owned by
    /// default, paired with [`Self::build_document_store_owners`]' bounded default, so every whole-document load
    /// (Import Document, example switch, archive restore) can replace the store; an app with its own owner catalog
    /// supplies its own job. Rejection returns the exact completed envelope and grants no replacement authority.
    #[expect(clippy::result_large_err, reason = "Unavailable initialization returns the original completed envelope; boxing the refusal would change its retained ownership boundary.")]
    fn build_document_store_initialization_job(
        envelope: ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: ActorId,
    ) -> ArtifactInitializationAdmission<Self::Snapshot, Self::Mutation> {
        Ok(bounded_document_store_initialization_job(envelope, Self::DOCUMENT_SCHEMA, operation, generation, actor))
    }

    /// 🔐️ Revalidates domain-owned publication authority in the atomic replacement branch.
    fn validate_document_store_publication(_operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        if Self::REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY {
            Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.publication-authority-missing"), "registered domain did not supply atomic publication authority"))
        } else {
            Ok(())
        }
    }

    /// 🧹️ The five bounded close lanes `VcsArtifactApp`'s ladder drives
    /// (`drive_artifact_owned_disposer`, lanes 0–4) are FRAMEWORK-owned by default, the same way
    /// the presence retirement lane below is. Each default is built from the app's own
    /// associated types — there is nothing app-specific in any of them — and an app that wants
    /// its own paging still declares one, because these are defaults, not replacements.
    ///
    /// 🪦️ They were `None`, which `drive_artifact_owned_disposer` turns into
    /// `interactive-job.close-owned-disposer-missing`: an app that declared no disposer could
    /// not be CLOSED at all. Measured on 2026-09-22 (ticket 26/09/18 slices TC3e §7a, TC4 §3):
    /// 229 of the 298 editor/viewer impls under `✏️s/🔌️plugins` sat on that fail-closed default,
    /// and it is what killed the first three-package trusted-catalog bootstrap — the `codec`
    /// resolver constructs EVERY app of a bundle to read its schema and closes each one it
    /// rejects, so one undeclared disposer anywhere in a package refuses the whole publication.
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

    /// 🧹️ The peer half of the same lane, framework-owned by the same default.
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::bounded_presence_root_retirement_factory::<Self::Presence>())
    }

    fn build_transient_store_disposer() -> ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        Some(bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }
    /// 📜️ Stable document schema id — prefer this over `artifact_schema(&self)`.
    const DOCUMENT_SCHEMA: &'static str;
    /// 📚️ Document body for `setActiveExample` when this app did not declare that action.
    /// `None` means there is no catalogue. `Some(Ok(body))` is DSL or JSON. An empty id is the initial snapshot.
    fn catalogue_example_document(_example_id: &str) -> Option<Result<String, Fault>> {
        None
    }
    /// 👁️✏️ Contract §2.3: `VcsArtifactApp` reads this to decide whether to reject the
    /// eight mutating verbs (see `VIEWER_REJECTED_ACTION_IDS`) with `viewer.read-only`. Defaults
    /// to `Editor` so every pre-surfaces direct `ArtifactApp` impl (hand-written, `document_app`)
    /// keeps its existing full-mutation behavior untouched; `EditorApp<E>`/`ViewerApp<V>` override
    /// it from `E::ROLE`/`V::ROLE` (contract §2.1).
    const ROLE: AppRole = AppRole::Editor;
    type Snapshot: Clone + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + 'static;
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): `+ 'static` added to the
    /// three mutation lanes below — every real impl was ALREADY effectively 'static (a plain
    /// owned enum/struct crossing the wasm component boundary via `OpBinary`; none of the ~30
    /// plugins declares a borrowed lifetime on any of these), but nothing forced the compiler to
    /// KNOW that until now: `⚛️reactor::spawn_task`'s `AsyncTask<M, C, D>` future must outlive
    /// the synchronous `dispatch_emit` call that spawns it, which needs `M/C/D: 'static` provable
    /// from `A: ArtifactApp` alone — the associated type itself, not merely `A`.
    ///
    /// 🏷️ `SemanticMutation` (design §16.2): every document operation labels itself in every shell locale through its
    /// leaf (`MutationKind::label`), so a history row, the history-edit editor and a tool-transaction row are never the
    /// operation's text line — `#[derive(Mutations)]` is the one way an aggregate answers it.
    type Mutation: protocol::SemanticMutation<Self::Snapshot> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Config: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + store::ConfigRecord + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    type ConfigMutation: ::protocol::Mutation<Self::Config> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + ::replication::OpBinaryReceiving + 'static;
    /// 📝️ Volatile draft snapshot — use {@link NoDraft} when the app has no draft lane.
    type Draft: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    /// 📝️ Draft-lane operations applied to {@link store::DraftStore}.
    type DraftMutation: ::protocol::Mutation<Self::Draft> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    /// 👥️ Shared live presence — use {@link NoPresence} when the app has no shareable live state.
    type Presence: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + store::ArtifactPresenceSnapshot + 'static;
    /// 👥️ Presence-lane operations applied to the app's typed presence snapshot.
    type PresenceMutation: ::protocol::Mutation<Self::Presence> + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    /// 🫧️ Ephemeral LOCAL-ONLY UI state — use {@link NoTransient} when the app has none.
    /// The fourth and last state mechanism; see `NoTransient`'s doc for how it differs from a
    /// draft (which is ephemeral *artifact* content, not UI state).
    type Transient: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    /// 🫧️ Transient-lane operations applied to {@link store::TransientStore}.
    type TransientMutation: ::protocol::Mutation<Self::Transient> + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;

    /// 👥️🫧️ The two EPHEMERAL lanes this command touches — presence (shared) and
    /// transient (local-only UI). Separate from `handle` because neither lane has an op log, an
    /// undo group, or a failure mode: they are applied unconditionally and cannot fail, so
    /// folding them into `handle`'s `Result<Emit, Fault>` would misrepresent them.
    ///
    /// Defaults to emitting nothing. An app-owned job invokes this from its measured, cancellable
    /// worker stage and transfers the result to its completion; dispatch never invokes it.
    async fn ephemeral(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _presence: &PresenceView<'_, Self::Presence>, _transient: &TransientView<'_, Self::Transient>) -> EphemeralEmit<Self> {
        EphemeralEmit::default()
    }
    /// 🎯️ B1: this app's closed, typed command enum — the SOLE dispatch surface for
    /// `handle` below, replacing the deleted stringly-typed `handle_action`/`handle_command`/
    /// `handle_typed_command` trio. Decoded off the wire once, by `VcsArtifactApp::dispatch_typed_command`,
    /// via `OpBinary::decode_op`; framework-reserved verbs (undo/redo/checkpoint/alternative/clipboard/
    /// revert/history-filter/noteShellCommand) never reach here — the wrapper intercepts those itself
    /// (see `VcsArtifactApp::dispatch_framework_action`) since they are host mechanics, not app behavior.
    type Command: ::protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static;

    async fn config_schema() -> &'static str {
        "config.empty"
    }
    async fn initial_snapshot() -> Self::Snapshot;
    async fn initial_config() -> Self::Config {
        Self::Config::default()
    }
    /// 🧭️ Resolves a host-owned control-plane action directly to one event-sourced
    /// configuration mutation. App interaction commands remain on the retained tool-operation
    /// path; hidden host catalogue publication does not impersonate an interactive reducer.
    fn host_configuration_mutation(_action: &str, _args: Option<&DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok(None)
    }
    /// ⌨️ This app's typing algebra: how the leaves `next` of one live typing delivery join the open run's net leaves
    /// `net` (design §13.2). The default is a single buffer — every delivery's leaves are the whole buffer, so the latest
    /// replaces the net; an app whose typing verb yields relative leaves (a text splice) composes them, and answers
    /// `Split` where the edit does not continue the run (a caret that jumped away).
    fn typing_fold(_net: &[Self::Mutation], next: &[Self::Mutation]) -> semio_framework_tool_machine::TypingFold<Self::Mutation> {
        semio_framework_tool_machine::TypingFold::Net(next.to_vec())
    }
    /// 🪧️ The label, in every shell locale, of the entity `id` (one of `kinds`, the reference's declared entity kinds)
    /// in `snapshot` — what a history-edit reference chip reads instead of the raw id. `None` shows the id as data.
    fn entity_label(_snapshot: &Self::Snapshot, _kinds: &[String], _id: &str) -> Option<LocalizedLabel> {
        None
    }
    /// 🖱️ The entity id one selected row `row` of a reference input's interaction domain names for an input of entity
    /// `kinds` — what a history edit's "Use selection" drafts (design §16.4). `None` drops a row the input cannot name.
    /// The default is the row id itself (the domain's rows ARE the entity ids); an app whose rows are composite (an
    /// owner prefix, a component path) answers the bare entity id.
    fn selection_reference_id(_kinds: &[String], row: &str) -> Option<String> {
        Some(row.to_string())
    }
    /// 🎯️ The leaf kinds (`SemanticDescriptor::kind`, e.g. `"drag-selection"`) that state the intent of a transaction the
    /// tool `tool` (the stamped `TransactionRef.tool`, `<appId>#<toolId>`) commits (design §19.1): its history row is
    /// labelled by the first of its operations of one of these kinds instead of its first operation, so a gesture whose
    /// support leaves come first still reads as what the user did. Empty (the default): the first operation labels it.
    fn tool_intent_kinds(_tool: &str) -> &'static [&'static str] {
        &[]
    }
    /// 📢️ The localized notice of every fault code this app's guest refuses with (design §20.12): `(code, label)`, the code
    /// `<app>.<area>.<name>`, the label complete in every locale × terminology with `{name}` placeholders filled from
    /// `Fault.params`. Stamped into `AppDefinition::fault_notices` at registration (validated there) and resolved by both
    /// shells after the framework's own table. Empty (the default): the app declares no notices.
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        &[]
    }
    async fn initial_draft() -> Self::Draft {
        Self::Draft::default()
    }
    /// 🧩️ B1: the pure heart of the app — a total, side-effect-free function from
    /// `(command, document, config, draft, engines)` to an {@link Emit}. No `&mut self`.
    /// `engines` is the host-owned {@link EngineHandles} bag (empty until WIT engine-derive/read
    /// is threaded through exchange).
    async fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        interaction: &InteractionView<'_>,
        view_state: Option<&ViewModel>,
        draft: &DraftView<'_, Self::Draft>,
        engines: &EngineHandles,
    ) -> ArtifactMutationOutcome<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>;
    /// 🏷️ `command`'s action/command id string — used for command-log labeling and
    /// `AppActionRegistry` kind-discipline lookup (`View`/`🐚️Shell`-kind must not emit
    /// `artifact_mutations`; `VcsArtifactApp::dispatch_typed_command_inner` enforces this when the
    /// registry has a matching declaration). Default: `"typed-command"` for every command (correct but
    /// generic — an app that wants per-command labels/kind-discipline overrides this to match the id
    /// the command was declared under in its `AppDefinition`, e.g. via `.operation`/`.view_action`).
    async fn command_id(_command: &Self::Command) -> &'static str {
        "typed-command"
    }
    /// 🎯️ Builds this app's typed `Command` from a host action id + JSON args — the bridge
    /// the React/wgpu shells still speak (`{action,args}`) until every call site sends `OpBinary`
    /// bytes directly. Default rejects (same error as the pre-bridge `dispatch_action` arm); apps
    /// that the shells drive must override this so chrome actions reach `handle`.
    async fn command_from_action(action: &str, _args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        Err(Fault::new(
            FaultOrigin::App,
            FaultCode::new("app.command.unsupported"),
            format!("action '{action}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"),
        ))
    }
    /// 🔐️ The agent lane's fill for an omitted [`ActionArgDef::target_revision`] argument: the token the rendered
    /// binding of the target `args` address carries in `doc` right now, or `None` when this app resolves no such target —
    /// the agent lane then refuses the call by name. The shell lanes never call it (ticket 26/09/23, G12 session 14c).
    async fn agent_target_revision(_action: &str, _args: &DslValue, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
        Ok(None)
    }

    /// 🧮️ This app's typed configuration spec — empty (the default) means "no configuration options."
    async fn config_spec() -> ConfigSpec {
        ConfigSpec::empty().await
    }
    /// 📋️ The `MediaType` this app copies fragments as and accepts pastes of by default — `None` (the
    /// default) means this app doesn't participate in the clipboard mechanism, and `VcsArtifactApp`'s
    /// injected `copy`/`cut`/`paste` actions silently no-op. Override alongside `copy_fragment`/
    /// `cut_operations`/`paste_operations`.
    async fn clipboard_media_type() -> Option<MediaType> {
        None
    }
    /// 📋️ Every `MediaType` this app accepts a paste of — defaults to just `clipboard_media_type()`.
    /// Override to additionally accept fragments copied from a compatible sibling app (e.g. puzzle
    /// accepting a block kind-definition fragment as a catalog merge).
    async fn clipboard_accepts() -> Vec<MediaType> {
        Self::clipboard_media_type().await.into_iter().collect()
    }
    /// 📋️ Builds a `ClipboardFragment` from the current selection. Called by `VcsArtifactApp`'s
    /// injected `copy` and `cut` actions; `cut` additionally calls `cut_operations`. Default: always
    /// empty (apps that don't override `clipboard_media_type` never reach here in practice, since the
    /// interception only calls this when a media type is declared).
    async fn copy_fragment(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        Err(ClipboardError::EmptySelection)
    }
    async fn cut_operations(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>) -> Vec<Self::Mutation> {
        Vec::new()
    }
    /// 🕹️ This app's `InteractionTopology` for the CURRENT document/config — one `DomainTopology`
    /// per domain whose `HierarchyProvider::Topology` the app itself supplies (a DAG, scene graph,
    /// AST parent chain, …). Default: empty (correct for `Flat`/`UiTree`/`PathDelimited` domains,
    /// which `VcsArtifactApp` derives itself — see `resolve_domain_topology`); an app that declares
    /// any `HierarchyProvider::Topology` domain MUST override this or that domain's transitive
    /// hover/select and range-selection degrade to single-target (an empty topology contains no
    /// node, so `DomainTopology::descendant_closure`/`index_of` find nothing).
    async fn interaction_topology(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, ValueError> {
        Ok((|| protocol::InteractionTopology::default())())
    }
    async fn paste_operations(_doc: &ArtifactView<'_, Self::Snapshot>, _fragment: &ClipboardFragment, _placement: &PastePlacement) -> Result<Vec<Self::Mutation>, ClipboardError> {
        Ok(Vec::new())
    }
    /// 🛍️ This app's STATIC catalogue payload as canonical JSON — the whole registered operator /
    /// palette catalogue a node-graph-backed app offers. It is published ONCE per app instance on the
    /// reserved `framework.section.catalogue` retained surface (`semio_framework::UiRefreshSection::Catalogue`),
    /// hash-conditional like every other reserved section, and referenced from a scene by operator kind
    /// id only. It deliberately does NOT ride on `NodeGraphScene`: the payload is ~100 KB with the real
    /// `brep`/`math` operator sets installed, three times `ui_contract::UI_FIXED_BYTES`, so embedding it
    /// per surface made every node-graph window unrenderable (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
    /// Default: `"{}"` — an app with no catalogue.
    async fn app_catalogue_json() -> String {
        String::from("{}")
    }
    /// ⏱️ Background work the host should dispatch after this pass. `view` is the host's own
    /// attached-window roster — the ONLY place an app can learn which concrete window instance owns
    /// a window-scoped self-dispatch chain, so an armed `Effect::DispatchAction` can carry that
    /// window's id in its `args` instead of landing on whichever window happens to be current
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). `None` means no surface is mounted yet: an app
    /// with window-scoped work must arm NOTHING and wait for the first `Event::SurfaceVisible`.
    async fn pending_effects(_owner: &ArtifactInstanceOperationOwnerHandle, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view: Option<&ViewModel>) -> Vec<Effect> {
        Vec::new()
    }
    /// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): return type flipped
    /// from `ui_wgpu::wgpu::UiNode` to `ui_runtime::ComponentTree` — the choke-point change every
    /// implementer of this trait must follow; see `📓️recipe-plugin.md` in this ticket's folder.
    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> UiAssemblyResult<ComponentTree>;
    /// 🎭️ Renders against the same instance-retained operation owner used by typed jobs.
    async fn render_with_instance_operation_owner(
        owner: &ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
    ) -> UiAssemblyResult<ComponentTree> {
        let _ = owner;
        Self::render(body_key, doc, cfg, view_state).await
    }
    /// 🧮️ Supplies the immutable local request snapshot used by retained reducers to render.
    ///
    /// 🕹️ `interaction` reads the framework-owned interaction mechanism (hover + selection) so a
    /// window can paint its own hovered/selected state instead of ever storing it again itself —
    /// see `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`.
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
        let rendered:crate::UiAssemblyResult<crate::ComponentTree>=async{
        let _ = (transient, interaction);
        Self::render_with_instance_operation_owner(owner, body_key, doc, cfg, view_state).await

        }.await;
        rendered.map(Some).map_err(Into::into)
        }
    /// 🪟️ Keyed by window INSTANCE id — an app with two open instances of the same kind (e.g. split
    /// panes) returns one entry per instance so their chrome/options never collapse together. Apps with
    /// a single window kind and no splitting return `vec![kind_id]`-worth of entries either way.
    async fn window_engagements(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        HashMap::new()
    }
    /// 🫧️ Projects window chrome from the exact transient root captured for the addressed window.
    ///
    /// 🕹️ `interaction` is the SAME framework-owned hover/selection `render_with_request_context`,
    /// `window_measures_with_request_context` and `context_menu_with_request_context` already carry —
    /// window chrome is the fourth and last per-request projection, so an engagement HUD reports what
    /// the user actually picked (cad's "N selected") instead of a permanently empty local snapshot.
    /// Default discards it and falls through to `window_engagements`, exactly like every other twin.
    async fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let _ = (transient, interaction);
        Self::window_engagements(doc, cfg, view_state).await
    }
    /// 🪟️ See `window_engagements` — same per-window-instance keying.
    async fn window_measures(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::new()
    }
    /// 🕹️ `window_measures`' interaction-aware twin — the same per-window-instance chrome, plus the
    /// framework-owned hover/selection an option group needs to gate itself on what the user picked
    /// (e.g. puzzle3d's Brush "Placement" picker, which only exists for a hovered/selected vortex).
    /// Default discards `interaction` and falls through to `window_measures`, exactly like
    /// `render_with_request_context` does — see `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`.
    async fn window_measures_with_request_context(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel, interaction: &InteractionView<'_>) -> HashMap<String, Vec<WindowMeasure>> {
        let _ = interaction;
        Self::window_measures(doc, cfg, view_state).await
    }
    /// 🛠️ Keyed by TOOL id (`AppDefinition.tools[].id`), not window instance — a tool's live options
    /// (e.g. puzzle3d fill's count slider) rendered in the mode-level tool panel rather than a
    /// window's utility-options rail. Reuses `WindowMeasure` as the shared control vocabulary; the
    /// `Group.active_utility_id` tag is simply unused for tool measures.
    async fn tool_measures(_doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::new()
    }
    /// 🖱️ Answers an on-demand right-click menu request — the WIT `context-menu` export's SDK
    /// counterpart. Called fresh at right-click time (never cached, never part of `refreshUi`);
    /// `request.menu.id` names the target the host resolved (a `UiMenuRef` a plugin attached to a
    /// `UiNode`, or a scene-surface convention id like `"world3d"`/`"nodeGraph"`/`"window"`).
    /// Default: empty — the host falls through to the next outer menu layer (window/OS chrome),
    /// so apps that never attach a `menu` never need to override this. `registry` is this app's
    /// `AppActionRegistry` (the same one `VcsArtifactApp` enforces the actions contract with) —
    /// pass it to `Menu::of(registry, view_state)` to resolve labels/icons from declared `ActionDefinition`s in the call's axes
    /// instead of hand-building rows.
    async fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel, _registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        Vec::new()
    }
    /// 🕹️ `context_menu`'s interaction-aware twin. `ContextMenuRequest.surface.selection` is
    /// CLIENT-supplied and only ever carries what that surface itself painted, so a granularity the
    /// surface does not model (a vortex marker, a target volume) never reaches a menu through it;
    /// `interaction` is the authoritative framework-owned selection for every declared domain.
    /// Default discards it and falls through to `context_menu` — see
    /// `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`.
    async fn context_menu_with_request_context(
        request: &ContextMenuRequest,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        interaction: &InteractionView<'_>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        let _ = interaction;
        Self::context_menu(request, doc, cfg, view_state, registry).await
    }
    /// 🌱️ Initial mutations applied through normal dispatch right after the store is constructed —
    /// replaces the old `seed(&mut ArtifactStore)` direct-store-touch hook (ticket
    /// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M4: this was the only place an app touched a
    /// store directly; routing genesis mutations through the same `ArtifactCommand::Apply` path
    /// every user edit takes removes that exception). Default: no mutations — only apps whose
    /// fixture is itself a rich history (e.g. a history-UI demo/exerciser) need this — every
    /// program driven purely by user actions leaves it untouched.
    async fn genesis() -> Vec<Self::Mutation> {
        Vec::new()
    }

    /// 🪪️ This app's own config+presence schema descriptor, declared by
    /// `PluginBuilder::document_app` the moment this type is bound to a plugin (ticket
    /// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c: app-scope schema is keyed off `Self`, so the
    /// builder call that already names `Self` is the correct place to register it, not a plugin-root
    /// side callback). `None` (the default) means this app has no schema of its own — a plugin
    /// library with zero document apps, or a document app whose config truly has no dedicated facet.
    async fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        None
    }

    /// 🔌️ This app's typed media I/O surface — `None` (the default) means "declares no ports beyond
    /// the implicit document ports" (`media_ports` below still returns those two). Override to return
    /// a real `AppIo` (e.g. `shooting_engine::shooting_io()`) to declare extra workflow ports.
    async fn io() -> Option<AppIo> {
        None
    }
    /// 📄️ Declares the exact natural representation this app can both decode and encode.
    async fn natural_file_codec() -> Option<NaturalFileCodec> {
        None
    }
    /// 💾️ Encodes genuine natural-format bytes; unsupported apps must keep the default refusal.
    async fn encode_natural_file(_snapshot: &Self::Snapshot) -> Result<Vec<u8>, MediaError> {
        Err(MediaError::NotImplemented)
    }
    /// 📂️ Decodes genuine natural-format bytes before the result enters the ordinary mutation lane.
    async fn decode_natural_file(_bytes: &[u8]) -> Result<Self::Snapshot, MediaError> {
        Err(MediaError::NotImplemented)
    }
    /// 🎞️ Declares this app's workflow ports — delegates to `Self::io().all_ports()` (implicit
    /// `document:in`/`document:out` plus any app-specific ports) when `io()` is overridden; an app that
    /// never overrides `io()` still gets no ports (matches the old `Vec::new()` default) since there is
    /// no document media type to synthesize the implicit pair from.
    async fn media_ports() -> Vec<MediaPortSpec> {
        match Self::io().await {
            Some(io) => io.all_ports().await,
            None => Vec::new(),
        }
    }
    /// 🎞️ Pure export of the current document onto one declared output port — must not mutate
    /// anything. Called by both the UI (preview/export) and a headless runner (moving media along a
    /// workflow edge). Default: the whole document pack, base64-wrapped, for `"artifact:out"`;
    /// `MediaError::NotImplemented` for any other port (apps declaring extra output ports override
    /// this to handle them, falling through to `ArtifactApp::export_media`'s default via `_ =>` for
    /// `"artifact:out"` if desired).
    /// 🌀️ `io-async-signatures`: async signature, behaviourally unchanged body — see
    /// `ArtifactSerializer::serialize`'s doc comment for the ready-future guarantee.
    async fn export_media(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Media, MediaError> {
        if port != "artifact:out" {
            return Err(MediaError::NotImplemented);
        }
        let media_type = Self::io().await.map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
        let bytes = doc.snapshot.encode_pack();
        Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
    }
    /// 🧮️ Exports with the exact immutable transient snapshot captured for this request.
    async fn export_media_with_request_context(owner: &ArtifactInstanceOperationOwnerHandle, port: &str, doc: &ArtifactView<'_, Self::Snapshot>, transient: &TransientView<'_, Self::Transient>) -> Result<Media, MediaError> {
        let _ = (owner, transient);
        Self::export_media(port, doc).await
    }
    /// 🎞️ Translates an incoming media value on one declared input port into a document load — never a mutation, so an
    /// import writes no history row: it is the load/genesis path (`Effect::LoadDocument` over a fresh history), the same path a
    /// catalogue example takes. Default: decodes a `"artifact:in"` structured (base64 pack) payload, or a natural file through the
    /// app's codec; `MediaError::NotImplemented` for any other port.
    async fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> ArtifactMutationOutcome<Self::Mutation, Self::ConfigMutation, Self::DraftMutation, MediaError> {
        if port == NATURAL_FILE_PORT {
            let codec = Self::natural_file_codec().await.ok_or(MediaError::NotImplemented)?;
            let bytes = natural_file_input_bytes(port, codec, media)?;
            let snapshot = Self::decode_natural_file(bytes).await?;
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
    /// 🎞️ Cheap identity for one output port's current value, without serializing the payload.
    /// Default re-derives it from `export_media`; override when a fingerprint is derivable without
    /// materializing the full export (e.g. from a cached head edit id).
    /// 🌀️ `io-async-signatures`: async signature (delegates to the now-async `export_media`
    /// above, behaviourally unchanged otherwise).
    async fn media_fingerprint(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<MediaFingerprint, MediaError> {
        match Self::export_media(port, doc).await {
            Ok(media) => Ok(MediaFingerprint::of(&media)),
            Err(error) => Err(error),
        }
    }
}

/// 🎞️ Rust mirror of WIT's `media-artifact` record (`framework/wit/📜️world.wit`): `descriptor` is the
/// parsed `descriptor-json`, `data` the sibling raw-bytes field. Deliberately separate from
/// `mesh::Media` (which pairs a `MediaType` with a `MediaPayload` for the declared-port workflow
/// via `export_media`/`import_media`) — `consume-media`/`produce-media` operate at the whole-document
/// level, not a specific `MediaPortSpec`, so `PluginApp::{consume_media, produce_media}` below default
/// to a document passthrough rather than requiring any `media_ports()` declaration at all.
#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
pub struct MediaArtifact {
    pub descriptor: MediaArtifactDescriptor,
    pub data: Vec<u8>,
}

/// 📄️ One editor-owned natural-file codec, matched exactly to its declared representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NaturalFileCodec {
    pub format_kind: &'static str,
    pub extension: &'static str,
    pub media_type: &'static str,
    pub binary: bool,
}

/// 🧭️ Monotonic counters reported by one format-owned natural-file decoder.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NaturalFileDecodeProgress {
    pub consumed_input_bytes: usize,
    pub decoded_items: usize,
    pub owned_output_bytes: usize,
    pub completed_work_units: u64,
}

/// 🧩️ One bounded turn of a format-owned natural-file decoder.
pub enum NaturalFileDecodeStep<T> {
    Progress(NaturalFileDecodeProgress),
    Complete { snapshot: T, progress: NaturalFileDecodeProgress },
}

/// 📂️ Format-owned incremental decode authority used by the reserved Open route.
pub trait NaturalFileDecodeCursor<T>: Send {
    fn advance(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<NaturalFileDecodeStep<T>, MediaError>;
    fn begin_close(&mut self);
    fn close_step(&mut self, grant:RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault>;
    fn retirement_demands(&self, body:usize) -> Result<RetirementDemand,ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

/// 📦️ Exact owner chosen by an editor for natural-file decoding.
pub enum NaturalFileDecodeOwner<T> {
    Controlled(Box<dyn NaturalFileDecodeCursor<T>>),
    Synchronous(Vec<u8>),
}

/// 📄️ The ABI port reserved for an editor's mounted natural-file codec.
pub const NATURAL_FILE_PORT: &str = semio_framework::NATURAL_FILE_PORT_ID;

pub(crate) fn natural_file_input_media(codec: NaturalFileCodec, media_type: MediaType, data: Vec<u8>) -> Media {
    Media { media_type, payload: MediaPayload::Intrinsic { schema: codec.format_kind.to_string(), value: DslValue::Bytes(data) } }
}

/// 🌱️ The load/genesis emission of one decoded whole document — what a natural-file import and the default `document:in`
/// importer publish: `Effect::LoadDocument` carrying the document's pack over a fresh empty history, exactly like a catalogue
/// example load. It is not a mutation and writes no history row; the runtime stamps the live store's identity on it.
pub fn document_load_emit<S: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot, Mutation, ConfigMutation, DraftMutation>(snapshot: &S, schema: &str) -> Emit<Mutation, ConfigMutation, DraftMutation> {
    let pack = snapshot.encode_pack();
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("", schema));
    Emit { effects: vec![Effect::LoadDocument { pack, spr }], ui_scope: UiDirtyScope::Full, ..Emit::default() }
}

pub(crate) fn natural_file_input_bytes<'a>(port: &str, codec: NaturalFileCodec, media: &'a Media) -> Result<&'a [u8], MediaError> {
    let MediaPayload::Intrinsic { schema, value: DslValue::Bytes(bytes) } = &media.payload else {
        return Err(MediaError::Payload(port.to_string(), "natural file importer requires intrinsic owned bytes".into()));
    };
    if schema != codec.format_kind {
        return Err(MediaError::Payload(port.to_string(), format!("natural file schema mismatch: expected {}, found {schema}", codec.format_kind)));
    }
    Ok(bytes)
}

fn natural_file_action_arg(id: &str, value: &impl semio_framework_value::ToValue) -> ActionArgDef {
    let mut argument = ActionArgDef::text(id, LocalizedLabel::native(id, id)).default_value(value);
    argument.presentation = Some(ArgPresentation::Hidden);
    argument
}

pub(crate) fn natural_file_action_definitions(codec: NaturalFileCodec) -> [ActionDefinition; 2] {
    let args = || {
        vec![
            natural_file_action_arg("formatKind", &codec.format_kind),
            natural_file_action_arg("extension", &codec.extension),
            natural_file_action_arg("mediaType", &codec.media_type),
            natural_file_action_arg("binary", &codec.binary),
        ]
    };
    [
        ActionDefinition::resumable_framework(SAVE_ARTIFACT_FILE_ACTION_ID, LocalizedLabel::native("Save File", "Datei speichern"), ActionKind::Shell, "save").with_category("transfer").with_args(args()).audience(CapabilityAudience::Chrome),
        ActionDefinition::resumable_framework(OPEN_ARTIFACT_FILE_ACTION_ID, LocalizedLabel::native("Open File…", "Datei öffnen…"), ActionKind::Shell, "folder-open")
            .with_category("transfer")
            .with_args(args())
            .audience(CapabilityAudience::Chrome),
    ]
}

pub(crate) fn mount_natural_file_actions(definition: &mut AppDefinition, codec: NaturalFileCodec) {
    if !definition.io.export_formats.iter().any(|format| format == codec.format_kind) {
        definition.io.export_formats.push(codec.format_kind.to_string());
    }
    if !definition.io.import_formats.iter().any(|format| format == codec.format_kind) {
        definition.io.import_formats.push(codec.format_kind.to_string());
    }
    for action in natural_file_action_definitions(codec) {
        for window in definition.window_kinds.iter_mut() {
            if !window.actions.iter().any(|entry| entry.id == action.id) {
                window.actions.push(action.clone());
            }
        }
    }
}

/// 🎞️ Rust mirror of WIT's `media-artifact.descriptor-json` JSON shape. `edge_id`/`port_id`/`kind_id`
/// are dataflow-driver bookkeeping the SDK default below leaves untouched (opaque pass-through for the
/// caller); `media_type` is the declared `MediaType` the wire claims to satisfy; `wire` is the actual
/// encoding; `blob_hash` is set instead of inline `data` when the payload already lives in the host's
/// blob store.
#[derive(Clone, Debug, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct MediaArtifactDescriptor {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edge_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub port_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    pub wire: MediaWireFormat,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blob_hash: Option<String>,
}

/// 🚧️ Failure producing/consuming a `MediaArtifact` through `PluginApp::{produce_media, consume_media}`.
#[derive(Debug, semio_framework_value::RetireOwned)]
pub enum MediaArtifactError {
    Payload(String),
    Value(ValueError),
    Export(MediaError),
    Import(Fault),
    SchemaMismatch { expected: String, found: String },
    NoImporter(String),
}

impl semio_framework_job::OriginalDomainFailure for MediaArtifactError{
 fn domain_code(&self)->&str{match self{Self::Payload(_)=>"media.artifact.payload",Self::Value(_)=>"media.artifact.value",Self::Export(_)=>"media.artifact.export",Self::Import(_)=>"media.artifact.import",Self::SchemaMismatch{..}=>"media.artifact.schema-mismatch",Self::NoImporter(_)=>"media.artifact.no-importer"}}
 fn domain_message(&self)->&str{match self{Self::Payload(message)|Self::NoImporter(message)=>message,Self::Value(error)=>&error.message,Self::Import(error)=>&error.message,Self::SchemaMismatch{expected,..}=>expected,Self::Export(error)=>match error{MediaError::UnknownPort(port)|MediaError::Incompatible{port,..}=>port,MediaError::Payload(_,message)=>message,MediaError::NotImplemented=>"media ports are not implemented for this app"}}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{match self{Self::Value(error)=>error.retained_progress(),Self::Import(error)=>error.retained_progress(),_=>Default::default()}}
}
impl semio_framework_job::OriginalDomainProjection for MediaArtifactError{
 fn projection_identity(&self)->usize{self as*const Self as usize}
 fn projection_view(&self,path:&[usize])->Result<semio_framework_job::OriginalDomainView<'_>,ValueError>{use semio_framework_job::OriginalDomainView as V;match path{
  []=>Ok(V::IntrinsicObject(2)),[0]=>Ok(V::IntrinsicText(match self{Self::Payload(_)=>"payload",Self::Value(_)=>"value",Self::Export(_)=>"export",Self::Import(_)=>"import",Self::SchemaMismatch{..}=>"schemaMismatch",Self::NoImporter(_)=>"noImporter"})),
  [1,rest @ ..]=>match self{Self::Payload(message)|Self::NoImporter(message)if rest.is_empty()=>Ok(V::IntrinsicText(message)),Self::Value(error)=>semio_framework_job::project_original_value_error(error,rest),Self::Import(error)=>semio_framework_job::project_original_fault(error,rest),Self::Export(error)=>crate::media_operation_refusal::project_media_error(error,rest),Self::SchemaMismatch{expected,found}=>match rest{[]=>Ok(V::IntrinsicObject(2)),[0]=>Ok(V::IntrinsicText(expected)),[1]=>Ok(V::IntrinsicText(found)),_=>Err(media_projection_error())},_=>Err(media_projection_error())},_=>Err(media_projection_error())
 }}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match path{[]=>["kind","original"].get(index).copied().ok_or_else(media_projection_error),[1,rest @ ..]=>match self{Self::Value(error)=>semio_framework_job::key_original_value_error(error,rest,index),Self::Import(error)=>semio_framework_job::key_original_fault(error,rest,index),Self::Export(error)=>crate::media_operation_refusal::key_media_error(error,rest,index),Self::SchemaMismatch{..}if rest.is_empty()=>["expected","found"].get(index).copied().ok_or_else(media_projection_error),_=>Err(media_projection_error())},_=>Err(media_projection_error())}}
}

/// 🧳️ Preserves media artifact domain refusals beside complete initiating turn failures.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum MediaArtifactOperationFailure{Domain(MediaArtifactError),Original(semio_framework_job::OriginalTurnFailure)}
impl From<MediaArtifactError> for MediaArtifactOperationFailure{fn from(source:MediaArtifactError)->Self{Self::Domain(source)}}
impl From<semio_framework_job::OriginalTurnFailure> for MediaArtifactOperationFailure{fn from(source:semio_framework_job::OriginalTurnFailure)->Self{Self::Original(source)}}
impl From<ValueError> for MediaArtifactOperationFailure{fn from(source:ValueError)->Self{Self::Original(source.into())}}
impl From<Fault> for MediaArtifactOperationFailure{fn from(source:Fault)->Self{Self::Original(source.into())}}

impl std::fmt::Display for MediaArtifactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Payload(message) => formatter.write_str(message),
            Self::Value(error) => std::fmt::Display::fmt(error,formatter),
            Self::Export(error) => std::fmt::Display::fmt(error,formatter),
            Self::Import(error) => formatter.write_str(&error.message),
            Self::SchemaMismatch { expected, found } => write!(formatter, "document schema mismatch: expected {expected}, found {found}"),
            Self::NoImporter(format) => write!(formatter, "no binary importer registered for format {format:?}"),
        }
    }
}

impl std::error::Error for MediaArtifactError {
    fn source(&self)->Option<&(dyn std::error::Error+'static)>{match self{Self::Value(error)=>Some(error),Self::Export(error)=>Some(error),_=>None}}
}

impl From<ValueError> for MediaArtifactError {
    fn from(error:ValueError)->Self{Self::Value(error)}
}

/// 🎞️ What consuming one `MediaArtifact` asks of the runtime: the import was applied (an ordinary, undoable edit or the app's
/// own typed operation), or the artifact is a whole document of the app's own schema, whose stamped archive the runtime admits
/// as a stepped document archive load (`📓️api-stepped-document-load.md` §4) — a whole document never folds inside the call.
#[derive(Debug)]
pub enum MediaConsumption {
    Applied,
    DocumentLoad(protocol::DocumentArchivePack),
}

/// 🚫️ The framework code a whole document of a foreign schema is refused with (`{found}`, `{expected}`; notice in the framework
/// table `FRAMEWORK_FAULT_NOTICE_LABELS`).
pub const MEDIA_SCHEMA_MISMATCH_CODE: &str = "plugin.media.schema-mismatch";

/// 🪧️ The codes of the whole-document load and save surface; each has its framework notice in every locale (kernel table
/// `FRAMEWORK_FAULT_NOTICE_LABELS`).
pub const DOCUMENT_LOAD_UNAVAILABLE_CODE: &str = "plugin.document-load.unavailable";

pub const DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE: &str = "plugin.document-load.too-many-members";

pub const DOCUMENT_LOAD_TOO_LARGE_CODE: &str = "plugin.document-load.too-large";

pub const DOCUMENT_LOAD_BUSY_CODE: &str = "plugin.document-load.busy";

pub const DOCUMENT_LOAD_INCOMPLETE_CODE: &str = "plugin.document-load.incomplete";

pub const DOCUMENT_LOAD_MEMBER_INVALID_CODE: &str = "plugin.document-load.member-invalid";

pub const DOCUMENT_LOAD_HISTORY_INVALID_CODE: &str = "plugin.document-load.history-invalid";

pub const DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE: &str = "plugin.document-load.operation-unknown";

pub const DOCUMENT_LOAD_CHANGED_CODE: &str = "plugin.document-load.changed";

pub const DOCUMENT_LOAD_FAILED_CODE: &str = "plugin.document-load.failed";

pub const DOCUMENT_LOAD_OTHER_DOCUMENT_CODE: &str = "plugin.document-load.other-document";

pub const DOCUMENT_LOAD_MEMBERS_DIFFER_CODE: &str = "plugin.document-load.members-differ";

/// 🚫️ A named refusal of the whole-document load and save surface: `code` is one of the `DOCUMENT_LOAD_*_CODE`s, the message
/// keeps the engineering detail a log reads.
pub fn document_load_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new(code), message)
}

/// 🎞️ A whole document on a media edge (`artifact:out`): the recursive document archive (parent pack + `.spr` + owned members) as
/// `pk:` base64 text on the `Document{schema}` wire — the `Structured{schema, json}` convention of `export_media`, which every host
/// carries losslessly as text.
pub async fn whole_document_media(port: &str, schema: String, archive: &protocol::DocumentArchivePack) -> Result<MediaArtifact, MediaArtifactError> {
    let bytes = protocol::encode_document_archive_bytes(archive).map_err(|error| MediaArtifactError::Payload(error.to_string()))?;
    Ok(MediaArtifact {
        descriptor: MediaArtifactDescriptor { edge_id: None, port_id: Some(port.to_string()), kind_id: None, media_type: None, wire: MediaWireFormat::Document { schema }, blob_hash: None },
        data: store::pack_rt::pack_value_to_base64(&bytes).into_bytes(),
    })
}

/// 🎞️ The archive a [`whole_document_media`] carrier holds.
pub async fn whole_document_archive(data: &[u8]) -> Result<protocol::DocumentArchivePack, MediaArtifactError> {
    let text = std::str::from_utf8(data).map_err(|_| MediaArtifactError::Payload("a whole-document media carrier is pk: base64 text".into()))?;
    let bytes = store::pack_rt::pack_value_from_base64(text).map_err(|error| MediaArtifactError::Payload(error.to_string()))?;
    protocol::decode_document_archive_bytes(&bytes).await.map_err(|error| MediaArtifactError::Payload(error.to_string()))
}

/// 🚫️ The fault a refused media artifact answers: a foreign document schema under [`MEDIA_SCHEMA_MISMATCH_CODE`] with both schemas
/// as notice parameters, anything else as the runtime's internal fault.
pub fn media_artifact_fault(error: MediaArtifactError) -> Fault {
    match error {
        MediaArtifactError::SchemaMismatch { expected, found } => Fault::new(FaultOrigin::Framework, FaultCode::new(MEDIA_SCHEMA_MISMATCH_CODE), format!("document schema mismatch: expected {expected}, found {found}")).with_param("expected", expected).with_param("found", found),
        other => Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), other.to_string()),
    }
}

include!("../../🎞️media/🛫️intrinsic/🦀️.rs");

include!("../../🎞️media/💥️fault/🦀️.rs");

impl MediaArtifact {
    /// 🎞️ Owns exact intrinsic bytes and descriptor text from one borrowed media owner.
    pub fn encode_intrinsic(port:&str,media_type:MediaType,schema:&str,value:&DslValue,owner:&mut crate::io::control::NativeSnapshotEncodeOwner<'_ ,'_>)->Result<Self,MediaArtifactError>{
        let (port,schema,data)=encode_intrinsic_parts(port,schema,value,owner)?;
        Ok(Self{descriptor:MediaArtifactDescriptor{edge_id:None,port_id:Some(port),kind_id:None,media_type:Some(media_type),wire:MediaWireFormat::Intrinsic{schema},blob_hash:None},data})
    }

    /// 🎞️ Reads one full inline intrinsic payload under the caller's import control.
    pub fn decode_intrinsic(&self,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Media,MediaArtifactError>{
        let MediaWireFormat::Intrinsic{schema}=&self.descriptor.wire else{return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"media wire is not intrinsic").into())};
        if self.descriptor.blob_hash.is_some(){return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"intrinsic media requires inline bytes").into())}
        let media_type=self.descriptor.media_type.ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"intrinsic media requires a declared media type"))?;
        let schema=control.copy_text(schema)?;let value=store::pack_rt::decode_wire_value_controlled(&self.data,&store::PackDecodeOptions::default(),control).map_err(|error|MediaArtifactError::Value(error.into_value_error()))?;
        Ok(Media{media_type,payload:MediaPayload::Intrinsic{schema,value}})
    }
}

#[cfg(test)]
include!("../../🧪️tests/🔬️app-owned-media-error/🦀️.rs");

/// 🔀️ `PluginApp::transaction_prepare`'s result: `foreign` is this member's own recursively
/// discovered foreign steps (contract §5.4 — empty when its prepared ops touch no other
/// artifact); `rejection` is `Some` iff this instance declines the prepare (one of the frozen
/// rejection codes, contract §5's taxonomy). Both concrete/object-safe (no `A::Mutation`
/// leakage), matching `AppFrame::TransactionPrepared`'s own flat `{foreign, rejection}` shape.
pub struct TransactionPrepareOutcome {
    pub foreign: Vec<protocol::ForeignStep>,
    pub rejection: Option<Fault>,
}

/// 👥️ Object-safe channel snapshot of the two EPHEMERAL lanes plus the zero-app-code
/// declared-broadcast interaction slice (contract-freeze §C7.6) — replaces the earlier flat
/// `(Vec<u8>, u64, u64)` tuple `PluginApp::ephemeral_snapshot` returned, adding `interaction`
/// without further widening that tuple's arity. `interaction` is
/// `encode_presence_interaction(&assemble_presence_interaction(app_id, &interaction_state,
/// &hover_specs, &selection_specs))` over the app's own declared broadcast domains — empty bytes
/// when no domain is declared or broadcasting right now. Frames straight onto
/// `AppFrame::Ephemeral`'s four fields, same order.
#[derive(Clone, Debug, PartialEq)]
pub struct EphemeralSnapshot {
    pub presence: Vec<u8>,
    pub presence_generation: u64,
    pub transient_generation: u64,
    pub interaction: Vec<u8>,
    /// ⏯️ This instance's tool run summary (`PluginApp::tool_run_presence`, contract §3.4) — the local
    /// `PresencePeer.tool_run` the host's heartbeat assembly stamps; `None` without a run.
    pub tool_run: Option<protocol::PresenceToolRun>,
    /// ⏪️ This instance's open history edit (`TimeTravelLedger::presence`) — the local `PresencePeer.history_edit`
    /// a host stamps; `None` while no history edit is open.
    pub history_edit: Option<protocol::PresenceHistoryEdit>,
}

/// 🔒️ Contract §2.3: the string-action verbs a `Viewer`-role instance must reject at
/// `dispatch_action`'s entry — `import` is not a string action (see `PluginApp::import_media`'s
/// own check in `dispatch_import_media`), so it is not in this list. `checkoutCheckpoint` and
/// `switchAlternative` are rejected too: each commits a `Checkout` history transition to the shared event log, which
/// moves the head (and the scoped history edits it projects) for every replica, so it is a write, not a read cursor.
/// `try_build_definition` never declares these on a viewer app, so no host offers or binds them to a
/// Spectator; this guard is the backstop for a dispatch no manifest offered.
pub(crate) const VIEWER_REJECTED_ACTION_IDS: [&str; 9] = ["undo", "redo", "commitCheckpoint", "createAlternative", semio_framework::SWITCH_ALTERNATIVE_ACTION_ID, "checkoutCheckpoint", REVERT_TO_COMMAND_ACTION_ID, "cut", "paste"];

/// 🔒️ Whether a `Viewer`-role instance rejects `action`: the [`VIEWER_REJECTED_ACTION_IDS`] and every history-edit
/// verb (editing history supersedes inputs, which a viewer never may).
pub(crate) fn viewer_rejects_action(action: &str) -> bool {
    VIEWER_REJECTED_ACTION_IDS.contains(&action) || is_time_travel_action_id(action)
}

//#endregion 🔖️Limits

/// ⏪️ Whether `action` is one of the reserved, host-driven history-edit verbs.
pub fn is_time_travel_action_id(action: &str) -> bool {
    HISTORY_EDIT_ACTION_IDS.contains(&action)
}
