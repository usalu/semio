//! 🖼️ Checked byte spans for solid RGBA8 rectangular edits.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RasterRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub color: [u8; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RasterRegionLimits {
    pub maximum_raster_bytes: usize,
    pub maximum_patch_bytes: usize,
    pub maximum_patches: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RasterRegionError {
    Empty,
    InvalidBudget,
    ExtentOverflow,
    NoncanonicalRaster,
    RasterTooLarge,
    BoundsOverflow,
    OutOfBounds,
    TooManyPatches,
    InvalidOrdinal,
}

impl RasterRegionError {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::InvalidBudget => "invalid-budget",
            Self::ExtentOverflow => "extent-overflow",
            Self::NoncanonicalRaster => "noncanonical-raster",
            Self::RasterTooLarge => "raster-too-large",
            Self::BoundsOverflow => "bounds-overflow",
            Self::OutOfBounds => "out-of-bounds",
            Self::TooManyPatches => "too-many-patches",
            Self::InvalidOrdinal => "invalid-ordinal",
        }
    }
}

impl std::fmt::Display for RasterRegionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}
impl std::error::Error for RasterRegionError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RasterBytePatch {
    pub index: usize,
    pub pixels: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PlanKind {
    Rows { rows_per_patch: usize },
    WideRows { chunks_per_row: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RasterRegionPlan {
    region: RasterRegion,
    byte_length: usize,
    row_bytes: usize,
    region_row_bytes: usize,
    patch_bytes: usize,
    patch_count: usize,
    kind: PlanKind,
}

impl RasterRegionPlan {
    /// 📏 Validates a complete raster and plans bounded, nonoverlapping spans without allocating pixels.
    pub fn new(width: u32, height: u32, byte_length: usize, region: RasterRegion, limits: RasterRegionLimits) -> Result<Self, RasterRegionError> {
        if region.width == 0 || region.height == 0 {
            return Err(RasterRegionError::Empty);
        }
        if limits.maximum_patch_bytes < 4 || !limits.maximum_patch_bytes.is_multiple_of(4) || limits.maximum_patches == 0 {
            return Err(RasterRegionError::InvalidBudget);
        }
        let row_bytes = usize::try_from(width).ok().and_then(|width| width.checked_mul(4)).ok_or(RasterRegionError::ExtentOverflow)?;
        let expected = usize::try_from(height).ok().and_then(|height| row_bytes.checked_mul(height)).ok_or(RasterRegionError::ExtentOverflow)?;
        if expected != byte_length {
            return Err(RasterRegionError::NoncanonicalRaster);
        }
        if expected > limits.maximum_raster_bytes {
            return Err(RasterRegionError::RasterTooLarge);
        }
        let right = region.x.checked_add(region.width).ok_or(RasterRegionError::BoundsOverflow)?;
        let bottom = region.y.checked_add(region.height).ok_or(RasterRegionError::BoundsOverflow)?;
        if right > width || bottom > height {
            return Err(RasterRegionError::OutOfBounds);
        }
        let region_row_bytes = region.width as usize * 4;
        let (kind, patch_count) = if row_bytes <= limits.maximum_patch_bytes {
            let rows_per_patch = limits.maximum_patch_bytes / row_bytes;
            (PlanKind::Rows { rows_per_patch }, (region.height as usize).div_ceil(rows_per_patch))
        } else {
            let chunks_per_row = region_row_bytes.div_ceil(limits.maximum_patch_bytes);
            (PlanKind::WideRows { chunks_per_row }, (region.height as usize).checked_mul(chunks_per_row).ok_or(RasterRegionError::TooManyPatches)?)
        };
        if patch_count > limits.maximum_patches {
            return Err(RasterRegionError::TooManyPatches);
        }
        Ok(Self { region, byte_length, row_bytes, region_row_bytes, patch_bytes: limits.maximum_patch_bytes, patch_count, kind })
    }

    pub const fn patch_count(&self) -> usize {
        self.patch_count
    }

    /// 🩹️ Materializes at most one planned payload; unchanged spans allocate no patch.
    pub fn patch(&self, pixels: &[u8], ordinal: usize) -> Result<Option<RasterBytePatch>, RasterRegionError> {
        if pixels.len() != self.byte_length {
            return Err(RasterRegionError::NoncanonicalRaster);
        }
        if ordinal >= self.patch_count {
            return Err(RasterRegionError::InvalidOrdinal);
        }
        let (index, length, rows, left, painted_bytes) = match self.kind {
            PlanKind::Rows { rows_per_patch } => {
                let relative_row = ordinal * rows_per_patch;
                let rows = rows_per_patch.min(self.region.height as usize - relative_row);
                ((self.region.y as usize + relative_row) * self.row_bytes, rows * self.row_bytes, rows, self.region.x as usize * 4, self.region_row_bytes)
            }
            PlanKind::WideRows { chunks_per_row } => {
                let relative_row = ordinal / chunks_per_row;
                let offset = ordinal % chunks_per_row * self.patch_bytes;
                let length = self.patch_bytes.min(self.region_row_bytes - offset);
                ((self.region.y as usize + relative_row) * self.row_bytes + self.region.x as usize * 4 + offset, length, 1, 0, length)
            }
        };
        let changed = (0..rows).any(|row| pixels[index + row * self.row_bytes + left..index + row * self.row_bytes + left + painted_bytes].chunks_exact(4).any(|pixel| pixel != self.region.color));
        if !changed {
            return Ok(None);
        }
        let mut patch = pixels[index..index + length].to_vec();
        for row in 0..rows {
            let start = row * self.row_bytes + left;
            for pixel in patch[start..start + painted_bytes].chunks_exact_mut(4) {
                pixel.copy_from_slice(&self.region.color);
            }
        }
        Ok(Some(RasterBytePatch { index, pixels: patch }))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
