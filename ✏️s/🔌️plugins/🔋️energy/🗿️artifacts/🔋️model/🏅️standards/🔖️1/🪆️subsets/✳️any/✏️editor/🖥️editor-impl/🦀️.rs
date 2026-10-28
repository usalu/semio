//! 🖥️ The `EnergyModelEditor` type with its `ArtifactEditor` implementation and the body renderer.

use super::*;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct EnergyModelEditor;

impl ArtifactEditor for EnergyModelEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<ExampleSource> {
        examples()
    }
    type Snapshot = EnergyModelSnapshot;
    type Mutation = EnergyModelMutation;
    type Config = EnergyModelConfig;
    type ConfigMutation = EnergyModelConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = EnergyModelEditorCommand;
    /// 🧩️ `structure`/`zones` are `s.stdio.semio@v1/{value,table}` members, so the roster that
    /// resolves their genesis dialects is stdio's closed `SemioMembers` (same as gis/sourcing).
    type Members = semio_s_artifact_stdio_semio::SemioMembers;

    const DIALECT: Dialect = MODEL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = ENERGY_MODEL_DOCUMENT_SCHEMA;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        crate::snapshot::energy_child_restore_projection(snapshot)
    }

    /// 🌱️ Both composed children derive from the model — see `crate::energy_genesis_child_pack`.
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::snapshot::energy_genesis_child_pack(snapshot, slot, child_id)
    
})())
}

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::tool_contract::ArtifactBoundedFirstStepProof> {
        ENERGY_MODEL_RETAINED_TOOL_IDS.iter().map(|tool| semio_framework_plugin::tool_contract::ArtifactBoundedFirstStepProof::new::<EditorApp<EnergyModelEditor>>(
            "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
            "s.energy.model@1/*#editor", "EnergyModelCommandJobFactory", tool, "energy.model", energy_model_contract(),
        ).with_factory_type::<EditorApp<EnergyModelEditor>, EnergyModelCommandJobFactory>()).collect()
    }

    /// 🎥️ The 3d window's own retained orbit pose — one bounded window-config store, keyed by window
    /// INSTANCE, so two open 3d panes never share a camera.
    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<model_window::config::EnergyModelWindowConfigOwner>()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(EnergyModelCommandJobFactory::new(&controller))
    }

    fn original_action_command_producer()->Option<&'static dyn semio_framework_plugin::tool_contract::ArtifactOriginalCommandProducerFactory<Self::Command>>{Some(&editor_commands::ENERGY_ORIGINAL_ACTION_FACTORY)}

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework::ErasedToolJob>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original tool producer request is absent"))?;
        if !ENERGY_MODEL_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.action_id() != original.tool_id {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "the energy model command does not match its exact registered tool").into());
        }
        if energy_model_extent(&original.command, &original.snapshot, &original.interaction_state).is_none() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "the energy model bounded route exceeded its declared work extent").into());
        }
        let tool_id = original.command.action_id();
                semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, EnergyModelEditorCommand::action_id, ENERGY_MODEL_RETAINED_RAW_BYTES, 1, false, |_request| EnergyModelCommandWork::new(tool_id))
    }


    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }


    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }


    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(EnergyModelStorePreparationFactory::default()))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(store::snapshot_clone_preparation::config_apply_preparation_factory::<Self::Config, Self::ConfigMutation>())
    }

    /// ⏯️ The `energySimulation` run job over the run's base snapshot and the config store's settings.
    /// The run is read-only and declares no `revalidateJob`, so only the `Run` purpose builds a job.
    fn build_tool_run_job(request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        if request.tool_id != tools::simulation::TOOL_ID || request.purpose != ToolRunJobPurpose::Run {
            return Ok(None);
        }
        let weather = if let Some(link) = request.snapshot.weather_link.as_ref() {
            if link.target.dialect.artifact_kind != "s.stdio.epw" || link.target.dialect.standard != "energyplus" || link.target.dialect.subset != "*" {
                return Err(Fault::from("The weather link must address a Stdio EPW EnergyPlus artifact. / Die Wetterverknüpfung muss auf ein Stdio-EPW-EnergyPlus-Artefakt verweisen."));
            }
            let host = request.host.as_ref().ok_or_else(||Fault::from("Weather artifact resolution is unavailable. / Das Wetterartefakt kann nicht aufgelöst werden."))?;
            Some(host.resolve_link_retained(semio_framework_pack_json::to_json_string(link),request.port.request_waker())?)
        } else { None };
        let job=EnergySimulationRunJob::new(request.identity, request.snapshot, request.config.simulation_template());
        Ok(Some(Box::new(match weather { Some(receiver)=>job.with_linked_weather(receiver,request.port),None=>job })))
    }



    fn command_id(command: &EnergyModelEditorCommand) -> &'static str {
        command.action_id()
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        crate::editor_commands::args_bridge::command_from_action(action, args)
    }

    /// ✏️ Delegates to the same pure [`reduce`] the retained bounded work step runs, so the
    /// interactive and retained routes can never diverge.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        if let Some(camera) = camera_emit(command, _view_state) {
            return camera;
        }
        reduce(command, doc)
    }

    /// 🎨️ The plain render path: no request context, so no live interaction domain to read. The 3d
    /// model window still renders — it simply paints nothing as selected or hovered.
    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        render_body(body_key, doc, cfg, view_state, &EnergyModelInteractionSnapshot::default())
    }

    /// 🕹️ The request-context render: resolves the framework-owned `energyModel` domain ONCE and
    /// threads its selection/hover into the whole body, so the 3d window's paint and (later) the
    /// inspector read one authority instead of either surface keeping selection of its own.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &semio_framework_plugin::ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        render_body(body_key, doc, cfg, view_state, &EnergyModelInteractionSnapshot::from_interaction(interaction))

        })();
        rendered.map(Some).map_err(Into::into)
        }
}

/// 🎨️ The one body dispatch both render entry points share, so the plain and the request-context
/// routes can never diverge.
fn render_body(
    body_key: &str,
    doc: &ArtifactView<'_, EnergyModelSnapshot>,
    cfg: &ConfigView<'_, EnergyModelConfig>,
    view_state: &semio_framework_plugin::ViewModel,
    interaction: &EnergyModelInteractionSnapshot,
) -> UiAssemblyResult<ComponentTree> {
    let node = match body_key {
        structure::BODY_KEY => structure::render(doc.snapshot)?,
        zones::BODY_KEY => zones::render(doc.snapshot)?,
        simulation::BODY_KEY => simulation::render(doc.original_tool_run(), cfg.snapshot, &doc.snapshot.model, doc.snapshot.weather_link.as_ref(), view_state.locale),
        model_window::BODY_KEY => {
            let energy=doc.original_tool_run().filter(|run|run.tool_id==crate::editor_edit::tools::simulation::TOOL_ID).and_then(|run|run.payload).and_then(crate::editor_results::indexed::IndexedEnergyResults::new);
            model_window::render_indexed(&doc.snapshot.model.read(),interaction,energy.as_ref(),crate::editor_config::result_field(cfg.snapshot),view_state.locale,model_window::config::current(cfg))?

        }
        artifact_panel::BODY_KEY => artifact_panel::render(doc.snapshot, interaction, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, artifact_panel::BODY_KEY))?,
        inspection_panel::BODY_KEY => inspection_panel::render(doc.snapshot, interaction, cfg.snapshot, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, inspection_panel::BODY_KEY))?,
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("energy.model.render", "the unknown-body label could not be assembled"))?,
    };
    Ok(semio_framework_plugin::built_to_component_tree(node))
}
//#endregion 🔖️Editor
