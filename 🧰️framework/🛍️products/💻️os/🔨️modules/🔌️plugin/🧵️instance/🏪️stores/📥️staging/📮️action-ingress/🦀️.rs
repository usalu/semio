//! 🎟️ Actual action parser slots retain unused original arguments and faults through paid receiving retirement.
use semio_framework_value::DslValue;
use semio_framework::Fault;
use semio_framework_value::retirement::RetireOwned as _;
use semio_framework_value::{RetireOwned, RetirementDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, ValueError, ValueRefusalKind};
use crate::stores_components::ActionIngress;
use semio_framework_plugin_decl_app::ArtifactApp;

/// 🫴️ Original parser input is transferred only by the receiving caller, never by a structural copy.
#[derive(RetireOwned)]
pub struct ActionIngressSource{pub action:Option<String>,pub args:Option<DslValue>,pub prepared:Option<DslValue>,pub assembled:Option<DslValue>,pub fault:Option<Fault>}

/// ♻️ One exact receiving record holds the original source until its acknowledged paid close.
pub struct ActionIngressRecord{pub source:std::mem::ManuallyDrop<Option<ActionIngressSource>>,pub retirement:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<ActionIngressSource>>>,pub caller:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,pub ready:bool}

impl ActionIngressRecord{
 pub fn receive(source:&mut Option<ActionIngressSource>,caller:&mut Option<semio_framework_job::OriginalCallerSourceLoan>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Self>,semio_framework_job::OriginalTurnFailure>{
  let grant=cx.retained_grant();let bytes=size_of::<ActionIngressSource>();if source.is_none()||cx.should_yield()||grant.maximum_items<1+usize::from(caller.is_none())||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(None)}
  if caller.is_none(){let Some(loan)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(None)};*caller=Some(loan);}
  Self::receive_with_loan(source,caller,cx)
 }
 fn receive_with_loan(source:&mut Option<ActionIngressSource>,caller:&mut Option<semio_framework_job::OriginalCallerSourceLoan>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Self>,semio_framework_job::OriginalTurnFailure>{
  let grant=cx.retained_grant();let bytes=size_of::<ActionIngressSource>();if source.is_none()||cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(None)}
  caller.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original action caller loan is absent"))?.validate_context(cx)?;
  cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
  Ok(Some(Self{source:std::mem::ManuallyDrop::new(source.take()),retirement:std::mem::ManuallyDrop::new(None),caller:std::mem::ManuallyDrop::new(caller.take()),ready:false}))
 }
 pub fn validate_context(&self,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{self.caller.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original action caller loan has returned"))?.validate_context(cx)}
 pub fn parser_slots(&mut self)->Result<(&mut Option<DslValue>,&mut Option<Fault>),ValueError>{if self.ready{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original action source has already left its parser"))}let source=self.source.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original action source is absent"))?;Ok((if source.assembled.is_some(){&mut source.assembled}else if source.prepared.is_some(){&mut source.prepared}else{&mut source.args},&mut source.fault))}
 pub fn loan_source(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<std::mem::ManuallyDrop<ActionIngressSource>>,semio_framework_job::OriginalTurnFailure>{
  self.validate_context(cx)?;let grant=cx.retained_grant();let bytes=2*size_of::<ActionIngressSource>();if cx.should_yield()||grant.maximum_items<2||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(None)}
  cx.consume_retained(RetainedCloneProgress{copied_items:2,copied_bytes:bytes,..Default::default()})?;
  Ok(self.source.take().map(std::mem::ManuallyDrop::new))
 }
 pub fn return_source(&mut self,source:ActionIngressSource,terminal:bool){assert!(!self.ready&&self.source.is_none());*self.source=Some(source);self.ready=terminal;}
 pub fn finish_parser(&mut self){self.ready=true;}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.retirement.is_none()&&self.caller.is_none()}
 pub fn ready(&self)->bool{self.ready}
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.source.is_none()&&self.retirement.is_none()&&!self.ready{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original action parser still owns its receiving loan"))}if let Some(owner)=self.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)}}if let Some(source)=self.source.as_ref(){return Ok(RetirementDemand{copy_bytes:size_of::<ActionIngressSource>(),capacity_bytes:source.retirement_birth_bytes().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original action source has no controlled retirement factory"))?,depth:1,..Default::default()})}Ok(self.caller.as_ref().map_or(Default::default(),|caller|caller.retirement_demands()))}
 pub fn close_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=self.retirement_demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(owner)=self.retirement.as_mut(){if owner.terminal_is_empty(){self.retirement.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}return owner.step(grant)}if let Some(source)=self.source.take(){let bytes=size_of::<ActionIngressSource>();let capacity=demand.capacity_bytes;match semio_framework_value::retirement::controlled::ControlledRetirement::new(source){Ok(owner)=>{*self.retirement=Some(owner);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:capacity,..Default::default()}))},Err((cause,source))=>{*self.source=Some(source);Err(cause)}}}else{Ok(RetainedCloneStep::Complete(Default::default()))}}
 pub fn close_original_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
  if self.source.is_none()&&self.retirement.is_none(){let step=self.caller.as_mut().unwrap().close_step(cx.original_source_parent()?,cx)?;if self.caller.as_ref().unwrap().terminal_is_empty(){self.caller.take();}return Ok(step)}
  self.validate_context(cx)?;let demand=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|self.close_granted(grant))
 }
}

