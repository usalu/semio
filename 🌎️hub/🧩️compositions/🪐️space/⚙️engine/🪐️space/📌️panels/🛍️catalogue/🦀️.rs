//! 🛍️ S Studio app — app catalogue panel: the drag-source tree of every registered plugin app,
//! nested by canonical document breadcrumb.

use semio_hub_space_foundation::terminology::SStudioLabels;
use semio_hub_space_foundation::S_PLAY_CATALOGUE_BODY_KEY;
use semio_os::{os_app_primary_output_kind, os_app_registration, workflow_palette};
use semio_plugin::plugin_app_close_prelude::{BuiltNode, HasBase};
use semio_ui_locale::{Locale, Terminology};
use semio_plugin::{tree_window_item, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiFixedMap, UiText};
use semio_ui_contract as ui;
use std::{borrow::Cow, collections::BTreeMap};

//#region 🔖️Manifest
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(semio_hub_space_foundation::S_PLAY_CATALOGUE_TAB_ID.into()),
        label: semio_ui_locale::LocalizedLabel::native(semio_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
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
    apps: Vec<CatalogueAppEntry>,
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

/// 🍃️ A document breadcrumb or a distinct registered application within it.
#[derive(Clone, Copy)]
enum CatalogueItem<'a> {
    Branch(&'a str, &'a AppCatalogueNode),
    Application(&'a CatalogueAppEntry),
}

impl<'a> CatalogueItem<'a> {
    fn segment(self) -> Cow<'a, str> {
        match self {
            Self::Branch(segment, _) => Cow::Borrowed(segment),
            Self::Application(app) => Cow::Owned(format!("{}/{}", app.plugin_id, app.app_id)),
        }
    }
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
fn app_catalogue_item(windows: &TreeWindows<'_>, id_path: &str, row: CatalogueItem<'_>) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let node_key = format!("s-play-catalogue.document.{id_path}");
    let mut children = Vec::new();
    let (label, app) = match row {
        CatalogueItem::Branch(label, node) => {
            children.extend(node.children.iter().map(|(segment, child)| CatalogueItem::Branch(segment, child)));
            match node.apps.as_slice() {
                [app] => (label, Some(app)),
                apps => {
                    children.extend(apps.iter().map(CatalogueItem::Application));
                    (label, None)
                }
            }
        }
        CatalogueItem::Application(app) => (app.label.as_str(), Some(app)),
    };
    let mut item = ui::tree_item(ui::Label::try_from(label).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue row label admission failed"))?)
        .try_id(node_key.clone())
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue row id admission failed"))?;
    if let Some(app) = app {
        item = item.icon(UiText::try_from_str(&app.app_id).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue icon admission failed"))?);
        if !app.yields.is_empty() {
            item = item.description(UiText::try_from_str(&app.yields).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue description admission failed"))?);
        }
        let mut drag_data = UiFixedMap::default();
        let key = UiText::try_from_str(semio_hub_space_foundation::S_PLAY_CATALOGUE_DRAG_MIME).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue drag mime admission failed"))?;
        let value = UiText::try_from_string(semio_framework_pack_json::json!({ "pluginId": app.plugin_id.as_str(), "appId": app.app_id.as_str(), "label": app.label.as_str() }).to_string())
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue drag payload admission failed"))?;
        drag_data.try_push(key, value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue drag map admission failed"))?;
        item = item.draggable(true).drag_data(drag_data);
    }
    tree_window_item(windows, item, &node_key, false, &children, |child| {
        app_catalogue_item(windows, &format!("{id_path}.{}", child.segment()), *child)
    })
}

/// 🎨️ Builds the app catalogue tree straight from the production registry — `workflow_palette()`
/// (every registered `(plugin_id, app_id)`) joined with `os_app_registration` for the document
/// breadcrumb/primary output kind. Always live, never stale.
pub async fn build_catalogue_tree(labels: &SStudioLabels, locale: Locale, windows: &TreeWindows<'_>) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let mut document = AppCatalogueNode::default();
    for entry in workflow_palette() {
        if entry.app_id == semio_hub_space_foundation::S_PLAY_APP_ID {
            continue;
        }
        let registration = os_app_registration(&entry.plugin_id, &entry.app_id);
        let doc_path = registration.as_ref().map(|row| row.breadcrumb.clone()).unwrap_or_default();
        let yields = registration.as_ref().map(os_app_primary_output_kind).unwrap_or_default();
        let mut node = &mut document;
        for segment in &doc_path {
            node = node.children.entry(segment.clone()).or_default();
        }
        let label = entry.label.resolve(Terminology::Native, locale).to_string();
        node.apps.push(CatalogueAppEntry { plugin_id: entry.plugin_id, app_id: entry.app_id, label, yields });
    }
    let roots: Vec<_> = document.children.iter().map(|(segment, node)| CatalogueItem::Branch(segment, node))
        .chain(document.apps.iter().map(CatalogueItem::Application)).collect();
    PanelTreeBuilder::new(semio_hub_space_foundation::S_PLAY_CATALOGUE_TAB_ID)?
        .window_section(
            windows,
            semio_hub_space_foundation::S_PLAY_CATALOGUE_TAB_ID,
            Some(ui::Label::try_from(labels.apps_section.as_str()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue section label admission failed"))?),
            true,
            &roots,
            |root| app_catalogue_item(windows, &root.segment(), *root),
        )?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
