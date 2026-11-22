//! 🌊️ The inverse JPEG 2000 discrete wavelet transform (reversible 5/3 and irreversible 9/7).
//! [ITU-T T.800 Annex F](https://www.itu.int/rec/T-REC-T.800)

const ALPHA: f32 = -1.586_134_3;
const BETA: f32 = -0.052_980_12;
const GAMMA: f32 = 0.882_911_1;
const DELTA: f32 = 0.443_506_85;
const K: f32 = 1.230_174_1;
const PAD: isize = 4;

fn mirror(index: isize, length: isize) -> usize {
    if length == 1 { return 0; }
    let period = 2 * (length - 1);
    let wrapped = index.rem_euclid(period);
    (if wrapped < length { wrapped } else { period - wrapped }) as usize
}

/// 🌊️ Inverse-filters one interleaved line whose first sample sits at absolute coordinate `origin`.
pub fn inverse_line(line: &mut [f32], origin: i64, reversible: bool, buffer: &mut Vec<f32>) {
    let length = line.len() as isize;
    if length == 1 {
        if origin & 1 == 1 { line[0] /= 2.0; }
        return;
    }
    buffer.clear();
    buffer.extend((-PAD..length + PAD).map(|i| line[mirror(i, length)]));
    let at = |i: isize| (i + PAD) as usize;
    let odd = |i: isize| (origin + i as i64) & 1 == 1;
    let step = |buffer: &mut Vec<f32>, from: isize, to: isize, parity_odd: bool, weight: f32, floor_div: Option<(f32, f32)>| {
        for i in from..to {
            if odd(i) != parity_odd { continue; }
            let sum = buffer[at(i - 1)] + buffer[at(i + 1)];
            buffer[at(i)] = match floor_div {
                Some((bias, divisor)) => buffer[at(i)] + weight * ((sum + bias) / divisor).floor(),
                None => buffer[at(i)] - weight * sum,
            };
        }
    };
    if reversible {
        step(buffer, -1, length + 1, false, -1.0, Some((2.0, 4.0)));
        step(buffer, 0, length, true, 1.0, Some((0.0, 2.0)));
    } else {
        for i in -PAD..length + PAD { if odd(i) { buffer[at(i)] /= K } else { buffer[at(i)] *= K } }
        step(buffer, -3, length + 3, false, DELTA, None);
        step(buffer, -2, length + 2, true, GAMMA, None);
        step(buffer, -1, length + 1, false, BETA, None);
        step(buffer, 0, length, true, ALPHA, None);
    }
    for (i, slot) in line.iter_mut().enumerate() { *slot = buffer[at(i as isize)]; }
}

/// 🌊️ Reconstructs one resolution from the lower resolution and its three detail subbands.
pub struct Band<'a> {
    pub x0: i64,
    pub y0: i64,
    pub width: usize,
    pub height: usize,
    pub samples: &'a [f32],
}

pub fn inverse_level(ll: &Band<'_>, hl: &Band<'_>, lh: &Band<'_>, hh: &Band<'_>, rx0: i64, ry0: i64, rx1: i64, ry1: i64, reversible: bool) -> Vec<f32> {
    let (width, height) = ((rx1 - rx0) as usize, (ry1 - ry0) as usize);
    let mut out = vec![0.0f32; width * height];
    for v in 0..height {
        for u in 0..width {
            let (au, av) = (rx0 + u as i64, ry0 + v as i64);
            let band = match (au & 1, av & 1) { (0, 0) => ll, (1, 0) => hl, (0, 1) => lh, _ => hh };
            let (bx, by) = ((au >> 1) - band.x0, (av >> 1) - band.y0);
            if bx >= 0 && by >= 0 && (bx as usize) < band.width && (by as usize) < band.height { out[v * width + u] = band.samples[by as usize * band.width + bx as usize]; }
        }
    }
    let mut buffer = Vec::new();
    for row in out.chunks_exact_mut(width.max(1)) { inverse_line(row, rx0, reversible, &mut buffer); }
    let mut column = vec![0.0f32; height];
    for u in 0..width {
        for v in 0..height { column[v] = out[v * width + u]; }
        inverse_line(&mut column, ry0, reversible, &mut buffer);
        for v in 0..height { out[v * width + u] = column[v]; }
    }
    out
}
