//! 🖼️ Package glue for owned raster image codecs and pixel-buffer utilities.

pub use semio_framework_pixels_codecs::{RasterError,RasterImage,Orientation,resize_bilinear,resize_triangle,tiff_orientation,encoded_orientation,apply_orientation,encode_png,encode_png_gray16,decode_png,PngScanlineDecoder,PngScanlineStep,png_encoding,png_decoding,jpeg_decoding,image_decoding};
#[cfg(feature="testkit")]
pub use semio_framework_pixels_codecs::testkit;

#[path = "../../✍️editing/🦀️.rs"]
pub mod editing;

#[path = "../../🧩️compositing/🦀️.rs"]
pub mod compositing;

#[path = "../../🖊️coverage/🦀️.rs"]
pub mod coverage;
#[path = "../../🎨️sampling/↗️affine/🦀️.rs"]
pub mod affine_sampling;

#[path="../../♻️retirement/🦀️.rs"]
pub mod retirement;
