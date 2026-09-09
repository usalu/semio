use super::*;

#[test]
fn inference_approval_request_accepts_only_job_digest_and_exact_body_bound() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/✅️inference-approval-v1/🔣️.json")).unwrap();
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
