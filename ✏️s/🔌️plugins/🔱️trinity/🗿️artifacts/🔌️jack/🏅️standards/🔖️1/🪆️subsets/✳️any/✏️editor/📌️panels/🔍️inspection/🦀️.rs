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

use semio_framework_plugin::{tree_item_desc, ui_node_list, Label, PanelTreeBuilder, PluginAssemblyError, UiAssemblyResult, BuiltNode};

pub(crate) fn render() -> UiAssemblyResult<BuiltNode> {
    let prompt = Label::try_from("Select one or more pieces").map_err(|_| PluginAssemblyError::new("trinity.inspection.label", "the fixed inspection label exceeds its UI bound"))?;
    let section_label = Label::try_from("Inspection").map_err(|_| PluginAssemblyError::new("trinity.inspection.section", "the fixed inspection section label exceeds its UI bound"))?;
    let items = ui_node_list([tree_item_desc("trinity-inspector.prompt", prompt, None)])?;
    PanelTreeBuilder::new("trinity-inspector")?.section("trinity-inspector.empty", Some(section_label), true, items)?.build()
}
