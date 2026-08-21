//! 👁️ Xlsx viewer (ecma-376/✳️transitional) — read-only counterpart of the sibling mutation-
//! capable surface for this subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
//! contract §2.2). `XlsxTransitionalViewer` implements `ArtifactViewer`, never
//! `ArtifactEditor`/`ArtifactApp` — `ViewerApp<XlsxTransitionalViewer>` (framework SDK) is the sole
//! runtime adapter, so this file can never structurally emit an artifact mutation. Must not import
//! anything from the sibling mutation-capable surface (viewer-purity policy — this file stays
//! greppable-clean of that surface's own module path).

use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::XlsxCellValue;
use crate::artifacts::xlsx::{XlsxMutation, XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};
use crate::viewer::xlsx::standards::v_ecma_376::subsets::transitional::modes::view;
use crate::viewer::xlsx::standards::v_ecma_376::subsets::transitional::modes::view::windows::main;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode, ViewEmit, Viewer};

//#region 🔖️Dialect
/// 🪪️ Duplicated from the sibling mutation-capable surface's own coordinate — see that file's doc
/// comment for why this is restated rather than shared: viewer purity forbids importing from that
/// surface.
pub const XLSX_TRANSITIONAL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
//#endregion 🔖️Dialect

//#region 🔖️TableProjection
/// 🧮 Read-only twin of the sibling mutation-capable surface's own cell-flattening helper — see
/// that file's doc comment for the flattening rationale. Duplicated rather than shared (viewer
/// purity).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn xlsx_flat_cells(document: &XlsxSnapshot) -> Vec<(String, u32, u32, XlsxCellValue)> {
    document.workbook.sheets.iter().flat_map(|sheet| sheet.cells.iter().map(move |cell| (sheet.name.clone(), cell.row, cell.col, cell.value.clone()))).collect()
}

/// 🔎 Read-only twin of the sibling mutation-capable surface's own cell-value renderer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn render_xlsx_cell_value(value: &XlsxCellValue, shared_strings: &[String]) -> String {
    match value {
        XlsxCellValue::Number(n) => format!("{n}"),
        XlsxCellValue::SharedString(index) => shared_strings.get(*index).cloned().unwrap_or_else(|| format!("#{index}")),
        XlsxCellValue::InlineString(text) => text.clone(),
        XlsxCellValue::Boolean(flag) => flag.to_string(),
        XlsxCellValue::Formula { expr, cached } => match cached {
            Some(cached) => format!("={expr} ({})", render_xlsx_cell_value(cached, shared_strings)),
            None => format!("={expr}"),
        },
        XlsxCellValue::Empty => String::new(),
    }
}
//#endregion 🔖️TableProjection

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant —
/// mirrors `🔋️energy`'s own `EnergyModelViewCommand::Noop`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum XlsxTransitionalViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for XlsxTransitionalViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(XlsxTransitionalViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct XlsxTransitionalViewer;

impl ArtifactViewer for XlsxTransitionalViewer {
    type Snapshot = XlsxSnapshot;
    type Mutation = XlsxMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = XlsxTransitionalViewCommand;

    const DIALECT: Dialect = XLSX_TRANSITIONAL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_XLSX_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> XlsxSnapshot {
        XlsxSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `Noop` variant never carries a config change.
    async fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _engines: &store::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))).await,
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_xlsx_transitional_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(XLSX_TRANSITIONAL_DIALECT)
        .document(["stdio", "xlsx", "transitional"])
        .icon_id("table")
        .mode_def(view::definition())
        .default_mode_id(view::XLSX_TRANSITIONAL_VIEW_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_xlsx_transitional_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_xlsx_transitional_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, XLSX_TRANSITIONAL_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<XlsxTransitionalViewer as ArtifactViewer>::DIALECT, XLSX_TRANSITIONAL_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_declares_the_main_window() {
        let def = create_xlsx_transitional_viewer();
        assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
