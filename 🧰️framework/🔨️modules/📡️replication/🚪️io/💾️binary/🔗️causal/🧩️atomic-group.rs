//! 🧩️ Reconstitutes a distinct large atomic document mutation message from bounded backbone frames.

use crate::CausalPayload;
use semio_framework_hash::Sha256;
use std::mem::{ManuallyDrop, size_of};
use semio_framework_value::{
    OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneProgress, RetainedCloneStep,
    RetirementDemand, RetirementPollExit, ValueError, ValueRefusalKind,
    paged::ByteReadSource, retained_clone::retained_clone_grant_admits, retirement::controlled::ControlledRetirement,
};

const TAG: [u8; 4] = [0x80, 0, b'G', 1];
pub const PREPARED_ATOMIC_EDIT_TAG: [u8; 4] = [0x80, 0, b'G', 0];
pub const ATOMIC_BACKBONE_TOOL: &str = "original-media";
pub const ATOMIC_EDIT_PREFIX_BYTES: usize = 4 + 2 + super::DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES;
pub const ATOMIC_BACKBONE_PART_BYTES: usize = 64 * 1024;
pub const ATOMIC_BACKBONE_MAXIMUM_BYTES: usize = 2 * 1024 * 1024;
pub const ATOMIC_BACKBONE_HEADER_BYTES: usize = 42;
const PARTS: usize = ATOMIC_BACKBONE_MAXIMUM_BYTES / ATOMIC_BACKBONE_PART_BYTES;

