//! 📦 Bounded receiving for the memberless version-1 folder archive.
use semio_os_channel_archive::archive_schema::DOCUMENT_ARCHIVE_VERSION;
use semio_framework_hash::Sha256;
use semio_framework_value::{
    OriginalRetirementFailure, RetainedAllocationEffect, RetainedCloneProgress, RetainedWorkOutcome,
    RetirementDemand, ValueError, ValueRefusalKind, paged::PagedBytes,
    retained_clone::retained_clone_grant_admits,
};
use semio_framework_value_persistent::list::{PagedListReservationFailure, PagedListReservationOutcome};
use std::mem::size_of;

pub const DOCUMENT_ARCHIVE_INGRESS_PAGE_BYTES: usize = 4096;
pub type DocumentArchivePages = PagedBytes<{isize::MAX as usize}>;

#[path = "📥️folder-load.rs"]
pub mod folder_archive_load;

/// 📐 The persisted encoded extent and the receiving domains each have their own authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DocumentArchiveIngressLimits {
    pub archive_bytes: usize,
    pub pack_bytes: usize,
    pub spr_bytes: usize,
}

/// 🪪 The previously verified folder record supplies the expected extent and SHA-256.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DocumentArchiveIngressSeal {
    pub bytes: usize,
    pub sha256: [u8; 32],
}

/// 🧱 The original fixed pages can move directly into a typed Store receiving or retirement lane.
#[derive(Debug, Default, semio_framework_value::RetireOwned)]
pub struct DocumentArchiveParentPages {
    pub pack: DocumentArchivePages,
    pub spr: DocumentArchivePages,
}

/// 🔏 Only exact, verified version-1 parent pages can receive this token.
pub struct VerifiedDocumentArchiveParent {
    pages: DocumentArchiveParentPages,
    seal: DocumentArchiveIngressSeal,
}

impl VerifiedDocumentArchiveParent {
    pub fn seal(&self) -> DocumentArchiveIngressSeal { self.seal }
    pub fn spr_len(&self) -> usize { self.pages.spr.len() }
    pub fn into_pages(self) -> DocumentArchiveParentPages { self.pages }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Phase {
    Version,
    PackLength,
    Pack,
    SprLength,
    Spr,
    Members,
    Framed,
    Ready,
    Cancelled,
    Refused,
    Taken,
}

/// 🎟 One retained archive cursor admits original page storage only while moving the ordered frontier.
pub struct PagedDocumentArchiveIngress {
    limits: DocumentArchiveIngressLimits,
    seal: DocumentArchiveIngressSeal,
    phase: Phase,
    accepted: usize,
    field_bytes: usize,
    length: u64,
    shift: u32,
    sha256: Sha256,
    pages: DocumentArchiveParentPages,
}

impl PagedDocumentArchiveIngress {
    pub fn new(seal: DocumentArchiveIngressSeal, limits: DocumentArchiveIngressLimits) -> Result<Self, ValueError> {
        if seal.bytes > limits.archive_bytes {
            return Err(limit("encoded archive exceeds the persisted wire authority"));
        }
        if seal.bytes < 6 {
            return Err(invalid("version-1 archive is shorter than a nonempty parent pair"));
        }
        Ok(Self {
            limits, seal, phase: Phase::Version, accepted: 0, field_bytes: 0,
            length: 0, shift: 0, sha256: Sha256::new(), pages: Default::default(),
        })
    }

    pub fn accepted(&self) -> usize { self.accepted }
    pub fn ready(&self) -> bool { self.phase == Phase::Ready }

    pub fn cancel(&mut self) {
        if self.phase != Phase::Taken {
            self.phase = Phase::Cancelled;
        }
    }

