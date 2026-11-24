//! 🧰️ Emits deterministic zlib stored blocks when the graph requests Flate without a predictor.
use std::io::Write;
use super::classic_graph_quote::GraphRefusal;
use super::classic_graph_writer::{WriteRefusal, try_write};
use semio_framework_value::ByteReadSource;

const BLOCK: usize = u16::MAX as usize;
const MODULUS: u32 = 65_521;

pub(crate) fn encoded_len(length: usize) -> Option<usize> {
    length.div_ceil(BLOCK).max(1).checked_mul(5)?.checked_add(6)?.checked_add(length)
}

pub(crate) struct StoredFlate {
    at: usize,
    block_remaining: usize,
    pending: [u8; 5],
    lo: usize,
    hi: usize,
    first: u32,
    second: u32,
    final_block: bool,
    checksum: bool,
}

impl StoredFlate {
    pub(crate) fn new() -> Self {
        Self { at: 0, block_remaining: 0, pending: [0x78, 0x01, 0, 0, 0], lo: 0, hi: 2,
            first: 1, second: 0, final_block: false, checksum: false }
    }

    /// 🪜️ Emits one structural prefix or one body span. The body span is read again for Adler-32.
    pub(crate) fn step(&mut self, data: &(impl ByteReadSource + ?Sized), sink: &mut dyn Write, grant: usize, remaining: usize)
        -> Result<(usize, usize, bool), WriteRefusal>
    {
        if self.lo != self.hi {
            let count = try_write(sink, &self.pending[self.lo..self.hi], grant, remaining)?;
            self.lo += count;
            return Ok((count, 0, false));
        }
        if self.block_remaining != 0 {
            let mut page = [0u8; 4096];
            let want = self.block_remaining.min(page.len());
            if data.read_bytes(self.at, &mut page[..want]) != want { return Err(GraphRefusal::SourceChanged.into()); }
            let count = try_write(sink, &page[..want], grant / 2, remaining)?;
            for byte in &page[..count] {
                self.first = (self.first + u32::from(*byte)) % MODULUS;
                self.second = (self.second + self.first) % MODULUS;
            }
            self.at += count;
            self.block_remaining -= count;
            return Ok((count, count, false));
        }
        if self.final_block {
            if self.checksum { return Ok((0, 0, true)); }
            self.pending[..4].copy_from_slice(&(self.second << 16 | self.first).to_be_bytes());
            self.hi = 4;
            self.checksum = true;
        } else {
            let length = (data.byte_len() - self.at).min(BLOCK);
            self.block_remaining = length;
            self.final_block = self.at + length == data.byte_len();
            let length = length as u16;
            self.pending[0] = u8::from(self.final_block);
            self.pending[1..3].copy_from_slice(&length.to_le_bytes());
            self.pending[3..5].copy_from_slice(&(!length).to_le_bytes());
            self.hi = 5;
        }
        self.lo = 0;
        let count = try_write(sink, &self.pending[..self.hi], grant, remaining)?;
        self.lo += count;
        Ok((count, 0, false))
    }
}
