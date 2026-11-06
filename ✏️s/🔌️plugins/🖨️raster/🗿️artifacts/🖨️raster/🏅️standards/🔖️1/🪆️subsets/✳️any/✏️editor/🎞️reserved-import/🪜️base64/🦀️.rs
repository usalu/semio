//! 🪜️ Original structured text and strict base64 output stay retained across bounded turns.
use semio_framework_io_base64::{Base64Control,Base64ControlError,Base64DecodeCursor,Base64Input};
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::RetainedCloneClose};
use std::mem::size_of;

fn base64_refusal(error:&Base64ControlError)->ValueError{let(kind,message)=match error{Base64ControlError::Codec(_)=>(ValueRefusalKind::InvalidValue,"Raster original base64 codec refused its source"),Base64ControlError::InputRange=>(ValueRefusalKind::InvalidValue,"Raster original base64 range is invalid"),Base64ControlError::OutputLimit=>(ValueRefusalKind::OwnershipLimit,"Raster original base64 output exceeds its limit"),Base64ControlError::Cancelled=>(ValueRefusalKind::Canceled,"Raster original base64 was cancelled")};ValueError::literal(kind,message)}


#[derive(Debug)]
pub(super) enum CodecFailure{Base64(Base64ControlError),Png(semio_framework_pixels::png_decoding::PngDecodeError)}
impl CodecFailure{
 fn kind(&self)->&'static str{use semio_framework_io_base64::Base64Error as E;match self{Self::Base64(Base64ControlError::Codec(E::InvalidLength))=>"invalidLength",Self::Base64(Base64ControlError::Codec(E::InvalidByte{..}))=>"invalidByte",Self::Base64(Base64ControlError::Codec(E::InvalidPadding))=>"invalidPadding",Self::Base64(Base64ControlError::Codec(E::NonCanonicalTrailingBits))=>"nonCanonicalTrailingBits",Self::Base64(Base64ControlError::InputRange)=>"inputRange",Self::Base64(Base64ControlError::OutputLimit)=>"outputLimit",Self::Base64(Base64ControlError::Cancelled)=>"cancelled",Self::Png(semio_framework_pixels::png_decoding::PngDecodeError::Invalid(_))=>"invalid",Self::Png(semio_framework_pixels::png_decoding::PngDecodeError::Incomplete)=>"incomplete",Self::Png(semio_framework_pixels::png_decoding::PngDecodeError::Cancelled)=>"cancelled"}}
}
impl semio_framework_job::OriginalDomainFailure for CodecFailure{
 fn domain_code(&self)->&str{match self{Self::Base64(_)=>"raster.import.base64",Self::Png(_)=>"raster.import.png"}}
 fn domain_message(&self)->&str{match self{Self::Png(semio_framework_pixels::png_decoding::PngDecodeError::Invalid(message))=>message,Self::Png(_)=>"Original PNG decoding refused",Self::Base64(_)=>"Original base64 decoding refused"}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{Default::default()}
}
impl semio_framework_job::OriginalDomainProjection for CodecFailure{
 fn projection_view(&self,path:&[usize])->Result<semio_framework_job::OriginalDomainView<'_>,ValueError>{use semio_framework_job::OriginalDomainView as V;use semio_framework_io_base64::Base64Error as E;match path{[]=>Ok(V::IntrinsicObject(4)),[0]=>Ok(V::Text(match self{Self::Base64(_)=>"base64",Self::Png(_)=>"png"})),[1]=>Ok(V::Text(self.kind())),[2]=>match self{Self::Base64(Base64ControlError::Codec(E::InvalidByte{index,..}))=>Ok(V::UInt(*index as u64)),Self::Png(semio_framework_pixels::png_decoding::PngDecodeError::Invalid(message))=>Ok(V::Text(message)),_=>Ok(V::IntrinsicNull)},[3]=>match self{Self::Base64(Base64ControlError::Codec(E::InvalidByte{byte,..}))=>Ok(V::UInt(*byte as u64)),_=>Ok(V::IntrinsicNull)},_=>Err(ValueError::invariant("Raster codec failure projection ordinal changed"))}}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path.is_empty(){["codec","kind","detail","byte"].get(index).copied().ok_or_else(||ValueError::invariant("Raster codec failure projection key changed"))}else{Err(ValueError::invariant("Raster codec failure projection path changed"))}}
}
struct CodecFailureRetirement(Option<CodecFailure>);
impl semio_framework_value::retirement::RetireOwned for CodecFailure{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(CodecFailureRetirement(Some(self)))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<CodecFailureRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl semio_framework_value::retirement::RetirementCursor for CodecFailureRetirement{
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:usize::from(self.0.is_some()),..Default::default()})}
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(self.0.is_some()))}
 fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{use semio_framework_value::retirement::RetirementStep;if self.0.is_none(){return RetirementStep::Complete(0)}if grant.maximum_items==0||grant.maximum_depth==0{return RetirementStep::BudgetExhausted}self.0.take();RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()},1)}
 fn terminal_is_empty(&self)->bool{self.0.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.0.is_some()))}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
