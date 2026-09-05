//! 📜️ One retained request owns verification, cancellation and the committed-input handoff.
//! Framing proves no semantic identity, typed snapshot, decompression or member publication.

#[path = "🗂️dictionary/🦀️.rs"]
pub(crate) mod dictionary;

#[path = "🏭️factory/🦀️.rs"]
pub(crate) mod factory;

use super::{ErasedSnapshotRetirement, MemberOpenAdmissionError, MemberOpenDiagnostic, MemberOpenInputStep, MemberOpenPhase, MemberOpenProgress, MemberOpenRequest, SnapshotRetirementStep};
use crate::os_spr::format::retained::{RetainedSprDiagnostic, RetainedSprLimits, RetainedSprVerification, VerifiedSprSpan};
use semio_framework_job::StepContext;
use std::mem::ManuallyDrop;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MemberHistoryInputStep {
    Pending(MemberOpenProgress),
    Ready,
    Rejected(MemberOpenDiagnostic),
}

/// 🧵️ Copy and verification have separate fuel charges; one pending byte survives a yield.
pub(crate) struct MemberHistoryVerification {
    request: ManuallyDrop<Option<MemberOpenRequest>>,
    scanner: Option<RetainedSprVerification>,
    span: Option<VerifiedSprSpan>,
    limits: RetainedSprLimits,
    total: usize,
    copied: usize,
    pending: Option<u8>,
    diagnostic: Option<MemberOpenDiagnostic>,
}

impl MemberHistoryVerification {
    pub fn new(request: MemberOpenRequest, limits: RetainedSprLimits) -> Result<Self, MemberOpenAdmissionError> {
        if let Err(diagnostic) = request.admitted_expected() {
            return Err(MemberOpenAdmissionError { diagnostic, request });
        }
        Ok(Self { request: ManuallyDrop::new(Some(request)), scanner: None, span: None, limits, total: 0, copied: 0, pending: None, diagnostic: None })
    }

    pub fn retained_input_bytes(&self) -> usize {
        self.request.as_ref().map_or(0, MemberOpenRequest::retained_input_bytes)
    }

    fn reject(&mut self, diagnostic: MemberOpenDiagnostic) -> MemberOpenDiagnostic {
        *self.diagnostic.get_or_insert(diagnostic)
    }

    fn check(&mut self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        if let Some(diagnostic) = self.diagnostic {
            return Err(diagnostic);
        }
        let result = self.request.as_ref().ok_or(MemberOpenDiagnostic::Stale).and_then(|request| request.check_step_authority(cx));
        result.map_err(|diagnostic| self.reject(diagnostic))
    }

    fn progress(&self) -> MemberHistoryInputStep {
        MemberHistoryInputStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Header, completed: self.scanner.as_ref().map_or(0, RetainedSprVerification::consumed), total: self.total as u64 })
    }

    pub fn step(&mut self, cx: &mut StepContext<'_>) -> MemberHistoryInputStep {
        if let Err(diagnostic) = self.check(cx) {
            return MemberHistoryInputStep::Rejected(diagnostic);
        }
        if self.span.is_some() {
            return MemberHistoryInputStep::Ready;
        }
        if self.scanner.is_none() {
            match self.request.as_mut().expect("checked retained request").step_input(cx) {
                MemberOpenInputStep::Pending(progress) => return MemberHistoryInputStep::Pending(progress),
                MemberOpenInputStep::Rejected(diagnostic) => return MemberHistoryInputStep::Rejected(self.reject(diagnostic)),
                MemberOpenInputStep::Framed(frame) => {
                    self.total = frame.history_range().1;
                    match RetainedSprVerification::new(self.total as u64, self.limits) {
                        Ok(scanner) => self.scanner = Some(scanner),
                        Err(error) => return MemberHistoryInputStep::Rejected(self.reject(diagnostic(error))),
                    }
                }
            }
        }
        cx.set_stage("member-open.history.verify");
        while !cx.should_yield() {
            if let Err(diagnostic) = self.check(cx) {
                return MemberHistoryInputStep::Rejected(diagnostic);
            }
            if self.scanner.as_ref().expect("retained scanner").consumed() == self.total as u64 {
                match self.scanner.as_mut().expect("retained scanner").finish() {
                    Ok(span) if span.sequence() > 0 => self.span = Some(span),
                    Ok(_) => return MemberHistoryInputStep::Rejected(self.reject(MemberOpenDiagnostic::Malformed)),
                    Err(error) => return MemberHistoryInputStep::Rejected(self.reject(diagnostic(error))),
                }
                if let Err(diagnostic) = self.check(cx) {
                    return MemberHistoryInputStep::Rejected(diagnostic);
                }
                return MemberHistoryInputStep::Ready;
            }
            if let Some(byte) = self.pending {
                let mut fuel = 1;
                let result = self.scanner.as_mut().expect("retained scanner").push(&[byte], &mut fuel);
                cx.consume_fuel(1);
                self.pending = None;
                if let Err(error) = result {
                    return MemberHistoryInputStep::Rejected(self.reject(diagnostic(error)));
                }
            } else {
                let mut byte = [0; 1];
                match self.request.as_ref().expect("checked retained request").copy_history_chunk(self.copied, &mut byte, cx) {
                    Ok(0) => return self.progress(),
                    Ok(1) => {
                        self.pending = Some(byte[0]);
                        self.copied += 1;
                    }
                    Ok(_) => unreachable!("one-byte destination bounds copy"),
                    Err(diagnostic) => return MemberHistoryInputStep::Rejected(self.reject(diagnostic)),
                }
            }
            if let Err(diagnostic) = self.check(cx) {
                return MemberHistoryInputStep::Rejected(diagnostic);
            }
        }
        self.progress()
    }

    pub fn take_ready(&mut self, cx: &mut StepContext<'_>) -> Result<Option<VerifiedMemberHistoryInput>, MemberOpenDiagnostic> {
        self.check(cx)?;
        if self.span.is_none() || cx.should_yield() {
            return Ok(None);
        }
        cx.consume_fuel(1);
        self.check(cx)?;
        if cx.deadline_exceeded() {
            return Ok(None);
        }
        let request = self.request.take().expect("checked retained request");
        let span = self.span.take().expect("verified span");
        self.scanner = None;
        self.pending = None;
        Ok(Some(VerifiedMemberHistoryInput { request: ManuallyDrop::new(Some(request)), span: Some(span), diagnostic: None }))
    }
}

