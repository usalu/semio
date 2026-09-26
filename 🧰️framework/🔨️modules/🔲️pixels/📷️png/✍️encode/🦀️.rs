//! 📷️ Incremental lossless PNG encoding for cancellable image edits.
use crate::{editing::{validate_image, PixelEditError, PixelProgress}, RasterImage};
use std::hash::Hasher;

pub struct EncodedPngImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub content_hash: u64,
}

pub struct PngEncodeJob {
    image: RasterImage,
    bytes: Vec<u8>,
    encoder: crate::component::deflate::ZlibEncodeCursor,
    hasher: std::collections::hash_map::DefaultHasher,
    cursor: usize,
    total: usize,
    cancelled: bool,
}

impl PngEncodeJob {
    pub fn new(image: RasterImage) -> Result<Self, PixelEditError> {
        validate_image(&image)?;
        let total=image.pixels.len()+image.height as usize;
        let mut bytes=Vec::new();
        bytes.try_reserve_exact((total*9).div_ceil(8)+total.div_ceil(4096)*32+64).map_err(|_|PixelEditError::Invalid("PNG candidate allocation failed"))?;
        bytes.extend_from_slice(&crate::component::PNG_SIGNATURE);
        let mut header=Vec::with_capacity(13);
        header.extend_from_slice(&image.width.to_be_bytes());
        header.extend_from_slice(&image.height.to_be_bytes());
        header.extend_from_slice(&[8,6,0,0,0]);
        crate::component::write_chunk(&mut bytes,b"IHDR",&header);
        let mut hasher=std::collections::hash_map::DefaultHasher::new();
        hasher.write(&image.width.to_le_bytes());hasher.write(&image.height.to_le_bytes());
        Ok(Self {image,bytes,encoder:crate::component::deflate::ZlibEncodeCursor::new(),hasher,cursor:0,total,cancelled:false})
    }

    pub fn advance(&mut self) -> Result<PixelProgress,PixelEditError> {
        if self.cancelled {return Err(PixelEditError::Cancelled);}
        if self.cursor<self.total {
            let end=(self.cursor+4096).min(self.total);
            let stride=self.image.width as usize*4+1;
            let mut block=Vec::with_capacity(end-self.cursor);
            for offset in self.cursor..end {
                let column=offset%stride;
                block.push(if column==0 {0} else {self.image.pixels[offset/stride*(stride-1)+column-1]});
            }
            self.hasher.write(&block);
            let compressed=self.encoder.push(&block,end==self.total);
            crate::component::write_chunk(&mut self.bytes,b"IDAT",&compressed);
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
        Ok(EncodedPngImage {width:self.image.width,height:self.image.height,data:self.bytes,content_hash:self.hasher.finish()})
    }

    pub fn cancel(&mut self) {
        self.cancelled=true;self.bytes=Vec::new();self.image.pixels=Vec::new();
    }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
