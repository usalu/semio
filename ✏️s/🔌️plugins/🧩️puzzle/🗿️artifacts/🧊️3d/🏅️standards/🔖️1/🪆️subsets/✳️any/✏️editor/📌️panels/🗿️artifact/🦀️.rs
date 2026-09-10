//! 📄️ Puzzle 3d play app panel — the document tree: objects (with their vortices nested), reference
//! planes, target volumes and attractions, each row selecting its entity and carrying inline
//! hide/lock actions. The rendered sections are memoized by `Puzzle3dPlayApp` against the fixture's
//! geometry fingerprint, so this builder only reruns when the document actually changes.
//!
//! 🧾️ The tree is **virtualised**, not a row per entity. A document scales to
//! `DOCUMENT_OBJECT_SLOTS`/`DOCUMENT_VORTEX_SLOTS` entries (2048 objects, 4096 vortices) while a built
//! node admits `UI_BUILT_CHILDREN_MAX` children and every interactive row costs
//! `UI_VALUE_ROW_COLLECTIONS`/`UI_VALUE_ROW_ITEMS` of the process-wide argument arena, so this builder
//! materialises one bounded page — [`SECTION_ROWS`] rows per section, [`page_rows`] rows in total across
//! them — and closes each section it truncates with a continuation row naming the omitted count. Nakagin
//! (180 objects, 358 vortices) rendered a fault before this: the arena refused a row's argument map at
//! roughly the eleventh object.

