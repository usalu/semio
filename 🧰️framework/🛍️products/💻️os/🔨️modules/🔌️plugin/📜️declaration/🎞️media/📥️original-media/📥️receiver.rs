//! 📥️ Retains one issued media source in paid pages without decoding or changing its editor.

use super::ingress::{OriginalMediaLoan, ORIGINAL_MEDIA_PAGE_BYTES};
use semio_framework_value::{
    paged::PagedBytes, RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand,
    ValueError, ValueRefusalKind, retirement::controlled::ControlledRetirement,
};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_framework_value_persistent::list::{
    PagedListReservationFailure, PagedListReservationOutcome,
};

/// 🧾 Holds immutable page bytes until every destination octet has a paid slot.
pub struct OriginalMediaPage<'a> {
    pub loan: OriginalMediaLoan<'a>,
    pub index: u32,
    bytes: &'a [u8],
    copied: usize,
}

impl<'a> OriginalMediaPage<'a> {
    pub fn new(loan: OriginalMediaLoan<'a>, index: u32, bytes: &'a [u8]) -> Self {
        Self { loan, index, bytes, copied: 0 }
    }

    pub fn is_complete(&self) -> bool { self.copied == self.bytes.len() }

    pub(crate) fn reloan(mut self, copied: usize) -> Self {
        self.copied = copied;
        self
    }

    pub(crate) fn copied(&self) -> usize { self.copied }
}

const DOCUMENT_BYTES: usize = 256 * 4;

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
pub(crate) struct MediaOrigin {
    operation_id: u64,
    generation: u64,
    instance_id: u32,
    document: [u8; DOCUMENT_BYTES],
    document_len: usize,
    revision: [u8; 32],
}

impl MediaOrigin {
    pub(crate) fn loan(&self) -> OriginalMediaLoan<'_> {
        OriginalMediaLoan {
            operation_id: self.operation_id, generation: self.generation, instance_id: self.instance_id,
            document_id: std::str::from_utf8(&self.document[..self.document_len]).expect("captured UTF-8 identity"),
            base_revision: &self.revision,
        }
    }

    pub(crate) fn capture(loan: OriginalMediaLoan<'_>) -> Result<Self, ValueError> {
        let name = loan.document_id.as_bytes();
        if name.is_empty() || name.len() > DOCUMENT_BYTES || loan.document_id.chars().take(257).count() > 256 {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Original media document identity exceeds its bound"));
        }
        let mut document = [0; DOCUMENT_BYTES];
        document[..name.len()].copy_from_slice(name);
        Ok(Self { operation_id: loan.operation_id, generation: loan.generation, instance_id: loan.instance_id,
            document, document_len: name.len(), revision: *loan.base_revision })
    }

    pub(crate) fn matches(&self, loan: OriginalMediaLoan<'_>) -> bool {
        self.operation_id == loan.operation_id && self.generation == loan.generation && self.instance_id == loan.instance_id
            && &self.document[..self.document_len] == loan.document_id.as_bytes() && &self.revision == loan.base_revision
    }
}

/// 🪪 Holds an already issued instance loan and the one unpublished original input.
pub struct OriginalMediaPagedReceiver<const N: usize> {
    origin: MediaOrigin,
    source: PagedMediaSource<N>,
}

impl<const N: usize> OriginalMediaPagedReceiver<N> {
    /// 🎰️ Opens source storage within the already admitted picker authority.
    pub fn new(loan: OriginalMediaLoan<'_>, declared: usize, maximum_file_bytes: usize) -> Result<Self, ValueError> {
        let origin = MediaOrigin::capture(loan)?;
        let source = PagedMediaSource::new(declared, maximum_file_bytes)?;
        Ok(Self { origin, source })
    }

    pub(crate) fn admits_origin(&self, loan: OriginalMediaLoan<'_>) -> bool { self.origin.matches(loan) }
    pub(crate) fn origin(&self) -> &MediaOrigin { &self.origin }
    pub(crate) fn accepted_bytes(&self) -> usize { self.source.accepted_bytes() }

    fn check_origin(&self, loan: OriginalMediaLoan<'_>) -> Result<(), ValueError> {
        if !self.admits_origin(loan) {
            return Err(ValueError::invariant("Original media no longer owns its captured loan"));
        }
        Ok(())
    }

    /// 🧮 Quotes bounded storage for the next authenticated picker page.
    pub fn demand(&self, page: &OriginalMediaPage<'_>) -> Result<RetirementDemand, ValueError> {
        self.check_origin(page.loan)?;
        self.source.demand(page.index, page.bytes, page.copied)
    }

    /// 🪢 Copies the admitted range of an authenticated picker page.
    pub fn append(&mut self, page: &mut OriginalMediaPage<'_>, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        self.check_origin(page.loan)?;
        self.source.append(page.index, page.bytes, &mut page.copied, effect)
    }

    pub fn cancel(&mut self) { self.source.cancel(); }
    pub fn terminal_is_empty(&self) -> bool { self.source.terminal_is_empty() }

    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.source.close_demand(body)
    }

    pub fn close_step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Option<semio_framework_value::OriginalRetirementPoll> {
        self.source.close_step(body, effect)
    }

    /// 🧺 Donates the complete source after authenticating the current revision.
    pub fn take(&mut self, loan: OriginalMediaLoan<'_>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<(ControlledRetirement<PagedBytes<N>>, RetainedCloneProgress)>, ValueError> {
        self.take_with_output(loan, 0, effect)
    }

    pub(crate) fn take_with_output(&mut self, loan: OriginalMediaLoan<'_>, output: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<(ControlledRetirement<PagedBytes<N>>, RetainedCloneProgress)>, ValueError> {
        self.check_origin(loan)?;
        self.source.take(output, effect)
    }
}

