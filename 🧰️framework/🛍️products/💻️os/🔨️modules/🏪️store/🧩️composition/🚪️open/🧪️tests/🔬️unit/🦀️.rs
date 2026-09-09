use super::*;
use crate::os_store::{OwnedSchemaDecodeCredits, OWNED_SCHEMA_DECODE_PAGE_BYTES};

fn request_for(bytes: &[u8]) -> MemberOpenRequest {
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: bytes.len().max(1).div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES), maximum_bytes: bytes.len().max(1) }).unwrap();
    for chunk in bytes.chunks(OWNED_SCHEMA_DECODE_PAGE_BYTES) {
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(chunk).unwrap()).unwrap();
    }
    pages.seal().unwrap();
    let expected = ArtifactRef { artifact_id: "member".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() } };
    MemberOpenRequest::new(OperationId(1), Generation(1), 1000, expected, None, pages).admit(1).unwrap_or_else(|_| panic!("admissible test input"))
}

fn retire_request(request: &mut MemberOpenRequest) {
    for _ in 0..100_000 {
        match request.close_step(1, 7).unwrap() {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 7),
            SnapshotRetirementStep::Complete => {
                assert!(request.terminal_is_empty());
                return;
            }
            SnapshotRetirementStep::Blocked => panic!("inline pages have no shared owner"),
        }
    }
    panic!("request retirement did not converge");
}

#[test]
fn member_open_input_framing_is_canonical_scoped_and_budgeted() {
    use semio_framework_job::{root_cancel_token, StepBudget};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["framing"].as_array().unwrap() {
        let bytes: Vec<u8> = serde_json::from_value(row["bytes"].clone()).unwrap();
        for fuel in [1, 2, 13] {
            let mut request = request_for(&bytes);
            let mut sequence = 0;
            let cancel = root_cancel_token();
            let mut zero = StepContext::new(OperationId(1), Generation(1), StepBudget::new(0, 999), cancel.clone(), || Some(1), &mut sequence);
            assert!(matches!(request.step_input(&mut zero), MemberOpenInputStep::Pending(MemberOpenProgress { completed: 0, .. })));
            let mut outcome = None;
            for _ in 0..16 {
                let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(fuel, 999), cancel.clone(), || Some(1), &mut sequence);
                let before = request.input_offset;
                let step = request.step_input(&mut cx);
                assert!(request.input_offset - before <= fuel as usize);
                if !matches!(step, MemberOpenInputStep::Pending(_)) {
                    outcome = Some(step);
                    break;
                }
            }
            match outcome.expect("bounded varint is terminal") {
                MemberOpenInputStep::Framed(frame) => {
                    assert!(row["reason"].is_null(), "{}", row["id"]);
                    assert_eq!(serde_json::json!(frame.snapshot_range()), row["snapshot"]);
                    assert_eq!(serde_json::json!(frame.history_range()), row["history"]);
                    let mut output = [0xcc; 8];
                    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(1, 999), cancel.clone(), || Some(1), &mut sequence);
                    assert_eq!(request.copy_snapshot_chunk(0, &mut output, &mut cx).unwrap(), 1);
                    assert_eq!(output[0], bytes[frame.snapshot_start]);
                    assert_eq!(&output[1..], &[0xcc; 7]);
                    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(8, 999), cancel.clone(), || Some(1), &mut sequence);
                    let count = request.copy_history_chunk(0, &mut output, &mut cx).unwrap();
                    assert_eq!(&output[..count], &bytes[frame.history_start..frame.history_start + count]);
                }
                MemberOpenInputStep::Rejected(reason) => {
                    assert_eq!(reason, MemberOpenDiagnostic::Malformed);
                    assert_eq!(row["reason"], "malformed");
                }
                MemberOpenInputStep::Pending(_) => unreachable!(),
            }
            assert_eq!(request.retained_input_bytes(), bytes.len());
            retire_request(&mut request);
        }
    }
    for (operation, generation, now, cancelled, expected) in
        [(2, 1, 1, false, MemberOpenDiagnostic::Stale), (1, 2, 1, false, MemberOpenDiagnostic::Stale), (1, 1, 1000, false, MemberOpenDiagnostic::Expired), (1, 1, 1, true, MemberOpenDiagnostic::Cancelled)]
    {
        let mut request = request_for(&[1, 97, 83]);
        let mut sequence = 0;
        let cancel = root_cancel_token();
        if cancelled {
            cancel.cancel_now();
        }
        let clock: fn() -> Option<u64> = if now == 1000 { || Some(1000) } else { || Some(1) };
        let mut cx = StepContext::new(OperationId(operation), Generation(generation), StepBudget::new(8, 2000), cancel, clock, &mut sequence);
        assert_eq!(request.step_input(&mut cx), MemberOpenInputStep::Rejected(expected));
        assert_eq!(request.input_offset, 0);
        let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(8, 999), root_cancel_token(), || Some(1), &mut sequence);
        assert_eq!(request.step_input(&mut cx), MemberOpenInputStep::Rejected(expected));
        assert_eq!(request.retained_input_bytes(), 3);
        retire_request(&mut request);
    }
    let mut request = request_for(&[1, 97, 83]);
    let mut sequence = 0;
    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(8, 1), root_cancel_token(), || Some(1), &mut sequence);
    assert!(matches!(request.step_input(&mut cx), MemberOpenInputStep::Pending(MemberOpenProgress { completed: 0, .. })));
    retire_request(&mut request);
    eprintln!("[DEBUG] member input: 8 canonical frames x 3 fuel grants, 4 sticky authority denials, exact step deadline and retained close");
}

