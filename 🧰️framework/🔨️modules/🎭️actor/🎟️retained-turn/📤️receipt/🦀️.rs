//! 🧾️ Data-only actor scope, return-escrow and body receipt codecs.
use semio_framework_actor::RetainedTurnInput;
use semio_framework_actor::retained_context::{OriginalActorAdmissionReply,OriginalActorCloseReply};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
pub use semio_framework_job_return_escrow_publication::OriginalEscrowHeaderWrite;
#[path="💼️job/🦀️.rs"]mod job;
pub use job::*;
#[path="⚠️failure/🦀️.rs"]mod actor_failure;
pub use actor_failure::*;
#[path="🌱️bootstrap/🦀️.rs"]mod actor_bootstrap;
pub use actor_bootstrap::{OriginalActorBootstrapFailure,OriginalBootstrapStep,OriginalActorBootstrapReceipt,OriginalActorBootstrapCall,OriginalActorBootstrapStatus,OriginalBootstrapEffects,OriginalBootstrapTransitionData,OriginalActorBootstrapTransitionProof,OriginalActorBootstrapTransitionCall};
pub struct OriginalSliceHeaderWriter<'a>{bytes:&'a mut[u8],written:usize}
impl<'a> OriginalSliceHeaderWriter<'a>{pub fn new(bytes:&'a mut[u8],length:usize)->Result<Self,ValueError>{if bytes.len()!=length{return Err(invalid())}Ok(Self{bytes,written:0})}}
impl OriginalEscrowHeaderWrite for OriginalSliceHeaderWriter<'_>{fn write(&mut self,bytes:&[u8])->Result<(),ValueError>{let end=self.written.checked_add(bytes.len()).filter(|end|*end<=self.bytes.len()).ok_or_else(invalid)?;self.bytes[self.written..end].copy_from_slice(bytes);self.written=end;Ok(())}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalActorFixedKind{InputFailure{input_returned:bool},QuiescentForCoreRetirement,Admission{ready:bool,refusal:Option<ValueRefusalKind>},InputAdmission{ready:bool,input_returned:bool,refusal:Option<ValueRefusalKind>},Close{complete:bool,blocked:bool,refusal:Option<ValueRefusalKind>},Failure{authority:Option<ValueRefusalKind>},TurnOutput{ready:bool,captured:bool,input_pending:bool,pointer:u32,length:u32}}
impl OriginalActorFixedKind{pub fn is_failure_for(self,kind:semio_framework_actor::ActorFailureCallKind)->bool{match self{Self::Failure{..}=>kind.source_scoped(),Self::InputFailure{..}=>kind==semio_framework_actor::ActorFailureCallKind::AdmitActorInput,_=>false}}pub fn input_disposition(self)->Option<(bool,bool,bool)>{match self{Self::InputAdmission{ready,input_returned,..}=>Some((ready,input_returned,false)),Self::InputFailure{input_returned}=>Some((false,input_returned,true)),_=>None}}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalActorFixedCall{pub input:RetainedTurnInput,pub spent:RetainedCloneProgress,pub work_units_spent:u64,pub kind:OriginalActorFixedKind}
/// 🪙️ The actual guest Source lends this immutable escrow snapshot with its fixed header.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalEscrowExtent{pub pointer:u32,pub length:u32}
/// 🧾️ Untrusted scalar values from one Source return-credit snapshot.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalEscrowRow{pub source_address:u64,pub source_birth_generation:u64,pub recipient_address:u64,pub destination_address:u64,pub destination_extent:u64,pub serial:u64,pub origin_epoch:u64,pub status:u64,pub held_items:u64,pub held_copy:u64,pub held_work:u64,pub held_release:u64,pub held_capacity:u64,pub redeemed_items:u64,pub redeemed_copy:u64,pub redeemed_work:u64,pub redeemed_release:u64,pub redeemed_capacity:u64,pub released_items:u64,pub released_copy:u64,pub released_work:u64,pub released_release:u64,pub released_capacity:u64}
/// 📋️ A parsed snapshot is untrusted until the actual producer and issuer validate its loan.
#[derive(Debug,PartialEq,Eq)]
pub struct OriginalEscrowSnapshot{pub source_address:u64,pub source_birth_generation:u64,pub recipient_address:u64,pub revision:u64,pub acknowledged_revision:u64,rows:[Option<OriginalEscrowRow>;32],count:usize}
impl OriginalEscrowSnapshot{
 pub fn rows(&self)->impl Iterator<Item=&OriginalEscrowRow>{self.rows[..self.count].iter().flatten()}
 pub fn len(&self)->usize{self.count}
 pub fn is_empty(&self)->bool{self.count==0}
 pub fn row_at(&self,index:usize)->Option<&OriginalEscrowRow>{if index>=self.count{return None}self.rows[index].as_ref()}
 pub fn read(bytes:&[u8])->Result<Self,ValueError>{
  if bytes.len()<48||&bytes[..4]!=b"SRE1"{return Err(invalid())}let count=u32::from_le_bytes(bytes[4..8].try_into().unwrap())as usize;if count>32||bytes.len()!=48+count*184{return Err(invalid())}let word=|offset:usize|u64::from_le_bytes(bytes[offset..offset+8].try_into().unwrap());let source_address=word(8);let source_birth_generation=word(16);let recipient_address=word(24);let revision=word(32);let acknowledged_revision=word(40);if source_address==0||source_birth_generation==0||recipient_address==0||revision==0||acknowledged_revision>=revision{return Err(invalid())}let mut rows=[None;32];let mut previous=None;
  for(index,slot)in rows[..count].iter_mut().enumerate(){let start=48+index*184;let mut v=[0u64;23];for(i,value)in v.iter_mut().enumerate(){*value=u64::from_le_bytes(bytes[start+i*8..start+i*8+8].try_into().unwrap())}let row=OriginalEscrowRow{source_address:v[0],source_birth_generation:v[1],recipient_address:v[2],destination_address:v[3],destination_extent:v[4],serial:v[5],origin_epoch:v[6],status:v[7],held_items:v[8],held_copy:v[9],held_work:v[10],held_release:v[11],held_capacity:v[12],redeemed_items:v[13],redeemed_copy:v[14],redeemed_work:v[15],redeemed_release:v[16],redeemed_capacity:v[17],released_items:v[18],released_copy:v[19],released_work:v[20],released_release:v[21],released_capacity:v[22]};let key=(row.source_address,row.source_birth_generation,row.recipient_address,row.serial);if (row.source_address,row.source_birth_generation,row.recipient_address)!=(source_address,source_birth_generation,recipient_address)||row.destination_address==0||row.serial==0||row.origin_epoch==0||!matches!(row.status,1|2)||previous.is_some_and(|old|old>=key)||row.redeemed_items.checked_add(row.released_items).is_none_or(|used|used>row.held_items||row.status==2&&used!=row.held_items)||row.redeemed_copy.checked_add(row.released_copy).is_none_or(|used|used>row.held_copy||row.status==2&&used!=row.held_copy)||row.redeemed_work.checked_add(row.released_work).is_none_or(|used|used>row.held_work||row.status==2&&used!=row.held_work)||row.redeemed_release.checked_add(row.released_release).is_none_or(|used|used>row.held_release||row.status==2&&used!=row.held_release)||row.redeemed_capacity.checked_add(row.released_capacity).is_none_or(|used|used>row.held_capacity||row.status==2&&used!=row.held_capacity){return Err(invalid())}previous=Some(key);*slot=Some(row)}Ok(Self{source_address,source_birth_generation,recipient_address,revision,acknowledged_revision,rows,count})
 }
}
/// 🧱️ Caller receipt storage is inline and reusable only after the original returned view is released.
pub struct OriginalActorFixedReceipt{bytes:[u8;144],borrowed:bool}
impl OriginalActorFixedReceipt{
 pub const fn empty()->Self{Self{bytes:[0;144],borrowed:false}}
 pub fn valid_escrow(escrow:OriginalEscrowExtent)->bool{(escrow.pointer==0&&escrow.length==0)||(escrow.pointer!=0&&(48..=5936).contains(&escrow.length)&&(escrow.length-48)%184==0)}
 pub fn is_borrowed(&self)->bool{self.borrowed}
 pub fn produce(&mut self,escrow:OriginalEscrowExtent,body:impl FnOnce()->OriginalActorFixedCall)->Result<&[u8;144],ValueError>{if self.borrowed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original actor receipt remains borrowed"))}self.write(body(),escrow)?;self.borrowed=true;Ok(&self.bytes)}
 pub fn owns_pointer(&self,pointer:usize)->bool{let start=self.bytes.as_ptr()as usize;pointer>=start&&pointer<start+144}
 pub fn hand_back(&mut self,pointer:usize,length:usize)->bool{if !self.borrowed||pointer!=self.bytes.as_ptr()as usize||length!=144{return false}self.borrowed=false;true}
 pub fn bytes(&self)->&[u8;144]{&self.bytes}
 pub fn write_admission(&mut self,original:&OriginalActorAdmissionReply,escrow:OriginalEscrowExtent)->Result<(),ValueError>{self.write(OriginalActorFixedCall{input:original.input,spent:original.spent,work_units_spent:original.work_units_spent,kind:OriginalActorFixedKind::Admission{ready:original.ready,refusal:original.refusal}},escrow)}
 pub fn write_close(&mut self,original:&OriginalActorCloseReply,escrow:OriginalEscrowExtent)->Result<(),ValueError>{self.write(OriginalActorFixedCall{input:original.input,spent:original.spent,work_units_spent:original.work_units_spent,kind:OriginalActorFixedKind::Close{complete:original.complete,blocked:original.blocked,refusal:original.refusal}},escrow)}
 /// 🧾️ Raw actual spend is preserved even when it cannot conserve the supplied grant.
 pub fn write(&mut self,original:OriginalActorFixedCall,escrow:OriginalEscrowExtent)->Result<(),ValueError>{
  assert!(!self.borrowed,"original actor receipt cannot overwrite its outstanding loan");
  assert!(!matches!(original.kind,OriginalActorFixedKind::TurnOutput{..}),"turn output requires its own receipt");
  Self::write_into(&mut self.bytes,original,escrow)
 }
 pub fn write_into(bytes:&mut[u8],original:OriginalActorFixedCall,escrow:OriginalEscrowExtent)->Result<(),ValueError>{let mut sink=OriginalSliceHeaderWriter::new(bytes,144)?;Self::write_to(&mut sink,original,escrow)}
 pub fn write_to(sink:&mut impl OriginalEscrowHeaderWrite,original:OriginalActorFixedCall,escrow:OriginalEscrowExtent)->Result<(),ValueError>{
  if !Self::valid_escrow(escrow){return Err(invalid())}
  let(kind,flags,refusal)=match original.kind{OriginalActorFixedKind::InputFailure{input_returned}=>(7,u8::from(input_returned)<<1,None),OriginalActorFixedKind::QuiescentForCoreRetirement=>(6,0,None),OriginalActorFixedKind::Admission{ready,refusal}=>(1,u8::from(ready),refusal),OriginalActorFixedKind::InputAdmission{ready,input_returned,refusal}=>{assert!(!ready||input_returned,"ready input requires actual returned wire proof");(4,u8::from(ready)|(u8::from(input_returned)<<1),refusal)},OriginalActorFixedKind::Close{complete,blocked,refusal}=>(2,u8::from(complete)|(u8::from(blocked)<<1),refusal),OriginalActorFixedKind::Failure{authority}=>(3,0,authority),OriginalActorFixedKind::TurnOutput{ready,captured,input_pending,..}=>(5,u8::from(ready)|(u8::from(captured)<<1)|(u8::from(input_pending)<<2),None)};sink.write(b"SAC3")?;sink.write(&[kind,flags,code(refusal),0])?;
  let input=original.input;let grant=input.grant;let spent=original.spent;macro_rules! word{($v:expr)=>{{sink.write(&($v as u64).to_le_bytes())?;}}}word!(input.operation);word!(input.generation);word!(input.epoch);word!(input.work.clock_binding);word!(input.work.absolute_deadline_us);word!(input.work.fuel);word!(grant.maximum_items);word!(grant.maximum_copy_bytes);word!(grant.maximum_capacity_bytes);word!(grant.maximum_release_bytes);word!(grant.maximum_depth);word!(spent.copied_items);word!(spent.copied_bytes);word!(spent.retained_capacity_bytes);word!(spent.released_bytes);word!(original.work_units_spent);
  sink.write(&escrow.pointer.to_le_bytes())?;sink.write(&escrow.length.to_le_bytes())?;Ok(())
 }
 pub fn write_failure_to(sink:&mut impl OriginalEscrowHeaderWrite,call:OriginalActorFixedCall,escrow:OriginalEscrowExtent,length:usize)->Result<(),ValueError>{if !matches!(call.kind,OriginalActorFixedKind::Failure{..})||!matches!(length,144|152){return Err(invalid())}Self::write_to(sink,call,escrow)?;if length==152{sink.write(&[0;8])?}Ok(())}
 pub fn read(bytes:&[u8])->Result<(OriginalActorFixedCall,OriginalEscrowExtent),ValueError>{
  if !matches!(bytes.len(),144|152)||&bytes[..4]!=b"SAC3"||bytes[7]!=0{return Err(invalid())}let refusal=kind(bytes[6])?;let flags=bytes[5];let kind=match bytes[4]{7 if bytes.len()==144&&matches!(flags,0|2)&&refusal.is_none()=>OriginalActorFixedKind::InputFailure{input_returned:flags==2},6 if bytes.len()==144&&flags==0&&refusal.is_none()=>OriginalActorFixedKind::QuiescentForCoreRetirement,1 if bytes.len()==144&&flags<=1=>OriginalActorFixedKind::Admission{ready:flags==1,refusal},2 if bytes.len()==144&&flags<=2=>OriginalActorFixedKind::Close{complete:flags&1!=0,blocked:flags&2!=0,refusal},3 if matches!(bytes.len(),144|152)&&flags==0&&(bytes.len()==144||bytes[144..].iter().all(|v|*v==0))=>OriginalActorFixedKind::Failure{authority:refusal},4 if bytes.len()==144&&matches!(flags,0|2|3)=>OriginalActorFixedKind::InputAdmission{ready:flags&1!=0,input_returned:flags&2!=0,refusal},5 if bytes.len()==152&&matches!(flags,0|2|3|4|6|7)&&refusal.is_none()=>{let pointer=u32::from_le_bytes(bytes[144..148].try_into().unwrap());let length=u32::from_le_bytes(bytes[148..152].try_into().unwrap());if (flags&1==0&&(pointer!=0||length!=0))||(flags&1!=0&&(pointer==0||length==0)){return Err(invalid())}OriginalActorFixedKind::TurnOutput{ready:flags&1!=0,captured:flags&2!=0,input_pending:flags&4!=0,pointer,length}},_=>return Err(invalid())};let escrow=OriginalEscrowExtent{pointer:u32::from_le_bytes(bytes[136..140].try_into().unwrap()),length:u32::from_le_bytes(bytes[140..144].try_into().unwrap())};if (escrow.pointer==0&&escrow.length!=0)||(escrow.pointer!=0&&(!(48..=4400).contains(&escrow.length)||(escrow.length-48)%136!=0)){return Err(invalid())}let mut values=[0u64;16];for(index,value)in values.iter_mut().enumerate(){*value=u64::from_le_bytes(bytes[8+index*8..16+index*8].try_into().unwrap());}let extent=|index:usize|usize::try_from(values[index]).map_err(|_|invalid());Ok((OriginalActorFixedCall{input:RetainedTurnInput{operation:values[0],generation:values[1],epoch:values[2],work:semio_framework_actor::OriginalActorTurnWork{clock_binding:values[3],absolute_deadline_us:values[4],fuel:values[5]},grant:RetainedCloneGrant{maximum_items:extent(6)?,maximum_copy_bytes:extent(7)?,maximum_capacity_bytes:extent(8)?,maximum_release_bytes:extent(9)?,maximum_depth:extent(10)?}},spent:RetainedCloneProgress{copied_items:extent(11)?,copied_bytes:extent(12)?,retained_capacity_bytes:extent(13)?,released_bytes:extent(14)?},work_units_spent:values[15],kind},escrow))
 }
}
/// 👁️ Parses an untrusted header word and the escrow extent within one immutable memory view.
pub fn borrowed_scalar_escrow(memory:&[u8],word:u64)->Result<(&[u8],&[u8]),ValueError>{
 let pointer=word as u32;let length=(word>>32)as u32;if pointer==0||!matches!(length,144|152|184|200){return Err(invalid())}let span=|p:u32,n:u32|{let start=p as usize;start.checked_add(n as usize).filter(|end|*end<=memory.len()).map(|end|&memory[start..end]).ok_or_else(invalid)};let raw=span(pointer,length)?;let offset=if raw.len()==200&&raw[..8]==[83,66,84,49,1,0,200,0]&&&raw[8..12]==b"SAC3"{OriginalActorBootstrapTransitionCall::read(raw)?;144}else if matches!(raw.len(),144|152)&&&raw[..4]==b"SAC3"{136}else if raw.len()==184&&matches!(&raw[..4],b"SJH2"|b"SAF1")&&&raw[8..12]==b"SAC3"{144}else{return Err(invalid())};let p=u32::from_le_bytes(raw[offset..offset+4].try_into().unwrap());let n=u32::from_le_bytes(raw[offset+4..offset+8].try_into().unwrap());if p==0||!(48..=4400).contains(&n)||(n-48)%136!=0{return Err(invalid())}Ok((raw,span(p,n)?))
}
/// 📦️ The output header lends the actual capsule extent until its fixed receipt returns.
pub struct OriginalActorTurnOutputReceipt{bytes:[u8;152],borrowed:bool}
impl OriginalActorTurnOutputReceipt{
 pub const fn empty()->Self{Self{bytes:[0;152],borrowed:false}}
 pub fn is_borrowed(&self)->bool{self.borrowed}
 pub fn produce(&mut self,original:OriginalActorFixedCall,escrow:OriginalEscrowExtent)->Result<&[u8;152],ValueError>{
  if self.borrowed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"turn output receipt remains borrowed"))}
  Self::write_into(&mut self.bytes,original,escrow)?;self.borrowed=true;Ok(&self.bytes)
 }
 pub fn write_into(bytes:&mut[u8],original:OriginalActorFixedCall,escrow:OriginalEscrowExtent)->Result<(),ValueError>{let mut sink=OriginalSliceHeaderWriter::new(bytes,152)?;Self::write_to(&mut sink,original,escrow)}
 pub fn write_to(sink:&mut impl OriginalEscrowHeaderWrite,original:OriginalActorFixedCall,escrow:OriginalEscrowExtent)->Result<(),ValueError>{
  let OriginalActorFixedKind::TurnOutput{ready,captured,pointer,length,..}=original.kind else{return Err(invalid())};
  if (ready&&!captured)||(!ready&&(pointer!=0||length!=0))||(ready&&(pointer==0||length==0)){return Err(invalid())}
  OriginalActorFixedReceipt::write_to(sink,original,escrow)?;sink.write(&pointer.to_le_bytes())?;sink.write(&length.to_le_bytes())?;Ok(())
 }
 pub fn owns_pointer(&self,pointer:usize)->bool{let start=self.bytes.as_ptr()as usize;pointer>=start&&pointer<start+152}
 pub fn hand_back(&mut self,pointer:usize,length:usize)->bool{if !self.borrowed||pointer!=self.bytes.as_ptr()as usize||length!=152{return false}self.borrowed=false;true}
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original fixed actor receipt has an invalid scalar header or native extent")}
fn code(kind:Option<ValueRefusalKind>)->u8{match kind{None=>0,Some(ValueRefusalKind::InvalidValue)=>1,Some(ValueRefusalKind::Canceled)=>2,Some(ValueRefusalKind::OwnershipLimit)=>3,Some(ValueRefusalKind::AllocationFailed)=>4,Some(ValueRefusalKind::WorkLimit)=>5,Some(ValueRefusalKind::DepthLimit)=>6,Some(ValueRefusalKind::UnsupportedOwner)=>7,Some(ValueRefusalKind::InvariantViolated)=>8}}
fn kind(code:u8)->Result<Option<ValueRefusalKind>,ValueError>{Ok(match code{0=>None,1=>Some(ValueRefusalKind::InvalidValue),2=>Some(ValueRefusalKind::Canceled),3=>Some(ValueRefusalKind::OwnershipLimit),4=>Some(ValueRefusalKind::AllocationFailed),5=>Some(ValueRefusalKind::WorkLimit),6=>Some(ValueRefusalKind::DepthLimit),7=>Some(ValueRefusalKind::UnsupportedOwner),8=>Some(ValueRefusalKind::InvariantViolated),_=>return Err(invalid())})}


/// 🧭️ Cheap data-only prefix grammar leaves caller, Source and physical-token qualification to their actual owners.
pub fn validate_original_source_header_prefix(bytes:&[u8])->Result<(),ValueError>{if bytes.len()<12{return Err(invalid())}if &bytes[..4]==b"SBT1"{if bytes.len()<16||bytes[..8]!=[83,66,84,49,1,0,200,0]||&bytes[8..12]!=b"SAC3"||!matches!((bytes[12],bytes[13]),(1,0|1)|(3,0)){return Err(invalid())}if bytes[15]!=0{return Err(invalid())}kind(bytes[14])?;return Ok(())}if &bytes[..4]==b"SAC3"{let flags=bytes[5];let authority=kind(bytes[6])?;if bytes[7]!=0||!match bytes[4]{1=>flags<=1,2=>flags<=2,3=>flags==0,4=>matches!(flags,0|2|3),5=>matches!(flags,0|2|3|4|6|7)&&authority.is_none(),6=>flags==0&&authority.is_none(),7=>matches!(flags,0|2)&&authority.is_none(),_=>false}{return Err(invalid())}return Ok(())}if bytes[7]!=0||bytes[6]>2||&bytes[8..12]!=b"SAC3"{return Err(invalid())}let flags=bytes[5];let valid=match &bytes[..4]{b"SJH2"=>match bytes[4]{1|2|4=>flags<=3,3=>flags<=15,5|6=>flags<=1,_=>false},b"SAF1"=>match bytes[4]{0=>flags==0&&bytes[6]==2,1|3|4=>flags<=1,2=>matches!(flags,0|1|3|5|7),_=>false},_=>false};if valid{Ok(())}else{Err(invalid())}}
/// 📋️ Full immutable data parsing never authenticates a producer or replaces the actual final controlled decoder.
pub fn validate_original_source_header_body(bytes:&[u8])->Result<(),ValueError>{validate_original_source_header_prefix(bytes)?;match bytes.get(..4){Some(b"SBT1")=>OriginalActorBootstrapTransitionCall::read(bytes).map(|_|()),Some(b"SAC3")=>OriginalActorFixedReceipt::read(bytes).map(|_|()),Some(b"SJH2")=>OriginalOwnedJobFixedReceipt::read(bytes).map(|_|()),Some(b"SAF1")=>ActorFailureFixedReceipt::read(bytes).map(|_|()),_=>Err(invalid())}}





#[cfg(test)]
mod release_escrow_codec{
 use super::*;
 #[test]
 fn release_currency_is_bounded_by_the_same_source_row(){
  let mut bytes=[0u8;232];bytes[..4].copy_from_slice(b"SRE1");bytes[4..8].copy_from_slice(&1u32.to_le_bytes());
  for(i,v)in [1u64,2,3,1,0].into_iter().enumerate(){bytes[8+i*8..16+i*8].copy_from_slice(&v.to_le_bytes())}
  let words=[1u64,2,3,4,0,1,1,1,0,0,0,12,64,0,0,0,7,16,0,0,0,5,48];
  for(i,v)in words.into_iter().enumerate(){bytes[48+i*8..56+i*8].copy_from_slice(&v.to_le_bytes())}
  let row=OriginalEscrowSnapshot::read(&bytes).unwrap();assert_eq!(row.row_at(0).unwrap().held_release,12);assert_eq!(row.row_at(0).unwrap().redeemed_release,7);assert_eq!(row.row_at(0).unwrap().released_release,5);
  bytes[48+7*8..56+7*8].copy_from_slice(&2u64.to_le_bytes());assert!(OriginalEscrowSnapshot::read(&bytes).is_ok());
  bytes[48+21*8..56+21*8].copy_from_slice(&6u64.to_le_bytes());assert!(OriginalEscrowSnapshot::read(&bytes).is_err());
  bytes[48+21*8..56+21*8].copy_from_slice(&4u64.to_le_bytes());assert!(OriginalEscrowSnapshot::read(&bytes).is_err());
  bytes[48+21*8..56+21*8].copy_from_slice(&5u64.to_le_bytes());bytes[48+22*8..56+22*8].copy_from_slice(&49u64.to_le_bytes());assert!(OriginalEscrowSnapshot::read(&bytes).is_err());
 }
}
