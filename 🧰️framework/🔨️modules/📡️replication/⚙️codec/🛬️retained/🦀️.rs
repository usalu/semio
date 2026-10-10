//! 🛬️ A borrowed wire field preserves its original native buffer and every granted receiving receipt.
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress};
use std::mem::ManuallyDrop;
#[path="🔣️json-string/🦀️.rs"]
mod json_string_source;
use json_string_source::RetainedWireJsonStringSource;


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalWireSourceIdentity{pub address:usize,pub extent:usize,pub authority:Option<(u64,u64)>}
pub trait RetainedWireByteSource{
 fn original_source_identity(&self)->OriginalWireSourceIdentity;
 fn byte_at(&self,index:usize)->Option<u8>;
}
impl RetainedWireByteSource for [u8]{
 fn original_source_identity(&self)->OriginalWireSourceIdentity{OriginalWireSourceIdentity{address:self.as_ptr()as usize,extent:self.len(),authority:None}}
 fn byte_at(&self,index:usize)->Option<u8>{self.get(index).copied()}
}
impl RetainedWireByteSource for Vec<u8>{
 fn original_source_identity(&self)->OriginalWireSourceIdentity{self.as_slice().original_source_identity()}
 fn byte_at(&self,index:usize)->Option<u8>{self.get(index).copied()}
}
impl<T:RetainedWireByteSource+?Sized> RetainedWireByteSource for &T{
 fn original_source_identity(&self)->OriginalWireSourceIdentity{(**self).original_source_identity()}
 fn byte_at(&self,index:usize)->Option<u8>{(**self).byte_at(index)}
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetainedWireFieldStep{Blocked,Progress,Ready,Closed}

#[derive(Clone,Copy)]
pub(crate) struct RetainedWireUtf8Validation{remaining:u8,lower:u8,upper:u8}
impl RetainedWireUtf8Validation{
 pub(crate) fn new()->Self{Self{remaining:0,lower:0x80,upper:0xbf}}
 pub(crate) fn is_complete(&self)->bool{self.remaining==0}
 pub(crate) fn consume(mut self,bytes:&[u8])->Result<Self,ValueError>{
  for byte in bytes{
   if self.remaining!=0{if *byte<self.lower||*byte>self.upper{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original wire text is not UTF8"))}self.remaining-=1;self.lower=0x80;self.upper=0xbf;}
   else{match *byte{0..=0x7f=>{},0xc2..=0xdf=>self.remaining=1,0xe0=>{self.remaining=2;self.lower=0xa0;},0xe1..=0xec|0xee..=0xef=>self.remaining=2,0xed=>{self.remaining=2;self.upper=0x9f;},0xf0=>{self.remaining=3;self.lower=0x90;},0xf1..=0xf3=>self.remaining=3,0xf4=>{self.remaining=3;self.upper=0x8f;},_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original wire text is not UTF8"))}}
  }
  Ok(self)
 }
}

#[derive(Clone,Copy)]
enum RetainedWireFieldSource{Plain,Json(RetainedWireJsonStringSource),Range{original_range:(usize,usize)},PrefixedUuid{original_prefix_range:(usize,usize),original_uuid_range:(usize,usize)},Timestamp{original_timestamp_range:(usize,usize),epoch_millis:i64,year:i64,month:u8,day:u8,hour:u8,minute:u8,second:u8,millis:u16,year_width:usize}}
pub struct RetainedWireFieldDecode{original:usize,extent:usize,authority:Option<(u64,u64)>,output_extent:usize,source:RetainedWireFieldSource,output:ManuallyDrop<Option<Vec<u8>>>,receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,phase:u8,closing:bool,text:bool,utf8:RetainedWireUtf8Validation}
impl RetainedWireFieldDecode{
 /// 👁️ Borrows the original payload without allocating any field or receipt backing.
 pub fn new<S:RetainedWireByteSource+?Sized>(source:&S,text:bool)->Self{let identity=source.original_source_identity();Self{original:identity.address,extent:identity.extent,authority:identity.authority,output_extent:identity.extent,source:RetainedWireFieldSource::Plain,output:ManuallyDrop::new(None),receipt:None,phase:0,closing:false,text,utf8:RetainedWireUtf8Validation::new()}}
 /// 📏️ Binds one original field range while retaining the complete source's actual owner identity.
 pub fn new_range<S:RetainedWireByteSource+?Sized>(source:&S,original_range:(usize,usize),text:bool)->Result<Self,ValueError>{let width=original_range.1.checked_sub(original_range.0).filter(|_|original_range.1<=source.original_source_identity().extent).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original wire field escaped its source"))?;let mut owner=Self::new(source,text);owner.output_extent=width;owner.source=RetainedWireFieldSource::Range{original_range};Ok(owner)}
 /// 🔣️ Binds exactly quoted original JSON and defers all syntax scanning and derived writes to paid pieces.
 pub fn new_json_string<S:RetainedWireByteSource+?Sized>(source:&S,original_range:(usize,usize))->Result<Self,ValueError>{let json=RetainedWireJsonStringSource::new(source,original_range)?;let mut decoder=Self::new(source,true);decoder.source=RetainedWireFieldSource::Json(json);decoder.output_extent=0;decoder.phase=4;Ok(decoder)}
 /// 🪪️ Binds the original prefix and sixteen UUID bytes without materializing derived text.
 pub fn new_prefixed_uuid<S:RetainedWireByteSource+?Sized>(source:&S,original_prefix_range:(usize,usize),original_uuid_range:(usize,usize))->Result<Self,ValueError>{
  let extent=source.original_source_identity().extent;let prefix=original_prefix_range.1.checked_sub(original_prefix_range.0).filter(|_|original_prefix_range.1<=extent).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original UUID prefix escaped its source"))?;
  original_uuid_range.1.checked_sub(original_uuid_range.0).filter(|width|*width==16&&original_uuid_range.1<=extent).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original UUID requires sixteen source bytes"))?;
  let output_extent=prefix.checked_add(37).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original UUID text extent overflow"))?;
  let mut decoder=Self::new(source,true);decoder.output_extent=output_extent;decoder.source=RetainedWireFieldSource::PrefixedUuid{original_prefix_range,original_uuid_range};Ok(decoder)
 }
 /// ⏱️ Binds one original tagged scalar and writes its derived timestamp only through funded native pieces.
 pub fn new_timestamp<S:RetainedWireByteSource+?Sized>(source:&S,original_timestamp_range:(usize,usize),previous_epoch_millis:Option<i64>)->Result<Self,ValueError>{
  let extent=source.original_source_identity().extent;if original_timestamp_range.0>=original_timestamp_range.1||original_timestamp_range.1>extent{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original timestamp escaped its source"))}let tag=source.byte_at(original_timestamp_range.0).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original timestamp tag is absent"))?;
  let mut position=original_timestamp_range.0+1;let value=Self::timestamp_varint(source,&mut position,original_timestamp_range.1)?;
  if tag==0{let length=usize::try_from(value).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original raw timestamp extent overflow"))?;let end=position.checked_add(length).filter(|end|*end==original_timestamp_range.1).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original raw timestamp extent is invalid"))?;let mut decoder=Self::new(source,true);decoder.output_extent=length;decoder.source=RetainedWireFieldSource::Range{original_range:(position,end)};return Ok(decoder)}
  if position!=original_timestamp_range.1{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original timestamp has trailing bytes"))}
  let epoch_millis=match tag{1=>value as i64,2=>{let previous=previous_epoch_millis.ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original delta timestamp has no original base"))?;let delta=((value>>1)as i64)^(-((value&1)as i64));previous.checked_add(delta).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original timestamp epoch overflow"))?},_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original timestamp tag is invalid"))};
  let(year,month,day)=crate::scalar::civil_from_days(epoch_millis.div_euclid(86_400_000));let rem=epoch_millis.rem_euclid(86_400_000);let hour=(rem/3_600_000)as u8;let minute=(rem%3_600_000/60_000)as u8;let second=(rem%60_000/1000)as u8;let millis=(rem%1000)as u16;let mut digits=1;let mut number=year.unsigned_abs();while number>=10{number/=10;digits+=1}let year_width=(digits+usize::from(year<0)).max(4);
  let mut decoder=Self::new(source,true);decoder.output_extent=year_width+16+if millis==0{0}else{4};decoder.source=RetainedWireFieldSource::Timestamp{original_timestamp_range,epoch_millis,year,month:month as u8,day:day as u8,hour,minute,second,millis,year_width};Ok(decoder)
 }
 fn timestamp_varint<S:RetainedWireByteSource+?Sized>(source:&S,position:&mut usize,end:usize)->Result<u64,ValueError>{let mut value=0u64;for shift in(0..70).step_by(7){let byte=source.byte_at(*position).filter(|_|*position<end).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original timestamp varint is truncated"))?;*position+=1;if shift==63&&byte>1{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original timestamp varint overflow"))}value|=u64::from(byte&127)<<shift;if byte&128==0{return Ok(value)}}Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original timestamp varint overflow"))}
 pub fn timestamp_epoch_millis(&self)->Option<i64>{match self.source{RetainedWireFieldSource::Timestamp{epoch_millis,..}=>Some(epoch_millis),_=>None}}
 fn decimal_byte(value:u64,width:usize,position:usize)->u8{let mut divisor=1;for _ in position+1..width{divisor*=10}b'0'+((value/divisor)%10)as u8}
 fn timestamp_byte(year:i64,month:u8,day:u8,hour:u8,minute:u8,second:u8,millis:u16,year_width:usize,position:usize)->u8{
  if position<year_width{if year<0&&position==0{return b'-'}let sign=usize::from(year<0);return Self::decimal_byte(year.unsigned_abs(),year_width-sign,position-sign)}
  match position-year_width{0|3=>b'-',1|2=>Self::decimal_byte(u64::from(month),2,position-year_width-1),4|5=>Self::decimal_byte(u64::from(day),2,position-year_width-4),6=>b'T',7|8=>Self::decimal_byte(u64::from(hour),2,position-year_width-7),9|12=>b':',10|11=>Self::decimal_byte(u64::from(minute),2,position-year_width-10),13|14=>Self::decimal_byte(u64::from(second),2,position-year_width-13),15 if millis!=0=>b'.',16..=18 if millis!=0=>Self::decimal_byte(u64::from(millis),3,position-year_width-16),_=>b'Z'}
 }
 fn source_byte<S:RetainedWireByteSource+?Sized>(&self,source:&S,position:usize)->Result<u8,ValueError>{
  let byte=|offset|source.byte_at(offset).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original wire byte is absent"));Ok(
  match self.source{
   RetainedWireFieldSource::Plain=>byte(position)?,
   RetainedWireFieldSource::Json(_)=>unreachable!("original JSON requires its granted sequential byte cursor"),
   RetainedWireFieldSource::Range{original_range}=>byte(original_range.0+position)?,
   RetainedWireFieldSource::Timestamp{original_timestamp_range,epoch_millis:_,year,month,day,hour,minute,second,millis,year_width}=>{let _=original_timestamp_range;Self::timestamp_byte(year,month,day,hour,minute,second,millis,year_width,position)},
   RetainedWireFieldSource::PrefixedUuid{original_prefix_range,original_uuid_range}=>{
    let prefix=original_prefix_range.1-original_prefix_range.0;if position<prefix{return byte(original_prefix_range.0+position)}let suffix=position-prefix;if suffix==0{return Ok(b'-')}let uuid_position=suffix-1;if matches!(uuid_position,8|13|18|23){return Ok(b'-')}let digits=uuid_position-[8,13,18,23].into_iter().filter(|dash|*dash<uuid_position).count();let byte=byte(original_uuid_range.0+digits/2)?;b"0123456789abcdef"[if digits%2==0{byte>>4}else{byte&15}as usize]
   }
  })
 }
 pub fn source_identity(&self)->(usize,usize){(self.original,self.extent)}
 pub fn original_source_identity(&self)->OriginalWireSourceIdentity{OriginalWireSourceIdentity{address:self.original,extent:self.extent,authority:self.authority}}
 /// 🧳️ Takes the exact published backing from its real caller slot after metadata admission.
 pub fn admit_received(original:&mut Option<Vec<u8>>,grant:RetainedCloneGrant)->Result<Option<Self>,ValueError>{
  if original.is_none()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
  let value=original.take().unwrap();let mut owner=Self::new(&value,false);*owner.output=Some(value);owner.phase=1;owner.closing=true;owner.receipt=Some((grant,RetainedCloneProgress{copied_items:1,..Default::default()}));Ok(Some(owner))
 }
 /// 🧵️ Preserves the published text allocation through its paid original receiving handoff.
 pub fn admit_received_text(original:&mut Option<String>,grant:RetainedCloneGrant)->Result<Option<Self>,ValueError>{
  if original.is_none()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
  let value=original.take().unwrap().into_bytes();let mut owner=Self::new(&value,false);*owner.output=Some(value);owner.phase=1;owner.closing=true;owner.receipt=Some((grant,RetainedCloneProgress{copied_items:1,..Default::default()}));Ok(Some(owner))
 }
 /// 📐️ Observes the same original published backing without source access or new work.
 pub fn close_demands(&self)->RetirementDemand{if self.receipt.is_some()||self.phase==3||!self.closing{return Default::default()}RetirementDemand{release_bytes:self.output.as_ref().map_or(0,Vec::capacity),depth:1,..Default::default()}}
 fn validate_source<S:RetainedWireByteSource+?Sized>(&self,source:&S)->Result<(),ValueError>{let identity=source.original_source_identity();if identity.address!=self.original||identity.extent!=self.extent||identity.authority!=self.authority{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original wire source identity or extent changed"))}Ok(())}
 pub fn receipt(&self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt}
 pub fn take_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt.take()}
 pub fn is_ready(&self)->bool{!self.closing&&self.phase==2&&self.receipt.is_none()}
 /// 📐️ Quotes only the original next physical frontier; metadata transfers cost zero payload copy.
 pub fn demands<S:RetainedWireByteSource+?Sized>(&self,source:&S,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
  self.validate_source(source)?;
  if self.receipt.is_some()||self.phase==3{return Ok(Default::default())}
  if self.closing{return Ok(self.close_demands())}
  if self.phase==4{return Ok(RetirementDemand{depth:1,..Default::default()})}
  if self.phase==0{return Ok(RetirementDemand{capacity_bytes:self.output_extent,depth:1,..Default::default()})}
  if self.phase==2{return Ok(RetirementDemand{depth:1,..Default::default()})}
  let start=self.output.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original wire buffer is absent"))?.len();
  let end=start.saturating_add(maximum_copy_bytes.min(4096)).min(self.output_extent);
  Ok(RetirementDemand{copy_bytes:end-start,depth:1,..Default::default()})
 }
 /// 🧵️ Allocates once and copies one actual original prefix per independently supplied turn.
 pub fn advance<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,grant:RetainedCloneGrant)->Result<RetainedWireFieldStep,ValueError>{
  self.validate_source(source)?;
  if self.receipt.is_some()||self.closing||self.phase==3{return Ok(RetainedWireFieldStep::Blocked)}
  if self.phase==2{return Ok(RetainedWireFieldStep::Ready)}
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedWireFieldStep::Blocked)}
  let demand=self.demands(source,grant.maximum_copy_bytes)?;
  if demand.capacity_bytes>grant.maximum_capacity_bytes||demand.copy_bytes>grant.maximum_copy_bytes||demand.release_bytes>grant.maximum_release_bytes{return Ok(RetainedWireFieldStep::Blocked)}
  let progress=if self.phase==4{
   let mut json=match self.source{RetainedWireFieldSource::Json(json)=>json,_=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original JSON scan cursor is absent"))};let done=json.scan_original_piece(source)?;if done{self.output_extent=json.decoded_extent();self.phase=0}self.source=RetainedWireFieldSource::Json(json);RetainedCloneProgress{copied_items:1,..Default::default()}
  }else if self.phase==0{
   let mut output=Vec::new();output.try_reserve_exact(self.output_extent).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original wire field allocation failed"))?;
   let capacity=output.capacity();*self.output=Some(output);self.phase=1;
   RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()}
  }else{
   let start=self.output.as_ref().unwrap().len();let end=start+demand.copy_bytes;
   if start==self.output_extent{if self.text&&!self.utf8.is_complete(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original wire UTF8 is truncated"))}self.phase=2;RetainedCloneProgress{copied_items:1,..Default::default()}}
   else if end==start{return Ok(RetainedWireFieldStep::Blocked)}
   else{
    let mut validation_source=match self.source{RetainedWireFieldSource::Json(json)=>Some(json),_=>None};let mut validated=self.utf8;for position in start..end{let byte=if let Some(json)=validation_source.as_mut(){json.copy_original_byte(source)?}else{self.source_byte(source,position)?};if self.text{validated=validated.consume(&[byte])?}}
    let mut copy_source=match self.source{RetainedWireFieldSource::Json(json)=>Some(json),_=>None};let mut copied_utf8=self.utf8;
    let copy_result=(||->Result<(),ValueError>{for position in start..end{let byte=if let Some(json)=copy_source.as_mut(){json.copy_original_byte(source)?}else{self.source_byte(source,position)?};if self.text{copied_utf8=copied_utf8.consume(&[byte])?}self.output.as_mut().unwrap().push(byte)}Ok(())})();
    if let Some(json)=copy_source{self.source=RetainedWireFieldSource::Json(json)}self.utf8=copied_utf8;
    let copied_bytes=self.output.as_ref().unwrap().len()-start;
    if let Err(error)=copy_result{let progress=RetainedCloneProgress{copied_items:usize::from(copied_bytes!=0),copied_bytes,..Default::default()};self.receipt=Some((grant,progress));return Err(error.with_retained_progress(progress))}
    RetainedCloneProgress{copied_items:1,copied_bytes: end - start,..Default::default()}
   }
  };
  self.receipt=Some((grant,progress));if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original wire allocation exceeded its admitted receipt").with_retained_progress(progress))}
  Ok(if self.phase==2{RetainedWireFieldStep::Ready}else{RetainedWireFieldStep::Progress})
 }
 /// 🫴️ Moves the same native byte backing only into the caller's real empty original recipient.
 pub fn take_into(&mut self,recipient:&mut Option<Vec<u8>>,grant:RetainedCloneGrant)->Result<RetainedWireFieldStep,ValueError>{
  if recipient.is_some()||!self.is_ready()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedWireFieldStep::Blocked)}
  *recipient=self.output.take();self.phase=3;self.receipt=Some((grant,RetainedCloneProgress{copied_items:1,..Default::default()}));Ok(RetainedWireFieldStep::Closed)
 }
 /// 🫴️ Transfers validated original UTF8 into an empty text slot without changing its allocation.
 pub fn take_text_into(&mut self,recipient:&mut Option<String>,grant:RetainedCloneGrant)->Result<RetainedWireFieldStep,ValueError>{
  if !self.text||recipient.is_some()||!self.is_ready()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedWireFieldStep::Blocked)}
  *recipient=Some(unsafe{String::from_utf8_unchecked(self.output.take().unwrap())});self.phase=3;self.receipt=Some((grant,RetainedCloneProgress{copied_items:1,..Default::default()}));Ok(RetainedWireFieldStep::Closed)
 }
 pub fn cancel(&mut self){self.closing=true}
 /// ♻️ Releases only this original field's actual backing after its original physical release preflight.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedWireFieldStep,ValueError>{
  if self.receipt.is_some()||!self.closing{return Ok(RetainedWireFieldStep::Blocked)}
  if self.phase==3{return Ok(RetainedWireFieldStep::Closed)}
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedWireFieldStep::Blocked)}
  let released=self.output.as_ref().map_or(0,Vec::capacity);if released>grant.maximum_release_bytes{return Ok(RetainedWireFieldStep::Blocked)}
  drop(self.output.take());self.phase=3;self.receipt=Some((grant,RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}));Ok(RetainedWireFieldStep::Closed)
 }
 pub fn terminal_is_empty(&self)->bool{self.phase==3&&self.output.is_none()&&self.receipt.is_none()}
}
impl Drop for RetainedWireFieldDecode{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original wire field abandoned receiving custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.output)}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
