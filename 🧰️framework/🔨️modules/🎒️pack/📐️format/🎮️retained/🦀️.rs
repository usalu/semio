//! 🪢️ Retained canonical Pack source admission: bounded, cancellable byte-at-a-time cursors for pages, segments, varints and the pack anchors.
//! The framing, writer and reader live in `semio-framework-pack-format`; the catalog cursors live in `semio-framework-pack-format-catalog`.

use semio_framework_pack_error::PackRefusal;
use semio_framework_replication_codec::{ContentHash, PackLimits};
use semio_framework_value::{ValueRefusalKind, RetirementDemand, ValueError};
use semio_framework_value::list::{PagedListError, PagedListAllocationError, PagedListRefusalKind};
use std::mem::size_of;
use semio_framework_pack_format::{FOOTER_MAGIC, FOOTER_SIZE, FORMAT_VERSION_MAJOR, Footer, HEADER_SIZE, Header, MAGIC, REQUIRED_KNOWN_MASK, Superblock};
#[cfg(test)]
use semio_framework_replication_codec::{ByteRange, CodecId};
#[cfg(test)]
use semio_framework_pack_source::PackSink;

pub fn retained_page_close_demands<T, const N: usize>(pages: &semio_framework_value::list::PagedList<T, N>) -> Result<RetirementDemand, ValueError> {
    let error = |error: PagedListError| ValueError::literal(ValueRefusalKind::OwnershipLimit, error.reason);
    if !pages.is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<T>(), depth: pages.next_pop_depth_demand().map_err(error)?, ..Default::default() }); }
    Ok(RetirementDemand { release_bytes: pages.next_release_allocation_bytes().map_err(error)?, depth: pages.next_release_depth_demand().map_err(error)?.max(1), ..Default::default() })
}

//#region 🔖️RetainedCanonicalSource
pub const RETAINED_PACK_PAGE_BYTES: usize = 4_096;

#[derive(Debug)]
pub struct RetainedPackPage {
    bytes: [u8; RETAINED_PACK_PAGE_BYTES],
    len: usize,
}

pub const RETAINED_PACK_MAXIMUM_PAGES: usize = isize::MAX as usize / size_of::<RetainedPackPage>();

type RetainedPackPages = semio_framework_value::list::PagedList<RetainedPackPage, RETAINED_PACK_MAXIMUM_PAGES>;

