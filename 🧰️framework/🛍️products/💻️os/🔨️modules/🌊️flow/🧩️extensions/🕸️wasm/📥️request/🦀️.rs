//! 📥️ The original invocation borrows caller octets and retains every native grammar owner.
use super::*;
use semio_framework_pack_json::{JsonGrammarCursor,JsonReadLimits,JsonSourceCursor,JsonMemberPolicy,Value,Number};
use semio_framework_value_native_owned::native_decoding::NativeDecodeContinuation;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

#[path="🎟️context/🦀️.rs"] mod original_context;
pub use original_context::EvaluationRequestContextInput;
use original_context::EvaluationRequestContext;

/// 🔎️ Key comparison borrows one stable registry slot under the unchanged caller authority.
#[derive(Default)]
struct EvaluationSlotLookup{epoch:Option<u64>,slot:usize,position:usize,selected:Option<usize>,done:bool}
impl EvaluationSlotLookup{
 fn step(&mut self,jobs:&EvaluationJobRegistry,identity:neural_engine::RegistryIdentity,request:&EvaluateRequest,grant:RetainedCloneGrant)->Result<(Option<Option<usize>>,RetainedCloneProgress),ValueError>{
  let empty=RetainedCloneProgress::default();if grant.maximum_items==0{return Ok((None,empty))}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original key lookup requires admitted depth"))}
  let item=RetainedCloneProgress{copied_items:1,..empty};if self.epoch!=Some(jobs.lookup_epoch){self.epoch=Some(jobs.lookup_epoch);self.slot=0;self.position=0;self.selected=None;self.done=false;return Ok((None,item))}
  if self.done{return Ok((Some(self.selected),empty))}if self.slot>=jobs.jobs.slot_count(){self.done=true;return Ok((Some(None),item))}
  if grant.maximum_depth<jobs.jobs.next_slot_depth_demand(self.slot)?{return Ok((None,empty))}
  let Some((key,_))=jobs.jobs.slot_entry(self.slot)else{self.slot+=1;self.position=0;return Ok((None,item))};
  if key.0!=identity||key.2!=request.node_hash||key.1.len()!=request.operator_id.len(){self.slot+=1;self.position=0;return Ok((None,item))}
  let expected=key.1.as_bytes();let actual=request.operator_id.as_bytes();if self.position==expected.len(){self.selected=Some(self.slot);self.done=true;return Ok((Some(self.selected),item))}
  let count=grant.maximum_copy_bytes.min(64).min(expected.len()-self.position);if count==0{return Ok((None,empty))}let mut copied=[0u8;64];copied[..count].copy_from_slice(&expected[self.position..self.position+count]);let equal=copied[..count]==actual[self.position..self.position+count];self.position+=count;if !equal{self.slot+=1;self.position=0;}else if self.position==expected.len(){self.selected=Some(self.slot);self.done=true;}Ok((if self.done{Some(self.selected)}else{None},RetainedCloneProgress{copied_bytes:count,..item}))
 }
}

type EvaluationOriginalInsertion=protocol::HistoryFoldIndexInsert<(neural_engine::RegistryIdentity,String,u64),EvaluateRequest>;

fn original_insertion_compare(source:&(neural_engine::RegistryIdentity,String,u64),candidate:&(neural_engine::RegistryIdentity,String,u64),position:usize,grant:RetainedCloneGrant)->Result<(Option<std::cmp::Ordering>,usize,RetainedCloneProgress),ValueError>{
 let empty=RetainedCloneProgress::default();if grant.maximum_items==0||grant.maximum_depth==0{return Ok((None,position,empty))}let item=RetainedCloneProgress{copied_items:1,..empty};let identity=source.0.cmp(&candidate.0);if identity!=std::cmp::Ordering::Equal{return Ok((Some(identity),0,item))}
 let original=source.1.as_bytes();let expected=candidate.1.as_bytes();let length=original.len().min(expected.len());if position>=length{return Ok((Some(original.len().cmp(&expected.len()).then_with(||source.2.cmp(&candidate.2))),position,item))}
 let count=grant.maximum_copy_bytes.min(64).min(length-position);if count==0{return Ok((None,position,empty))}let mut copied=[0;64];copied[..count].copy_from_slice(&expected[position..position+count]);let order=original[position..position+count].cmp(&copied[..count]);let position=position+count;let order=if order!=std::cmp::Ordering::Equal{Some(order)}else if position==length{Some(original.len().cmp(&expected.len()).then_with(||source.2.cmp(&candidate.2)))}else{None};Ok((order,position,RetainedCloneProgress{copied_bytes:count,..item}))
}