#[derive(Default)]
pub(super) struct Base64Ingress{cursor:Option<Base64DecodeCursor<'static>>,pub(super) bytes:Option<Vec<u8>>,text:Option<String>,retirement:RetainedCloneClose,pub(super) failure:Option<Base64ControlError>}
impl Base64Ingress{
 pub(super) fn started(&self)->bool{self.cursor.is_some()}
 pub(super) fn begin(&mut self,text:&mut String,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.cursor.is_some(){return Ok(true)}
  let copy=size_of::<Base64DecodeCursor<'static>>()+size_of::<String>();
  semio_framework_job::admit_original_construction(cx,&mut self.cursor,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:copy,capacity_bytes:0,release_bytes:0,depth:1},|_|{
   let cursor=Base64DecodeCursor::new(Base64Input::OwnedText(std::mem::take(text)));
   RetainedWorkOutcome{result:Ok((cursor,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})),work:1}
  })
 }
 pub(super) fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.bytes.is_some(){return Ok(true)}
  let cursor=self.cursor.as_ref().ok_or_else(||ValueError::invariant("Raster base64 source is absent"))?;
  if cursor.refusal().is_some(){self.failure=cursor.refusal().cloned();return Err(base64_refusal(self.failure.as_ref().unwrap()).into())}
  if cursor.is_complete(){let copy=size_of::<Option<Vec<u8>>>();return semio_framework_job::admit_original_construction(cx,&mut self.bytes,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:copy,capacity_bytes:0,release_bytes:0,depth:1},|_|{let bytes=self.cursor.as_mut().unwrap().take_output().ok_or_else(||ValueError::invariant("Raster decoded bytes are absent"));RetainedWorkOutcome{result:bytes.map(|bytes|(bytes,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})),work:1}})}
  let available=usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX);let selected=cursor.next_work_demand(available);if selected==0{return Ok(false)}
  let copy=size_of::<Base64DecodeCursor<'static>>()+cursor.next_copy_byte_demand(available);let capacity=cursor.next_capacity_byte_demand();let cancel=cx.borrowed_original_cancel_token()?;let now=cx.clock_source();let deadline=cx.deadline_us();let mut output=None;
  semio_framework_job::admit_original_construction(cx,&mut output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:copy,capacity_bytes:capacity,release_bytes:0,depth:1},|_|{
   let cursor=self.cursor.as_mut().unwrap();let mut observe=|_|!cancel.is_cancelled_now()&&now().is_some_and(|instant|instant<deadline);let mut control=Base64Control{maximum_output_bytes:67108864,progress:&mut observe};let result=cursor.step(selected,&mut control);let receipt=cursor.last_turn_receipt();let progress=RetainedCloneProgress{copied_items:1,copied_bytes:receipt.copied_bytes,retained_capacity_bytes:receipt.allocated_capacity_bytes,..Default::default()};
   let result=match result{Ok(_)=>Ok(((),progress)),Err(error)=>{let primary=base64_refusal(&error);self.failure=Some(error);Err(primary.with_retained_progress(progress))}};RetainedWorkOutcome{result,work:receipt.work_units as u64}
  })?;Ok(false)
 }
 pub(super) fn close_work(&self,body:usize)->Result<u64,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_work::<String>(false,&None,body)}if self.cursor.is_some(){return Ok(1)}if self.text.is_some(){return self.retirement.next_owner_work::<String>(true,&None,body)}if self.bytes.is_some(){return self.retirement.next_owner_work::<Vec<u8>>(true,&None,body)}Ok(0)}
 pub(super) fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.demands(body)}
  if self.cursor.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Base64DecodeCursor<'static>>()+size_of::<String>()+size_of::<Option<Vec<u8>>>(),depth:1,..Default::default()})}
  if self.text.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,body)}
  if self.bytes.is_some(){return self.retirement.next_owner_demands::<Vec<u8>>(true,&None,body)}
  Ok(Default::default())
 }
 pub(super) fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant)}
  if self.cursor.is_some(){let d=match self.close_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e.into()),work:0}};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let parts=self.cursor.take().unwrap().into_parts();self.text=Some(match parts.input{Base64Input::OwnedText(text)=>text,_=>unreachable!()});if parts.output.is_some(){self.bytes=parts.output}return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}}
  macro_rules! retire{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,grant);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work}}};}retire!(text);retire!(bytes);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub(super) fn empty(&self)->bool{self.cursor.is_none()&&self.text.is_none()&&self.bytes.is_none()&&self.retirement.is_empty()}
}

