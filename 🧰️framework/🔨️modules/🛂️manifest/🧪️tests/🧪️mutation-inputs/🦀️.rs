//! 🧪️ `mutation_input_defs` over the language-agnostic corpus `🧫️fixtures/🧫️mutation-inputs/🔣️.json`: every case's
//! descriptors equal its `expectedInputs` in canonical JSON (the byte string the TypeScript twin `mutationInputDefs`
//! is held to by `🟦️.ts` beside this file), and every refused case names the corpus' error class and input pointer.
//! The third-party half (npm `jsonschema`, Python `jsonschema`) judges the same corpus in `🟦️.ts` and `🐍️.py`.

use super::*;

const CORPUS: &str = include_str!("../../🧫️fixtures/🧫️mutation-inputs/🔣️.json");

fn corpus() -> DslValue {
    semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(CORPUS, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the corpus is JSON"))
}

/// 🧾️ Canonical JSON: object keys sorted, arrays in order, an integral number within the safe range written as an integer.
fn canonical(value: &DslValue) -> String {
    match value {
        DslValue::Null => "null".to_string(),
        DslValue::Bool(flag) => flag.to_string(),
        DslValue::Number(number) => {
            let value = number.as_f64();
            if value.is_finite() && value.fract() == 0.0 && value.abs() <= 9_007_199_254_740_991.0 { (value as i64).to_string() } else { value.to_string() }
        }
        DslValue::String(text) => semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&DslValue::String(text.clone()))),
        DslValue::Bytes(bytes) => format!("[{}]", bytes.iter().map(u8::to_string).collect::<Vec<_>>().join(",")),
        DslValue::Array(items) => format!("[{}]", items.iter().map(canonical).collect::<Vec<_>>().join(",")),
        DslValue::Object(entries) => {
            let mut sorted: Vec<&(String, DslValue)> = entries.iter().collect();
            sorted.sort_by(|left, right| left.0.cmp(&right.0));
            format!("{{{}}}", sorted.iter().map(|(key, value)| format!("{}:{}", canonical(&DslValue::String(key.clone())), canonical(value))).collect::<Vec<_>>().join(","))
        }
    }
}

fn resolver(corpus: &DslValue) -> impl Fn(&str) -> Option<DslValue> + '_ {
    move |id: &str| corpus.get("documents").and_then(|documents| documents.get(id)).cloned()
}

#[test]
fn every_corpus_case_reads_to_its_canonical_descriptors_or_its_refusal() {
    let corpus = corpus();
    let cases = corpus.get("cases").and_then(DslValue::as_array).expect("cases");
    assert!(cases.len() >= 18, "the corpus covers every reader rule");
    for case in cases {
        let name = case.get("name").and_then(DslValue::as_str).expect("name");
        let schema = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(case.get("input").and_then(|input| input.get("leafSchema")).expect("leafSchema")));
        let outcome = mutation_input_defs(&schema, &resolver(&corpus));
        match (case.get("expectedInputs"), case.get("expectedError"), outcome) {
            (Some(expected), None, Ok(inputs)) => assert_eq!(canonical(&DslValue::Array(inputs.iter().map(ToValue::to_value).collect())), canonical(expected), "{name}"),
            (None, Some(expected), Err(error)) => assert_eq!(
                (canonical(&error.code.to_value()), error.pointer.as_str()),
                (canonical(expected.get("code").expect("code")), expected.get("pointer").and_then(DslValue::as_str).expect("pointer")),
                "{name}: {error}"
            ),
            (_, _, outcome) => panic!("{name}: unexpected outcome {outcome:?}"),
        }
    }
}

#[test]
fn every_corpus_case_audits_to_every_finding_it_names() {
    let corpus = corpus();
    for case in corpus.get("cases").and_then(DslValue::as_array).expect("cases") {
        let name = case.get("name").and_then(DslValue::as_str).expect("name");
        let schema = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(case.get("input").and_then(|input| input.get("leafSchema")).expect("leafSchema")));
        let audit = mutation_input_audit(&schema, &resolver(&corpus));
        let expected: Vec<DslValue> = match (case.get("expectedFindings"), case.get("expectedError")) {
            (Some(DslValue::Array(findings)), _) => findings.clone(),
            (None, Some(error)) => vec![error.clone()],
            _ => Vec::new(),
        };
        let pair = |finding: &DslValue| (canonical(finding.get("code").expect("code")), finding.get("pointer").and_then(DslValue::as_str).expect("pointer").to_string());
        assert_eq!(audit.findings.iter().map(|finding| (canonical(&finding.code.to_value()), finding.pointer.clone())).collect::<Vec<_>>(), expected.iter().map(pair).collect::<Vec<_>>(), "{name}");
        if let Some(inputs) = case.get("expectedInputs") {
            assert_eq!(canonical(&DslValue::Array(audit.inputs.iter().map(ToValue::to_value).collect())), canonical(inputs), "{name}");
        }
    }
}

