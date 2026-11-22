//! 🔡️ Bounded decoders for ASCIIHexDecode and RunLengthDecode.

use semio_framework_value::ByteReadSource;

#[derive(Clone, Copy)]
enum Run {
    Control,
    Literal(u8),
    RepeatByte(u8),
    Repeat { count: u8, byte: u8 },
}

enum Filter { Hex(Option<u8>), RunLength(Run) }

pub(super) struct SimpleFilterCursor {
    filter: Filter,
    input: usize,
    pending: Option<u8>,
    complete: bool,
}

pub(super) struct SimpleFilterReceipt {
    pub(super) input: usize,
    pub(super) written: usize,
    pub(super) work: usize,
    pub(super) complete: bool,
    pub(super) source_changed: bool,
}

impl SimpleFilterCursor {
    pub(super) fn ascii_hex() -> Self { Self::new(Filter::Hex(None)) }
    pub(super) fn run_length() -> Self { Self::new(Filter::RunLength(Run::Control)) }

    fn new(filter: Filter) -> Self { Self { filter, input: 0, pending: None, complete: false } }

    pub(super) fn advance(&mut self, source: &(impl ByteReadSource + ?Sized), out: &mut [u8], budget: usize) -> SimpleFilterReceipt {
        let mut work = 0;
        let mut written = 0;
        while work < budget {
            if let Some(pending) = self.pending {
                let Some(output) = out.get_mut(written) else { break };
                *output = pending;
                self.pending = None;
                work += 1;
                written += 1;
                continue;
            }
            if self.complete { break; }
            work += 1;
            if let Filter::RunLength(Run::Repeat { count, byte }) = self.filter {
                self.pending = Some(byte);
                self.filter = Filter::RunLength(if count == 1 { Run::Control } else { Run::Repeat { count: count - 1, byte } });
                continue;
            }
            let byte = source.byte_at(self.input);
            if byte.is_none() && self.input < source.byte_len() {
                return SimpleFilterReceipt { input: self.input, written, work, complete: false, source_changed: true };
            }
            if byte.is_some() { self.input += 1; }
            match &mut self.filter {
                Filter::Hex(high) => match byte {
                    Some(b'>') | None => {
                        self.pending = high.take().map(|value| value << 4);
                        self.complete = true;
                    }
                    Some(byte) => {
                        let Some(nibble) = (byte as char).to_digit(16).map(|value| value as u8) else { continue };
                        if let Some(high) = high.take() { self.pending = Some(high << 4 | nibble); } else { *high = Some(nibble); }
                    }
                },
                Filter::RunLength(run) => {
                    let Some(byte) = byte else { self.complete = true; continue };
                    *run = match *run {
                        Run::Control => match byte {
                            128 => { self.complete = true; Run::Control }
                            0..=127 => Run::Literal(byte + 1),
                            _ => Run::RepeatByte((257 - u16::from(byte)) as u8),
                        },
                        Run::Literal(count) => {
                            self.pending = Some(byte);
                            if count == 1 { Run::Control } else { Run::Literal(count - 1) }
                        }
                        Run::RepeatByte(count) => Run::Repeat { count, byte },
                        Run::Repeat { .. } => unreachable!(),
                    };
                }
            }
        }
        SimpleFilterReceipt { input: self.input, written, work, complete: self.complete && self.pending.is_none(), source_changed: false }
    }
}
