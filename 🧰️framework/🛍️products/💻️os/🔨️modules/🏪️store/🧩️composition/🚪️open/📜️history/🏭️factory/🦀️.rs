//! 🏭️ Closed factory declarations stay paired with the retained input through semantic admission.
//! Selection is not space or pin authorization; no typed document is hydrated here.

use super::dictionary::{MemberHistoryDictionaryLimits, MemberHistoryDictionaryOwner, MemberHistoryDictionaryStep, VerifiedMemberHistoryDictionary};
use super::{ErasedSnapshotRetirement, MemberOpenDiagnostic, MemberOpenPhase, MemberOpenProgress, VerifiedMemberHistoryInput};
use crate::os_store::MemberFactory;
use semio_framework_job::StepContext;
use semio_framework_value::{ValueError, RetirementDemand};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
use std::{marker::PhantomData, mem::ManuallyDrop};

/// 🪪️ The owning MemberFactory macro emits these same four creation/open literals.
#[derive(Clone, Copy)]
pub struct MemberOpenDeclaration {
    pub kind: &'static str,
    pub standard: &'static str,
    pub subset: &'static str,
    pub schema: &'static str,
}

pub(crate) struct MemberFactorySelectionRejected {
    pub diagnostic: MemberOpenDiagnostic,
    pub input: VerifiedMemberHistoryInput,
}
pub(crate) enum MemberFactorySelectionStep {
    Pending(MemberOpenProgress),
    Ready,
    Rejected(MemberOpenDiagnostic),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Validate,
    Unique,
    Match,
    Complete,
}

pub(crate) struct MemberFactorySelection<M: MemberFactory> {
    input: ManuallyDrop<Option<VerifiedMemberHistoryInput>>,
    row: usize,
    earlier: usize,
    selected: Option<usize>,
    phase: Phase,
    completed: u64,
    diagnostic: Option<MemberOpenDiagnostic>,
    closing: bool,
    factory: PhantomData<fn() -> M>,
}

fn check_input(input: &Option<VerifiedMemberHistoryInput>, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
    let input = input.as_ref().ok_or(MemberOpenDiagnostic::Stale)?;
    if let Some(error) = input.diagnostic {
        return Err(error);
    }
    input.request.as_ref().ok_or(MemberOpenDiagnostic::Stale)?.check_step_authority(cx)
}

fn close_owner<O: ErasedSnapshotRetirement>(input: &mut Option<O>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    crate::os_store::artifact_retirement_owner_close(input, grant)
}

impl<M: MemberFactory> MemberFactorySelection<M> {
    pub(crate) fn begin(input: VerifiedMemberHistoryInput, cx: &StepContext<'_>) -> Result<Self, MemberFactorySelectionRejected> {
        let authority = input.diagnostic.map_or(Ok(()), Err).and_then(|_| input.request.as_ref().ok_or(MemberOpenDiagnostic::Stale)?.check_step_authority(cx));
        let diagnostic = authority.err().or_else(|| (M::OPEN_DECLARATIONS.len() > 64).then_some(MemberOpenDiagnostic::Capacity)).or_else(|| M::OPEN_DECLARATIONS.is_empty().then_some(MemberOpenDiagnostic::Identity));
        if let Some(diagnostic) = diagnostic {
            return Err(MemberFactorySelectionRejected { diagnostic, input });
        }
        Ok(Self { input: ManuallyDrop::new(Some(input)), row: 0, earlier: 0, selected: None, phase: Phase::Validate, completed: 0, diagnostic: None, closing: false, factory: PhantomData })
    }

    fn check(&mut self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        if let Some(error) = self.diagnostic {
            return Err(error);
        }
        let checked = if self.closing { Err(MemberOpenDiagnostic::Cancelled) } else { check_input(&self.input, cx) };
        if let Err(error) = checked {
            self.diagnostic = Some(error);
        }
        checked
    }

    fn unit(&mut self) -> Result<(), MemberOpenDiagnostic> {
        let declarations = M::OPEN_DECLARATIONS;
        if self.row == declarations.len() {
            if self.selected.is_none() {
                return Err(MemberOpenDiagnostic::Identity);
            }
            self.phase = Phase::Complete;
            return Ok(());
        }
        let declaration = &declarations[self.row];
        match self.phase {
            Phase::Validate => {
                let text = |value: &str| !value.is_empty() && value.len() <= 256 && !value.chars().any(char::is_control);
                if ![declaration.kind, declaration.standard, declaration.subset, declaration.schema].into_iter().all(text) || !semio_framework_artifact_reference::is_canonical_artifact_kind(declaration.kind) {
                    return Err(MemberOpenDiagnostic::Identity);
                }
                self.earlier = 0;
                self.phase = Phase::Unique;
            }
            Phase::Unique => {
                if self.earlier == self.row {
                    self.phase = Phase::Match;
                } else {
                    let previous = &declarations[self.earlier];
                    if (previous.kind, previous.standard, previous.subset) == (declaration.kind, declaration.standard, declaration.subset) {
                        return Err(MemberOpenDiagnostic::Identity);
                    }
                    self.earlier += 1;
                }
            }
            Phase::Match => {
                let expected = self.input.as_ref().and_then(|input| input.request.as_ref()).ok_or(MemberOpenDiagnostic::Stale)?.admitted_expected()?;
                if (expected.dialect.artifact_kind.as_str(), expected.dialect.standard.as_str(), expected.dialect.subset.as_str()) == (declaration.kind, declaration.standard, declaration.subset) {
                    self.selected = Some(self.row);
                }
                self.row += 1;
                self.phase = Phase::Validate;
            }
            Phase::Complete => {}
        }
        Ok(())
    }

