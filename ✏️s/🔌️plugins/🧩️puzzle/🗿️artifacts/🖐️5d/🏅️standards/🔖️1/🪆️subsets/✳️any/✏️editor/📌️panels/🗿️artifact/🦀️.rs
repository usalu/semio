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
//! 🙈️ A part row additionally carries two INLINE toggles (show/hide, lock/unlock) dispatching
//! `setSelectionFlag {entity, flag, ids, value}` — a row action, not a binding, and each asks for the
//! INVERSE of the state the row is in. A target-volume row carries the same pair through its OWN verb,
//! `setTargetVolumeFlag {flag, id, value}`, because `setSelectionFlag` writes the part slice only. Grips
//! and fasteners carry no such flag in this document model, so their rows carry no toggle. A row whose
//! toggles the argument arena refuses is still built.

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{
    find_part_by_grip_full_id, puzzle5d_grip_full_id, puzzle5d_part_display_label, ui_label, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dGrip, Puzzle5dPart, Puzzle5dScene, Puzzle5dTargetVolume, PUZZLE5D_GRANULARITY_FASTENER,
    PUZZLE5D_GRANULARITY_GRIP, PUZZLE5D_GRANULARITY_PART, PUZZLE5D_GRANULARITY_TARGET_VOLUME, PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_PLAY_CONTROLLER_ID,
};
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, Buildable, BuiltNode, HasBase, RowAction, RowActionPlacement, Trigger};
use semio_framework_plugin::{
    tree_window_item, ActionFactory, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, UiValue, FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
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

/// 🔁️ One inline row toggle's `setSelectionFlag` args. `value` is the flag state the click ASKS FOR —
/// always the inverse of the row's current one, so "Show"/"Unlock" really un-hides and unlocks instead
/// of re-applying the state the row was already in (the hardcoded-`true` defect puzzle 3d carried).
///
/// 🔑️ `UiMapBuilder::push` admits keys in STRICTLY ASCENDING order only: `entity`, `flag`, `ids`,
/// `value`.
fn flag_args(entity: &str, id: &str, flag: &str, value: bool) -> UiAssemblyResult<UiValue> {
    let text = |value: &str| UiText::try_from_str(value).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.document.flag", "puzzle5d row flag text admission failed"));
    let mut ids = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.document.flag", "puzzle5d row flag list admission failed"))?;
    ids.push(text(id)?).map_err(|_| PluginAssemblyError::new("ui.document.flag", "puzzle5d row flag id admission failed"))?;
    let mut args = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.document.flag", "puzzle5d row flag map admission failed"))?;
    for (key, value) in [("entity", text(entity)?), ("flag", text(flag)?), ("ids", UiValue::List(ids.finish())), ("value", UiValue::Bool(value))] {
        args.push(key.to_owned(), value).map_err(|_| PluginAssemblyError::new("ui.document.flag", "puzzle5d row flag entry admission failed"))?;
    }
    Ok(UiValue::Map(args.finish()))
}

fn hide_lock_actions(hidden: bool, locked: bool, labels: &Puzzle5dLabels, id: &str) -> UiAssemblyResult<[RowAction; 2]> {
    let binding = |flag: &str, value: bool| -> UiAssemblyResult<ActionBinding> {
        let (action, args) = ActionFactory::new(PUZZLE5D_PLAY_CONTROLLER_ID).action("setSelectionFlag", Some(flag_args(PUZZLE5D_GRANULARITY_PART, id, flag, value)?))?;
        Ok(ActionBinding { trigger: Trigger::Activate, action, args, capability: None })
    };
    Ok([
        RowAction {
            icon: ui_text(if hidden { "eye-off" } else { "eye" })?,
            label: Some(ui_label(if hidden { labels.show.as_str() } else { labels.hide.as_str() })?),
            action: binding("hidden", !hidden)?,
            placement: RowActionPlacement::Row,
        },
        RowAction {
            icon: ui_text(if locked { "lock" } else { "lock-open" })?,
            label: Some(ui_label(if locked { labels.unlock.as_str() } else { labels.lock.as_str() })?),
            action: binding("locked", !locked)?,
            placement: RowActionPlacement::Row,
        },
    ])
}

/// 🪙️ Attaches a row's INLINE toggles, and keeps the row when the argument arena cannot afford them.
/// The arena is process-global and one page serves every panel of every plugin at once, so on a
/// flagship document propagating a refusal would end the whole parts section at zero rows: a row
/// without its toggles is still a pick target, a row that was never materialised is neither.
fn with_hide_lock_actions(item: ui::TreeItemBuilder, hidden: bool, locked: bool, labels: &Puzzle5dLabels, id: &str) -> ui::TreeItemBuilder {
    let Ok(actions) = hide_lock_actions(hidden, locked, labels, id) else {
        return item;
    };
    let mut item = item;
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
    let item = with_hide_lock_actions(item, hidden, locked, labels, &part.id);
    tree_window_item(windows, item, &part.id, false, &part.grips, |grip| grip_row(part, grip))
}

/// 🔁️ One target-volume toggle's `setTargetVolumeFlag` args, keys in STRICTLY ASCENDING order
/// (`flag`, `id`, `value`). A volume is NOT flagged through `setSelectionFlag`: that verb writes the
/// part slice only, so 5G's own `setTargetVolumeFlag` is the one authority over these two flags.
fn volume_flag_args(id: &str, flag: &str, value: bool) -> UiAssemblyResult<UiValue> {
    let text = |value: &str| UiText::try_from_str(value).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.document.volume-flag", "puzzle5d volume flag text admission failed"));
    let mut args = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.document.volume-flag", "puzzle5d volume flag map admission failed"))?;
    for (key, value) in [("flag", text(flag)?), ("id", text(id)?), ("value", UiValue::Bool(value))] {
        args.push(key.to_owned(), value).map_err(|_| PluginAssemblyError::new("ui.document.volume-flag", "puzzle5d volume flag entry admission failed"))?;
    }
    Ok(UiValue::Map(args.finish()))
}

fn volume_hide_lock_actions(volume: &Puzzle5dTargetVolume, labels: &Puzzle5dLabels) -> UiAssemblyResult<[RowAction; 2]> {
    let binding = |flag: &str, value: bool| -> UiAssemblyResult<ActionBinding> {
        let (action, args) = ActionFactory::new(PUZZLE5D_PLAY_CONTROLLER_ID).action("setTargetVolumeFlag", Some(volume_flag_args(&volume.id, flag, value)?))?;
        Ok(ActionBinding { trigger: Trigger::Activate, action, args, capability: None })
    };
    Ok([
        RowAction {
            icon: ui_text(if volume.hidden { "eye-off" } else { "eye" })?,
            label: Some(ui_label(if volume.hidden { labels.show.as_str() } else { labels.hide.as_str() })?),
            action: binding("hidden", !volume.hidden)?,
            placement: RowActionPlacement::Row,
        },
        RowAction {
            icon: ui_text(if volume.locked { "lock" } else { "lock-open" })?,
            label: Some(ui_label(if volume.locked { labels.unlock.as_str() } else { labels.lock.as_str() })?),
            action: binding("locked", !volume.locked)?,
            placement: RowActionPlacement::Row,
        },
    ])
}

/// 🧊️ One target-volume row: a pick target of the same interaction domain, dimmed while hidden, with the
/// two inline toggles a part row carries. Its toggles degrade exactly like a part's do.
fn target_volume_row(volume: &Puzzle5dTargetVolume, labels: &Puzzle5dLabels) -> UiAssemblyResult<BuiltNode> {
    let mut item = pick_item(&volume.id, volume.id.clone(), "box-select", PUZZLE5D_GRANULARITY_TARGET_VOLUME)?.dimmed(volume.hidden);
    if let Ok(actions) = volume_hide_lock_actions(volume, labels) {
        for row_action in actions {
            match item.try_row_action(row_action) {
                Ok(next) => item = next,
                Err((refused, _)) => {
                    item = refused;
                    break;
                }
            }
        }
    }
    item.try_build().map_err(|_| PluginAssemblyError::new("ui.document.target-volume", "target volume row admission failed"))
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
        .window_section_or_placeholder(windows, PARTS_SECTION, Some(ui_label(labels.parts.as_str())?), true, &document.parts, |part| part_row(windows, document, part, labels), ui_label(labels.none.as_str())?)?
        .window_section_or_placeholder(windows, TARGET_VOLUMES_SECTION, Some(ui_label(labels.target_volumes.as_str())?), false, &document.target_volumes, |volume| target_volume_row(volume, labels), ui_label(labels.none.as_str())?)?
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
