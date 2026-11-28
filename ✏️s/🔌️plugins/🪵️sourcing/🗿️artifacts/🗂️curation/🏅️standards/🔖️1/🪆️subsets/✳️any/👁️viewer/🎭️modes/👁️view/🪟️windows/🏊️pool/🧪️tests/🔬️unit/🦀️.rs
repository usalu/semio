use crate::stock_of;
use semio_plugin::app::TableWindowKit;
#[allow(unused_imports)]
use crate::CurationSnapshot;
#[allow(unused_imports)]
use semio_plugin::app::{TableView, WindowKit};
#[allow(unused_imports)]
use semio_plugin::{BuiltNode, UiAssemblyResult, WindowKindDefinition};

use crate::viewer::sourcing::modes::view::windows::pool::*;

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_framework_table_window_kit() {
    let def = definition();
    assert_eq!(def.id, TableWindowKit::KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn view_model_lists_every_stock_row_with_five_columns() {
    let document = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let stock = stock_of(&document);
    let view = view_model(&document, semio_plugin::Locale::En);
    assert_eq!(view.columns, ["Name", "Module", "Typology", "Availability", "Curated"]);
    assert_eq!(view.rows.len(), stock.len());
}

#[semio_framework_async_macros::async_test]
async fn view_model_is_localized_and_shows_the_curation_read_only() {
    let mut document = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let first = stock_of(&document).remove(0);
    crate::schema::domain::curation_set(&mut document, &first.id, 2);
    let german = view_model(&document, semio_plugin::Locale::De);
    assert_eq!(german.columns, ["Name", "Modul", "Typologie", "Verfügbarkeit", "Kuratiert"]);
    assert_eq!(german.rows[0][0], "Brettschichtholz GL24h 200×400");
    assert_eq!(german.rows[0][1], "Balken");
    assert_eq!(german.rows[0][2], "Balken / Massivholz / Brettschichtholz");
    assert_eq!(german.rows[0][4], "2");
    assert_eq!(view_model(&document, semio_plugin::Locale::En).rows[0][4], "2");
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_table_ui_node() {
    let document = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let json = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(render(&document, semio_plugin::Locale::En).expect("bounded table"))).expect("bounded retained fixture projection");
    assert!(json.contains("table"), "expected a table UiNode: {json}");
}
