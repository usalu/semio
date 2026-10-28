//! ➖️ Original entity removal receives an exact unique source identity before central publication.
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;
#[derive(Default)]
pub(super) struct EntityRemoval {
    binding: Option<(usize, usize, usize, usize)>,
    id: u64,
    row: usize,
    matches: usize,
    phase: u8,
    closing: bool,
}
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
impl EntityRemoval {
    #[cfg(test)]
    pub(super) fn metadata(&self) -> (Option<(usize, usize, usize, usize)>, u64, usize, usize, u8, bool) {
        (self.binding, self.id, self.row, self.matches, self.phase, self.closing)
    }
    pub(super) fn address(event: &editing::SnapshotEditEvent, address: &pointer::PathAdmission) -> Option<usize> {
        let editing::SnapshotEditEvent::RemoveValue { path } = event else {
            return None;
        };
        (address.field_count() == 2 && address.field_is(path, 0, "entities")).then(|| address.field_index(path, 1)).flatten()
    }
    pub(super) fn demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: 2 * size_of::<Self>() + 2 * size_of::<IfcMutation>() + size_of::<Option<Vec<IfcMutation>>>(), capacity_bytes: if self.phase == 2 { size_of::<IfcMutation>() } else { 0 }, depth: 1, ..Default::default() }
    }
    pub(super) fn advance(
        &mut self,
        event: &editing::SnapshotEditEvent,
        snapshot: &IfcSnapshot,
        address: &pointer::PathAdmission,
        pending: &mut Option<Vec<IfcMutation>>,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC original entity removal cancelled").into());
        }
        let binding = (event as *const _ as usize, snapshot as *const _ as usize, address as *const _ as usize, pending as *const _ as usize);
        if self.closing || self.binding.is_some_and(|original| original != binding) {
            return Err(invalid("IFC entity removal original owner changed or closed").into());
        }
        let demand = self.demands();
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
            return Ok(false);
        }
        if self.phase == 3 {
            return Ok(true);
        }
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        match self.phase {
            0 => {
                let position = Self::address(event, address).ok_or_else(|| invalid("IFC entity removal requires its complete source ordinal"))?;
                let entity = snapshot.entities.get(position).ok_or_else(|| invalid("IFC entity removal source does not exist"))?;
                self.id = entity.id;
                self.binding = Some(binding);
                self.phase = 1;
            }
            1 => {
                if let Some(entity) = snapshot.entities.get(self.row) {
                    self.matches += usize::from(entity.id == self.id);
                    self.row += 1;
                } else if self.matches != 1 {
                    return Err(invalid("IFC entity removal requires a unique source identity").into());
                } else {
                    self.phase = 2;
                }
            }
            _ => {
                if pending.is_some() {
                    return Err(invalid("IFC entity removal primitive receiver is occupied").into());
                }
                let mut mutations = Vec::new();
                mutations.try_reserve_exact(1).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC entity removal primitive allocation failed"))?;
                mutations.push(IfcMutation::RemoveEntity(semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::remove_entity::RemoveEntity { id: self.id }));
                progress.retained_capacity_bytes = mutations.capacity() * size_of::<IfcMutation>();
                *pending = Some(mutations);
                self.phase = 3;
            }
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(self.phase == 3)
    }
    pub(super) fn begin_close(&mut self) {
        self.closing = true;
    }
    pub(super) fn terminal_is_empty(&self) -> bool {
        self.binding.is_none()
    }
    pub(super) fn close_demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: size_of::<Option<(usize, usize, usize, usize)>>(), depth: 1, ..Default::default() }
    }
    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Err(invalid("IFC entity removal close was not started"));
        }
        let demand = self.close_demands();
        if !retained_clone_grant_admits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        self.binding = None;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }
}
