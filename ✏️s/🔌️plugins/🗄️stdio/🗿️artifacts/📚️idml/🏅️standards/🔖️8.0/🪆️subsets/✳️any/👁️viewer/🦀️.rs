//! 👁️ Idml viewer — the read-only counterpart of the editor: the same story runs, never a mutation.

use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};
use semio_ui_locale::Label;
use semio_s_artifact_stdio_idml::{IdmlMutation, IdmlSnapshot, DOCUMENT_SCHEMA};

#[path = "🎭️modes/👁️view/🦀️.rs"]
pub mod modes;

use modes::main;

/// 🪪️ Artifact coordinate `s.stdio.idml@8.0/*`, duplicated from the editor so this file never imports it.
pub const IDML_VIEWER_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.idml", standard: StandardId("8.0"), subset: SubsetId::ANY };

/// 👁️ The viewer declares no actions: one inert command variant.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IdmlViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for IdmlViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(IdmlViewCommand::Noop)
    }
}

#[derive(Default, Clone, Copy)]
pub struct IdmlViewer;

impl ArtifactViewer for IdmlViewer {
    type Snapshot = IdmlSnapshot;
    type Mutation = IdmlMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = IdmlViewCommand;

    const DIALECT: Dialect = IDML_VIEWER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DOCUMENT_SCHEMA;

    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_plugin::ViewModel>,
        _engines: &semio_framework_2d_compute::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_plugin::built_to_component_tree),
            _ => semio_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

/// 🧩️ The viewer's manifest.
pub fn create_idml_viewer() -> semio_plugin::AppDefinition {
    Viewer::builder(IDML_VIEWER_DIALECT)
        .document(["semio", "stdio", "idml"])
        .icon_id("file-text")
        .mode_def(modes::definition())
        .default_mode_id(modes::IDML_VIEW_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(modes::layout())
        .build_definition()
}
