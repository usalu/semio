use super::*;
use semio_framework_pack_json::{JsonMemberPolicy, Value};

fn expected_origin(value: &Value) -> RustGeneratorOrigin {
    RustGeneratorOrigin {
        kind: match value["kind"].as_str().unwrap() { "direct-import" => RustGeneratorOriginKind::DirectImport, "qualified" => RustGeneratorOriginKind::Qualified, _ => panic!("closed generator origin") },
        import_start: value["importStart"].as_u64().map(|value| value.try_into().unwrap()),
        import_end: value["importEnd"].as_u64().map(|value| value.try_into().unwrap()),
    }
}

fn expected_output(value: &Value) -> RustGeneratedTokenOutput {
    RustGeneratedTokenOutput {
        macro_name: value["macro"].as_str().unwrap().into(),
        start: value["start"].as_u64().unwrap().try_into().unwrap(),
        end: value["end"].as_u64().unwrap().try_into().unwrap(),
        line: value["line"].as_u64().unwrap().try_into().unwrap(),
        origin: expected_origin(&value["origin"]),
        inputs: value["inputs"].as_array().unwrap().iter().map(|input| RustGeneratedTokenInput {
            kind: match input["kind"].as_str().unwrap() { "include" => RustGeneratedInputKind::Include, "include_str" => RustGeneratedInputKind::IncludeStr, "include_bytes" => RustGeneratedInputKind::IncludeBytes, _ => panic!("closed generated input") },
            expression: input["expression"].as_str().unwrap().into(),
            start: input["start"].as_u64().unwrap().try_into().unwrap(),
            end: input["end"].as_u64().unwrap().try_into().unwrap(),
            line: input["line"].as_u64().unwrap().try_into().unwrap(),
        }).collect(),
    }
}

#[test]
fn generated_inputs_retain_every_shared_macro_expression_and_utf16_position() {
    let fixture = semio_framework_pack_json::parse(include_str!("../🧫️fixtures/🔣️.json"), JsonMemberPolicy::Reject).unwrap();
    let outputs = semio_framework_pack_json::parse(include_str!("../🧫️fixtures/🎯️outputs/🔣️.json"), JsonMemberPolicy::Reject).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let expected = outputs["cases"].as_array().unwrap().iter().find(|row| row["id"].as_str() == Some(id)).unwrap();
        let expected: Vec<RustGeneratedTokenOutput> = expected["outputs"].as_array().unwrap().iter().map(expected_output).collect();
        let actual = inspect_rust_generated_token_outputs(case["source"].as_str().unwrap()).unwrap();
        assert_eq!(actual, expected, "{id}");
        assert_eq!(actual[0].origin.provider(), "quote");
        assert_eq!(actual[0].origin.symbol(), "quote");
        let expressions: Vec<&str> = actual.iter().flat_map(|row| row.inputs.iter().map(|input| input.expression.as_str())).collect();
        let expected: Vec<&str> = case["expressions"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect();
        assert_eq!(expressions, expected, "{id}");
        eprintln!("[DEBUG] Rust generated input {id}: {actual:?}");
    }
}

#[test]
fn unresolved_quote_origins_refuse_and_local_authored_macro_is_never_a_generator() {
    let fixture = semio_framework_pack_json::parse(include_str!("../🧫️fixtures/🔣️.json"), JsonMemberPolicy::Reject).unwrap();
    for case in fixture["unsupported"].as_array().unwrap() {
        let error = inspect_rust_generated_token_outputs(case["source"].as_str().unwrap()).unwrap_err();
        assert_eq!(error.kind, RustGeneratorErrorKind::UnresolvedOrigin);
        assert_eq!(error.code(), "unresolved-generator-origin");
        eprintln!("[DEBUG] Rust generated refusal {}: {error}", case["id"].as_str().unwrap());
    }
    for case in fixture["authoredMacros"].as_array().unwrap() {
        assert!(inspect_rust_generated_token_outputs(case["source"].as_str().unwrap()).unwrap().is_empty());
        assert!(case["source"].as_str().unwrap().contains("include_str!($path)"));
        eprintln!("[DEBUG] Rust authored local quote remains an authored macro {}", case["id"].as_str().unwrap());
    }
}

