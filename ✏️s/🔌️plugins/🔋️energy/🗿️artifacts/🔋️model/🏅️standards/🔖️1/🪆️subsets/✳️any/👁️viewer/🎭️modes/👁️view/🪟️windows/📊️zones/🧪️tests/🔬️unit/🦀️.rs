use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

/// 🏠️ One zone row of the table under test.
fn zone(id: u32, name: &str) -> crate::model::Zone {
    crate::model::Zone { id: crate::model::EntityId(id), name: name.into(), volume_m3: 129.6, multiplier: 1, conditioned: true, part_of_total_floor_area: true }
}

/// 📊️ The table's row records, read the way both hosts read them: the scene spine with its paged `rows` lane merged back in.
fn rows(table: &BuiltNode) -> Vec<serde_json::Value> {
    let scene = semio_framework_plugin::artifact_app_laws::built_surface_scene::<semio_framework_plugin::TableScene>(table).expect("the table scene decodes");
    serde_json::from_str(&scene.rows_json).expect("the rows lane is JSON")
}

#[semio_framework_async_macros::async_test]
async fn render_lists_one_row_per_zone() {
    let empty = render(&EnergyModelSnapshot::default()).expect("the empty table window assembles");
    assert_eq!(empty.key.as_str(), WINDOW_KIND_ID);
    assert!(rows(&empty).is_empty());
    let mut document = EnergyModelSnapshot::default();
    document.model.zones = vec![zone(1, "Ground floor"), zone(2, "Attic")];
    let rows = rows(&render(&document).expect("the table window assembles"));
    assert_eq!(rows.len(), 2);
    assert_eq!((rows[0]["0"].as_str(), rows[0]["1"].as_str()), (Some("1"), Some("Ground floor")));
    assert_eq!((rows[1]["0"].as_str(), rows[1]["1"].as_str()), (Some("2"), Some("Attic")));
}
