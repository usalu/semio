//! 📜️ Shared image extent, validation and cooperative progress vocabulary.
use crate::RasterImage;
pub const MAX_IMAGE_SIDE: u32 = 16_384;
pub const MAX_IMAGE_PIXELS: usize = 16 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelProgress {
    pub completed: usize,
    pub total: usize,
    pub done: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum PixelEditError {
    Invalid(&'static str),
    Incomplete,
    Cancelled,
}

impl PixelEditError {
    /// 🗣️ Borrows the exact diagnostic message for funded publication.
    pub fn message(&self) -> &str {match self {Self::Invalid(message)=>message,Self::Incomplete=>"Pixel operation is incomplete",Self::Cancelled=>"Pixel operation cancelled"}}
}
impl std::fmt::Display for PixelEditError {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(self.message())}
}

impl std::error::Error for PixelEditError {}

pub fn validate_extent(width: u32, height: u32) -> Result<usize, PixelEditError> {
    let count = u64::from(width) * u64::from(height);
    if width == 0 || height == 0 || width > MAX_IMAGE_SIDE || height > MAX_IMAGE_SIDE || count > MAX_IMAGE_PIXELS as u64 {
        return Err(PixelEditError::Invalid("Image extent exceeds the pixel budget"));
    }
    Ok(count as usize)
}

pub fn validate_image(image: &RasterImage) -> Result<(), PixelEditError> {
    if image.pixels.len() != validate_extent(image.width, image.height)? * 4 {
        return Err(PixelEditError::Invalid("RGBA8 length does not match image extent"));
    }
    Ok(())
}
