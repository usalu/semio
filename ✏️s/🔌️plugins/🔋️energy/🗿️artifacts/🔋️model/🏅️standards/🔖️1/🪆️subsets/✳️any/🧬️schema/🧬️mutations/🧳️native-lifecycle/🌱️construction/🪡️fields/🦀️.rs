//! 🪡️ Typed original field selectors compose without collecting a record or erasing its retirement.
use super::*;
use crate::snapshot::state::fields::RetainedSealedField;
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedWorkPoll,retained_clone::{RetainedClone,RetainedCloneGrant}};
/// 🎯️ Each authored field selector borrows one exact original payload field.
pub trait RowFieldSource:Send+Sync+'static{type Payload:Send+Sync+'static;type Value:RetireOwned+Send+Sync;fn field(payload:RetainedCloneRef<'_,Self::Payload>)->RetainedCloneRef<'_,Self::Value>;}
fn unborn(_:&mut OriginalRetirementCloseLoan<'_>,_:RetainedCloneGrant)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
/// 🧵️ The existing retained field owner creates only the field selected by its native recipe.
pub struct SealedRowField<S:RowFieldSource> where S::Value:RetainedClone{field:RetainedSealedField<S::Value>,closing:bool,flight:Flight}
impl<S:RowFieldSource> SealedRowField<S> where S::Value:RetainedClone{
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(field);
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:RowFieldSource] SealedRowField<S>,[S::Value:RetainedClone],|s|s.begin_close());
impl<S:RowFieldSource> RowFieldsPreparation<S::Payload> for SealedRowField<S> where S::Value:RetainedClone{
 type Fields=SealedShared<S::Value>;
 fn new()->Self{Self{field:RetainedSealedField::new(),closing:false,flight:Default::default()}}
 fn quote(&self,p:RetainedCloneRef<'_,S::Payload>,b:usize,m:u64)->Result<Quote,ValueError>{let f=S::field(p);Ok((self.field.demand(f,b,m)?,self.field.advance_work_demand(f,b,m)?))}
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,S::Payload>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.field.advance_reply(S::field(p),b,m,e,r,recover)}
 fn ready(&self)->bool{self.field.ready()}
 fn take_quote(&self)->Result<Quote,ValueError>{Ok((self.field.take_demand()?,1))}
 fn take(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Fields,RetainedCloneProgress)>,ValueError>{self.field.take(e)}
}
/// 🧩️ A fixed pair retains the exact separately admitted typed child fields until their final transfer.
pub struct RowFieldsPair<P:Send+Sync+'static,L:RowFieldsPreparation<P>,R:RowFieldsPreparation<P>>{left:Option<Box<L>>,right:Option<Box<R>>,a:Held<L::Fields>,b:Held<R::Fields>,phase:u8,closing:bool,flight:Flight,marker:std::marker::PhantomData<fn()->P>}
impl<P:Send+Sync+'static,L:RowFieldsPreparation<P>,R:RowFieldsPreparation<P>> RowFieldsPair<P,L,R>{
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(right,left,a,b);
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([P:Send+Sync+'static,L:RowFieldsPreparation<P>,R:RowFieldsPreparation<P>] RowFieldsPair<P,L,R>,[],|s|s.begin_close());
impl<P:Send+Sync+'static,L:RowFieldsPreparation<P>,R:RowFieldsPreparation<P>> RowFieldsPreparation<P> for RowFieldsPair<P,L,R>{
 type Fields=(L::Fields,R::Fields);
 fn new()->Self{Self{left:None,right:None,a:Default::default(),b:Default::default(),phase:0,closing:false,flight:Default::default(),marker:Default::default()}}
 fn quote(&self,p:RetainedCloneRef<'_,P>,body:usize,m:u64)->Result<Quote,ValueError>{if self.closing||self.phase>=8||!self.flight.idle(){return Err(invalid());}let nb=body.saturating_sub(1);match self.phase{
  0=>Ok((birth::<L>(),1)),1=>{let(d,w)=self.left.as_ref().ok_or_else(invalid)?.quote(p,nb,m)?;Ok((nested(d,1)?,w))},2=>{let(d,w)=self.left.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<L::Fields>>()+1)?,w))},3=>Ok((Cursor::demand(&self.left,body)?,Cursor::work(&self.left,body)?)),
  4=>Ok((birth::<R>(),1)),5=>{let(d,w)=self.right.as_ref().ok_or_else(invalid)?.quote(p,nb,m)?;Ok((nested(d,1)?,w))},6=>{let(d,w)=self.right.as_ref().ok_or_else(invalid)?.take_quote()?;Ok((nested(d,2*size_of::<Option<R::Fields>>()+1)?,w))},7=>Ok((Cursor::demand(&self.right,body)?,Cursor::work(&self.right,body)?)),_=>Err(invalid())}}
 fn advance_reply<'r>(&mut self,payload:RetainedCloneRef<'_,P>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=RowFieldsPreparation::quote(self,payload,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   let mut p=RetainedCloneProgress::default();let nb=body.saturating_sub(1);
   match self.phase{
    0=>{self.left=Some(Box::new(L::new()));self.phase=1;p=receipt(birth::<L>());},
    1=>{let c=self.left.as_mut().ok_or_else(invalid)?;let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| c.advance_reply(payload,nb,max,&mut effect.nested(child(g,1)),r,false),c.advance_reply(payload,nb,max,&mut effect.nested(child(g,1)),r,true));p=add(step?.progress(),1)?;if c.ready(){self.phase=2;}},
    2=>{let extra=2*size_of::<Option<L::Fields>>()+1;let c=self.left.as_mut().ok_or_else(invalid)?;let(v,progress)=c.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.a.set(v);c.begin_close();self.phase=3;p=add(progress,extra)?;},
    3=>{let step=custody::finish(&mut self.flight,&mut self.left,body,effect,&mut work,recover)?;p=step.progress();if self.left.is_none(){self.phase=4;}},
    4=>{self.right=Some(Box::new(R::new()));self.phase=5;p=receipt(birth::<R>());},
    5=>{let c=self.right.as_mut().ok_or_else(invalid)?;let step=protocol::retained_routed!(self.flight,effect,recover,1,body,max,None,work,|r| c.advance_reply(payload,nb,max,&mut effect.nested(child(g,1)),r,false),c.advance_reply(payload,nb,max,&mut effect.nested(child(g,1)),r,true));p=add(step?.progress(),1)?;if c.ready(){self.phase=6;}},
    6=>{let extra=2*size_of::<Option<R::Fields>>()+1;let c=self.right.as_mut().ok_or_else(invalid)?;let(v,progress)=c.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.b.set(v);c.begin_close();self.phase=7;p=add(progress,extra)?;},
    7=>{let step=custody::finish(&mut self.flight,&mut self.right,body,effect,&mut work,recover)?;p=step.progress();if self.right.is_none(){self.phase=8;}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original typed row field pair")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.phase==8{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn ready(&self)->bool{!self.closing&&self.phase==8}
 fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.a.is_none()||self.b.is_none(){return Err(invalid());}Ok((scalar(2*size_of::<Option<L::Fields>>()+2*size_of::<Option<R::Fields>>()+size_of::<Self::Fields>()+1),1))}
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Fields,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(invalid());}self.phase=9;Ok(Some(((self.a.take().ok_or_else(invalid)?,self.b.take().ok_or_else(invalid)?),receipt(d))))}
}

