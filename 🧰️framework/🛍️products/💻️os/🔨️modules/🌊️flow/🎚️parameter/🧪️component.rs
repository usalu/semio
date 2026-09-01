//! 🧪️ One-widget scalar range semantics shared by every Flow graph consumer.

use super::*;

//#region 🧪️ParameterSemantics
#[test]
fn graph_parameter_preserves_labels_and_existing_range_expansion() {
    let fixture = crate::os_pack::json::parse(include_str!("🧪️fixtures/🔣️graph-parameter.json")).unwrap();
    for case in fixture.get("cases").and_then(crate::os_pack::json::Value::as_array).unwrap() {
        let before = case.get("before").unwrap();
        let mut widget = Widget::InputSlider {
            id: case.get("widgetId").and_then(crate::os_pack::json::Value::as_str).unwrap().into(), label: case.get("label").and_then(crate::os_pack::json::Value::as_str).unwrap().into(),
            value: before.get("value").and_then(crate::os_pack::json::Value::as_f64).unwrap(), min: before.get("min").and_then(crate::os_pack::json::Value::as_f64).unwrap(), max: before.get("max").and_then(crate::os_pack::json::Value::as_f64).unwrap(), step: before.get("step").and_then(crate::os_pack::json::Value::as_f64).unwrap(),
        };
        assert!(set_widget_slider_value(&mut widget, case.get("request").and_then(crate::os_pack::json::Value::as_f64).unwrap()));
        let wire = crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&widget));
        assert_eq!(wire.get("id"), case.get("widgetId")); assert_eq!(wire.get("label"), case.get("label"));
        let after = case.get("after").unwrap();
        for field in ["value", "min", "max", "step"] { assert_eq!(wire.get(field).and_then(crate::os_pack::json::Value::as_f64), after.get(field).and_then(crate::os_pack::json::Value::as_f64)); }
        assert!(!set_widget_slider_value(&mut widget, f64::NAN));
        assert_eq!(crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&widget)), wire);
    }
}
//#endregion 🧪️ParameterSemantics
