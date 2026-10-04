use super::*;
use semio_framework_dsl_record::{DslField,NativeSchemaControl,RecordSpecProducer,Shape};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl};
use semio_framework_value::{native_decoding::NativeDecodeProgress, native_encoding::NativeEncodeProgress};
use serde_json::{json, Value};

fn fixture() -> Value { serde_json::from_str(include_str!("../../🧫️fixtures/🎛️metadata/🔣️.json")).unwrap() }
fn producers() -> [RecordSpecProducer; 11] {
    [dwg_xrecord_value_spec_producer(), dwg_table_control_entry_spec_producer(), table_control_body_spec_producer(), dwg_complex_color_value_spec_producer(), table_record_body_spec_producer(), dwg_evaluation_variant_spec_producer(), dwg_evaluation_expression_value_spec_producer(), dwg_visual_style_property_spec_producer::<u32>(), dwg_constraint_node_spec_producer(), dwg_entity_body_spec_producer(), dwg_logical_object_body_spec_producer()]
}
fn summary(spec: &semio_framework_dsl_record::RecordSpec) -> Value {
    json!({ "keyword": spec.keyword, "layout": "inline", "fields": spec.fields.iter().map(|field| {
        assert_eq!(spec.layout, semio_framework_dsl_record::RecordLayout::Inline);
        assert_eq!(field.position, None);
        assert!(!field.flatten && !field.is_call_name && field.defines.is_none());
        let labels = match &field.shape { Shape::Enum(labels) => labels.iter().map(|(label, ordinal)| json!({ "label": label, "ordinal": ordinal })).collect::<Vec<_>>(), _ => Vec::new() };
        json!({ "id": field.id, "key": field.key, "optional": field.optional, "labels": labels })
    }).collect::<Vec<_>>() })
}
fn shape_identity(shape: &Shape) -> Value {
    match shape {
        Shape::Tuple(child, count) => json!(["Tuple", count, shape_identity(child)]),
        Shape::List(child) => json!(["List", shape_identity(child)]),
        Shape::Block(child) => json!(["Block", shape_identity(child)]),
        Shape::Map(child) => json!(["Map", shape_identity(child)]),
        Shape::Record(producer) => json!(["Record", producer.ordinary as usize]),
        Shape::Table(producer) => json!(["Table", producer.ordinary as usize]),
        Shape::Statements(rows) => json!(["Statements", rows.iter().map(|(label, producer)| json!([label, producer.ordinary as usize])).collect::<Vec<_>>()]),
        _ => json!(format!("{shape:?}")),
    }
}
fn assert_same_shapes(left: &semio_framework_dsl_record::RecordSpec, right: &semio_framework_dsl_record::RecordSpec) {
    assert_eq!(left.fields.len(), right.fields.len());
    for (left, right) in left.fields.iter().zip(&right.fields) { assert_eq!(shape_identity(&left.shape), shape_identity(&right.shape), "{}", left.key); }
}

#[test]
fn dwg_controlled_metadata_all_eleven_match_authored_json_and_ordinary_in_both_directions() {
    let corpus = fixture();
    assert_eq!(corpus["cases"].as_array().unwrap().len(), 11);
    assert_eq!(producers().len(), 11);
    for (case, producer) in corpus["cases"].as_array().unwrap().iter().zip(producers()) {
        let expected = json!({ "keyword": case["keyword"], "layout": case["layout"], "fields": case["fields"] });
        let ordinary = (producer.ordinary)();
        assert_eq!(summary(&ordinary), expected, "{}", case["factory"]);
        let mut accept_decode = |_: NativeDecodeProgress| true;
        let mut decode = NativeDecodeControl::new(1_000_000, &mut accept_decode);
        let decoded = producer.decode(&mut decode).unwrap();
        let mut accept_encode = |_: NativeEncodeProgress| true;
        let mut encode = NativeEncodeControl::new(1_000_000, &mut accept_encode);
        let encoded = producer.encode(&mut encode).unwrap();
        assert_eq!(summary(&decoded), expected);
        assert_eq!(summary(&encoded), expected);
        assert_same_shapes(&ordinary, &decoded);
        assert_same_shapes(&ordinary, &encoded);
        assert_eq!(decode.owned_bytes(), encode.owned_bytes());
        assert!(decode.owned_bytes() > 0);
    }
}

