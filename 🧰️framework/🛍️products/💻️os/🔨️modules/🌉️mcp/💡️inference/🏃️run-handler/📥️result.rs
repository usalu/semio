//! 🏁️ Settles the result of one guest operation after its retained origin closes.
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_os_mcp_port::channel_port::InferenceOutcome;

#[derive(Debug)]
pub(crate) struct GuestInferenceRun {
    pub inference_schema: String,
    pub payload: Option<Vec<u8>>,
}

/// 🔎️ Projects guest-owned bytes as structured JSON where possible.
pub(crate) fn inference_run_result_value(payload: &[u8]) -> Option<serde_json::Value> {
    std::str::from_utf8(payload).ok().and_then(|text| serde_json::from_str(text).ok())
}

pub(super) fn completed_guest_run(outcome: InferenceOutcome) -> Result<GuestInferenceRun, GatewayError> {
    if outcome.complete {
        return Ok(GuestInferenceRun { inference_schema: outcome.inference_schema, payload: outcome.payload });
    }
    let payload = outcome.payload.as_deref();
    Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, "guest inference returned an incomplete result")
        .with_details(serde_json::json!({
            "field": "complete",
            "status": "FAILED",
            "complete": false,
            "inferenceSchema": outcome.inference_schema,
            "payload": payload.and_then(inference_run_result_value),
            "payloadBytes": payload.map_or(0, <[u8]>::len),
        })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_completed_guest_inference_can_settle_as_success_or_offer_a_commit() {
        let cases: serde_json::Value = serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/guest-result.json")).unwrap();
        for row in cases.as_array().unwrap().iter().filter(|row| row["outcome"] != "WRONG_SCHEMA") {
            let name = row["name"].as_str().unwrap();
            let payload = row["wire"].as_str().map(|value| value.as_bytes().to_vec());
            let outcome = InferenceOutcome {
                inference_schema: row["returned"].as_str().unwrap().into(),
                complete: row["complete"].as_bool().unwrap(),
                payload: payload.clone(),
            };
            let result = completed_guest_run(outcome);
            if row["outcome"] == "SUCCEEDED" {
                let run = result.unwrap();
                assert_eq!(run.inference_schema, row["requested"].as_str().unwrap(), "{name}");
                assert_eq!(run.payload, payload, "{name}");
            } else {
                let error = result.unwrap_err();
                assert_eq!(error.code, GatewayErrorCode::PreconditionFailed, "{name}");
                for field in ["payload", "payloadBytes"] {
                    assert_eq!(error.details[field], row[field], "{name}");
                }
                assert_eq!(error.details["inferenceSchema"], row["requested"], "{name}");
                assert_eq!(error.details["complete"], false, "{name}");
                assert_eq!(error.details["status"], row["outcome"], "{name}");
            }
        }
    }
}