    pub(crate) fn step(&mut self, cx: &mut StepContext<'_>) -> MemberFactorySelectionStep {
        if let Err(error) = self.check(cx) {
            return MemberFactorySelectionStep::Rejected(error);
        }
        cx.set_stage("member-open.factory.select");
        while self.phase != Phase::Complete && !cx.should_yield() {
            if let Err(error) = self.check(cx) {
                return MemberFactorySelectionStep::Rejected(error);
            }
            let result = self.unit();
            cx.consume_fuel(1);
            self.completed += 1;
            if let Err(error) = result {
                self.diagnostic = Some(error);
                return MemberFactorySelectionStep::Rejected(error);
            }
            if let Err(error) = self.check(cx) {
                return MemberFactorySelectionStep::Rejected(error);
            }
        }
        if self.phase == Phase::Complete {
            MemberFactorySelectionStep::Ready
        } else {
            let count = M::OPEN_DECLARATIONS.len() as u64;
            MemberFactorySelectionStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Validate, completed: self.completed, total: count * (count - 1) / 2 + count * 3 + 1 })
        }
    }

    pub(crate) fn take_ready(&mut self, cx: &mut StepContext<'_>) -> Result<Option<SelectedMemberHistoryInput<M>>, MemberOpenDiagnostic> {
        self.check(cx)?;
        if self.phase != Phase::Complete || cx.should_yield() {
            return Ok(None);
        }
        cx.consume_fuel(1);
        self.check(cx)?;
        if cx.deadline_exceeded() {
            return Ok(None);
        }
        let declaration = &M::OPEN_DECLARATIONS[self.selected.ok_or(MemberOpenDiagnostic::Identity)?];
        Ok(Some(SelectedMemberHistoryInput { input: ManuallyDrop::new(self.input.take()), declaration, diagnostic: None, closing: false, factory: PhantomData }))
    }
}

impl<M: MemberFactory> ErasedSnapshotRetirement for MemberFactorySelection<M> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items != 0 && grant.maximum_depth >= self.next_depth_demand()? { self.closing = true; self.selected = None; }
        close_owner(&mut self.input, grant)
    }
    fn terminal_is_empty(&self) -> bool { self.input.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.depth) }
}
impl<M: MemberFactory> MemberFactorySelection<M> {
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { crate::os_store::artifact_retirement_owner_demands(&self.input, body) }
}
impl<M: MemberFactory> Drop for MemberFactorySelection<M> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "factory selection requires retained close or handoff");
    }
}

pub(crate) struct SelectedMemberHistoryInput<M: MemberFactory> {
    input: ManuallyDrop<Option<VerifiedMemberHistoryInput>>,
    declaration: &'static MemberOpenDeclaration,
    diagnostic: Option<MemberOpenDiagnostic>,
    closing: bool,
    factory: PhantomData<fn() -> M>,
}

impl<M: MemberFactory> SelectedMemberHistoryInput<M> {
    fn check(&mut self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        if let Some(error) = self.diagnostic {
            return Err(error);
        }
        let checked = if self.closing { Err(MemberOpenDiagnostic::Cancelled) } else { check_input(&self.input, cx) };
        if let Err(error) = checked {
            self.diagnostic = Some(error);
        }
        checked
    }

    pub(crate) fn begin_dictionary(&mut self, limits: MemberHistoryDictionaryLimits, cx: &mut StepContext<'_>) -> Result<Option<SelectedMemberHistoryDictionary<M>>, MemberOpenDiagnostic> {
        self.check(cx)?;
        if cx.should_yield() {
            return Ok(None);
        }
        cx.consume_fuel(1);
        self.check(cx)?;
        if cx.deadline_exceeded() {
            return Ok(None);
        }
        let input = self.input.take().ok_or(MemberOpenDiagnostic::Stale)?;
        match MemberHistoryDictionaryOwner::begin(input, self.declaration.schema, limits, cx) {
            Ok(owner) => Ok(Some(SelectedMemberHistoryDictionary { owner: ManuallyDrop::new(Some(owner)), declaration: self.declaration, factory: PhantomData })),
            Err(rejected) => {
                self.input = ManuallyDrop::new(Some(rejected.input));
                self.diagnostic = Some(rejected.diagnostic);
                Err(rejected.diagnostic)
            }
        }
    }
}

