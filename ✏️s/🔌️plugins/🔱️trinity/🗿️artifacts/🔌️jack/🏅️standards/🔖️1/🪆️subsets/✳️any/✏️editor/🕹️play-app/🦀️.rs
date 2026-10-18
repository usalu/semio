//! 🕹️ Jack play app: the `TrinityJackPlayApp` editor and its `ArtifactEditor` implementation.

use super::*;
use semio_framework_2d_compute::EngineHandles;
use crate::editor::jack::commands;

//#region 🔖️TrinityJackPlayApp
/// 🔱️ Trinity Jack play app — a jack-query editor over a live {@link JackSnapshot} projection.
#[derive(Default)]
pub struct TrinityJackPlayApp<C:crate::standards::v1::subsets::any::schema::composition::JackComposition>(pub(super) std::marker::PhantomData<C>);

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> ArtifactEditor for TrinityJackPlayApp<C> {
    /// 🧩️ The authored composition supplies the actual roster for its selected content behavior.
    type Members = C::Members;
    type Snapshot = JackSnapshot;
    type Mutation = TrinityGraphMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;

    type Command = TrinityJackCommand;

    const DIALECT: Dialect = TRINITY_JACK_DIALECT;
    /// 🧬️ The crate's one loaded-parent child projection (`crate::standards::v1::subsets::any::schema::snapshot::jack_child_restore_projection`).
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        crate::standards::v1::subsets::any::schema::snapshot::jack_child_restore_projection(snapshot)
    }
    const DOCUMENT_SCHEMA: &'static str = TRINITY_GRAPH_SCHEMA;

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
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

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<JackEditorWindowTransientOwner>()?;
        crate::editor::jack::transient::register_results_window(registry)
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        crate::editor::jack::window_config::register(registry)
    }

    fn retained_window_transient_target(command: &Self::Command) -> Option<(&str, &'static str)> {
        match command {
            TrinityJackCommand::RunQuery { results_window_id, .. } | TrinityJackCommand::LoadExampleQuery { results_window_id, .. } => Some((results_window_id, TRINITY_JACK_PLAY_WINDOW_RESULTS)),
            _ => None,
        }
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        const OWNER_FILE: &str = "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs";
        const CONTROLLER: &str = "s.trinity.jack@1/*#editor";
        let config_contract = ToolExecutionContract::bounded_first_step(8_192, 64, 64, 16_384, 7_500);
        let query_contract = ToolExecutionContract::resumable(16_384, 64, 1, 1_048_576, 7_500, 1, 1);
        let mut proofs = Vec::new();
        for tool in JACK_RETAINED_WINDOW_CONFIG_TOOL_IDS {
            proofs.push(
                semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Self>>(OWNER_FILE, CONTROLLER, "JackRetainedWindowConfigJobFactory", tool, TRINITY_GRAPH_SCHEMA, config_contract)
                    .with_factory_type::<EditorApp<Self>, JackRetainedWindowConfigJobFactory<C>>(),
            );
        }
        for tool in JACK_RETAINED_DOCUMENT_TOOL_IDS {
            proofs.push(
                semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Self>>(OWNER_FILE, CONTROLLER, "JackRetainedDocumentJobFactory", tool, TRINITY_GRAPH_SCHEMA, ToolExecutionContract::bounded_first_step(32_768, 64, 64, 262_144, 7_500))
                    .with_factory_type::<EditorApp<Self>, JackRetainedDocumentJobFactory<C>>(),
            );
        }
        proofs.push(
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Self>>(OWNER_FILE, CONTROLLER, "JackRetainedTransientJobFactory", "textSelect", TRINITY_GRAPH_SCHEMA, config_contract)
                .with_factory_type::<EditorApp<Self>, JackRetainedTransientJobFactory<C>>(),
        );
        for tool in commands::query::job::JACK_QUERY_TOOL_IDS {
            proofs.push(
                semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Self>>(OWNER_FILE, CONTROLLER, "JackQueryJobFactory", tool, TRINITY_GRAPH_SCHEMA, query_contract)
                    .with_factory_type::<EditorApp<Self>, commands::query::job::JackQueryJobFactory<C>>(),
            );
        }
        proofs
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(JackRetainedWindowConfigJobFactory::<C>::new(&controller))?;
        registry.register(JackRetainedTransientJobFactory::<C>::new(&controller))?;
        registry.register(JackRetainedDocumentJobFactory::<C>::new(&controller))?;
        registry.register(commands::query::job::JackQueryJobFactory::<C>::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if commands::query::job::JACK_QUERY_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return commands::query::job::build_job::<C>(request).map(Some);
        }
        if JACK_RETAINED_TRANSIENT_TOOL_IDS.contains(&request.tool_id.as_str()) {
            if Self::command_id(&request.command) != request.tool_id {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "Jack command does not match its exact registered tool"));
            }
            let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(JackEditorWindowTransientCommandWork { consumed: false });
            let operation = AppOperationContext {
                app_instance_id: request.app_instance_id,
                parent_document_id: request.parent_document_id.clone(),
                operation_id: request.operation.operation.0,
                generation: request.operation.generation.0,
                canonical_base_revision: request.canonical_base_revision,
                retained: request.retained,
                authoring_seed: request.authoring_seed.clone(),
            };
            let payload = ArtifactRetainedCommandPayload::new(
                semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                    command: *request.command,
                    snapshot: request.snapshot,
                    snapshot_read: request.snapshot_read,
                    config: request.config,
                    history: request.history,
                    interaction_state: request.interaction_state,
                    interaction_hover: request.interaction_hover,
                    context: Some(request.context),
                    operation,
                    completion: request.completion,
                },
                TrinityJackPlayApp::<C>::command_id,
                JACK_RETAINED_RAW_BYTES,
                JACK_RETAINED_WORK_ITEMS,
                work,
            );
            return Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)));
        }
        let tool_id = Self::command_id(&request.command);
        let (work, raw_bytes): (Box<dyn ArtifactCommandWork<EditorApp<Self>>>, usize) = if JACK_RETAINED_DOCUMENT_TOOL_IDS.contains(&request.tool_id.as_str()) {
            if tool_id != request.tool_id {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "Jack command does not match its exact registered tool"));
            }
            if jack_retained_document_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("trinity.jack.retained-capacity"), "the jack command exceeds the capacity of one bounded edit"));
            }
            (Box::new(BoundedArtifactCommandWork::new(tool_id, jack_retained_document_reduce, jack_retained_document_extent)), JACK_RETAINED_DOCUMENT_RAW_BYTES)
        } else if JACK_RETAINED_WINDOW_CONFIG_TOOL_IDS.contains(&request.tool_id.as_str()) {
            if tool_id != request.tool_id {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "Jack command does not match its exact registered tool"));
            }
            if jack_retained_window_config_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("trinity.jack.retained-capacity"), "the jack command exceeds the capacity of one bounded edit"));
            }
            (Box::new(BoundedArtifactCommandWork::new(tool_id, jack_retained_window_config_reduce, jack_retained_window_config_extent)), JACK_RETAINED_RAW_BYTES)
        } else {
            return Ok(None);
        };
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            TrinityJackPlayApp::<C>::command_id,
            raw_bytes,
            JACK_RETAINED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::standards::v1::subsets::any::schema::host::jack_envelope_decode_owner_bundle())
    }

    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::standards::v1::subsets::any::schema::host::jack_document_store_initialization_job(envelope, operation, generation, actor))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    /// ⏯️ Builds the reorganize tool's layout run over the run's base (`ToolRunDefinition.runJob`); resumed from the
    /// ledger's checkpoint and provisional moves on a settings change.
    fn build_tool_run_job(request: semio_framework_plugin::ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolRunJob>, Fault> {
        if request.tool_id != edit::tools::reorganize::TOOL_ID || request.purpose != semio_framework_plugin::ToolRunJobPurpose::Run {
            return Ok(None);
        }
        let content = C::bind(&request.snapshot, &request.children)?;
        edit::tools::reorganize::build_job(request.identity, &content, request.checkpoint, request.member_ops).map(Some)
    }

    /// 📢️ The localized notices of the editor's own refusal codes (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        crate::standards::v1::subsets::any::schema::content::jack_fault_notices()
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
        C::genesis_child_pack(snapshot, slot, child_id)
    }

    fn initial_snapshot() -> JackSnapshot {
        C::initial_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(jack_io())
    }

    // 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetFixture` — see
    // `📓️taxonomy.md`'s forbidden vocabulary), so this intentionally falls back to the trait
    // default (`None`) rather than overriding — the `"artifact:in"` media port therefore reports
    // `MediaError::NotImplemented`; a real whole-snapshot load goes through `reset_document_effect`
    // (`Effect::LoadDocument`, outside undo history), see `editor::jack::commands::set_active_example`
    // and `editor::jack::commands::load_document_json`.

    /// 🔌️ `"graph:out"` fans the live query-graph projection out to other graph-consuming workflow
    /// nodes, in addition to the implicit `"artifact:out"` — both encode the same `JackSnapshot` pack.
    fn export_media(port: &str, doc: &ArtifactView<'_, JackSnapshot>) -> Result<Media, MediaError> {
        match port {
            "graph:out" | "artifact:out" => {
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity }, payload: MediaPayload::Structured { schema: TRINITY_GRAPH_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ Maps each `TrinityJackCommand` variant back to the action id it was declared under in
    /// `create_trinity_jack_app`.
    /// 🎯️ Host-action bridge into the closed `TrinityJackCommand` enum — see `args_bridge`.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_plugin::DslValue>, original_fault:&mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault)=original_fault.take(){return Err(fault)}
        args_bridge::command_from_action(action, args.as_ref())
    }

    fn command_id(command: &TrinityJackCommand) -> &'static str {
        match command {
            TrinityJackCommand::LoadDocumentJson { .. } => "loadDocumentJson",
            TrinityJackCommand::DeleteSelection => "deleteSelection",
            TrinityJackCommand::PatchNodes { .. } => "patchNodes",
            TrinityJackCommand::RunQuery { .. } => "runQuery",
            TrinityJackCommand::LoadExampleQuery { .. } => "loadExampleQuery",
            TrinityJackCommand::SetActiveExample { .. } => "setActiveExample",
            TrinityJackCommand::SetViewport { .. } => "nodeGraphViewport",
            TrinityJackCommand::TextEdit { .. } => "textEdit",
            TrinityJackCommand::TextSelect { .. } => "textSelect",
            TrinityJackCommand::FormatDocument => "formatDocument",
            TrinityJackCommand::SetLodMode { .. } => "setLodMode",
        }
    }

    fn handle(
        command: &TrinityJackCommand,
        doc: &ArtifactView<'_, JackSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<TrinityGraphMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        let snapshot = doc.snapshot;
        Ok(match command {
            TrinityJackCommand::LoadDocumentJson { json } => commands::load_document_json::load_document_json(json),
            TrinityJackCommand::DeleteSelection => commands::delete_selection::delete_selection::<C>(snapshot, &doc.children, &interaction.selection("ast").ids)?,
            TrinityJackCommand::PatchNodes { node_ids, field, value } => commands::patch_nodes::patch_nodes::<C>(snapshot, &doc.children, node_ids, &interaction.selection("ast").ids, field, value)?,
            TrinityJackCommand::RunQuery { .. } | TrinityJackCommand::LoadExampleQuery { .. } => return Err(Fault::from("query execution requires its retained operation owner")),
            TrinityJackCommand::SetActiveExample { example_id } => commands::set_active_example::set_active_example::<C>(example_id),
            TrinityJackCommand::SetViewport { viewport, .. } => return commands::set_viewport::set_viewport(viewport, view_state),
            TrinityJackCommand::TextEdit { text } => commands::text_edit::text_edit(text)?,
            TrinityJackCommand::TextSelect { .. } => return Err(Fault::from("text selection requires its retained transient operation owner")),
            TrinityJackCommand::FormatDocument => commands::format_document::format_document(&snapshot.query),
            TrinityJackCommand::SetLodMode { value } => return commands::set_lod_mode::set_lod_mode(value, view_state),
        })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, JackSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let snapshot = doc.snapshot;
        let labels = semio_framework_plugin::resolve_labels::<crate::editor::jack::terminology::TrinityJackLabels>(view_state);
        let content = || C::bind(snapshot, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new("trinity.jack.content", fault.message));
        let root = match body_key {
            TRINITY_JACK_PLAY_BODY_GRAPH => edit::windows::graph::render(TRINITY_JACK_PLAY_SURFACE_GRAPH, TRINITY_JACK_PLAY_CONTROLLER_ID, snapshot, &content()?, crate::editor::jack::window_config::current(cfg)),
            TRINITY_JACK_PLAY_BODY_EDITOR => edit::windows::editor::render(TRINITY_JACK_PLAY_SURFACE_EDITOR, TRINITY_JACK_PLAY_CONTROLLER_ID, snapshot, &graph_from_document::<C>(snapshot,&doc.children)?, None),
            TRINITY_JACK_PLAY_BODY_RESULTS => edit::windows::results::render(TRINITY_JACK_PLAY_SURFACE_RESULTS, TRINITY_JACK_PLAY_CONTROLLER_ID, None, None),
            TRINITY_JACK_PLAY_BODY_ARTIFACT => crate::editor::jack::panels::document::render(&content()?, cfg.snapshot, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, TRINITY_JACK_PLAY_BODY_ARTIFACT)),
            TRINITY_JACK_PLAY_BODY_CATALOGUE => crate::editor::jack::panels::catalogue::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, TRINITY_JACK_PLAY_BODY_CATALOGUE)),
            TRINITY_JACK_PLAY_BODY_INSPECTION => crate::editor::jack::panels::inspection::render(),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("trinity.body.label", "the fixed Trinity body label exceeds its UI bound")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(root))
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, JackSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        if body_key == TRINITY_JACK_PLAY_BODY_RESULTS {
            let result = transient.window::<JackResultsWindowTransientOwner>();
            let root = edit::windows::results::render(TRINITY_JACK_PLAY_SURFACE_RESULTS, TRINITY_JACK_PLAY_CONTROLLER_ID, result.and_then(|state| state.result.as_ref()), result.and_then(|state| state.query_error.as_deref()))?;
            return Ok(semio_framework_plugin::built_to_component_tree(root));
        }
        if body_key == TRINITY_JACK_PLAY_BODY_EDITOR {
            let selection = transient.window::<JackEditorWindowTransientOwner>().and_then(|window| window.selection.as_ref());
            let root = edit::windows::editor::render(TRINITY_JACK_PLAY_SURFACE_EDITOR, TRINITY_JACK_PLAY_CONTROLLER_ID, doc.snapshot, &graph_from_document::<C>(doc.snapshot,&doc.children)?, selection)?;
            return Ok(semio_framework_plugin::built_to_component_tree(root));
        }
        Self::render(body_key, doc, cfg, view_state)

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_measures(_doc: &ArtifactView<'_, JackSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let Some(window) = crate::editor::jack::window_config::current(cfg) else { return HashMap::new() };
        HashMap::from([(window_id.to_string(), vec![edit::windows::graph::trinity_lod_measure(window_id, &window.lod_mode, jack_window_action)])])
    }

    fn context_menu(request: &ContextMenuRequest, _doc: &ArtifactView<'_, JackSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        // 🕹️ Selection is framework-owned now (domain "ast") — `context_menu` has no `InteractionView`,
        // so the request's own surface-carried selection groups are the only source; no config fallback.
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &[], &[]);
        let menu = Menu::of(registry, view_state).action("runQuery").action("formatDocument").group("mode", |m| m.action("setActiveExample")).group("open", |m| m.action("loadExampleQuery"));
        // 🩹️ `Direct`, not `ViaNodeGraphEdit`: jack's own `TrinityJackCommand::DeleteSelection` is a
        // real standalone command (no `nodeGraphEdit`-style JSON-operations envelope exists for jack),
        // so the context-menu row must dispatch the `deleteSelection` action id directly.
        menu.item(node_graph_delete_selection_spec(semio_framework_plugin::delete_selection().resolve(view_state.terminology, view_state.locale), view_state, &nodes, &edges, NodeGraphDeleteDispatch::Direct)).build()
    }

    /// 🕹️ Domain "ast" topology: every snapshot node is a `TopologyNode`, parented by the source node
    /// of its first incoming connection (roots — nodes with no incoming edge — get `parent: None`).
    /// `MergeMode::Range` is not declared for this domain, so `ordered`'s sequence need not be a strict
    /// pre-order — `descendant_closure`/`ancestors` only need the (id, parent) pairs, not list order.
    fn interaction_topology(doc: &ArtifactView<'_, JackSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 let raw=C::bind(doc.snapshot,&doc.children).map_err(|fault|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,fault.message))?;
 let mut parent_of=std::collections::BTreeMap::new();
 for index in 0..semio_s_artifact_trinity_jack_graph_query::JackGraphQuery::edge_count(&raw){if let Some(edge)=semio_s_artifact_trinity_jack_graph_query::JackGraphQuery::edge(&raw,index){parent_of.entry(edge.target.to_string()).or_insert_with(||edge.source.to_string());}}
 let ordered=(0..semio_s_artifact_trinity_jack_graph_query::JackGraphQuery::node_count(&raw)).filter_map(|index|semio_s_artifact_trinity_jack_graph_query::JackGraphQuery::node_id(&raw,index)).map(|id|TopologyNode{id:id.to_string(),granularity:"node".into(),parent:parent_of.get(id).cloned()}).collect();
 let mut domains=std::collections::BTreeMap::new();domains.insert("ast".into(),DomainTopology{ordered});Ok(InteractionTopology{domains})
 }
}
//#endregion 🔖️TrinityJackPlayApp
