//! 🧵️ Original JSON source and control share bounded parse and retirement wallets.
use super::{JsonError,JsonReadLimits,JsonReadSource,JsonParsedValue,JsonSourceCursor,JsonMemberPolicy,normal_remaining};
use semio_framework_value::{ValueError, ValueRefusalKind, ErasedSnapshotRetirement, retirement::RetireOwned, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::mem::ManuallyDrop;

/// 🛂️ Original parse and retirement limits.
#[derive(Clone,Copy,Debug)]
pub struct JsonReadPolicy{pub limits:JsonReadLimits,pub normal:RetainedCloneGrant,pub retirement:RetainedCloneGrant}

/// 🪪️ One source, control and partial candidate retain exact receipts.
pub struct JsonReadOperation<'operation,'callback,S:JsonReadSource+Copy,V:JsonParsedValue>{
    #[doc(hidden)] pub source:S,cursor:ManuallyDrop<Option<JsonSourceCursor<S,V>>>,#[doc(hidden)] pub closing:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    #[doc(hidden)] pub value:ManuallyDrop<Option<V>>,fault:ManuallyDrop<Option<JsonError>>,
    #[doc(hidden)] pub control:&'operation mut NativeDecodeControl<'callback>,#[doc(hidden)] pub policy:JsonReadPolicy,#[doc(hidden)] pub normal:RetainedCloneProgress,#[doc(hidden)] pub retirement:RetainedCloneProgress,
    normal_step:RetainedCloneProgress,retirement_step:RetainedCloneProgress,#[doc(hidden)] pub position:usize,close_started:bool,
}
static CLOSED_REFUSAL:JsonError=JsonError::Native(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON operation cannot parse after cleanup admission"));
fn intersect(total:RetainedCloneGrant,turn:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:total.maximum_items.min(turn.maximum_items),maximum_copy_bytes:total.maximum_copy_bytes.min(turn.maximum_copy_bytes),maximum_capacity_bytes:total.maximum_capacity_bytes.min(turn.maximum_capacity_bytes),maximum_release_bytes:total.maximum_release_bytes.min(turn.maximum_release_bytes),maximum_depth:total.maximum_depth.min(turn.maximum_depth)}}
fn debit(total:RetainedCloneGrant,used:&mut RetainedCloneProgress,receipt:RetainedCloneProgress)->Result<(),ValueError>{
    *used=used.checked_add(receipt).map_err(|error|error.with_retained_progress(receipt))?;
    if !used.fits(total){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON operation exceeded original cumulative authority").with_retained_progress(receipt));}Ok(())
}
fn admit<T:RetireOwned>(original:&mut Option<T>,closing:&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
    if original.is_none(){return Err(ValueError::invariant("original JSON outcome retained before admission"));}
    match semio_framework_value::admit_owned_retirement(original,effect).result?{
        Some((owner,receipt))=>{*closing=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},
        None=>Ok(RetainedCloneStep::Progress(Default::default())),
    }
}
fn settled(poll:semio_framework_value::OriginalRetirementPoll)->Result<RetainedCloneStep,ValueError>{match poll{
    semio_framework_value::RetainedWorkPoll::Pending{..}=>Err(ValueError::invariant("original JSON cleanup left its producer reply pending inside the step")),
    semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(|failure|{let progress=failure.retained_progress();let(primary,work,receipt,..)=failure.into_parts();primary.or(work).or(receipt).unwrap_or_else(||ValueError::invariant("original JSON cleanup refused")).with_retained_progress(progress)}),
}}
impl<'operation,'callback,S:JsonReadSource+Copy,V:JsonParsedValue> JsonReadOperation<'operation,'callback,S,V>{
    /// 🎟️ Binds the original policy and control without allocation.
    pub fn new(source:S,members:JsonMemberPolicy,policy:JsonReadPolicy,control:&'operation mut NativeDecodeControl<'callback>)->Result<Self,ValueError>{
        let cursor=JsonSourceCursor::new(source,members,policy.limits)?;
        Ok(Self{source,cursor:ManuallyDrop::new(Some(cursor)),closing:ManuallyDrop::new(None),value:ManuallyDrop::new(None),fault:ManuallyDrop::new(None),control,policy,normal:Default::default(),retirement:Default::default(),normal_step:Default::default(),retirement_step:Default::default(),position:0,close_started:false})
    }
    pub fn source(&self)->S{self.source}
    pub fn position(&self)->usize{self.position}
    pub fn normal_receipt(&self)->RetainedCloneProgress{self.normal}
    pub fn retirement_receipt(&self)->RetainedCloneProgress{self.retirement}
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.normal_step}
    pub fn retirement_step_progress(&self)->RetainedCloneProgress{self.retirement_step}
    /// 🪟️ Borrows the result under original cleanup authority.
    pub fn value(&self)->Option<&V>{self.value.as_ref()}
    /// 🚨️ Borrows the diagnostic and duplicate-name backing.
    pub fn error(&self)->Option<&JsonError>{self.fault.as_ref()}
    /// ⏱️ Caps this turn by the remaining parse allowance.
    pub fn step(&mut self,turn:RetainedCloneGrant,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<Option<&V>,&JsonError>{
        self.normal_step=Default::default();
        if self.close_started{return Err(&CLOSED_REFUSAL);}
        if self.value.is_none()&&self.fault.is_none(){
            match normal_remaining(self.policy.normal,self.normal){
                Err(error)=>{*self.fault=Some(error.into());},
                Ok(remaining)=>{
                    let admitted=intersect(remaining,turn);let cursor=self.cursor.as_mut().expect("open JSON operation retains original grammar");
                    let result=cursor.step(admitted.maximum_items,self.control,&mut effect.nested(admitted));self.normal_step=cursor.normal_step_progress();self.position=cursor.position();
                    match result{Ok(value)=>{*self.value=value;},Err(error)=>{*self.fault=Some(error);}}
                    if let Err(error)=debit(self.policy.normal,&mut self.normal,self.normal_step){if self.fault.is_none(){*self.fault=Some(error.into());}}
                },
            }
        }
        match self.fault.as_ref(){Some(error)=>Err(error),None=>Ok(self.value.as_ref())}
    }
    /// ♻️ Posts cleanup receipts; refusals retain their owners.
    pub fn close_step(&mut self,turn:RetainedCloneGrant,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
        self.retirement_step=Default::default();
        let admitted=intersect(normal_remaining(self.policy.retirement,self.retirement)?,turn);
        let result=if self.closing.is_some(){settled(semio_framework_value::close_factory_ticket(&mut self.closing,&mut effect.nested(admitted)))}else if let Some(cursor)=self.cursor.take(){
            let result=cursor.into_retirement(&mut effect.nested(admitted)).result;
            match result{
                Ok((owner,receipt))=>{*self.closing=Some(owner);self.close_started=true;Ok(RetainedCloneStep::Progress(receipt))},
                Err((error,cursor))=>{*self.cursor=Some(cursor);Err(error)},
            }
        }else if self.fault.is_some(){admit(&mut self.fault,&mut self.closing,&mut effect.nested(admitted))}else if self.value.is_some(){admit(&mut self.value,&mut self.closing,&mut effect.nested(admitted))}else{Ok(RetainedCloneStep::Complete(Default::default()))};
        self.retirement_step=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};debit(self.policy.retirement,&mut self.retirement,self.retirement_step)?;
        match result{Ok(RetainedCloneStep::Complete(receipt))if !self.terminal_is_empty()=>Ok(RetainedCloneStep::Progress(receipt)),result=>result}
    }
    pub fn terminal_is_empty(&self)->bool{self.close_started&&self.cursor.is_none()&&self.closing.is_none()&&self.value.is_none()&&self.fault.is_none()}
}
impl<S:JsonReadSource+Copy,V:JsonParsedValue> Drop for JsonReadOperation<'_,'_,S,V>{
    fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"JSON operation abandoned its original grammar, outcome or physical cleanup owner");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor);ManuallyDrop::drop(&mut self.closing);ManuallyDrop::drop(&mut self.value);ManuallyDrop::drop(&mut self.fault);}}}
}
