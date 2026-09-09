
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
fn fault_inline_layout_stays_within_the_language_neutral_budget() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧯️fault/🔣️.json")).unwrap();
    let maximum = fixture["maximumInlineBytes"].as_u64().unwrap() as usize;
    let actual = size_of::<Fault>();
    eprintln!("[DEBUG] Fault inline bytes={actual} maximum={maximum}");
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
        assert_eq!(rebuilt, fault);
    }
    eprintln!("[DEBUG] Fault wire projection matched four independent JSON vectors and preserved all scope fields");
}
