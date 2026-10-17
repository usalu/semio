//! 📋️ Errors and tables of the deflate codec: the inflate error causes, the length and distance base tables, the code length order and the canonical code builders. See [`super`].

//#region 🔖️Errors
/// 🚨️ Incremental inflate reports its closed syntax and output-limit causes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeflateError {
    BadBlockType,
    BadStoredLength,
    BadHuffmanCode,
    BadDistance,
    UnexpectedEnd,
    OutputLimitExceeded,
}
//#endregion 🔖️Errors

//#region 🔖️Tables
pub(super) const LENGTH_BASE: [u16; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];

pub(super) const LENGTH_EXTRA: [u8; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];

pub(super) const DIST_BASE: [u32; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];

pub(super) const DIST_EXTRA: [u8; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];

pub(super) const CLC_ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

pub(super) fn length_index_for(len: u16) -> usize {
    for index in (0..LENGTH_BASE.len()).rev() {
        if len >= LENGTH_BASE[index] {
            return index;
        }
    }
    0
}

pub(super) fn distance_index_for(distance: u32) -> usize {
    for index in (0..DIST_BASE.len()).rev() {
        if distance >= DIST_BASE[index] {
            return index;
        }
    }
    0
}

pub(super) fn fixed_literal_length_lengths() -> [u8; 288] {
    let mut lengths = [0u8; 288];
    lengths[0..144].fill(8);
    lengths[144..256].fill(9);
    lengths[256..280].fill(7);
    lengths[280..288].fill(8);
    lengths
}

pub(super) fn fixed_distance_lengths() -> [u8; 30] {
    [5u8; 30]
}

/// 🔢️ RFC 1951 §3.2.2's canonical-code assignment: codes ordered first by length then by symbol,
/// consecutive within a length. Used by the encoder; the decoder's `Huffman::build` derives the
/// same assignment implicitly from `counts`/`symbols` without ever materializing the bit patterns.
pub(super) fn canonical_codes(lengths: &[u8]) -> Vec<u16> {
    let max_len = lengths.iter().copied().max().unwrap_or(0) as usize;
    let mut bl_count = vec![0u16; max_len + 1];
    for &len in lengths {
        if len > 0 {
            bl_count[len as usize] += 1;
        }
    }
    let mut code = 0u16;
    let mut next_code = vec![0u16; max_len + 1];
    for bits in 1..=max_len {
        code = (code + bl_count[bits - 1]) << 1;
        next_code[bits] = code;
    }
    let mut codes = vec![0u16; lengths.len()];
    for (symbol, &len) in lengths.iter().enumerate() {
        if len > 0 {
            codes[symbol] = next_code[len as usize];
            next_code[len as usize] += 1;
        }
    }
    codes
}
//#endregion 🔖️Tables