pub fn is_atomic_backbone_tool(tool: &str) -> bool {
    tool.split_once('#').is_some_and(|(app, operation)| !app.is_empty() && operation == ATOMIC_BACKBONE_TOOL)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct AtomicBackboneKey {
    pub digest: [u8; 32],
    pub extent: u32,
}

impl AtomicBackboneKey {
    pub fn count(self) -> usize { (self.extent as usize).div_ceil(ATOMIC_BACKBONE_PART_BYTES) }

    fn part_bytes(self, ordinal: usize) -> Result<usize, ValueError> {
        if self.extent == 0 || self.extent as usize > ATOMIC_BACKBONE_MAXIMUM_BYTES || ordinal >= self.count() {
            return Err(refusal("atomic backbone part ordinal or logical extent is invalid"));
        }
        Ok((self.extent as usize - ordinal * ATOMIC_BACKBONE_PART_BYTES).min(ATOMIC_BACKBONE_PART_BYTES))
    }
}

#[derive(Clone, Copy)]
pub struct AtomicBackbonePart {
    pub key: AtomicBackboneKey,
    pub ordinal: u16,
}

impl AtomicBackbonePart {
    pub fn matches(frame: &[u8]) -> bool { frame.starts_with(&TAG) }

    pub fn matches_paged(frame: &(impl ByteReadSource + ?Sized)) -> bool {
        let mut tag = [0; 4];
        frame.read_bytes(0, &mut tag) == tag.len() && tag == TAG
    }

    /// 📏️ Binds the fixed header to the only permitted extent for its frame ordinal.
    pub fn encode_header(self) -> Result<[u8; ATOMIC_BACKBONE_HEADER_BYTES], ValueError> {
        self.key.part_bytes(usize::from(self.ordinal))?;
        let mut header = [0; ATOMIC_BACKBONE_HEADER_BYTES];
        header[..4].copy_from_slice(&TAG);
        header[4..36].copy_from_slice(&self.key.digest);
        header[36..40].copy_from_slice(&self.key.extent.to_le_bytes());
        header[40..].copy_from_slice(&self.ordinal.to_le_bytes());
        Ok(header)
    }

    pub fn parse(frame: &[u8]) -> Result<(Self, &[u8]), ValueError> {
        let part = Self::parse_paged(frame)?;
        Ok((part, &frame[ATOMIC_BACKBONE_HEADER_BYTES..]))
    }

    pub fn parse_paged(frame: &(impl ByteReadSource + ?Sized)) -> Result<Self, ValueError> {
        let mut header = [0; ATOMIC_BACKBONE_HEADER_BYTES];
        if frame.read_bytes(0, &mut header) != header.len() { return Err(refusal("atomic backbone frame is truncated")); }
        if header[..4] != TAG { return Err(refusal("atomic backbone frame has a foreign version")); }
        let key = AtomicBackboneKey { digest: header[4..36].try_into().unwrap(), extent: u32::from_le_bytes(header[36..40].try_into().unwrap()) };
        let ordinal = u16::from_le_bytes(header[40..].try_into().unwrap());
        if key.part_bytes(usize::from(ordinal))? + ATOMIC_BACKBONE_HEADER_BYTES != frame.byte_len() {
            return Err(refusal("atomic backbone frame has a noncanonical byte length"));
        }
        Ok(Self { key, ordinal })
    }
}

/// 📤️ Opens fixed physical pages only from the immutable prepared logical Store message.
pub struct AtomicBackboneSender {
    source: usize,
    key: AtomicBackboneKey,
    digest: Sha256,
    hashed: usize,
    ordinal: usize,
    frame: ManuallyDrop<Option<Vec<u8>>>,
    closing: bool,
}

impl AtomicBackboneSender {
    pub fn matches_prepared(input: &(impl ByteReadSource + ?Sized)) -> bool {
        let mut tag = [0; 4];
        input.read_bytes(0, &mut tag) == tag.len() && tag == PREPARED_ATOMIC_EDIT_TAG
    }

    pub fn edit_prefix(edit_id: &str, output: &mut [u8; ATOMIC_EDIT_PREFIX_BYTES]) -> Result<usize, ValueError> {
        if edit_id.is_empty() || edit_id.len() > super::DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES {
            return Err(refusal("atomic backbone edit identity is outside the precise prefix bound"));
        }
        output[..4].copy_from_slice(&PREPARED_ATOMIC_EDIT_TAG);
        let len = edit_id.len();
        let width = if len < 128 {
            output[4] = len as u8;
            1
        } else {
            output[4] = (len as u8 & 127) | 128;
            output[5] = (len >> 7) as u8;
            2
        };
        let start = 4 + width;
        output[start..start + len].copy_from_slice(edit_id.as_bytes());
        Ok(start + len)
    }

    pub fn begin<S: ByteReadSource>(input: &S) -> Result<Self, ValueError> {
        if !Self::matches_prepared(input) { return Err(refusal("atomic backbone sender lacks a prepared edit")); }
        let extent = u32::try_from(input.byte_len()).map_err(|_| refusal("atomic backbone message extent is not addressable"))?;
        let key = AtomicBackboneKey { extent, digest: [0; 32] };
        key.part_bytes(0)?;
        Ok(Self { source: input as *const S as usize, key, digest: Sha256::new(), hashed: 0, ordinal: 0,
            frame: ManuallyDrop::new(None), closing: false })
    }

    pub fn terminal_is_empty(&self) -> bool { self.frame.is_none() && (self.closing || self.ordinal == self.key.count()) }

    pub fn next_demand(&self) -> Result<RetirementDemand, ValueError> {
        if self.closing { return Err(refusal("atomic backbone sender is closing")); }
        if self.hashed != self.key.extent as usize {
            return Ok(RetirementDemand { copy_bytes: (self.key.extent as usize - self.hashed).min(4096), depth: 1, ..Default::default() });
        }
        if self.terminal_is_empty() { return Ok(Default::default()); }
        let extent = self.key.part_bytes(self.ordinal)? + ATOMIC_BACKBONE_HEADER_BYTES;
        Ok(match self.frame.as_ref() {
            None => RetirementDemand { copy_bytes: ATOMIC_BACKBONE_HEADER_BYTES + size_of::<Option<Vec<u8>>>(),
                capacity_bytes: extent, depth: 1, ..Default::default() },
            Some(frame) if frame.len() < extent => RetirementDemand { copy_bytes: 2 * (extent - frame.len()).min(2048),
                depth: 1, ..Default::default() },
            Some(_) => RetirementDemand { copy_bytes: size_of::<Option<Vec<u8>>>() + size_of::<usize>(),
                depth: 1, ..Default::default() },
        })
    }

    pub fn advance<S: ByteReadSource>(
        &mut self, input: &S, output: &mut Option<Vec<u8>>, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        if input as *const S as usize != self.source || input.byte_len() != self.key.extent as usize {
            return Err(refusal("atomic backbone sender changed its prepared original message"));
        }
        if output.is_some() || self.terminal_is_empty() { return Ok(Default::default()); }
        let demand = self.next_demand()?;
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(demand.copy_bytes as u64)? {
            return Ok(Default::default());
        }
        if self.hashed != self.key.extent as usize {
            let mut scratch = [0; 4096];
            let count = input.read_bytes(self.hashed, &mut scratch[..demand.copy_bytes]);
            if count != demand.copy_bytes { return Err(refusal("atomic backbone sender lost an admitted source page")); }
            self.digest.update(&scratch[..count]);
            self.hashed += count;
            if self.hashed == self.key.extent as usize { self.key.digest = self.digest.clone().finalize(); }
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() });
        }
        let mut retained_capacity_bytes = 0;
        if let Some(frame) = self.frame.as_mut() {
            if frame.len() == self.key.part_bytes(self.ordinal)? + ATOMIC_BACKBONE_HEADER_BYTES {
                self.ordinal += 1;
                *output = self.frame.take();
            } else {
                let count = demand.copy_bytes / 2;
                let offset = self.ordinal * ATOMIC_BACKBONE_PART_BYTES + frame.len() - ATOMIC_BACKBONE_HEADER_BYTES;
                let mut scratch = [0; 2048];
                if input.read_bytes(offset, &mut scratch[..count]) != count {
                    return Err(refusal("atomic backbone sender lost an admitted frame body"));
                }
                frame.extend_from_slice(&scratch[..count]);
            }
        } else {
            let part = AtomicBackbonePart { key: self.key, ordinal: self.ordinal as u16 };
            let mut frame = Vec::new();
            frame.try_reserve_exact(demand.capacity_bytes).map_err(|_| refusal("atomic backbone frame allocation was refused"))?;
            frame.extend_from_slice(&part.encode_header()?);
            retained_capacity_bytes = frame.capacity();
            *self.frame = Some(frame);
        }
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes, ..Default::default() })
    }

    pub fn close_demand(&self) -> RetirementDemand {
        if self.terminal_is_empty() { return Default::default(); }
        RetirementDemand { copy_bytes: size_of::<Option<Vec<u8>>>() + size_of::<bool>(),
            release_bytes: self.frame.as_ref().map_or(0, Vec::capacity), depth: 1, ..Default::default() }
    }

    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        let demand = self.close_demand();
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? { return Ok(Default::default()); }
        drop(self.frame.take());
        self.closing = true;
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() })
    }
}

