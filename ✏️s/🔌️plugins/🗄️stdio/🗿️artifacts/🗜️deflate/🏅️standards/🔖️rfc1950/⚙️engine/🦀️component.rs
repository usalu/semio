//! ⚙️ DeflateEngine — zlib (RFC1950) + DEFLATE (RFC1951) + Adler32, hand-rolled.

use crate::artifacts::deflate::{DeflateArtifact, DeflateDiff, DeflateMutation, DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};

//#region Adler32
/// 🧮 Adler-32 (RFC1950).
pub fn adler32(data: &[u8]) -> u32 {
    const MOD: u32 = 65521;
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % MOD;
        b = (b + a) % MOD;
    }
    (b << 16) | a
}
//#endregion Adler32

//#region BitIO
struct BitWriter {
    out: Vec<u8>,
    cur: u8,
    nbits: u8,
}

impl BitWriter {
    fn new() -> Self {
        Self { out: Vec::new(), cur: 0, nbits: 0 }
    }
    fn write_bits(&mut self, mut value: u32, mut count: u8) {
        while count > 0 {
            let take = (8 - self.nbits).min(count);
            let mask = (1u32 << take) - 1;
            self.cur |= ((value & mask) as u8) << self.nbits;
            self.nbits += take;
            value >>= take;
            count -= take;
            if self.nbits == 8 {
                self.out.push(self.cur);
                self.cur = 0;
                self.nbits = 0;
            }
        }
    }
    fn align_byte(&mut self) {
        if self.nbits > 0 {
            self.out.push(self.cur);
            self.cur = 0;
            self.nbits = 0;
        }
    }
    fn finish(mut self) -> Vec<u8> {
        self.align_byte();
        self.out
    }
}

struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    cur: u8,
    nbits: u8,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0, cur: 0, nbits: 0 }
    }
    fn read_bits(&mut self, count: u8) -> Result<u32, String> {
        let mut out = 0u32;
        for i in 0..count {
            if self.nbits == 0 {
                if self.pos >= self.data.len() {
                    return Err("unexpected end of deflate stream".into());
                }
                self.cur = self.data[self.pos];
                self.pos += 1;
                self.nbits = 8;
            }
            let bit = (self.cur & 1) as u32;
            self.cur >>= 1;
            self.nbits -= 1;
            out |= bit << i;
        }
        Ok(out)
    }
    fn align_byte(&mut self) {
        self.nbits = 0;
        self.cur = 0;
    }
}
//#endregion BitIO

//#region Huffman
fn reverse_bits(mut v: u32, len: u8) -> u32 {
    let mut r = 0u32;
    for _ in 0..len {
        r = (r << 1) | (v & 1);
        v >>= 1;
    }
    r
}

fn build_codes(lengths: &[u8]) -> Vec<(u32, u8)> {
    let mut bl_count = [0u32; 16];
    for &l in lengths {
        if l > 0 {
            bl_count[l as usize] += 1;
        }
    }
    let mut next_code = [0u32; 16];
    let mut code = 0u32;
    for bits in 1..=15 {
        code = (code + bl_count[bits - 1]) << 1;
        next_code[bits] = code;
    }
    let mut codes = vec![(0u32, 0u8); lengths.len()];
    for (i, &len) in lengths.iter().enumerate() {
        if len != 0 {
            let c = next_code[len as usize];
            next_code[len as usize] += 1;
            codes[i] = (reverse_bits(c, len), len);
        }
    }
    codes
}

struct HuffDecoder {
    table: Vec<Option<(u16, u8)>>,
    max_bits: u8,
}

impl HuffDecoder {
    fn from_lengths(lengths: &[u8]) -> Result<Self, String> {
        let max_bits = lengths.iter().copied().max().unwrap_or(0);
        if max_bits > 15 {
            return Err("invalid huffman length".into());
        }
        let size = 1usize << max_bits;
        let mut table = vec![None; size.max(1)];
        let codes = build_codes(lengths);
        for (sym, &(code, len)) in codes.iter().enumerate() {
            if len == 0 {
                continue;
            }
            let step = 1usize << len;
            let mut fill = code as usize;
            while fill < size {
                table[fill] = Some((sym as u16, len));
                fill += step;
            }
        }
        Ok(Self { table, max_bits })
    }

    fn decode(&self, br: &mut BitReader<'_>) -> Result<u16, String> {
        if self.max_bits == 0 {
            return Err("empty huffman alphabet".into());
        }
        let mut acc = 0u32;
        for len in 1..=self.max_bits {
            let bit = br.read_bits(1)?;
            acc |= bit << (len - 1);
            if let Some(Some((sym, l))) = self.table.get(acc as usize) {
                if *l == len {
                    return Ok(*sym);
                }
            }
        }
        Err("invalid huffman symbol".into())
    }
}

