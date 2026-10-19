//! 🎛️ Artifact editor of the remodeling editor: the dispatch-only implementation of the artifact editor trait for the remodeling play app. See [`super`].

use super::*;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️RemodelingPlayApp
impl ArtifactEditor for RemodelingPlayApp {
    /// 📢️ The localized notices of this editor's user-reachable refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        use semio_framework_ui_locale::LocalizedLabel;
        static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 5]> = std::sync::LazyLock::new(|| {
            [
            ("remodeling.import.open", LocalizedLabel::native("An import is still running in this window.", "In diesem Fenster läuft noch ein Import.")),
            ("remodeling.import.window-required", LocalizedLabel::native("A streamed import needs an open window.", "Ein gestreamter Import braucht ein geöffnetes Fenster.")),
            ("remodeling.stream.unknown-camera", LocalizedLabel::native("This camera is not calibrated in the document.", "Diese Kamera ist im Dokument nicht kalibriert.")),
            ("remodeling.qc-report.missing", LocalizedLabel::native("There is no quality report to export yet.", "Es gibt noch keinen Qualitätsbericht zum Exportieren.")),
            ("remodeling.retained.extent", LocalizedLabel::native("The document is too large for this action.", "Das Dokument ist für diese Aktion zu groß.")),
            ]
        });
        &*NOTICES
    }

    type Snapshot = RemodelingSnapshot;
    type Mutation = RemodelingMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = RemodelingCommand;

    const DIALECT: Dialect = semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::REMODELING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = REMODELING_DOCUMENT_SCHEMA;

    /// 📬️ The document lane's one-item retained preparation — the framework's generic bounded
    /// preparation (the shooting/dag/trinity precedent), never a copied preparation factory. Without one,
    /// every route declaring `ArtifactToolPublicationLane::Artifact` is registered with an unsupported
    /// publication contract and stays dispatch-dead, no matter how it is classified. Item-count
    /// admission (`REMODELING_RETAINED_WORK_ITEMS`) is `remodeling_retained_extent`'s at dispatch.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

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

    /// 🫧️ Every window kind's transient partition — where the import a window started lives between its dispatches.
    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        transient::register(registry)
    }

    /// 📨️ A closing window aborts the import it started with zero trace; every other host fact leaves an import running.
    fn host_event(event: &semio_framework_plugin::HostEvent) -> Option<RemodelingCommand> {
        match event {
            semio_framework_plugin::HostEvent::Retiring { .. } => Some(RemodelingCommand::ImportAbort(import_abort::ImportAbort { reason: Some("retired".to_string()) })),
            _ => None,
        }
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<model::windows::model::config::RemodelingModelWindowConfigOwner>()?;
        registry.register::<capture::windows::frames::config::RemodelingFramesWindowConfigOwner>()?;
        registry.register::<analyze::windows::report::config::RemodelingReportWindowConfigOwner>()
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<RemodelingPlayApp>,
        owner_file: "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.remodel.remodeling@1/*#editor",
        artifact_schema: "remodeling.scene",
        factory: "RemodelingRetainedCommandJobFactory",
        factory_type: RemodelingRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "importFramePayload", "importVideoFramePayload", "importVideoDone", "importVideoBytesPayload",
            "addStream", "removeStream", "setStreamSync",
            "editCalibration", "calibrateCameras", "addGcp", "removeGcp", "placeGcpObservation",
            "setIngestParams", "setFeatureParams", "setMatchParams", "setSfmParams", "setDenseParams", "setMeshParams", "setMotionParams", "setGeoParams",
            "resetPlaceholderMesh", "clearSparse", "clearDense", "clearMeshResult", "clearTracks", "clearGeoProducts", "clearResult",
            "setCamera", "setLayerVisibility", "setFrameCursor", "setReportTable", "importFrames", "importVideo", "exportQcReport",
            "setActiveExample", "importAbort"
        ]
    }

    /// ⏯️ The reconstruction tool's run job over the run's base, and its finalize revalidation over the head
    /// (`📋️tool-run-contract.md` §3.7).
    fn build_tool_run_job(request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        if request.tool_id != model::tools::reconstruction::TOOL_ID {
            return Ok(None);
        }
        let provisional = u32::try_from(request.provisional.len()).map_err(|_| Fault::from("remodeling.reconstruction.provisional-count"))?;
        Ok(Some(match request.purpose {
            ToolRunJobPurpose::Run => Box::new(ReconstructionRunJob::new(request.identity, request.snapshot, request.checkpoint, provisional)),
            ToolRunJobPurpose::Revalidate => Box::new(ReconstructionRevalidateJob::new(request.identity, request.snapshot, request.checkpoint, provisional)),
        }))
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(RemodelingRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !REMODELING_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), "Remodeling command does not match its exact registered tool"));
        }
        if remodeling_retained_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("remodeling.retained.extent"), "Remodeling bounded route exceeded its declared work extent"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(RemodelingWindowCommandWork::new(tool_id));
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
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            RemodelingCommand::command_id,
            REMODELING_RETAINED_RAW_BYTES,
            REMODELING_RETAINED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 🚀️ Boots on the registered boot example when its committed text parses, falling back to the
    /// artifact's own `default_remodeling_scene()` otherwise — the same shape `🧱️block`'s
    /// `default_block2d_snapshot()` uses, so an unparsable example degrades instead of panicking.
    fn initial_snapshot() -> RemodelingSnapshot {
        semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::examples::boot_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(remodeling_io())
    }

    /// 🎞️ `mesh:out` (the current reconstructed mesh, GLB-encoded) plus the inherited `document:out`
    /// default (the pack of `doc.snapshot`, replicated inline — overriding `export_media` shadows the
    /// trait's provided body for every port, not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, RemodelingSnapshot>) -> Result<Media, MediaError> {
        match port {
            "mesh:out" => {
                let mesh =
                    semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::resolve_bounded_remodeling_mesh(&doc.snapshot.durable_artifacts, &doc.snapshot.results.mesh.mesh).ok_or_else(|| MediaError::Payload(port.to_string(), "mesh:out: bounded composed mesh content is unavailable".into()))?;
                let bytes = MeshExporter::export(&GlbExporter, &mesh).map_err(|error| MediaError::Payload(port.to_string(), error))?;
                Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "3d.mesh".into(), json: base64_codec::base64_standard_encode(bytes) } })
            }
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `photos:in` — inserts an incoming photo as one new frame on the well-known
    /// `REMODELING_WORKFLOW_PHOTOS_STREAM_ID` image-sequence stream (creating it on the first import).
    /// `document:in` stays `MediaError::NotImplemented`, unchanged from the inherited default: remodeling
    /// has no whole-document-replace `Mutation` variant to satisfy `whole_document_mutation`
    /// (`RemodelingMutation` is deliberately field-granular — see that enum's doc comment).
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, RemodelingSnapshot>) -> Result<Emit<RemodelingMutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "photos:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "photos:in only accepts a Structured base64-image payload".into()));
                };
                let bytes = base64_codec::base64_standard_decode(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let (width, height) = decode_still_image("image/png", &bytes).map_or((0, 0), |image| (image.width, image.height));
                let scene = doc.snapshot;
                let stream_id = REMODELING_WORKFLOW_PHOTOS_STREAM_ID;
                let frame_index = scene.streams.iter().find(|stream| stream.id == stream_id).map_or(0, |stream| stream.frames.len() as u32);
                let asset_key = format!("{stream_id}-frame-{frame_index}");
                let asset = ImageAsset { mime: "image/png".into(), data: json.clone(), width, height };
                let mut mutations = vec![semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::create_asset::create_asset(asset_key.clone(), asset)];
                match scene.streams.iter().any(|stream| stream.id == stream_id) {
                    true => mutations.push(semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::add_stream_frame::add_stream_frame(stream_id.to_string(), FrameRef { index: frame_index, timestamp_ms: f64::from(frame_index) * 1000.0 / 30.0, asset_id: asset_key }, MediaKind::ImageSequence)),
                    false => mutations.push(semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::create_stream::create_stream(MediaStream {
                        id: stream_id.to_string(),
                        name: "Workflow Photos".into(),
                        kind: MediaKind::ImageSequence,
                        camera_id: None,
                        sync_offset_ms: 0.0,
                        fps_hint: 30.0,
                        frames: vec![FrameRef { index: 0, timestamp_ms: 0.0, asset_id: asset_key }],
                        source: None,
                    })),
                }
                Ok(Emit::mutations(mutations))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &RemodelingCommand) -> &'static str {
        command.command_id()
    }

    /// 🌉️ Host action id + JSON args → typed command. **Forward-fix, not a preserved behaviour**: the
    /// pre-migration `remodeling_ui` never implemented this at all, so every one of remodeling's own actions
    /// (the media drop zone, the layer toggles, every command-palette entry) fell through to the
    /// framework's reserved-action error and could only be reached by a direct `dispatch_typed` —
    /// i.e. the whole manifest surface was dead from the host's side. Arg keys are the camelCase ids
    /// declared in `🔖️Manifest`; each is read leniently (missing → the manifest's own default) because
    /// `effective_action_args` stages defaults before dispatch and select-typed args arrive as strings.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<RemodelingCommand, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &RemodelingCommand,
        doc: &ArtifactView<'_, RemodelingSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<RemodelingMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        let mut emit = command.dispatch(doc, cfg)?;
        if let Some(view) = _view_state {
            if let Some(mutation) = remodeling_window_config_mutation(command, cfg.window, view)? { emit.window_config_mutations.push(mutation); }
        }
        Ok(emit)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, RemodelingSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let scene = doc.snapshot;
        let labels = remodeling_labels(view_state);
        let built = match body_key {
            model::windows::model::REMODELING_PLAY_BODY_MAIN => model::windows::model::render(scene, &model::windows::model::config::current(cfg)),
            capture::windows::frames::REMODELING_PLAY_BODY_FRAMES => capture::windows::frames::render(scene, &capture::windows::frames::config::current(cfg)),
            analyze::windows::report::REMODELING_PLAY_BODY_REPORT => analyze::windows::report::render(scene, &analyze::windows::report::config::current(cfg)),
            media::REMODELING_PLAY_BODY_MEDIA => media::render(scene, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, media::REMODELING_PLAY_BODY_MEDIA)),
            document::REMODELING_PLAY_BODY_PIPELINE => document::render(scene, doc.tool_run(), view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, document::REMODELING_PLAY_BODY_PIPELINE)),
            results::REMODELING_PLAY_BODY_RESULTS => results::render(scene, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, results::REMODELING_PLAY_BODY_RESULTS)),
            parameters::REMODELING_PLAY_BODY_PARAMETERS => parameters::render(scene, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, parameters::REMODELING_PLAY_BODY_PARAMETERS)),
            calibration_panel::REMODELING_PLAY_BODY_CALIBRATION => calibration_panel::render(scene, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, calibration_panel::REMODELING_PLAY_BODY_CALIBRATION)),
            tracks::REMODELING_PLAY_BODY_TRACKS => tracks::render(scene, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, tracks::REMODELING_PLAY_BODY_TRACKS)),
            quality::REMODELING_PLAY_BODY_QC => quality::render(scene, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, quality::REMODELING_PLAY_BODY_QC)),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        };
        built.map(semio_framework_plugin::built_to_component_tree)
    }

    /// 👁️ Dynamic per-render window measures — the Model window's layer toggles must reflect the LIVE
    /// config, so they are supplied here rather than frozen into the manifest.
    fn window_measures(_doc: &ArtifactView<'_, RemodelingSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(model::windows::model::REMODELING_PLAY_WINDOW_MAIN.to_string(), model::windows::model::window_measures(&model::windows::model::config::current(cfg), remodeling_labels(view_state)))])
    }
}
//#endregion 🔖️RemodelingPlayApp
