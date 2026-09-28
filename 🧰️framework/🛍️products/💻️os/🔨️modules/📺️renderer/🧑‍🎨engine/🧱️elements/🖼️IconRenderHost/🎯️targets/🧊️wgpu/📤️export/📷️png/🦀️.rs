//! 📷️ Bounded conversion from encoded premultiplied framebuffer bytes to a PNG candidate.
use semio_framework_pixels::{editing::validate_image, png_encoding::PngEncodeJob, RasterImage};

const PIXELS_PER_STEP: usize = 1024;

enum Phase {
    Normalize { image: RasterImage, pixel: usize },
    Encode { encoder: PngEncodeJob, completed: usize, total: usize },
    Complete(Option<Vec<u8>>),
    Cancelled,
}

pub(super) struct IconPngExport {
    phase: Phase,
}

impl IconPngExport {
    pub(super) fn new(width: u32, height: u32, rgba: Vec<u8>) -> Result<Self, String> {
        let image = RasterImage { width, height, pixels: rgba };
        validate_image(&image).map_err(|error| error.to_string())?;
        Ok(Self { phase: Phase::Normalize { image, pixel: 0 } })
    }

    pub(super) fn advance(&mut self) -> Result<bool, String> {
        match &mut self.phase {
            Phase::Normalize { image, pixel } => {
                let end = (*pixel + PIXELS_PER_STEP).min(image.pixels.len() / 4);
                for rgba in image.pixels[*pixel * 4..end * 4].chunks_exact_mut(4) {
                    let alpha = u32::from(rgba[3]);
                    for channel in &mut rgba[..3] {
                        *channel = if alpha == 0 { 0 } else { ((u32::from(*channel) * 255 + alpha / 2) / alpha).min(255) as u8 };
                    }
                }
                *pixel = end;
                if end == image.pixels.len() / 4 {
                    let total = image.pixels.len() + image.height as usize;
                    let encoder = PngEncodeJob::new(std::mem::take(image)).map_err(|error| error.to_string())?;
                    self.phase = Phase::Encode { encoder, completed: 0, total };
                }
            }
            Phase::Encode { encoder, completed, .. } => {
                let progress = encoder.advance().map_err(|error| error.to_string())?;
                *completed = progress.completed;
                if progress.done {
                    let Phase::Encode { encoder, .. } = std::mem::replace(&mut self.phase, Phase::Cancelled) else { unreachable!() };
                    self.phase = Phase::Complete(Some(encoder.into_result().map_err(|error| error.to_string())?.data));
                }
            }
            Phase::Complete(_) => return Ok(true),
            Phase::Cancelled => return Err("icon PNG export was cancelled".into()),
        }
        Ok(matches!(self.phase, Phase::Complete(_)))
    }

    pub(super) fn progress(&self) -> (u8, usize, usize) {
        match &self.phase {
            Phase::Normalize { image, pixel } => (0, *pixel, image.pixels.len() / 4),
            Phase::Encode { completed, total, .. } => (1, *completed, *total),
            Phase::Complete(bytes) => (2, usize::from(bytes.is_none()), 1),
            Phase::Cancelled => (3, 0, 0),
        }
    }

    pub(super) fn take_png(&mut self) -> Option<Vec<u8>> {
        match &mut self.phase {
            Phase::Complete(bytes) => bytes.take(),
            _ => None,
        }
    }

    pub(super) fn cancel(&mut self) {
        if let Phase::Encode { encoder, .. } = &mut self.phase {
            encoder.cancel();
        }
        self.phase = Phase::Cancelled;
    }
}

#[cfg(test)]
#[path = "../../../../🧪️tests/📤️png-export/🦀️.rs"]
mod tests;
