//! 🧮️ The DWG bit codes (B, BB, BS, BL, BD, DD, H, RC, RS, RL, RD, CMC, ENC, OT, MS, MC) and the three parallel streams of an R2007+ record (data, strings, handles) that merge into one byte-aligned record.
//! <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf> (2 Bit codes, 20.1 Common non-entity object format)

use crate::checksum::{crc16, CRC16_SEED};

/// 🔌️ How a handle reference relates its holder to its target (the high nibble of the handle code).
#[derive(Clone, Copy)]
pub enum Ref {
    Absolute = 0,
    SoftOwner = 2,
    HardOwner = 3,
    SoftPointer = 4,
    HardPointer = 5,
}

/// 🌊️ A growing bit string, most significant bit first.
#[derive(Default, Clone)]
pub struct Bits {
    bytes: Vec<u8>,
    len: usize,
}

impl Bits {
    /// ➕️ An empty bit string.
    pub fn new() -> Bits {
        Bits::default()
    }

    /// 📏️ The number of bits written.
    pub fn len(&self) -> usize {
        self.len
    }

    /// ❔️ Whether no bit was written.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 🔘️ Writes one bit (B).
    pub fn bit(&mut self, value: bool) {
        if self.len.is_multiple_of(8) {
            self.bytes.push(0);
        }
        if value {
            self.bytes[self.len / 8] |= 0x80 >> (self.len % 8);
        }
        self.len += 1;
    }

    /// 🔢️ Writes the low `count` bits of `value`, most significant first.
    pub fn bits(&mut self, value: u64, count: u32) {
        for shift in (0..count).rev() {
            self.bit((value >> shift) & 1 == 1);
        }
    }

    /// 🧱️ Appends every bit of `other`.
    pub fn append(&mut self, other: &Bits) {
        if self.len.is_multiple_of(8) {
            self.bytes.extend_from_slice(&other.bytes);
            self.len += other.len;
            return;
        }
        for at in 0..other.len {
            self.bit(other.bytes[at / 8] & (0x80 >> (at % 8)) != 0);
        }
    }

    /// 📐️ Pads with zero bits up to the next byte boundary.
    pub fn align(&mut self) {
        while !self.len.is_multiple_of(8) {
            self.bit(false);
        }
    }

    /// 📦️ The bytes, the last one padded with zero bits.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// 🔡️ Raw byte (RC).
    pub fn rc(&mut self, value: u8) {
        self.bits(u64::from(value), 8);
    }

    /// 🧵️ Raw bytes.
    pub fn raw(&mut self, values: &[u8]) {
        values.iter().for_each(|value| self.rc(*value));
    }

    /// 📎️ Raw little-endian short (RS).
    pub fn rs(&mut self, value: u16) {
        self.raw(&value.to_le_bytes());
    }

    /// 📌️ Raw little-endian long (RL).
    pub fn rl(&mut self, value: u32) {
        self.raw(&value.to_le_bytes());
    }

    /// 🎯️ Raw little-endian double (RD).
    pub fn rd(&mut self, value: f64) {
        self.raw(&value.to_le_bytes());
    }

    /// 🧭️ Two raw doubles (2RD).
    pub fn rd2(&mut self, x: f64, y: f64) {
        self.rd(x);
        self.rd(y);
    }

    /// 🔟️ Bit short (BS): 0, 1..255 as a byte, 256, or a raw short.
    pub fn bs(&mut self, value: i16) {
        match value {
            0 => self.bits(2, 2),
            1..=255 => {
                self.bits(1, 2);
                self.rc(value as u8);
            }
            256 => self.bits(3, 2),
            _ => {
                self.bits(0, 2);
                self.rs(value as u16);
            }
        }
    }

    /// 💯️ Bit long (BL): 0, 1..255 as a byte, or a raw long.
    pub fn bl(&mut self, value: i32) {
        match value {
            0 => self.bits(2, 2),
            1..=255 => {
                self.bits(1, 2);
                self.rc(value as u8);
            }
            _ => {
                self.bits(0, 2);
                self.rl(value as u32);
            }
        }
    }

    /// 🔬️ Bit double (BD): 0.0, 1.0 or a raw double.
    pub fn bd(&mut self, value: f64) {
        if value == 0.0 {
            self.bits(2, 2);
        } else if value == 1.0 {
            self.bits(1, 2);
        } else {
            self.bits(0, 2);
            self.rd(value);
        }
    }

    /// 🧊️ Three bit doubles (3BD).
    pub fn bd3(&mut self, x: f64, y: f64, z: f64) {
        self.bd(x);
        self.bd(y);
        self.bd(z);
    }

    /// 🪞️ Bit double with default (DD): the default itself, or the bytes of `value` that differ from it.
    pub fn dd(&mut self, value: f64, default: f64) {
        let (new, old) = (value.to_le_bytes(), default.to_le_bytes());
        if new == old {
            self.bits(0, 2);
        } else if new[4..] == old[4..] {
            self.bits(1, 2);
            self.raw(&new[..4]);
        } else if new[6..] == old[6..] {
            self.bits(2, 2);
            self.raw(&new[4..6]);
            self.raw(&new[..4]);
        } else {
            self.bits(3, 2);
            self.rd(value);
        }
    }