impl RetainedPackPage {
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[expect(clippy::result_large_err, reason = "Invalid page length returns the caller-owned fixed byte array without allocating on rejection.")]
    pub fn try_from_array(bytes: [u8; RETAINED_PACK_PAGE_BYTES], len: usize) -> Result<Self, [u8; RETAINED_PACK_PAGE_BYTES]> {
        if len == 0 || len > RETAINED_PACK_PAGE_BYTES {
            return Err(bytes);
        }
        Ok(Self { bytes, len })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn into_array(self) -> ([u8; RETAINED_PACK_PAGE_BYTES], usize) {
        (self.bytes, self.len)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedPackSourceEvent {
    Byte { offset: u64, value: u8 },
    Complete { bytes: u64, pages: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackSourceProgress {
    pub admitted_pages: usize,
    pub admitted_bytes: usize,
    pub reserved_pages: usize,
    pub allocated_bytes: usize,
    pub consumed_bytes: u64,
    pub sealed: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedPackSourceAllocationStep {
    pub progressed: bool,
    pub allocated_bytes: usize,
}

/// 📥️ Holds a source-owned borrowed kind, reason and optional actual allocation witness.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackSourceFault { pub kind:ValueRefusalKind, pub reason:&'static str, pub allocated_bytes:Option<usize> }

impl RetainedPackSourceFault {
    /// 🪪️ Requires explicit producer authority at the refusal site.
    pub const fn refusal(kind:ValueRefusalKind,reason:&'static str)->Self{Self{kind,reason,allocated_bytes:None}}
    /// 📋️ Keeps the actual lower kind and static reason.
    pub const fn from_paged_refusal(error:PagedListError)->Self{Self::refusal(retained_paged_kind(error.kind),error.reason)}
    /// 🧱️ Keeps zero or nonzero actual lower allocation metadata.
    pub const fn from_paged_allocation(error:PagedListAllocationError)->Self{Self::allocation(retained_paged_kind(error.kind),error.reason,error.allocated_bytes)}
    /// 📏️ Retains the provider's byte witness independently of the machine kind.
    pub const fn allocation(kind:ValueRefusalKind,reason:&'static str,allocated_bytes:usize)->Self{Self{kind,reason,allocated_bytes:Some(allocated_bytes)}}
    /// 📤️ Projects into the canonical Pack owner without allocating.
    pub const fn into_pack_refusal(self,what:&'static str,offset:u64)->PackRefusal{match self.allocated_bytes{None=>PackRefusal::RetainedMalformed{kind:self.kind,what,offset,detail:self.reason},Some(allocated_bytes)=>PackRefusal::RetainedAllocation{kind:self.kind,allocated_bytes,what,offset,detail:self.reason}}}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackSourceAllocationError { pub allocated_bytes:usize, pub fault:RetainedPackSourceFault }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedPackCloseStep {
    Pending { released_items: usize, released_bytes: usize },
    Complete,
}

pub struct RetainedPackSourceCursor {
    pages: RetainedPackPages,
    maximum_pages: usize,
    maximum_payload_bytes: usize,
    maximum_allocation_bytes: usize,
    admitted_bytes: usize,
    page: usize,
    byte: usize,
    consumed: u64,
    sealed: bool,
    cancelled: bool,
    completed: bool,
    allocation_fault: Option<RetainedPackSourceFault>,
    closed: bool,
}

impl RetainedPackSourceCursor {
    pub fn try_new(maximum_pages: usize, maximum_payload_bytes: usize, maximum_allocation_bytes: usize) -> Result<Self, RetainedPackSourceFault> {
        if maximum_pages==0||maximum_payload_bytes==0{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.zero-credits"));}
        if maximum_allocation_bytes==0{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.zero-credits"));}
        if maximum_pages > RETAINED_PACK_MAXIMUM_PAGES {
            return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.page-credits"));
        }
        if maximum_payload_bytes > maximum_pages.checked_mul(RETAINED_PACK_PAGE_BYTES).ok_or(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.credit-overflow"))? {
            return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.byte-credits"));
        }
        if maximum_allocation_bytes > isize::MAX as usize {
            return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.allocation-credits"));
        }
        Ok(Self {
            pages: RetainedPackPages::default(),
            maximum_pages,
            maximum_payload_bytes,
            maximum_allocation_bytes,
            admitted_bytes: 0,
            page: 0,
            byte: 0,
            consumed: 0,
            sealed: false,
            cancelled: false,
            completed: false,
            allocation_fault: None,
            closed: false,
        })
    }

    pub fn next_allocation_bytes(&self) -> Result<usize, RetainedPackSourceFault> {
        if let Some(fault) = self.allocation_fault {
            return Err(fault);
        }
        if self.closed||self.sealed{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.source-closed"));}
        if self.cancelled{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::Canceled,"retained-pack.source-closed"));}
        if self.pages.len() == self.maximum_pages {
            return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.page-credits"));
        }
        let requested = self.pages.next_allocation_bytes().map_err(RetainedPackSourceFault::from_paged_refusal)?;
        self.pages
            .allocated_bytes()
            .checked_add(requested)
            .filter(|total| *total <= self.maximum_allocation_bytes)
            .ok_or(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.allocation-credits"))?;
        Ok(requested)
    }

    pub fn reserve_page(&mut self, maximum_bytes: usize) -> Result<RetainedPackSourceAllocationStep, RetainedPackSourceAllocationError> {
        let rejected=|fault|RetainedPackSourceAllocationError{allocated_bytes:0,fault};
        let requested = self.next_allocation_bytes().map_err(rejected)?;
        let remaining = self.maximum_allocation_bytes - self.pages.allocated_bytes();
        let step = match self.pages.reserve_one(maximum_bytes.min(remaining)) {
            Ok(step) => step,
            Err(error) => {
                if error.allocated_bytes != 0 {
                    self.allocation_fault=Some(RetainedPackSourceFault::from_paged_allocation(error));
                }
                return Err(RetainedPackSourceAllocationError{allocated_bytes:error.allocated_bytes,fault:RetainedPackSourceFault::from_paged_allocation(error)});
            }
        };
        if self.pages.allocated_bytes() > self.maximum_allocation_bytes {
            let fault=RetainedPackSourceFault::allocation(ValueRefusalKind::OwnershipLimit,"retained-pack actual allocation exceeds physical credits; owner retained",step.allocated_bytes);
            self.allocation_fault=Some(fault);
            return Err(RetainedPackSourceAllocationError {
                allocated_bytes: step.allocated_bytes,
                fault,
            });
        }
        debug_assert!(step.allocated_bytes == 0 || step.allocated_bytes >= requested);
        Ok(RetainedPackSourceAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
    }

    pub fn has_reserved_page(&self) -> bool {
        self.allocation_fault.is_none() && self.pages.has_reserved_slot() && self.pages.len() < self.maximum_pages
    }

    pub fn allocated_bytes(&self) -> usize {
        self.pages.allocated_bytes()
    }

    /// 📏️ Observes the original page movement or whole backing release before close.
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        retained_page_close_demands(&self.pages)
    }

    pub fn next_release_allocation_bytes(&self) -> Result<usize, PagedListError> {
        self.pages.next_release_allocation_bytes()
    }

    pub fn preflight_page(&self, len: usize) -> Result<(), RetainedPackSourceFault> {
        if let Some(fault) = self.allocation_fault {
            return Err(fault);
        }
        if self.closed||self.sealed{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.source-closed"));}
        if self.cancelled{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::Canceled,"retained-pack.source-closed"));}
        let pages = self.pages.len().checked_add(1).ok_or(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.page-overflow"))?;
        let bytes = self.admitted_bytes.checked_add(len).ok_or(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.byte-overflow"))?;
        if len==0||len>RETAINED_PACK_PAGE_BYTES{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.producer-handback"));}
        if pages>self.maximum_pages||bytes>self.maximum_payload_bytes{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.producer-handback"));}
        if !self.has_reserved_page() {
            return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.page-allocation-required"));
        }
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "A full retained source returns the exact page owner without allocating beyond admitted capacity.")]
    pub fn admit_page(&mut self, page: RetainedPackPage) -> Result<(), RetainedPackPage> {
        if self.preflight_page(page.len()).is_err() {
            return Err(page);
        }
        let len = page.len();
        match self.pages.push_reserved(page) {
            Ok(()) => {
                self.admitted_bytes += len;
                Ok(())
            }
            Err(page) => Err(page),
        }
    }

    pub fn seal(&mut self) -> Result<(), RetainedPackSourceFault> {
        if let Some(fault) = self.allocation_fault {
            return Err(fault);
        }
        if self.closed{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.source-closed"));}
        if self.cancelled{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::Canceled,"retained-pack.source-closed"));}
        self.sealed = true;
        Ok(())
    }

    pub fn request_cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn progress(&self) -> RetainedPackSourceProgress {
        RetainedPackSourceProgress {
            admitted_pages: self.pages.len(),
            admitted_bytes: self.admitted_bytes,
            reserved_pages: self.pages.capacity(),
            allocated_bytes: self.pages.allocated_bytes(),
            consumed_bytes: self.consumed,
            sealed: self.sealed,
        }
    }

    pub fn grant(&mut self) -> Result<Option<RetainedPackSourceEvent>, RetainedPackSourceFault> {
        if let Some(fault) = self.allocation_fault {
            return Err(fault);
        }
        if self.closed{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.source-closed"));}
        if self.cancelled{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::Canceled,"retained-pack.cancelled"));}
        if !self.sealed {
            return Ok(None);
        }
        if self.completed {
            return Ok(Some(RetainedPackSourceEvent::Complete { bytes: self.consumed, pages: self.pages.len() }));
        }
        while self.page < self.pages.len() && self.byte == self.pages.get(self.page).expect("admitted retained Pack page").len {
            self.page += 1;
            self.byte = 0;
        }
        if self.page == self.pages.len() {
            self.completed = true;
            return Ok(Some(RetainedPackSourceEvent::Complete { bytes: self.consumed, pages: self.pages.len() }));
        }
        let value = self.pages.get(self.page).expect("admitted retained Pack page").bytes[self.byte];
        let offset = self.consumed;
        self.byte += 1;
        self.consumed += 1;
        Ok(Some(RetainedPackSourceEvent::Byte { offset, value }))
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<RetainedPackCloseStep, PagedListError> {
        if maximum_items == 0 && maximum_bytes == 0 {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.cancelled = true;
        if !self.pages.is_empty() && maximum_items == 0 {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(page) = self.pages.pop() {
            self.admitted_bytes -= page.len();
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.pages.terminal_is_empty() {
            let step = self.pages.release_empty_page(maximum_bytes)?;
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: step.released_allocation_bytes });
        }
        self.closed = true;
        Ok(RetainedPackCloseStep::Complete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed && self.admitted_bytes == 0 && self.pages.terminal_is_empty()
    }

    #[cfg(test)]
    fn retained_page_ptr(&self, index: usize) -> Option<*const RetainedPackPage> {
        self.pages.backing_ptr(index)
    }

    #[cfg(test)]
    fn initialized_pages(&self) -> usize {
        self.pages.initialized_len()
    }
}

impl Drop for RetainedPackSourceCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained canonical pack source reached Drop before terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackSegmentHeader {
    pub offset: u64,
    pub kind: u8,
    pub flags: u8,
    pub stored_len: u64,
    pub raw_len: u64,
    pub payload_offset: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedPackSegmentEvent {
    Begin(RetainedPackSegmentHeader),
    RawByte { segment: RetainedPackSegmentHeader, index: u64, value: u8 },
    Complete { segment: RetainedPackSegmentHeader, wire_len: u64 },
    PackComplete { bytes: u64, segments: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// 🔢️ One admitted byte of the canonical unsigned wire integer.
pub enum RetainedVarintStep {
    Pending,
    Complete(u64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// 🚨️ Fixed native noncanonical-varint refusal retaining the original wire offset.
pub enum RetainedVarintFault{Overlong{offset:u64},NonMinimal{offset:u64}}

impl RetainedVarintFault{
    fn into_pack_refusal(self)->PackRefusal{match self{Self::Overlong{offset}=>PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"varint",offset,detail:"overlong retained varint"},Self::NonMinimal{..}=>PackRefusal::NonCanonical("non-minimal retained varint")}}
    pub fn into_catalog_fault(self,code:&'static str)->RetainedPackCatalogFault{let(offset,reason)=match self{Self::Overlong{offset}=>(offset,"overlong retained varint"),Self::NonMinimal{offset}=>(offset,"non-minimal retained varint")};RetainedPackCatalogFault{cause:RetainedPackCatalogCause::Value{kind:ValueRefusalKind::InvalidValue,reason},code,offset}}
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// 🔢️ Incrementally validates the existing canonical unsigned wire format.
pub struct RetainedVarintCursor {
    value: u64,
    bytes: u8,
}

impl RetainedVarintCursor {
    pub fn admit(&mut self, byte: u8, offset: u64) -> Result<RetainedVarintStep, RetainedVarintFault> {
        if self.bytes >= 10 || (self.bytes == 9 && ((byte & 0x80) != 0 || byte & 0x7f > 1)) {
            return Err(RetainedVarintFault::Overlong{offset:offset-self.bytes as u64});
        }
        let payload = (byte & 0x7f) as u64;
        self.value |= payload << (self.bytes as u32 * 7);
        self.bytes += 1;
        if byte & 0x80 != 0 {
            return Ok(RetainedVarintStep::Pending);
        }
        if self.bytes > 1 && payload == 0 {
            return Err(RetainedVarintFault::NonMinimal{offset:offset-(self.bytes-1) as u64});
        }
        Ok(RetainedVarintStep::Complete(self.value))
    }

    pub fn preview(self, byte: u8, offset: u64) -> Result<RetainedVarintStep, RetainedVarintFault> {
        let mut cursor = self;
        cursor.admit(byte, offset)
    }
}

/// 🚦️ Borrowed admission keeps stored faults intact and separates scheduling from lifecycle.
#[derive(Clone,Copy,Debug)]
pub enum RetainedPackSegmentAdmission<'a>{Fault(&'a PackRefusal),Closed,Complete,Pending,InflaterBackpressure}

#[derive(Debug)]
enum RetainedPackSegmentPhase {
    Header(usize),
    Kind,
    Flags,
    StoredLen(RetainedVarintCursor),
    RawLen(RetainedVarintCursor),
    Begin,
    Payload,
    Crc(usize),
    Trailer,
    Complete,
    Closed,
}

pub struct RetainedPackSegmentCursor {
    limits: PackLimits,
    pending: Option<RetainedPackSourceEvent>,
    phase: RetainedPackSegmentPhase,
    segment: RetainedPackSegmentHeader,
    payload_seen: u64,
    raw_seen: u64,
    crc: semio_framework_replication_codec::Crc32cCursor,
    payload_crc: semio_framework_replication_codec::Crc32cCursor,
    stored_crc: [u8; 4],
    segments: u64,
    total: u64,
    trailer_seen: usize,
    #[cfg(feature = "deflate")]
    inflater: Option<semio_framework_replication_codec::DeflateRetainedCursor>,
    maximum_inflater_allocation_bytes: usize,
    fault: Option<PackRefusal>,
    closed: bool,
}

impl RetainedPackSegmentCursor {
    /// 📏️ Observes one original segment frame or its retained decompressor close action.
    pub fn retirement_demands(&self) -> RetirementDemand {
        if self.terminal_is_empty() { return Default::default(); }
        if self.pending.is_some() { return RetirementDemand { copy_bytes: size_of::<RetainedPackSourceEvent>(), depth: 1, ..Default::default() }; }
        #[cfg(feature = "deflate")]
        if let Some(inflater) = self.inflater.as_ref() { return if inflater.terminal_is_empty() { RetirementDemand { copy_bytes: size_of::<semio_framework_replication_codec::DeflateRetainedCursor>(), depth: 1, ..Default::default() } } else { inflater.retirement_demands() }; }
        RetirementDemand { depth: 1, ..Default::default() }
    }
    pub fn try_new(limits: PackLimits, maximum_inflater_allocation_bytes: usize) -> Result<Self, PackRefusal> {
        if limits.max_segment_len == 0 || limits.max_file_len < (HEADER_SIZE + FOOTER_SIZE) as u64 {
            return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvalidValue,limit:"retained pack limits"});
        }
        if maximum_inflater_allocation_bytes > isize::MAX as usize {
            return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"retained inflater physical ceiling exceeds address space"});
        }
        Ok(Self {
            limits,
            pending: None,
            phase: RetainedPackSegmentPhase::Header(0),
            segment: RetainedPackSegmentHeader { offset: 0, kind: 0, flags: 0, stored_len: 0, raw_len: 0, payload_offset: 0 },
            payload_seen: 0,
            raw_seen: 0,
            crc: semio_framework_replication_codec::Crc32cCursor::new(),
            payload_crc: semio_framework_replication_codec::Crc32cCursor::new(),
            stored_crc: [0; 4],
            segments: 0,
            total: 0,
            trailer_seen: 0,
            #[cfg(feature = "deflate")]
            inflater: None,
            maximum_inflater_allocation_bytes,
            fault: None,
            closed: false,
        })
    }

    /// 🔁️ Replays a verified terminal source while retaining admitted inflater backing.
    pub fn replay(&mut self)->Result<(),PackRefusal>{
        if self.closed||self.fault.is_some()||self.pending.is_some()||!matches!(self.phase,RetainedPackSegmentPhase::Complete){return Err(PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvariantViolated,what:"retained-segment",offset:self.total,detail:"replay requires verified completion"})}
        self.phase=RetainedPackSegmentPhase::Header(0);self.payload_seen=0;self.raw_seen=0;self.crc=semio_framework_replication_codec::Crc32cCursor::new();self.payload_crc=semio_framework_replication_codec::Crc32cCursor::new();self.stored_crc=[0;4];self.segments=0;self.total=0;self.trailer_seen=0;Ok(())
    }

    pub fn preflight(&self)->Result<(),RetainedPackSegmentAdmission<'_>>{
        if let Some(fault)=self.fault.as_ref(){return Err(RetainedPackSegmentAdmission::Fault(fault));}
        if self.closed||matches!(self.phase,RetainedPackSegmentPhase::Closed){return Err(RetainedPackSegmentAdmission::Closed);}
        if matches!(self.phase,RetainedPackSegmentPhase::Complete){return Err(RetainedPackSegmentAdmission::Complete);}
        if self.pending.is_some(){return Err(RetainedPackSegmentAdmission::Pending);}
        #[cfg(feature = "deflate")]
        if matches!(self.phase, RetainedPackSegmentPhase::Payload)
            && self.segment.flags & 1 != 0
            && self.inflater.as_ref().is_some_and(|inflater| !inflater.can_admit())
        {
            return Err(RetainedPackSegmentAdmission::InflaterBackpressure);
        }
        Ok(())
    }

    pub fn admit(&mut self, event: RetainedPackSourceEvent) -> Result<(), RetainedPackSourceEvent> {
        if self.preflight().is_err() {
            return Err(event);
        }
        self.pending = Some(event);
        Ok(())
    }

    fn take_byte(&mut self) -> Result<Option<(u64, u8)>, PackRefusal> {
        match self.pending.take() {
            Some(RetainedPackSourceEvent::Byte { offset, value }) => {
                if offset!=self.total{return Err(PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvariantViolated,what:"retained-segment",offset,detail:"non-contiguous or over-limit source"});}
                if offset>=self.limits.max_file_len{return Err(PackRefusal::RetainedMalformed{kind:ValueRefusalKind::WorkLimit,what:"retained-segment",offset,detail:"non-contiguous or over-limit source"});}
                self.total += 1;
                Ok(Some((offset, value)))
            }
            Some(event @ RetainedPackSourceEvent::Complete { .. }) => {
                self.pending = Some(event);
                Ok(None)
            }
            None => Ok(None),
        }
    }

    fn begin_segment(&mut self) -> Result<(), PackRefusal> {
        if self.segment.stored_len > self.limits.max_segment_len || self.segment.raw_len > self.limits.max_segment_len {
            return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"segment length exceeds max_segment_len"});
        }
        if self.segment.kind == semio_framework_replication_codec::KIND_END && (self.segment.stored_len != 0 || self.segment.raw_len != 0) {
            return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvalidValue, what: "end-segment", offset: self.segment.offset, detail: "END payload must be empty" });
        }
        let codec = (self.segment.flags >> 1) & 0x07;
        if self.segment.flags & 0xf0 != 0 {
            return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvalidValue, what: "segment", offset: self.segment.offset + 1, detail: "reserved segment flags are set" });
        }
        if self.segment.flags & 1 == 0 {
            if codec != 0 || self.segment.raw_len != self.segment.stored_len {
                return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvalidValue, what: "segment", offset: self.segment.offset + 1, detail: "identity segment length or codec mismatch" });
            }
        } else if codec != 1 {
            return Err(PackRefusal::UnsupportedCodec(codec));
        } else {
            #[cfg(feature = "deflate")]
            {
                if let Some(inflater) = self.inflater.as_mut() {
                    inflater.reset(self.segment.raw_len, self.limits.max_segment_len)?;
                } else {
                    self.inflater = Some(semio_framework_replication_codec::DeflateRetainedCursor::try_new(
                        self.segment.raw_len,
                        self.limits.max_segment_len,
                        self.maximum_inflater_allocation_bytes,
                    )?);
                }
            }
            #[cfg(not(feature = "deflate"))]
            return Err(PackRefusal::UnsupportedCodec(codec));
        }
        Ok(())
    }

    pub fn grant(&mut self) -> Result<Option<RetainedPackSegmentEvent>, PackRefusal> {
        if let Some(fault) = &self.fault {
            return Err(fault.clone());
        }
        let result = self.grant_inner();
        match result {
            Ok(step) => Ok(step),
            Err(fault) => {
                self.fault = Some(fault);
                Err(self.fault.as_ref().expect("retained segment first fault").clone())
            }
        }
    }

    fn grant_inner(&mut self) -> Result<Option<RetainedPackSegmentEvent>, PackRefusal> {
        match self.phase {
            RetainedPackSegmentPhase::Header(index) => {
                let Some((_, _)) = self.take_byte()? else { return Ok(None) };
                self.phase = if index + 1 == HEADER_SIZE { RetainedPackSegmentPhase::Kind } else { RetainedPackSegmentPhase::Header(index + 1) };
                Ok(None)
            }
            RetainedPackSegmentPhase::Kind => {
                let Some((offset, value)) = self.take_byte()? else { return Ok(None) };
                self.segment = RetainedPackSegmentHeader { offset, kind: value, flags: 0, stored_len: 0, raw_len: 0, payload_offset: 0 };
                self.payload_seen = 0;
                self.raw_seen = 0;
                self.crc = semio_framework_replication_codec::Crc32cCursor::new();
                self.payload_crc = semio_framework_replication_codec::Crc32cCursor::new();
                self.crc.update_page(&[value]);
                self.phase = RetainedPackSegmentPhase::Flags;
                Ok(None)
            }
            RetainedPackSegmentPhase::Flags => {
                let Some((_, value)) = self.take_byte()? else { return Ok(None) };
                self.segment.flags = value;
                self.crc.update_page(&[value]);
                self.phase = RetainedPackSegmentPhase::StoredLen(RetainedVarintCursor::default());
                Ok(None)
            }
            RetainedPackSegmentPhase::StoredLen(mut cursor) => {
                let Some((offset, value)) = self.take_byte()? else { return Ok(None) };
                self.crc.update_page(&[value]);
                match cursor.admit(value, offset).map_err(RetainedVarintFault::into_pack_refusal)? {
                    RetainedVarintStep::Pending => self.phase = RetainedPackSegmentPhase::StoredLen(cursor),
                    RetainedVarintStep::Complete(value) => {
                        self.segment.stored_len = value;
                        if self.segment.flags & 1 == 0 {
                            self.segment.raw_len = value;
                            self.segment.payload_offset = self.total;
                            self.phase = RetainedPackSegmentPhase::Begin;
                        } else {
                            self.phase = RetainedPackSegmentPhase::RawLen(RetainedVarintCursor::default());
                        }
                    }
                }
                Ok(None)
            }
            RetainedPackSegmentPhase::RawLen(mut cursor) => {
                let Some((offset, value)) = self.take_byte()? else { return Ok(None) };
                self.crc.update_page(&[value]);
                match cursor.admit(value, offset).map_err(RetainedVarintFault::into_pack_refusal)? {
                    RetainedVarintStep::Pending => self.phase = RetainedPackSegmentPhase::RawLen(cursor),
                    RetainedVarintStep::Complete(value) => {
                        self.segment.raw_len = value;
                        self.segment.payload_offset = self.total;
                        self.phase = RetainedPackSegmentPhase::Begin;
                    }
                }
                Ok(None)
            }
            RetainedPackSegmentPhase::Begin => {
                self.begin_segment()?;
                self.phase = RetainedPackSegmentPhase::Payload;
                Ok(Some(RetainedPackSegmentEvent::Begin(self.segment)))
            }
            RetainedPackSegmentPhase::Payload => {
                if self.segment.flags & 1 == 0 {
                    if self.payload_seen == self.segment.stored_len {
                        self.phase = RetainedPackSegmentPhase::Crc(0);
                        return Ok(None);
                    }
                    let Some((_, value)) = self.take_byte()? else { return Ok(None) };
                    self.crc.update_page(&[value]);
                    let index = self.raw_seen;
                    self.payload_crc.update_page(&[value]);
                    self.payload_seen += 1;
                    self.raw_seen += 1;
                    return Ok(Some(RetainedPackSegmentEvent::RawByte { segment: self.segment, index, value }));
                }
                #[cfg(feature = "deflate")]
                {
                    if self.payload_seen < self.segment.stored_len && self.inflater.as_ref().expect("compressed segment has inflater").can_admit() {
                        let Some((_, value)) = self.take_byte()? else { return Ok(None) };
                        self.crc.update_page(&[value]);
                        self.payload_crc.update_page(&[value]);
                        self.inflater.as_mut().expect("compressed segment has inflater").admit_byte(value).expect("preflight established exact handback");
                        self.payload_seen += 1;
                    }
                    let inflater = self.inflater.as_mut().expect("compressed segment has inflater");
                    match inflater.grant(self.payload_seen == self.segment.stored_len)? {
                        semio_framework_replication_codec::DeflateRetainedStep::NeedInput => Ok(None),
                        semio_framework_replication_codec::DeflateRetainedStep::Byte(value) => {
                            let index = self.raw_seen;
                            self.raw_seen += 1;
                            Ok(Some(RetainedPackSegmentEvent::RawByte { segment: self.segment, index, value }))
                        }
                        semio_framework_replication_codec::DeflateRetainedStep::Complete => {
                            if self.raw_seen != self.segment.raw_len {
                                return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvalidValue, what: "segment", offset: self.segment.payload_offset, detail: "raw length mismatch" });
                            }
                            self.phase = RetainedPackSegmentPhase::Crc(0);
                            Ok(None)
                        }
                    }
                }
                #[cfg(not(feature = "deflate"))]
                Err(PackRefusal::UnsupportedCodec((self.segment.flags >> 1) & 0x07))
            }
            RetainedPackSegmentPhase::Crc(index) => {
                let Some((offset, value)) = self.take_byte()? else { return Ok(None) };
                self.stored_crc[index] = value;
                if index + 1 < 4 {
                    self.phase = RetainedPackSegmentPhase::Crc(index + 1);
                    return Ok(None);
                }
                if u32::from_le_bytes(self.stored_crc) != self.crc.finish() {
                    return Err(PackRefusal::ChecksumMismatch { segment: "segment", offset });
                }
                let wire_len = self.total - self.segment.offset;
                self.segments += 1;
                self.phase = if self.segment.kind == semio_framework_replication_codec::KIND_END { RetainedPackSegmentPhase::Trailer } else { RetainedPackSegmentPhase::Kind };
                Ok(Some(RetainedPackSegmentEvent::Complete { segment: self.segment, wire_len }))
            }
            RetainedPackSegmentPhase::Trailer => match self.pending.take() {
                Some(RetainedPackSourceEvent::Byte { offset, .. }) => {
                    if offset != self.total || self.total >= self.limits.max_file_len || self.trailer_seen == FOOTER_SIZE {
                        return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated, what: "retained-footer", offset, detail: "non-contiguous trailer" });
                    }
                    self.total += 1;
                    self.trailer_seen += 1;
                    Ok(None)
                }
                Some(RetainedPackSourceEvent::Complete { bytes, .. }) => {
                    if bytes != self.total || self.trailer_seen != FOOTER_SIZE {
                        return Err(PackRefusal::Truncated(self.total));
                    }
                    self.phase = RetainedPackSegmentPhase::Complete;
                    Ok(Some(RetainedPackSegmentEvent::PackComplete { bytes, segments: self.segments }))
                }
                None => Ok(None),
            },
            RetainedPackSegmentPhase::Complete => Ok(Some(RetainedPackSegmentEvent::PackComplete { bytes: self.total, segments: self.segments })),
            RetainedPackSegmentPhase::Closed => Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated, what: "retained-segment", offset: self.total, detail: "cursor is closed" }),
        }
    }

    pub fn next_allocation_bytes(&self) -> Option<usize> {
        #[cfg(feature = "deflate")]
        {
            self.inflater.as_ref().and_then(semio_framework_replication_codec::DeflateRetainedCursor::next_allocation_bytes)
        }
        #[cfg(not(feature = "deflate"))]
        {
            None
        }
    }

    pub fn reserve_allocation(&mut self, maximum_bytes: usize) -> Result<RetainedPackSourceAllocationStep, RetainedPackSourceAllocationError> {
        #[cfg(feature = "deflate")]
        if let Some(inflater) = self.inflater.as_mut() {
            return inflater
                .reserve_allocation(maximum_bytes)
                .map(|step| RetainedPackSourceAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
                .map_err(|error| RetainedPackSourceAllocationError{allocated_bytes:error.allocated_bytes,fault:RetainedPackSourceFault::allocation(error.kind,error.reason,error.allocated_bytes)});
        }
        Ok(RetainedPackSourceAllocationStep::default())
    }

    pub fn allocated_bytes(&self) -> usize {
        #[cfg(feature = "deflate")]
        {
            self.inflater.as_ref().map_or(0, semio_framework_replication_codec::DeflateRetainedCursor::allocated_bytes)
        }
        #[cfg(not(feature = "deflate"))]
        {
            0
        }
    }

    pub fn retained_inflater_ptr(&self) -> Option<usize> {
        #[cfg(feature = "deflate")]
        {
            self.inflater.as_ref().and_then(semio_framework_replication_codec::DeflateRetainedCursor::retained_history_ptr)
        }
        #[cfg(not(feature = "deflate"))]
        {
            None
        }
    }

    /// 🧾️ Borrows the verified stored payload checksum independently of framing bytes.
    pub fn verified_payload_crc32c(&self,segment:RetainedPackSegmentHeader)->Option<u32>{
        (self.fault.is_none()&&self.segment==segment&&matches!(self.phase,RetainedPackSegmentPhase::Kind|RetainedPackSegmentPhase::Trailer)).then(||self.payload_crc.finish())
    }

    pub fn next_release_allocation_bytes(&self) -> Option<usize> {
        if self.pending.is_some() {
            return None;
        }
        #[cfg(feature = "deflate")]
        {
            self.inflater.as_ref().and_then(semio_framework_replication_codec::DeflateRetainedCursor::next_release_allocation_bytes)
        }
        #[cfg(not(feature = "deflate"))]
        {
            None
        }
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> RetainedPackCloseStep {
        if self.closed {
            return RetainedPackCloseStep::Complete;
        }
        if maximum_items == 0 && maximum_bytes == 0 {
            return RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.pending.is_some() {
            if maximum_items == 0 {
                return RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.pending = None;
            return RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        #[cfg(feature = "deflate")]
        if let Some(inflater) = self.inflater.as_mut() {
            match inflater.close_step(maximum_items, maximum_bytes) {
                semio_framework_replication_codec::RetainedInflateCloseStep::Pending { released_items, released_bytes } => {
                    return RetainedPackCloseStep::Pending { released_items, released_bytes };
                }
                semio_framework_replication_codec::RetainedInflateCloseStep::Complete => {}
            }
            drop(self.inflater.take());
            return RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if maximum_items == 0 {
            return RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.phase = RetainedPackSegmentPhase::Closed;
        self.closed = true;
        RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed && self.pending.is_none() && {
            #[cfg(feature = "deflate")]
            {
                self.inflater.is_none()
            }
            #[cfg(not(feature = "deflate"))]
            {
                true
            }
        }
    }
}

impl Drop for RetainedPackSegmentCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained segment cursor reached Drop before terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// 🗂️ Retains the actual borrowed producer cause independently of Catalog context.
pub enum RetainedPackCatalogCause{
    Value{kind:ValueRefusalKind,reason:&'static str},
    Paged(PagedListError),
    Allocation(PagedListAllocationError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// 🧭️ Carries source metadata alongside the Catalog context and byte offset.
pub struct RetainedPackCatalogFault {
    pub cause:RetainedPackCatalogCause,
    pub code: &'static str,
    pub offset: u64,
}

impl RetainedPackCatalogFault{
    /// 🪪️ Requires the producer to author the machine kind at its precise refusal site.
    pub const fn refusal(kind:ValueRefusalKind,code:&'static str,offset:u64)->Self{Self{cause:RetainedPackCatalogCause::Value{kind,reason:code},code,offset}}
    /// 📋️ Keeps the lower borrowed kind and reason without classifying its prose.
    pub const fn from_paged_refusal(error:PagedListError,code:&'static str,offset:u64)->Self{Self{cause:RetainedPackCatalogCause::Paged(error),code,offset}}
    /// 🧱️ Keeps the actual lower allocation witness with its original cause.
    pub const fn from_paged_allocation(error:PagedListAllocationError,code:&'static str,offset:u64)->Self{Self{cause:RetainedPackCatalogCause::Allocation(error),code,offset}}
    /// 🔎️ Projects the closed source cause without consulting its Catalog code.
    pub const fn kind(&self)->ValueRefusalKind{match self.cause{RetainedPackCatalogCause::Value{kind,..}=>kind,RetainedPackCatalogCause::Paged(error)=>retained_paged_kind(error.kind),RetainedPackCatalogCause::Allocation(error)=>retained_paged_kind(error.kind)}}
    /// 📤️ Projects into the canonical Pack owner while retaining source metadata.
    pub const fn into_pack_refusal(self,what:&'static str)->PackRefusal{match self.cause{
        RetainedPackCatalogCause::Value{kind,reason}=>PackRefusal::RetainedMalformed{kind,what,offset:self.offset,detail:reason},
        RetainedPackCatalogCause::Paged(error)=>PackRefusal::from_paged_refusal(error,what,self.offset),
        RetainedPackCatalogCause::Allocation(error)=>PackRefusal::from_paged_allocation(error,what,self.offset),
    }}
}

const fn retained_paged_kind(kind:PagedListRefusalKind)->ValueRefusalKind{match kind{PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,PagedListRefusalKind::InvariantViolated=>ValueRefusalKind::InvariantViolated}}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedPackAnchorPhase {
    Collect,
    VerifyHeader(usize),
    VerifyFooter(usize),
    Ready,
    Closed,
}

pub struct RetainedPackAnchorCursor {
    header: [u8; HEADER_SIZE],
    header_len: usize,
    footer_ring: [u8; FOOTER_SIZE],
    footer_len: usize,
    footer_head: usize,
    total: u64,
    ordered_footer: [u8; FOOTER_SIZE],
    header_crc: semio_framework_replication_codec::Crc32cCursor,
    footer_crc: semio_framework_replication_codec::Crc32cCursor,
    phase: RetainedPackAnchorPhase,
    value: Option<Superblock>,
    handed_back: bool,
    fault: Option<PackRefusal>,
}

impl RetainedPackAnchorCursor {
    pub fn new() -> Self {
        Self {
            header: [0; HEADER_SIZE],
            header_len: 0,
            footer_ring: [0; FOOTER_SIZE],
            footer_len: 0,
            footer_head: 0,
            total: 0,
            ordered_footer: [0; FOOTER_SIZE],
            header_crc: semio_framework_replication_codec::Crc32cCursor::new(),
            footer_crc: semio_framework_replication_codec::Crc32cCursor::new(),
            phase: RetainedPackAnchorPhase::Collect,
            value: None,
            handed_back: false,
            fault: None,
        }
    }

    pub fn grant(&mut self, event: Option<RetainedPackSourceEvent>) -> Result<bool, PackRefusal> {
        if let Some(fault) = &self.fault {
            return Err(fault.clone());
        }
        let result = self.grant_inner(event);
        match result {
            Ok(step) => Ok(step),
            Err(fault) => {
                self.fault = Some(fault);
                Err(self.fault.as_ref().expect("retained anchor first fault").clone())
            }
        }
    }

    fn grant_inner(&mut self, event: Option<RetainedPackSourceEvent>) -> Result<bool, PackRefusal> {
        match self.phase {
            RetainedPackAnchorPhase::Collect => match event {
                Some(RetainedPackSourceEvent::Byte { offset, value }) => {
                    if offset != self.total {
                        return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated, what: "retained-anchor", offset, detail: "non-contiguous source event" });
                    }
                    if self.header_len < HEADER_SIZE {
                        self.header[self.header_len] = value;
                        self.header_len += 1;
                    }
                    self.footer_ring[self.footer_head] = value;
                    self.footer_head = (self.footer_head + 1) % FOOTER_SIZE;
                    self.footer_len = (self.footer_len + 1).min(FOOTER_SIZE);
                    self.total += 1;
                    Ok(false)
                }
                Some(RetainedPackSourceEvent::Complete { bytes, .. }) => {
                    if bytes != self.total || self.header_len != HEADER_SIZE || self.footer_len != FOOTER_SIZE {
                        return Err(PackRefusal::Truncated(self.total));
                    }
                    for index in 0..FOOTER_SIZE {
                        self.ordered_footer[index] = self.footer_ring[(self.footer_head + index) % FOOTER_SIZE];
                    }
                    self.phase = RetainedPackAnchorPhase::VerifyHeader(0);
                    Ok(false)
                }
                None => Ok(false),
            },
            RetainedPackAnchorPhase::VerifyHeader(index) => {
                if event.is_some() {
                    return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated, what: "retained-anchor", offset: self.total, detail: "source replay after completion" });
                }
                if index < 20 {
                    self.header_crc.update_page(&self.header[index..index + 1]);
                    self.phase = RetainedPackAnchorPhase::VerifyHeader(index + 1);
                    return Ok(false);
                }
                let stored = u32::from_le_bytes(self.header[20..24].try_into().expect("fixed header crc"));
                if stored != self.header_crc.finish() {
                    return Err(PackRefusal::ChecksumMismatch { segment: "header", offset: 20 });
                }
                if self.header[..8] != MAGIC {
                    return Err(PackRefusal::BadMagic);
                }
                if self.header[24..32] != [0; 8] {
                    return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvalidValue, what: "header", offset: 24, detail: "reserved header bytes are nonzero" });
                }
                self.phase = RetainedPackAnchorPhase::VerifyFooter(0);
                Ok(false)
            }
            RetainedPackAnchorPhase::VerifyFooter(index) => {
                if event.is_some() {
                    return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated, what: "retained-anchor", offset: self.total, detail: "source replay after completion" });
                }
                if index < 80 {
                    self.footer_crc.update_page(&self.ordered_footer[index..index + 1]);
                    self.phase = RetainedPackAnchorPhase::VerifyFooter(index + 1);
                    return Ok(false);
                }
                let stored = u32::from_le_bytes(self.ordered_footer[80..84].try_into().expect("fixed footer crc"));
                if stored != self.footer_crc.finish() {
                    return Err(PackRefusal::ChecksumMismatch { segment: "footer", offset: self.total - 4 });
                }
                if self.ordered_footer[..8] != FOOTER_MAGIC {
                    return Err(PackRefusal::BadMagic);
                }
                let version_major = u16::from_le_bytes(self.header[8..10].try_into().expect("header major"));
                let version_minor = u16::from_le_bytes(self.header[10..12].try_into().expect("header minor"));
                let required_flags = u32::from_le_bytes(self.header[12..16].try_into().expect("header flags"));
                let optional_flags = u32::from_le_bytes(self.header[16..20].try_into().expect("header optional flags"));
                if version_major != FORMAT_VERSION_MAJOR {
                    return Err(PackRefusal::UnsupportedVersion { major: version_major, minor: version_minor });
                }
                let unknown = required_flags & !REQUIRED_KNOWN_MASK;
                if unknown != 0 {
                    return Err(PackRefusal::UnknownRequiredFlags(unknown));
                }
                let footer_major = u16::from_le_bytes(self.ordered_footer[8..10].try_into().expect("footer major"));
                let footer_minor = u16::from_le_bytes(self.ordered_footer[10..12].try_into().expect("footer minor"));
                let footer_flags = u32::from_le_bytes(self.ordered_footer[12..16].try_into().expect("footer flags"));
                let file_len = u64::from_le_bytes(self.ordered_footer[32..40].try_into().expect("footer length"));
                let manifest_offset = u64::from_le_bytes(self.ordered_footer[16..24].try_into().expect("manifest offset"));
                let manifest_len = u64::from_le_bytes(self.ordered_footer[24..32].try_into().expect("manifest length"));
                let manifest_end = manifest_offset.checked_add(manifest_len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"manifest span overflow"})?;
                if footer_major != version_major || footer_minor != version_minor || footer_flags != required_flags || file_len != self.total {
                    return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvalidValue, what: "footer", offset: self.total - FOOTER_SIZE as u64, detail: "anchor identity mismatch" });
                }
                if manifest_offset < HEADER_SIZE as u64 || manifest_len == 0 || manifest_end > self.total - FOOTER_SIZE as u64 {
                    return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvalidValue, what: "footer", offset: self.total - FOOTER_SIZE as u64, detail: "manifest span is outside the segment area" });
                }
                let mut content_hash = [0; 32];
                content_hash.copy_from_slice(&self.ordered_footer[40..72]);
                self.value = Some(Superblock {
                    header: Header { version_major, version_minor, required_flags, optional_flags },
                    footer: Footer {
                        version_major: footer_major,
                        version_minor: footer_minor,
                        required_flags: footer_flags,
                        manifest_offset,
                        manifest_len,
                        file_len,
                        content_hash: ContentHash(content_hash),
                        prev_footer_offset: u64::from_le_bytes(self.ordered_footer[72..80].try_into().expect("previous footer")),
                    },
                });
                self.phase = RetainedPackAnchorPhase::Ready;
                Ok(true)
            }
            RetainedPackAnchorPhase::Ready => Ok(true),
            RetainedPackAnchorPhase::Closed => Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated, what: "retained-anchor", offset: self.total, detail: "anchor is closed" }),
        }
    }

    pub fn take(&mut self) -> Option<Superblock> {
        if self.phase != RetainedPackAnchorPhase::Ready || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.value.take()
    }

    pub fn close_step(&mut self) -> RetainedPackCloseStep {
        self.value = None;
        self.phase = RetainedPackAnchorPhase::Closed;
        self.handed_back = true;
        RetainedPackCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.handed_back && self.value.is_none() && matches!(self.phase, RetainedPackAnchorPhase::Ready | RetainedPackAnchorPhase::Closed)
    }
}

impl Default for RetainedPackAnchorCursor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RetainedPackAnchorCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained pack anchors reached Drop before handback or close");
    }
}
//#endregion 🔖️RetainedCanonicalSource

#[cfg(test)]
#[path = "../🧪️tests/🔬️retained-pack-source-laws/🦀️.rs"]
mod retained_pack_source_laws;

#[cfg(test)]
#[path = "../🧭️producer/📥️source/🧪️tests/🦀️.rs"]
mod source_producer_authority_tests;
