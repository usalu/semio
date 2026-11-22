//! 🧱️ JPEG 2000 tier-1 EBCOT code-block decoding (significance, refinement and cleanup passes).
//! [ITU-T T.800 Annex D](https://www.itu.int/rec/T-REC-T.800)

use super::super::arithmetic::{Context, MqDecoder};

pub const STYLE_BYPASS: u8 = 0x01;
pub const STYLE_RESET: u8 = 0x02;
pub const STYLE_TERMINATE_ALL: u8 = 0x04;
pub const STYLE_VERTICAL_CAUSAL: u8 = 0x08;
pub const STYLE_SEGMENTATION: u8 = 0x20;

const SIGNIFICANT: u8 = 1;
const NEGATIVE: u8 = 2;
const REFINED: u8 = 4;
const VISITED: u8 = 8;
const RUN: usize = 17;

/// 🧱️ How many passes the codeword segment starting at pass `index` may hold.
pub fn segment_capacity(style: u8, index: u32) -> u32 {
    if style & STYLE_TERMINATE_ALL != 0 { 1 }
    else if style & STYLE_BYPASS != 0 { if index < 10 { 10 - index } else if index % 3 == 1 { 2 } else { 1 } }
    else { u32::MAX }
}

fn is_raw(style: u8, index: u32) -> bool { style & STYLE_BYPASS != 0 && index >= 10 && index % 3 != 0 }

struct RawBits<'a> { data: &'a [u8], position: usize, current: u8, count: u32 }

impl RawBits<'_> {
    fn bit(&mut self) -> u8 {
        if self.count == 0 {
            let previous = self.current;
            self.current = self.data.get(self.position).copied().unwrap_or(0xFF);
            self.position += 1;
            self.count = if previous == 0xFF { 7 } else { 8 };
        }
        self.count -= 1;
        self.current >> self.count & 1
    }
}

enum Coder<'a> { Mq(MqDecoder<'a>), Raw(RawBits<'a>) }

impl Coder<'_> {
    fn bit(&mut self, context: &mut Context) -> u8 { match self { Coder::Mq(mq) => mq.bit(context), Coder::Raw(raw) => raw.bit() } }
    fn raw(&self) -> bool { matches!(self, Coder::Raw(_)) }
}
const UNIFORM: usize = 18;

fn contexts() -> [Context; 19] {
    let mut all = [Context::default(); 19];
    all[0] = Context::with_state(4);
    all[RUN] = Context::with_state(3);
    all[UNIFORM] = Context::with_state(46);
    all
}

struct Block {
    width: usize,
    height: usize,
    orientation: u8,
    flags: Vec<u8>,
    magnitude: Vec<u32>,
    causal: bool,
}

impl Block {
    fn flag(&self, x: isize, y: isize, row: usize) -> u8 {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize { return 0; }
        if self.causal && y as usize > row && y as usize % 4 == 0 && row % 4 == 3 { return 0; }
        self.flags[y as usize * self.width + x as usize]
    }

    fn neighbours(&self, x: usize, y: usize) -> (u32, u32, u32) {
        let (xi, yi) = (x as isize, y as isize);
        let sig = |dx: isize, dy: isize| u32::from(self.flag(xi + dx, yi + dy, y) & SIGNIFICANT != 0);
        (sig(-1, 0) + sig(1, 0), sig(0, -1) + sig(0, 1), sig(-1, -1) + sig(1, -1) + sig(-1, 1) + sig(1, 1))
    }

    fn significance_context(&self, x: usize, y: usize) -> usize {
        let (mut h, mut v, d) = self.neighbours(x, y);
        if self.orientation == 1 { std::mem::swap(&mut h, &mut v); }
        if self.orientation == 3 {
            let hv = h + v;
            return match d { 0 => [0, 1, 2][hv.min(2) as usize], 1 => [3, 4, 5][hv.min(2) as usize], 2 => if hv >= 1 { 7 } else { 6 }, _ => 8 };
        }
        match (h, v, d) {
            (2, _, _) => 8,
            (1, v, _) if v >= 1 => 7,
            (1, _, d) => if d >= 1 { 6 } else { 5 },
            (_, 2, _) => 4,
            (_, 1, _) => 3,
            (_, _, d) => d.min(2) as usize,
        }
    }

    fn sign_contribution(&self, x: isize, y: isize, row: usize) -> i32 {
        let flag = self.flag(x, y, row);
        if flag & SIGNIFICANT == 0 { 0 } else if flag & NEGATIVE != 0 { -1 } else { 1 }
    }

    fn sign_context(&self, x: usize, y: usize) -> (usize, u8) {
        let (xi, yi) = (x as isize, y as isize);
        let h = (self.sign_contribution(xi - 1, yi, y) + self.sign_contribution(xi + 1, yi, y)).clamp(-1, 1);
        let v = (self.sign_contribution(xi, yi - 1, y) + self.sign_contribution(xi, yi + 1, y)).clamp(-1, 1);
        match (h, v) {
            (1, 1) => (13, 0), (1, 0) => (12, 0), (1, -1) => (11, 0),
            (0, 1) => (10, 0), (0, 0) => (9, 0), (0, -1) => (10, 1),
            (-1, 1) => (11, 1), (-1, 0) => (12, 1), _ => (13, 1),
        }
    }

    fn refinement_context(&self, x: usize, y: usize) -> usize {
        if self.flags[y * self.width + x] & REFINED != 0 { return 16; }
        let (h, v, d) = self.neighbours(x, y);
        if h + v + d > 0 { 15 } else { 14 }
    }

