
use super::*;

#[test]
fn code_serializes_screaming_snake() {
    assert_eq!(serde_json::to_string(&GatewayErrorCode::InputInvalid).unwrap(), "\"INPUT_INVALID\"");
    assert_eq!(serde_json::to_string(&GatewayErrorCode::SideEffectRejected).unwrap(), "\"SIDE_EFFECT_REJECTED\"");
}

#[test]
fn tool_error_payload_carries_all_fields() {
    let error = GatewayError::new(GatewayErrorCode::NotFound, "no such capability").with_details(serde_json::json!({"capabilityId": "x"}));
    let payload = error.to_tool_error_payload();
    assert_eq!(payload["code"], "NOT_FOUND");
    assert_eq!(payload["message"], "no such capability");
    assert_eq!(payload["details"]["capabilityId"], "x");
    assert_eq!(payload["retryable"], false);
}

#[test]
fn json_rpc_parts_map_input_invalid_to_invalid_params() {
    let error = GatewayError::new(GatewayErrorCode::InputInvalid, "bad arg");
    let (code, message, _data) = error.to_json_rpc_parts();
    assert_eq!(code, -32602);
    assert_eq!(message, "bad arg");
}

#[test]
fn implements_std_error() {
    let error = GatewayError::new(GatewayErrorCode::Internal, "boom");
    let boxed: Box<dyn std::error::Error> = Box::new(error.clone());
    assert_eq!(boxed.to_string(), "Internal: boom");
    assert_eq!(error.to_string(), "Internal: boom");
}

#[test]
fn retryable_flag_round_trips() {
    let error = GatewayError::new(GatewayErrorCode::BudgetExceeded, "quota").retryable();
    assert!(error.retryable);
    let payload = error.to_tool_error_payload();
    assert_eq!(payload["retryable"], true);
}

#[test]
fn every_code_round_trips_through_json() {
    let codes = [
        GatewayErrorCode::InputInvalid,
        GatewayErrorCode::PreconditionFailed,
        GatewayErrorCode::RevisionConflict,
        GatewayErrorCode::PermissionDenied,
        GatewayErrorCode::ApprovalRequired,
        GatewayErrorCode::PluginUnavailable,
        GatewayErrorCode::SideEffectRejected,
        GatewayErrorCode::Cancelled,
        GatewayErrorCode::CompensationFailed,
        GatewayErrorCode::NotFound,
        GatewayErrorCode::BudgetExceeded,
        GatewayErrorCode::Internal,
    ];
    for code in codes {
        let json = serde_json::to_string(&code).unwrap();
        let back: GatewayErrorCode = serde_json::from_str(&json).unwrap();
        assert_eq!(back, code);
    }
}
