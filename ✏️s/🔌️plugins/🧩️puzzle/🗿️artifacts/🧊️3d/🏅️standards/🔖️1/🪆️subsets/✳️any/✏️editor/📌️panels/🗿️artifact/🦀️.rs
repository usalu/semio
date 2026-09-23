//! 📄️ Puzzle 3d play app panel — the document tree: objects (with their vortices nested), reference
//! planes, target volumes and attractions, each row a pick target of the tree's interaction domain and
//! carrying inline hide/lock actions.
//!
//! 🪟️ The tree is **virtualised**, not paged. Every container reports its full `total` and materialises
//! only the row window the host asked for (`TreeWindows::for_body`, threaded in from
//! `Puzzle3dPlayApp::render_body`), so the scrollbar spans the whole document and there is no `+N`
//! continuation row anywhere. A document scales to `DOCUMENT_OBJECT_SLOTS`/`DOCUMENT_VORTEX_SLOTS`
//! entries (2048 objects, 4096 vortices) while one built node admits `UI_BUILT_CHILDREN_MAX` children;
//! the window the host requests is clamped to that, and a row refused by the argument arena ends the
//! window early rather than faulting. Nakagin (180 objects, 358 vortices) faulted before this: the
//! arena refused a row's argument map at roughly the eleventh object, and picks now cost zero arena
//! because the tree carries ONE `interactionSelect` binding instead of one per row.