use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use std::collections::HashMap;
use crate::editor::puzzle3d::{
    puzzle3d_vortex_full_id, ui_label, Puzzle3dAttraction, Puzzle3dFixture, Puzzle3dObject, Puzzle3dReference, Puzzle3dTargetVolume, Puzzle3dVortex, PUZZLE3D_GRANULARITY_ATTRACTION, PUZZLE3D_GRANULARITY_OBJECT, PUZZLE3D_GRANULARITY_REFERENCE,
    PUZZLE3D_GRANULARITY_TARGET_VOLUME, PUZZLE3D_GRANULARITY_VORTEX, PUZZLE3D_INTERACTION_DOMAIN, PUZZLE3D_PLAY_CONTROLLER_ID,
};
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, Buildable, BuiltNode, HasBase, HasChildren, RowAction, RowActionPlacement, Trigger};
use semio_framework_plugin::{
    ActionFactory, InteractionTarget, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiAssemblyResult, UiFixedList, UiText, UiValue, FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
    FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, INTERACTION_SELECT_ACTION_ID,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.3d.play.document";
const ROOT: &str = "puzzle3d-play-document";
/// 🗂️ Rows one section of this page materialises before it truncates: a built node admits
/// `UI_BUILT_CHILDREN_MAX` children and the last of them carries the section's continuation row.
pub const SECTION_ROWS: usize = ui::UI_BUILT_CHILDREN_MAX - 1;
/// 🗄️ Sections this outliner presents — objects, references, target volumes, attractions. Each is assembled
/// with the sections after it reserved out of the shared page, so the first one cannot consume it whole.
pub const SECTIONS: usize = 4;
/// 🧱 Host reconcile credits one `FlatPresentedNode` per presented node (~0.46 MiB). The 8 MiB
/// surface byte cap therefore admits at most 16 nodes; 18 Nakagin rows landed 30 KiB over.
pub const PANEL_RECONCILE_NODE_BUDGET: usize = 16;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Rows
fn action(action: &str, args: Option<UiValue>) -> UiAssemblyResult<(semio_framework_ui_contract::ActionId, Option<UiValue>)> {
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

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> UiValue {
    UiValue::Number(value.into())
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

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = UiAssemblyResult<BuiltNode>>) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut nodes = UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

fn select_action(granularity: &str, id: &str) -> UiAssemblyResult<(semio_framework_ui_contract::ActionId, Option<UiValue>)> {
    let targets = serde_json::to_string(&[InteractionTarget { granularity: granularity.into(), id: id.into() }]).map_err(|error| PluginAssemblyError::new("ui.action-argument", error.to_string()))?;
    let args = ui_value_map([("domainId", ui_value_text(PUZZLE3D_INTERACTION_DOMAIN)?), ("merge", ui_value_text("replace")?), ("method", ui_value_text("pick")?), ("targets", ui_value_text(targets)?)])?;
    action(INTERACTION_SELECT_ACTION_ID, Some(args))
}

fn binding(action: UiAssemblyResult<(semio_framework_ui_contract::ActionId, Option<UiValue>)>) -> UiAssemblyResult<ActionBinding> {
    let (action, args) = action?;
    Ok(ActionBinding { trigger: Trigger::Activate, action, args, capability: None })
}

fn selectable_item(id: impl AsRef<str>, label: impl AsRef<str>, icon: &str, action: UiAssemblyResult<(semio_framework_ui_contract::ActionId, Option<UiValue>)>) -> UiAssemblyResult<semio_framework_ui_contract::TreeItemBuilder> {
    let (action_id, args) = action?;
    let builder = ui::tree_item(ui_label(label)?)
        .try_id(id.as_ref())
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d document id admission failed"))?
        .icon(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d document icon admission failed"))?);
    match args {
        Some(args) => builder.try_on_with(Trigger::Activate, action_id, args),
        None => builder.try_on(Trigger::Activate, action_id),
    }
    .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d document action admission failed"))
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
fn object_row(object: &Puzzle3dObject, labels: &Puzzle3dLabels, budget: &mut RowBudget) -> UiAssemblyResult<BuiltNode> {
    let vortices = paged_section(&format!("{ROOT}.object.{}", object.id), &object.vortices, budget, |vortex, _| vortex_row(&object.id, vortex))?;
    let mut item = selectable_item(&object.id, object.object_kind.clone().unwrap_or_else(|| object.id.clone()), "box", select_action(PUZZLE3D_GRANULARITY_OBJECT, &object.id))?
        .default_open(false)
        .dimmed(object.hidden)
        .try_children(vortices)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d object children admission failed"))?;
    for row_action in hide_lock_actions(object.hidden, object.locked, labels, "object", &object.id)? {
        item = item.try_row_action(row_action).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d object row action admission failed"))?;
    }
    item.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d object row admission failed"))
}

fn vortex_row(object_id: &str, vortex: &Puzzle3dVortex) -> UiAssemblyResult<BuiltNode> {
    let full_id = puzzle3d_vortex_full_id(object_id, &vortex.id);
    selectable_item(&full_id, vortex.vortex_kind.clone().unwrap_or_else(|| vortex.id.clone()), "circle-dot", select_action(PUZZLE3D_GRANULARITY_VORTEX, &full_id))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d vortex row admission failed"))
}

fn reference_row(reference: &Puzzle3dReference, labels: &Puzzle3dLabels) -> UiAssemblyResult<BuiltNode> {
    let mut item = selectable_item(&reference.id, reference.id.clone(), "globe", select_action(PUZZLE3D_GRANULARITY_REFERENCE, &reference.id))?
        .description(UiText::try_from_str(&reference.source.url).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d reference description admission failed"))?)
        .dimmed(reference.hidden);
    for row_action in hide_lock_actions(reference.hidden, reference.locked, labels, "reference", &reference.id)? {
        item = item.try_row_action(row_action).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d reference row action admission failed"))?;
    }
    item.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d reference row admission failed"))
}

fn target_volume_row(volume: &Puzzle3dTargetVolume, labels: &Puzzle3dLabels) -> UiAssemblyResult<BuiltNode> {
    let mut item = selectable_item(&volume.id, volume.id.clone(), "cylinder", select_action(PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume.id))?.dimmed(volume.hidden);
    for row_action in hide_lock_actions(volume.hidden, volume.locked, labels, "targetVolume", &volume.id)? {
        item = item.try_row_action(row_action).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d target-volume row action admission failed"))?;
    }
    item.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d target-volume row admission failed"))
}

fn attraction_row(attraction: &Puzzle3dAttraction) -> UiAssemblyResult<BuiltNode> {
    selectable_item(&attraction.id, format!("{} → {}", attraction.attracting, attraction.attracted), "link", select_action(PUZZLE3D_GRANULARITY_ATTRACTION, &attraction.id))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d attraction row admission failed"))
}
//#endregion 🔖️Rows

//#region 🔖️Paging
// 🧾️ The virtualised-panel paging law itself lives in the SDK
// (`semio_framework_plugin::{panel_page_rows, PanelRowBudget, panel_continuation_row, paged_panel_section}`)
// so every panel that outgrows one `UI_VALUE_PAGE_ROWS` page — this document tree, the procedural
// operator catalogue — pages identically. This module keeps only the puzzle3d-specific quota and the
// names its own laws already read.

/// 🧮️ See `semio_framework_plugin::panel_page_rows`.
pub fn page_rows() -> usize {
    semio_framework_plugin::panel_page_rows()
}

/// 🧱 Small documents keep the SDK page so nested vortices stay complete. A document past that
/// page (Nakagin, 180 objects) uses the host reconcile envelope so the panel stays ≤16 nodes.
fn page_rows_for(fixture: &Puzzle3dFixture) -> usize {
    let entities = fixture.objects.len()
        + fixture.objects.iter().map(|object| object.vortices.len()).sum::<usize>()
        + fixture.references.len()
        + fixture.target_volumes.len()
        + fixture.attractions.len();
    if entities <= semio_framework_plugin::panel_page_rows() {
        semio_framework_plugin::panel_page_rows()
    } else {
        PANEL_RECONCILE_NODE_BUDGET.saturating_sub(1 + SECTIONS)
    }
}

/// 🔒️ One page at a time under test. Every law that materialises a whole panel page draws on the
/// process-global `UiValue` arena, so two of them running in parallel each observe a partly spent arena and
/// page shorter than their own law expects. This guard is the unit-test stand-in for the reactor's
/// one-turn-at-a-time discipline — production needs none, because a short page is still a correct page.
#[cfg(test)]
pub(crate) static PANEL_PAGE_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// 🧮️ See `semio_framework_plugin::PanelRowBudget`.
pub type RowBudget = semio_framework_plugin::PanelRowBudget;

/// ➕️ See `semio_framework_plugin::panel_continuation_row`.
pub fn continuation_row(section_id: &str, omitted: usize) -> UiAssemblyResult<BuiltNode> {
    continuation_row_from(section_id, omitted, None)
}

fn page_action(section_id: &str, page: u32) -> UiAssemblyResult<(semio_framework_ui_contract::ActionId, Option<UiValue>)> {
    action("setPanelPage", Some(ui_value_map([("page", ui_value_number(f64::from(page))), ("section", ui_value_text(section_id)?)])?))
}

/// 📄 Continuation that names the omitted count and, when a next page exists, advances `setPanelPage`.
pub fn continuation_row_from(section_id: &str, omitted: usize, next_page: Option<u32>) -> UiAssemblyResult<BuiltNode> {
    match next_page {
        Some(page) => selectable_item(format!("{section_id}.more"), format!("+{omitted}"), "ellipsis", page_action(section_id, page))?.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d continuation row admission failed")),
        None => semio_framework_plugin::panel_continuation_row(section_id, omitted),
    }
}

fn section_page(pages: &HashMap<String, u32>, section_id: &str, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    let max_page = (len - 1) / SECTION_ROWS;
    (pages.get(section_id).copied().unwrap_or(0) as usize).min(max_page)
}

/// 🗂️ See `semio_framework_plugin::paged_panel_section` — bound to this panel's own [`SECTION_ROWS`] quota.
pub fn paged_section<T>(section_id: &str, entries: &[T], budget: &mut RowBudget, row: impl FnMut(&T, &mut RowBudget) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    paged_section_from(section_id, entries, &HashMap::new(), budget, row)
}

/// 📄 One section page starting at the cursor in `pages`, with an Activate continuation that advances it.
pub fn paged_section_from<T>(section_id: &str, entries: &[T], pages: &HashMap<String, u32>, budget: &mut RowBudget, mut row: impl FnMut(&T, &mut RowBudget) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let page = section_page(pages, section_id, entries.len());
    let offset = page.saturating_mul(SECTION_ROWS).min(entries.len());
    let slice = &entries[offset..];
    let mut items = UiFixedList::<BuiltNode>::default();
    let quota = slice.len().min(SECTION_ROWS);
    let truncated = slice.len() > SECTION_ROWS;
    let mut placed = 0;
    for entry in slice {
        if placed == SECTION_ROWS || (truncated && budget.remaining() <= 1) || !budget.spend() {
            break;
        }
        placed += 1;
        let node = match budget.nested(quota - placed, |nested| row(entry, nested)) {
            Ok(node) => node,
            Err(error) if error.code == "ui.fixed-capacity" => {
                placed -= 1;
                break;
            }
            Err(error) => return Err(error),
        };
        if items.try_push(node).is_err() {
            placed -= 1;
            break;
        }
    }
    if placed < slice.len() {
        if budget.spend() {
            if let Ok(more) = continuation_row_from(section_id, slice.len() - placed, Some(page as u32 + 1)) {
                let _ = items.try_push(more);
            }
        }
    }
    Ok(items)
}
//#endregion 🔖️Paging

//#region 🔖️Render
/// 🌳️ The four document sections as one bounded page, memoized by the app against the fixture's geometry
/// fingerprint and the resolved label set.
pub fn render(fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels) -> UiAssemblyResult<BuiltNode> {
    render_from(fixture, labels, &HashMap::new())
}

/// 📄 Same tree as [`render`], starting each section at its `setPanelPage` cursor.
pub fn render_from(fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels, pages: &HashMap<String, u32>) -> UiAssemblyResult<BuiltNode> {
    let budget = &mut RowBudget::new(page_rows_for(fixture));
    let objects = budget.nested(SECTIONS - 1, |share| paged_section_from(&format!("{ROOT}.objects"), &fixture.objects, pages, share, |object, share| object_row(object, labels, share)))?;
    let references = budget.nested(SECTIONS - 2, |share| paged_section_from(&format!("{ROOT}.references"), &fixture.references, pages, share, |reference, _| reference_row(reference, labels)))?;
    let target_volumes = budget.nested(SECTIONS - 3, |share| paged_section_from(&format!("{ROOT}.target-volumes"), &fixture.target_volumes, pages, share, |volume, _| target_volume_row(volume, labels)))?;
    let attractions = budget.nested(SECTIONS - 4, |share| paged_section_from(&format!("{ROOT}.attractions"), &fixture.attractions, pages, share, |attraction, _| attraction_row(attraction)))?;
    PanelTreeBuilder::new(ROOT)?
        .section(format!("{ROOT}.objects"), Some(ui_label(labels.objects.as_str())?), true, objects)?
        .section(format!("{ROOT}.references"), Some(ui_label(labels.references.as_str())?), false, references)?
        .section(format!("{ROOT}.target-volumes"), Some(ui_label(labels.target_volumes.as_str())?), false, target_volumes)?
        .section(format!("{ROOT}.attractions"), Some(ui_label(labels.attractions.as_str())?), false, attractions)?
        .interaction_domain(PUZZLE3D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
