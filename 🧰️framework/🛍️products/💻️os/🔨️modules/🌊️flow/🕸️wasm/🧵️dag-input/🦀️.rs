//! 🧵️ Operation-owned DAG native codecs retain lifetime authority independently of poll fuel.
use super::*;
use crate::infinite::board::io::text::dag_input::{retained::{DagInputCursor,DagInputKind},output::{DagJsonSource,DagOutputKind,DagOutputPreparation}};
use semio_framework_pack_json::JsonBorrowedWriteCursor;
use crate::infinite::board::ports::directed_dag::input_application::{DagInputApplication,DagInputDisplaced};
use semio_framework_value::retirement::controlled::{ControlledRetirement,admit_typed_controlled_retirement};
use semio_framework_value::native_decoding::NativeDecodeContinuation;
use semio_framework_value::native_encoding::NativeEncodeContinuation;
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ErasedSnapshotRetirement,retained_clone::RetainedCloneGrant,retirement::{RetireOwned,admit_owned_retirement}};

pub(super) struct FlowDagAction{
    operation:u16,admission:Option<FlowFeatureAdmission>,input_ceiling:usize,output_ceiling:usize,program:FlowProgramState,observer:Rc<FlowOperationObserver>,
    input:Option<DagInputCursor>,output:Option<JsonBorrowedWriteCursor>,kind:Option<DagOutputKind>,
    application:Option<DagInputApplication>,displaced:Option<Box<ControlledRetirement<DagInputDisplaced>>>,
    output_preparation:Option<DagOutputPreparation>,output_prepared:bool,
    decode:Option<NativeDecodeContinuation>,encode:Option<NativeEncodeContinuation>,decode_started:bool,encode_started:bool,
    retirement:Option<Box<dyn ErasedSnapshotRetirement>>,transitions:u64,
    close_capacity:usize,close_copy:usize,close_release:usize,receipt:semio_framework_value::retained_clone::RetainedCloneProgress,
}
impl FlowDagAction{
    pub(super) fn new(operation:u16,args:&FlowArguments)->Self{
        let input=match operation{2514=>Some(DagInputKind::Progress),2515=>Some(DagInputKind::Statuses),2525=>Some(DagInputKind::Selection),2528=>Some(DagInputKind::Channels),_=>None};
        let kind=match operation{2518=>Some(DagOutputKind::Nodes),2519=>Some(DagOutputKind::Edges),2520=>Some(DagOutputKind::Selection),2522=>Some(DagOutputKind::Hover),2523=>Some(DagOutputKind::Channels),_=>None};
        Self{operation,admission:None,input_ceiling:0,output_ceiling:0,program:FlowProgramState::new(args),observer:Rc::new(FlowOperationObserver::new()),input:input.map(DagInputCursor::new),output:kind.map(|_|JsonBorrowedWriteCursor::new()),kind,application:None,displaced:None,output_preparation:kind.map(DagOutputPreparation::new),output_prepared:false,decode:None,encode:None,decode_started:false,encode_started:false,retirement:None,transitions:0,close_capacity:0,close_copy:0,close_release:0,receipt:Default::default()}
    }
    fn progress(&mut self)->FlowFeatureStep{self.transitions=self.transitions.saturating_add(1);FlowFeatureStep::Progress{completed:self.transitions,total:0}}
    fn domain_step(&mut self,domain:&mut FlowDomainAdapter,args:&FlowArguments,budget:AbiWorkBudget)->FlowFeatureStep{
        if !self.observer.bound.get(){if let Err(failure)=self.observer.begin_poll(budget){return FlowFeatureStep::Failed(failure)}}
        if self.retirement.is_some(){let before=self.receipt;return match self.drain_retirement(budget.retained){Ok(_)=>if self.receipt==before{FlowFeatureStep::Yield}else{self.progress()},Err(error)=>self.program.finish_domain(Err(error))}}
        if let Some(value)=self.input.as_mut().and_then(DagInputCursor::take_retired_update){
            let mut original=Some(value);let result=self.close_owned(&mut original,budget.retained);if let Some(value)=original{self.input.as_mut().unwrap().restore_retired_update(value)}return match result{Ok(_)=>self.progress(),Err(error)=>self.program.finish_domain(Err(error))}
        }
        if self.input.is_some(){
            if let Some(displaced)=self.displaced.as_mut(){
                if budget.retained.maximum_items==0||budget.retained.maximum_depth==0{return FlowFeatureStep::Yield}
                *displaced.original_mut().expect("pre-admitted untouched displaced custody")=self.application.as_mut().unwrap().commit(&mut domain.host.dag);
                self.receipt=self.receipt.checked_add(semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,..Default::default()}).expect("bounded atomic exchange receipt");self.retirement=self.displaced.take().map(|owner|owner as Box<dyn ErasedSnapshotRetirement>);return self.program.finish_domain(ok())
            }
            let source=match text(args,"json"){Ok(value)=>value,Err(error)=>return self.program.finish_domain(Err(error))};
            let observer=self.observer.clone();let mut observe=|_|observer.observe();
            let mut control=match self.decode.take(){Some(receipt)=>NativeDecodeControl::resume(receipt,&mut observe),None if !self.decode_started=>{self.decode_started=true;Ok(NativeDecodeControl::new_retained(&mut observe))},None=>return self.program.finish_domain(Err(abi_failure(AbiErrorCode::Busy)))};
            let grant=match control.as_ref(){Ok(control)=>RetainedCloneGrant{maximum_capacity_bytes:budget.retained.maximum_capacity_bytes.min(self.input_ceiling.saturating_sub(control.owned_bytes())),..budget.retained},Err(_)=>budget.retained};
            let result=match control.as_mut(){Ok(control)=>if let Some(application)=self.application.as_mut(){application.step(&domain.host.dag,1,control,grant)}else{self.input.as_mut().unwrap().step(source,1,control,grant)},Err(error)=>Err(semio_framework_value::ValueError::literal(error.kind,"DAG input continuation refused"))};
            let progress=self.application.as_ref().map(DagInputApplication::normal_step_progress).unwrap_or_else(||self.input.as_ref().unwrap().normal_step_progress());self.receipt=match self.receipt.checked_add(progress){Ok(receipt)=>receipt,Err(error)=>return self.program.finish_domain(Err(self.failure(error)))};
            if let Ok(control)=control{match control.pause(){Ok(receipt)=>self.decode=Some(receipt),Err(error)=>return self.program.finish_domain(Err(dag_input_failure(error)))}}
            match result{
                Ok(false)=>if progress.copied_items==0{FlowFeatureStep::Yield}else{self.progress()},
                Ok(true)=>{
                    if self.application.is_none(){self.application=Some(DagInputApplication::new(self.input.as_mut().unwrap().take_facts().expect("complete admitted original DAG facts")));return self.progress()}
                    let Some(items)=budget.retained.maximum_items.checked_sub(progress.copied_items)else{return self.program.finish_domain(Err(abi_failure(AbiErrorCode::NoCredit)))};let Some(capacity)=budget.retained.maximum_capacity_bytes.checked_sub(progress.retained_capacity_bytes)else{return self.program.finish_domain(Err(abi_failure(AbiErrorCode::NoCredit)))};
                    let bytes=semio_framework_value::retirement::owned_retirement_birth_bytes::<DagInputDisplaced>();let grant=match self.close_grant(RetainedCloneGrant{maximum_items:items,maximum_capacity_bytes:capacity,..budget.retained},0,bytes,0,1){Ok(grant)=>grant,Err(_)=>return if progress.copied_items==0{FlowFeatureStep::Yield}else{self.progress()}};
                    match admit_typed_controlled_retirement(DagInputDisplaced::empty(),grant){Ok((owner,receipt))=>{self.close_capacity-=receipt.retained_capacity_bytes;self.receipt=self.receipt.checked_add(receipt).expect("bounded displaced birth receipt");self.displaced=Some(owner);self.progress()},Err((error,_))=>self.program.finish_domain(Err(self.failure(error)))}
                },
                Err(error)=>self.program.finish_domain(Err(self.failure(error))),
            }
        }else{
            let observer=self.observer.clone();let mut observe=|_|observer.observe();
            let mut control=match self.encode.take(){Some(receipt)=>NativeEncodeControl::resume(receipt,&mut observe),None if !self.encode_started=>{self.encode_started=true;Ok(NativeEncodeControl::new_retained(&mut observe))},None=>return self.program.finish_domain(Err(abi_failure(AbiErrorCode::Busy)))};
            let grant=match control.as_ref(){Ok(control)=>RetainedCloneGrant{maximum_capacity_bytes:budget.retained.maximum_capacity_bytes.min(self.output_ceiling.saturating_sub(control.owned_bytes())),..budget.retained},Err(_)=>budget.retained};
            let prepared_before=self.output_prepared;
            let result=match control.as_mut(){Ok(control)=>if !self.output_prepared{self.output_preparation.as_mut().unwrap().step(&domain.host.dag,1,control,grant).map(|complete|{self.output_prepared=complete;None})}else{let source=self.output_preparation.as_ref().unwrap().view(&domain.host.dag);self.output.as_mut().unwrap().step(&DagJsonSource{source:&source,kind:self.kind.unwrap()},1,control,grant)},Err(error)=>Err(semio_framework_value::ValueError::literal(error.kind,"DAG output continuation refused"))};
            let progress=if prepared_before{self.output.as_ref().unwrap().normal_step_progress()}else{self.output_preparation.as_ref().unwrap().normal_step_progress()};self.receipt=match self.receipt.checked_add(progress){Ok(receipt)=>receipt,Err(error)=>return self.program.finish_domain(Err(self.failure(error)))};
            if let Ok(control)=control{match control.pause(){Ok(receipt)=>self.encode=Some(receipt),Err(error)=>return self.program.finish_domain(Err(dag_input_failure(error)))}}
            match result{Ok(None)=>if progress.copied_items==0{FlowFeatureStep::Yield}else{self.progress()},Ok(Some(output))=>{self.program.output=output.into_bytes();self.program.phase=FlowProgramPhase::Publish;self.progress()},Err(error)=>self.program.finish_domain(Err(self.failure(error)))}
        }
    }
    fn failure(&self,error:semio_framework_value::ValueError)->FlowFailure{
        if error.kind==semio_framework_value::ValueRefusalKind::Canceled{if let Some(code)=self.observer.failure.get(){return abi_failure(code)}}dag_input_failure(error)
    }
    fn drain_retirement(&mut self,admitted:RetainedCloneGrant)->Result<bool,FlowFailure>{
        if self.retirement.as_ref().is_some_and(|owner|owner.terminal_is_empty()){
            let bytes=std::mem::size_of_val(self.retirement.as_ref().unwrap().as_ref());if admitted.maximum_items==0||admitted.maximum_depth==0||bytes>admitted.maximum_release_bytes||bytes>self.close_release{return Ok(false)}self.close_release-=bytes;self.receipt=self.receipt.checked_add(semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}).map_err(dag_input_failure)?;self.retirement=None;return Ok(false)
        }
        let owner=self.retirement.as_deref_mut().map(|owner|owner as &mut dyn ErasedSnapshotRetirement);
        if let Some(owner)=owner{
            let copy=owner.next_copy_byte_demand().map_err(dag_input_failure)?.min(admitted.maximum_copy_bytes);let capacity=owner.next_capacity_byte_demand(copy).map_err(dag_input_failure)?;let release=owner.next_release_byte_demand().map_err(dag_input_failure)?;let depth=owner.next_depth_demand().map_err(dag_input_failure)?;
            if admitted.maximum_items==0||copy>admitted.maximum_copy_bytes||capacity>admitted.maximum_capacity_bytes||release>admitted.maximum_release_bytes||depth>admitted.maximum_depth||copy>self.close_copy||capacity>self.close_capacity||release>self.close_release{return Ok(false)}
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
            let step=owner.close_step(grant).map_err(dag_input_failure)?;let progress=match step{semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)|semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)=>progress};self.close_capacity-=progress.retained_capacity_bytes;self.close_copy-=progress.copied_bytes;self.close_release-=progress.released_bytes;self.receipt=self.receipt.checked_add(progress).map_err(dag_input_failure)?;
            return Ok(false)
        }
        Ok(true)
    }
    fn close_grant(&self,admitted:RetainedCloneGrant,copy:usize,capacity:usize,release:usize,depth:usize)->Result<RetainedCloneGrant,FlowFailure>{
        if admitted.maximum_items==0||copy>admitted.maximum_copy_bytes||capacity>admitted.maximum_capacity_bytes||release>admitted.maximum_release_bytes||depth>admitted.maximum_depth||copy>self.close_copy||capacity>self.close_capacity||release>self.close_release{return Err(abi_failure(AbiErrorCode::NoCredit))}
        Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth})
    }
    fn close_owned<T:RetireOwned>(&mut self,original:&mut Option<T>,admitted:RetainedCloneGrant)->Result<bool,FlowFailure>{
        if original.is_some(){let capacity=semio_framework_value::retirement::owned_retirement_birth_bytes::<T>();let grant=match self.close_grant(admitted,0,capacity,0,1){Ok(grant)=>grant,Err(_)=>return Ok(false)};
            match admit_owned_retirement(original.take().unwrap(),grant){Ok((owner,progress))=>{self.close_capacity-=progress.retained_capacity_bytes;self.receipt=self.receipt.checked_add(progress).map_err(dag_input_failure)?;self.retirement=Some(owner)},Err((error,value))=>{*original=Some(value);return Err(dag_input_failure(error))}}
            return Ok(false)
        }Ok(true)
    }
}
impl FlowActionState for FlowDagAction{
    fn retained_receipt(&self)->Option<semio_framework_value::retained_clone::RetainedCloneProgress>{Some(self.receipt)}
    #[cfg(test)]fn operation(&self)->u16{self.operation}
    fn bind_observer(&mut self,observer:Rc<FlowOperationObserver>){self.observer=observer}
    fn bind_admission(&mut self,admission:FlowFeatureAdmission)->Result<(),FlowFailure>{
        if self.admission.is_some()||admission.session.generation()==0||admission.request_generation==0{return Err(abi_failure(AbiErrorCode::StaleGeneration))}
        self.admission=Some(admission);self.input_ceiling=protocol::FLOW_MAX_REQUEST_BYTES;self.output_ceiling=protocol::FLOW_MAX_OUTPUT_BYTES;
        self.close_capacity=protocol::FLOW_MAX_OUTPUT_BYTES;self.close_copy=protocol::FLOW_MAX_OUTPUT_BYTES;self.close_release=protocol::FLOW_MAX_OUTPUT_BYTES;Ok(())
    }
    fn advance(&mut self,domain:&mut FlowDomainAdapter,args:&FlowArguments,budget:AbiWorkBudget)->FlowFeatureStep{
        if self.admission.is_none(){return FlowFeatureStep::Failed(abi_failure(AbiErrorCode::StaleGeneration))}
        if budget.byte_credit==0&&!budget.cancelled&&!budget.interrupted&&budget.deadline_ms.is_none_or(|deadline|budget.now_ms<deadline){return FlowFeatureStep::Yield}
        if let Err(failure)=self.observer.check_budget(budget){return FlowFeatureStep::Failed(failure)}
        match self.program.phase{
            FlowProgramPhase::Decode=>self.program.decode_step(args),FlowProgramPhase::Validate=>self.program.validate_step(args),FlowProgramPhase::Checkpoint=>self.program.checkpoint_step(self.operation),
            FlowProgramPhase::Domain if self.program.domain_cursor==0=>self.program.domain_ready_step(),FlowProgramPhase::Domain=>self.domain_step(domain,args,budget),
            FlowProgramPhase::Encode=>self.program.encode_step(),FlowProgramPhase::Publish=>self.program.publish_step(domain,self.operation),FlowProgramPhase::Complete=>self.program.complete_step(),FlowProgramPhase::Sealed=>FlowFeatureStep::Yield,
        }
    }
    fn close_step(&mut self,arguments:&mut FlowArguments,budget:AbiWorkBudget)->Result<bool,FlowFailure>{
        if budget.byte_credit==0{return Ok(false)}
        if let Some(owner)=self.displaced.take(){assert!(self.retirement.is_none());self.retirement=Some(owner as Box<dyn ErasedSnapshotRetirement>);return Ok(false)}
        if !self.drain_retirement(budget.retained)?{return Ok(false)}
        if self.application.is_some(){let mut application=self.application.take();let result=self.close_owned(&mut application,budget.retained);self.application=application;return result.map(|_|false)}
        if self.input.is_some(){let mut input=self.input.take();let result=self.close_owned(&mut input,budget.retained);self.input=input;return result.map(|_|false)}
        if self.output.is_some(){let mut output=self.output.take();let result=self.close_owned(&mut output,budget.retained);self.output=output;return result.map(|_|false)}
        if self.output_preparation.is_some(){let mut output=self.output_preparation.take();let result=self.close_owned(&mut output,budget.retained);self.output_preparation=output;return result.map(|_|false)}
        if arguments.payload.capacity()!=0{let mut payload=Some(std::mem::take(&mut arguments.payload));let result=self.close_owned(&mut payload,budget.retained);arguments.payload=payload.unwrap_or_default();return result.map(|_|false)}
        if self.program.output.capacity()!=0{let mut payload=Some(std::mem::take(&mut self.program.output));let result=self.close_owned(&mut payload,budget.retained);self.program.output=payload.unwrap_or_default();return result.map(|_|false)}
        Ok(true)
    }
}
