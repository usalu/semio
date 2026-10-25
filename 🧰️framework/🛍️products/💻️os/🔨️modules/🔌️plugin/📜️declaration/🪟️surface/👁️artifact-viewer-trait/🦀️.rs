//! 👁️ Artifact viewer trait: the read-only surface contract `ArtifactViewer` with its snapshot, view and viewer hooks. See [`super`].

use super::*;
use semio_framework_2d_compute::EngineHandles;

/// 👁️ Authoring trait for a read-only surface (contract §2.2) — the mutation-incapable twin of
/// `ArtifactEditor`. `Mutation` stays decode-only (the store's op log must still decode past
/// edits an `ArtifactEditor` made) but `handle` returns `ViewEmit`, which cannot structurally
/// carry an artifact or draft mutation. `ViewerApp<V>` below is the sole `ArtifactApp` implementor.
pub trait ArtifactViewer: Default + Send + 'static {
    /// 📬️ Borrows this viewer's original typed completion retirement birth.
    fn completion_retirement_birth_bytes(value:&ArtifactToolCompletionValue<ViewerApp<Self>>)->Option<usize>{semio_framework_plugin_decl_app::completion_retirement::derived_completion_birth_bytes(value)}
    /// 🎟️ Retains the viewer completion on every unsupported or unfunded cleanup admission.
    fn admit_completion_retirement(value:&mut Option<ArtifactToolCompletionValue<ViewerApp<Self>>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{semio_framework_plugin_decl_app::completion_retirement::derived_admit_completion_retirement(value,grant)}
    /// 🧮️ Declares the physical authority frame for this viewer's decoded editor-authored mutation batch.
    fn owned_mutation_batch_birth_bytes() -> Option<usize> { None }

    /// 📥️ Admits a decoded original batch through the viewer's explicit first-party owner.
    fn admit_owned_mutation_batch(_values: &mut Option<Vec<Self::Mutation>>, _grant: RetainedCloneGrant) -> Result<Option<(store::MemberStoreOwnedBatch, RetainedCloneProgress)>, ValueError> { Ok(None) }

    /// 🧩️ Read-only twin of `ArtifactEditor::Members` — the roster this viewer's composed children
    /// are opened through, carried by the app rather than by whoever registers it.
    type Members: ReferenceSpaceMember + MemberFactory + Send + 'static = NoMembers;
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
        Ok(bounded_document_store_initialization_job::<_,_,::semio_framework_os_kernel::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>(envelope, Self::DOCUMENT_SCHEMA, operation, generation, actor))
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
        Some(semio_framework_plugin_decl_views::presence_retirement::bounded_presence_store_disposer::<Self::Presence, Self::PresenceMutation>())
    }

    fn build_transient_store_disposer() -> ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        Some(bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin_decl_views::presence_retirement::bounded_presence_root_retirement_factory::<Self::Presence>())
    }

    /// 🎭️ Always `Viewer` in practice — defaulted so implementors never restate it.
    const ROLE: AppRole = AppRole::Viewer;
    const DIALECT: Dialect;
    /// 🪟️ Supplies the viewer's bounded projection of real parent child fields.
    fn child_restore_projection(_snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
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
    type Mutation: Clone + ::protocol::ColdMutation<Self::Snapshot> + protocol::SemanticMutation<Self::Snapshot> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Config: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + store::ConfigRecord + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    type ConfigMutation: Clone + ::protocol::ColdMutation<Self::Config> + ::protocol::Mutation<Self::Config> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::replication::OpBinaryReceiving + 'static;
    type Presence: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + store::ArtifactPresenceSnapshot + 'static;
    type PresenceMutation: Clone + ::protocol::ColdMutation<Self::Presence> + ::protocol::Mutation<Self::Presence> + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Transient: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    type TransientMutation: Clone + ::protocol::ColdMutation<Self::Transient> + ::protocol::Mutation<Self::Transient> + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Command: ::protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static;

    fn mounted_job_maintenance_demands(_instance_id: u32, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn mounted_job_maintenance_phase(_instance_id: u32) -> Option<MountedOwnerPhaseV1> { None }

    fn mounted_job_maintenance_step(_instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,_grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()))
    }

    fn mounted_job_close_demands(_instance_id: u32, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn mounted_job_close_step(_instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,_grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default()))
    }

    fn mounted_jobs_terminal_is_empty(_instance_id: u32) -> bool {
        true
    }

    fn mounted_job_prepare_snapshot_read(_operation: AppRenderOperationContext, _snapshot: &Self::Snapshot, _windows: &WindowConfigOwnerRegistry, _view: Option<&ViewModel>, _grant: RetainedCloneGrant) -> crate::MountedSnapshotReadPreparation {
        crate::MountedSnapshotReadPreparation::default()
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
    fn build_tool_job(_request:&mut Option<ArtifactOwnedToolJobRequest<ViewerApp<Self>>>,_output:&mut Option<semio_framework::ErasedToolJob>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"registered surface tool lacks its original source-preserving producer").into())
    }

    /// 🧭️ Builds a persistent producer/import job for one explicitly registered viewer route.
    fn build_reserved_tool_job(_request: ArtifactReservedToolJobRequest<ViewerApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        Ok(None)
    }

    /// 🎞️ Builds an owned resumable producer for an explicitly registered viewer media-output port.
    fn build_media_export_job(_request:&mut Option<ArtifactMediaExportJobRequest<ViewerApp<Self>>>,_body:&mut Option<Box<dyn semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob+Send>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"registered media surface lacks its concrete source-preserving producer").into())
    }

    /// 🧹️ Supplies the exact bounded disposer for snapshots retained by viewer output jobs.

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
        Some(semio_framework_plugin_decl_views::presence_retirement::bounded_presence_root_retirement_factory::<Self::Presence>())
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
    ) -> semio_framework_plugin_ownership::sdk_fault::OriginalRenderResult<ComponentTree> {
        let rendered:crate::UiAssemblyResult<semio_framework_ui_runtime::ComponentTree>=(||{
        let _ = (transient, interaction);
        Self::render(body_key, doc, cfg, view_state)

        })();
        rendered.map(Some).map_err(Into::into)
        }
    fn pending_effects(_owner: &ArtifactInstanceOperationOwnerHandle, _doc: &crate::ArtifactPendingView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view: Option<&ViewModel>) -> Vec<Effect> {
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
