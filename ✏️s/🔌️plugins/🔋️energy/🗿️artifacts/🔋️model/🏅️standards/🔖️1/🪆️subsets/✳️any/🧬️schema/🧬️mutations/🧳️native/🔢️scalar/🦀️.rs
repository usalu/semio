//! 🔢️ Schema-declared scalar row changes prepare only an original row, changed page path and declared model branch.
use crate::{snapshot::{original::EnergyModelSnapshot,state::fields::{EntityRow,RetainedEntityLookup,RetainedBranchProjection}},diff::original::EnergyModelDiff};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,RetainedOwnedProjection,retirement::RetireOwned,shared_issuance::SealedShared,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneSource,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_persistent::list::persistent::{PersistentList,PersistentSet};
use protocol::mutation::retained::retained_mutation_demand_fits;
use std::mem::{size_of,ManuallyDrop};
use super::{reference::{ReferenceCollection,ReferenceLookup},row::{RowBranchPreparation,ScalarBranch}};
use semio_framework_value_snapshot_projection::snapshot_projection::SnapshotProjection;
/// 🔢️ Fixed copied payloads have no owned traversal or hidden allocation.
pub trait NativeScalar:Copy+PartialEq+RetireOwned+Send+Sync{}
impl<T:Copy+PartialEq+RetireOwned+Send+Sync> NativeScalar for T{}
/// 📦️ The authored row storage explicitly distinguishes a direct scalar from an independently sealed value.
pub trait ScalarStorage:Send+Sync+'static{
 type Value:NativeScalar;type Stored:RetireOwned+Send+Sync;
 fn value(stored:&Self::Stored)->Self::Value;
 fn birth_demand()->RetirementDemand;
 fn admit(value:Self::Value,g:RetainedCloneGrant)->Result<(Self::Stored,RetainedCloneProgress),ValueError>;
}
pub struct DirectScalar<T:NativeScalar>(std::marker::PhantomData<T>);
impl<T:NativeScalar> ScalarStorage for DirectScalar<T>{type Value=T;type Stored=T;fn value(value:&T)->T{*value}fn birth_demand()->RetirementDemand{scalar(2*size_of::<T>())}fn admit(value:T,g:RetainedCloneGrant)->Result<(T,RetainedCloneProgress),ValueError>{let d=Self::birth_demand();if !retained_mutation_demand_fits(d,g){return Err(invalid());}Ok((value,receipt(d)))}}
pub struct SharedScalar<T:NativeScalar>(std::marker::PhantomData<T>);
impl<T:NativeScalar> ScalarStorage for SharedScalar<T>{type Value=T;type Stored=SealedShared<T>;fn value(value:&Self::Stored)->T{*value.get()}fn birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>()+2*size_of::<SealedShared<T>>(),capacity_bytes:SealedShared::<T>::birth_bytes(),depth:1,..Default::default()}}fn admit(value:T,g:RetainedCloneGrant)->Result<(Self::Stored,RetainedCloneProgress),ValueError>{let d=Self::birth_demand();if !retained_mutation_demand_fits(d,g){return Err(invalid());}SealedShared::admit(value,semio_framework_value::shared_issuance::SharedIssuer::owned(),g).map_err(|(e,_)|e).map(|(value,p)|{let mut r=receipt(d);r.retained_capacity_bytes=p.retained_capacity_bytes;(value,r)})}}
/// 🧩️ A fixed authored union maps copied source fields to its actual stored value without owned intermediates.
pub trait FixedScalarMapping:Send+Sync+'static{type Value:NativeScalar;type Stored:NativeScalar;fn into_stored(value:Self::Value)->Option<Self::Stored>;fn from_stored(value:Self::Stored)->Self::Value;}
pub struct MappedSharedScalar<M:FixedScalarMapping>(std::marker::PhantomData<M>);
impl<M:FixedScalarMapping> ScalarStorage for MappedSharedScalar<M>{type Value=M::Value;type Stored=SealedShared<M::Stored>;fn value(stored:&Self::Stored)->Self::Value{M::from_stored(*stored.get())}fn birth_demand()->RetirementDemand{let mut d=SharedScalar::<M::Stored>::birth_demand();d.copy_bytes+=2*size_of::<M::Value>();d}fn admit(value:Self::Value,g:RetainedCloneGrant)->Result<(Self::Stored,RetainedCloneProgress),ValueError>{let d=Self::birth_demand();if !retained_mutation_demand_fits(d,g){return Err(invalid());}let extra=2*size_of::<M::Value>();let value=M::into_stored(value).ok_or_else(invalid)?;let(value,p)=SharedScalar::<M::Stored>::admit(value,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,..g})?;Ok((value,add(p,extra)?))}}
type Stored<S>=<<S as ScalarRowPath>::Storage as ScalarStorage>::Stored;
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
 original:Option<RetainedCloneBinding>,key:Option<RetainedCloneBinding>,authored:Option<RetainedCloneBinding>,
 lookup:Option<Box<RetainedEntityLookup<S::Row>>>,row:Option<Box<RetainedOwnedProjection<S::Row>>>,row_stage:Option<Box<RetainedBranchProjection<S::Row>>>,publication:Option<Box<RowBranchPreparation<ScalarBranch<S>>>>,
 row_value:ManuallyDrop<Option<SealedShared<S::Row>>>,row_source:ManuallyDrop<Option<RetainedCloneSource<SealedShared<S::Row>>>>,
 decision:ManuallyDrop<Option<EnergyModelDiff>>,
 value:ManuallyDrop<Option<Stored<S>>>,inverse:ManuallyDrop<Option<Stored<S>>>,authored_value:Option<S::Scalar>,inverse_taken:bool,rank:Option<usize>,phase:u8,outcome:Option<ScalarOutcome>,closing:bool,retirement:RetainedCloneClose,
}
impl<S:ScalarRowPath> ScalarRowPreparation<S>{
 pub fn new()->Self{Self::with_authored_validation(true)}
 pub fn with_authored_validation(authored_valid:bool)->Self{Self{authored_valid,body:None,marker:Default::default(),reference:None,reference_index:0,reference_found:false,original:None,key:None,authored:None,lookup:None,row:None,row_stage:None,publication:None,row_value:ManuallyDrop::new(None),row_source:ManuallyDrop::new(None),decision:ManuallyDrop::new(None),value:ManuallyDrop::new(None),inverse:ManuallyDrop::new(None),authored_value:None,inverse_taken:false,rank:None,phase:0,outcome:None,closing:false,retirement:Default::default()}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==99}
 pub fn outcome(&self)->Option<ScalarOutcome>{self.outcome}
 fn branch(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,S::Branch>{S::branch(original)}
 fn rows(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<S::Row>>{S::rows(original)}
 fn row(&self)->Result<RetainedCloneRef<'_,S::Row>,ValueError>{self.row.as_ref().ok_or_else(invalid)?.borrow()}
 fn source(&self)->Result<RetainedCloneRef<'_,SealedShared<S::Row>>,ValueError>{Ok(self.row_source.as_ref().ok_or_else(invalid)?.borrow())}
 fn check(&self,value:RetainedCloneRef<'_,S::Scalar>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{value.check_binding(&self.authored)?;key.check_binding(&self.key)?;original.check_binding(&self.original)?;Ok(())}
 pub fn demand(&self,value:RetainedCloneRef<'_,S::Scalar>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>=99{return Err(invalid());}self.check(value,key,original)?;if self.body.is_some_and(|b|b!=body){return Err(invalid());}match self.phase{
   0=>nested(scalar(original.binding_copy_bytes()+if self.body.is_none(){size_of::<usize>()}else{0}),1),1=>nested(scalar(key.binding_copy_bytes()),1),2=>nested(scalar(value.binding_copy_bytes()),1),
   3=>Ok(birth::<RetainedEntityLookup<S::Row>>()),
   4=>nested(self.lookup.as_ref().ok_or_else(invalid)?.demand(Self::rows(original),key)?,1),
   5=>if self.lookup.as_ref().ok_or_else(invalid)?.result()==Some(None){Ok(scalar(2))}else{{let mut d=nested(self.lookup.as_ref().ok_or_else(invalid)?.take_demand()?,2*size_of::<RetainedOwnedProjection<S::Row>>()+2*size_of::<Option<Box<RetainedOwnedProjection<S::Row>>>>()+2*size_of::<Option<usize>>()+1)?;d.capacity_bytes=d.capacity_bytes.checked_add(size_of::<RetainedOwnedProjection<S::Row>>()).ok_or_else(invalid)?;Ok(d)}},
   6=>cursor_demand(&self.lookup,body),
   7|29=>Ok(scalar(2*size_of::<S::Scalar>()+2*size_of::<Option<S::Scalar>>()+3)),
   26=>Ok(birth::<ReferenceLookup>()),
   27=>nested(self.reference.as_ref().ok_or_else(invalid)?.demand(original,S::reference_key(value)?)?,2),
   28=>nested(cursor_demand(&self.reference,body.saturating_sub(2*size_of::<usize>()+1))?,2*size_of::<usize>()+1),
   25=>nested(S::Storage::birth_demand(),2*size_of::<Option<S::Scalar>>()+2*size_of::<Option<Stored<S>>>()+1),
   8=>Ok(birth::<RetainedBranchProjection<S::Row>>()),
   9|11=>nested(self.row_stage.as_ref().ok_or_else(invalid)?.demand(self.row()?,body.saturating_sub(1))?,1),
   10=>nested(RetainedBranchProjection::<S::Row>::exchange_demand::<Stored<S>>(),1),
   12=>nested(self.row_stage.as_ref().ok_or_else(invalid)?.take_demand()?,2*size_of::<Option<SealedShared<S::Row>>>()+1),
   13=>cursor_demand(&self.row_stage,body),
   14=>{let d=RetainedCloneSource::<SealedShared<S::Row>>::owned_constructor_demand::<()>();Ok(RetirementDemand{copy_bytes:RetainedCloneSource::<SealedShared<S::Row>>::constructor_copy_bytes()+2*size_of::<Option<SealedShared<S::Row>>>()+2*size_of::<Option<RetainedCloneSource<SealedShared<S::Row>>>>()+1,capacity_bytes:d.capacity_bytes,depth:d.depth+1,..Default::default()})},
   15=>Ok(birth::<RowBranchPreparation<ScalarBranch<S>>>()),
   16=>nested(self.publication.as_ref().ok_or_else(invalid)?.demand(self.source()?,original,body.saturating_sub(1))?,1),
   17=>nested(self.publication.as_ref().ok_or_else(invalid)?.take_demand()?,2*size_of::<Option<EnergyModelDiff>>()+1),
   18=>cursor_demand(&self.publication,body),
   _=>Err(invalid()),
  }
 }
 pub fn advance(&mut self,value:RetainedCloneRef<'_,S::Scalar>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.demand(value,key,original,body)?;let body_birth=if self.body.is_none(){size_of::<usize>()}else{0};if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}self.body=Some(body);let mut p=receipt(d);match self.phase{
   0=>{p=add(original.bind(&mut self.original,child(g,1+body_birth))?.ok_or_else(invalid)?,1+body_birth)?;self.phase=1;},1=>{p=add(key.bind(&mut self.key,child(g,1))?.ok_or_else(invalid)?,1)?;self.phase=2;},2=>{p=add(value.bind(&mut self.authored,child(g,1))?.ok_or_else(invalid)?,1)?;self.phase=3;},
   3=>{self.lookup=Some(Box::new(RetainedEntityLookup::new()));self.phase=4;},
   4=>{let lookup=self.lookup.as_mut().ok_or_else(invalid)?;p=add(lookup.advance(Self::rows(original),key,child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if lookup.result().is_some(){self.phase=5;}},
   5=>{let lookup=self.lookup.as_mut().ok_or_else(invalid)?;if lookup.result()==Some(None){self.outcome=Some(ScalarOutcome::Missing);self.phase=99;}else{let extra=2*size_of::<RetainedOwnedProjection<S::Row>>()+2*size_of::<Option<Box<RetainedOwnedProjection<S::Row>>>>()+2*size_of::<Option<usize>>()+1;let(row,progress)=lookup.take(RetainedCloneGrant{maximum_capacity_bytes:g.maximum_capacity_bytes-size_of::<RetainedOwnedProjection<S::Row>>(),..child(g,extra)}).map_err(|e|tag(e,extra))?.ok_or_else(invalid)?;self.rank=lookup.result().ok_or_else(invalid)?;self.row=Some(Box::new(row));lookup.begin_close();self.phase=6;p=add(progress,extra)?;p.retained_capacity_bytes=p.retained_capacity_bytes.checked_add(size_of::<RetainedOwnedProjection<S::Row>>()).ok_or_else(invalid)?;}},
   6=>{p=close_cursor(&mut self.lookup,g)?;if self.lookup.is_none(){self.phase=7;}},
   7=>{let authored=*value.get();if !self.authored_valid||!S::valid(authored){self.outcome=Some(ScalarOutcome::Invalid);self.phase=99;}else if !S::valid_reference(authored,*key.get()){self.outcome=Some(ScalarOutcome::InvalidReference);self.phase=99;}else if S::reference_collections().is_empty()||!S::has_reference(authored){self.phase=29;}else{self.phase=26;}},
   26=>{self.reference=Some(Box::new(ReferenceLookup::new(*S::reference_collections().get(self.reference_index).ok_or_else(invalid)?)));self.phase=27;},
   27=>{let reference=self.reference.as_mut().ok_or_else(invalid)?;p=add(reference.advance(original,S::reference_key(value)?,child(g,2)).map_err(|e|tag(e,2))?.progress(),2)?;if let Some(found)=reference.result(){self.reference_found=found;reference.begin_close();self.phase=28;}},
   28=>{let extra=2*size_of::<usize>()+1;p=add(close_cursor(&mut self.reference,child(g,extra)).map_err(|e|tag(e,extra))?,extra)?;if self.reference.is_none(){if self.reference_found{self.phase=29;}else if self.reference_index+1<S::reference_collections().len(){self.reference_index+=1;self.phase=26;}else{self.outcome=Some(ScalarOutcome::InvalidReference);self.phase=99;}}},
   29=>{let authored=*value.get();if S::read(self.row()?.get())==authored{self.outcome=Some(S::unchanged_outcome());self.phase=99;}else{self.authored_value=Some(authored);self.phase=25;}},
   25=>{let extra=2*size_of::<Option<S::Scalar>>()+2*size_of::<Option<Stored<S>>>()+1;let(value,progress)=S::Storage::admit(self.authored_value.ok_or_else(invalid)?,child(g,extra)).map_err(|e|tag(e,extra))?;*self.value=Some(value);self.authored_value=None;self.phase=8;p=add(progress,extra)?;},
   8=>{self.row_stage=Some(Box::new(RetainedBranchProjection::new()));self.phase=9;},
   9|11=>{let row=self.row.as_ref().ok_or_else(invalid)?.borrow()?;let stage=self.row_stage.as_mut().ok_or_else(invalid)?;p=add(stage.advance(row,body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if self.phase==9&&stage.raw_ready(){self.phase=10;}else if self.phase==11&&stage.ready(){self.phase=12;}},
   10=>{let stage=self.row_stage.as_mut().ok_or_else(invalid)?;let(_,progress)=stage.exchange_with(S::replace,&mut self.value,&mut self.inverse,child(g,1)).map_err(|e|tag(e,1))?;stage.begin_seal()?;self.phase=11;p=add(progress,1)?;},
   12=>{let extra=2*size_of::<Option<SealedShared<S::Row>>>()+1;let stage=self.row_stage.as_mut().ok_or_else(invalid)?;let(row,progress)=stage.take(child(g,extra)).map_err(|e|tag(e,extra))?.ok_or_else(invalid)?;*self.row_value=Some(row);stage.begin_close();self.phase=13;p=add(progress,extra)?;},
   13=>{p=close_cursor(&mut self.row_stage,g)?;if self.row_stage.is_none(){self.phase=14;}},
   14=>{let extra=2*size_of::<Option<SealedShared<S::Row>>>()+2*size_of::<Option<RetainedCloneSource<SealedShared<S::Row>>>>()+1;let row=self.row_value.take().ok_or_else(invalid)?;match RetainedCloneSource::admit_owned(row,(),child(g,extra)){Ok((source,progress))=>{*self.row_source=Some(source);p=add(progress,extra)?;self.phase=15;},Err((error,row,()))=>{*self.row_value=Some(row);return Err(tag(error,extra));}}},
   15=>{self.publication=Some(Box::new(RowBranchPreparation::new(self.rank.ok_or_else(invalid)?)));self.phase=16;},
   16=>{let row=self.row_source.as_ref().ok_or_else(invalid)?.borrow();let publication=self.publication.as_mut().ok_or_else(invalid)?;p=add(publication.advance(row,original,body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if publication.ready(){self.phase=17;}},
   17=>{let extra=2*size_of::<Option<EnergyModelDiff>>()+1;let publication=self.publication.as_mut().ok_or_else(invalid)?;let(diff,progress)=publication.take(child(g,extra)).map_err(|e|tag(e,extra))?.ok_or_else(invalid)?;*self.decision=Some(diff);publication.begin_close();self.phase=18;p=add(progress,extra)?;},
   18=>{p=close_cursor(&mut self.publication,g)?;if self.publication.is_none(){self.phase=99;self.outcome=Some(ScalarOutcome::Changed);}},
   _=>return Err(invalid()),
  }semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"native original Energy scalar row path")?;Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid());}Ok(scalar(2*size_of::<Option<EnergyModelDiff>>()+1))}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}self.phase=100;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 pub fn inverse_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.phase!=100||self.inverse.is_none()||self.inverse_taken{return Err(invalid());}Ok(scalar(2*size_of::<Option<S::Scalar>>()))}
 pub fn take_inverse(&mut self,g:RetainedCloneGrant)->Result<Option<(S::Scalar,RetainedCloneProgress)>,ValueError>{let d=self.inverse_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}self.inverse_taken=true;Ok(Some((S::Storage::value(self.inverse.as_ref().ok_or_else(invalid)?),receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;if let Some(p)=self.reference.as_mut(){p.begin_close();}if let Some(p)=self.lookup.as_mut(){p.begin_close();}if let Some(p)=self.row_stage.as_mut(){p.begin_close();}if let Some(p)=self.publication.as_mut(){p.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}let body=self.body.unwrap_or(body);if self.reference.is_some(){return cursor_demand(&self.reference,body);}if self.publication.is_some(){return cursor_demand(&self.publication,body);}if self.row_stage.is_some(){return cursor_demand(&self.row_stage,body);}if self.lookup.is_some(){return cursor_demand(&self.lookup,body);}if let Some(source)=self.row_source.as_ref(){if source.terminal_is_empty(){return Ok(scalar(2*size_of::<Option<RetainedCloneSource<SealedShared<S::Row>>>>()));}return source.next_close_demand(body);}if self.row.is_some(){return cursor_demand(&self.row,body);}if !self.retirement.is_empty()||self.decision.is_some(){return self.retirement.next_owner_demands::<EnergyModelDiff>(self.decision.is_some(),&None,body);}if self.row_value.is_some(){return self.retirement.next_owner_demands::<SealedShared<S::Row>>(true,&None,body);}if self.value.is_some()||self.inverse.is_some(){return self.retirement.next_owner_demands::<Stored<S>>(true,&None,body);}if self.authored_value.is_some(){return Ok(scalar(2*size_of::<Option<S::Scalar>>()));}RetainedCloneBinding::close_demand(if self.original.is_some(){&self.original}else if self.authored.is_some(){&self.authored}else{&self.key},body)}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(self.body.unwrap_or(g.maximum_copy_bytes))?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let g=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes,..g};if self.reference.is_some(){return close_cursor(&mut self.reference,g).map(RetainedCloneStep::Progress);}if self.publication.is_some(){return close_cursor(&mut self.publication,g).map(RetainedCloneStep::Progress);}if self.row_stage.is_some(){return close_cursor(&mut self.row_stage,g).map(RetainedCloneStep::Progress);}if self.lookup.is_some(){return close_cursor(&mut self.lookup,g).map(RetainedCloneStep::Progress);}
  if let Some(source)=self.row_source.as_mut(){if source.terminal_is_empty(){drop(self.row_source.take());return Ok(RetainedCloneStep::Progress(receipt(d)));}return source.close_step(g).map(|s|RetainedCloneStep::Progress(s.progress()));}if self.row.is_some(){return close_cursor(&mut self.row,g).map(RetainedCloneStep::Progress);}if !self.retirement.is_empty(){return self.retirement.select(self.body.unwrap_or(g.maximum_copy_bytes))?.step(g).map(|s|RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.decision,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.row_value,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.value,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.inverse,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if self.authored_value.is_some(){self.authored_value=None;return Ok(RetainedCloneStep::Progress(receipt(d)));}let s=RetainedCloneBinding::close_one(if self.original.is_some(){&mut self.original}else if self.authored.is_some(){&mut self.authored}else{&mut self.key},g)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(s.progress())}else{RetainedCloneStep::Progress(s.progress())})
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.reference.is_none()&&self.lookup.is_none()&&self.row.is_none()&&self.row_stage.is_none()&&self.publication.is_none()&&self.row_source.is_none()&&self.row_value.is_none()&&self.decision.is_none()&&self.value.is_none()&&self.inverse.is_none()&&self.authored_value.is_none()&&self.retirement.is_empty()&&self.original.is_none()&&self.key.is_none()&&self.authored.is_none()}
}
pub trait CursorClose{fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;fn terminal_is_empty(&self)->bool;}
impl CursorClose for ReferenceLookup{fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{Self::close_demand(self,b)}fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::close_step(self,g)}fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}}
impl<R:EntityRow> CursorClose for RetainedEntityLookup<R>{fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{Self::close_demand(self,b)}fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::close_step(self,g)}fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}}
impl<B:SnapshotProjection<Output:crate::snapshot::state::fields::BranchProjectionOutput<B>>+Send> CursorClose for RetainedBranchProjection<B>{fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{Self::close_demand(self,b)}fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::close_step(self,g)}fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}}
impl<R:RetireOwned+Send+Sync> CursorClose for PersistentSet<R>{fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{Self::close_demand(self,b)}fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::close_step(self,g)}fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}}
pub(crate) fn cursor_demand<C:CursorClose>(cursor:&Option<Box<C>>,body:usize)->Result<RetirementDemand,ValueError>{let c=cursor.as_ref().ok_or_else(invalid)?;if c.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:2*size_of::<Option<Box<C>>>(),release_bytes:size_of::<C>(),depth:1,..Default::default()})}else{c.close_demand(body)}}
pub(crate) fn close_cursor<C:CursorClose>(cursor:&mut Option<Box<C>>,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=cursor_demand(cursor,g.maximum_copy_bytes)?;if !retained_mutation_demand_fits(d,g){return Ok(Default::default());}let c=cursor.as_mut().ok_or_else(invalid)?;if c.terminal_is_empty(){drop(cursor.take());Ok(receipt(d))}else{Ok(c.close_step(g)?.progress())}}
pub(crate) fn birth<C>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<C>()+2*size_of::<Option<Box<C>>>(),capacity_bytes:size_of::<C>(),depth:1,..Default::default()}}
pub(crate) fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Zone scalar preparation lost its original source or candidate path")}
pub(crate) fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
pub(crate) fn nested(mut d:RetirementDemand,extra:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(extra).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
pub(crate) fn child(g:RetainedCloneGrant,extra:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,maximum_depth:g.maximum_depth-1,..g}}
pub(crate) fn add(mut p:RetainedCloneProgress,extra:usize)->Result<RetainedCloneProgress,ValueError>{p.copied_bytes=p.copied_bytes.checked_add(extra).ok_or_else(invalid)?;Ok(p)}
pub(crate) fn tag(e:ValueError,extra:usize)->ValueError{let p=e.retained_progress();match add(p,extra){Ok(p)=>e.with_retained_progress(p),Err(f)=>f.with_retained_progress(p)}}
pub(crate) fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl<S:ScalarRowPath> Drop for ScalarRowPreparation<S>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"native Zone scalar preparation retains original custody");if empty{unsafe{ManuallyDrop::drop(&mut self.row_value);ManuallyDrop::drop(&mut self.row_source);ManuallyDrop::drop(&mut self.decision);}}}}


#[path="🪆️leaf/🦀️.rs"]mod leaf;
pub use leaf::{ScalarLeafSpec,ScalarLeafPreparation};

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="🧩️transform/🦀️.rs"]mod transform;pub use transform::{OriginalScalarTransform,TransformedScalarPreparation};

impl<R:Sync> CursorClose for RetainedOwnedProjection<R>{fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{Self::close_demand(self,b)}fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::close_step(self,g)}fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}}
