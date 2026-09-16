
use super::*;
use semio_framework_os::{ArtifactPresentation, MediaClass, MediaForm, MediaType};
use semio_framework_plugin::{App, AppIo, LocalizedLabel, TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

async fn seed_app(plugin_id: &str, app_id: &str, label: &str, document: &[&str], artifact_schema: &str) {
    let definition = App::builder(app_id, LocalizedLabel::data(label))
        .await
        .document(document.iter().map(|segment| segment.to_string()))
        .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
        .await
        .window_kind("main", LocalizedLabel::native("Main", "Hauptansicht"), format!("{app_id}.main"), semio_framework_ui_contract::SurfaceKind::Canvas2d, "square-pen")
        .await
        .io(AppIo::from_artifact(document_schema, MediaType { class: MediaClass::Data, form: MediaForm::Value }, ArtifactPresentation { id: app_id.into(), name: label.into(), dimension: String::new(), component_kind: app_id.into() }).await)
        .await
        .build_definition();
    semio_framework_os::register_app_io(plugin_id, &definition);
}

/// 🛍️ The catalogue body exactly as the host reads it, for the host-known windows in `requests`.
async fn window_body(requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..Default::default() };
    let tree = build_catalogue_tree(semio_framework_plugin::resolve_labels::<SStudioLabels>(&ViewModel::default()), Locale::En, &TreeWindows::for_body(&view, S_PLAY_CATALOGUE_BODY_KEY)).await.expect("catalogue tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).expect("catalogue projection")
}

#[semio_framework_async_macros::async_test]
async fn catalogue_tree_nests_apps_by_canonical_document() {
    seed_app("puzzle", "s.puzzle2d@1/*#editor", "Puzzle 2D", &["semio", "puzzle", "2d"], "puzzle2d.document").await;
    seed_app("puzzle", "s.puzzle3d@1/*#editor", "Puzzle 3D", &["semio", "puzzle", "3d"], "puzzle3d.document").await;
    let body_key = S_PLAY_CATALOGUE_BODY_KEY;
    let json = window_body(vec![
        TreeWindowRequest { body_key: body_key.into(), node_key: "s-play-catalogue.document.semio".into(), open: Some(true), offset: 0, rows: 32 },
        TreeWindowRequest { body_key: body_key.into(), node_key: "s-play-catalogue.document.semio.puzzle".into(), open: Some(true), offset: 0, rows: 32 },
    ])
    .await;
    assert!(json.contains("s-play-catalogue.document.semio.puzzle.2d"), "an opened branch materialises its children: {json}");
    assert!(json.contains("s-play-catalogue.document.semio.puzzle.3d"), "an opened branch materialises its children: {json}");
    assert_eq!(json.matches("\"label\":\"puzzle\"").count(), 1);
}

//#region 🪟️WindowLaws
/// 🪟️ Law (a): every container stamps its FULL extent and materialises at most its slice — no `+N`.
#[semio_framework_async_macros::async_test]
async fn registry_catalogue_stamps_totals_and_never_a_continuation_row() {
    seed_app("puzzle", "s.puzzle2d@1/*#editor", "Puzzle 2D", &["semio", "puzzle", "2d"], "puzzle2d.document").await;
    let json = window_body(Vec::new()).await;
    assert!(json.contains("\"total\":"), "the roster section stamps its extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("s-play-catalogue.document.").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): a branch is authored CLOSED now, so it stamps its `total` and materialises zero
/// children until the host opens it — the lazy expansion this panel exists to get.
#[semio_framework_async_macros::async_test]
async fn closed_branch_stamps_total_and_materialises_no_children() {
    seed_app("puzzle", "s.puzzle2d@1/*#editor", "Puzzle 2D", &["semio", "puzzle", "2d"], "puzzle2d.document").await;
    seed_app("puzzle", "s.puzzle3d@1/*#editor", "Puzzle 3D", &["semio", "puzzle", "3d"], "puzzle3d.document").await;
    let json = window_body(Vec::new()).await;
    assert!(json.contains("s-play-catalogue.document.semio"), "the root branch is materialised: {json}");
    assert!(!json.contains("s-play-catalogue.document.semio.puzzle"), "a closed branch materialises no children at any depth: {json}");
    assert!(json.matches("\"total\":").count() >= 2, "the section AND the closed branch both stamp an extent: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw branch id.
#[semio_framework_async_macros::async_test]
async fn host_window_materialises_exactly_its_slice() {
    for index in 0..6 {
        seed_app("slice", &format!("s.slice{index}@1/*#editor"), &format!("Slice {index}"), &["semio", "slice", &format!("k{index}")], "slice.document").await;
    }
    let body_key = S_PLAY_CATALOGUE_BODY_KEY;
    let json = window_body(vec![
        TreeWindowRequest { body_key: body_key.into(), node_key: "s-play-catalogue.document.semio".into(), open: Some(true), offset: 0, rows: 32 },
        TreeWindowRequest { body_key: body_key.into(), node_key: "s-play-catalogue.document.semio.slice".into(), open: Some(true), offset: 2, rows: 2 },
    ])
    .await;
    assert!(json.contains("\"offset\":2"), "the opened branch reports its offset: {json}");
    assert!(json.contains("s-play-catalogue.document.semio.slice.k2"), "entry 2 is inside the window: {json}");
    assert!(json.contains("s-play-catalogue.document.semio.slice.k3"), "entry 3 is inside the window: {json}");
    assert!(!json.contains("s-play-catalogue.document.semio.slice.k1"), "the entry before the window stays out: {json}");
    assert!(!json.contains("s-play-catalogue.document.semio.slice.k4"), "the entry after the window stays out: {json}");
}

/// 🪟️ Law (d) for a deliberately UNBOUND tree: the catalogue mixes breadcrumb segments and app ids in
/// one id namespace and is not document-bound at all, so it declares no interaction domain and stamps
/// no pick granularity — every app leaf keeps its own drag payload instead.
#[semio_framework_async_macros::async_test]
async fn unbound_catalogue_keeps_row_payloads_and_declares_no_domain() {
    seed_app("puzzle", "s.puzzle2d@1/*#editor", "Puzzle 2D", &["semio", "puzzle", "2d"], "puzzle2d.document").await;
    let body_key = S_PLAY_CATALOGUE_BODY_KEY;
    let json = window_body(vec![
        TreeWindowRequest { body_key: body_key.into(), node_key: "s-play-catalogue.document.semio".into(), open: Some(true), offset: 0, rows: 32 },
        TreeWindowRequest { body_key: body_key.into(), node_key: "s-play-catalogue.document.semio.puzzle".into(), open: Some(true), offset: 0, rows: 32 },
    ])
    .await;
    assert!(!json.contains("interactionDomain"), "the catalogue binds no domain: {json}");
    assert!(!json.contains("granularity"), "an unbound tree stamps no pick granularity: {json}");
    assert!(json.contains("draggable"), "app leaves keep their own drag payload: {json}");
}
//#endregion 🪟️WindowLaws
