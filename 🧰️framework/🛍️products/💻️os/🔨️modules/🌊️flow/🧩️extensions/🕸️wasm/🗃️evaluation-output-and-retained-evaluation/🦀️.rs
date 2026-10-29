//! 🗃️ Evaluation output and retained evaluation of the wasm extension glue: the evaluation output, the retained evaluation owner with its steps and its retirement cursor. See [`super`].

use super::*;

// #region ⏱️EvaluationJobs
pub struct EvaluationOutput {
 pub(super) output:Option<Dictionary>,pub(super) envelope:Option<pack::record::OwnedProjectedPackWireCursor<EvaluationEnvelopeSource>>,encoded_source:Option<EvaluationEnvelopeSource>,pub(super) pending_source:Option<neural_engine::PendingExtensionEval>,pub(super) wire:Option<EvaluationWire>,pub(super) active:std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,pub(super) stage:u8,pub(super) units:usize,pub(super) faulted:bool,pub(super) complete:bool,pub(super) external_pending:bool,pub(super) closing:bool,pub(super) normal_progress:RetainedCloneProgress,
}

impl EvaluationOutput {
 pub(super) fn new(output:Dictionary)->Self{let faulted=false;Self{output:Some(output),envelope:None,encoded_source:None,pending_source:None,wire:None,active:std::mem::ManuallyDrop::new(None),stage:0,units:0,faulted,complete:true,external_pending:false,closing:false,normal_progress:Default::default()}}
 pub fn envelope(source:EvaluationEnvelopeSource,faulted:bool)->Self{let complete=source.done;let external_pending=source.pending.is_some();Self{output:None,envelope:Some(pack::record::OwnedProjectedPackWireCursor::new(source,1)),encoded_source:None,pending_source:None,wire:None,active:std::mem::ManuallyDrop::new(None),stage:2,units:0,faulted,complete,external_pending,closing:false,normal_progress:Default::default()}}
 pub fn pending(original:neural_engine::PendingExtensionEval)->Self{Self::envelope(EvaluationEnvelopeSource{output:None,done:false,cancellable:true,phase:"external-invoke",units_done:0,units_total:0,pending:Some(original)},false)}
 pub(super) fn phase(&self)->&'static str{if self.closing||self.active.is_some(){"output-retire"}else{match self.stage{0=>"output-envelope-bind",2=>self.envelope.as_ref().unwrap().phase(),3=>"output-pending-transfer",4=>"output-source-close",_=>"complete"}}}
 pub fn normal_step_progress(&self)->RetainedCloneProgress{self.normal_progress}
 pub(super) fn step(&mut self,maximum_units:usize,base_units:usize,grant:RetainedCloneGrant)->Result<(Option<EvaluationReply>,RetainedCloneProgress),EvaluationFailure>{self.normal_progress=Default::default();let result=self.step_original(maximum_units,base_units,grant);self.normal_progress=match &result{Ok((_,progress))=>*progress,Err(original)=>original.retained_progress()};result}
 pub(super) fn step_original(&mut self,maximum_units:usize,base_units:usize,grant:RetainedCloneGrant)->Result<(Option<EvaluationReply>,RetainedCloneProgress),EvaluationFailure>{
  let empty=RetainedCloneProgress::default();if maximum_units==0||grant.maximum_items==0||grant.maximum_depth==0{return Ok((None,empty))}if self.closing{return Err(EvaluationFailure::Literal("original Pack output is closing"))}
  if self.active.is_some(){let step=evaluation_child_close(&mut self.active,grant)?;return Ok((None,step.progress()))}
  match self.stage{
   0=>{self.envelope=Some(pack::record::OwnedProjectedPackWireCursor::new(EvaluationEnvelopeSource{output:self.output.take(),done:true,cancellable:false,phase:"complete",units_done:base_units,units_total:base_units,pending:None},1));self.stage=2;Ok((None,RetainedCloneProgress{copied_items:1,..empty}))},
   2=>{match self.envelope.as_mut().unwrap().advance(grant){Ok(step)=>{if let Some(ready)=step.ready{assert!(self.envelope.as_ref().unwrap().terminal_is_empty());self.envelope=None;self.encoded_source=Some(ready.source);self.wire=Some(ready.wire);self.stage=3;}Ok((None,step.progress))},Err(original)=>{self.normal_progress=original.reason.retained_progress();Err(EvaluationFailure::Pack(original))}}},
   3=>{if self.external_pending{self.pending_source=self.encoded_source.as_mut().unwrap().pending.take();assert!(self.pending_source.is_some())}self.stage=4;Ok((None,RetainedCloneProgress{copied_items:1,..empty}))},
   4 if self.encoded_source.is_some()=>{let step=evaluation_admit_original(&mut self.encoded_source,&mut self.active,grant)?;Ok((None,step.progress()))},
   4=>{self.stage=5;Ok((None,RetainedCloneProgress{copied_items:1,..empty}))},
   5=>{self.stage=6;Ok((Some(EvaluationReply{retirement_posted:false,external_pending:self.pending_source.is_some(),pending_source:self.pending_source.take(),wire:self.wire.take(),returned_request:None,refusal:None,complete:self.complete,faulted:self.faulted,retirement_progress:empty}),RetainedCloneProgress{copied_items:1,..empty}))},
   _=>Ok((None,empty)),
  }
 }
 pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Ok(RetirementDemand{depth:1,..Default::default()})}if self.active.is_some(){return evaluation_child_demands(&self.active,copy)}if let Some(cursor)=self.envelope.as_ref(){return cursor.next_demand(copy)}
  if self.pending_source.is_some(){return Ok(evaluation_owned_birth::<neural_engine::PendingExtensionEval>())}if self.encoded_source.is_some(){return Ok(evaluation_owned_birth::<EvaluationEnvelopeSource>())}if self.output.is_some(){return Ok(evaluation_owned_birth::<Dictionary>())}if self.wire.is_some(){return Ok(evaluation_owned_birth::<EvaluationWire>())}Ok(Default::default())
 }
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,EvaluationFailure>{
  let empty=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(empty))}
  if !self.closing{self.closing=true;if let Some(cursor)=self.envelope.as_mut(){cursor.begin_close()}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
  let step=if self.active.is_some(){evaluation_child_close(&mut self.active,grant)?}else if let Some(cursor)=self.envelope.as_mut(){let step=cursor.close_step(grant)?;if cursor.terminal_is_empty(){self.envelope=None}step}else if self.pending_source.is_some(){evaluation_admit_original(&mut self.pending_source,&mut self.active,grant)?}else if self.encoded_source.is_some(){evaluation_admit_original(&mut self.encoded_source,&mut self.active,grant)?}else if self.output.is_some(){evaluation_admit_original(&mut self.output,&mut self.active,grant)?}else{evaluation_admit_original(&mut self.wire,&mut self.active,grant)?};
  Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.output.is_none()&&self.envelope.is_none()&&self.encoded_source.is_none()&&self.pending_source.is_none()&&self.wire.is_none()&&self.active.is_none()}
}

