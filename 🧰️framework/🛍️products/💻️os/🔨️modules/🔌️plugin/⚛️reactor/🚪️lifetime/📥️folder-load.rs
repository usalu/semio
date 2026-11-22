//! 🎟 Keeps one native folder reopen on the exact live instance until transfer or paid close.
use semio_framework::kernel::ActorInstanceLifetime;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, RetainedWorkPoll, ValueError, retirement::controlled::ControlledRetirement};
use semio_os_kernel::channel::archive_ingress::{
    DocumentArchiveIngressLimits, DocumentArchiveIngressSeal, DocumentArchiveParentPages,
    PagedDocumentArchiveIngress, VerifiedDocumentArchiveParent,
};
use semio_os_kernel::channel::archive_ingress::folder_archive_load::{
    FolderArchiveLoadAction as Action, FolderArchiveLoadKey, FolderArchiveLoadReply as Reply, FolderArchiveLoadRequest,
};

const LIMITS: DocumentArchiveIngressLimits = DocumentArchiveIngressLimits {
    archive_bytes: 8 * 1024 * 1024,
    pack_bytes: 8 * 1024 * 1024,
    spr_bytes: 4 * 1024 * 1024,
};

struct Intake {
    key: FolderArchiveLoadKey,
    seal: DocumentArchiveIngressSeal,
    source: PagedDocumentArchiveIngress,
    verified: Option<VerifiedDocumentArchiveParent>,
    retirement: Option<ControlledRetirement<DocumentArchiveParentPages>>,
    submitted: bool,
}

#[derive(Default)]
pub struct MountedFolderArchiveLoad {
    intake: Option<Box<Intake>>,
}

