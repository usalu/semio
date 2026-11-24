//! 🧮️ Evaluates PDF functions (types 0, 2, 3 and arrays) and converts colour-space values to sRGB for canvas previews.
//! [PDF 32000-1 §7.10 Functions](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf)

use super::*;
use semio_s_artifact_stdio_pdf_paint_codecs as paint_codecs;

const MAX_FUNCTION_DEPTH: usize = 16;
const MAX_SAMPLED_INPUTS: usize = 4;
const D50: [f64; 3] = [0.9642, 1.0, 0.8249];

/// 🔢️ Evaluates a function at `input`, `None` for PostScript calculators and malformed definitions.
pub fn evaluate(function: &PdfFunction, input: &[f64]) -> Option<Vec<f64>> {
    evaluate_at(function, input, 0)
}

fn evaluate_at(function: &PdfFunction, input: &[f64], depth: usize) -> Option<Vec<f64>> {
    if depth > MAX_FUNCTION_DEPTH || input.iter().any(|x| !x.is_finite()) { return None; }
    let out = match function {
        PdfFunction::Exponential { domain, range, c0, c1, n } => {
            let x = clip(*input.first()?, domain.get(..2)?);
            let weight = if *n == 1.0 { x } else { x.powf(*n) };
            if !weight.is_finite() || c0.len() != c1.len() || c0.is_empty() { return None; }
            limited(c0.iter().zip(c1).map(|(a, b)| a + weight * (b - a)).collect(), range.as_deref())
        }
        PdfFunction::Sampled { domain, range, size, bits_per_sample, encode, decode, samples, .. } => sampled(domain, range, size, *bits_per_sample, encode.as_deref(), decode.as_deref(), samples, input)?,
        PdfFunction::Stitching { domain, range, functions, bounds, encode } => {
            let count = functions.len();
            if count == 0 || bounds.len() + 1 != count || encode.len() != 2 * count || domain.len() < 2 { return None; }
            let x = clip(*input.first()?, &domain[..2]);
            let index = bounds.iter().position(|bound| x < *bound).unwrap_or(count - 1);
            let low = if index == 0 { domain[0] } else { bounds[index - 1] };
            let high = if index == count - 1 { domain[1] } else { bounds[index] };
            let mapped = if high == low { encode[2 * index] } else { encode[2 * index] + (x - low) * (encode[2 * index + 1] - encode[2 * index]) / (high - low) };
            limited(evaluate_at(&functions[index], &[mapped], depth + 1)?, range.as_deref())
        }
        PdfFunction::Array { functions } => {
            let mut out = Vec::new();
            for function in functions { out.extend(evaluate_at(function, input, depth + 1)?); }
            out
        }
        PdfFunction::PostScript { domain, range, code } => {
            let inputs = domain.len() / 2;
            if input.len() < inputs || range.len() < 2 { return None; }
            let clipped: Vec<f64> = (0..inputs).map(|axis| clip(input[axis], &domain[2 * axis..2 * axis + 2])).collect();
            limited(paint_codecs::postscript::PostScriptProgram::parse(code)?.run(&clipped, range.len() / 2)?, Some(range))
        }
    };
    out.iter().all(|value| value.is_finite()).then_some(out)
}

#[allow(clippy::too_many_arguments, reason = "the sampled function dictionary keys are consumed verbatim")]
fn sampled(domain: &[f64], range: &[f64], size: &[u32], bits: u32, encode: Option<&[f64]>, decode: Option<&[f64]>, samples: &[u32], input: &[f64]) -> Option<Vec<f64>> {
    let (inputs, outputs) = (domain.len() / 2, range.len() / 2);
    if inputs == 0 || outputs == 0 || inputs > MAX_SAMPLED_INPUTS || input.len() < inputs || size.len() != inputs || size.iter().any(|&s| s == 0) || !(1..=32).contains(&bits) { return None; }
    let total = size.iter().try_fold(outputs, |total, &s| total.checked_mul(s as usize))?;
    if samples.len() != total { return None; }
    let mut base = [0usize; MAX_SAMPLED_INPUTS];
    let mut fraction = [0.0f64; MAX_SAMPLED_INPUTS];
    for axis in 0..inputs {
        let x = clip(input[axis], &domain[2 * axis..2 * axis + 2]);
        let last = f64::from(size[axis] - 1);
        let (e0, e1) = encode.and_then(|values| values.get(2 * axis..2 * axis + 2)).map_or((0.0, last), |values| (values[0], values[1]));
        let span = domain[2 * axis + 1] - domain[2 * axis];
        let encoded = if span == 0.0 { e0 } else { e0 + (x - domain[2 * axis]) * (e1 - e0) / span }.clamp(0.0, last);
        base[axis] = encoded.floor() as usize;
        fraction[axis] = encoded - encoded.floor();
    }
    let mut out = vec![0.0; outputs];
    for corner in 0..1usize << inputs {
        let (mut weight, mut index, mut stride) = (1.0, 0usize, 1usize);
        for axis in 0..inputs {
            let upper = corner >> axis & 1 == 1;
            let position = if upper { (base[axis] + 1).min(size[axis] as usize - 1) } else { base[axis] };
            weight *= if upper { fraction[axis] } else { 1.0 - fraction[axis] };
            index += position * stride;
            stride *= size[axis] as usize;
        }
        if weight == 0.0 { continue; }
        for (channel, value) in out.iter_mut().enumerate() { *value += weight * f64::from(samples[index * outputs + channel]); }
    }
    let maximum = ((1u64 << bits) - 1) as f64;
    for (channel, value) in out.iter_mut().enumerate() {
        let (d0, d1) = decode.and_then(|values| values.get(2 * channel..2 * channel + 2)).map_or((range[2 * channel], range[2 * channel + 1]), |values| (values[0], values[1]));
        *value = d0 + *value / maximum * (d1 - d0);
    }
    Some(limited(out, Some(range)))
}

