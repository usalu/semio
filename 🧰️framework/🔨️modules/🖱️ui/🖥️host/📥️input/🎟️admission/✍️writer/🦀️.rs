//! ✍️ Private byte-buffer work primitive; resident funding and source identity belong to its retained parent.

use std::mem::size_of;

//#region 🔖️Work
#[derive(Clone, Copy)]
pub(super) struct InputWriteGrant {
    pub items: usize,
    pub work_bytes: usize,
    pub physical_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum InputWriteKind {
    Blocked, Allocated, Validated, Copied, Sealed, Unsealed, InvalidUtf8,
    AllocationFault, CapacityFault, StateFault, Inspected, Released,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct InputWriteStep {
    pub kind: InputWriteKind,
    pub work_bytes: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

impl InputWriteStep {
    fn new(kind: InputWriteKind, work_bytes: usize) -> Self {
        Self { kind, work_bytes, allocated_bytes: 0, released_bytes: 0 }
    }
}

impl InputWriteGrant {
    fn permits(self, work_bytes: usize, physical_bytes: usize) -> bool {
        self.items != 0 && self.work_bytes >= work_bytes && self.physical_bytes >= physical_bytes
    }
}
//#endregion 🔖️Work

//#region 🔤️Utf8
#[derive(Clone, Copy)]
struct Utf8State {
    remaining: u8,
    lower: u8,
    upper: u8,
}

impl Utf8State {
    const fn new() -> Self { Self { remaining: 0, lower: 0x80, upper: 0xbf } }

    fn admit(&mut self, byte: u8) -> bool {
        if self.remaining != 0 {
            if byte < self.lower || byte > self.upper { return false; }
            self.remaining -= 1;
            self.lower = 0x80;
            self.upper = 0xbf;
            return true;
        }
        let (remaining, lower, upper) = match byte {
            0x00..=0x7f => (0, 0x80, 0xbf),
            0xc2..=0xdf => (1, 0x80, 0xbf),
            0xe0 => (2, 0xa0, 0xbf),
            0xe1..=0xec | 0xee..=0xef => (2, 0x80, 0xbf),
            0xed => (2, 0x80, 0x9f),
            0xf0 => (3, 0x90, 0xbf),
            0xf1..=0xf3 => (3, 0x80, 0xbf),
            0xf4 => (3, 0x80, 0x8f),
            _ => return false,
        };
        self.remaining = remaining;
        self.lower = lower;
        self.upper = upper;
        true
    }
}
//#endregion 🔤️Utf8

//#region ✍️Buffer
pub(super) struct InputByteBuffer {
    bytes: Vec<u8>,
    text: Option<String>,
    expected: usize,
    pending: Option<u8>,
    utf8: Utf8State,
    fault: Option<InputWriteKind>,
    admitted: bool,
    closing: bool,
    inspected: usize,
}

impl InputByteBuffer {
    pub(super) const fn new() -> Self {
        Self { bytes: Vec::new(), text: None, expected: 0, pending: None, utf8: Utf8State::new(), fault: None, admitted: false, closing: false, inspected: 0 }
    }

    pub(super) fn reserve(&mut self, logical_bytes: usize, minimum_capacity: usize, grant: InputWriteGrant) -> InputWriteStep {
        if self.admitted || self.closing || self.capacity() != 0 { return InputWriteStep::new(InputWriteKind::StateFault, 0); }
        if logical_bytes > super::DISCRETE_EVENT_BYTE_CAPACITY || minimum_capacity < logical_bytes || minimum_capacity > super::DISCRETE_EVENT_BYTE_CAPACITY {
            return InputWriteStep::new(InputWriteKind::CapacityFault, 0);
        }
        if !grant.permits(size_of::<Vec<u8>>(), minimum_capacity) { return InputWriteStep::new(InputWriteKind::Blocked, 0); }
        if self.bytes.try_reserve_exact(minimum_capacity).is_err() { return InputWriteStep::new(InputWriteKind::AllocationFault, 0); }
        let allocated_bytes = self.bytes.capacity();
        self.expected = logical_bytes;
        self.admitted = true;
        let kind = if allocated_bytes > grant.physical_bytes {
            self.fault = Some(InputWriteKind::AllocationFault);
            InputWriteKind::AllocationFault
        } else { InputWriteKind::Allocated };
        InputWriteStep { kind, work_bytes: size_of::<Vec<u8>>(), allocated_bytes, released_bytes: 0 }
    }

    pub(super) fn validate_byte(&mut self, byte: u8, grant: InputWriteGrant) -> InputWriteStep {
        if let Some(fault) = self.fault { return InputWriteStep::new(fault, 0); }
        if !grant.permits(1, 0) { return InputWriteStep::new(InputWriteKind::Blocked, 0); }
        if !self.admitted || self.closing || self.text.is_some() || self.pending.is_some() || self.bytes.len() >= self.expected {
            return InputWriteStep::new(InputWriteKind::StateFault, 0);
        }
        if !self.utf8.admit(byte) {
            self.fault = Some(InputWriteKind::InvalidUtf8);
            return InputWriteStep::new(InputWriteKind::InvalidUtf8, 1);
        }
        self.pending = Some(byte);
        InputWriteStep::new(InputWriteKind::Validated, 1)
    }

    pub(super) fn copy_validated(&mut self, grant: InputWriteGrant) -> InputWriteStep {
        if let Some(fault) = self.fault { return InputWriteStep::new(fault, 0); }
        if !grant.permits(1, 0) { return InputWriteStep::new(InputWriteKind::Blocked, 0); }
        if self.closing || self.text.is_some() || self.pending.is_none() { return InputWriteStep::new(InputWriteKind::StateFault, 0); }
        if self.bytes.len() >= self.bytes.capacity() { return InputWriteStep::new(InputWriteKind::CapacityFault, 0); }
        self.bytes.push(self.pending.take().expect("checked pending byte"));
        InputWriteStep::new(InputWriteKind::Copied, 1)
    }

    pub(super) const fn seal_work_bytes() -> usize { 4 * size_of::<Vec<u8>>() }

    pub(super) fn seal(&mut self, grant: InputWriteGrant) -> InputWriteStep {
        if let Some(fault) = self.fault { return InputWriteStep::new(fault, 0); }
        if !grant.permits(Self::seal_work_bytes(), 0) { return InputWriteStep::new(InputWriteKind::Blocked, 0); }
        if !self.admitted || self.closing || self.text.is_some() || self.pending.is_some() { return InputWriteStep::new(InputWriteKind::StateFault, 0); }
        if self.bytes.len() != self.expected || self.utf8.remaining != 0 {
            self.fault = Some(InputWriteKind::InvalidUtf8);
            return InputWriteStep::new(InputWriteKind::InvalidUtf8, 1);
        }
        self.text = Some(unsafe { String::from_utf8_unchecked(std::mem::take(&mut self.bytes)) });
        InputWriteStep::new(InputWriteKind::Sealed, Self::seal_work_bytes())
    }

    pub(super) fn bytes(&self) -> &[u8] {
        self.text.as_ref().map_or(self.bytes.as_slice(), |text| text.as_bytes())
    }

    pub(super) fn text(&self) -> Option<&str> { self.text.as_deref() }

    pub(super) fn capacity(&self) -> usize {
        self.bytes.capacity() + self.text.as_ref().map_or(0, String::capacity)
    }

    pub(super) fn inspected_bytes(&self) -> usize { self.inspected }

    pub(super) fn close_step(&mut self, grant: InputWriteGrant) -> InputWriteStep {
        if !grant.permits(1, 0) { return InputWriteStep::new(InputWriteKind::Blocked, 0); }
        if self.text.is_some() {
            if !grant.permits(Self::seal_work_bytes(), 0) { return InputWriteStep::new(InputWriteKind::Blocked, 0); }
            self.bytes = self.text.take().expect("checked sealed text").into_bytes();
            self.closing = true;
            return InputWriteStep::new(InputWriteKind::Unsealed, Self::seal_work_bytes());
        }
        self.closing = true;
        let total = self.bytes.len() + usize::from(self.pending.is_some());
        if self.inspected < total {
            let bytes = (total - self.inspected).min(grant.work_bytes);
            let start = self.inspected.min(self.bytes.len());
            let end = (self.inspected + bytes).min(self.bytes.len());
            self.bytes[start..end].fill(0);
            if self.inspected + bytes > self.bytes.len() { self.pending = None; }
            self.inspected += bytes;
            return InputWriteStep::new(InputWriteKind::Inspected, bytes);
        }
        let capacity = self.capacity();
        if !grant.permits(Self::seal_work_bytes(), capacity) { return InputWriteStep::new(InputWriteKind::Blocked, 0); }
        self.bytes.clear();
        if let Some(text) = self.text.as_mut() { text.clear(); }
        self.pending = None;
        drop(std::mem::take(&mut self.bytes));
        drop(self.text.take());
        self.admitted = false;
        InputWriteStep { kind: InputWriteKind::Released, work_bytes: Self::seal_work_bytes(), allocated_bytes: 0, released_bytes: capacity }
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        !self.admitted && self.capacity() == 0 && self.bytes.is_empty() && self.text.is_none() && self.pending.is_none()
    }
}
//#endregion ✍️Buffer