impl MountedFolderArchiveLoad {
    pub fn turn(
        &mut self,
        live: ActorInstanceLifetime,
        request: FolderArchiveLoadRequest<'_>,
        admitted_target: bool,
        submit: impl FnOnce(u64, &mut Option<VerifiedDocumentArchiveParent>, &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure>,
        cx: &mut StepContext<'_>,
    ) -> Result<Reply, OriginalTurnFailure> {
        if request.key.instance != live.instance_id { return Ok(Reply::Refused); }
        let scoped = FolderArchiveLoadKey {
            activation: live.activation_generation,
            lifetime: live.guest_lifetime,
            ..request.key
        };
        if let Action::Begin { seal, .. } = request.action {
            if !admitted_target { return Ok(Reply::Refused); }
            if let Some(intake) = self.intake.as_ref() {
                if intake.key != scoped || intake.seal != seal { return Ok(Reply::Refused); }
            } else {
                let copy = size_of::<Intake>() + size_of::<Option<Box<Intake>>>();
                let bytes = size_of::<Intake>();
                let charge = RetainedCloneProgress { copied_items: 1, copied_bytes: copy, retained_capacity_bytes: bytes, ..Default::default() };
                if cx.should_yield() || cx.fuel_remaining() == 0 || !funded(cx, charge) { return Ok(Reply::Pending); }
                let source = PagedDocumentArchiveIngress::new(seal, LIMITS)?;
                self.intake = Some(Box::new(Intake { key: scoped, seal, source, verified: None, retirement: None, submitted: false }));
                cx.consume_retained(charge)?;
                cx.consume_fuel(1);
            }
            return Ok(Reply::Admitted { activation: scoped.activation, lifetime: scoped.lifetime });
        }
        if request.key != scoped { return Ok(Reply::Refused); }
        let Some(intake) = self.intake.as_mut() else {
            return Ok(if matches!(request.action, Action::Cancel) { Reply::Closed } else { Reply::Refused });
        };
        if intake.key != request.key { return Ok(Reply::Refused); }

        match request.action {
            Action::Page { offset, bytes } => {
                let Some((turn, work)) = cx.with_original_effect(cx.retained_grant(), |_, effect| intake.source.step(offset as usize, bytes, effect))? else {
                    return Ok(Reply::Pending);
                };
                cx.consume_work(work);
                match turn.result {
                    Ok(progress) => cx.consume_retained(progress)?,
                    Err(error) => { cx.consume_retained(error.retained_progress())?; return Ok(Reply::Refused); }
                }
                Ok(if intake.source.accepted() > offset as usize {
                    Reply::Received { through: intake.source.accepted() as u32 }
                } else { Reply::Pending })
            }
            Action::Finish => {
                if intake.verified.is_some() { return Ok(Reply::Verified); }
                if !intake.source.ready() {
                    let Some((turn, work)) = cx.with_original_effect(cx.retained_grant(), |_, effect| intake.source.finish(effect))? else {
                        return Ok(Reply::Pending);
                    };
                    cx.consume_work(work);
                    match turn.result {
                        Ok(progress) => cx.consume_retained(progress)?,
                        Err(error) => { cx.consume_retained(error.retained_progress())?; return Ok(Reply::Refused); }
                    }
                    return Ok(Reply::Pending);
                }
                let Some((transfer, work)) = cx.with_original_effect(cx.retained_grant(), |_, effect| intake.source.take_verified(effect))? else {
                    return Ok(Reply::Pending);
                };
                cx.consume_work(work);
                if let Some((verified, progress)) = transfer? {
                    intake.verified = Some(verified);
                    cx.consume_retained(progress)?;
                    return Ok(Reply::Verified);
                }
                Ok(Reply::Pending)
            }
            Action::Publish => {
                let operation = intake.key.operation().ok_or_else(|| ValueError::invariant("folder archive lost its admitted operation"))?;
                if !intake.submitted {
                    if intake.verified.is_none() { return Ok(Reply::Refused); }
                    if !submit(operation, &mut intake.verified, cx)? { return Ok(Reply::Pending); }
                    if intake.verified.is_some() {
                        return Err(ValueError::invariant("folder archive recipient did not accept its original pages").into());
                    }
                    intake.submitted = true;
                }
                if self.release(cx)? { Ok(Reply::Submitted) } else { Ok(Reply::Pending) }
            }
            Action::Cancel => if self.cancel(cx)? { Ok(Reply::Closed) } else { Ok(Reply::Pending) },
            Action::Begin { .. } => unreachable!("begin is dispatched before the live exchange"),
        }
    }

    pub fn cancel(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let Some(intake) = self.intake.as_mut() else { return Ok(true); };
        if let Some(owner) = intake.retirement.as_mut() {
            if owner.terminal_is_empty() { return self.release(cx); }
            let Some((poll, work)) = cx.with_original_effect(cx.retained_grant(), |_, effect| owner.step(effect.grant().maximum_copy_bytes, effect))? else {
                return Ok(false);
            };
            cx.consume_work(work);
            if let RetainedWorkPoll::Complete(turn) = poll {
                match turn.result {
                    Ok(step) => cx.consume_retained(step.progress())?,
                    Err(error) => return Err(error.into()),
                }
            }
            return Ok(false);
        }
        if intake.verified.is_some() {
            let charge = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Intake>(), ..Default::default() };
            if cx.retirement_should_yield() || cx.fuel_remaining() == 0 || !funded(cx, charge) { return Ok(false); }
            intake.retirement = intake.verified.take().map(|owner| ControlledRetirement::retain(owner.into_pages()));
            cx.consume_retained(charge)?;
            cx.consume_fuel(1);
            return Ok(false);
        }
        if intake.submitted { return self.release(cx); }
        intake.source.cancel();
        let Some((transfer, work)) = cx.with_original_effect(cx.retained_grant(), |_, effect| intake.source.take_unverified(effect))? else {
            return Ok(false);
        };
        cx.consume_work(work);
        if let Some((pages, progress)) = transfer? {
            intake.retirement = Some(ControlledRetirement::retain(pages));
            cx.consume_retained(progress)?;
        }
        Ok(false)
    }

    fn release(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let charge = RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<Option<Box<Intake>>>(), released_bytes: size_of::<Intake>(), ..Default::default()
        };
        if cx.retirement_should_yield() || cx.fuel_remaining() == 0 || !funded(cx, charge) { return Ok(false); }
        cx.consume_retained(charge)?;
        cx.consume_fuel(1);
        self.intake.take();
        Ok(true)
    }
}

fn funded(cx: &StepContext<'_>, charge: RetainedCloneProgress) -> bool {
    let grant = cx.retained_grant();
    grant.maximum_depth != 0 && charge.fits(grant)
}
