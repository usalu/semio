//! 🖼️ Original encoded sources and physical image children publish under independent grants.
use super::{RasterImage,png_decoding::{PngDecodeInput,PngDecodeJob},jpeg_decoding::{JpegDecodeInput,JpegDecodeJob}};
use semio_framework_io_binary_source::{BinarySourceInput,BinarySourceJob,BinarySourceProgress};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
#[derive(Clone,Copy,Debug)]
pub struct ImageDecodeInput<'a>{pub mime:&'a str,pub data:&'a str,pub max_source_bytes:usize,pub max_bytes:usize,pub max_pixels:usize,pub max_chunks:usize}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ImageDecodeProgress{pub phase:&'static str,pub source_completed:usize,pub source_total:usize,pub bytes:usize,pub total_bytes:usize,pub pixels:usize,pub total_pixels:usize,pub work:u64,pub done:bool}
#[derive(Clone,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub enum ImageDecodeError{Invalid(&'static str),UnsupportedMime,Incomplete,Cancelled}
impl std::fmt::Display for ImageDecodeError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Invalid(value)=>f.write_str(value),Self::UnsupportedMime=>f.write_str("Unsupported image media type"),Self::Incomplete=>f.write_str("Image preparation is incomplete"),Self::Cancelled=>f.write_str("Image preparation cancelled")}}}
impl std::error::Error for ImageDecodeError{}
semio_framework_value::artifact_retire_leaf!(ImageDecodeProgress);
#[derive(semio_framework_value::RetireOwned)]
pub struct ImageDecodeJob{
 source_identity:usize,source_length:usize,jpeg_source:bool,max_bytes:usize,max_pixels:usize,max_chunks:usize,phase:&'static str,source:BinarySourceJob,source_progress:BinarySourceProgress,
 work:u64,pixels:usize,total_pixels:usize,bytes:Vec<u8>,png:Option<PngDecodeJob>,jpeg:Option<JpegDecodeJob>,output:RasterImage,cancelled:bool,failed:Option<ImageDecodeError>,
}
impl ImageDecodeJob{
 pub fn new(input:ImageDecodeInput<'_>)->Result<Self,ImageDecodeError>{
  if input.data.is_empty()||!(1..=268439552).contains(&input.max_source_bytes)||input.data.len()>input.max_source_bytes||input.mime.is_empty()||input.mime.len()>128||!(8..=67108864).contains(&input.max_bytes)||!(1..=16777216).contains(&input.max_pixels)||!(1..=65536).contains(&input.max_chunks){return Err(ImageDecodeError::Invalid("Invalid image source contract"));}
  let jpeg_source=input.mime.eq_ignore_ascii_case("image/jpeg");if !jpeg_source&&!input.mime.eq_ignore_ascii_case("image/png"){return Err(ImageDecodeError::UnsupportedMime);}
  let source=BinarySourceJob::new(BinarySourceInput{mime:input.mime,data:input.data,min_bytes:8,max_source_bytes:input.max_source_bytes,max_bytes:input.max_bytes,max_work:1000000000}).map_err(|_|ImageDecodeError::Invalid("Invalid encoded image source"))?;let source_progress=source.progress();
  Ok(Self{source_identity:input.data.as_ptr()as usize,source_length:input.data.len(),jpeg_source,max_bytes:input.max_bytes,max_pixels:input.max_pixels,max_chunks:input.max_chunks,phase:source_progress.phase,source,source_progress,work:0,pixels:0,total_pixels:0,bytes:Vec::new(),png:None,jpeg:None,output:RasterImage::default(),cancelled:false,failed:None})
 }
 fn check(&self)->Result<(),ImageDecodeError>{if self.cancelled{return Err(ImageDecodeError::Cancelled);}if self.phase=="transferred"{return Err(ImageDecodeError::Incomplete);}if let Some(error)=&self.failed{return Err(error.clone());}Ok(())}
 pub fn progress(&self)->ImageDecodeProgress{let source=self.source_progress;ImageDecodeProgress{phase:self.phase,source_completed:source.source_completed,source_total:source.source_total,bytes:source.bytes,total_bytes:source.total_bytes,pixels:self.pixels,total_pixels:self.total_pixels,work:self.work,done:self.phase=="complete"}}
 pub fn next_copy_byte_demand(&self)->Result<usize,ImageDecodeError>{Ok(match self.phase{"complete"|"transferred"=>0,"source-handoff"=>size_of::<Vec<u8>>(),"decoder"=>if self.jpeg_source{size_of::<JpegDecodeJob>()+size_of::<JpegDecodeInput>()}else{size_of::<PngDecodeJob>()+size_of::<PngDecodeInput>()},"image-handoff"=>size_of::<RasterImage>(),"jpeg"=>self.jpeg.as_ref().unwrap().next_copy_byte_demand(),"png"=>self.png.as_ref().unwrap().next_copy_byte_demand().map_err(|_|ImageDecodeError::Invalid("PNG work demand refused"))?,_=>self.source.next_copy_byte_demand()})}
 pub fn next_capacity_byte_demand(&self)->Result<usize,ImageDecodeError>{Ok(match self.phase{"jpeg"=>self.jpeg.as_ref().unwrap().next_capacity_byte_demand(),"png"=>self.png.as_ref().unwrap().next_capacity_byte_demand().map_err(|_|ImageDecodeError::Invalid("PNG capacity demand refused"))?,"source-handoff"|"decoder"|"image-handoff"|"complete"|"transferred"=>0,_=>self.source.next_capacity_byte_demand()})}
 fn step(&mut self,source:&str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ImageDecodeError>{
  match self.phase{
   "source-handoff"=>{let Some((bytes,receipt))=self.source.take_result(grant).map_err(|_|ImageDecodeError::Invalid("Encoded image handoff refused"))?else{return Ok(Default::default());};self.bytes=bytes;self.phase="decoder";Ok(receipt)},
   "decoder"=>{if self.jpeg_source{let input=JpegDecodeInput{data:std::mem::take(&mut self.bytes),max_pixels:self.max_pixels,max_bytes:self.max_bytes,max_segments:self.max_chunks,max_working_bytes:536870912};match JpegDecodeJob::new(input){Ok(job)=>self.jpeg=Some(job),Err((_,input))=>{self.bytes=input.data;return Err(ImageDecodeError::Invalid("JPEG input admission refused"));}}self.phase="jpeg";}else{let input=PngDecodeInput{data:std::mem::take(&mut self.bytes),max_pixels:self.max_pixels,max_bytes:self.max_bytes,max_chunks:self.max_chunks};match PngDecodeJob::new(input){Ok(job)=>self.png=Some(job),Err((_,input))=>{self.bytes=input.data;return Err(ImageDecodeError::Invalid("PNG input admission refused"));}}self.phase="png";}Ok(RetainedCloneProgress{copied_items:1,copied_bytes:grant.maximum_copy_bytes,..Default::default()})},
   "jpeg"=>{let(progress,receipt)=self.jpeg.as_mut().unwrap().advance(grant).map_err(|_|ImageDecodeError::Invalid("JPEG reconstruction refused"))?;self.pixels=progress.pixels;self.total_pixels=progress.total_pixels;if progress.done{self.phase="image-handoff";}Ok(receipt)},
   "png"=>{let(progress,receipt)=self.png.as_mut().unwrap().advance(grant).map_err(|_|ImageDecodeError::Invalid("PNG reconstruction refused"))?;self.pixels=progress.pixels;self.total_pixels=progress.total_pixels;if progress.done{self.phase="image-handoff";}Ok(receipt)},
   "image-handoff"=>{let output=if self.jpeg_source{self.jpeg.as_mut().unwrap().take_result(grant).map_err(|_|ImageDecodeError::Invalid("JPEG image handoff refused"))?}else{self.png.as_mut().unwrap().take_result(grant).map_err(|_|ImageDecodeError::Invalid("PNG image handoff refused"))?};let Some((image,receipt))=output else{return Ok(Default::default());};self.output=image;self.phase="complete";Ok(receipt)},
   _=>{let(progress,receipt)=self.source.advance(source,grant).map_err(|_|ImageDecodeError::Invalid("Encoded image source refused"))?;self.source_progress=progress;self.phase=if progress.done{"source-handoff"}else{progress.phase};Ok(receipt)},
  }
 }
 pub fn advance(&mut self,source:&str,grant:RetainedCloneGrant)->Result<(ImageDecodeProgress,RetainedCloneProgress),ImageDecodeError>{
  self.check()?;if source.as_ptr()as usize!=self.source_identity||source.len()!=self.source_length{return Err(ImageDecodeError::Invalid("Original encoded image source changed"));}let mut receipt=RetainedCloneProgress::default();
  for _ in 0..grant.maximum_items{if self.phase=="complete"{break;}let copy=self.next_copy_byte_demand()?;let capacity=self.next_capacity_byte_demand()?;if grant.maximum_depth==0||copy>grant.maximum_copy_bytes.saturating_sub(receipt.copied_bytes)||capacity>grant.maximum_capacity_bytes.saturating_sub(receipt.retained_capacity_bytes){break;}let child=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:grant.maximum_capacity_bytes-receipt.retained_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes-receipt.released_bytes,maximum_depth:grant.maximum_depth};let step=match self.step(source,child){Ok(value)=>value,Err(error)=>{self.failed=Some(error.clone());return Err(error);}};if step.copied_items==0{break;}self.work+=step.copied_items as u64;receipt.copied_items+=step.copied_items;receipt.copied_bytes+=step.copied_bytes;receipt.retained_capacity_bytes+=step.retained_capacity_bytes;receipt.released_bytes+=step.released_bytes;
  }
  Ok((self.progress(),receipt))
 }
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn result(&self)->Result<&RasterImage,ImageDecodeError>{self.check()?;if self.phase!="complete"{return Err(ImageDecodeError::Incomplete);}Ok(&self.output)}
 pub fn take_result(&mut self,grant:RetainedCloneGrant)->Result<Option<(RasterImage,RetainedCloneProgress)>,ImageDecodeError>{self.result()?;if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<RasterImage>()||grant.maximum_depth==0{return Ok(None);}self.phase="transferred";Ok(Some((std::mem::take(&mut self.output),RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<RasterImage>(),..Default::default()})))}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;