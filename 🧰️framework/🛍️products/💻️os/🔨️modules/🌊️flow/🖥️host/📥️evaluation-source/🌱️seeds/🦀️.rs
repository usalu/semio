//! 🌱️ Original builtin seed dictionaries retain exact projected parameter value leases.
use super::*;
use semio_framework_value_ordered::ordered::SharedOwner;

#[derive(semio_framework_value::RetireOwned)]
enum SeedDisplaced{Dictionary(Dictionary)}
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct SeedOwners{output:Option<HistoryFoldIndex<String,Dictionary>>,inner:Option<Dictionary>,outer:Option<Dictionary>,key:Option<String>,value:Option<NeuralValue>,shared:Option<SharedOwner<NeuralValue>>,entry:Option<BudgetedOwnedEntry>,buffer:Vec<u8>,displaced:Option<ControlledRetirement<SeedDisplaced>>}
/// 🌱️ Advances the original projected tree's builtin values under each incoming full grant.
pub struct FlowEvaluationSeeds{owners:ManuallyDrop<Option<SeedOwners>>,retirement:Option<ControlledRetirement<SeedOwners>>,source:usize,node:usize,param:usize,offset:usize,kind:u8,pub(super) phase:u8,closing:bool,progress:RetainedCloneProgress}
impl FlowEvaluationSeeds{
 pub fn new()->Self{Self{owners:ManuallyDrop::new(Some(SeedOwners{output:Some(HistoryFoldIndex::new()),inner:None,outer:None,key:None,value:None,shared:None,entry:None,buffer:Vec::new(),displaced:None})),retirement:None,source:0,node:0,param:0,offset:0,kind:0,phase:0,closing:false,progress:Default::default()}}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 fn kind(&self)->&'static str{match self.kind{0=>"number",1=>"text",2=>"image",_=>unreachable!()}}
 fn value_key(&self)->&'static str{if self.kind==2{"dataUrl"}else{"value"}}
 fn check(&self,tree:&Tree)->Result<(),ValueError>{if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original builtin seed source is closing"))}if self.source!=0&&self.source!=tree.neurons.as_ptr()as usize{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original builtin seed tree owner changed"))}Ok(())}
 fn text<'a>(&'a self,tree:&'a Tree)->Option<&'a str>{match self.phase{2=>Some("schema"),3|11=>Some(self.kind()),7=>Some(self.value_key()),15=>Some(&tree.neurons[self.node].id),_=>None}}
 pub fn next_demands(&self,tree:&Tree,copy:usize)->Result<RetirementDemand,ValueError>{self.check(tree)?;let owners=self.owners.as_ref().unwrap();if let Some(owner)=owners.displaced.as_ref(){return FlowEvaluationSource::demand(owner,copy)}if matches!(self.phase,5|9|13){let mut d=owners.entry.as_ref().unwrap().next_demands(copy)?;d.depth+=1;return Ok(d)}if matches!(self.phase,6|10|14){let mut d=owners.entry.as_ref().unwrap().next_close_demands(copy)?;d.depth+=1;return Ok(d)}let mut d=RetirementDemand{depth:1,..Default::default()};if let Some(text)=self.text(tree){d.copy_bytes=if owners.buffer.capacity()>=text.len(){text.len().saturating_sub(owners.buffer.len()).min(copy)}else{0};d.capacity_bytes=if owners.buffer.capacity()<text.len(){text.len()}else{0};}if self.phase==1{d.copy_bytes=tree.neurons[self.node].params.entry_at_rank(self.param).map_or(0,|(key,_)|usize::from(self.offset<key.len().min(self.value_key().len())));}if self.phase==16{let index=owners.output.as_ref().unwrap();let key=owners.key.as_ref().unwrap();d=RetirementDemand{copy_bytes:index.next_insert_copy_byte_demand(key)?,capacity_bytes:index.next_insert_capacity_byte_demand(key,copy)?,release_bytes:index.next_insert_release_byte_demand(key)?,depth:index.next_insert_depth_demand(key)?};}Ok(d)}
 pub fn step(&mut self,tree:&Tree,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<HistoryFoldIndex<String,Dictionary>>,ValueError>{self.progress=Default::default();let result=self.advance(tree,effect);result.map_err(|error|error.with_retained_progress(self.progress))}
 fn advance(&mut self,tree:&Tree,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<HistoryFoldIndex<String,Dictionary>>,ValueError>{let grant=effect.grant();self.check(tree)?;if grant.maximum_items==0{return Ok(None)}let d=self.next_demands(tree,grant.maximum_copy_bytes)?;if grant.maximum_depth<d.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original builtin seed source exceeds admitted depth"))}if !matches!(self.phase,5|6|9|10|13|14)&&(grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes){return Ok(None)}if !(matches!(self.phase,5|6|9|10|13|14)||self.owners.as_ref().unwrap().displaced.is_some())&&!effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original builtin seed event exceeds its work ceiling"))}self.source=tree.neurons.as_ptr()as usize;
  if let Some(owner)=self.owners.as_mut().unwrap().displaced.as_mut(){let poll=owner.step(grant.maximum_copy_bytes,effect);self.progress=owner.step_progress();if let RetainedWorkPoll::Complete(turn)=poll{turn.result.map_err(|failure|failure.into_value_error())?;}else{return Ok(None)}if owner.terminal_is_empty(){self.owners.as_mut().unwrap().displaced=None}return Ok(None)}
  if let Some(text)=self.text(tree){let len=text.len();let raw=text.as_ptr();let owners=self.owners.as_mut().unwrap();if owners.buffer.capacity()<len{owners.buffer.try_reserve_exact(len).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original builtin seed string reservation refused"))?;self.progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:owners.buffer.capacity(),..Default::default()};}else if owners.buffer.len()<len{let offset=owners.buffer.len();let count=(len-offset).min(grant.maximum_copy_bytes);unsafe{owners.buffer.extend_from_slice(std::slice::from_raw_parts(raw.add(offset),count))}self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()};}else{let text=unsafe{String::from_utf8_unchecked(std::mem::take(&mut owners.buffer))};self.progress.copied_items=1;if self.phase==3{owners.value=Some(NeuralValue::Atom(Atom::String(text)))}else{owners.key=Some(text)}self.phase+=1;}return Ok(None)}
  self.progress.copied_items=1;match self.phase{
   0=>{if self.node==tree.neurons.len(){self.phase=255}else{self.kind=match tree.neurons[self.node].kind.as_str(){"core.number"=>0,"core.text"=>1,"core.image"=>2,_=>{self.node+=1;return Ok(None)}};self.param=0;self.offset=0;self.phase=1}},
   1=>{let Some((key,_))=tree.neurons[self.node].params.entry_at_rank(self.param)else{self.node+=1;self.phase=0;return Ok(None)};let expected=self.value_key();let length=key.len().min(expected.len());let count=(length-self.offset).min(grant.maximum_copy_bytes);let same=key.as_bytes()[self.offset..self.offset+count]==expected.as_bytes()[self.offset..self.offset+count];self.offset+=count;self.progress.copied_bytes=count;if !same||self.offset==length{if same&&key.len()==expected.len(){self.owners.as_mut().unwrap().shared=tree.neurons[self.node].params.value_shared_at_rank(self.param);self.phase=2;}else{self.param+=1;}self.offset=0;}},
   4=>{let owners=self.owners.as_mut().unwrap();owners.entry=Some(BudgetedOwnedEntry::new(Dictionary::new(),owners.key.take().unwrap(),owners.value.take().unwrap()));self.phase=5},
   5|9|13=>{let owners=self.owners.as_mut().unwrap();let child=owners.entry.as_mut().unwrap();let poll=child.step(grant.maximum_copy_bytes,&mut effect.nested(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}));self.progress=child.step_progress();let result=match poll{RetainedWorkPoll::Pending{..}=>return Ok(None),RetainedWorkPoll::Complete(turn)=>turn.result.map_err(|failure|failure.into_value_error())?};if let Some(output)=result{if self.phase==13{owners.outer=Some(output)}else{owners.inner=Some(output)}self.phase+=1;}},
   6|10|14=>{let owners=self.owners.as_mut().unwrap();let child=owners.entry.as_mut().unwrap();let poll=child.close_step(grant.maximum_copy_bytes,&mut effect.nested(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}));self.progress=child.step_progress();if let RetainedWorkPoll::Complete(turn)=poll{turn.result.map_err(|failure|failure.into_value_error())?;}else{return Ok(None)}if child.terminal_is_empty(){owners.entry=None;self.phase+=1;}},
   8=>{let owners=self.owners.as_mut().unwrap();owners.entry=Some(BudgetedOwnedEntry::from_shared_value(owners.inner.take().unwrap(),owners.key.take().unwrap(),owners.shared.take().unwrap()));self.phase=9},
   12=>{let owners=self.owners.as_mut().unwrap();owners.entry=Some(BudgetedOwnedEntry::new(Dictionary::new(),owners.key.take().unwrap(),NeuralValue::Dictionary(owners.inner.take().unwrap())));self.phase=13},
   16=>{let owners=self.owners.as_mut().unwrap();let index=owners.output.as_mut().unwrap();let key=owners.key.as_ref().unwrap();if index.next_insert_capacity_byte_demand(key,grant.maximum_copy_bytes)?!=0{match index.reserve_insert_step(key,grant){Ok(p)=>self.progress=p,Err((error,p))=>{self.progress=p;return Err(error)}}}else{match index.insert_reserved(owners.key.take().unwrap(),owners.outer.take().unwrap(),grant){Ok((previous,p))=>{self.progress=p;if let Some(previous)=previous{owners.displaced=Some(ControlledRetirement::retain(SeedDisplaced::Dictionary(previous)))}self.node+=1;self.phase=0},Err((error,key,value))=>{owners.key=Some(key);owners.outer=Some(value);return Err(error)}}}},
   255=>return Ok(self.owners.as_mut().unwrap().output.take()),
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original builtin seed phase is invalid")),
  }Ok(None)
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.retirement.as_ref(){return FlowEvaluationSource::demand(owner,copy)}Ok(RetirementDemand{depth:usize::from(self.owners.is_some()),..Default::default()})}
 pub fn next_close_work(&self,body:usize)->Result<u64,ValueError>{if let Some(owner)=self.retirement.as_ref(){return owner.next_computational_work(body)}Ok(u64::from(self.owners.is_some()))}
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  self.progress=Default::default();let grant=effect.grant();
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original builtin seed closure was not requested").into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
   if let Some(owner)=self.retirement.as_mut(){let result=RetirementPollExit::resume(owner.step(body,effect));self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.retirement=None}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
   if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original builtin seed closure requires admitted depth").into())}
   if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original builtin seed closure exceeds its work ceiling").into())}
   let owners=self.owners.take().ok_or_else(||ValueError::invariant("original builtin seed owners absent"))?;self.retirement=Some(ControlledRetirement::retain(owners));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))
  })();
  settle_close(effect,result,&mut self.progress)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.owners.is_none()&&self.retirement.is_none()}
}
impl Drop for FlowEvaluationSeeds{fn drop(&mut self){assert!(self.terminal_is_empty(),"original builtin seed must finish explicit closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owners)}}}}
/// 🧹️ Owns a closing builtin seed projection as its retirement cursor.
pub struct SeedRetirement(FlowEvaluationSeeds);
impl RetirementCursor for SeedRetirement{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.0.next_close_demands(body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.0.next_close_work(body)}
 fn close_into<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:CloseReply<'reply>)->Result<CloseComplete<'reply>,ValueError>{close_reply(self.0.close_step(body,effect),effect,reply)}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.next_close_demands(copy).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl RetireOwned for FlowEvaluationSeeds{
 type RetirementCursor=SeedRetirement;
 fn retirement_seed(mut self)->SeedRetirement{self.begin_close();SeedRetirement(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SeedRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
