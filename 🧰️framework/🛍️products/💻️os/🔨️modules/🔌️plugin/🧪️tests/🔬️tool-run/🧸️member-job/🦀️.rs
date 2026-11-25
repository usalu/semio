//! 🪆️ The toy member job: one unit appends `SetCount(base + unit)` and `SetLabel("unit-<unit>")` to the owned member, so the child-target run laws mount it as a plain fixture.
use super::*;

/// 🧸️ One unit appends `SetCount(base + unit)` and `SetLabel("unit-<unit>")` to the member, `base` being the member's count
/// in the composed read the job starts from; one unit per step, so progress spans driver turns and a single step is one unit.
pub struct ToyMemberJob {
    pub original:ToyPortJobCustody,
    pub base_count: i32,
    pub target: u32,
    pub done: u32,
    pub phase: u8,
    pub progress_source: Option<ToolRunProgress>,
    pub closing: bool,
}

/// 🏭️ The member job a request describes, continuing after the member ops the run already holds.
#[doc(hidden)]
pub fn toy_member_job(request: ToolRunJobRequest<'_, ToyRunApp>) -> Result<ToyMemberJob, Fault> {
    let member = &fixture()["member"];
    let base_count = request.children.typed_read::<TestSnapshot>(text(&member["slot"]), text(&member["childId"]))?.count;
    let done = (request.member_ops.len() / number(&fixture()["opsPerUnit"]) as usize) as u32;
    Ok(ToyMemberJob { original:ToyPortJobCustody::new(ToolRunTickWriter::with_provisional_base(request.identity,request.member_ops.len()as u32),request.port), base_count, target: toy_target(&request.config), done, phase: 0, progress_source: None, closing: false })
}

impl ToyMemberJob {
    fn run<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
        }
        let grant = cx.retained_grant();
        match self.phase {
            0 => {
                if self.done >= self.target {
                    return semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into);
                }
                let unit = self.done.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original Toy member unit overflow"))?;
                if self.original.unit.advance(&mut self.original.writer, self.base_count, unit, cx)? {
                    self.done = unit;
                    self.phase = 1;
                }
            }
            1 => {
                if ToyRunJob::metadata(cx)? {
                    let identity = self.original.writer.identity();
                    self.progress_source = Some(ToolRunProgress { identity, sequence: 0, state: ToolRunState::Running, stage: 0, completed: u64::from(self.done), total: Some(u64::from(self.target)), counters: Vec::new(), units_per_second: 0.0, conflicts: 0, steps: ToolRunStepRing::new() });
                    self.phase = 2;
                }
            }
            2 => {
                let result = self.original.writer.admit_original_progress(&mut self.progress_source, grant);
                if ToyRunJob::receive(cx, result)? {
                    self.phase = 3;
                }
            }
            3 => {
                let original = self.original.emission.advance_preview(&self.original.writer, cx)?;
                if original.is_some() {
                    self.phase = 4;
                }
                return Ok(original);
            }
            4 => {
                if self.original.emission.close_in_context(cx)? {
                    self.phase = 5;
                }
            }
            5 => {
                if self.original.reset_writer(cx)? {
                    self.phase = 0;
                }
            }
            _ => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original member phase is absent").into()),
        }
        Ok(None)
    }
}

impl semio_framework_job::InteractiveJob for ToyMemberJob {
    toy_job_step!();

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        self.original.emission.borrow_outcome(descriptor)
    }

    toy_port_job_close!();
}
