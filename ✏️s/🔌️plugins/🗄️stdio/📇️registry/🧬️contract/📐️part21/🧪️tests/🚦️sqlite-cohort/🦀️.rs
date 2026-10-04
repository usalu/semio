use super::*;
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, ValueRefusalKind};

fn cohort() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🚦️sqlite-cohort/🔣️.json")).unwrap()
}

fn decimal(value: &serde_json::Value) -> Part21Decimal {
    Part21Decimal {
        negative: value["negative"].as_bool().unwrap(),
        coefficient: value["coefficient"].as_str().unwrap().into(),
        scale: u32::try_from(value["scale"].as_u64().unwrap()).unwrap(),
        exponent: value.get("exponent").map(|v| i32::try_from(v.as_i64().unwrap()).unwrap()),
    }
}

fn values() -> Vec<Part21Value> {
    let f = cohort();
    let mut values = vec![
        Part21Value::Ref(0),
        Part21Value::Ref(u64::MAX),
        Part21Value::Str(f["literalText"].as_str().unwrap().into()),
        Part21Value::Enum("UNKNOWN".into()),
        Part21Value::Int(i64::MIN),
        Part21Value::Int(i64::MAX),
        Part21Value::List(vec![]),
        Part21Value::Typed { name: "".into(), items: vec![Part21Value::Int(i64::MAX)] },
        Part21Value::Unset,
        Part21Value::Derived,
    ];
    values.extend(f["decimals"].as_array().unwrap().iter().chain(f["decimalOwnedExtras"].as_array().unwrap()).map(|v| Part21Value::Real(decimal(v))));
    values
}

fn independent_value(value: &Part21Value) -> serde_json::Value {
    match value {
        Part21Value::Ref(id) => serde_json::json!({"kind":"ref","value":id}),
        Part21Value::Str(text) => serde_json::json!({"kind":"str","value":text}),
        Part21Value::Enum(text) => serde_json::json!({"kind":"enum","value":text}),
        Part21Value::Int(integer) => serde_json::json!({"kind":"int","value":integer}),
        Part21Value::Real(decimal) => {
            let mut value = serde_json::json!({"negative":decimal.negative,"coefficient":decimal.coefficient,"scale":decimal.scale});
            if let Some(exponent) = decimal.exponent {
                value["exponent"] = exponent.into();
            }
            serde_json::json!({"kind":"real","value":value})
        }
        Part21Value::List(items) => serde_json::json!({"kind":"list","values":items.iter().map(independent_value).collect::<Vec<_>>()}),
        Part21Value::Typed { name, items } => serde_json::json!({"kind":"typed","typeName":name,"values":items.iter().map(independent_value).collect::<Vec<_>>()}),
        Part21Value::Unset => serde_json::json!({"kind":"unset"}),
        Part21Value::Derived => serde_json::json!({"kind":"derived"}),
    }
}

#[test]
fn part21_cohort_controlled_constructor_preserves_every_owned_variant() {
    for value in values() {
        let wire = value.to_value();
        let restored = Part21Value::from_value_controlled(&wire, &mut NativeDecodeControl::new(1048576, &mut |_| true)).unwrap();
        assert_eq!(restored, value);
        let independent: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&restored)).unwrap();
        assert_eq!(independent, independent_value(&value));
    }
    let instance = Part21Instance { id: u64::MAX, entities: vec![("".into(), values()), ("REPEATED".into(), vec![]), ("REPEATED".into(), vec![])] };
    assert_eq!(Part21Instance::from_value_controlled(&instance.to_value(), &mut NativeDecodeControl::new(1048576, &mut |_| true)).unwrap(), instance);
    let independent: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&instance)).unwrap();
    assert_eq!(
        independent,
        serde_json::json!({"id":instance.id,"entities":instance.entities.iter().map(|(name,arguments)|serde_json::json!({"typeName":name,"arguments":arguments.iter().map(independent_value).collect::<Vec<_>>()})).collect::<Vec<_>>()})
    );
}

#[test]
fn part21_cohort_controlled_output_preserves_exact_decimal_fields_without_display() {
    for value in values() {
        let actual = value.to_value_controlled(&mut NativeEncodeControl::new(1048576, &mut |_| true)).unwrap();
        assert_eq!(actual, value.to_value());
        let independent: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&actual)).unwrap();
        assert_eq!(independent, independent_value(&value));
        assert_eq!(Part21Value::from_value(actual).unwrap(), value);
    }
    let instance = Part21Instance { id: u64::MAX, entities: vec![("".into(), values())] };
    let actual = instance.to_value_controlled(&mut NativeEncodeControl::new(1048576, &mut |_| true)).unwrap();
    assert_eq!(actual, instance.to_value());
    let independent: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&actual)).unwrap();
    assert_eq!(
        independent,
        serde_json::json!({"id": instance.id, "entities": instance.entities.iter().map(|(name,arguments)|serde_json::json!({"typeName":name,"arguments":arguments.iter().map(independent_value).collect::<Vec<_>>()})).collect::<Vec<_>>()})
    );
}

