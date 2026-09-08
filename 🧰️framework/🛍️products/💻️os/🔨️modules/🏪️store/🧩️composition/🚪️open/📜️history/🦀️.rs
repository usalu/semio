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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