#[cfg(test)]
mod tests{
 use super::*;
 use semio_plugin::artifact_app_laws::OriginalWorkFixture;
 #[test]
 fn original_base64_source_survives_finite_work_and_funded_close(){
  let cases:serde_json::Value=serde_json::from_str(include_str!("../🧪️structured-wire/image-cases.json")).unwrap();let mut source=cases["cases"][0]["base64"].as_str().unwrap().to_owned();let pointer=source.as_ptr();let mut caller=OriginalWorkFixture::new();let grant=OriginalWorkFixture::stock();let mut ingress=Base64Ingress::default();
  caller.work_turn(grant,0,|cx|{assert!(!ingress.begin(&mut source,cx)?);Ok(())}).unwrap();assert_eq!(source.as_ptr(),pointer);
  caller.work_turn(grant,4096,|cx|{assert!(ingress.begin(&mut source,cx)?);Ok(())}).unwrap();assert!(source.is_empty());assert!(matches!(ingress.cursor.as_ref().unwrap().input(),Base64Input::OwnedText(text) if text.as_ptr()==pointer));
  for _ in 0..10000{if ingress.bytes.is_some(){break}caller.work_turn(grant,4096,|cx|{ingress.step(cx)?;Ok(())}).unwrap()}assert_eq!(&ingress.bytes.as_ref().unwrap()[..8],&[137,80,78,71,13,10,26,10]);let pointer=ingress.bytes.as_ref().unwrap().as_ptr();let mut png=PngIngress::default();caller.work_turn(grant,4096,|cx|{png.step(&mut ingress.bytes,cx)?;Ok(())}).unwrap();assert_eq!(png.job.as_ref().unwrap().source().as_ptr(),pointer);let before=png.job.as_ref().unwrap().progress();caller.work_turn(grant,0,|cx|{assert!(!png.step(&mut ingress.bytes,cx)?);Ok(())}).unwrap();assert_eq!(png.job.as_ref().unwrap().progress(),before);for _ in 0..200000{if png.phase==5{break}caller.work_turn(grant,4096,|cx|{png.step(&mut ingress.bytes,cx)?;Ok(())}).unwrap()}assert_eq!(png.phase,5);assert_eq!(png.job.as_ref().unwrap().source().as_ptr(),pointer);let row=&cases["cases"][0];assert_eq!(png.image.as_ref().unwrap().pixels,vec![10,20,30,255]);assert_eq!(serde_json::to_value(png.metadata.as_ref().unwrap()).unwrap(),row["metadata"]);let pixel_pointer=png.image.as_ref().unwrap().pixels.as_ptr();let mut builder=RasterCandidateBuilder::default();let mut candidate=None;for _ in 0..10000{if candidate.is_some(){break}caller.work_turn(grant,4096,|cx|{builder.step(&mut png,&mut candidate,cx)?;Ok(())}).unwrap()}let image=candidate.as_ref().unwrap().1.as_ref().unwrap();assert_eq!(image.frames[0].rgba8.as_ptr(),pixel_pointer);let(_,oracle,_)=semio_s_artifact_raster_raster_io::raster_image_layer_and_asset(row["base64"].as_str().unwrap()).unwrap();assert_eq!(image,&oracle);assert_eq!(image.metadata.len(),2);assert_eq!(image.metadata[1].value,"café");let mut candidate_retirement=RetainedCloneClose::default();for _ in 0..10000{if candidate.is_none()&&candidate_retirement.is_empty(){break}caller.work_turn(grant,4096,|cx|{let d=candidate_retirement.next_owner_demands::<super::super::Candidate>(candidate.is_some(),&None,cx.retained_grant().maximum_copy_bytes)?;let work=candidate_retirement.next_owner_work::<super::super::Candidate>(candidate.is_some(),&None,cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_quoted_retained_step(cx,d,work,|grant|{if !candidate_retirement.is_empty(){return candidate_retirement.step_granted(grant)}let turn=candidate_retirement.begin_granted(&mut candidate,grant);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work}})?;Ok(())}).unwrap()}assert!(candidate.is_none()&&candidate_retirement.is_empty());for _ in 0..10000{if builder.empty(){break}caller.work_turn(grant,4096,|cx|{let demand=builder.close_demands(cx.retained_grant().maximum_copy_bytes)?;let work=builder.close_work(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_quoted_retained_step(cx,demand,work,|grant|builder.close_step(grant))?;Ok(())}).unwrap()}assert!(builder.empty());caller.work_turn(grant,0,|cx|{let demand=png.close_demands(cx.retained_grant().maximum_copy_bytes)?;let work=png.close_work(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_quoted_retained_step(cx,demand,work,|grant|png.close_step(grant))?;Ok(())}).unwrap();assert_eq!(png.job.as_ref().unwrap().source().as_ptr(),pointer);for _ in 0..10000{if png.empty(){break}caller.work_turn(grant,4096,|cx|{let demand=png.close_demands(cx.retained_grant().maximum_copy_bytes)?;let work=png.close_work(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_quoted_retained_step(cx,demand,work,|grant|png.close_step(grant))?;Ok(())}).unwrap()}assert!(png.empty());
  for _ in 0..10000{if ingress.empty(){break}caller.work_turn(grant,4096,|cx|{let demand=ingress.close_demands(cx.retained_grant().maximum_copy_bytes)?;let work=ingress.close_work(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_quoted_retained_step(cx,demand,work,|grant|ingress.close_step(grant))?;Ok(())}).unwrap()}assert!(ingress.empty());caller.finish();
 }
 #[test]
 fn original_png_cancellation_retains_source_until_funded_close(){
  let cases:serde_json::Value=serde_json::from_str(include_str!("../🧪️structured-wire/image-cases.json")).unwrap();let mut source=Some(semio_framework_io_base64::base64_standard_decode(cases["cases"][0]["base64"].as_str().unwrap()).unwrap());let pointer=source.as_ref().unwrap().as_ptr();let mut caller=OriginalWorkFixture::new();let grant=OriginalWorkFixture::stock();let mut png=PngIngress::default();caller.work_turn(grant,4096,|cx|{png.step(&mut source,cx)?;Ok(())}).unwrap();assert!(source.is_none());assert_eq!(png.job.as_ref().unwrap().source().as_ptr(),pointer);png.job.as_mut().unwrap().cancel();caller.work_turn(grant,0,|cx|{let d=png.close_demands(cx.retained_grant().maximum_copy_bytes)?;let work=png.close_work(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_quoted_retained_step(cx,d,work,|grant|png.close_step(grant))?;Ok(())}).unwrap();assert_eq!(png.job.as_ref().unwrap().source().as_ptr(),pointer);for _ in 0..10000{if png.empty(){break}caller.work_turn(grant,4096,|cx|{let d=png.close_demands(cx.retained_grant().maximum_copy_bytes)?;let work=png.close_work(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_quoted_retained_step(cx,d,work,|grant|png.close_step(grant))?;Ok(())}).unwrap()}assert!(png.empty());caller.finish();
 }

 #[test]
 fn original_codec_domain_keeps_invalid_byte_and_denied_publication(){
  use semio_framework_io_base64::Base64Error;use semio_framework_job::{OriginalDomainReservation,OriginalDomainView,OriginalTurnPrimary,admit_original_domain_failure};let mut caller=OriginalWorkFixture::new();let grant=OriginalWorkFixture::stock();let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧪️structured-wire/image-cases.json")).unwrap();let row=&corpus["errors"][0];let expected_index=row["index"].as_u64().unwrap()as usize;let expected_byte=row["byte"].as_u64().unwrap()as u8;let mut frame=OriginalDomainReservation::<CodecFailure>::new();let mut ingress=Base64Ingress::default();let mut source=row["text"].as_str().unwrap().to_owned();let pointer=source.as_ptr();caller.work_turn(grant,4096,|cx|{assert!(frame.prepare(cx)?);assert!(ingress.begin(&mut source,cx)?);Ok(())}).unwrap();
  for _ in 0..100{if frame.pending(){break}caller.work_turn(grant,4096,|cx|{frame.with_body(cx,|same,_|ingress.step(same).map(|_|()))?;Ok(())}).unwrap()}assert!(frame.pending());assert!(matches!(ingress.failure.as_ref(),Some(Base64ControlError::Codec(Base64Error::InvalidByte{index,byte})) if *index==expected_index&&*byte==expected_byte));let mut capture_denied=grant;capture_denied.maximum_items=0;caller.work_turn(capture_denied,4096,|cx|{assert!(!frame.capture_original(cx,||CodecFailure::Base64(ingress.failure.take().unwrap()))?);Ok(())}).unwrap();assert!(ingress.failure.is_some()&&frame.pending_original().is_none());caller.work_turn(grant,4096,|cx|{assert!(frame.capture_original(cx,||CodecFailure::Base64(ingress.failure.take().unwrap()))?);Ok(())}).unwrap();assert!(matches!(frame.pending_original(),Some(CodecFailure::Base64(Base64ControlError::Codec(Base64Error::InvalidByte{index,byte}))) if *index==expected_index&&*byte==expected_byte));assert!(matches!(ingress.cursor.as_ref().unwrap().input(),Base64Input::OwnedText(text)if text.as_ptr()==pointer));let identity=frame.allocation_identity();let mut denied=grant;denied.maximum_items=0;caller.work_turn(denied,4096,|cx|{assert!(admit_original_domain_failure(&mut frame,cx)?.is_none());Ok(())}).unwrap();assert_eq!(frame.allocation_identity(),identity);assert!(frame.pending());let mut output=None;caller.work_turn(grant,4096,|cx|{output=admit_original_domain_failure(&mut frame,cx)?;Ok(())}).unwrap();let failure=output.unwrap();let OriginalTurnPrimary::Domain(domain)=&failure.primary else{panic!("exact Raster codec domain")};assert!(matches!(domain.projection().projection_view(&[0,1]).unwrap(),OriginalDomainView::Text(kind) if kind==row["kind"].as_str().unwrap()));assert!(matches!(domain.projection().projection_view(&[0,2]).unwrap(),OriginalDomainView::UInt(value) if value==expected_index as u64));assert!(matches!(domain.projection().projection_view(&[0,3]).unwrap(),OriginalDomainView::UInt(value) if value==expected_byte as u64));caller.retire_owned(failure,grant);assert!(frame.terminal_is_empty());for _ in 0..10000{if ingress.empty(){break}caller.work_turn(grant,4096,|cx|{let d=ingress.close_demands(cx.retained_grant().maximum_copy_bytes)?;let work=ingress.close_work(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_quoted_retained_step(cx,d,work,|grant|ingress.close_step(grant))?;Ok(())}).unwrap()}assert!(ingress.empty());caller.finish();
 }

}

use semio_framework_pixels::{RasterImage,png_decoding::{PngDecodeJob,PngDecodeInput,PngDecodeError,PngDecodedProperties}};

fn png_refusal(error:&PngDecodeError)->ValueError{match error{PngDecodeError::Invalid(message)=>ValueError::literal(ValueRefusalKind::InvalidValue,message),PngDecodeError::Incomplete=>ValueError::invariant("Raster original PNG is incomplete"),PngDecodeError::Cancelled=>ValueError::literal(ValueRefusalKind::Canceled,"Raster original PNG was cancelled")}}

#[derive(Default)]
pub(super) struct PngIngress{
 pub(super) job:Option<PngDecodeJob>,pub(super) properties:Option<PngDecodedProperties>,pub(super) metadata:Option<Vec<(String,String)>>,pub(super) image:Option<RasterImage>,pub(super) failure:Option<PngDecodeError>,phase:u8,retirement:RetainedCloneClose,
}
impl PngIngress{
 pub(super) fn started(&self)->bool{self.job.is_some()}
 pub(super) fn ready(&self)->bool{self.phase==5}
 pub(super) fn step(&mut self,bytes:&mut Option<Vec<u8>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if let Some(error)=self.failure.as_ref(){return Err(png_refusal(error).into())}
  if self.phase==5{return Ok(true)}
  if self.job.is_none(){
   if bytes.is_none(){return Err(ValueError::invariant("Raster PNG bytes are absent").into())}
   let copy=size_of::<PngDecodeJob>()+size_of::<PngDecodeInput>();
   return semio_framework_job::admit_original_construction(cx,&mut self.job,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:copy,capacity_bytes:0,release_bytes:0,depth:1},|_|{
    let input=PngDecodeInput{data:bytes.take().unwrap(),max_pixels:16777216,max_bytes:67108864,max_chunks:65536};
    let result=match PngDecodeJob::new(input){Ok(job)=>Ok((job,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})),Err((error,input))=>{*bytes=Some(input.data);let primary=png_refusal(&error);self.failure=Some(error);Err(primary.with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<Option<Vec<u8>>>()+size_of::<PngDecodeInput>(),..Default::default()}))}};
    RetainedWorkOutcome{result,work:1}
   }).map(|_|false)
  }
  if cx.fuel_remaining()==0{return Ok(false)}
  let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,..cx.retained_grant()};
  let job=self.job.as_ref().unwrap();
  let quoted:Result<RetirementDemand,PngDecodeError>=(||Ok(match self.phase{
   0=>RetirementDemand{copy_bytes:job.next_copy_byte_demand()?,capacity_bytes:job.next_capacity_byte_demand()?,depth:job.next_depth_demand()?,..Default::default()},
   1=>RetirementDemand{copy_bytes:size_of::<PngDecodedProperties>(),depth:1,..Default::default()},
   2=>PngDecodeJob::text_metadata_birth_demands(),
   3=>job.text_metadata_demands()?,
   4=>RetirementDemand{copy_bytes:if self.metadata.is_none(){size_of::<Vec<(String,String)>>()+size_of::<Option<Vec<(String,String)>>>()}else{size_of::<RasterImage>()+size_of::<Option<RasterImage>>()},depth:1,..Default::default()},
   _=>return Err(PngDecodeError::Incomplete),
  }))();let demand=match quoted{Ok(demand)=>demand,Err(error)=>{let primary=png_refusal(&error);self.failure=Some(error);return Err(primary.into())}};
  let mut output=None;
  semio_framework_job::admit_original_construction(cx,&mut output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth},|_|{
   let job=self.job.as_mut().unwrap();let mut work=1;
   let result:Result<RetainedCloneProgress,PngDecodeError>=match self.phase{
    0=>match job.advance(grant){Ok((progress,receipt))=>{work=job.last_turn_work();if progress.done{self.phase=1}Ok(receipt)},Err(error)=>{work=job.last_turn_work();Err(error)}},
    1=>job.decoded_properties().map(|properties|{self.properties=Some(properties);self.phase=2;RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}),
    2=>job.begin_text_metadata(67108864,grant).map(|receipt|{if receipt.is_some(){self.phase=3}receipt.unwrap_or_default()}),
    3=>match job.advance_text_metadata(grant){Ok((done,receipt))=>{work=job.text_metadata_last_work();if done{self.phase=4}Ok(receipt)},Err(error)=>{work=job.text_metadata_last_work();Err(error)}},
    4=>{if self.metadata.is_none(){job.take_text_metadata(grant).map(|value|{if let Some((rows,receipt))=value{self.metadata=Some(rows);let mut receipt=receipt;receipt.copied_bytes+=size_of::<Option<Vec<(String,String)>>>();receipt}else{RetainedCloneProgress::default()}})}else{job.take_result(grant).map(|value|{if let Some((image,receipt))=value{self.image=Some(image);self.phase=5;let mut receipt=receipt;receipt.copied_bytes+=size_of::<Option<RasterImage>>();receipt}else{RetainedCloneProgress::default()}})}},
    _=>unreachable!(),
   };
   let result=match result{Ok(receipt)=>Ok(((),receipt)),Err(error)=>{let progress=if self.phase==3{job.text_metadata_last_progress()}else if self.phase==0{job.last_turn_progress()}else{RetainedCloneProgress::default()};let primary=png_refusal(&error);self.failure=Some(error);Err(primary.with_retained_progress(progress))}};
   RetainedWorkOutcome{result,work}
  })?;Ok(self.phase==5)
 }
 pub(super) fn close_work(&self,body:usize)->Result<u64,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_work::<String>(false,&None,body)}if self.job.is_some(){return self.retirement.next_owner_work::<PngDecodeJob>(true,&None,body)}if self.metadata.is_some(){return self.retirement.next_owner_work::<Vec<(String,String)>>(true,&None,body)}if self.image.is_some(){return self.retirement.next_owner_work::<RasterImage>(true,&None,body)}Ok(0)}
 pub(super) fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.job.is_some(){return self.retirement.next_owner_demands::<PngDecodeJob>(true,&None,body)}if self.metadata.is_some(){return self.retirement.next_owner_demands::<Vec<(String,String)>>(true,&None,body)}if self.image.is_some(){return self.retirement.next_owner_demands::<RasterImage>(true,&None,body)}Ok(Default::default())}
 pub(super) fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{if !self.retirement.is_empty(){return self.retirement.step_granted(grant)}macro_rules! retire{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,grant);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work}}};}retire!(job);retire!(metadata);retire!(image);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
 pub(super) fn empty(&self)->bool{self.job.is_none()&&self.metadata.is_none()&&self.image.is_none()&&self.retirement.is_empty()}
}