/// 🪪️ One bounded invocation keeps the exact caller source until its paid response exists.
pub struct EvaluationInvokeCursor {
    operation:u64,generation:u64,pointer:usize,length:usize,
    parser:Option<JsonGrammarCursor<Value>>,continuation:Option<NativeDecodeContinuation>,
    root:Option<Value>,request:Option<EvaluateRequest>,reply_pending:Option<neural_engine::PendingExtensionEval>,active:Option<Box<dyn ErasedSnapshotRetirement>>,
    field:usize,result_field:usize,result_required:u8,required:u8,decoded:bool,normal_context_posted:bool,normal_progress:RetainedCloneProgress,
    pub slot:Option<usize>,lookup:Option<EvaluationSlotLookup>,insertion:Option<EvaluationOriginalInsertion>,insertion_registry:Option<neural_engine::SharedRegistry>,round_units:u64,
    suppressed_wire:Option<EvaluationWire>,ready_wire:Option<EvaluationWire>,ready_complete:bool,ready_faulted:bool,ready_external:bool,wait_terminal:bool,fault:Option<EvaluationFailure>,close_failure:Option<original_failure::EvaluationCloseFailure>,response:Option<EvaluationOutput>,response_failed:bool,context:Option<EvaluationRequestContext>,context_prepared:bool,closing:bool,
}
impl EvaluationInvokeCursor {
    pub fn new(operation:u64,generation:u64,request:&[u8])->Self {
        Self {operation,generation,pointer:request.as_ptr()as usize,length:request.len(),parser:None,continuation:None,root:None,request:None,reply_pending:None,active:None,field:0,result_field:0,result_required:0,required:0,decoded:false,normal_context_posted:false,normal_progress:Default::default(),slot:None,lookup:None,insertion:None,insertion_registry:None,round_units:0,suppressed_wire:None,ready_wire:None,ready_complete:false,ready_faulted:false,ready_external:false,wait_terminal:false,fault:None,close_failure:None,response:None,response_failed:false,context:None,context_prepared:false,closing:false}
    }
    /// 🧾️ Reports whether this exact last step already posted its original context receipt.
    pub fn normal_context_posted(&self)->bool{self.normal_context_posted}
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.normal_progress}
    pub fn matches(&self,operation:u64,generation:u64,request:&[u8])->bool{self.operation==operation&&self.generation==generation&&self.pointer==request.as_ptr()as usize&&self.length==request.len()}
    pub fn begin_close(&mut self){self.closing=true;}
    pub fn terminal_is_empty(&self)->bool{self.ready_wire.is_none()&&self.suppressed_wire.is_none()&&self.context.is_none()&&self.parser.is_none()&&self.continuation.is_none()&&self.root.is_none()&&self.request.is_none()&&self.reply_pending.is_none()&&self.active.is_none()&&self.fault.is_none()&&self.close_failure.is_none()&&self.response.is_none()&&self.slot.is_none()&&self.lookup.is_none()&&self.insertion.is_none()&&self.insertion_registry.is_none()}
    pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(context)=&self.context{return context.retirement_demands(copy);}
        if self.active.is_some(){return evaluation_child_demands(&self.active,copy);}
        if self.parser.is_some(){return Ok(evaluation_owned_birth::<JsonGrammarCursor<Value>>());}
        if self.root.is_some(){return Ok(evaluation_owned_birth::<Value>());}
        if self.ready_wire.is_some()||self.suppressed_wire.is_some(){return Ok(evaluation_owned_birth::<EvaluationWire>());}
        if self.insertion.is_some(){return Ok(evaluation_owned_birth::<EvaluationOriginalInsertion>());}
        if self.insertion_registry.is_some(){return Ok(evaluation_owned_birth::<neural_engine::SharedRegistry>());}
        if self.request.is_some(){return Ok(evaluation_owned_birth::<EvaluateRequest>());}
        if self.reply_pending.is_some(){return Ok(evaluation_owned_birth::<neural_engine::PendingExtensionEval>());}
        if self.close_failure.is_some(){return Ok(evaluation_owned_birth::<original_failure::EvaluationCloseFailure>());}if self.fault.is_some(){return Ok(evaluation_owned_birth::<EvaluationFailure>());}
        if let Some(response)=&self.response{return response.retirement_demands(copy);}
        Ok(RetirementDemand{depth:usize::from(self.continuation.is_some()||self.slot.is_some()||self.lookup.is_some()),..Default::default()})
    }
    pub fn retirement_demands_with_registry(&self,registry:&neural_engine::SharedRegistry,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(slot)=self.slot{let jobs=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);if let Some((key,owner))=jobs.jobs.slot_entry(slot){if key.0!=registry.owner_identity(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original invocation close found another registry owner"));}if !owner.close_fields_terminal(false,false){return owner.retirement_demands(copy,false,false);}}return Ok(RetirementDemand{depth:1,..Default::default()});}
        self.retirement_demands(copy)
    }
    pub fn close_step_with_registry(&mut self,registry:&neural_engine::SharedRegistry,grant:RetainedCloneGrant)->Result<RetainedCloneStep,EvaluationFailure>{
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(slot)=self.slot{let mut jobs=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);if let Some((key,owner))=jobs.jobs.slot_entry_mut(slot){if key.0!=registry.owner_identity(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original invocation close found another registry owner").into());}if !owner.close_fields_terminal(false,false){owner.cancel();return owner.close_step(grant,false,false);}}self.slot=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        self.close_step(grant)
    }
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,EvaluationFailure>{
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(context)=&mut self.context{let step=context.close_step(grant)?;if context.terminal_is_empty(){self.context=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if self.active.is_some(){return evaluation_child_close(&mut self.active,grant).map_err(Into::into);}
        if self.parser.is_some(){return evaluation_admit_original(&mut self.parser,&mut self.active,grant).map_err(Into::into);}
        if self.root.is_some(){return evaluation_admit_original(&mut self.root,&mut self.active,grant).map_err(Into::into);}
        if self.ready_wire.is_some(){return evaluation_admit_original(&mut self.ready_wire,&mut self.active,grant).map_err(Into::into);}
        if self.suppressed_wire.is_some(){return evaluation_admit_original(&mut self.suppressed_wire,&mut self.active,grant).map_err(Into::into);}
        if self.insertion.is_some(){return evaluation_admit_original(&mut self.insertion,&mut self.active,grant).map_err(Into::into);}
        if self.insertion_registry.is_some(){return evaluation_admit_original(&mut self.insertion_registry,&mut self.active,grant).map_err(Into::into);}
        if self.request.is_some(){return evaluation_admit_original(&mut self.request,&mut self.active,grant).map_err(Into::into);}
        if self.reply_pending.is_some(){return evaluation_admit_original(&mut self.reply_pending,&mut self.active,grant).map_err(Into::into);}
        if self.close_failure.is_some(){return evaluation_admit_original(&mut self.close_failure,&mut self.active,grant).map_err(Into::into);}if self.fault.is_some(){return evaluation_admit_original(&mut self.fault,&mut self.active,grant).map_err(Into::into);}
        if let Some(response)=&mut self.response{let step=response.close_step(grant)?;if response.terminal_is_empty(){self.response=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if self.continuation.is_some()||self.slot.is_some()||self.lookup.is_some(){
            if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original invocation metadata requires admitted depth").into());}
            if self.continuation.is_some(){self.continuation=None;}else if self.lookup.is_some(){self.lookup=None;}else{self.slot=None;}
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    pub fn original_request(&self)->Option<&EvaluateRequest>{self.request.as_ref().or_else(||self.insertion.as_ref().and_then(|owner|owner.original().map(|(_,request)|request)))}
    fn fail(&mut self,fault:EvaluationFailure){assert!(self.fault.is_none());self.fault=Some(fault);self.closing=true;}
    fn retain_reply(&mut self,mut reply:EvaluationReply)->Option<EvaluationReply>{
        assert!(self.request.is_none()&&self.reply_pending.is_none());self.request=reply.returned_request.take();self.reply_pending=reply.pending_source.take();
        if let Some(fault)=reply.refusal.take(){
            if self.request.is_some(){self.slot=None;}
            self.ready_wire=reply.wire.take();self.fail(fault);return None;
        }
        if reply.wire.is_none(){return None;}
        if let Some(returned)=self.request.as_ref(){
            if returned.resume&&returned.external_result.is_none()&&(!self.wait_terminal||reply.complete||reply.external_pending){self.ready_wire=reply.wire.take();self.ready_complete=reply.complete;self.ready_faulted=reply.faulted;self.ready_external=reply.external_pending;return None;}
            self.suppressed_wire=reply.wire.take();return None;
        }
        if self.wait_terminal&&!reply.complete&&!reply.external_pending{self.suppressed_wire=reply.wire.take();return None;}
        if self.reply_pending.is_some(){self.ready_wire=reply.wire.take();self.ready_complete=reply.complete;self.ready_faulted=reply.faulted;self.ready_external=reply.external_pending;return None;}
        self.slot=None;Some(reply)
    }
    fn decode_field(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,EvaluationFailure>{
        let empty=RetainedCloneProgress::default();
        if grant.maximum_items==0{return Ok(empty);}
        if grant.maximum_depth<2{return Err("original request fields require admitted object depth".into());}
        let Value::Object(root)=self.root.as_mut().ok_or(EvaluationFailure::Literal("original request root is absent"))? else{return Err("original request requires an object".into());};
        if self.request.is_none(){self.request=Some(EvaluateRequest{neuron_id:String::new(),operator_id:String::new(),input_json:String::new(),node_hash:0,dependency_json:String::new(),operator_version:String::new(),cancellation_id:String::new(),window_id:String::new(),window_kind_id:String::new(),extension_id:String::new(),round_units:0,resume:false,external_result:None});return Ok(RetainedCloneProgress{copied_items:1,..empty});}
        let Some((key,value))=root.iter_mut().nth(self.field)else{
            if self.required!=7{return Err("original request requires neuronId, operatorId and inputJson".into());}
            self.decoded=true;self.closing=true;return Ok(RetainedCloneProgress{copied_items:1,..empty});
        };
        let request=self.request.as_mut().unwrap();
        if key=="externalResult"{
            let Value::Object(fields)=value else{return Err("external result requires an object".into());};
            if request.external_result.is_none(){request.external_result=Some(EvaluationExternalResult{neuron_id:String::new(),extension_id:String::new(),operator_id:String::new(),node_hash:0,output:DslValue::Null});return Ok(RetainedCloneProgress{copied_items:1,..empty});}
            let Some((name,value))=fields.iter_mut().nth(self.result_field)else{if self.result_required!=31{return Err("external result requires all five original fields".into());}self.field+=1;self.result_field=0;return Ok(RetainedCloneProgress{copied_items:1,..empty});};
            let original=request.external_result.as_mut().unwrap();
            if name=="nodeHash"{let Value::Number(Number::UInt(hash))=value else{return Err("external result node hash requires an unsigned integer".into());};original.node_hash=*hash;self.result_required|=8;}
            else if name=="output"{if !matches!(value,Value::Object(_)){return Err("external result output requires a typed Dictionary object".into())}original.output=semio_framework_pack_json::to_dsl_value(value);*value=Value::Null;self.result_required|=16;}
            else{let(bit,destination)=match &*name{"neuronId"=>(1,&mut original.neuron_id),"extensionId"=>(2,&mut original.extension_id),"operatorId"=>(4,&mut original.operator_id),_=>return Err("external result contains an unknown field".into())};let Value::String(text)=value else{return Err("external result string field has a different type".into());};if text.is_empty(){return Err("external result identity must be nonempty".into());}let Value::String(text)=std::mem::replace(value,Value::Null)else{unreachable!()};*destination=text;self.result_required|=bit;}
            self.result_field+=1;return Ok(RetainedCloneProgress{copied_items:1,..empty});
        }
        let string=match &*key{"neuronId"=>{self.required|=1;Some(&mut request.neuron_id)},"operatorId"=>{self.required|=2;Some(&mut request.operator_id)},"inputJson"=>{self.required|=4;Some(&mut request.input_json)},"dependencyJson"=>Some(&mut request.dependency_json),"operatorVersion"=>Some(&mut request.operator_version),"cancellationId"=>Some(&mut request.cancellation_id),"windowId"=>Some(&mut request.window_id),"windowKindId"=>Some(&mut request.window_kind_id),"extensionId"=>Some(&mut request.extension_id),_=>None};
        if let Some(destination)=string{
            if !matches!(value,Value::String(_)){return Err("original request string field has a different type".into());}
            if (key=="neuronId"||key=="operatorId")&&matches!(value,Value::String(text)if text.is_empty()){return Err("original request identity must be nonempty".into());}
            let Value::String(original)=std::mem::replace(value,Value::Null)else{unreachable!()};*destination=original;
        }else if key=="resume"{let Value::Bool(original)=value else{return Err("original resume requires a boolean".into());};request.resume=*original;}
        else{
            let Value::Number(Number::UInt(original))=value else{return Err("original request numeric field requires an unsigned integer".into());};
            match &*key{"nodeHash"=>request.node_hash=*original,"roundUnits"=>request.round_units=*original,_=>return Err("original request contains an unknown field".into())}
        }
        self.field+=1;Ok(RetainedCloneProgress{copied_items:1,..empty})
    }
    fn insertion_step(&mut self,registry:&neural_engine::SharedRegistry,jobs:&mut EvaluationJobRegistry,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        let empty=RetainedCloneProgress::default();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(empty)}
        if self.insertion_registry.is_none(){self.insertion_registry=Some(registry.clone());return Ok(RetainedCloneProgress{copied_items:1,..empty})}
        let insertion=self.insertion.as_mut().unwrap();let(ready,receipt)=insertion.step(&mut jobs.jobs,grant,original_insertion_compare)?;if !ready{return Ok(receipt)}
        let remaining=evaluation_remaining_grant(grant,receipt);let(slot,committed)=insertion.commit_with(&mut jobs.jobs,remaining,|request|RetainedEvaluation::new(self.insertion_registry.take().unwrap(),request))?;let physical=receipt.checked_add(committed)?;
        if let Some(slot)=slot{self.slot=Some(slot);self.insertion=None;jobs.admission_owner=None;jobs.lookup_epoch=jobs.lookup_epoch.checked_add(1).expect("original registry inserted key epoch");}
        Ok(physical)
    }
    pub fn step(&mut self,registry:&neural_engine::SharedRegistry,request:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<(Option<EvaluationReply>,RetainedCloneProgress),EvaluationFailure>{self.step_recorded(registry,request,cx,None)}
    pub fn step_with_context(&mut self,registry:&neural_engine::SharedRegistry,request:&[u8],cx:&mut semio_framework_job::StepContext<'_>,context:EvaluationRequestContextInput<'_>)->Result<(Option<EvaluationReply>,RetainedCloneProgress),EvaluationFailure>{self.step_recorded(registry,request,cx,Some(context))}
    fn step_recorded(&mut self,registry:&neural_engine::SharedRegistry,request:&[u8],cx:&mut semio_framework_job::StepContext<'_>,context:Option<EvaluationRequestContextInput<'_>>)->Result<(Option<EvaluationReply>,RetainedCloneProgress),EvaluationFailure>{
        let before=cx.retained_progress();self.normal_progress=Default::default();let result=self.step_original(registry,request,cx,context);
        self.normal_progress=match &result{Ok((_,progress))=>*progress,Err(original)if !self.normal_context_posted=>original.retained_progress(),Err(_)=>{let after=cx.retained_progress();RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}}};result
    }
    fn step_original(&mut self,registry:&neural_engine::SharedRegistry,request:&[u8],cx:&mut semio_framework_job::StepContext<'_>,context_input:Option<EvaluationRequestContextInput<'_>>)->Result<(Option<EvaluationReply>,RetainedCloneProgress),EvaluationFailure>{
        self.normal_context_posted=false;

        let available=cx.retained_grant();let grant=RetainedCloneGrant{maximum_items:available.maximum_items.min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX)),..available};let cancelled=cx.is_cancelled();let empty=RetainedCloneProgress::default();
        if (self.operation,self.generation)!=(cx.operation().0,cx.generation().0){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original evaluation cursor belongs to another caller").into())}
        if cx.should_yield(){return Ok((None,empty))}
        if self.pointer!=request.as_ptr()as usize||self.length!=request.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation source changed while pending").into());}
        if self.close_failure.is_some(){let before=cx.retained_progress();self.normal_context_posted=true;let admitted=match semio_framework_job::admit_original_domain_failure(&mut self.close_failure,cx){Ok(value)=>value,Err(original)=>Some(original)};let after=cx.retained_progress();let progress=RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes};return Ok((admitted.map(|original|EvaluationReply{retirement_posted:true,external_pending:false,pending_source:None,wire:None,complete:false,faulted:true,retirement_progress:progress,returned_request:None,refusal:Some(EvaluationFailure::Original(original))}),progress))}
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok((None,empty));}
        if let Some(input)=context_input.as_ref(){self.wait_terminal=input.wait_terminal;}
        if self.ready_wire.is_some()&&!cancelled&&!self.closing{
            if self.active.is_some()||self.request.is_some()||self.reply_pending.is_some(){let step=if self.active.is_some(){evaluation_child_close(&mut self.active,grant)}else if self.request.is_some(){evaluation_admit_original(&mut self.request,&mut self.active,grant)}else{evaluation_admit_original(&mut self.reply_pending,&mut self.active,grant)}?;return Ok((None,step.progress()));}
            self.slot=None;return Ok((Some(EvaluationReply{retirement_posted:false,external_pending:self.ready_external,pending_source:None,wire:self.ready_wire.take(),complete:self.ready_complete,faulted:self.ready_faulted,retirement_progress:Default::default(),returned_request:None,refusal:None}),RetainedCloneProgress{copied_items:1,..empty}));
        }
        if self.suppressed_wire.is_some()||(!self.closing&&self.active.is_some()){let step=if self.active.is_some(){evaluation_child_close(&mut self.active,grant)}else{evaluation_admit_original(&mut self.suppressed_wire,&mut self.active,grant)}?;return Ok((None,step.progress()));}
        if cancelled&&!self.closing{if let Some(slot)=self.slot{let mut jobs=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);if let Some((_,owner))=jobs.jobs.slot_entry_mut(slot){owner.cancel();}}self.fail(EvaluationFailure::Literal("original invocation cancelled"));}
        if let Some(response)=&mut self.response{
            if self.response_failed{let step=response.close_step(grant)?;if response.terminal_is_empty(){self.response=None;self.response_failed=false;}return Ok((None,step.progress()));}
            let result=response.step(1,0,grant);let receipt=response.normal_step_progress();
            return match result{Ok((Some(reply),_))=>{assert!(response.terminal_is_empty());self.response=None;self.slot=None;Ok((Some(reply),receipt))},Ok((None,_))=>Ok((None,receipt)),Err(fault)=>{self.response_failed=true;self.fail(fault);Ok((None,receipt))}};
        }
        if self.closing{
            if self.fault.is_some(){if let Some(slot)=self.slot{let mut jobs=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);if let Some((_,owner))=jobs.jobs.slot_entry_mut(slot){owner.cancel();let step=owner.close_step(grant,false,false)?;if owner.close_fields_terminal(false,false){self.slot=None;}return Ok((None,step.progress()));}self.slot=None;return Ok((None,RetainedCloneProgress{copied_items:1,..empty}));}}
            if self.fault.is_some()&&self.context.is_some(){let context=self.context.as_mut().unwrap();let step=context.close_step(grant)?;if context.terminal_is_empty(){self.context=None;}return Ok((None,step.progress()));}
            if self.active.is_some()||self.parser.is_some()||self.root.is_some()||self.continuation.is_some()||self.lookup.is_some(){
                let step=if self.active.is_some(){evaluation_child_close(&mut self.active,grant)}else if self.parser.is_some(){evaluation_admit_original(&mut self.parser,&mut self.active,grant)}else if self.root.is_some(){evaluation_admit_original(&mut self.root,&mut self.active,grant)}else{if self.continuation.is_some(){self.continuation=None;}else{self.lookup=None;}Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))};
                return step.map(|step|(None,step.progress())).map_err(Into::into);
            }
            if self.fault.is_some(){
                if self.ready_wire.is_some(){return evaluation_admit_original(&mut self.ready_wire,&mut self.active,grant).map(|step|(None,step.progress())).map_err(Into::into);}
                if self.request.is_some(){return evaluation_admit_original(&mut self.request,&mut self.active,grant).map(|step|(None,step.progress())).map_err(Into::into);}
                if self.insertion.is_some(){return evaluation_admit_original(&mut self.insertion,&mut self.active,grant).map(|step|(None,step.progress())).map_err(Into::into);}
                if self.insertion_registry.is_some(){return evaluation_admit_original(&mut self.insertion_registry,&mut self.active,grant).map(|step|(None,step.progress())).map_err(Into::into);}
                if self.reply_pending.is_some(){return evaluation_admit_original(&mut self.reply_pending,&mut self.active,grant).map(|step|(None,step.progress())).map_err(Into::into);}
                let before=cx.retained_progress();let admitted=match semio_framework_job::admit_original_domain_failure(&mut self.fault,cx){Ok(failure)=>failure,Err(failure)=>Some(failure)};
                self.normal_context_posted=true;let after=cx.retained_progress();let receipt=RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes};
                return Ok((admitted.map(|original|EvaluationReply{retirement_posted:true,external_pending:false,pending_source:None,wire:None,complete:false,faulted:true,retirement_progress:receipt,returned_request:None,refusal:Some(EvaluationFailure::Original(original))}),receipt));
            }
            self.closing=false;
        }
        if self.insertion.is_some(){let policy=grant;let mut jobs=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);let result=self.insertion_step(registry,&mut jobs,policy);drop(jobs);return match result{Ok(receipt)=>Ok((None,receipt)),Err(error)=>{let receipt=error.retained_progress();self.fail(error.into());Ok((None,receipt))}};}
        if self.decoded&&self.request.is_some(){
            if !self.context_prepared{if let Some(input)=context_input{
                if self.context.is_none(){self.context=Some(EvaluationRequestContext::new());return Ok((None,RetainedCloneProgress{copied_items:1,..empty}));}
                let context=self.context.as_mut().unwrap();let result=context.step(self.request.as_mut().unwrap(),input,cx);let receipt=context.normal_step_progress();
                match result{Ok(true)=>{assert!(context.terminal_is_empty());self.context=None;self.context_prepared=true;},Ok(false)=>{},Err(fault)=>self.fail(fault)}
                return Ok((None,receipt));
            }}
            if self.lookup.is_none(){self.lookup=Some(EvaluationSlotLookup::default());return Ok((None,RetainedCloneProgress{copied_items:1,..empty}));}
            let policy=grant;let mut jobs=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);let lookup=self.lookup.as_mut().unwrap();
            let(existing,physical)=lookup.step(&jobs,registry.owner_identity(),self.request.as_ref().unwrap(),policy)?;let Some(existing)=existing else{return Ok((None,physical))};
            let remaining=evaluation_remaining_grant(policy,physical);if remaining.maximum_items==0{return Ok((None,physical))}
            let mut original=self.request.take().unwrap();self.round_units=original.round_units;
            if existing.is_none(){
                if original.resume||original.external_result.is_some(){drop(jobs);let reply=EvaluationReply::refused(original,physical,"evaluation continuation has no retained owner");return Ok((self.retain_reply(reply),physical))}
                if jobs.closing_owner.is_some()||jobs.jobs.len()>=EVALUATION_JOB_CAPACITY{drop(jobs);let reply=EvaluationReply::refused(original,physical,"evaluation capacity remains owned by pending work");return Ok((self.retain_reply(reply),physical))}
                let key=(registry.owner_identity(),std::mem::take(&mut original.operator_id),original.node_hash);self.insertion=Some(protocol::HistoryFoldIndexInsert::new(key,original));self.lookup=None;if jobs.jobs.is_empty(){jobs.admission_owner=Some(registry.owner_identity());}drop(jobs);return Ok((None,physical.checked_add(RetainedCloneProgress{copied_items:1,..empty})?))
            }
            let reply=evaluation_admit_original_request(registry,original,&mut jobs,existing,physical,&mut self.slot,cx);let receipt=reply.retirement_progress;self.lookup=None;drop(jobs);
            return Ok((self.retain_reply(reply),receipt));
        }
        if let Some(slot)=self.slot{
            let policy=grant;let mut jobs=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let reply=evaluation_drive_original(registry,&mut jobs,slot,policy,self.round_units,empty,None,cx);let receipt=reply.retirement_progress;self.normal_context_posted=reply.retirement_posted;drop(jobs);return Ok((self.retain_reply(reply),receipt));
        }
        if self.root.is_some(){let result=self.decode_field(grant);return match result{Ok(receipt)=>Ok((None,receipt)),Err(fault)=>{self.fail(fault);Ok((None,RetainedCloneProgress{copied_items:1,..empty}))}};}
        if self.parser.is_none(){let source=JsonSourceCursor::<_,Value>::new(request,JsonMemberPolicy::Reject,JsonReadLimits{maximum_bytes:request.len()as u64,maximum_allocation_bytes:grant.maximum_capacity_bytes,maximum_depth:grant.maximum_depth,maximum_items:request.len()as u64})?;self.parser=Some(source.into_grammar().1);return Ok((None,RetainedCloneProgress{copied_items:1,..empty}));}
        let mut callback=|_|!cx.should_yield()&&!cx.is_cancelled();
        let mut control=match self.continuation.take(){Some(original)=>NativeDecodeControl::resume(original,&mut callback)?,None=>NativeDecodeControl::new_retained(&mut callback)};
        let mut source=JsonSourceCursor::from_grammar(request,self.parser.take().unwrap());
        let result=source.step(1,&mut control,grant);let receipt=source.normal_step_progress();self.parser=Some(source.into_grammar().1);self.continuation=Some(control.pause().map_err(|error|error.with_retained_progress(receipt))?);
        match result{Ok(Some(original))=>self.root=Some(original),Ok(None)=>{},Err(fault)=>self.fail(fault.into())};Ok((None,receipt))
    }
}

