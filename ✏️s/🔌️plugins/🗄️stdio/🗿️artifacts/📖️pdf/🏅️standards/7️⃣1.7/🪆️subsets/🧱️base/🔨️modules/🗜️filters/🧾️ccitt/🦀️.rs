//! 🧾️ CCITT Group 3 and Group 4 fax decoding for `/CCITTFaxDecode` images.
//! [ITU-T T.4 and T.6](https://www.itu.int/rec/T-REC-T.6)

use std::collections::HashMap;
use std::sync::OnceLock;

use super::{Control, RasterError};

const MAX_ROWS: usize = 1 << 20;

/// 🎛️ The `/CCITTFaxDecode` decode parameters (ISO 32000-1 Table 11).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CcittParameters {
    pub k: i32,
    pub columns: u32,
    pub rows: u32,
    pub black_is_1: bool,
    pub byte_align: bool,
}

const WHITE: &[(u16, &str)] = &[
    (0, "00110101"), (1, "000111"), (2, "0111"), (3, "1000"), (4, "1011"), (5, "1100"), (6, "1110"), (7, "1111"), (8, "10011"), (9, "10100"),
    (10, "00111"), (11, "01000"), (12, "001000"), (13, "000011"), (14, "110100"), (15, "110101"), (16, "101010"), (17, "101011"), (18, "0100111"),
    (19, "0001100"), (20, "0001000"), (21, "0010111"), (22, "0000011"), (23, "0000100"), (24, "0101000"), (25, "0101011"), (26, "0010011"),
    (27, "0100100"), (28, "0011000"), (29, "00000010"), (30, "00000011"), (31, "00011010"), (32, "00011011"), (33, "00010010"), (34, "00010011"),
    (35, "00010100"), (36, "00010101"), (37, "00010110"), (38, "00010111"), (39, "00101000"), (40, "00101001"), (41, "00101010"), (42, "00101011"),
    (43, "00101100"), (44, "00101101"), (45, "00000100"), (46, "00000101"), (47, "00001010"), (48, "00001011"), (49, "01010010"), (50, "01010011"),
    (51, "01010100"), (52, "01010101"), (53, "00100100"), (54, "00100101"), (55, "01011000"), (56, "01011001"), (57, "01011010"), (58, "01011011"),
    (59, "01001010"), (60, "01001011"), (61, "00110010"), (62, "00110011"), (63, "00110100"),
    (64, "11011"), (128, "10010"), (192, "010111"), (256, "0110111"), (320, "00110110"), (384, "00110111"), (448, "01100100"), (512, "01100101"),
    (576, "01101000"), (640, "01100111"), (704, "011001100"), (768, "011001101"), (832, "011010010"), (896, "011010011"), (960, "011010100"),
    (1024, "011010101"), (1088, "011010110"), (1152, "011010111"), (1216, "011011000"), (1280, "011011001"), (1344, "011011010"), (1408, "011011011"),
    (1472, "010011000"), (1536, "010011001"), (1600, "010011010"), (1664, "011000"), (1728, "010011011"),
];

const BLACK: &[(u16, &str)] = &[
    (0, "0000110111"), (1, "010"), (2, "11"), (3, "10"), (4, "011"), (5, "0011"), (6, "0010"), (7, "00011"), (8, "000101"), (9, "000100"),
    (10, "0000100"), (11, "0000101"), (12, "0000111"), (13, "00000100"), (14, "00000111"), (15, "000011000"), (16, "0000010111"), (17, "0000011000"),
    (18, "0000001000"), (19, "00001100111"), (20, "00001101000"), (21, "00001101100"), (22, "00000110111"), (23, "00000101000"), (24, "00000010111"),
    (25, "00000011000"), (26, "000011001010"), (27, "000011001011"), (28, "000011001100"), (29, "000011001101"), (30, "000001101000"),
    (31, "000001101001"), (32, "000001101010"), (33, "000001101011"), (34, "000011010010"), (35, "000011010011"), (36, "000011010100"),
    (37, "000011010101"), (38, "000011010110"), (39, "000011010111"), (40, "000001101100"), (41, "000001101101"), (42, "000011011010"),
    (43, "000011011011"), (44, "000001010100"), (45, "000001010101"), (46, "000001010110"), (47, "000001010111"), (48, "000001100100"),
    (49, "000001100101"), (50, "000001010010"), (51, "000001010011"), (52, "000000100100"), (53, "000000110111"), (54, "000000111000"),
    (55, "000000100111"), (56, "000000101000"), (57, "000001011000"), (58, "000001011001"), (59, "000000101011"), (60, "000000101100"),
    (61, "000001011010"), (62, "000001100110"), (63, "000001100111"),
    (64, "0000001111"), (128, "000011001000"), (192, "000011001001"), (256, "000001011011"), (320, "000000110011"), (384, "000000110100"),
    (448, "000000110101"), (512, "0000001101100"), (576, "0000001101101"), (640, "0000001001010"), (704, "0000001001011"), (768, "0000001001100"),
    (832, "0000001001101"), (896, "0000001110010"), (960, "0000001110011"), (1024, "0000001110100"), (1088, "0000001110101"),
    (1152, "0000001110110"), (1216, "0000001110111"), (1280, "0000001010010"), (1344, "0000001010011"), (1408, "0000001010100"),
    (1472, "0000001010101"), (1536, "0000001011010"), (1600, "0000001011011"), (1664, "0000001100100"), (1728, "0000001100101"),
];

