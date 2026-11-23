//! 🎭️ Samples PDF image color and alpha together on the image unit square.

use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfColorSpace, PdfImage, PdfImageBody, PdfImageMask, PdfInlineImage};
use semio_framework_artifact_reference::ArtifactRef;

const MAX_CHANNELS: usize = 8;

type ResolveSamples<'a> = dyn Fn(&ArtifactRef, (u32, u32), usize) -> Option<Vec<u8>> + 'a;

/// 🌈️ Converts one colour of a PDF colour space to sRGB, supplied by the renderer that owns colour management.
pub type ConvertColour<'a> = dyn Fn(&PdfColorSpace, &[f64]) -> Option<[f64; 3]> + 'a;

enum Samples<'a> {
    Borrowed(&'a [u32]),
    Decoded(Vec<u8>),
}

struct SampleGrid<'a> {
    width: u32,
    height: u32,
    channels: usize,
    stride: usize,
    offset: usize,
    max: u32,
    decode: &'a [f64],
    space: Option<&'a PdfColorSpace>,
    samples: Samples<'a>,
}

impl<'a> SampleGrid<'a> {
    fn new(size: (u32, u32), bits: u32, channels: usize, decode: &'a [f64], space: Option<&'a PdfColorSpace>, body: &'a PdfImageBody, resolve: &ResolveSamples<'_>) -> Option<Self> {
        Self::strided(size, bits, channels, channels, 0, decode, space, body, resolve)
    }

    #[allow(clippy::too_many_arguments, reason = "one argument per image dictionary entry plus the interleave layout")]
    fn strided(size: (u32, u32), bits: u32, channels: usize, stride: usize, offset: usize, decode: &'a [f64], space: Option<&'a PdfColorSpace>, body: &'a PdfImageBody, resolve: &ResolveSamples<'_>) -> Option<Self> {
        let (width, height) = size;
        let length = (width as usize).checked_mul(height as usize)?.checked_mul(channels)?;
        let full = (width as usize).checked_mul(height as usize)?.checked_mul(stride)?;
        if length == 0 || channels == 0 || channels > MAX_CHANNELS || !matches!(bits, 1 | 2 | 4 | 8 | 16)
            || (!decode.is_empty() && (decode.len() != channels * 2 || !decode.iter().all(|x| x.is_finite()))) {
            return None;
        }
        let samples = match body {
            PdfImageBody::Samples { values } if values.len() == length && stride == channels => Samples::Borrowed(values),
            PdfImageBody::Artifact { reference } if matches!(bits, 1 | 8) => {
                let bytes = resolve(reference, size, stride)?;
                if bytes.len() != full { return None; }
                Samples::Decoded(bytes)
            }
            _ => return None,
        };
        Some(Self { width, height, channels, stride, offset, max: (1 << bits) - 1, decode, space, samples })
    }

    fn pixel(&self, u: f64, v: f64) -> [u32; MAX_CHANNELS] {
        let column = ((u * f64::from(self.width)) as usize).min(self.width as usize - 1);
        let row = ((v * f64::from(self.height)) as usize).min(self.height as usize - 1);
        let offset = (row * self.width as usize + column) * self.stride + self.offset;
        let mut pixel = [0; MAX_CHANNELS];
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
        let (low, high) = if self.decode.is_empty() { default_decode(self.space, channel, self.max) } else { (self.decode[2 * channel], self.decode[2 * channel + 1]) };
        low + f64::from(sample.min(self.max)) / f64::from(self.max) * (high - low)
    }
}

