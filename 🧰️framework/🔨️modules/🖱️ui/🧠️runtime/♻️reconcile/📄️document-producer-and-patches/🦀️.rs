//! 📄️ Document producer and patches: the job step, the document fault, the document producer and outcome, and the ready, published and acknowledged patches. See [`super`].

use super::*;

//#region ⏭️ResumableReconcile
//#region 🎟️RetainedAuthority
/// 🚦️ One admitted reconciliation opportunity result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceReconcileJobStep {
    MoreWork,
    Ready,
    Fault,
}

/// 📄️ Why a retained renderer-document producer stopped before publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceDocumentFault {
    StaleGeneration { expected: u64, actual: u64 },
    StaleRevision { expected: ui_contract::UiRevision, actual: ui_contract::UiRevision },
    Cancelled,
    Build(ui_contract::UiDocumentBuildError),
}

/// ⏭️ One bounded renderer-document production opportunity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceDocumentProducerStep {
    MoreWork,
    Ready,
    Fault,
}

/// 🧵️ Generation-qualified producer which admits one retained node page per opportunity.
pub struct SurfaceDocumentProducer {
    pub(super) generation: u64,
    pub(super) revision: ui_contract::UiRevision,
    lease: Option<ui_contract::UiDocumentLease>,
    pub(super) ready: bool,
    pub(super) fault: Option<SurfaceDocumentFault>,
}

impl SurfaceDocumentProducer {
    pub fn try_new(current: &SurfaceReconciler, generation: u64) -> Result<Self, ui_contract::UiDocumentLeaseError> {
        if generation == 0 { return Err(ui_contract::UiDocumentLeaseError::StaleHandle); }
        let mut lease = None;
        if !current.capture_document(&mut lease, SURFACE_RECONCILE_PAGE_BYTES)? { return Err(ui_contract::UiDocumentLeaseError::StaleHandle); }
        Ok(Self { generation, revision: current.revision(), lease, ready: false, fault: None })
    }

    pub fn generation(&self) -> u64 { self.generation }
    pub fn fault(&self) -> Option<&SurfaceDocumentFault> { self.fault.as_ref() }

    pub fn drive_one(&mut self, current: &SurfaceReconciler, cx: &mut semio_framework_job::StepContext<'_>) -> SurfaceDocumentProducerStep {
        if self.fault.is_some() { return SurfaceDocumentProducerStep::Fault; }
        if cx.generation().0 != self.generation {
            self.fault = Some(SurfaceDocumentFault::StaleGeneration { expected: self.generation, actual: cx.generation().0 });
            return SurfaceDocumentProducerStep::Fault;
        }
        if cx.is_cancelled() { self.fault = Some(SurfaceDocumentFault::Cancelled); return SurfaceDocumentProducerStep::Fault; }
        if cx.should_yield() { return SurfaceDocumentProducerStep::MoreWork; }
        if !current.shadow.document_root_matches(self.lease.as_ref()) {
            self.fault = Some(SurfaceDocumentFault::StaleRevision { expected: self.revision, actual: current.revision() });
            return SurfaceDocumentProducerStep::Fault;
        }
        self.ready = true;
        cx.consume_fuel(1);
        SurfaceDocumentProducerStep::Ready
    }

    pub fn take_ready(mut self) -> Result<SurfaceDocumentOutcome, Self> {
        if !self.ready || self.fault.is_some() { return Err(self); }
        let Some(lease) = self.lease.take() else { return Err(self); };
        Ok(SurfaceDocumentOutcome { generation: self.generation, lease })
    }

    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
        let Some(lease) = self.lease.as_mut() else { return Ok(ui_contract::UiValueRetirementStep { complete: true, ..Default::default() }); };
        let progress = lease.close_read_step_with_grant(items, bytes)?;
        if progress.complete { self.lease = None; }
        Ok(progress)
    }
}

/// 📬️ Complete generation-qualified fixed document transferred to a renderer consumer.
pub struct SurfaceDocumentOutcome {
    pub(super) generation: u64,
    lease: ui_contract::UiDocumentLease,
}

impl SurfaceDocumentOutcome {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn header(&self) -> Result<ui_contract::UiDocumentLeaseHeader, ui_contract::UiDocumentLeaseError> {
        self.lease.header()
    }

