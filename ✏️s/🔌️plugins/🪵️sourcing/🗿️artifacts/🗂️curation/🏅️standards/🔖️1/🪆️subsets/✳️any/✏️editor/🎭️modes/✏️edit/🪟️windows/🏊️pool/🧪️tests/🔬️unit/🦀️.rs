
use super::*;
use crate::Filters;
use crate::editor::sourcing::unit_tests::context::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn pool_render_respects_query_filter() {
    let document = crate::schema::default_document();
    let cfg = SourcingCurationConfig { filters: Filters { query: "glulam".into(), ..Default::default() }, ..Default::default() };
    let names: Vec<String> = pool_kinds(&document, &cfg).into_iter().map(|kind| kind.name).collect();
    assert!(names.iter().any(|name| name.contains("Glulam")));
    assert!(!names.iter().any(|name| name.contains("Hollow Core")));
}

#[semio_framework_async_macros::async_test]
async fn pool_row_carries_the_drag_payload_and_a_stepper_bounded_by_availability() {
    let document = crate::schema::default_document();
    let kind = crate::stock_of(&document).remove(0);
    let row: serde_json::Value = serde_json::from_str(&protocol::json::to_json_string(&pool_row(&document, &kind))).unwrap();
    assert_eq!(row["id"], kind.id.as_str());
    assert_eq!(row["_drag"]["objectId"], kind.id.as_str());
    assert_eq!(row["curated"]["kind"], "stepper");
    assert_eq!(row["curated"]["max"].as_f64().unwrap(), kind.availability as f64);
    assert_eq!(row["curated"]["action"]["action"], "curationSetCount");
    assert_eq!(row["curated"]["action"]["args"]["objectId"], kind.id.as_str());
}

#[semio_framework_async_macros::async_test]
async fn pool_scene_names_columns_by_id_and_drops_onto_the_pool() {
    let document = crate::schema::default_document();
    let node = render(&document, &SourcingCurationConfig::default(), crate::editor::sourcing::terminology::sourcing_curation_labels(&semio_framework_plugin::ViewModel::default())).expect("bounded pool");
    let semio_framework_plugin::Component::Surface(props) = node.component else { panic!("expected a table surface") };
    let scene: semio_framework_plugin::TableScene = semio_framework_ui_scene::decode(&props).expect("table scene");
    let columns: serde_json::Value = serde_json::from_str(&scene.columns_json).unwrap();
    assert_eq!(columns[0]["id"], "name");
    assert_eq!(scene.row_drag_mime.as_deref(), Some(crate::editor::sourcing::SOURCING_DRAG_MIME));
    assert!(scene.drop_action_json.expect("drop action").contains("dropOnPool"));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_table_surface_and_body_key() {
    let def = definition();
    assert_eq!(def.body_key, SOURCING_CURATION_BODY_POOL);
    assert!(matches!(def.surface_kind, SurfaceKind::Table));
}

#[semio_framework_async_macros::async_test]
async fn renders_pool_table_scene() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SOURCING_CURATION_BODY_POOL).await.contains("table"));
}