impl Drop for ActionIngressRecord{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original action ingress reached Drop before paid terminal retirement");}}

impl ActionIngress {
 pub fn action_source_pending(&self)->bool{self.pending_action_ingress.is_some()||(0..16).any(|id|self.action_ingress_sources.get(id).is_some_and(|record|!record.ready()))}

 pub fn action_source_capture_ready(&self,action:&str,cx:&semio_framework_job::StepContext<'_>)->bool{
  for id in 0..16{if let Some(record)=self.action_ingress_sources.get(id){if !record.ready(){return record.source.as_ref().and_then(|source|source.action.as_deref())==Some(action)&&self.pending_action_ingress_identity==Some((cx.operation(),cx.generation()))}}}
  if let Some(original)=self.pending_action_ingress.as_ref(){return original.action.as_deref()==Some(action)&&self.pending_action_ingress_identity==Some((cx.operation(),cx.generation()))}
  let backing=if self.action_ingress_sources.allocation_admitted{0}else{16*size_of::<std::mem::MaybeUninit<(u64,ActionIngressRecord)>>()};
  let grant=cx.retained_grant();!cx.should_yield()&&grant.maximum_items>=5+usize::from(backing!=0)&&grant.maximum_copy_bytes>=4*size_of::<ActionIngressSource>()+action.len()&&grant.maximum_capacity_bytes>=backing+action.len()&&grant.maximum_depth>=2
 }

 pub fn stage_action_source(&mut self,action:&str,args:&mut Option<DslValue>,fault:&mut Option<Fault>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<u64>,semio_framework_job::OriginalTurnFailure>{
  if !self.action_ingress_sources.allocation_admitted{if cx.should_yield(){return Ok(None)}let step=self.action_ingress_sources.admit_backing_granted(cx.retained_grant())?;cx.consume_retained(step.progress())?;if !self.action_ingress_sources.allocation_admitted{return Ok(None)}}
  for id in 0..16{if let Some(record)=self.action_ingress_sources.get(id){if !record.ready(){record.validate_context(cx)?;return if record.source.as_ref().and_then(|source|source.action.as_deref())==Some(action)&&self.pending_action_ingress_identity==Some((cx.operation(),cx.generation())){Ok(Some(id))}else{Ok(None)}}}}
  if let Some(original)=self.pending_action_ingress.as_ref(){if original.action.as_deref()!=Some(action)||self.pending_action_ingress_identity!=Some((cx.operation(),cx.generation())){return Ok(None)}}else{
   let grant=cx.retained_grant();let bytes=size_of::<ActionIngressSource>()+action.len();if cx.should_yield()||grant.maximum_items<2||grant.maximum_copy_bytes<bytes||grant.maximum_capacity_bytes<action.len()||grant.maximum_depth<2{return Ok(None)}
   let Some(caller)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(None)};*self.pending_action_ingress_caller=Some(caller);
   cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<ActionIngressSource>(),..Default::default()})?;
   *self.pending_action_ingress=Some(ActionIngressSource{action:Some(String::new()),args:args.take(),prepared:None,assembled:None,fault:fault.take()});self.pending_action_ingress_identity=Some((cx.operation(),cx.generation()));
   let original=self.pending_action_ingress.as_mut().unwrap();let name=original.action.as_mut().unwrap();name.try_reserve_exact(action.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original action name capture allocation refused"))?;name.push_str(action);cx.consume_retained(RetainedCloneProgress{copied_items:0,copied_bytes:action.len(),retained_capacity_bytes:name.capacity(),..Default::default()})?;
  }
  self.pending_action_ingress_caller.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original pending action caller loan is absent"))?.validate_context(cx)?;
  let Some(slot)=(0..16).find(|index|self.action_ingress_sources.slot_is_vacant(*index))else{return Ok(None)};
  let Some(record)=ActionIngressRecord::receive_with_loan(&mut self.pending_action_ingress,&mut self.pending_action_ingress_caller,cx)?else{return Ok(None)};
  self.action_ingress_sources.insert_admitted(slot as u64,record);
  Ok(Some(slot as u64))
 }

