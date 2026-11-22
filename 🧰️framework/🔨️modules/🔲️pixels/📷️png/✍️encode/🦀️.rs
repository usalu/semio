//! 📷️ Incremental lossless PNG encoding for cancellable image edits.
use crate::{validate_image, PixelEditError, PixelProgress, RasterImage};
use crate::component::deflate::ZLIB_ENCODE_BLOCK;
use semio_framework_value::RetirementDemand;
use std::hash::Hasher;
#[path="🎟️work/🦀️.rs"]
mod original_work;
pub use original_work::PngEncodeWork;

/// 🔐️ First-party immutable raster custody supports owned pixels and actual shared read leases.
pub trait PngRasterSource:semio_framework_value::retirement::RetireOwned {fn raster(&self)->&RasterImage;}
impl PngRasterSource for RasterImage {fn raster(&self)->&RasterImage{self}}
impl PngRasterSource for crate::retirement::RasterLease {fn raster(&self)->&RasterImage{self}}
#[derive(semio_framework_value::RetireOwned)]
#[doc(hidden)]
pub struct PngEncodeOwners<S:PngRasterSource> {pub image:S,pub bytes:Vec<u8>,}

impl<S:PngRasterSource> semio_framework_value::retirement::RetireOwned for PngEncodeJob<S> {
    type RetirementCursor=<PngEncodeOwners<S> as semio_framework_value::retirement::RetireOwned>::RetirementCursor;
    fn retirement_seed(self)->Self::RetirementCursor{semio_framework_value::retirement::RetireOwned::retirement_seed(PngEncodeOwners {image:self.image,bytes:self.bytes})}
    fn controlled_retirement_supported()->bool{true}
}

#[derive(semio_framework_value::RetireOwned)]
pub struct EncodedPngImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub content_hash: u64,
}

pub struct PngEncodeJob<S:PngRasterSource=RasterImage> {
    #[doc(hidden)] pub image: S,
    #[doc(hidden)] pub bytes: Vec<u8>,
    encoder: crate::component::deflate::ZlibEncodeCursor,
    hasher: std::collections::hash_map::DefaultHasher,
    cursor: usize,
    #[doc(hidden)] pub total: usize,
    cancelled: bool,
    maximum_bytes: usize,
}

impl<S:PngRasterSource> PngEncodeJob<S> {
    pub fn new(image: S) -> Result<Self, PixelEditError> {
        Self::with_maximum_bytes(image,67_108_864).map_err(|(error,_)|error)
    }

    /// 📏️ Refusal returns original pixel custody before admitting private output backing.
    pub fn with_maximum_bytes(image: S, maximum_bytes: usize) -> Result<Self, (PixelEditError,S)> {
        let demand = match Self::birth_demand(image.raster(), maximum_bytes) {
            Ok(demand) => demand,
            Err(error) => return Err((error, image)),
        };
        let total = image.raster().pixels.len() + image.raster().height as usize;
        let mut bytes = Vec::new();
        if bytes.try_reserve_exact(demand.capacity_bytes).is_err() {
            return Err((PixelEditError::Invalid("PNG candidate allocation failed"), image));
        }
        bytes.extend_from_slice(&crate::component::PNG_SIGNATURE);
        let mut header = [0, 0, 0, 0, 0, 0, 0, 0, 8, 6, 0, 0, 0];
        header[..4].copy_from_slice(&image.raster().width.to_be_bytes());
        header[4..8].copy_from_slice(&image.raster().height.to_be_bytes());
        crate::component::write_chunk(&mut bytes, b"IHDR", &header);
        let mut hasher=std::collections::hash_map::DefaultHasher::new();
        hasher.write(&image.raster().width.to_le_bytes());hasher.write(&image.raster().height.to_le_bytes());
        Ok(Self {image,bytes,encoder:crate::component::deflate::ZlibEncodeCursor::new(),hasher,cursor:0,total,cancelled:false,maximum_bytes})
    }

    /// 📐️ Quotes the exact contiguous backing and header bytes before moving the raster.
    pub fn birth_demand(image: &RasterImage, maximum_bytes: usize) -> Result<RetirementDemand, PixelEditError> {
        validate_image(image)?;
        let total = image.pixels.len() + image.height as usize;
        let capacity_bytes = (total * 9).div_ceil(8) + total.div_ceil(ZLIB_ENCODE_BLOCK) * 32 + 64;
        if !(8..=67_108_864).contains(&maximum_bytes) || capacity_bytes > maximum_bytes {
            return Err(PixelEditError::Invalid("PNG encoded byte limit exceeded"));
        }
        Ok(RetirementDemand { capacity_bytes, copy_bytes: 33, depth: 1, ..Default::default() })
    }

    /// 🔢️ Quotes filter input and bounded compressed output for one fixed-Huffman block.
    pub fn next_copy_byte_demand(&self) -> usize {
        let input = (self.total - self.cursor).min(ZLIB_ENCODE_BLOCK);
        input + (input * 9).div_ceil(8) + 32
    }

    pub fn advance(&mut self) -> Result<PixelProgress,PixelEditError> {
        if self.cancelled {return Err(PixelEditError::Cancelled);}
        if self.cursor<self.total {
            let end = (self.cursor + ZLIB_ENCODE_BLOCK).min(self.total);
            let stride = self.image.raster().width as usize * 4 + 1;
            let mut block = [0; ZLIB_ENCODE_BLOCK];
            let block = &mut block[..end - self.cursor];
            for (byte, offset) in block.iter_mut().zip(self.cursor..end) {
                let column = offset % stride;
                if column != 0 {
                    *byte = self.image.raster().pixels[offset / stride * (stride - 1) + column - 1];
                }
            }
            self.hasher.write(block);
            let at = crate::component::begin_chunk(&mut self.bytes, b"IDAT");
            self.encoder.push(block, end == self.total, &mut self.bytes);
            crate::component::finish_chunk(&mut self.bytes, at);
            if self.bytes.len()>self.maximum_bytes{return Err(PixelEditError::Invalid("PNG encoded byte limit exceeded"));}
            self.cursor=end;
            if self.cursor==self.total {crate::component::write_chunk(&mut self.bytes,b"IEND",&[]);}
        }
        Ok(self.progress())
    }

    /// 📊️ Reports input consumed by this encoder and whether all PNG chunks are ready.
    pub fn progress(&self) -> PixelProgress {
        PixelProgress { completed: self.cursor, total: self.total, done: self.cursor == self.total }
    }

    pub fn result(&self) -> Result<&[u8],PixelEditError> {
        if self.cancelled {return Err(PixelEditError::Cancelled);}
        if self.cursor!=self.total {return Err(PixelEditError::Incomplete);}
        Ok(&self.bytes)
    }

    pub fn into_result(self) -> Result<EncodedPngImage,PixelEditError> {
        self.result()?;
        Ok(EncodedPngImage {width:self.image.raster().width,height:self.image.raster().height,data:self.bytes,content_hash:self.hasher.finish()})
    }

    /// 🎁️ Transfers completed bytes while retaining every encoder and pixel owner for explicit close.
    pub fn take_result(&mut self) -> Result<EncodedPngImage,PixelEditError> {
        self.result()?;
        Ok(EncodedPngImage {width:self.image.raster().width,height:self.image.raster().height,data:std::mem::take(&mut self.bytes),content_hash:self.hasher.finish()})
    }

    pub fn cancel(&mut self) {
        self.cancelled=true;
    }
}
#[cfg(test)]#[path="🧪️tests/birth.rs"]
mod birth_tests;
