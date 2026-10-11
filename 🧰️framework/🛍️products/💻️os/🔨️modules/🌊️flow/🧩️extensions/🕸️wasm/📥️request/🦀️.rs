//! 📥️ The original invocation borrows caller octets and retains every native grammar owner.
use super::*;
use semio_framework_pack_json::{JsonGrammarCursor,JsonReadLimits,JsonSourceCursor,JsonMemberPolicy,Value,Number};
use semio_framework_value::native_decoding::NativeDecodeContinuation;
use semio_framework_value::NativeDecodeControl;

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

/// 🪪️ One bounded invocation keeps the exact caller source until its paid response exists.
pub struct EvaluationInvokeCursor {
    operation:u64,generation:u64,pointer:usize,length:usize,
    parser:Option<JsonGrammarCursor<Value>>,continuation:Option<NativeDecodeContinuation>,
    root:Option<Value>,request:Option<EvaluateRequest>,reply_pending:Option<neural_engine::PendingExtensionEval>,active:Option<Box<dyn ErasedSnapshotRetirement>>,
    field:usize,result_field:usize,result_required:u8,retained_field:usize,required:u8,policy:Option<RetainedCloneGrant>,
    slot:Option<usize>,lookup:Option<EvaluationSlotLookup>,budget:usize,wall:u64,round_units:u64,
    suppressed_wire:Option<String>,ready_wire:Option<String>,ready_complete:bool,ready_faulted:bool,ready_external:bool,wait_terminal:bool,fault:Option<EvaluationFailure>,response:Option<EvaluationOutput>,response_failed:bool,context:Option<EvaluationRequestContext>,context_prepared:bool,closing:bool,
}
impl EvaluationInvokeCursor {
    pub fn new(operation:u64,generation:u64,request:&[u8])->Self {
        Self {operation,generation,pointer:request.as_ptr()as usize,length:request.len(),parser:None,continuation:None,root:None,request:None,reply_pending:None,active:None,field:0,result_field:0,result_required:0,retained_field:0,required:0,policy:None,slot:None,lookup:None,budget:0,wall:0,round_units:0,suppressed_wire:None,ready_wire:None,ready_complete:false,ready_faulted:false,ready_external:false,wait_terminal:false,fault:None,response:None,response_failed:false,context:None,context_prepared:false,closing:false}
    }
    pub fn matches(&self,operation:u64,generation:u64,request:&[u8])->bool{self.operation==operation&&self.generation==generation&&self.pointer==request.as_ptr()as usize&&self.length==request.len()}
    pub fn begin_close(&mut self){self.closing=true;}
    pub fn terminal_is_empty(&self)->bool{self.ready_wire.is_none()&&self.suppressed_wire.is_none()&&self.context.is_none()&&self.parser.is_none()&&self.continuation.is_none()&&self.root.is_none()&&self.request.is_none()&&self.reply_pending.is_none()&&self.active.is_none()&&self.fault.is_none()&&self.response.is_none()&&self.slot.is_none()&&self.lookup.is_none()}
    pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(context)=&self.context{return context.retirement_demands(copy);}
        if self.active.is_some(){return evaluation_child_demands(&self.active,copy);}
        if self.parser.is_some(){return Ok(evaluation_owned_birth::<JsonGrammarCursor<Value>>());}
        if self.root.is_some(){return Ok(evaluation_owned_birth::<Value>());}
        if self.ready_wire.is_some()||self.suppressed_wire.is_some(){return Ok(evaluation_owned_birth::<String>());}
        if self.request.is_some(){return Ok(evaluation_owned_birth::<EvaluateRequest>());}
        if self.reply_pending.is_some(){return Ok(evaluation_owned_birth::<neural_engine::PendingExtensionEval>());}
        if self.fault.is_some(){return Ok(evaluation_owned_birth::<EvaluationFailure>());}
        if let Some(response)=&self.response{return response.retirement_demands(copy);}
        Ok(RetirementDemand{depth:usize::from(self.continuation.is_some()||self.slot.is_some()||self.lookup.is_some()),..Default::default()})
    }
    pub fn retirement_demands_with_registry(&self,registry:&neural_engine::SharedRegistry,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(slot)=self.slot{let jobs=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);if let Some((key,owner))=jobs.jobs.slot_entry(slot){if key.0!=registry.owner_identity(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original invocation close found another registry owner"));}if !owner.close_fields_terminal(false,false){return owner.retirement_demands(copy,false,false);}}return Ok(RetirementDemand{depth:1,..Default::default()});}
        self.retirement_demands(copy)
    }
    pub fn close_step_with_registry(&mut self,registry:&neural_engine::SharedRegistry,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(slot)=self.slot{let mut jobs=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);if let Some((key,owner))=jobs.jobs.slot_entry_mut(slot){if key.0!=registry.owner_identity(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original invocation close found another registry owner"));}if !owner.close_fields_terminal(false,false){owner.cancel();return owner.close_step(grant,false,false);}}self.slot=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        self.close_step(grant)
    }
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(context)=&mut self.context{let step=context.close_step(grant)?;if context.terminal_is_empty(){self.context=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if self.active.is_some(){return evaluation_child_close(&mut self.active,grant);}
        if self.parser.is_some(){return evaluation_admit_original(&mut self.parser,&mut self.active,grant);}
        if self.root.is_some(){return evaluation_admit_original(&mut self.root,&mut self.active,grant);}
        if self.ready_wire.is_some(){return evaluation_admit_original(&mut self.ready_wire,&mut self.active,grant);}
        if self.suppressed_wire.is_some(){return evaluation_admit_original(&mut self.suppressed_wire,&mut self.active,grant);}
        if self.request.is_some(){return evaluation_admit_original(&mut self.request,&mut self.active,grant);}
        if self.reply_pending.is_some(){return evaluation_admit_original(&mut self.reply_pending,&mut self.active,grant);}
        if self.fault.is_some(){return evaluation_admit_original(&mut self.fault,&mut self.active,grant);}
        if let Some(response)=&mut self.response{let step=response.close_step(grant)?;if response.terminal_is_empty(){self.response=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if self.continuation.is_some()||self.slot.is_some()||self.lookup.is_some(){
            if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original invocation metadata requires admitted depth"));}
            if self.continuation.is_some(){self.continuation=None;}else if self.lookup.is_some(){self.lookup=None;}else{self.slot=None;}
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    pub fn original_request(&self)->Option<&EvaluateRequest>{self.request.as_ref()}
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
        if self.request.is_none(){self.request=Some(EvaluateRequest{retained:grant,neuron_id:String::new(),operator_id:String::new(),input_json:String::new(),node_hash:0,budget:0,wall_micros:0,dependency_json:String::new(),operator_version:String::new(),cancellation_id:String::new(),round_units:0,resume:false,external_result:None});return Ok(RetainedCloneProgress{copied_items:1,..empty});}
        let Some((key,value))=root.iter_mut().nth(self.field)else{
            if self.required!=15{return Err("original request requires neuronId, operatorId, inputJson and retained".into());}
            self.policy=Some(self.request.as_ref().unwrap().retained);self.closing=true;return Ok(RetainedCloneProgress{copied_items:1,..empty});
        };
        let request=self.request.as_mut().unwrap();
        if key=="retained"{
            let Value::Object(policy)=value else{return Err("original retained policy requires an object".into());};
            let Some((axis,value))=policy.iter().nth(self.retained_field)else{if self.retained_field!=5{return Err("original retained policy requires all five axes".into());}self.required|=8;self.field+=1;self.retained_field=0;return Ok(RetainedCloneProgress{copied_items:1,..empty});};
            let Value::Number(Number::UInt(value))=value else{return Err("original retained axis requires an unsigned integer".into());};
            let value=usize::try_from(*value).map_err(|_|EvaluationFailure::Literal("original retained axis exceeds address space"))?;
            match &*axis{"maximumItems"=>request.retained.maximum_items=value,"maximumCopyBytes"=>request.retained.maximum_copy_bytes=value,"maximumCapacityBytes"=>request.retained.maximum_capacity_bytes=value,"maximumReleaseBytes"=>request.retained.maximum_release_bytes=value,"maximumDepth"=>request.retained.maximum_depth=value,_=>return Err("original retained policy contains an unknown axis".into())};
            self.retained_field+=1;return Ok(RetainedCloneProgress{copied_items:1,..empty});
        }
        if key=="externalResult"{
            let Value::Object(fields)=value else{return Err("external result requires an object".into());};
            if request.external_result.is_none(){request.external_result=Some(EvaluationExternalResult{neuron_id:String::new(),extension_id:String::new(),operator_id:String::new(),node_hash:0,output_json:String::new()});return Ok(RetainedCloneProgress{copied_items:1,..empty});}
            let Some((name,value))=fields.iter_mut().nth(self.result_field)else{if self.result_required!=31{return Err("external result requires all five original fields".into());}self.field+=1;self.result_field=0;return Ok(RetainedCloneProgress{copied_items:1,..empty});};
            let original=request.external_result.as_mut().unwrap();
            if name=="nodeHash"{let Value::Number(Number::UInt(hash))=value else{return Err("external result node hash requires an unsigned integer".into());};original.node_hash=*hash;self.result_required|=8;}
            else{let(bit,destination)=match &*name{"neuronId"=>(1,&mut original.neuron_id),"extensionId"=>(2,&mut original.extension_id),"operatorId"=>(4,&mut original.operator_id),"outputJson"=>(16,&mut original.output_json),_=>return Err("external result contains an unknown field".into())};let Value::String(text)=value else{return Err("external result string field has a different type".into());};if bit!=16&&text.is_empty(){return Err("external result identity must be nonempty".into());}let Value::String(text)=std::mem::replace(value,Value::Null)else{unreachable!()};*destination=text;self.result_required|=bit;}
            self.result_field+=1;return Ok(RetainedCloneProgress{copied_items:1,..empty});
        }
        let string=match &*key{"neuronId"=>{self.required|=1;Some(&mut request.neuron_id)},"operatorId"=>{self.required|=2;Some(&mut request.operator_id)},"inputJson"=>{self.required|=4;Some(&mut request.input_json)},"dependencyJson"=>Some(&mut request.dependency_json),"operatorVersion"=>Some(&mut request.operator_version),"cancellationId"=>Some(&mut request.cancellation_id),_=>None};
        if let Some(destination)=string{
            if !matches!(value,Value::String(_)){return Err("original request string field has a different type".into());}
            if (key=="neuronId"||key=="operatorId")&&matches!(value,Value::String(text)if text.is_empty()){return Err("original request identity must be nonempty".into());}
            let Value::String(original)=std::mem::replace(value,Value::Null)else{unreachable!()};*destination=original;
        }else if key=="resume"{let Value::Bool(original)=value else{return Err("original resume requires a boolean".into());};request.resume=*original;}
        else{
            let Value::Number(Number::UInt(original))=value else{return Err("original request numeric field requires an unsigned integer".into());};
            match &*key{"nodeHash"=>request.node_hash=*original,"budget"=>request.budget=*original,"wallMicros"=>request.wall_micros=*original,"roundUnits"=>request.round_units=*original,_=>return Err("original request contains an unknown field".into())}
        }
        self.field+=1;Ok(RetainedCloneProgress{copied_items:1,..empty})
    }
    pub fn step(&mut self,registry:&neural_engine::SharedRegistry,request:&[u8],grant:RetainedCloneGrant,cancelled:bool)->Result<(Option<EvaluationReply>,RetainedCloneProgress),ValueError>{self.step_original(registry,request,grant,cancelled,None)}
    pub fn step_with_context(&mut self,registry:&neural_engine::SharedRegistry,request:&[u8],grant:RetainedCloneGrant,cancelled:bool,context:EvaluationRequestContextInput<'_>)->Result<(Option<EvaluationReply>,RetainedCloneProgress),ValueError>{self.step_original(registry,request,grant,cancelled,Some(context))}
    fn step_original(&mut self,registry:&neural_engine::SharedRegistry,request:&[u8],grant:RetainedCloneGrant,cancelled:bool,context_input:Option<EvaluationRequestContextInput<'_>>)->Result<(Option<EvaluationReply>,RetainedCloneProgress),ValueError>{
        let empty=RetainedCloneProgress::default();
        if self.pointer!=request.as_ptr()as usize||self.length!=request.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original invocation source changed while pending"));}
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok((None,empty));}
        if let Some(input)=context_input.as_ref(){self.wait_terminal=input.wait_terminal;}
        if self.ready_wire.is_some()&&!cancelled&&!self.closing{
            if self.active.is_some()||self.request.is_some()||self.reply_pending.is_some(){let step=if self.active.is_some(){evaluation_child_close(&mut self.active,grant)}else if self.request.is_some(){evaluation_admit_original(&mut self.request,&mut self.active,grant)}else{evaluation_admit_original(&mut self.reply_pending,&mut self.active,grant)}?;return Ok((None,step.progress()));}
            self.slot=None;return Ok((Some(EvaluationReply{external_pending:self.ready_external,pending_source:None,wire:self.ready_wire.take(),complete:self.ready_complete,faulted:self.ready_faulted,retirement_progress:Default::default(),returned_request:None,refusal:None}),RetainedCloneProgress{copied_items:1,..empty}));
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
                return step.map(|step|(None,step.progress()));
            }
            if self.fault.is_some(){
                if self.ready_wire.is_some(){return evaluation_admit_original(&mut self.ready_wire,&mut self.active,grant).map(|step|(None,step.progress()));}
                if self.request.is_some(){return evaluation_admit_original(&mut self.request,&mut self.active,grant).map(|step|(None,step.progress()));}
                if self.reply_pending.is_some(){return evaluation_admit_original(&mut self.reply_pending,&mut self.active,grant).map(|step|(None,step.progress()));}
                self.response=Some(EvaluationOutput::fault(self.fault.take().unwrap(),0));return Ok((None,RetainedCloneProgress{copied_items:1,..empty}));
            }
            self.closing=false;
        }
        if self.policy.is_some()&&self.request.is_some(){
            if !self.context_prepared{if let Some(input)=context_input{
                if self.context.is_none(){self.context=Some(EvaluationRequestContext::new());return Ok((None,RetainedCloneProgress{copied_items:1,..empty}));}
                let context=self.context.as_mut().unwrap();let result=context.step(self.request.as_mut().unwrap(),input,grant,cancelled);let receipt=context.normal_step_progress();
                match result{Ok(true)=>{assert!(context.terminal_is_empty());self.context=None;self.context_prepared=true;},Ok(false)=>{},Err(fault)=>self.fail(fault)}
                return Ok((None,receipt));
            }}
            if self.lookup.is_none(){self.lookup=Some(EvaluationSlotLookup::default());return Ok((None,RetainedCloneProgress{copied_items:1,..empty}));}
            let policy=intersect(grant,self.policy.unwrap());let mut jobs=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);let lookup=self.lookup.as_mut().unwrap();
            let(existing,physical)=lookup.step(&jobs,registry.owner_identity(),self.request.as_ref().unwrap(),policy)?;let Some(existing)=existing else{return Ok((None,physical))};
            let remaining=evaluation_remaining_grant(policy,physical);if remaining.maximum_items==0{return Ok((None,physical))}
            let mut original=self.request.take().unwrap();original.retained=policy;self.budget=if original.budget==0{EVALUATE_STEP_BUDGET}else{usize::try_from(original.budget).unwrap_or(usize::MAX)};self.wall=if original.wall_micros==0{EVALUATE_STEP_WALL_MICROS}else{original.wall_micros};self.round_units=original.round_units;
            let reply=evaluation_admit_original_request(registry,original,&mut jobs,existing,physical,&mut self.slot);let receipt=reply.retirement_progress;self.lookup=None;drop(jobs);
            return Ok((self.retain_reply(reply),receipt));
        }
        if let Some(slot)=self.slot{
            let policy=intersect(grant,self.policy.unwrap());let mut jobs=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let reply=evaluation_drive_original(registry,&mut jobs,slot,policy,self.budget,self.wall,self.round_units,empty,None);let receipt=reply.retirement_progress;drop(jobs);return Ok((self.retain_reply(reply),receipt));
        }
        if self.root.is_some(){let result=self.decode_field(grant);return match result{Ok(receipt)=>Ok((None,receipt)),Err(fault)=>{self.fail(fault);Ok((None,RetainedCloneProgress{copied_items:1,..empty}))}};}
        if self.parser.is_none(){let source=JsonSourceCursor::<_,Value>::new(request,JsonMemberPolicy::Reject,JsonReadLimits{maximum_bytes:request.len()as u64,maximum_allocation_bytes:grant.maximum_capacity_bytes,maximum_depth:grant.maximum_depth,maximum_items:request.len()as u64})?;self.parser=Some(source.into_grammar().1);return Ok((None,RetainedCloneProgress{copied_items:1,..empty}));}
        let mut callback=|_|!cancelled;
        let mut control=match self.continuation.take(){Some(original)=>NativeDecodeControl::resume(original,&mut callback)?,None=>NativeDecodeControl::new_retained(&mut callback)};
        let mut source=JsonSourceCursor::from_grammar(request,self.parser.take().unwrap());
        let result=source.step(1,&mut control,grant);let receipt=source.normal_step_progress();self.parser=Some(source.into_grammar().1);self.continuation=Some(control.pause().map_err(|error|error.with_retained_progress(receipt))?);
        match result{Ok(Some(original))=>self.root=Some(original),Ok(None)=>{},Err(fault)=>self.fail(fault.into())};Ok((None,receipt))
    }
}
fn intersect(a:RetainedCloneGrant,b:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:a.maximum_items.min(b.maximum_items),maximum_copy_bytes:a.maximum_copy_bytes.min(b.maximum_copy_bytes),maximum_capacity_bytes:a.maximum_capacity_bytes.min(b.maximum_capacity_bytes),maximum_release_bytes:a.maximum_release_bytes.min(b.maximum_release_bytes),maximum_depth:a.maximum_depth.min(b.maximum_depth)}}

#[cfg(test)]
mod tests{
 use super::*;
 /// 🔎️ Exact original key bytes survive bounded lookup, structural restart and full source close.
 #[test]
 fn original_sdk_slot_lookup_keeps_same_request_under_fixed_copy(){
  for copy in [1,3,64]{
   let fixture=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};let setup=RetainedCloneGrant{maximum_copy_bytes:4096,..fixture};let resources=ExtensionEvaluationResources::new(Registry::new());let registry=resources.registry();let mut jobs=EvaluationJobRegistry::default();
   let make=|operator:&str|EvaluateRequest{retained:fixture,neuron_id:"original fixture neuron".into(),operator_id:operator.into(),input_json:"original source 源".into(),node_hash:17,budget:1,wall_micros:1,dependency_json:String::new(),operator_version:"v1".into(),cancellation_id:"original cancellation".into(),round_units:1,resume:false,external_result:None};
   for operator in ["other operator", "original operator 源", "later operator"]{let key=(registry.owner_identity(),operator.to_string(),17);while jobs.jobs.next_insert_capacity_byte_demand(&key,setup.maximum_copy_bytes).unwrap()!=0{jobs.jobs.reserve_insert_step(&key,setup).unwrap();}let mut source=make(operator);source.operator_id.clear();let owner=RetainedEvaluation::new(registry.clone(),source);assert!(jobs.jobs.insert_reserved(key,owner,setup).unwrap_or_else(|(error,_,_)|panic!("original lookup fixture admission: {error}")).0.is_none());jobs.lookup_epoch+=1;}
   let bytes=b"original borrowed raw caller";let mut owner=EvaluationInvokeCursor::new(71,3,bytes);owner.request=Some(make("original operator 源"));owner.lookup=Some(EvaluationSlotLookup::default());let pointer=owner.request.as_ref().unwrap().input_json.as_ptr();let expected=jobs.jobs.iter().find(|(key,_)|key.1=="original operator 源").unwrap().0.1.as_ptr();
   let mut restarted=false;let mut found=None;
   for turn in 0..100000{let original=owner.request.as_ref().unwrap();assert_eq!(original.input_json.as_ptr(),pointer);let lookup=owner.lookup.as_mut().unwrap();let before=(lookup.epoch,lookup.slot,lookup.position,lookup.selected,lookup.done);let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||lookup.step(&jobs,registry.owner_identity(),original,RetainedCloneGrant{maximum_items:0,..fixture}));assert_eq!(step.unwrap(),(None,Default::default()));assert_eq!((lookup.epoch,lookup.slot,lookup.position,lookup.selected,lookup.done),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||lookup.step(&jobs,registry.owner_identity(),original,fixture));let(status,receipt)=step.unwrap();assert!(receipt.fits(fixture));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if !restarted&&lookup.position!=0{let operator="new original registry entry";let key=(registry.owner_identity(),operator.to_string(),17);while jobs.jobs.next_insert_capacity_byte_demand(&key,setup.maximum_copy_bytes).unwrap()!=0{jobs.jobs.reserve_insert_step(&key,setup).unwrap();}let mut source=make(operator);source.operator_id.clear();let retained=RetainedEvaluation::new(registry.clone(),source);assert!(jobs.jobs.insert_reserved(key,retained,setup).unwrap_or_else(|(error,_,_)|panic!("original lookup mutation admission: {error}")).0.is_none());jobs.lookup_epoch+=1;restarted=true;}else if let Some(slot)=status{found=slot;break}assert!(turn<99999,"original key lookup stalled");}
   let slot=found.expect("original key lookup found registered source");assert_eq!(jobs.jobs.slot_entry(slot).unwrap().0.1.as_ptr(),expected);assert!(restarted);
   for turn in 0..100000{let demand=owner.retirement_demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1),..fixture};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant));let receipt=step.unwrap().progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));if owner.terminal_is_empty(){break}assert!(turn<99999,"original lookup source close stalled");}
   let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));for retained in jobs.jobs.slot_values_mut(){retained.cancel();while !retained.close_fields_terminal(false,false){retained.close_step(fixture,false,false).unwrap();}}evaluation_retire_cold(jobs.jobs);let mut resources=resources;semio_framework_plugin::ExtensionResourceOwner::begin_close(&mut resources);while !semio_framework_plugin::ExtensionResourceOwner::terminal_is_empty(&resources){let demand=semio_framework_plugin::ExtensionResourceOwner::retirement_demands(&resources,copy).unwrap();semio_framework_plugin::ExtensionResourceOwner::close_step(&mut resources,RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1),..fixture}).unwrap();}eprintln!("[DEBUG] Original slot lookup copy={copy} sameRequest=true boundedKey=true epochRestart=true System0=true sourceCloseConservation=true Drop0=true");
  }
 }
 /// 🗂️ An actual indexed refusal keeps its original request, native cause and child receipt.
 #[test]
 fn original_sdk_indexed_refusal_keeps_source_cause_and_actual_receipt(){
  for copy in [1,3,64]{
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};let mut resources=ExtensionEvaluationResources::new(Registry::new());let key=(resources.registry().owner_identity(),String::from("original indexed operator 源"),17);let mut index=protocol::HistoryFoldIndex::<_,()>::new();let(error,receipt)=index.reserve_insert_step(&key,RetainedCloneGrant{maximum_capacity_bytes:0,..grant}).expect_err("actual original index capacity refusal");assert_eq!(receipt,Default::default());assert!(index.terminal_is_empty());let error=error.under("original indexed admission").with_retained_progress(receipt);let pointer=error.message.as_ptr();let kind=error.kind;
   let request=EvaluateRequest{retained:grant,neuron_id:"original indexed neuron".into(),operator_id:key.1,input_json:"original indexed input 源".into(),node_hash:17,budget:1,wall_micros:1,dependency_json:String::new(),operator_version:"v1".into(),cancellation_id:"original indexed cancellation".into(),round_units:1,resume:false,external_result:None};let source=request.input_json.as_ptr();let(reply,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||EvaluationReply::refused_original(request,receipt,EvaluationFailure::Native(error)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(reply.retirement_progress,receipt);assert_eq!(reply.returned_request.as_ref().unwrap().input_json.as_ptr(),source);assert!(matches!(&reply.refusal,Some(EvaluationFailure::Native(error))if error.message.as_ptr()==pointer&&error.kind==kind&&error.retained_progress()==receipt));
   let mut cursor=EvaluationInvokeCursor::new(71,3,b"original indexed borrowed raw caller");assert!(cursor.retain_reply(reply).is_none());for turn in 0..100000{let demand=cursor.retirement_demands(copy).unwrap();let turn_grant=RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1),..grant};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(turn_grant));let receipt=step.unwrap().progress();assert!(receipt.fits(turn_grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));if cursor.terminal_is_empty(){break}assert!(turn<99999,"original indexed failure source close stalled");}let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));semio_framework_plugin::ExtensionResourceOwner::begin_close(&mut resources);while !semio_framework_plugin::ExtensionResourceOwner::terminal_is_empty(&resources){let demand=semio_framework_plugin::ExtensionResourceOwner::retirement_demands(&resources,copy).unwrap();semio_framework_plugin::ExtensionResourceOwner::close_step(&mut resources,RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1),..grant}).unwrap();}eprintln!("[DEBUG] Original indexed refusal copy={copy} sameRequest=true sameNativeCause=true actualChildReceipt=true SystemConservation=true Drop0=true");
  }
 }
 /// 📥️ A returned refusal retains exact source and cause headers until granted full closure.
 #[test]
 fn original_sdk_raw_recipient_keeps_whole_request_and_cause(){
  for copy in [1,3,64]{
   let bytes=b"original borrowed caller";let mut cursor=EvaluationInvokeCursor::new(71,3,bytes);cursor.slot=Some(71);
   let request=EvaluateRequest{retained:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128},neuron_id:"outer original neuron".into(),operator_id:"original operator".into(),input_json:String::new(),node_hash:11,budget:1,wall_micros:1,dependency_json:String::new(),operator_version:"same original version".into(),cancellation_id:"same original cancellation".into(),round_units:1,resume:true,external_result:Some(EvaluationExternalResult{neuron_id:"nested neuron".into(),extension_id:"nested extension".into(),operator_id:"nested operator".into(),node_hash:17,output_json:"same returned output 源".into()})};
   let pointer=request.external_result.as_ref().unwrap().output_json.as_ptr();let cause=String::from("same owned duplicate member 源");let cause_pointer=cause.as_ptr();let mut reply=EvaluationReply::pending_request(request,RetainedCloneProgress{copied_items:1,..Default::default()});reply.refusal=Some(EvaluationFailure::Json(semio_framework_pack_json::JsonError::DuplicateMember{name:cause,offset:13}));
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.retain_reply(reply));assert!(result.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(cursor.slot.is_none());assert_eq!(cursor.original_request().unwrap().external_result.as_ref().unwrap().output_json.as_ptr(),pointer);assert!(matches!(&cursor.fault,Some(EvaluationFailure::Json(semio_framework_pack_json::JsonError::DuplicateMember{name,offset}))if name.as_ptr()==cause_pointer&&*offset==13));
   for turn in 0..100000{let demand=cursor.retirement_demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(RetainedCloneGrant{maximum_items:0,..grant}));assert_eq!(step.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(grant));let receipt=step.unwrap().progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));if cursor.terminal_is_empty(){break}assert!(turn<99999,"raw recipient close stalled");}
   let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Original raw recipient copy={copy} returnedRequestSamePointer=true typedCauseSamePointer=true SystemConservation=true Drop0=true");
  }
 }
}

