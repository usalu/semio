
use super::*;

#[test]
fn authored_slider_labels_survive_json_dag_and_chrome() {
    let fixture = crate::os_pack::json::parse(include_str!("../../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🏷️slider-labels.json")).unwrap();
    for row in fixture.get("cases").and_then(crate::os_pack::json::Value::as_array).unwrap() {
        let widget_value = row.get("widget").cloned().expect("fixture widget");
        let widget: Widget = crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(&widget_value)).unwrap();
        let label = row.get("expectedDagName").and_then(crate::os_pack::json::Value::as_str).unwrap();
        assert_eq!(widget_to_dag_node(&widget, 0, &OrderedMap::new(), &[], &HashMap::new(), (72.0, 14.0)).name, label);
        assert_eq!(widget_label(&widget), label);
        let widget_encoded = crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&widget));
        assert_eq!(widget_encoded.get("label").and_then(crate::os_pack::json::Value::as_str), Some(label));
        let chrome_encoded = crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&widget_chrome(&widget)));
        assert_eq!(chrome_encoded.get("label").and_then(crate::os_pack::json::Value::as_str), Some(label));
        let descriptor: WidgetDescriptor = crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(&widget_value)).unwrap();
        let widget_id = widget_value.get("id").and_then(crate::os_pack::json::Value::as_str).unwrap();
        assert_eq!(widget_from_descriptor(&descriptor, widget_id.into(), &HashMap::new()), widget);
        let missing = crate::os_pack::json::object(widget_value.as_object().unwrap().iter().filter(|(key, _)| *key != "label").map(|(key, value)| (key.to_string(), value.clone())));
        assert!(<Widget as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&missing)).is_err());
        assert!(<WidgetDescriptor as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&missing)).is_err());
    }
}
