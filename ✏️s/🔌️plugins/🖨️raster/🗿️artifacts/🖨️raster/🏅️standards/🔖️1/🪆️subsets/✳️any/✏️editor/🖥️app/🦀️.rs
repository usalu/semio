//! 🖥️ The raster editor app: the unit app struct, its artifact-editor implementation, the body render and the proof carriers.

use super::*;
use semio_framework_plugin::ArtifactEditor;
use semio_s_artifact_raster_raster_editor_commands::RasterCommand;
use semio_s_artifact_raster_raster_editor_panels::panels;
use crate::config::{RasterConfig, RasterConfigMutation};
use crate::modes::edit::windows::{composite, navigator};
use crate::presence::{RasterPresence, RasterPresenceMutation};
use crate::terminology::raster_play_labels;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::RASTER_DOCUMENT_SCHEMA;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework::ToolExecutionContract;
use semio_framework_plugin::app::InteractionView;
use semio_framework_job::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactReservedToolInput;
use semio_framework_plugin::ArtifactReservedToolJob;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_artifact_reference::Dialect;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;
use semio_s_artifact_raster_raster_editor_commands::commands::flatten_layers;
use semio_s_artifact_raster_raster_editor_commands::commands::mask_from_selection;

pub(super) struct RasterCommandProofs;

