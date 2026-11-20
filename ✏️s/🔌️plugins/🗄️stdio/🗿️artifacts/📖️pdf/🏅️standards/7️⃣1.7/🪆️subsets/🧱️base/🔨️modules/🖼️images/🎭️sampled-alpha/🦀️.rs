//! 🎭️ Samples PDF image color and alpha together on the image unit square.

use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfColorSpace, PdfImage, PdfImageBody, PdfImageMask, PdfInlineImage};
use semio_framework_artifact_reference::ArtifactRef;

type ResolveSamples<'a> = dyn Fn(&ArtifactRef, (u32, u32), usize) -> Option<Vec<u8>> + 'a;

enum Samples<'a> {
    Borrowed(&'a [u32]),
    Decoded(Vec<u8>),
}

struct SampleGrid<'a> {
    width: u32,
    height: u32,
    channels: usize,
    max: u32,
    decode: &'a [f64],
    samples: Samples<'a>,
}

impl<'a> SampleGrid<'a> {
    fn new(size: (u32, u32), bits: u32, channels: usize, decode: &'a [f64], body: &'a PdfImageBody, resolve: &ResolveSamples<'_>) -> Option<Self> {
        let (width, height) = size;
        let length = (width as usize).checked_mul(height as usize)?.checked_mul(channels)?;
        if length == 0 || !matches!(bits, 1 | 2 | 4 | 8 | 16)
            || (!decode.is_empty() && (decode.len() != channels * 2 || !decode.iter().all(|x| x.is_finite()))) {
            return None;
        }
        let samples = match body {
            PdfImageBody::Samples { values } if values.len() == length => Samples::Borrowed(values),
            PdfImageBody::Artifact { reference } if bits == 8 => {
                let bytes = resolve(reference, size, channels)?;
                if bytes.len() != length { return None; }
                Samples::Decoded(bytes)
            }
            _ => return None,
        };
        Some(Self { width, height, channels, max: (1 << bits) - 1, decode, samples })
    }

    fn pixel(&self, u: f64, v: f64) -> [u32; 3] {
        let column = ((u * f64::from(self.width)) as usize).min(self.width as usize - 1);
        let row = ((v * f64::from(self.height)) as usize).min(self.height as usize - 1);
        let offset = (row * self.width as usize + column) * self.channels;
        let mut pixel = [0; 3];
        let channels = &mut pixel[..self.channels];
        match &self.samples {
            Samples::Borrowed(values) => channels.copy_from_slice(&values[offset..offset + self.channels]),
            Samples::Decoded(values) => {
                for (channel, &sample) in channels.iter_mut().zip(&values[offset..]) {
                    *channel = u32::from(sample);
                }
            }
        }
        pixel
    }

    fn component(&self, channel: usize, sample: u32) -> f64 {
        let (low, high) = if self.decode.is_empty() { (0.0, 1.0) } else { (self.decode[2 * channel], self.decode[2 * channel + 1]) };
        (low + f64::from(sample.min(self.max)) / f64::from(self.max) * (high - low)).clamp(0.0, 1.0)
    }
}

enum Alpha<'a> {
    Opaque,
    ColorKey(&'a [u32]),
    Stencil(SampleGrid<'a>),
    Soft { grid: SampleGrid<'a>, matte: &'a [f64] },
}

/// 🖼️ Image samples ready for straight-alpha RGBA preview.
pub struct PdfImagePreviewSamples<'a> {
    color: SampleGrid<'a>,
    alpha: Alpha<'a>,
}

impl<'a> PdfImagePreviewSamples<'a> {
    /// 📦️ Prepares a sampled inline image in a supported device color space.
    pub fn inline(image: &'a PdfInlineImage, resolve: &ResolveSamples<'_>) -> Option<Self> {
        if image.image_mask { return None; }
        let color = SampleGrid::new((image.width, image.height), image.bits_per_component, channels(image.color_space.as_ref()?)?, &image.decode, &image.body, resolve)?;
        Some(Self { color, alpha: Alpha::Opaque })
    }

