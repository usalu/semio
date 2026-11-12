//! 📋️ Original membership changes retain the actual row, selected item and rank while rebuilding bounded vector fragments.
use crate::{snapshot::{original::EnergyModelSnapshot,state::fields::{EntityRow,RetainedEntityLookup,RetainedBranchProjection}},diff::original::EnergyModelDiff,row::{RowBranchPath,RowTail,tail_step,RowOutcome},reference::{ReferenceCollection,ReferenceLookup},leaf::RowFieldPreparation,custody::{self,Cursor,Held,Bound},scalar::{NativeScalar,Quote,birth,invalid,scalar,nested,child,add,receipt,bind}};
use semio_framework_value::{RetirementDemand,RetirementPollExit,RetainedAllocationEffect,ValueError,RetainedOwnedProjection,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneRef,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_progress}};
use semio_framework_value_snapshot_projection::snapshot_projection::SnapshotProjection;
use protocol::retained_mutation_demand_fits;
use crate::retained::{Done,Flight,Reply,conclude};
use std::mem::size_of;
#[path="🧺️storage/🦀️.rs"]mod storage;pub use storage::MembershipValues;
type Builder<S>=<<S as MembershipPath>::Values as MembershipValues<<S as MembershipPath>::Item>>::Builder;
/// 🧭️ Each authored membership policy names its original row field, member, rank and foreign collection.
pub trait MembershipPath:RowBranchPath where Self::Row:EntityRow+SnapshotProjection<Output=Box<Self::Row>>{
 type Payload:Send+Sync+'static;type Item:NativeScalar+'static;type Values:MembershipValues<Self::Item>;
 const INSERT:bool;const MOVE:bool=false;const REPLACE:bool=false;const INVALID_RANK:RowOutcome=RowOutcome::Missing;const PRECHECK_INDEX:bool=false;const BY_INDEX:bool;const SORTED:bool;const RANK_FIRST:bool;const DUPLICATE:Option<RowOutcome>;
 fn key(p:RetainedCloneRef<'_,Self::Payload>)->RetainedCloneRef<'_,u32>;
 fn member(p:&Self::Payload)->Option<Self::Item>;
 fn index(p:&Self::Payload)->Option<usize>;
 fn destination(_: &Self::Payload)->Option<usize>{None}
 fn item_key(v:Self::Item)->u32;
 fn valid_member(_:Self::Item)->bool{true}
 fn reference()->Option<ReferenceCollection>{None}
 fn reference_key(_:RetainedCloneRef<'_,Self::Payload>)->Result<RetainedCloneRef<'_,u32>,ValueError>{Err(invalid())}
 fn read(r:&Self::Row)->&SealedShared<Self::Values>;
 fn select(r:&mut Self::Row)->&mut SealedShared<Self::Values>;
}
/// ↩️ The original selected item and ordinal survive candidate publication for the exact opposite semantic intent.
#[derive(Clone,Copy)]pub struct MembershipInverse<I:Copy>{pub index:u32,pub item:I}
pub struct MembershipPreparation<S:MembershipPath> where S::Row:EntityRow+SnapshotProjection<Output=Box<S::Row>>{
 original:Bound,input:Bound,body:Option<usize>,lookup:Option<Box<RetainedEntityLookup<S::Row>>>,row:Option<Box<RetainedOwnedProjection<S::Row>>>,reference:Option<Box<ReferenceLookup>>,tail:Option<Box<RowTail<S>>>,
 values:Held<Builder<S>>,raw:Held<S::Values>,replacement:Held<SealedShared<S::Values>>,displaced:Held<SealedShared<S::Values>>,
 rank:Option<usize>,scan:usize,found:Option<usize>,ordered:Option<usize>,selected:Option<MembershipInverse<S::Item>>,copied:usize,outcome:Option<RowOutcome>,source_target:bool,phase:u8,closing:bool,flight:Flight,
}
impl<S:MembershipPath> MembershipPreparation<S> where S::Row:EntityRow+SnapshotProjection<Output=Box<S::Row>>{
 fn old(&self)->Result<&S::Values,ValueError>{Ok(S::read(self.row.as_ref().ok_or_else(invalid)?.borrow()?.get()).get())}
 pub fn source_target(&self)->bool{self.source_target}
 fn fail(&mut self,outcome:RowOutcome,source:bool){self.outcome=Some(outcome);self.source_target=source;self.phase=99;}
 fn count(&self)->Result<usize,ValueError>{if S::MOVE||S::REPLACE{Ok(self.old()?.len())}else if S::INSERT{self.old()?.len().checked_add(1).ok_or_else(invalid)}else{self.old()?.len().checked_sub(1).ok_or_else(invalid)}}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 custody_impl!(tail,reference,lookup,row,values,raw,replacement,displaced,original,input);
}
crate::custody::std_cursor!([S:MembershipPath] MembershipPreparation<S>,[S::Row:EntityRow+SnapshotProjection<Output=Box<S::Row>>],|s|s.begin_close());
impl<S:MembershipPath> RowFieldPreparation for MembershipPreparation<S> where S::Row:EntityRow+SnapshotProjection<Output=Box<S::Row>>{
 type Input=S::Payload;type Inverse=MembershipInverse<S::Item>;
 fn new()->Self{Self{original:Default::default(),input:Default::default(),body:None,lookup:None,row:None,reference:None,tail:None,values:Default::default(),raw:Default::default(),replacement:Default::default(),displaced:Default::default(),rank:None,scan:0,found:None,ordered:None,selected:None,copied:0,outcome:None,source_target:false,phase:0,closing:false,flight:Default::default()}}
 fn check(&self,p:RetainedCloneRef<'_,Self::Input>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{p.check_binding(&self.input.0)?;o.check_binding(&self.original.0)?;Ok(())}
 fn quote(&self,p:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64)->Result<Quote,ValueError>{if self.closing||self.phase>=99||self.body.is_some_and(|b|b!=body)||!self.flight.idle(){return Err(invalid());}self.check(p,k,o)?;
  let lookup=||self.lookup.as_ref().ok_or_else(invalid);let reference=||self.reference.as_ref().ok_or_else(invalid);
  match self.phase{
   0=>Ok((nested(scalar(o.binding_copy_bytes()+size_of::<usize>()),1)?,o.binding_work_demand(&self.original.0)?)),
   1=>Ok((nested(scalar(p.binding_copy_bytes()),1)?,p.binding_work_demand(&self.input.0)?)),
   2=>Ok((birth::<RetainedEntityLookup<S::Row>>(),1)),
   3=>Ok((nested(lookup()?.demand(S::rows(o),S::key(p))?,1)?,lookup()?.advance_work_demand(S::rows(o),S::key(p))?)),
   4=>if lookup()?.result()==Some(None){Ok((scalar(3),1))}else{let mut d=nested(lookup()?.take_demand()?,2*size_of::<RetainedOwnedProjection<S::Row>>()+4*size_of::<usize>()+1)?;d.capacity_bytes+=size_of::<RetainedOwnedProjection<S::Row>>();Ok((d,1))},
   5=>Ok((Cursor::demand(&self.lookup,body)?,Cursor::work(&self.lookup,body)?)),
   6|9|10=>Ok((scalar(8*size_of::<usize>()+4*size_of::<S::Item>()+8),1)),
   7=>Ok((birth::<ReferenceLookup>(),1)),
   8=>{let(d,w)=reference()?.quote(o,S::reference_key(p)?)?;Ok((nested(d,1)?,w))},
   11=>Ok((Cursor::demand(&self.reference,body)?,Cursor::work(&self.reference,body)?)),
   12=>Ok((nested(S::Values::birth_demand(self.count()?)?,2*size_of::<Option<Builder<S>>>()+1)?,1)),
   13=>Ok((scalar(4*size_of::<S::Item>()+4*size_of::<usize>()+1),1)),
   14=>Ok((RetirementDemand{copy_bytes:2*size_of::<Builder<S>>()+2*size_of::<S::Values>()+1,depth:1,..Default::default()},1)),
   31=>{let mut d=SealedShared::<S::Values>::source_admission_demand()?;d.copy_bytes=d.copy_bytes.checked_add(2*size_of::<Option<S::Values>>()+1).ok_or_else(invalid)?;Ok((d,SealedShared::<S::Values>::source_admission_work_demand()))},
   15=>Ok((scalar(1),1)),
   30=>self.tail.as_ref().ok_or_else(invalid)?.quote::<SealedShared<S::Values>>(self.row.as_ref().ok_or_else(invalid)?.borrow()?,o,body,max),
   _=>Err(invalid())}}
 fn advance_reply<'r>(&mut self,payload:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote(payload,k,o,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   let mut p=RetainedCloneProgress::default();
   match self.phase{
 0=>{p=add(bind(o,&mut self.original,&mut effect.nested(child(g,1+size_of::<usize>())),&mut work)?,1+size_of::<usize>())?;self.body=Some(body);self.phase=1;},
 1=>{p=add(bind(payload,&mut self.input,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=2;},
 2=>{self.lookup=Some(Box::new(RetainedEntityLookup::new()));self.phase=3;p=receipt(birth::<RetainedEntityLookup<S::Row>>());},
 3=>{let c=self.lookup.as_mut().ok_or_else(invalid)?;p=add(c.advance(S::rows(o),S::key(payload),&mut effect.nested(child(g,1)))?.progress(),1)?;if c.result().is_some(){self.phase=4;}},
 4=>{let c=self.lookup.as_mut().ok_or_else(invalid)?;if c.result()==Some(None){self.fail(RowOutcome::Missing,false);p=receipt(scalar(3));}else{let extra=2*size_of::<RetainedOwnedProjection<S::Row>>()+4*size_of::<usize>()+1;let bytes=size_of::<RetainedOwnedProjection<S::Row>>();let mut cg=child(g,extra);cg.maximum_capacity_bytes=cg.maximum_capacity_bytes.saturating_sub(bytes);let(row,r)=c.take(&mut effect.nested(cg))?.ok_or_else(invalid)?;self.rank=c.result().ok_or_else(invalid)?;self.row=Some(Box::new(row));c.begin_close();p=add(r,extra)?;p.retained_capacity_bytes+=bytes;self.phase=5;}},
 5=>{let step=custody::finish(&mut self.flight,&mut self.lookup,body,effect,&mut work,recover)?;p=step.progress();if self.lookup.is_none(){self.phase=6;}},
 6=>{p=receipt(scalar(1));let len=self.old()?.len();let invalid_rank=S::PRECHECK_INDEX&&S::index(payload.get()).is_some_and(|index|if S::REPLACE{index>=len}else{index>len});if invalid_rank{self.fail(S::INVALID_RANK,false);}else if (S::INSERT||S::REPLACE)&&S::member(payload.get()).is_some_and(|v|!S::valid_member(v)){self.fail(RowOutcome::Invalid,true);}else if (S::INSERT||S::REPLACE)&&S::reference().is_some(){self.phase=7;}else{self.phase=9;}},
 7=>{self.reference=Some(Box::new(ReferenceLookup::new(S::reference().ok_or_else(invalid)?)));self.phase=8;p=receipt(birth::<ReferenceLookup>());},
 8=>{let c=self.reference.as_mut().ok_or_else(invalid)?;p=add(c.advance(o,S::reference_key(payload)?,&mut effect.nested(child(g,1)))?.progress(),1)?;if let Some(found)=c.result(){if !found{self.outcome=Some(RowOutcome::InvalidReference);self.source_target=true;}c.begin_close();self.phase=11;}},
 11=>{let step=custody::finish(&mut self.flight,&mut self.reference,body,effect,&mut work,recover)?;p=step.progress();if self.reference.is_none(){self.phase=if self.outcome.is_some(){99}else{9};}},
 9=>{p=receipt(scalar(1));if S::BY_INDEX&&!S::SORTED&&S::DUPLICATE.is_none(){self.phase=10;}else if self.scan<self.old()?.len(){let index=self.scan;let item=self.old()?.item(index);if S::member(payload.get())==Some(item)&&self.found.is_none(){self.found=Some(index);}if S::SORTED&&self.ordered.is_none()&&S::member(payload.get()).is_some_and(|v|S::item_key(item)>S::item_key(v)){self.ordered=Some(index);}self.scan+=1;}else{self.phase=10;}},
 10=>{p=receipt(scalar(1));let len=self.old()?.len();let rank=S::index(payload.get()).unwrap_or_else(||self.ordered.unwrap_or(len));let duplicate=S::INSERT&&self.found.is_some()&&S::DUPLICATE.is_some();let invalid_rank=if S::MOVE{rank>=len||S::destination(payload.get()).is_none_or(|to|to>=len)}else if S::INSERT{rank>len}else if S::BY_INDEX{rank>=len}else{self.found.is_none()};if invalid_rank&&(!duplicate||S::RANK_FIRST){self.fail(S::INVALID_RANK,!S::INSERT&&!S::BY_INDEX);}else if S::REPLACE&&S::member(payload.get())==Some(self.old()?.item(rank)){self.fail(RowOutcome::Unchanged,false);}else if S::MOVE&&Some(rank)==S::destination(payload.get()){self.fail(RowOutcome::Unchanged,false);}else if duplicate{self.fail(S::DUPLICATE.ok_or_else(invalid)?,S::DUPLICATE==Some(RowOutcome::Duplicate));}else if invalid_rank{self.fail(RowOutcome::Missing,!S::BY_INDEX);}else{let index=if S::INSERT||S::BY_INDEX{rank}else{self.found.ok_or_else(invalid)?};let item=if S::INSERT{S::member(payload.get()).ok_or_else(invalid)?}else{self.old()?.item(index)};self.selected=Some(MembershipInverse{index:u32::try_from(index).map_err(|_|invalid())?,item});self.phase=12;}},
 12=>{let count=self.count()?;let values=S::Values::begin(count)?;self.values.set(values);self.phase=13;p=receipt(scalar(1));},13=>{p=receipt(scalar(1));let count=self.count()?;if self.copied==count{self.phase=14;}else{let selected=self.selected.ok_or_else(invalid)?;let index=selected.index as usize;let offset=self.copied;let item=if S::REPLACE&&offset==index{S::member(payload.get()).ok_or_else(invalid)?}else if S::INSERT&&offset==index{selected.item}else{let source=if S::MOVE{let to=S::destination(payload.get()).ok_or_else(invalid)?;if offset==to{index}else if index<to&&offset>=index&&offset<to{offset+1}else if index>to&&offset>to&&offset<=index{offset-1}else{offset}}else if S::INSERT&&offset>index{offset-1}else if !S::INSERT&&!S::REPLACE&&offset>=index{offset+1}else{offset};self.old()?.item(source)};S::Values::push(self.values.get_mut().ok_or_else(invalid)?,offset,item)?;self.copied+=1;}},14=>{let values=S::Values::finish(self.values.take().ok_or_else(invalid)?)?;self.raw.set(values);self.phase=31;p=receipt(scalar(1));},
 31=>{let turn=SealedShared::admit_from(self.raw.slot(),SharedIssuer::owned(),self.replacement.slot(),&mut effect.nested(g));work+=turn.work;p=turn.result?.progress();if self.replacement.is_some(){self.phase=15;}},
 15=>{self.tail=Some(Box::new(RowTail::new(self.rank.ok_or_else(invalid)?)));self.phase=30;p=receipt(scalar(1));},
 30=>{let Self{tail,row,replacement,displaced,..}=self;let tail=tail.as_mut().ok_or_else(invalid)?;let row=row.as_ref().ok_or_else(invalid)?.borrow()?;
  let mut exchange=|stage:&mut RetainedBranchProjection<S::Row>,e:&mut RetainedAllocationEffect<'_>|stage.exchange(S::select,replacement.slot(),displaced.slot(),e);
  let step=tail_step::<S,SealedShared<S::Values>>(tail,row,o,body,max,effect,&mut work,recover,&mut exchange)?;p=step.progress();if tail.ready(){self.outcome=Some(RowOutcome::Changed);self.phase=99;}},
 _=>return Err(invalid().into())}
   admit_retained_clone_progress(g,p,"original Energy membership")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn ready(&self)->bool{!self.closing&&self.phase==99}fn outcome(&self)->Option<RowOutcome>{self.outcome}
 fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.tail.is_none(){return Err(invalid());}self.tail.as_ref().ok_or_else(invalid)?.take_quote()}
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{self.take_quote()?;let taken=self.tail.as_mut().ok_or_else(invalid)?.take(effect)?;if taken.is_some(){self.phase=100;}Ok(taken)}
 fn inverse_quote(&self,_:usize,_:u64)->Result<Quote,ValueError>{if self.closing||self.phase!=100||self.selected.is_none(){return Err(invalid());}Ok((scalar(1),1))}
 fn inverse_ready(&self)->bool{!self.closing&&self.phase==100&&self.selected.is_some()}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{if !self.inverse_ready(){return Err(invalid());}Ok((scalar(2*size_of::<Option<Self::Inverse>>()),1))}
 fn take_inverse(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Inverse,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_inverse_quote()?;if !retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}Ok(Some((self.selected.take().ok_or_else(invalid)?,receipt(d))))}
}
