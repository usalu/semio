//! 🖥️ The raster editor app: the unit app struct, its artifact-editor implementation, the body render and the proof carriers.

use super::*;
#[path="🌱️ephemeral/🦀️.rs"]
pub mod original_ephemeral;
use semio_plugin::ArtifactEditor;
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
use semio_plugin::app::InteractionView;
use semio_framework_job::{RetainedCloneGrant, RetainedCloneProgress};
use semio_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_plugin::AppOperationContext;
use semio_plugin::ArtifactOwnedToolJobRequest;
use semio_plugin::ArtifactReservedToolInput;
use semio_plugin::ArtifactReservedToolJobRequest;
use semio_plugin::ArtifactToolFactoryRegistry;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_framework_artifact_reference::Dialect;
use semio_plugin::DraftView;
use semio_plugin::EditorApp;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_ui_locale::Label;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::Media;
use semio_plugin::MediaClass;
use semio_plugin::MediaError;
use semio_plugin::MediaForm;
use semio_plugin::MediaPayload;
use semio_plugin::MediaType;
use semio_plugin::NoDraft;
use semio_plugin::NoDraftMutation;
use semio_plugin::WindowMeasure;
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;
use semio_s_artifact_raster_raster_editor_commands::commands::flatten_layers;
use semio_s_artifact_raster_raster_editor_commands::commands::mask_from_selection;

pub(super) struct RasterCommandProofs;

