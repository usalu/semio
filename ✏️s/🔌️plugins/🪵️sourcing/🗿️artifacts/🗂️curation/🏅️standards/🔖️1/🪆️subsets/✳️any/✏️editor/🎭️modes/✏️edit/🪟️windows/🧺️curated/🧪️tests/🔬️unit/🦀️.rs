
use super::*;
use crate::Filters;
use crate::editor::sourcing::SourcingCurationCommand;
use crate::editor::sourcing::commands::{curation_add, curation_remove, sort_table};
use crate::editor::sourcing::unit_tests::context::{dispatch, new_app, render as render_body, row_ids, table_of};

/// 🧫️ The demo stock with its two first kinds curated at different counts — the only fixture these
/// tests need to see the curated table's ordering and its row actions.
fn curated_document() -> CurationSnapshot {
    let mut document = crate::schema::default_document();
    let ids: Vec<String> = crate::stock_of(&document).iter().take(2).map(|kind| kind.id.clone()).collect();
    crate::schema::curation_set(&mut document, &ids[0], 3);
    crate::schema::curation_set(&mut document, &ids[1], 1);
    document
}

fn labels() -> &'static SourcingLabels {
    crate::editor::sourcing::terminology::sourcing_curation_labels(&semio_framework_plugin::ViewModel::default())
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_table_surface_and_body_key() {
    let def = definition();
    assert_eq!(def.body_key, SOURCING_CURATION_BODY_CURATED);
    assert!(matches!(def.surface_kind, SurfaceKind::Table));
}

#[semio_framework_async_macros::async_test]
async fn renders_curated_table_scene() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SOURCING_CURATION_BODY_CURATED).await.contains("table"));
}

/// ↕️ LAW: the curated headers are clickable to sort, and the scene echoes the active sort back so the
/// renderer can draw the indicator on the right column.
#[semio_framework_async_macros::async_test]
async fn curated_columns_are_sortable_and_echo_the_active_sort() {
    let cfg = SourcingCurationConfig { filters: Filters { sort: Some(crate::TableSort { column_id: "count".into(), direction: crate::SortDirection::Desc }), ..Default::default() }, ..Default::default() };
    let node = render(&curated_document(), &cfg, labels()).expect("bounded curated table");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("expected a table surface") };
    let scene: semio_framework_plugin::TableScene = semio_framework_ui_scene::decode(props).expect("table scene");
    let columns: serde_json::Value = serde_json::from_str(&scene.columns_json).unwrap();
    for index in 0..3 {
        assert_eq!(columns[index]["sortable"], true, "column {index} must be sortable");
    }
    assert_eq!(columns[3]["id"], "actions");
    assert!(scene.sort_json.expect("the active sort reaches the renderer").contains("count"));
    assert_eq!(scene.row_drag_mime.as_deref(), Some(crate::editor::sourcing::SOURCING_DRAG_MIME));
    assert!(scene.drop_action_json.expect("drop action").contains("dropOnCurated"));
}

/// ↕️ LAW: the shared `Filters::sort` reorders the curated rows when it names one of this table's own
/// columns, and leaves them in document order otherwise.
#[semio_framework_async_macros::async_test]
async fn curated_rows_follow_the_shared_sort_only_for_their_own_columns() {
    let document = curated_document();
    let document_order: Vec<String> = curated_rows(&document, &SourcingCurationConfig::default()).into_iter().map(|(item, _)| item.object_id).collect();
    let counts = |column: &str, direction: crate::SortDirection| {
        let cfg = SourcingCurationConfig { filters: Filters { sort: Some(crate::TableSort { column_id: column.into(), direction }), ..Default::default() }, ..Default::default() };
        curated_rows(&document, &cfg).into_iter().map(|(item, _)| item.count).collect::<Vec<_>>()
    };
    assert_eq!(counts("count", crate::SortDirection::Asc), vec![1, 3]);
    assert_eq!(counts("count", crate::SortDirection::Desc), vec![3, 1]);
    let foreign = SourcingCurationConfig { filters: Filters { sort: Some(crate::TableSort { column_id: "module".into(), direction: crate::SortDirection::Desc }), ..Default::default() }, ..Default::default() };
    assert_eq!(curated_rows(&document, &foreign).into_iter().map(|(item, _)| item.object_id).collect::<Vec<_>>(), document_order, "the pool's own column must not reorder the curated table");
}

/// 🧺️ LAW: a curated row carries at most two actions — curate one more while headroom remains, and
/// uncurate — both dispatching the same commands the pool and the drop targets use.
#[semio_framework_async_macros::async_test]
async fn curated_rows_offer_curate_and_uncurate_within_the_two_action_budget() {
    let mut document = curated_document();
    let object_id = document.curated[0].object_id.clone();
    let rows = curated_rows(&document, &SourcingCurationConfig::default());
    let (item, kind) = rows.iter().find(|(item, _)| item.object_id == object_id).expect("the curated row");
    let actions = curated_row_actions(item, kind, labels());
    assert_eq!(actions.len(), 2);
    assert_eq!(actions[0].action.action.as_str(), "curationAdd");
    assert_eq!(actions[1].action.action.as_str(), "curationRemove");
    let availability = kind.availability;
    crate::schema::curation_set(&mut document, &object_id, availability);
    let saturated = curated_rows(&document, &SourcingCurationConfig::default());
    let (item, kind) = saturated.iter().find(|(item, _)| item.object_id == object_id).expect("the saturated row");
    let actions = curated_row_actions(item, kind, labels());
    assert_eq!(actions.len(), 1, "a fully curated kind offers uncurate only");
    assert_eq!(actions[0].action.action.as_str(), "curationRemove");
}

/// ⚖️ LAW: dispatching the curated table's own controls through the live app changes what it renders —
/// a curate adds the row, a sort reorders it, an uncurate takes it away again.
#[semio_framework_async_macros::async_test]
async fn dispatching_curate_sort_and_uncurate_changes_the_rendered_curated_table() {
    let mut app = new_app().await;
    let stock: Vec<String> = row_ids(&mut app, crate::editor::sourcing::modes::edit::windows::pool::SOURCING_CURATION_BODY_POOL).await;
    let (first, second) = (stock[0].clone(), stock[1].clone());
    dispatch(&mut app, SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: first.clone() })).await;
    dispatch(&mut app, SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: second.clone() })).await;
    dispatch(&mut app, SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: second.clone() })).await;
    assert_eq!(row_ids(&mut app, SOURCING_CURATION_BODY_CURATED).await, vec![first.clone(), second.clone()]);
    dispatch(&mut app, SourcingCurationCommand::SortTable(sort_table::SortTable { column_id: "count".into(), direction: "desc".into() })).await;
    assert_eq!(row_ids(&mut app, SOURCING_CURATION_BODY_CURATED).await, vec![second.clone(), first.clone()], "the count sort puts the larger count first");
    let rows = table_of(&mut app, SOURCING_CURATION_BODY_CURATED).await.1;
    assert_eq!(rows[0]["count"]["value"].as_f64().unwrap(), 2.0);
    dispatch(&mut app, SourcingCurationCommand::CurationRemove(curation_remove::CurationRemove { object_id: second })).await;
    assert_eq!(row_ids(&mut app, SOURCING_CURATION_BODY_CURATED).await, vec![first]);
}
