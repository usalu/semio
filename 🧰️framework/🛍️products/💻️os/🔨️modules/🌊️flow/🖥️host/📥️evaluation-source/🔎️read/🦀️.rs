//! 🔎️ Current successful Flow outputs are admitted against the original funded source and baseline owners.
use neural_engine as neural;
use neural::{Atom, Dictionary, EvalChannels, Neuron, OperatorInfo, Tree, TreeSnapshot, Value as NeuralValue};
use protocol::causal::transition::HistoryFoldIndex;
use semio_framework_artifact_flow_flow::{artifact::*, widget_id_for};
use semio_framework_os_flow_host_core::*;
use semio_framework_os_flow_host_source::{seeds::FlowEvaluationSeeds, FlowEvaluationSource};
use semio_framework_os_flow_operators::catalogue::flow_neuron_kind_info_map;
use semio_framework_os_flow_operators::registry::flow_extension_registry_generation;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::controlled::ControlledRetirement};
use std::sync::Arc;
use semio_framework_job::StepContext;
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep,arc_custody::{SharedControlledRetirement,shared_retirement_birth_bytes}};
use std::mem::{ManuallyDrop,size_of};
/// 🧭️ The session baseline custody a funded current-source read inspects; the evaluation session is its one implementor.
pub trait FlowSourceBaseline {
    fn previous_snapshot(&self) -> Option<&Arc<TreeSnapshot>>;
    fn previous_channels(&self) -> Option<&Arc<EvalChannels>>;
    fn baseline_geometry_pending(&self) -> bool;
    fn preview_cancelled(&self) -> bool;
    fn pending_host(&self) -> Option<&FlowHost>;
    fn flow_extension_generation(&self) -> u64;
}

#[path="⚖️compare/🦀️.rs"]
mod compare;
use compare::SourceCompare;
#[path="📨️request/🦀️.rs"]
pub mod request;
pub use request::{FlowPreviewTessellationRequest,FlowPreviewTessellationWriter};

pub const CURRENT_SOURCE_CHECK_BYTES:usize=size_of::<[usize;8]>()+size_of::<[usize;4]>()+size_of::<(u64,u64)>()+size_of::<[u8;32]>()+size_of::<u64>()+2*size_of::<usize>();

pub fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
pub fn admitted(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items!=0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}
struct ReadLease<T:RetireOwned+Sync>(Arc<T>);
impl<T:RetireOwned+Sync> RetireOwned for ReadLease<T>{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(SharedControlledRetirement::lease(self.0))}fn retirement_birth_bytes(&self)->Option<usize>{Some(shared_retirement_birth_bytes::<T>())}fn controlled_retirement_supported()->bool{true}}