    /// 🔗️ Handle reference (H): code and byte count in one byte, then the big-endian value.
    pub fn handle(&mut self, kind: Ref, value: u64) {
        let count = (8 - value.leading_zeros() / 8) as u8;
        self.rc(((kind as u8) << 4) | count);
        self.raw(&value.to_be_bytes()[8 - usize::from(count)..]);
    }

    /// 🎨️ Colour as written in tables, headers and styles (CMC, R2004+): a legacy zero, the packed value, no names.
    pub fn cmc(&mut self, packed: u32) {
        self.bs(0);
        self.bl(packed as i32);
        self.rc(0);
    }

    /// 🏷️ Object type (OT, R2010+): a bit pair then one or two bytes.
    pub fn object_type(&mut self, value: u16) {
        match value {
            0..=255 => {
                self.bits(0, 2);
                self.rc(value as u8);
            }
            0x1F0..=0x2EF => {
                self.bits(1, 2);
                self.rc((value - 0x1F0) as u8);
            }
            _ => {
                self.bits(2, 2);
                self.rs(value);
            }
        }
    }

    /// 🔤️ Unicode text (TU, R2007+): character count then UTF-16LE.
    pub fn text(&mut self, value: &str) {
        let units: Vec<u16> = value.encode_utf16().collect();
        self.bs(units.len() as i16);
        units.into_iter().for_each(|unit| self.rs(unit));
    }
}

/// 🌐️ The packed CMC colour value of an ACI index (0xC3 in the high byte).
pub const fn aci(index: u16) -> u32 {
    0xC300_0000 | index as u32
}

/// 🥇️ The packed CMC colour value "by layer".
pub const BY_LAYER: u32 = 0xC000_0000;

/// 🥈️ The packed CMC colour value "by block".
pub const BY_BLOCK: u32 = 0xC100_0000;

/// 🛤️ The data, string and handle streams of one record.
#[derive(Default)]
pub struct Streams {
    pub main: Bits,
    pub text: Bits,
    pub handles: Bits,
}

impl Streams {
    /// 🆕️ Three empty streams.
    pub fn new() -> Streams {
        Streams::default()
    }

    /// ✍️ Writes a string into the string stream.
    pub fn string(&mut self, value: &str) {
        self.text.text(value);
    }

    /// ⚓️ Writes a reference into the handle stream.
    pub fn reference(&mut self, kind: Ref, value: u64) {
        self.handles.handle(kind, value);
    }

    /// 🧷️ Merges data, strings (with their size words and the presence flag) and handles; with `size_prefix` the first 32 bits of the data (a placeholder written by the caller) are patched with the bit position after the flag. Returns the bytes and the bit size of the handle stream including the final padding.
    pub fn merge(self, size_prefix: bool) -> (Vec<u8>, usize) {
        let mut out = self.main;
        let text_bits = self.text.len();
        if text_bits > 0 {
            out.append(&self.text);
            if text_bits < 0x8000 {
                out.rs(text_bits as u16);
            } else {
                out.rs((text_bits >> 15) as u16);
                out.rs(((text_bits & 0x7FFF) | 0x8000) as u16);
            }
            out.bit(true);
        } else {
            out.bit(false);
        }
        let handle_start = out.len();
        if size_prefix {
            out.bytes[..4].copy_from_slice(&(handle_start as u32).to_le_bytes());
        }
        out.append(&self.handles);
        out.align();
        let handle_bits = out.len() - handle_start;
        (out.bytes, handle_bits)
    }
}

/// 📶️ Modular short (MS): 15 data bits per little-endian word, bit 15 of the high byte flags another word.
pub fn modular_short(out: &mut Vec<u8>, value: usize) {
    if value < 0x8000 {
        out.extend_from_slice(&(value as u16).to_le_bytes());
    } else {
        out.extend_from_slice(&[value as u8, ((value >> 8) as u8 & 0x7F) | 0x80, (value >> 15) as u8, (value >> 23) as u8]);
    }
}

/// 📳️ Modular char (MC), unsigned: 7 data bits per byte, bit 7 flags another byte.
pub fn modular_char(out: &mut Vec<u8>, mut value: usize) {
    loop {
        let byte = (value & 0x7F) as u8;
        value >>= 7;
        if value == 0 {
            out.push(byte);
            return;
        }
        out.push(byte | 0x80);
    }
}

/// 🧾️ Frames a merged record for the object section: MS size, MC handle-stream bits, data, CRC-16 over all three.
pub fn frame(data: &[u8], handle_bits: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() + 8);
    modular_short(&mut out, data.len());
    modular_char(&mut out, handle_bits);
    out.extend_from_slice(data);
    let crc = crc16(CRC16_SEED, &out);
    out.extend_from_slice(&crc.to_le_bytes());
    out
}