fn fixed_lit_lengths() -> Vec<u8> {
    let mut l = vec![0u8; 288];
    for i in 0..=143 { l[i] = 8; }
    for i in 144..=255 { l[i] = 9; }
    for i in 256..=279 { l[i] = 7; }
    for i in 280..=287 { l[i] = 8; }
    l
}

fn fixed_dist_lengths() -> Vec<u8> {
    vec![5u8; 32]
}
//#endregion Huffman

//#region DeflateCodec
const LEN_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LEN_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049,
    3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13,
];

/// 🗜️ Raw DEFLATE compress (fixed Huffman literals + end).
pub fn deflate_raw(data: &[u8]) -> Vec<u8> {
    let lit_codes = build_codes(&fixed_lit_lengths());
    let mut bw = BitWriter::new();
    bw.write_bits(0b011, 3);
    for &byte in data {
        let (code, len) = lit_codes[byte as usize];
        bw.write_bits(code, len);
    }
    let (eob, elen) = lit_codes[256];
    bw.write_bits(eob, elen);
    bw.finish()
}

fn inflate_block_stored(br: &mut BitReader<'_>, out: &mut Vec<u8>) -> Result<(), String> {
    br.align_byte();
    if br.pos + 4 > br.data.len() {
        return Err("truncated stored block".into());
    }
    let len = u16::from_le_bytes([br.data[br.pos], br.data[br.pos + 1]]);
    let nlen = u16::from_le_bytes([br.data[br.pos + 2], br.data[br.pos + 3]]);
    br.pos += 4;
    if len ^ 0xFFFF != nlen {
        return Err("stored block LEN/NLEN mismatch".into());
    }
    let end = br.pos + len as usize;
    if end > br.data.len() {
        return Err("truncated stored payload".into());
    }
    out.extend_from_slice(&br.data[br.pos..end]);
    br.pos = end;
    Ok(())
}

fn inflate_codes(
    br: &mut BitReader<'_>,
    out: &mut Vec<u8>,
    lit: &HuffDecoder,
    dist: &HuffDecoder,
) -> Result<(), String> {
    loop {
        let sym = lit.decode(br)? as usize;
        if sym < 256 {
            out.push(sym as u8);
        } else if sym == 256 {
            break;
        } else if sym <= 285 {
            let idx = sym - 257;
            if idx >= LEN_BASE.len() {
                return Err("invalid length symbol".into());
            }
            let mut length = LEN_BASE[idx] as usize;
            let extra = LEN_EXTRA[idx];
            if extra > 0 {
                length += br.read_bits(extra)? as usize;
            }
            let dsym = dist.decode(br)? as usize;
            if dsym >= DIST_BASE.len() {
                return Err("invalid distance symbol".into());
            }
            let mut distance = DIST_BASE[dsym] as usize;
            let dextra = DIST_EXTRA[dsym];
            if dextra > 0 {
                distance += br.read_bits(dextra)? as usize;
            }
            if distance == 0 || distance > out.len() {
                return Err("invalid backreference".into());
            }
            for _ in 0..length {
                let b = out[out.len() - distance];
                out.push(b);
            }
        } else {
            return Err("invalid lit/len symbol".into());
        }
    }
    Ok(())
}

fn inflate_dynamic(br: &mut BitReader<'_>, out: &mut Vec<u8>) -> Result<(), String> {
    let hlit = br.read_bits(5)? as usize + 257;
    let hdist = br.read_bits(5)? as usize + 1;
    let hclen = br.read_bits(4)? as usize + 4;
    const ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
    let mut cl_lens = vec![0u8; 19];
    for i in 0..hclen {
        cl_lens[ORDER[i]] = br.read_bits(3)? as u8;
    }
    let cl_dec = HuffDecoder::from_lengths(&cl_lens)?;
    let mut lens = Vec::with_capacity(hlit + hdist);
    while lens.len() < hlit + hdist {
        let sym = cl_dec.decode(br)? as usize;
        match sym {
            0..=15 => lens.push(sym as u8),
            16 => {
                let rep = br.read_bits(2)? as usize + 3;
                let prev = *lens.last().ok_or("bad repeat")?;
                lens.extend(std::iter::repeat(prev).take(rep));
            }
            17 => {
                let rep = br.read_bits(3)? as usize + 3;
                lens.extend(std::iter::repeat(0u8).take(rep));
            }
            18 => {
                let rep = br.read_bits(7)? as usize + 11;
                lens.extend(std::iter::repeat(0u8).take(rep));
            }
            _ => return Err("bad code-length symbol".into()),
        }
    }
    if lens.len() < hlit + hdist {
        return Err("incomplete dynamic trees".into());
    }
    let lit = HuffDecoder::from_lengths(&lens[..hlit])?;
    let dist = HuffDecoder::from_lengths(&lens[hlit..hlit + hdist])?;
    inflate_codes(br, out, &lit, &dist)
}

