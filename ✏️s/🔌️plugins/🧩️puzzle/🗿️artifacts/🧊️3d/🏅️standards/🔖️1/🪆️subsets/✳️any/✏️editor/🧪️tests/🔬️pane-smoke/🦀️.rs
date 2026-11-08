//! 🪧️ Aggregator pane smoke: the fixture the demonstrator page loads (Concrete Forest), every pane body in en and de, catalogue add of the page's kind into both views and the document row, then undo, redo and the exact retained close.

use crate::context::*;
use super::*;

/// 🔒️ The worker-job retirement slots are process-global: mounted laws of one binary run one at a time.
static AGGREGATOR_SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

const AGGREGATOR_SEED_ID: &str = "seed-left-001";
const AGGREGATOR_KIND_ID: &str = "Hexagonal Cut Concrete Forest Left";
const AGGREGATOR_MESH_URL: &str = "hexagonal-cut-concrete-forest-left.glb";
const AGGREGATOR_VORTEX_COUNT: usize = 11;
const AGGREGATOR_VIEW_IDS: [&str; 2] = ["puzzle3d-main-top", "puzzle3d-main-perspective"];

fn object_ids(app: &Puzzle3dApp) -> Vec<String> {
    let mut ids: Vec<String> = projection_of(app).get("objects").and_then(Value::as_array).into_iter().flatten().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    ids.sort();
    ids
}

async fn view_instance_ids(app: &mut Puzzle3dApp, view_id: &str) -> Vec<String> {
    let node = render_window_refresh(app, main::BODY_KEY, view_id).await;
    let mut ids: Vec<String> = instances_of(&node).iter().filter_map(|instance| instance.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    ids.sort();
    ids
}

async fn assert_every_pane_body_renders(app: &mut Puzzle3dApp, locale: semio_ui_locale::Locale) {
    app.set_label_axes(locale, semio_ui_locale::Terminology::Native);
    for view_id in AGGREGATOR_VIEW_IDS {
        let node = render_window_refresh(app, main::BODY_KEY, view_id).await;
        assert!(instance_count(&node) > 0, "{view_id} renders world instances in {locale:?}");
    }
    for body_key in [artifact::BODY_KEY, inspection::BODY_KEY, settings_panel::BODY_KEY, catalogue::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
        let body = render_panel_body(app, body_key, Some(AGGREGATOR_VIEW_IDS[0])).await;
        assert!(body.to_string().len() > 16, "{body_key} renders a non-empty body in {locale:?}");
    }
    assert_document_tree_in_catalog(app, locale).await;
}

/// 🔎️ The document tree carries the `objects` and `references` catalog strings of `locale`.
async fn assert_document_tree_in_catalog(app: &mut Puzzle3dApp, locale: semio_ui_locale::Locale) {
    let labels = puzzle3d_labels(&semio_plugin::ViewModel::new(locale, semio_ui_locale::Terminology::Native)).expect("admitted host axis");
    let tree = render_panel_body(app, artifact::BODY_KEY, Some(AGGREGATOR_VIEW_IDS[0])).await.to_string();
    for label in [labels.objects.as_str(), labels.references.as_str()] {
        assert!(tree.contains(label), "the document tree carries the {locale:?} catalog string {label:?}");
    }
}

/// 🧩️ LAW: the Aggregator pane loads the page's Concrete Forest fixture, renders every body, adds the catalogue kind into both views and the document, undoes, redoes and closes in en and de with the store drop witness satisfied and its catalog strings rendered.
#[semio_framework_async_macros::async_test]
async fn aggregator_pane_loads_applies_undoes_and_closes() {
    let _serial = AGGREGATOR_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    for locale in semio_plugin::artifact_app_laws::PANE_SMOKE_LOCALES {
        aggregator_cycle(locale).await;
    }
}

/// 🔁️ One whole load, render, apply, undo, redo and close cycle on a fresh app whose every dispatch and render runs under `locale`.
async fn aggregator_cycle(locale: semio_ui_locale::Locale) {
    let mut app = app().await;
    let loaded = object_ids(&app);
    assert_eq!(loaded.len(), CONCRETE_FOREST_EXAMPLE_SNAPSHOT.objects.len(), "the booted document is the page fixture");
    assert!(loaded.iter().any(|id| id == AGGREGATOR_SEED_ID), "the page fixture carries the seed object");

    assert_every_pane_body_renders(&mut app, locale).await;

    let mut loaded_views = Vec::new();
    for view_id in AGGREGATOR_VIEW_IDS {
        let ids = view_instance_ids(&mut app, view_id).await;
        assert_eq!(ids, loaded, "{view_id} shows exactly the document objects");
        loaded_views.push(ids);
    }

    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": AGGREGATOR_KIND_ID })), None).await.expect("catalogue add");
    let added = object_ids(&app);
    let new_ids: Vec<&String> = added.iter().filter(|id| !loaded.contains(id)).collect();
    assert_eq!(new_ids.len(), 1, "the catalogue add created exactly one object");
    let new_id = new_ids[0].clone();
    assert!(new_id.starts_with("puzzle3d.object."), "the new object id is minted by the document: {new_id}");

    for view_id in AGGREGATOR_VIEW_IDS {
        assert_eq!(view_instance_ids(&mut app, view_id).await, added, "{view_id} shows the new instance");
        let node = render_window_refresh(&mut app, main::BODY_KEY, view_id).await;
        assert!(scene_meshes_of(&node).iter().any(|mesh| mesh.to_string().contains(AGGREGATOR_MESH_URL)), "{view_id} publishes the kind mesh");
        let own_vortices = vortices_of(&node).iter().filter(|vortex| vortex.get("objectId").and_then(Value::as_str) == Some(new_id.as_str())).count();
        assert_eq!(own_vortices, AGGREGATOR_VORTEX_COUNT, "{view_id} shows the kind's vortices on the new instance");
    }
    let row = render_panel_body(&mut app, artifact::BODY_KEY, Some(AGGREGATOR_VIEW_IDS[0])).await.to_string();
    assert!(row.contains(&new_id) && row.contains(AGGREGATOR_KIND_ID), "the document tree carries the new row");

    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_ids(&app), loaded, "undo removes the added object");
    for (view_id, before) in AGGREGATOR_VIEW_IDS.iter().zip(&loaded_views) {
        assert_eq!(&view_instance_ids(&mut app, view_id).await, before, "{view_id} drops the instance on undo");
    }
    let row = render_panel_body(&mut app, artifact::BODY_KEY, Some(AGGREGATOR_VIEW_IDS[0])).await.to_string();
    assert!(!row.contains(&new_id), "the document tree drops the row on undo");

    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(object_ids(&app), added, "redo restores the added object");
    for view_id in AGGREGATOR_VIEW_IDS {
        assert_eq!(view_instance_ids(&mut app, view_id).await, added, "{view_id} shows the instance again on redo");
    }
    let row = render_panel_body(&mut app, artifact::BODY_KEY, Some(AGGREGATOR_VIEW_IDS[0])).await.to_string();
    assert!(row.contains(&new_id), "the document tree restores the row on redo");
    assert_document_tree_in_catalog(&mut app, locale).await;

    assert!(close_witness(&mut app).expect("original Puzzle3D app retirement"), "the app reaches terminal-empty ownership");
}

