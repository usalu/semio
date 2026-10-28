//! 🔤️ Resident choices borrow immutable declaration words while their contiguous backing retires under the original owner.
use semio_framework_plugin::app::staged_form::StagedScalarChoice;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, retained_clone_grant_admits};
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, RetirementStep};
use semio_framework_value::{RetirementDemand, ValueError};
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
impl Drop for Choices {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.rows.is_none(), "declaration choices require original retirement");
    }
}
impl RetireOwned for Choices {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(self)
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
    fn controlled_retirement_supported() -> bool {
        true
    }
}
impl RetirementCursor for Choices {
    fn next_demand(&self, _: usize) -> Result<RetirementDemand, ValueError> {
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
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if self.rows.is_none() {
            return RetirementStep::Complete;
        }
        let demand = match self.next_demand(grant.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => return RetirementStep::Failure(error),
        };
        if !retained_clone_grant_admits(grant, demand) {
            return RetirementStep::BudgetExhausted;
        }
        if !self.rows.as_ref().unwrap().is_empty() {
            self.rows.as_mut().unwrap().pop();
        } else {
            drop(self.rows.take());
        }
        RetirementStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() })
    }
    fn terminal_is_empty(&self) -> bool {
        self.rows.is_none()
    }
    fn next_close_byte_demand(&self) -> Option<usize> {
        Some(self.rows.as_ref().filter(|rows| rows.is_empty()).map_or(0, |rows| rows.capacity() * size_of::<StagedScalarChoice>()))
    }
    fn next_work_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.next_demand(usize::MAX)?.copy_bytes)
    }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> {
        Some(0)
    }
    fn terminal_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[cfg(test)]
#[path = "type_input_choices_tests.rs"]
mod tests;
      