/// 📦 Pays for the storage and retirement of an already authenticated source.
pub(crate) struct PagedMediaSource<const N: usize> {
    declared: usize,
    next_page: u32,
    owner: Option<ControlledRetirement<PagedBytes<N>>>,
    cancelled: bool,
}

impl<const N: usize> PagedMediaSource<N> {
    pub(crate) fn new(declared: usize, maximum_file_bytes: usize) -> Result<Self, ValueError> {
        if declared == 0 || declared > maximum_file_bytes || declared > N || declared > 512 * 1024 * 1024 {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Original media exceeds its issued source extent"));
        }
        Ok(Self { declared, next_page: 0, owner: Some(ControlledRetirement::new(PagedBytes::default()).map_err(|(error, _)| error)?), cancelled: false })
    }

    pub(crate) fn accepted_bytes(&self) -> usize {
        self.owner.as_ref().and_then(ControlledRetirement::original).map_or(0, PagedBytes::len)
    }

    /// 📏 Quotes either the next destination backing or one bounded copied byte range.
    pub(crate) fn demand(&self, index: u32, page: &[u8], copied: usize) -> Result<RetirementDemand, ValueError> {
        let bytes = self.check(index, page, copied)?;
        let backed = bytes.retained_bytes().capacity() - bytes.len();
        if backed == 0 { return bytes.byte_backing_demand(); }
        let count = backed.min(page.len() - copied);
        Ok(RetirementDemand { copy_bytes: count, depth: bytes.retained_bytes().next_push_depth_demand().map_err(ValueError::from)?.max(1), ..Default::default() })
    }

    fn check(&self, index: u32, page: &[u8], copied: usize) -> Result<&PagedBytes<N>, ValueError> {
        if self.cancelled || index != self.next_page {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Original media source is canceled or its page is out of order"));
        }
        let start = (index as usize).checked_mul(ORIGINAL_MEDIA_PAGE_BYTES).filter(|offset| *offset < self.declared)
            .ok_or_else(|| ValueError::invariant("Original media page exceeds its declared extent"))?;
        let bytes = self.owner.as_ref().and_then(ControlledRetirement::original).ok_or_else(|| ValueError::invariant("Original media input is sealed"))?;
        if page.len() != (self.declared - start).min(ORIGINAL_MEDIA_PAGE_BYTES) || copied >= page.len()
            || bytes.len() != start + copied {
            return Err(ValueError::invariant("Original media page extent or continuation changed"));
        }
        Ok(bytes)
    }

    /// 🧵 Reserves one physical backing or copies the admitted range from its immutable page.
    pub(crate) fn append(&mut self, index: u32, page: &[u8], copied: &mut usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        let mut demand = self.demand(index, page, *copied)?;
        let grant = effect.grant();
        let bytes = self.owner.as_mut().and_then(ControlledRetirement::original_mut).expect("verified original media source");
        if !bytes.retained_bytes().has_reserved_slot() {
            if !retained_clone_grant_admits(grant, demand) { return Ok(Default::default()); }
            return match bytes.retained_bytes_mut().reserve_one_funded(effect).result {
                Ok(PagedListReservationOutcome::Ready(progress)) => Ok(progress),
                Ok(PagedListReservationOutcome::Denied) => Ok(Default::default()),
                Err(PagedListReservationFailure::Work(error)) => Err(error),
                Err(PagedListReservationFailure::Allocation { error, .. }) => Err(ValueError::from(error.refusal())
                    .with_retained_progress(RetainedCloneProgress { retained_capacity_bytes: error.allocated_bytes, ..Default::default() })),
            };
        }
        let copy = demand.copy_bytes.min(grant.maximum_copy_bytes).min(usize::try_from(effect.remaining_work_units()?).unwrap_or(usize::MAX));
        demand.copy_bytes = copy;
        if copy == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(copy as u64)? { return Ok(Default::default()); }
        let start = *copied;
        for byte in &page[start..start + copy] {
            bytes.push_reserved_byte(*byte).map_err(|error| error.with_retained_progress(RetainedCloneProgress {
                copied_items: usize::from(*copied != start), copied_bytes: *copied - start, ..Default::default()
            }))?;
            *copied += 1;
        }
        if *copied == page.len() { self.next_page += 1; }
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() })
    }

    /// 🛑 Forbids any subsequent source write or publication while the owner retires.
    pub fn cancel(&mut self) { self.cancelled = true; }

    pub fn terminal_is_empty(&self) -> bool { self.owner.as_ref().is_none_or(ControlledRetirement::terminal_is_empty) }

    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.owner.as_ref().map_or(Ok(Default::default()), |owner| owner.next_demand(body))
    }

    pub fn close_step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Option<semio_framework_value::OriginalRetirementPoll> {
        self.cancel();
        self.owner.as_mut().map(|owner| owner.step(body, effect))
    }

    /// 🎟️ Donates the complete original to its already authenticated format owner.
    pub(crate) fn take(&mut self, output: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<(ControlledRetirement<PagedBytes<N>>, RetainedCloneProgress)>, ValueError> {
        if self.cancelled || self.owner.as_ref().and_then(ControlledRetirement::original).is_none_or(|bytes| bytes.len() != self.declared) {
            return Err(ValueError::invariant("Original media publication needs a complete source"));
        }
        let demand = RetirementDemand { copy_bytes: std::mem::size_of_val(&self.owner).checked_add(output)
            .ok_or_else(|| ValueError::invariant("Original media output quote exceeds its extent"))?, depth: 1, ..Default::default() };
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? { return Ok(None); }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        Ok(self.owner.take().map(|owner| (owner, progress)))
    }
}
