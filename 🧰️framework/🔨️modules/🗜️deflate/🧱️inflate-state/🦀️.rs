//! 🧱️ Inflate state of the deflate codec: the outcome, the phase, the retained allocation and close records, the retained history and the inflater record. See [`super`].

use super::*;

//#region 🔖️Inflate
/// 🚦️ What one `Inflater::advance` call produced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InflateOutcome {
    NeedInput,
    Wrote(u8),
    Done,
}

pub(super) enum Phase {
    BlockHeader,
    StoredLen,
    StoredCopy { remaining: u16 },
    DynamicCounts,
    DynamicClcLengths { read: usize, hclen: usize, hlit: usize, hdist: usize, clc_lengths: [u8; 19] },
    DynamicCodeLengths { clc: Huffman, hlit: usize, hdist: usize, lengths: [u8; 318], len: usize },
    DynamicRepeatPrev { clc: Huffman, hlit: usize, hdist: usize, lengths: [u8; 318], len: usize, prev: u8 },
    DynamicRepeatZero { clc: Huffman, hlit: usize, hdist: usize, lengths: [u8; 318], len: usize, bits: u32, base: u32 },
    DecodeSymbol { lit_len: Huffman, dist: Huffman },
    LengthExtra { lit_len: Huffman, dist: Huffman, base_len: u16, extra: u8 },
    DecodeDistanceSymbol { lit_len: Huffman, dist: Huffman, length: u16 },
    DistanceExtra { lit_len: Huffman, dist: Huffman, length: u16, base_dist: u32, extra: u8 },
    CopyMatch { lit_len: Huffman, dist: Huffman, distance: u32, remaining: u16 },
    Done,
}

