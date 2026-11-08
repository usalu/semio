//! 🌱️ Native vacant byte and field plans initialize only their funded metadata part.
use super::*;
use std::mem::{MaybeUninit,size_of};
enum SeedKind<T>{Bytes(unsafe fn(*mut MaybeUninit<T>,usize,usize)),Fields{parts:usize,size:unsafe fn(*mut MaybeUninit<T>,usize)->Option<usize>,write:unsafe fn(*mut MaybeUninit<T>,usize)->bool}}
impl<T> Copy for SeedKind<T>{}
impl<T> Clone for SeedKind<T>{fn clone(&self)->Self{*self}}
/// 🧬️ First-party evidence for initializing vacant metadata in exclusive untyped backing.
pub struct NativeSeedTemplate<T>{kind:SeedKind<T>}
impl<T> Copy for NativeSeedTemplate<T>{}
impl<T> Clone for NativeSeedTemplate<T>{fn clone(&self)->Self{*self}}
impl<T> NativeSeedTemplate<T>{
 /// 🛡️ Creates evidence for one immutable empty byte template.
 /// # Safety
 /// The function must copy the requested in-bounds bytes from the same immutable vacant T on every call, including padding through MaybeUninit bytes. It must not allocate, unwind, call user code, retain the destination or duplicate resources, identities or authority.
 pub const unsafe fn new(copy:unsafe fn(*mut MaybeUninit<T>,usize,usize))->Self{Self{kind:SeedKind::Bytes(copy)}}
 /// 🧩️ Supplies an exact native field plan for metadata that cannot be statically promoted.
 /// # Safety
 /// Each in-range size query must be pure and exact. Each matching write must return true, initialize only that part, allocate nothing, create no resource or authority, call no user code and never unwind. All parts together must initialize a valid vacant T; every incomplete prefix must remain safely releasable without dropping T.
 pub const unsafe fn fields(parts:usize,size:unsafe fn(*mut MaybeUninit<T>,usize)->Option<usize>,write:unsafe fn(*mut MaybeUninit<T>,usize)->bool)->Self{Self{kind:SeedKind::Fields{parts,size,write}}}
 /// 📏️ Observes the actual fixed field plan without invoking any native writer.
 pub const fn field_parts(&self)->Option<usize>{match self.kind{SeedKind::Fields{parts,..}=>Some(parts),SeedKind::Bytes(_)=>None}}
 /// 🧮️ Counts the exact native subparts used when composing a larger vacant field plan.
 pub const fn part_count(&self)->usize{match self.kind{SeedKind::Fields{parts,..}=>parts,SeedKind::Bytes(_)=>{let n=size_of::<T>()/1024+if size_of::<T>()%1024==0{0}else{1};if n==0{1}else{n}}}}
 /// 📐️ Queries one subpart at the original exclusive untyped destination.
 /// # Safety
 /// The destination must provide the correctly aligned storage for T, and its selected prefix must obey this same native template.
 pub unsafe fn part_size(&self,target:*mut MaybeUninit<T>,ordinal:usize)->Option<usize>{if ordinal>=self.part_count(){return None}match self.kind{SeedKind::Fields{size,..}=>unsafe{size(target,ordinal)},SeedKind::Bytes(_)=>{let offset=ordinal.checked_mul(1024)?;Some(size_of::<T>().checked_sub(offset)?.min(1024))}}}
 /// 🪡️ Writes only the corresponding native subpart of a composed vacant plan.
 /// # Safety
 /// The original destination and selected ordinal must match the preceding exact part query; all existing prefix bytes must remain in their original custody.
 pub unsafe fn write_part(&self,target:*mut MaybeUninit<T>,ordinal:usize)->bool{let Some(bytes)=(unsafe{self.part_size(target,ordinal)})else{return false};unsafe{match self.kind{SeedKind::Fields{write,..}=>write(target,ordinal),SeedKind::Bytes(copy)=>{copy(target,ordinal*1024,bytes);true}}}}
 fn same(self,other:Self)->bool{match(self.kind,other.kind){(SeedKind::Bytes(a),SeedKind::Bytes(b))=>std::ptr::fn_addr_eq(a,b),(SeedKind::Fields{parts:a,size:b,write:c},SeedKind::Fields{parts:x,size:y,write:z})=>a==x&&std::ptr::fn_addr_eq(b,y)&&std::ptr::fn_addr_eq(c,z),_=>false}}
 fn part(self,target:*mut MaybeUninit<T>,cursor:usize)->Result<(usize,usize,bool),ValueError>{match self.kind{
  SeedKind::Bytes(_)=>{let remaining=size_of::<T>().checked_sub(cursor).ok_or_else(||invalid("native template prefix exceeds backing"))?;let count=1024.min(remaining);let next=cursor.checked_add(count).ok_or_else(||invalid("native template cursor overflow"))?;Ok((count,next,next==size_of::<T>()))},
  SeedKind::Fields{parts,size,..}=>{if cursor>=parts{return Err(invalid("native field seed index exceeds its plan"))}let bytes=unsafe{size(target,cursor)}.ok_or_else(||invalid("native field seed omitted its actual quote"))?;let next=cursor.checked_add(1).ok_or_else(||invalid("native field seed index overflow"))?;Ok((bytes,next,next==parts))}
 }}
 unsafe fn write(self,target:*mut MaybeUninit<T>,cursor:usize,bytes:usize)->bool{unsafe{match self.kind{SeedKind::Bytes(copy)=>{copy(target,cursor,bytes);true},SeedKind::Fields{write,..}=>write(target,cursor)}}}
}
pub(super) struct SeedState<T>{template:NativeSeedTemplate<T>,pub(super) work:crate::OriginalWorkBindingIdentity,cursor:usize,copied:usize}
impl<T> NativeOwnedStorage<T>{
 /// 📍️ Reports committed vacant metadata bytes; typed access stays unavailable until completion.
 pub fn seeded_bytes(&self)->usize{if self.initialized{size_of::<T>()}else{self.seed.as_ref().map_or(0,|seed|seed.copied)}}
 /// 📐️ Quotes the exact next native part and its resident custody metadata.
 pub fn template_seed_demand(&self,template:NativeSeedTemplate<T>)->Result<RetirementDemand,ValueError>{
  if self.pointer.is_none()||self.initialized{return Err(invalid("native template requires its exclusive uninitialized backing"))}if self.seed.as_ref().is_some_and(|seed|!seed.template.same(template)){return Err(invalid("native template changed during partial initialization"))}
  let cursor=self.seed.as_ref().map_or(0,|seed|seed.cursor);let target=self.pointer.ok_or_else(||invalid("native template backing absent"))?.as_ptr().cast::<MaybeUninit<T>>();let(bytes,_,complete)=template.part(target,cursor)?;let metadata=if self.seed.is_none()&&complete{size_of::<bool>()}else{size_of::<Option<SeedState<T>>>()+usize::from(complete)*size_of::<bool>()};
  Ok(RetirementDemand{copy_bytes:bytes.checked_add(metadata).ok_or_else(||invalid("native template quote overflow"))?,depth:1,..Default::default()})
 }
 /// 🪡️ Initializes one native part without exposing a partially valid typed owner.
 pub fn initialize_template(&mut self,template:NativeSeedTemplate<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  let before=effect.performed_boundary();let result=(||{
   if self.seed.as_ref().is_some_and(|seed|!seed.work.matches_effect(effect)){return Err(invalid("native partial template changed its original Work"))}let demand=self.template_seed_demand(template)?;if !admitted(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(RetainedCloneStep::Progress(Default::default()))}
   let cursor=self.seed.as_ref().map_or(0,|seed|seed.cursor);let target=self.pointer.ok_or_else(||invalid("native template backing disappeared"))?.as_ptr().cast::<MaybeUninit<T>>();let(bytes,next,complete)=template.part(target,cursor)?;let copied=self.seed.as_ref().map_or(0,|seed|seed.copied).checked_add(bytes).ok_or_else(||invalid("native field seed copy extent overflow"))?;if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native template exceeded its original Work"))}
   let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};if !unsafe{template.write(target,cursor,bytes)}{return Err(invalid("native field seed violated its quoted plan").with_retained_progress(progress))}
   if complete{self.seed=None;self.initialized=true}else{self.seed=Some(SeedState{template,work:crate::OriginalWorkBindingIdentity::from_effect(effect),cursor:next,copied})}Ok(if complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
  })();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same native template Work receipt")}
 }
}
