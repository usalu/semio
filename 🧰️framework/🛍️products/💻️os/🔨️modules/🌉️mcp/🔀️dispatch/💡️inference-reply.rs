//! 🎯️ Correlates an inference reply at the shared artifact channel boundary.
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_os_mcp_port::channel_port::{AppFrame, InferenceOutcome, map_fault};

pub(super) fn accept_inference_reply(requested_schema: &str, frame: Option<AppFrame>) -> Result<InferenceOutcome, GatewayError> {
    match frame {
        Some(AppFrame::Inferred { inference_schema, complete, payload }) if inference_schema == requested_schema => {
            Ok(InferenceOutcome { inference_schema, complete, payload })
        }
        Some(AppFrame::Inferred { inference_schema, .. }) => {
            Err(GatewayError::new(GatewayErrorCode::Internal, "guest inference answered for a different schema")
                .with_details(serde_json::json!({ "field": "inferenceSchema", "expectedInferenceSchema": requested_schema, "receivedInferenceSchema": inference_schema })))
        }
        Some(AppFrame::Error(fault)) => Err(map_fault(&fault)),
        Some(other) => Err(GatewayError::new(GatewayErrorCode::Internal, format!("`{requested_schema}` answered with {other:?} instead of an inference result"))),
        None => Err(GatewayError::new(GatewayErrorCode::Internal, format!("`{requested_schema}` produced no reply frame"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inference_reply_requires_the_original_schema_and_preserves_partial_outcomes() {
        let cases: serde_json::Value = serde_json::from_str(include_str!("../💡️inference/🧪️tests/🧫️fixtures/guest-result.json")).unwrap();
        for row in cases.as_array().unwrap() {
            let requested = row["requested"].as_str().unwrap();
            let name = row["name"].as_str().unwrap();
            let returned = row["returned"].as_str().unwrap();
            let complete = row["complete"].as_bool().unwrap();
            let payload = row["wire"].as_str().map(|value| value.as_bytes().to_vec());
            let reply = AppFrame::Inferred { inference_schema: returned.into(), complete, payload: payload.clone() };
            let outcome = accept_inference_reply(requested, Some(reply));
            if row["outcome"] == "WRONG_SCHEMA" {
                let error = outcome.unwrap_err();
                assert_eq!(error.code, GatewayErrorCode::Internal, "{name}");
                assert_eq!(error.details["field"], "inferenceSchema", "{name}");
                assert_eq!(error.details["expectedInferenceSchema"], requested, "{name}");
                assert_eq!(error.details["receivedInferenceSchema"], returned, "{name}");
            } else {
                assert_eq!(outcome.unwrap(), InferenceOutcome { inference_schema: requested.into(), complete, payload }, "{name}");
            }
        }
    }
}
