//! 🔣️ JBIG2 Huffman tables: the standard tables B.1 to B.15 and custom tables (segment type 53).
//! [ITU-T T.88 Annex B](https://www.itu.int/rec/T-REC-T.88) · table values cross-checked with the public jbig2dec `jbig2_hufftab.c`

use std::collections::HashMap;

use crate::RasterError;
use Kind::{Lower, OutOfBand, Ordinary, Upper};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Ordinary,
    Lower,
    Upper,
    OutOfBand,
}

/// 📋 One table line: prefix length, range length and the smallest value (the largest for the lower range).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line {
    pub prefix_length: u8,
    pub range_length: u8,
    pub range_low: i32,
    pub kind: Kind,
}

const STANDARD: [&[Line]; 15] = [
    &[
        Line { prefix_length: 1, range_length: 4, range_low: 0, kind: Ordinary },
        Line { prefix_length: 2, range_length: 8, range_low: 16, kind: Ordinary },
        Line { prefix_length: 3, range_length: 16, range_low: 272, kind: Ordinary },
        Line { prefix_length: 0, range_length: 32, range_low: -1, kind: Lower },
        Line { prefix_length: 3, range_length: 32, range_low: 65808, kind: Upper },
    ],
    &[
        Line { prefix_length: 1, range_length: 0, range_low: 0, kind: Ordinary },
        Line { prefix_length: 2, range_length: 0, range_low: 1, kind: Ordinary },
        Line { prefix_length: 3, range_length: 0, range_low: 2, kind: Ordinary },
        Line { prefix_length: 4, range_length: 3, range_low: 3, kind: Ordinary },
        Line { prefix_length: 5, range_length: 6, range_low: 11, kind: Ordinary },
        Line { prefix_length: 0, range_length: 32, range_low: -1, kind: Lower },
        Line { prefix_length: 6, range_length: 32, range_low: 75, kind: Upper },
        Line { prefix_length: 6, range_length: 0, range_low: 0, kind: OutOfBand },
    ],
    &[
        Line { prefix_length: 8, range_length: 8, range_low: -256, kind: Ordinary },
        Line { prefix_length: 1, range_length: 0, range_low: 0, kind: Ordinary },
        Line { prefix_length: 2, range_length: 0, range_low: 1, kind: Ordinary },
        Line { prefix_length: 3, range_length: 0, range_low: 2, kind: Ordinary },
        Line { prefix_length: 4, range_length: 3, range_low: 3, kind: Ordinary },
        Line { prefix_length: 5, range_length: 6, range_low: 11, kind: Ordinary },
        Line { prefix_length: 8, range_length: 32, range_low: -257, kind: Lower },
        Line { prefix_length: 7, range_length: 32, range_low: 75, kind: Upper },
        Line { prefix_length: 6, range_length: 0, range_low: 0, kind: OutOfBand },
    ],
    &[
        Line { prefix_length: 1, range_length: 0, range_low: 1, kind: Ordinary },
        Line { prefix_length: 2, range_length: 0, range_low: 2, kind: Ordinary },
        Line { prefix_length: 3, range_length: 0, range_low: 3, kind: Ordinary },
        Line { prefix_length: 4, range_length: 3, range_low: 4, kind: Ordinary },
        Line { prefix_length: 5, range_length: 6, range_low: 12, kind: Ordinary },
        Line { prefix_length: 0, range_length: 32, range_low: -1, kind: Lower },
        Line { prefix_length: 5, range_length: 32, range_low: 76, kind: Upper },
    ],
    &[
        Line { prefix_length: 7, range_length: 8, range_low: -255, kind: Ordinary },
        Line { prefix_length: 1, range_length: 0, range_low: 1, kind: Ordinary },
        Line { prefix_length: 2, range_length: 0, range_low: 2, kind: Ordinary },
        Line { prefix_length: 3, range_length: 0, range_low: 3, kind: Ordinary },
        Line { prefix_length: 4, range_length: 3, range_low: 4, kind: Ordinary },
        Line { prefix_length: 5, range_length: 6, range_low: 12, kind: Ordinary },
        Line { prefix_length: 7, range_length: 32, range_low: -256, kind: Lower },
        Line { prefix_length: 6, range_length: 32, range_low: 76, kind: Upper },
    ],
    &[
        Line { prefix_length: 5, range_length: 10, range_low: -2048, kind: Ordinary },
        Line { prefix_length: 4, range_length: 9, range_low: -1024, kind: Ordinary },
        Line { prefix_length: 4, range_length: 8, range_low: -512, kind: Ordinary },
        Line { prefix_length: 4, range_length: 7, range_low: -256, kind: Ordinary },
        Line { prefix_length: 5, range_length: 6, range_low: -128, kind: Ordinary },
        Line { prefix_length: 5, range_length: 5, range_low: -64, kind: Ordinary },
        Line { prefix_length: 4, range_length: 5, range_low: -32, kind: Ordinary },
        Line { prefix_length: 2, range_length: 7, range_low: 0, kind: Ordinary },
        Line { prefix_length: 3, range_length: 7, range_low: 128, kind: Ordinary },
        Line { prefix_length: 3, range_length: 8, range_low: 256, kind: Ordinary },
        Line { prefix_length: 4, range_length: 9, range_low: 512, kind: Ordinary },
        Line { prefix_length: 4, range_length: 10, range_low: 1024, kind: Ordinary },
        Line { prefix_length: 6, range_length: 32, range_low: -2049, kind: Lower },
        Line { prefix_length: 6, range_length: 32, range_low: 2048, kind: Upper },
    ],
    &[
        Line { prefix_length: 4, range_length: 9, range_low: -1024, kind: Ordinary },
        Line { prefix_length: 3, range_length: 8, range_low: -512, kind: Ordinary },
        Line { prefix_length: 4, range_length: 7, range_low: -256, kind: Ordinary },
        Line { prefix_length: 5, range_length: 6, range_low: -128, kind: Ordinary },
        Line { prefix_length: 5, range_length: 5, range_low: -64, kind: Ordinary },
        Line { prefix_length: 4, range_length: 5, range_low: -32, kind: Ordinary },
        Line { prefix_length: 4, range_length: 5, range_low: 0, kind: Ordinary },
        Line { prefix_length: 5, range_length: 5, range_low: 32, kind: Ordinary },
        Line { prefix_length: 5, range_length: 6, range_low: 64, kind: Ordinary },
        Line { prefix_length: 4, range_length: 7, range_low: 128, kind: Ordinary },
        Line { prefix_length: 3, range_length: 8, range_low: 256, kind: Ordinary },
        Line { prefix_length: 3, range_length: 9, range_low: 512, kind: Ordinary },
        Line { prefix_length: 3, range_length: 10, range_low: 1024, kind: Ordinary },
        Line { prefix_length: 5, range_length: 32, range_low: -1025, kind: Lower },
        Line { prefix_length: 5, range_length: 32, range_low: 2048, kind: Upper },
    ],
    &[
        Line { prefix_length: 8, range_length: 3, range_low: -15, kind: Ordinary },
        Line { prefix_length: 9, range_length: 1, range_low: -7, kind: Ordinary },
        Line { prefix_length: 8, range_length: 1, range_low: -5, kind: Ordinary },
        Line { prefix_length: 9, range_length: 0, range_low: -3, kind: Ordinary },
        Line { prefix_length: 7, range_length: 0, range_low: -2, kind: Ordinary },
        Line { prefix_length: 4, range_length: 0, range_low: -1, kind: Ordinary },
        Line { prefix_length: 2, range_length: 1, range_low: 0, kind: Ordinary },
        Line { prefix_length: 5, range_length: 0, range_low: 2, kind: Ordinary },
        Line { prefix_length: 6, range_length: 0, range_low: 3, kind: Ordinary },
        Line { prefix_length: 3, range_length: 4, range_low: 4, kind: Ordinary },
        Line { prefix_length: 6, range_length: 1, range_low: 20, kind: Ordinary },
        Line { prefix_length: 4, range_length: 4, range_low: 22, kind: Ordinary },
        Line { prefix_length: 4, range_length: 5, range_low: 38, kind: Ordinary },
        Line { prefix_length: 5, range_length: 6, range_low: 70, kind: Ordinary },
        Line { prefix_length: 5, range_length: 7, range_low: 134, kind: Ordinary },
        Line { prefix_length: 6, range_length: 7, range_low: 262, kind: Ordinary },
        Line { prefix_length: 7, range_length: 8, range_low: 390, kind: Ordinary },
        Line { prefix_length: 6, range_length: 10, range_low: 646, kind: Ordinary },
        Line { prefix_length: 9, range_length: 32, range_low: -16, kind: Lower },
        Line { prefix_length: 9, range_length: 32, range_low: 1670, kind: Upper },
        Line { prefix_length: 2, range_length: 0, range_low: 0, kind: OutOfBand },
    ],
    &[
        Line { prefix_length: 8, range_length: 4, range_low: -31, kind: Ordinary },
        Line { prefix_length: 9, range_length: 2, range_low: -15, kind: Ordinary },
        Line { prefix_length: 8, range_length: 2, range_low: -11, kind: Ordinary },
        Line { prefix_length: 9, range_length: 1, range_low: -7, kind: Ordinary },
        Line { prefix_length: 7, range_length: 1, range_low: -5, kind: Ordinary },
        Line { prefix_length: 4, range_length: 1, range_low: -3, kind: Ordinary },
        Line { prefix_length: 3, range_length: 1, range_low: -1, kind: Ordinary },
        Line { prefix_length: 3, range_length: 1, range_low: 1, kind: Ordinary },
        Line { prefix_length: 5, range_length: 1, range_low: 3, kind: Ordinary },
        Line { prefix_length: 6, range_length: 1, range_low: 5, kind: Ordinary },
        Line { prefix_length: 3, range_length: 5, range_low: 7, kind: Ordinary },
        Line { prefix_length: 6, range_length: 2, range_low: 39, kind: Ordinary },
        Line { prefix_length: 4, range_length: 5, range_low: 43, kind: Ordinary },
        Line { prefix_length: 4, range_length: 6, range_low: 75, kind: Ordinary },
        Line { prefix_length: 5, range_length: 7, range_low: 139, kind: Ordinary },
        Line { prefix_length: 5, range_length: 8, range_low: 267, kind: Ordinary },
        Line { prefix_length: 6, range_length: 8, range_low: 523, kind: Ordinary },
        Line { prefix_length: 7, range_length: 9, range_low: 779, kind: Ordinary },
        Line { prefix_length: 6, range_length: 11, range_low: 1291, kind: Ordinary },
        Line { prefix_length: 9, range_length: 32, range_low: -32, kind: Lower },
        Line { prefix_length: 9, range_length: 32, range_low: 3339, kind: Upper },
        Line { prefix_length: 2, range_length: 0, range_low: 0, kind: OutOfBand },
    ],
    &[
        Line { prefix_length: 7, range_length: 4, range_low: -21, kind: Ordinary },
        Line { prefix_length: 8, range_length: 0, range_low: -5, kind: Ordinary },
        Line { prefix_length: 7, range_length: 0, range_low: -4, kind: Ordinary },
        Line { prefix_length: 5, range_length: 0, range_low: -3, kind: Ordinary },
        Line { prefix_length: 2, range_length: 2, range_low: -2, kind: Ordinary },
        Line { prefix_length: 5, range_length: 0, range_low: 2, kind: Ordinary },
        Line { prefix_length: 6, range_length: 0, range_low: 3, kind: Ordinary },
        Line { prefix_length: 7, range_length: 0, range_low: 4, kind: Ordinary },
        Line { prefix_length: 8, range_length: 0, range_low: 5, kind: Ordinary },
        Line { prefix_length: 2, range_length: 6, range_low: 6, kind: Ordinary },
        Line { prefix_length: 5, range_length: 5, range_low: 70, kind: Ordinary },
        Line { prefix_length: 6, range_length: 5, range_low: 102, kind: Ordinary },
        Line { prefix_length: 6, range_length: 6, range_low: 134, kind: Ordinary },
        Line { prefix_length: 6, range_length: 7, range_low: 198, kind: Ordinary },
        Line { prefix_length: 6, range_length: 8, range_low: 326, kind: Ordinary },
        Line { prefix_length: 6, range_length: 9, range_low: 582, kind: Ordinary },
        Line { prefix_length: 6, range_length: 10, range_low: 1094, kind: Ordinary },
        Line { prefix_length: 7, range_length: 11, range_low: 2118, kind: Ordinary },
        Line { prefix_length: 8, range_length: 32, range_low: -22, kind: Lower },
        Line { prefix_length: 8, range_length: 32, range_low: 4166, kind: Upper },
        Line { prefix_length: 2, range_length: 0, range_low: 0, kind: OutOfBand },
    ],
    &[
        Line { prefix_length: 1, range_length: 0, range_low: 1, kind: Ordinary },
        Line { prefix_length: 2, range_length: 1, range_low: 2, kind: Ordinary },
        Line { prefix_length: 4, range_length: 0, range_low: 4, kind: Ordinary },
        Line { prefix_length: 4, range_length: 1, range_low: 5, kind: Ordinary },
        Line { prefix_length: 5, range_length: 1, range_low: 7, kind: Ordinary },
        Line { prefix_length: 5, range_length: 2, range_low: 9, kind: Ordinary },
        Line { prefix_length: 6, range_length: 2, range_low: 13, kind: Ordinary },
        Line { prefix_length: 7, range_length: 2, range_low: 17, kind: Ordinary },
        Line { prefix_length: 7, range_length: 3, range_low: 21, kind: Ordinary },
        Line { prefix_length: 7, range_length: 4, range_low: 29, kind: Ordinary },
        Line { prefix_length: 7, range_length: 5, range_low: 45, kind: Ordinary },
        Line { prefix_length: 7, range_length: 6, range_low: 77, kind: Ordinary },
        Line { prefix_length: 0, range_length: 32, range_low: -1, kind: Lower },
        Line { prefix_length: 7, range_length: 32, range_low: 141, kind: Upper },
    ],
    &[
        Line { prefix_length: 1, range_length: 0, range_low: 1, kind: Ordinary },
        Line { prefix_length: 2, range_length: 0, range_low: 2, kind: Ordinary },
        Line { prefix_length: 3, range_length: 1, range_low: 3, kind: Ordinary },
        Line { prefix_length: 5, range_length: 0, range_low: 5, kind: Ordinary },
        Line { prefix_length: 5, range_length: 1, range_low: 6, kind: Ordinary },
        Line { prefix_length: 6, range_length: 1, range_low: 8, kind: Ordinary },
        Line { prefix_length: 7, range_length: 0, range_low: 10, kind: Ordinary },
        Line { prefix_length: 7, range_length: 1, range_low: 11, kind: Ordinary },
        Line { prefix_length: 7, range_length: 2, range_low: 13, kind: Ordinary },
        Line { prefix_length: 7, range_length: 3, range_low: 17, kind: Ordinary },
        Line { prefix_length: 7, range_length: 4, range_low: 25, kind: Ordinary },
        Line { prefix_length: 8, range_length: 5, range_low: 41, kind: Ordinary },
        Line { prefix_length: 8, range_length: 32, range_low: 73, kind: Ordinary },
        Line { prefix_length: 0, range_length: 32, range_low: -1, kind: Lower },
        Line { prefix_length: 0, range_length: 32, range_low: 0, kind: Upper },
    ],
    &[
        Line { prefix_length: 1, range_length: 0, range_low: 1, kind: Ordinary },
        Line { prefix_length: 3, range_length: 0, range_low: 2, kind: Ordinary },
        Line { prefix_length: 4, range_length: 0, range_low: 3, kind: Ordinary },
        Line { prefix_length: 5, range_length: 0, range_low: 4, kind: Ordinary },
        Line { prefix_length: 4, range_length: 1, range_low: 5, kind: Ordinary },
        Line { prefix_length: 3, range_length: 3, range_low: 7, kind: Ordinary },
        Line { prefix_length: 6, range_length: 1, range_low: 15, kind: Ordinary },
        Line { prefix_length: 6, range_length: 2, range_low: 17, kind: Ordinary },
        Line { prefix_length: 6, range_length: 3, range_low: 21, kind: Ordinary },
        Line { prefix_length: 6, range_length: 4, range_low: 29, kind: Ordinary },
        Line { prefix_length: 6, range_length: 5, range_low: 45, kind: Ordinary },
        Line { prefix_length: 7, range_length: 6, range_low: 77, kind: Ordinary },
        Line { prefix_length: 0, range_length: 32, range_low: -1, kind: Lower },
        Line { prefix_length: 7, range_length: 32, range_low: 141, kind: Upper },
    ],
    &[
        Line { prefix_length: 3, range_length: 0, range_low: -2, kind: Ordinary },
        Line { prefix_length: 3, range_length: 0, range_low: -1, kind: Ordinary },
        Line { prefix_length: 1, range_length: 0, range_low: 0, kind: Ordinary },
        Line { prefix_length: 3, range_length: 0, range_low: 1, kind: Ordinary },
        Line { prefix_length: 3, range_length: 0, range_low: 2, kind: Ordinary },
        Line { prefix_length: 0, range_length: 32, range_low: -1, kind: Lower },
        Line { prefix_length: 0, range_length: 32, range_low: 3, kind: Upper },
    ],
    &[
        Line { prefix_length: 7, range_length: 4, range_low: -24, kind: Ordinary },
        Line { prefix_length: 6, range_length: 2, range_low: -8, kind: Ordinary },
        Line { prefix_length: 5, range_length: 1, range_low: -4, kind: Ordinary },
        Line { prefix_length: 4, range_length: 0, range_low: -2, kind: Ordinary },
        Line { prefix_length: 3, range_length: 0, range_low: -1, kind: Ordinary },
        Line { prefix_length: 1, range_length: 0, range_low: 0, kind: Ordinary },
        Line { prefix_length: 3, range_length: 0, range_low: 1, kind: Ordinary },
        Line { prefix_length: 4, range_length: 0, range_low: 2, kind: Ordinary },
        Line { prefix_length: 5, range_length: 1, range_low: 3, kind: Ordinary },
        Line { prefix_length: 6, range_length: 2, range_low: 5, kind: Ordinary },
        Line { prefix_length: 7, range_length: 4, range_low: 9, kind: Ordinary },
        Line { prefix_length: 7, range_length: 32, range_low: -25, kind: Lower },
        Line { prefix_length: 7, range_length: 32, range_low: 25, kind: Upper },
    ],
];