pub const RETAINED_INFLATE_WINDOW_BYTES: usize = 32 * 1024;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedInflateAllocationStep {
    pub progressed: bool,
    pub allocated_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedInflateAllocationError {
    pub kind: ValueRefusalKind,
    pub allocated_bytes: usize,
    pub reason: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedInflateCloseStep {
    Pending { released_items: usize, released_bytes: usize },
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedInflateProgress {
    pub history_bytes: usize,
    pub allocated_bytes: usize,
    pub maximum_distance: usize,
    pub observed_distance_one: bool,
    pub dynamic_repeat_codes: u8,
    pub allocation_ready: bool,
    pub terminal: bool,
}

pub(super) struct RetainedInflateHistory {
    pub(super) bytes: std::mem::ManuallyDrop<Vec<u8>>,
    pub(super) target: usize,
    pub(super) maximum_allocation_bytes: usize,
    pub(super) length: usize,
    pub(super) write: usize,
    allocation_fault: Option<RetainedInflateAllocationError>,
    pub(super) physical_closed: bool,
}

impl RetainedInflateHistory {
    pub(super) fn new(target: usize, maximum_allocation_bytes: usize) -> Result<Self, DeflateError> {
        if target > maximum_allocation_bytes || maximum_allocation_bytes > isize::MAX as usize {
            return Err(DeflateError::OutputLimitExceeded);
        }
        Ok(Self {
            bytes: std::mem::ManuallyDrop::new(Vec::new()),
            target,
            maximum_allocation_bytes,
            length: 0,
            write: 0,
            allocation_fault: None,
            physical_closed: false,
        })
    }

    pub(super) fn allocated_bytes(&self) -> usize {
        self.bytes.capacity()
    }

    pub(super) fn ready(&self) -> bool {
        self.allocation_fault.is_none() && (self.target == 0 || self.allocated_bytes() >= self.target) && self.allocated_bytes() <= self.maximum_allocation_bytes && !self.physical_closed
    }

    pub(super) fn next_allocation_bytes(&self) -> Option<usize> {
        (!self.ready() && self.allocation_fault.is_none() && self.target != 0).then_some(self.target)
    }

    pub(super) fn reserve(&mut self, maximum_bytes: usize) -> Result<RetainedInflateAllocationStep, RetainedInflateAllocationError> {
        if let Some(fault) = self.allocation_fault {
            return Err(fault);
        }
        let Some(exact) = self.next_allocation_bytes() else { return Ok(RetainedInflateAllocationStep::default()) };
        if maximum_bytes < exact {
            return Ok(RetainedInflateAllocationStep::default());
        }
        if self.bytes.try_reserve_exact(exact).is_err() {
            let fault = RetainedInflateAllocationError { kind: ValueRefusalKind::AllocationFailed, allocated_bytes: self.allocated_bytes(), reason: "retained inflate history allocation failed" };
            self.allocation_fault = Some(fault);
            return Err(fault);
        }
        let allocated_bytes = self.allocated_bytes();
        if allocated_bytes < exact {
            let fault = RetainedInflateAllocationError { kind: ValueRefusalKind::InvariantViolated, allocated_bytes, reason: "retained inflate history allocation below requested reservation" };
            self.allocation_fault = Some(fault);
            return Err(fault);
        }
        if allocated_bytes > maximum_bytes || allocated_bytes > self.maximum_allocation_bytes {
            let fault = RetainedInflateAllocationError { kind: ValueRefusalKind::OwnershipLimit, allocated_bytes, reason: "retained inflate history allocation exceeded physical ceiling" };
            self.allocation_fault = Some(fault);
            return Err(fault);
        }
        Ok(RetainedInflateAllocationStep { progressed: true, allocated_bytes })
    }

    pub(super) fn push(&mut self, byte: u8) -> Result<(), DeflateError> {
        if !self.ready() || self.target == 0 {
            return Err(DeflateError::OutputLimitExceeded);
        }
        if self.length < self.target {
            self.bytes.push(byte);
            self.length += 1;
            self.write = self.length % self.target;
        } else {
            self.bytes[self.write] = byte;
            self.write = (self.write + 1) % self.target;
        }
        Ok(())
    }

    pub(super) fn read_back(&self, distance: usize) -> Result<u8, DeflateError> {
        if distance == 0 || distance > self.length {
            return Err(DeflateError::BadDistance);
        }
        let index = if self.length < self.target { self.length - distance } else { (self.write + self.target - distance) % self.target };
        Ok(self.bytes[index])
    }

    pub(super) fn reset(&mut self) {
        self.bytes.clear();
        self.length = 0;
        self.write = 0;
    }

    pub(super) fn release(&mut self, maximum_bytes: usize) -> RetainedInflateCloseStep {
        let allocated_bytes = self.allocated_bytes();
        if allocated_bytes > maximum_bytes {
            return RetainedInflateCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let bytes = std::mem::replace(&mut self.bytes, std::mem::ManuallyDrop::new(Vec::new()));
        drop(std::mem::ManuallyDrop::into_inner(bytes));
        self.physical_closed = true;
        if allocated_bytes == 0 {
            RetainedInflateCloseStep::Complete
        } else {
            RetainedInflateCloseStep::Pending { released_items: 0, released_bytes: allocated_bytes }
        }
    }
}

impl Drop for RetainedInflateHistory {
    fn drop(&mut self) {
        assert!(self.physical_closed && self.allocated_bytes() == 0, "retained inflate history reached Drop before exact physical release");
    }
}

pub(super) enum InflateHistory {
    Cold(Vec<u8>),
    Retained(RetainedInflateHistory),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum RetainedInflateClosePhase {
    Open,
    DecoderLogical,
    HistoryPhysical,
    Closed,
}

/// 🌊️ Resumable raw-DEFLATE decoder that yields exactly one output byte (or `NeedInput`/`Done`)
/// per `advance` call, driven by admitting at most one pending input byte at a time — the exact
/// shape `📡️replication`'s mounted pack reader needs to hand back control between I/O grants.
/// Keeps every byte it has produced so far (`output`) so LZ77 back-references can index into it;
/// callers that only need a one-shot `Vec<u8>` should use `inflate` instead.
pub struct Inflater {
    pub(super) reader: BitReader,
    pub(super) history: InflateHistory,
    pub(super) phase: Phase,
    pub(super) final_block: bool,
    pub(super) maximum_distance: usize,
    pub(super) observed_distance_one: bool,
    pub(super) dynamic_repeat_codes: u8,
    pub(super) retained_close_phase: RetainedInflateClosePhase,
}

impl Default for Inflater {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🔖️Inflate
