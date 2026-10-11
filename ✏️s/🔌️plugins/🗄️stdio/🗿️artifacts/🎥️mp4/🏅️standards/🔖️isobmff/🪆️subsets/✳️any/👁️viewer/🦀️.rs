//! 👁️ `mp4` viewer (any) — the read-only counterpart of `✏️editor` for this
//! subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Mp4Viewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Mp4Viewer>` is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling `editor` module (`policyViewerPurityBreaches`).

use crate::standards::isobmff::subsets::any::schema::mutations::Mp4Mutation;
use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
use crate::viewer::mp4::modes::view;
use crate::viewer::mp4::modes::view::windows::main;
use crate::{MP4_DIALECT, STDIO_MP4_DOCUMENT_SCHEMA};
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ArtifactViewer;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::ViewEmit;
use semio_framework_plugin::Viewer;
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum Mp4ViewCommand {
    Noop,
}

impl protocol::OpBinary for Mp4ViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Mp4ViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Mp4Viewer;

impl ArtifactViewer for Mp4Viewer {
    type Snapshot = Mp4Snapshot;
    type Mutation = Mp4Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Mp4ViewCommand;

    const DIALECT: Dialect = MP4_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_MP4_DOCUMENT_SCHEMA;

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::ViewerApp<Self>>) -> Result<(), Fault> {
        registry.register(crate::standards::isobmff::subsets::any::io::playback::Mp4MediaExportJobFactory::<semio_framework_plugin::ViewerApp<Self>>::new(registry.controller_id()))
    }

    fn initial_snapshot() -> Self::Snapshot {
        Mp4Snapshot::default()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(semio_s_artifact_stdio_contract::media_export::playback_app_io::<crate::standards::isobmff::subsets::any::io::playback::Mp4PlaybackExport>())
    }

    fn build_media_export_job(request: semio_framework_plugin::ArtifactMediaExportJobRequest<semio_framework_plugin::ViewerApp<Self>>) -> Result<Option<semio_framework_plugin::ArtifactReservedToolJob>, Fault> {
        if request.port != semio_s_artifact_stdio_contract::media_export::PLAYBACK_PORT_ID || request.tool_id != semio_s_artifact_stdio_contract::media_export::PLAYBACK_TOOL_ID { return Ok(None); }
        Ok(Some(semio_framework_plugin::ArtifactReservedToolJob::new(crate::standards::isobmff::subsets::any::io::playback::Mp4PlaybackExportJob::new(request)?)))
    }

    fn build_snapshot_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactSnapshotDisposer<Self::Snapshot>>> {
        Some(Box::new(semio_s_artifact_stdio_contract::media_export::RetireOwnedSnapshotDisposer::<Mp4Snapshot>::default()))
    }

    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot, view_state.locale, semio_s_artifact_stdio_contract::media_transport_resource(doc, "s.stdio.mp4@isobmff/*#viewer")).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_mp4_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(MP4_DIALECT).document(["semio", "mp4"]).icon_id("play").mode_def(view::definition()).default_mode_id(view::MODE_ID).window_kind_def(main::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
