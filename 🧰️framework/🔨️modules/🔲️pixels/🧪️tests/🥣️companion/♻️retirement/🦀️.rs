//! ♻️ The raster and mask lease retirement laws and the allocation witnesses the other laws share.
pub use semio_framework_pixels::retirement::*;
use crate::RasterImage;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::retirement::arc_custody::SharedControlledRetirement;
use std::sync::Arc;

#[path = "../../../♻️retirement/🧪️tests/🦀️.rs"]
pub mod tests;