#[test]
fn part21_cohort_controlled_input_cancels_during_owned_unicode_copy() {
    let f = cohort();
    let text = f["longText"]["unit"].as_str().unwrap().repeat(usize::try_from(f["longText"]["repeat"].as_u64().unwrap()).unwrap());
    let value = Part21Value::Typed { name: text, items: vec![Part21Value::Unset] }.to_value();
    let mut reached = false;
    let error = Part21Value::from_value_controlled(
        &value,
        &mut NativeDecodeControl::new(1048576, &mut |p| {
            if p.total >= 65536 && p.completed >= 65536 && p.completed < p.total {
                reached = true;
                false
            } else {
                true
            }
        }),
    )
    .unwrap_err();
    assert_eq!(error.kind, ValueRefusalKind::Canceled);
    assert!(reached);
    let error = Part21Value::from_value_controlled(&value, &mut NativeDecodeControl::new(1, &mut |_| true)).unwrap_err();
    assert_eq!(error.kind, ValueRefusalKind::OwnershipLimit);
}

#[test]
fn part21_cohort_controlled_output_cancels_during_owned_unicode_copy() {
    let f = cohort();
    let text = f["longText"]["unit"].as_str().unwrap().repeat(usize::try_from(f["longText"]["repeat"].as_u64().unwrap()).unwrap());
    let value = Part21Value::Str(text);
    let mut reached = false;
    let error = value
        .to_value_controlled(&mut NativeEncodeControl::new(1048576, &mut |p| {
            if p.total >= 65536 && p.completed >= 65536 && p.completed < p.total {
                reached = true;
                false
            } else {
                true
            }
        }))
        .unwrap_err();
    assert_eq!(error.kind, ValueRefusalKind::Canceled);
    assert!(reached);
    assert_eq!(value.to_value_controlled(&mut NativeEncodeControl::new(1, &mut |_| true)).unwrap_err().kind, ValueRefusalKind::OwnershipLimit);
}

#[test]
fn part21_cohort_controlled_objects_refuse_unknown_duplicate_and_wrong_typed_fields() {
    for row in cohort()["controlled"]["malformedValues"].as_array().unwrap() {
        let value = semio_framework_pack_json::from_json_str::<DslValue>(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(Part21Value::from_value_controlled(&value, &mut NativeDecodeControl::new(1_048_576, &mut |_| true)).unwrap_err().kind, ValueRefusalKind::InvalidValue);
    }
    let duplicate = DslValue::object([("kind".into(), "unset".to_value()), ("kind".into(), "derived".to_value())]);
    assert_eq!(Part21Value::from_value_controlled(&duplicate, &mut NativeDecodeControl::new(1_048_576, &mut |_| true)).unwrap_err().kind, ValueRefusalKind::InvalidValue);
}

#[test]
fn part21_cohort_controlled_instances_preserve_literal_field_order_and_exact_cumulative_bound() {
    let instance = Part21Instance { id: u64::MAX, entities: vec![("".into(), values()), ("REPEATED".into(), vec![]), ("REPEATED".into(), vec![])] };
    let mut yes = |_| true;
    let mut encode = NativeEncodeControl::new(1_048_576, &mut yes);
    let value = instance.to_value_controlled(&mut encode).unwrap();
    assert_eq!(value.as_object().unwrap().iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), ["id", "entities"]);
    assert_eq!(value["entities"].as_array().unwrap().iter().map(|row| row["typeName"].as_str().unwrap()).collect::<Vec<_>>(), ["", "REPEATED", "REPEATED"]);
    let encode_bytes = encode.owned_bytes();
    assert_eq!(instance.to_value_controlled(&mut NativeEncodeControl::new(encode_bytes, &mut |_| true)).unwrap(), value);
    assert_eq!(instance.to_value_controlled(&mut NativeEncodeControl::new(encode_bytes - 1, &mut |_| true)).unwrap_err().kind, ValueRefusalKind::OwnershipLimit);
    let mut decode_yes = |_| true;
    let mut decode = NativeDecodeControl::new(1_048_576, &mut decode_yes);
    assert_eq!(Part21Instance::from_value_controlled(&value, &mut decode).unwrap(), instance);
    let decode_bytes = decode.owned_bytes();
    assert_eq!(Part21Instance::from_value_controlled(&value, &mut NativeDecodeControl::new(decode_bytes, &mut |_| true)).unwrap(), instance);
    assert_eq!(Part21Instance::from_value_controlled(&value, &mut NativeDecodeControl::new(decode_bytes - 1, &mut |_| true)).unwrap_err().kind, ValueRefusalKind::OwnershipLimit);
}