/// 🔢️ Rows containing only fixed authored scalars have no variable field birth.
pub struct FixedRowFields<P:Send+Sync+'static>{phase:u8,closing:bool,marker:std::marker::PhantomData<fn()->P>}
impl<P:Send+Sync+'static> Cursor for FixedRowFields<P>{
 fn mark(&mut self){self.closing=true;}
 fn finished(&self)->bool{self.closing}
 fn demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(scalar(0))}
 fn work(&self,_:usize)->Result<u64,ValueError>{Ok(0)}
 fn birth(&self)->bool{false}
 fn control(&self)->bool{false}
 fn birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{unborn(o,g)}
 fn close<'r>(&mut self,_:Option<&mut OriginalRetirementCloseLoan<'_>>,_:usize,_:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,_:bool)->Result<Done<'r>,ValueError>{Ok(r.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
}
impl<P:Send+Sync+'static> RowFieldsPreparation<P> for FixedRowFields<P>{
 type Fields=();fn new()->Self{Self{phase:0,closing:false,marker:Default::default()}}
 fn quote(&self,_:RetainedCloneRef<'_,P>,_:usize,_:u64)->Result<Quote,ValueError>{if self.closing||self.phase!=0{return Err(invalid());}Ok((scalar(1),1))}
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,P>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{let q=if recover{None}else{Some(self.quote(p,b,m))};custody::gated(e,r,q,m,|_,_|{self.phase=1;Ok(RetainedCloneStep::Complete(receipt(scalar(1))))})}
 fn ready(&self)->bool{!self.closing&&self.phase==1}
 fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready(){return Err(invalid());}Ok((scalar(1),1))}
 fn take(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<((),RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if e.remaining_work_units()?<1||!retained_mutation_demand_fits(d,e.grant()){return Ok(None);}if !e.performed(1)?{return Err(invalid());}self.phase=2;Ok(Some(((),receipt(d))))}
}