use semio_s_artifact_raster_raster_model::{SemioImageFrame,SemioImageMetadataEntry,SemioImageSnapshot,SemioColorspace,STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA,RasterLayerNode,RasterTransform,raster_id_bytes,write_raster_id};

#[derive(Default)]
pub(super) struct RasterCandidateBuilder{strings:[Option<String>;6],frames:Option<Vec<SemioImageFrame>>,metadata:Option<Vec<SemioImageMetadataEntry>>,row:usize,phase:u8,retirement:RetainedCloneClose}
impl RasterCandidateBuilder{
 pub(super) fn step(&mut self,png:&mut PngIngress,candidate:&mut Option<super::Candidate>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if candidate.is_some(){return Ok(true)}
  if png.phase!=5{return Ok(false)}
  if self.phase<6{
   let index=self.phase as usize;let(prior,tail)=self.strings.split_at_mut(index);let literal=match index{0=>"image-in-asset",1=>"layer",2=>"Imported Image",3=>"normal",4=>prior[0].as_ref().ok_or_else(||ValueError::invariant("Raster asset identity is absent"))?.as_str(),_=>STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};let capacity=if index<2{raster_id_bytes(literal).ok_or_else(||ValueError::invariant("Raster identity extent overflow"))?}else{literal.len()};let copy=size_of::<String>()+capacity;
   let phase=&mut self.phase;semio_framework_job::admit_original_construction(cx,&mut tail[0],semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:copy,capacity_bytes:capacity,release_bytes:0,depth:1},|_|{
    let mut value=String::new();let result=match value.try_reserve_exact(capacity){Err(_)=>Err(ValueError::invariant("Raster string allocation refused")),Ok(())=>{if index<2{write_raster_id(literal,&mut value).expect("funded Raster identity output")}else{value.push_str(literal)}let actual=value.capacity();*phase+=1;Ok((value,RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:actual,..Default::default()}))}};RetainedWorkOutcome{result,work:1}
   })?;return Ok(false)
  }
  if self.phase==6{let capacity=size_of::<SemioImageFrame>();let phase=&mut self.phase;semio_framework_job::admit_original_construction(cx,&mut self.frames,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:size_of::<Vec<SemioImageFrame>>(),capacity_bytes:capacity,release_bytes:0,depth:1},|_|{let mut frames=Vec::new();let result=match frames.try_reserve_exact(1){Ok(())=>{let actual=frames.capacity()*size_of::<SemioImageFrame>();*phase=7;Ok((frames,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<SemioImageFrame>>(),retained_capacity_bytes:actual,..Default::default()}))},Err(_)=>Err(ValueError::invariant("Raster frame allocation refused"))};RetainedWorkOutcome{result,work:1}})?;return Ok(false)}
  if self.phase==7{let count=png.metadata.as_ref().ok_or_else(||ValueError::invariant("Raster text metadata is absent"))?.len();let capacity=count.checked_mul(size_of::<SemioImageMetadataEntry>()).ok_or_else(||ValueError::invariant("Raster metadata extent overflow"))?;let phase=&mut self.phase;semio_framework_job::admit_original_construction(cx,&mut self.metadata,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:size_of::<Vec<SemioImageMetadataEntry>>(),capacity_bytes:capacity,release_bytes:0,depth:1},|_|{let mut rows=Vec::new();let result=match rows.try_reserve_exact(count){Ok(())=>{let actual=rows.capacity()*size_of::<SemioImageMetadataEntry>();*phase=8;Ok((rows,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<SemioImageMetadataEntry>>(),retained_capacity_bytes:actual,..Default::default()}))},Err(_)=>Err(ValueError::invariant("Raster metadata allocation refused"))};RetainedWorkOutcome{result,work:1}})?;return Ok(false)}
  if self.phase==8{let rows=png.metadata.as_mut().ok_or_else(||ValueError::invariant("Raster metadata owner is absent"))?;if self.row<rows.len(){let copy=size_of::<(String,String)>()+size_of::<SemioImageMetadataEntry>();let mut output=None;semio_framework_job::admit_original_construction(cx,&mut output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:copy,capacity_bytes:0,release_bytes:0,depth:1},|_|{let(key,value)=std::mem::take(&mut rows[self.row]);self.metadata.as_mut().unwrap().push(SemioImageMetadataEntry{key,value});self.row+=1;RetainedWorkOutcome{result:Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})),work:1}})?;return Ok(false)}let mut output=None;let copy=size_of::<u8>();semio_framework_job::admit_original_construction(cx,&mut output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:copy,capacity_bytes:0,release_bytes:0,depth:1},|_|{self.phase=9;RetainedWorkOutcome{result:Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})),work:1}})?;return Ok(false)}
  let properties=png.properties.ok_or_else(||ValueError::invariant("Raster PNG properties are absent"))?;let colorspace=match properties.color_type{0=>SemioColorspace::Grayscale,2=>SemioColorspace::Rgb,3=>SemioColorspace::Indexed,4=>SemioColorspace::GrayscaleAlpha,6=>SemioColorspace::Rgba,_=>return Err(ValueError::invariant("Raster PNG colorspace is invalid").into())};if png.image.is_none(){return Err(ValueError::invariant("Raster decoded image is absent").into())}
  let copy=size_of::<super::Candidate>()+size_of::<SemioImageSnapshot>()+size_of::<SemioImageFrame>()+size_of::<RasterImage>();
  semio_framework_job::admit_original_construction(cx,candidate,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:copy,capacity_bytes:0,release_bytes:0,depth:1},|_|{
   let image=png.image.take().unwrap();let mut frames=self.frames.take().unwrap();frames.push(SemioImageFrame{delay_ms:0,rgba8:image.pixels});let mut take=|index:usize|self.strings[index].take().unwrap();let asset=take(0);let id=take(1);let name=take(2);let blend=take(3);let key=take(4);let schema=take(5);let snapshot=SemioImageSnapshot{schema,width:properties.width,height:properties.height,colorspace,bit_depth:properties.bit_depth,frames,icc:None,metadata:self.metadata.take().unwrap()};let layer=RasterLayerNode::Pixel{id,name,visible:true,locked:false,opacity:1.0,blend_mode:blend,transform:RasterTransform::default(),mask:None,width:Some(properties.width),height:Some(properties.height),image_key:Some(key)};
   RetainedWorkOutcome{result:Ok(((asset,Some(snapshot),layer),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})),work:1}
  })
 }
 pub(super) fn close_work(&self,body:usize)->Result<u64,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_work::<String>(false,&None,body)}for value in &self.strings{if value.is_some(){return self.retirement.next_owner_work::<String>(true,&None,body)}}if self.frames.is_some(){return self.retirement.next_owner_work::<Vec<SemioImageFrame>>(true,&None,body)}if self.metadata.is_some(){return self.retirement.next_owner_work::<Vec<SemioImageMetadataEntry>>(true,&None,body)}Ok(0)}
 pub(super) fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(body)}for value in &self.strings{if value.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,body)}}if self.frames.is_some(){return self.retirement.next_owner_demands::<Vec<SemioImageFrame>>(true,&None,body)}if self.metadata.is_some(){return self.retirement.next_owner_demands::<Vec<SemioImageMetadataEntry>>(true,&None,body)}Ok(Default::default())}
 pub(super) fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{if !self.retirement.is_empty(){return self.retirement.step_granted(grant)}for value in &mut self.strings{if value.is_some(){let turn=self.retirement.begin_granted(value,grant);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work}}}macro_rules! retire{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,grant);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work}}};}retire!(frames);retire!(metadata);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
 pub(super) fn empty(&self)->bool{self.strings.iter().all(Option::is_none)&&self.frames.is_none()&&self.metadata.is_none()&&self.retirement.is_empty()}
}
