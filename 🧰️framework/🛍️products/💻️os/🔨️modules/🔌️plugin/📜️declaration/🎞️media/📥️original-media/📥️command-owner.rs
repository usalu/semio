//! 📥️ Retains the exact picker command through paid guest ingress and source handoff.

use super::{bridge::OriginalMediaCommandBinding, ingress::OriginalMediaLoan, receiver::{MediaOrigin, OriginalMediaPagedReceiver}};
use semio_framework_replication_command_ingress::{CommandPageCursor, FixedCommandPage, COMMAND_MAXIMUM_BYTES};
use semio_framework_value::{
    OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand,
    ValueError, ValueRefusalKind, paged::PagedBytes, retained_clone::retained_clone_grant_admits,
    retirement::controlled::ControlledRetirement,
};

struct PendingPage {
    header: CommandPageCursor,
    body: ControlledRetirement<FixedCommandPage>,
    copied: usize,
    complete: bool,
}

/// 🎬 Holds one original input and its guest-selected target before format admission.
pub struct OriginalMediaCommand<T, const N: usize> {
    transport: (u64, u64, u8, u32),
    target: T,
    receiver: OriginalMediaPagedReceiver<N>,
    page: Option<PendingPage>,
    cancelled: bool,
}

/// 🎁 Donates the retained byte owner and the captured edit target together.
#[derive(semio_framework_value::RetireOwned)]
pub struct OriginalMediaFormatInput<T, const N: usize> {
    #[value(retire_with = "std::mem::drop")]
    origin: MediaOrigin,
    pub target: T,
    pub bytes: ControlledRetirement<PagedBytes<N>>,
}

impl<T, const N: usize> OriginalMediaFormatInput<T, N> {
    pub fn loan(&self) -> OriginalMediaLoan<'_> { self.origin.loan() }
    pub fn admits_origin(&self, current: OriginalMediaLoan<'_>) -> bool { self.origin.matches(current) }
}

fn paid(effect: &mut RetainedAllocationEffect<'_>, demand: RetirementDemand, work: u64) -> Result<bool, ValueError> {
    Ok(retained_clone_grant_admits(effect.grant(), demand)
        && effect.remaining_work_units()? >= work && effect.performed(work)?)
}

fn moved(demand: RetirementDemand) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }
}

fn copy_demand<T>() -> RetirementDemand {
    RetirementDemand { copy_bytes: size_of::<T>(), depth: 1, ..Default::default() }
}

impl<T: Copy, const N: usize> OriginalMediaCommand<T, N> {
    pub const BIRTH_WORK: u64 = 1 + (256 * 4);

    pub fn birth_demand() -> RetirementDemand { copy_demand::<Option<Self>>() }

    /// 🪪 Starts a receiver only from the issued picker binding and its live artifact identity.
    pub fn admit(
        issued: &OriginalMediaCommandBinding<'_>, current: OriginalMediaLoan<'_>, target: T, maximum: usize,
        recipient: &mut Option<Self>, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        Self::admit_with_metadata(issued, current, target, maximum, recipient, 0, effect)
    }

