//! 🔍️ Trinity Rewrite app — Inspection panel (selected before-fixture node geometry/identity fields).
//!
//! 🕹️ ticket `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`: node selection is now
//! framework-owned (`InteractionView`), but `ArtifactApp::render` was NOT given an `interaction`
//! parameter (only `handle`/`copy_fragment`/`cut_operations` were, per the W3b plugin-SDK pass) —
//! see `semio_framework_plugin::ArtifactApp::render`'s signature. A per-selection details form built
//! purely inside `render` therefore has no data source anymore; this panel degrades to a static
//! prompt until a future wave either threads interaction state through `render` or restructures this
//! panel as an `interaction_domain`-bound tree the client filters by presence. Flagged, not silently
//! dropped (see `editor::jack::panels::inspection` for the identical trinity-wide gap).

use semio_framework_plugin::Label;

pub(crate) fn render() -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_framework_plugin::built_text_node(Label::data("Select one or more pieces")).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("trinity.inspection.label", "the fixed inspection label exceeds its UI bound"))
}