impl Drop for EvaluationOutput{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Pack evaluation output requires full paid close")}}

pub struct RetainedEvaluation {
    pub registry:Option<neural_engine::SharedRegistry>,pub(super) lease:Option<neural_engine::RegistryLeaseRetirement>,pub(super) owner_identity:neural_engine::RegistryIdentity,
    pub request_admission:Option<EvaluationRequestAdmission>,pub(super) discarded_request:Option<EvaluationRequestAdmission>,pub(super) request_initial:bool,pub(super) discarded_operator:Option<String>,pub(super) neuron_id:String,pub(super) pending_wave:Option<neural_engine::evaluation::wave::PendingWaveCursor>,pending_wave_closing:bool,pub(super) external_pending:Option<neural_engine::PendingExtensionEval>,pub(super) external_completion:Option<EvaluationExternalCompletion>,pub(super) discarded_neuron:Option<String>,
    job:Option<Box<dyn neural_engine::OperatorJob>>,pub(super) admission:Option<EvaluationInputPreparation>,pub(super) replacement:Option<EvaluationInputPreparation>,pub(super) candidate:Option<(Dictionary,String)>,
    dependency_identity:String,pub(super) version:String,pub(super) cancellation_id:String,next_version:Option<String>,next_cancellation:Option<String>,
    pub(super) retirement:neural_engine::ValueRetirement,pub(super) finished:Option<Dictionary>,pub(super) output:Option<EvaluationOutput>,pub(super) fault:Option<EvaluationFailure>,
    pub units:usize,identity_position:Option<usize>,pub(super) cancelled:bool,pub(super) restarting:bool,pub(super) normal_progress:RetainedCloneProgress,pub(super) normal_context_posted:bool,
    pub plan:neural_engine::OperatorPlanCursor,planning:bool,finish:neural_engine::OperatorFinishCursor,
    pub(super) active:std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub(super) response:Option<EvaluationOutput>,pub(super) response_failed:bool,pub(super) continuation_identity:Option<EvaluationContinuationIdentity>,pub(super) compact_request:Option<EvaluateRequest>,pub(super) compact_cleanup:bool,pub(super) compact_active:std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,completed_reply:Option<EvaluationReply>,pending_input:Option<Dictionary>,pub(super) discarded_candidate:Option<(Dictionary,String)>,pub(super) discarded_preparation:Option<EvaluationInputPreparation>,pub(super) discarded_version:Option<String>,pub(super) discarded_cancellation:Option<String>,
}

impl RetainedEvaluation {
    pub(super) fn new(registry:neural_engine::SharedRegistry,request:EvaluateRequest)->Self {
        Self {finish:neural_engine::OperatorFinishCursor::new(),plan:neural_engine::OperatorPlanCursor::new(),planning:false,owner_identity:registry.owner_identity(),registry:Some(registry),lease:None,active:std::mem::ManuallyDrop::new(None),response:None,response_failed:false,continuation_identity:None,compact_request:None,compact_cleanup:false,compact_active:std::mem::ManuallyDrop::new(None),completed_reply:None,pending_input:None,discarded_candidate:None,discarded_preparation:None,discarded_version:None,discarded_cancellation:None,request_admission:Some(EvaluationRequestAdmission::new(request)),discarded_request:None,request_initial:true,discarded_operator:None,neuron_id:String::new(),pending_wave:None,pending_wave_closing:false,external_pending:None,external_completion:None,discarded_neuron:None,job:None,admission:None,replacement:None,candidate:None,dependency_identity:String::new(),version:String::new(),cancellation_id:String::new(),next_version:None,next_cancellation:None,retirement:Default::default(),finished:None,output:None,fault:None,units:0,identity_position:None,cancelled:false,restarting:false,normal_progress:Default::default(),normal_context_posted:false}
    }
    pub(super) fn current_version(&self)->&str{self.request_admission.as_ref().and_then(|owner|owner.request.as_ref()).map_or_else(||self.next_version.as_deref().unwrap_or(&self.version),|request|request.operator_version.as_str())}
    pub(super) fn current_neuron(&self)->&str{self.request_admission.as_ref().and_then(|owner|owner.request.as_ref()).map_or(&self.neuron_id,|request|&request.neuron_id)}
    pub(super) fn current_cancellation(&self)->&str{self.request_admission.as_ref().and_then(|owner|owner.request.as_ref()).map_or_else(||self.next_cancellation.as_deref().unwrap_or(&self.cancellation_id),|request|request.cancellation_id.as_str())}
    pub fn progress(&self)->neural_engine::OperatorProgress {
        let (done,total,phase)=if self.cancelled {(self.units,self.units.saturating_add(1),if self.restarting {"retire-invalidated"}else{"input-retire"})}
        else if self.request_admission.is_some(){(self.units,self.units.saturating_add(1),"request-admit")}
        else if self.completed_reply.is_some() {(self.units,self.units.saturating_add(1),"output-retire")}
        else if !self.retirement.terminal_is_empty() {(self.units,self.units.saturating_add(1),"input-retire")}
        else if let Some(admission)=&self.admission {let (done,total,phase)=admission.progress();(self.units.saturating_add(done),self.units.saturating_add(total),phase)}
        else if self.pending_wave.is_some(){(self.units,self.units.saturating_add(1),if self.pending_wave_closing{"external-source-retire"}else{"external-wave"})}
        else if let Some(job)=&self.job {let progress=job.progress();(self.units.saturating_add(progress.units_done),self.units.saturating_add(progress.units_total),progress.phase)}
        else if let Some(output)=&self.output {(self.units.saturating_add(output.units),self.units.saturating_add(output.units).saturating_add(1),output.phase())}
        else {(self.units,self.units.saturating_add(1),if self.identity_position.is_some(){"identity-compare"}else{"input-admit"})};
        neural_engine::OperatorProgress {units_done:done,units_total:total,phase}
    }
    pub fn cancel(&mut self) {
        if !self.cancelled {if let Some(output)=&mut self.output {self.units=self.units.saturating_add(output.units);output.units=0;}}
        self.cancelled=true;self.planning=false;self.restarting=false;if let Some(admission)=&mut self.admission {admission.cancel();}if let Some(job)=&mut self.job {job.cancel();}
    }
    pub fn retirement_demands(&self,copy:usize,preserve_candidate:bool,preserve_fault:bool)->Result<RetirementDemand,ValueError>{
        if self.continuation_identity.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if !self.retirement.terminal_is_empty(){return evaluation_value_demands(&self.retirement,copy);}
        if self.active.is_some(){return evaluation_child_demands(&self.active,copy);}
        if self.compact_active.is_some(){return evaluation_child_demands(&self.compact_active,copy);}
        if self.compact_request.is_some(){return Ok(evaluation_owned_birth::<EvaluateRequest>());}
        if let Some(response)=&self.response{return response.retirement_demands(copy);}
        if let Some(completion)=&self.external_completion{return completion.retirement_demands(copy);}
        if self.external_pending.is_some(){return Ok(evaluation_owned_birth::<neural_engine::PendingExtensionEval>());}
        if let Some(wave)=&self.pending_wave{return wave.next_close_demands(copy);}
        if self.discarded_neuron.is_some(){return Ok(RetirementDemand {depth:1,..Default::default()});}
        if self.discarded_request.is_some()||(!preserve_candidate&&self.request_admission.is_some()){return Ok(evaluation_owned_birth::<EvaluationRequestAdmission>());}
        if let Some(previous)=&self.discarded_preparation{return previous.retirement_demands(copy);}
        if self.discarded_candidate.is_some(){return Ok(evaluation_owned_birth::<(Dictionary,String)>());}
        if self.pending_input.is_some()||self.discarded_operator.is_some()||self.discarded_version.is_some()||self.discarded_cancellation.is_some(){return Ok(RetirementDemand {depth:1,..Default::default()});}
        if let Some(admission)=&self.admission{return admission.retirement_demands(copy);}
        if !preserve_candidate {if let Some(replacement)=&self.replacement{return replacement.retirement_demands(copy);}}
        if let Some(job)=&self.job{
            if job.terminal_is_empty(){return Ok(RetirementDemand {release_bytes:size_of_val(job.as_ref()),depth:1,..Default::default()});}
            return Ok(RetirementDemand {copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(copy)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?});
        }
        if let Some(output)=&self.output{return output.retirement_demands(copy);}
        if !preserve_candidate&&self.candidate.is_some(){return Ok(evaluation_owned_birth::<(Dictionary,String)>());}
        if !preserve_fault&&self.fault.is_some(){return Ok(evaluation_owned_birth::<EvaluationFailure>());}
        if let Some(lease)=&self.lease{return Ok(RetirementDemand {copy_bytes:lease.next_copy_byte_demand()?,capacity_bytes:lease.next_capacity_byte_demand(copy)?,release_bytes:lease.next_release_byte_demand()?,depth:lease.next_depth_demand()?});}
        Ok(RetirementDemand {depth:usize::from(!self.close_fields_terminal(preserve_candidate,preserve_fault)),..Default::default()})
    }
    pub fn close_fields_terminal(&self,preserve_candidate:bool,preserve_fault:bool)->bool{
        self.continuation_identity.is_none()&&self.external_pending.is_none()&&self.external_completion.is_none()&&self.response.is_none()&&self.compact_request.is_none()&&self.compact_active.is_none()&&self.pending_wave.is_none()&&self.discarded_neuron.is_none()&&self.neuron_id.capacity()==0&&(preserve_candidate||self.request_admission.is_none())&&self.discarded_request.is_none()&&self.discarded_operator.is_none()&&(preserve_fault||self.fault.is_none())&&(preserve_fault||self.completed_reply.is_none())&&self.pending_input.is_none()&&self.discarded_candidate.is_none()&&self.discarded_preparation.is_none()&&self.discarded_version.is_none()&&self.discarded_cancellation.is_none()&&self.active.is_none()&&self.retirement.terminal_is_empty()&&self.admission.is_none()&&self.job.is_none()&&self.finished.is_none()&&self.output.is_none()&&self.dependency_identity.capacity()==0&&self.version.capacity()==0&&self.cancellation_id.capacity()==0&&(preserve_candidate||(self.replacement.is_none()&&self.candidate.is_none()&&self.next_version.is_none()&&self.next_cancellation.is_none()&&self.registry.is_none()&&self.lease.is_none()))
    }
    pub(super) fn close_step(&mut self,grant:RetainedCloneGrant,preserve_candidate:bool,preserve_fault:bool)->Result<RetainedCloneStep,EvaluationFailure>{
        let empty=RetainedCloneProgress::default();
        if self.close_fields_terminal(preserve_candidate,preserve_fault){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes,preserve_candidate,preserve_fault)?;
        if self.continuation_identity.is_some(){if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original comparison metadata close requires depth").into())}self.continuation_identity=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation close exceeds supplied depth").into());}
        let step=if !self.retirement.terminal_is_empty(){self.retirement.close_step(grant)?}
        else if self.active.is_some(){evaluation_child_close(&mut self.active,grant)?}
        else if self.compact_active.is_some(){evaluation_child_close(&mut self.compact_active,grant)?}
        else if self.compact_request.is_some(){evaluation_admit_original(&mut self.compact_request,&mut self.compact_active,grant)?}
        else if let Some(response)=&mut self.response{let step=response.close_step(grant)?;if response.terminal_is_empty(){self.response=None;}step}
        else if let Some(completion)=&mut self.external_completion{let step=completion.close_step(grant)?;if completion.terminal_is_empty(){self.external_completion=None;}step}
        else if self.external_pending.is_some(){evaluation_admit_original(&mut self.external_pending,&mut self.active,grant)?}
        else if let Some(wave)=&mut self.pending_wave{wave.begin_close();let step=wave.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,wave.terminal_is_empty(),"original pending evaluation wave")?;if wave.terminal_is_empty(){self.pending_wave=None;self.pending_wave_closing=false;}step}
        else if self.discarded_neuron.is_some(){let original=self.discarded_neuron.take().unwrap();match self.retirement.text(original,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,original))=>{self.discarded_neuron=Some(original);return Err(error.into());}}}
        else if self.neuron_id.capacity()!=0{let original=std::mem::take(&mut self.neuron_id);match self.retirement.text(original,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,original))=>{self.neuron_id=original;return Err(error.into());}}}
        else if self.discarded_request.is_some(){evaluation_admit_original(&mut self.discarded_request,&mut self.active,grant)?}
        else if !preserve_candidate&&self.request_admission.is_some(){evaluation_admit_original(&mut self.request_admission,&mut self.active,grant)?}
        else if self.intermediate_pending(){self.close_intermediate(grant)?}
        else if let Some(admission)=&mut self.admission{
            let step=admission.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,admission.terminal_is_empty(),"evaluation admission")?;
            if admission.terminal_is_empty(){self.admission=None;}step
        }else if !preserve_candidate&&self.replacement.is_some(){
            let replacement=self.replacement.as_mut().unwrap();let step=replacement.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,replacement.terminal_is_empty(),"evaluation replacement")?;
            if replacement.terminal_is_empty(){self.replacement=None;}step
        }else if let Some(job)=&mut self.job{
            if job.terminal_is_empty(){
                let bytes=size_of_val(job.as_ref());
                if grant.maximum_release_bytes<bytes{return Ok(RetainedCloneStep::Progress(empty));}
                drop(self.job.take());RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty})
            }else{let step=job.close_step(grant)?;semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,job.terminal_is_empty(),"evaluation operator job")?}
        }else if let Some(value)=self.finished.take(){
            match self.retirement.push_dictionary(value,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,value))=>{self.finished=Some(value);self.finish=neural_engine::OperatorFinishCursor::new();return Err(error.into());}}
        }else if let Some(output)=&mut self.output{
            let step=output.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,output.terminal_is_empty(),"evaluation output")?;
            if output.terminal_is_empty(){self.output=None;}step
        }else if self.dependency_identity.capacity()!=0||self.version.capacity()!=0||self.cancellation_id.capacity()!=0{
            let original=if self.dependency_identity.capacity()!=0{&mut self.dependency_identity}else if self.version.capacity()!=0{&mut self.version}else{&mut self.cancellation_id};
            let value=std::mem::take(original);match self.retirement.text(value,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,value))=>{*original=value;return Err(error.into());}}
        }else if !preserve_candidate&&self.candidate.is_some(){evaluation_admit_original(&mut self.candidate,&mut self.active,grant)?}
        else if !preserve_candidate&&(self.next_version.is_some()||self.next_cancellation.is_some()){
            let original=if self.next_version.is_some(){&mut self.next_version}else{&mut self.next_cancellation};let value=original.take().unwrap();match self.retirement.text(value,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,value))=>{*original=Some(value);return Err(error.into());}}
        }else if !preserve_fault&&self.completed_reply.is_some(){
            evaluation_admit_original(&mut self.completed_reply,&mut self.active,grant)?
        }else if !preserve_fault&&self.fault.is_some(){
            evaluation_admit_original(&mut self.fault,&mut self.active,grant)?
        }else if let Some(lease)=&mut self.lease{
            let step=lease.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,lease.terminal_is_empty(),"evaluation registry source lease")?;if lease.terminal_is_empty(){self.lease=None;}step
        }else if !preserve_candidate&&self.registry.is_some(){self.lease=Some(self.registry.take().unwrap().into_retirement());RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty})}
        else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation close lost its retained frontier").into());};
        let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,RetainedCloneStep::Progress(step.progress()),false,"original evaluation parent")?;
        Ok(if self.close_fields_terminal(preserve_candidate,preserve_fault){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    fn intermediate_pending(&self)->bool{!self.retirement.terminal_is_empty()||self.active.is_some()||(self.pending_input.is_some()&&!self.planning)||self.discarded_request.is_some()||self.discarded_candidate.is_some()||self.discarded_preparation.is_some()||self.discarded_operator.is_some()||self.discarded_version.is_some()||self.discarded_cancellation.is_some()}
    pub(super) fn close_intermediate(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if !self.retirement.terminal_is_empty(){return self.retirement.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.active.is_some(){return evaluation_child_close(&mut self.active,grant);}
        if self.discarded_request.is_some(){return evaluation_admit_original(&mut self.discarded_request,&mut self.active,grant);}
        if let Some(previous)=&mut self.discarded_preparation{
            let step=previous.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,previous.terminal_is_empty(),"discarded original input")?;if previous.terminal_is_empty(){self.discarded_preparation=None;}return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.discarded_candidate.is_some(){return evaluation_admit_original(&mut self.discarded_candidate,&mut self.active,grant);}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation intermediate handoff exceeds admitted depth"));}
        if let Some(value)=self.pending_input.take(){return match self.retirement.push_dictionary(value,grant){Ok(progress)=>Ok(RetainedCloneStep::Progress(progress)),Err((error,value))=>{self.pending_input=Some(value);Err(error)}};}
        let original=if self.discarded_operator.is_some(){&mut self.discarded_operator}else if self.discarded_version.is_some(){&mut self.discarded_version}else{&mut self.discarded_cancellation};
        if let Some(value)=original.take(){return match self.retirement.text(value,grant){Ok(progress)=>Ok(RetainedCloneStep::Progress(progress)),Err((error,value))=>{*original=Some(value);Err(error)}};}
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn close_finished_job(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        let job=self.job.as_mut().unwrap();let empty=RetainedCloneProgress::default();
        if grant.maximum_items==0{return Ok(empty);}
        if job.terminal_is_empty(){
            if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"terminal evaluation job shell requires admitted depth"));}
            let bytes=size_of_val(job.as_ref());if grant.maximum_release_bytes<bytes{return Ok(empty);}drop(self.job.take());return Ok(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty});
        }
        let step=job.close_step(grant)?;semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,job.terminal_is_empty(),"finished original operator job").map(|step|step.progress())
    }
    pub fn normal_step_progress(&self)->RetainedCloneProgress {self.normal_progress}
    pub(super) fn advance(&mut self,registry:&Registry,operator:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<(EvaluationStep,RetainedCloneProgress),EvaluationFailure>{
        self.normal_progress=Default::default();self.normal_context_posted=false;let result=self.advance_original(registry,operator,cx);if let Ok((_,progress))=&result {self.normal_progress=*progress;}result
    }
    fn advance_original(&mut self,registry:&Registry,operator:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<(EvaluationStep,RetainedCloneProgress),EvaluationFailure>{
        let grant=cx.retained_grant();let maximum_units=usize::from(!cx.should_yield());let maximum_bytes=grant.maximum_copy_bytes;
        use neural_engine::OperatorJobStep;
        let empty=RetainedCloneProgress::default();
        if maximum_units==0||grant.maximum_items==0{return Ok((EvaluationStep::Working(self.progress()),empty));}
        if !self.cancelled&&self.completed_reply.as_ref().is_some_and(|reply|reply.external_pending){let mut reply=self.completed_reply.take().unwrap();self.external_pending=reply.pending_source.take();return Ok((EvaluationStep::Done(reply),RetainedCloneProgress{copied_items:1,..empty}));}
        if !self.cancelled&&self.completed_reply.is_some(){
            let step=self.close_step(grant,false,true).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;let receipt=step.progress();
            if matches!(step,RetainedCloneStep::Complete(_)){return Ok((EvaluationStep::Done(self.completed_reply.take().unwrap()),receipt));}
            return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if self.cancelled{
            let step=self.close_step(grant,self.restarting,true).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;let receipt=step.progress();
            if matches!(step,RetainedCloneStep::Complete(_)){
                if let Some(fault)=self.fault.take(){return Ok((EvaluationStep::Fault(fault),receipt));}
                if self.restarting{if self.replacement.is_some(){self.admission=self.replacement.take();}if self.request_admission.is_none(){self.version=self.next_version.take().unwrap();self.cancellation_id=self.next_cancellation.take().unwrap();}else{self.request_initial=true;}self.cancelled=false;self.restarting=false;self.units=0;self.identity_position=None;return Ok((EvaluationStep::Working(neural_engine::OperatorProgress {units_done:0,units_total:0,phase:"restart-admission"}),receipt));}
                return Ok((EvaluationStep::Cancelled(neural_engine::OperatorProgress {units_done:self.units,units_total:self.units,phase:"cancelled"}),receipt));
            }
            return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if let Some(completion)=&mut self.external_completion{let result=completion.step(grant);let receipt=completion.normal_step_progress();self.normal_progress=receipt;match result{Ok(Some(dictionary))=>{assert!(completion.terminal_is_empty());self.external_completion=None;self.finished=Some(dictionary);self.finish=neural_engine::OperatorFinishCursor::new();},Ok(None)=>{},Err(error)=>{self.fault=Some(error);self.cancel();}}return Ok((EvaluationStep::Working(self.progress()),receipt));}
        if self.external_pending.is_some()&&self.finished.is_none(){return Ok((EvaluationStep::Working(neural_engine::OperatorProgress{units_done:self.units,units_total:self.units.saturating_add(1),phase:"external-await"}),empty));}
        if let Some(owner)=&mut self.request_admission{
            let receipt=owner.step_with_operator(operator,grant).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;
            if let Some((request,operator,version))=owner.take_ready(){if self.neuron_id.capacity()!=0{self.discarded_neuron=Some(std::mem::take(&mut self.neuron_id));}self.neuron_id=request.neuron_id;if self.request_initial{self.version=request.operator_version;self.cancellation_id=request.cancellation_id;}else{self.discarded_version=self.next_version.replace(request.operator_version);self.discarded_cancellation=self.next_cancellation.replace(request.cancellation_id);}if request.operator_id.capacity()!=0{self.discarded_operator=Some(request.operator_id);}self.admission=Some(EvaluationInputPreparation::new(request.input_json,request.dependency_json,operator,version));self.request_admission=None;}
            self.units=self.units.saturating_add(receipt.copied_items);return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if self.intermediate_pending(){let step=self.close_intermediate(grant).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;return Ok((EvaluationStep::Working(self.progress()),step.progress()));}
        if self.finished.is_some()&&self.job.is_some(){let receipt=self.close_finished_job(grant).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;return Ok((EvaluationStep::Working(self.progress()),receipt));}
        if let Some(admission)=&mut self.admission{
            let result=admission.step(maximum_units,maximum_bytes,grant);
            let receipt=match result{
                Ok((Some(value),receipt))=>{self.units=self.units.saturating_add(admission.progress().0);assert!(admission.terminal_is_empty());self.admission=None;self.candidate=Some(value);if self.job.is_some(){self.identity_position=Some(0);}receipt},
                Ok((None,receipt))=>receipt,
                Err(error)=>{let receipt=admission.normal_step_progress();self.units=self.units.saturating_add(admission.progress().0);self.fault=Some(error);self.cancelled=true;if let Some(job)=&mut self.job{job.cancel();}receipt},
            };return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if let Some(position)=self.identity_position{
            let identity=&self.candidate.as_ref().unwrap().1;
            let changed=identity.len()!=self.dependency_identity.len()||(position<identity.len()&&identity.as_bytes()[position]!=self.dependency_identity.as_bytes()[position]);
            if changed{self.job.as_mut().unwrap().cancel();self.cancelled=true;self.restarting=true;self.identity_position=None;}
            else if position==identity.len(){
                self.discarded_candidate=self.candidate.take();
                if let Some(version)=self.next_version.take(){self.discarded_version=Some(std::mem::replace(&mut self.version,version));}
                if let Some(id)=self.next_cancellation.take(){self.discarded_cancellation=Some(std::mem::replace(&mut self.cancellation_id,id));}self.identity_position=None;
            }else{self.identity_position=Some(position+1);}
            self.units=self.units.saturating_add(1);return Ok((EvaluationStep::Working(self.progress()),empty));
        }
        if let Some((input,identity))=self.candidate.take(){
            self.dependency_identity=identity;self.pending_input=Some(input);self.plan=neural_engine::OperatorPlanCursor::new();self.planning=true;
            self.units=self.units.saturating_add(1);return Ok((EvaluationStep::Working(self.progress()),empty));
        }
        if self.planning {
            let result=registry.dispatch_job(operator,&mut self.pending_input,&mut self.plan,grant);self.normal_progress=self.plan.step_progress();let receipt=self.normal_progress;
            match result {
                Ok(neural_engine::OperatorPlanStep::Working)=>{},
                Ok(neural_engine::OperatorPlanStep::Admitted(neural_engine::OperatorPlanAdmission::Job(job)))=>{self.job=Some(job);self.planning=false;},
                Ok(neural_engine::OperatorPlanStep::Admitted(neural_engine::OperatorPlanAdmission::Pending(original)))=>{self.pending_wave=Some(neural_engine::evaluation::wave::PendingWaveCursor::new(original));self.pending_wave_closing=false;self.planning=false;},
                Ok(neural_engine::OperatorPlanStep::Admitted(neural_engine::OperatorPlanAdmission::Immediate(original)))=>{self.pending_input=Some(original);self.planning=false;self.fault=Some("original operator has no retained evaluation plan".into());self.cancelled=true;},
                Err(error)=>{self.fault=Some(error.into());self.planning=false;self.cancelled=true;},
            }
            self.units=self.units.saturating_add(1);return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if let Some(wave)=&mut self.pending_wave{
            if self.pending_wave_closing{let step=wave.close_step(grant).map_err(|error|{self.normal_progress=wave.progress();EvaluationFailure::from(error)})?;let receipt=step.progress();if wave.terminal_is_empty(){self.pending_wave=None;self.pending_wave_closing=false;}return Ok((EvaluationStep::Working(self.progress()),receipt));}
            let result=wave.step(&self.neuron_id,grant);let receipt=wave.progress();self.normal_progress=receipt;
            match result{Ok(Some(original))=>{self.output=Some(EvaluationOutput::pending(original));wave.begin_close();self.pending_wave_closing=true;},Ok(None)=>{},Err(error)=>{self.fault=Some(error.into());self.cancelled=true;}}
            return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if let Some(job)=&mut self.job{
            let demand=RetirementDemand {copy_bytes:job.next_step_copy_byte_demand().map_err(EvaluationFailure::from)?,capacity_bytes:job.next_step_capacity_byte_demand(grant.maximum_copy_bytes).map_err(EvaluationFailure::from)?,release_bytes:job.next_step_release_byte_demand().map_err(EvaluationFailure::from)?,depth:job.next_step_depth_demand().map_err(EvaluationFailure::from)?};
            if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok((EvaluationStep::Working(self.progress()),empty));}
            self.normal_context_posted=true;let receipt=match job.step(cx){
                Ok((step,receipt))=>{
                    self.normal_progress=job.normal_step_progress();
                    if self.normal_progress!=receipt{return Err("original operator reply differs from its actual normal step receipt".into());}
                    if !receipt.fits(grant){return Err("original operator step exceeded the caller full grant".into());}
                    match step{OperatorJobStep::Working(progress)=>return Ok((EvaluationStep::Working(neural_engine::OperatorProgress {units_done:self.units.saturating_add(progress.units_done),units_total:self.units.saturating_add(progress.units_total),phase:progress.phase}),receipt)),OperatorJobStep::Done(value)=>{self.units=self.units.saturating_add(job.progress().units_total);self.finished=Some(value);self.finish=neural_engine::OperatorFinishCursor::new();},OperatorJobStep::Cancelled(_)=>{self.cancelled=true;}}receipt
                },
                Err(error)=>{let receipt=job.normal_step_progress();self.normal_progress=receipt;self.fault=Some(EvaluationFailure::Work(error));self.cancelled=true;job.cancel();receipt},
            };return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if self.finished.is_some()&&self.external_pending.is_some(){let step=evaluation_admit_original(&mut self.external_pending,&mut self.active,grant).map_err(EvaluationFailure::from)?;return Ok((EvaluationStep::Working(self.progress()),step.progress()));}
        if self.finished.is_some(){
            let result=registry.finish_job(operator,&mut self.finished,&mut self.finish,grant);self.normal_progress=self.finish.step_progress();let receipt=self.normal_progress;
            match result {Ok(Some(output))=>self.output=Some(EvaluationOutput::new(output)),Ok(None)=>{},Err(error)=>{self.fault=Some(error.into());self.cancelled=true;}}
            return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if let Some(output)=&mut self.output{
            match output.step(maximum_units,self.units,grant){Ok((Some(wire),receipt))=>{assert!(output.terminal_is_empty());self.output=None;self.completed_reply=Some(wire);return Ok((EvaluationStep::Working(self.progress()),receipt));},Ok((None,receipt))=>return Ok((EvaluationStep::Working(self.progress()),receipt)),Err(error)=>{let receipt=output.normal_step_progress();self.fault=Some(error);self.cancelled=true;return Ok((EvaluationStep::Working(self.progress()),receipt));}}
        }
        Ok((EvaluationStep::Working(self.progress()),empty))
    }
}

struct EvaluationRetirement(RetainedEvaluation);

impl semio_framework_value::retirement::RetirementCursor for EvaluationRetirement {
    fn close_step(&mut self,_grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{assert!(self.0.close_fields_terminal(false,false));semio_framework_value::retirement::RetirementStep::Complete}
    fn terminal_is_empty(&self)->bool{self.0.close_fields_terminal(false,false)}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_birth_bytes(&self,_copy:usize)->Option<usize>{Some(0)}
    fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

impl RetireOwned for RetainedEvaluation {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{assert!(self.close_fields_terminal(false,false));Box::new(EvaluationRetirement(self))}
    fn retirement_birth_bytes(&self)->Option<usize>{self.close_fields_terminal(false,false).then_some(size_of::<EvaluationRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}
// #endregion ⏱️EvaluationJobs
