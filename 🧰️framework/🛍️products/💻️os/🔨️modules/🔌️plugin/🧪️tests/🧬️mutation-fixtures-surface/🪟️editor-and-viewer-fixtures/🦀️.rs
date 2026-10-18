/// 🪟️ Editor and viewer fixtures of the surface fixture: the `ArtifactEditor` implementation of the editor fixture and the viewer fixture with its `ArtifactViewer` implementation.

impl ArtifactEditor for SurfaceEditorFixture {
    fn owned_mutation_batch_birth_bytes() -> Option<usize> { Some(store::MemberStoreOwnedBatch::scaffold_byte_demand::<Self::Mutation>()) }
    fn admit_owned_mutation_batch(values: &mut Option<Vec<Self::Mutation>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(store::MemberStoreOwnedBatch, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> { store::MemberStoreOwnedBatch::admit::<Self::Mutation>(values, grant) }
    const DIALECT: Dialect = SURFACE_TESTKIT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-surface/v1";
    type Snapshot = SurfaceSnapshot;
    type Mutation = SurfaceMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = crate::app_views::NoTransient;
    type TransientMutation = crate::app_views::NoTransientMutation;
    type Command = SurfaceEditorCommand;

    fn natural_file_codec() -> Option<crate::app_contract::NaturalFileCodec> {
        Some(crate::app_contract::NaturalFileCodec { format_kind: "semio.testkit-surface-natural/v1", extension: ".surface", media_type: "application/vnd.semio.testkit-surface", binary: true })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework::MediaError> {
        Ok(snapshot.count.to_be_bytes().to_vec())
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework::MediaError> {
        let bytes: [u8; 4] = bytes.try_into().map_err(|_| semio_framework::MediaError::Payload(crate::app_contract::NATURAL_FILE_PORT.into(), "surface natural file requires one signed 32-bit count".into()))?;
        Ok(SurfaceSnapshot { count: i32::from_be_bytes(bytes) })
    }

    fn build_natural_file_decode_owner(bytes: Vec<u8>, contract: ToolExecutionContract) -> Result<crate::app_contract::NaturalFileDecodeOwner<Self::Snapshot>, semio_framework::MediaError> {
        if !bytes.is_empty() && bytes.len() % 4 == 0 {
            Ok(crate::app_contract::NaturalFileDecodeOwner::Controlled(Box::new(SurfaceNaturalDecodeCursor::new(bytes,contract.max_work_units_per_step as usize))))
        } else {
            Ok(crate::app_contract::NaturalFileDecodeOwner::Synchronous(bytes))
        }
    }

    fn build_instance_operation_owner() -> Box<dyn crate::tool_contract::ArtifactInstanceOperationOwner> {
        media_owner_context::owner("editor-instance")
    }

    fn export_media_with_request_context(
        owner: &crate::tool_contract::ArtifactInstanceOperationOwnerHandle,
        port: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _transient: &crate::window_transient::TransientView<'_, Self::Transient>,
    ) -> Result<Media, semio_framework::MediaError> {
        if port == "retained:out" {
            media_owner_context::export(owner, port, doc)
        } else {
            Self::export_media(port, doc)
        }
    }

    crate::bounded_first_step_tool_proofs! {
        owner: EditorApp<SurfaceEditorFixture>, owner_file: "plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs", controller: "testkit.surface@1/*#editor", artifact_schema: "semio.testkit-surface/v1",
        factory: "SurfaceFixtureFactory", factory_type: SurfaceFixtureFactory,
        contract: SURFACE_TOOL_CONTRACT, tools: ["increment"]
    }

    fn command_id(command: &Self::Command) -> &'static str {
        match command {
            SurfaceEditorCommand::Increment => SURFACE_TOOL_ID,
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(SurfaceFixtureFactory { keys: vec![ToolFactoryKey::new(registry.controller_id(), SURFACE_TOOL_ID)] })
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        let job = SurfaceFixtureJob { owners: crate::app_testing::mutation_fixture::job_close::FixtureJobOwners::new(request.command, request.completion).with_context(request.context), count: request.snapshot.count, page: 0 };
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(crate::app_testing::mutation_fixture::wire::preparation_factory::<Self::Snapshot, Self::Mutation>("testkit-surface-artifact-retained"))
    }

    fn document_store_owners_source_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { crate::app_testing::mutation_fixture::wire::document_store_owners_source_demands::<Self::Snapshot, Self::Mutation>() }
    fn build_document_store_owners(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(crate::app_testing::mutation_fixture::wire::admit_document_store_owners::<Self::Snapshot, Self::Mutation>("testkit-surface-artifact-retained", grant))
    }
    fn build_document_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::ArtifactStore<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }
    fn build_config_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::ConfigStore<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }
    fn build_draft_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::DraftStore<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_disposer::<Self::Draft, Self::DraftMutation>())
    }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::app_testing::mutation_fixture::no_state::presence_local_root_retirement_factory())
    }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::app_testing::mutation_fixture::no_state::presence_peer_retirement_factory())
    }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(crate::app_testing::mutation_fixture::no_state::transient_local_root_retirement_factory())
    }
    fn build_presence_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        Some(crate::app_testing::mutation_fixture::no_state::presence_store_disposer())
    }
    fn build_transient_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        Some(crate::app_testing::mutation_fixture::no_state::transient_store_disposer())
    }
    fn initial_snapshot() -> SurfaceSnapshot {
        SurfaceSnapshot::default()
    }

    fn handle(
        command: &SurfaceEditorCommand,
        doc: &ArtifactView<'_, SurfaceSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &crate::app_views::InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, NoDraft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<SurfaceMutation>, Fault> {
        match command {
            SurfaceEditorCommand::Increment => Ok(Emit { artifact_mutations: vec![SetSurfaceCount { value: doc.snapshot.count + 1 }.into()], ..Default::default() }),
        }
    }

    fn render(_body_key: &str, doc: &ArtifactView<'_, SurfaceSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("count={}", doc.snapshot.count)))
    }
}

