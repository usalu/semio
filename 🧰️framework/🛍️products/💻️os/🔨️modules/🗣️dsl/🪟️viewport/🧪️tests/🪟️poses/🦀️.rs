use super::*;
use crate::{DslValue, FromValue};
use crate::os_dsl::schema::{parse_exact, print_record, JoinMode, ParseOptions, Writer};

fn check<T: DslField + FromValue + PartialEq + std::fmt::Debug>(value: &serde_json::Value) {
    let pose = <T as FromValue>::from_value(DslValue::from(value.clone())).unwrap();
    let Shape::Record(spec) = T::shape() else { panic!("viewport record shape"); };
    let spec = spec();
    let FieldValue::Record(record) = DslField::to_value(&pose) else { panic!("viewport record value"); };
    for mode in [JoinMode::Inline, JoinMode::Document] {
        let mut writer = Writer::new();
        print_record(&record, &spec, &mut writer);
        let text = writer.render(mode);
        let decoded = parse_exact(&text, &spec, &ParseOptions::default()).unwrap();
        assert_eq!(<T as DslField>::from_value(&FieldValue::Record(decoded)).unwrap(), pose);
    }
    let packed = crate::os_store::pack_rt::encode_document(&spec, &record, &crate::os_store::PackEncodeOptions::default()).unwrap();
    let (decoded, _) = crate::os_store::pack_rt::decode_document(&packed, &spec, &crate::os_store::PackDecodeOptions::default()).unwrap();
    assert_eq!(<T as DslField>::from_value(&FieldValue::Record(decoded)).unwrap(), pose);
}

#[test]
fn viewport_ownership_dsl_and_pack_preserve_shared_native_records() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🖱️ui/🪟️viewport/🧪️tests/🧫️fixtures/🪟️poses/🔣️.json")).unwrap();
    let mut count = 0;
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["valid"] == true) {
        if row["dimension"] == "2d" { check::<Viewport2d>(&row["value"]); }
        else { check::<Viewport3dOrbit>(&row["value"]); }
        count += 1;
    }
    assert_eq!(count, 4);
    eprintln!("[DEBUG] Four shared viewport records round-tripped through inline text, document text and Pack without an app wrapper");
}

#[test]
fn viewport_ownership_dsl_rejects_invalid_pose_and_foreign_fields() {
    let pose = Viewport3dOrbit { position: [8.0, -3.0, 5.0], target: [0.0; 3], zoom: 1.25, up: None };
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        let FieldValue::Record(mut record) = DslField::to_value(&pose) else { unreachable!() };
        record.fields.insert(3, FieldValue::Float(invalid));
        assert!(<Viewport3dOrbit as DslField>::from_value(&FieldValue::Record(record)).is_err());
    }
    let FieldValue::Record(mut record) = DslField::to_value(&pose) else { unreachable!() };
    record.fields.insert(42, FieldValue::Text("foreign locale".into()));
    assert!(<Viewport3dOrbit as DslField>::from_value(&FieldValue::Record(record)).is_err());
    let FieldValue::Record(mut record) = DslField::to_value(&pose) else { unreachable!() };
    record.fields.insert(1, FieldValue::Tuple(vec![FieldValue::Float(1.0), FieldValue::Float(2.0)]));
    assert!(<Viewport3dOrbit as DslField>::from_value(&FieldValue::Record(record)).is_err());
    eprintln!("[DEBUG] Shared viewport DSL binding rejected invalid zoom, vector arity and foreign record ownership");
}
