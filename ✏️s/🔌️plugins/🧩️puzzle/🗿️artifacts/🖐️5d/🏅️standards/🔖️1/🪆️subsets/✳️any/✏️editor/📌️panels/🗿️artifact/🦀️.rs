//! 📄️ Puzzle 5d play app panel — the document tree: parts (with their grips nested), target volumes and
//! fasteners,
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
//!
//! 🙈️ A part row additionally carries ONE target `{entity, hidden, ids, locked}` and two INLINE toggles naming the
//! set-verbs `setSelectionHidden`/`setSelectionLocked` — row actions, not bindings, and the target states the INVERSE
//! of the state the row is in, so replaying a stale click sets a value and never flips one. A target-volume row carries
//! the same pair through its OWN verbs, `setTargetVolumeHidden`/`setTargetVolumeLocked` over `{hidden, id, locked}`,
//! because the selection verbs write the part slice only. Grips and fasteners carry no such flag in this document model,
//! so their rows carry no toggle. A row whose toggles the argument arena refuses is still built.

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{
    find_part_by_grip_full_id, puzzle5d_grip_full_id, puzzle5d_part_display_label, ui_label, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dGrip, Puzzle5dPart, Puzzle5dScene, Puzzle5dTargetVolume, PUZZLE5D_GRANULARITY_FASTENER,
    PUZZLE5D_GRANULARITY_GRIP, PUZZLE5D_GRANULARITY_PART, PUZZLE5D_GRANULARITY_TARGET_VOLUME, PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_PLAY_CONTROLLER_ID,
};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, BuiltNode, HasBase, RowActionPlacement};
use semio_framework_plugin::{
    row_action, row_target, tree_window_item, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, UiValue, FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
    FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.5d.play.artifact";
pub const ROOT: &str = "puzzle5d-play-document";
pub const PARTS_SECTION: &str = "puzzle5d-play-document.parts";
pub const FASTENERS_SECTION: &str = "puzzle5d-play-document.fasteners";
pub const TARGET_VOLUMES_SECTION: &str = "puzzle5d-play-document.target-volumes";
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
fn fastener_label(document: &Puzzle5dDocument, fastener: &Puzzle5dFastener) -> String {
    let side = |full_id: &str| find_part_by_grip_full_id(document, full_id).map_or_else(|| full_id.to_string(), |(part, _)| puzzle5d_part_display_label(part, document));
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

/// 🎯️ A flag row's ONE target: its identity entries and the flag state each inline toggle ASKS FOR — always the inverse
/// of the row's current one, stated as a value, so "Show"/"Unlock" really un-hide and unlock instead of re-applying the
/// state the row was already in (the hardcoded-`true` defect puzzle 3d carried), and a replayed click changes nothing.
///
/// 🔑️ `UiMapBuilder::push` admits keys in STRICTLY ASCENDING order only, so `entries` arrive sorted by key.
fn flag_target<const N: usize>(entries: [(&str, UiValue); N]) -> UiAssemblyResult<ui::RowTarget> {
    let mut args = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.document.flag", "puzzle5d row target map admission failed"))?;
    for (key, value) in entries {
        args.push(key.to_owned(), value).map_err(|_| PluginAssemblyError::new("ui.document.flag", "puzzle5d row target entry admission failed"))?;
    }
    row_target(PUZZLE5D_PLAY_CONTROLLER_ID, Some(UiValue::Map(args.finish())), None)
}

fn flag_text(value: &str) -> UiAssemblyResult<UiValue> {
    UiText::try_from_str(value).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.document.flag", "puzzle5d row target text admission failed"))
}

/// 🎯️ A part row's target: `{entity, hidden, ids, locked}` names exactly that part.
fn part_flag_target(id: &str, hidden: bool, locked: bool) -> UiAssemblyResult<ui::RowTarget> {
    let mut ids = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.document.flag", "puzzle5d row target list admission failed"))?;
    ids.push(flag_text(id)?).map_err(|_| PluginAssemblyError::new("ui.document.flag", "puzzle5d row target id admission failed"))?;
    flag_target([("entity", flag_text(PUZZLE5D_GRANULARITY_PART)?), ("hidden", UiValue::Bool(!hidden)), ("ids", UiValue::List(ids.finish())), ("locked", UiValue::Bool(!locked))])
}

/// 🎬️ A flag row's two inline toggles — `verbs` names the collection's own set-verbs.
fn hide_lock_actions(hidden: bool, locked: bool, labels: &Puzzle5dLabels, verbs: [&str; 2]) -> UiAssemblyResult<[ui::RowAction; 2]> {
    Ok([
        row_action(if hidden { "eye-off" } else { "eye" }, if hidden { labels.show.as_str() } else { labels.hide.as_str() }, verbs[0], RowActionPlacement::Row)?,
        row_action(if locked { "lock" } else { "lock-open" }, if locked { labels.unlock.as_str() } else { labels.lock.as_str() }, verbs[1], RowActionPlacement::Row)?,
    ])
}

/// 🪙️ Attaches a row's target and INLINE toggles, and keeps the row when the argument arena cannot afford them.
/// The arena is process-global and one page serves every panel of every plugin at once, so on a
/// flagship document propagating a refusal would end the whole parts section at zero rows: a row
/// without its toggles is still a pick target, a row that was never materialised is neither.
fn with_hide_lock_actions(item: ui::TreeItemBuilder, target: UiAssemblyResult<ui::RowTarget>, actions: UiAssemblyResult<[ui::RowAction; 2]>) -> ui::TreeItemBuilder {
    let (Ok(target), Ok(actions)) = (target, actions) else {
        return item;
    };
    let mut item = item.target(target);
    for row_action in actions {
        match item.try_row_action(row_action) {
            Ok(next) => item = next,
            Err((refused, _)) => return refused,
        }
    }
    item
}

/// 🪟️ One part row and its windowed grip list.
fn part_row(windows: &TreeWindows<'_>, document: &Puzzle5dDocument, part: &Puzzle5dPart, labels: &Puzzle5dLabels) -> UiAssemblyResult<BuiltNode> {
    let hidden = part.part_2d.hidden.unwrap_or(false);
    let locked = part.part_2d.locked.unwrap_or(false);
    let item = pick_item(&part.id, puzzle5d_part_display_label(part, document), "box", PUZZLE5D_GRANULARITY_PART)?.description(ui_text(&part.part_kind)?).dimmed(hidden);
    let item = with_hide_lock_actions(item, part_flag_target(&part.id, hidden, locked), hide_lock_actions(hidden, locked, labels, ["setSelectionHidden", "setSelectionLocked"]));
    tree_window_item(windows, item, &part.id, false, &part.grips, |grip| grip_row(part, grip))
}

/// 🧊️ One target-volume row: a pick target of the same interaction domain, dimmed while hidden, with the
/// two inline toggles a part row carries — through the volume's OWN set-verbs, because the selection verbs write the part
/// slice only. Its toggles degrade exactly like a part's do.
fn target_volume_row(volume: &Puzzle5dTargetVolume, labels: &Puzzle5dLabels) -> UiAssemblyResult<BuiltNode> {
    let item = pick_item(&volume.id, volume.id.clone(), "box-select", PUZZLE5D_GRANULARITY_TARGET_VOLUME)?.dimmed(volume.hidden);
    let target = flag_text(&volume.id).and_then(|id| flag_target([("hidden", UiValue::Bool(!volume.hidden)), ("id", id), ("locked", UiValue::Bool(!volume.locked))]));
    with_hide_lock_actions(item, target, hide_lock_actions(volume.hidden, volume.locked, labels, ["setTargetVolumeHidden", "setTargetVolumeLocked"]))
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.document.target-volume", "target volume row admission failed"))
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
        .interaction_domain(PUZZLE5D_PLAY_CONTROLLER_ID, PUZZLE5D_INTERACTION_DOMAIN)?
        .window_section_or_placeholder(windows, PARTS_SECTION, Some(ui_label(labels.parts.as_str())?), true, &document.parts, |part| part_row(windows, document, part, labels), ui_label(labels.none.as_str())?)?
        .window_section_or_placeholder(windows, TARGET_VOLUMES_SECTION, Some(ui_label(labels.target_volumes.as_str())?), false, &document.target_volumes, |volume| target_volume_row(volume, labels), ui_label(labels.none.as_str())?)?
        .window_section_or_placeholder(windows, FASTENERS_SECTION, Some(ui_label(labels.fasteners.as_str())?), false, &document.fasteners, |fastener| fastener_row(document, fastener), ui_label(labels.none.as_str())?)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