#[test]
fn descriptors_round_trip_through_their_wire_value() {
    let corpus = corpus();
    for case in corpus.get("cases").and_then(DslValue::as_array).expect("cases").iter().filter(|case| case.get("expectedInputs").is_some()) {
        let schema = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(case.get("input").and_then(|input| input.get("leafSchema")).expect("leafSchema")));
        for input in mutation_input_defs(&schema, &resolver(&corpus)).expect("declared") {
            assert_eq!(ActionArgDef::from_value(input.to_value()).expect("decodes"), input);
        }
    }
}

#[test]
fn derived_controls_follow_the_annotation_and_the_inference_rules() {
    let corpus = corpus();
    let inputs = |name: &str| {
        let case = corpus.get("cases").and_then(DslValue::as_array).expect("cases").iter().find(|case| case.get("name").and_then(DslValue::as_str) == Some(name)).expect("case");
        mutation_input_defs(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(case.get("input").and_then(|input| input.get("leafSchema")).expect("leafSchema"))), &resolver(&corpus)).expect("declared")
    };
    let drag = inputs("annotated-drag-selection");
    assert!(matches!(drag[0].control(), ActionArgControl::Reference { many: true, ref domain, .. } if domain.as_deref() == Some("vortex")));
    assert!(matches!(drag[1].control(), ActionArgControl::Stepper { snap_source: Some(SnapSource::Config { ref key }), .. } if key == "gridFactor"));
    assert!(matches!(inputs("dial-in-degrees")[0].control(), ActionArgControl::Dial { min, max, .. } if min < 0.0 && max > 3.0));
    assert!(matches!(inputs("log-slider-with-soft-range")[0].control(), ActionArgControl::Slider { min, max, scale: Some(NumberScale::Log), .. } if min == 0.1 && max == 10.0));
    let inferred = inputs("inferred-from-glossary");
    assert!(matches!(inferred[1].control(), ActionArgControl::Stepper { step: Some(step), .. } if step == 1.0));
    assert!(matches!(inferred[3].control(), ActionArgControl::Slider { min, max, .. } if min == 0.0 && max == 1.0));
    assert_eq!(inferred[4].control(), ActionArgControl::Toggle);
    assert!(matches!(inputs("local-defs-override-options-nullable")[1].control(), ActionArgControl::Segmented { ref options } if options.len() == 2));
    assert!(matches!(inputs("nested-object-array-and-vector")[2].control(), ActionArgControl::Vector { dims: 3, ref unit, step: Some(step), .. } if unit.as_deref() == Some("m") && step == 0.5));
    assert!(matches!(inputs("integer-references")[0].control(), ActionArgControl::Reference { id_type: ReferenceIdType::Integer, ref granularity, .. } if granularity.as_deref() == Some("zone")));
    assert!(matches!(inferred.iter().find(|input| input.key() == "layerId").expect("layerId").control(), ActionArgControl::Reference { id_type: ReferenceIdType::String, .. }));
    let colors = inputs("color-rgb-and-rgba");
    assert_eq!((colors[0].control(), colors[1].control()), (ActionArgControl::Color { alpha: true }, ActionArgControl::Color { alpha: false }));
    assert!(matches!(inputs("vector-with-grid-facets")[0].control(), ActionArgControl::Vector { dims: 3, min: Some(min), max: Some(max), snap_source: Some(SnapSource::Config { ref key }), precision: Some(2), display_factor: Some(factor), .. } if min == -100.0 && max == 100.0 && key == "gridFactor" && factor == 100.0));
    let sourced = inputs("option-source-from-the-previewed-document");
    assert!(matches!(&sourced[1].schema, ArgSchema::String { options, option_source: Some(OptionSource::Snapshot { pointer }), .. } if options.is_empty() && pointer == "/hostSnapshot/widgets/{id}/params"));
    assert_eq!(sourced[1].control(), ActionArgControl::Select { options: Vec::new() }, "a sourced choice is a select before its options resolve");
    let text = inputs("multiline-text");
    assert_eq!((text[0].control(), text[1].control()), (ActionArgControl::Multiline, ActionArgControl::Text { placeholder: None }), "a plain string is one line unless it is declared multiline");
    let keyed = inputs("keyed-list-of-typed-values");
    let ArgSchema::Object { fields } = &keyed[0].schema else { panic!("a keyed list is an object holding its entries") };
    let ArgSchema::Array { items, .. } = &fields[0].schema else { panic!("the entries are a list") };
    let ArgSchema::Object { fields: record } = &**items else { panic!("each entry is a record") };
    assert_eq!((keyed[0].nullable, record.iter().map(ActionArgDef::key).collect::<Vec<_>>(), record[0].control(), record[1].control()), (true, vec!["questionId".to_string(), "value".to_string()], ActionArgControl::Text { placeholder: None }, ActionArgControl::Text { placeholder: None }), "a keyed list reads as declared records of text inputs, with no widget of its own");
}