/// 🧩️ LAW: one catalogue add alone settles within the authored turn bound and grows the document by exactly one object, isolating the dispatch stall from every later smoke step.
#[semio_framework_async_macros::async_test]
async fn aggregator_catalogue_add_settles_alone() {
    let _serial = AGGREGATOR_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut app = app().await;
    let loaded = object_ids(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": AGGREGATOR_KIND_ID })), None).await.expect("catalogue add settles");
    assert_eq!(object_ids(&app).len(), loaded.len() + 1, "the catalogue add created exactly one object");
    assert!(close_witness(&mut app).expect("original Puzzle3D app retirement"), "the app reaches terminal-empty ownership");
}

/// ↩️ LAW: a framework-reserved verb admits under the fixture's original caller — undo after one catalogue add returns the document to its loaded objects.
#[semio_framework_async_macros::async_test]
async fn aggregator_reserved_undo_admits_under_the_original_caller() {
    let _serial = AGGREGATOR_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut app = app().await;
    let loaded = object_ids(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": AGGREGATOR_KIND_ID })), None).await.expect("catalogue add");
    dispatch(&mut app, "undo", None, None).await.expect("undo admits under the original caller");
    assert_eq!(object_ids(&app), loaded, "undo removes the added object");
    assert!(close_witness(&mut app).expect("original Puzzle3D app retirement"), "the app reaches terminal-empty ownership");
}