 pub async fn receive_action_command<A: ArtifactApp>(&mut self,action:&str,args:&mut Option<DslValue>,fault:&mut Option<Fault>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<A::Command>,semio_framework_job::OriginalTurnFailure>{
  let Some(slot)=self.stage_action_source(action, args, fault, cx)?else{return Ok(None)};
  let source=self.action_ingress_sources.get_mut(slot).expect("original admitted parser source");let (args,fault)=source.parser_slots()?;
  let result=A::command_from_action(action,args,fault).await;
  self.action_ingress_sources.get_mut(slot).unwrap().finish_parser();self.pending_action_ingress_identity=None;result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
 }

 pub(crate) fn selected_action_record(&self,closing:bool)->Option<u64>{(0..16).find(|id|self.action_ingress_sources.get(*id).is_some_and(|record|closing||record.ready()))}

 pub fn action_ingress_sources_idle(&self)->bool{self.selected_action_record(false).is_none()&&self.pending_action_ingress_retirement.is_none()}

 pub fn action_ingress_sources_closed(&self)->bool{self.action_ingress_sources.is_empty()&&self.pending_action_ingress_retirement.is_none()&&self.pending_action_ingress.is_none()&&self.pending_action_ingress_caller.is_none()&&self.action_ingress_sources.slots.is_empty()}

 pub fn action_ingress_sources_demands(&self,body:usize,closing:bool)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.pending_action_ingress_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)}}if self.pending_action_ingress.is_none(){if let Some(caller)=self.pending_action_ingress_caller.as_ref(){return Ok(caller.retirement_demands())}}if closing&&self.pending_action_ingress.is_some(){return semio_framework_os_kernel::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_action_ingress)}if let Some(id)=self.selected_action_record(closing){let record=self.action_ingress_sources.get(id).unwrap();if record.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<ActionIngressRecord>(),depth:1,..Default::default()})}return record.retirement_demands(body)}if closing{Ok(RetirementDemand{release_bytes:self.action_ingress_sources.empty_backing_byte_demand().unwrap(),depth:usize::from(!self.action_ingress_sources.slots.is_empty()),..Default::default()})}else{Ok(Default::default())}}

 pub fn close_action_ingress_sources(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant,closing:bool)->Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{let demand=self.action_ingress_sources_demands(grant.maximum_copy_bytes, closing)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
  if let Some(owner)=self.pending_action_ingress_retirement.as_mut(){if owner.terminal_is_empty(){self.pending_action_ingress_retirement.take();self.pending_action_ingress_identity=None;return Ok((semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}let step=owner.step(grant)?;return Ok((semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(step.progress()),false))}
  if self.pending_action_ingress.is_none(){if let Some(caller)=self.pending_action_ingress_caller.as_mut(){let step=caller.close_step(cx.original_source_parent()?,cx)?;if caller.terminal_is_empty(){self.pending_action_ingress_caller.take();self.pending_action_ingress_identity=None;}return Ok((semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(step.progress()),false))}}
  if closing{if let Some(caller)=self.pending_action_ingress_caller.as_ref(){caller.validate_context(cx)?;}if let Some(source)=self.pending_action_ingress.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new(Some(source)){Ok(owner)=>{*self.pending_action_ingress_retirement=Some(owner);return Ok((semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}),false))},Err((cause,source))=>{*self.pending_action_ingress=source;return Err(cause.into())}}}}
  if let Some(id)=self.selected_action_record(closing){let record=self.action_ingress_sources.get_mut(id).unwrap();if record.terminal_is_empty(){drop(self.action_ingress_sources.remove(id));return Ok((semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<ActionIngressRecord>(),..Default::default()}),false))}let step=record.close_original_step(cx)?;Ok((semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(step.progress()),false))}else if closing{let step=self.action_ingress_sources.close_empty_backing_granted(grant)?;Ok((semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(step.progress()),self.action_ingress_sources_closed()))}else{Ok((semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(Default::default()),true))}
 })}
}

