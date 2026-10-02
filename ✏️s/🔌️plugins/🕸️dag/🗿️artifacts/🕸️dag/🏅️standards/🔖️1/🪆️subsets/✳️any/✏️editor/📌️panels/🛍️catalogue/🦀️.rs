//! 🛍️ DAG play app panel — the node-kind catalogue (drag/click-to-add palette).

use crate::editor::dag::terminology::DagPlayLabels;
use crate::editor::dag::{dag_action, ui_value_map, ui_value_text};
use semio_framework_plugin::tree_item_with_action;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL;

//#region 🔖️Constants
pub const DAG_PLAY_BODY_CATALOGUE: &str = "dag.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(DAG_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(labels: &DagPlayLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let kinds = [("computation", labels.kind_computation), ("slider", labels.kind_slider), ("select", labels.kind_select), ("screen", labels.kind_screen), ("note", labels.kind_note), ("preview", labels.kind_preview)];
    PanelTreeBuilder::new("dag-play-catalogue")?
        .window_section(
            windows,
            "dag-play-catalogue.node-kinds",
            Some(semio_framework_ui_contract::Label::try_from(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "catalogue heading admission failed"))?),
            true,
            &kinds,
            |(kind, label)| {
                let args = ui_value_map([("kind", ui_value_text(kind)?)])?;
                tree_item_with_action(format!("dag-play-catalogue.kind.{kind}"), label.as_str(), Some((*kind).into()), dag_action("addNode", Some(args))?)
            },
        )?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