#[test]
fn dwg_controlled_metadata_generic_scalar_and_lazy_nested_record_have_distinct_actual_shapes() {
    let corpus = fixture();
    for ((producer, owner), witness) in [(dwg_visual_style_property_spec_producer::<u32>(), "u32"), (dwg_visual_style_property_spec_producer::<DwgTableControlEntry>(), "DwgTableControlEntry")].into_iter().zip(corpus["genericWitnesses"].as_array().unwrap()) {
        assert_eq!(witness["type"], owner);
        let expected = witness["valueShape"].as_str().unwrap();
        let ordinary = (producer.ordinary)();
        let mut accepted = |_: NativeDecodeProgress| true;
        let mut control = NativeDecodeControl::new(1_000_000, &mut accepted);
        let actual = producer.decode(&mut control).unwrap();
        assert_same_shapes(&ordinary, &actual);
        assert_eq!(match actual.fields[0].shape { Shape::UInt => "UInt", Shape::Record(_) => "Record", _ => panic!("generic child") }, expected);
        let fields = corpus["cases"][7]["fields"].as_array().unwrap();
        let metadata_bytes = fields.len() * std::mem::size_of::<semio_framework_dsl_record::FieldSpec>() + fields.iter().map(|field| field["key"].as_str().unwrap().len() + field["labels"].as_array().unwrap().iter().map(|label| std::mem::size_of::<(String, u32)>() + label["label"].as_str().unwrap().len()).sum::<usize>()).sum::<usize>();
        assert_eq!(control.owned_bytes(), metadata_bytes);
        if let Shape::Record(child) = &actual.fields[0].shape {
            let before = control.owned_bytes();
            let nested = child.decode(&mut control).unwrap();
            assert_eq!(summary(&nested), summary(&dwg_table_control_entry_spec()));
            assert!(control.owned_bytes() > before);
        }
        let mut accepted = |_: NativeEncodeProgress| true;
        let mut control = NativeEncodeControl::new(1_000_000, &mut accepted);
        let encoded = producer.encode(&mut control).unwrap();
        assert_same_shapes(&ordinary, &encoded);
        assert_eq!(control.owned_bytes(), metadata_bytes);
        if let Shape::Record(child) = &encoded.fields[0].shape {
            let before = control.owned_bytes();
            let nested = child.encode(&mut control).unwrap();
            assert_eq!(summary(&nested), summary(&dwg_table_control_entry_spec()));
            assert!(control.owned_bytes() > before);
        }
    }
}

#[test]
fn dwg_controlled_metadata_shapes_refuse_cancellation_before_any_allocation() {
    macro_rules! check { ($owner:ty) => {{
        let mut reject = |_: NativeDecodeProgress| false;
        let mut control = NativeDecodeControl::new(1_000_000, &mut reject);
        assert_eq!(<$owner as DslField>::shape_controlled(&mut control).unwrap_err().kind, ValueRefusalKind::Canceled);
        assert_eq!(control.owned_bytes(), 0);
        let mut reject = |_: NativeEncodeProgress| false;
        let mut control = NativeEncodeControl::new(1_000_000, &mut reject);
        assert_eq!(<$owner as DslField>::shape_controlled(&mut control).unwrap_err().kind, ValueRefusalKind::Canceled);
        assert_eq!(control.owned_bytes(), 0);
    }}; }
    check!(DwgXRecordValue); check!(DwgTableControlEntry); check!(DwgTableControlBody); check!(DwgComplexColorValue); check!(DwgTableRecordBody); check!(DwgEvaluationVariant); check!(DwgEvaluationExpressionValue); check!(DwgVisualStyleProperty<u32>); check!(DwgConstraintNode); check!(DwgEntityBody); check!(DwgLogicalObjectBody);
    for producer in producers() {
        let mut accepted = |_: NativeDecodeProgress| true;
        let mut control = NativeDecodeControl::new(0, &mut accepted);
        assert_eq!(producer.decode(&mut control).unwrap_err().kind, ValueRefusalKind::OwnershipLimit);
        assert_eq!(control.owned_bytes(), 0);
        let mut accepted = |_: NativeEncodeProgress| true;
        let mut control = NativeEncodeControl::new(0, &mut accepted);
        assert_eq!(producer.encode(&mut control).unwrap_err().kind, ValueRefusalKind::OwnershipLimit);
        assert_eq!(control.owned_bytes(), 0);
    }
}