const EXTENDED: &[(u16, &str)] = &[
    (1792, "00000001000"), (1856, "00000001100"), (1920, "00000001101"), (1984, "000000010010"), (2048, "000000010011"), (2112, "000000010100"),
    (2176, "000000010101"), (2240, "000000010110"), (2304, "000000010111"), (2368, "000000011100"), (2432, "000000011101"), (2496, "000000011110"),
    (2560, "000000011111"),
];

type RunTable = HashMap<(u8, u16), u16>;

fn tables() -> &'static [RunTable; 2] {
    static TABLES: OnceLock<[RunTable; 2]> = OnceLock::new();
    TABLES.get_or_init(|| {
        let build = |codes: &[(u16, &str)]| -> RunTable {
            codes.iter().chain(EXTENDED).map(|&(run, bits)| ((bits.len() as u8, u16::from_str_radix(bits, 2).unwrap()), run)).collect()
        };
        [build(WHITE), build(BLACK)]
    })
}

struct Bits<'a> {
    data: &'a [u8],
    position: usize,
}

impl Bits<'_> {
    fn total(&self) -> usize { self.data.len() * 8 }

    fn bit(&self, at: usize) -> u32 { u32::from(self.data[at >> 3] >> (7 - (at & 7)) & 1) }

    fn peek(&self, count: usize) -> u32 {
        (0..count).fold(0, |value, offset| value << 1 | if self.position + offset < self.total() { self.bit(self.position + offset) } else { 0 })
    }

    fn align(&mut self) { self.position = self.position.div_ceil(8) * 8; }

    fn at_end(&self) -> bool { self.position >= self.total() }

    fn skip_eol(&mut self) -> bool {
        let mut at = self.position;
        while at < self.total() && self.bit(at) == 0 { at += 1; }
        if at < self.total() && at - self.position >= 11 {
            self.position = at + 1;
            return true;
        }
        false
    }

    fn run(&mut self, black: bool) -> Result<u32, RasterError> {
        let table = &tables()[usize::from(black)];
        let mut total = 0u32;
        loop {
            let mut found = None;
            for length in 2..=13u8 {
                if self.position + length as usize > self.total() { break; }
                if let Some(&run) = table.get(&(length, self.peek(length as usize) as u16)) {
                    found = Some((length, run));
                    break;
                }
            }
            let (length, run) = found.ok_or(RasterError::Malformed("invalid CCITT run code"))?;
            self.position += length as usize;
            total += u32::from(run);
            if run < 64 { return Ok(total); }
        }
    }
}

enum Mode { Pass, Horizontal, Vertical(i32), Eol, Extension }

fn mode(bits: &mut Bits<'_>) -> Result<Mode, RasterError> {
    let table: [(u8, u32, Mode); 9] = [
        (1, 0b1, Mode::Vertical(0)), (3, 0b011, Mode::Vertical(1)), (3, 0b010, Mode::Vertical(-1)), (3, 0b001, Mode::Horizontal),
        (4, 0b0001, Mode::Pass), (6, 0b000011, Mode::Vertical(2)), (6, 0b000010, Mode::Vertical(-2)), (7, 0b0000011, Mode::Vertical(3)),
        (7, 0b0000010, Mode::Vertical(-3)),
    ];
    for (length, code, found) in table {
        if bits.peek(length as usize) == code {
            bits.position += length as usize;
            return Ok(found);
        }
    }
    if bits.peek(7) == 0b0000001 { bits.position += 7; return Ok(Mode::Extension); }
    if bits.peek(12) == 1 || bits.peek(11) == 0 { return Ok(Mode::Eol); }
    Err(RasterError::Malformed("invalid CCITT two-dimensional mode"))
}

