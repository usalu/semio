//! ✏️ Artifact editor trait: the editable surface contract `ArtifactEditor` with its snapshot, mutation, view and history hooks. See [`super`].

use super::*;
use semio_framework_2d_compute::EngineHandles;

pub trait ArtifactEditor: Default + Send + 'static {
    /// 📬️ Borrows this editor's original typed completion retirement birth.
    fn completion_retirement_birth_bytes(value:&ArtifactToolCompletionValue<EditorApp<Self>>)->Option<usize>{semio_framework_plugin_decl_app::completion_retirement::derived_completion_birth_bytes(value)}
    /// 🎟️ Retains the editor completion on every unsupported or unfunded cleanup admission.
    fn admit_completion_retirement(value:&mut Option<ArtifactToolCompletionValue<EditorApp<Self>>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{semio_framework_plugin_decl_app::completion_retirement::derived_admit_completion_retirement(value,grant)}
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
    type Members: ReferenceSpaceMember + MemberFactory + Send + 'static = NoMembers;
    const DIALECT: Dialect;
    /// 🧬️ Supplies the editor's bounded projection of real parent child fields.
    fn child_restore_projection(_snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
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
    type Snapshot: ::semio_framework_os_kernel::StoreState + PartialEq + semio_framework_value::ToValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactPack + pack::snapshot::ArtifactPackMetadata + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + 'static;
    type Mutation: ::semio_framework_os_kernel::StoreMutationFor<Self::Snapshot> + protocol::SemanticMutation<Self::Snapshot> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Config: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + store::ConfigRecord + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    type ConfigMutation: Clone + ::protocol::ColdMutation<Self::Config> + ::protocol::Mutation<Self::Config> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::replication::OpBinaryReceiving + 'static;
    type Draft: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    type DraftMutation: Clone + ::protocol::ColdMutation<Self::Draft> + ::protocol::Mutation<Self::Draft> + store::ArtifactCanonicalJsonTree + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Presence: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + store::ArtifactPresenceSnapshot + 'static;
    type PresenceMutation: Clone + ::protocol::ColdMutation<Self::Presence> + ::protocol::Mutation<Self::Presence> + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Transient: semio_framework_value::retirement::RetireOwned + Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    type TransientMutation: Clone + ::protocol::ColdMutation<Self::Transient> + ::protocol::Mutation<Self::Transient> + PartialEq + semio_framework_value::retirement::RetireOwned + Send + ::protocol::OpText + ::protocol::OpBinary + 'static;
    type Command: ::protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static;

    /// 📜️ Exact owner-local bounded reducer proofs forwarded by `EditorApp<Self>`.
    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        Vec::new()
    }

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
    fn typing_fold(_net: &[Self::Mutation], _next: &[Self::Mutation]) -> semio_framework_tool_machine::TypingFold<Self::Mutation> {
        semio_framework_tool_machine::TypingFold::Split
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
    fn build_tool_job(_request:&mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>,_output:&mut Option<semio_framework::ErasedToolJob>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"registered surface tool lacks its original source-preserving producer").into())
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
    fn build_media_export_job(_request:&mut Option<ArtifactMediaExportJobRequest<EditorApp<Self>>>,_body:&mut Option<Box<dyn semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob+Send>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"registered media surface lacks its concrete source-preserving producer").into())
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
        Some(semio_framework_plugin_decl_views::presence_retirement::bounded_presence_root_retirement_factory::<Self::Presence>())
    }

    /// 👥️ Grants exact typed retirement for peer presence snapshots to this editor adapter.
    /// Framework-owned by default, for the same reason as `ArtifactApp`'s twin: `EditorApp<E>`
    /// forwards THIS answer, so a `None` here would take the framework's default back out.
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin_decl_views::presence_retirement::bounded_presence_root_retirement_factory::<Self::Presence>())
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
        Self::admit_document_store_publication_authority(operation, generation);
        Ok(bounded_document_store_initialization_job::<_,_,EditorDocumentStoreOwnerProducer<Self>>(envelope, Self::DOCUMENT_SCHEMA, operation, generation, actor))
    }

    /// 🎟️ Grants the lease the framework-default initializer's replacement presents to [`Self::validate_document_store_publication`]; a domain with [`Self::REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY`] admits its own here, every other domain keeps this no-op.
    fn admit_document_store_publication_authority(_operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation) {}

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
        Some(semio_framework_plugin_decl_views::presence_retirement::bounded_presence_store_disposer::<Self::Presence, Self::PresenceMutation>())
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
    fn reference_birth() -> Option<semio_framework_plugin_decl_app::ReferenceArtifactAppBirth<Self::Snapshot>> { None }
    fn initial_config() -> Self::Config {
        Self::Config::default()
    }
    /// 🧭️ Resolves a host-owned control-plane action to one configuration mutation.
    /// `EditorApp<E>` forwards this exact owner hook to the runtime adapter.
    fn original_host_configuration(_action:&str)->Option<&'static semio_framework_plugin_decl_emission::OriginalHostConfigurationDeclaration<Self::ConfigMutation>>{None}
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
    /// 🏭️ The declared native producer owns original action arguments, command birth and exact operation wire.
    fn original_action_command_producer()->Option<&'static dyn semio_framework_plugin_decl_app::tool_contract::ArtifactOriginalCommandProducerFactory<Self::Command>>{None}
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
    fn pending_effects(_owner: &ArtifactInstanceOperationOwnerHandle, _doc: &crate::ArtifactPendingView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view: Option<&ViewModel>) -> Vec<Effect> {
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
    /// 🧳️ Retains query state under the original render identity until funded retirement.
    type RenderWork: semio_framework_value::retirement::RetireOwned + Send + 'static = ();
    fn uses_render_work(_body_key: &str) -> bool { false }
    /// 📨️ Retains only the original donor accepted for this exact window render work.
    fn receive_render_work_request(_work:&mut Self::RenderWork,_request:&mut Option<semio_framework::action_staging::StagedFormRequest>,_view:&ViewModel,_cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<()>,semio_framework_job::OriginalTurnFailure>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"staging.render-request-unsupported").into())}
    /// 🎟️ Creates only the work admitted and receipted by this original turn.
    fn begin_render_work(_body_key: &str, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<Self::RenderWork>, semio_framework_job::OriginalTurnFailure> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact did not declare retained render work").into())
    }
    /// 📏️ Quotes the next original query action before the scheduler lends its independent physical currencies.
    fn render_work_demand(_work:&Self::RenderWork,_maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(Default::default())}
    /// 🪢️ Advances the same query owner; a completed tree remains private until retirement.
    fn render_with_work_context(
        _work: &mut Self::RenderWork,
        _owner: &ArtifactInstanceOperationOwnerHandle,
        _body_key: &str,
        _source: &std::sync::Arc<Self::Snapshot>,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _view_state: &ViewModel,
        _transient: &TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> semio_framework_plugin_ownership::sdk_fault::OriginalRenderResult<ComponentTree> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact did not implement retained render work").into())
    }
    fn render_with_request_context(
        owner: &ArtifactInstanceOperationOwnerHandle,
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
        let codec=<Self::Snapshot as store::os_store::serialization::ArtifactStoreSnapshot>::reference_snapshot_codec().ok_or(MediaError::NotImplemented)?;
        let bytes=(codec.encode)(doc.snapshot).map_err(|error|MediaError::Payload(port.to_string(),error.to_string()))?;
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
            return document_reference_load_emit(&snapshot, Self::DOCUMENT_SCHEMA);
        }
        if port != "artifact:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
        };
        let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let codec=<Self::Snapshot as store::os_store::serialization::ArtifactStoreSnapshot>::reference_snapshot_codec().ok_or(MediaError::NotImplemented)?;
        let snapshot=(codec.decode)(&bytes).map_err(|error| MediaError::Payload(port.to_string(),error.to_string()))?;
        document_reference_load_emit(&snapshot, Self::DOCUMENT_SCHEMA)
    }
    /// 🌀️ `io-async-signatures`: async signature (delegates to the now-async `export_media`).
    fn media_fingerprint(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<MediaFingerprint, MediaError> {
        match Self::export_media(port, doc) {
            Ok(media) => Ok(MediaFingerprint::of(&media)),
            Err(error) => Err(error),
        }
    }
}

/// 🏭️ Uses the surface's actual owner catalog for every native replay initialization.
struct EditorDocumentStoreOwnerProducer<A>(std::marker::PhantomData<fn()->A>);
impl<A:ArtifactEditor> store::os_store::document_store_owners::DocumentStoreOwnerProducer<A::Snapshot,A::Mutation> for EditorDocumentStoreOwnerProducer<A>{
 fn source_demands()->Result<RetirementDemand,ValueError>{A::document_store_owners_source_demands()}
 fn admit_sources(grant:RetainedCloneGrant)->Result<(store::os_store::document_store_owners::DocumentStoreOwners<A::Snapshot,A::Mutation>,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<A::Snapshot,A::Mutation>>{
  A::build_document_store_owners(grant).unwrap_or_else(||Err(store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError{error:ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"surface did not supply its declared native document owner catalog"),owners:None,progress:Default::default()}))
 }
}