fn clip(x: f64, bounds: &[f64]) -> f64 {
    x.max(bounds[0]).min(bounds[1])
}

fn limited(mut values: Vec<f64>, range: Option<&[f64]>) -> Vec<f64> {
    if let Some(range) = range {
        for (channel, value) in values.iter_mut().enumerate() {
            if let Some(bounds) = range.get(2 * channel..2 * channel + 2) { *value = clip(*value, bounds); }
        }
    }
    values
}

#[derive(Clone, Copy)]
enum CmykModel {
    Fill,
    Image,
}

/// 🌈️ Converts components of `space` to sRGB, resolving named spaces, tint transforms and palettes.
pub fn rgb(snapshot: &PdfSnapshot, space: &PdfColorSpace, components: &[f64]) -> Option<[f64; 3]> {
    rgb_at(snapshot, space, components, 0, CmykModel::Fill)
}

/// 🖼️ Converts image sample components to sRGB; DeviceCMYK follows MuPDF's image conversion instead of its fill-colour corners.
pub fn image_rgb(snapshot: &PdfSnapshot, space: &PdfColorSpace, components: &[f64]) -> Option<[f64; 3]> {
    rgb_at(snapshot, space, components, 0, CmykModel::Image)
}

fn rgb_at(snapshot: &PdfSnapshot, space: &PdfColorSpace, components: &[f64], depth: usize, model: CmykModel) -> Option<[f64; 3]> {
    if depth > MAX_FUNCTION_DEPTH { return None; }
    let unit = |index: usize| components.get(index).map(|value| value.clamp(0.0, 1.0));
    Some(match space {
        PdfColorSpace::DeviceGray => [unit(0)?; 3],
        PdfColorSpace::DeviceRgb => [unit(0)?, unit(1)?, unit(2)?],
        PdfColorSpace::CalGray { gamma, .. } => [srgb_encode(unit(0)?.powf(gamma.unwrap_or(1.0))); 3],
        PdfColorSpace::CalRgb { white_point, gamma, matrix, .. } => {
            let gamma = gamma.unwrap_or([1.0; 3]);
            let [a, b, c] = [unit(0)?.powf(gamma[0]), unit(1)?.powf(gamma[1]), unit(2)?.powf(gamma[2])];
            let m = matrix.unwrap_or([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
            xyz_to_srgb([m[0] * a + m[3] * b + m[6] * c, m[1] * a + m[4] * b + m[7] * c, m[2] * a + m[5] * b + m[8] * c], *white_point)
        }
        PdfColorSpace::DeviceCmyk => match model {
            CmykModel::Fill => cmyk(unit(0)?, unit(1)?, unit(2)?, unit(3)?),
            CmykModel::Image => cmyk_image(unit(0)?, unit(1)?, unit(2)?, unit(3)?),
        },
        PdfColorSpace::Lab { white_point, range, .. } => lab(*white_point, range.unwrap_or([-100.0, 100.0, -100.0, 100.0]), components.get(..3)?),
        PdfColorSpace::IccBased { components: count, alternate, .. } => {
            let device = match (alternate, count) {
                (Some(alternate), _) => return rgb_at(snapshot, alternate, components, depth + 1, model),
                (None, 1) => PdfColorSpace::DeviceGray,
                (None, 3) => PdfColorSpace::DeviceRgb,
                (None, 4) => PdfColorSpace::DeviceCmyk,
                _ => return None,
            };
            rgb_at(snapshot, &device, components, depth + 1, model)?
        }
        PdfColorSpace::Indexed { base, hival, palette } => {
            let width = base.components()? as usize;
            let index = (*components.first()?).round().clamp(0.0, f64::from(*hival)) as usize;
            let entry = palette.get(index * width..(index + 1) * width)?;
            let values: Vec<f64> = match base.as_ref() {
                PdfColorSpace::Lab { range, .. } => {
                    let range = range.unwrap_or([-100.0, 100.0, -100.0, 100.0]);
                    vec![f64::from(entry[0]) / 255.0 * 100.0, range[0] + f64::from(entry[1]) / 255.0 * (range[1] - range[0]), range[2] + f64::from(entry[2]) / 255.0 * (range[3] - range[2])]
                }
                _ => entry.iter().map(|&byte| f64::from(byte) / 255.0).collect(),
            };
            rgb_at(snapshot, base, &values, depth + 1, model)?
        }
        PdfColorSpace::Separation { alternate, tint_transform, .. } | PdfColorSpace::DeviceN { alternate, tint_transform, .. } => {
            rgb_at(snapshot, alternate, &evaluate(tint_transform, components)?, depth + 1, model)?
        }
        PdfColorSpace::Named { name } => rgb_at(snapshot, &snapshot.color_spaces.iter().find(|named| &named.name == name)?.color_space, components, depth + 1, model)?,
        PdfColorSpace::Pattern { .. } => return None,
    })
}

use semio_s_artifact_stdio_pdf_cmyk_image_clut::{GRID as CMYK_IMAGE_GRID, TABLE as CMYK_IMAGE_CLUT};

/// 🖼️ DeviceCMYK image samples to sRGB by multilinear interpolation of the 17-node grid MuPDF 1.26.7's image conversion yields (see `🌈️cmyk-image-clut/📜️cmyk-image-clut.py`).
fn cmyk_image(c: f64, m: f64, y: f64, k: f64) -> [f64; 3] {
    let last = (CMYK_IMAGE_GRID - 1) as f64;
    let position = [c, m, y, k].map(|value| value.clamp(0.0, 1.0) * last);
    let base = position.map(|value| (value.floor() as usize).min(CMYK_IMAGE_GRID - 2));
    let fraction = [0, 1, 2, 3].map(|axis| position[axis] - base[axis] as f64);
    let mut out = [0.0; 3];
    for corner in 0..16usize {
        let (mut weight, mut index) = (1.0, 0usize);
        for axis in 0..4 {
            let upper = corner >> (3 - axis) & 1;
            weight *= if upper == 1 { fraction[axis] } else { 1.0 - fraction[axis] };
            index = index * CMYK_IMAGE_GRID + base[axis] + upper;
        }
        if weight == 0.0 { continue; }
        for channel in 0..3 { out[channel] += weight * f64::from(CMYK_IMAGE_CLUT[index * 3 + channel]) / 255.0; }
    }
    out
}

/// 🖨️ DeviceCMYK to sRGB by multilinear interpolation of the 16 hypercube corners MuPDF 1.26.7 renders (index `c << 3 | m << 2 | y << 1 | k`).
fn cmyk(c: f64, m: f64, y: f64, k: f64) -> [f64; 3] {
    const CORNERS: [[u8; 3]; 16] = [
        [255, 255, 255], [34, 31, 31], [255, 241, 0], [27, 25, 0], [236, 0, 139], [35, 0, 0], [237, 28, 36], [34, 0, 0],
        [0, 173, 239], [0, 14, 35], [0, 165, 80], [0, 18, 0], [46, 48, 146], [0, 0, 1], [54, 54, 57], [0, 0, 0],
    ];
    let mut out = [0.0; 3];
    for (corner, colour) in CORNERS.iter().enumerate() {
        let weight = [(c, 3), (m, 2), (y, 1), (k, 0)].iter().fold(1.0, |weight, &(value, bit)| weight * if corner >> bit & 1 == 1 { value } else { 1.0 - value });
        for (channel, sample) in out.iter_mut().zip(colour) { *channel += weight * f64::from(*sample) / 255.0; }
    }
    out
}

fn srgb_encode(value: f64) -> f64 {
    let value = value.clamp(0.0, 1.0);
    if value <= 0.0031308 { 12.92 * value } else { 1.055 * value.powf(1.0 / 2.4) - 0.055 }
}

fn xyz_to_srgb(xyz: [f64; 3], white: [f64; 3]) -> [f64; 3] {
    let adapted = [xyz[0] * D50[0] / white[0], xyz[1] * D50[1] / white[1], xyz[2] * D50[2] / white[2]];
    [
        3.1338561 * adapted[0] - 1.6168667 * adapted[1] - 0.4906146 * adapted[2],
        -0.9787684 * adapted[0] + 1.9161415 * adapted[1] + 0.0334540 * adapted[2],
        0.0719453 * adapted[0] - 0.2289914 * adapted[1] + 1.4052427 * adapted[2],
    ].map(srgb_encode)
}

fn lab(white: [f64; 3], range: [f64; 4], values: &[f64]) -> [f64; 3] {
    let (lightness, a, b) = (values[0].clamp(0.0, 100.0), values[1].clamp(range[0], range[1]), values[2].clamp(range[2], range[3]));
    let m = (lightness + 16.0) / 116.0;
    let g = |x: f64| if x >= 6.0 / 29.0 { x * x * x } else { 108.0 / 841.0 * (x - 4.0 / 29.0) };
    let xyz = [white[0] * g(m + a / 500.0), white[1] * g(m), white[2] * g(m - b / 200.0)];
    xyz_to_srgb(xyz, white)
}