fn compare_text(offset:&mut usize,left:&str,right:&str,progress:&mut RetainedCloneProgress,grant:RetainedCloneGrant)->Option<bool>{
 if left.len()!=right.len(){*offset=0;progress.copied_bytes=0;return Some(false)}let count=(left.len()-*offset).min(grant.maximum_copy_bytes);let equal=left.as_bytes()[*offset..*offset+count]==right.as_bytes()[*offset..*offset+count];progress.copied_bytes=count;*offset+=count;if !equal||*offset==left.len(){*offset=0;Some(equal)}else{None}
}
/// 🧯️ Read admission never creates geometry or mutates a failed source.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum FlowSourceReadFailure{Origin,Revision,Registry,Pending,MissingBaseline,ChangedSource,MissingWidget,MissingPort,IncompatiblePort,MissingOutput,FailedOutput}
/// 📥️ The command's original payload names the actual registry endpoint.
#[derive(Clone,Copy)]
pub enum FlowSourceReadTarget<'a>{Port{kind:&'a str,port:&'a str},Preview}
/// 📥️ A source output is admitted for one registered input or the code-owned preview consumer.
pub struct FlowSourceReadRequest<'a>{pub source_widget:&'a str,pub source_port:&'a str,pub target:FlowSourceReadTarget<'a>}
impl FlowSourceReadRequest<'_>{
 pub fn target_port(&self)->Option<&str>{match self.target{FlowSourceReadTarget::Port{port,..}=>Some(port),FlowSourceReadTarget::Preview=>None}}
 fn original_binding(&self)->[usize;8]{let mut binding=[self.source_widget.as_ptr()as usize,self.source_widget.len(),self.source_port.as_ptr()as usize,self.source_port.len(),0,0,0,0];match self.target{FlowSourceReadTarget::Port{kind,port}=>binding[4..].copy_from_slice(&[kind.as_ptr()as usize,kind.len(),port.as_ptr()as usize,port.len()]),FlowSourceReadTarget::Preview=>binding[4]=usize::MAX}binding}
}
/// 🧊️ Exact successful identities stay borrowed from the admitted channel owner.
pub struct FlowCurrentOutput<'a>{identity:&'a str,geometry:bool,revision:[u8;32],binding:[usize;8],origin:(u64,u64)}
impl FlowCurrentOutput<'_>{pub fn geometry_handle(&self)->Option<&str>{self.geometry.then_some(self.identity)}pub fn preview_handle(&self)->Option<&str>{(self.geometry&&self.binding[4]==usize::MAX).then_some(self.identity)}pub fn indexed_mesh(&self)->Option<&str>{(!self.geometry).then_some(self.identity)}pub fn revision(&self)->[u8;32]{self.revision}pub fn admits(&self,request:&FlowSourceReadRequest<'_>,revision:[u8;32],operation:u64,generation:u64)->bool{self.binding==request.original_binding()&&self.revision==revision&&self.origin==(operation,generation)}}
/// 🪪️ A donated preview source keeps the original read and exact channel memory through asynchronous work.
#[derive(semio_framework_value::RetireOwned)]
pub struct FlowPreviewSource{read:FlowSourceRead,identity:semio_framework_value_ordered::ordered::SharedOwner<NeuralValue>}
impl FlowPreviewSource{
 pub fn original_handle(&self)->&str{self.identity.as_atom().and_then(Atom::as_str).unwrap()}
 pub fn original_revision(&self)->[u8;32]{self.read.revision}
 pub fn original_operation(&self)->(u64,u64){self.read.origin.unwrap()}
}
/// 🚦️ A denied source donation returns the whole original owner to its initiating command.
#[derive(Debug)]
pub enum FlowPreviewSourceTransfer{Working,Refused(FlowSourceReadFailure),Failed(ValueError)}
/// ⏳️ Read admission exposes progress without an alternate evaluation engine.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum FlowSourceReadStep{Working,Ready,Refused(FlowSourceReadFailure)}

/// 🎟️ The final output header is lent only after its original context funds revalidation and the copy.
pub enum FlowCurrentOutputRead<'a>{Working,Ready(FlowCurrentOutput<'a>),Refused(FlowSourceReadFailure)}

