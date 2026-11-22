//! 🧮️ The MQ arithmetic decoder shared by JPEG 2000 tier-1 and JBIG2 (software conventions).
//! [ITU-T T.88 Annex E](https://www.itu.int/rec/T-REC-T.88) · [ITU-T T.800 Annex C](https://www.itu.int/rec/T-REC-T.800)

/// 🎲️ One adaptive context: the probability state index and the more probable symbol.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Context {
    state: u8,
    mps: u8,
}

impl Context {
    /// 🌱️ A context starting in probability state `state`.
    pub const fn with_state(state: u8) -> Self { Self { state, mps: 0 } }
}

const QE: [(u16, u8, u8, u8); 47] = [
    (0x5601, 1, 1, 1), (0x3401, 2, 6, 0), (0x1801, 3, 9, 0), (0x0AC1, 4, 12, 0), (0x0521, 5, 29, 0), (0x0221, 38, 33, 0), (0x5601, 7, 6, 1),
    (0x5401, 8, 14, 0), (0x4801, 9, 14, 0), (0x3801, 10, 14, 0), (0x3001, 11, 17, 0), (0x2401, 12, 18, 0), (0x1C01, 13, 20, 0), (0x1601, 29, 21, 0),
    (0x5601, 15, 14, 1), (0x5401, 16, 14, 0), (0x5101, 17, 15, 0), (0x4801, 18, 16, 0), (0x3801, 19, 17, 0), (0x3401, 20, 18, 0), (0x3001, 21, 19, 0),
    (0x2801, 22, 19, 0), (0x2401, 23, 20, 0), (0x2201, 24, 21, 0), (0x1C01, 25, 22, 0), (0x1801, 26, 23, 0), (0x1601, 27, 24, 0), (0x1401, 28, 25, 0),
    (0x1201, 29, 26, 0), (0x1101, 30, 27, 0), (0x0AC1, 31, 28, 0), (0x09C1, 32, 29, 0), (0x08A1, 33, 30, 0), (0x0521, 34, 31, 0), (0x0441, 35, 32, 0),
    (0x02A1, 36, 33, 0), (0x0221, 37, 34, 0), (0x0141, 38, 35, 0), (0x0111, 39, 36, 0), (0x0085, 40, 37, 0), (0x0049, 41, 38, 0), (0x0025, 42, 39, 0),
    (0x0015, 43, 40, 0), (0x0009, 44, 41, 0), (0x0005, 45, 42, 0), (0x0001, 45, 43, 0), (0x5601, 46, 46, 0),
];

/// 🎰️ Decodes binary decisions from one MQ-coded segment.
pub struct MqDecoder<'a> {
    data: &'a [u8],
    position: usize,
    chigh: u32,
    clow: u32,
    count: i32,
    a: u32,
}

impl<'a> MqDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let mut decoder = Self { data, position: 0, chigh: u32::from(data.first().copied().unwrap_or(0xFF)), clow: 0, count: 0, a: 0x8000 };
        decoder.byte_in();
        decoder.chigh = ((decoder.chigh << 7) & 0xFFFF) | ((decoder.clow >> 9) & 0x7F);
        decoder.clow = (decoder.clow << 7) & 0xFFFF;
        decoder.count -= 7;
        decoder
    }

    fn at(&self, index: usize) -> u32 { u32::from(self.data.get(index).copied().unwrap_or(0xFF)) }

    fn byte_in(&mut self) {
        if self.at(self.position) == 0xFF {
            if self.at(self.position + 1) > 0x8F {
                self.clow += 0xFF00;
                self.count = 8;
            } else {
                self.position += 1;
                self.clow += self.at(self.position) << 9;
                self.count = 7;
            }
        } else {
            self.position += 1;
            self.clow += if self.position < self.data.len() { self.at(self.position) << 8 } else { 0xFF00 };
            self.count = 8;
        }
        if self.clow > 0xFFFF {
            self.chigh += self.clow >> 16;
            self.clow &= 0xFFFF;
        }
    }

    /// 🎲️ Decodes one decision under `context`.
    pub fn bit(&mut self, context: &mut Context) -> u8 {
        let (qe, next_mps, next_lps, switch) = QE[context.state as usize];
        let qe = u32::from(qe);
        let mut a = self.a - qe;
        let decision;
        if self.chigh < qe {
            if a < qe {
                a = qe;
                decision = context.mps;
                context.state = next_mps;
            } else {
                a = qe;
                decision = 1 ^ context.mps;
                if switch == 1 { context.mps = decision; }
                context.state = next_lps;
            }
        } else {
            self.chigh -= qe;
            if a & 0x8000 != 0 {
                self.a = a;
                return context.mps;
            }
            if a < qe {
                decision = 1 ^ context.mps;
                if switch == 1 { context.mps = decision; }
                context.state = next_lps;
            } else {
                decision = context.mps;
                context.state = next_mps;
            }
        }
        loop {
            if self.count == 0 { self.byte_in(); }
            a <<= 1;
            self.chigh = ((self.chigh << 1) & 0xFFFF) | ((self.clow >> 15) & 1);
            self.clow = (self.clow << 1) & 0xFFFF;
            self.count -= 1;
            if a & 0x8000 != 0 { break; }
        }
        self.a = a;
        decision
    }
}
