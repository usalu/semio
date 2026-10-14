//! 🧫️ The defining command-value producer preserves borrowed input on cancellation and depth refusal.
use super::*;
fn ui_list(values: impl IntoIterator<Item = UiValue>) -> UiList {
    let mut builder = UiListBuilder::try_new().expect("fixed fixture list");
    for value in values { builder.push(value).expect("fixed fixture page"); }
    builder.finish()
}
fn fixture_text(value: &str) -> UiText { UiText::try_from_str(value).expect("bounded fixture text") }

#[test]
    fn retained_ui_value_bridge_cancel_and_deadline_preserve_the_original_owner() {
        let original = UiValue::List(ui_list([UiValue::Text(fixture_text("kept"))]));
        let mut producer = UiCommandJsonProducer::try_new(&original).expect("credited alias");
        assert_eq!(producer.drive_one(false, true), UiCommandJsonStep::MoreWork);
        assert_eq!(producer.items, 0);
        assert_eq!(producer.drive_one(true, false), UiCommandJsonStep::Cancelled);
        let UiValue::List(values) = original else { panic!("original list") };
        assert_eq!(values.len(), 1);
    }

    #[test]
    fn retained_ui_value_bridge_rejects_depth_plus_one_without_consuming_the_original() {
        let mut original = UiValue::Null;
        for _ in 0..=UI_COMMAND_VALUE_DEPTH {
            original = UiValue::List(ui_list([original]));
        }
        let mut producer = UiCommandJsonProducer::try_new(&original).expect("credited root alias");
        let mut opportunities = 0;
        loop {
            opportunities += 1;
            match producer.drive_one(false, false) {
                UiCommandJsonStep::MoreWork => continue,
                UiCommandJsonStep::Fault => break,
                step => panic!("unexpected bridge step {step:?}"),
            }
        }
        assert!(opportunities > UI_COMMAND_VALUE_DEPTH);
        let UiValue::List(values) = original else { panic!("original list") };
        assert_eq!(values.len(), 1);
    }
