//! ✅ Approval identifies an offered job and digest; execution authority is never client supplied.

use serde::{Deserialize, Serialize};
use super::{hex, REQUEST_MAX_BYTES};
use crate::inference::InferenceErrorV1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceApprovalRequestV1 {
    pub schema: String,
    pub version: u32,
    pub job_id: String,
    pub proposal_hash: String,
}

impl InferenceApprovalRequestV1 {
    pub fn decode(bytes: &[u8]) -> Result<Self, InferenceErrorV1> {
        if bytes.len() > REQUEST_MAX_BYTES { return Err(InferenceErrorV1::Bounds); }
        let request: Self = serde_json::from_slice(bytes).map_err(|_| InferenceErrorV1::Invalid)?;
        if request.schema != "semio.hub.inference-approval/v1" || request.version != 1 || !hex(&request.job_id, 32) || !hex(&request.proposal_hash, 64) {
            return Err(InferenceErrorV1::Invalid);
        }
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inference_approval_request_accepts_only_job_digest_and_exact_body_bound() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/✅️inference-approval-v1/🔣️.json")).unwrap();
        let request = &fixture["request"];
        let bytes = serde_json::to_vec(request).unwrap();
        let decoded = InferenceApprovalRequestV1::decode(&bytes).unwrap();
        assert_eq!(decoded.job_id, request["jobId"]);
        assert_eq!(decoded.proposal_hash, request["proposalHash"]);
        assert_eq!(REQUEST_MAX_BYTES as u64, fixture["maximumBytes"]);
        for hostile in fixture["hostile"].as_array().unwrap() {
            let mut candidate = request.clone();
            candidate[hostile["field"].as_str().unwrap()] = hostile["value"].clone();
            assert_eq!(InferenceApprovalRequestV1::decode(&serde_json::to_vec(&candidate).unwrap()), Err(InferenceErrorV1::Invalid), "{}", hostile["field"]);
        }
        let mut boundary = bytes;
        boundary.resize(REQUEST_MAX_BYTES, b' ');
        assert!(InferenceApprovalRequestV1::decode(&boundary).is_ok());
        boundary.push(b' ');
        assert_eq!(InferenceApprovalRequestV1::decode(&boundary), Err(InferenceErrorV1::Bounds));
    }
}
