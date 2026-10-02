//! 🧼️ Authored field names equal to the derive's own locals (`field`, `record`, `control`, `value`, `fields`, `keyword`)
//! derive, print, parse and round-trip under ordinary and controlled ownership.

use super::*;

#[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
struct HygienicRecord {
    field: String,
    record: u32,
    control: bool,
    value: Vec<String>,
    fields: Option<String>,
    keyword: String,
}

#[derive(Clone, Debug, PartialEq, DslEnum)]
enum HygienicVariant {
    Probe {
        field: String,
        record: u32,
        control: bool,
        value: Vec<String>,
        fields: Option<String>,
        keyword: String,
    },
}

fn hygienic_cases() -> (Vec<String>, Vec<HygienicRecord>) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️hygienic-bindings/🔣️.json")).expect("neutral hygiene vectors");
    let names = fixture["names"].as_array().unwrap().iter().map(|name| name.as_str().unwrap().to_string()).collect();
    let cases = fixture["cases"].as_array().unwrap().iter().map(|case| serde_json::from_value(case.clone()).expect("serde hygiene case")).collect();
    (names, cases)
}

fn variant_of(source: &HygienicRecord) -> HygienicVariant {
    HygienicVariant::Probe { field: source.field.clone(), record: source.record, control: source.control, value: source.value.clone(), fields: source.fields.clone(), keyword: source.keyword.clone() }
}

#[test]
fn hygienic_record_names_round_trip_ordinary_and_controlled() {
    let (names, cases) = hygienic_cases();
    let spec = HygienicRecord::__dsl_spec();
    assert_eq!(spec.fields.iter().map(|field| field.key.to_string()).collect::<Vec<_>>(), names);
    for source in cases {
        let ordinary = source.__dsl_to_record();
        let mut accept = |_| true;
        assert_eq!(source.__dsl_to_record_controlled(&mut NativeEncodeControl::new(65536, &mut accept)).expect("controlled projection"), ordinary);
        let text = print(&ordinary, &spec, JoinMode::Inline);
        let parsed = parse(&text, &spec, &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline }).expect("hygienic record text parse");
        assert_eq!(HygienicRecord::__dsl_from_record(&parsed).expect("ordinary construction"), source);
        let mut admit = |_| true;
        assert_eq!(HygienicRecord::__dsl_from_record_controlled(&parsed, &mut NativeDecodeControl::new(1_000_000, &mut admit)).expect("controlled construction"), source);
        assert_eq!(serde_json::to_value(&source).expect("serde record"), serde_json::to_value(HygienicRecord::__dsl_from_record(&parsed).unwrap()).unwrap());
    }
}

#[test]
fn hygienic_variant_names_round_trip_ordinary_and_controlled() {
    let (names, cases) = hygienic_cases();
    let producer = HygienicVariant::variants()[0].1;
    let spec = (producer.ordinary)();
    assert_eq!(spec.fields.iter().map(|field| field.key.to_string()).collect::<Vec<_>>(), names);
    for source in cases {
        let variant = variant_of(&source);
        let (keyword, ordinary) = variant.to_named_record();
        assert_eq!(keyword, "probe");
        assert_eq!(ordinary, source.__dsl_to_record());
        let mut accept = |_| true;
        assert_eq!(variant.to_named_record_controlled(&mut NativeEncodeControl::new(65536, &mut accept)).expect("controlled variant projection"), (keyword.clone(), ordinary.clone()));
        let text = print(&ordinary, &spec, JoinMode::Inline);
        let parsed = parse(&text, &spec, &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline }).expect("hygienic variant text parse");
        assert_eq!(HygienicVariant::from_named_record(&keyword, &parsed).expect("ordinary variant construction"), variant);
        let mut admit = |_| true;
        assert_eq!(HygienicVariant::from_named_record_controlled(&keyword, &parsed, &mut NativeDecodeControl::new(1_000_000, &mut admit)).expect("controlled variant construction"), variant);
        variant.retire_decoded_variant();
    }
}
