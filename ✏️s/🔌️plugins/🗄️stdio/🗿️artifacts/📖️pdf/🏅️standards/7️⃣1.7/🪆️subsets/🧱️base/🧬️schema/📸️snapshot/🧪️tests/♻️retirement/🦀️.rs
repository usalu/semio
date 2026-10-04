use super::{PdfAction, PdfColorSpace, PdfDate, PdfDictEntry, PdfFormField, PdfFormFieldKind, PdfFunction, PdfObject, PdfOutlineItem};
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_framework_value::native_decoding::{NativeDecodeControl, NativeDecodeProgress};
use semio_framework_value::native_encoding::{NativeEncodeControl, NativeEncodeProgress};

fn exponential() -> PdfFunction {
    PdfFunction::Exponential { domain: vec![0.0, 1.0], range: None, c0: vec![0.0], c1: vec![1.0], n: 1.0 }
}

fn roundtrip<T: FromValue + ToValue>(wire: &serde_json::Value) {
    let input = DslValue::from(wire);
    let ordinary = T::from_value(input.clone()).unwrap();
    assert_eq!(serde_json::Value::from(&ordinary.to_value()), *wire);
    let mut accept = |_| true;
    let mut decoding = NativeDecodeControl::new(1_000_000, &mut accept);
    let controlled = T::from_value_controlled(&input, &mut decoding).unwrap();
    let mut accept = |_| true;
    let mut encoding = NativeEncodeControl::new(1_000_000, &mut accept);
    let output = controlled.to_value_controlled(&mut encoding).unwrap();
    assert_eq!(serde_json::Value::from(&output), *wire);
    assert!(encoding.owned_bytes() > 0);
    T::retire_decoded(ordinary);
    T::retire_decoded(controlled);
    DslValue::retire_decoded(output);
}

#[test]
fn pdf_recursive_retirement_owned_wire_corpus_roundtrips_with_serde_json() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/♻️retirement/🔣️.json")).unwrap();
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 10);
    for row in fixture["cases"].as_array().unwrap() {
        match row["owner"].as_str().unwrap() {
            "color" => roundtrip::<PdfColorSpace>(&row["wire"]),
            "function" => roundtrip::<PdfFunction>(&row["wire"]),
            "action" => roundtrip::<PdfAction>(&row["wire"]),
            "outline" => roundtrip::<PdfOutlineItem>(&row["wire"]),
            "form" => roundtrip::<PdfFormField>(&row["wire"]),
            "date" => roundtrip::<PdfDate>(&row["wire"]),
            owner => panic!("unexpected owner {owner}"),
        }
    }
}

fn refuse<T: FromValue + ToValue>(source: T) {
    let input = source.to_value();
    let mut cancel = |_| false;
    let mut decoding = NativeDecodeControl::new(1_000_000, &mut cancel);
    assert!(T::from_value_controlled(&input, &mut decoding).is_err());
    assert_eq!(decoding.owned_bytes(), 0);
    let mut accept = |_| true;
    let mut zero = NativeDecodeControl::new(0, &mut accept);
    assert!(T::from_value_controlled(&input, &mut zero).is_err());
    assert_eq!(zero.owned_bytes(), 0);
    let mut cancel = |_| false;
    let mut encoding = NativeEncodeControl::new(1_000_000, &mut cancel);
    assert!(source.to_value_controlled(&mut encoding).is_err());
    assert_eq!(encoding.owned_bytes(), 0);
    let mut accept = |_| true;
    let mut zero = NativeEncodeControl::new(0, &mut accept);
    assert!(source.to_value_controlled(&mut zero).is_err());
    assert_eq!(zero.owned_bytes(), 0);
    T::retire_decoded(source);
    DslValue::retire_decoded(input);
}

#[test]
fn pdf_recursive_retirement_cancellation_and_zero_budget_remain_closed() {
    refuse(PdfColorSpace::Pattern { base: Some(Box::new(PdfColorSpace::DeviceRgb)) });
    refuse(PdfFunction::Array { functions: vec![exponential()] });
}

#[test]
fn pdf_recursive_retirement_depth_refusal_cleans_partial_owned_children() {
    let mut function = exponential();
    for _ in 0..100 { function = PdfFunction::Array { functions: vec![function] }; }
    let input = function.to_value();
    let mut accept = |_| true;
    let mut decoding = NativeDecodeControl::new(1_000_000, &mut accept);
    let error = PdfFunction::from_value_controlled(&input, &mut decoding).err().unwrap();
    assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::DepthLimit);
    let mut accept = |_| true;
    let mut encoding = NativeEncodeControl::new(1_000_000, &mut accept);
    assert_eq!(function.to_value_controlled(&mut encoding).unwrap_err().kind,semio_framework_value::ValueRefusalKind::DepthLimit);
    PdfFunction::retire_decoded(function);
    DslValue::retire_decoded(input);
}

