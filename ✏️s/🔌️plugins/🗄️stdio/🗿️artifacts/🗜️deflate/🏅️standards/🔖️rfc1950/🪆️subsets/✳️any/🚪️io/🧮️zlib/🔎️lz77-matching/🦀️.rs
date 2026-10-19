//! 🔎️ LZ77 matching of the zlib codec: the length and distance tables, the hash chain constants, the hash, the longest match search and the length and distance symbols. See [`super`].


//#region 🦑️DissolvedEngineCodec
//#region DeflateCodec
pub(super) const LEN_BASE: [u16; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];

pub(super) const LEN_EXTRA: [u8; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];

pub(super) const DIST_BASE: [u16; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];

pub(super) const DIST_EXTRA: [u8; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];

//#region Lz77
/// 🪟️ RFC1951 §3.2.5 sliding window: matches can reference up to this many bytes back.
pub const WINDOW: usize = 32 * 1024;

/// 📏️ Shortest a match is worth emitting as length+distance instead of two-plus literals.
pub(super) const MIN_MATCH: usize = 3;

/// 📏️ Longest length symbol 285 can encode (258 = LEN_BASE[28] + 2^5-1 extra bits).
pub(super) const MAX_MATCH: usize = 258;

/// 🔑️ 3-byte-prefix hash table size (2^15 buckets over a 3-byte key -- zlib's own classic choice).
const HASH_BITS: u32 = 15;

pub const HASH_SIZE: usize = 1 << HASH_BITS;

#[inline]
pub(super) fn hash3(data: &[u8], i: usize) -> usize {
    let v = (data[i] as u32) | ((data[i + 1] as u32) << 8) | ((data[i + 2] as u32) << 16);
    ((v.wrapping_mul(0x9E3779B1)) >> (32 - HASH_BITS)) as usize
}

/// 🔎️ Longest match at `pos` found by walking the hash chain, verified byte-by-byte (hash
/// collisions are possible; a wrong-length "match" would corrupt the stream, so every candidate
/// is checked in full before being accepted). `max_chain` bounds how many chain entries we walk
/// before settling -- a simple, deterministic time/ratio tradeoff (higher = better ratio, slower).
pub(super) fn longest_match(data: &[u8], pos: usize, head: &[i32], prev: &[i32], max_chain: usize) -> Option<(usize, usize)> {
    if pos + MIN_MATCH > data.len() {
        return None;
    }
    let limit = (data.len() - pos).min(MAX_MATCH);
    let mut best_len = 0usize;
    let mut best_dist = 0usize;
    let mut candidate = head[hash3(data, pos)];
    let min_candidate = pos.saturating_sub(WINDOW - 1) as i32;
    let mut chain = 0usize;
    while candidate >= min_candidate && candidate >= 0 && chain < max_chain {
        let cpos = candidate as usize;
        if best_len == 0 || (cpos + best_len < data.len() && data[cpos + best_len] == data[pos + best_len]) {
            let mut len = 0usize;
            while len < limit && data[cpos + len] == data[pos + len] {
                len += 1;
            }
            if len > best_len {
                best_len = len;
                best_dist = pos - cpos;
                if len >= limit {
                    break;
                }
            }
        }
        candidate = prev[cpos & (WINDOW - 1)];
        chain += 1;
    }
    if best_len >= MIN_MATCH {
        Some((best_len, best_dist))
    } else {
        None
    }
}

pub(super) fn length_symbol(len: usize) -> (usize, u32, u8) {
    for (idx, &base) in LEN_BASE.iter().enumerate().rev() {
        if len >= base as usize {
            return (257 + idx, (len - base as usize) as u32, LEN_EXTRA[idx]);
        }
    }
    unreachable!("length_symbol called with len < MIN_MATCH")
}

pub(super) fn distance_symbol(dist: usize) -> (usize, u32, u8) {
    for (idx, &base) in DIST_BASE.iter().enumerate().rev() {
        if dist >= base as usize {
            return (idx, (dist - base as usize) as u32, DIST_EXTRA[idx]);
        }
    }
    unreachable!("distance_symbol called with dist == 0")
}
//#endregion Lz77

//#region 🧵️StreamingEncode
pub(super) const DEFLATE_MAX_CHAIN: usize = 128;

pub(super) const DEFLATE_CHECKPOINT_MAGIC: &[u8; 4] = b"SDJ1";

#[derive(Clone, Copy)]
pub(super) struct PendingMatch {
    pub(super) start: usize,
    pub(super) length: usize,
    pub(super) distance: usize,
}
//#endregion 🧵️StreamingEncode
//#endregion DeflateCodec
//#endregion 🦑️DissolvedEngineCodec
