//! 🛍️ Imperative play app panel — the action catalogue: the fixed set of step kinds a step can be
//! created from.

use crate::editor::imperative::imperative_action;
use crate::editor::imperative::terminology::ImperativeLabels;
use semio_framework_plugin::{tree_item_with_action, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_BODY_CATALOGUE: &str = "imperative.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
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
pub async fn render(labels: &ImperativeLabels) -> UiNode {
    let actions = [("state.set", labels.action_state_set), ("log.print", labels.action_log_print), ("control.if", labels.action_control_if), ("control.while", labels.action_control_while), ("math.add", labels.action_math_add)];
    let builder = PanelTreeBuilder::new("imperative-play-catalogue");
    let action_items: Vec<UiTreeItemNode> = actions.iter().map(|(kind, label)| tree_item_with_action(builder.item_id("action", kind), *label, Some((*kind).into()), imperative_action("addStep", Some(json!({ "kind": kind }))))).collect();
    builder.section("imperative-play-catalogue.actions", Some(semio_framework_plugin::Label::data(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)), true, action_items).selected(vec![]).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::imperative::testkit::{imperative_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_step_kinds_in_native_locale_by_default() {
        let mut app = imperative_app();
        let json = render_body(&mut app, IMPERATIVE_PLAY_BODY_CATALOGUE);
        assert!(json.contains("Set state"));
        assert!(json.contains("Print log"));
        assert!(json.contains("While"));
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_resolves_native_german_from_the_config_locale() {
        use crate::editor::imperative::commands::set_locale;
        use crate::editor::imperative::testkit::dispatch;
        use crate::editor::imperative::ImperativeCommand;
        let mut app = imperative_app();
        dispatch(&mut app, ImperativeCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        let json = render_body(&mut app, IMPERATIVE_PLAY_BODY_CATALOGUE);
        assert!(json.contains("Zustand setzen"));
        assert!(json.contains("Log ausgeben"));
        assert!(json.contains("Solange"));
    }
}
//#endregion 🧪️Tests