/// 🔣️ A prefix-code table with its canonical code assignment.
#[derive(Clone, Debug)]
pub struct Table {
    lines: Vec<Line>,
    codes: HashMap<(u8, u32), usize>,
}

/// 📖️ Reads bits MSB first.
pub struct BitSource<'a> {
    pub data: &'a [u8],
    pub position: usize,
}

impl BitSource<'_> {
    pub fn bit(&mut self) -> Result<u32, RasterError> {
        let byte = *self.data.get(self.position >> 3).ok_or(RasterError::Malformed("truncated JBIG2 Huffman data"))?;
        let bit = u32::from(byte >> (7 - (self.position & 7)) & 1);
        self.position += 1;
        Ok(bit)
    }

    pub fn bits(&mut self, count: u32) -> Result<u32, RasterError> {
        (0..count).try_fold(0u32, |value, _| Ok(value << 1 | self.bit()?))
    }

    pub fn align(&mut self) { self.position = self.position.div_ceil(8) * 8; }
}

impl Table {
    /// 🏗️ Assigns canonical codes (T.88 §B.3) to `lines`.
    pub fn new(lines: Vec<Line>) -> Self {
        let longest = lines.iter().map(|line| line.prefix_length).max().unwrap_or(0) as usize;
        let mut counts = vec![0u32; longest + 2];
        for line in &lines { counts[line.prefix_length as usize] += 1; }
        counts[0] = 0;
        let mut first = vec![0u32; longest + 2];
        let mut next = vec![0u32; longest + 2];
        for length in 1..=longest {
            first[length] = (first[length - 1] + counts[length - 1]) << 1;
            next[length] = first[length];
        }
        let mut codes = HashMap::new();
        for (index, line) in lines.iter().enumerate() {
            let length = line.prefix_length as usize;
            if length == 0 { continue; }
            codes.insert((line.prefix_length, next[length]), index);
            next[length] += 1;
        }
        Self { lines, codes }
    }

