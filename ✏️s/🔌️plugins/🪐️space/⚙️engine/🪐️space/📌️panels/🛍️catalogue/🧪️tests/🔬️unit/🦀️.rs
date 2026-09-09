
use super::*;
use semio_framework_os::{ArtifactPresentation, MediaClass, MediaForm, MediaType};
use semio_framework_plugin::{App, AppIo, LocalizedLabel};

async fn seed_app(plugin_id: &str, app_id: &str, label: &str, document: &[&str], document_schema: &str) {
    let definition = App::builder(app_id, LocalizedLabel::data(label))
        .await
        .document(document.iter().map(|segment| segment.to_string()))
        .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
        .await
        .window_kind("main", LocalizedLabel::native("Main", "Hauptansicht"), format!("{app_id}.main"), semio_framework_ui_contract::SurfaceKind::Canvas2d, "square-pen")
        .await
        .io(AppIo::from_document(document_schema, MediaType { class: MediaClass::Data, form: MediaForm::Value }, ArtifactPresentation { id: app_id.into(), name: label.into(), dimension: String::new(), component_kind: app_id.into() }).await)
        .await
        .build_definition();
    semio_framework_os::register_app_io(plugin_id, &definition);
}

#[semio_framework_async_macros::async_test]
async fn catalogue_tree_nests_apps_by_canonical_document() {
    seed_app("puzzle", "s.puzzle2d@1/*#editor", "Puzzle 2D", &["semio", "puzzle", "2d"], "puzzle2d.document").await;
    seed_app("puzzle", "s.puzzle3d@1/*#editor", "Puzzle 3D", &["semio", "puzzle", "3d"], "puzzle3d.document").await;
    let tree = build_catalogue_tree(semio_framework_plugin::resolve_labels::<SStudioLabels>(&semio_framework_plugin::ViewModel::default()), Locale::En).await.expect("catalogue tree");
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).expect("catalogue projection");
    assert!(json.contains("s-play-catalogue.document.semio.puzzle.2d"));
    assert!(json.contains("s-play-catalogue.document.semio.puzzle.3d"));
    assert_eq!(json.matches("\"label\":\"puzzle\"").count(), 1);
}