#[derive(semio_framework_value::RetireOwned)]
struct ReadSources{infos:Option<ReadLease<HistoryFoldIndex<String,OperatorInfo>>>,baseline:Option<ReadLease<TreeSnapshot>>,channels:Option<ReadLease<EvalChannels>>,source:Option<FlowEvaluationSource>,tree:Option<Tree>,seeds_source:Option<FlowEvaluationSeeds>,seeds:Option<HistoryFoldIndex<String,Dictionary>>,comparison:SourceCompare}
/// 🔎️ Retains the original query frontier and every alias until explicit funded closure.
pub struct FlowSourceRead{sources:ManuallyDrop<Option<ReadSources>>,retirement:Option<ControlledRetirement<ReadSources>>,revision:[u8;32],binding:Option<[usize;8]>,snapshot_binding:Option<[usize;4]>,origin:Option<(u64,u64)>,registry_generation:u64,phase:u8,widget:usize,scan:usize,offset:usize,source_info:usize,target_info:usize,source_port:usize,target_port:usize,channel:usize,output:usize,schema:usize,identity:usize,geometry:bool,failure:Option<FlowSourceReadFailure>,closing:bool,progress:RetainedCloneProgress}
impl FlowSourceRead{
 pub fn new(revision:[u8;32])->Self{Self{sources:ManuallyDrop::new(Some(ReadSources{infos:None,baseline:None,channels:None,source:Some(FlowEvaluationSource::new()),tree:None,seeds_source:Some(FlowEvaluationSeeds::new()),seeds:None,comparison:SourceCompare::new()})),retirement:None,revision,binding:None,snapshot_binding:None,origin:None,registry_generation:0,phase:0,widget:0,scan:0,offset:0,source_info:0,target_info:0,source_port:0,target_port:0,channel:0,output:0,schema:0,identity:0,geometry:false,failure:None,closing:false,progress:Default::default()}}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 fn reject(&mut self,reason:FlowSourceReadFailure)->FlowSourceReadStep{self.failure=Some(reason);FlowSourceReadStep::Refused(reason)}
 pub fn next_demands(&self,snapshot:&FlowHostSnapshot,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}let fields=self.sources.as_ref().ok_or_else(||refusal("original read source is absent"))?;let copy=copy.saturating_sub(CURRENT_SOURCE_CHECK_BYTES);
  let mut demand=match self.phase{0..=2=>RetirementDemand{copy_bytes:match self.phase{0=>size_of::<ReadLease<HistoryFoldIndex<String,OperatorInfo>>>()+size_of::<[usize;8]>()+size_of::<[usize;4]>()+size_of::<(u64,u64)>(),1=>size_of::<ReadLease<TreeSnapshot>>(),_=>size_of::<ReadLease<EvalChannels>>()},depth:1,..Default::default()},3=>fields.source.as_ref().unwrap().next_demands(snapshot,&fields.infos.as_ref().unwrap().0,copy)?,4=>fields.source.as_ref().unwrap().next_close_demands(copy)?,5|9=>fields.comparison.demands()?,6|10=>fields.comparison.demands()?,26=>RetirementDemand{copy_bytes:size_of::<SourceCompare>(),depth:1,..Default::default()},7=>fields.seeds_source.as_ref().unwrap().next_demands(fields.tree.as_ref().unwrap(),copy)?,8=>fields.seeds_source.as_ref().unwrap().next_close_demands(copy)?,255=>RetirementDemand{copy_bytes:size_of::<FlowCurrentOutput>(),depth:1,..Default::default()},_=>RetirementDemand{copy_bytes:size_of::<usize>()*4,depth:1,..Default::default()}};if matches!(self.phase,12..=16|22..=23){let infos=&fields.infos.as_ref().unwrap().0;let slot=match self.phase{12|14=>self.scan,13|16|22=>self.source_info,_=>self.target_info};if slot<infos.slot_count(){demand.depth=demand.depth.max(infos.next_slot_depth_demand(slot)?.saturating_add(1));}}
  if matches!(self.phase,17..=25|255){let channels=&fields.channels.as_ref().unwrap().0.outputs;let slot=if self.phase==17{self.scan}else{self.channel};if slot<channels.slot_count(){demand.depth=demand.depth.max(channels.next_slot_depth_demand(slot)?.saturating_add(1));}if self.phase>=18{demand.depth=demand.depth.max(semio_framework_value_ordered::ordered::MAX_AVL_HEIGHT+2);}}
  demand.copy_bytes=demand.copy_bytes.max(size_of::<FlowSourceReadFailure>()).checked_add(CURRENT_SOURCE_CHECK_BYTES).ok_or_else(||refusal("original read comparison body overflows"))?;demand.depth=demand.depth.max(1);Ok(demand)
 }
 pub fn step(&mut self,snapshot:&FlowHostSnapshot,revision:[u8;32],session:&impl FlowSourceBaseline,request:&FlowSourceReadRequest<'_>,cx:&mut StepContext<'_>)->Result<FlowSourceReadStep,ValueError>{
  self.progress=Default::default();if self.closing{return Err(refusal("original read admission is closing"))}if cx.is_cancelled()||cx.should_yield()||cx.retained_grant().maximum_items==0{return Ok(FlowSourceReadStep::Working)}let grant=cx.retained_grant();let d=self.next_demands(snapshot,grant.maximum_copy_bytes)?;if !admitted(d,grant){return Ok(FlowSourceReadStep::Working)}if let Some(failure)=self.failure{return Ok(FlowSourceReadStep::Refused(failure))}
  macro_rules! reject{($reason:expr)=>{{self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<FlowSourceReadFailure>()+CURRENT_SOURCE_CHECK_BYTES,..Default::default()};cx.consume_retained(self.progress)?;cx.consume_fuel(1);return Ok(self.reject($reason));}}}
  let origin=(cx.operation().0,cx.generation().0);if let Some(reason)=self.current_failure(snapshot,revision,session,request,origin){reject!(reason)}
  if self.phase==255{self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:CURRENT_SOURCE_CHECK_BYTES,..Default::default()};cx.consume_retained(self.progress)?;cx.consume_fuel(1);return Ok(FlowSourceReadStep::Ready)}
  let child_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-CURRENT_SOURCE_CHECK_BYTES,..grant};let child_demand=RetirementDemand{copy_bytes:d.copy_bytes-CURRENT_SOURCE_CHECK_BYTES,..d};let result=self.advance(snapshot,session,request,child_grant,child_demand);self.progress.copied_items=self.progress.copied_items.max(1);self.progress.copied_bytes=self.progress.copied_bytes.checked_add(CURRENT_SOURCE_CHECK_BYTES).ok_or_else(||refusal("original read comparison receipt overflows"))?;if self.phase>0&&self.origin.is_none(){self.origin=Some(origin)}cx.consume_retained(self.progress)?;cx.consume_fuel(1);result.map_err(|error|error.with_retained_progress(self.progress))
 }
 fn advance(&mut self,snapshot:&FlowHostSnapshot,session:&impl FlowSourceBaseline,request:&FlowSourceReadRequest<'_>,grant:RetainedCloneGrant,d:RetirementDemand)->Result<FlowSourceReadStep,ValueError>{
  let fields=self.sources.as_mut().unwrap();self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  match self.phase{
   0=>{fields.infos=Some(ReadLease(flow_neuron_kind_info_map()));self.binding=Some(request.original_binding());self.snapshot_binding=Some([snapshot.widgets.as_ptr()as usize,snapshot.widgets.len(),snapshot.synapses.as_ptr()as usize,snapshot.synapses.len()]);self.registry_generation=flow_extension_registry_generation();self.phase=1;},
   1=>{fields.baseline=Some(ReadLease(Arc::clone(session.previous_snapshot().unwrap())));self.phase=2;},
   2=>{fields.channels=Some(ReadLease(Arc::clone(session.previous_channels().unwrap())));self.phase=3;},
   3=>{let child=fields.source.as_mut().unwrap();let result=child.step(snapshot,&fields.infos.as_ref().unwrap().0,grant);self.progress=child.step_progress();if let Some(tree)=result?{fields.tree=Some(tree);child.begin_close();self.phase=4;}},
   4=>{let child=fields.source.as_mut().unwrap();let result=child.close_step(grant);self.progress=child.step_progress();result?;if child.terminal_is_empty(){fields.source=None;self.phase=5;}},
   5=>{let result=fields.comparison.step(fields.tree.as_ref().unwrap(),fields.baseline.as_ref().unwrap().0.tree_source(),grant);self.progress=fields.comparison.step_progress();result?;if let Some(equal)=fields.comparison.result(){if !equal{return Ok(self.reject(FlowSourceReadFailure::ChangedSource))}fields.comparison.begin_close();self.phase=6;}},
   6=>{let result=fields.comparison.close_step(grant);self.progress=fields.comparison.step_progress();result?;if fields.comparison.terminal_is_empty(){self.phase=26;}},
   26=>{fields.comparison=SourceCompare::new();self.phase=7;},
   7=>{let child=fields.seeds_source.as_mut().unwrap();let result=child.step(fields.tree.as_ref().unwrap(),grant);self.progress=child.step_progress();if let Some(seeds)=result?{fields.seeds=Some(seeds);child.begin_close();self.phase=8;}},
   8=>{let child=fields.seeds_source.as_mut().unwrap();let result=child.close_step(grant);self.progress=child.step_progress();result?;if child.terminal_is_empty(){fields.seeds_source=None;self.phase=9;}},
   9=>{let result=fields.comparison.step(fields.seeds.as_ref().unwrap(),fields.baseline.as_ref().unwrap().0.seed_source(),grant);self.progress=fields.comparison.step_progress();result?;if let Some(equal)=fields.comparison.result(){if !equal{return Ok(self.reject(FlowSourceReadFailure::ChangedSource))}fields.comparison.begin_close();self.phase=10;}},
   10=>{let result=fields.comparison.close_step(grant);self.progress=fields.comparison.step_progress();result?;if fields.comparison.terminal_is_empty(){self.phase=11;}},
   11=>{
    let Some(widget)=snapshot.widgets.get(self.widget)else{return Ok(self.reject(FlowSourceReadFailure::MissingWidget))};match compare_text(&mut self.offset,widget_id_for(widget),request.source_widget,&mut self.progress,grant){Some(true)=>{if !matches!(widget,Widget::Neuron{..}){return Ok(self.reject(FlowSourceReadFailure::MissingPort))}self.scan=0;self.phase=12;},Some(false)=>self.widget+=1,None=>{}}
   },
   12|14=>{
    let infos=&fields.infos.as_ref().unwrap().0;if self.scan==infos.slot_count(){return Ok(self.reject(FlowSourceReadFailure::MissingPort))}if let Some((key,_))=infos.slot_entry(self.scan){let wanted=if self.phase==12{match &snapshot.widgets[self.widget]{Widget::Neuron{neuron_kind,..}=>neuron_kind.as_str(),_=>unreachable!()}}else{match request.target{FlowSourceReadTarget::Port{kind,..}=>kind,FlowSourceReadTarget::Preview=>unreachable!()}};match compare_text(&mut self.offset,key,wanted,&mut self.progress,grant){Some(true)=>{if self.phase==12{self.source_info=self.scan;self.phase=13}else{self.target_info=self.scan;self.phase=15}self.scan=0;},Some(false)=>self.scan+=1,None=>{}}}else{self.scan+=1}
   },
   13|15=>{
    let info=fields.infos.as_ref().unwrap().0.slot_entry(if self.phase==13{self.source_info}else{self.target_info}).unwrap().1;let ports=if self.phase==13{&info.outputs}else{&info.inputs};let Some(port)=ports.get(self.scan)else{return Ok(self.reject(FlowSourceReadFailure::MissingPort))};let wanted=if self.phase==13{request.source_port}else{request.target_port().unwrap()};match compare_text(&mut self.offset,&port.name,wanted,&mut self.progress,grant){Some(true)=>{if self.phase==13{self.source_port=self.scan;self.phase=if matches!(request.target,FlowSourceReadTarget::Preview){16}else{14}}else{self.target_port=self.scan;self.phase=16}self.scan=0;},Some(false)=>self.scan+=1,None=>{}}
   },
   16=>{if fields.infos.as_ref().unwrap().0.slot_entry(self.source_info).unwrap().1.outputs[self.source_port].cardinality.is_collection(){return Ok(self.reject(FlowSourceReadFailure::IncompatiblePort))}self.phase=17;},
   17=>{
    let outputs=&fields.channels.as_ref().unwrap().0.outputs;if self.scan==outputs.slot_count(){return Ok(self.reject(FlowSourceReadFailure::MissingOutput))}if let Some((key,_))=outputs.slot_entry(self.scan){match compare_text(&mut self.offset,key,request.source_widget,&mut self.progress,grant){Some(true)=>{self.channel=self.scan;self.scan=0;self.phase=18;},Some(false)=>self.scan+=1,None=>{}}}else{self.scan+=1}
   },
   18|19=>{
    let outputs=fields.channels.as_ref().unwrap().0.outputs.slot_entry(self.channel).unwrap().1;let Some((key,_))=outputs.entry_at_rank(self.scan)else{if self.phase==18{self.scan=0;self.phase=19;return Ok(FlowSourceReadStep::Working)}return Ok(self.reject(FlowSourceReadFailure::MissingOutput))};let wanted=if self.phase==18{"error"}else{request.source_port};match compare_text(&mut self.offset,key,wanted,&mut self.progress,grant){Some(true)=>{if self.phase==18{return Ok(self.reject(FlowSourceReadFailure::FailedOutput))}self.output=self.scan;self.scan=0;self.phase=20;},Some(false)=>self.scan+=1,None=>{}}
   },
   20|24=>{
    let Some(value)=fields.channels.as_ref().unwrap().0.outputs.slot_entry(self.channel).unwrap().1.entry_at_rank(self.output).unwrap().1.as_dictionary()else{return Ok(self.reject(FlowSourceReadFailure::MissingOutput))};let Some((key,_))=value.entry_at_rank(self.scan)else{return Ok(self.reject(FlowSourceReadFailure::MissingOutput))};let wanted=if self.phase==20{"$schema"}else if self.geometry{"handle"}else{"data"};match compare_text(&mut self.offset,key,wanted,&mut self.progress,grant){Some(true)=>{if self.phase==20{self.schema=self.scan;self.phase=21}else{self.identity=self.scan;self.phase=25}self.scan=0;},Some(false)=>self.scan+=1,None=>{}}
   },
   21=>{
    let value=fields.channels.as_ref().unwrap().0.outputs.slot_entry(self.channel).unwrap().1.entry_at_rank(self.output).unwrap().1.as_dictionary().unwrap();let schema=value.entry_at_rank(self.schema).unwrap().1.as_atom().and_then(Atom::as_str);self.geometry=match schema{Some("geometry")=>true,Some("mesh")=>false,_=>return Ok(self.reject(FlowSourceReadFailure::MissingOutput))};self.phase=22;
   },
   22|23=>{
    let info=fields.infos.as_ref().unwrap().0.slot_entry(if self.phase==22{self.source_info}else{self.target_info}).unwrap().1;let types=if self.phase==22{&info.outputs[self.source_port].value_types}else{&info.inputs[self.target_port].value_types};let Some(value)=types.get(self.scan)else{return Ok(self.reject(FlowSourceReadFailure::IncompatiblePort))};match compare_text(&mut self.offset,value,if self.geometry{"geometry"}else{"mesh"},&mut self.progress,grant){Some(true)=>{self.scan=0;self.phase=if self.phase==22&&matches!(request.target,FlowSourceReadTarget::Preview){24}else{self.phase+1};},Some(false)=>self.scan+=1,None=>{}}
   },
   25=>{
    let value=fields.channels.as_ref().unwrap().0.outputs.slot_entry(self.channel).unwrap().1.entry_at_rank(self.output).unwrap().1.as_dictionary().unwrap();if value.entry_at_rank(self.identity).unwrap().1.as_atom().and_then(Atom::as_str).is_none_or(str::is_empty){return Ok(self.reject(FlowSourceReadFailure::MissingOutput))}self.phase=255;return Ok(FlowSourceReadStep::Ready);
   },_=>return Err(refusal("original read admission phase is invalid")),
  }Ok(FlowSourceReadStep::Working)
 }
 fn current_failure(&self,snapshot:&FlowHostSnapshot,revision:[u8;32],session:&impl FlowSourceBaseline,request:&FlowSourceReadRequest<'_>,origin:(u64,u64))->Option<FlowSourceReadFailure>{
if self.origin.is_some_and(|original|original!=origin){return Some(FlowSourceReadFailure::Origin)}
  if self.snapshot_binding.is_some_and(|original|original!=[snapshot.widgets.as_ptr()as usize,snapshot.widgets.len(),snapshot.synapses.as_ptr()as usize,snapshot.synapses.len()]){return Some(FlowSourceReadFailure::ChangedSource)}
  if session.baseline_geometry_pending()||session.preview_cancelled()||session.pending_host().is_some_and(|host|host.has_pending_evaluation()||!host.pending_evaluation_ids().is_empty()){return Some(FlowSourceReadFailure::Pending)}
  if revision!=self.revision{return Some(FlowSourceReadFailure::Revision)}if self.binding.is_some_and(|binding|binding!=request.original_binding()){return Some(FlowSourceReadFailure::ChangedSource)}if session.previous_snapshot().is_none()||session.previous_channels().is_none(){return Some(FlowSourceReadFailure::MissingBaseline)}
  let generation=flow_extension_registry_generation();if session.flow_extension_generation()!=generation||self.phase>0&&self.registry_generation!=generation{return Some(FlowSourceReadFailure::Registry)}
  let fields=self.sources.as_ref().unwrap();if fields.baseline.as_ref().is_some_and(|source|!Arc::ptr_eq(&source.0,session.previous_snapshot().unwrap()))||fields.channels.as_ref().is_some_and(|source|!Arc::ptr_eq(&source.0,session.previous_channels().unwrap())){return Some(FlowSourceReadFailure::ChangedSource)}
  None
 }
 pub fn admit_current_output<'a>(&'a mut self,snapshot:&FlowHostSnapshot,revision:[u8;32],session:&impl FlowSourceBaseline,request:&FlowSourceReadRequest<'_>,cx:&mut StepContext<'_>)->Result<FlowCurrentOutputRead<'a>,ValueError>{
  self.progress=Default::default();if self.closing{return Err(refusal("original read admission is closing"))}if self.phase!=255||cx.is_cancelled()||cx.should_yield(){return Ok(FlowCurrentOutputRead::Working)}let grant=cx.retained_grant();if !admitted(self.next_demands(snapshot,grant.maximum_copy_bytes)?,grant){return Ok(FlowCurrentOutputRead::Working)}if let Some(reason)=self.failure.or_else(||self.current_failure(snapshot,revision,session,request,(cx.operation().0,cx.generation().0))){self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<FlowSourceReadFailure>()+CURRENT_SOURCE_CHECK_BYTES,..Default::default()};cx.consume_retained(self.progress)?;cx.consume_fuel(1);self.reject(reason);return Ok(FlowCurrentOutputRead::Refused(reason))}self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<FlowCurrentOutput>()+CURRENT_SOURCE_CHECK_BYTES,..Default::default()};cx.consume_retained(self.progress)?;cx.consume_fuel(1);Ok(FlowCurrentOutputRead::Ready(self.current_output(request).ok_or_else(||refusal("original admitted output identity is absent"))?))
 }
 pub fn current_output<'a>(&'a self,request:&FlowSourceReadRequest<'_>)->Option<FlowCurrentOutput<'a>>{
  if self.phase!=255||self.failure.is_some()||self.closing||self.binding!=Some(request.original_binding()){return None}let value=self.sources.as_ref()?.channels.as_ref()?.0.outputs.slot_entry(self.channel)?.1.entry_at_rank(self.output)?.1.as_dictionary()?;Some(FlowCurrentOutput{identity:value.entry_at_rank(self.identity)?.1.as_atom()?.as_str()?,geometry:self.geometry,revision:self.revision,binding:self.binding?,origin:self.origin?})
 }
 pub fn into_preview_source(mut self,snapshot:&FlowHostSnapshot,revision:[u8;32],session:&impl FlowSourceBaseline,request:&FlowSourceReadRequest<'_>,cx:&mut StepContext<'_>)->Result<FlowPreviewSource,(FlowPreviewSourceTransfer,Self)>{
  if !matches!(request.target,FlowSourceReadTarget::Preview){return Err((FlowPreviewSourceTransfer::Refused(FlowSourceReadFailure::IncompatiblePort),self))}
  let alias_bytes=size_of::<semio_framework_value_ordered::ordered::SharedOwner<NeuralValue>>();let demand=match self.next_preview_source_demands(snapshot,cx.retained_grant().maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return Err((FlowPreviewSourceTransfer::Failed(error),self))};if !admitted(demand,cx.retained_grant()){return Err((FlowPreviewSourceTransfer::Working,self))}
  let result=self.admit_current_output(snapshot,revision,session,request,cx);let outcome=match result{Ok(FlowCurrentOutputRead::Ready(output))if output.preview_handle().is_some()=>None,Ok(FlowCurrentOutputRead::Refused(reason))=>Some(FlowPreviewSourceTransfer::Refused(reason)),Ok(_)=>Some(FlowPreviewSourceTransfer::Working),Err(error)=>Some(FlowPreviewSourceTransfer::Failed(error))};if let Some(outcome)=outcome{return Err((outcome,self))}
  let identity=self.sources.as_ref().unwrap().channels.as_ref().unwrap().0.outputs.slot_entry(self.channel).unwrap().1.entry_at_rank(self.output).unwrap().1.as_dictionary().unwrap().value_shared_at_rank(self.identity).unwrap();if let Err(error)=cx.consume_retained(RetainedCloneProgress{copied_bytes:alias_bytes,..Default::default()}){return Err((FlowPreviewSourceTransfer::Failed(error),self))}Ok(FlowPreviewSource{read:self,identity})
 }
 /// 📏️ The original admission and channel alias form one source transfer event.
 pub fn next_preview_source_demands(&self,snapshot:&FlowHostSnapshot,copy:usize)->Result<RetirementDemand,ValueError>{let mut demand=self.next_demands(snapshot,copy)?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<semio_framework_value_ordered::ordered::SharedOwner<NeuralValue>>()).ok_or_else(||refusal("original preview source header overflows"))?;Ok(demand)}
 pub fn begin_close(&mut self){self.closing=true;if let Some(fields)=self.sources.as_mut(){if let Some(child)=fields.source.as_mut(){child.begin_close()}if let Some(child)=fields.seeds_source.as_mut(){child.begin_close()}fields.comparison.begin_close();}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.retirement.as_ref(){return FlowEvaluationSource::demand(owner,copy)}Ok(RetirementDemand{depth:usize::from(self.sources.is_some()),..Default::default()})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.progress=Default::default();if !self.closing{return Err(refusal("original read admission close was not requested"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.next_close_demands(grant.maximum_copy_bytes)?;if !admitted(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}let step=if let Some(owner)=self.retirement.as_mut(){let result=owner.step(grant);self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.retirement=None}step}else{self.retirement=Some(ControlledRetirement::new(self.sources.take().unwrap()).unwrap_or_else(|_|unreachable!()));RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})};self.progress=step.progress();Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(self.progress)}else{RetainedCloneStep::Progress(self.progress)})}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.sources.is_none()&&self.retirement.is_none()}
}
impl Drop for FlowSourceRead{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original read admission must finish explicit custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.sources)}}}}
struct ReadRetirement(FlowSourceRead);
impl RetirementCursor for ReadRetirement{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.0.close_step(grant){Ok(step)if step.progress()!=Default::default()=>RetirementStep::Progress(step.progress()),Ok(RetainedCloneStep::Complete(_))=>RetirementStep::Complete,Ok(_)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.0.retirement.as_ref().map_or(Ok(0),|owner|owner.next_copy_byte_demand())}
 fn next_selected_work_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.0.next_close_demands(copy)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.next_close_demands(copy).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_demands(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl RetireOwned for FlowSourceRead{fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(ReadRetirement(self))}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<ReadRetirement>())}fn controlled_retirement_supported()->bool{true}}
