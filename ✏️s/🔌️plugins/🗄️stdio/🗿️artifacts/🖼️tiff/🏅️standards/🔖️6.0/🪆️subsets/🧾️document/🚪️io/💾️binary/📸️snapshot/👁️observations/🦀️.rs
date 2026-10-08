//! 👁️ Ephemeral native carrier observations; never authored snapshot fields.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TiffNativeObservations {
 pub ifd_count: usize,
 pub raster: bool,
 pub compression: Option<Vec<u32>>,
 pub photometric: Option<Vec<u32>>,
 pub bits_per_sample: Option<Vec<u32>>,
 pub tile_width: Option<Vec<u32>>,
 pub tile_length: Option<Vec<u32>>,
 pub strip_offsets: Option<Vec<u32>>,
}