    pub(crate) fn admit_with_metadata(
        issued: &OriginalMediaCommandBinding<'_>, current: OriginalMediaLoan<'_>, target: T, maximum: usize,
        recipient: &mut Option<Self>, envelope_bytes: usize, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        if issued.loan != current || issued.owner == 0 || issued.seq == 0 || issued.kind == 0 {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Original media command has no live picker authority"));
        }
        if issued.declared == 0 || issued.declared as usize > maximum.min(N).min(COMMAND_MAXIMUM_BYTES) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Original media command exceeds the admitted transport extent"));
        }
        if recipient.is_some() { return Ok(Default::default()); }
        let mut demand = Self::birth_demand();
        demand.copy_bytes = demand.copy_bytes.checked_add(envelope_bytes)
            .ok_or_else(|| ValueError::invariant("Original media birth quote overflowed"))?;
        if !paid(effect, demand, Self::BIRTH_WORK)? { return Ok(Default::default()); }
        let progress = moved(demand);
        let receiver = OriginalMediaPagedReceiver::new(issued.loan, issued.declared as usize, maximum)
            .map_err(|error| error.with_retained_progress(progress))?;
        *recipient = Some(Self { transport: (issued.owner, issued.seq, issued.kind, issued.declared),
            target, receiver, page: None, cancelled: false });
        Ok(progress)
    }

    fn check_live(&mut self, current: OriginalMediaLoan<'_>) -> Result<(), ValueError> {
        if !self.receiver.admits_origin(current) { self.abandon(); }
        if self.cancelled {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "Original media no longer owns the live PDF revision"));
        }
        Ok(())
    }

    fn binding<'a>(&self, loan: OriginalMediaLoan<'a>) -> OriginalMediaCommandBinding<'a> {
        let (owner, seq, kind, declared) = self.transport;
        OriginalMediaCommandBinding { loan, owner, seq, kind, declared }
    }

    pub fn stage_demand() -> RetirementDemand { copy_demand::<Option<PendingPage>>() }

    /// 📤 Moves one exact WIT page into guest custody and keeps it across interrupted turns.
    pub fn stage_page(
        &mut self, current: OriginalMediaLoan<'_>,
        incoming: &mut Option<(CommandPageCursor, FixedCommandPage)>, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        self.check_live(current)?;
        let Some((header, page)) = incoming.as_ref() else { return Ok(Default::default()); };
        {
            let binding = self.binding(current);
            let offered = binding.body(header, page)?;
            if self.page.is_some() { return Ok(Default::default()); }
            self.receiver.demand(&offered)?;
        }
        let demand = Self::stage_demand();
        if !paid(effect, demand, 1)? { return Ok(Default::default()); }
        let progress = moved(demand);
        let (header, page) = incoming.take().expect("admitted command page");
        match ControlledRetirement::new(page) {
            Ok(body) => self.page = Some(PendingPage { header, body, copied: 0, complete: false }),
            Err((error, page)) => {
                *incoming = Some((header, page));
                return Err(error.with_retained_progress(progress));
            }
        }
        Ok(progress)
    }

    /// 🔁 Reborrows the same retained page at its paid frontier on renewed turns.
    pub fn copy_page(&mut self, current: OriginalMediaLoan<'_>, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        self.check_live(current)?;
        let (owner, seq, kind, declared) = self.transport;
        let binding = OriginalMediaCommandBinding { loan: current, owner, seq, kind, declared };
        let Some(page) = self.page.as_mut().filter(|page| !page.complete) else { return Ok(Default::default()); };
        let raw = page.body.original().expect("copying page retains its source");
        let mut prefix = binding.body(&page.header, raw)?.reloan(page.copied);
        let receipt = self.receiver.append(&mut prefix, effect);
        page.copied = prefix.copied();
        page.complete = prefix.is_complete();
        if receipt.is_err() { self.abandon(); }
        receipt
    }

    pub fn accepted_bytes(&self) -> usize { self.receiver.accepted_bytes() }

    pub fn page_retirement_demand(&self, body: usize) -> Result<Option<RetirementDemand>, ValueError> {
        self.page.as_ref().filter(|page| (page.complete || self.cancelled) && !page.body.terminal_is_empty())
            .map(|page| page.body.next_demand(body)).transpose()
    }

    /// ♻️ Returns the staged WIT page backing before its page acknowledgment.
    pub fn retire_page(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Option<OriginalRetirementPoll> {
        self.page.as_mut().filter(|page| page.complete || self.cancelled)
            .filter(|page| !page.body.terminal_is_empty()).map(|page| page.body.step(body, effect))
    }

    /// ✅ Acknowledges only a fully copied page whose staged input has closed.
    pub fn acknowledge_page(
        &mut self, current: OriginalMediaLoan<'_>, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<Option<(u32, RetainedCloneProgress)>, ValueError> {
        self.check_live(current)?;
        if !self.page.as_ref().is_some_and(|page| page.complete && page.body.terminal_is_empty()) {
            return Ok(None);
        }
        let demand = Self::stage_demand();
        if !paid(effect, demand, 1)? { return Ok(None); }
        let page = self.page.take().expect("acknowledged Original page");
        Ok(Some((page.header.page_index, moved(demand))))
    }

    /// 🎯 Offers the complete byte owner to the format factory inside the current revision.
    pub fn complete_into(
        &mut self, current: OriginalMediaLoan<'_>, output: &mut Option<OriginalMediaFormatInput<T, N>>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        self.complete_with_metadata(current, output, 0, effect)
    }

    pub(crate) fn complete_with_metadata(
        &mut self, current: OriginalMediaLoan<'_>, output: &mut Option<OriginalMediaFormatInput<T, N>>,
        envelope_bytes: usize, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        self.check_live(current)?;
        if self.page.is_some() || output.is_some() { return Ok(Default::default()); }
        let output_bytes = size_of::<Option<OriginalMediaFormatInput<T, N>>>().checked_add(envelope_bytes)
            .ok_or_else(|| ValueError::invariant("Original media handoff quote overflowed"))?;
        let Some((bytes, progress)) = self.receiver.take_with_output(current, output_bytes, effect)? else {
            return Ok(Default::default());
        };
        *output = Some(OriginalMediaFormatInput { origin: *self.receiver.origin(), target: self.target, bytes });
        Ok(progress)
    }

    /// 🛑 Cancels the issued transfer without trusting an unsolicited command body.
    pub fn cancel(&mut self, issued: OriginalMediaLoan<'_>) -> Result<(), ValueError> {
        if !self.receiver.admits_origin(issued) { return Err(ValueError::invariant("Cancellation names another issued media command")); }
        self.abandon();
        Ok(())
    }

    pub fn abandon(&mut self) {
        self.cancelled = true;
        self.receiver.cancel();
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.page.as_ref().is_none_or(|page| page.body.terminal_is_empty()) && self.receiver.terminal_is_empty()
    }

    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        match self.page.as_ref() {
            Some(page) if !page.body.terminal_is_empty() => page.body.next_demand(body),
            _ => self.receiver.close_demand(body),
        }
    }

    pub fn close_step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Option<OriginalRetirementPoll> {
        self.abandon();
        if let Some(retired) = self.retire_page(body, effect) { return Some(retired); }
        if self.receiver.terminal_is_empty() { return None; }
        self.receiver.close_step(body, effect)
    }
}
