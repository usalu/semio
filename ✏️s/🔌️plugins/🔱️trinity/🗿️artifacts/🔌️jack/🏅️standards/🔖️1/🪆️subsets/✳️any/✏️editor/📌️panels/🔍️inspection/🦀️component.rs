//! 🔍️ Trinity Jack app — Inspection panel (selected node geometry/identity fields).
//!
//! 🕹️ ticket `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`: node selection is now
//! framework-owned (`InteractionView`), but `ArtifactApp::render` was NOT given an `interaction`
//! parameter (only `handle`/`copy_fragment`/`cut_operations` were, per the W3b plugin-SDK pass) —
//! see `semio_framework_plugin::ArtifactApp::render`'s signature. A per-selection details form built
//! purely inside `render` therefore has no data source anymore; this panel degrades to a static
//! prompt until a future wave either threads interaction state through `render` or restructures this
//! panel as an `interaction_domain`-bound tree the client filters by presence. Flagged, not silently
//! dropped.

use semio_framework_plugin::{ui_declarative_sections_to_tree, ui_text, Label, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

pub(crate) async fn render() -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "trinity-inspector.empty".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![ui_text(Label::data("Select one or more pieces"))],
        menu: None,
    }])
}
