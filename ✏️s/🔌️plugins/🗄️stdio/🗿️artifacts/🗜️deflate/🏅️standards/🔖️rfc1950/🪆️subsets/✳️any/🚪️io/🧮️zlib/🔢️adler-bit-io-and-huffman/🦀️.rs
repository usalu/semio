//! 🔢️ Checksum, bit IO and Huffman of the zlib codec: the Adler-32 checksum, the bit writer and reader, the canonical code builder, the Huffman decoder and the fixed code lengths. See [`super`].


//#region 🦑️DissolvedEngineCodec
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
#[derive(value_derive::ToValue, value_derive::FromValue)]
pub(super) struct BitWriter {
    pub(super) out: Vec<u8>,
    pub(super) cur: u8,
    pub(super) nbits: u8,
}

impl BitWriter {
    pub(super) fn new() -> Self {
        Self { out: Vec::new(), cur: 0, nbits: 0 }
    }
    pub(super) fn write_bits(&mut self, mut value: u32, mut count: u8) {
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
    pub(super) fn align_byte(&mut self) {
        if self.nbits > 0 {
            self.out.push(self.cur);
            self.cur = 0;
            self.nbits = 0;
        }
    }
}

pub(super) struct BitReader<'a> {
    pub(super) data: &'a [u8],
    pub(super) pos: usize,
    pub(super) cur: u8,
    pub(super) nbits: u8,
}

impl<'a> BitReader<'a> {
    pub(super) fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0, cur: 0, nbits: 0 }
    }
    pub(super) fn read_bits(&mut self, count: u8) -> Result<u32, String> {
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
    pub(super) fn align_byte(&mut self) {
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

pub(super) fn build_codes(lengths: &[u8]) -> Vec<(u32, u8)> {
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

pub(super) struct HuffDecoder {
    table: Vec<Option<(u16, u8)>>,
    max_bits: u8,
}

impl HuffDecoder {
    pub(super) fn from_lengths(lengths: &[u8]) -> Result<Self, String> {
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

    pub(super) fn decode(&self, br: &mut BitReader<'_>) -> Result<u16, String> {
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

pub(super) fn fixed_lit_lengths() -> Vec<u8> {
    let mut l = vec![0u8; 288];
    l[..=143].fill(8);
    l[144..=255].fill(9);
    l[256..=279].fill(7);
    l[280..=287].fill(8);
    l
}

pub(super) fn fixed_dist_lengths() -> Vec<u8> {
    vec![5u8; 32]
}
//#endregion Huffman
//#endregion 🦑️DissolvedEngineCodec