impl<M: MemberFactory> ErasedSnapshotRetirement for SelectedMemberHistoryInput<M> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items != 0 && grant.maximum_depth >= self.next_depth_demand()? { self.closing = true; }
        close_owner(&mut self.input, grant)
    }
    fn terminal_is_empty(&self) -> bool { self.input.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.depth) }
}
impl<M: MemberFactory> SelectedMemberHistoryInput<M> {
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { crate::os_store::artifact_retirement_owner_demands(&self.input, body) }
}
impl<M: MemberFactory> Drop for SelectedMemberHistoryInput<M> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "selected input requires retained close or semantic handoff");
    }
}

pub(crate) struct SelectedMemberHistoryDictionary<M: MemberFactory> {
    owner: ManuallyDrop<Option<MemberHistoryDictionaryOwner>>,
    declaration: &'static MemberOpenDeclaration,
    factory: PhantomData<fn() -> M>,
}

impl<M: MemberFactory> SelectedMemberHistoryDictionary<M> {
    pub(crate) fn step(&mut self, cx: &mut StepContext<'_>) -> MemberHistoryDictionaryStep {
        self.owner.as_mut().map_or(MemberHistoryDictionaryStep::Rejected(MemberOpenDiagnostic::Stale), |owner| owner.step(cx))
    }
    pub(crate) fn take_ready(&mut self, cx: &mut StepContext<'_>) -> Result<Option<SelectedVerifiedMemberHistory<M>>, MemberOpenDiagnostic> {
        let owner = self.owner.as_mut().ok_or(MemberOpenDiagnostic::Stale)?;
        let Some(input) = owner.take_ready(cx)? else {
            return Ok(None);
        };
        self.owner.take();
        Ok(Some(SelectedVerifiedMemberHistory { input: ManuallyDrop::new(Some(input)), declaration: self.declaration, factory: PhantomData }))
    }
}

impl<M: MemberFactory> ErasedSnapshotRetirement for SelectedMemberHistoryDictionary<M> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        close_owner(&mut self.owner, grant)
    }
    fn terminal_is_empty(&self) -> bool { self.owner.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.depth) }
}
impl<M: MemberFactory> SelectedMemberHistoryDictionary<M> {
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { crate::os_store::artifact_retirement_owner_demands(&self.owner, body) }
}
impl<M: MemberFactory> Drop for SelectedMemberHistoryDictionary<M> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "selected dictionary requires bounded retirement");
    }
}

pub(crate) struct SelectedVerifiedMemberHistory<M: MemberFactory> {
    input: ManuallyDrop<Option<VerifiedMemberHistoryDictionary>>,
    declaration: &'static MemberOpenDeclaration,
    factory: PhantomData<fn() -> M>,
}

impl<M: MemberFactory> SelectedVerifiedMemberHistory<M> {
    pub(crate) fn check_step_authority(&mut self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        self.input.as_mut().ok_or(MemberOpenDiagnostic::Stale)?.check_step_authority(cx)
    }

    pub(crate) fn initial_history_is_exact(&self) -> bool {
        self.input.as_ref().is_some_and(VerifiedMemberHistoryDictionary::initial_history_is_exact)
    }

    pub(crate) fn verified_end(&self) -> u64 {
        self.input.as_ref().map_or(0, VerifiedMemberHistoryDictionary::verified_end)
    }

    pub(crate) fn copy_verified_history_chunk(&mut self, offset: usize, output: &mut [u8], cx: &mut StepContext<'_>) -> Result<usize, MemberOpenDiagnostic> {
        self.input.as_mut().ok_or(MemberOpenDiagnostic::Stale)?.copy_verified_history_chunk(offset, output, cx)
    }

    pub(crate) fn clone_initial_identity(&mut self, cx: &StepContext<'_>) -> Result<(semio_framework_artifact_reference::ArtifactRef, Option<crate::os_store::OwnerRef>, &'static str, crate::os_spr::ActorId), MemberOpenDiagnostic> {
        let (expected, owner, schema, actor) = self.input.as_mut().ok_or(MemberOpenDiagnostic::Stale)?.clone_initial_identity(cx)?;
        if schema != self.declaration.schema {
            return Err(MemberOpenDiagnostic::Identity);
        }
        Ok((expected, owner, schema, actor))
    }
}

impl<M: MemberFactory> ErasedSnapshotRetirement for SelectedVerifiedMemberHistory<M> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        close_owner(&mut self.input, grant)
    }
    fn terminal_is_empty(&self) -> bool { self.input.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.depth) }
}
impl<M: MemberFactory> SelectedVerifiedMemberHistory<M> {
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { crate::os_store::artifact_retirement_owner_demands(&self.input, body) }
}
impl<M: MemberFactory> Drop for SelectedVerifiedMemberHistory<M> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "selected semantic input requires bounded retirement");
    }
}

#[cfg(test)]
#[path = "🧪️tests/🏭️factory/🦀️.rs"]
mod tests;
