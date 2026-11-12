//! 🔤️ Resident choices borrow immutable declaration words while their contiguous backing retires under the original owner.
use semio_plugin_decl_staged_form::StagedScalarChoice;
use super::{RetainedWorkOutcome,idle_step,paid,refused,retire_self};
use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits};
use semio_framework_value::{RetainedAllocationEffect, RetirementDemand, ValueError};
use std::mem::{ManuallyDrop, size_of};
pub(super) struct Choices {
    rows: ManuallyDrop<Option<Vec<StagedScalarChoice<'static>>>>,
}
impl Choices {
    pub(super) fn new(rows: Vec<StagedScalarChoice<'static>>) -> Self {
        Self { rows: ManuallyDrop::new(Some(rows)) }
    }
    pub(super) fn rows(&self) -> &[StagedScalarChoice<'static>] {
        self.rows.as_deref().unwrap_or(&[])
    }
    pub(super) fn push(&mut self, key: &'static str, label: &'static str) {
        self.rows.as_mut().expect("original choice backing").push(StagedScalarChoice { key, label });
    }
}
impl semio_plugin_decl_staged_form::StagedScalarChoiceSource for Choices {
    fn len(&self)->usize{self.rows().len()}
    fn choice(&self,index:usize)->Option<StagedScalarChoice<'_>>{self.rows().get(index).copied()}
}
impl Drop for Choices {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.rows.is_none(), "declaration choices require original retirement");
    }
}
impl Choices {
    fn begin_close(&mut self) {}
    fn empty(&self) -> bool {
        self.rows.is_none()
    }
    fn close_demands(&self, _: usize) -> Result<RetirementDemand, ValueError> {
        Ok(match self.rows.as_ref() {
            Some(rows) if !rows.is_empty() => RetirementDemand { copy_bytes: 2 * size_of::<Self>() + size_of::<StagedScalarChoice>() + size_of::<usize>(), depth: 1, ..Default::default() },
            Some(rows) => RetirementDemand {
                copy_bytes: 2 * size_of::<Self>() + size_of::<Option<Vec<StagedScalarChoice>>>() + size_of::<Vec<StagedScalarChoice>>(),
                release_bytes: rows.capacity() * size_of::<StagedScalarChoice>(),
                depth: 1,
                ..Default::default()
            },
            None => Default::default(),
        })
    }
    fn close_owned(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome {
        if self.rows.is_none() {
            return idle_step(RetainedCloneStep::Complete(Default::default()));
        }
        let demand = match self.close_demands(body) {
            Ok(demand) => demand,
            Err(error) => return refused(error),
        };
        if !retained_clone_grant_admits(effect.grant(), demand) {
            return idle_step(RetainedCloneStep::Progress(Default::default()));
        }
        match paid(effect) {
            Ok(true) => {}
            Ok(false) => return idle_step(RetainedCloneStep::Progress(Default::default())),
            Err(error) => return refused(error),
        }
        if !self.rows.as_ref().unwrap().is_empty() {
            self.rows.as_mut().unwrap().pop();
        } else {
            drop(self.rows.take());
        }
        RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() })), work: effect.performed_work() }
    }
}
retire_self!([] Choices);

#[cfg(test)]
#[path = "../type_input_choices_tests.rs"]
mod tests;
      