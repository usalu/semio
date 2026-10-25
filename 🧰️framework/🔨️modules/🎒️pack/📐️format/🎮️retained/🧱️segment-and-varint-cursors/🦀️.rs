//! 🧱️ Segment and varint cursors of the retained Pack source: the segment header and events, the varint cursor and the segment cursor with its drop guard. See [`super`].

use super::*;

//#region 🔖️RetainedCanonicalSource
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
    pub(super) fn into_pack_refusal(self)->PackRefusal{match self{Self::Overlong{offset}=>PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"varint",offset,detail:"overlong retained varint"},Self::NonMinimal{..}=>PackRefusal::NonCanonical("non-minimal retained varint")}}
    pub fn into_catalog_fault(self,code:&'static str)->RetainedPackCatalogFault{let(offset,reason)=match self{Self::Overlong{offset}=>(offset,"overlong retained varint"),Self::NonMinimal{offset}=>(offset,"non-minimal retained varint")};RetainedPackCatalogFault{cause:RetainedPackCatalogCause::Value{kind:ValueRefusalKind::InvalidValue,reason},code,offset}}
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// 🖱️ Incrementally validates the existing canonical unsigned wire format.
pub struct RetainedVarintCursor {
    pub(super) value: u64,
    pub(super) bytes: u8,
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
pub(super) enum RetainedPackSegmentPhase {
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
    pub(super) limits: PackLimits,
    pub(super) pending: Option<RetainedPackSourceEvent>,
    pub(super) phase: RetainedPackSegmentPhase,
    pub(super) segment: RetainedPackSegmentHeader,
    payload_seen: u64,
    raw_seen: u64,
    crc: semio_framework_replication_codec::Crc32cCursor,
    payload_crc: semio_framework_replication_codec::Crc32cCursor,
    stored_crc: [u8; 4],
    segments: u64,
    pub(super) total: u64,
    trailer_seen: usize,
    #[cfg(feature = "deflate")]
    pub(super) inflater: Option<semio_framework_replication_codec::DeflateRetainedCursor>,
    maximum_inflater_allocation_bytes: usize,
    pub(super) fault: Option<PackRefusal>,
    pub(super) closed: bool,
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

    pub(super) fn grant_inner(&mut self) -> Result<Option<RetainedPackSegmentEvent>, PackRefusal> {
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
//#endregion 🔖️RetainedCanonicalSource
