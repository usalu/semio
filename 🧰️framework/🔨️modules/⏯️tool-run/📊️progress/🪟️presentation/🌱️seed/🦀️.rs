//! 🌱️ Admits the original identifier and empty joint presentation through independently funded turns.
use super::{ToolRunPresentation,ToolRunPresentationBody,ToolRunProgress,ToolRunEntityProvenance};
use super::super::{ToolRunIdentity,ToolRunState,ToolRunStepRing};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{RetireOwned,RetirementCursor,shared::sealed::{SealedShared,SharedIssuer}}};
use std::mem::{ManuallyDrop,size_of};

#[derive(semio_framework_value::RetireOwned)]
struct State{input:Option<String>,tool_id:Option<SealedShared<String>>,provenance:Option<ToolRunEntityProvenance>,identity:ToolRunIdentity,state:ToolRunState,output:Option<ToolRunPresentation>,phase:u8,cancelled:bool}
pub struct ToolRunPresentationSeed{state:ManuallyDrop<State>,transferred:bool}
impl ToolRunPresentationSeed{
 pub fn new(tool_id:String,identity:ToolRunIdentity,state:ToolRunState)->Self{Self{state:ManuallyDrop::new(State{input:Some(tool_id),tool_id:None,provenance:None,identity,state,output:None,phase:0,cancelled:false}),transferred:false}}
 pub fn complete(&self)->bool{self.state.phase==3&&!self.state.cancelled}
 pub fn original_identifier(&self)->&str{if let Some(input)=self.state.input.as_ref(){input}else if let Some(id)=self.state.tool_id.as_ref(){id.get()}else{self.state.output.as_ref().expect("seed original identifier remains retained before output handoff").body().tool_id.get()}}
 pub fn cancel(&mut self){self.state.cancelled=true;}
 pub fn next_demands(&self)->Result<RetirementDemand,ValueError>{if self.state.cancelled||self.state.phase>=3{return Ok(Default::default());}Ok(match self.state.phase{0=>RetirementDemand{copy_bytes:size_of::<String>(),capacity_bytes:SealedShared::<String>::birth_bytes(),depth:1,..Default::default()},1=>RetirementDemand{copy_bytes:ToolRunEntityProvenance::birth_copy_bytes(),capacity_bytes:ToolRunEntityProvenance::birth_bytes(),depth:1,..Default::default()},2=>ToolRunPresentation::birth_demands(),_=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"seed phase is invalid"))})}
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.state.cancelled||self.state.phase>=3||grant.maximum_items==0{return Ok(Default::default());}let quote=self.next_demands()?;if quote.copy_bytes>grant.maximum_copy_bytes||quote.capacity_bytes>grant.maximum_capacity_bytes||quote.release_bytes>grant.maximum_release_bytes||quote.depth>grant.maximum_depth{return Ok(Default::default());}
  let state=&mut*self.state;match state.phase{
   0=>{let birth=SealedShared::<String>::prepare_birth(grant)?;let(tool_id,mut receipt)=birth.materialize(state.input.take().unwrap(),SharedIssuer::owned());receipt.copied_bytes=quote.copy_bytes;state.tool_id=Some(tool_id);state.phase=1;Ok(receipt)},
   1=>{let(provenance,receipt)=ToolRunEntityProvenance::admit_empty(grant)?;state.provenance=Some(provenance);state.phase=2;Ok(receipt)},
   2=>{let body=ToolRunPresentationBody{tool_id:state.tool_id.take().unwrap(),progress:ToolRunProgress{identity:state.identity,sequence:0,state:state.state,stage:0,completed:0,total:None,counters:Vec::new(),units_per_second:0.0,conflicts:0,steps:ToolRunStepRing::default()},provenance:state.provenance.take().unwrap(),payload:None};match ToolRunPresentation::admit(body,grant){Ok((output,receipt))=>{state.output=Some(output);state.phase=3;Ok(receipt)},Err((error,body))=>{state.tool_id=Some(body.tool_id);state.provenance=Some(body.provenance);Err(error)}}},
   _=>unreachable!(),
  }
 }
 pub fn take_output(&mut self,grant:RetainedCloneGrant)->Result<Option<(ToolRunPresentation,RetainedCloneProgress)>,ValueError>{if !self.complete()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<ToolRunPresentation>()||grant.maximum_depth==0{return Ok(None);}let output=self.state.output.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"seed lost its unpublished presentation"))?;self.state.phase=4;Ok(Some((output,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<ToolRunPresentation>(),..Default::default()})))}
}
impl RetireOwned for ToolRunPresentationSeed{fn retirement(mut self)->Box<dyn RetirementCursor>{self.transferred=true;unsafe{ManuallyDrop::take(&mut self.state)}.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.state.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}fn retirement_element_copy_bytes()->usize{size_of::<Self>()}}
impl Drop for ToolRunPresentationSeed{fn drop(&mut self){if !self.transferred&&!std::thread::panicking(){panic!("original presentation seed requires controlled retirement")}}}