/// 🤝️ The resource consumes only its actual child receipt from the original caller context.
pub fn evaluate_invoke_json(owner:&ExtensionEvaluationResources,request:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_plugin::ExtensionInvokeStep,semio_framework::Fault>{
    let grant=cx.retained_grant();let mut pending=owner.invoke.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if pending.as_ref().is_some_and(|cursor|!cursor.matches(cx.operation().0,cx.generation().0,request)){return Ok(semio_framework_plugin::ExtensionInvokeStep{payload:None,retained_progress:Default::default(),refusal:Some(ValueError::literal(ValueRefusalKind::OwnershipLimit,"evaluation invocation retains another original caller"))});}
    if grant.maximum_items==0||grant.maximum_depth==0||cx.should_yield(){return Ok(semio_framework_plugin::ExtensionInvokeStep{payload:None,retained_progress:Default::default(),refusal:None});}
    if pending.is_none(){*pending=Some(EvaluationInvokeCursor::new(cx.operation().0,cx.generation().0,request));}
    let cursor=pending.as_mut().unwrap();let result=cursor.step(owner.registry(),request,grant,cx.is_cancelled());
    let (reply,receipt,mut refusal)=match result{Ok((reply,receipt))=>(reply,receipt,None),Err(error)=>{cursor.begin_close();let receipt=error.retained_progress();(None,receipt,Some(error))}};
    if let Err(error)=cx.consume_retained(receipt){if refusal.is_none(){refusal=Some(error);}}
    cx.consume_fuel(1);let payload=reply.and_then(|reply|reply.wire).map(String::into_bytes);
    if payload.is_some(){assert!(cursor.terminal_is_empty());*pending=None;}
    Ok(semio_framework_plugin::ExtensionInvokeStep{payload,retained_progress:receipt,refusal})
}
