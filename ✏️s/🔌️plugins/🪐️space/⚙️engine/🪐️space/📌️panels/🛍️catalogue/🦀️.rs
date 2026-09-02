//! 🛍️ S Studio app — app catalogue panel: the drag-source tree of every registered plugin app,
//! nested by canonical document breadcrumb.

use crate::engine::space::terminology::SStudioLabels;
use crate::engine::space::S_PLAY_CATALOGUE_BODY_KEY;
use semio_framework_os::{os_app_primary_output_kind, os_app_registration, workflow_palette};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, BuiltNode, HasBase, HasChildren};
use semio_framework_plugin::{Locale, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, Terminology, UiFixedList, UiFixedMap, UiText};
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

/// 🌳️ Builds a catalogue tree item on top of the SDK's `tree_item_desc` skeleton — only the per-app
/// drag-data/icon/children extensions are this app's own concern.
fn app_catalogue_item(id_path: &str, label: &str, node: AppCatalogueNode) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut children = UiFixedList::<BuiltNode>::default();
    for (segment, child) in node.children {
        let child_path = format!("{id_path}.{segment}");
        let child = app_catalogue_item(&child_path, &segment, child)?;
        children.try_push(child).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue child admission failed"))?;
    }
    let app = node.app;
    let mut item = ui::tree_item(ui::Label::try_from(label).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue row label admission failed"))?)
        .try_id(format!("s-play-catalogue.document.{id_path}"))
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue row id admission failed"))?
        .default_open(!children.is_empty())
        .try_children(children)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue children admission failed"))?;
    if let Some(app) = &app {
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
    item.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue row admission failed"))
}

/// 🎨️ Builds the app catalogue tree straight from the production registry — `workflow_palette()`
/// (every registered `(plugin_id, app_id)`) joined with `os_app_registration` for the document
/// breadcrumb/primary output kind. Always live, never stale.
pub async fn build_catalogue_tree(labels: &SStudioLabels, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
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
    let mut items = UiFixedList::<BuiltNode>::default();
    for (segment, node) in document.children {
        let item = app_catalogue_item(&segment, &segment, node)?;
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue root admission failed"))?;
    }
    // 🪹️ An app with an empty `breadcrumb` (`registration.breadcrumb == []`) has nowhere to
    // descend to in the loop above, so its `.app` lands on the ROOT `document` node itself rather than
    // inside `.children` — without this, it's silently dropped from the catalogue entirely. Surface it
    // as its own top-level leaf, keyed by `app_id` (there's no document segment to key off) with its
    // own registry label as the display text.
    if let Some(app) = document.app {
        let id = app.app_id.clone();
        let label = app.label.clone();
        let item = app_catalogue_item(&id, &label, AppCatalogueNode { children: BTreeMap::new(), app: Some(app) })?;
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue ungrouped app admission failed"))?;
    }
    PanelTreeBuilder::new(crate::engine::space::S_PLAY_CATALOGUE_TAB_ID)?
        .section(
            crate::engine::space::S_PLAY_CATALOGUE_TAB_ID,
            Some(ui::Label::try_from(labels.apps_section.as_str()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "space catalogue section label admission failed"))?),
            true,
            items,
        )?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_os::{ArtifactPresentation, MediaClass, MediaForm, MediaType};
    use semio_framework_plugin::{App, AppIo, LocalizedLabel, SurfaceKind};

    async fn seed_app(plugin_id: &str, app_id: &str, label: &str, document: &[&str], document_schema: &str) {
        let definition = App::builder(app_id, LocalizedLabel::data(label))
            .document(document.iter().map(|segment| segment.to_string()))
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .window_kind("main", LocalizedLabel::native("Main", "Hauptansicht"), format!("{app_id}.main"), SurfaceKind::Canvas2d, "square-pen")
            .io(AppIo::from_document(document_schema, MediaType { class: MediaClass::Data, form: MediaForm::Value }, ArtifactPresentation { id: app_id.into(), name: label.into(), dimension: String::new(), component_kind: app_id.into() }))
            .build_definition();
        semio_framework_os::register_app_io(plugin_id, &definition);
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_tree_nests_apps_by_canonical_document() {
        seed_app("puzzle", "s.puzzle2d@1/*#editor", "Puzzle 2D", &["semio", "puzzle", "2d"], "puzzle2d.document");
        seed_app("puzzle", "s.puzzle3d@1/*#editor", "Puzzle 3D", &["semio", "puzzle", "3d"], "puzzle3d.document");
        let config = crate::engine::space::config::SpaceConfig::default();
        let tree = build_catalogue_tree(semio_framework_plugin::resolve_labels_for_locale::<SStudioLabels>(&config.locale), semio_framework_plugin::locale_from_str(&config.locale));
        let json = pack::to_json_string(&tree);
        assert!(json.contains("s-play-catalogue.document.semio.puzzle.2d"));
        assert!(json.contains("s-play-catalogue.document.semio.puzzle.3d"));
        assert_eq!(json.matches("\"label\":\"puzzle\"").count(), 1);
    }
}
//#endregion 🧪️Tests
