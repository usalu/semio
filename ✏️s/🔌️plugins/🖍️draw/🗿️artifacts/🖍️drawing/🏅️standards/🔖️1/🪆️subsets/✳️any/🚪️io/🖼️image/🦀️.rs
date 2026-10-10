//! 🖼️ Native encoded sources admit logical RGBA assets before semantic tracing or rendering.
use crate::DrawingImageAsset;
use semio_framework_pixels::{RasterImage,image_decoding::{ImageDecodeInput,ImageDecodeJob,ImageDecodeProgress,ImageDecodeError}};

use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
#[derive(Clone,Copy,Debug)]
pub struct DrawingImageAdmissionProgress{pub decoding:Option<ImageDecodeProgress>,pub samples:usize,pub total_samples:usize,pub work:u64,pub done:bool}
#[derive(semio_framework_value::RetireOwned)]
pub struct DrawingImageAdmissionJob{source_identity:usize,source_length:usize,decoder:ImageDecodeJob,decoding:Option<ImageDecodeProgress>,image:Option<RasterImage>,asset:Option<DrawingImageAsset>,samples:usize,work:u64,phase:u8,cancelled:bool,failure:Option<ImageDecodeError>}
semio_framework_value::artifact_retire_leaf!(DrawingImageAdmissionProgress);
impl DrawingImageAdmissionJob{
 pub fn new(input:ImageDecodeInput<'_>)->Result<Self,ImageDecodeError>{Ok(Self{source_identity:input.data.as_ptr()as usize,source_length:input.data.len(),decoder:ImageDecodeJob::new(input)?,decoding:None,image:None,asset:None,samples:0,work:0,phase:0,cancelled:false,failure:None})}
 fn check(&self)->Result<(),ImageDecodeError>{if self.cancelled{return Err(ImageDecodeError::Cancelled);}if self.phase==4{return Err(ImageDecodeError::Incomplete);}if let Some(error)=&self.failure{return Err(error.clone());}Ok(())}
 pub fn progress(&self)->DrawingImageAdmissionProgress{DrawingImageAdmissionProgress{decoding:self.decoding,samples:self.samples,total_samples:self.asset.as_ref().map_or(0,|asset|asset.width as usize*asset.height as usize),work:self.work,done:self.phase==3}}
 pub fn next_copy_byte_demand(&self)->Result<usize,ImageDecodeError>{match self.phase{0=>self.decoder.next_copy_byte_demand(),1=>Ok(size_of::<RasterImage>()+size_of::<DrawingImageAsset>()),2=>{let asset=self.asset.as_ref().unwrap();Ok(if asset.samples.has_reserved_slot()||self.samples==asset.width as usize*asset.height as usize{4+2*size_of::<usize>()}else{asset.samples.next_reserve_copy_byte_demand().map_err(|_|ImageDecodeError::Invalid("Drawing sample metadata demand refused"))?+2*size_of::<usize>()})},_=>Ok(0)}}
 pub fn next_capacity_byte_demand(&self)->Result<usize,ImageDecodeError>{match self.phase{0=>self.decoder.next_capacity_byte_demand(),2=>{let asset=self.asset.as_ref().unwrap();if self.samples==asset.width as usize*asset.height as usize{Ok(0)}else{asset.samples.next_allocation_bytes().map_err(|_|ImageDecodeError::Invalid("Drawing sample allocation demand refused"))}},_=>Ok(0)}}
 fn step(&mut self,source:&str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ImageDecodeError>{
  if self.phase==0{let(progress,receipt)=self.decoder.advance(source,grant)?;self.decoding=Some(progress);if progress.done{self.phase=1;}return Ok(receipt);}
  if self.phase==1{let Some((image,receipt))=self.decoder.take_result(grant)?else{return Ok(Default::default());};self.asset=Some(DrawingImageAsset{width:image.width,height:image.height,samples:Default::default()});self.image=Some(image);self.phase=2;return Ok(RetainedCloneProgress{copied_bytes:receipt.copied_bytes+size_of::<DrawingImageAsset>(),..receipt});}
  let image=self.image.as_ref().unwrap();let at=self.samples*4;if at==image.pixels.len(){self.phase=3;return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:4+2*size_of::<usize>(),..Default::default()});}
  let asset=self.asset.as_mut().unwrap();if !asset.samples.has_reserved_slot(){let copy=asset.samples.next_reserve_copy_byte_demand().map_err(|_|ImageDecodeError::Invalid("Drawing sample metadata refused"))?;let reserved=asset.samples.reserve_one_funded(grant).map_err(|_|ImageDecodeError::Invalid("Drawing sample allocation refused"))?;return Ok(RetainedCloneProgress{copied_items:reserved.copied_items,copied_bytes:if reserved.copied_items>0{copy+2*size_of::<usize>()}else{0},retained_capacity_bytes:reserved.retained_capacity_bytes,..Default::default()});}
  asset.samples.push_reserved(image.pixels[at..at+4].try_into().unwrap()).map_err(|_|ImageDecodeError::Invalid("Drawing sample reservation refused"))?;self.samples+=1;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:4+2*size_of::<usize>(),..Default::default()})
 }
 pub fn advance(&mut self,source:&str,grant:RetainedCloneGrant)->Result<(DrawingImageAdmissionProgress,RetainedCloneProgress),ImageDecodeError>{self.check()?;if source.as_ptr()as usize!=self.source_identity||source.len()!=self.source_length{return Err(ImageDecodeError::Invalid("Original drawing image source changed"));}let mut receipt=RetainedCloneProgress::default();for _ in 0..grant.maximum_items{if self.phase==3{break;}let copy=self.next_copy_byte_demand()?;let capacity=self.next_capacity_byte_demand()?;if grant.maximum_depth==0||copy>grant.maximum_copy_bytes.saturating_sub(receipt.copied_bytes)||capacity>grant.maximum_capacity_bytes.saturating_sub(receipt.retained_capacity_bytes){break;}let child=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:grant.maximum_capacity_bytes-receipt.retained_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes-receipt.released_bytes,maximum_depth:grant.maximum_depth};let step=match self.step(source,child){Ok(step)=>step,Err(error)=>{self.failure=Some(error.clone());return Err(error);}};if step.copied_items==0{break;}self.work+=step.copied_items as u64;receipt.copied_items+=step.copied_items;receipt.copied_bytes+=step.copied_bytes;receipt.retained_capacity_bytes+=step.retained_capacity_bytes;receipt.released_bytes+=step.released_bytes;}Ok((self.progress(),receipt))}
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn result(&self)->Result<&DrawingImageAsset,ImageDecodeError>{self.check()?;if self.phase!=3{return Err(ImageDecodeError::Incomplete);}self.asset.as_ref().ok_or(ImageDecodeError::Incomplete)}
 pub fn take_result(&mut self,grant:RetainedCloneGrant)->Result<Option<(DrawingImageAsset,RetainedCloneProgress)>,ImageDecodeError>{self.result()?;if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<DrawingImageAsset>()||grant.maximum_depth==0{return Ok(None);}self.phase=4;Ok(Some((self.asset.take().unwrap(),RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<DrawingImageAsset>(),..Default::default()})))}
}
#[derive(Clone,Copy,Debug)]
pub struct DrawingImageEmissionProgress{pub phase:&'static str,pub samples:usize,pub total_samples:usize,pub encoded_bytes:usize,pub work:u64,pub done:bool}

/// 📤️ The original semantic sample owner remains borrowed through refusal and cancellation.
pub struct DrawingImageEmissionJob<'a>{asset:&'a DrawingImageAsset,maximum_encoded_bytes:usize,pixels:Vec<u8>,samples:usize,encoder:Option<semio_framework_pixels::png_encoding::PngEncodeJob>,bytes:Option<Vec<u8>>,phase:&'static str,work:u64,cancelled:bool,failure:Option<String>}
impl<'a> DrawingImageEmissionJob<'a>{
 pub fn new(asset:&'a DrawingImageAsset,maximum_pixels:usize,maximum_encoded_bytes:usize)->Result<Self,String>{
  let count=(asset.width as usize).checked_mul(asset.height as usize).ok_or("Drawing sample extent overflow")?;
  if count==0||count>maximum_pixels||maximum_pixels>16_777_216||count!=asset.samples.len()||!(8..=67_108_864).contains(&maximum_encoded_bytes){return Err("Invalid drawing image emission contract".into());}
  let raw=count.checked_mul(4).and_then(|value|value.checked_add(asset.height as usize)).ok_or("Drawing image emission capacity overflow")?;let capacity=(raw*9).div_ceil(8)+raw.div_ceil(4096)*32+64;if capacity>maximum_encoded_bytes{return Err("Drawing image emission candidate byte limit exceeded".into());}
  Ok(Self{asset,maximum_encoded_bytes,pixels:Vec::new(),samples:0,encoder:None,bytes:None,phase:"samples",work:0,cancelled:false,failure:None})
 }
 fn check(&self)->Result<(),String>{if self.cancelled{return Err("Drawing image emission cancelled".into());}if let Some(error)=&self.failure{return Err(error.clone());}Ok(())}
 fn step(&mut self)->Result<(),String>{
  if self.phase=="samples"{if let Some(sample)=self.asset.samples.get(self.samples){self.pixels.try_reserve(4).map_err(|_|"Drawing image emission sample allocation refused")?;self.pixels.extend_from_slice(sample);self.samples+=1;return Ok(());}
   self.encoder=Some(semio_framework_pixels::png_encoding::PngEncodeJob::new(RasterImage{width:self.asset.width,height:self.asset.height,pixels:std::mem::take(&mut self.pixels)}).map_err(|error|error.to_string())?);self.phase="encoding";return Ok(());}
  let progress=self.encoder.as_mut().unwrap().advance().map_err(|error|error.to_string())?;
  if progress.done{let bytes=self.encoder.take().unwrap().into_result().map_err(|error|error.to_string())?.data;if bytes.len()>self.maximum_encoded_bytes{return Err("Drawing image emission byte limit exceeded".into());}self.bytes=Some(bytes);self.phase="complete";}Ok(())
 }
 pub fn advance(&mut self,grant:usize)->Result<DrawingImageEmissionProgress,String>{
  if grant==0||grant as u128>9_007_199_254_740_991{return Err("Invalid drawing image emission work grant".into());}self.check()?;
  for _ in 0..grant{if self.phase=="complete"{break;}if let Err(error)=self.step(){self.failure=Some(error.clone());return Err(error);}self.work+=1;}
  Ok(DrawingImageEmissionProgress{phase:self.phase,samples:self.samples,total_samples:self.asset.samples.len(),encoded_bytes:self.bytes.as_ref().map_or(0,Vec::len),work:self.work,done:self.phase=="complete"})
 }
 fn release(&mut self){if let Some(encoder)=&mut self.encoder{encoder.cancel();}self.encoder=None;self.pixels=Vec::new();self.bytes=None;}
 pub fn cancel(&mut self){self.cancelled=true;self.release();}
 pub fn result(&self)->Result<&[u8],String>{self.check()?;self.bytes.as_deref().ok_or_else(||"Drawing image emission incomplete".into())}
 pub fn into_result(mut self)->Result<Vec<u8>,String>{self.result()?;Ok(self.bytes.take().unwrap())}
}

/// 🎛️ Physical image emitters expose real bounded sample and PNG progress plus cancellation.
pub fn drawing_image_png_controlled(asset:&DrawingImageAsset,maximum_pixels:usize,maximum_encoded_bytes:usize,progress:&mut dyn FnMut(DrawingImageEmissionProgress)->bool)->Result<Vec<u8>,String>{
 let mut job=DrawingImageEmissionJob::new(asset,maximum_pixels,maximum_encoded_bytes)?;loop{let state=job.advance(1)?;if !progress(state){job.cancel();return Err("Drawing image emission cancelled".into());}if state.done{return job.into_result();}}
}
/// 📤️ Encode a semantic asset as a PNG only at the native output boundary.
pub fn drawing_image_png(asset:&DrawingImageAsset)->Result<Vec<u8>,String>{drawing_image_png_controlled(asset,16_777_216,67_108_864,&mut |_|true)}
/// 🌐️ Physical SVG and browser image sources are emitted from admitted sample state.
pub fn drawing_image_data_uri_controlled(asset:&DrawingImageAsset,progress:&mut dyn FnMut(DrawingImageEmissionProgress)->bool)->Result<String,String>{
 let mut work=0;let bytes=drawing_image_png_controlled(asset,16_777_216,67_108_864,&mut |mut state|{work=state.work;state.done=false;progress(state)})?;
 let encoded=base64_codec::base64_standard_encode_controlled(&bytes,&mut base64_codec::Base64Control{maximum_output_bytes:89_478_488,progress:&mut |state|{work+=1;progress(DrawingImageEmissionProgress{phase:"base64",samples:asset.samples.len(),total_samples:asset.samples.len(),encoded_bytes:state.completed,work,done:state.completed==state.total})}}).map_err(|error|error.to_string())?;
 Ok(format!("data:image/png;base64,{encoded}"))
}
/// 🌐️ Emit a complete native data URI from the same controlled sample owner.
pub fn drawing_image_data_uri(asset:&DrawingImageAsset)->Result<String,String>{drawing_image_data_uri_controlled(asset,&mut |_|true)}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

/// 🖥️ Admit a completed semantic scene node into the browser image-source output format.
pub fn prepared_scene_node_value(node:&crate::schema::scene_view::PreparedSceneNode<'_>,document:&crate::DrawingSnapshot)->Result<semio_framework_value::DslValue,String>{
 use semio_framework_value::ToValue;
 let mut value=node.to_value();
 if let Some((asset,width,height))=node.image{let source=document.assets.get(&asset.id).ok_or("Missing authored image identity")?;let src=drawing_image_data_uri(source)?;
  let semio_framework_value::DslValue::Object(members)=&mut value else{unreachable!()};let image=members.iter_mut().find(|(key,_)|key=="image").ok_or("Missing semantic image projection")?;image.1=semio_framework_value::DslValue::object([("src".into(),src.to_value()),("width".into(),width.to_value()),("height".into(),height.to_value())]);
 }
 Ok(value)
}

/// 🧪️ Cold fixture construction retains every decoder child until its quoted physical close.
#[cfg(test)]
pub(crate) fn drawing_image_from_png(bytes:&[u8])->DrawingImageAsset{
 let source=base64_codec::base64_standard_encode(bytes);
 let mut job=DrawingImageAdmissionJob::new(ImageDecodeInput{mime:"image/png",data:&source,max_source_bytes:268439552,max_bytes:67108864,max_pixels:16777216,max_chunks:65536}).expect("valid fixture PNG source");
 let grant=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:16777216,maximum_capacity_bytes:536870912,maximum_release_bytes:0,maximum_depth:128};
 while !job.advance(&source,grant).expect("fixture PNG admission").0.done{}
 let asset=job.take_result(grant).unwrap().unwrap().0;
 let mut close=semio_framework_value::retirement::controlled::ControlledRetirement::new(job).unwrap_or_else(|_|panic!("fixture decoder owner unsupported"));
 for _ in 0..2000000{if close.terminal_is_empty(){return asset;}let copy=close.next_copy_byte_demand().unwrap();let release=close.next_release_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:close.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:close.next_depth_demand().unwrap()};assert!(close.step(grant).unwrap().progress().fits(grant));}
 panic!("fixture decoder close stalled")
}