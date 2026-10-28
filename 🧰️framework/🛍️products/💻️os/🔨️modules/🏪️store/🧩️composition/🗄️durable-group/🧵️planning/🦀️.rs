//! 🧵️ Durable planning retains bounded output around the original canonical byte producer.
use super::*;
use semio_framework_os_store_canonical_json::{ArtifactCanonicalJson, ArtifactCanonicalJsonCursor};
use semio_framework_value::{NativeEncodeControl, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_os_store_durable_group_wire::lifted_durable_group::{DurableBorrowedValue,DurableBorrowedDecisionFields};
/// 🪪️ Projects the same owned decision into canonical wire fields without cloning its payloads.
pub trait DurableDecisionSource {
    fn fields(&self) -> DurableBorrowedDecisionFields<'_>;
    fn set_anchor_hash(&mut self, hash: String);
    fn set_decision_hash(&mut self, hash: String);
}
pub fn decision_source(decision: &impl DurableDecisionSource, unsigned: bool) -> DurableBorrowedValue<'_> { let fields = decision.fields(); if unsigned { DurableBorrowedValue::Unsigned(fields) } else { DurableBorrowedValue::Decision(fields) } }

#[path = "🎒️pack/🦀️.rs"]
mod pack;
pub use pack::DurablePackOutputV1;
#[path = "🧷️member/🦀️.rs"]
mod member;
pub use member::DurableUnboundPlanningV1;
#[path = "🪪️decision/🦀️.rs"]
mod decision;
pub use decision::DurableDecisionIdentityV1;
#[path = "📐️budget/🦀️.rs"]
mod budget;
pub use budget::DurableJsonBudgetV1;

#[derive(Default)]
pub struct DurableCanonicalOutputV1 {
    pub(super) cursor: Option<Box<ArtifactCanonicalJsonCursor>>,
    pub(super) output: Option<Vec<u8>>,
    pub(super) length: usize,
    pub(super) phase: u8,
    pub(super) budget:DurableJsonBudgetV1,
}

pub fn permit(demand: RetirementDemand, grant: RetainedCloneGrant) -> bool { grant.maximum_items != 0 && demand.copy_bytes <= grant.maximum_copy_bytes && demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth }
pub fn copy(bytes: usize) -> RetirementDemand { RetirementDemand { copy_bytes: bytes, depth: 1, ..Default::default() } }

impl DurableCanonicalOutputV1 {
    pub fn phase(&self) -> &'static str { match self.phase { 0 => "admit", 1 => "measure", 2 => "allocate", 3 => "write", 4 => "ready", _ => "closed" } }
    pub fn next_demand(&self) -> RetirementDemand {
        match self.phase {
            0 => RetirementDemand { capacity_bytes:size_of::<ArtifactCanonicalJsonCursor>(),..copy(size_of::<ArtifactCanonicalJsonCursor>() + size_of::<Option<Box<ArtifactCanonicalJsonCursor>>>()) },
            1 | 3 if self.cursor.as_ref().is_some_and(|cursor| !cursor.is_complete()) => {
                let mut demand = self.cursor.as_ref().unwrap().next_encode_demand();
                demand.copy_bytes += self.cursor.as_ref().unwrap().next_encode_output_bound() + size_of::<usize>()+size_of::<DurableJsonBudgetV1>();
                demand
            }
            1 => copy(size_of::<ArtifactCanonicalJsonCursor>() + size_of::<u8>()),
            2 => RetirementDemand { capacity_bytes: self.length, ..copy(size_of::<Vec<u8>>() + size_of::<u8>()) },
            3 => RetirementDemand {release_bytes:size_of::<ArtifactCanonicalJsonCursor>(),..copy(size_of::<Option<Box<ArtifactCanonicalJsonCursor>>>() + size_of::<u8>())},
            _ => Default::default(),
        }
    }

    pub fn advance(&mut self, source: &(impl ArtifactCanonicalJson + ?Sized), maximum: usize, grant: RetainedCloneGrant, native: &mut NativeEncodeControl<'_>) -> Result<RetainedCloneProgress, ValueError> {
        let demand = self.next_demand();
        if self.phase == 4 || !permit(demand, grant) { return Ok(Default::default()); }
        native.checkpoint()?;
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        match self.phase {
            0 => { native.admit_turn_capacity(grant.maximum_capacity_bytes)?;native.charge(size_of::<ArtifactCanonicalJsonCursor>())?;self.cursor = Some(Box::new(ArtifactCanonicalJsonCursor::with_maximum_depth(DURABLE_OWNED_GROUP_JSON_MAX_DEPTH)?)); self.phase = 1;progress.retained_capacity_bytes=size_of::<ArtifactCanonicalJsonCursor>(); }
            1 | 3 if !self.cursor.as_ref().unwrap().is_complete() => {
                let child = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - size_of::<usize>() - self.cursor.as_ref().unwrap().next_encode_output_bound()-size_of::<DurableJsonBudgetV1>(), ..grant };
                let mut byte = [0];
                let step = self.cursor.as_mut().unwrap().encode_chunk_admitted(source, &mut byte, child).map_err(|error| error.reason)?;
                progress = step.ownership.progress();
                if step.written_bytes != 0 {
                    if self.phase==1{self.budget.advance_byte(byte[0]).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"durable canonical output exceeds declared JSON structure budget").with_retained_progress(progress))?;}
                    if self.phase == 1 { self.length = self.length.checked_add(1).filter(|length| *length <= maximum).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "durable canonical output exceeds its exact carrier limit").with_retained_progress(progress))?; }
                    else { let output = self.output.as_mut().unwrap(); if output.len() >= self.length { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "durable canonical source changed after measurement").with_retained_progress(progress)); } output.push(byte[0]); }
                    progress.copied_bytes += step.written_bytes + size_of::<usize>()+size_of::<DurableJsonBudgetV1>();
                }
            }
            1 => { self.budget.finish().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"durable canonical source ended outside its declared JSON structure"))?;**self.cursor.as_mut().unwrap()=ArtifactCanonicalJsonCursor::with_maximum_depth(DURABLE_OWNED_GROUP_JSON_MAX_DEPTH)?; self.phase = 2; }
            2 => {
                native.admit_turn_capacity(grant.maximum_capacity_bytes)?;
                self.output = Some(native.allocate_vec(self.length)?);
                self.phase = 3;
                progress.retained_capacity_bytes = self.length;
            }
            3 => {
                if self.output.as_ref().is_none_or(|output| output.len() != self.length) { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "durable canonical source shortened after measurement")); }
                self.cursor = None; self.phase = 4;progress.released_bytes=size_of::<ArtifactCanonicalJsonCursor>();
            }
            _ => return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "durable canonical planner has an invalid phase")),
        }
        if !progress.fits(grant) { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "durable canonical child exceeds original caller grant").with_retained_progress(progress)); }
        Ok(progress)
    }

    pub fn ready(&self) -> bool { self.phase == 4 && self.output.as_ref().is_some_and(|output| output.len() == self.length) }
    pub fn restart_empty(&mut self,grant:RetainedCloneGrant)->Option<RetainedCloneProgress>{let bytes=size_of::<usize>()+size_of::<u8>()+size_of::<DurableJsonBudgetV1>();if !self.terminal_is_empty()||!permit(copy(bytes),grant){return None;}self.length=0;self.phase=0;self.budget=Default::default();Some(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})}
    pub fn take(&mut self, grant: RetainedCloneGrant) -> Option<(Vec<u8>, RetainedCloneProgress)> {
        let bytes = size_of::<Option<Vec<u8>>>() + size_of::<u8>();
        if !self.ready() || !permit(copy(bytes), grant) { return None; }
        self.phase = 5;
        Some((self.output.take()?, RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }))
    }

    pub fn next_close_demand(&self) -> RetirementDemand { RetirementDemand { release_bytes: self.output.as_ref().map_or(0, Vec::capacity)+usize::from(self.cursor.is_some())*size_of::<ArtifactCanonicalJsonCursor>(), ..copy(size_of::<Option<Vec<u8>>>() + size_of::<Option<Box<ArtifactCanonicalJsonCursor>>>() + size_of::<u8>()) } }
    pub fn close(&mut self, grant: RetainedCloneGrant) -> RetainedCloneProgress {
        if self.terminal_is_empty() { return Default::default(); }
        let demand = self.next_close_demand();
        if !permit(demand, grant) { return Default::default(); }
        self.output = None; self.cursor = None; self.phase = 5;
        RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() }
    }
    pub fn terminal_is_empty(&self) -> bool { self.output.is_none() && self.cursor.is_none() && self.phase == 5 }
}

impl Drop for DurableCanonicalOutputV1 {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.output.is_none(), "durable canonical planning must hand off or retire its exact output before Drop"); }
}
