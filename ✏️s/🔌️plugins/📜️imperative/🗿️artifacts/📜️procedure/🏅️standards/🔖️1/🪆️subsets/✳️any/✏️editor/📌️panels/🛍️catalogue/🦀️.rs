//! 🛍️ Imperative play app panel — the action catalogue: the fixed set of step kinds a step can be
//! created from.

use crate::editor::procedure::terminology::ImperativeLabels;
use crate::editor::procedure::IMPERATIVE_PLAY_APP_ID;
use semio_framework_plugin::{tree_item_with_action, ActionFactory, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_BODY_CATALOGUE: &str = "imperative.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(IMPERATIVE_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(labels: &ImperativeLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let actions = [("state.set", labels.action_state_set), ("log.print", labels.action_log_print), ("control.if", labels.action_control_if), ("control.while", labels.action_control_while), ("math.add", labels.action_math_add)];
    let builder = PanelTreeBuilder::new("imperative-play-catalogue")?;
    let action_factory = ActionFactory::new(IMPERATIVE_PLAY_APP_ID);
    let mut action_items = semio_framework_plugin::UiFixedList::default();
    for (kind, label) in actions {
        let kind_value = semio_framework_plugin::UiText::try_from_str(kind)
            .map(semio_framework_plugin::UiValue::Text)
            .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.kind", "fixed action kind admission failed"))?;
        let mut args = semio_framework_plugin::UiMapBuilder::try_new()
            .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.args", "fixed action argument map admission failed"))?;
        args.push("kind".to_owned(), kind_value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.args.kind", "fixed action kind argument admission failed"))?;
        let item = tree_item_with_action(builder.item_id("action", kind)?, label.as_str(), Some(kind.into()), action_factory.action("addStep", Some(semio_framework_plugin::UiValue::Map(args.finish())))?)?;
        action_items
            .try_push(item)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.items", "fixed catalogue item admission failed"))?;
    }
    builder.section("imperative-play-catalogue.actions", Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()), true, action_items)?.selected([])?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedure::testkit::{imperative_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_step_kinds_in_native_locale_by_default() {
        let mut app = imperative_app().await;
        let json = render_body(&mut app, IMPERATIVE_PLAY_BODY_CATALOGUE).await;
        assert!(json.contains("Set state"));
        assert!(json.contains("Print log"));
        assert!(json.contains("While"));
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_resolves_native_german_from_the_config_locale() {
        use crate::editor::procedure::commands::set_locale;
        use crate::editor::procedure::testkit::dispatch;
        use crate::editor::procedure::ImperativeCommand;
        let mut app = imperative_app().await;
        dispatch(&mut app, ImperativeCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() })).await;
        let json = render_body(&mut app, IMPERATIVE_PLAY_BODY_CATALOGUE).await;
        assert!(json.contains("Zustand setzen"));
        assert!(json.contains("Log ausgeben"));
        assert!(json.contains("Solange"));
    }
}
//#endregion 🧪️Tests
