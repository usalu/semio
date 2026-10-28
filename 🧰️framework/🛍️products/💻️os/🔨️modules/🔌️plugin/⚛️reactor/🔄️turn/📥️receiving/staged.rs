//! 📥️ Standalone staged bytes remain original-owned through Pack, census, exact movement and close.
use super::ReceivingSource;
use semio_framework::action_staging::{StagedFormLimits,StagedFormRequest,StagedFormRequestCursor};
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_job_native_decode::{OriginalNativeDecodeIssued,OriginalNativeDecodePolicy,OriginalNativeDecodeSource};
use semio_framework_os_kernel::os_pack::pack_rt::{RetainedRecordReceiving,wire_value_spec,take_received_wire_value};
use semio_framework_value::{DslValue,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,size_of};

#[derive(semio_framework_value::RetireOwned)]
struct Inputs {wire:Vec<u8>,value:Option<DslValue>,parsed:Option<Box<DslValue>>,typed:Option<StagedFormRequest>,issue:Option<ValueError>,cursor:StagedFormRequestCursor}
pub struct StagedRequestInput {
    witness:ReceivingSource,
    inputs:ManuallyDrop<Option<Inputs>>,
    retiring:ManuallyDrop<Option<ControlledRetirement<Inputs>>>,
    receiver:ManuallyDrop<Option<RetainedRecordReceiving>>,
    native:ManuallyDrop<OriginalNativeDecodeSource>,
    limits:StagedFormLimits,
    maximum_owned_bytes:usize,
    phase:u8,
    failed:bool,
    closing:bool,
    closed:bool,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.source-invalid")}
fn funded(cx:&StepContext<'_>,demand:RetirementDemand,closing:bool)->bool{
    let grant=cx.retained_grant();
    !(if closing{cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})&&grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth
}
fn post(cx:&mut StepContext<'_>,progress:RetainedCloneProgress)->Result<(),OriginalTurnFailure>{cx.consume_fuel(u64::from(progress.copied_items!=0));cx.consume_retained(progress)?;Ok(())}
fn close_metadata(cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !funded(cx,RetirementDemand{depth:1,..Default::default()},true){return Ok(false);}post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(true)}
impl StagedRequestInput {
    pub(super) fn validate_unissued(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.witness.validate(cx)?;if self.phase!=0||self.failed||self.closing{return Err(invalid().into());}Ok(())}
    /// 🎟️ Adopts bytes only after the initiating caller funds inline custody and explicit decoder capacity policy.
    pub fn admit(donor:&mut Option<Vec<u8>>,limits:StagedFormLimits,maximum_owned_bytes:usize,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{
        let limits=limits.admitted()?;let wire=donor.as_ref().ok_or_else(invalid)?;
        if wire.len()>limits.wire_bytes()?{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"staging.resource-limit").into());}
        if maximum_owned_bytes==0{return Err(invalid().into());}
        let bytes=size_of::<Self>()+size_of::<Inputs>();
        if !funded(cx,RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()},false){return Ok(None);}
        let witness=ReceivingSource::new(cx)?;let cursor=StagedFormRequestCursor::new_wire(limits)?;
        post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
        Ok(Some(Self{witness,inputs:ManuallyDrop::new(Some(Inputs{wire:donor.take().unwrap(),value:None,parsed:None,typed:None,issue:None,cursor})),retiring:ManuallyDrop::new(None),receiver:ManuallyDrop::new(None),native:ManuallyDrop::new(OriginalNativeDecodeSource::empty(OriginalNativeDecodePolicy{maximum_owned_bytes})),limits,maximum_owned_bytes,phase:0,failed:false,closing:false,closed:false}))
    }
    pub fn source_bytes(&self)->Option<&[u8]>{self.inputs.as_ref().map(|inputs|inputs.wire.as_slice())}
    fn advance(&mut self,cx:&mut StepContext<'_>)->Result<Option<StagedFormRequest>,OriginalTurnFailure>{
        if !self.witness.admit(cx)?||cx.should_yield()||cx.is_cancelled(){return Ok(None);}
        match self.phase{
            0=>{
                let grant=cx.retained_grant();let wire=&self.inputs.as_ref().unwrap().wire;
                let maximum=self.limits.wire_bytes()? as u64;
                let limits=pack::PackLimits{max_file_len:maximum,max_segment_len:maximum,max_symbols:self.limits.nodes as u32,max_depth:(self.limits.depth+2)as u16,max_items:maximum,max_total_alloc:self.maximum_owned_bytes as u64};
                if let Some((receiver,progress))=RetainedRecordReceiving::admit(wire,(0,wire.len()),wire_value_spec(),limits,grant)?{*self.receiver=Some(receiver);post(cx,progress)?;self.phase=1;}Ok(None)
            },
            1=>{
                let receiver=self.receiver.as_mut().unwrap();if receiver.is_ready(){self.phase=2;return Ok(None);}
                let wire=&self.inputs.as_ref().unwrap().wire;let demand=receiver.demands(wire)?;
                if !funded(cx,demand,false){return Ok(None);}
                let Some(outcome)=self.native.receive_step(cx,|control,grant|{
                    let result=receiver.step(wire,control,grant);let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
                    OriginalNativeDecodeIssued{value:result.map(|_|()),grant,progress,work:u64::from(progress.copied_items!=0)}
                })?else{return Ok(None)};
                if let Some(failure)=outcome.receiving_refusal{if let Err(error)=outcome.value{self.inputs.as_mut().unwrap().issue=Some(error);}return Err(failure);}outcome.value?;Ok(None)
            },
            2=>{
                let progress=take_received_wire_value(self.receiver.as_mut().unwrap(),&mut self.inputs.as_mut().unwrap().value,cx.retained_grant())?;
                post(cx,progress)?;if progress.copied_items!=0{self.phase=3;}Ok(None)
            },
            3=>{
                let bytes=size_of::<DslValue>();let demand=RetirementDemand{copy_bytes:2*bytes,capacity_bytes:bytes,depth:1,..Default::default()};
                if !funded(cx,demand,false){return Ok(None);}
                let inputs=self.inputs.as_mut().unwrap();inputs.parsed=Some(Box::new(inputs.value.take().ok_or_else(invalid)?));
                post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:bytes,..Default::default()})?;self.phase=4;Ok(None)
            },
            4=>{
                let inputs=self.inputs.as_mut().unwrap();
                let result=inputs.cursor.step(inputs.parsed.as_deref_mut().ok_or_else(invalid)?,&mut inputs.typed,cx.retained_grant());
                let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
                if let Err(failure)=post(cx,progress){if let Err(error)=result{inputs.issue=Some(error);}return Err(failure);}result?;
                if inputs.typed.is_some(){self.phase=5;}Ok(inputs.typed.take())
            },
            5=>Ok(None),
            _=>Err(invalid().into()),
        }
    }
    /// 🧷️ Only the authentic initiating caller may advance this private stationary parsed root.
    pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<Option<StagedFormRequest>,OriginalTurnFailure>{
        self.witness.validate(cx)?;
        if self.failed||self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"staged receiving cannot resume after refusal or close").into());}
        let result=self.advance(cx);if result.is_err(){self.failed=true;}result
    }
    /// ♻️ Decoder, private payload and original loans retire before this input becomes terminal.
    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.witness.validate(cx)?;self.closing=true;
        if let Some(receiver)=self.receiver.as_mut(){
            if !receiver.terminal_is_empty(){let demand=receiver.close_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|receiver.close_step(grant))?;return Ok(false);}
            if !close_metadata(cx)?{return Ok(false);}self.receiver.take();return Ok(false);
        }
        if let Some(owner)=self.retiring.as_mut(){
            if !owner.terminal_is_empty(){semio_framework_job::close_original_retained_step(cx,owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?,|grant|owner.step(grant))?;return Ok(false);}
            if !close_metadata(cx)?{return Ok(false);}self.retiring.take();return Ok(false);
        }
        if self.inputs.is_some(){
            let demand=RetirementDemand{copy_bytes:size_of::<Inputs>(),depth:1,..Default::default()};if !funded(cx,demand,true){return Ok(false);}
            let inputs=self.inputs.take().unwrap();match ControlledRetirement::new(inputs){Ok(owner)=>*self.retiring=Some(owner),Err((error,inputs))=>{*self.inputs=Some(inputs);return Err(error.into());}}
            post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;return Ok(false);
        }
        if !self.native.terminal_is_empty(){self.native.close_step(cx)?;return Ok(false);}
        if !self.witness.close(cx)?{return Ok(false);}
        if !close_metadata(cx)?{return Ok(false);}self.closed=true;Ok(true)
    }
    pub fn terminal_is_empty(&self)->bool{self.closed&&self.inputs.is_none()&&self.retiring.is_none()&&self.receiver.is_none()&&self.native.terminal_is_empty()}
}
impl Drop for StagedRequestInput{
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"staged request input requires original paid retirement");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.native)}}}
}
