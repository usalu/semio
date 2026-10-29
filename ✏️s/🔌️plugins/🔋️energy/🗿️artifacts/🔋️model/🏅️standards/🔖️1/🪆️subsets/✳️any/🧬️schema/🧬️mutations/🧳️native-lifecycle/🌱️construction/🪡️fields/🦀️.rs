//! 🪡️ Typed original field selectors compose without collecting a record or erasing its retirement.
use super::*;
use crate::snapshot::state::fields::RetainedSealedField;
use semio_framework_value::retained_clone::RetainedClone;
/// 🎯️ Each authored field selector borrows one exact original payload field.
pub trait RowFieldSource:Send+Sync+'static{type Payload:Send+Sync+'static;type Value:RetireOwned+Send+Sync;fn field(payload:RetainedCloneRef<'_,Self::Payload>)->RetainedCloneRef<'_,Self::Value>;}
/// 🧵️ The existing retained field owner creates only the field selected by its native recipe.
pub struct SealedRowField<S:RowFieldSource> where S::Value:RetainedClone{field:RetainedSealedField<S::Value>}
impl<S:RowFieldSource> RowFieldsPreparation<S::Payload> for SealedRowField<S> where S::Value:RetainedClone{
 type Fields=SealedShared<S::Value>;
 fn new()->Self{Self{field:RetainedSealedField::new()}}
 fn demand(&self,p:RetainedCloneRef<'_,S::Payload>,b:usize)->Result<RetirementDemand,ValueError>{self.field.demand(S::field(p),b)}
 fn advance(&mut self,p:RetainedCloneRef<'_,S::Payload>,b:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.field.advance(S::field(p),b,g)}
 fn ready(&self)->bool{self.field.ready()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{self.field.take_demand()}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(Self::Fields,RetainedCloneProgress)>,ValueError>{self.field.take(g)}
 fn begin_close(&mut self){self.field.begin_close();}
}
impl<S:RowFieldSource> CursorClose for SealedRowField<S> where S::Value:RetainedClone{fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{self.field.close_demand(b)}fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.field.close_step(g)}fn terminal_is_empty(&self)->bool{self.field.terminal_is_empty()}}
/// 🧩️ A fixed pair retains the exact separately admitted typed child fields until their final transfer.
pub struct RowFieldsPair<P:Send+Sync+'static,L:RowFieldsPreparation<P>,R:RowFieldsPreparation<P>>{left:Option<Box<L>>,right:Option<Box<R>>,a:ManuallyDrop<Option<L::Fields>>,b:ManuallyDrop<Option<R::Fields>>,retirement:RetainedCloneClose,phase:u8,closing:bool,marker:std::marker::PhantomData<fn()->P>}
impl<P:Send+Sync+'static,L:RowFieldsPreparation<P>,R:RowFieldsPreparation<P>> RowFieldsPreparation<P> for RowFieldsPair<P,L,R>{
 type Fields=(L::Fields,R::Fields);
 fn new()->Self{Self{left:None,right:None,a:ManuallyDrop::new(None),b:ManuallyDrop::new(None),retirement:Default::default(),phase:0,closing:false,marker:Default::default()}}
 fn demand(&self,p:RetainedCloneRef<'_,P>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=8{return Err(invalid());}match self.phase{0=>Ok(birth::<L>()),1=>nested(self.left.as_ref().ok_or_else(invalid)?.demand(p,body.saturating_sub(1))?,1),2=>nested(self.left.as_ref().ok_or_else(invalid)?.take_demand()?,2*size_of::<Option<L::Fields>>()+1),3=>cursor_demand(&self.left,body),4=>Ok(birth::<R>()),5=>nested(self.right.as_ref().ok_or_else(invalid)?.demand(p,body.saturating_sub(1))?,1),6=>nested(self.right.as_ref().ok_or_else(invalid)?.take_demand()?,2*size_of::<Option<R::Fields>>()+1),7=>cursor_demand(&self.right,body),_=>Err(invalid())}}
 fn advance(&mut self,payload:RetainedCloneRef<'_,P>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demand(payload,body)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let mut p=receipt(d);match self.phase{0=>{self.left=Some(Box::new(L::new()));self.phase=1;},1=>{let c=self.left.as_mut().ok_or_else(invalid)?;p=add(c.advance(payload,body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if c.ready(){self.phase=2;}},2=>{let extra=2*size_of::<Option<L::Fields>>()+1;let c=self.left.as_mut().ok_or_else(invalid)?;let(v,progress)=c.take(child(g,extra)).map_err(|e|tag(e,extra))?.ok_or_else(invalid)?;*self.a=Some(v);c.begin_close();self.phase=3;p=add(progress,extra)?;},3=>{p=close_cursor(&mut self.left,g)?;if self.left.is_none(){self.phase=4;}},4=>{self.right=Some(Box::new(R::new()));self.phase=5;},5=>{let c=self.right.as_mut().ok_or_else(invalid)?;p=add(c.advance(payload,body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if c.ready(){self.phase=6;}},6=>{let extra=2*size_of::<Option<R::Fields>>()+1;let c=self.right.as_mut().ok_or_else(invalid)?;let(v,progress)=c.take(child(g,extra)).map_err(|e|tag(e,extra))?.ok_or_else(invalid)?;*self.b=Some(v);c.begin_close();self.phase=7;p=add(progress,extra)?;},7=>{p=close_cursor(&mut self.right,g)?;if self.right.is_none(){self.phase=8;}},_=>return Err(invalid())}semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"original typed row field pair")?;Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 fn ready(&self)->bool{!self.closing&&self.phase==8}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready()||self.a.is_none()||self.b.is_none(){return Err(invalid());}Ok(scalar(2*size_of::<Option<L::Fields>>()+2*size_of::<Option<R::Fields>>()+size_of::<Self::Fields>()+1))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(Self::Fields,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}self.phase=9;Ok(Some(((self.a.take().ok_or_else(invalid)?,self.b.take().ok_or_else(invalid)?),receipt(d))))}
 fn begin_close(&mut self){self.closing=true;if let Some(c)=self.left.as_mut(){c.begin_close();}if let Some(c)=self.right.as_mut(){c.begin_close();}}
}
impl<P:Send+Sync+'static,L:RowFieldsPreparation<P>,R:RowFieldsPreparation<P>> CursorClose for RowFieldsPair<P,L,R>{
 fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if self.right.is_some(){return cursor_demand(&self.right,b);}if self.left.is_some(){return cursor_demand(&self.left,b);}if !self.retirement.is_empty()||self.a.is_some(){return self.retirement.next_owner_demands::<L::Fields>(self.a.is_some(),&None,b);}if self.b.is_some(){return self.retirement.next_owner_demands::<R::Fields>(true,&None,b);}Ok(scalar(0))}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(g.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.right.is_some(){return close_cursor(&mut self.right,g).map(RetainedCloneStep::Progress);}if self.left.is_some(){return close_cursor(&mut self.left,g).map(RetainedCloneStep::Progress);}if !self.retirement.is_empty(){return self.retirement.step_granted(g);}if let Some(s)=self.retirement.begin_granted(&mut self.a,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.b,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}Ok(RetainedCloneStep::Complete(Default::default()))}
 fn terminal_is_empty(&self)->bool{self.closing&&self.left.is_none()&&self.right.is_none()&&self.a.is_none()&&self.b.is_none()&&self.retirement.is_empty()}
}
impl<P:Send+Sync+'static,L:RowFieldsPreparation<P>,R:RowFieldsPreparation<P>> Drop for RowFieldsPair<P,L,R>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"original row field pair retains its actual children");if empty{unsafe{ManuallyDrop::drop(&mut self.a);ManuallyDrop::drop(&mut self.b);}}}}

/// 🔢️ Rows containing only fixed authored scalars have no variable field birth.
pub struct FixedRowFields<P:Send+Sync+'static>{phase:u8,closing:bool,marker:std::marker::PhantomData<fn()->P>}
impl<P:Send+Sync+'static> RowFieldsPreparation<P> for FixedRowFields<P>{
 type Fields=();fn new()->Self{Self{phase:0,closing:false,marker:Default::default()}}
 fn demand(&self,_:RetainedCloneRef<'_,P>,_:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase!=0{return Err(invalid());}Ok(scalar(1))}
 fn advance(&mut self,p:RetainedCloneRef<'_,P>,b:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demand(p,b)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}self.phase=1;Ok(RetainedCloneStep::Complete(receipt(d)))}
 fn ready(&self)->bool{!self.closing&&self.phase==1}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(scalar(1))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<((),RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}self.phase=2;Ok(Some(((),receipt(d))))}
 fn begin_close(&mut self){self.closing=true;}
}
impl<P:Send+Sync+'static> CursorClose for FixedRowFields<P>{fn close_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}Ok(scalar(0))}fn close_step(&mut self,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(invalid());}Ok(RetainedCloneStep::Complete(Default::default()))}fn terminal_is_empty(&self)->bool{self.closing}}

/// 🔹️ Fixed unit enums retain their actual Copy representation through an admitted immutable field birth.
pub struct CopyRowField<S:RowFieldSource>(std::marker::PhantomData<fn()->S>);
impl<S:RowFieldSource> RowBirthSpec for CopyRowField<S> where S::Value:Copy{
 type Payload=S::Payload;type Context=();type Row=S::Value;type Fields=FixedRowFields<S::Payload>;
 fn finish(p:RetainedCloneRef<'_,S::Payload>,_:(),_:())->Result<S::Value,ValueError>{Ok(*S::field(p).get())}
}
impl<S:RowBirthSpec<Context=()>,F:RowBirthForm<S::Row>> RowFieldsPreparation<S::Payload> for RowBirthPreparation<S,F>{
 type Fields=F::Output;
 fn new()->Self{Self::new()}
 fn demand(&self,p:RetainedCloneRef<'_,S::Payload>,b:usize)->Result<RetirementDemand,ValueError>{Self::demand(self,p,b)}
 fn advance(&mut self,p:RetainedCloneRef<'_,S::Payload>,b:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::advance(self,p,b,g)}
 fn ready(&self)->bool{Self::ready(self)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{Self::take_demand(self)}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(Self::Fields,RetainedCloneProgress)>,ValueError>{Self::take(self,g)}
 fn begin_close(&mut self){Self::begin_close(self)}
}

/// ↩️ Inverse record fields retain exact original typed sources while copying only the required payload field.
pub struct OwnedRowField<S:RowFieldSource> where S::Value:RetainedClone{field:protocol::RetainedFieldPreparation<S::Value>}
impl<S:RowFieldSource> RowFieldsPreparation<S::Payload> for OwnedRowField<S> where S::Value:RetainedClone{
 type Fields=S::Value;
 fn new()->Self{Self{field:Default::default()}}
 fn demand(&self,p:RetainedCloneRef<'_,S::Payload>,b:usize)->Result<RetirementDemand,ValueError>{self.field.demand(S::field(p),b)}
 fn advance(&mut self,p:RetainedCloneRef<'_,S::Payload>,b:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.field.advance(S::field(p),b,g)}
 fn ready(&self)->bool{self.field.value().is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{self.field.exchange_demand()}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(Self::Fields,RetainedCloneProgress)>,ValueError>{let(value,p)=self.field.take(g)?;Ok(value.map(|value|(value,p)))}
 fn begin_close(&mut self){self.field.begin_close();}
}
impl<S:RowFieldSource> CursorClose for OwnedRowField<S> where S::Value:RetainedClone{fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{self.field.close_demand(b)}fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.field.close_step(g)}fn terminal_is_empty(&self)->bool{self.field.terminal_is_empty()}}