    pub fn try_read(&self) -> Result<ui_contract::UiDocumentRead<'_>, ui_contract::UiDocumentLeaseError> {
        self.lease.try_read()
    }

    pub fn try_alias(&self) -> Result<Self, ui_contract::UiDocumentLeaseError> {
        let mut alias = None;
        if !self.lease.try_alias_into(&mut alias, SURFACE_RECONCILE_PAGE_BYTES)? { return Err(ui_contract::UiDocumentLeaseError::AliasCapacity); }
        Ok(Self { generation: self.generation, lease: alias.unwrap() })
    }

    pub fn into_lease(self) -> ui_contract::UiDocumentLease {
        self.lease
    }

    pub fn close_step(&mut self) -> bool {
        self.lease.close_read_step_with_grant(1, SURFACE_COMPONENT_COPY_WORK_BYTES).expect("document outcome retains exact retirement authority").complete
    }
}

/// 📨️ Generation-qualified patch owner carrying its share of the live reconciliation credit.
pub struct SurfaceReconcileReadyPatch {
    pub(super) generation: u64,
    pub(super) patch: ui_contract::UiPendingPatch,
    pub(super) credit: Option<ui_contract::UiResidentPermit>,
    pub(super) handback: Option<SurfaceReconcileHandbackReservation>,
}

pub(super) fn pending_surface_patch(patch: Option<ui_contract::UiPatch>) -> ui_contract::UiPendingPatch {
    let mut owner = ui_contract::UiPendingPatch::default();
    *owner.source_mut().expect("new pending patch is writable") = patch;
    owner
}

fn close_surface_patch_owner(patch: &mut ui_contract::UiPendingPatch, credit: &mut Option<ui_contract::UiResidentPermit>, handback: &mut Option<SurfaceReconcileHandbackReservation>, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
    if items == 0 || bytes == 0 { return Ok(Default::default()); }
    if !patch.terminal_is_empty() {
        let mut step = patch.close_step(items, bytes)?;
        step.complete = false;
        return Ok(step);
    }
    if credit.is_some() { let mut step = release_surface_reconcile_one(credit)?; step.complete = false; return Ok(step); }
    if handback.is_some() { let mut step = release_surface_reconcile_handback_one(handback)?; step.complete = false; return Ok(step); }
    Ok(ui_contract::UiValueRetirementStep { complete: true, ..Default::default() })
}

impl SurfaceReconcileReadyPatch {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn surface(&self) -> Option<&ui_contract::SurfaceId> {
        self.patch.get().map(|patch| &patch.surface)
    }

    pub fn revision(&self) -> ui_contract::UiRevision {
        self.patch.get().map_or_else(ui_contract::UiRevision::default, |patch| patch.revision)
    }

    pub const fn required_publish_bytes() -> usize { size_of::<SurfaceReconcilePublishedPatch>() + size_of::<ui_contract::UiPatch>() + size_of::<ui_contract::SurfaceId>() }

    /// 📬️ Both destination slots and fixed initialization/moves are admitted before detaching the source.
    pub fn publish_into(&mut self, payload: &mut ui_contract::UiPendingPatch, published: &mut Option<SurfaceReconcilePublishedPatch>, admitted_bytes: usize) -> Result<usize, &'static str> {
        if admitted_bytes < Self::required_publish_bytes() || published.is_some() || !payload.terminal_is_empty() { return Ok(0); }
        if payload.source_mut()?.is_some() { return Ok(0); }
        let Some(source) = self.patch.get() else { return Ok(0); };
        let metadata = pending_surface_patch(Some(ui_contract::UiPatch { surface: source.surface.clone(), base_revision: source.base_revision, revision: source.revision, ops: ui_contract::UiPatchOps::default() }));
        let revision = source.revision;
        *payload.source_mut()? = self.patch.source_mut()?.take();
        *published = Some(SurfaceReconcilePublishedPatch { generation: self.generation, metadata, revision, credit: self.credit.take(), handback: self.handback.take() });
        Ok(Self::required_publish_bytes())
    }

    pub fn close_step(&mut self) -> bool {
        self.close_step_with_grant(1, 4096).expect("ready patch retirement remains exact").complete
    }

    pub fn close_step_with_grant(&mut self, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
        close_surface_patch_owner(&mut self.patch, &mut self.credit, &mut self.handback, items, bytes)
    }
    pub fn terminal_is_empty(&self) -> bool { self.patch.terminal_is_empty() && self.credit.is_none() && self.handback.is_none() }
}