fn row_1d(bits: &mut Bits<'_>, columns: u32) -> Result<Vec<u32>, RasterError> {
    let (mut changes, mut at, mut black) = (Vec::new(), 0u32, false);
    while at < columns {
        at = (at + bits.run(black)?).min(columns);
        changes.push(at);
        black = !black;
    }
    Ok(changes)
}

fn row_2d(bits: &mut Bits<'_>, reference: &[u32], columns: u32) -> Result<Vec<u32>, RasterError> {
    let (mut changes, mut a0, mut black) = (Vec::<u32>::new(), -1i64, false);
    while a0 < i64::from(columns) {
        let mut index = usize::from(black);
        while index < reference.len() && i64::from(reference[index]) <= a0 { index += 2; }
        let b1 = reference.get(index).copied().unwrap_or(columns);
        let b2 = reference.get(index + 1).copied().unwrap_or(columns);
        match mode(bits)? {
            Mode::Pass => a0 = i64::from(b2),
            Mode::Horizontal => {
                let start = a0.max(0) as u32;
                let first = bits.run(black)?;
                let second = bits.run(!black)?;
                let one = (start + first).min(columns);
                let two = (one + second).min(columns);
                changes.push(one);
                changes.push(two);
                a0 = i64::from(two);
            }
            Mode::Vertical(delta) => {
                let a1 = (i64::from(b1) + i64::from(delta)).clamp(a0.max(0), i64::from(columns)) as u32;
                changes.push(a1);
                a0 = i64::from(a1);
                black = !black;
                if a1 >= columns { break; }
            }
            Mode::Eol | Mode::Extension => return Err(RasterError::Malformed("unexpected CCITT code inside a row")),
        }
    }
    Ok(changes)
}

fn paint(changes: &[u32], columns: u32, black_is_1: bool, out: &mut Vec<u8>) {
    let stride = columns.div_ceil(8) as usize;
    let base = out.len();
    out.resize(base + stride, if black_is_1 { 0 } else { 0xFF });
    let mut black = false;
    let mut at = 0u32;
    for &edge in changes.iter().chain(std::iter::once(&columns)) {
        if black {
            for x in at..edge.min(columns) {
                let byte = base + (x >> 3) as usize;
                let mask = 0x80u8 >> (x & 7);
                if black_is_1 { out[byte] |= mask } else { out[byte] &= !mask }
            }
        }
        at = edge.min(columns);
        black = !black;
    }
    if !black_is_1 && columns & 7 != 0 { out[base + stride - 1] |= 0xFFu8 >> (columns & 7); }
}

/// 🧾️ Decodes a CCITT stream into packed 1-bit rows (`BlackIs1` decides the bit sense).
pub fn decode(data: &[u8], parameters: &CcittParameters, control: Control<'_>) -> Result<Vec<u8>, RasterError> {
    let columns = parameters.columns.max(1);
    let limit = if parameters.rows > 0 { parameters.rows as usize } else { MAX_ROWS };
    let mut bits = Bits { data, position: 0 };
    let (mut out, mut reference, mut rows) = (Vec::new(), Vec::<u32>::new(), 0usize);
    while rows < limit && !bits.at_end() {
        if rows % 64 == 0 && !control(rows as f32 / limit.min(MAX_ROWS) as f32) { return Err(RasterError::Cancelled); }
        if parameters.byte_align { bits.align(); }
        let mut two_dimensional = parameters.k < 0;
        if parameters.k >= 0 {
            let had_eol = bits.skip_eol();
            if parameters.k > 0 && had_eol {
                two_dimensional = bits.peek(1) == 0;
                bits.position += 1;
            }
            while bits.peek(12) == 1 { bits.skip_eol(); }
            if bits.at_end() { break; }
        } else if bits.peek(12) == 1 {
            break;
        }
        let changes = if two_dimensional { row_2d(&mut bits, &reference, columns) } else { row_1d(&mut bits, columns) };
        let changes = match changes {
            Ok(changes) => changes,
            Err(error) if rows == 0 => return Err(error),
            Err(_) => break,
        };
        paint(&changes, columns, parameters.black_is_1, &mut out);
        reference = changes;
        rows += 1;
    }
    if parameters.rows > 0 {
        let stride = columns.div_ceil(8) as usize;
        out.resize(parameters.rows as usize * stride, if parameters.black_is_1 { 0 } else { 0xFF });
    }
    Ok(out)
}