/// 🔹️ Fixed unit enums retain their actual Copy representation through an admitted immutable field birth.
pub struct CopyRowField<S:RowFieldSource>(std::marker::PhantomData<fn()->S>);
impl<S:RowFieldSource> RowBirthSpec for CopyRowField<S> where S::Value:Copy{
 type Payload=S::Payload;type Context=();type Row=S::Value;type Fields=FixedRowFields<S::Payload>;
 fn finish(p:RetainedCloneRef<'_,S::Payload>,_:(),_:())->Result<S::Value,ValueError>{Ok(*S::field(p).get())}
}
impl<S:RowBirthSpec<Context=()>,F:RowBirthForm<S::Row>> RowFieldsPreparation<S::Payload> for RowBirthPreparation<S,F>{
 type Fields=F::Output;
 fn new()->Self{Self::new()}
 fn quote(&self,p:RetainedCloneRef<'_,S::Payload>,b:usize,m:u64)->Result<Quote,ValueError>{Self::quote(self,p,b,m)}
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,S::Payload>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{Self::advance_reply(self,p,b,m,e,r,recover)}
 fn ready(&self)->bool{Self::ready(self)}
 fn take_quote(&self)->Result<Quote,ValueError>{Self::take_quote(self)}
 fn take(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Fields,RetainedCloneProgress)>,ValueError>{Self::take(self,e)}
}

/// ↩️ Inverse record fields retain exact original typed sources while copying only the required payload field.
pub struct OwnedRowField<S:RowFieldSource> where S::Value:RetainedClone{field:protocol::RetainedFieldPreparation<S::Value>,value:Held<S::Value>,closing:bool,flight:Flight}
impl<S:RowFieldSource> OwnedRowField<S> where S::Value:RetainedClone{
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(field,value);
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:RowFieldSource] OwnedRowField<S>,[S::Value:RetainedClone],|s|s.begin_close());
impl<S:RowFieldSource> RowFieldsPreparation<S::Payload> for OwnedRowField<S> where S::Value:RetainedClone{
 type Fields=S::Value;
 fn new()->Self{Self{field:Default::default(),value:Default::default(),closing:false,flight:Default::default()}}
 fn quote(&self,p:RetainedCloneRef<'_,S::Payload>,b:usize,m:u64)->Result<Quote,ValueError>{let f=S::field(p);Ok((self.field.demand(f,b,m)?,self.field.advance_work_demand(f,b,m)?))}
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,S::Payload>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let q=if recover{None}else{Some(RowFieldsPreparation::quote(self,p,b,m))};let g=e.grant();let f=S::field(p);let nb=b.saturating_sub(1);
  custody::gated(e,r,q,m,|effect,work|{let Self{field,flight,..}=self;let step=protocol::retained_routed!(*flight,effect,recover,1,b,m,None,*work,|r| field.advance_into(f,nb,m,&mut effect.nested(child(g,1)),r),field.recover_advance_into(f,nb,m,&mut effect.nested(child(g,1)),r));Ok(RetainedCloneStep::Progress(add(step?.progress(),1)?))})
 }
 fn ready(&self)->bool{self.field.value().is_some()}
 fn take_quote(&self)->Result<Quote,ValueError>{Ok((self.field.exchange_demand()?,1))}
 fn take(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Fields,RetainedCloneProgress)>,ValueError>{
  let(d,_)=self.take_quote()?;if !retained_mutation_demand_fits(d,e.grant()){return Ok(None);}let g=e.grant();let mut work:u128=0;let Self{field,value,..}=self;let slot=value.slot();
  let step=native_child(e,&mut work,|e,r|field.take_into(slot,&mut e.nested(g),r))??;let v=value.take().ok_or_else(invalid)?;Ok(Some((v,step.progress())))
 }
}
