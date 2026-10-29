//! 🧠️ Artifact editor: the `ArtifactEditor` implementation of the puzzle2d play app and its remaining operations. See [`super`].

use super::*;
use semio_s_artifact_puzzle_2d_snapshot::editor::puzzle2d::window;
use semio_s_artifact_puzzle_2d_mut_any_10::Puzzle2dPlaySnapshot;
use semio_s_artifact_puzzle_2d_snapshot::editor::puzzle2d::window::Puzzle2dWindowTransient;
use crate::mirror::editor::puzzle2d::modes::edit::windows::overview;
use crate::mirror::editor::puzzle2d::modes::edit::windows::detail;
use crate::mirror::editor::puzzle2d::modes::edit::windows::selection;
use std::collections::HashMap;
use crate::Puzzle2dMutation;
use semio_framework_plugin::NoPresence as Puzzle2dPresence;
use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::presence::Puzzle2dPresenceMutation;
use semio_framework_2d_compute::EngineHandles;
use semio_framework::kernel::ClipboardFragment;
use semio_framework::kernel::ClipboardError;
use semio_framework::kernel::PastePlacement;

//#region 🔖️PlayApp
//#region 🧵️ReservedJobs
impl ArtifactEditor for Puzzle2dPlayApp {
    const DIALECT: Dialect = crate::standards::v1::subsets::any::schema::records::PUZZLE2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE2D_BOARD_SNAPSHOT_SCHEMA;
    type Snapshot = Puzzle2dPlaySnapshot;
    type Mutation = Puzzle2dMutation;
    type Config = Puzzle2dConfig;
    type ConfigMutation = Puzzle2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = Puzzle2dPresence;
    type PresenceMutation = Puzzle2dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = Puzzle2dCommand;

    /// 🐁️ The six framework interaction verbs answered out of this app's OWN narrow scopes instead of
    /// the framework's blanket [`UiDirtyScope::Full`]. A hover fires on every pointermove, so it
    /// repaints the three canvas panes alone — never the panels, the engagement bar, the measures or
    /// the labels; a pick additionally repaints the outliner/inspector rows that highlight it. A verb
    /// that touched a domain this app does not declare answers `None` and keeps the framework's widest scope.
    fn interaction_scope(verb: InteractionVerb, domains: &[&str]) -> Option<UiDirtyScope> {
        if domains.is_empty() || domains.iter().any(|domain| *domain != PUZZLE2D_INTERACTION_DOMAIN) {
            return None;
        }
        Some(match verb {
            InteractionVerb::Hover => puzzle2d_window_only_scope(),
            InteractionVerb::Select | InteractionVerb::ClearSelection | InteractionVerb::SelectAll => puzzle2d_select_scope(),
            InteractionVerb::SetSelectionMode | InteractionVerb::SetGranularity => puzzle2d_window_and_engagements_scope(),
        })
    }

    /// 🗺️ The `vortex` domain's whole pickable universe, in board order: every node with its handles under it, every
    /// edge and every target region (a region answers to the node granularity, as `puzzle2d_selection_targets`
    /// classifies it) — what `selectAll` enumerates and what keeps a selection inside the document. A host fact never
    /// reaches this editor: the window's gesture slot ends an open gesture on a blur, a lost capture, a utility switch,
    /// a closing window, a frozen document and a moved base.
    fn interaction_topology(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle2dConfig>) -> Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError> {
        let document = doc.snapshot.typed();
        let mut ordered = Vec::new();
        for node in &document.nodes {
            ordered.push(semio_framework_plugin::TopologyNode { id: node.id.to_string_owner(), granularity: PUZZLE2D_GRANULARITY_NODE.into(), parent: None });
            ordered.extend(node.handles.iter().map(|handle| semio_framework_plugin::TopologyNode { id: handle.id.to_string_owner(), granularity: PUZZLE2D_GRANULARITY_HANDLE.into(), parent: Some(node.id.to_string_owner()) }));
        }
        ordered.extend(document.edges.iter().map(|edge| semio_framework_plugin::TopologyNode { id: edge.id.to_string_owner(), granularity: PUZZLE2D_GRANULARITY_EDGE.into(), parent: None }));
        ordered.extend(document.target_regions.iter().map(|region| semio_framework_plugin::TopologyNode { id: region.id.to_string_owner(), granularity: PUZZLE2D_GRANULARITY_NODE.into(), parent: None }));
        Ok(semio_framework_plugin::InteractionTopology { domains: std::collections::BTreeMap::from([(PUZZLE2D_INTERACTION_DOMAIN.to_string(), semio_framework_plugin::DomainTopology { ordered })]) })
    }