impl RasterCommandProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<RasterPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.raster.raster@1/*#editor",
        artifact_schema: "raster.document",
        factory: "RasterRetainedCommandJobFactory",
        factory_type: RasterRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "addLayer", "dropLayerKind", "setLayerVisible", "toggleLayerVisible", "deleteLayer", "duplicateLayer", "patchLayer", "patchLayers", "moveLayer",
            "setBrushSize", "setBrushOpacity", "setCompositeViewport", "setCamera", "setCameraZoom", "setActiveExample", "flattenLayers", "mergeDown", "maskFromSelection", "setBrushColor", "setBrushHardness", "setPaintTarget", "setMaskValue", "setPixelSelection", "paintStroke", "fillRegion", "setFillTolerance", "applyFilter", "transformImage", "fillSelection"
        ]
    }

}

pub(super) struct RasterDownloadProofs;

impl RasterDownloadProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<RasterPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.raster.raster@1/*#editor",
        artifact_schema: "raster.document",
        factory: "RasterDownloadJobFactory",
        factory_type: media_export::RasterDownloadJobFactory,
        tools: {"exportPng" => ToolExecutionContract::resumable(4096,4096,1,33_554_432,2000,64,1)}
    }
}

//#region 🔖️RasterPlayApp
/// 🧪️ B1: unit struct — every former `RasterConfig` field now lives in
/// `crate::editor::raster::config::RasterConfig`, written through `RasterConfigMutation`s.
#[derive(Default)]
pub struct RasterPlayApp;

impl ArtifactEditor for RasterPlayApp {
    /// 📢️ The localized notices of the raster tool refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        semio_s_artifact_raster_raster_editor_commands::commands::paint_stroke::raster_fault_notices()
    }

    /// ⏳️ The layers panel renders the live operations with their Cancel controls; nothing else shows them.
    fn operation_progress_scope() -> semio_framework::kernel::UiDirtyScope {
        semio_framework::kernel::UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![crate::panels::document::RASTER_PLAY_BODY_LAYERS.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false }
    }

    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![semio_s_artifact_raster_raster_examples::art_raster_demo::source()]
    }
    type Snapshot = RasterSnapshot;
    type Mutation = RasterMutation;
    type Config = RasterConfig;
    type ConfigMutation = RasterConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = RasterPresence;
    type PresenceMutation = RasterPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = RasterCommand;

    const DIALECT: Dialect = semio_s_artifact_raster_raster_model::RASTER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = RASTER_DOCUMENT_SCHEMA;

    /// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields.
    /// Without it every live envelope load faults with `editor did not declare a loaded-parent child
    /// projection` before the decoded document can replace the store. `assets` declares no child slot
    /// (see `🧬️schema/📸️snapshot`), so the projection is honestly empty.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("raster.child-projection"), error.to_string()))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(RasterStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(store::mutation_apply_preparation_factory::<RasterConfig, RasterConfigMutation>())
    }

    fn bounded_first_step_tool_proofs()->Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        RasterCommandProofs::bounded_first_step_tool_proofs().into_iter().chain(RasterDownloadProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(RasterRetainedCommandJobFactory::new(&controller_id))?;
        registry.register(media_export::RasterMediaExportJobFactory::new(&controller_id))?;
        registry.register(media_export::RasterDownloadJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if request.tool_id=="exportPng" {return media_export::build_download_job(request);}
        if !RASTER_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "Raster command does not match its exact registered tool"));
        }
        if raster_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("raster.document.too-large"), "the raster document exceeds the retained command's work bound"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = if tool_id == "paintStroke" {
            Box::new(semio_s_artifact_raster_raster_editor_commands::commands::paint_stroke::PaintStrokeWork::default())
        } else if tool_id == "flattenLayers" {
            Box::new(flatten_layers::LayerBakeWork::<false>::default())
        } else if tool_id == "mergeDown" {
            Box::new(flatten_layers::LayerBakeWork::<true>::default())
        } else if tool_id == "maskFromSelection" {
            Box::new(mask_from_selection::MaskFromSelectionWork::default())
        } else {
            Box::new(BoundedArtifactCommandWork::new(tool_id, raster_retained_reduce, raster_retained_extent))
        };
        let operation_context = AppOperationContext {
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
                operation: operation_context,
                completion: request.completion,
            },
            RasterCommand::command_id,
            RASTER_RETAINED_RAW_BYTES,
            RASTER_RETAINED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(semio_s_artifact_raster_raster_host::raster_envelope_decode_owner_bundle())
    }

    fn document_store_owners_source_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_artifact_raster_raster_host::raster_document_store_owners_source_demands()
    }

    fn build_document_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(semio_s_artifact_raster_raster_host::raster_document_store_owners_admission(grant))
    }

    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_s_artifact_raster_raster_host::raster_document_store_initialization_job(envelope, operation, generation, actor))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    /// 🧹️ The config and draft lanes' owner catalogs are the framework's bounded defaults; their
    /// disposers close through the matching bounded store disposers (forms precedent) — without them
    /// a closing instance faults `interactive-job.close-owned-disposer-missing … config-store`.
    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    /// 👥️ Presence/transient lanes: raster's `RasterPresence` is plain inline data (its own
    /// one-turn retirement in `👥️presence/🦀️.rs`), the transient lane is `NoTransient` — the same
    /// rows forms/process3d declare.
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(crate::presence::raster_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🫧️ The Composite window's transient partition — where a streamed stroke lives between its dispatches.
    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        composite::transient::register(registry)
    }

    /// 📨️ Every host event ends the window's open stroke with zero trace under the reason the tool records: a blur
    /// `blur`, a lost pointer capture `captureLost`, a utility switch or a closing window `retired`, an opened history
    /// edit `frozen`. A remote edit (`BaseMoved`) keeps the stroke: its leaf names points and brush and repaints on any
    /// base. A window with no stroke in flight takes no write.
    fn host_event(event: &semio_framework_plugin::HostEvent) -> Option<RasterCommand> {
        use semio_framework_plugin::HostEvent;
        use semio_framework_tool_machine::ToolAbortReason;
        let reason = match event {
            HostEvent::WindowBlurred { .. } => ToolAbortReason::Blur,
            HostEvent::PointerCaptureLost { .. } => ToolAbortReason::CaptureLost,
            HostEvent::UtilityChanged { .. } | HostEvent::Retiring { .. } => ToolAbortReason::Retired,
            HostEvent::TimeTravelFrozen { .. } => ToolAbortReason::Frozen,
            HostEvent::BaseMoved { .. } => return None,
        };
        Some(RasterCommand::PaintStroke(semio_s_artifact_raster_raster_editor_commands::commands::paint_stroke::PaintStroke { layer_id: String::new(), tool: String::new(), xs: Vec::new(), ys: Vec::new(), phase: Some("abort".to_string()), reason: Some(reason.as_str().to_string()), gesture: None }))
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::config::schema::app_schema_descriptor())
    }

    /// 📄️ Boots on the constant EMPTY shell `empty_raster_snapshot()` (zero layers, zero assets) —
    /// NOT the `📚️examples/🎬️demo` carrier and not even `empty_raster_document()`'s one Background
    /// layer. The document is event-sourced: the initial snapshot is the constant every replica
    /// agrees on, and the shell replays `setActiveExample demo` on every boot so the demo lands
    /// through the bounded `set-active-example` mutations (the same document flow block2d/forms
    /// use). Found at the first react boots of ticket 26/09/05/RASTER-PLUGIN-END-TO-END
    /// (2026-09-16).
    fn initial_snapshot() -> RasterSnapshot {
        semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(raster_io())
    }

    /// 📤️ Captures image output in a retained operation with framework-owned cancellation and disposal.
    fn build_media_export_job(request:&mut Option<semio_framework_plugin::ArtifactMediaExportJobRequest<EditorApp<Self>>>,body:&mut Option<Box<dyn semio_framework_plugin::ArtifactReservedJob+Send>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if request.as_ref().is_some_and(|request|request.port!="image:out"||request.tool_id!=media_export::TOOL_ID){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"raster media producer requires its exact registered output port").into())}
        media_export::RasterImageExportJob::admit_original(request,body,cx)
    }

    fn interaction_topology(doc:&ArtifactView<'_,RasterSnapshot>,_cfg:&ConfigView<'_,RasterConfig>)->Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError> {
        Ok(semio_framework_plugin::InteractionTopology {domains:std::collections::BTreeMap::from([(RASTER_INTERACTION_DOMAIN.into(),layer_selection::layer_topology(&doc.snapshot.layers))])})
    }

    /// 📦️ Batch serializers for structured image media and editable artifact packs.
    fn export_media(port: &str, doc: &ArtifactView<'_, RasterSnapshot>) -> Result<Media, MediaError> {
        match port {
            "image:out" => raster_composite_media(doc.snapshot),
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `image:in` inserts the incoming raster media as a new composited layer + embedded asset —
    /// two real semantic mutations (`add-layer-asset` then `create-layer`, in dependency order)
    /// bundled in one `Emit`, never a whole-document replace (`RasterMutation` has no such variant
    /// anymore). Falls through to the inherited `document:in` load default for any other port.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, RasterSnapshot>) -> Result<Emit<RasterMutation, RasterConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "image:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json: png_base64, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "image:in only accepts a Structured (base64 PNG) payload".into()));
        };
        let (asset_id, asset, layer) = semio_s_artifact_raster_raster_io::raster_image_layer_and_asset(png_base64).map_err(|error| MediaError::Payload(port.to_string(), error))?;
        Ok(Emit::mutations(vec![
            RasterMutation::AddLayerAsset(semio_s_artifact_raster_raster_mutations_pixels::add_layer_asset::mutation::AddLayerAsset { asset_id, asset }),
            RasterMutation::CreateLayer(semio_s_artifact_raster_raster_mutations_layers::create_layer::mutation::CreateLayer { parent_id: None, index: doc.snapshot.layers.len(), layer: Box::new(layer) }),
        ]))
    }

    /// 🎞️ The ONE reserved route raster owns: `import-media`. Every inbound media delivery goes
    /// through `dispatch_import_media` → `build_artifact_reserved_media_job`, which fails closed
    /// unless this builder hands back a concrete resumable importer — see [`RasterImportJob`].
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if request.tool_id.as_str() != RASTER_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(Fault::from("raster import-media admits a decoded media value, never a wire payload"));
        }
        let ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(Fault::from("raster import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(ArtifactReservedToolJob::new(RasterImportJob::new(request, port, media))))
    }

    fn command_id(command: &RasterCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ See `args_bridge` — without this override the trait default refuses every shell action.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &RasterCommand,
        doc: &ArtifactView<'_, RasterSnapshot>,
        cfg: &ConfigView<'_, RasterConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<RasterMutation, RasterConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn window_measures(_doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(composite::RASTER_PLAY_WINDOW_COMPOSITE.into(), composite::window_measures(cfg.snapshot, raster_play_labels(view_state)))])
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_raster_body(body_key, doc, None, cfg, view_state, &[], None)
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, RasterSnapshot>,
        cfg: &ConfigView<'_, RasterConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        let preview = match body_key {
            composite::RASTER_PLAY_BODY_COMPOSITE => transient.window::<composite::transient::RasterCompositeWindowTransientOwner>().and_then(|window| semio_s_artifact_raster_raster_editor_commands::commands::paint_stroke::raster_stroke_preview(doc.snapshot, window)),
            _ => None,
        };
        let rendered = render_raster_body(body_key, doc, preview.as_ref(), cfg, view_state, &interaction.selection(RASTER_INTERACTION_DOMAIN).ids, interaction.hover(RASTER_INTERACTION_DOMAIN, "pointer").ids.first().map(String::as_str));
        if let Some(preview) = preview {
            semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(preview);
        }
        rendered

        })();
        rendered.map(Some).map_err(Into::into)
        }
}

/// 🖼️ Projects live selection into both paint windows without a second selection store; the Composite window paints
/// `preview` (the committed document with its stroke in flight) when it holds one.
pub(super) fn render_raster_body(body_key: &str, doc: &ArtifactView<'_, RasterSnapshot>, preview: Option<&RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>, view_state: &semio_framework_plugin::ViewModel, selected_ids: &[String], hovered_id: Option<&str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = preview.unwrap_or(doc.snapshot);
        let config = cfg.snapshot;
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or("selectMarquee");
        let labels = raster_play_labels(view_state);
        let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
        let node = match body_key {
            composite::RASTER_PLAY_BODY_COMPOSITE => composite::render(document, config, active_utility, selected_ids, hovered_id)?,
            navigator::RASTER_PLAY_BODY_NAVIGATOR => navigator::render(document, config, active_utility, selected_ids, hovered_id)?,
            crate::panels::document::RASTER_PLAY_BODY_LAYERS => crate::panels::document::render(document, config, labels, &windows)?,
            crate::panels::masks::RASTER_PLAY_BODY_MASKS => crate::panels::masks::render(document, config, labels, &windows)?,
            crate::panels::catalogue::RASTER_PLAY_BODY_CATALOGUE => crate::panels::catalogue::render(labels, &windows)?,
            crate::panels::inspection::RASTER_PLAY_BODY_PROPERTIES => crate::panels::inspection::render(document, config, selected_ids, labels)?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "raster unknown-body label admission failed"))?,
        };
        let node=if body_key==crate::panels::document::RASTER_PLAY_BODY_LAYERS&&!doc.operations().is_empty() {
            use semio_framework_plugin::{Buildable,HasBase,HasChildren};
            let progress=semio_framework_plugin::app::operation_progress::operation_progress_controls(doc.operations(),RASTER_PLAY_CONTROLLER_ID,view_state.locale)?;
            semio_framework_ui_contract::column().try_id("raster.layers.with-progress").map_err(|_|semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity","raster progress container"))?.try_children([progress,node]).map_err(|_|semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity","raster progress children"))?.try_build().map_err(|_|semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity","raster progress build"))?
        }else{node};
        Ok(semio_framework_plugin::built_to_component_tree(node))
}
