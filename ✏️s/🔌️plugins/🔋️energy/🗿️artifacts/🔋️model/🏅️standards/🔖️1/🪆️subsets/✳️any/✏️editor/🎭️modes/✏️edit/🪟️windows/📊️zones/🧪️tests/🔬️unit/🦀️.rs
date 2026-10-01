use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window_keeping_the_kit_action() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.actions.iter().any(|action| action.id == "set-cell"), "the kit's own edit action must survive");
    assert_eq!(def.actions.len(), 1 + actions().len());
}

#[semio_framework_async_macros::async_test]
async fn every_authored_action_is_localized_in_english_and_german() {
    for action in actions() {
        assert!(
            semio_framework::Terminology::ALL.iter().all(|&terminology| action.label.resolve(terminology, semio_framework::Locale::En) != action.label.resolve(terminology, semio_framework::Locale::De)),
            "action {} is not really translated",
            action.id
        );
        for arg in &action.args {
            assert!(
                semio_framework::Terminology::ALL.iter().all(|&terminology| arg.label.resolve(terminology, semio_framework::Locale::En) != arg.label.resolve(terminology, semio_framework::Locale::De)),
                "arg {} of action {} is not really translated",
                arg.id,
                action.id
            );
        }
    }
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