#[test]
fn part21_cohort_controlled_list_admission_reaches_real_interior_units() {
    let count = usize::try_from(cohort()["controlled"]["wideItems"].as_u64().unwrap()).unwrap();
    let value = Part21Value::List((0..count).map(|_| Part21Value::Unset).collect());
    let wire = value.to_value();
    let mut decoded_interior = false;
    let decoded = Part21Value::from_value_controlled(
        &wire,
        &mut NativeDecodeControl::new(1_048_576, &mut |p| {
            if p.total == count && p.completed == 256 {
                decoded_interior = true;
                false
            } else {
                true
            }
        }),
    )
    .unwrap_err();
    assert!(decoded_interior);
    assert_eq!(decoded.kind, ValueRefusalKind::Canceled);
    let mut encoded_interior = false;
    let encoded = value
        .to_value_controlled(&mut NativeEncodeControl::new(1_048_576, &mut |p| {
            if p.total == count && p.completed == 256 {
                encoded_interior = true;
                false
            } else {
                true
            }
        }))
        .unwrap_err();
    assert!(encoded_interior);
    assert_eq!(encoded.kind, ValueRefusalKind::Canceled);
}

#[test]
fn part21_cohort_controlled_recursive_owner_refuses_depth_before_unbounded_construction() {
    let depth = usize::try_from(cohort()["controlled"]["recursiveDepth"].as_u64().unwrap()).unwrap();
    let mut value = Part21Value::Unset;
    let mut wire = DslValue::object([("kind".into(), "unset".to_value())]);
    for _ in 0..depth {
        value = Part21Value::Typed { name: String::new(), items: vec![value] };
        wire = DslValue::object([("kind".into(), "typed".to_value()), ("typeName".into(), String::new().to_value()), ("values".into(), DslValue::Array(vec![wire]))]);
    }
    assert_eq!(Part21Value::from_value_controlled(&wire, &mut NativeDecodeControl::new(1_048_576, &mut |_| true)).unwrap_err().kind, ValueRefusalKind::DepthLimit);
    assert_eq!(value.to_value_controlled(&mut NativeEncodeControl::new(1_048_576, &mut |_| true)).unwrap_err().kind, ValueRefusalKind::DepthLimit);
    <DslValue as FromValue>::retire_decoded(wire);
    <Part21Value as FromValue>::retire_decoded(value);
}

fn cohort_header() -> Part21Header {
    let f = cohort();
    Part21Header::from_value(semio_framework_pack_json::from_json_str::<DslValue>(&f["headerCase"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()).unwrap()
}

fn independent_header(header: &Part21Header) -> serde_json::Value {
    serde_json::json!({"fileDescription": header.file_description.iter().map(independent_value).collect::<Vec<_>>(), "fileName": header.file_name.iter().map(independent_value).collect::<Vec<_>>(), "fileSchema": header.file_schema.iter().map(independent_value).collect::<Vec<_>>()})
}

#[test]
fn part21_cohort_controlled_header_document_fields_match_independent_exact_output() {
    let header = cohort_header();
    let actual = header.to_value_controlled(&mut NativeEncodeControl::new(1_048_576, &mut |_| true)).unwrap();
    assert_eq!(actual.as_object().unwrap().iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), ["fileDescription", "fileName", "fileSchema"]);
    let independent: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&actual)).unwrap();
    assert_eq!(independent, independent_header(&header));
    assert_eq!(Part21Header::from_value_controlled(&actual, &mut NativeDecodeControl::new(1_048_576, &mut |_| true)).unwrap(), header);
    let document = Part21Document { header, instances: vec![Part21Instance { id: u64::MAX, entities: vec![("".into(), values()), ("R".into(), vec![]), ("R".into(), vec![])] }] };
    let actual = document.to_value_controlled(&mut NativeEncodeControl::new(1_048_576, &mut |_| true)).unwrap();
    assert_eq!(actual.as_object().unwrap().iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), ["header", "instances"]);
    let independent: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&actual)).unwrap();
    assert_eq!(
        independent,
        serde_json::json!({"header": independent_header(&document.header), "instances": [{"id": u64::MAX, "entities": document.instances[0].entities.iter().map(|(name,arguments)|serde_json::json!({"typeName":name,"arguments":arguments.iter().map(independent_value).collect::<Vec<_>>()})).collect::<Vec<_>>()}]})
    );
    assert_eq!(Part21Document::from_value_controlled(&actual, &mut NativeDecodeControl::new(1_048_576, &mut |_| true)).unwrap(), document);
}