    /// 🧩️ Prepares an image and resolves its applicable referenced mask, if any.
    pub fn xobject(image: &'a PdfImage, resource: impl FnOnce(&str) -> Option<&'a PdfImage>, resolve: &ResolveSamples<'_>) -> Option<Self> {
        if image.image_mask || image.soft_mask_in_data.unwrap_or_default() != 0 { return None; }
        let color = SampleGrid::new((image.width, image.height), image.bits_per_component, channels(image.color_space.as_ref()?)?, &image.decode, &image.body, resolve)?;
        let alpha = if let Some(id) = &image.soft_mask {
            let mask = resource(id)?;
            if mask.image_mask || mask.color_space != Some(PdfColorSpace::DeviceGray) || mask.mask.is_some() || mask.soft_mask.is_some()
                || mask.soft_mask_in_data.unwrap_or_default() != 0 {
                return None;
            }
            let matte = mask.matte.as_deref().unwrap_or(&[]);
            if mask.matte.is_some() && (matte.len() != color.channels || mask.width != image.width || mask.height != image.height
                || matte.iter().any(|x| !(0.0..=1.0).contains(x))) {
                return None;
            }
            Alpha::Soft { grid: SampleGrid::new((mask.width, mask.height), mask.bits_per_component, 1, &mask.decode, &mask.body, resolve)?, matte }
        } else {
            match &image.mask {
                None => Alpha::Opaque,
                Some(PdfImageMask::ColorKey { ranges }) if ranges.len() == color.channels * 2
                    && ranges.chunks_exact(2).all(|bounds| bounds[0] <= bounds[1] && bounds[1] <= color.max) => Alpha::ColorKey(ranges),
                Some(PdfImageMask::ColorKey { .. }) => return None,
                Some(PdfImageMask::Stencil { image }) => {
                    let mask = resource(image)?;
                    if !mask.image_mask || mask.bits_per_component != 1 || !matches!(mask.decode.as_slice(), [] | [0.0, 1.0] | [1.0, 0.0]) {
                        return None;
                    }
                    Alpha::Stencil(SampleGrid::new((mask.width, mask.height), 1, 1, &mask.decode, &mask.body, resolve)?)
                }
            }
        };
        Some(Self { color, alpha })
    }

    /// 📐️ Returns the sample density of the most detailed grid along each axis.
    pub fn dimensions(&self) -> (u32, u32) {
        let grid = match &self.alpha {
            Alpha::Stencil(grid) | Alpha::Soft { grid, .. } => grid,
            _ => &self.color,
        };
        (self.color.width.max(grid.width), self.color.height.max(grid.height))
    }

    /// 🎨️ Samples color and alpha at a normalized image position.
    pub fn rgba(&self, u: f64, v: f64) -> [u8; 4] {
        if !(0.0..1.0).contains(&u) || !(0.0..1.0).contains(&v) { return [0; 4]; }
        let samples = self.color.pixel(u, v);
        let mut matte = &[][..];
        let opacity = match &self.alpha {
            Alpha::Opaque => 1.0,
            Alpha::ColorKey(ranges) => {
                if samples.iter().zip(ranges.chunks_exact(2)).all(|(&sample, bounds)| (bounds[0]..=bounds[1]).contains(&sample)) { return [0; 4]; }
                1.0
            }
            Alpha::Stencil(grid) => 1.0 - grid.component(0, grid.pixel(u, v)[0]),
            Alpha::Soft { grid, matte: background } => {
                matte = background;
                grid.component(0, grid.pixel(u, v)[0])
            }
        };
        let alpha = unit_byte(opacity);
        if alpha == 0 { return [0; 4]; }
        let rgb = [0, 1, 2].map(|channel| {
            let channel = channel.min(self.color.channels - 1);
            let mut color = self.color.component(channel, samples[channel]);
            if let Some(&background) = matte.get(channel) {
                color = background + (color - background) / opacity;
            }
            unit_byte(color)
        });
        [rgb[0], rgb[1], rgb[2], alpha]
    }
}

fn channels(space: &PdfColorSpace) -> Option<usize> {
    match space {
        PdfColorSpace::DeviceGray => Some(1),
        PdfColorSpace::DeviceRgb => Some(3),
        _ => None,
    }
}

fn unit_byte(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}
