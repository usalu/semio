//! 📚️ A genuine original recreation bundle emits one typed inverse intent per paid turn.
use super::*;
/// ↩️ Authored deletion sources retain the original bundle and move each exact inverse piece without gathering a second source.
pub trait MultiDeletePath:DeleteRowPath where Self::Row:EntityRow{type InverseItem:RetireOwned+Send+'static;fn inverse_count(row:&Self::Row)->Result<usize,ValueError>;fn inverse_item_demand(bundle:&Self::Inverse,ordinal:usize,body:usize)->Result<RetirementDemand,ValueError>;fn take_inverse_item(bundle:&mut Self::Inverse,ordinal:usize,body:usize,g:RetainedCloneGrant)->Result<Option<(Self::InverseItem,RetainedCloneProgress)>,ValueError>;}
pub struct MultiDeleteRowPreparation<S:MultiDeletePath>where S::Row:EntityRow{base:DeleteRowPreparation<S>,bundle:Held<S::Inverse>,item:Held<S::InverseItem>,ordinal:usize,phase:u8,closing:bool,flight:Flight}
impl<S:MultiDeletePath> MultiDeleteRowPreparation<S> where S::Row:EntityRow{
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(item,bundle,base);
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:MultiDeletePath] MultiDeleteRowPreparation<S>,[S::Row:EntityRow],|s|s.begin_close());
impl<S:MultiDeletePath> RowFieldPreparation for MultiDeleteRowPreparation<S>where S::Row:EntityRow{
 type Input=S::Payload;type Inverse=S::InverseItem;
 fn new()->Self{Self{base:<DeleteRowPreparation<S> as RowFieldPreparation>::new(),bundle:Default::default(),item:Default::default(),ordinal:0,phase:0,closing:false,flight:Default::default()}}
 fn check(&self,p:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{self.base.check(p,k,o)}
 fn quote(&self,p:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64)->Result<Quote,ValueError>{RowFieldPreparation::quote(&self.base,p,k,o,b,m)}
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.base.advance_reply(p,k,o,b,m,e,r,recover)}
 fn ready(&self)->bool{!self.closing&&self.base.ready()}fn outcome(&self)->Option<RowOutcome>{self.base.outcome()}
 fn take_quote(&self)->Result<Quote,ValueError>{self.base.take_quote()}
 fn take(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{self.base.take(e)}
 fn inverse_count(&self)->Result<usize,ValueError>{if self.closing||self.base.phase!=100{return Err(invalid());}let row=self.base.row.as_ref().ok_or_else(invalid)?.borrow()?;<S as MultiDeletePath>::inverse_count(row.get())}
 fn inverse_quote(&self,b:usize,m:u64)->Result<Quote,ValueError>{if self.closing||self.base.body.is_some_and(|v|v!=b){return Err(invalid());}match self.phase{
  0=>if RowFieldPreparation::inverse_ready(&self.base){let(d,w)=self.base.take_inverse_quote()?;Ok((nested(d,2*size_of::<Option<S::Inverse>>()+1)?,w))}else{self.base.inverse_quote(b,m)},
  1=>Ok((nested(S::inverse_item_demand(self.bundle.get().ok_or_else(invalid)?,self.ordinal,b.saturating_sub(1))?,2*size_of::<Option<S::InverseItem>>()+1)?,1)),
  _=>Err(invalid())}}
 fn prepare_inverse_reply<'r>(&mut self,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  if self.phase==0&&!RowFieldPreparation::inverse_ready(&self.base){return self.base.prepare_inverse_reply(b,m,e,r,recover)}
  let q=if recover{None}else{Some(RowFieldPreparation::inverse_quote(self,b,m))};
  custody::gated(e,r,q,m,|effect,_|{let p=match self.phase{
   0=>{let extra=2*size_of::<Option<S::Inverse>>()+1;let g=effect.grant();let(bundle,p)=self.base.take_inverse(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.bundle.set(bundle);self.phase=1;add(p,extra)?},
   1=>{let extra=2*size_of::<Option<S::InverseItem>>()+1;let g=effect.grant();let(item,p)=S::take_inverse_item(self.bundle.get_mut().ok_or_else(invalid)?,self.ordinal,b.saturating_sub(1),child(g,extra))?.ok_or_else(invalid)?;self.item.set(item);self.phase=2;add(p,extra)?},
   _=>return Err(invalid().into())};
   Ok(if self.phase==2&&self.item.is_some(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})
 }
 fn inverse_ready(&self)->bool{!self.closing&&self.phase==2&&self.item.is_some()}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{if !RowFieldPreparation::inverse_ready(self){return Err(invalid());}Ok((scalar(2*size_of::<Option<S::InverseItem>>()+2*size_of::<usize>()+1),1))}
 fn take_inverse(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Inverse,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_inverse_quote()?;if !retained_mutation_demand_fits(d,e.grant()){return Ok(None);}let ordinal=self.ordinal.checked_add(1).ok_or_else(invalid)?;let item=self.item.take().ok_or_else(invalid)?;self.ordinal=ordinal;self.phase=1;Ok(Some((item,receipt(d))))}
}