/// 🧭️ The admitted protocol command determines its explicit original envelope category.
#[derive(Clone,Copy,RetireOwned)]
pub enum OriginalParsedCommandKind{Command,Artifact}

/// 📦️ Fixed original receiving slots keep complete source bodies beside their declared typed projections.
#[derive(RetireOwned)]
pub struct OriginalParsedCommandSource{
 pub kind:OriginalParsedCommandKind,
 pub envelope:Option<DslValue>,
 pub args:Option<DslValue>,
 pub view_source:Option<DslValue>,
 pub view:Option<semio_framework::ViewModel>,
 pub action_address:Option<semio_framework::manifest::ActionAddress>,
 pub command_address:Option<semio_framework::manifest::CommandAddress>,
 pub meta:Option<semio_framework_plugin_decl_emission::ActionMeta>,
 pub failure:Option<semio_framework_job::OriginalTurnFailure>,
 args_extracted:bool,
}

impl OriginalParsedCommandSource{
 /// 🪹️ The fixed receiving header owns no body allocation before the actual issuer funds decoding.
 pub fn new(kind:OriginalParsedCommandKind)->Self{Self{kind,envelope:None,args:None,view_source:None,view:None,action_address:None,command_address:None,meta:None,failure:None,args_extracted:false}}
 /// 🫴️ Fixed typed address slots receive exact original text backing without allocating a copied projection.
 pub fn prepare(&mut self,control:&mut semio_framework_value::NativeDecodeControl<'_>,grant:RetainedCloneGrant,maximum_work:u64)->semio_framework_job_native_decode::OriginalNativeDecodeIssued<bool>{
  let mut progress=RetainedCloneProgress::default();let mut work=0usize;
  let value=(||{
   if maximum_work<128{return Ok(false)}
   let Some(envelope)=self.envelope.as_ref()else{return Ok(false)};
   let DslValue::Object(fields)=envelope else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original command envelope must be an object"))};
   let(head,key)=if matches!(self.kind,OriginalParsedCommandKind::Artifact){("action","args")}else{("address","arguments")};
   work+=1;if fields.len()>2{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original command envelope exceeds its declared fields"))}
   for(index,(name,_))in fields.iter().enumerate(){work+=1;if name!=head&&name!=key||fields[..index].iter().any(|(previous,_)|previous==name){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original command envelope has undeclared or repeated fields"))}}
   work+=fields.len();if !fields.iter().any(|(name,_)|name==head)||matches!(self.kind,OriginalParsedCommandKind::Command)&&!fields.iter().any(|(name,_)|name==key){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original command envelope is missing a declared field"))}
   if matches!(self.kind,OriginalParsedCommandKind::Artifact)&&!matches!(envelope.get("action"),Some(DslValue::String(_))){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original artifact action must be a string"))}
   let needs_address=matches!(self.kind,OriginalParsedCommandKind::Command)&&self.action_address.is_none()&&self.command_address.is_none();
   if matches!(self.kind,OriginalParsedCommandKind::Command){
    if !self.args_extracted&&!matches!(envelope.get("arguments"),Some(DslValue::Object(_))){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original command arguments must be an object"))}
    if needs_address&&!envelope.get("address").is_some_and(|address|original_command_address(address,&mut work)){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original command address has undeclared or missing fields"))}
   }
   control.advance(work)?;control.checkpoint()?;
   if self.view_source.is_some()&&self.view.is_none(){return Ok(false)}
   let address_bytes=if needs_address{let address=envelope.get("address").unwrap();if address.get("windowInstanceId").is_some(){size_of::<semio_framework::manifest::ActionAddress>()}else{size_of::<semio_framework::manifest::CommandAddress>()}}else{0};
   let argument_bytes=if envelope.get(key).is_some(){size_of::<DslValue>()}else{size_of::<bool>()};
   let bytes=address_bytes+if self.args_extracted{0}else{argument_bytes};let items=usize::from(needs_address)+usize::from(!self.args_extracted);
   if grant.maximum_items<items||grant.maximum_copy_bytes<bytes||items>0&&grant.maximum_depth==0{return Ok(false)}
   if needs_address{
    let address=original_field_mut(self.envelope.as_mut().unwrap(),"address");
    if address.get("windowInstanceId").is_some(){self.action_address=Some(semio_framework::manifest::ActionAddress{plugin_id:original_take_text(address,"pluginId"),app_id:original_take_text(address,"appId"),mode_id:original_take_text(address,"modeId"),window_kind_id:original_take_text(address,"windowKindId"),window_instance_id:original_take_text(address,"windowInstanceId"),action_id:original_take_text(address,"actionId")});}
    else{
     let command_id=original_take_text(address,"commandId");let source=original_field_mut(address,"owner");
     let owner=if matches!(source,DslValue::String(name)if name=="os"){semio_framework::manifest::CommandOwnerAddress::Os}else{
      let DslValue::Object(fields)=source else{unreachable!("original closed owner was validated")};
      let(tag,value)=&mut fields[0];match tag.as_str(){"plugin"=>semio_framework::manifest::CommandOwnerAddress::Plugin{plugin_id:original_take_text(value,"pluginId")},"app"=>semio_framework::manifest::CommandOwnerAddress::App{plugin_id:original_take_text(value,"pluginId"),app_id:original_take_text(value,"appId")},"mode"=>semio_framework::manifest::CommandOwnerAddress::Mode{plugin_id:original_take_text(value,"pluginId"),app_id:original_take_text(value,"appId"),mode_id:original_take_text(value,"modeId")},_=>unreachable!("original closed owner was validated")}
     };self.command_address=Some(semio_framework::manifest::CommandAddress{owner,command_id});
    }
    progress.copied_items+=1;progress.copied_bytes+=address_bytes;work+=1;
   }
   if !self.args_extracted{
    let DslValue::Object(fields)=self.envelope.as_mut().unwrap()else{unreachable!("original envelope was validated")};
    if let Some((_,value))=fields.iter_mut().find(|(name,_)|name==key){self.args=Some(std::mem::replace(value,DslValue::Null));}
    self.args_extracted=true;progress.copied_items+=1;progress.copied_bytes+=argument_bytes;work+=1;
   }
   Ok(true)
  })();
  semio_framework_job_native_decode::OriginalNativeDecodeIssued{value,grant,progress,work:work as u64}
 }
}

fn original_field_mut<'a>(value:&'a mut DslValue,key:&str)->&'a mut DslValue{let DslValue::Object(fields)=value else{unreachable!("original closed object was validated")};&mut fields.iter_mut().find(|(name,_)|name==key).expect("original declared field exists").1}

fn original_take_text(value:&mut DslValue,key:&str)->String{let DslValue::String(text)=original_field_mut(value,key)else{unreachable!("original closed text was validated")};std::mem::take(text)}

#[doc(hidden)]
pub fn original_fields(value:&DslValue,keys:&[&str],strings:bool,work:&mut usize)->bool{
 *work+=1;let DslValue::Object(fields)=value else{return false};if fields.len()!=keys.len(){return false}
 for key in keys{let mut count=0;for(name,value)in fields{*work+=1;if name==key{if strings&&!matches!(value,DslValue::String(_)){return false}count+=1;}}if count!=1{return false}}
 true
}

fn original_command_address(value:&DslValue,work:&mut usize)->bool{
 if original_fields(value,&["pluginId","appId","modeId","windowKindId","windowInstanceId","actionId"],true,work){return true}
 if !original_fields(value,&["owner","commandId"],false,work)||!matches!(value.get("commandId"),Some(DslValue::String(_))){return false}
 let Some(owner)=value.get("owner")else{return false};if matches!(owner,DslValue::String(name)if name=="os"){return true}
 let DslValue::Object(fields)=owner else{return false};if fields.len()!=1{return false}
 match fields[0].0.as_str(){"plugin"=>original_fields(&fields[0].1,&["pluginId"],true,work),"app"=>original_fields(&fields[0].1,&["pluginId","appId"],true,work),"mode"=>original_fields(&fields[0].1,&["pluginId","appId","modeId"],true,work),_=>false}
}
