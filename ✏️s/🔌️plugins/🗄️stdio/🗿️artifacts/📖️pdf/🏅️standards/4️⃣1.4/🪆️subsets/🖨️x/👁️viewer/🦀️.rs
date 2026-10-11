//! 👁️ PDF/X Document (1.4) viewer -- the read-only counterpart of the mutation-capable surface for this
//! subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Pdf14XViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` -- `ViewerApp<Pdf14XViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! mutation. Must not import anything from the sibling mutation-capable surface (viewer purity).

use crate::viewer::pdf14x::modes::view;
use crate::viewer::pdf14x::modes::view::windows::main;
use crate::standards::v1_4::subsets::base::schema::{mutations::PdfMutation, snapshot::PdfSnapshot};
use crate::{PDF_ARTIFACT_SCHEMA_ID, STDIO_PDF_DOCUMENT_SCHEMA};
use {semio_framework_plugin::built_to_component_tree,semio_framework_plugin::ArtifactView,semio_framework_plugin::ArtifactViewer,semio_framework_plugin::ComponentTree,semio_framework_plugin::ConfigView,semio_framework_artifact_reference::Dialect,semio_framework_plugin::Fault,semio_framework_plugin::NoConfig,semio_framework_plugin::NoConfigMutation,semio_framework_plugin::NoPresence,semio_framework_plugin::NoPresenceMutation,semio_framework_plugin::NoTransient,semio_framework_plugin::NoTransientMutation,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::ViewEmit,semio_framework_plugin::Viewer};

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract: this file's own surface-id
/// coordinate -- `s.stdio.pdf@1.4/x` -- measured directly against `PDF_ARTIFACT_SCHEMA_ID`
/// and this file's own on-disk standard/subset location. Own copy -- never imported from the mutation-capable surface, per this file's own purity rule.
pub const PDF14X_DIALECT: Dialect = Dialect { artifact_kind: PDF_ARTIFACT_SCHEMA_ID, standard: StandardId("1.4"), subset: SubsetId("x") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Pdf14XViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Pdf14XViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Pdf14XViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Pdf14XViewer;

impl ArtifactViewer for Pdf14XViewer {
    type Snapshot = PdfSnapshot;
    type Mutation = PdfMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Pdf14XViewCommand;

    const DIALECT: Dialect = PDF14X_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PDF_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> PdfSnapshot {
        PdfSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `Noop` variant never carries a config change. Kept as a
    /// real dispatch (not `unreachable!()`) so a future view-only action is a pure addition.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &semio_framework_2d::compute::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render_windowed(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY)).map(built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_pdf14_x_viewer() -> semio_framework_plugin::AppDefinition {
    let builder = Viewer::builder(PDF14X_DIALECT);
    let builder = builder.document(["stdio", "pdf", "1.4", "x"]);
    let builder = builder.icon_id("file-text");
    let builder = builder.mode_def(view::definition());
    let builder = builder.default_mode_id(view::PDF14X_VIEW_MODE_ID);
    let builder = builder.window_kind_def(main::definition());
    let builder = builder.default_layout(view::layout());
    builder.build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