    fn decode_sign(&mut self, mq: &mut Coder<'_>, cx: &mut [Context; 19], x: usize, y: usize) {
        let (context, flip) = self.sign_context(x, y);
        let negative = if mq.raw() { mq.bit(&mut cx[0]) } else { mq.bit(&mut cx[context]) ^ flip };
        self.flags[y * self.width + x] |= SIGNIFICANT | if negative == 1 { NEGATIVE } else { 0 };
    }
}

/// 🧱️ The decoded coefficient magnitudes in half units (`2 * |c|`, midpoint reconstruction) with their signs.
pub struct DecodedBlock {
    pub half_magnitudes: Vec<u32>,
    pub negative: Vec<bool>,
    pub lowest_plane: u32,
}

/// 🧱️ Decodes the coding passes held by the codeword `segments` of a `width x height` code-block holding `planes` magnitude bit-planes.
pub fn decode_block(segments: &[(Vec<u8>, u32)], width: usize, height: usize, orientation: u8, planes: u32, style: u8) -> DecodedBlock {
    let mut block = Block { width, height, orientation, flags: vec![0; width * height], magnitude: vec![0; width * height], causal: style & STYLE_VERTICAL_CAUSAL != 0 };
    let mut cx = contexts();
    let mut plane = planes as i32 - 1;
    let mut lowest = planes;
    let mut kind = 2;
    let mut index = 0u32;
    'segments: for (data, passes) in segments {
        let mut coder = if is_raw(style, index) { Coder::Raw(RawBits { data, position: 0, current: 0, count: 0 }) } else { Coder::Mq(MqDecoder::new(data)) };
        for _ in 0..*passes {
            if plane < 0 { break 'segments; }
            let bit = plane as u32;
            match kind {
                0 => significance_pass(&mut block, &mut coder, &mut cx, bit),
                1 => refinement_pass(&mut block, &mut coder, &mut cx, bit),
                _ => {
                    cleanup_pass(&mut block, &mut coder, &mut cx, bit);
                    if style & STYLE_SEGMENTATION != 0 { for _ in 0..4 { coder.bit(&mut cx[UNIFORM]); } }
                }
            }
            if style & STYLE_RESET != 0 { cx = contexts(); }
            lowest = bit;
            if kind == 2 { plane -= 1; }
            kind = (kind + 1) % 3;
            index += 1;
        }
    }
    let negative = block.flags.iter().map(|flag| flag & NEGATIVE != 0).collect();
    DecodedBlock { half_magnitudes: block.magnitude, negative, lowest_plane: lowest }
}

fn become_significant(block: &mut Block, index: usize, bit: u32) {
    block.magnitude[index] = 3 << bit;
}

fn significance_pass(block: &mut Block, mq: &mut Coder<'_>, cx: &mut [Context; 19], bit: u32) {
    for stripe in (0..block.height).step_by(4) {
        for x in 0..block.width {
            for y in stripe..(stripe + 4).min(block.height) {
                let index = y * block.width + x;
                if block.flags[index] & SIGNIFICANT != 0 { continue; }
                let context = block.significance_context(x, y);
                if context == 0 { continue; }
                block.flags[index] |= VISITED;
                if mq.bit(&mut cx[context]) == 1 {
                    block.decode_sign(mq, cx, x, y);
                    become_significant(block, index, bit);
                }
            }
        }
    }
}

fn refinement_pass(block: &mut Block, mq: &mut Coder<'_>, cx: &mut [Context; 19], bit: u32) {
    for stripe in (0..block.height).step_by(4) {
        for x in 0..block.width {
            for y in stripe..(stripe + 4).min(block.height) {
                let index = y * block.width + x;
                if block.flags[index] & (SIGNIFICANT | VISITED) != SIGNIFICANT { continue; }
                let context = block.refinement_context(x, y);
                let decision = mq.bit(&mut cx[context]);
                block.flags[index] |= REFINED;
                if decision == 1 { block.magnitude[index] += 1 << bit } else { block.magnitude[index] -= 1 << bit }
            }
        }
    }
}

fn cleanup_pass(block: &mut Block, mq: &mut Coder<'_>, cx: &mut [Context; 19], bit: u32) {
    for stripe in (0..block.height).step_by(4) {
        for x in 0..block.width {
            let mut start = stripe;
            let full = stripe + 4 <= block.height;
            let isolated = full && (stripe..stripe + 4).all(|y| block.flags[y * block.width + x] & (SIGNIFICANT | VISITED) == 0 && block.significance_context(x, y) == 0);
            if isolated {
                if mq.bit(&mut cx[RUN]) == 0 { continue; }
                let first = (mq.bit(&mut cx[UNIFORM]) << 1 | mq.bit(&mut cx[UNIFORM])) as usize;
                let y = stripe + first;
                block.decode_sign(mq, cx, x, y);
                become_significant(block, y * block.width + x, bit);
                start = y + 1;
            }
            for y in start..(stripe + 4).min(block.height) {
                let index = y * block.width + x;
                if block.flags[index] & (SIGNIFICANT | VISITED) != 0 { continue; }
                let context = block.significance_context(x, y);
                if mq.bit(&mut cx[context]) == 1 {
                    block.decode_sign(mq, cx, x, y);
                    become_significant(block, index, bit);
                }
            }
        }
    }
    for flag in &mut block.flags { *flag &= !VISITED; }
}
