//! 📷️ Incremental lossless PNG encoding for cancellable image edits.
use crate::{editing::{validate_image, PixelEditError, PixelProgress}, RasterImage};
use std::hash::Hasher;

/// 🔐️ First-party immutable raster custody supports owned pixels and actual shared read leases.
pub trait PngRasterSource:semio_framework_value::retirement::RetireOwned {fn raster(&self)->&RasterImage;}
impl PngRasterSource for RasterImage {fn raster(&self)->&RasterImage{self}}
impl PngRasterSource for crate::retirement::RasterLease {fn raster(&self)->&RasterImage{self}}
#[derive(semio_framework_value::RetireOwned)]
struct PngEncodeOwners<S:PngRasterSource> {image:S,bytes:Vec<u8>,encoder:crate::component::deflate::ZlibEncodeCursor}

impl<S:PngRasterSource> semio_framework_value::retirement::RetireOwned for PngEncodeJob<S> {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(PngEncodeOwners {image:self.image,bytes:self.bytes,encoder:self.encoder})}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.image),deferred_birth_bytes_for(&self.bytes),deferred_birth_bytes_for(&self.encoder)])}
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
    image: S,
    bytes: Vec<u8>,
    encoder: crate::component::deflate::ZlibEncodeCursor,
    hasher: std::collections::hash_map::DefaultHasher,
    cursor: usize,
    total: usize,
    cancelled: bool,
    maximum_bytes: usize,
}

impl<S:PngRasterSource> PngEncodeJob<S> {
    pub fn new(image: S) -> Result<Self, PixelEditError> {
        Self::with_maximum_bytes(image,67_108_864).map_err(|(error,_)|error)
    }

    /// 📏️ Refusal returns original pixel custody before admitting private output backing.
    pub fn with_maximum_bytes(image: S, maximum_bytes: usize) -> Result<Self, (PixelEditError,S)> {
        if let Err(error)=validate_image(image.raster()){return Err((error,image));}
        let total=image.raster().pixels.len()+image.raster().height as usize;
        let maximum=(total*9).div_ceil(8)+total.div_ceil(4096)*32+64;
        if !(8..=67_108_864).contains(&maximum_bytes)||maximum>maximum_bytes{return Err((PixelEditError::Invalid("PNG encoded byte limit exceeded"),image));}
        let mut bytes=Vec::new();
        if bytes.try_reserve_exact(maximum).is_err(){return Err((PixelEditError::Invalid("PNG candidate allocation failed"),image));}
        bytes.extend_from_slice(&crate::component::PNG_SIGNATURE);
        let mut header=Vec::with_capacity(13);
        header.extend_from_slice(&image.raster().width.to_be_bytes());
        header.extend_from_slice(&image.raster().height.to_be_bytes());
        header.extend_from_slice(&[8,6,0,0,0]);
        crate::component::write_chunk(&mut bytes,b"IHDR",&header);
        let mut hasher=std::collections::hash_map::DefaultHasher::new();
        hasher.write(&image.raster().width.to_le_bytes());hasher.write(&image.raster().height.to_le_bytes());
        Ok(Self {image,bytes,encoder:crate::component::deflate::ZlibEncodeCursor::new(),hasher,cursor:0,total,cancelled:false,maximum_bytes})
    }

    pub fn advance(&mut self) -> Result<PixelProgress,PixelEditError> {
        if self.cancelled {return Err(PixelEditError::Cancelled);}
        if self.cursor<self.total {
            let end=(self.cursor+4096).min(self.total);
            let stride=self.image.raster().width as usize*4+1;
            let mut block=Vec::with_capacity(end-self.cursor);
            for offset in self.cursor..end {
                let column=offset%stride;
                block.push(if column==0 {0} else {self.image.raster().pixels[offset/stride*(stride-1)+column-1]});
            }
            self.hasher.write(&block);
            let compressed=self.encoder.push(&block,end==self.total);
            crate::component::write_chunk(&mut self.bytes,b"IDAT",&compressed);
            if self.bytes.len()>self.maximum_bytes{return Err(PixelEditError::Invalid("PNG encoded byte limit exceeded"));}
            self.cursor=end;
            if self.cursor==self.total {crate::component::write_chunk(&mut self.bytes,b"IEND",&[]);}
        }
        Ok(PixelProgress {completed:self.cursor,total:self.total,done:self.cursor==self.total})
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

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
