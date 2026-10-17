//! 🏠️ The studio app instance: bounded retained command jobs, config store preparation and the artifact app implementation.

use super::*;

//#region 🔖️SpaceApp
/// 🧪️ Unit app instance — config lives in `SpaceConfig`; remote presence stays unavailable until
/// `ArtifactHost` supplies its typed instance-owned presence view.
#[derive(Default, Clone, Copy)]
pub struct SpaceApp;

//#region 🧵️RetainedCommands
pub(super) const SPACE_BOUNDED_TOOL_IDS: &[&str] = &[
    "setActivePanelTab",
    "nodeGraphViewport",
    "presenceHeartbeat",
    "workflowEngagementInput",
    "compiledDagEngagementInput",
    "closeFocusedInstance",
    "setActiveExample",
    "importSpacePack",
    "goHome",
    "navigateVirtualFileSystemNode",
    "spawnApp",
    "deleteSelection",
    "openSpace",
    "openInstance",
    "importSpacePackPayload",
    "setAppRegistrations",
    "patchParameter",
    "addParameter",
    "removeParameter",
    "moveMediaNode",
    "connectMediaPorts",
    "disconnectMediaEdge",
    "removeAppInstance",
    "copyAppInstance",
    "duplicateAppInstance",
    "pasteAppInstance",
    "renameAppInstance",
    "patchMediaNodes",
    "patchAppInstances",
    "bindParameterField",
    "unbindParameterField",
    "reorganizeWorkflow",
    "workflowEngagementSubmit",
    "compiledDagEngagementSubmit",
    "nodeGraphEdit",
    "exportMedia",
    "importMedia",
    "importMediaPayload",
    "exportStudioPack",
    "exportStudioDsl",
];
const SPACE_RETAINED_PAYLOAD_SCHEMA: &str = "os.workflow.space.tool-command.v1";
/// 📦️ `setAppRegistrations` carries the whole live catalog as one JSON argument and
/// `importSpacePackPayload` a whole base64 `.pack` data URL, so the studio's wire ceiling is sized for
/// a catalog, not for a click.
const SPACE_BOUNDED_RAW_BYTES: usize = 4 * 1024 * 1024;
const SPACE_BOUNDED_WORK_ITEMS: usize = 1;

/// 🧾️ The single contract this app's factory publishes AND its proof rows carry —
/// `validate_tool_job_rows` joins the two by exact equality, so both sides must read it from here.
const SPACE_BOUNDED_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

fn space_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(SPACE_BOUNDED_RAW_BYTES, 64, SPACE_BOUNDED_WORK_ITEMS as u64, SPACE_BOUNDED_OUTPUT_BYTES, 7_500)
}

fn space_bounded_extent(command: &SpaceCommand, _snapshot: &WorkflowSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    SPACE_BOUNDED_TOOL_IDS.contains(&command.command_id()).then_some(SPACE_BOUNDED_WORK_ITEMS)
}

