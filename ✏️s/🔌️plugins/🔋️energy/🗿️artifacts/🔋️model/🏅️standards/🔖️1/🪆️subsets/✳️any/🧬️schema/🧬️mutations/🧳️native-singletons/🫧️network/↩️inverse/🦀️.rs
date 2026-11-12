//! ↩️ The displaced original singleton issues one genuine source while its pair and link arrays become the exact authored inverse.
use super::*;
impl<S:NetworkPath> NetworkPreparation<S>{
 fn old(&self)->Result<Option<&Network>,ValueError>{Ok(self.old_source.get().ok_or_else(invalid)?.borrow().get().as_ref().map(SealedShared::get))}
 fn old_links_of(source:&Sourced<OptionalNetwork>)->Result<RetainedCloneRef<'_,Vec<u32>>,ValueError>{source.get().ok_or_else(invalid)?.borrow().try_project(0,|v|v.as_ref().ok_or_else(invalid).map(|network|network.get().link_ids.get()))}
 fn inverse_phase_quote(&self,b:usize,max:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase!=100||self.inverse_phase>=9||self.body.is_some_and(|v|v!=b)||!self.flight.idle(){return Err(invalid())}let nb=b.saturating_sub(1);
  match self.inverse_phase{
   0=>{let extra=2*size_of::<Option<OptionalNetwork>>()+2*size_of::<Option<semio_framework_value::retained_clone::RetainedCloneSource<OptionalNetwork>>>()+2*size_of::<Option<StagedSourceAdmission<OptionalNetwork,OptionalNetwork>>>()+1;match self.old_admit.get(){Some(a)=>{let d=a.next_demand()?;Ok((RetirementDemand{copy_bytes:d.copy_bytes+extra,depth:d.depth+1,..d},a.next_work_demand()?))},None=>Ok((RetirementDemand{copy_bytes:2*size_of::<StagedSourceAdmission<OptionalNetwork,OptionalNetwork>>()+extra,depth:2,..Default::default()},1))}},
   1=>Ok((scalar(2*size_of::<Option<NetworkInverse>>()+2*size_of::<usize>()+1),1)),
   2|3=>Ok((RetirementDemand{copy_bytes:2*size_of::<Vec<u32>>()+1,capacity_bytes:self.old()?.map_or(0,|v|v.zone_node_ids.get().len()).checked_mul(size_of::<u32>()).ok_or_else(invalid)?,depth:1,..Default::default()},1)),
   4=>Ok((scalar(2*size_of::<Pair>()+4*size_of::<u32>()+2*size_of::<usize>()+1),1)),
   5=>if self.old()?.is_some(){Ok((birth::<RetainedFieldPreparation<Vec<u32>>>(),1))}else{Ok((scalar(1),1))},
   6=>{let s=self.inverse_cursor.as_ref().ok_or_else(invalid)?;let l=Self::old_links_of(&self.old_source)?;Ok((nested(s.demand(l,nb,max)?,1)?,s.advance_work_demand(l,nb,max)?))},
   7=>Ok((nested(self.inverse_cursor.as_ref().ok_or_else(invalid)?.exchange_demand()?,2*size_of::<Vec<u32>>()+1)?,1)),
   8=>Ok((nested(Cursor::demand(&self.inverse_cursor,b)?,0)?,Cursor::work(&self.inverse_cursor,b)?)),
   _=>Err(invalid()),
  }
 }
 fn prepare_inverse_phase<'r>(&mut self,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let quote=if recover{None}else{Some(self.inverse_phase_quote(body,max))};let g=effect.grant();let nb=body.saturating_sub(1);
  custody::gated(effect,reply,quote,max,|effect,work|{
   let mut p=RetainedCloneProgress::default();
   match self.inverse_phase{
    0=>{let extra=2*size_of::<Option<OptionalNetwork>>()+2*size_of::<Option<semio_framework_value::retained_clone::RetainedCloneSource<OptionalNetwork>>>()+2*size_of::<Option<StagedSourceAdmission<OptionalNetwork,OptionalNetwork>>>()+1;
     if self.old_admit.is_none()&&self.old_source.get().is_none(){let old=self.old.take().ok_or_else(invalid)?;self.old_admit.set(StagedSourceAdmission::owned(old));p=receipt(scalar(extra));}
     else{let a=self.old_admit.get_mut().ok_or_else(invalid)?;let turn=if a.is_projecting(){a.project_owned(effect)}else{a.advance(self.old_source.slot(),effect)};*work+=turn.work;p=add(turn.result?.progress(),extra)?;if a.terminal_is_empty(){drop(self.old_admit.take());}}
     if self.old_source.get().is_some(){self.inverse_phase=1;}},
    1=>{let old=self.old()?;let inverse=NetworkInverse{present:old.is_some(),zone_ids:Vec::new(),node_ids:Vec::new(),outdoor_node_id:old.map_or(0,|v|v.outdoor_node_id),link_ids:Vec::new()};self.inverse.set(inverse);self.inverse_index=0;self.inverse_phase=2;p=receipt(scalar(1));},
    2|3=>{let n=self.old()?.map_or(0,|v|v.zone_node_ids.get().len());let(value,bytes)=custody::exact_vec::<u32>(n,effect)?;p=receipt(scalar(1));p.retained_capacity_bytes=bytes;let inverse=self.inverse.get_mut().ok_or_else(invalid)?;if self.inverse_phase==2{inverse.zone_ids=value;self.inverse_phase=3;}else{inverse.node_ids=value;self.inverse_phase=4;}},
    4=>{p=receipt(scalar(1));let old=self.old()?;let n=old.map_or(0,|v|v.zone_node_ids.get().len());if self.inverse_index==n{self.inverse_phase=5;}else{let pair=old.ok_or_else(invalid)?.zone_node_ids.get()[self.inverse_index];let inverse=self.inverse.get_mut().ok_or_else(invalid)?;inverse.zone_ids.push(pair.0.0);inverse.node_ids.push(pair.1);self.inverse_index+=1;}},
    5=>{p=receipt(scalar(1));if self.old()?.is_some(){self.inverse_cursor=Some(Box::new(Default::default()));self.inverse_phase=6;}else{self.inverse_phase=8;}},
    6=>{let Self{inverse_cursor,old_source,flight,inverse_phase,..}=self;let original=Self::old_links_of(old_source)?;let c=inverse_cursor.as_deref_mut().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(*flight,effect,recover,1,body,max,None,*work,|r| c.advance_into(original,nb,max,&mut effect.nested(child(g,1)),r),c.recover_advance_into(original,nb,max,&mut effect.nested(child(g,1)),r));
     p=add(step?.progress(),1)?;if c.value().is_some(){*inverse_phase=7;}},
    7=>{let extra=2*size_of::<Vec<u32>>()+1;let Self{inverse_cursor,inverse_links,inverse,..}=self;let c=inverse_cursor.as_deref_mut().ok_or_else(invalid)?;let slot=inverse_links.slot();let step=native_child(effect,work,|e,r|c.take_into(slot,&mut e.nested(child(g,extra)),r))??;inverse.get_mut().ok_or_else(invalid)?.link_ids=inverse_links.take().ok_or_else(invalid)?;p=add(step.progress(),extra)?;self.inverse_phase=8;},
    8=>{let step=custody::finish(&mut self.flight,&mut self.inverse_cursor,body,effect,work,recover)?;p=step.progress();if self.inverse_cursor.is_none(){self.inverse_phase=9;}},
    _=>return Err(invalid().into()),
   }
   Ok(if self.inverse_phase==9{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })
 }
}
impl<S:NetworkPath> RowFieldPreparation for NetworkPreparation<S>{
 type Input=S::Payload;type Inverse=NetworkInverse;
 fn new()->Self{Self{authored:Default::default(),original:Default::default(),body:None,phase:0,index:0,outcome:None,closing:false,flight:Default::default(),raw_pairs:Default::default(),pairs:Default::default(),links:Default::default(),raw_network:Default::default(),network:Default::default(),value:Default::default(),old:Default::default(),decision:Default::default(),link_cursor:None,staged:Default::default(),output:None,old_source:Default::default(),old_admit:Default::default(),inverse:Default::default(),inverse_cursor:None,inverse_links:Default::default(),inverse_phase:0,inverse_index:0,marker:Default::default()}}
 fn check(&self,p:RetainedCloneRef<'_,Self::Input>,key:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{if key.is_some(){return Err(invalid())}self.check_sources(p,o)}
 fn quote(&self,p:RetainedCloneRef<'_,Self::Input>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64)->Result<Quote,ValueError>{Self::quote(self,p,o,b,m)}
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,Self::Input>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{Self::advance_reply(self,p,o,b,m,e,r,recover)}
 fn ready(&self)->bool{!self.closing&&self.phase==99}fn outcome(&self)->Option<RowOutcome>{self.outcome}
 fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid())}Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None)}if !effect.performed(1)?{return Err(invalid())}self.phase=100;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 fn inverse_quote(&self,b:usize,m:u64)->Result<Quote,ValueError>{self.inverse_phase_quote(b,m)}
 fn prepare_inverse_reply<'r>(&mut self,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.prepare_inverse_phase(b,m,e,r,recover)}
 fn inverse_ready(&self)->bool{!self.closing&&self.phase==100&&self.inverse_phase==9&&self.inverse.is_some()}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{if !self.inverse_ready(){return Err(invalid())}Ok((scalar(2*size_of::<Option<NetworkInverse>>()+1),1))}
 fn take_inverse(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(NetworkInverse,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_inverse_quote()?;if !retained_mutation_demand_fits(d,effect.grant()){return Ok(None)}self.phase=101;Ok(Some((self.inverse.take().ok_or_else(invalid)?,receipt(d))))}
}
