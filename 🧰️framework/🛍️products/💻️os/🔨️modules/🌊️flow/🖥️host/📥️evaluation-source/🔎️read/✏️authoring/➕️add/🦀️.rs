//! ➕️ Original add-widget authoring assembles its two mutation leaves without a mutable host.
use super::super::*;
use super::{identity::IdentityWork,widget::{WidgetWork,WidgetPlan},text::FundedText};
use semio_framework_artifact_flow_flow::{FlowSnapshotSource,WidgetDescriptor,Widget,FlowMutation,FlowLayoutEntry,WidgetLayout,AddWidget,ChangeLayout};
use semio_framework_job::{admit_original_construction,OriginalConstructionDemand,OriginalTurnFailure,StepContext,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
use super::storage::{storage_demands,storage_ready,storage_close_demands,storage_close_step};
use semio_framework_value_native_storage::NativeOwnedStorage;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original add-widget authoring changed its retained frontier")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
#[derive(semio_framework_value::RetireOwned)]
struct Fields{id:Option<String>,layout_id:Option<String>,widget:Option<Widget>,entries:Option<Vec<FlowLayoutEntry>>,output:Option<Vec<FlowMutation>>,error:Option<FlowCoreError>}
pub(super) struct AddWork{fields:ManuallyDrop<Fields>,identity:Option<IdentityWork>,widget:NativeOwnedStorage<WidgetWork>,text:Option<FundedText>,retirement:RetainedCloneClose,phase:u8,serial:Option<u64>,closing:bool}
impl AddWork{
 pub(super) fn new()->Self{Self{fields:ManuallyDrop::new(Fields{id:None,layout_id:None,widget:None,entries:None,output:None,error:None}),identity:None,widget:NativeOwnedStorage::empty(),text:None,retirement:Default::default(),phase:0,serial:None,closing:false}}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let d=fixed(bytes);if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(receipt(d))?;cx.consume_fuel(1);Ok(true)}
 fn info<'a>(descriptor:&WidgetDescriptor,infos:&'a HistoryFoldIndex<String,OperatorInfo>)->Option<&'a OperatorInfo>{match descriptor{WidgetDescriptor::Neuron{neuron_kind,..}=>infos.get(neuron_kind),_=>None}}
 pub(super) fn serial(&self)->Option<u64>{self.serial}
 pub(super) fn next_demands(&self,descriptor:&WidgetDescriptor,infos:&HistoryFoldIndex<String,OperatorInfo>,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(id)=self.identity.as_ref(){return if self.phase==1{id.next_demands(descriptor,copy)}else if id.terminal_is_empty(){Ok(fixed(size_of::<Option<IdentityWork>>()))}else{id.close_demands(copy)}}
  if self.phase==3{return storage_demands(&self.widget,|_|Ok(fixed(1)))}
  if self.widget.reserved(){return if self.phase==4{storage_demands(&self.widget,|work|work.next_demands(self.fields.id.as_deref().ok_or_else(invalid)?,WidgetPlan::New(descriptor),Self::info(descriptor,infos),copy))}else{storage_close_demands(&self.widget,|work|work.close_demands(copy),|work|work.terminal_is_empty())}}
  if let Some(text)=self.text.as_ref(){return if text.terminal_is_empty(){Ok(fixed(size_of::<Option<FundedText>>()))}else if self.fields.layout_id.is_some(){text.close_demands(copy)}else{text.next_demands(self.fields.id.as_deref().ok_or_else(invalid)?,copy)}}
  Ok(if self.phase==7{RetirementDemand{capacity_bytes:2*size_of::<FlowMutation>(),..fixed(128)}}else if self.phase==12{RetirementDemand{capacity_bytes:size_of::<FlowLayoutEntry>(),..fixed(128)}}else{fixed(size_of::<IdentityWork>().max(2*size_of::<FlowMutation>()+size_of::<FlowLayoutEntry>()).max(128))})
 }
 pub(super) fn step(&mut self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,descriptor:&WidgetDescriptor,x:f64,y:f64,cx:&mut StepContext<'_>)->Result<Option<Result<Vec<FlowMutation>,FlowCoreError>>,OriginalTurnFailure>{
  if self.closing{return Err(invalid().into())}
  match self.phase{
   0=>if Self::paid(cx,size_of::<IdentityWork>())?{self.identity=Some(IdentityWork::new());self.phase=1},
   1=>if let Some((result,serial))=self.identity.as_mut().ok_or_else(invalid)?.step(source,descriptor,cx)?{self.serial=serial;match result{Ok(id)=>self.fields.id=Some(id),Err(error)=>self.fields.error=Some(error)}self.identity.as_mut().unwrap().begin_close();self.phase=2},
   2=>{let identity=self.identity.as_mut().ok_or_else(invalid)?;if !identity.terminal_is_empty(){let d=identity.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|identity.close_step(g))?;return Ok(None)}if Self::paid(cx,size_of::<Option<IdentityWork>>())?{self.identity=None;self.phase=if self.fields.error.is_some(){10}else{3}}},
   3=>if storage_ready(&mut self.widget,cx,WidgetWork::new)?&&Self::paid(cx,1)?{self.phase=4},
   4=>if let Some(widget)=self.widget.get_mut().ok_or_else(invalid)?.step(self.fields.id.as_deref().ok_or_else(invalid)?,WidgetPlan::New(descriptor),Self::info(descriptor,infos),cx)?{self.fields.widget=Some(widget);self.widget.get_mut().unwrap().begin_close();self.phase=5},
   5=>{if !self.widget.terminal_is_empty(){let d=storage_close_demands(&self.widget,|work|work.close_demands(cx.retained_grant().maximum_copy_bytes),|work|work.terminal_is_empty())?;close_original_retained_step(cx,d,|grant|storage_close_step(&mut self.widget,grant,|work,g|work.close_step(g),|work|work.terminal_is_empty()))?;return Ok(None)}if Self::paid(cx,1)?{self.phase=6}},
   6=>{if self.text.is_none(){if self.fields.layout_id.is_some(){if Self::paid(cx,1)?{self.phase=7}return Ok(None)}if Self::paid(cx,size_of::<FundedText>())?{self.text=Some(FundedText::new())}return Ok(None)}
    let text=self.text.as_mut().unwrap();if self.fields.layout_id.is_none(){if text.ready(){text.take_into(&mut self.fields.layout_id,cx)?;if self.fields.layout_id.is_some(){text.begin_close()}}else{text.advance(self.fields.id.as_deref().ok_or_else(invalid)?,cx)?;}return Ok(None)}
    if !text.terminal_is_empty(){let d=text.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|text.close_step(g))?;return Ok(None)}if Self::paid(cx,size_of::<Option<FundedText>>())?{self.text=None;self.phase=7}},
   7=>{let d=self.next_demands(descriptor,infos,cx.retained_grant().maximum_copy_bytes)?;let mut accepted=None;admit_original_construction(cx,&mut accepted,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:0,depth:1},|_|{let result=(||{let mut output=Vec::new();output.try_reserve_exact(2).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original add-widget mutation roster refused"))?;let actual=output.capacity()*size_of::<FlowMutation>();self.fields.output=Some(output);self.phase=12;Ok(((),RetainedCloneProgress{retained_capacity_bytes:actual,..receipt(d)}))})();RetainedWorkOutcome{result,work:1}})?;},
   12=>{let d=self.next_demands(descriptor,infos,cx.retained_grant().maximum_copy_bytes)?;let mut accepted=None;admit_original_construction(cx,&mut accepted,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:0,depth:1},|_|{let result=(||{let mut entries=Vec::new();entries.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original add-widget layout roster refused"))?;let actual=entries.capacity()*size_of::<FlowLayoutEntry>();self.fields.entries=Some(entries);self.phase=8;Ok(((),RetainedCloneProgress{retained_capacity_bytes:actual,..receipt(d)}))})();RetainedWorkOutcome{result,work:1}})?;},
   8=>if Self::paid(cx,2*size_of::<FlowMutation>()+size_of::<FlowLayoutEntry>())?{let fields=&mut*self.fields;let entries=fields.entries.as_mut().ok_or_else(invalid)?;entries.push(FlowLayoutEntry{id:fields.layout_id.take().ok_or_else(invalid)?,layout:Some(WidgetLayout{x,y})});let output=fields.output.as_mut().ok_or_else(invalid)?;output.push(FlowMutation::AddWidget(AddWidget{index:u32::try_from(source.widgets().len()).unwrap_or(u32::MAX),widget:fields.widget.take().ok_or_else(invalid)?}));output.push(FlowMutation::ChangeLayout(ChangeLayout{entries:fields.entries.take().ok_or_else(invalid)?}));self.phase=9},
   9=>if Self::paid(cx,size_of::<Option<Vec<FlowMutation>>>())?{self.phase=11;return Ok(Some(Ok(self.fields.output.take().ok_or_else(invalid)?)))},
   10=>if Self::paid(cx,size_of::<Option<FlowCoreError>>())?{self.phase=11;return Ok(Some(Err(self.fields.error.take().ok_or_else(invalid)?)))},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub(super) fn restore_output(&mut self,result:Result<Vec<FlowMutation>,FlowCoreError>){assert_eq!(self.phase,11);match result{Ok(output)=>{self.fields.output=Some(output);self.phase=9},Err(error)=>{self.fields.error=Some(error);self.phase=10}}}
 pub(super) fn begin_close(&mut self){self.closing=true;if let Some(id)=self.identity.as_mut(){id.begin_close()}if let Some(widget)=self.widget.get_mut(){widget.begin_close()}if let Some(text)=self.text.as_mut(){text.begin_close()}}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  macro_rules! child{($field:ident,$ty:ty)=>{if let Some(child)=self.$field.as_ref(){return if child.terminal_is_empty(){Ok(fixed(size_of::<Option<$ty>>()))}else{child.close_demands(copy)}}}}
  child!(identity,IdentityWork);if !self.widget.terminal_is_empty(){return storage_close_demands(&self.widget,|work|work.close_demands(copy),|work|work.terminal_is_empty())}child!(text,FundedText);if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  macro_rules! owner{($field:ident,$ty:ty)=>{if self.fields.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,copy)}}}
  owner!(id,String);owner!(layout_id,String);owner!(widget,Widget);owner!(entries,Vec<FlowLayoutEntry>);owner!(output,Vec<FlowMutation>);owner!(error,FlowCoreError);Ok(Default::default())
 }
 pub(super) fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  macro_rules! child{($field:ident,$ty:ty)=>{if let Some(child)=self.$field.as_mut(){if child.terminal_is_empty(){let d=fixed(size_of::<Option<$ty>>());if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.$field=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(d))),work:1}}return child.close_step(g)}}}
  child!(identity,IdentityWork);if !self.widget.terminal_is_empty(){return storage_close_step(&mut self.widget,g,|work,g|work.close_step(g),|work|work.terminal_is_empty())}child!(text,FundedText);if !self.retirement.is_empty(){return self.retirement.step_granted(g)}
  macro_rules! owner{($field:ident)=>{if self.fields.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.$field,g);return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}}
  owner!(id);owner!(layout_id);owner!(widget);owner!(entries);owner!(output);owner!(error);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.identity.is_none()&&self.widget.terminal_is_empty()&&self.text.is_none()&&self.retirement.is_empty()&&self.fields.id.is_none()&&self.fields.layout_id.is_none()&&self.fields.widget.is_none()&&self.fields.entries.is_none()&&self.fields.output.is_none()&&self.fields.error.is_none()}
}
impl Drop for AddWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original add-widget authoring requires terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
