//! 📦️ Append storage used by a typed sequence receiver.
use super::*;
use semio_framework_value_persistent::list::{PagedList, PagedListError, PagedListRefusalKind, PagedListReservationFailure, PagedListReservationOutcome};
use std::mem::size_of;

pub trait SequenceStorage<T: RetireOwned>: RetireOwned + Sized {
    fn birth_demand(length: usize) -> Result<RetirementDemand, ValueError>;
    fn birth(length: usize, actual: &mut Option<usize>) -> Result<(Self, usize), ValueError>;
    fn append_quote(&self, length: usize) -> Result<(RetirementDemand, u64), ValueError>;
    fn append(&mut self, input: &mut Option<T>, length: usize, effect: &mut RetainedAllocationEffect<'_>) -> OwnedPoll;
}

impl<T: RetireOwned> SequenceStorage<T> for Vec<T> {
    fn birth_demand(length: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { copy_bytes: size_of::<Self>(), capacity_bytes: length.checked_mul(size_of::<T>()).ok_or_else(invariant)?, depth: 1, ..Default::default() })
    }

    fn birth(length: usize, actual: &mut Option<usize>) -> Result<(Self, usize), ValueError> {
        let output = allocate_empty_vec::<T>(length, actual)?;
        let capacity = output.capacity() * size_of::<T>();
        Ok((output, capacity))
    }

    fn append_quote(&self, _: usize) -> Result<(RetirementDemand, u64), ValueError> {
        Ok((payload::<T>(), 1))
    }

    fn append(&mut self, input: &mut Option<T>, _: usize, effect: &mut RetainedAllocationEffect<'_>) -> OwnedPoll {
        local(effect, payload::<T>(), || {
            if self.len() == self.capacity() { return Err(invariant()); }
            self.push(input.take().ok_or_else(invariant)?);
            Ok(())
        })
    }
}

fn list_refusal(error: PagedListError) -> ValueError {
    let kind = match error.kind {
        PagedListRefusalKind::OwnershipLimit => ValueRefusalKind::OwnershipLimit,
        PagedListRefusalKind::AllocationFailed => ValueRefusalKind::AllocationFailed,
        PagedListRefusalKind::InvariantViolated => ValueRefusalKind::InvariantViolated,
    };
    ValueError::literal(kind, error.reason)
}

impl<T: RetireOwned, const N: usize> SequenceStorage<T> for PagedList<T, N> {
    fn birth_demand(length: usize) -> Result<RetirementDemand, ValueError> {
        if length > N { return Err(invalid()); }
        Ok(payload::<Self>())
    }

    fn birth(_: usize, _: &mut Option<usize>) -> Result<(Self, usize), ValueError> {
        Ok((Self::new(), 0))
    }

    fn append_quote(&self, length: usize) -> Result<(RetirementDemand, u64), ValueError> {
        if self.has_reserved_slot() {
            return Ok((self.push_reserved_source_demand().map_err(list_refusal)?, self.push_reserved_source_work_demand().map_err(list_refusal)?));
        }
        let copy_bytes = self.next_capacity_copy_byte_demand(length).map_err(list_refusal)?;
        let capacity_bytes = self.next_capacity_allocation_bytes(length).map_err(list_refusal)?.ok_or_else(invariant)?;
        let depth = self.next_capacity_depth_demand(length).map_err(list_refusal)?;
        let work = self.reserve_capacity_one_work_demand(length).map_err(list_refusal)?;
        Ok((RetirementDemand { copy_bytes, capacity_bytes, depth, ..Default::default() }, work))
    }

    fn append(&mut self, input: &mut Option<T>, length: usize, effect: &mut RetainedAllocationEffect<'_>) -> OwnedPoll {
        let (ask, work) = match self.append_quote(length) {
            Ok(quote) => quote,
            Err(error) => return done(effect, Err(error.into())),
        };
        let remaining = match effect.remaining_work_units() {
            Ok(remaining) => remaining,
            Err(error) => return done(effect, Err(error.into())),
        };
        if !admits(effect, ask) || remaining < work { return idle(effect); }
        if self.has_reserved_slot() {
            let result = effect.performed(work).and_then(|accepted| {
                if !accepted { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "owned sequence append exceeded original work")); }
                let step = self.push_reserved_source_step(input, work, effect.grant());
                step.result.map(|step| step.progress())
            }).map_err(Into::into);
            return done(effect, result);
        }
        let turn = self.reserve_capacity_one_funded(length, effect);
        let result = match turn.result {
            Ok(PagedListReservationOutcome::Denied) => Ok(Default::default()),
            Ok(PagedListReservationOutcome::Ready(progress)) => Ok(progress),
            Err(PagedListReservationFailure::Work(error)) => Err(error.into()),
            Err(PagedListReservationFailure::Allocation { error, copied_bytes }) => Err(list_refusal(error.refusal())
                .with_retained_progress(RetainedCloneProgress { copied_bytes, retained_capacity_bytes: error.allocated_bytes, ..Default::default() }).into()),
        };
        done(effect, result)
    }
}
