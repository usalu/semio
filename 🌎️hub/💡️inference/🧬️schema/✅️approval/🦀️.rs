//! ✅ Approval identifies an offered job and digest; execution authority is never client supplied.

use super::{hex, REQUEST_MAX_BYTES};
use crate::inference::InferenceErrorV1;
use serde::{Deserialize, Serialize};

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
        if bytes.len() > REQUEST_MAX_BYTES {
            return Err(InferenceErrorV1::Bounds);
        }
        let request: Self = serde_json::from_slice(bytes).map_err(|_| InferenceErrorV1::Invalid)?;
        if request.schema != "semio.hub.inference-approval/v1" || request.version != 1 || !hex(&request.job_id, 32) || !hex(&request.proposal_hash, 64) {
            return Err(InferenceErrorV1::Invalid);
        }
        Ok(request)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
