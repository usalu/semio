//! ✍️ Artifact editor: the `ArtifactEditor` implementation of the puzzle5d play app. See [`super`].

use super::*;

//#region 🔖️PlayApp
impl ArtifactEditor for Puzzle5dPlayApp {
    const DIALECT: semio_framework_plugin::app::Dialect = crate::PUZZLE5D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE5D_SCHEMA;
    type Snapshot = Puzzle5dPlaySnapshot;
    type Mutation = Puzzle5dMutation;
    type Config = Puzzle5dConfig;
    type ConfigMutation = Puzzle5dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = Puzzle5dPresence;
    type PresenceMutation = Puzzle5dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = Puzzle5dCommand;

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        window_ownership::register_config(registry)
    }

    /// 🪧️ A history-edit reference chip names its board/world entity as the outliner does ([`puzzle5d_entity_label`]).
    fn entity_label(snapshot: &Puzzle5dPlaySnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        puzzle5d_entity_label(snapshot.typed(), kinds, id)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        window_ownership::register_transient(registry)
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

    /// 👥️ Puzzle 5d presence is inline camera scalars, so the default root is its exact empty terminal.
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |_| true).expect("default Puzzle5d presence is the exact empty terminal")))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Puzzle5dStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Puzzle5dConfigStorePreparationFactory))
    }

    /// ⏯️ The fill utility's run and revalidate jobs and the brush utility's suggestions run: the puzzle 3d planners
    /// behind the 5d translation.
    fn build_tool_run_job(request: semio_framework_plugin::ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolRunJob>, Fault> {
        match request.tool_id {
            board2d::utilities::brush::UTILITY_ID => crate::editor::puzzle5d::precompute::brush::build_run_job(request),
            fill_tool::TOOL_ID => crate::editor::puzzle5d::precompute::fill::build_run_job(request),
            _ => Ok(None),
        }
    }

    /// 📜️ The retained command catalog's proofs plus the two host-configuration verbs' generic ones.
    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Puzzle5dRetainedCommandProofs::proofs();
        proofs.extend(Puzzle5dHostConfigurationProofs::proofs());
        proofs
    }

    /// 🧠️ What one document instance retains across turns: puzzle 3d's brush suggestions link, which the brush run
    /// job follows and publishes to.
    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(Puzzle3dInstanceOperationOwner::default())
    }

    /// 🚦️ Starts, wakes or aborts the brush suggestions run for the grip the link points at.
    fn pending_effects(owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, doc: &semio_framework_plugin::ArtifactPendingView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>, _view: Option<&semio_framework_plugin::ViewModel>) -> Vec<Effect> {
        owner.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(semio_s_artifact_puzzle_3d_editor_modes::windows::main::utilities::brush::run_effects(&mut owner.brush_suggestions, doc.tool_run()))).unwrap_or_default()
    }


    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(Puzzle5dRetainedCommandJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: &mut Option<semio_framework_plugin::ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework::ErasedToolJob>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        let original=request.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original puzzle5d producer request is absent"))?;
        if !PUZZLE5D_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {return Ok(false);}
        if original.command.action_id()!=original.tool_id {return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"puzzle5d-command-tool-mismatch").into());}
        let tool_id=original.command.action_id();
        macro_rules! work {
            (|$request:ident| $make:expr) => {
                semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request,output,cx,Puzzle5dCommand::action_id,PUZZLE5D_RETAINED_RAW_BYTES,crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS,true,|$request|{let _=&$request;$make})
            };
        }
        macro_rules! seed {
            (|$request:ident,$seed:ident| $make:expr) => {{
                let bytes=original.authoring_seed.len();
                let demand=semio_framework_value::RetirementDemand{copy_bytes:bytes,capacity_bytes:bytes,depth:1,..Default::default()};
                semio_framework_plugin::retained_command::producer::admit_retained_tool_producer_measured(request,output,cx,Puzzle5dCommand::action_id,PUZZLE5D_RETAINED_RAW_BYTES,crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS,true,demand,|$request,_|{
                    let $seed=$request.authoring_seed.clone();let capacity=$seed.capacity();
                    Ok(($make,semio_framework_job::RetainedCloneProgress{copied_bytes:bytes,retained_capacity_bytes:capacity,..Default::default()}))
                })
            }};
        }
        macro_rules! owned {
            ($owner:ty, |$request:ident, $scope:ident| $make:expr) => {{
                let demand = <$owner>::original_birth_demands()?;
                semio_framework_plugin::retained_command::producer::admit_retained_tool_producer_measured_scoped(
                    request, output, cx, Puzzle5dCommand::action_id, PUZZLE5D_RETAINED_RAW_BYTES,
                    crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, true, demand,
                    |$request, $scope| { let _ = &$request; $make },
                )
            }};
        }
        let nonce=puzzle5d_operation_nonce(&original.operation);
        match tool_id {
            "engagementAbort" | "openVortexSuggestions" | "closeVortexSuggestions" => seed!(|request, seed| Puzzle5dWindowCommandWork::new(tool_id, seed).bound(request.instance_operation_owner.clone())),
            window if PUZZLE5D_WINDOW_TOOL_IDS.contains(&window) => seed!(|request, seed| Puzzle5dWindowCommandWork::new(window, seed).bound(request.instance_operation_owner.clone())),
            "addBrushPart" | "addPartKind" => owned!(Puzzle5dAddBrushPartWork, |request, scope| Puzzle5dAddBrushPartWork::from_original_construction(tool_id, scope).map(|(work, progress)| (work.bound(nonce), progress))),
            "applyBoardEvents" => owned!(Puzzle5dBoardEventsWork, |request, scope| Puzzle5dBoardEventsWork::from_original_construction(scope).map(|(work, progress)| (work.bound(nonce), progress))),
            "translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => seed!(|request, seed| Puzzle5dTransformWork::new(tool_id, seed)),
            "patchPart" if puzzle5d_inspector_nudge(original.command.args()).is_some() => seed!(|request, seed| Puzzle5dTransformWork::new(tool_id, seed)),
            "focusSelection" => work!(|request| Puzzle5dFocusSelectionWork::default()),
            "patchPart" => owned!(Puzzle5dPatchPartWork, |request, scope| Puzzle5dPatchPartWork::from_original_construction(scope)),
            "patchFastener" => owned!(Puzzle5dPatchFastenerWork, |request, scope| Puzzle5dPatchFastenerWork::from_original_construction(scope)),
            "editFastener" => owned!(Puzzle5dEditFastenerWork, |request, scope| Puzzle5dEditFastenerWork::from_original_construction(scope)),
            "retargetFastener" => owned!(Puzzle5dRetargetFastenerWork, |request, scope| Puzzle5dRetargetFastenerWork::from_original_construction(scope)),
            "proximityConnect" => owned!(Puzzle5dProximityConnectWork, |request, scope| Puzzle5dProximityConnectWork::from_original_construction(scope).map(|(work, progress)| (work.bound(nonce), progress))),
            "patchGrip" => owned!(Puzzle5dPatchGripWork, |request, scope| Puzzle5dPatchGripWork::from_original_construction(scope)),
            "deleteFastener" => owned!(Puzzle5dDeleteFastenerWork, |request, scope| Puzzle5dDeleteFastenerWork::from_original_construction(scope)),
            "addNode" => owned!(Puzzle5dAddNodeWork, |request, scope| Puzzle5dAddNodeWork::from_original_construction(scope).map(|(work, progress)| (work.bound(nonce), progress))),
            "createFastener" => work!(|request| Puzzle5dCreateFastenerWork::default().bound(nonce)),
            "exportSnapshot" => work!(|request| Puzzle5dExportWork::default()),
            "setActiveExample" => owned!(Puzzle5dSetActiveExampleWork, |request, scope| Puzzle5dSetActiveExampleWork::from_original_construction(scope)),
            "registerBrushMesh" => work!(|request| Puzzle5dRegisterBrushMeshWork::default()),
            "setPartKindWeight" | "setGripKindWeight" => owned!(Puzzle5dKindWeightWork, |_request, scope| Puzzle5dKindWeightWork::from_original_construction(tool_id, scope)),
            "worldPointerDown" | "canvasPointerDown" => work!(|request| crate::retained_command::NoopPuzzleCommandWork::new(tool_id)),
            _ => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce::<Puzzle5dPlayApp>, puzzle5d_retained_extent)),
        }
    }

    fn build_reserved_tool_job(mut request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if !["copy", "cut", "paste", "import-media"].contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        let raw = std::mem::take(&mut request.raw_wire);
        request.raw_wire = match puzzle5d_preflight_reserved_wire(raw, request.contract.max_raw_wire_bytes) {
            Ok(raw) => raw,
            Err((fault, rejected)) => {
                drop(rejected);
                return Err(fault);
            }
        };
        let job = match request.tool_id.as_str() {
            "copy" => {
                let interaction = match &request.input {
                    ArtifactReservedToolInput::Action { interaction, .. } => interaction.clone(),
                    _ => return Err(Fault::from("puzzle5d copy requires action input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dClipboardJob::new(Puzzle5dClipboardVerb::Copy, request, &interaction))
            }
            "cut" => {
                let interaction = match &request.input {
                    ArtifactReservedToolInput::Action { interaction, .. } => interaction.clone(),
                    _ => return Err(Fault::from("puzzle5d cut requires action input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dClipboardJob::new(Puzzle5dClipboardVerb::Cut, request, &interaction))
            }
            "paste" => {
                let args = match &request.input {
                    ArtifactReservedToolInput::Action { args, .. } => args.clone(),
                    _ => return Err(Fault::from("puzzle5d paste requires action input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dPasteJob::new(request, args.map(|value| semio_framework_pack_json::from_dsl_value(&value))))
            }
            "import-media" => {
                let (port, media) = match &request.input {
                    ArtifactReservedToolInput::Media { port, media } => (port.clone(), media.clone()),
                    _ => return Err(Fault::from("puzzle5d import-media requires media input")),
                };
                ArtifactReservedToolJob::new(Puzzle5dImportJob::new(request, port, media))
            }
            _ => return Ok(None),
        };
        Ok(Some(job))
    }

    /// 📎 Ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d: replaces the old
    /// `crate::editor::puzzle5d::config::schema::register_app_schema()` self-registering call, which
    /// puzzle's plugin root used to reach `.setup()` for — `register_document_app`/`document_app`
    /// now call this automatically the moment `Puzzle5dPlayApp` is bound to a plugin, exactly like
    /// `🗒️note`'s own `app_schema` override.
    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::puzzle5d::config::schema::app_schema_descriptor())
    }

    /// 🚀️ Boots on `default_document()` and on NOTHING else.
    ///
    /// 🐛️ This used to pre-warm three statics before returning: `NAKAGIN_EXAMPLE_DOCUMENT` (168 355 B
    /// of DSL), `CAPSULE_DREAM_EXAMPLE_DOCUMENT` (3 035 200 B of DSL → ~3.5 MB of JSON → a typed
    /// `Puzzle5dDocument` of 2 880 parts) and `PUZZLE5D_EXAMPLE_OPERATIONS` — the 4×4 matrix of
    /// PAIRWISE semantic diffs between all four example documents, i.e. sixteen diffs, several of them
    /// between a 2 880-part and a 180-part document. Every mount of this app paid all of it before its
    /// first frame, which is why a pane took seconds to boot and why this crate's own example-switch
    /// and retained-import laws ran for over a minute each (the 2026-09-21 test binary was killed by
    /// the 10-minute watchdog). None of it is needed to produce the boot document, and nothing is lost:
    /// each static is a `LazyLock`, forced only when its example is switched to. That matrix has since been
    /// deleted outright.
    fn reference_birth()->Option<fn()->Puzzle5dPlaySnapshot>{Some(||{
        Puzzle5dPlaySnapshot::new(puzzle5d_snapshot_from_document(&default_document()).expect("authored initial document admits"))
    })}

    fn clipboard_media_type() -> Option<MediaType> {
        Some(MediaType { class: MediaClass::Kit, form: MediaForm::Design })
    }

    fn copy_fragment(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        let (part_ids, fastener_ids) = puzzle5d_interaction_part_and_fastener_ids(interaction);
        puzzle5d_copy_fragment(doc.snapshot, &part_ids, &fastener_ids)
    }

    /// ✂️ B1: `ArtifactApp::cut_operations`'s signature carries no config output channel (it
    /// returns a bare `Vec<Self::Mutation>`, not an `Emit`), so this can only emit the document
    /// removal; clearing the selection is left to the framework's own post-cut selection reconciliation
    /// (the cut parts/fasteners are gone from the document either way, so a stale selection referencing
    /// them is inert until the next real selection action overwrites it).
    fn cut_operations(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>, interaction: &InteractionView<'_>) -> Vec<Puzzle5dMutation> {
        let (part_ids, fastener_ids) = puzzle5d_interaction_part_and_fastener_ids(interaction);
        puzzle5d_cut_operations(doc.snapshot, &part_ids, &fastener_ids)
    }

    /// 📋️ B1: `ArtifactApp::paste_operations` carries no `ConfigView` at all (only `doc`/
    /// `fragment`/`placement`), so the new selection can't be threaded through this call; a following
    /// `setSelection` command (which the host already issues after a paste in practice) is what
    /// actually selects the pasted parts now.
    fn paste_operations(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle5dMutation>, ClipboardError> {
        let expected = Self::clipboard_media_type().unwrap_or(MediaType { class: MediaClass::Kit, form: MediaForm::Design });
        if fragment.media_type != expected {
            return Err(ClipboardError::IncompatibleMediaType(fragment.media_type));
        }
        let fragment_value: Value = parse(&fragment.dsl_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_parts: Vec<Puzzle5dPart> = puzzle5d_record_from_projection(fragment_value.get("parts").cloned().unwrap_or_else(|| semio_framework_pack_json::json!([]))).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_fasteners: Vec<Puzzle5dFastener> = puzzle5d_record_from_projection(fragment_value.get("fasteners").cloned().unwrap_or_else(|| semio_framework_pack_json::json!([]))).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let before = puzzle5d_editor_projection(&doc.snapshot);
        let document: Puzzle5dDocument = <Puzzle5dDocument as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&before)).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let delta = paste_delta_2d(&fragment_parts, &document.parts, placement);
        let (fresh_parts, fresh_fasteners) = paste_selection_local(&document, &fragment_parts, &fragment_fasteners, delta);
        puzzle5d_paste_mutations(fresh_parts, fresh_fasteners).map_err(|error| ClipboardError::ParseFailed(error.to_string()))
    }

    /// 🏷️ Maps each `Puzzle5dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle5dCommand) -> &'static str {
        command.action_id()
    }

    /// 📢️ The localized notices of this guest's refusal codes ([`puzzle5d_fault_notices`]).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        puzzle5d_fault_notices()
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        if let Some(flag) = puzzle5d_flag_value_argument(action) {
            args.and_then(|value| value.get(flag)).and_then(semio_framework_value::DslValue::as_bool).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("puzzle5d.action.flag-value-required"), format!("action '{action}' requires the boolean '{flag}' it sets")))?;
        }
        let args = args.map(semio_framework_pack_json::from_dsl_value);
        let window_id = args.as_ref().and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(Value::as_str).map(str::to_string);
        Puzzle5dCommand::try_from_action(action, args, window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 5D action '{action}'")))
    }

    /// 🧩️ Thin typed-command adapter — reconstructs the exact `(action, args, window_id)`
    /// triple `handle_action_impl` expects from the typed `Puzzle5dCommand`.
    fn handle(
        command: &Puzzle5dCommand,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Puzzle5dMutation, Puzzle5dConfigMutation, Self::DraftMutation>, Fault> {
        let selection = interaction.selection(PUZZLE5D_INTERACTION_DOMAIN);
        let window_id = view_state.and_then(|view| view.window_id.as_deref()).or_else(|| command.window_id()).unwrap_or(crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
        Ok(puzzle5d_handle_action::<Self>(authoring_seed, command.action_id(), command.args(), command.window_id(), doc.snapshot, &runtime, view_state, selection, None, None).0)
    }

    /// 🕹️ `vortex` domain topology (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
    /// parts and fasteners as flat roots, grips nested under their owning part (mirrors puzzle3d's
    /// object→vortex-marker nesting).
    fn interaction_topology(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle5dConfig>) -> Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let document: Puzzle5dDocument = puzzle5d_document_from_snapshot(doc.snapshot.typed()).expect("admitted snapshot projects to native editor document");
        let mut ordered = Vec::new();
        for part in &document.parts {
            ordered.push(semio_framework_plugin::TopologyNode { id: part.id.clone(), granularity: PUZZLE5D_GRANULARITY_PART.into(), parent: None });
            for grip in &part.grips {
                ordered.push(semio_framework_plugin::TopologyNode { id: puzzle5d_grip_full_id(&part.id, &grip.id), granularity: PUZZLE5D_GRANULARITY_GRIP.into(), parent: Some(part.id.clone()) });
            }
        }
        for fastener in &document.fasteners {
            ordered.push(semio_framework_plugin::TopologyNode { id: fastener.id.clone(), granularity: PUZZLE5D_GRANULARITY_FASTENER.into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(PUZZLE5D_INTERACTION_DOMAIN.to_string(), semio_framework_plugin::DomainTopology { ordered });
        semio_framework_plugin::InteractionTopology { domains }
    
})())
}

    /// 🔌️ Declares puzzle5d's typed media I/O surface: the implicit document ports (from
    /// `.document([...])`/`.artifact_kind(...)` in `create_puzzle5d_app`) plus `kit:in` (accepting a
    /// `kit.catalog` fragment shaped like block3d's `puzzle3d_catalog_fragment`, fanning IN from
    /// potentially many producers) and `design:out` (this app's own `5d.puzzle` design artifact, fanning
    /// OUT to potentially many consumers).
    fn io() -> Option<AppIo> {
        let io = ::semio_framework_async::poll::resolve_ready(AppIo::from_artifact(
            "puzzle.5d",
            MediaType { class: MediaClass::Kit, form: MediaForm::Design },
            ArtifactPresentation { id: "5d.puzzle".into(), name: "5D Puzzle".into(), dimension: "5d".into(), component_kind: "puzzle5d".into() },
        ));
        Some(::semio_framework_async::poll::resolve_ready(io.with_ports(vec![
            MediaPortSpec {
                id: "kit:in".into(),
                label: "Kit Catalog".into(),
                direction: MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                kind_id: Some("kit.catalog".into()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
            MediaPortSpec {
                id: "design:out".into(),
                label: "5D Puzzle Design".into(),
                direction: MediaPortDirection::Out,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design },
                kind_id: Some("5d.puzzle".into()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
        ])))
    }

    /// 🧵️ The synchronous editor callback is deliberately closed: production import enters only
    /// remains batch-only until an artifact-lane preparation owner can retire its publication roots.
    fn import_media(_port: &str, _media: &Media, _doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>) -> Result<Emit<Puzzle5dMutation, Puzzle5dConfigMutation, Self::DraftMutation>, MediaError> {
        Err(MediaError::NotImplemented)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let projection = puzzle5d_editor_projection(&doc.snapshot);
        let window_for_body = if body_key == board2d::BODY_KEY { crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID } else { crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID };
        let window_id = view_state.window_id.as_deref().unwrap_or(window_for_body);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        let labels = puzzle5d_labels(view_state).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.localization.unsupported", "puzzle5d has no authored label set for the host's locale/terminology axes"))?;
        let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
        let node = match body_key {
            board2d::BODY_KEY => board2d::render(&envelope, labels, None),
            world3d::BODY_KEY => world3d::render(&envelope, labels, doc.tool_run(), &crate::editor::puzzle5d::precompute::puzzle5d_mesh_lane(doc.snapshot, &envelope.document), None),
            artifact_panel::BODY_KEY => artifact_panel::render(&envelope, labels, &windows),
            catalogue::BODY_KEY => catalogue::render(&envelope, labels, &windows),
            inspection::BODY_KEY => inspection::render(&envelope, labels, &windows),
            settings_panel::BODY_KEY => settings_panel::render(&envelope, labels, settings_panel::panel_window_id(view_state)),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d unknown-body label admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        let projection = puzzle5d_editor_projection(&doc.snapshot);
        let window_kind = window_ownership::kind_for_view(view_state).unwrap_or_else(|| if body_key == board2d::BODY_KEY { crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID } else { crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID });
        let window_id = view_state.window_id.as_deref().unwrap_or(window_kind);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::transient_from_view(transient), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection_with_interaction(&projection, runtime, &active_utility, Puzzle5dInteractionSnapshot::from_interaction(interaction));
        let labels = puzzle5d_labels(view_state).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.localization.unsupported", "puzzle5d has no authored label set for the host's locale/terminology axes"))?;
        let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
        let menu_target = envelope.runtime.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.as_str());
        let suggestions = menu_target.and_then(|target| owner.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(owner.brush_suggestions.found(target).cloned())).ok().flatten());
        let node = match body_key {
            board2d::BODY_KEY => board2d::render(&envelope, labels, suggestions.as_ref()),
            world3d::BODY_KEY => world3d::render(&envelope, labels, doc.tool_run(), &crate::editor::puzzle5d::precompute::puzzle5d_mesh_lane(doc.snapshot, &envelope.document), suggestions.as_ref()),
            artifact_panel::BODY_KEY => artifact_panel::render(&envelope, labels, &windows),
            catalogue::BODY_KEY => catalogue::render(&envelope, labels, &windows),
            inspection::BODY_KEY => inspection::render(&envelope, labels, &windows),
            settings_panel::BODY_KEY => settings_panel::render(&envelope, labels, settings_panel::panel_window_id(view_state)),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d unknown-body label admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_engagements(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let projection = puzzle5d_editor_projection(&doc.snapshot);
        let Some(labels) = puzzle5d_labels(view_state) else { return HashMap::new() };
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_kind = window_ownership::kind_for_view(view_state).unwrap_or(crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        HashMap::from([(window_id.to_string(), edit::puzzle5d_engagement(&envelope, window_kind, labels, doc.tool_run()))])
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let Some(labels) = puzzle5d_labels(view_state) else { return HashMap::new() };
        let window_kind = window_ownership::kind_for_view(view_state).unwrap_or(crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::transient_from_view(transient), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&puzzle5d_editor_projection(&doc.snapshot), runtime, &active_utility);
        HashMap::from([(window_id.to_string(), edit::puzzle5d_engagement(&envelope, window_kind, labels, doc.tool_run()))])
    }

    fn window_measures(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let projection = puzzle5d_editor_projection(&doc.snapshot);
        let Some(labels) = puzzle5d_labels(view_state) else { return HashMap::new() };
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_kind = window_ownership::kind_for_view(view_state).unwrap_or(crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        let measures = if window_kind == crate::editor::puzzle5d::entities::PUZZLE5D_BOARD_WINDOW_KIND_ID { board2d::window_measures(&envelope, labels) } else { world3d::window_measures(&envelope, labels) };
        HashMap::from([(window_id.to_string(), measures)])
    }

    /// 🛠️ The mode-level tool options rail: the fill tool's count entry and distribution trees, live against
    /// this instance's run so the count reads `loading` while the run is working.
    fn tool_measures(doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(labels) = puzzle5d_labels(view_state) else { return HashMap::new() };
        let window_id = view_state.window_id.as_deref().unwrap_or(crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID);
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let envelope = scene_from_projection(&puzzle5d_editor_projection(&doc.snapshot), runtime, &active_utility);
        HashMap::from([(fill_tool::TOOL_ID.to_string(), fill_tool::measures(&envelope, labels, doc.tool_run()))])
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &Puzzle5dInteractionSnapshot::default(), registry)
    }

    /// 🕹️ The live-`vortex`-domain twin: a grip or fastener selected in EITHER pane reaches the menu even though
    /// the world host's surface only reports its painted part ids.
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &Puzzle5dInteractionSnapshot::from_interaction(interaction), registry)
    }
}
//#endregion 🔖️PlayApp
