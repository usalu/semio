//! 🧪️ One-widget scalar range semantics shared by every Flow graph consumer.

use super::*;

//#region 🧪️ParameterSemantics
#[test]
fn graph_parameter_preserves_labels_and_existing_range_expansion() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️graph-parameter.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let before = &case["before"];
        let mut widget = Widget::InputSlider {
            id: case["widgetId"].as_str().unwrap().into(), label: case["label"].as_str().unwrap().into(),
            value: before["value"].as_f64().unwrap(), min: before["min"].as_f64().unwrap(), max: before["max"].as_f64().unwrap(), step: before["step"].as_f64().unwrap(),
        };
        assert!(set_widget_slider_value(&mut widget, case["request"].as_f64().unwrap()));
        let wire = serde_json::to_value(&widget).unwrap();
        assert_eq!(wire["id"], case["widgetId"]); assert_eq!(wire["label"], case["label"]);
        for field in ["value", "min", "max", "step"] { assert_eq!(wire[field].as_f64(), case["after"][field].as_f64()); }
        assert!(!set_widget_slider_value(&mut widget, f64::NAN));
        assert_eq!(serde_json::to_value(&widget).unwrap(), wire);
    }
}
//#endregion 🧪️ParameterSemantics
