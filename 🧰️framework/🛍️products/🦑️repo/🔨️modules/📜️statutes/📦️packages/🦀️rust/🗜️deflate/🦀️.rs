//! 🗜️ Deflate: the stored-block gzip encoder and the inflating gzip decoder with CRC-32 verification.

use std::sync::OnceLock;

//#region 🗜️Deflate

fn crc32(data: &[u8]) -> u32 {
    static TABLE: OnceLock<[u32; 256]> = OnceLock::new();
    let table = TABLE.get_or_init(|| {
        let mut table = [0u32; 256];
        for (index, slot) in table.iter_mut().enumerate() {
            let mut value = index as u32;
            for _ in 0..8 {
                value = if value & 1 == 1 { 0xEDB8_8320 ^ (value >> 1) } else { value >> 1 };
            }
            *slot = value;
        }
        table
    });
    let mut crc = 0xFFFF_FFFFu32;
    for byte in data {
        crc = table[((crc ^ u32::from(*byte)) & 0xFF) as usize] ^ (crc >> 8);
    }
    crc ^ 0xFFFF_FFFF
}

/// 🗜️ The gzip member wrapping the input, written as stored DEFLATE blocks.
pub fn gzip_encode(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x1f, 0x8b, 0x08, 0x00, 0, 0, 0, 0, 0x04, 0xff];
    if data.is_empty() {
        out.extend_from_slice(&[0x01, 0x00, 0x00, 0xff, 0xff]);
    } else {
        let chunks: Vec<&[u8]> = data.chunks(65535).collect();
        for (index, chunk) in chunks.iter().enumerate() {
            let final_block = u8::from(index + 1 == chunks.len());
            out.push(final_block);
            let length = chunk.len() as u16;
            out.extend_from_slice(&length.to_le_bytes());
            out.extend_from_slice(&(!length).to_le_bytes());
            out.extend_from_slice(chunk);
        }
    }
    out.extend_from_slice(&crc32(data).to_le_bytes());
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out
}

struct BitReader<'a> {
    data: &'a [u8],
    position: usize,
    bit: u32,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8], position: usize) -> Self {
        Self { data, position, bit: 0 }
    }

    fn read_bit(&mut self) -> Result<u32, String> {
        let byte = *self.data.get(self.position).ok_or_else(|| "truncated deflate stream".to_string())?;
        let value = (u32::from(byte) >> self.bit) & 1;
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.position += 1;
        }
        Ok(value)
    }

    fn read_bits(&mut self, count: u32) -> Result<u32, String> {
        let mut value = 0u32;
        for index in 0..count {
            value |= self.read_bit()? << index;
        }
        Ok(value)
    }

    fn align(&mut self) {
        if self.bit != 0 {
            self.bit = 0;
            self.position += 1;
        }
    }
}

struct Huffman {
    counts: Vec<u16>,
    symbols: Vec<u16>,
}

impl Huffman {
    fn new(lengths: &[u8]) -> Self {
        let max = lengths.iter().copied().max().unwrap_or(0) as usize;
        let mut counts = vec![0u16; max + 1];
        for length in lengths {
            counts[*length as usize] += 1;
        }
        if !counts.is_empty() {
            counts[0] = 0;
        }
        let mut offsets = vec![0u16; max + 2];
        for length in 1..=max {
            offsets[length + 1] = offsets[length] + counts[length];
        }
        let mut symbols = vec![0u16; lengths.len()];
        for (symbol, length) in lengths.iter().enumerate() {
            if *length != 0 {
                symbols[offsets[*length as usize] as usize] = symbol as u16;
                offsets[*length as usize] += 1;
            }
        }
        Self { counts, symbols }
    }

    fn decode(&self, reader: &mut BitReader<'_>) -> Result<u16, String> {
        let mut code = 0i32;
        let mut first = 0i32;
        let mut index = 0i32;
        for length in 1..self.counts.len() {
            code |= reader.read_bit()? as i32;
            let count = i32::from(self.counts[length]);
            if code - first < count {
                return Ok(self.symbols[(index + (code - first)) as usize]);
            }
            index += count;
            first = (first + count) << 1;
            code <<= 1;
        }
        Err("invalid huffman code".to_string())
    }
}

