//! 🛍️ Note play app panel — the block-kind catalogue: a read-only reference list.

use crate::editor::note::terminology::NotePlayLabels;
use crate::editor::note::ui_label;
use semio_framework_ui_contract::{Buildable, HasBase};
use semio_framework_plugin::{BuiltNode, UiAssemblyResult, UiFixedList, PanelTreeBuilder, PluginAssemblyError, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const NOTE_PLAY_BODY_CATALOGUE: &str = "note.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(NOTE_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(labels: &NotePlayLabels) -> UiAssemblyResult<BuiltNode> {
    let mut children = UiFixedList::default();
    for (index, label) in [labels.catalogue_text, labels.catalogue_image, labels.catalogue_table, labels.catalogue_math, labels.catalogue_ink, labels.catalogue_group].into_iter().enumerate() {
        let child = semio_framework_ui_contract::text(ui_label(label.as_str())?).try_id(format!("note-catalogue.kind.{index}")).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note catalogue key admission failed"))?.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note catalogue text admission failed"))?;
        children.try_push(child).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note catalogue child admission failed"))?;
    }
    PanelTreeBuilder::new("note-catalogue")?.section("note-catalogue.section", Some(ui_label(labels.catalogue_title.as_str())?), true, children)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::note::testkit::{note_app, render as render_body};
    use crate::editor::note::NOTE_PLAY_BODY_CATALOGUE as BODY_CATALOGUE;

    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_every_block_kind() {
        let mut app = note_app().await;
        let json = render_body(&mut app, BODY_CATALOGUE).await;
        assert!(json.contains("Block kinds"));
        assert!(json.contains("text — rich text block"));
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_resolves_german_locale() {
        use crate::editor::note::commands::set_locale::SetLocale;
        use crate::editor::note::testkit::dispatch;
        use crate::editor::note::NoteCommand;

        let mut app = note_app().await;
        dispatch(&mut app, NoteCommand::SetLocale(SetLocale { value: "de-DE".into() })).await;
        let json = render_body(&mut app, BODY_CATALOGUE).await;
        assert!(json.contains("Blockarten"));
        assert!(json.contains("Text — reicher Textblock"));
    }
}
//#endregion 🧪️Tests
