//! 📦️ Bounded ASCII85 groups with one input or output byte per unit.

use semio_framework_value::ByteReadSource;

#[derive(Clone, Copy)]
enum Phase { Start, Prefix, Digits, Terminator, Complete, Failed }

pub(super) struct Ascii85Cursor {
    phase: Phase,
    input: usize,
    digits: usize,
    value: u32,
    block: [u8; 4],
    read: usize,
    written: usize,
}

pub(super) struct Ascii85Receipt {
    pub(super) input: usize,
    pub(super) written: usize,
    pub(super) work: usize,
    pub(super) done: Result<bool, &'static str>,
    pub(super) source_changed: bool,
}

impl Ascii85Cursor {
    pub(super) fn new() -> Self {
        Self { phase: Phase::Start, input: 0, digits: 0, value: 0, block: [0; 4], read: 0, written: 0 }
    }

    pub(super) fn advance(&mut self, source: &(impl ByteReadSource + ?Sized), output: &mut [u8], limit: usize) -> Ascii85Receipt {
        let mut work = 0;
        let mut written = 0;
        while work < limit {
            if self.read < self.written {
                let Some(target) = output.get_mut(written) else { break };
                *target = self.block[self.read];
                self.read += 1;
                work += 1;
                written += 1;
                continue;
            }
            if matches!(self.phase, Phase::Complete | Phase::Failed) { break; }
            let byte = source.byte_at(self.input);
            work += 1;
            if byte.is_none() && self.input < source.byte_len() {
                return Ascii85Receipt { input: self.input, written, work, done: Ok(false), source_changed: true };
            }
            if matches!(self.phase, Phase::Terminator) {
                if byte == Some(b'>') { self.input += 1; }
                self.finish();
                continue;
            }
            if byte.is_some() { self.input += 1; }
            match self.phase {
                Phase::Start if byte == Some(b'<') => { self.phase = Phase::Prefix; continue; }
                Phase::Prefix if byte == Some(b'~') => { self.phase = Phase::Digits; continue; }
                Phase::Prefix => self.push(b'<' - b'!'),
                _ => {}
            }
            self.phase = Phase::Digits;
            match byte {
                None => self.finish(),
                Some(b'~') => self.phase = Phase::Terminator,
                Some(0 | 9 | 10 | 12 | 13 | 32) => {}
                Some(b'z') if self.digits == 0 => self.publish([0; 4], 4),
                Some(byte @ b'!'..=b'u') => self.push(byte - b'!'),
                _ => self.phase = Phase::Failed,
            }
        }
        let done = match self.phase {
            Phase::Failed => Err("bad ASCII85 byte"),
            Phase::Complete => Ok(self.read == self.written),
            _ => Ok(false),
        };
        Ascii85Receipt { input: self.input, written, work, done, source_changed: false }
    }

    fn push(&mut self, digit: u8) {
        self.value = self.value.wrapping_mul(85).wrapping_add(u32::from(digit));
        self.digits += 1;
        if self.digits == 5 { self.publish(self.value.to_be_bytes(), 4); }
    }

    fn publish(&mut self, block: [u8; 4], size: usize) {
        self.block = block;
        self.read = 0;
        self.written = size;
        self.value = 0;
        self.digits = 0;
    }

    fn finish(&mut self) {
        self.phase = Phase::Complete;
        if self.digits > 1 {
            let size = self.digits - 1;
            for _ in self.digits..5 { self.value = self.value.wrapping_mul(85).wrapping_add(84); }
            self.publish(self.value.to_be_bytes(), size);
        }
    }
}