    /// 📚️ Standard table `number` (1 to 15).
    pub fn standard(number: usize) -> Option<Self> {
        STANDARD.get(number.checked_sub(1)?).map(|lines| Self::new(lines.to_vec()))
    }

    /// 🧩️ Builds a custom table from a type 53 segment body.
    pub fn custom(data: &[u8]) -> Result<Self, RasterError> {
        let flags = *data.first().ok_or(RasterError::Malformed("truncated JBIG2 table segment"))?;
        let (out_of_band, prefix_bits, range_bits) = (flags & 1 == 1, u32::from(flags >> 1 & 7) + 1, u32::from(flags >> 4 & 7) + 1);
        let word = |at: usize| data.get(at..at + 4).map(|b| i32::from_be_bytes([b[0], b[1], b[2], b[3]])).ok_or(RasterError::Malformed("truncated JBIG2 table segment"));
        let (low, high) = (word(1)?, word(5)?);
        let mut bits = BitSource { data, position: 9 * 8 };
        let mut lines = Vec::new();
        let mut current = i64::from(low);
        while current < i64::from(high) {
            let (prefix, range) = (bits.bits(prefix_bits)? as u8, bits.bits(range_bits)? as u8);
            lines.push(Line { prefix_length: prefix, range_length: range, range_low: current as i32, kind: Kind::Ordinary });
            current += 1i64 << range;
            if lines.len() > 1 << 16 { return Err(RasterError::TooLarge); }
        }
        lines.push(Line { prefix_length: bits.bits(prefix_bits)? as u8, range_length: 32, range_low: low.wrapping_sub(1), kind: Kind::Lower });
        lines.push(Line { prefix_length: bits.bits(prefix_bits)? as u8, range_length: 32, range_low: high, kind: Kind::Upper });
        if out_of_band { lines.push(Line { prefix_length: bits.bits(prefix_bits)? as u8, range_length: 0, range_low: 0, kind: Kind::OutOfBand }); }
        Ok(Self::new(lines))
    }

    /// 🔣️ Decodes one value; `None` is the out-of-band marker.
    pub fn decode(&self, bits: &mut BitSource<'_>) -> Result<Option<i32>, RasterError> {
        let (mut length, mut code) = (0u8, 0u32);
        loop {
            code = code << 1 | bits.bit()?;
            length += 1;
            if let Some(&index) = self.codes.get(&(length, code)) {
                let line = self.lines[index];
                return Ok(match line.kind {
                    OutOfBand => None,
                    Lower => Some(line.range_low.wrapping_sub(bits.bits(32)? as i32)),
                    Upper => Some(line.range_low.wrapping_add(bits.bits(32)? as i32)),
                    Ordinary => Some(line.range_low + bits.bits(u32::from(line.range_length))? as i32),
                });
            }
            if length > 32 { return Err(RasterError::Malformed("invalid JBIG2 Huffman code")); }
        }
    }
}