#[expect(clippy::too_many_arguments, reason = "BoundedArtifactCommandWork requires the full retained reducer context callback")]
fn space_bounded_reduce(
    command: &SpaceCommand,
    snapshot: &WorkflowSnapshot,
    config: &SpaceConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<SpaceApp>>,
    operation: &AppOperationContext,
) -> Result<Emit<WorkflowMutation, SpaceConfigMutation, NoDraftMutation>, Fault> {
    if !SPACE_BOUNDED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.retained.route"), "the bounded Space reducer rejects document, registry, payload, and graph routes"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    // 🕹️ `openInstance` with no explicit node and `deleteSelection` read the live `graph` selection,
    // which the macro-generated 3-arg `dispatch` cannot see — the retained path reads it off the same
    // `InteractionState` `SpaceApp::handle` passes to their `apply`.
    let selected = || interaction.selection.get(S_PLAY_INTERACTION_DOMAIN).map_or_else(Vec::new, |selection| selection.ids.clone());
    if let SpaceCommand::OpenInstance(payload) = command {
        return Ok(semio_hub_space_operations::resolve_future(semio_hub_space_commands_authoring::open_instance::open_with_selection(payload, &doc, config, &selected())));
    }
    if let SpaceCommand::DeleteSelection(_) = command {
        return Ok(semio_hub_space_operations::resolve_future(semio_hub_space_commands_authoring::delete_selection::delete_selected(config, &selected())));
    }
    match command {
        SpaceCommand::NodeGraphEdit(payload) => return semio_hub_space_operations::resolve_future(semio_hub_space_commands_authoring::node_graph_edit::edit(payload, &doc)),
        SpaceCommand::ReorganizeWorkflow(_) => return Ok(semio_hub_space_operations::resolve_future(semio_hub_space_commands_authoring::reorganize_workflow::reorganize_selected(&doc, &selected()))),
        SpaceCommand::CopyAppInstance(_) => return Ok(Emit::config(vec![SpaceConfigMutation::SetClipboard(ClipboardSetting { node_ids: selected() })])),
        SpaceCommand::DuplicateAppInstance(_) => return Ok(semio_hub_space_operations::resolve_future(semio_hub_space_commands_authoring::duplicate_app_instance::duplicate_nodes(selected(), snapshot))),
        SpaceCommand::RemoveAppInstance(payload) => return Ok(semio_hub_space_operations::resolve_future(semio_hub_space_commands_authoring::remove_app_instance::remove_with_selection(payload, config, &selected()))),
        SpaceCommand::RenameAppInstance(payload) => return Ok(semio_hub_space_operations::resolve_future(semio_hub_space_commands_authoring::rename_app_instance::rename_with_selection(payload, &doc, config, &selected()))),
        _ => {}
    }
    if let SpaceCommand::PresenceHeartbeat(payload) = command {
        let identity = context
            .and_then(|context| context.view_state.as_ref())
            .and_then(crate::home_session_identity)
            .ok_or_else(|| Fault::from("s.space.session-identity-required"))?;
        return semio_hub_space_commands_media::presence_heartbeat::handle_with_identity(payload, identity, &doc, &ConfigView { snapshot: config, window: None });
    }
    command.dispatch(&doc, &ConfigView { snapshot: config, window: None })
}

pub struct SpaceCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl SpaceCommandJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: SPACE_BOUNDED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for SpaceCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<SpaceApp>;
    type Job = ArtifactRetainedCommandJob<SpaceApp>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SPACE_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        space_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SPACE_BOUNDED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("bounded Space command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SpaceCommandJobFactory {
    type Owner = SpaceApp;
    const TOOL_IDS: &'static [&'static str] = SPACE_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = S_WORKFLOW_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActivePanelTab", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "presenceHeartbeat", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "workflowEngagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "compiledDagEngagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "closeFocusedInstance", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importSpacePack", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "goHome", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "navigateVirtualFileSystemNode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "spawnApp", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "openSpace", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "openInstance", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importSpacePackPayload", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setAppRegistrations", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchParameter", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addParameter", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeParameter", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveMediaNode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "connectMediaPorts", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "disconnectMediaEdge", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeAppInstance", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "copyAppInstance", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "duplicateAppInstance", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "pasteAppInstance", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "renameAppInstance", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchMediaNodes", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchAppInstances", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "bindParameterField", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "unbindParameterField", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "reorganizeWorkflow", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "workflowEngagementSubmit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "compiledDagEngagementSubmit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "exportMedia", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importMedia", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importMediaPayload", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "exportStudioPack", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "exportStudioDsl", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️ConfigStorePreparation
/// 📏️ Sized for a real session, not for a single toggle: `openSpace` publishes a space id, a focus
/// reset, a clipboard reset and an active node in one turn, and a studio's clipboard/collapsed sets
/// are node-id lists.
pub(crate) const SPACE_CONFIG_RETAINED_BYTES: usize = 65_536;
//#endregion 📬️ConfigStorePreparation

impl ArtifactApp for SpaceApp {
    const DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.space.studio", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };
    type Snapshot = WorkflowSnapshot;
    type Mutation = WorkflowMutation;
    type Config = SpaceConfig;
    type ConfigMutation = SpaceConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = SpacePresence;
    type PresenceMutation = SpacePresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = SpaceCommand;