impl Drop for SurfaceReconcileReadyPatch {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let generation = self.generation;
        handback_surface_reconcile(Box::new(SurfaceReconcileRetained {
            output_handback: None,
            generation,
            phase: SurfaceReconcileJobPhase::Closing,
            current: None,
            source: None,
            cursor: None,
            candidate: None,
            patch: take(&mut self.patch),
            retire_tree: SurfaceTreeRetireCursor::default(),
            fault: None,
            usage: SurfaceReconcileUsage::default(),
            credit: self.credit.take(),
            handback: self.handback.take(),
        }));
    }
}

/// 📮️ Credit witness retained by the reactor until the published revision is acknowledged or closed.
pub struct SurfaceReconcilePublishedPatch {
    pub(super) generation: u64,
    pub(super) metadata: ui_contract::UiPendingPatch,
    pub(super) revision: ui_contract::UiRevision,
    pub(super) credit: Option<ui_contract::UiResidentPermit>,
    pub(super) handback: Option<SurfaceReconcileHandbackReservation>,
}

impl SurfaceReconcilePublishedPatch {
    /// ♻️ Releases one exact published owner without handing its terminal into a global queue.
    pub fn close_step(&mut self) -> bool {
        self.close_step_with_grant(1, 4096).expect("published patch retirement remains exact").complete
    }

    pub fn close_step_with_grant(&mut self, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
        close_surface_patch_owner(&mut self.metadata, &mut self.credit, &mut self.handback, items, bytes)
    }

    /// 🧾️ Verifies this published handle owns neither payload nor admission/handback authority.
    pub fn terminal_is_empty(&self) -> bool {
        self.metadata.terminal_is_empty() && self.credit.is_none() && self.handback.is_none()
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn matches(&self, surface: &str, revision: u64) -> bool {
        self.surface().is_some_and(|exact| exact.0.as_str() == surface) && self.revision.0 == revision
    }

    pub fn surface(&self) -> Option<&ui_contract::SurfaceId> {
        self.metadata.get().map(|patch| &patch.surface)
    }

    pub fn revision(&self) -> ui_contract::UiRevision {
        self.revision
    }

    pub const fn required_acknowledge_bytes() -> usize { size_of::<Self>() + size_of::<SurfaceReconcilePublishedAck>() }

    /// ✅️ A mismatched receipt, occupied target, or insufficient grant leaves both slots untouched.
    pub fn acknowledge_into(source: &mut Option<Self>, target: &mut Option<SurfaceReconcilePublishedAck>, surface: &str, revision: u64, admitted_bytes: usize) -> Result<bool, &'static str> {
        if admitted_bytes < Self::required_acknowledge_bytes() || target.is_some() || !source.as_ref().is_some_and(|owner| owner.matches(surface, revision)) { return Ok(false); }
        *target = Some(SurfaceReconcilePublishedAck { owner: source.take().ok_or("published source lost after receipt validation")? });
        Ok(true)
    }
}

/// ✅️ Unforgeable ACK authority produced only by consuming the exact published patch owner.
pub struct SurfaceReconcilePublishedAck {
    pub(super) owner: SurfaceReconcilePublishedPatch,
}

impl SurfaceReconcilePublishedAck {
    pub fn generation(&self) -> u64 {
        self.owner.generation()
    }

    pub fn surface(&self) -> Option<&ui_contract::SurfaceId> {
        self.owner.surface()
    }

    pub fn revision(&self) -> ui_contract::UiRevision {
        self.owner.revision()
    }

    pub fn close_step_with_grant(&mut self, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> { self.owner.close_step_with_grant(items, bytes) }
    pub fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }
}

impl Drop for SurfaceReconcilePublishedPatch {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        handback_surface_reconcile(Box::new(SurfaceReconcileRetained {
            output_handback: None,
            generation: self.generation,
            phase: SurfaceReconcileJobPhase::Closing,
            current: None,
            source: None,
            cursor: None,
            candidate: None,
            patch: take(&mut self.metadata),
            retire_tree: SurfaceTreeRetireCursor::default(),
            fault: None,
            usage: SurfaceReconcileUsage::default(),
            credit: self.credit.take(),
            handback: self.handback.take(),
        }));
    }
}
//#endregion 🎟️RetainedAuthority
//#endregion ⏭️ResumableReconcile