/// 🔒️ Private-field, non-clone input owner; only the committed range is readable under authority.
pub(crate) struct VerifiedMemberHistoryInput {
    request: ManuallyDrop<Option<MemberOpenRequest>>,
    span: Option<VerifiedSprSpan>,
    diagnostic: Option<MemberOpenDiagnostic>,
}

impl VerifiedMemberHistoryInput {
    pub fn verified_end(&self) -> u64 {
        self.span.as_ref().map_or(0, VerifiedSprSpan::end)
    }
    pub fn tail_bytes(&self) -> u64 {
        self.span.as_ref().map_or(0, VerifiedSprSpan::tail)
    }
    pub fn retained_input_bytes(&self) -> usize {
        self.request.as_ref().map_or(0, MemberOpenRequest::retained_input_bytes)
    }

    pub fn copy_verified_history_chunk(&mut self, offset: usize, output: &mut [u8], cx: &mut StepContext<'_>) -> Result<usize, MemberOpenDiagnostic> {
        if let Some(diagnostic) = self.diagnostic {
            return Err(diagnostic);
        }
        let result = self.copy(offset, output, cx);
        if let Err(diagnostic) = result {
            self.diagnostic = Some(diagnostic);
        }
        result
    }

    fn copy(&self, offset: usize, output: &mut [u8], cx: &mut StepContext<'_>) -> Result<usize, MemberOpenDiagnostic> {
        let request = self.request.as_ref().ok_or(MemberOpenDiagnostic::Stale)?;
        request.check_step_authority(cx)?;
        let end = usize::try_from(self.span.as_ref().ok_or(MemberOpenDiagnostic::Stale)?.end()).map_err(|_| MemberOpenDiagnostic::Capacity)?;
        let remaining = end.checked_sub(offset).ok_or(MemberOpenDiagnostic::Malformed)?;
        let maximum = output.len().min(remaining);
        let copied = request.copy_history_chunk(offset, &mut output[..maximum], cx)?;
        request.check_step_authority(cx)?;
        Ok(copied)
    }
}

fn diagnostic(error: RetainedSprDiagnostic) -> MemberOpenDiagnostic {
    match error {
        RetainedSprDiagnostic::Capacity => MemberOpenDiagnostic::Capacity,
        RetainedSprDiagnostic::Cancelled => MemberOpenDiagnostic::Cancelled,
        RetainedSprDiagnostic::State => MemberOpenDiagnostic::Stale,
        _ => MemberOpenDiagnostic::Malformed,
    }
}

fn close_request(request: &mut ManuallyDrop<Option<MemberOpenRequest>>, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
    let Some(retained) = request.as_mut() else {
        return Ok(SnapshotRetirementStep::Complete);
    };
    match retained.close_step(items, bytes)? {
        SnapshotRetirementStep::Complete if retained.terminal_is_empty() => {
            request.take();
            Ok(SnapshotRetirementStep::Complete)
        }
        SnapshotRetirementStep::Complete => Err("member history request returned false terminal".into()),
        step => Ok(step),
    }
}

impl ErasedSnapshotRetirement for MemberHistoryVerification {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.terminal_is_empty() {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.reject(MemberOpenDiagnostic::Cancelled);
        self.scanner = None;
        self.span = None;
        self.pending = None;
        close_request(&mut self.request, items, bytes)
    }
    fn terminal_is_empty(&self) -> bool {
        self.request.is_none() && self.scanner.is_none() && self.span.is_none() && self.pending.is_none()
    }
}

impl ErasedSnapshotRetirement for VerifiedMemberHistoryInput {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.terminal_is_empty() {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.diagnostic.get_or_insert(MemberOpenDiagnostic::Cancelled);
        self.span = None;
        close_request(&mut self.request, items, bytes)
    }
    fn terminal_is_empty(&self) -> bool {
        self.request.is_none() && self.span.is_none()
    }
}

impl Drop for MemberHistoryVerification {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "history verification input requires transfer or bounded retirement");
    }
}

impl Drop for VerifiedMemberHistoryInput {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "verified history input requires adoption or bounded retirement");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_io::{ArtifactDialect, ArtifactRef};
    use crate::os_store::{OwnedSchemaDecodeCredits, OwnedSchemaDecodePage, OwnedSchemaDecodePages, OWNED_SCHEMA_DECODE_PAGE_BYTES};
    use semio_framework_job::{root_cancel_token, Generation, OperationId, StepBudget};

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("🧫️fixture/🔣️.json")).unwrap()
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
}