    /// 🪧️ A history-edit reference chip names its board entity as the outliner does ([`puzzle2d_entity_label`]).
    fn entity_label(snapshot: &Puzzle2dPlaySnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        puzzle2d_entity_label(snapshot.value(), kinds, id)
    }



    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Puzzle2dConfigStorePreparationFactory))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Puzzle2dArtifactStorePreparationFactory))
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

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    /// 👥️ Canonical window views own camera publication, leaving the app presence root empty.
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |_| true).expect("default Puzzle2d presence is the exact empty terminal")))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        window::register_config(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        window::register_transient(registry)
    }

    /// 📎 Ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d: replaces the old
    /// `crate::editor::puzzle2d::config::schema::register_app_schema()` self-registering call, which
    /// puzzle's plugin root used to reach `.setup()` for — `register_document_app`/`document_app`
    /// now call this automatically the moment `Puzzle2dPlayApp` is bound to a plugin, exactly like
    /// `🗒️note`'s own `app_schema` override.
    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::puzzle2d::config::schema::app_schema_descriptor())
    }

    /// 🚀️ The empty snapshot, nothing else. The example documents are `LazyLock`s the example load forces
    /// when it is actually switched to: warming them here parsed the Concrete Forest and Nakagin documents on
    /// every construction of this app — every mount, and every `codec` call of a hub creating or validating a
    /// document, where the interpreter paid for two documents the snapshot never reads.
    fn reference_birth()->Option<fn()->Result<Puzzle2dPlaySnapshot,semio_framework_plugin::Fault>>{Some(||{
        Ok(Puzzle2dPlaySnapshot::new(crate::standards::v1::subsets::any::schema::empty_puzzle2d_snapshot()))
    })}

    /// 🏷️ Maps each `Puzzle2dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle2dCommand) -> &'static str {
        command.action_id()
    }

    /// 📢️ The localized notices of this guest's refusal codes ([`puzzle2d_fault_notices`]).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        puzzle2d_fault_notices()
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() {
            return Err(fault);
        }
        let args = args.as_ref();
        let window_id = args.and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
        if let Some(flag) = puzzle2d_flag_value_argument(action) {
            args.and_then(|value| value.get(flag)).and_then(semio_framework_value::DslValue::as_bool).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("puzzle2d.action.flag-value-required"), format!("action '{action}' requires the boolean '{flag}' it sets")))?;
        }
        let args = args.map(semio_framework_pack_json::from_dsl_value);
        Puzzle2dCommand::try_from_action(action, args, window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 2D action '{action}'")))
    }

    /// 🎬️ Routes the example load through its own `Work` before document materialization; every other
    /// command runs the one shared [`puzzle2d_dispatch_emit`] pipeline the retained generic reduce runs.
    fn handle(
        command: &Puzzle2dCommand,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation, Self::DraftMutation>, Fault> {
        let config = cfg.snapshot;
        let (action, _args) = (command.action_id(), command.args());
        if action == "setActiveExample" {
            return puzzle2d_active_example_emit(command, doc.snapshot, config);
        }
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_transient = Puzzle2dWindowTransient::default();
        let window_kind = view_state.and_then(window::kind_for_view).unwrap_or(semio_s_artifact_puzzle_2d_inferences::viewer::puzzle2d::modes::view::windows::board::WINDOW_KIND_ID);
        let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
        let revision = doc.operation_optional().map(|operation| puzzle2d_revision_hex(&operation.canonical_base_revision));
        let base_revision = revision.as_ref().map_or("", |hex| std::str::from_utf8(hex).expect("lowercase hexadecimal is ASCII"));
        puzzle2d_dispatch_emit::<Self>(command, doc.snapshot, config, &window_config, &window_transient, window_kind, view_state, puzzle2d_active_utility(view_state), interaction.selection(PUZZLE2D_INTERACTION_DOMAIN), authoring_seed, base_revision, &semio_framework_plugin::app::GestureSlot::detached(), doc.operation_optional().cloned())
            .map(|(emit, _)| emit)
    }

    /// 🧭️ The two framework-injected host-configuration verbs, resolved to no Config mutation of this
    /// app's own. The shell owns their session state and forwards the resolved value here so the app
    /// may clear its scratch; puzzle2d's active utility IS host view state
    /// (`puzzle2d_active_utility` reads `ViewModel::active_utility_id`) and its brush slot lives in
    /// the WINDOW TRANSIENT lane, which this Config-typed seam cannot address — the transient is
    /// rebuilt from the next dispatch anyway. Without this hook (and the proofs below) both verbs fell
    /// through to `admit_command_json` and failed closed with `interactive-job.missing-factory`, so
    /// the Fill tool tab could not even be selected.
    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Puzzle2dRetainedCommandProofs::proofs();
        proofs.extend(Puzzle2dHostConfigurationProofs::proofs());
        proofs
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Puzzle2dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<semio_framework_plugin::ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework::ErasedToolJob>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        let original=request.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original puzzle2d producer request is absent"))?;
        if !PUZZLE2D_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {return Ok(false);}
        if original.command.action_id()!=original.tool_id {return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"puzzle2d-command-tool-mismatch").into());}
        let tool_id=original.command.action_id();
        macro_rules! work {
            (|$request:ident| $make:expr) => {
                semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request,output,cx,Puzzle2dCommand::action_id,PUZZLE2D_COMMAND_RAW_BYTES,crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS,true,|$request|{let _=&$request;$make})
            };
        }
        macro_rules! owned {
            ($owner:ty,|$scope:ident| $make:expr) => {
                semio_framework_plugin::retained_command::producer::admit_retained_tool_producer_measured_scoped(request,output,cx,Puzzle2dCommand::action_id,PUZZLE2D_COMMAND_RAW_BYTES,crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS,true,<$owner>::original_birth_demands()?,|_request,$scope|$make)
            };
        }
        match tool_id {
            "setActiveExample" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new("setActiveExample", puzzle2d_active_example_reduce, puzzle2d_active_example_extent)),
            "forceLayout" => owned!(Puzzle2dForceLayoutWork,|scope| Puzzle2dForceLayoutWork::from_original_construction("forceLayout",scope)),
            "reorganize" => owned!(Puzzle2dForceLayoutWork,|scope| Puzzle2dForceLayoutWork::from_original_construction("reorganize",scope)),
            "addNode" => work!(|request| Puzzle2dWindowCommandWork::<Puzzle2dPlayApp>::new("addNode", puzzle2d_add_node_extent)),
            "exportSnapshot" => work!(|request| Puzzle2dExportWork::default()),
            "applyBoardEvents" => work!(|request| Puzzle2dWindowCommandWork::<Puzzle2dPlayApp>::new("applyBoardEvents", puzzle2d_board_events_extent)),
            generic if PUZZLE2D_GENERIC_TOOL_IDS.contains(&generic) => work!(|request| Puzzle2dWindowCommandWork::<Puzzle2dPlayApp>::new(generic, puzzle2d_generic_extent)),
            host_only if PUZZLE2D_HOST_ONLY_TOOL_IDS.contains(&host_only) => work!(|request| crate::retained_command::NoopPuzzleCommandWork::new(host_only)),
            "redrawHandles" => owned!(Puzzle2dRedrawHandlesWork,|scope| Puzzle2dRedrawHandlesWork::from_original_construction(scope)),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"puzzle2d-command-tool-unmapped").into()),
        }
    }

    /// ⏯️ Builds the fill tool run's jobs (`ToolRunDefinition.runJob` / `.revalidateJob`): the run job over the run's
    /// base with the config's requested count, resuming from the ledger's checkpoint and provisional ops, and the
    /// revalidate job over the committed head.
    fn build_tool_run_job(request: semio_framework_plugin::ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolRunJob>, Fault> {
        if request.tool_id != fill::TOOL_ID {
            return Ok(None);
        }
        Ok(Some(match request.purpose {
            semio_framework_plugin::ToolRunJobPurpose::Run => {
                Box::new(fill_run::Puzzle2dFillRunJob::new(request.identity, request.snapshot, crate::editor::puzzle2d::config::PUZZLE2D_DEFAULT_SUGGESTION_OFFSET, request.config.fill_count, request.checkpoint, request.provisional).map_err(Fault::from)?)
            }
            semio_framework_plugin::ToolRunJobPurpose::Revalidate => Box::new(fill_run::Puzzle2dFillRevalidateJob::new(
                request.identity,
                request.snapshot,
                request.provisional,
                request.checkpoint,
                request.config.contact_tolerance - request.config.brush_placement_overlap_budget,
            )),
        }))
    }

    /// 🔌️ Declares puzzle2d's typed media I/O surface — the implicit document ports plus `kit:in` and
    /// `design:out`. `kit:in`'s producer is `Block2dPlayApp`'s `"catalog:out"` port, whose `kit.catalog`
    /// payload is literally a puzzle2d manifest fragment (`portKinds`/`wireKinds`/`edgeKinds`/
    /// `nodeKinds`/`kindCompatibility` — `crate::standards::v1::subsets::any::schema::inferences::puzzle2d_manifest_fragment`),
    /// so this is a 2d↔2d vocabulary, not block3d's object/vortex-kind one; `Puzzle2dImportJob` does the
    /// normalization one bounded row per step.
    fn io() -> Option<AppIo> {
        let io = ::semio_framework_async::poll::resolve_ready(AppIo::from_artifact(
            "puzzle.2d",
            MediaType { class: MediaClass::TwoD, form: MediaForm::Design },
            ArtifactPresentation { id: "2d.puzzle".into(), name: "2D Puzzle".into(), dimension: "2d".into(), component_kind: "puzzle2d".into() },
        ));
        Some(::semio_framework_async::poll::resolve_ready(io.with_ports(vec![
            MediaPortSpec {
                id: PUZZLE2D_IMPORT_PORT.into(),
                label: "Kit Catalog".into(),
                direction: MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                kind_id: Some("kit.catalog".into()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
            MediaPortSpec {
                id: "design:out".into(),
                label: "Puzzle Design".into(),
                direction: MediaPortDirection::Out,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Design },
                kind_id: Some("2d.puzzle".into()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
        ])))
    }

    /// 📋️ The `MediaType` a puzzle 2d copy writes and a paste accepts — without it the framework's
    /// injected `copy`/`cut`/`paste` actions silently no-op.
    fn clipboard_media_type() -> Option<MediaType> {
        Some(MediaType { class: MediaClass::TwoD, form: MediaForm::Design })
    }

    fn copy_fragment(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle2dConfig>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        let marks = Puzzle2dInteractionSnapshot::from_interaction(interaction);
        puzzle2d_copy_fragment_from(doc.snapshot.value(), &puzzle2d_selected_node_ids(doc.snapshot.value(), marks.selected_ids()))
    }

    fn cut_operations(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle2dConfig>, interaction: &InteractionView<'_>) -> Vec<Puzzle2dMutation> {
        let marks = Puzzle2dInteractionSnapshot::from_interaction(interaction);
        let selected = puzzle2d_selected_node_ids(doc.snapshot.value(), marks.selected_ids());
        if puzzle2d_selection_is_locked(doc.snapshot.value(), &selected) {
            return Vec::new();
        }
        puzzle2d_cut_operations_from(doc.snapshot, &selected)
    }

    fn paste_operations(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle2dMutation>, ClipboardError> {
        puzzle2d_paste_operations_on(doc.snapshot, fragment, placement).map(|(mutations, _)| mutations)
    }

    /// 🎞️ The reserved routes puzzle2d owns: `import-media` (the framework registers no importer on an
    /// app's behalf, and every inbound media delivery goes through `dispatch_import_media` →
    /// `build_artifact_reserved_media_job`, never the unbounded one-shot `ArtifactApp::import_media`
    /// seam) and the three clipboard verbs, whose fragment vocabulary is this artifact's own
    /// `puzzle.2d.clipboard.v1` — see [`Puzzle2dClipboardJob`].
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if matches!(request.tool_id.as_str(), "copy" | "cut" | "paste") {
            return Ok(Some(ArtifactReservedToolJob::new(Puzzle2dClipboardJob::new(request))));
        }
        if request.tool_id.as_str() != PUZZLE2D_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(Fault::from("puzzle2d import-media admits a decoded media value, never a wire payload"));
        }
        let ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(Fault::from("puzzle2d import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(ArtifactReservedToolJob::new(Puzzle2dImportJob::new(request, port, media))))
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, cfg, view_state, &Puzzle2dWindowTransient::default(), Puzzle2dInteractionSnapshot::default())
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> semio_framework_plugin::plugin_app_close_prelude::semio_framework_plugin_ownership::sdk_fault::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, cfg, view_state, &window::transient_from_view(transient), Puzzle2dInteractionSnapshot::from_interaction(interaction)).map(Some).map_err(Into::into)
    }

    fn window_engagements(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let labels = puzzle2d_labels(view_state);
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_kind = window::kind_for_view(view_state).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let envelope = puzzle2d_scene_for(doc.snapshot.value().clone(), window::runtime(cfg.snapshot, &window_config, &Puzzle2dWindowTransient::default(), Some(window_kind)), puzzle2d_active_utility(Some(view_state)));
        HashMap::from([(window_id.to_string(), edit::canvas::puzzle2d_engagement(&envelope, &puzzle_board_host(), window_kind, labels))])
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_transient = window::transient_from_view(transient);
        let window_kind = window::kind_for_view(view_state).unwrap_or(semio_s_artifact_puzzle_2d_inferences::viewer::puzzle2d::modes::view::windows::board::WINDOW_KIND_ID);
        let envelope = puzzle2d_scene_for(doc.snapshot.value().clone(), window::runtime(cfg.snapshot, &window_config, &window_transient, Some(window_kind)), puzzle2d_active_utility(Some(view_state)));
        HashMap::from([(window_id.to_string(), edit::canvas::puzzle2d_engagement(&envelope, &puzzle_board_host(), window_kind, puzzle2d_labels(view_state)))])
    }

    fn window_measures(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = puzzle2d_labels(view_state);
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_kind = window::kind_for_view(view_state).unwrap_or(semio_s_artifact_puzzle_2d_inferences::viewer::puzzle2d::modes::view::windows::board::WINDOW_KIND_ID);
        let envelope = puzzle2d_scene_for(doc.snapshot.value().clone(), window::runtime(cfg.snapshot, &window_config, &Puzzle2dWindowTransient::default(), Some(window_kind)), puzzle2d_active_utility(Some(view_state)));
        let measures = match window_kind {
            semio_s_artifact_puzzle_2d_inferences::viewer::puzzle2d::modes::view::windows::board::WINDOW_KIND_ID => detail::window_measures(&envelope, labels),
            semio_s_artifact_puzzle_2d_inferences::viewer::puzzle2d::modes::view::windows::board::WINDOW_KIND_ID => selection::window_measures(&envelope, labels),
            _ => overview::window_measures(&envelope, labels),
        };
        HashMap::from([(window_id.to_string(), measures)])
    }

    fn tool_measures(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_kind = window::kind_for_view(view_state).unwrap_or(semio_s_artifact_puzzle_2d_inferences::viewer::puzzle2d::modes::view::windows::board::WINDOW_KIND_ID);
        let envelope = puzzle2d_scene_for(doc.snapshot.value().clone(), window::runtime(cfg.snapshot, &window_config, &Puzzle2dWindowTransient::default(), Some(window_kind)), puzzle2d_active_utility(Some(view_state)));
        let labels = puzzle2d_labels(view_state);
        HashMap::from([(fill::TOOL_ID.to_string(), vec![fill::measures(&envelope, labels)])])
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        _cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, view_state, &Puzzle2dInteractionSnapshot::default(), registry)
    }

    /// 🕹️ The context menu reads the AUTHORITATIVE framework-owned selection when the surface names
    /// none of its own — a right-click on an outliner row or an empty board still targets what the
    /// user selected; the surface's own hits/selection win when present, so a right-click on an
    /// unselected node still targets that node.
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        _cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, view_state, &Puzzle2dInteractionSnapshot::from_interaction(interaction), registry)
    }
}

impl Puzzle2dPlayApp {
    fn context_menu_body(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &Puzzle2dInteractionSnapshot,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let mut selected: Vec<String> = request.surface.as_ref().map(|surface| surface.selection.iter().flat_map(|g| g.ids.iter().cloned()).collect()).unwrap_or_default();
        if selected.is_empty() {
            selected = interaction.selected.clone();
        }
        ::semio_framework_async::poll::resolve_ready(puzzle2d_context_menu_items(registry, doc.snapshot.value(), &selected, view_state))
    }
}
//#endregion 🧵️ReservedJobs
//#endregion 🔖️PlayApp