impl RasterCommandProofs {
    semio_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<RasterPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.raster.raster@1/*#editor",
        artifact_schema: "raster.document",
        factory: "RasterRetainedCommandJobFactory",
        factory_type: RasterRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "addLayer", "dropLayerKind", "setLayerVisible", "toggleLayerVisible", "deleteLayer", "duplicateLayer", "patchLayer", "patchLayers", "moveLayer",
            "setBrushSize", "setBrushOpacity", "setCompositeViewport", "setCamera", "setCameraZoom", "setActiveExample", "setBrushColor", "setBrushHardness", "setPaintTarget", "setMaskValue", "setFillTolerance", "setFilterKind", "setFilterAmount", "setImageGrid"
        ]
    }

}

pub(super) struct RasterDownloadProofs;

pub(super) struct RasterSelectionProofs;
pub(super) struct RasterMaskSelectionProofs;
pub(super) struct RasterLayerBakeProofs;
pub(super) struct RasterPixelEditProofs;
impl RasterPixelEditProofs{
 semio_plugin::bounded_first_step_tool_proofs!{
  owner:EditorApp<RasterPlayApp>,
  owner_file:"✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
  controller:"s.raster.raster@1/*#editor",
  artifact_schema:"raster.document",
  factory:"RasterPixelEditJobFactory",
  factory_type:RasterPixelEditJobFactory,
  tools:{
   "paintStroke"=>ToolExecutionContract::resumable(RASTER_PIXEL_EDIT_RAW_BYTES,RASTER_PIXEL_EDIT_DECODED_ITEMS,1024,RASTER_PIXEL_EDIT_OUTPUT_BYTES,7500,64,1),
   "fillRegion"=>ToolExecutionContract::resumable(RASTER_PIXEL_EDIT_RAW_BYTES,RASTER_PIXEL_EDIT_DECODED_ITEMS,1024,RASTER_PIXEL_EDIT_OUTPUT_BYTES,7500,64,1),
   "applyFilter"=>ToolExecutionContract::resumable(RASTER_PIXEL_EDIT_RAW_BYTES,RASTER_PIXEL_EDIT_DECODED_ITEMS,1024,RASTER_PIXEL_EDIT_OUTPUT_BYTES,7500,64,1),
   "transformImage"=>ToolExecutionContract::resumable(RASTER_PIXEL_EDIT_RAW_BYTES,RASTER_PIXEL_EDIT_DECODED_ITEMS,1024,RASTER_PIXEL_EDIT_OUTPUT_BYTES,7500,64,1),
   "fillSelection"=>ToolExecutionContract::resumable(RASTER_PIXEL_EDIT_RAW_BYTES,RASTER_PIXEL_EDIT_DECODED_ITEMS,1024,RASTER_PIXEL_EDIT_OUTPUT_BYTES,7500,64,1)
  }
 }
}
impl RasterLayerBakeProofs {
    semio_plugin::bounded_first_step_tool_proofs! {
        owner:EditorApp<RasterPlayApp>,
        owner_file:"✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller:"s.raster.raster@1/*#editor",
        artifact_schema:"raster.document",
        factory:"RasterLayerBakeJobFactory",
        factory_type:RasterLayerBakeJobFactory,
        tools:{"flattenLayers"=>ToolExecutionContract::resumable(RASTER_RETAINED_RAW_BYTES,4096,1024,RASTER_LAYER_BAKE_OUTPUT_BYTES,7500,64,1),"mergeDown"=>ToolExecutionContract::resumable(RASTER_RETAINED_RAW_BYTES,4096,1024,RASTER_LAYER_BAKE_OUTPUT_BYTES,7500,64,1)}
    }
}
impl RasterMaskSelectionProofs {
    semio_plugin::bounded_first_step_tool_proofs! {
        owner:EditorApp<RasterPlayApp>,
        owner_file:"✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller:"s.raster.raster@1/*#editor",
        artifact_schema:"raster.document",
        factory:"RasterMaskSelectionJobFactory",
        factory_type:RasterMaskSelectionJobFactory,
        tools:{"maskFromSelection"=>ToolExecutionContract::resumable(RASTER_MASK_SELECTION_RAW_BYTES,4096,1,RASTER_MASK_SELECTION_OUTPUT_BYTES,7500,64,1)}
    }
}
impl RasterSelectionProofs {
    semio_plugin::bounded_first_step_tool_proofs! {
        owner:EditorApp<RasterPlayApp>,
        owner_file:"✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller:"s.raster.raster@1/*#editor",
        artifact_schema:"raster.document",
        factory:"RasterSelectPixelsJobFactory",
        factory_type:RasterSelectPixelsJobFactory,
        tools:{"selectPixels"=>ToolExecutionContract::resumable(RASTER_SELECT_PIXELS_RAW_BYTES,4096,1024,RASTER_SELECT_PIXELS_OUTPUT_BYTES,7500,64,1)}
    }
}

impl RasterDownloadProofs {
    semio_plugin::bounded_first_step_tool_proofs! {
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

#[cfg(test)]
#[path="🧪️host-event/🦀️.rs"]
mod host_event_birth_tests;

impl ArtifactEditor for RasterPlayApp {
    type Members=semio_s_artifact_stdio_semio::SemioMembers;
    /// 📢️ The localized notices of the raster tool refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        semio_s_artifact_raster_raster_editor_commands::commands::paint_stroke::raster_fault_notices()
    }

    /// ⏳️ The layers panel renders the live operations with their Cancel controls; nothing else shows them.
    fn operation_progress_scope() -> semio_framework::kernel::UiDirtyScope {
        semio_framework::kernel::UiDirtyScope::Partial { window_bodies: vec![composite::RASTER_PLAY_BODY_COMPOSITE.to_string()], panel_bodies: vec![crate::panels::document::RASTER_PLAY_BODY_LAYERS.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false }
    }

    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_plugin::ExampleSource> {
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
    type Transient = semio_plugin::NoTransient;
    type TransientMutation = semio_plugin::NoTransientMutation;

    type Command = RasterCommand;

    const DIALECT: Dialect = semio_s_artifact_raster_raster_model::RASTER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = RASTER_DOCUMENT_SCHEMA;

    /// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields.
    /// Without it every live envelope load faults with `editor did not declare a loaded-parent child
    /// projection` before the decoded document can replace the store. `assets` declares no child slot
    /// (see `🧬️schema/📸️snapshot`), so the projection is honestly empty.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("raster.child-projection"), error.to_string()))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(RasterStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(store::snapshot_clone_preparation::config_apply_preparation_factory::<RasterConfig, RasterConfigMutation>())
    }

    fn bounded_first_step_tool_proofs()->Vec<semio_plugin::ArtifactBoundedFirstStepProof> {
        RasterCommandProofs::bounded_first_step_tool_proofs().into_iter().chain(RasterPixelEditProofs::bounded_first_step_tool_proofs()).chain(RasterDownloadProofs::bounded_first_step_tool_proofs()).chain(RasterSelectionProofs::bounded_first_step_tool_proofs()).chain(RasterMaskSelectionProofs::bounded_first_step_tool_proofs()).chain(RasterLayerBakeProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(RasterRetainedCommandJobFactory::new(&controller_id))?;
        registry.register(RasterPixelEditJobFactory::new(&controller_id))?;
        registry.register(RasterSelectPixelsJobFactory::new(&controller_id))?;
        registry.register(RasterMaskSelectionJobFactory::new(&controller_id))?;
        registry.register(RasterLayerBakeJobFactory::new(&controller_id))?;
        registry.register(media_export::RasterMediaExportJobFactory::new(&controller_id))?;
        registry.register(media_export::RasterDownloadJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_plugin::ErasedToolJob>, cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if original.tool_id=="exportPng" {return media_export::build_download_job(request, output, cx);}
        if original.tool_id=="selectPixels"{
            if !matches!(original.command,RasterCommand::SelectPixels(_)){return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated,"selection command does not match its registered tool").into());}
            return semio_plugin::retained_command::producer::admit_retained_tool_producer(request,output,cx,RasterCommand::command_id,RASTER_SELECT_PIXELS_RAW_BYTES,RASTER_RETAINED_WORK_ITEMS,false,|_|semio_s_artifact_raster_raster_editor_commands::commands::select_pixels::SelectPixelsWork::default());
        }
        if original.tool_id=="maskFromSelection"{
            if !matches!(original.command,RasterCommand::MaskFromSelection(_)){return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated,"mask command does not match its registered tool").into());}
            return semio_plugin::retained_command::producer::admit_retained_tool_producer(request,output,cx,RasterCommand::command_id,RASTER_RETAINED_RAW_BYTES,RASTER_RETAINED_WORK_ITEMS,false,|_|mask_from_selection::MaskFromSelectionWork::default());
        }
        if RASTER_PIXEL_EDIT_TOOL_IDS.contains(&original.tool_id.as_str()){
            if original.command.command_id()!=original.tool_id{return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated,"pixel command does not match its registered tool").into())}
            let tool=original.command.command_id();
            if tool=="paintStroke"{
                return semio_plugin::retained_command::producer::admit_retained_tool_producer(request,output,cx,RasterCommand::command_id,RASTER_PIXEL_EDIT_RAW_BYTES,RASTER_RETAINED_WORK_ITEMS,false,|_|semio_s_artifact_raster_raster_editor_commands::commands::paint_stroke::PaintStrokeWork::default());
            }
            return semio_plugin::retained_command::producer::admit_retained_tool_producer(request,output,cx,RasterCommand::command_id,RASTER_PIXEL_EDIT_RAW_BYTES,RASTER_RETAINED_WORK_ITEMS,false,|_|semio_s_artifact_raster_raster_editor_commands::commands::pixel_edit::PixelLeafWork::new(tool));
        }
        if !raster_retained_accepts(&original.tool_id) {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "Raster command does not match its exact registered tool").into());
        }
        if raster_retained_extent(&original.command, &original.snapshot, &original.interaction_state) != Some(1) {
            return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "the raster document exceeds the retained command's work bound").into());
        }
        let tool_id = original.command.command_id();
        if tool_id == "flattenLayers" {
            return semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, RasterCommand::command_id, RASTER_RETAINED_RAW_BYTES, RASTER_RETAINED_WORK_ITEMS, false, |_| flatten_layers::LayerBakeWork::<false>::default());
        }
        if tool_id == "mergeDown" {
            return semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, RasterCommand::command_id, RASTER_RETAINED_RAW_BYTES, RASTER_RETAINED_WORK_ITEMS, false, |_| flatten_layers::LayerBakeWork::<true>::default());
        }
        semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, RasterCommand::command_id, RASTER_RETAINED_RAW_BYTES, RASTER_RETAINED_WORK_ITEMS, false, |_| BoundedArtifactCommandWork::new(tool_id, raster_retained_reduce, raster_retained_extent))
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

    fn build_document_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    /// 🧹️ The config and draft lanes' owner catalogs are the framework's bounded defaults; their
    /// disposers close through the matching bounded store disposers (forms precedent) — without them
    /// a closing instance faults `interactive-job.close-owned-disposer-missing … config-store`.
    fn build_config_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_plugin::no_draft_store_disposer())
    }

    /// 👥️ Presence/transient lanes: raster's `RasterPresence` is plain inline data (its own
    /// one-turn retirement in `👥️presence/🦀️.rs`), the transient lane is `NoTransient` — the same
    /// rows forms/process3d declare.
    fn build_presence_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(crate::presence::raster_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🫧️ The Composite window's transient partition — where a streamed stroke lives between its dispatches.
    fn register_window_transient_owners(registry: &mut semio_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        composite::transient::register(registry)
    }

    /// 📨️ Every host event ends the window's open stroke with zero trace under the reason the tool records: a blur
    /// `blur`, a lost pointer capture `captureLost`, a utility switch or a closing window `retired`, an opened history
    /// edit `frozen`. A remote edit (`BaseMoved`) keeps the stroke: its leaf names points and brush and repaints on any
    /// base. A window with no stroke in flight takes no write.
    fn host_event_demands(event: &semio_plugin::HostEvent) -> Option<semio_framework_value::RetirementDemand> {
        let bytes=match event{semio_plugin::HostEvent::WindowBlurred{..}=>9,semio_plugin::HostEvent::PointerCaptureLost{..}=>16,semio_plugin::HostEvent::UtilityChanged{..}|semio_plugin::HostEvent::Retiring{..}=>12,semio_plugin::HostEvent::TimeTravelFrozen{..}=>11,semio_plugin::HostEvent::BaseMoved{..}=>16};
        let header=std::mem::size_of::<RasterCommand>();Some(semio_framework_value::RetirementDemand{copy_bytes:header.saturating_mul(2).saturating_add(bytes),capacity_bytes:header.saturating_add(bytes),depth:1,..Default::default()})
    }

    fn host_event(event: &semio_plugin::HostEvent) -> Option<RasterCommand> {
        use semio_plugin::HostEvent;
        use semio_framework_tool_machine::ToolAbortReason;
        let reason = match event {
            HostEvent::WindowBlurred { .. } => ToolAbortReason::Blur,
            HostEvent::PointerCaptureLost { .. } => ToolAbortReason::CaptureLost,
            HostEvent::UtilityChanged { .. } | HostEvent::Retiring { .. } => ToolAbortReason::Retired,
            HostEvent::TimeTravelFrozen { .. } => ToolAbortReason::Frozen,
            HostEvent::BaseMoved { .. } => return Some(RasterCommand::PaintStroke(semio_s_artifact_raster_raster_editor_commands::commands::paint_stroke::PaintStroke{phase:Some("refresh".into()),reason:Some("baseMoved".into()),..Default::default()})),
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
    fn reference_birth()->Option<fn()->Result<RasterSnapshot,semio_plugin::Fault>>{Some(||{
        Ok(semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot())
    })}

    fn io() -> Option<semio_plugin::AppIo> {
        Some(raster_io())
    }

    /// 📤️ Captures image output in a retained operation with framework-owned cancellation and disposal.
    fn build_media_export_job(request:&mut Option<semio_plugin::ArtifactMediaExportJobRequest<EditorApp<Self>>>,body:&mut Option<Box<dyn semio_plugin::ArtifactReservedJob+Send>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if request.as_ref().is_some_and(|request|request.port!="image:out"||request.tool_id!=media_export::TOOL_ID){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"raster media producer requires its exact registered output port").into())}
        media_export::RasterImageExportJob::admit_original(request,body,cx)
    }

    fn interaction_topology(doc:&ArtifactView<'_,RasterSnapshot>,_cfg:&ConfigView<'_,RasterConfig>)->Result<semio_plugin::InteractionTopology, semio_framework_value::ValueError> {
        Ok(semio_plugin::InteractionTopology {domains:std::collections::BTreeMap::from([(RASTER_INTERACTION_DOMAIN.into(),layer_selection::layer_topology(&doc.snapshot.layers))])})
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

    /// 🖼️ The original workflow text wire contains one raw PNG Base64 payload.
    fn accepts_original_structured_media(port:&str,schema:&str,media_type:&MediaType)->bool{port=="image:in"&&schema=="2d.image"&&media_type.class==MediaClass::TwoD&&media_type.form==MediaForm::Raster}

    /// 🎞️ Media admission belongs to the reserved retained importer.
    fn import_media(_port:&str,_media:&Media,_doc:&ArtifactView<'_,RasterSnapshot>)->Result<Emit<RasterMutation,RasterConfigMutation,Self::DraftMutation>,MediaError>{Err(MediaError::NotImplemented)}

    /// 🎞️ The ONE reserved route raster owns: `import-media`. Every inbound media delivery goes
    /// through `dispatch_import_media` → `build_artifact_reserved_media_job`, which fails closed
    /// unless this builder hands back a concrete resumable importer — see [`RasterImportJob`].
    fn build_reserved_tool_job(request:&mut Option<ArtifactReservedToolJobRequest<EditorApp<Self>>>,output:&mut Option<Box<dyn semio_plugin::ArtifactReservedJob+Send>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if output.is_some(){return semio_plugin::tool_contract::admit_reserved_work::<EditorApp<Self>,RasterImportJob>(request,output,cx)}
        let original=request.as_ref().ok_or_else(||semio_framework_value::ValueError::invariant("Raster original reserved request is absent"))?;
        if original.tool_id.as_str()!=RASTER_IMPORT_TOOL_ID{return Ok(false)}
        if !original.raw_wire.is_empty(){return Err(semio_framework_value::ValueError::invariant("Raster reserved import requires decoded media").into())}
        if !matches!(original.input.as_ref(),Some(ArtifactReservedToolInput::Media{..})){return Err(semio_framework_value::ValueError::invariant("Raster reserved import media is absent").into())}
        semio_plugin::tool_contract::admit_reserved_work::<EditorApp<Self>,RasterImportJob>(request,output,cx)
    }

    fn command_id(command: &RasterCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ See `args_bridge` — without this override the trait default refuses every shell action.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        if action=="selectPixels"{
            use semio_s_artifact_raster_raster_editor_commands::commands::select_pixels::SelectPixels;
            return SelectPixels::from_original_action_value(args).map(RasterCommand::SelectPixels).map_err(Fault::from);
        }
        if action=="maskFromSelection"{
            use semio_s_artifact_raster_raster_editor_commands::commands::mask_from_selection::MaskFromSelection;
            return MaskFromSelection::from_original_action_value(args).map(RasterCommand::MaskFromSelection).map_err(Fault::from);
        }
        let args = args.as_ref();
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &RasterCommand,
        doc: &ArtifactView<'_, RasterSnapshot>,
        cfg: &ConfigView<'_, RasterConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<RasterMutation, RasterConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn window_measures(_doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>, view_state: &semio_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(composite::RASTER_PLAY_WINDOW_COMPOSITE.into(), composite::window_measures(cfg.snapshot, raster_play_labels(view_state)))])
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        render_raster_body(body_key, doc, None, cfg, view_state, &[], None)
    }

    fn render_with_request_context(
        _owner: &semio_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, RasterSnapshot>,
        cfg: &ConfigView<'_, RasterConfig>,
        view_state: &semio_plugin::ViewModel,
        transient: &semio_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        _cx:&mut semio_plugin::StepContext<'_>,
    ) -> semio_plugin::OriginalRenderResult<semio_plugin::ComponentTree> {
        let preview=if body_key==composite::RASTER_PLAY_BODY_COMPOSITE{doc.render_operation_ref().and_then(|operation|transient.window::<composite::transient::RasterCompositeWindowTransientOwner>().and_then(|window|composite::transient::cached_stroke_preview(window,&operation.canonical_base_revision)))}else{None};
        render_raster_body(body_key,doc,preview,cfg,view_state,&interaction.selection(RASTER_INTERACTION_DOMAIN).ids,interaction.hover(RASTER_INTERACTION_DOMAIN,"pointer").ids.first().map(String::as_str)).map(Some).map_err(Into::into)
        }
}

/// 🖼️ Projects live selection into both paint windows without a second selection store; the Composite window paints
/// `preview` (the committed document with its stroke in flight) when it holds one.
pub(super) fn render_raster_body(body_key: &str, doc: &ArtifactView<'_, RasterSnapshot>, preview: Option<&RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>, view_state: &semio_plugin::ViewModel, selected_ids: &[String], hovered_id: Option<&str>) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        let document = preview.unwrap_or(doc.snapshot);
        let config = cfg.snapshot;
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or("selectMarquee");
        let labels = raster_play_labels(view_state);
        let windows = semio_plugin::TreeWindows::for_body(view_state, body_key);
        let node = match body_key {
            composite::RASTER_PLAY_BODY_COMPOSITE => composite::render(document, config, active_utility, selected_ids, hovered_id, selection_cancellation(doc.operations(),view_state.window_id.as_deref()))?,
            navigator::RASTER_PLAY_BODY_NAVIGATOR => navigator::render(document, config, active_utility, selected_ids, hovered_id)?,
            crate::panels::document::RASTER_PLAY_BODY_LAYERS => crate::panels::document::render(document, config, labels, &windows)?,
            crate::panels::masks::RASTER_PLAY_BODY_MASKS => crate::panels::masks::render(document, config, labels, &windows)?,
            crate::panels::catalogue::RASTER_PLAY_BODY_CATALOGUE => crate::panels::catalogue::render(labels, &windows)?,
            crate::panels::inspection::RASTER_PLAY_BODY_PROPERTIES => crate::panels::inspection::render(document, config, selected_ids, labels)?,
            _ => semio_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_plugin::PluginAssemblyError::new("ui.fixed-capacity", "raster unknown-body label admission failed"))?,
        };
        let node=if body_key==crate::panels::document::RASTER_PLAY_BODY_LAYERS&&!doc.operations().is_empty() {
            use semio_plugin::{Buildable,HasBase,HasChildren};
            let progress=semio_plugin::app::operation_progress::operation_progress_controls(doc.operations(),RASTER_PLAY_CONTROLLER_ID,view_state.locale)?;
            semio_ui_contract::column().try_id("raster.layers.with-progress").map_err(|_|semio_plugin::PluginAssemblyError::new("ui.fixed-capacity","raster progress container"))?.try_children([progress,node]).map_err(|_|semio_plugin::PluginAssemblyError::new("ui.fixed-capacity","raster progress children"))?.try_build().map_err(|_|semio_plugin::PluginAssemblyError::new("ui.fixed-capacity","raster progress build"))?
        }else{node};
        Ok(semio_plugin::built_to_component_tree(node))
}

/// ⎋️ Only one exact admitted selection owner is offered to its original window; the host checks actor authority.
fn selection_cancellation(operations:&[semio_plugin::ArtifactOperationProgress],window:Option<&str>)->Option<semio_plugin::Paint2dSelectionCancellation>{
    let window=window?;
    let mut matching=operations.iter().filter(|operation|operation.tool_id=="selectPixels"&&!operation.cancelling&&operation.window_id.as_deref()==Some(window));
    let operation=matching.next()?;
    if matching.next().is_some(){return None;}
    Some(semio_plugin::Paint2dSelectionCancellation{actor:operation.actor.clone(),app_instance_id:operation.app_instance_id,window_id:window.into(),operation_id:format!("{:016x}",operation.operation_id),generation:format!("{:016x}",operation.generation)})
}
