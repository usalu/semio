//! 📄️ Puzzle 3d play app panel — the document tree: objects (with their vortices nested), reference
//! planes, target volumes and attractions, each row selecting its entity and carrying inline
//! hide/lock actions. The rendered sections are memoized by `Puzzle3dPlayApp` against the fixture's
//! geometry fingerprint, so this builder only reruns when the document actually changes.

use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{
    puzzle3d_vortex_full_id, ui_label, Puzzle3dFixture, PUZZLE3D_GRANULARITY_ATTRACTION, PUZZLE3D_GRANULARITY_OBJECT, PUZZLE3D_GRANULARITY_REFERENCE, PUZZLE3D_GRANULARITY_TARGET_VOLUME, PUZZLE3D_GRANULARITY_VORTEX, PUZZLE3D_INTERACTION_DOMAIN,
    PUZZLE3D_PLAY_CONTROLLER_ID,
};
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, Buildable, BuiltNode, HasBase, HasChildren, Label, RowAction, RowActionPlacement, Trigger};
use semio_framework_plugin::{
    ActionFactory, InteractionTarget, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiAssemblyResult, UiFixedList, UiText, UiValue, FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
    FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, INTERACTION_SELECT_ACTION_ID,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.3d.play.document";
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
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
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

fn flag_args(entity: &str, id: &str, flag: &str) -> UiAssemblyResult<UiValue> {
    ui_value_map([("entity", ui_value_text(entity)?), ("flag", ui_value_text(flag)?), ("ids", ui_value_list([ui_value_text(id)?])?), ("value", ui_value_bool(true))])
}

fn hide_lock_actions(hidden: bool, locked: bool, labels: &Puzzle3dLabels, entity: &str, id: &str) -> UiAssemblyResult<[RowAction; 2]> {
    Ok([
        RowAction {
            icon: UiText::try_from_str(if hidden { "eye-off" } else { "eye" }).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d visibility icon admission failed"))?,
            label: Some(ui_label(if hidden { labels.show.as_str() } else { labels.hide.as_str() })?),
            action: binding(action("setSelectionFlag", Some(flag_args(entity, id, "hidden")?)))?,
            placement: RowActionPlacement::Row,
        },
        RowAction {
            icon: UiText::try_from_str(if locked { "lock" } else { "lock-open" }).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d lock icon admission failed"))?,
            label: Some(ui_label(if locked { labels.unlock.as_str() } else { labels.lock.as_str() })?),
            action: binding(action("setSelectionFlag", Some(flag_args(entity, id, "locked")?)))?,
            placement: RowActionPlacement::Row,
        },
    ])
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🌳️ The four document sections, memoized by the app against the fixture's geometry fingerprint.
pub fn render(fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut object_items = UiFixedList::<BuiltNode>::default();
    for object in &fixture.objects {
        let mut vortex_items = UiFixedList::<BuiltNode>::default();
        for vortex in &object.vortices {
            let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
            let item = selectable_item(&full_id, vortex.vortex_kind.clone().unwrap_or_else(|| vortex.id.clone()), "circle-dot", select_action(PUZZLE3D_GRANULARITY_VORTEX, &full_id))?
                .try_build()
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d vortex row admission failed"))?;
            vortex_items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d vortex list admission failed"))?;
        }
        let mut item = selectable_item(&object.id, object.object_kind.clone().unwrap_or_else(|| object.id.clone()), "box", select_action(PUZZLE3D_GRANULARITY_OBJECT, &object.id))?
            .default_open(false)
            .dimmed(object.hidden)
            .try_children(vortex_items)
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d object children admission failed"))?;
        for row_action in hide_lock_actions(object.hidden, object.locked, labels, "object", &object.id)? {
            item = item.try_row_action(row_action).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d object row action admission failed"))?;
        }
        let item = item.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d object row admission failed"))?;
        object_items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d object list admission failed"))?;
    }
    let mut reference_items = UiFixedList::<BuiltNode>::default();
    for reference in &fixture.references {
        let mut item = selectable_item(&reference.id, reference.id.clone(), "globe", select_action(PUZZLE3D_GRANULARITY_REFERENCE, &reference.id))?
            .description(UiText::try_from_str(&reference.source.url).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d reference description admission failed"))?)
            .dimmed(reference.hidden);
        for row_action in hide_lock_actions(reference.hidden, reference.locked, labels, "reference", &reference.id)? {
            item = item.try_row_action(row_action).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d reference row action admission failed"))?;
        }
        let item = item.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d reference row admission failed"))?;
        reference_items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d reference list admission failed"))?;
    }
    let mut target_volume_items = UiFixedList::<BuiltNode>::default();
    for volume in &fixture.target_volumes {
        let mut item = selectable_item(&volume.id, volume.id.clone(), "cylinder", select_action(PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume.id))?.dimmed(volume.hidden);
        for row_action in hide_lock_actions(volume.hidden, volume.locked, labels, "targetVolume", &volume.id)? {
            item = item.try_row_action(row_action).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d target-volume row action admission failed"))?;
        }
        let item = item.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d target-volume row admission failed"))?;
        target_volume_items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d target-volume list admission failed"))?;
    }
    let mut attraction_items = UiFixedList::<BuiltNode>::default();
    for attraction in &fixture.attractions {
        let item = selectable_item(&attraction.id, format!("{} → {}", attraction.attracting, attraction.attracted), "link", select_action(PUZZLE3D_GRANULARITY_ATTRACTION, &attraction.id))?
            .try_build()
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d attraction row admission failed"))?;
        attraction_items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d attraction list admission failed"))?;
    }
    PanelTreeBuilder::new("puzzle3d-play-document")?
        .section("puzzle3d-play-document.objects", Some(ui_label(labels.objects.as_str())?), true, object_items)?
        .section("puzzle3d-play-document.references", Some(ui_label(labels.references.as_str())?), false, reference_items)?
        .section("puzzle3d-play-document.target-volumes", Some(ui_label(labels.target_volumes.as_str())?), false, target_volume_items)?
        .section("puzzle3d-play-document.attractions", Some(ui_label(labels.attractions.as_str())?), false, attraction_items)?
        .interaction_domain(PUZZLE3D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render
