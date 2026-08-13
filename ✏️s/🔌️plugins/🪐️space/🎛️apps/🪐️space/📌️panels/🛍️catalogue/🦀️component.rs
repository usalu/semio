//! 🛍️ S Studio app — app catalogue panel: the drag-source tree of every registered plugin app,
//! nested by canonical document breadcrumb.

use crate::apps::space::terminology::SStudioLabels;
use crate::apps::space::S_PLAY_CATALOGUE_BODY_KEY;
use semio_framework_os::{os_app_primary_output_kind, os_app_registration, workflow_palette};
use semio_framework_plugin::{tree_item_desc, IconName, Label, Locale, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, Terminology, UiNode, UiTreeItemNode};
use serde_json::json;
use std::collections::{BTreeMap, HashMap};

//#region 🔖️Manifest
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(crate::apps::space::S_PLAY_CATALOGUE_TAB_ID.into()),
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
fn app_catalogue_item(path: &[String], label: &str, node: AppCatalogueNode) -> UiTreeItemNode {
    let id_path = path.join(".");
    let children = node
        .children
        .into_iter()
        .map(|(segment, child)| {
            let mut child_path = path.to_vec();
            child_path.push(segment.clone());
            app_catalogue_item(&child_path, &segment, child)
        })
        .collect::<Vec<_>>();
    let app = node.app;
    let description = app.as_ref().and_then(|entry| (!entry.yields.is_empty()).then(|| entry.yields.clone()));
    let mut item = tree_item_desc(format!("s-play-catalogue.document.{id_path}"), Label::data(label), description);
    item.icon_id = app.as_ref().and_then(|entry| IconName::from_str(&entry.app_id));
    item.default_open = (!children.is_empty()).then_some(true);
    if let Some(app) = &app {
        let mut drag_data = HashMap::new();
        drag_data.insert(crate::apps::space::S_PLAY_CATALOGUE_DRAG_MIME.into(), json!({ "pluginId": app.plugin_id, "appId": app.app_id, "label": app.label }).to_string());
        item.draggable = Some(true);
        item.drag_data = Some(drag_data);
    }
    item.items = (!children.is_empty()).then_some(children);
    item
}

/// 🎨️ Builds the app catalogue tree straight from the production registry — `workflow_palette()`
/// (every registered `(plugin_id, app_id)`) joined with `os_app_registration` for the document
/// breadcrumb/primary output kind. Always live, never stale.
pub fn build_catalogue_tree(labels: &SStudioLabels, locale: Locale) -> UiNode {
    let mut document = AppCatalogueNode::default();
    for entry in workflow_palette() {
        if entry.app_id == crate::apps::space::S_PLAY_APP_ID {
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
    let mut items: Vec<UiTreeItemNode> = document.children.into_iter().map(|(segment, node)| app_catalogue_item(std::slice::from_ref(&segment), &segment, node)).collect();
    // 🪹️ An app with an empty `breadcrumb` (`registration.breadcrumb == []`) has nowhere to
    // descend to in the loop above, so its `.app` lands on the ROOT `document` node itself rather than
    // inside `.children` — without this, it's silently dropped from the catalogue entirely. Surface it
    // as its own top-level leaf, keyed by `app_id` (there's no document segment to key off) with its
    // own registry label as the display text.
    if let Some(app) = document.app {
        let id = app.app_id.clone();
        let label = app.label.clone();
        items.push(app_catalogue_item(&[id], &label, AppCatalogueNode { children: BTreeMap::new(), app: Some(app) }));
    }
    PanelTreeBuilder::new(crate::apps::space::S_PLAY_CATALOGUE_TAB_ID).section(crate::apps::space::S_PLAY_CATALOGUE_TAB_ID, Some(labels.apps_section.into()), true, items).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_os::{ArtifactPresentation, MediaClass, MediaForm, MediaType};
    use semio_framework_plugin::{App, AppIo, LocalizedLabel, SurfaceKind};

    fn seed_app(plugin_id: &str, app_id: &str, label: &str, document: &[&str], document_schema: &str) {
        let definition = App::builder(app_id, LocalizedLabel::data(label))
            .document(document.iter().map(|segment| segment.to_string()))
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .window_kind("main", LocalizedLabel::native("Main", "Hauptansicht"), format!("{app_id}.main"), SurfaceKind::Canvas2d, "square-pen")
            .io(AppIo::from_document(document_schema, MediaType { class: MediaClass::Data, form: MediaForm::Value }, ArtifactPresentation { id: app_id.into(), name: label.into(), dimension: String::new(), component_kind: app_id.into() }))
            .build_definition();
        semio_framework_os::register_app_io(plugin_id, &definition);
    }

    #[test]
    fn catalogue_tree_nests_apps_by_canonical_document() {
        seed_app("puzzle", "puzzle2d-play", "Puzzle 2D", &["semio", "puzzle", "2d"], "puzzle2d.document");
        seed_app("puzzle", "puzzle3d-play", "Puzzle 3D", &["semio", "puzzle", "3d"], "puzzle3d.document");
        let config = crate::apps::space::config::SpaceConfig::default();
        let tree = build_catalogue_tree(semio_framework_plugin::resolve_labels_for_locale::<SStudioLabels>(&config.locale), semio_framework_plugin::locale_from_str(&config.locale));
        let json = serde_json::to_string(&tree).unwrap();
        assert!(json.contains("s-play-catalogue.document.semio.puzzle.2d"));
        assert!(json.contains("s-play-catalogue.document.semio.puzzle.3d"));
        assert_eq!(json.matches("\"label\":\"puzzle\"").count(), 1);
    }
}
//#endregion 🧪️Tests
