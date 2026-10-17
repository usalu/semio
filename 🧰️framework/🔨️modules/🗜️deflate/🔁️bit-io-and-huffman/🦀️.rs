//! 🔁️ Bit I/O and Huffman of the deflate codec: the bit reader and writer and the canonical Huffman decode table. See [`super`].

use super::*;

//#region 🔖️BitIo
pub(super) struct BitReader {
    pub(super) buffer: u32,
    pub(super) bits: u32,
}

impl BitReader {
    pub(super) fn new() -> Self {
        Self { buffer: 0, bits: 0 }
    }

    fn push_byte(&mut self, byte: u8) {
        self.buffer |= (byte as u32) << self.bits;
        self.bits += 8;
    }

    pub(super) fn ensure(&mut self, want: u32, pending: &mut Option<u8>, input_complete: bool) -> Result<bool, DeflateError> {
        while self.bits < want {
            if let Some(byte) = pending.take() {
                self.push_byte(byte);
                continue;
            }
            return if input_complete { Err(DeflateError::UnexpectedEnd) } else { Ok(false) };
        }
        Ok(true)
    }

    pub(super) fn ensure_huffman(&mut self, pending: &mut Option<u8>, input_complete: bool) -> Result<bool, DeflateError> {
        match self.ensure(15, pending, input_complete) {
            Ok(ready) => Ok(ready),
            Err(DeflateError::UnexpectedEnd) => Ok(true),
            Err(error) => Err(error),
        }
    }

    fn peek(&self, n: u32) -> u32 {
        if n == 0 {
            0
        } else {
            self.buffer & ((1u32 << n) - 1)
        }
    }

    fn drop_bits(&mut self, n: u32) {
        self.buffer >>= n;
        self.bits = self.bits.saturating_sub(n);
    }

    pub(super) fn take(&mut self, n: u32) -> u32 {
        let value = self.peek(n);
        self.drop_bits(n);
        value
    }

    fn take_bit(&mut self) -> Option<u32> {
        (self.bits > 0).then(|| self.take(1))
    }

    pub(super) fn align_byte(&mut self) {
        let remainder = self.bits % 8;
        self.drop_bits(remainder);
    }
}

pub(super) struct BitWriter {
    pub(super) bytes: Vec<u8>,
    pub(super) buffer: u32,
    pub(super) bits: u32,
}

impl BitWriter {
    pub(super) fn new() -> Self {
        Self { bytes: Vec::new(), buffer: 0, bits: 0 }
    }

    pub(super) fn write_bits(&mut self, value: u32, n: u32) {
        if n == 0 {
            return;
        }
        self.buffer |= (value & ((1u32 << n) - 1)) << self.bits;
        self.bits += n;
        while self.bits >= 8 {
            self.bytes.push((self.buffer & 0xff) as u8);
            self.buffer >>= 8;
            self.bits -= 8;
        }
    }

    pub(super) fn write_huffman_code(&mut self, code: u16, len: u8) {
        for shift in (0..len).rev() {
            self.write_bits(((code >> shift) & 1) as u32, 1);
        }
    }

    pub(super) fn finish(mut self) -> Vec<u8> {
        if self.bits > 0 {
            self.bytes.push((self.buffer & 0xff) as u8);
        }
        self.bytes
    }
}
//#endregion 🔖️BitIo

//#region 🔖️Huffman
/// 🌲️ A canonical Huffman decode table built the same way `puff.c` (Mark Adler's public-domain
/// reference `inflate`) does: `counts[len]` plus `symbols` sorted by `(len, code)`, decoded
/// bit-by-bit by tracking the running `(code, first, index)` triple per length — no bit-reversed
/// lookup table needed.
pub(super) struct Huffman {
    pub(super) counts: [u16; 16],
    symbols: [u16; 288],
    pub(super) complete: bool,
}

impl Huffman {
    pub(super) fn build(lengths: &[u8]) -> Result<Self, DeflateError> {
        let mut counts = [0u16; 16];
        for &len in lengths {
            if len as usize >= counts.len() {
                return Err(DeflateError::BadHuffmanCode);
            }
            counts[len as usize] += 1;
        }
        counts[0] = 0;
        let mut available = 1i32;
        for count in &counts[1..] {
            available = available * 2 - i32::from(*count);
            if available < 0 {
                return Err(DeflateError::BadHuffmanCode);
            }
        }
        let mut offsets = [0u16; 16];
        for len in 1..16 {
            offsets[len] = offsets[len - 1] + counts[len - 1];
        }
        if lengths.len() > 288 {
            return Err(DeflateError::BadHuffmanCode);
        }
        let mut symbols = [0u16; 288];
        for (symbol, &len) in lengths.iter().enumerate() {
            if len != 0 {
                symbols[offsets[len as usize] as usize] = symbol as u16;
                offsets[len as usize] += 1;
            }
        }
        Ok(Self { counts, symbols, complete: available == 0 })
    }

    pub(super) fn valid_alphabet(&self, allow_empty: bool) -> bool {
        self.complete || (self.counts[1] == 1 && self.counts[2..].iter().all(|&n| n == 0)) || (allow_empty && self.counts.iter().all(|&n| n == 0))
    }

    pub(super) fn decode(&self, reader: &mut BitReader) -> Result<u16, DeflateError> {
        let mut code: i32 = 0;
        let mut first: i32 = 0;
        let mut index: i32 = 0;
        for len in 1..16 {
            code |= reader.take_bit().ok_or(DeflateError::UnexpectedEnd)? as i32;
            let count = self.counts[len] as i32;
            if code - first < count {
                return Ok(self.symbols[(index + (code - first)) as usize]);
            }
            index += count;
            first += count;
            first <<= 1;
            code <<= 1;
        }
        Err(DeflateError::BadHuffmanCode)
    }
}
//#endregion 🔖️Huffman
