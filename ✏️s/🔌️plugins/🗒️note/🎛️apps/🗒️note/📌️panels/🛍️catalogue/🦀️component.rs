//! 🛍️ Note play app panel — the block-kind catalogue: a read-only reference list.

use crate::apps::note::terminology::NotePlayLabels;
use semio_framework_plugin::{ui_declarative_sections_to_tree, ui_text, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const NOTE_PLAY_BODY_CATALOGUE: &str = "note.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), group: PanelGroup::Workbench, body_key: Some(NOTE_PLAY_BODY_CATALOGUE.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(labels: &NotePlayLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "note-catalogue".into(),
        label: Some(labels.catalogue_title.into()),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![ui_text(labels.catalogue_text), ui_text(labels.catalogue_image), ui_text(labels.catalogue_table), ui_text(labels.catalogue_math), ui_text(labels.catalogue_ink), ui_text(labels.catalogue_group)],
        menu: None,
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::note::testkit::{note_app, render as render_body};
    use crate::apps::note::NOTE_PLAY_BODY_CATALOGUE as BODY_CATALOGUE;

    #[test]
    fn catalogue_lists_every_block_kind() {
        let mut app = note_app();
        let json = render_body(&mut app, BODY_CATALOGUE);
        assert!(json.contains("Block kinds"));
        assert!(json.contains("text — rich text block"));
    }

    #[test]
    fn catalogue_resolves_german_locale() {
        use crate::apps::note::commands::locale::set_locale::SetLocale;
        use crate::apps::note::testkit::dispatch;
        use crate::apps::note::NoteCommand;

        let mut app = note_app();
        dispatch(&mut app, NoteCommand::SetLocale(SetLocale { value: "de-DE".into() }));
        let json = render_body(&mut app, BODY_CATALOGUE);
        assert!(json.contains("Blockarten"));
        assert!(json.contains("Text — reicher Textblock"));
    }
}
//#endregion 🧪️Tests