impl Drop for AtomicBackboneSender {
    fn drop(&mut self) {
        assert!(self.frame.is_none() || std::thread::panicking(), "atomic backbone sender still owns its paid frame");
    }
}

/// 🪺️ Keeps admitted message owners private until the complete canonical batch can be verified.
pub struct AtomicBackboneSections {
    key: AtomicBackboneKey,
    pieces: [Option<CausalPayload>; PARTS],
    received: usize,
    closing: bool,
    closing_part: Option<ControlledRetirement<CausalPayload>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtomicPartStatus { Pending, Accepted, Complete }

impl AtomicBackboneSections {
    pub fn birth_demand() -> RetirementDemand { RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() } }

    pub fn new(key: AtomicBackboneKey) -> Result<Self, ValueError> {
        key.part_bytes(0)?;
        Ok(Self { key, pieces: std::array::from_fn(|_| None), received: 0, closing: false, closing_part: None })
    }

    /// 📥️ Moves a separately funded original part into the exact immutable batch owner.
    pub fn admit(
        &mut self, part: AtomicBackbonePart, input: &mut Option<CausalPayload>, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<(AtomicPartStatus, RetainedCloneProgress), ValueError> {
        let bytes = input.as_ref().ok_or_else(|| refusal("atomic backbone has no original frame owner"))?;
        let origin = AtomicBackbonePart::parse_paged(bytes)?;
        let ordinal = usize::from(part.ordinal);
        if self.closing || part.key != self.key || origin.key != part.key || origin.ordinal != part.ordinal {
            return Err(refusal("atomic backbone part changed its authenticated group or extent"));
        }
        if self.pieces[ordinal].is_some() { return Err(refusal("atomic backbone part ordinal is already retained")); }
        let copy_bytes = 2 * size_of::<Option<CausalPayload>>() + size_of::<usize>();
        let quote = RetirementDemand { copy_bytes, depth: 1, ..Default::default() };
        if !retained_clone_grant_admits(effect.grant(), quote) || !effect.performed(1)? {
            return Ok((AtomicPartStatus::Pending, Default::default()));
        }
        self.pieces[ordinal] = input.take();
        self.received += 1;
        let status = if self.received == self.key.count() { AtomicPartStatus::Complete } else { AtomicPartStatus::Accepted };
        Ok((status, RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() }))
    }

    /// 🗝️ Binds a seal cursor to this exact complete receiver in its stable runtime allocation.
    pub fn complete(&self) -> Option<AtomicBackboneDigest> {
        (!self.closing && self.received == self.key.count()).then(|| AtomicBackboneDigest {
            input: self as *const Self as usize, key: self.key, cursor: 0, hash: Sha256::new(),
        })
    }