    const APP_ID: &'static str = S_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = S_WORKFLOW_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: SpaceApp,
        owner_file: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs",
        controller: "s.space.studio@1/*#editor",
        artifact_schema: "os.workflow",
        factory: "SpaceCommandJobFactory",
        factory_type: SpaceCommandJobFactory,
        contract: space_bounded_contract(),
        tools: [
            "setActivePanelTab",
            "nodeGraphViewport",
            "presenceHeartbeat",
            "workflowEngagementInput",
            "compiledDagEngagementInput",
            "closeFocusedInstance",
            "setActiveExample",
            "importSpacePack",
            "goHome",
            "navigateVirtualFileSystemNode",
            "spawnApp",
            "deleteSelection",
            "openSpace",
            "openInstance",
            "importSpacePackPayload",
            "setAppRegistrations",
            "patchParameter",
            "addParameter",
            "removeParameter",
            "moveMediaNode",
            "connectMediaPorts",
            "disconnectMediaEdge",
            "removeAppInstance",
            "copyAppInstance",
            "duplicateAppInstance",
            "pasteAppInstance",
            "renameAppInstance",
            "patchMediaNodes",
            "patchAppInstances",
            "bindParameterField",
            "unbindParameterField",
            "reorganizeWorkflow",
            "workflowEngagementSubmit",
            "compiledDagEngagementSubmit",
            "nodeGraphEdit",
            "exportMedia",
            "importMedia",
            "importMediaPayload",
            "exportStudioPack",
            "exportStudioDsl",
        ]
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        crate::space_retained_store_preparation::<Self::Snapshot, Self::Mutation>("space-studio-artifact-retained", SPACE_BOUNDED_OUTPUT_BYTES)
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(SpaceCommandJobFactory::new(&controller))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        crate::space_retained_store_preparation::<Self::Config, Self::ConfigMutation>("space-studio-config-retained", SPACE_CONFIG_RETAINED_BYTES)
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !SPACE_BOUNDED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.retained.tool-mismatch"), "Space command does not match its exact registered tool"));
        }
        if space_bounded_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.retained.extent"), "Space bounded route exceeded its declared work extent"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, space_bounded_reduce, space_bounded_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation: operation_context, completion: request.completion },
            SpaceCommand::command_id,
            SPACE_BOUNDED_RAW_BYTES,
            SPACE_BOUNDED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    async fn initial_snapshot() -> WorkflowSnapshot {
        empty_workflow_snapshot().await
    }

    async fn command_id(command: &SpaceCommand) -> &'static str {
        command.command_id()
    }

    /// 🪪️ `s.space.space`'s config+presence schema descriptor (ticket
    /// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) — `register_document_app` registers it the
    /// moment this type is bound to the plugin, completing the app-schema declaration for `🪐️space`.
    async fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(semio_hub_space_foundation::config::schema::app_schema_descriptor().await)
    }

    /// 🎯️ Bridges shell `{action,args}` JSON onto typed `SpaceCommand` until every call site speaks OpBinary.
    async fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<SpaceCommand, Fault> {
        if let Some(fault)=original_fault.take(){return Err(fault)}
        let args=args.as_ref();
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_str).map(str::to_string);
        let f64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_f64);
        let string_vec = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_array).map(|items| items.iter().filter_map(DslValue::as_str).map(str::to_string).collect::<Vec<_>>()).unwrap_or_default();
        let json_field = |key: &str| args.and_then(|value| value.get(key)).map(|raw| raw.as_str().map_or_else(|| semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(raw)), str::to_string));
        let node_id = || str_field("nodeId").or_else(|| str_field("node_id")).or_else(|| str_field("instanceId")).or_else(|| str_field("instance_id"));
        match action {
            "patchParameter" => Ok(SpaceCommand::PatchParameter(semio_hub_space_commands_authoring::patch_parameter::PatchParameter {
                parameter_id: str_field("parameterId").or_else(|| str_field("parameter_id")).unwrap_or_default(),
                field: str_field("field").unwrap_or_default(),
                value: json_field("value").unwrap_or_else(|| "null".into()),
            })),
            "addParameter" => Ok(SpaceCommand::AddParameter(semio_hub_space_commands_authoring::add_parameter::AddParameter { name: str_field("name").unwrap_or_else(|| "Parameter".into()), kind: str_field("kind").or_else(|| str_field("type")).unwrap_or_else(|| "numeric".into()) })),
            "removeParameter" => Ok(SpaceCommand::RemoveParameter(semio_hub_space_commands_authoring::remove_parameter::RemoveParameter { parameter_id: str_field("parameterId").or_else(|| str_field("parameter_id")).unwrap_or_default() })),
            "spawnApp" => Ok(SpaceCommand::SpawnApp(semio_hub_space_commands_authoring::spawn_app::SpawnApp {
                plugin_id: str_field("pluginId").or_else(|| str_field("plugin_id")).unwrap_or_default(),
                app_id: str_field("appId").or_else(|| str_field("app_id")).unwrap_or_default(),
                x: f64_field("x").unwrap_or(0.0),
                y: f64_field("y").unwrap_or(0.0),
            })),
            "moveMediaNode" => Ok(SpaceCommand::MoveMediaNode(semio_hub_space_commands_authoring::move_media_node::MoveMediaNode { node_id: node_id().unwrap_or_default(), x: f64_field("x").unwrap_or(0.0), y: f64_field("y").unwrap_or(0.0) })),
            "connectMediaPorts" => Ok(SpaceCommand::ConnectMediaPorts(semio_hub_space_commands_media::connect_media_ports::ConnectMediaPorts {
                source_node_id: str_field("sourceNodeId").or_else(|| str_field("source_node_id")).unwrap_or_default(),
                source_port_id: str_field("sourcePortId").or_else(|| str_field("source_port_id")).unwrap_or_default(),
                target_node_id: str_field("targetNodeId").or_else(|| str_field("target_node_id")).unwrap_or_default(),
                target_port_id: str_field("targetPortId").or_else(|| str_field("target_port_id")).unwrap_or_default(),
            })),
            "disconnectMediaEdge" => Ok(SpaceCommand::DisconnectMediaEdge(semio_hub_space_commands_media::disconnect_media_edge::DisconnectMediaEdge { edge_id: str_field("edgeId").or_else(|| str_field("edge_id")).unwrap_or_default() })),
            "removeAppInstance" => Ok(SpaceCommand::RemoveAppInstance(semio_hub_space_commands_authoring::remove_app_instance::RemoveAppInstance { node_id: node_id() })),
            "deleteSelection" => Ok(SpaceCommand::DeleteSelection(semio_hub_space_commands_authoring::delete_selection::DeleteSelection {})),
            "copyAppInstance" => Ok(SpaceCommand::CopyAppInstance(semio_hub_space_commands_authoring::copy_app_instance::CopyAppInstance {})),
            "duplicateAppInstance" => Ok(SpaceCommand::DuplicateAppInstance(semio_hub_space_commands_authoring::duplicate_app_instance::DuplicateAppInstance {})),
            "pasteAppInstance" => Ok(SpaceCommand::PasteAppInstance(semio_hub_space_commands_authoring::paste_app_instance::PasteAppInstance {})),
            "renameAppInstance" => Ok(SpaceCommand::RenameAppInstance(semio_hub_space_commands_authoring::rename_app_instance::RenameAppInstance { label: str_field("label").or_else(|| str_field("name")) })),
            "patchMediaNodes" => {
                let ids = string_vec("nodeIds");
                Ok(SpaceCommand::PatchMediaNodes(semio_hub_space_commands_media::patch_media_nodes::PatchMediaNodes {
                    node_ids: if ids.is_empty() { node_id().map(|id| vec![id]).unwrap_or_default() } else { ids },
                    field: str_field("field").unwrap_or_default(),
                    axis: str_field("axis"),
                    value: json_field("value").unwrap_or_else(|| "null".into()),
                }))
            }
            "patchAppInstances" => {
                let ids = string_vec("nodeIds");
                Ok(SpaceCommand::PatchAppInstances(semio_hub_space_commands_authoring::patch_app_instances::PatchAppInstances {
                    node_ids: if ids.is_empty() { node_id().map(|id| vec![id]).unwrap_or_default() } else { ids },
                    field: str_field("field").unwrap_or_default(),
                    value: json_field("value").unwrap_or_else(|| "null".into()),
                }))
            }
            "bindParameterField" => Ok(SpaceCommand::BindParameterField(semio_hub_space_commands_authoring::bind_parameter_field::BindParameterField {
                node_id: node_id().unwrap_or_default(),
                field_path: str_field("fieldPath").or_else(|| str_field("field_path")).unwrap_or_default(),
                parameter_id: str_field("parameterId").or_else(|| str_field("parameter_id")).unwrap_or_default(),
            })),
            "unbindParameterField" => {
                Ok(SpaceCommand::UnbindParameterField(semio_hub_space_commands_authoring::unbind_parameter_field::UnbindParameterField { node_id: node_id().unwrap_or_default(), field_path: str_field("fieldPath").or_else(|| str_field("field_path")).unwrap_or_default() }))
            }
            "reorganizeWorkflow" => Ok(SpaceCommand::ReorganizeWorkflow(semio_hub_space_commands_authoring::reorganize_workflow::ReorganizeWorkflow {})),
            "workflowEngagementSubmit" => Ok(SpaceCommand::WorkflowEngagementSubmit(semio_hub_space_commands_media::workflow_engagement_submit::WorkflowEngagementSubmit { value: str_field("value") })),
            "compiledDagEngagementSubmit" => Ok(SpaceCommand::CompiledDagEngagementSubmit(semio_hub_space_commands_media::compiled_dag_engagement_submit::CompiledDagEngagementSubmit {})),
            "nodeGraphEdit" => Ok(SpaceCommand::NodeGraphEdit(semio_hub_space_commands_authoring::node_graph_edit::NodeGraphEdit { operations_json: json_field("operations").or_else(|| json_field("operationsJson")).unwrap_or_else(|| "[]".into()) })),
            "setActivePanelTab" => Ok(SpaceCommand::SetActivePanelTab(semio_hub_space_commands_media::set_active_panel_tab::SetActivePanelTab { tab_id: str_field("tabId").or_else(|| str_field("tab_id")).unwrap_or_default() })),
            "nodeGraphViewport" => {
                let value = args.and_then(|value| value.get("viewport")).cloned().ok_or_else(|| Fault::from("nodeGraphViewport requires viewport"))?;
                let viewport = <semio_framework_os_kernel::Viewport2d as semio_framework_value::FromValue>::from_value(value).map_err(|error| Fault::from(format!("invalid nodeGraphViewport viewport: {error}")))?;
                Ok(SpaceCommand::NodeGraphViewport(semio_hub_space_commands_authoring::node_graph_viewport::NodeGraphViewport { viewport }))
            }
            "presenceHeartbeat" => Ok(SpaceCommand::PresenceHeartbeat(semio_hub_space_commands_media::presence_heartbeat::PresenceHeartbeat {})),
            "workflowEngagementInput" => Ok(SpaceCommand::WorkflowEngagementInput(semio_hub_space_commands_media::workflow_engagement_input::WorkflowEngagementInput { value: str_field("value").unwrap_or_default() })),
            "compiledDagEngagementInput" => Ok(SpaceCommand::CompiledDagEngagementInput(semio_hub_space_commands_media::compiled_dag_engagement_input::CompiledDagEngagementInput { value: str_field("value").unwrap_or_default() })),
            "setActiveExample" => Ok(SpaceCommand::SetActiveExample(semio_hub_space_commands_media::set_active_example::SetActiveExample { example_id: str_field("exampleId").or_else(|| str_field("example_id")).unwrap_or_default() })),
            "exportMedia" => Ok(SpaceCommand::ExportMedia(semio_hub_space_commands_media::export_media::ExportMedia { node_id: node_id().unwrap_or_default(), format: str_field("format").unwrap_or_default(), document_json: str_field("documentJson").unwrap_or_default() })),
            "importMedia" => Ok(SpaceCommand::ImportMedia(semio_hub_space_commands_media::import_media::ImportMedia { node_id: node_id().unwrap_or_default(), format: str_field("format").unwrap_or_default() })),
            "importMediaPayload" => Ok(SpaceCommand::ImportMediaPayload(semio_hub_space_commands_media::import_media_payload::ImportMediaPayload { payload: str_field("payload").or_else(|| str_field("dsl")).unwrap_or_default() })),
            "exportStudioPack" => Ok(SpaceCommand::ExportStudioPack(semio_hub_space_commands_media::export_studio_pack::ExportStudioPack {})),
            "exportStudioDsl" => Ok(SpaceCommand::ExportStudioDsl(semio_hub_space_commands_media::export_studio_dsl::ExportStudioDsl {})),
            "importSpacePack" => Ok(SpaceCommand::ImportSpacePack(semio_hub_space_commands_media::import_space_pack::ImportSpacePack {})),
            "importSpacePackPayload" => Ok(SpaceCommand::ImportSpacePackPayload(semio_hub_space_commands_media::import_space_pack_payload::ImportSpacePackPayload { payload: str_field("payload").or_else(|| str_field("dsl")).unwrap_or_default() })),
            "openSpace" => Ok(SpaceCommand::OpenSpace(semio_hub_space_commands_media::open_space::OpenSpace { space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default() })),
            "openInstance" => Ok(SpaceCommand::OpenInstance(semio_hub_space_commands_authoring::open_instance::OpenInstance { node_id: node_id() })),
            "closeFocusedInstance" => Ok(SpaceCommand::CloseFocusedInstance(semio_hub_space_commands_authoring::close_focused_instance::CloseFocusedInstance {})),
            "goHome" => Ok(SpaceCommand::GoHome(semio_hub_space_commands_media::go_home::GoHome {})),
            "navigateVirtualFileSystemNode" => Ok(SpaceCommand::NavigateVirtualFileSystemNode(semio_hub_space_commands_media::navigate_virtual_file_system_node::NavigateVirtualFileSystemNode {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).or_else(|| str_field("nodeId")).or_else(|| str_field("node_id")).unwrap_or_default(),
            })),
            "setAppRegistrations" => Ok(SpaceCommand::SetAppRegistrations(semio_hub_space_commands_media::set_app_registrations::SetAppRegistrations { json: json_field("json").unwrap_or_else(|| "[]".into()) })),
            other => Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.unhandled-action"), format!("space: unhandled action id {other}"))),
        }
    }

    /// 🕹️ `deleteSelection`/`nodeGraphEdit`/`reorganizeWorkflow`/`copyAppInstance`/
    /// `duplicateAppInstance`/`removeAppInstance`/`renameAppInstance`/`openInstance` read the `graph`
    /// interaction domain directly (bypassing the `app_commands!`-generated `dispatch`, whose per-row
    /// `$module::handle(payload, doc, cfg)` signature is framework-fixed and has no `interaction`
    /// slot) — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    async fn handle(
        command: &SpaceCommand,
        doc: &ArtifactView<'_, WorkflowSnapshot>,
        cfg: &ConfigView<'_, SpaceConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<WorkflowMutation, SpaceConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            SpaceCommand::DeleteSelection(payload) => semio_hub_space_commands_authoring::delete_selection::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::NodeGraphEdit(payload) => semio_hub_space_commands_authoring::node_graph_edit::edit(payload, doc).await,
            SpaceCommand::ReorganizeWorkflow(payload) => semio_hub_space_commands_authoring::reorganize_workflow::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::CopyAppInstance(payload) => semio_hub_space_commands_authoring::copy_app_instance::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::DuplicateAppInstance(payload) => semio_hub_space_commands_authoring::duplicate_app_instance::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::RemoveAppInstance(payload) => semio_hub_space_commands_authoring::remove_app_instance::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::RenameAppInstance(payload) => semio_hub_space_commands_authoring::rename_app_instance::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::OpenInstance(payload) => semio_hub_space_commands_authoring::open_instance::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::PresenceHeartbeat(payload) => {
                let identity = view_state
                    .and_then(crate::home_session_identity)
                    .ok_or_else(|| Fault::from("s.space.session-identity-required"))?;
                semio_hub_space_commands_media::presence_heartbeat::handle_with_identity(payload, identity, doc, cfg)
            }
            _ => command.dispatch(doc, cfg),
        }
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology` — every workflow node is registered at both the
    /// "instance" and "media-node" granularities (a node IS the app instance now, see the kernel
    /// `🔁️workflow` crate's `🔖️InstanceIdentity` doc) so `selectAll`/range-selection behave correctly
    /// under either granularity; no real parent/child structure exists in the node graph, so every
    /// node is a root.
    async fn interaction_topology(doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
        let mut ordered = Vec::new();
        for node in &doc.snapshot.graph.nodes {
            ordered.push(TopologyNode { id: node.id.clone(), granularity: "instance".into(), parent: None });
            ordered.push(TopologyNode { id: node.id.clone(), granularity: "media-node".into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(S_PLAY_INTERACTION_DOMAIN.to_string(), DomainTopology { ordered });
        Ok(InteractionTopology { domains })
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let projection = doc.snapshot;
        let config = cfg.snapshot;
        let labels = semio_framework_plugin::resolve_labels::<SStudioLabels>(view_state);
        match body_key {
            semio_hub_space_foundation::S_PLAY_BODY_WORKFLOW => {
                crate::modes::main::windows::workflow::render(&SpaceApp, projection, config).await.map(semio_framework_plugin::built_to_component_tree)
            }
            crate::modes::main::windows::media_vfs::S_PLAY_BODY_MEDIA_VFS => crate::modes::main::windows::media_vfs::render(projection, view_state).await.map(semio_framework_plugin::built_to_component_tree),
            crate::modes::main::windows::compiled_dag::S_PLAY_BODY_COMPILED_DAG => crate::modes::main::windows::compiled_dag::render(projection).await.map(semio_framework_plugin::built_to_component_tree),
            S_PLAY_CATALOGUE_BODY_KEY => crate::panels::catalogue::build_catalogue_tree(labels, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, S_PLAY_CATALOGUE_BODY_KEY))
                .await
                .map(semio_framework_plugin::built_to_component_tree),
            // 🧬️ `parameters`/`inspection` are now ported to the contract `BuiltNode` DSL (SEMANTIC-
            // UI-CONTRACT-AND-RENDERER-FAMILY, 26/08/20) — both `render` fns are U1-sync (the contract
            // builder's own sync-only ruling), so no `.await` here, only the `map` bridge into `ComponentTree`.
            S_PLAY_PARAMETERS_BODY_KEY => crate::panels::parameters::render(projection, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, S_PLAY_PARAMETERS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            // 🕹️ `render` carries no `InteractionView` (ArtifactApp's breaking pass only added it to
            // `handle`/`copy_fragment`/`cut_operations` — see ticket 26/08/14's w3b-summary.md) — the
            // inspector degrades to its "no selection" default until a future wave threads interaction
            // into render. Flagged as a discovered framework gap, not worked around here.
            S_PLAY_INSPECTOR_BODY_KEY => crate::panels::inspection::render(projection, &[], labels, &semio_framework_plugin::TreeWindows::for_body(view_state, S_PLAY_INSPECTOR_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    async fn window_measures(doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<semio_framework_plugin::WindowMeasure>> {
        HashMap::from([(semio_hub_space_foundation::S_PLAY_WINDOW_WORKFLOW.into(), crate::modes::main::windows::workflow::window_measures(cfg.snapshot, &doc.snapshot.graph.nodes, view_state).await)])
    }

    /// 🕹️ `context_menu` carries no `InteractionView` (same gap as `render` — see ticket 26/08/14's
    /// w3b-summary.md), so the selection-dependent rows below always take the "nothing selected"
    /// branch rather than reading a stale/wrong selection.
    async fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &ArtifactView<'_, WorkflowSnapshot>,
        _cfg: &ConfigView<'_, SpaceConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let labels = semio_framework_plugin::resolve_labels::<SStudioLabels>(view_state);
        space_workflow_context_menu_items(registry, labels, view_state, request.surface.as_ref(), &[]).await
    }
}
//#endregion 🔖️SpaceApp