const LENGTH_BASE: [u16; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LENGTH_EXTRA: [u8; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DIST_BASE: [u16; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
const DIST_EXTRA: [u8; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];
const CODE_LENGTH_ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

fn fixed_trees() -> (Huffman, Huffman) {
    let mut literal_lengths = vec![8u8; 288];
    for length in literal_lengths.iter_mut().take(256).skip(144) {
        *length = 9;
    }
    for length in literal_lengths.iter_mut().take(280).skip(256) {
        *length = 7;
    }
    (Huffman::new(&literal_lengths), Huffman::new(&[5u8; 30]))
}

fn inflate(data: &[u8], start: usize) -> Result<Vec<u8>, String> {
    let mut reader = BitReader::new(data, start);
    let mut out: Vec<u8> = Vec::new();
    loop {
        let final_block = reader.read_bit()?;
        let kind = reader.read_bits(2)?;
        match kind {
            0 => {
                reader.align();
                if reader.position + 4 > data.len() {
                    return Err("truncated stored block".to_string());
                }
                let length = u16::from_le_bytes([data[reader.position], data[reader.position + 1]]) as usize;
                reader.position += 4;
                if reader.position + length > data.len() {
                    return Err("truncated stored payload".to_string());
                }
                out.extend_from_slice(&data[reader.position..reader.position + length]);
                reader.position += length;
            }
            1 | 2 => {
                let (literals, distances) = if kind == 1 {
                    fixed_trees()
                } else {
                    let hlit = reader.read_bits(5)? as usize + 257;
                    let hdist = reader.read_bits(5)? as usize + 1;
                    let hclen = reader.read_bits(4)? as usize + 4;
                    let mut code_lengths = [0u8; 19];
                    for index in 0..hclen {
                        code_lengths[CODE_LENGTH_ORDER[index]] = reader.read_bits(3)? as u8;
                    }
                    let code_tree = Huffman::new(&code_lengths);
                    let mut lengths = vec![0u8; hlit + hdist];
                    let mut index = 0usize;
                    while index < lengths.len() {
                        let symbol = code_tree.decode(&mut reader)?;
                        match symbol {
                            0..=15 => {
                                lengths[index] = symbol as u8;
                                index += 1;
                            }
                            16 => {
                                let previous = if index == 0 { return Err("repeat with no previous length".to_string()) } else { lengths[index - 1] };
                                let repeat = 3 + reader.read_bits(2)? as usize;
                                for _ in 0..repeat {
                                    if index >= lengths.len() {
                                        return Err("length repeat overflow".to_string());
                                    }
                                    lengths[index] = previous;
                                    index += 1;
                                }
                            }
                            17 => {
                                let repeat = 3 + reader.read_bits(3)? as usize;
                                index = (index + repeat).min(lengths.len());
                            }
                            18 => {
                                let repeat = 11 + reader.read_bits(7)? as usize;
                                index = (index + repeat).min(lengths.len());
                            }
                            _ => return Err("invalid code length symbol".to_string()),
                        }
                    }
                    (Huffman::new(&lengths[..hlit]), Huffman::new(&lengths[hlit..]))
                };
                loop {
                    let symbol = literals.decode(&mut reader)?;
                    if symbol == 256 {
                        break;
                    }
                    if symbol < 256 {
                        out.push(symbol as u8);
                        continue;
                    }
                    let index = symbol as usize - 257;
                    if index >= LENGTH_BASE.len() {
                        return Err("invalid length symbol".to_string());
                    }
                    let length = LENGTH_BASE[index] as usize + reader.read_bits(u32::from(LENGTH_EXTRA[index]))? as usize;
                    let dist_symbol = distances.decode(&mut reader)? as usize;
                    if dist_symbol >= DIST_BASE.len() {
                        return Err("invalid distance symbol".to_string());
                    }
                    let distance = DIST_BASE[dist_symbol] as usize + reader.read_bits(u32::from(DIST_EXTRA[dist_symbol]))? as usize;
                    if distance > out.len() {
                        return Err("distance beyond window".to_string());
                    }
                    let from = out.len() - distance;
                    for offset in 0..length {
                        let byte = out[from + offset];
                        out.push(byte);
                    }
                }
            }
            _ => return Err("reserved deflate block type".to_string()),
        }
        if final_block == 1 {
            return Ok(out);
        }
    }
}

/// 🗜️ The payload of a gzip member, verifying its CRC-32 and length trailer.
pub fn gzip_decode(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 18 || data[0] != 0x1f || data[1] != 0x8b {
        return Err("not a gzip member".to_string());
    }
    if data[2] != 8 {
        return Err("unsupported gzip compression method".to_string());
    }
    let flags = data[3];
    let mut cursor = 10usize;
    if flags & 0x04 != 0 {
        let extra = u16::from_le_bytes([data[cursor], data[cursor + 1]]) as usize;
        cursor += 2 + extra;
    }
    if flags & 0x08 != 0 {
        while cursor < data.len() && data[cursor] != 0 {
            cursor += 1;
        }
        cursor += 1;
    }
    if flags & 0x10 != 0 {
        while cursor < data.len() && data[cursor] != 0 {
            cursor += 1;
        }
        cursor += 1;
    }
    if flags & 0x02 != 0 {
        cursor += 2;
    }
    let payload = inflate(data, cursor)?;
    let trailer = &data[data.len() - 8..];
    let expected_crc = u32::from_le_bytes([trailer[0], trailer[1], trailer[2], trailer[3]]);
    let expected_length = u32::from_le_bytes([trailer[4], trailer[5], trailer[6], trailer[7]]);
    if crc32(&payload) != expected_crc {
        return Err("gzip crc mismatch".to_string());
    }
    if payload.len() as u32 != expected_length {
        return Err("gzip length mismatch".to_string());
    }
    Ok(payload)
}

//#endregion 🗜️Deflate
