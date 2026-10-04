
use super::*;

#[test]
fn describe_pairs_the_code_with_the_message() {
    let fault = Fault::new(FaultOrigin::Renderer, "renderer.command.rejected", "envelope set is full");
    assert_eq!(fault.describe(), "renderer.command.rejected: envelope set is full");
}

#[test]
fn describe_keeps_the_code_when_the_message_is_empty() {
    let fault = Fault::new(FaultOrigin::Os, "os.fault.decode", "");
    assert_eq!(fault.describe(), "os.fault.decode: ");
}

#[test]
fn describe_survives_a_wire_round_trip() {
    let fault = Fault::new(FaultOrigin::Plugin, "plugin.host.body-too-large", "body exceeds the admitted budget");
    let decoded = decode_fault_bytes(&encode_fault_bytes(&fault));
    assert_eq!(decoded.describe(), fault.describe());
}

#[test]
fn bounded_fault_wire_preserves_source_span_and_named_path() {
    let mut fault = Fault::new(FaultOrigin::App, "snapshot-edit.invalid-source", "unexpected token".repeat(256)).with_param("path", "/rows/2/name");
    fault.span = Some(TextSpan::with_length(4, 9, 1));
    let encoded = encode_fault_bytes_bounded(&fault, 480).expect("the bounded canonical carrier admits a narrowed source diagnostic");
    assert!(encoded.len() <= 480);
    let oracle: serde_json::Value = serde_json::from_slice(&encoded).expect("the independent JSON oracle accepts the bounded Fault wire");
    assert_eq!(oracle["span"], serde_json::json!({ "line": 4, "column": 9, "length": 1 }));
    assert_eq!(oracle["params"]["path"], "/rows/2/name");
    let decoded = try_decode_fault_bytes(&encoded).expect("the bounded Fault wire decodes");
    assert_eq!(decoded.code.0, "snapshot-edit.invalid-source");
    assert_eq!(decoded.span, fault.span);
    assert_eq!(decoded.param("path"), Some("/rows/2/name"));
}

#[test]
fn fault_inline_layout_stays_within_the_language_neutral_budget() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧯️fault/🔣️.json")).unwrap();
    let maximum = fixture["maximumInlineBytes"].as_u64().unwrap() as usize;
    let actual = size_of::<Fault>();
    assert!(actual <= maximum);
}

#[test]
fn fault_wire_projection_matches_language_neutral_serde_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧯️fault/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let input: DslValue = serde_json::from_value(row["input"].clone()).unwrap();
        let fault = Fault::from_value(input).unwrap();
        let actual: serde_json::Value = serde_json::from_slice(&encode_fault_bytes(&fault)).unwrap();
        assert_eq!(actual, row["expected"], "{}", row["id"]);
        assert_eq!(decode_fault_bytes(&encode_fault_bytes(&fault)), fault);
        let mut rebuilt = Fault::new(fault.origin, fault.code.clone(), fault.message.clone())
            .with_scope(FaultScope::from_value(fault.scope.to_value()).unwrap())
            .with_retryable(fault.retryable);
        rebuilt.severity = fault.severity;
        rebuilt.span = fault.span;
        rebuilt.causes = fault.causes.clone();
        rebuilt.params = fault.params.clone();
        assert_eq!(rebuilt, fault);
    }
}

#[test]
fn notice_params_are_named_data_never_read_from_the_message() {
    let fault = Fault::new(FaultOrigin::App, "generation3d.gumball.kind-unavailable", "widget kind brep.mesh.translate is unavailable").with_param("kind", "brep.mesh.translate").with_param("kind", "brep.xform.translate");
    assert_eq!(fault.param("kind"), Some("brep.xform.translate"));
    assert_eq!(fault.params.as_deref().map(|params| params.0.len()), Some(1));
    assert_eq!(decode_fault_bytes(&encode_fault_bytes(&fault)), fault);
    assert_eq!(Fault::new(FaultOrigin::App, "app.refused", "").param("kind"), None);
    assert!(["kind", "n", "widgetId"].iter().all(|name| is_fault_param_name(name)));
    assert!(["", "Kind", "1n", "widget-id", "{n}"].iter().all(|name| !is_fault_param_name(name)));
}
