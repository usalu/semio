//! 🎞️ Image codecs own their image contract and native source lifecycle.
#[path="../../../🦀️.rs"]
mod component;
pub use component::{RasterError,RasterImage,Orientation,resize_bilinear,resize_triangle,tiff_orientation,encoded_orientation,apply_orientation,encode_png,encode_png_gray16,decode_png,PngScanlineDecoder,PngScanlineStep,png_encoding,png_decoding,jpeg_decoding,image_decoding};
pub use semio_framework_2d_retire::{BlockingTurns,EffectTurns};
#[path="../../../🖼️image/📜️contract/🦀️.rs"]
mod image_contract;
pub use image_contract::{MAX_IMAGE_SIDE,MAX_IMAGE_PIXELS,PixelProgress,PixelEditError,validate_extent,validate_image};
#[path="../../../♻️retirement/🖼️image/🦀️.rs"]
pub mod retirement;
#[cfg(feature="testkit")]
pub use component::testkit;
