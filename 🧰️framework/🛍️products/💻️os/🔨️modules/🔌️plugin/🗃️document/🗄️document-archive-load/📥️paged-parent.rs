//! 🪟 Advances the declared typed Pack receiver on the original paged archive field.
use super::*;
use store::os_store::serialization::{ArtifactStoreSnapshot, PagedSnapshotReceiver};
use semio_framework_value::{ByteReadSource, RetainedWorkPoll};
use protocol::channel::archive_ingress::DocumentArchiveParentPages;

pub struct PagedArchiveParent<P> {
    pub pages: Option<DocumentArchiveParentPages>,
    receiver: Option<Box<dyn PagedSnapshotReceiver<P>>>,
    digest: semio_framework_hash::Hasher,
    hashed: usize,
}

impl<P> PagedArchiveParent<P> {
    pub fn new(pages: DocumentArchiveParentPages) -> Self {
        Self { pages: Some(pages), receiver: None, digest: semio_framework_hash::Hasher::new(), hashed: 0 }
    }

    pub fn spr(&self) -> Option<&dyn protocol::RetainedWireByteSource> {
        self.pages.as_ref().map(|pages| &pages.spr as &dyn protocol::RetainedWireByteSource)
    }

    pub fn receiver_ready(&self) -> bool {
        self.receiver.as_ref().is_some_and(|receiver| receiver.is_ready())
    }

    pub fn digest(&self) -> [u8; 32] { *self.digest.finalize().as_bytes() }

    pub fn take_initial(&mut self, initial: &mut Option<P>, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let receiver = self.receiver.as_mut().ok_or_else(|| ValueError::invariant("archive typed Pack receiver absent"))?;
        receiver.take_into(initial, grant)
    }

    pub fn receiver_close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        match self.receiver.as_ref() {
            Some(receiver) if !receiver.terminal_is_empty() => receiver.close_demands(body),
            Some(receiver) => Ok(RetirementDemand { copy_bytes: size_of::<Option<Box<dyn PagedSnapshotReceiver<P>>>>(), release_bytes: size_of_val(receiver.as_ref()), depth: 1, ..Default::default() }),
            None => Ok(Default::default()),
        }
    }

    pub fn close_receiver(&mut self, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::OriginalRetirementPoll {
        let Some(receiver) = self.receiver.as_mut() else {
            return RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome { work: 0, result: Ok(RetainedCloneStep::Complete(Default::default())) });
        };
        if receiver.terminal_is_empty() {
            use semio_framework_value::RetirementPollExit as Exit;
            let result = (|| {
                let demand = self.receiver_close_demand(effect.grant().maximum_copy_bytes)?;
                if effect.grant().maximum_items == 0 || !semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(), demand)
                    || effect.remaining_work_units()? == 0 || !effect.performed(1)?
                {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                self.receiver.take();
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress {
                    copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default()
                }))
            })();
            return Exit::settle(effect, result);
        }
        receiver.close_step(effect)
    }

    pub fn has_receiver(&self) -> bool { self.receiver.is_some() }
}

impl<P: ArtifactStoreSnapshot> PagedArchiveParent<P> {
    pub fn advance(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<RetainedCloneProgress, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::{OriginalTurnFailure, OriginalTurnPrimary};
        let Some(pages) = self.pages.as_ref() else {
            return Err(ValueError::invariant("archive typed Pack has no original pages").into());
        };
        let source = &pages.pack;
        let grant = cx.retained_grant();
        if self.hashed < source.len() {
            let copy = (source.len() - self.hashed).min(4096).min(grant.maximum_copy_bytes)
                .min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));
            if copy == 0 || grant.maximum_items == 0 || grant.maximum_depth == 0 || cx.should_yield() { return Ok(Default::default()); }
            let mut page = [0; 4096];
            if source.read_bytes(self.hashed, &mut page[..copy]) != copy {
                return Err(ValueError::invariant("archive Pack hash lost its source extent").into());
            }
            self.digest.update(&page[..copy]);
            self.hashed += copy;
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() };
            cx.consume_retained(progress)?;
            cx.consume_fuel(copy as u64);
            return Ok(progress);
        }

        let mut scope = None;
        let mut progress = Default::default();
        let mut primary = None;
        let mut effect_work = 0;
        let completed = cx.with_native_receiving_work(&mut scope, Default::default(), |native, issued, work| {
            if let Some(receiver) = self.receiver.as_mut() {
                if receiver.is_ready() { return Ok(()); }
                let demand = receiver.demands(source, issued.maximum_copy_bytes)?;
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(issued, demand) {
                    return Ok(());
                }
                if let Some(turn) = native.with_effect(issued, work, |native, effect| receiver.step(source, native, issued.maximum_copy_bytes, effect))? {
                    effect_work = turn.work;
                    match turn.result {
                        Ok(actual) => progress = actual,
                        Err(error) => { progress = error.retained_progress(); primary = Some(error); }
                    }
                }
            } else {
                let limits = protocol::PackLimits { max_file_len: 8 * 1024 * 1024, max_total_alloc: 32 * 1024 * 1024, ..Default::default() };
                if let Some((receiver, actual)) = P::admit_paged_snapshot(source, limits, native, issued)? {
                    self.receiver = Some(receiver);
                    progress = actual;
                    effect_work = 1;
                }
            }
            Ok(())
        })?;
        if !completed { return Ok(Default::default()); }
        let scope = scope.ok_or_else(|| ValueError::invariant("paged archive Pack lost its original receiving result"))?;
        let issued = cx.receiving_issued_grant().ok_or_else(|| ValueError::invariant("paged archive Pack lost its receiving grant"))?;
        if scope.source_refusal.is_some() || scope.call.is_err() {
            let progress = scope.retained_progress();
            let mut failure = OriginalTurnFailure::from(OriginalTurnPrimary::NativeSource(scope));
            failure.context_refusal = cx.consume_receiving_retained(issued, progress).err();
            cx.consume_work(effect_work);
            return Err(failure);
        }
        let receipt = cx.consume_receiving_retained(issued, progress);
        cx.consume_work(effect_work);
        if let Some(primary) = primary {
            return Err(OriginalTurnFailure { primary: OriginalTurnPrimary::Receiving(primary), effects: progress, context_refusal: receipt.err(), receipt_refusal: None, work_refusal: None });
        }
        receipt?;
        Ok(progress)
    }
}