enum Alpha<'a> {
    Opaque,
    ColorKey(&'a [u32]),
    Stencil(SampleGrid<'a>),
    Soft { grid: SampleGrid<'a>, matte: &'a [f64] },
    InData(SampleGrid<'a>),
}

/// 🖼️ Image samples ready for straight-alpha RGBA preview.
pub struct PdfImagePreviewSamples<'a> {
    color: SampleGrid<'a>,
    alpha: Alpha<'a>,
    convert: &'a ConvertColour<'a>,
    fill: Option<[f64; 3]>,
}

impl<'a> PdfImagePreviewSamples<'a> {
    /// 📦️ Prepares a sampled inline image, painting a stencil with `fill`.
    pub fn inline(image: &'a PdfInlineImage, fill: [f64; 3], convert: &'a ConvertColour<'a>, resolve: &ResolveSamples<'_>) -> Option<Self> {
        if image.image_mask { return Self::stencil((image.width, image.height), image.bits_per_component, &image.decode, &image.body, fill, convert, resolve); }
        let space = image.color_space.as_ref()?;
        let color = SampleGrid::new((image.width, image.height), image.bits_per_component, space.components()? as usize, &image.decode, Some(space), &image.body, resolve)?;
        Self::probed(color, Alpha::Opaque, convert, None)
    }

    fn stencil(size: (u32, u32), bits: u32, decode: &'a [f64], body: &'a PdfImageBody, fill: [f64; 3], convert: &'a ConvertColour<'a>, resolve: &ResolveSamples<'_>) -> Option<Self> {
        if bits != 1 || !matches!(decode, [] | [0.0, 1.0] | [1.0, 0.0]) { return None; }
        let color = SampleGrid::new(size, 1, 1, decode, None, body, resolve)?;
        let mask = SampleGrid::new(size, 1, 1, decode, None, body, resolve)?;
        Some(Self { color, alpha: Alpha::Stencil(mask), convert, fill: Some(fill) })
    }

    fn probed(color: SampleGrid<'a>, alpha: Alpha<'a>, convert: &'a ConvertColour<'a>, fill: Option<[f64; 3]>) -> Option<Self> {
        let probe: [f64; MAX_CHANNELS] = std::array::from_fn(|channel| default_decode(color.space, channel, color.max).0);
        convert(color.space?, &probe[..color.channels])?;
        Some(Self { color, alpha, convert, fill })
    }

    /// 🧩️ Prepares an image and resolves its applicable referenced mask, if any.
    pub fn xobject(image: &'a PdfImage, fill: [f64; 3], convert: &'a ConvertColour<'a>, resource: impl FnOnce(&str) -> Option<&'a PdfImage>, resolve: &ResolveSamples<'_>) -> Option<Self> {
        if image.image_mask { return Self::stencil((image.width, image.height), image.bits_per_component, &image.decode, &image.body, fill, convert, resolve); }
        let in_data = image.soft_mask_in_data.unwrap_or_default();
        if in_data > 1 || (in_data == 1 && (image.soft_mask.is_some() || image.mask.is_some())) { return None; }
        let space = image.color_space.as_ref()?;
        let channels = space.components()? as usize;
        if in_data == 1 {
            let color = SampleGrid::strided((image.width, image.height), image.bits_per_component, channels, channels + 1, 0, &image.decode, Some(space), &image.body, resolve)?;
            let alpha = SampleGrid::strided((image.width, image.height), image.bits_per_component, 1, channels + 1, channels, &[], None, &image.body, resolve)?;
            return Self::probed(color, Alpha::InData(alpha), convert, None);
        }
        let color = SampleGrid::new((image.width, image.height), image.bits_per_component, space.components()? as usize, &image.decode, Some(space), &image.body, resolve)?;
        let alpha = if let Some(id) = &image.soft_mask {
            let mask = resource(id)?;
            if mask.image_mask || mask.color_space != Some(PdfColorSpace::DeviceGray) || mask.mask.is_some() || mask.soft_mask.is_some()
                || mask.soft_mask_in_data.unwrap_or_default() != 0 {
                return None;
            }
            let matte = mask.matte.as_deref().unwrap_or(&[]);
            if mask.matte.is_some() && (matte.len() != color.channels || !matches!(space, PdfColorSpace::DeviceGray | PdfColorSpace::DeviceRgb) || mask.width != image.width || mask.height != image.height
                || matte.iter().any(|x| !(0.0..=1.0).contains(x))) {
                return None;
            }
            Alpha::Soft { grid: SampleGrid::new((mask.width, mask.height), mask.bits_per_component, 1, &mask.decode, None, &mask.body, resolve)?, matte }
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
                    Alpha::Stencil(SampleGrid::new((mask.width, mask.height), 1, 1, &mask.decode, None, &mask.body, resolve)?)
                }
            }
        };
        Self::probed(color, alpha, convert, None)
    }

    /// 📐️ Returns the sample density of the most detailed grid along each axis.
    pub fn dimensions(&self) -> (u32, u32) {
        let grid = match &self.alpha {
            Alpha::Stencil(grid) | Alpha::Soft { grid, .. } | Alpha::InData(grid) => grid,
            _ => &self.color,
        };
        (self.color.width.max(grid.width), self.color.height.max(grid.height))
    }

    /// 🫧️ Samples color and alpha bilinearly between the four nearest sample centres, weighting colour by opacity, the way MuPDF smooths a modest upscale; the borders clamp to the edge samples.
    pub fn rgba_smooth(&self, u: f64, v: f64) -> [u8; 4] {
        let (width, height) = self.dimensions();
        let (px, py) = (u * f64::from(width) - 0.5, v * f64::from(height) - 0.5);
        let (x0, y0) = (px.floor(), py.floor());
        let (fx, fy) = (px - x0, py - y0);
        let centre = |index: f64, count: u32| (index.clamp(0.0, f64::from(count) - 1.0) + 0.5) / f64::from(count);
        let mut sums = [0.0; 4];
        for (dy, wy) in [(0.0, 1.0 - fy), (1.0, fy)] {
            for (dx, wx) in [(0.0, 1.0 - fx), (1.0, fx)] {
                let [r, g, b, a] = self.rgba(centre(x0 + dx, width), centre(y0 + dy, height));
                let weight = wx * wy * f64::from(a);
                sums[0] += weight * f64::from(r);
                sums[1] += weight * f64::from(g);
                sums[2] += weight * f64::from(b);
                sums[3] += weight;
            }
        }
        if sums[3] <= 0.0 { return [0; 4]; }
        let [r, g, b] = [0, 1, 2].map(|channel| (sums[channel] / sums[3]).round() as u8);
        [r, g, b, sums[3].round().min(255.0) as u8]
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
            Alpha::InData(grid) => grid.component(0, grid.pixel(u, v)[0]),
            Alpha::Soft { grid, matte: background } => {
                matte = background;
                grid.component(0, grid.pixel(u, v)[0])
            }
        };
        let alpha = unit_byte(opacity);
        if alpha == 0 { return [0; 4]; }
        let mut values = [0.0; MAX_CHANNELS];
        for (channel, value) in values.iter_mut().enumerate().take(self.color.channels) { *value = self.color.component(channel, samples[channel]); }
        let Some(color) = self.fill.or_else(|| (self.convert)(self.color.space?, &values[..self.color.channels])) else { return [0; 4] };
        let rgb = [0, 1, 2].map(|channel| {
            let mut color = color[channel];
            if let Some(&background) = matte.get(channel.min(self.color.channels - 1)) {
                color = background + (color - background) / opacity;
            }
            unit_byte(color)
        });
        [rgb[0], rgb[1], rgb[2], alpha]
    }
}

fn default_decode(space: Option<&PdfColorSpace>, channel: usize, maximum: u32) -> (f64, f64) {
    match space {
        Some(PdfColorSpace::Indexed { .. }) => (0.0, f64::from(maximum)),
        Some(PdfColorSpace::Lab { range, .. }) => match channel {
            0 => (0.0, 100.0),
            3.. => (0.0, 1.0),
            _ => range.map_or((-100.0, 100.0), |range| (range[2 * channel - 2], range[2 * channel - 1])),
        },
        _ => (0.0, 1.0),
    }
}

fn unit_byte(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}