#[cfg(test)]
mod tests{
 use super::*;
 const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:134217728,maximum_capacity_bytes:268435456,maximum_release_bytes:536870912,maximum_depth:4096};
 /// 🎟️ One original fixture caller whose paid turns admit the registry of a fresh extension resource owner.
 fn admitted(registry:Registry)->(ExtensionEvaluationResources,semio_framework_job::OriginalJobFixtureCaller){
  let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(71),semio_framework_job::Generation(3),STOCK,||Some(1000),1500,65536);let mut resources=ExtensionEvaluationResources::new(registry);
  for turn in 0..65536{if caller.turn(STOCK,|cx|resources.admit_registry_on_original(cx)).unwrap()==Some(true){return(resources,caller)}assert!(turn<65535,"original registry admission stalled")}
  unreachable!()
 }
 /// ♻️ Closes the resource owner and its caller source root under paid turns of the same fixture caller.
 fn close_admitted(mut resources:ExtensionEvaluationResources,mut caller:semio_framework_job::OriginalJobFixtureCaller,copy:usize){
  use semio_framework_plugin::ExtensionResourceOwner;resources.begin_close();
  for turn in 0..65536{if resources.terminal_is_empty(){break}caller.turn(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,..STOCK},|cx|resources.close_step(cx)).unwrap();assert!(turn<65535,"original resource close stalled")}
  assert!(resources.terminal_is_empty());drop(resources);let stock=caller.stock();caller.retire(stock);
 }
 /// 🪴️ The same insertion source survives fixed-copy comparisons, real mutations and exact paid slot transfer.
 #[test]
 fn original_sdk_indexed_insertion_keeps_source_and_actual_slot(){
  use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;use semio_framework_value::retirement::controlled::ControlledRetirement;
  for copy in [1,3,64]{
   let(resources,owner_caller)=admitted(Registry::new());let identity=resources.registry().unwrap().owner_identity();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};
   let make=|name:&str|EvaluateRequest{neuron_id:"original insertion neuron".into(),operator_id:String::new(),input_json:name.into(),node_hash:17,dependency_json:String::new(),operator_version:"v1".into(),cancellation_id:"original insertion cancellation".into(),window_id:String::new(),window_kind_id:String::new(),extension_id:String::new(),round_units:1,resume:false,external_result:None};
   let((mut index,mut insertion),source)=observe(||{let mut index=protocol::HistoryFoldIndex::new();for name in ["alpha","omega","zeta"]{index.insert((identity,name.to_owned(),17),make(name));}let insertion=protocol::HistoryFoldIndexInsert::new((identity,"original insertion 源".to_owned(),17),make("original source body 源"));(index,insertion)});let key_pointer=insertion.original().unwrap().0.1.as_ptr();let body_pointer=insertion.original().unwrap().1.input_json.as_ptr();let mut allocated=source.requested_bytes;let mut freed=source.released_bytes;let mut found=None;let mut mutated=false;
   for turn in 0..100000{
    assert_eq!(insertion.original().unwrap().0.1.as_ptr(),key_pointer);assert_eq!(insertion.original().unwrap().1.input_json.as_ptr(),body_pointer);
    let(result,heap)=observe(||insertion.step(&mut index,RetainedCloneGrant{maximum_items:0,..grant},original_insertion_compare));assert_eq!(result.unwrap(),(false,Default::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let(result,heap)=observe(||insertion.step(&mut index,grant,original_insertion_compare));let(ready,receipt)=result.unwrap();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));allocated+=heap.requested_bytes;freed+=heap.released_bytes;
    if turn==3&&!mutated{let(_,heap)=observe(||{index.insert((identity,"between".to_owned(),17),make("between"));});allocated+=heap.requested_bytes;freed+=heap.released_bytes;mutated=true;}
    if ready{let(result,heap)=observe(||insertion.commit_with(&mut index,grant,|request|request));let(slot,receipt)=result.unwrap();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if let Some(slot)=slot{found=Some(slot);break}}
    assert!(turn<99999,"original insertion stalled");
   }
   let slot=found.unwrap();let(key,value)=index.slot_entry(slot).unwrap();assert_eq!(key.1.as_ptr(),key_pointer);assert_eq!(value.input_json.as_ptr(),body_pointer);assert!(insertion.terminal_is_empty());let(_,heap)=observe(||drop(insertion));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(index.len(),5);let expected=serde_json::json!(["alpha","between","omega","original insertion 源","zeta"]);assert_eq!(serde_json::to_value(index.keys().map(|key|&key.1).collect::<Vec<_>>()).unwrap(),expected);
   let(mut close,heap)=observe(||ControlledRetirement::new(index).unwrap_or_else(|_|panic!("original insertion index retirement refused")));allocated+=heap.requested_bytes;freed+=heap.released_bytes;
   for turn in 0..100000{if close.terminal_is_empty(){break}let grant=RetainedCloneGrant{maximum_capacity_bytes:close.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap(),..grant};let(result,heap)=observe(||close.step(grant));let receipt=result.unwrap().progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));allocated+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn<99999,"original insertion close stalled")}
   assert_eq!(allocated,freed);let(_,heap)=observe(||drop(close));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   for stop in [0,1,5]{
    let((mut index,mut cancelled),heap)=observe(||{let index=protocol::HistoryFoldIndex::<_,EvaluateRequest>::new();let cancelled=protocol::HistoryFoldIndexInsert::new((identity,"cancelled original key 源".to_owned(),17),make("cancelled original source 源"));(index,cancelled)});let mut allocated=heap.requested_bytes;let mut freed=heap.released_bytes;let pointer=cancelled.original().unwrap().1.input_json.as_ptr();
    for _ in 0..stop{let(result,heap)=observe(||cancelled.step(&mut index,grant,original_insertion_compare));let(_,receipt)=result.unwrap();assert_eq!(cancelled.original().unwrap().1.input_json.as_ptr(),pointer);assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));allocated+=heap.requested_bytes;freed+=heap.released_bytes;}
    let(mut close,heap)=observe(||ControlledRetirement::new((index,cancelled)).unwrap_or_else(|_|panic!("cancelled insertion source retirement refused")));allocated+=heap.requested_bytes;freed+=heap.released_bytes;
    for turn in 0..100000{if close.terminal_is_empty(){break}let grant=RetainedCloneGrant{maximum_capacity_bytes:close.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap(),..grant};let(result,heap)=observe(||close.step(RetainedCloneGrant{maximum_items:0,..grant}));assert_eq!(result.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(result,heap)=observe(||close.step(grant));let receipt=result.unwrap().progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));allocated+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn<99999,"cancelled insertion close stalled");}
    assert_eq!(allocated,freed);let(_,heap)=observe(||drop(close));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   }
   close_admitted(resources,owner_caller,copy);
  }
 }
 /// 🔎️ Exact original key bytes survive bounded lookup, structural restart and full source close.
 #[test]
 fn original_sdk_slot_lookup_keeps_same_request_under_fixed_copy(){
  for copy in [1,3,64]{
   let fixture=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};let setup=RetainedCloneGrant{maximum_copy_bytes:4096,..fixture};let(resources,owner_caller)=admitted(Registry::new());let registry=resources.registry().unwrap();let mut jobs=EvaluationJobRegistry::default();
   let make=|operator:&str|EvaluateRequest{neuron_id:"original fixture neuron".into(),operator_id:operator.into(),input_json:"original source 源".into(),node_hash:17,dependency_json:String::new(),operator_version:"v1".into(),cancellation_id:"original cancellation".into(),window_id:String::new(),window_kind_id:String::new(),extension_id:String::new(),round_units:1,resume:false,external_result:None};
   for operator in ["other operator", "original operator 源", "later operator"]{let key=(registry.owner_identity(),operator.to_string(),17);while jobs.jobs.next_insert_capacity_byte_demand(&key,setup.maximum_copy_bytes).unwrap()!=0{jobs.jobs.reserve_insert_step(&key,setup).unwrap();}let mut source=make(operator);source.operator_id.clear();let owner=RetainedEvaluation::new(registry.clone(),source);assert!(jobs.jobs.insert_reserved(key,owner,setup).unwrap_or_else(|(error,_,_)|panic!("original lookup fixture admission: {error}")).0.is_none());jobs.lookup_epoch+=1;}
   let bytes=b"original borrowed raw caller";let mut owner=EvaluationInvokeCursor::new(71,3,bytes);owner.request=Some(make("original operator 源"));owner.lookup=Some(EvaluationSlotLookup::default());let pointer=owner.request.as_ref().unwrap().input_json.as_ptr();let expected=jobs.jobs.iter().find(|(key,_)|key.1=="original operator 源").unwrap().0.1.as_ptr();
   let mut restarted=false;let mut found=None;
   for turn in 0..100000{let original=owner.request.as_ref().unwrap();assert_eq!(original.input_json.as_ptr(),pointer);let lookup=owner.lookup.as_mut().unwrap();let before=(lookup.epoch,lookup.slot,lookup.position,lookup.selected,lookup.done);let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||lookup.step(&jobs,registry.owner_identity(),original,RetainedCloneGrant{maximum_items:0,..fixture}));assert_eq!(step.unwrap(),(None,Default::default()));assert_eq!((lookup.epoch,lookup.slot,lookup.position,lookup.selected,lookup.done),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||lookup.step(&jobs,registry.owner_identity(),original,fixture));let(status,receipt)=step.unwrap();assert!(receipt.fits(fixture));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if !restarted&&lookup.position!=0{let operator="new original registry entry";let key=(registry.owner_identity(),operator.to_string(),17);while jobs.jobs.next_insert_capacity_byte_demand(&key,setup.maximum_copy_bytes).unwrap()!=0{jobs.jobs.reserve_insert_step(&key,setup).unwrap();}let mut source=make(operator);source.operator_id.clear();let retained=RetainedEvaluation::new(registry.clone(),source);assert!(jobs.jobs.insert_reserved(key,retained,setup).unwrap_or_else(|(error,_,_)|panic!("original lookup mutation admission: {error}")).0.is_none());jobs.lookup_epoch+=1;restarted=true;}else if let Some(slot)=status{found=slot;break}assert!(turn<99999,"original key lookup stalled");}
   let slot=found.expect("original key lookup found registered source");assert_eq!(jobs.jobs.slot_entry(slot).unwrap().0.1.as_ptr(),expected);assert!(restarted);
   for turn in 0..100000{let demand=owner.retirement_demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1),..fixture};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant));let receipt=step.unwrap().progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));if owner.terminal_is_empty(){break}assert!(turn<99999,"original lookup source close stalled");}
   let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));for retained in jobs.jobs.slot_values_mut(){retained.cancel();while !retained.close_fields_terminal(false,false){retained.close_step(fixture,false,false).unwrap();}}evaluation_retire_cold(jobs.jobs);close_admitted(resources,owner_caller,copy);
  }
 }
 /// 🗂️ An actual indexed refusal keeps its original request, native cause and child receipt.
 #[test]
 fn original_sdk_indexed_refusal_keeps_source_cause_and_actual_receipt(){
  for copy in [1,3,64]{
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};let(resources,owner_caller)=admitted(Registry::new());let key=(resources.registry().unwrap().owner_identity(),String::from("original indexed operator 源"),17);let mut index=protocol::HistoryFoldIndex::<_,()>::new();let(error,receipt)=index.reserve_insert_step(&key,RetainedCloneGrant{maximum_capacity_bytes:0,..grant}).expect_err("actual original index capacity refusal");assert_eq!(receipt,Default::default());assert!(index.terminal_is_empty());let error=error.under("original indexed admission").with_retained_progress(receipt);let pointer=error.message.as_ptr();let kind=error.kind;
   let request=EvaluateRequest{neuron_id:"original indexed neuron".into(),operator_id:key.1,input_json:"original indexed input 源".into(),node_hash:17,dependency_json:String::new(),operator_version:"v1".into(),cancellation_id:"original indexed cancellation".into(),window_id:String::new(),window_kind_id:String::new(),extension_id:String::new(),round_units:1,resume:false,external_result:None};let source=request.input_json.as_ptr();let(reply,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||EvaluationReply::refused_original(request,receipt,EvaluationFailure::Native(error)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(reply.retirement_progress,receipt);assert_eq!(reply.returned_request.as_ref().unwrap().input_json.as_ptr(),source);assert!(matches!(&reply.refusal,Some(EvaluationFailure::Native(error))if error.message.as_ptr()==pointer&&error.kind==kind&&error.retained_progress()==receipt));
   let mut cursor=EvaluationInvokeCursor::new(71,3,b"original indexed borrowed raw caller");assert!(cursor.retain_reply(reply).is_none());for turn in 0..100000{let demand=cursor.retirement_demands(copy).unwrap();let turn_grant=RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1),..grant};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(turn_grant));let receipt=step.unwrap().progress();assert!(receipt.fits(turn_grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));if cursor.terminal_is_empty(){break}assert!(turn<99999,"original indexed failure source close stalled");}let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));close_admitted(resources,owner_caller,copy);
  }
 }
 /// 📥️ A returned refusal retains exact source and cause headers until granted full closure.
 #[test]
 fn original_sdk_raw_recipient_keeps_whole_request_and_cause(){
  for copy in [1,3,64]{
   let bytes=b"original borrowed caller";let mut cursor=EvaluationInvokeCursor::new(71,3,bytes);cursor.slot=Some(71);
   let request=EvaluateRequest{neuron_id:"outer original neuron".into(),operator_id:"original operator".into(),input_json:String::new(),node_hash:11,dependency_json:String::new(),operator_version:"same original version".into(),cancellation_id:"same original cancellation".into(),window_id:String::new(),window_kind_id:String::new(),extension_id:String::new(),round_units:1,resume:true,external_result:Some(EvaluationExternalResult{neuron_id:"nested neuron".into(),extension_id:"nested extension".into(),operator_id:"nested operator".into(),node_hash:17,output:DslValue::object([("value".into(),DslValue::String("same returned output 源".into()))])})};
   let pointer=request.external_result.as_ref().unwrap().output.as_object().unwrap().as_ptr();let cause=String::from("same owned duplicate member 源");let cause_pointer=cause.as_ptr();let mut reply=EvaluationReply::pending_request(request,RetainedCloneProgress{copied_items:1,..Default::default()});reply.refusal=Some(EvaluationFailure::Json(semio_framework_pack_json::JsonError::DuplicateMember{name:cause,offset:13}));
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.retain_reply(reply));assert!(result.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(cursor.slot.is_none());assert_eq!(cursor.original_request().unwrap().external_result.as_ref().unwrap().output.as_object().unwrap().as_ptr(),pointer);assert!(matches!(&cursor.fault,Some(EvaluationFailure::Json(semio_framework_pack_json::JsonError::DuplicateMember{name,offset}))if name.as_ptr()==cause_pointer&&*offset==13));
   for turn in 0..100000{let demand=cursor.retirement_demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(RetainedCloneGrant{maximum_items:0,..grant}));assert_eq!(step.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(grant));let receipt=step.unwrap().progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));if cursor.terminal_is_empty(){break}assert!(turn<99999,"raw recipient close stalled");}
   let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }
 }
 /// 🧯️ Actual original invocation donates the complete JSON failure only after source closure, then closes its full native owner.
 #[test]
 fn original_completed_preview_sdk_fault_donates_whole_original_domain_on_same_caller(){
  use semio_framework_job::{OriginalCallerSourceParent, OperationId, Generation, StepBudget, OriginalDomainView as V, close_original_retained_step};
  use semio_framework_job_payload::{JobPayloadAuthority};
  use semio_framework_value::retirement::controlled::ControlledRetirement;
  let law:serde_json::Value=serde_json::from_str(include_str!("../⚠️failure/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let stock=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:134217728,maximum_capacity_bytes:268435456,maximum_release_bytes:536870912,maximum_depth:4096};
  for body in[1,3,64]{let((mut parent,mut resources,request),initial)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let(cancel,_)=semio_framework_job::CancelToken::admit_root(stock).unwrap().unwrap();let(authority,_)=JobPayloadAuthority::admit(OperationId(71),Generation(3),stock).unwrap().unwrap();let(parent,_)=OriginalCallerSourceParent::admit(authority,cancel,stock).ok().unwrap();(parent,ExtensionEvaluationResources::new(Registry::new()),law["source"].as_str().unwrap().to_owned())});let(mut born,mut freed)=(initial.requested_bytes,initial.released_bytes);let mut sequence=0;let mut recipient=Default::default();let mut cx=parent.borrow_context(StepBudget::new(32768,1500,stock),||Some(1000),&mut sequence,&mut recipient).unwrap();let identity=cx.original_recipient_identity();let mut failure=None;
   for turn in 0..32768{let before=cx.retained_progress();let mut original_answer=None;let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,..stock},|same|{original_answer=Some(evaluate_invoke_json(&mut resources,request.as_bytes(),same)?);Ok(())}));answer.unwrap();let answer=original_answer.take().unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(answer.retained_progress.retained_capacity_bytes,answer.retained_progress.released_bytes));assert_eq!(cx.retained_progress(),before.checked_add(answer.retained_progress).unwrap());assert!(answer.payload.is_none());assert_eq!(cx.original_recipient_identity(),identity);born+=heap.requested_bytes;freed+=heap.released_bytes;if let Some(original)=answer.refusal{failure=Some(original);break}assert!(turn<32767);}
   let failure=failure.unwrap();assert!(resources.invoke.lock().unwrap().is_none());let domain=failure.primary_domain().unwrap();let projection=domain.projection();assert!(matches!(projection.projection_view(&[0]).unwrap(),V::IntrinsicText("json")));assert!(matches!(projection.projection_view(&[1,0]).unwrap(),V::IntrinsicText("duplicateMember")));assert!(matches!(projection.projection_view(&[1,1]).unwrap(),V::IntrinsicText(value)if value==law["expectedName"].as_str().unwrap()));assert!(matches!(projection.projection_view(&[1,2]).unwrap(),V::IntrinsicNumber(semio_framework_value::Number::UInt(offset))if offset>0));assert!(failure.receipt_refusal.is_none()&&failure.context_refusal.is_none());let(mut closing,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ControlledRetirement::new((request,failure)).unwrap_or_else(|_|panic!("whole original SDK request/fault retirement")));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));parent.cancellation().unwrap().cancel_now();
   for turn in 0..32768{if closing.terminal_is_empty(){break}let before=cx.retained_progress();let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,..stock},|same|close_original_retained_step(same,closing.selected_demand(body)?,|grant|closing.step(grant)).map(|step|step.progress())));let receipt=answer.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(cx.retained_progress(),before.checked_add(receipt).unwrap());born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn<32767)}assert!(closing.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(closing));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));semio_framework_plugin::ExtensionResourceOwner::begin_close(&mut resources);
   for turn in 0..32768{if semio_framework_plugin::ExtensionResourceOwner::terminal_is_empty(&resources){break}let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,..stock},|same|semio_framework_plugin::ExtensionResourceOwner::close_step(&mut resources,same).map(|step|step.progress().unwrap_or_default())));let receipt=answer.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn<32767)}assert!(semio_framework_plugin::ExtensionResourceOwner::terminal_is_empty(&resources));let live=resources.debug_live_fields();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(resources));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0),"{live}");drop(cx);
   for turn in 0..128{if parent.terminal_is_empty(){break}let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||parent.close_step(stock));let receipt=answer.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn<127)}assert!(parent.terminal_is_empty());assert_eq!(born,freed);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(parent));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }
 }

}

