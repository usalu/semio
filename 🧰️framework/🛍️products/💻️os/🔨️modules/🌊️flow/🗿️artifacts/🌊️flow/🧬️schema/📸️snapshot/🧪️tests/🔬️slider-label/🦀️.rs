use super::*;

#[test]
fn authored_slider_labels_survive_json_dag_and_chrome() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🏷️slider-labels.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let third_party: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏷️slider-labels.json")).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&fixture)).unwrap(), third_party);
    for row in fixture.get("cases").and_then(semio_framework_pack_json::Value::as_array).unwrap() {
        let widget_value = row.get("widget").cloned().expect("fixture widget");
        let widget: Widget = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&widget_value)).unwrap();
        let label = row.get("expectedDagName").and_then(semio_framework_pack_json::Value::as_str).unwrap();
        assert_eq!(widget_to_dag_node(&widget, 0, &OrderedMap::new(), &[], &HashMap::new(), (72.0, 14.0)).name, label);
        assert_eq!(widget_label(&widget), label);
        let widget_encoded = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&widget));
        assert_eq!(widget_encoded.get("label").and_then(semio_framework_pack_json::Value::as_str), Some(label));
        let chrome_encoded = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&widget_chrome(&widget)));
        assert_eq!(chrome_encoded.get("label").and_then(semio_framework_pack_json::Value::as_str), Some(label));
        let descriptor: WidgetDescriptor = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&widget_value)).unwrap();
        let widget_id = widget_value.get("id").and_then(semio_framework_pack_json::Value::as_str).unwrap();
        assert_eq!(widget_from_descriptor(&descriptor, widget_id.into(), &HashMap::new()), widget);
        let missing = semio_framework_pack_json::object(widget_value.as_object().unwrap().iter().filter(|(key, _)| *key != "label").map(|(key, value)| (key.to_string(), value.clone())));
        assert!(<Widget as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&missing)).is_err());
        assert!(<WidgetDescriptor as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&missing)).is_err());
    }
}
