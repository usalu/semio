//! 🧪️ Replays the closed multilingual corpus through real native codecs and the Serde oracle.
use crate::{Locale, LocalizedLabel, Terminology};
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError, NativeDecodeControl, NativeEncodeControl};
use semio_framework_value::native_decoding::NativeDecodeProgress;
use semio_framework_value::native_encoding::NativeEncodeProgress;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("closed locale corpus") }
fn value(input: &serde_json::Value) -> DslValue {
    match input["kind"].as_str().expect("value kind") {
        "null" => DslValue::Null,
        "boolean" => DslValue::Bool(input["value"].as_bool().unwrap()),
        "number" => DslValue::float(input["value"].as_f64().unwrap()),
        "string" => DslValue::String(input["value"].as_str().unwrap().into()),
        "array" => DslValue::Array(input["items"].as_array().unwrap().iter().map(value).collect()),
        "object" => DslValue::Object(input["entries"].as_array().unwrap().iter().map(|entry| (entry[0].as_str().unwrap().into(), value(&entry[1]))).collect()),
        _ => panic!("undeclared corpus value kind"),
    }
}

#[test]
fn complete_portable_label_corpus_matches_native_admission_and_independent_serde() {
    let corpus = fixture();
    let rows = corpus["labels"].as_array().unwrap();
    assert_eq!(rows.len(), 32);
    for row in rows {
        let id = row["id"].as_str().unwrap();
        let expected = row["accepted"].as_bool().unwrap();
        let input = value(&row["value"]);
        let ordinary = LocalizedLabel::from_value(input.clone());
        let mut accepted = |_| true;
        let mut control = NativeDecodeControl::new(1_000_000, &mut accepted);
        let controlled = LocalizedLabel::from_value_controlled(&input, &mut control);
        let independent = serde_json::from_str::<LocalizedLabel>(row["rawJson"].as_str().unwrap());
        assert_eq!(ordinary.is_ok(), expected, "{id} ordinary admission");
        assert_eq!(controlled.is_ok(), expected, "{id} controlled admission");
        assert_eq!(independent.is_ok(), expected, "{id} independent Serde admission");
        if expected {
            let label = controlled.unwrap();
            assert_eq!(label, ordinary.unwrap());
            assert_eq!(label, independent.unwrap());
            for terminology in Terminology::ALL {
                for locale in Locale::ALL {
                    let key = format!("{}.{}", terminology.as_str(), locale.as_str());
                    assert_eq!(label.resolve(terminology, locale), row["cells"][key].as_str().unwrap(), "{id} cell");
                }
            }
            let mut accepted = |_| true;
            let mut control = NativeEncodeControl::new(1_000_000, &mut accepted);
            let output = label.to_value_controlled(&mut control).unwrap();
            assert_eq!(serde_json::Value::from(&output), serde_json::to_value(&label).unwrap(), "{id} controlled output");
            assert_eq!(output, label.to_value(), "{id} ordinary output");
            <DslValue as FromValue>::retire_decoded(output);
        }
        <DslValue as FromValue>::retire_decoded(input);
    }
    println!("[DEBUG] native locale corpus executed all32 duplicate-aware label vectors");
}

#[test]
fn explicit_locale_authority_replays_all_portable_id_and_language_tag_vectors() {
    let corpus = fixture();
    let rows = corpus["locales"].as_array().unwrap();
    assert_eq!(rows.len(), 17);
    for row in rows {
        let actual = row["input"].as_str().and_then(|input| if row["mode"] == "id" { Locale::parse(input) } else { Locale::from_language_tag(input).ok() });
        assert_eq!(actual.is_some(), row["accepted"].as_bool().unwrap(), "{} admission", row["id"]);
        assert_eq!(actual.map(Locale::as_str), row["locale"].as_str(), "{} identity", row["id"]);
    }
    println!("[DEBUG] native locale authority executed all17 vectors without default language");
}

fn decode_at_depth(control: &mut NativeDecodeControl<'_>, remaining: usize, input: &DslValue) -> Result<LocalizedLabel, ValueError> {
    if remaining == 0 { LocalizedLabel::from_value_controlled(input, control) } else { control.scoped_depth(64, |control| decode_at_depth(control, remaining - 1, input)) }
}
fn encode_at_depth(control: &mut NativeEncodeControl<'_>, remaining: usize, input: &LocalizedLabel) -> Result<DslValue, ValueError> {
    if remaining == 0 { input.to_value_controlled(control) } else { control.scoped_depth(64, |control| encode_at_depth(control, remaining - 1, input)) }
}

#[test]
fn native_label_construction_refuses_early_interior_cancel_zero_bytes_and_depth() {
    let label = LocalizedLabel::native("one", "zwei");
    let input = label.to_value();
    let mut early = |_: NativeDecodeProgress| false;
    let mut control = NativeDecodeControl::new(1_000_000, &mut early);
    assert!(LocalizedLabel::from_value_controlled(&input, &mut control).is_err());
    assert_eq!(control.owned_bytes(), 0);
    let mut early = |_: NativeEncodeProgress| false;
    let mut control = NativeEncodeControl::new(1_000_000, &mut early);
    assert!(label.to_value_controlled(&mut control).is_err());
    assert_eq!(control.owned_bytes(), 0);
    let mut accepted = |_| true;
    let mut control = NativeDecodeControl::new(0, &mut accepted);
    assert!(LocalizedLabel::from_value_controlled(&input, &mut control).is_err());
    assert_eq!(control.owned_bytes(), 0);
    let mut accepted = |_| true;
    let mut control = NativeEncodeControl::new(0, &mut accepted);
    assert!(label.to_value_controlled(&mut control).is_err());
    assert_eq!(control.owned_bytes(), 0);
    let mut admitted_first_cell = false;
    let mut interior = false;
    let mut cancel = |event: NativeDecodeProgress| {
        if event.total == 3 && event.completed == 3 { admitted_first_cell = true; }
        if admitted_first_cell && event.total == 4 && event.completed == 0 { interior = true; false } else { true }
    };
    let mut control = NativeDecodeControl::new(1_000_000, &mut cancel);
    assert!(LocalizedLabel::from_value_controlled(&input, &mut control).is_err());
    drop(control);
    assert!(interior);
    let mut admitted_first_cell = false;
    let mut interior = false;
    let mut cancel = |event: NativeEncodeProgress| {
        if event.total == 3 && event.completed == 3 { admitted_first_cell = true; }
        if admitted_first_cell && event.total == 4 && event.completed == 0 { interior = true; false } else { true }
    };
    let mut control = NativeEncodeControl::new(1_000_000, &mut cancel);
    assert!(label.to_value_controlled(&mut control).is_err());
    drop(control);
    assert!(interior);
    let mut accepted = |_| true;
    let mut control = NativeDecodeControl::new(1_000_000, &mut accepted);
    assert!(decode_at_depth(&mut control, 64, &input).is_err());
    assert_eq!(control.owned_bytes(), 0);
    let mut accepted = |_| true;
    let mut control = NativeEncodeControl::new(1_000_000, &mut accepted);
    assert!(encode_at_depth(&mut control, 64, &label).is_err());
    assert_eq!(control.owned_bytes(), 0);
    <DslValue as FromValue>::retire_decoded(input);
}