use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{
    puzzle3d_object_display_label, puzzle3d_vortex_full_id, ui_label, Puzzle3dAttraction, Puzzle3dFixture, Puzzle3dObject, Puzzle3dReference, Puzzle3dTargetVolume, Puzzle3dVortex, PUZZLE3D_GRANULARITY_ATTRACTION,
    PUZZLE3D_GRANULARITY_OBJECT, PUZZLE3D_GRANULARITY_REFERENCE, PUZZLE3D_GRANULARITY_TARGET_VOLUME, PUZZLE3D_GRANULARITY_VORTEX, PUZZLE3D_INTERACTION_DOMAIN, PUZZLE3D_PLAY_CONTROLLER_ID,
};
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, Buildable, BuiltNode, HasBase, RowAction, RowActionPlacement, Trigger};
use semio_framework_plugin::{
    tree_window_item, ActionFactory, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, UiValue, FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
    FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.3d.play.artifact";
pub(crate) const ROOT: &str = "puzzle3d-play-document";
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
fn action(action: &str, args: Option<UiValue>) -> UiAssemblyResult<(ui::ActionId, Option<UiValue>)> {
    ActionFactory::new(PUZZLE3D_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> UiAssemblyResult<UiValue> {
    UiText::try_from_str(value.as_ref()).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> UiValue {
    UiValue::Bool(value)
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = UiValue>) -> UiAssemblyResult<UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, UiValue)>) -> UiAssemblyResult<UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(UiValue::Map(builder.finish()))
}

/// 🕹️ One pick row of the tree's interaction domain: keyed by the RAW target id and carrying only its
/// granularity. The activation a click needs is the ONE `interactionSelect` binding
/// [`PanelTreeBuilder::interaction_domain`] stamps on the tree root, so a row costs zero argument arena
/// and a 128-row window is affordable where 11 per-row argument maps were not.
fn pick_item(id: impl AsRef<str>, label: impl AsRef<str>, icon: &str, granularity: &str) -> UiAssemblyResult<ui::TreeItemBuilder> {
    Ok(ui::tree_item(ui_label(label)?)
        .try_id(id.as_ref())
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d document id admission failed"))?
        .icon(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d document icon admission failed"))?)
        .granularity(UiText::try_from_str(granularity).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d document granularity admission failed"))?))
}

fn binding(action: UiAssemblyResult<(ui::ActionId, Option<UiValue>)>) -> UiAssemblyResult<ActionBinding> {
    let (action, args) = action?;
    Ok(ActionBinding { trigger: Trigger::Activate, action, args, capability: None })
}

/// 🔁️ One inline row toggle's `setSelectionFlag` args. `value` is the flag state the click ASKS FOR —
/// always the inverse of the row's current one, the same negation the context menu (`!all_hidden`) and
/// the inspection panel (`!pressed`) already carry. A hardcoded `true` here made "Show"/"Unlock"
/// re-apply the state the row was already in, so an outliner-hidden object could never be un-hidden
/// from the row that hid it.
fn flag_args(entity: &str, id: &str, flag: &str, value: bool) -> UiAssemblyResult<UiValue> {
    ui_value_map([("entity", ui_value_text(entity)?), ("flag", ui_value_text(flag)?), ("ids", ui_value_list([ui_value_text(id)?])?), ("value", ui_value_bool(value))])
}

fn hide_lock_actions(hidden: bool, locked: bool, labels: &Puzzle3dLabels, entity: &str, id: &str) -> UiAssemblyResult<[RowAction; 2]> {
    Ok([
        RowAction {
            icon: UiText::try_from_str(if hidden { "eye-off" } else { "eye" }).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d visibility icon admission failed"))?,
            label: Some(ui_label(if hidden { labels.show.as_str() } else { labels.hide.as_str() })?),
            action: binding(action("setSelectionFlag", Some(flag_args(entity, id, "hidden", !hidden)?)))?,
            placement: RowActionPlacement::Row,
        },
        RowAction {
            icon: UiText::try_from_str(if locked { "lock" } else { "lock-open" }).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d lock icon admission failed"))?,
            label: Some(ui_label(if locked { labels.unlock.as_str() } else { labels.lock.as_str() })?),
            action: binding(action("setSelectionFlag", Some(flag_args(entity, id, "locked", !locked)?)))?,
            placement: RowActionPlacement::Row,
        },
    ])
}

/// 🪙️ Attaches a row's INLINE toggles, and keeps the row when the argument arena cannot afford them.
///
/// 🧾️ A row's two inline toggles are a map plus a nested id list each, and the arena is process-global
/// and holds `UI_VALUE_LIVE_PAGES` page — one — for every panel of every plugin at once, so on the
/// flagship document the catalogue's own rows could leave the outliner too little credit for the FIRST
/// object row's toggles; propagating that refusal ended the section at zero rows: four section headers
/// and nothing selectable, measured in the browser as `outliner entityRows=0`
/// (26/09/02/PUZZLE-3D-END-TO-END wave B44 §6.2). A row without its toggles is still a pick target and
/// still names its entity; a row that was never materialised is neither.
fn with_hide_lock_actions(item: ui::TreeItemBuilder, hidden: bool, locked: bool, labels: &Puzzle3dLabels, entity: &str, id: &str) -> ui::TreeItemBuilder {
    let Ok(actions) = hide_lock_actions(hidden, locked, labels, entity, id) else {
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

fn object_row(object: &Puzzle3dObject, fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let item = pick_item(&object.id, puzzle3d_object_display_label(object, fixture), "box", PUZZLE3D_GRANULARITY_OBJECT)?.dimmed(object.hidden);
    let item = with_hide_lock_actions(item, object.hidden, object.locked, labels, "object", &object.id);
    tree_window_item(windows, item, &object.id, false, &object.vortices, |vortex| vortex_row(&object.id, vortex))
}

fn vortex_row(object_id: &str, vortex: &Puzzle3dVortex) -> UiAssemblyResult<BuiltNode> {
    let full_id = puzzle3d_vortex_full_id(object_id, &vortex.id);
    pick_item(&full_id, vortex.vortex_kind.clone().unwrap_or_else(|| vortex.id.clone()), "circle-dot", PUZZLE3D_GRANULARITY_VORTEX)?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d vortex row admission failed"))
}

fn reference_row(reference: &Puzzle3dReference, labels: &Puzzle3dLabels) -> UiAssemblyResult<BuiltNode> {
    let item = pick_item(&reference.id, reference.id.clone(), "globe", PUZZLE3D_GRANULARITY_REFERENCE)?
        .description(UiText::try_from_str(&reference.source.url).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d reference description admission failed"))?)
        .dimmed(reference.hidden);
    with_hide_lock_actions(item, reference.hidden, reference.locked, labels, "reference", &reference.id).try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d reference row admission failed"))
}

fn target_volume_row(volume: &Puzzle3dTargetVolume, labels: &Puzzle3dLabels) -> UiAssemblyResult<BuiltNode> {
    let item = pick_item(&volume.id, volume.id.clone(), "cylinder", PUZZLE3D_GRANULARITY_TARGET_VOLUME)?.dimmed(volume.hidden);
    with_hide_lock_actions(item, volume.hidden, volume.locked, labels, "targetVolume", &volume.id).try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d target-volume row admission failed"))
}

fn attraction_row(attraction: &Puzzle3dAttraction) -> UiAssemblyResult<BuiltNode> {
    pick_item(&attraction.id, format!("{} → {}", attraction.attracting, attraction.attracted), "link", PUZZLE3D_GRANULARITY_ATTRACTION)?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d attraction row admission failed"))
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🌳️ The four document sections, each windowed against the host's own open/scroll state for this body.
pub fn render(fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    PanelTreeBuilder::new(ROOT)?
        .interaction_domain(PUZZLE3D_PLAY_CONTROLLER_ID, PUZZLE3D_INTERACTION_DOMAIN)?
        .window_section(windows, &format!("{ROOT}.objects"), Some(ui_label(labels.objects.as_str())?), true, &fixture.objects, |object| object_row(object, fixture, labels, windows))?
        .window_section(windows, &format!("{ROOT}.references"), Some(ui_label(labels.references.as_str())?), false, &fixture.references, |reference| reference_row(reference, labels))?
        .window_section(windows, &format!("{ROOT}.target-volumes"), Some(ui_label(labels.target_volumes.as_str())?), false, &fixture.target_volumes, |volume| target_volume_row(volume, labels))?
        .window_section(windows, &format!("{ROOT}.attractions"), Some(ui_label(labels.attractions.as_str())?), false, &fixture.attractions, attraction_row)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
