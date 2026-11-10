//! 🧵️ Original Host source, builtin seed, snapshot, and channel evaluation continuation.
use super::*;
use super::seeds::FlowEvaluationSeeds;
use semio_framework_job::{StepContext,OriginalTurnFailure};
use neural::OperatorWorkFailure;
use semio_framework_value_ordered::ordered::SharedOwner;
use semio_framework_value::retirement::allocation_return::original::SharedOriginalRelease;

fn retire_alias<T:Send+Sync+'static>(alias:SharedOwner<T>){match alias.retire(){SharedOriginalRelease::Alias=>{},SharedOriginalRelease::Original(original)=>original.abandon()}}
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct WorkSources{source:Option<FlowEvaluationSource>,seeds_source:Option<FlowEvaluationSeeds>,tree:Option<Tree>,shared_tree:Option<SharedOwner<Tree>>,seeds:Option<HistoryFoldIndex<String,Dictionary>>,shared_seeds:Option<SharedOwner<HistoryFoldIndex<String,Dictionary>>>,infos:Option<SharedOwner<HistoryFoldIndex<String,OperatorInfo>>>,registry:Option<neural::SharedRegistry>,cache:Option<SharedOwner<neural::NeuralCache>>,previous:Option<SharedOwner<EvalChannels>>,snapshot:Option<TreeSnapshot>,evaluation:Option<neural::BudgetedEvalState>,channels:Option<EvalChannels>,remaining:Vec<String>,pending:Vec<neural::PendingExtensionEval>,fault:Option<OperatorWorkFailure>}
#[derive(semio_framework_value::RetireOwned)]
pub struct FlowEvaluationCompletion{pub snapshot:TreeSnapshot,pub channels:EvalChannels,pub remaining:Vec<String>,pub pending_extensions:Vec<neural::PendingExtensionEval>,pub generation:u64}
/// 🔬️ Finite pointer observations expose no mutable owner or replacement authority.
#[cfg(any(test,feature="testkit"))]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct FlowEvaluationWorkDiagnostic{pub phase:u8,pub tree:Option<usize>,pub seeds:Option<usize>,pub snapshot_tree:Option<usize>,pub snapshot_seeds:Option<usize>}
/// 🧵️ Owns the original source continuation through every accepted normal and closing event.
pub struct FlowEvaluationWork{sources:ManuallyDrop<Option<WorkSources>>,retirement:Option<ControlledRetirement<WorkSources>>,revision:u64,generation:u64,origin:Option<semio_framework_job::OriginalSourceBinding>,pub(super) phase:u8,closing:bool,progress:RetainedCloneProgress,last_work:u64}
impl FlowEvaluationWork{
 pub fn new(infos:SharedOwner<HistoryFoldIndex<String,OperatorInfo>>,registry:neural::SharedRegistry,previous:Option<SharedOwner<EvalChannels>>,revision:u64,generation:u64)->Self{Self{sources:ManuallyDrop::new(Some(WorkSources{source:Some(FlowEvaluationSource::new()),seeds_source:Some(FlowEvaluationSeeds::new()),tree:None,shared_tree:None,seeds:None,shared_seeds:None,infos:Some(infos),registry:Some(registry),cache:None,previous,snapshot:None,evaluation:None,channels:None,remaining:Vec::new(),pending:Vec::new(),fault:None})),retirement:None,revision,generation,origin:None,phase:0,closing:false,progress:Default::default(),last_work:0}}
 /// 🔗️ Moves the genuine preborn reply cache lease into this original evaluation.
 pub fn with_cache(mut self,cache:SharedOwner<neural::NeuralCache>)->Self{self.sources.as_mut().unwrap().cache=Some(cache);self}
 #[cfg(any(test,feature="testkit"))]
 #[doc(hidden)]
 pub fn test_diagnostic(&self)->FlowEvaluationWorkDiagnostic{let fields=self.sources.as_ref();FlowEvaluationWorkDiagnostic{phase:self.phase,tree:fields.and_then(|fields|fields.shared_tree.as_ref().map(|tree|tree.identity())),seeds:fields.and_then(|fields|fields.shared_seeds.as_ref().map(|seeds|seeds.identity())),snapshot_tree:fields.and_then(|fields|fields.snapshot.as_ref().map(|snapshot|snapshot.tree_source()as*const Tree as usize)),snapshot_seeds:fields.and_then(|fields|fields.snapshot.as_ref().map(|snapshot|snapshot.seed_source()as*const HistoryFoldIndex<String,Dictionary> as usize))}}
 pub fn output_was_transferred(&self)->bool{self.phase==255}
 pub fn restore_completion(&mut self,output:FlowEvaluationCompletion){let fields=self.sources.as_mut().unwrap();assert!(fields.snapshot.is_none()&&fields.channels.is_none());fields.snapshot=Some(output.snapshot);fields.channels=Some(output.channels);fields.remaining=output.remaining;fields.pending=output.pending_extensions;self.phase=10;}
 pub fn source_matches(&self,revision:u64,generation:u64)->bool{self.revision==revision&&self.generation==generation}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.last_work}
 pub fn is_closing(&self)->bool{self.closing}
 pub fn fault(&self)->Option<&OperatorWorkFailure>{self.sources.as_ref()?.fault.as_ref()}
 pub fn next_demands(&self,source:&dyn FlowSnapshotSource,copy:usize)->Result<RetirementDemand,ValueError>{if self.closing{return self.next_close_demands(copy)}if self.origin.is_none(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_job::OriginalSourceBinding>>(),depth:1,..Default::default()})}let fields=self.sources.as_ref().unwrap();let d=match self.phase{0=>fields.source.as_ref().unwrap().next_demands(source,fields.infos.as_ref().unwrap().get(),copy)?,1=>fields.source.as_ref().unwrap().next_close_demands(copy)?,2=>SharedOwner::<Tree>::source_admission_demand()?,3=>fields.seeds_source.as_ref().unwrap().next_demands(fields.shared_tree.as_ref().unwrap().get(),copy)?,4=>fields.seeds_source.as_ref().unwrap().next_close_demands(copy)?,5=>SharedOwner::<HistoryFoldIndex<String,Dictionary>>::source_admission_demand()?,8=>fields.evaluation.as_ref().unwrap().next_step_demands(copy)?,9=>fields.evaluation.as_ref().unwrap().next_close_demands(copy)?,_=>RetirementDemand{depth:1,..Default::default()}};Ok(d)}
 pub fn step(&mut self,source:&dyn FlowSnapshotSource,cx:&mut StepContext<'_>)->Result<Option<FlowEvaluationCompletion>,OperatorWorkFailure>{
  let start=cx.retained_progress();let start_work=cx.original_total_fuel_remaining();
  let result=(||{for _ in 0..512{let before=(cx.retained_progress(),cx.fuel_remaining());if let Some(output)=self.step_once(source,cx)?{return Ok(Some(output))}if cx.should_yield()||cx.is_cancelled()||(cx.retained_progress(),cx.fuel_remaining())==before{break}}Ok(None)})();
  let end=cx.retained_progress();self.progress=RetainedCloneProgress{copied_items:end.copied_items-start.copied_items,copied_bytes:end.copied_bytes-start.copied_bytes,retained_capacity_bytes:end.retained_capacity_bytes-start.retained_capacity_bytes,released_bytes:end.released_bytes-start.released_bytes};self.last_work=start_work-cx.original_total_fuel_remaining();result
 }
 fn step_once(&mut self,source:&dyn FlowSnapshotSource,cx:&mut StepContext<'_>)->Result<Option<FlowEvaluationCompletion>,OperatorWorkFailure>{
  self.progress=Default::default();if cx.is_cancelled()||cx.should_yield(){return Ok(None)}let grant=cx.retained_grant();let d=self.next_demands(source,grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes||grant.maximum_depth<d.depth{return Ok(None)}let origin=semio_framework_job::OriginalSourceBinding::borrow(cx);if self.origin.is_none(){self.origin=Some(origin);self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};cx.consume_fuel(1);cx.consume_retained(self.progress)?;return Ok(None)}if self.origin.is_some_and(|original|!original.matches(cx)){cx.consume_fuel(1);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original evaluation Work initiating operation changed").into())}self.origin=Some(origin);
  if self.phase==8{let fields=self.sources.as_mut().unwrap();let child=fields.evaluation.as_mut().unwrap();let result=child.step(cx);self.progress=child.normal_step_progress();match result{Ok(neural::BudgetedEvalStep::Working)=>{},Ok(neural::BudgetedEvalStep::Complete(output))=>{fields.channels=Some(output.channels);fields.remaining=output.remaining;fields.pending=output.pending_extensions;child.begin_close();self.phase=9;},Err(error)=>{self.begin_close();return Err(error)}}return Ok(None)}
  self.last_work=0;let outcome=cx.with_original_effect(RetainedCloneGrant{maximum_items:1,..grant},|_,effect|self.advance(source,effect)).map_err(OperatorWorkFailure::from)?;let Some((result,performed))=outcome else{return Ok(None)};let moved=self.progress!=Default::default()||result.is_err();if performed==0&&moved{cx.consume_fuel(1)}match result{Ok(output)=>{if let Err(error)=cx.consume_retained(self.progress){if let Some(output)=output{self.restore_completion(output)}return Err(OriginalTurnFailure::from(error).into())}Ok(output)},Err(error)=>Err(OriginalTurnFailure::debit_original(error.with_retained_progress(self.progress),cx).into())}
 }
 fn advance(&mut self,source:&dyn FlowSnapshotSource,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<FlowEvaluationCompletion>,ValueError>{let grant=effect.grant();if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original evaluation Work is closing"))}if grant.maximum_items==0{return Ok(None)}let d=self.next_demands(source,grant.maximum_copy_bytes)?;if !matches!(self.phase,0|1|3|4|8|9){if grant.maximum_depth<d.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original Work exceeds admitted depth"))}if grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes{return Ok(None)}}let fields=self.sources.as_mut().unwrap();self.progress.copied_items=1;match self.phase{
  0=>{let child=fields.source.as_mut().unwrap();let result=child.step(source,fields.infos.as_ref().unwrap().get(),effect);self.progress=child.step_progress();if let Some(tree)=result?{fields.tree=Some(tree);child.begin_close();self.phase=1;}},
  1=>{let child=fields.source.as_mut().unwrap();let poll=child.close_step(grant.maximum_copy_bytes,effect);self.progress=child.step_progress();if let RetainedWorkPoll::Complete(turn)=poll{turn.result.map_err(|failure|failure.into_value_error())?;}else{return Ok(None)}if child.terminal_is_empty(){fields.source=None;self.phase=2;}},
  2=>{let mut tree=fields.tree.take();let admitted=SharedOwner::<Tree>::admit_from(&mut tree,&mut fields.shared_tree,effect);fields.tree=tree;match admitted.result{Ok(step)=>{self.progress=step.progress();if fields.shared_tree.is_some(){self.phase=3}},Err(error)=>{self.progress=error.retained_progress();return Err(error)}}},
  3=>{let child=fields.seeds_source.as_mut().unwrap();let result=child.step(fields.shared_tree.as_ref().unwrap().get(),effect);self.progress=child.step_progress();if let Some(seeds)=result?{fields.seeds=Some(seeds);child.begin_close();self.phase=4;}},
  4=>{let child=fields.seeds_source.as_mut().unwrap();let poll=child.close_step(grant.maximum_copy_bytes,effect);self.progress=child.step_progress();if let RetainedWorkPoll::Complete(turn)=poll{turn.result.map_err(|failure|failure.into_value_error())?;}else{return Ok(None)}if child.terminal_is_empty(){fields.seeds_source=None;self.phase=5;}},
  5=>{let mut seeds=fields.seeds.take();let admitted=SharedOwner::<HistoryFoldIndex<String,Dictionary>>::admit_from(&mut seeds,&mut fields.shared_seeds,effect);fields.seeds=seeds;match admitted.result{Ok(step)=>{self.progress=step.progress();if fields.shared_seeds.is_some(){self.phase=6}},Err(error)=>{self.progress=error.retained_progress();return Err(error)}}},
  6=>{match TreeSnapshot::capture_retained(fields.shared_tree.as_ref().unwrap().alias(),fields.shared_seeds.as_ref().unwrap().alias(),grant){Ok((snapshot,p))=>{fields.snapshot=Some(snapshot);self.progress=p;self.phase=7},Err((_error,tree,seeds))=>{retire_alias(tree);retire_alias(seeds);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original preflighted snapshot handoff refused"))}}},
  7=>{fields.evaluation=Some(neural::BudgetedEvalState::new(fields.shared_tree.take().unwrap(),fields.shared_seeds.take().unwrap(),HistoryFoldIndex::new(),fields.infos.take().unwrap(),fields.registry.take().unwrap(),fields.previous.take()));if let Some(cache)=fields.cache.take(){let evaluation=fields.evaluation.take().unwrap();fields.evaluation=Some(evaluation.with_cache(cache));}self.phase=8;},
  8=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original evaluation Work requires its initiating context")), 
  9=>{let child=fields.evaluation.as_mut().unwrap();let poll=child.close_step(grant.maximum_copy_bytes,effect);self.progress=child.normal_step_progress();if let RetainedWorkPoll::Complete(turn)=poll{turn.result.map_err(|failure|failure.into_value_error())?;}else{return Ok(None)}if child.terminal_is_empty(){fields.evaluation=None;self.phase=10;}},
  10=>{self.phase=255;return Ok(Some(FlowEvaluationCompletion{snapshot:fields.snapshot.take().unwrap(),channels:fields.channels.take().unwrap(),remaining:std::mem::take(&mut fields.remaining),pending_extensions:std::mem::take(&mut fields.pending),generation:self.generation}))},
  _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Work output was already transferred")),
 }Ok(None)}
 pub fn begin_close(&mut self){self.closing=true;if let Some(fields)=self.sources.as_mut(){if let Some(child)=fields.source.as_mut(){child.begin_close()}if let Some(child)=fields.seeds_source.as_mut(){child.begin_close()}if let Some(child)=fields.evaluation.as_mut(){child.begin_close()}}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.retirement.as_ref(){return FlowEvaluationSource::demand(owner,copy)}Ok(RetirementDemand{depth:usize::from(self.sources.is_some()),..Default::default()})}
 pub fn next_close_work(&self,body:usize)->Result<u64,ValueError>{if let Some(owner)=self.retirement.as_ref(){return owner.next_computational_work(body)}Ok(u64::from(self.sources.is_some()))}
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  self.progress=Default::default();let grant=effect.grant();
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Work close was not requested").into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
   if let Some(owner)=self.retirement.as_mut(){let result=RetirementPollExit::resume(owner.step(body,effect));self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.retirement=None}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
   if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original Work closure requires admitted depth").into())}
   if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Work handoff exceeds its work ceiling").into())}
   let sources=self.sources.take().ok_or_else(||ValueError::invariant("original Work sources absent"))?;self.retirement=Some(ControlledRetirement::retain(sources));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))
  })();
  settle_close(effect,result,&mut self.progress)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.sources.is_none()&&self.retirement.is_none()}
}
impl Drop for FlowEvaluationWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original evaluation Work must reach explicit terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.sources)}}}}
/// 🧹️ Owns a closing evaluation Work as its retirement cursor.
pub struct WorkRetirement(FlowEvaluationWork);
impl RetirementCursor for WorkRetirement{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.0.next_close_demands(body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.0.next_close_work(body)}
 fn close_into<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:CloseReply<'reply>)->Result<CloseComplete<'reply>,ValueError>{close_reply(self.0.close_step(body,effect),effect,reply)}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.next_close_demands(copy).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_demands(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl RetireOwned for FlowEvaluationWork{type RetirementCursor=WorkRetirement;fn retirement_seed(mut self)->WorkRetirement{self.begin_close();WorkRetirement(self)}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<WorkRetirement>())}fn controlled_retirement_supported()->bool{true}}
