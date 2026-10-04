//! 🖼️ Cooperative encoded-image preparation with private pixel publication.
use super::{RasterImage,png_decoding::{PngDecodeInput,PngDecodeJob}};
use semio_framework_io_binary_source::{BinarySourceInput,BinarySourceJob,BinarySourceProgress};
use std::sync::Arc;
#[derive(Clone,Debug)]
pub struct ImageDecodeInput{pub mime:String,pub data:Arc<String>,pub max_source_bytes:usize,pub max_bytes:usize,pub max_pixels:usize,pub max_chunks:usize}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ImageDecodeProgress{pub phase:&'static str,pub source_completed:usize,pub source_total:usize,pub bytes:usize,pub total_bytes:usize,pub pixels:usize,pub total_pixels:usize,pub work:u64,pub done:bool}
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum ImageDecodeError{Invalid(String),UnsupportedMime(String),Incomplete,Cancelled}
impl std::fmt::Display for ImageDecodeError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Invalid(v)=>f.write_str(v),Self::UnsupportedMime(v)=>write!(f,"Unsupported image media type: {v}"),Self::Incomplete=>f.write_str("Image preparation is incomplete"),Self::Cancelled=>f.write_str("Image preparation cancelled")}}}
impl std::error::Error for ImageDecodeError{}
fn invalid(v:impl ToString)->ImageDecodeError{ImageDecodeError::Invalid(v.to_string())}
/// 🧩️ Real byte-source and PNG child jobs share private publication and caller grants.
pub struct ImageDecodeJob{
 max_bytes:usize,max_pixels:usize,max_chunks:usize,phase:&'static str,source:Option<BinarySourceJob>,source_progress:BinarySourceProgress,
 work:u64,pixels:usize,total_pixels:usize,png:Option<PngDecodeJob>,output:RasterImage,cancelled:bool,failed:Option<ImageDecodeError>,
}
impl ImageDecodeJob{
 pub fn new(input:ImageDecodeInput)->Result<Self,ImageDecodeError>{
  if input.data.is_empty()||!(1..=268439552).contains(&input.max_source_bytes)||input.data.len()>input.max_source_bytes||input.mime.is_empty()||input.mime.len()>128||!(8..=67108864).contains(&input.max_bytes)||!(1..=16777216).contains(&input.max_pixels)||!(1..=65536).contains(&input.max_chunks){return Err(invalid("Invalid image source contract"));}
  if !input.mime.eq_ignore_ascii_case("image/png"){return Err(ImageDecodeError::UnsupportedMime(input.mime));}
  let source_length=input.data.len();let url=input.data.as_bytes().get(..5).is_some_and(|v|v.eq_ignore_ascii_case(b"data:"));let at=if url{5}else{0};let phase=if url{"header"}else{"validate"};
  let source=BinarySourceJob::new(BinarySourceInput{mime:input.mime,data:input.data,min_bytes:8,max_source_bytes:input.max_source_bytes,max_bytes:input.max_bytes,max_work:1000000000}).map_err(invalid)?;
  Ok(Self{max_bytes:input.max_bytes,max_pixels:input.max_pixels,max_chunks:input.max_chunks,phase,source:Some(source),source_progress:BinarySourceProgress{phase,source_completed:at,source_total:source_length*2,bytes:0,total_bytes:0,work:0,done:false},work:0,pixels:0,total_pixels:0,png:None,output:RasterImage::default(),cancelled:false,failed:None})
 }
 fn check(&self)->Result<(),ImageDecodeError>{if self.cancelled{return Err(ImageDecodeError::Cancelled);}if let Some(e)=&self.failed{return Err(e.clone());}Ok(())}
 fn release(&mut self){if let Some(source)=&mut self.source{source.cancel();}self.source=None;if let Some(png)=&mut self.png{png.cancel();}self.png=None;self.output.pixels=Vec::new();}
 fn source_step(&mut self)->Result<(),ImageDecodeError>{
  let p=self.source.as_mut().unwrap().advance(1).map_err(invalid)?;self.source_progress=p;
  if p.done{let bytes=self.source.take().unwrap().into_result().map_err(invalid)?;self.png=Some(PngDecodeJob::new(PngDecodeInput{data:Arc::new(bytes),max_bytes:self.max_bytes,max_pixels:self.max_pixels,max_chunks:self.max_chunks}).map_err(invalid)?);self.phase="png";}
  else{self.phase=p.phase;}Ok(())
 }
 fn png_step(&mut self)->Result<(),ImageDecodeError>{
  let p=self.png.as_mut().unwrap().advance(1).map_err(invalid)?;self.pixels=p.pixels;self.total_pixels=p.total_pixels;
  if p.done{self.output=self.png.take().unwrap().into_result().map_err(invalid)?;self.phase="complete";}Ok(())
 }
 pub fn advance(&mut self,budget:usize)->Result<ImageDecodeProgress,ImageDecodeError>{
  if budget==0||budget as u64>9007199254740991{return Err(invalid("Image work grant must be a positive integer"));}self.check()?;
  for _ in 0..budget{if self.phase=="complete"{break;}let result=if self.phase=="png"{self.png_step()}else{self.source_step()};if let Err(e)=result{self.failed=Some(e.clone());self.release();return Err(e);}self.work+=1;}
  let p=self.source_progress;
  Ok(ImageDecodeProgress{phase:self.phase,source_completed:p.source_completed,source_total:p.source_total,bytes:p.bytes,total_bytes:p.total_bytes,pixels:self.pixels,total_pixels:self.total_pixels,work:self.work,done:self.phase=="complete"})
 }
 pub fn cancel(&mut self){self.cancelled=true;self.release();}
 pub fn result(&self)->Result<&RasterImage,ImageDecodeError>{self.check()?;if self.phase!="complete"{return Err(ImageDecodeError::Incomplete);}Ok(&self.output)}
 pub fn into_result(self)->Result<RasterImage,ImageDecodeError>{self.check()?;if self.phase!="complete"{return Err(ImageDecodeError::Incomplete);}Ok(self.output)}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