/// 🗜️ Raw DEFLATE inflate (stored + fixed + dynamic).
pub fn inflate_raw(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut br = BitReader::new(data);
    let mut out = Vec::new();
    loop {
        let bfinal = br.read_bits(1)?;
        let btype = br.read_bits(2)?;
        match btype {
            0 => inflate_block_stored(&mut br, &mut out)?,
            1 => {
                let lit = HuffDecoder::from_lengths(&fixed_lit_lengths())?;
                let dist = HuffDecoder::from_lengths(&fixed_dist_lengths())?;
                inflate_codes(&mut br, &mut out, &lit, &dist)?;
            }
            2 => inflate_dynamic(&mut br, &mut out)?,
            _ => return Err("reserved BTYPE".into()),
        }
        if bfinal == 1 {
            break;
        }
    }
    Ok(out)
}

/// 🗜️ Zlib-wrap compress (CMF/FLG + raw deflate + Adler32).
pub fn zlib_compress(data: &[u8]) -> Result<Vec<u8>, String> {
    let raw = deflate_raw(data);
    let mut out = Vec::with_capacity(2 + raw.len() + 4);
    out.push(0x78);
    out.push(0x01);
    out.extend_from_slice(&raw);
    out.extend_from_slice(&adler32(data).to_be_bytes());
    Ok(out)
}

/// 🗜️ Zlib unwrap + inflate + Adler32 verify.
pub fn zlib_decompress(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 6 {
        return Err("zlib stream too short".into());
    }
    let cmf = data[0];
    let flg = data[1];
    if (cmf & 0x0F) != 8 {
        return Err("unsupported zlib compression method".into());
    }
    if ((cmf as u16) * 256 + flg as u16) % 31 != 0 {
        return Err("zlib CMF/FLG check failed".into());
    }
    if flg & 0x20 != 0 {
        return Err("zlib preset dictionary not supported".into());
    }
    let adler_bytes = &data[data.len() - 4..];
    let expect = u32::from_be_bytes([adler_bytes[0], adler_bytes[1], adler_bytes[2], adler_bytes[3]]);
    let raw = &data[2..data.len() - 4];
    let out = inflate_raw(raw)?;
    let got = adler32(&out);
    if got != expect {
        return Err(format!("adler32 mismatch: expected {expect:#010x}, got {got:#010x}"));
    }
    Ok(out)
}
//#endregion DeflateCodec

//#region DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_deflate_snapshot() -> DeflateSnapshot {
    DeflateSnapshot::default()
}
//#endregion DocumentHelpers

//#region Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::deflate::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::DocumentCodec::of::<DeflateSnapshot, DeflateMutation>(
        STDIO_DEFLATE_DOCUMENT_SCHEMA,
    ));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.deflate",
        extension: Some("zz"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::deflate::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::deflate::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::deflate::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::deflate::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.deflate"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.deflate`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(
        crate::artifacts::deflate::schema::deflate_artifact_schema_descriptor(),
    );
}
//#endregion Register

//#region ArtifactEngine
/// ⚙️ `stdio.deflate` artifact engine.
pub struct DeflateEngine {
    artifact_state: DeflateArtifact,
    snapshot_state: DeflateSnapshot,
}

impl DeflateEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: DeflateSnapshot) -> Self {
        let artifact_state = DeflateArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}

impl protocol::ArtifactEngine for DeflateEngine {
    type Artifact = DeflateArtifact;
    type Snapshot = DeflateSnapshot;
    type Mutation = DeflateMutation;
    type Diff = DeflateDiff;

    fn artifact(&self) -> &Self::Artifact { &self.artifact_state }
    fn snapshot(&self) -> &Self::Snapshot { &self.snapshot_state }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
    }
}
//#endregion ArtifactEngine

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adler32_empty_is_one() {
        assert_eq!(adler32(b""), 1);
    }

    #[test]
    fn zlib_round_trip() {
        let payloads: &[&[u8]] = &[b"", b"a", b"hello zlib", &[0u8; 64], b"abracadabra abracadabra"];
        for p in payloads {
            let enc = zlib_compress(p).expect("compress");
            let dec = zlib_decompress(&enc).expect("decompress");
            assert_eq!(&dec, p);
        }
    }

    #[test]
    fn raw_deflate_round_trip() {
        let p = b"stdio-deflate-conformance";
        let enc = deflate_raw(p);
        let dec = inflate_raw(&enc).expect("inflate");
        assert_eq!(dec, p);
    }

    #[test]
    fn codec_round_trip() {
        let payload = b"pack-envelope-payload";
        let zz = zlib_compress(payload).unwrap();
        let snap = DeflateSnapshot {
            schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
            bytes: zz.clone(),
        };
        let pack = store::DocumentPack::encode_pack(&snap);
        let decoded = <DeflateSnapshot as store::DocumentPack>::decode_pack(&pack).expect("decode");
        assert_eq!(decoded.bytes, zz);
        assert_eq!(zlib_decompress(&decoded.bytes).unwrap(), payload);
    }
}
//#endregion Tests