#[test]
fn pdf_recursive_retirement_deep_function_and_color_forests_use_bounded_stack() {
    std::thread::Builder::new().stack_size(128 * 1024).spawn(|| {
        let mut function = exponential();
        let mut color = PdfColorSpace::DeviceRgb;
        for index in 0..24_000 {
            function = if index % 2 == 0 { PdfFunction::Array { functions: vec![function] } } else {
                PdfFunction::Stitching { domain: vec![0.0, 1.0], range: None, functions: vec![function], bounds: vec![], encode: vec![] }
            };
            color = match index % 5 {
                0 => PdfColorSpace::Pattern { base: Some(Box::new(color)) },
                1 => PdfColorSpace::Indexed { base: Box::new(color), hival: 0, lookup: vec![0] },
                2 => PdfColorSpace::IccBased { components: 3, profile: vec![0], alternate: Some(Box::new(color)), range: None },
                3 => PdfColorSpace::Separation { name: "spot".into(), alternate: Box::new(color), tint_transform: exponential() },
                _ => PdfColorSpace::DeviceN { names: vec!["spot".into()], alternate: Box::new(color), tint_transform: exponential(), attributes: Some(vec![PdfDictEntry { key: "witness".into(), value: PdfObject::Array(vec![PdfObject::Bool(true)]) }]) },
            };
        }
        PdfFunction::retire_decoded(function);
        PdfColorSpace::retire_decoded(color);
    }).unwrap().join().unwrap();
}

#[test]
fn pdf_recursive_retirement_interior_cancel_and_later_child_error_clean_admitted_siblings() {
    let source = PdfFunction::Array { functions: (0..1024).map(|_| exponential()).collect() };
    let input = source.to_value();
    let mut observed = false;
    let mut cancel = |progress: NativeDecodeProgress| {
        if progress.total == 1024 && progress.completed == 256 { observed = true; false } else { true }
    };
    let mut decoding = NativeDecodeControl::new(10_000_000, &mut cancel);
    assert!(PdfFunction::from_value_controlled(&input, &mut decoding).is_err());
    assert!(decoding.owned_bytes() > 0);
    drop(decoding);
    assert!(observed);
    let mut observed = false;
    let mut cancel = |progress: NativeEncodeProgress| {
        if progress.total == 1024 && progress.completed == 256 { observed = true; false } else { true }
    };
    let mut encoding = NativeEncodeControl::new(10_000_000, &mut cancel);
    assert!(source.to_value_controlled(&mut encoding).is_err());
    assert!(encoding.owned_bytes() > 0);
    drop(encoding);
    assert!(observed);
    PdfFunction::retire_decoded(source);
    DslValue::retire_decoded(input);
    let malformed = DslValue::String("malformed-function".into());
    let mut accept = |_| true;
    let expected = PdfFunction::from_value_controlled(&malformed, &mut NativeDecodeControl::new(1_000_000, &mut accept)).err().unwrap();
    let mut first = exponential();
    for _ in 0..12 { first = PdfFunction::Array { functions: vec![first] }; }
    let later_error = DslValue::Object(vec![("kind".into(), DslValue::String("array".into())), ("functions".into(), DslValue::Array(vec![first.to_value(), malformed]))]);
    let mut accept = |_| true;
    let mut decoding = NativeDecodeControl::new(1_000_000, &mut accept);
    let actual = PdfFunction::from_value_controlled(&later_error, &mut decoding).err().unwrap();
    assert_eq!(actual.kind,expected.kind);assert_eq!(actual.message, format!("functions.1.{}", expected.message));
    assert!(decoding.owned_bytes() > 0);
    PdfFunction::retire_decoded(first);
    DslValue::retire_decoded(later_error);
}

#[test]
fn pdf_recursive_retirement_actions_outlines_and_forms_use_owned_bounded_stack() {
    std::thread::Builder::new().stack_size(128 * 1024).spawn(|| {
        let mut action = PdfAction::uri("https://example.invalid/");
        let mut outline = PdfOutlineItem::to_page("Leaf", 0);
        let mut field = PdfFormField { name: "Leaf".into(), kind: PdfFormFieldKind::Container, flags: 0, alternate_name: None, mapping_name: None, default_appearance: None, quadding: None, widgets: vec![], children: vec![], additional_actions: vec![], extra: vec![] };
        for _ in 0..24_000 {
            let mut parent = PdfAction::uri("https://example.invalid/");
            parent.next.push(action);
            action = parent;
            let mut parent = PdfOutlineItem::to_page("Root", 0);
            parent.children.push(outline);
            parent.action = Some(PdfAction::uri("https://example.invalid/"));
            outline = parent;
            field = PdfFormField { children: vec![field], name: "Root".into(), kind: PdfFormFieldKind::Container, flags: 0, alternate_name: None, mapping_name: None, default_appearance: None, quadding: None, widgets: vec![], additional_actions: vec![], extra: vec![] };
        }
        PdfAction::retire_decoded(action);
        PdfOutlineItem::retire_decoded(outline);
        PdfFormField::retire_decoded(field);
    }).unwrap().join().unwrap();
    refuse(PdfAction::uri("https://example.invalid/"));
    refuse(PdfOutlineItem::to_page("Root", 0));
    let input = DslValue::Object(vec![("year".into(), DslValue::int(2026))]);
    let mut accept = |_| true;
    let date = PdfDate::from_value_controlled(&input, &mut NativeDecodeControl::new(1_000_000, &mut accept)).unwrap();
    assert_eq!((date.year, date.month, date.day, date.hour, date.minute, date.second, date.offset_minutes), (2026, 1, 1, 0, 0, 0, None));
}
