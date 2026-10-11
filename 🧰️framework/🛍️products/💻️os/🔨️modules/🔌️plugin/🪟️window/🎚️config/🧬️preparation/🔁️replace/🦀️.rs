//! 🔁️ Whole-record replace edit for fixed-size window configs: copies the replacement into the post record, boxes the previous record as the inverse row, and retires both through original custody.
use super::preparation_custody::WindowEditCustody;
use crate::store;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneBirthDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep}};
use semio_framework_value::retirement::RetireOwned;
use std::{marker::PhantomData,mem::size_of};
use store::snapshot_clone_preparation::{RetainedCloneEdit,RetainedCloneEditCursor,RetainedCloneEditStep};

/// 🪟️ Declares the one replace mutation of a fixed-size window config record.
pub trait WindowConfigReplaceMutation<S>: Send + Sync + 'static {
    fn replacement(&self) -> &S;
    fn restoring(previous: Box<S>) -> Self;
    fn admissible(&self) -> bool { true }
}

/// 🔁️ Generic whole-record replace edit installed through `WindowConfigOwner::build_retained_edit`.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct WindowConfigReplaceEdit<S: 'static, M: 'static>(PhantomData<fn() -> (S, M)>);
impl<S: 'static, M: 'static> WindowConfigReplaceEdit<S, M> { pub fn new() -> Self { Self(PhantomData) } }
impl<S: 'static, M: 'static> Default for WindowConfigReplaceEdit<S, M> { fn default() -> Self { Self::new() } }

/// 🔁️ Four bounded turns: copy replacement, box previous record, reserve inverse row, push inverse row.
pub struct WindowConfigReplaceCursor<S: Copy + RetireOwned + 'static, M: RetireOwned + 'static> {
    stage: u8,
    cancelled: bool,
    custody: WindowEditCustody<(Option<Box<S>>, Option<Vec<M>>)>,
}

impl<S, M> RetainedCloneEdit<S, M> for WindowConfigReplaceEdit<S, M>
where S: RetainedClone + Copy + RetireOwned + Send + Sync + 'static, M: WindowConfigReplaceMutation<S> + RetireOwned {
    type Cursor = WindowConfigReplaceCursor<S, M>;
    fn preflight(&self, mutation: &M, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || !mutation.admissible() { return Err("window config replace mutation has invalid lane or record authority".into()); }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: store::snapshot_clone_preparation::CONFIG_EDIT_RETAINED_BASELINE_BYTES })
    }
    fn begin_demand(&self) -> RetainedCloneBirthDemand { RetainedCloneBirthDemand { capacity_bytes: 0, depth: 1 } }
    fn snapshot_cursor_birth_demand(&self) -> RetainedCloneBirthDemand { RetainedCloneBirthDemand { capacity_bytes: 0, depth: 1 } }
    fn begin(&self, grant: RetainedCloneGrant) -> Result<(Self::Cursor, RetainedCloneProgress), ValueError> {
        let mut receipt = self.begin_demand().admit(grant)?;
        receipt.copied_bytes = size_of::<Self::Cursor>();
        if !receipt.fits(grant) { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "window config replace cursor transfer exceeds grant")); }
        Ok((WindowConfigReplaceCursor { stage: 0, cancelled: false, custody: WindowEditCustody::new((None, None)) }, receipt))
    }
}

impl<S: Copy + RetireOwned + 'static, M: RetireOwned + 'static> WindowConfigReplaceCursor<S, M> {
    fn normal_demand(&self) -> RetirementDemand {
        let (copy_bytes, capacity_bytes) = match self.stage {
            0 => (size_of::<S>() * 2, 0),
            1 => (size_of::<S>() + size_of::<Box<S>>(), size_of::<S>()),
            2 => (size_of::<Vec<M>>(), size_of::<M>()),
            3 => (size_of::<M>() * 2 + size_of::<Box<S>>(), 0),
            _ => (0, 0),
        };
        RetirementDemand { copy_bytes, capacity_bytes, depth: 2, ..Default::default() }
    }
}

impl<S, M> RetainedCloneEditCursor<S, M> for WindowConfigReplaceCursor<S, M>
where S: RetainedClone + Copy + RetireOwned + Send + Sync + 'static, M: WindowConfigReplaceMutation<S> + RetireOwned {
    fn advance(&mut self, base: RetainedCloneRef<'_, S>, post: &mut S, mutation: RetainedCloneRef<'_, M>, grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> {
        if self.cancelled || self.custody.is_closing() || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
        if self.stage == 4 { return Ok(RetainedCloneEditStep::Complete(Default::default())); }
        let demand = self.normal_demand();
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.depth > grant.maximum_depth { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
        let refusal = |message: &'static str| ValueError::literal(ValueRefusalKind::InvariantViolated, message);
        match self.stage {
            0 => { *post = *mutation.get().replacement(); }
            1 => self.custody.original_mut().0 = Some(Box::new(*base.get())),
            2 => {
                let mut rows = Vec::new();
                rows.try_reserve_exact(1).map_err(|_| ValueError::literal(ValueRefusalKind::OwnershipLimit, "window config inverse row allocation failed"))?;
                self.custody.original_mut().1 = Some(rows);
            }
            _ => {
                let fields = self.custody.original_mut();
                let previous = fields.0.take().ok_or_else(|| refusal("window config inverse record absent"))?;
                fields.1.as_mut().ok_or_else(|| refusal("window config inverse rows absent"))?.push(M::restoring(previous));
            }
        }
        self.stage += 1;
        let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, released_bytes: 0 };
        Ok(if self.stage == 4 { RetainedCloneEditStep::Complete(receipt) } else { RetainedCloneEditStep::Progress(receipt) })
    }
    fn take_inverse(&mut self) -> Option<Vec<M>> { (self.stage == 4).then(|| self.custody.original_mut().1.take()).flatten() }
    fn foreign_step_presence(&self) -> bool { false }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) -> bool { self.custody.begin_close() }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.custody.step(grant) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.custody.demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.custody.demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.custody.demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.custody.demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.custody.terminal_is_empty() }
}