#[test]
fn part21_cohort_controlled_instance_decimal_closed_fields_refuse_wrong_domains() {
    let f = cohort();
    for row in f["controlled"]["malformedInstances"].as_array().unwrap() {
        let value = semio_framework_pack_json::from_json_str::<DslValue>(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(Part21Instance::from_value_controlled(&value, &mut NativeDecodeControl::new(1_048_576, &mut |_| true)).unwrap_err().kind, ValueRefusalKind::InvalidValue);
    }
    for row in f["controlled"]["malformedDecimals"].as_array().unwrap() {
        let value = semio_framework_pack_json::from_json_str::<DslValue>(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(Part21Decimal::from_value_controlled(&value, &mut NativeDecodeControl::new(1_048_576, &mut |_| true)).unwrap_err().kind, ValueRefusalKind::InvalidValue);
    }
}

#[test]
fn part21_cohort_controlled_owner_restores_parent_work_limit_category() {
    let f = cohort();
    let total = usize::try_from(f["controlled"]["workload"]["parentTotal"].as_u64().unwrap()).unwrap();
    let before = usize::try_from(f["controlled"]["workload"]["parentBefore"].as_u64().unwrap()).unwrap();
    let value = Part21Value::Str(f["literalText"].as_str().unwrap().into());
    let mut yes = |_| true;
    let mut input = NativeDecodeControl::new(1_048_576, &mut yes);
    input.begin_stage(total).unwrap();
    input.advance(before).unwrap();
    assert_eq!(Part21Value::from_value_controlled(&value.to_value(), &mut input).unwrap(), value);
    input.advance(total - before).unwrap();
    assert_eq!(input.step().unwrap_err().kind, ValueRefusalKind::WorkLimit);
    let mut encode_yes = |_| true;
    let mut output = NativeEncodeControl::new(1_048_576, &mut encode_yes);
    output.begin_stage(total).unwrap();
    output.advance(before).unwrap();
    assert_eq!(value.to_value_controlled(&mut output).unwrap(), value.to_value());
    output.advance(total - before).unwrap();
    assert_eq!(output.step().unwrap_err().kind, ValueRefusalKind::WorkLimit);
}

#[test]
fn part21_cohort_controlled_long_borrowed_key_refuses_during_actual_key_work() {
    let f = cohort();
    let key = f["longText"]["unit"].as_str().unwrap().repeat(usize::try_from(f["longText"]["repeat"].as_u64().unwrap()).unwrap());
    let input = DslValue::object([("kind".into(), "unset".to_value()), (key.clone(), DslValue::Null)]);
    let mut reached = false;
    let error = Part21Value::from_value_controlled(
        &input,
        &mut NativeDecodeControl::new(65_536, &mut |p| {
            if p.total == key.len() && p.completed >= 65_536 && p.completed < p.total {
                reached = true;
                false
            } else {
                true
            }
        }),
    )
    .unwrap_err();
    assert_eq!(error.kind, ValueRefusalKind::Canceled);
    assert!(reached);
}

#[test]
fn part21_cohort_controlled_partial_named_owner_retains_paid_copy_and_invalid_refusal() {
    let f = cohort();
    let name = f["longText"]["unit"].as_str().unwrap().repeat(usize::try_from(f["longText"]["repeat"].as_u64().unwrap()).unwrap());
    let input = DslValue::object([("kind".into(), "typed".to_value()), ("typeName".into(), name.to_value()), ("values".into(), DslValue::Array(vec![DslValue::object([("kind".into(), "unknown".to_value())])]))]);
    let mut copied = false;
    let mut callback = |p: semio_framework_value::native_decoding::NativeDecodeProgress| {
        if p.total == name.len() && p.completed == p.total {
            copied = true;
        }
        true
    };
    let mut control = NativeDecodeControl::new(1_048_576, &mut callback);
    let error = Part21Value::from_value_controlled(&input, &mut control).unwrap_err();
    assert_eq!(error.kind, ValueRefusalKind::InvalidValue);
    assert!(control.owned_bytes() >= name.len());
    assert!(copied);
}
