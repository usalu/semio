//! 🧰️ Document helpers shared by two or more command modules: play actions, UI values, selection, config mutations and the workflow context menu.

use super::*;

//#region 🔖️DocumentHelpers
pub(crate) fn s_play_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(S_PLAY_CONTROLLER_ID).action(action, args)
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

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: builds a framework `interactionSelect`
/// action targeting one `(granularity, id)` pair in the `graph` domain — replaces the deleted
/// `selectInstance`/`nodeGraphSelect`/`setMediaNodeSelection`/`setAppInstanceSelection` action builders
/// every measure/document row used to construct by hand.
pub(crate) async fn space_interaction_select(granularity: &str, id: &str) -> ActionDescriptor {
    let targets = semio_framework_pack_json::to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    ActionDescriptor {
        controller_id: S_PLAY_CONTROLLER_ID.into(),
        action: INTERACTION_SELECT_ACTION_ID.into(),
        args: Some(DslValue::object(vec![
            ("domainId".to_string(), DslValue::String(S_PLAY_INTERACTION_DOMAIN.into())),
            ("targets".to_string(), DslValue::String(targets)),
            ("merge".to_string(), DslValue::String("replace".into())),
            ("method".to_string(), DslValue::String("pick".into())),
        ])),
    }
}

/// 🔧️ Small pure fold applying a batch of `SpaceConfigMutation`s onto a snapshot — used where a
/// command handler needs the POST-command config (not the pre-command `cfg.snapshot`) to build a
/// derived side value (the presence broadcast) in the very same call, without reaching back into a
/// store this pure function doesn't own.
#[cfg(any(test, feature = "testkit"))]
pub(crate) async fn apply_config_mutations(config: &SpaceConfig, operations: &[SpaceConfigMutation]) -> SpaceConfig {
    use protocol::Mutation;
    operations.iter().fold(config.clone(), |acc, operation| operation.diff(&acc).diff().clone())
}

/// 🫀️ Remote presence is host-owned. Until `ArtifactHost` threads the typed `PresenceView` into render,
/// Space shows no remote peers instead of retaining cross-instance state in a process-global registry.
pub(crate) async fn presence_peers_json(_app: &SpaceApp, _config: &SpaceConfig) -> String {
    "[]".into()
}

/// 🖱️ On-demand space workflow context menu from hit-test and selection snapshot.
pub(super) async fn space_workflow_context_menu_items(
    registry: &semio_framework_plugin::AppActionRegistry,
    labels: &SStudioLabels,
    axes: &semio_framework_plugin::ViewModel,
    surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>,
    selected_node_ids: &[String],
) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, selection_domains_from_surface, ContextMenuItemSpec, Menu};

    let hits: &[semio_framework_plugin::ContextMenuHit] = surface.map_or(&[], |target| target.hits.as_slice());
    let (nodes, _) = selection_domains_from_surface(surface, selected_node_ids, &[]);
    let hit_node = hits.iter().find(|hit| hit.domain == "node").map(|hit| hit.id.as_str());
    let mut menu = Menu::of(registry, axes);
    if hits.is_empty() {
        menu = menu
            .item(ContextMenuItemSpec { id: "paste-instance".into(), label: Some(labels.context_paste.into()), icon: Some("clipboard".into()), action: Some("pasteAppInstance".into()), ..Default::default() })
            
            .item(ContextMenuItemSpec { id: "select-all".into(), label: Some(labels.context_select_all.into()), icon: Some("maximize-2".into()), action: Some(SELECT_ALL_ACTION_ID.into()), ..Default::default() })
            
            .group("transform", |m| m.item(ContextMenuItemSpec { id: "reorganize".into(), label: Some(labels.context_reorganize.into()), icon: Some("layout-grid".into()), action: Some("reorganizeWorkflow".into()), ..Default::default() }))
            ;
    }
    if hit_node.is_some() || !nodes.is_empty() {
        menu = menu
            .item(ContextMenuItemSpec { id: "open-instance".into(), label: Some(labels.context_open_instance.into()), icon: Some("external-link".into()), action: Some("openInstance".into()), ..Default::default() })
            
            .item(ContextMenuItemSpec { id: "duplicate-instance".into(), label: Some(labels.context_duplicate.into()), icon: Some("copy".into()), action: Some("duplicateAppInstance".into()), ..Default::default() })
            
            .group("transfer", |m| m.item(ContextMenuItemSpec { id: "copy-instance".into(), label: Some(labels.context_copy.into()), icon: Some("clipboard-copy".into()), action: Some("copyAppInstance".into()), ..Default::default() }))
            
            .group("settings", |m| m.item(ContextMenuItemSpec { id: "rename-instance".into(), label: Some(labels.context_rename_label.into()), icon: Some("edit-3".into()), action: Some("renameAppInstance".into()), ..Default::default() }))
            ;
        if !nodes.is_empty() {
            menu = menu
                .group("selection", |m| {
                    m.item(ContextMenuItemSpec { id: "clear-selection".into(), label: Some(labels.context_clear_selection.into()), icon: Some("square-dashed".into()), action: Some(CLEAR_SELECTION_ACTION_ID.into()), ..Default::default() })
                })
                ;
        }
        let phrase = selection_count_phrase(axes.locale, &[(nodes.len().max(if hit_node.is_some() && nodes.is_empty() { 1 } else { 0 }), semio_framework_plugin::SelectionKind::Node)]);
        let remove_label = phrase.map_or_else(|| labels.context_remove.as_str().to_string(), |phrase| format!("{} ({phrase})", labels.context_remove.as_str()));
        let args = hit_node.map(|node_id| DslValue::object([("nodeId".to_string(), DslValue::String(node_id.into()))]));
        menu = menu.item(ContextMenuItemSpec { id: "remove-instance".into(), label: Some(remove_label), icon: Some("trash".into()), action: Some("removeAppInstance".into()), args, destructive: Some(true), ..Default::default() });
    }
    menu.build()
}
//#endregion 🔖️DocumentHelpers