#[test]
fn dwg_controlled_metadata_field_and_enum_loops_cancel_interior_in_both_directions() {
    for (producer, total) in [(dwg_logical_object_body_spec_producer(), 44), (dwg_xrecord_value_spec_producer(), 11)] {
        let mut observed = false;
        let mut cancel = |event: NativeDecodeProgress| { if event.total == total && event.completed > 0 && event.completed < total { observed = true; false } else { true } };
        let mut control = NativeDecodeControl::new(1_000_000, &mut cancel);
        assert_eq!(producer.decode(&mut control).unwrap_err().kind, ValueRefusalKind::Canceled);
        let owned = control.owned_bytes(); drop(control);
        assert!(observed && owned > 0);
        let mut observed = false;
        let mut cancel = |event: NativeEncodeProgress| { if event.total == total && event.completed > 0 && event.completed < total { observed = true; false } else { true } };
        let mut control = NativeEncodeControl::new(1_000_000, &mut cancel);
        assert_eq!(producer.encode(&mut control).unwrap_err().kind, ValueRefusalKind::Canceled);
        let owned = control.owned_bytes(); drop(control);
        assert!(observed && owned > 0);
    }
}

#[test]
fn dwg_controlled_metadata_exact_vec_key_enum_label_and_box_ceiling_frontiers() {
    let field_bytes = 9 * std::mem::size_of::<semio_framework_dsl_record::FieldSpec>();
    let enum_bytes = 11 * std::mem::size_of::<(String, u32)>();
    let corpus = fixture(); let fields = corpus["cases"][0]["fields"].as_array().unwrap();
    let labels_bytes: usize = fields[0]["labels"].as_array().unwrap().iter().map(|label| label["label"].as_str().unwrap().len()).sum();
    let keys_before_box: usize = fields.iter().take(6).map(|field| field["key"].as_str().unwrap().len()).sum();
    let box_frontier = field_bytes + enum_bytes + labels_bytes + keys_before_box;
    let cases = [(field_bytes - 1, 0), (field_bytes + enum_bytes - 1, field_bytes), (field_bytes + enum_bytes, field_bytes + enum_bytes), (box_frontier, box_frontier)];
    for (maximum, admitted) in cases {
        let mut accepted = |_: NativeDecodeProgress| true; let mut decode = NativeDecodeControl::new(maximum, &mut accepted);
        assert!(dwg_xrecord_value_spec_producer().decode(&mut decode).is_err()); assert_eq!(decode.owned_bytes(), admitted);
        let mut accepted = |_: NativeEncodeProgress| true; let mut encode = NativeEncodeControl::new(maximum, &mut accepted);
        assert!(dwg_xrecord_value_spec_producer().encode(&mut encode).is_err()); assert_eq!(encode.owned_bytes(), admitted);
    }
    let fields = 2 * std::mem::size_of::<semio_framework_dsl_record::FieldSpec>();
    for (maximum, admitted) in [(fields, fields), (fields + "has_handle".len() - 1, fields), (fields + "has_handle".len(), fields + "has_handle".len())] {
        let mut accepted = |_: NativeDecodeProgress| true; let mut decode = NativeDecodeControl::new(maximum, &mut accepted);
        assert!(dwg_table_control_entry_spec_producer().decode(&mut decode).is_err()); assert_eq!(decode.owned_bytes(), admitted);
        let mut accepted = |_: NativeEncodeProgress| true; let mut encode = NativeEncodeControl::new(maximum, &mut accepted);
        assert!(dwg_table_control_entry_spec_producer().encode(&mut encode).is_err()); assert_eq!(encode.owned_bytes(), admitted);
    }
}

fn nested_decode(control: &mut NativeDecodeControl<'_>, depth: usize) -> Result<semio_framework_dsl_record::RecordSpec, ValueError> {
    if depth == 0 { dwg_table_control_entry_spec_producer().decode(control) } else { control.scoped_depth(64, |control| nested_decode(control, depth - 1)) }
}
fn nested_encode(control: &mut NativeEncodeControl<'_>, depth: usize) -> Result<semio_framework_dsl_record::RecordSpec, ValueError> {
    if depth == 0 { dwg_table_control_entry_spec_producer().encode(control) } else { control.scoped_depth(64, |control| nested_encode(control, depth - 1)) }
}
#[test]
fn dwg_controlled_metadata_producer_retains_actual_depth_refusal() {
    let mut accepted = |_: NativeDecodeProgress| true; let mut decode = NativeDecodeControl::new(1_000_000, &mut accepted);
    assert_eq!(nested_decode(&mut decode, 64).unwrap_err().kind, ValueRefusalKind::DepthLimit); assert_eq!(decode.owned_bytes(), 0);
    let mut accepted = |_: NativeEncodeProgress| true; let mut encode = NativeEncodeControl::new(1_000_000, &mut accepted);
    assert_eq!(nested_encode(&mut encode, 64).unwrap_err().kind, ValueRefusalKind::DepthLimit); assert_eq!(encode.owned_bytes(), 0);
}