    /// 🪜 A single turn admits one backing allocation, one framing octet, or one bounded body fragment.
    pub fn step(
        &mut self,
        offset: usize,
        bytes: &[u8],
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalRetirementFailure> {
        let work = effect.performed_boundary();
        let result = self.advance(offset, bytes, effect);
        if result.is_err() && !matches!(self.phase, Phase::Ready | Phase::Cancelled | Phase::Taken) {
            self.phase = Phase::Refused;
        }
        RetainedWorkOutcome {
            work: effect.performed_since(work).expect("archive receipt keeps its original Work"),
            result,
        }
    }

    /// 🧾 The durable source explicitly closes its byte stream, which must match the committed extent.
    pub fn finish(
        &mut self,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalRetirementFailure> {
        if self.accepted != self.seal.bytes && !matches!(self.phase, Phase::Cancelled | Phase::Taken) {
            self.phase = Phase::Refused;
            return RetainedWorkOutcome { work: 0, result: Err(invalid("archive source ended before its committed extent").into()) };
        }
        self.step(self.accepted, &[], effect)
    }

    fn advance(
        &mut self,
        offset: usize,
        bytes: &[u8],
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, OriginalRetirementFailure> {
        if self.phase == Phase::Cancelled {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "archive ingress was cancelled").into());
        }
        if self.phase == Phase::Ready && bytes.is_empty() && offset == self.accepted {
            return Ok(Default::default());
        }
        if matches!(self.phase, Phase::Ready | Phase::Refused | Phase::Taken) {
            return Err(invalid("archive ingress no longer accepts input").into());
        }
        if offset != self.accepted || bytes.len() > DOCUMENT_ARCHIVE_INGRESS_PAGE_BYTES
            || bytes.len() > self.seal.bytes - self.accepted
        {
            return Err(invalid("archive page does not begin at its exact bounded frontier").into());
        }

        if self.phase == Phase::Framed {
            let copy = size_of::<Sha256>() + size_of::<[u8; 32]>();
            if !pay(effect, copy, 128, 1)? {
                return Ok(Default::default());
            }
            if self.sha256.clone().finalize() != self.seal.sha256 {
                return Err(invalid("archive bytes do not match the committed SHA-256").into());
            }
            self.phase = Phase::Ready;
            return Ok(receipt(copy));
        }
        if self.accepted == self.seal.bytes {
            return Err(invalid("archive ended before its zero-member frame completed").into());
        }
        let Some(&byte) = bytes.first() else { return Ok(Default::default()); };

        if matches!(self.phase, Phase::Pack | Phase::Spr) {
            let pack = self.phase == Phase::Pack;
            let pages = if pack { &mut self.pages.pack } else { &mut self.pages.spr };
            let list = pages.retained_bytes_mut();
            if !list.has_reserved_slot() {
                let turn = list.reserve_capacity_one_funded(self.field_bytes, effect);
                return match turn.result {
                    Ok(PagedListReservationOutcome::Ready(progress)) => Ok(progress),
                    Ok(PagedListReservationOutcome::Denied) => Ok(Default::default()),
                    Err(PagedListReservationFailure::Work(error)) => Err(error.into()),
                    Err(PagedListReservationFailure::Allocation { error, .. }) => {
                        let progress = RetainedCloneProgress {
                            copied_items: usize::from(error.allocated_bytes != 0),
                            retained_capacity_bytes: error.allocated_bytes,
                            ..Default::default()
                        };
                        Err(ValueError::from(error.refusal()).with_retained_progress(progress).into())
                    }
                };
            }
            let demand = list.push_reserved_source_demand().map_err(ValueError::from)?;
            let work_per_byte = list.push_reserved_source_work_demand().map_err(ValueError::from)? + 2;
            let work_bytes = usize::try_from(effect.remaining_work_units()? / work_per_byte).unwrap_or(usize::MAX);
            let copy_bytes = effect.grant().maximum_copy_bytes.saturating_sub(size_of::<Sha256>()) / demand.copy_bytes;
            let count = bytes.len().min(list.capacity() - list.len()).min(self.field_bytes - list.len())
                .min(work_bytes).min(copy_bytes);
            let copy = demand.copy_bytes * count + size_of::<Sha256>();
            let work = work_per_byte * count as u64;
            if count == 0 || !pay(effect, copy, work, demand.depth)? {
                return Ok(Default::default());
            }
            for &byte in &bytes[..count] {
                list.push_reserved(byte).map_err(|_| ValueError::invariant("archive octet lost its reserved backing"))?;
            }
            if list.len() == self.field_bytes {
                self.phase = if pack { Phase::SprLength } else { Phase::Members };
            }
            self.acknowledge(&bytes[..count]);
            return Ok(receipt(copy));
        }

        let copy = size_of::<Sha256>() + 4 * size_of::<usize>();
        if !pay(effect, copy, 2, 1)? {
            return Ok(Default::default());
        }
        match self.phase {
            Phase::Version if byte == DOCUMENT_ARCHIVE_VERSION => self.phase = Phase::PackLength,
            Phase::Version => return Err(invalid("unsupported document archive format").into()),
            Phase::PackLength | Phase::SprLength => self.length_byte(byte)?,
            Phase::Members if byte == 0 && self.accepted + 1 == self.seal.bytes => self.phase = Phase::Framed,
            Phase::Members => return Err(invalid("memberless archive must end with exactly one zero member byte").into()),
            _ => return Err(ValueError::invariant("archive framing reached an invalid frontier").into()),
        }
        self.acknowledge(&bytes[..1]);
        Ok(receipt(copy))
    }

    fn length_byte(&mut self, byte: u8) -> Result<(), ValueError> {
        if self.shift == 63 && byte > 1 {
            return Err(invalid("archive field length exceeds u64"));
        }
        self.length |= u64::from(byte & 127) << self.shift;
        if byte >= 128 {
            self.shift += 7;
            return Ok(());
        }
        if byte == 0 && self.shift != 0 {
            return Err(invalid("archive field length is not minimally framed"));
        }
        let pack = self.phase == Phase::PackLength;
        let maximum = if pack { self.limits.pack_bytes } else { self.limits.spr_bytes };
        let length = usize::try_from(self.length).map_err(|_| limit("archive field exceeds address space"))?;
        if length > maximum || length > isize::MAX as usize {
            return Err(limit("archive field exceeds its independent receiving authority"));
        }
        let framing = if pack { 3 } else { 1 };
        if length == 0 || length > (self.seal.bytes - self.accepted - 1).saturating_sub(framing) {
            return Err(invalid("archive field crosses its declared frame"));
        }
        self.shift = 0;
        self.length = 0;
        self.field_bytes = length;
        self.phase = if pack { Phase::Pack } else { Phase::Spr };
        Ok(())
    }

    fn acknowledge(&mut self, bytes: &[u8]) {
        self.sha256.update(bytes);
        self.accepted += bytes.len();
    }

    /// 🫴 The receiving owner moves the original verified pages without cloning either parent.
    pub fn take_verified(
        &mut self,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<Option<(VerifiedDocumentArchiveParent, RetainedCloneProgress)>, ValueError> {
        if !self.ready() {
            return Ok(None);
        }
        let copy = size_of::<VerifiedDocumentArchiveParent>() + size_of::<DocumentArchiveParentPages>();
        if !pay(effect, copy, 1, 1)? {
            return Ok(None);
        }
        self.phase = Phase::Taken;
        Ok(Some((VerifiedDocumentArchiveParent { pages: std::mem::take(&mut self.pages), seal: self.seal }, receipt(copy))))
    }

    /// 🪦 Refused or cancelled input moves to the actual bounded retirement recipient.
    pub fn take_unverified(
        &mut self,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<Option<(DocumentArchiveParentPages, RetainedCloneProgress)>, ValueError> {
        let copy = 2 * size_of::<DocumentArchiveParentPages>();
        if !matches!(self.phase, Phase::Refused | Phase::Cancelled) || !pay(effect, copy, 1, 1)? {
            return Ok(None);
        }
        self.phase = Phase::Taken;
        Ok(Some((std::mem::take(&mut self.pages), receipt(copy))))
    }
}

impl Drop for PagedDocumentArchiveIngress {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (
            self.pages.pack.retained_bytes().terminal_is_empty()
                && self.pages.spr.retained_bytes().terminal_is_empty()
        ), "document archive ingress still owns untransferred pages");
    }
}

fn pay(effect: &mut RetainedAllocationEffect<'_>, copy: usize, work: u64, depth: usize) -> Result<bool, ValueError> {
    let grant = effect.grant();
    let demand = RetirementDemand { copy_bytes: copy, depth, ..Default::default() };
    if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand)
        || effect.remaining_work_units()? < work
    {
        return Ok(false);
    }
    effect.performed(work)
}

fn receipt(copy: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() }
}

fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}

fn limit(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::OwnershipLimit, message)
}
