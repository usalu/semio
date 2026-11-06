//! 🧺️ Owned remodeling rows gain capacity through bounded header moves without another source authority.
use protocol::retained_mutation_demand_fits;
use semio_framework_value::retained_clone::{admit_retained_clone_progress, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retained_clone::allocation::backing::{allocate_exact_vector, exact_vector_bytes};
use semio_framework_value::{OriginalRetirementFailure, OriginalRetirementOutcome, RetainedWorkOutcome, RetireOwned, RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::{size_of, ManuallyDrop};

pub(crate) struct OwnedRowsBuffer<T: RetireOwned> {
    input: ManuallyDrop<Option<Vec<T>>>,
    output: ManuallyDrop<Option<Vec<T>>>,
    retirement: RetainedCloneClose,
    required: usize,
    reverse: usize,
    phase: u8,
    closing: bool,
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, "owned remodeling row capacity frontier changed")
}
fn scalar(copy_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
}

impl<T: RetireOwned> OwnedRowsBuffer<T> {
    pub(crate) fn new(input: Vec<T>, required: usize) -> Result<Self, (ValueError, Vec<T>)> {
        let required = required.max(input.len());
        if !Vec::<T>::controlled_retirement_supported() {
            return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner, "remodeling rows require typed retirement"), input));
        }
        if let Err(error) = exact_vector_bytes::<T>(required) {
            return Err((error, input));
        }
        Ok(Self { input: ManuallyDrop::new(Some(input)), output: ManuallyDrop::new(None), retirement: Default::default(), required, reverse: 0, phase: 0, closing: false })
    }

    pub(crate) fn demand(&self, body: usize, maximum_work_units: u64) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.phase >= 4 {
            return Err(invalid());
        }
        if maximum_work_units == 0 {
            return Ok(Default::default());
        }
        match self.phase {
            0 => {
                let input = self.input.as_ref().ok_or_else(invalid)?;
                Ok(RetirementDemand { capacity_bytes: if input.capacity() >= self.required { 0 } else { exact_vector_bytes::<T>(self.required)? }, ..scalar(2 * size_of::<Vec<T>>() + size_of::<u8>()) })
            }
            1 => Ok(scalar(if self.input.as_ref().ok_or_else(invalid)?.is_empty() { size_of::<usize>() + size_of::<u8>() } else { 2 * size_of::<T>() + 2 * size_of::<usize>() })),
            2 => Ok(scalar(if self.reverse < self.output.as_ref().ok_or_else(invalid)?.len() / 2 { 3 * size_of::<T>() + size_of::<usize>() } else { size_of::<u8>() })),
            3 => {
                if self.retirement.is_empty() && self.input.is_none() {
                    Ok(scalar(size_of::<u8>()))
                } else {
                    self.retirement.next_owner_demands::<Vec<T>>(self.input.is_some(), &None, body)
                }
            }
            _ => Err(invalid()),
        }
    }

    pub(crate) fn work_demand(&self, body: usize, maximum_work_units: u64) -> Result<u64, ValueError> {
        self.demand(body, maximum_work_units)?;
        if maximum_work_units == 0 {
            return Ok(0);
        }
        let work = if self.phase == 3 && (!self.retirement.is_empty() || self.input.is_some()) { self.retirement.next_owner_work::<Vec<T>>(self.input.is_some(), &None, body)? } else { 1 };
        Ok(if work <= maximum_work_units { work } else { 0 })
    }

    pub(crate) fn advance(&mut self, body: usize, maximum_work_units: u64, grant: RetainedCloneGrant) -> OriginalRetirementOutcome {
        let mut work = 0;
        let result: Result<_, OriginalRetirementFailure> = (|| {
            let selected = self.work_demand(body, maximum_work_units)?;
            if selected == 0 {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            let demand = self.demand(body, maximum_work_units)?;
            if !retained_mutation_demand_fits(demand, grant) {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            if self.phase == 3 && (!self.retirement.is_empty() || self.input.is_some()) {
                let turn = if self.retirement.is_empty() {
                    let turn = self.retirement.begin_granted(&mut self.input, grant);
                    RetainedWorkOutcome { result: turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into), work: turn.work }
                } else {
                    self.retirement.step_granted(grant)
                };
                work = turn.work;
                return turn.result.map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            work = 1;
            let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
            match self.phase {
                0 => {
                    if self.input.as_ref().ok_or_else(invalid)?.capacity() >= self.required {
                        *self.output = self.input.take();
                        self.phase = 4;
                    } else {
                        let output = allocate_exact_vector::<T>(self.required).map_err(|error| error.with_retained_progress(progress))?;
                        progress.retained_capacity_bytes = output.capacity().checked_mul(size_of::<T>()).ok_or_else(invalid)?;
                        *self.output = Some(output);
                        self.phase = 1;
                    }
                }
                1 => {
                    if self.input.as_ref().ok_or_else(invalid)?.is_empty() {
                        self.reverse = 0;
                        self.phase = 2;
                    } else {
                        let output = self.output.as_mut().ok_or_else(invalid)?;
                        if output.len() == output.capacity() {
                            return Err(invalid().with_retained_progress(progress).into());
                        }
                        output.push(self.input.as_mut().ok_or_else(invalid)?.pop().expect("quoted owned row"));
                    }
                }
                2 => {
                    let output = self.output.as_mut().ok_or_else(invalid)?;
                    if self.reverse < output.len() / 2 {
                        let end = output.len() - self.reverse - 1;
                        output.swap(self.reverse, end);
                        self.reverse += 1;
                    } else {
                        self.phase = 3;
                    }
                }
                3 => {
                    self.phase = 4;
                }
                _ => return Err(invalid().into()),
            }
            let progress = admit_retained_clone_progress(grant, progress, "owned remodeling row capacity")?;
            Ok(if self.ready() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
        })();
        RetainedWorkOutcome { result, work }
    }

    pub(crate) fn ready(&self) -> bool {
        !self.closing && self.phase == 4 && self.output.is_some()
    }

    pub(crate) fn take_demand(&self) -> Result<RetirementDemand, ValueError> {
        if !self.ready() {
            return Err(invalid());
        }
        Ok(scalar(2 * size_of::<Vec<T>>() + size_of::<u8>()))
    }

    pub(crate) fn take(&mut self, grant: RetainedCloneGrant) -> RetainedWorkOutcome<Option<(Vec<T>, RetainedCloneProgress)>, ValueError> {
        let demand = match self.take_demand() {
            Ok(demand) => demand,
            Err(error) => return RetainedWorkOutcome { result: Err(error), work: 0 },
        };
        if !retained_mutation_demand_fits(demand, grant) {
            return RetainedWorkOutcome { result: Ok(None), work: 0 };
        }
        self.phase = 5;
        RetainedWorkOutcome { result: Ok(Some((self.output.take().expect("quoted original row output"), RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))), work: 1 }
    }

    pub(crate) fn begin_close(&mut self) {
        self.closing = true;
    }

    pub(crate) fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.closing {
            return Err(invalid());
        }
        if !self.retirement.is_empty() {
            return self.retirement.demands(body);
        }
        self.retirement.next_owner_demands::<Vec<T>>(self.input.is_some() || self.output.is_some(), &None, body)
    }

    pub(crate) fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        self.close_demand(body)?;
        self.retirement.next_owner_work::<Vec<T>>(self.input.is_some() || self.output.is_some(), &None, body)
    }

    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> OriginalRetirementOutcome {
        let demand = match self.close_demand(grant.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => return RetainedWorkOutcome { result: Err(error.into()), work: 0 },
        };
        if !retained_mutation_demand_fits(demand, grant) {
            return RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work: 0 };
        }
        let turn = if !self.retirement.is_empty() {
            self.retirement.step_granted(grant)
        } else {
            let slot = if self.input.is_some() { &mut self.input } else { &mut self.output };
            let turn = self.retirement.begin_granted(slot, grant);
            RetainedWorkOutcome { result: turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Complete(Default::default()))).map_err(Into::into), work: turn.work }
        };
        RetainedWorkOutcome { result: turn.result.map(|step| if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) }), work: turn.work }
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.closing && self.input.is_none() && self.output.is_none() && self.retirement.is_empty()
    }
}

impl<T: RetireOwned> Drop for OwnedRowsBuffer<T> {
    fn drop(&mut self) {
        let empty = self.input.is_none() && self.output.is_none() && self.retirement.is_empty();
        assert!(std::thread::panicking() || empty, "owned remodeling row buffer requires funded closure");
        if empty {
            unsafe {
                ManuallyDrop::drop(&mut self.input);
                ManuallyDrop::drop(&mut self.output);
            }
        }
    }
}

#[cfg(test)]
#[path = "🧪️.rs"]
mod tests;
