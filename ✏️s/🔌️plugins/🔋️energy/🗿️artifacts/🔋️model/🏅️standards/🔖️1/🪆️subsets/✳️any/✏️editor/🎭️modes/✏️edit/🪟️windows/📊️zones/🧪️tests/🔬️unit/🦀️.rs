
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

#[semio_framework_async_macros::async_test]
async fn render_lists_one_row_per_zone() {
    let document = EnergyModelSnapshot::default();
    let table = render(&document).expect("the table window assembles");
    assert_eq!(table.key.as_str(), WINDOW_KIND_ID);
    assert!(table.children.is_empty());
}