#[test]
fn a_selected_id_converts_to_the_payload_value_the_reference_id_table_names_and_spells_back() {
    let corpus = corpus();
    for row in corpus.get("referenceIds").and_then(DslValue::as_array).expect("referenceIds") {
        let id_type = ReferenceIdType::from_value(row.get("idType").expect("idType").clone()).expect("idType");
        let text = row.get("text").and_then(DslValue::as_str).expect("text");
        let value = id_type.id_value(text);
        assert_eq!(canonical(value.as_ref().unwrap_or(&DslValue::Null)), canonical(row.get("value").expect("value")), "{id_type:?} {text:?}");
        assert_eq!(value.as_ref().and_then(reference_id_text).as_deref(), row.get("spelled").and_then(DslValue::as_str), "{id_type:?} {text:?}");
    }
}

#[test]
fn the_glossary_labels_every_frequent_input_in_every_locale() {
    let glossary = input_label_glossary();
    assert!(glossary.len() >= 200, "{}", glossary.len());
    for (name, label) in glossary {
        for terminology in Terminology::ALL {
            for locale in Locale::ALL {
                assert!(!label.resolve(terminology, locale).is_empty(), "{name} {terminology} {locale}");
            }
        }
    }
    assert_eq!(glossary.get("newName").map(|label| label.resolve(Terminology::Native, Locale::De)), Some("Neuer Name"));
    let mut english_by_german: BTreeMap<&str, &str> = BTreeMap::new();
    for (name, label) in glossary {
        let (german, english) = (label.resolve(Terminology::Native, Locale::De), label.resolve(Terminology::Native, Locale::En));
        let first = *english_by_german.entry(german).or_insert(english);
        assert_eq!(first, english, "{name}: two glossary names share the German label {german:?} but not the English one, so one form would show two identical labels");
    }
}

#[test]
fn a_mutation_input_key_decodes_its_pointer_segment() {
    let input = ActionArgDef::text("/new~1name~0x", LocalizedLabel::native("Name", "Name"));
    assert_eq!(input.key(), "new/name~x");
    assert_eq!(ActionArgDef::text("plain", LocalizedLabel::native("Plain", "Schlicht")).key(), "plain");
}

#[test]
fn a_candidate_payload_regains_the_discriminator_its_leaf_schema_requires() {
    let corpus = corpus();
    let case = corpus.get("cases").and_then(DslValue::as_array).expect("cases").iter().find(|case| case.get("name").and_then(DslValue::as_str) == Some("annotated-drag-selection")).expect("case");
    let schema = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(case.get("input").and_then(|input| input.get("leafSchema")).expect("leafSchema")));
    let payload = DslValue::object([("targets".to_string(), DslValue::Array(vec![DslValue::String("n1".to_string())])), ("dx".to_string(), DslValue::float(2.0)), ("dy".to_string(), DslValue::float(-1.0))]);
    let instance = mutation_input_instance(&schema, &resolver(&corpus), &payload).expect("an object payload");
    assert_eq!(instance.get("mutation").and_then(DslValue::as_str), Some("dragSelection"));
    let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(&schema).expect("the leaf compiles");
    assert!(validator.is_valid_json(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&instance))));
    assert!(!validator.is_valid_json(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&payload))));
    assert!(mutation_input_instance(&schema, &resolver(&corpus), &DslValue::Array(Vec::new())).is_err());
}

#[test]
fn a_union_payload_regains_the_constants_of_the_variant_it_names() {
    let corpus = corpus();
    let case = corpus.get("cases").and_then(DslValue::as_array).expect("cases").iter().find(|case| case.get("name").and_then(DslValue::as_str) == Some("discriminated-union-by-const")).expect("case");
    let schema = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(case.get("input").and_then(|input| input.get("leafSchema")).expect("leafSchema")));
    let restore = DslValue::object([("phase".to_string(), DslValue::String("restore".to_string())), ("index".to_string(), DslValue::uint(2))]);
    let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(&schema).expect("the leaf compiles");
    let instance = mutation_input_instance(&schema, &resolver(&corpus), &restore).expect("a restore payload");
    assert!(validator.is_valid_json(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&instance))));
    assert!(mutation_input_instance(&schema, &resolver(&corpus), &DslValue::object([("index".to_string(), DslValue::uint(2))])).is_err());
    assert!(mutation_input_instance(&schema, &resolver(&corpus), &DslValue::object([("phase".to_string(), DslValue::String("other".to_string()))])).is_err());
    let inputs = mutation_input_defs(&schema, &resolver(&corpus)).expect("a discriminated union");
    assert!(matches!(inputs[0].control(), ActionArgControl::Segmented { ref options } if options.len() == 2));
    assert_eq!(inputs.iter().skip(1).map(|input| input.group.as_deref()).collect::<Vec<_>>(), [Some("apply"), Some("restore")]);
}
