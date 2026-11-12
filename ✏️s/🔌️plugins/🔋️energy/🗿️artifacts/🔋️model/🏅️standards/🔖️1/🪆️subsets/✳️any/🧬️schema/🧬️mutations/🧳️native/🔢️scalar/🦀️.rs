//! 🔢️ Schema-declared scalar row changes prepare only an original row, changed page path and declared model branch.
use crate::{snapshot::{original::EnergyModelSnapshot,state::fields::{EntityRow,RetainedEntityLookup}},diff::original::EnergyModelDiff,custody::{self,Cursor,Held,Bound}};
use semio_framework_value::{RetirementDemand,RetirementPollExit,RetainedAllocationEffect,RetainedWorkOutcome,ValueError,ValueRefusalKind,RetainedOwnedProjection,retirement::RetireOwned,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_progress}};
use semio_framework_value_persistent::list::persistent::PersistentList;
use protocol::{retained_mutation_demand_fits};
use crate::retained::{Done,Flight,Reply,conclude};
use std::mem::size_of;
use super::{reference::{ReferenceCollection,ReferenceLookup},row::{RowTail,ScalarBranch,tail_step}};
use semio_framework_value_snapshot_projection::snapshot_projection::SnapshotProjection;
/// 🧾️ A quote states one step's byte demand and the exact work it will be charged.
pub type Quote=(RetirementDemand,u64);
/// 🔢️ Fixed copied payloads have no owned traversal or hidden allocation.
pub trait NativeScalar:Copy+PartialEq+RetireOwned+Send+Sync{}
impl<T:Copy+PartialEq+RetireOwned+Send+Sync> NativeScalar for T{}
/// 📦️ The authored row storage explicitly distinguishes a direct scalar from an independently sealed value.
pub trait ScalarStorage:Send+Sync+'static{
 type Value:NativeScalar;type Raw:NativeScalar;type Stored:RetireOwned+Send+Sync;
 fn raw(value:Self::Value)->Option<Self::Raw>;
 fn value(stored:&Self::Stored)->Self::Value;
 fn quote()->Result<Quote,ValueError>;
 fn admit(raw:&mut Option<Self::Raw>,output:&mut Option<Self::Stored>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep>;
}
pub struct DirectScalar<T:NativeScalar>(std::marker::PhantomData<T>);
impl<T:NativeScalar> ScalarStorage for DirectScalar<T>{type Value=T;type Raw=T;type Stored=T;fn raw(value:T)->Option<T>{Some(value)}fn value(value:&T)->T{*value}fn quote()->Result<Quote,ValueError>{Ok((scalar(2*size_of::<T>()),1))}fn admit(raw:&mut Option<T>,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep>{let d=scalar(2*size_of::<T>());if !retained_mutation_demand_fits(d,effect.grant()){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}*output=raw.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}}}
pub struct SharedScalar<T:NativeScalar>(std::marker::PhantomData<T>);
impl<T:NativeScalar> ScalarStorage for SharedScalar<T>{type Value=T;type Raw=T;type Stored=SealedShared<T>;fn raw(value:T)->Option<T>{Some(value)}fn value(value:&Self::Stored)->T{*value.get()}fn quote()->Result<Quote,ValueError>{let mut d=SealedShared::<T>::source_admission_demand()?;d.copy_bytes=d.copy_bytes.checked_add(2*size_of::<Option<T>>()+1).ok_or_else(invalid)?;Ok((d,SealedShared::<T>::source_admission_work_demand()))}fn admit(raw:&mut Option<T>,output:&mut Option<Self::Stored>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep>{SealedShared::admit_from(raw,SharedIssuer::owned(),output,effect)}}
/// 🧩️ A fixed authored union maps copied source fields to its actual stored value without owned intermediates.
pub trait FixedScalarMapping:Send+Sync+'static{type Value:NativeScalar;type Stored:NativeScalar;fn into_stored(value:Self::Value)->Option<Self::Stored>;fn from_stored(value:Self::Stored)->Self::Value;}
pub struct MappedSharedScalar<M:FixedScalarMapping>(std::marker::PhantomData<M>);
impl<M:FixedScalarMapping> ScalarStorage for MappedSharedScalar<M>{type Value=M::Value;type Raw=M::Stored;type Stored=SealedShared<M::Stored>;fn raw(value:M::Value)->Option<M::Stored>{M::into_stored(value)}fn value(stored:&Self::Stored)->Self::Value{M::from_stored(*stored.get())}fn quote()->Result<Quote,ValueError>{SharedScalar::<M::Stored>::quote()}fn admit(raw:&mut Option<M::Stored>,output:&mut Option<Self::Stored>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep>{SharedScalar::<M::Stored>::admit(raw,output,effect)}}
type Stored<S>=<<S as ScalarRowPath>::Storage as ScalarStorage>::Stored;
type Raw<S>=<<S as ScalarRowPath>::Storage as ScalarStorage>::Raw;
pub use crate::row::RowOutcome as ScalarOutcome;

/// 🧭️ Authored schema recipes provide typed paths, scalar limits and one native branch placement.
pub trait ScalarRowPath:Send+Sync+'static{
 type Scalar:NativeScalar;
 type Storage:ScalarStorage<Value=Self::Scalar>;
 type Row:EntityRow+SnapshotProjection<Output=Box<Self::Row>>;
 type Branch:SnapshotProjection<Output=Box<Self::Branch>>+Send+Sync;
 fn branch(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>;
 fn rows(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>;
 fn rows_mut(branch:&mut Self::Branch)->&mut PersistentList<Self::Row>;
 fn replace(row:&mut Self::Row,value:Stored<Self>)->Stored<Self>;
 fn read(row:&Self::Row)->Self::Scalar;
 fn valid(value:Self::Scalar)->bool;
 fn valid_reference(_:Self::Scalar,_:u32)->bool{true}
 fn unchanged_outcome()->ScalarOutcome{ScalarOutcome::Unchanged}
 fn has_reference(_:Self::Scalar)->bool{true}
 fn reference_collections()->&'static[ReferenceCollection]{&[]}
 fn reference_key(_:RetainedCloneRef<'_,Self::Scalar>)->Result<RetainedCloneRef<'_,u32>,ValueError>{Err(invalid())}
 type Output:crate::row::BranchOutput<Self::Branch>;
 fn output()->Self::Output;
}

/// 🧳️ Each actual typed cursor has a separately funded birth and close before the next path stage.
pub struct ScalarRowPreparation<S:ScalarRowPath>{
 authored_valid:bool,body:Option<usize>,marker:std::marker::PhantomData<fn()->S>,reference:Option<Box<ReferenceLookup>>,reference_index:usize,reference_found:bool,
 original:Bound,key:Bound,authored:Bound,
 lookup:Option<Box<RetainedEntityLookup<S::Row>>>,row:Option<Box<RetainedOwnedProjection<S::Row>>>,tail:Option<Box<RowTail<ScalarBranch<S>>>>,
 value:Held<Stored<S>>,inverse:Held<Stored<S>>,raw:Option<Raw<S>>,inverse_taken:bool,rank:Option<usize>,phase:u8,outcome:Option<ScalarOutcome>,closing:bool,flight:Flight,
}
impl<S:ScalarRowPath> ScalarRowPreparation<S>{
 pub fn new()->Self{Self::with_authored_validation(true)}
 pub fn with_authored_validation(authored_valid:bool)->Self{Self{authored_valid,body:None,marker:Default::default(),reference:None,reference_index:0,reference_found:false,original:Default::default(),key:Default::default(),authored:Default::default(),lookup:None,row:None,tail:None,value:Default::default(),inverse:Default::default(),raw:None,inverse_taken:false,rank:None,phase:0,outcome:None,closing:false,flight:Default::default()}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==99}
 pub fn outcome(&self)->Option<ScalarOutcome>{self.outcome}
 fn row(&self)->Result<RetainedCloneRef<'_,S::Row>,ValueError>{self.row.as_ref().ok_or_else(invalid)?.borrow()}
 pub fn check(&self,value:RetainedCloneRef<'_,S::Scalar>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{value.check_binding(&self.authored.0)?;key.check_binding(&self.key.0)?;original.check_binding(&self.original.0)?;Ok(())}
 pub fn quote(&self,value:RetainedCloneRef<'_,S::Scalar>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=99||!self.flight.idle(){return Err(invalid());}self.check(value,key,original)?;if self.body.is_some_and(|b|b!=body){return Err(invalid());}
  let lookup=||self.lookup.as_ref().ok_or_else(invalid);let rows=S::rows(original);
  match self.phase{
   0=>Ok((nested(scalar(original.binding_copy_bytes()+if self.body.is_none(){size_of::<usize>()}else{0}),1)?,original.binding_work_demand(&self.original.0)?)),
   1=>Ok((nested(scalar(key.binding_copy_bytes()),1)?,key.binding_work_demand(&self.key.0)?)),
   2=>Ok((nested(scalar(value.binding_copy_bytes()),1)?,value.binding_work_demand(&self.authored.0)?)),
   3=>Ok((birth::<RetainedEntityLookup<S::Row>>(),1)),
   4=>Ok((nested(lookup()?.demand(rows,key)?,1)?,lookup()?.advance_work_demand(rows,key)?)),
   5=>if lookup()?.result()==Some(None){Ok((scalar(2),1))}else{let mut d=nested(lookup()?.take_demand()?,2*size_of::<RetainedOwnedProjection<S::Row>>()+2*size_of::<Option<Box<RetainedOwnedProjection<S::Row>>>>()+2*size_of::<Option<usize>>()+1)?;d.capacity_bytes=d.capacity_bytes.checked_add(size_of::<RetainedOwnedProjection<S::Row>>()).ok_or_else(invalid)?;Ok((d,1))},
   6=>Ok((Cursor::demand(&self.lookup,body)?,Cursor::work(&self.lookup,body)?)),
   7|29=>Ok((scalar(2*size_of::<S::Scalar>()+2*size_of::<Option<S::Scalar>>()+3),1)),
   26=>Ok((birth::<ReferenceLookup>(),1)),
   27=>{let(d,w)=self.reference.as_ref().ok_or_else(invalid)?.quote(original,S::reference_key(value)?)?;Ok((nested(d,2)?,w))},
   28=>Ok((Cursor::demand(&self.reference,body)?,Cursor::work(&self.reference,body)?)),
   25=>{let(d,w)=S::Storage::quote()?;Ok((nested(d,2*size_of::<Option<S::Scalar>>()+2*size_of::<Option<Stored<S>>>()+1)?,w))},
   30=>self.tail.as_ref().ok_or_else(invalid)?.quote::<Stored<S>>(self.row()?,original,body,maximum_work_units),
   _=>Err(invalid()),
  }
 }
 pub fn advance_reply<'r>(&mut self,value:RetainedCloneRef<'_,S::Scalar>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;let body_birth=if self.body.is_none(){size_of::<usize>()}else{0};
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote(value,key,original,body,maximum_work_units)?;
    if u>maximum_work_units||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   self.body=Some(body);let mut p=RetainedCloneProgress::default();
   match self.phase{
    0=>{p=add(bind(original,&mut self.original,&mut effect.nested(child(g,1+body_birth)),&mut work)?,1+body_birth)?;self.phase=1;},
    1=>{p=add(bind(key,&mut self.key,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=2;},
    2=>{p=add(bind(value,&mut self.authored,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=3;},
    3=>{self.lookup=Some(Box::new(RetainedEntityLookup::new()));self.phase=4;p=receipt(birth::<RetainedEntityLookup<S::Row>>());},
    4=>{let lookup=self.lookup.as_mut().ok_or_else(invalid)?;p=add(lookup.advance(S::rows(original),key,&mut effect.nested(child(g,1)))?.progress(),1)?;if lookup.result().is_some(){self.phase=5;}},
    5=>{let lookup=self.lookup.as_mut().ok_or_else(invalid)?;if lookup.result()==Some(None){self.outcome=Some(ScalarOutcome::Missing);self.phase=99;p=receipt(scalar(2));}else{let extra=2*size_of::<RetainedOwnedProjection<S::Row>>()+2*size_of::<Option<Box<RetainedOwnedProjection<S::Row>>>>()+2*size_of::<Option<usize>>()+1;let(row,progress)=lookup.take(&mut effect.nested(RetainedCloneGrant{maximum_capacity_bytes:g.maximum_capacity_bytes.saturating_sub(size_of::<RetainedOwnedProjection<S::Row>>()),..child(g,extra)}))?.ok_or_else(invalid)?;self.rank=lookup.result().ok_or_else(invalid)?;self.row=Some(Box::new(row));lookup.begin_close();self.phase=6;p=add(progress,extra)?;p.retained_capacity_bytes=p.retained_capacity_bytes.checked_add(size_of::<RetainedOwnedProjection<S::Row>>()).ok_or_else(invalid)?;}},
    6=>{let step=custody::finish(&mut self.flight,&mut self.lookup,body,effect,&mut work,recover)?;p=step.progress();if self.lookup.is_none(){self.phase=7;}},
    7=>{let authored=*value.get();if !self.authored_valid||!S::valid(authored){self.outcome=Some(ScalarOutcome::Invalid);self.phase=99;}else if !S::valid_reference(authored,*key.get()){self.outcome=Some(ScalarOutcome::InvalidReference);self.phase=99;}else if S::reference_collections().is_empty()||!S::has_reference(authored){self.phase=29;}else{self.phase=26;}p=receipt(scalar(1));},
    26=>{self.reference=Some(Box::new(ReferenceLookup::new(*S::reference_collections().get(self.reference_index).ok_or_else(invalid)?)));self.phase=27;p=receipt(birth::<ReferenceLookup>());},
    27=>{let reference=self.reference.as_mut().ok_or_else(invalid)?;p=add(reference.advance(original,S::reference_key(value)?,&mut effect.nested(child(g,2)))?.progress(),2)?;if let Some(found)=reference.result(){self.reference_found=found;reference.begin_close();self.phase=28;}},
    28=>{let step=custody::finish(&mut self.flight,&mut self.reference,body,effect,&mut work,recover)?;p=step.progress();if self.reference.is_none(){if self.reference_found{self.phase=29;}else if self.reference_index+1<S::reference_collections().len(){self.reference_index+=1;self.phase=26;}else{self.outcome=Some(ScalarOutcome::InvalidReference);self.phase=99;}}},
    29=>{let authored=*value.get();if S::read(self.row()?.get())==authored{self.outcome=Some(S::unchanged_outcome());self.phase=99;}else{self.raw=Some(S::Storage::raw(authored).ok_or_else(invalid)?);self.phase=25;}p=receipt(scalar(1));},
    25=>{let extra=2*size_of::<Option<S::Scalar>>()+2*size_of::<Option<Stored<S>>>()+1;let turn=S::Storage::admit(&mut self.raw,self.value.slot(),&mut effect.nested(child(g,extra)));work+=turn.work;p=add(turn.result?.progress(),extra)?;if self.value.is_some(){self.phase=8;}},
    8=>{self.tail=Some(Box::new(RowTail::new(self.rank.ok_or_else(invalid)?)));self.phase=30;p=receipt(scalar(1));},
    30=>{let Self{tail,row,value:stored,inverse,..}=self;let tail=tail.as_mut().ok_or_else(invalid)?;let row=row.as_ref().ok_or_else(invalid)?.borrow()?;
     let mut exchange=|stage:&mut crate::snapshot::state::fields::RetainedBranchProjection<S::Row>,e:&mut RetainedAllocationEffect<'_>|stage.exchange_with(S::replace,stored.slot(),inverse.slot(),e);
     let step=tail_step::<ScalarBranch<S>,Stored<S>>(tail,row,original,body,maximum_work_units,effect,&mut work,recover,&mut exchange)?;p=step.progress();if tail.ready(){self.outcome=Some(ScalarOutcome::Changed);self.phase=99;}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"native original Energy scalar row path")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 pub fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.tail.is_none(){return Err(invalid());}self.tail.as_ref().ok_or_else(invalid)?.take_quote()}
 pub fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{self.take_quote()?;let taken=self.tail.as_mut().ok_or_else(invalid)?.take(effect)?;if taken.is_some(){self.phase=100;}Ok(taken)}
 pub fn inverse_quote(&self)->Result<Quote,ValueError>{if self.closing||self.phase!=100||self.inverse.is_none()||self.inverse_taken{return Err(invalid());}Ok((scalar(2*size_of::<Option<S::Scalar>>()),1))}
 pub fn take_inverse(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(S::Scalar,RetainedCloneProgress)>,ValueError>{let(d,_)=self.inverse_quote()?;if !retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}self.inverse_taken=true;Ok(Some((S::Storage::value(self.inverse.get().ok_or_else(invalid)?),receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;self.mark_close();}
 custody_impl!(reference,tail,lookup,row,value,inverse,original,authored,key);
}
crate::custody::std_cursor!([S:ScalarRowPath] ScalarRowPreparation<S>,[],|s|s.begin_close());
pub fn birth<C>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<C>()+2*size_of::<Option<Box<C>>>(),capacity_bytes:size_of::<C>(),depth:1,..Default::default()}}
pub fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Zone scalar preparation lost its original source or candidate path")}
pub fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
pub fn nested(mut d:RetirementDemand,extra:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(extra).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
pub fn child(g:RetainedCloneGrant,extra:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(extra),maximum_depth:g.maximum_depth.saturating_sub(1),..g}}
pub fn add(mut p:RetainedCloneProgress,extra:usize)->Result<RetainedCloneProgress,ValueError>{p.copied_bytes=p.copied_bytes.checked_add(extra).ok_or_else(invalid)?;Ok(p)}
pub fn tag(e:ValueError,extra:usize)->ValueError{let p=e.retained_progress();match add(p,extra){Ok(p)=>e.with_retained_progress(p),Err(f)=>f.with_retained_progress(p)}}
pub fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
/// 🔗️ Binds one original alias into its slot, charging the alias work to the turn.
pub fn bind<T:?Sized+Sync>(source:RetainedCloneRef<'_,T>,slot:&mut Bound,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<RetainedCloneProgress,ValueError>{let turn=source.bind(&mut slot.0,effect);*work+=turn.work;turn.result?.ok_or_else(invalid)}


#[path="🪆️leaf/🦀️.rs"]mod leaf;
pub use leaf::{ScalarLeafSpec,ScalarLeafPreparation};

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="🧩️transform/🦀️.rs"]mod transform;pub use transform::{OriginalScalarTransform,TransformedScalarPreparation};

