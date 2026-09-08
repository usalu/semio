use super::*;
use crate::os_io::{ArtifactDialect, ArtifactRef};
use crate::os_store::{OwnedSchemaDecodeCredits, OwnedSchemaDecodePage, OwnedSchemaDecodePages, OWNED_SCHEMA_DECODE_PAGE_BYTES};
use semio_framework_job::{root_cancel_token, Generation, OperationId, StepBudget};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixture/🔣️.json")).unwrap()
}
fn hex(value: &str) -> Vec<u8> {
    value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

fn history(fixture: &serde_json::Value, operation: &str) -> Vec<u8> {
    let mut history = hex(fixture["historyHex"].as_str().unwrap());
    match operation {
        "torn" => history.extend_from_slice(&[10, 1, 2]),
        "paged-tail" => {
            history.extend_from_slice(&[136, 39]);
            history.resize(4499, 0);
        }
        "header-only" => history.truncate(32),
        "bad-crc" => history[80] ^= 1,
        "exact" => {}
        _ => panic!("unknown neutral input"),
    }
    history
}

fn request(history: &[u8]) -> MemberOpenRequest {
    let mut bytes = vec![1, 170];
    bytes.extend_from_slice(history);
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: bytes.len().div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES), maximum_bytes: bytes.len() }).unwrap();
    for chunk in bytes.chunks(OWNED_SCHEMA_DECODE_PAGE_BYTES) {
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(chunk).unwrap()).unwrap();
    }
    pages.seal().unwrap();
    let expected = ArtifactRef { artifact_id: "flow-member".into(), dialect: ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() } };
    MemberOpenRequest::new(OperationId(7), Generation(11), 1000, expected, None, pages).admit(1).unwrap_or_else(|_| panic!("neutral request admission"))
}

fn retire(owner: &mut dyn ErasedSnapshotRetirement, grant: usize) -> usize {
    let mut released = 0;
    for _ in 0..20_000 {
        match owner.close_step(1, grant).unwrap() {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= grant);
                released += released_bytes;
            }
            SnapshotRetirementStep::Complete => {
                assert!(owner.terminal_is_empty());
                return released;
            }
            SnapshotRetirementStep::Blocked => panic!("exclusive retained input cannot block"),
        }
    }
    panic!("bounded input retirement did not converge");
}

fn drive(owner: &mut MemberHistoryVerification, fuel: u64) -> MemberHistoryInputStep {
    let mut sequence = 0;
    for _ in 0..20_000 {
        let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(fuel, 999), root_cancel_token(), || Some(1), &mut sequence);
        let copied = owner.copied;
        let before = owner.scanner.as_ref().map_or(0, RetainedSprVerification::consumed);
        let result = owner.step(&mut cx);
        let after = owner.scanner.as_ref().map_or(0, RetainedSprVerification::consumed);
        assert!((owner.copied - copied) as u64 + after - before <= fuel - cx.fuel_remaining());
        assert!(owner.copied as u64 == after || owner.copied as u64 == after + 1);
        if !matches!(result, MemberHistoryInputStep::Pending(_)) {
            return result;
        }
    }
    panic!("bounded verifier did not converge");
}

#[test]
fn member_history_verification_retains_input_and_bounds_verified_handoff() {
    let fixture = fixture();
    for row in fixture["inputs"].as_array().unwrap() {
        let bytes = history(&fixture, row["operation"].as_str().unwrap());
        for grant in [1, 7, 4096] {
            let mut owner = MemberHistoryVerification::new(request(&bytes), RetainedSprLimits::default()).unwrap_or_else(|_| panic!("admitted owner"));
            let mut sequence = 0;
            let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(0, 999), root_cancel_token(), || Some(1), &mut sequence);
            assert!(matches!(owner.step(&mut cx), MemberHistoryInputStep::Pending(_)));
            assert_eq!(owner.copied, 0);
            assert!(owner.take_ready(&mut cx).unwrap().is_none());
            let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(7, 1), root_cancel_token(), || Some(1), &mut sequence);
            assert!(matches!(owner.step(&mut cx), MemberHistoryInputStep::Pending(_)));
            assert_eq!(owner.copied, 0);
            let result = drive(&mut owner, grant);
            assert_eq!(owner.retained_input_bytes(), bytes.len() + 2);
            if row["error"].is_null() {
                assert_eq!(result, MemberHistoryInputStep::Ready);
                let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(0, 999), root_cancel_token(), || Some(1), &mut sequence);
                assert!(owner.take_ready(&mut cx).unwrap().is_none());
                assert_eq!(owner.retained_input_bytes(), bytes.len() + 2);
                let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 1), root_cancel_token(), || Some(1), &mut sequence);
                assert!(owner.take_ready(&mut cx).unwrap().is_none());
                assert_eq!(owner.retained_input_bytes(), bytes.len() + 2);
                let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(4096, 999), root_cancel_token(), || Some(1), &mut sequence);
                let mut witness = owner.take_ready(&mut cx).unwrap().expect("single ready owner");
                assert!(owner.terminal_is_empty());
                assert!(matches!(owner.take_ready(&mut cx), Err(MemberOpenDiagnostic::Stale)));
                assert_eq!(witness.verified_end(), row["verifiedEnd"].as_u64().unwrap());
                assert_eq!(witness.tail_bytes(), row["tailBytes"].as_u64().unwrap());
                let mut output = [204; 32];
                let end = witness.verified_end() as usize;
                assert_eq!(witness.copy_verified_history_chunk(end - 1, &mut output, &mut cx).unwrap(), 1);
                assert_eq!(output[0], bytes[end - 1]);
                assert!(output[1..].iter().all(|byte| *byte == 204));
                assert_eq!(witness.copy_verified_history_chunk(end, &mut output, &mut cx).unwrap(), 0);
                assert_eq!(witness.copy_verified_history_chunk(end + 1, &mut output, &mut cx), Err(MemberOpenDiagnostic::Malformed));
                assert_eq!(retire(&mut witness, grant as usize), row["retiredBytes"].as_u64().unwrap() as usize);
            } else {
                assert_eq!(result, MemberHistoryInputStep::Rejected(MemberOpenDiagnostic::Malformed));
                assert_eq!(retire(&mut owner, grant as usize), row["retiredBytes"].as_u64().unwrap() as usize);
            }
        }
    }
    eprintln!("[DEBUG] retained history input: 5 wire cases x 3 grants, separate copy/hash credit, scoped single handoff, exact 4531-byte paged retirement, no typed publication");
}