/// 🤝️ The resource consumes only its actual child receipt from the original caller context.
pub fn evaluate_invoke_json(owner:&mut ExtensionEvaluationResources,request:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_plugin::ExtensionInvokeStep,semio_framework::Fault>{
    let grant=cx.retained_grant();
    let pending=owner.invoke.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner);
    if pending.as_ref().is_some_and(|cursor|!cursor.matches(cx.operation().0,cx.generation().0,request)){return Ok(semio_framework_plugin::ExtensionInvokeStep{payload:None,retained_progress:Default::default(),refusal:Some(ValueError::literal(ValueRefusalKind::OwnershipLimit,"evaluation invocation retains another original caller").into())});}
    if grant.maximum_items==0||grant.maximum_depth==0||cx.should_yield()||cx.is_cancelled(){return Ok(semio_framework_plugin::ExtensionInvokeStep{payload:None,retained_progress:Default::default(),refusal:None});}
    if pending.is_none(){
        let receipt=RetainedCloneProgress{copied_items:1,..Default::default()};
        if let Err(original)=cx.consume_retained(receipt){return Ok(semio_framework_plugin::ExtensionInvokeStep{payload:None,retained_progress:Default::default(),refusal:Some(original.into())});}
        cx.consume_fuel(1);
        *pending=Some(EvaluationInvokeCursor::new(cx.operation().0,cx.generation().0,request));
        return Ok(semio_framework_plugin::ExtensionInvokeStep{payload:None,retained_progress:receipt,refusal:None});
    }
    if owner.registry.is_none(){
        let before=cx.retained_progress();
        let admitted=owner.admit_registry_on_original(cx);
        let after=cx.retained_progress();
        let receipt=RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes};
        return Ok(semio_framework_plugin::ExtensionInvokeStep{payload:None,retained_progress:receipt,refusal:admitted.err()});
    }
    let pending=owner.invoke.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner);
    let cursor=pending.as_mut().unwrap();let before_fuel=cx.fuel_remaining();let result=cursor.step(owner.registry.as_ref().expect("admitted original registry"),request,cx);
    let (reply,receipt,refusal)=match result{Ok((reply,receipt))=>{let refusal=if cursor.normal_context_posted{None}else{cx.consume_retained(receipt).err().map(Into::into)};(reply,receipt,refusal)},Err(closure)=>{let receipt=cursor.normal_step_progress();let receipt_refusal=if cursor.normal_context_posted{None}else{cx.consume_retained(receipt).err()};assert!(cursor.close_failure.is_none());cursor.close_failure=Some(original_failure::EvaluationCloseFailure{initial:cursor.fault.take(),closure,receipt_refusal});cursor.begin_close();(None,receipt,None)}};
    if cx.fuel_remaining()==before_fuel{cx.consume_fuel(receipt.copied_items as u64)}let mut reply=reply;let refusal=match reply.as_mut().and_then(|reply|reply.refusal.take()){Some(EvaluationFailure::Original(original))=>Some(original),Some(original)=>{cursor.fail(original);refusal},None=>refusal};let payload=reply.and_then(|reply|reply.wire).map(|wire|wire.to_vec_owner());
    if payload.is_some(){assert!(cursor.terminal_is_empty());*pending=None;}else if refusal.is_some()&&cursor.terminal_is_empty(){*pending=None;}
    Ok(semio_framework_plugin::ExtensionInvokeStep{payload,retained_progress:receipt,refusal})
}
