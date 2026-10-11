//! 🧬️ Generic structural edit for any document lane: the `Mutation<S>` contract (inverse rows, diff, diff application) driven through the paged one-item preparation.
//!
//! The snapshot is cloned page by page under the grants by the preparation. `Mutation::apply_diff` is whole-snapshot, so the apply turn is one snapshot-sized turn quoted from the exact retained capacity of that clone; the displaced clone is then retired through controlled custody in separately granted release turns.
use super::{PagedOneItemEdit, PagedOneItemEditStep, PagedOneItemPreparationFactory};
use crate::os_spr::{apply_diff, MutationDiff, OpBinary};
use crate::snapshot_clone_preparation::RetainedEditCustody;
use crate::{ArtifactCanonicalJsonTree, ArtifactStoreOneItemPreparationFactory, Mutation};
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{controlled::ControlledRetirement, RetireOwned};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::{mem::size_of, sync::Arc};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage {
    Inverse,
    Apply,
    Retire,
    Done,
}

/// 🧬️ Bounded turns: inverse rows, snapshot-sized apply, granted retirement of the displaced clone, hand-over of the rows.
pub struct MutationApplyEdit<S: RetainedClone, M: RetireOwned + Send + 'static> {
    stage: Stage,
    clone_bytes: usize,
    displaced: Option<ControlledRetirement<S>>,
    custody: RetainedEditCustody<(Option<Vec<M>>, ())>,
}

impl<S: RetainedClone, M: RetireOwned + Send + 'static> Default for MutationApplyEdit<S, M> {
    fn default() -> Self {
        Self { stage: Stage::Inverse, clone_bytes: 0, displaced: None, custody: RetainedEditCustody::new((None, ())) }
    }
}

fn displaced_demand<S: RetainedClone>(owner: &ControlledRetirement<S>, body: usize) -> Result<RetirementDemand, ValueError> {
    if owner.terminal_is_empty() {
        return Ok(RetirementDemand { copy_bytes: size_of::<Option<ControlledRetirement<S>>>(), depth: 1, ..Default::default() });
    }
    Ok(RetirementDemand {
        copy_bytes: owner.next_copy_byte_demand()?,
        capacity_bytes: owner.next_capacity_byte_demand(body)?,
        release_bytes: owner.next_release_byte_demand()?,
        depth: owner.next_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "displaced snapshot retirement depth overflow"))?,
    })
}

fn step_displaced<S: RetainedClone>(slot: &mut Option<ControlledRetirement<S>>, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
    let owner = slot.as_mut().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "displaced snapshot retirement is absent"))?;
    let demand = displaced_demand(owner, grant.maximum_copy_bytes)?;
    if grant.maximum_depth < demand.depth {
        return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "displaced snapshot retirement exceeds admitted depth"));
    }
    if grant.maximum_items == 0 || demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes {
        return Ok(Default::default());
    }
    if owner.terminal_is_empty() {
        *slot = None;
        return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() });
    }
    Ok(owner.step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })?.progress())
}

impl<S, M> PagedOneItemEdit<S, M> for MutationApplyEdit<S, M>
where
    S: RetainedClone,
    M: Mutation<S> + OpBinary + RetireOwned + Send + Sync + 'static,
{
    const PREFIX: &'static str = "mutation-apply";

    fn recognizes(_mutation: &M) -> bool {
        true
    }

    fn preflight(mutation: &M) -> Result<usize, String> {
        OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "mutation-apply-encode-failed".to_owned())
    }

    fn advance(&mut self, post: &mut S, mutation: &M, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<M>, ValueError> {
        let none = || Ok(PagedOneItemEditStep::Progress(Default::default()));
        if grant.maximum_items == 0 || grant.maximum_depth < 2 {
            return none();
        }
        match self.stage {
            Stage::Inverse => {
                let bound = mutation.inverse_rows();
                let copy = size_of::<Vec<M>>();
                let capacity = bound.checked_mul(size_of::<M>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "inverse row layout overflow"))?;
                if grant.maximum_copy_bytes < copy || grant.maximum_capacity_bytes < capacity {
                    return none();
                }
                let rows = Mutation::inverse(mutation, &*post)?;
                let actual = rows.capacity().saturating_mul(size_of::<M>());
                let admitted = rows.len() <= bound && actual <= grant.maximum_capacity_bytes;
                self.custody.original_mut().0 = Some(rows);
                if !admitted {
                    return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "mutation inverse exceeded its declared rows or admitted capacity"));
                }
                self.stage = Stage::Apply;
                Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, retained_capacity_bytes: actual, released_bytes: 0 }))
            }
            Stage::Apply => {
                let copy = size_of::<S>() * 2;
                if grant.maximum_copy_bytes < copy || grant.maximum_capacity_bytes < self.clone_bytes {
                    return none();
                }
                let diff = Mutation::diff(mutation, &*post).into_parts().0;
                let applied = apply_diff(&diff, &*post);
                MutationDiff::retire_cold(diff);
                let next = applied.map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, format!("mutation-apply refused its diff: {}", error.message)))?;
                match ControlledRetirement::new(std::mem::replace(post, next)) {
                    Ok(owner) => self.displaced = Some(owner),
                    Err((error, previous)) => {
                        let next = std::mem::replace(post, previous);
                        <M::Diff as MutationDiff<S>>::retire_projection(next);
                        return Err(error);
                    }
                }
                self.stage = Stage::Retire;
                Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, retained_capacity_bytes: self.clone_bytes, released_bytes: 0 }))
            }
            Stage::Retire => {
                let progress = step_displaced(&mut self.displaced, grant)?;
                if self.displaced.is_some() || progress == RetainedCloneProgress::default() {
                    return Ok(PagedOneItemEditStep::Progress(progress));
                }
                let rows = self.custody.original_mut().0.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "mutation inverse rows are absent"))?;
                self.stage = Stage::Done;
                Ok(PagedOneItemEditStep::CompleteRows(progress, rows))
            }
            Stage::Done => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "mutation-apply edit already completed")),
        }
    }

    fn cloned(&mut self, capacity_bytes: usize) {
        self.clone_bytes = capacity_bytes;
    }

    fn begin_close(&mut self) {
        self.custody.begin_close();
    }

    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        match self.displaced.as_ref() {
            Some(owner) => displaced_demand(owner, body),
            None => self.custody.demands(body),
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.displaced.is_some() {
            return step_displaced(&mut self.displaced, grant).map(RetainedCloneStep::Progress);
        }
        self.custody.step(grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.displaced.is_none() && self.custody.terminal_is_empty()
    }
}

impl<S: RetainedClone, M: RetireOwned + Send + 'static> Drop for MutationApplyEdit<S, M> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.displaced.is_none() && self.custody.terminal_is_empty()), "mutation-apply edit requires complete admitted closure");
    }
}

/// 🏭️ Builds the paged one-item preparation factory of any document lane from its `Mutation<S>` contract.
pub fn mutation_apply_preparation_factory<S, M>() -> Arc<dyn ArtifactStoreOneItemPreparationFactory<S, M>>
where
    S: RetainedClone,
    M: Mutation<S> + OpBinary + ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
{
    Arc::new(PagedOneItemPreparationFactory::<S, M, MutationApplyEdit<S, M>>::default())
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