#[test]
fn member_history_verification_rechecks_every_owner_transition_and_retires_exact_bytes() {
    let fixture = fixture();
    let bytes = history(&fixture, "exact");
    for row in fixture["lifecycle"].as_array().unwrap() {
        let mut owner = MemberHistoryVerification::new(request(&bytes), RetainedSprLimits::default()).unwrap_or_else(|_| panic!("admitted owner"));
        let mut sequence = 0;
        let at = row["at"].as_str().unwrap();
        if at == "pending-byte" {
            let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(2, 999), root_cancel_token(), || Some(1), &mut sequence);
            assert!(matches!(owner.step(&mut cx), MemberHistoryInputStep::Pending(_)));
            assert!(owner.pending.is_some());
        } else if at == "ready" || at == "witness" {
            assert_eq!(drive(&mut owner, 7), MemberHistoryInputStep::Ready);
        }
        let mut witness = if at == "witness" {
            let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
            owner.take_ready(&mut cx).unwrap()
        } else {
            None
        };
        let event = row["event"].as_str().unwrap();
        let cancel = root_cancel_token();
        if event == "none" {
            let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(7, 999), cancel, || Some(1), &mut sequence);
            let mut witness = owner.take_ready(&mut cx).unwrap().expect("positive exact-one handoff");
            assert!(owner.terminal_is_empty());
            assert!(matches!(owner.take_ready(&mut cx), Err(MemberOpenDiagnostic::Stale)));
            assert_eq!(witness.retained_input_bytes(), row["retainedBytes"].as_u64().unwrap() as usize);
            assert_eq!(retire(&mut witness, 7), row["retiredBytes"].as_u64().unwrap() as usize);
            continue;
        }
        if event == "cancel" {
            cancel.cancel_now();
        }
        let operation = if event == "operation" { 8 } else { 7 };
        let generation = if event == "generation" { 12 } else { 11 };
        let clock: fn() -> Option<u64> = match event {
            "expired" => || Some(1000),
            "clock-absent" => || None,
            _ => || Some(1),
        };
        let expected = match row["error"].as_str().unwrap() {
            "stale" => MemberOpenDiagnostic::Stale,
            "cancelled" => MemberOpenDiagnostic::Cancelled,
            "expired" => MemberOpenDiagnostic::Expired,
            _ => unreachable!(),
        };
        let mut cx = StepContext::new(OperationId(operation), Generation(generation), StepBudget::new(7, 999), cancel, clock, &mut sequence);
        let mut output = [204; 4];
        if let Some(witness) = witness.as_mut() {
            assert_eq!(witness.copy_verified_history_chunk(0, &mut output, &mut cx), Err(expected));
        } else if at == "ready" {
            assert!(matches!(owner.take_ready(&mut cx), Err(error) if error == expected));
        } else {
            assert_eq!(owner.step(&mut cx), MemberHistoryInputStep::Rejected(expected));
        }
        assert_eq!(output, [204; 4]);
        let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(7, 999), root_cancel_token(), || Some(1), &mut sequence);
        if let Some(witness) = witness.as_mut() {
            assert_eq!(witness.copy_verified_history_chunk(0, &mut output, &mut cx), Err(expected));
            assert_eq!(witness.retained_input_bytes(), row["retainedBytes"].as_u64().unwrap() as usize);
            assert_eq!(retire(witness, 1), row["retiredBytes"].as_u64().unwrap() as usize);
        } else {
            assert_eq!(owner.step(&mut cx), MemberHistoryInputStep::Rejected(expected));
            assert_eq!(owner.retained_input_bytes(), row["retainedBytes"].as_u64().unwrap() as usize);
            assert_eq!(retire(&mut owner, 1), row["retiredBytes"].as_u64().unwrap() as usize);
        }
    }
    let mut closed = request(&bytes);
    assert_eq!(retire(&mut closed, 7), 287);
    let failure = MemberHistoryVerification::new(closed, RetainedSprLimits::default()).err().expect("retired admission fails without panic");
    assert_eq!(failure.diagnostic, MemberOpenDiagnostic::Stale);
    assert!(failure.request.terminal_is_empty());
    eprintln!("[DEBUG] retained history lifecycle: 13 owner traces, exact-one transfer, sticky denial before/after handoff, retired-request preservation, zero semantic hydration");
}
