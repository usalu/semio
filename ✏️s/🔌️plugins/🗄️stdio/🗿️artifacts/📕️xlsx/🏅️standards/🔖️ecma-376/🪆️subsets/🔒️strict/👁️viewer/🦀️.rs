//! 👁️ Xlsx viewer (ecma-376/🔒️strict) — read-only counterpart of the sibling mutation-capable
//! surface for this subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2).
//! `XlsxStrictViewer` implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` —
//! `ViewerApp<XlsxStrictViewer>` (framework SDK) is the sole runtime adapter, so this file can never
//! structurally emit an artifact mutation. Must not import anything from the sibling mutation-
//! capable surface (viewer-purity policy — this file stays greppable-clean of that surface's own
//! module path).

use crate::viewer::xlsx::standards::v_ecma_376::subsets::strict::modes::view;
use crate::viewer::xlsx::standards::v_ecma_376::subsets::strict::modes::view::windows::main;
use crate::{XlsxMutation, XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, ViewEmit, Viewer};

//#region 🔖️Dialect
/// 🪪️ Duplicated from the sibling mutation-capable surface's own coordinate — see that file's doc
/// comment for why this is restated rather than shared: viewer purity forbids importing from that
/// surface.
pub const XLSX_STRICT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant —
/// mirrors `🔋️energy`'s own `EnergyModelViewCommand::Noop`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum XlsxStrictViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for XlsxStrictViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(XlsxStrictViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct XlsxStrictViewer;

impl ArtifactViewer for XlsxStrictViewer {
    type Snapshot = XlsxSnapshot;
    type Mutation = XlsxMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = XlsxStrictViewCommand;

    const DIALECT: Dialect = XLSX_STRICT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_XLSX_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> XlsxSnapshot {
        XlsxSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `Noop` variant never carries a config change.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &store::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_xlsx_strict_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(XLSX_STRICT_DIALECT)
        .document(["stdio", "xlsx", "strict"])
        .icon_id("table")
        .mode_def(view::definition())
        .default_mode_id(view::XLSX_STRICT_VIEW_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