    pub fn terminal_is_empty(&self) -> bool { self.received == 0 && self.closing_part.is_none() }

    /// ♻️ Retires one independent paged frame owner at a time before releasing the receiver.
    pub fn close(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        if let Some(part) = self.closing_part.as_mut() {
            if !part.terminal_is_empty() {
                return part.step(body, effect);
            }
        }
        let result = self.close_next(effect).map(RetainedCloneStep::Progress).map_err(Into::into);
        RetirementPollExit::settle(effect, result)
    }

    fn close_next(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        let copy_bytes = size_of::<Option<CausalPayload>>() + size_of::<Option<ControlledRetirement<CausalPayload>>>() + size_of::<usize>() + 1;
        let demand = RetirementDemand { copy_bytes, depth: 1, ..Default::default() };
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? { return Ok(Default::default()); }
        self.closing = true;
        if self.closing_part.is_some() {
            self.closing_part = None;
        } else {
            let part = self.pieces.iter_mut().find_map(Option::take).expect("received atomic frames are still owned");
            self.closing_part = Some(ControlledRetirement::new(part).map_err(|(error, _)| error)?);
            self.received -= 1;
        }
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() })
    }
}

/// 🪪️ Verifies the exact admitted pages incrementally before releasing a private decoding loan.
pub struct AtomicBackboneDigest {
    input: usize,
    key: AtomicBackboneKey,
    hash: Sha256,
    cursor: usize,
}

impl AtomicBackboneDigest {
    pub fn advance<'a>(
        &mut self, input: &'a AtomicBackboneSections, work: usize, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<(Option<VerifiedAtomicBackbone<'a>>, RetainedCloneProgress), ValueError> {
        if input as *const AtomicBackboneSections as usize != self.input || input.key != self.key ||
            input.closing || input.received != input.key.count()
        {
            return Err(refusal("atomic backbone seal changed its exact original receiver"));
        }
        let mut scratch = [0; 4096];
        let count = (self.key.extent as usize - self.cursor).min(work).min(scratch.len());
        if count != 0 {
            let quote = RetirementDemand { copy_bytes: count, depth: 1, ..Default::default() };
            if !retained_clone_grant_admits(effect.grant(), quote) || !effect.performed(count as u64)? { return Ok((None, Default::default())); }
            let source = VerifiedAtomicBackbone { extent: self.key.extent as usize, pieces: &input.pieces };
            if source.read_bytes(self.cursor, &mut scratch[..count]) != count { return Err(refusal("atomic backbone retained parts changed during sealing")); }
            self.hash.update(&scratch[..count]);
            self.cursor += count;
        }
        let progress = RetainedCloneProgress { copied_items: usize::from(count != 0), copied_bytes: count, ..Default::default() };
        if self.cursor != self.key.extent as usize { return Ok((None, progress)); }
        if self.hash.clone().finalize() != self.key.digest {
            return Err(refusal("atomic backbone message digest differs").with_retained_progress(progress));
        }
        Ok((Some(VerifiedAtomicBackbone { extent: self.key.extent as usize, pieces: &input.pieces }), progress))
    }
}

/// 🔐️ Borrows all and only the fully authenticated canonical group bytes.
pub struct VerifiedAtomicBackbone<'a> {
    extent: usize,
    pieces: &'a [Option<CausalPayload>; PARTS],
}

impl ByteReadSource for VerifiedAtomicBackbone<'_> {
    fn byte_len(&self) -> usize { self.extent }
    fn byte_at(&self, index: usize) -> Option<u8> {
        if index >= self.extent { return None; }
        self.pieces.get(index / ATOMIC_BACKBONE_PART_BYTES)?.as_ref()?
            .byte_at(ATOMIC_BACKBONE_HEADER_BYTES + index % ATOMIC_BACKBONE_PART_BYTES)
    }
    fn read_bytes(&self, offset: usize, output: &mut [u8]) -> usize {
        let available = self.byte_len().saturating_sub(offset).min(output.len());
        let mut written = 0;
        while written < available {
            let offset = offset + written;
            let piece = self.pieces[offset / ATOMIC_BACKBONE_PART_BYTES].as_ref().expect("verified backbone source part");
            let start = ATOMIC_BACKBONE_HEADER_BYTES + offset % ATOMIC_BACKBONE_PART_BYTES;
            let count = (available - written).min(piece.len() - start);
            let read = piece.read_bytes(start, &mut output[written..written + count]);
            written += read;
            if read != count { break; }
        }
        written
    }
}

fn refusal(reason: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, reason) }
