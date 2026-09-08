
use super::*;
use crate::os_pack::json::Value;

#[semio_framework_async_macros::async_test]
async fn primitive_shapes_map_onto_the_expected_json_schema_types() {
    assert_eq!(shape_json_schema(&Shape::Bool)["type"], Value::from("boolean"));
    assert_eq!(shape_json_schema(&Shape::Int)["type"], Value::from("integer"));
    assert_eq!(shape_json_schema(&Shape::UInt)["minimum"], Value::from(0u64));
    assert_eq!(shape_json_schema(&Shape::Float)["type"], Value::from("number"));
    assert_eq!(shape_json_schema(&Shape::Text)["type"], Value::from("string"));
    assert_eq!(shape_json_schema(&Shape::Count)["x-semio-shape"], Value::from("count"));
    assert_eq!(shape_json_schema(&Shape::Expr)["x-semio-shape"], Value::from("expr"));
}

#[semio_framework_async_macros::async_test]
async fn ref_carries_the_entity_kind_and_quantity_carries_its_unit() {
    let ref_schema = shape_json_schema(&Shape::Ref("material"));
    assert_eq!(ref_schema["type"], Value::from("string"));
    assert_eq!(ref_schema["x-semio-ref"], Value::from("material"));

    let quantity_schema = shape_json_schema(&Shape::Quantity(crate::os_dsl::unit_by_symbol("GPa").unwrap()));
    assert_eq!(quantity_schema["type"], Value::from("number"));
    assert_eq!(quantity_schema["x-semio-unit"], Value::from("GPa"));
}

#[semio_framework_async_macros::async_test]
async fn enum_becomes_a_string_enum_of_its_tags() {
    let schema = shape_json_schema(&Shape::Enum(vec![("visible".into(), 0), ("hidden".into(), 1)]));
    assert_eq!(schema["type"], Value::from("string"));
    assert_eq!(schema["enum"], Value::Array(vec![Value::from("visible"), Value::from("hidden")]));
}

#[semio_framework_async_macros::async_test]
async fn list_and_fixed_tuple_map_onto_json_schema_arrays() {
    let list = shape_json_schema(&Shape::List(Box::new(Shape::Float)));
    assert_eq!(list["type"], Value::from("array"));
    assert_eq!(list["items"]["type"], Value::from("number"));

    let tuple = shape_json_schema(&Shape::Tuple(Box::new(Shape::Float), Some(3)));
    assert_eq!(tuple["minItems"], Value::from(3u64));
    assert_eq!(tuple["maxItems"], Value::from(3u64));
}

fn camera_spec() -> RecordSpec {
    RecordSpec::new(Some("camera"), RecordLayout::Inline, vec![FieldSpec::new(0, "x", Shape::Float), FieldSpec::new(1, "y", Shape::Float), FieldSpec::new(2, "zoom", Shape::Float), FieldSpec::new(3, "label", Shape::Text).optional()])
}

#[semio_framework_async_macros::async_test]
async fn record_spec_json_schema_covers_required_and_optional_fields() {
    let schema = record_spec_json_schema(&camera_spec());
    assert_eq!(schema["type"], Value::from("object"));
    assert_eq!(schema["properties"]["x"]["type"], Value::from("number"));
    assert_eq!(schema["properties"]["label"]["type"], Value::from("string"));
    assert_eq!(schema["x-semio-keyword"], Value::from("camera"));
    let required: Vec<String> = schema["required"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
    assert!(required.contains(&"x".to_string()) && required.contains(&"zoom".to_string()));
    assert!(!required.contains(&"label".to_string()), "optional field must not be required");
}

fn writer_note_spec() -> RecordSpec {
    RecordSpec::new(Some("query"), RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text).positional(0), FieldSpec::new(1, "body", Shape::Embed("jack"))])
}

#[semio_framework_async_macros::async_test]
async fn record_spec_json_schema_round_trips_embed_and_positional_fields() {
    let schema = record_spec_json_schema(&writer_note_spec());
    assert_eq!(schema["properties"]["id"]["type"], Value::from("string"));
    assert_eq!(schema["properties"]["body"]["x-semio-shape"], Value::from("embed"));
    assert_eq!(schema["properties"]["body"]["x-semio-lang"], Value::from("jack"));
}

fn nested_point_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "x", Shape::Float), FieldSpec::new(1, "y", Shape::Float)])
}

#[semio_framework_async_macros::async_test]
async fn record_shape_recurses_via_record_spec_json_schema() {
    let spec = RecordSpec::new(Some("marker"), RecordLayout::Inline, vec![FieldSpec::new(0, "at", Shape::Record(nested_point_spec))]);
    let schema = record_spec_json_schema(&spec);
    assert_eq!(schema["properties"]["at"]["type"], Value::from("object"));
    assert_eq!(schema["properties"]["at"]["properties"]["x"]["type"], Value::from("number"));
}

#[semio_framework_async_macros::async_test]
async fn flatten_splices_nested_fields_into_the_same_properties_map() {
    let spec = RecordSpec::new(Some("shape"), RecordLayout::Inline, vec![FieldSpec::new(0, "origin", Shape::Record(nested_point_spec)).flatten(), FieldSpec::new(1, "label", Shape::Text)]);
    let schema = record_spec_json_schema(&spec);
    let properties = schema["properties"].as_object().unwrap();
    assert!(properties.contains_key("x") && properties.contains_key("y"), "flatten must splice into the parent's properties, not nest");
    assert!(properties.contains_key("label"));
    assert!(!properties.contains_key("origin"), "the flatten carrier field itself is not a property");
}
