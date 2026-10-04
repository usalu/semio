//! 🧪️ Source-positioned refusal ownership crosses the actual container error boundary.
use super::*;
use semio_framework_diagnostic::{TextError, TextSpan};
use semio_framework_value::ValueRefusalKind;

fn kind(text: &str) -> ValueRefusalKind {
    match text {
        "InvalidValue" => ValueRefusalKind::InvalidValue,
        "Canceled" => ValueRefusalKind::Canceled,
        "OwnershipLimit" => ValueRefusalKind::OwnershipLimit,
        "AllocationFailed" => ValueRefusalKind::AllocationFailed,
        "WorkLimit" => ValueRefusalKind::WorkLimit,
        "DepthLimit" => ValueRefusalKind::DepthLimit,
        "UnsupportedOwner" => ValueRefusalKind::UnsupportedOwner,
        "InvariantViolated" => ValueRefusalKind::InvariantViolated,
        _ => panic!("unknown authored source refusal kind"),
    }
}

#[test]
fn canonical_pack_text_refusal_retains_kind_span_expected_and_original_cause() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📍️text-refusal/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let expected_kind = kind(row["kind"].as_str().unwrap());
        let span = TextSpan {
            line: u32::try_from(row["span"]["line"].as_u64().unwrap()).unwrap(),
            column: u32::try_from(row["span"]["column"].as_u64().unwrap()).unwrap(),
            length: u32::try_from(row["span"]["length"].as_u64().unwrap()).unwrap(),
        };
        let message = row["message"].as_str().unwrap();
        let error = match row["expected"].as_str() {
            Some(expected) => TextError::expected(expected_kind, message, span, expected),
            None => TextError::new(expected_kind, message, span),
        };
        let allocation = error.message.as_ptr();
        let wrapped = PackError::from(error);
        let PackError::Refusal(PackRefusal::TextRefusal(refusal)) = &wrapped else { panic!("typed source refusal variant") };
        assert_eq!(refusal.kind, expected_kind);
        assert_eq!(refusal.span, span);
        assert_eq!(refusal.expected.as_deref(), row["expected"].as_str());
        assert_eq!(refusal.message.as_ptr(), allocation);
        assert_eq!(wrapped.to_string(), row["display"].as_str().unwrap());
        let source = std::error::Error::source(&wrapped).unwrap().downcast_ref::<TextError>().unwrap();
        assert!(std::ptr::eq(source, refusal));
    }
}
