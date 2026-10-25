//! 🕹️ Rewriting play app: the `TrinityRewritingPlayApp` editor and its `ArtifactEditor` implementation.

use super::*;
use semio_framework_2d_compute::EngineHandles;
use crate::editor::rewriting::window_config;
use crate::editor::rewriting::modes::edit;
use semio_s_artifact_trinity_jack_semio_content::SemioJackContentOwner;

//#region 🔖️TrinityRewritingPlayApp
/// ♻️ Trinity Rewriting play app — a parametric-rewriting editor over a {@link RewritingSnapshot} projection.
#[derive(Default)]
pub struct TrinityRewritingPlayApp;

impl ArtifactEditor for TrinityRewritingPlayApp {
    /// 🧩️ The roster the composed `s.stdio.semio` `workingGraph` child opens through (design §20.15).
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = RewritingSnapshot;
    type Mutation = RewriteRuleMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = TrinityRewritingCommand;

    const DIALECT: Dialect = TRINITY_REWRITING_DIALECT;
    /// 🧬️ The crate's one loaded-parent child projection (`crate::standards::v1::subsets::any::schema::snapshot::rewriting_child_restore_projection`).
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        crate::standards::v1::subsets::any::schema::snapshot::rewriting_child_restore_projection(snapshot)
    }
    const DOCUMENT_SCHEMA: &'static str = REWRITE_RULE_SCHEMA;

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
    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let documents = REWRITING_DOCUMENT_TOOL_IDS.iter().map(|tool| {
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Self>>(
                "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
                "s.trinity.rewriting@1/*#editor",
                "RewritingDocumentJobFactory",
                tool,
                REWRITE_RULE_SCHEMA,
                semio_framework::ToolExecutionContract::bounded_first_step(32_768, 64, 1, 262_144, 7_500),
            )
            .with_factory_type::<EditorApp<Self>, RewritingDocumentJobFactory>()
        });
        window_config::job::TOOL_IDS
            .iter()
            .map(|tool| {
                semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Self>>(
                    "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧵️job/🦀️.rs",
                    "s.trinity.rewriting@1/*#editor",
                    "RewritingWindowConfigJobFactory",
                    tool,
                    REWRITE_RULE_SCHEMA,
                    window_config::job::contract(),
                )
                .with_factory_type::<EditorApp<Self>, window_config::job::RewritingWindowConfigJobFactory>()
            })
            .chain(documents)
            .collect()
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(window_config::job::RewritingWindowConfigJobFactory::new(&controller))?;
        registry.register(RewritingDocumentJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<semio_framework_plugin::app::ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if REWRITING_DOCUMENT_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return rewriting_build_document_tool_job(request, output, cx);
        }
        window_config::job::build_job(request, output, cx)
    }

    /// 🧾️ Store publication authority for the `Artifact` lane — without it the host refuses every document
    /// verb at dispatch (`declares the unsupported artifact publication lane`).
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    /// 🎯️ Host-action bridge into the closed `TrinityRewritingCommand` enum — see `args_bridge`.
    fn command_from_action(action: &str, args: Option<&semio_framework_plugin::DslValue>) -> Result<Self::Command, Fault> {
        args_bridge::command_from_action(action, args)
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        window_config::register(registry)
    }

    fn initial_snapshot() -> RewritingSnapshot {
        default_rule_state()
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {
        crate::standards::v1::subsets::any::content::genesis_working_child_pack(snapshot, slot, child_id)
    }

    /// 📢️ The localized notices of the editor's own refusal codes (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        crate::standards::v1::subsets::any::content::rewriting_fault_notices()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(rewriting_io())
    }

    /// 🔌️ `"graph:in"` loads an incoming `trinity.graph` pack as this rule's `working_graph`
    /// working graph — a single targeted field edit, not a whole-document replace.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, RewritingSnapshot>) -> Result<Emit<RewriteRuleMutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "graph:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "graph:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let snapshot = <JackSnapshot as ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let _ = doc;
                Ok(Emit::mutations(vec![schema::mutations::edit_working_graph(snapshot)]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🔌️ `"graph:out"` re-emits the rule-applied result graph, alongside the implicit `"artifact:out"`.
    fn export_media(port: &str, doc: &ArtifactView<'_, RewritingSnapshot>) -> Result<Media, MediaError> {
        match port {
            "graph:out" => {
                let state = crate::standards::v1::subsets::any::content::composed(doc.snapshot, &doc.children).map_err(|fault| MediaError::Payload(port.to_string(), fault.message))?;
                let snapshot = rewritten_graph(&state).map_err(|error| MediaError::Payload(port.to_string(), error))?;
                let bytes = ArtifactPack::encode_pack(&snapshot);
                Ok(Media {
                    media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity },
                    payload: MediaPayload::Structured { schema: semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::graph::TRINITY_GRAPH_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ Maps each `TrinityRewritingCommand` variant back to the action id it was declared under in
    /// `create_rewriting_app`.
    fn command_id(command: &TrinityRewritingCommand) -> &'static str {
        match command {
            TrinityRewritingCommand::NodeGraphEdit { .. } => "nodeGraphEdit",
            TrinityRewritingCommand::SetLhsJson { .. } => "setLhsJson",
            TrinityRewritingCommand::SetRhsJson { .. } => "setRhsJson",
            TrinityRewritingCommand::SetParameter { .. } => "setParameter",
            TrinityRewritingCommand::AddRuleClause { .. } => "addRuleClause",
            TrinityRewritingCommand::ResetRule => "resetRule",
            TrinityRewritingCommand::SetActiveExample { .. } => "setActiveExample",
            TrinityRewritingCommand::PatchNodes { .. } => "patchNodes",
            TrinityRewritingCommand::SetViewport { .. } => "nodeGraphViewport",
            TrinityRewritingCommand::Reorganize => "reorganize",
            TrinityRewritingCommand::SetLodMode { .. } => "setLodMode",
            TrinityRewritingCommand::AddWorkingNode { .. } => "addWorkingNode",
        }
    }

    fn handle(
        command: &TrinityRewritingCommand,
        doc: &ArtifactView<'_, RewritingSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<RewriteRuleMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        let state = doc.snapshot;
        Ok(match command {
            TrinityRewritingCommand::NodeGraphEdit { surface_id, operations_json } => crate::editor::rewriting::commands::node_graph_edit::node_graph_edit(state, &doc.children, surface_id, operations_json, "")?,
            TrinityRewritingCommand::SetLhsJson { value } => crate::editor::rewriting::commands::set_lhs::set_lhs(state, value)?,
            TrinityRewritingCommand::SetRhsJson { value } => crate::editor::rewriting::commands::set_rhs::set_rhs(state, value)?,
            TrinityRewritingCommand::SetParameter { name, value } => crate::editor::rewriting::commands::set_parameter::set_parameter(state, name, value),
            TrinityRewritingCommand::AddRuleClause { kind } => crate::editor::rewriting::commands::add_rule_clause_command::add_rule_clause_command(state, kind)?,
            TrinityRewritingCommand::ResetRule => crate::editor::rewriting::commands::reset_rule::reset_rule(state),
            TrinityRewritingCommand::SetActiveExample { example_id } => crate::editor::rewriting::commands::set_active_example::set_active_example(example_id),
            TrinityRewritingCommand::PatchNodes { node_ids, field, value } => crate::editor::rewriting::commands::patch_nodes::patch_nodes(state, &doc.children, node_ids, &interaction.selection("graph").ids, field, value)?,
            TrinityRewritingCommand::SetViewport { surface_id, viewport } => crate::editor::rewriting::commands::set_viewport::set_viewport(surface_id, viewport, view_state)?,
            TrinityRewritingCommand::Reorganize => crate::editor::rewriting::commands::reorganize::reorganize(state),
            TrinityRewritingCommand::SetLodMode { value } => crate::editor::rewriting::commands::set_lod_mode::set_lod_mode(value, view_state)?,
            TrinityRewritingCommand::AddWorkingNode { kind, name, x, y } => crate::editor::rewriting::commands::add_working_node_command::add_working_node_command(state, &doc.children, kind.as_deref(), name.as_deref(), *x, *y)?,
        })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, RewritingSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let state = doc.snapshot;
        let config = cfg.snapshot;
        let window_config = window_config::current(cfg).cloned().unwrap_or_default();
        let labels = semio_framework_plugin::resolve_labels::<crate::editor::rewriting::terminology::TrinityRewritingLabels>(view_state);
        let composed = || crate::standards::v1::subsets::any::content::composed(state, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new("trinity.rewriting.working-graph", fault.message));
        let root = match body_key {
            TRINITY_REWRITING_PLAY_BODY_BEFORE => edit::windows::before::render(&composed()?, &window_config),
            TRINITY_REWRITING_PLAY_BODY_AFTER => edit::windows::after::render(&composed()?, &window_config),
            TRINITY_REWRITING_PLAY_BODY_LHS => edit::windows::lhs::render(state, &window_config),
            TRINITY_REWRITING_PLAY_BODY_RHS => edit::windows::rhs::render(state, &window_config),
            TRINITY_REWRITING_PLAY_BODY_JACK => edit::windows::jack::render(state, config),
            TRINITY_REWRITING_PLAY_BODY_PARAMETERS => edit::windows::parameters::render(state, labels),
            TRINITY_REWRITING_PLAY_BODY_ARTIFACT => crate::editor::rewriting::panels::document::render(&composed()?, config, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, TRINITY_REWRITING_PLAY_BODY_ARTIFACT)),
            TRINITY_REWRITING_PLAY_BODY_CATALOGUE => crate::editor::rewriting::panels::catalogue::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, TRINITY_REWRITING_PLAY_BODY_CATALOGUE)),
            TRINITY_REWRITING_PLAY_BODY_INSPECTION => crate::editor::rewriting::panels::inspection::render(),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("trinity.body.label", "the fixed Trinity body label exceeds its UI bound")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(root))
    }

    fn window_measures(_doc: &ArtifactView<'_, RewritingSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let Some(config) = window_config::current(cfg) else { return HashMap::new() };
        HashMap::from([(window_id.to_string(), vec![trinity_rewriting_lod_measure(window_id, &config.lod_mode)])])
    }

    fn context_menu(request: &ContextMenuRequest, _doc: &ArtifactView<'_, RewritingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &[], &[]);

        let menu = Menu::of(registry, view_state)
            .action("addRuleClause")
            .action("setParameter")
            .action("reorganize")
            .group("create", |m| m.action("addWorkingNode"))
            .group("transform", |m| m.action("patchNodes"))
            .group("history", |m| m.action("resetRule"))
            .group("mode", |m| m.action("setLodMode").action("setActiveExample"))
            .group("tools", |m| m.action("setLhsJson").action("setRhsJson"));
        menu.item(node_graph_delete_selection_spec(semio_framework_plugin::delete_selection().resolve(view_state.terminology, view_state.locale), view_state, &nodes, &edges, NodeGraphDeleteDispatch::ViaNodeGraphEdit)).build()
    }

    /// 🕹️ Domain "graph" topology: unions three node universes under one "node" granularity —
    /// (1) the Before snapshot's own nodes, parented by the source node of their first incoming
    /// connection, each with a variable-reference child when its name resolves one (`var_from_node_name`
    /// — "AST parents + variable references"); (2) the LHS semantic graph (`lhs-where` parented by
    /// `lhs-match` via their one edge); (3) the RHS semantic graph (its clause nodes have no inherent
    /// parent order, so they're roots). `MergeMode::Range` is not declared for this domain, so
    /// `ordered`'s sequence need not be a strict pre-order.
    fn interaction_topology(doc: &ArtifactView<'_, RewritingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 let composed=crate::standards::v1::subsets::any::content::composed(doc.snapshot,&doc.children).map_err(|fault|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,fault.message))?;let state=&composed;
 semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::inferences::topology::compute_topology(&state.working_graph)?;
 let owner=semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::content::jack_content_for_handle(&state.working_graph.content)?;let raw=owner.snapshot();let mut ordered=Vec::new();let mut parent_of=BTreeMap::new();
 for edge in &raw.edges{parent_of.entry(edge.target.value.clone()).or_insert_with(||edge.source.value.clone());}
 for node in &raw.nodes{ordered.push(TopologyNode{id:node.id.value.clone(),granularity:"node".into(),parent:parent_of.get(&node.id.value).cloned()});if let Some(var)=var_from_node_name(&node.label){ordered.push(TopologyNode{id:var,granularity:"node".into(),parent:Some(node.id.value.clone())});}}
 let lhs=lhs_semantic_graph_snapshot(&state.lhs,&state.rule_layout);let lhs_owner=semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::content::jack_content_for_handle(&lhs.content)?;let lhs_raw=lhs_owner.snapshot();let mut parent_of=BTreeMap::new();for edge in &lhs_raw.edges{parent_of.entry(edge.target.value.clone()).or_insert_with(||edge.source.value.clone());}
 for node in &lhs_raw.nodes{ordered.push(TopologyNode{id:node.id.value.clone(),granularity:"node".into(),parent:parent_of.get(&node.id.value).cloned()});}
 let rhs=rhs_semantic_graph_snapshot(&state.rhs,&state.rule_layout);let rhs_owner=semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::content::jack_content_for_handle(&rhs.content)?;for node in &rhs_owner.snapshot().nodes{ordered.push(TopologyNode{id:node.id.value.clone(),granularity:"node".into(),parent:None});}
 let mut domains=BTreeMap::new();domains.insert("graph".into(),DomainTopology{ordered});Ok(InteractionTopology{domains})
 }
}
//#endregion 🔖️TrinityRewritingPlayApp