#[derive(Default)]
struct SurfaceViewerFixture;

impl ArtifactViewer for SurfaceViewerFixture {
    fn owned_mutation_batch_birth_bytes() -> Option<usize> { Some(store::MemberStoreOwnedBatch::scaffold_byte_demand::<Self::Mutation>()) }
    fn admit_owned_mutation_batch(values: &mut Option<Vec<Self::Mutation>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(store::MemberStoreOwnedBatch, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> { store::MemberStoreOwnedBatch::admit::<Self::Mutation>(values, grant) }
    const DIALECT: Dialect = SURFACE_TESTKIT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-surface/v1";
    type Snapshot = SurfaceSnapshot;
    type Mutation = SurfaceMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = crate::app_views::NoTransient;
    type TransientMutation = crate::app_views::NoTransientMutation;
    type Command = SurfaceViewerCommand;

    fn build_instance_operation_owner() -> Box<dyn crate::tool_contract::ArtifactInstanceOperationOwner> {
        media_owner_context::owner("viewer-instance")
    }

    fn export_media_with_request_context(
        owner: &crate::tool_contract::ArtifactInstanceOperationOwnerHandle,
        port: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _transient: &crate::window_transient::TransientView<'_, Self::Transient>,
    ) -> Result<Media, semio_framework::MediaError> {
        if port == "retained:out" {
            media_owner_context::export(owner, port, doc)
        } else {
            Self::export_media(port, doc)
        }
    }

    fn mounted_job_maintenance_demands(instance_id: u32, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(if instance_id == u32::MAX { semio_framework_value::RetirementDemand { release_bytes: 7, depth: 1, ..Default::default() } } else { Default::default() })
    }

    fn mounted_job_maintenance_step(instance_id:u32,cx:&mut semio_framework_job::StepContext<'_>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let demand=Self::mounted_job_maintenance_demands(instance_id,grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,_|Ok(if instance_id==u32::MAX{(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}),false)}else{(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(Default::default()),true)}))
    }
    fn mounted_job_close_step(instance_id:u32,cx:&mut semio_framework_job::StepContext<'_>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,Default::default(),|_,_|Ok(if instance_id==u32::MAX{(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked{reason:"surface viewer lifecycle witness"},false)}else{(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(Default::default()),true)}))
    }

    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        instance_id != u32::MAX
    }

    fn config_schema() -> &'static str {
        "semio.testkit-surface.viewer-config/v1"
    }

    fn command_id(_command: &Self::Command) -> &'static str {
        "surface-viewer-noop"
    }

    fn build_document_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::ArtifactStore<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }
    fn build_config_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::ConfigStore<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::app_testing::mutation_fixture::no_state::presence_local_root_retirement_factory())
    }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::app_testing::mutation_fixture::no_state::presence_peer_retirement_factory())
    }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(crate::app_testing::mutation_fixture::no_state::transient_local_root_retirement_factory())
    }
    fn build_presence_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        Some(crate::app_testing::mutation_fixture::no_state::presence_store_disposer())
    }
    fn build_transient_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        Some(crate::app_testing::mutation_fixture::no_state::transient_store_disposer())
    }
    fn initial_snapshot() -> SurfaceSnapshot {
        SurfaceSnapshot::default()
    }

    fn handle(
        _command: &SurfaceViewerCommand,
        _doc: &ArtifactView<'_, SurfaceSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &crate::app_views::InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<NoConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(_body_key: &str, doc: &ArtifactView<'_, SurfaceSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("count={}", doc.snapshot.count)))
    }
}
