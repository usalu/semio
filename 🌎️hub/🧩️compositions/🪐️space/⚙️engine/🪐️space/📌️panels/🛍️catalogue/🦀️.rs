//! 🛍️ S Studio app — app catalogue panel: the drag-source tree of every registered plugin app,
//! nested by canonical document breadcrumb.

use crate::engine::space::terminology::SStudioLabels;
use crate::engine::space::S_PLAY_CATALOGUE_BODY_KEY;
use semio_framework_os::{os_app_primary_output_kind, os_app_registration, workflow_palette};
use semio_framework_plugin::plugin_app_close_prelude::{BuiltNode, HasBase};
use semio_framework_plugin::{tree_window_item, Locale, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, Terminology, TreeWindows, UiFixedMap, UiText};
use semio_framework_ui_contract as ui;
use std::collections::BTreeMap;

//#region 🔖️Manifest
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(crate::engine::space::S_PLAY_CATALOGUE_TAB_ID.into()),
        label: semio_framework_plugin::LocalizedLabel::native(semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(S_PLAY_CATALOGUE_BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Manifest

//#region 🔖️Render
#[derive(Default)]
struct AppCatalogueNode {
    children: BTreeMap<String, AppCatalogueNode>,
    app: Option<CatalogueAppEntry>,
}

/// 🎨️ One catalogue leaf's presentation — a thin projection of `registry::AppPaletteEntry` (the
/// `workflow_palette()` entry) plus its resolved `document` breadcrumb/`yields`, both sourced from
/// `os_app_registration` (`AppPaletteEntry` itself doesn't carry them). Built fresh from the registry
/// every render, never cached in config.
struct CatalogueAppEntry {
    plugin_id: String,
    app_id: String,
    label: String,
    yields: String,
}

/// 🌳️ One top-level catalogue branch — the first breadcrumb segment, or (for an app whose
/// `registration.breadcrumb` is empty) that app's own id with its registry label.
struct CatalogueRoot {
    segment: String,
    label: String,
    node: AppCatalogueNode,
}

/// 🌳️ Builds a catalogue tree item on top of the SDK's windowed group row — only the per-app
/// drag-data/icon extensions are this app's own concern.
///
/// 🪟️ Every level is a `tree_window_item` keyed by its own authored id, and every BRANCH is authored
/// CLOSED (`default_open: false`): this tree enumerates the whole installed-app registry, not one
/// document, so eagerly expanding each branch (the pre-virtualisation `default_open(!children
/// .is_empty())`) materialised the entire registry at every depth on every render. Closed now means a
/// branch stamps its full `total` and materialises its children only once the HOST opens it — lazy
/// expansion, host-owned, surviving refreshes.
fn app_catalogue_item(windows: &TreeWindows<'_>, id_path: &str, label: &str, node: &AppCatalogueNode) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let node_key = format!("s-play-catalogue.document.{id_path}");
    let children: Vec<(&String, &AppCatalogueNode)> = node.children.iter().collect();
    let mut item = ui::tree_item(ui::Label::try_from(label).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue row label admission failed"))?)
        .try_id(node_key.clone())
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue row id admission failed"))?;
    if let Some(app) = &node.app {
        item = item.icon(UiText::try_from_str(&app.app_id).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue icon admission failed"))?);
        if !app.yields.is_empty() {
            item = item.description(UiText::try_from_str(&app.yields).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue description admission failed"))?);
        }
        let mut drag_data = UiFixedMap::default();
        let key = UiText::try_from_str(crate::engine::space::S_PLAY_CATALOGUE_DRAG_MIME).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue drag mime admission failed"))?;
        let value = UiText::try_from_string(pack::json!({ "pluginId": app.plugin_id.as_str(), "appId": app.app_id.as_str(), "label": app.label.as_str() }).to_string())
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue drag payload admission failed"))?;
        drag_data.try_push(key, value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue drag map admission failed"))?;
        item = item.draggable(true).drag_data(drag_data);
    }
    tree_window_item(windows, item, &node_key, false, &children, |(segment, child)| app_catalogue_item(windows, &format!("{id_path}.{segment}"), segment.as_str(), child))
}

/// 🎨️ Builds the app catalogue tree straight from the production registry — `workflow_palette()`
/// (every registered `(plugin_id, app_id)`) joined with `os_app_registration` for the document
/// breadcrumb/primary output kind. Always live, never stale.
pub async fn build_catalogue_tree(labels: &SStudioLabels, locale: Locale, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut document = AppCatalogueNode::default();
    for entry in workflow_palette() {
        if entry.app_id == crate::engine::space::S_PLAY_APP_ID {
            continue;
        }
        let registration = os_app_registration(&entry.plugin_id, &entry.app_id);
        let doc_path = registration.as_ref().map(|row| row.breadcrumb.clone()).unwrap_or_default();
        let yields = registration.as_ref().map(os_app_primary_output_kind).unwrap_or_default();
        let mut node = &mut document;
        for segment in &doc_path {
            node = node.children.entry(segment.clone()).or_default();
        }
        // 🗺️ `AppPaletteEntry.label` is a full locale×terminology `LocalizedLabel` now; the catalogue
        // has no app-specific terminology axis of its own, so it always projects the `Native` cell at
        // the Studio app's own active locale.
        let label = entry.label.resolve(Terminology::Native, locale).to_string();
        node.app = Some(CatalogueAppEntry { plugin_id: entry.plugin_id, app_id: entry.app_id, label, yields });
    }
    let mut roots: Vec<CatalogueRoot> = document.children.into_iter().map(|(segment, node)| CatalogueRoot { label: segment.clone(), segment, node }).collect();
    // 🪹️ An app with an empty `breadcrumb` (`registration.breadcrumb == []`) has nowhere to
    // descend to in the loop above, so its `.app` lands on the ROOT `document` node itself rather than
    // inside `.children` — without this, it's silently dropped from the catalogue entirely. Surface it
    // as its own top-level leaf, keyed by `app_id` (there's no document segment to key off) with its
    // own registry label as the display text.
    if let Some(app) = document.app {
        roots.push(CatalogueRoot { segment: app.app_id.clone(), label: app.label.clone(), node: AppCatalogueNode { children: BTreeMap::new(), app: Some(app) } });
    }
    PanelTreeBuilder::new(crate::engine::space::S_PLAY_CATALOGUE_TAB_ID)?
        .window_section(
            windows,
            crate::engine::space::S_PLAY_CATALOGUE_TAB_ID,
            Some(ui::Label::try_from(labels.apps_section.as_str()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue section label admission failed"))?),
            true,
            &roots,
            |root| app_catalogue_item(windows, &root.segment, &root.label, &root.node),
        )?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