#[test]
fn member_open_request_rejection_retains_exact_pages_and_identity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let bytes = row["bytes"].as_u64().unwrap() as usize;
        let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: bytes.max(1).div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES), maximum_bytes: bytes.max(1) }).unwrap();
        for chunk in vec![0x63; bytes].chunks(OWNED_SCHEMA_DECODE_PAGE_BYTES) {
            pages.admit_page(OwnedSchemaDecodePage::try_from_slice(chunk).unwrap()).unwrap();
        }
        if row["sealed"].as_bool().unwrap() {
            pages.seal().unwrap();
        }
        let expected = ArtifactRef { artifact_id: row["artifactId"].as_str().unwrap().into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() } };
        let owner = row["ownerChildId"].as_str().map(|child_id| OwnerRef { parent: ArtifactRef { artifact_id: "parent".into(), dialect: expected.dialect.clone() }, slot: "content".into(), child_id: child_id.into() });
        let request = MemberOpenRequest::new(OperationId(1), Generation(1), row["expiresAtUs"].as_u64().unwrap(), expected.clone(), owner.clone(), pages);
        let result = request.admit(row["nowUs"].as_u64().unwrap());
        assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
        let mut request = match result {
            Ok(request) => request,
            Err(rejected) => {
                let reason = match rejected.diagnostic {
                    MemberOpenDiagnostic::Unsealed => "unsealed",
                    MemberOpenDiagnostic::Empty => "empty",
                    MemberOpenDiagnostic::Expired => "expired",
                    MemberOpenDiagnostic::Identity => "identity",
                    MemberOpenDiagnostic::Owner => "owner",
                    _ => panic!("unexpected admission reason"),
                };
                assert_eq!(reason, row["reason"].as_str().unwrap());
                rejected.request
            }
        };
        assert_eq!(request.expected(), &expected);
        assert_eq!(request.owner(), owner.as_ref());
        assert_eq!(request.retained_input_bytes(), bytes);
        assert!(matches!(request.close_step(0, 0).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        assert_eq!(request.retained_input_bytes(), bytes);
        let identity_bytes = expected.artifact_id.len()
            + expected.dialect.artifact_kind.len()
            + expected.dialect.standard.len()
            + expected.dialect.subset.len()
            + owner.as_ref().map_or(0, |owner| owner.parent.artifact_id.len() + owner.parent.dialect.artifact_kind.len() + owner.parent.dialect.standard.len() + owner.parent.dialect.subset.len() + owner.slot.len() + owner.child_id.len());
        let mut released = 0;
        for _ in 0..100_000 {
            match request.close_step(1, 7).unwrap() {
                SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1 && released_bytes <= 7);
                    released += released_bytes;
                }
                SnapshotRetirementStep::Complete => break,
                SnapshotRetirementStep::Blocked => panic!("request owns no shared root"),
            }
        }
        assert!(request.terminal_is_empty());
        assert_eq!(released, bytes + identity_bytes, "{}: exact input and identity byte ownership", row["id"]);
    }
}
