//! 📄️ Puzzle 5d play app panel — the document tree: parts (with their grips nested) and fasteners,
//! each row a pick target of the `vortex` interaction domain (ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so the framework paints selected/hovered
//! presence after render.
//!
//! 🪟️ Every container is windowed: both sections and each part's grip list publish their full
//! `total` through `TreeWindow` and materialise only the slice the host asked for
//! (`ViewModel::tree_windows`, read once per render as [`TreeWindows`]). Nothing is capped, nothing
//! is truncated, and no `+N` row exists.
//!
//! 🎯️ Rows carry a `granularity` and no binding of their own — the tree root owns the single
//! `interactionSelect` binding [`PanelTreeBuilder::interaction_domain`] stamps, and every row is
//! keyed by its raw entity id.

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{
    find_part_by_grip_full_id, puzzle5d_grip_full_id, ui_label, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dGrip, Puzzle5dPart, Puzzle5dScene, PUZZLE5D_GRANULARITY_FASTENER, PUZZLE5D_GRANULARITY_GRIP, PUZZLE5D_GRANULARITY_PART,
    PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_PLAY_CONTROLLER_ID,
};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, BuiltNode, HasBase};
use semio_framework_plugin::{
    tree_window_item, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.5d.play.artifact";
pub const ROOT: &str = "puzzle5d-play-document";
pub const PARTS_SECTION: &str = "puzzle5d-play-document.parts";
pub const FASTENERS_SECTION: &str = "puzzle5d-play-document.fasteners";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Rows
/// 🏷️ A part's display label: its flat text, else its volume label, else its kind.
pub fn part_label(part: &Puzzle5dPart) -> String {
    if !part.part_2d.text.is_empty() {
        return part.part_2d.text.clone();
    }
    part.part_3d.label.clone().unwrap_or_else(|| part.part_kind.clone())
}

fn fastener_label(document: &Puzzle5dDocument, fastener: &Puzzle5dFastener) -> String {
    let side = |full_id: &str| find_part_by_grip_full_id(document, full_id).map_or_else(|| full_id.to_string(), |(part, _)| part_label(part));
    format!("{} → {}", side(&fastener.source), side(&fastener.target))
}

fn ui_text(value: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new("ui.document.text", "puzzle5d document text admission failed"))
}

/// 🎯️ One pick row of the tree's interaction domain — keyed by the raw entity id, carrying its
/// granularity instead of a per-row `interactionSelect` argument map.
fn pick_item(id: &str, label: impl AsRef<str>, icon: &str, granularity: &str) -> UiAssemblyResult<ui::TreeItemBuilder> {
    Ok(ui::tree_item(ui_label(label)?)
        .try_id(id)
        .map_err(|_| PluginAssemblyError::new("ui.tree-item.id", "tree item id admission failed"))?
        .icon(ui_text(icon)?)
        .granularity(ui_text(granularity)?))
}

fn grip_row(part: &Puzzle5dPart, grip: &Puzzle5dGrip) -> UiAssemblyResult<BuiltNode> {
    let full_id = puzzle5d_grip_full_id(&part.id, &grip.id);
    pick_item(&full_id, format!("{} ({})", grip.id, grip.grip_kind), "circle-dot", PUZZLE5D_GRANULARITY_GRIP)?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.document.grip", "grip row admission failed"))
}

/// 🪟️ One part row and its windowed grip list.
fn part_row(windows: &TreeWindows<'_>, part: &Puzzle5dPart) -> UiAssemblyResult<BuiltNode> {
    let item = pick_item(&part.id, part_label(part), "box", PUZZLE5D_GRANULARITY_PART)?.description(ui_text(&part.part_kind)?);
    tree_window_item(windows, item, &part.id, false, &part.grips, |grip| grip_row(part, grip))
}

fn fastener_row(document: &Puzzle5dDocument, fastener: &Puzzle5dFastener) -> UiAssemblyResult<BuiltNode> {
    pick_item(&fastener.id, fastener_label(document, fastener), "link", PUZZLE5D_GRANULARITY_FASTENER)?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.document.fastener", "fastener row admission failed"))
}
//#endregion 🔖️Rows

//#region 🔖️Render
pub fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let document = &envelope.document;
    PanelTreeBuilder::new(ROOT)?
        .window_section_or_placeholder(windows, PARTS_SECTION, Some(ui_label(labels.parts.as_str())?), true, &document.parts, |part| part_row(windows, part), ui_label(labels.none.as_str())?)?
        .window_section_or_placeholder(windows, FASTENERS_SECTION, Some(ui_label(labels.fasteners.as_str())?), false, &document.fasteners, |fastener| fastener_row(document, fastener), ui_label(labels.none.as_str())?)?
        .interaction_domain(PUZZLE5D_PLAY_CONTROLLER_ID, PUZZLE5D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
