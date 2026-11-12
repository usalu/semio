//! 🔢 Genuine original-byte decoding into a funded typed IDML version field.
use super::{IdmlVersionView,IdmlVersionDecoder};
use semio_framework_value::{RetainedAllocationEffect,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_progress}};
use semio_os_kernel::{RetainedFlight as Flight,retained_polled as polled};
use std::mem::{ManuallyDrop,size_of};
const FIXED:usize=256;
const UNIT:usize=64;
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items!=0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}
/// 🧳 One original lexical span becomes one typed String with no unquoted source clone.
pub struct RetainedIdmlVersion {view:IdmlVersionView,decoder:IdmlVersionDecoder,source:Option<RetainedCloneBinding>,value:ManuallyDrop<Option<String>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl RetainedIdmlVersion {
    /// 🌱 Only fixed source coordinates are captured before the original lease is funded.
    pub fn new(view:IdmlVersionView)->Self{Self{view,decoder:IdmlVersionDecoder::new(view),source:None,value:ManuallyDrop::new(None),retirement:Default::default(),phase:0,closing:false}}
    fn steps(body:usize,maximum:u64)->usize{((body.saturating_sub(FIXED))/UNIT).min(1024).min(usize::try_from(maximum.saturating_sub(1)).unwrap_or(usize::MAX))}
    fn check(&self,original:RetainedCloneRef<'_,Vec<u8>>)->Result<(),ValueError>{
        if self.closing||self.phase>1{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML version field already settled"));}
        if self.view.value.start>self.view.value.end||self.view.value.end>original.get().len()||self.view.decoded_length<3||self.view.decoded_length>crate::MAX_PACKAGE_BYTES{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"IDML version source extent is not admitted"));}
        Ok(())
    }
    /// 📏 Separately quotes original custody, String capacity, or bounded reference decoding.
    pub fn demand(&self,original:RetainedCloneRef<'_,Vec<u8>>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{
        self.check(original)?;
        if !original.check_binding(&self.source)?{return Ok(RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()});}
        if self.phase==0{return Ok(RetirementDemand{copy_bytes:FIXED+size_of::<String>()*2,capacity_bytes:self.view.decoded_length,depth:1,..Default::default()});}
        if body<FIXED+UNIT{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"IDML version body cannot fit one source unit"));}
        Ok(RetirementDemand{copy_bytes:FIXED+Self::steps(body,maximum.max(2))*UNIT,depth:1,..Default::default()})
    }
    /// ⛽ Quotes the alias duplication, the reservation or the bounded decoding slice in original work units.
    pub fn work_demand(&self,original:RetainedCloneRef<'_,Vec<u8>>,body:usize,maximum:u64)->Result<u64,ValueError>{
        self.check(original)?;
        let work=if !original.check_binding(&self.source)?{original.binding_work_demand(&self.source)?}else if self.phase==0{1}else{Self::steps(body,maximum).max(1) as u64};
        Ok(if work<=maximum{work}else{0})
    }
    /// ▶ Appends at most one admitted ASCII scalar after bounded original member reads.
    pub fn advance(&mut self,original:RetainedCloneRef<'_,Vec<u8>>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<RetainedCloneStep,ValueError>{
        let grant=effect.grant();
        let demand=self.demand(original,body,maximum)?;if !fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
        let turn=original.bind(&mut self.source,effect);*work+=turn.work;
        if let Some(progress)=turn.result?{return Ok(RetainedCloneStep::Progress(progress));}
        if self.phase==0{*work+=1;let mut value=String::new();value.try_reserve_exact(self.view.decoded_length).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"IDML version capacity reservation failed"))?;let capacity=value.capacity();*self.value=Some(value);self.phase=1;return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity,..Default::default()},"IDML decoded version reservation")?));}
        let step=self.decoder.advance(original.get(),Self::steps(body,maximum).max(1));*work+=step.work.max(1) as u128;let progress=RetainedCloneProgress{copied_items:1,copied_bytes:FIXED+step.work*UNIT,..Default::default()};
        if let Some(error)=step.error{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,error).with_retained_progress(progress));}
        if let Some(byte)=step.byte{let value=self.value.as_mut().expect("reserved version field");if value.len()>=self.view.decoded_length||value.len()==value.capacity(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML version append exceeds its admitted capacity").with_retained_progress(progress));}value.push(byte as char);}
        if step.done{self.phase=2;return Ok(RetainedCloneStep::Complete(admit_retained_clone_progress(grant,progress,"IDML decoded version field")?));}
        Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant,progress,"IDML decoded version source")?))
    }
    /// 👁 A field becomes transferable only after complete source/reference decoding.
    pub fn ready(&self)->bool{!self.closing&&self.phase==2&&self.value.is_some()}
    /// 📤 Moving the typed field needs only its original fixed String headers.
    pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML version field is not ready"));}Ok(RetirementDemand{copy_bytes:size_of::<String>()*2+size_of::<u8>(),depth:1,..Default::default()})}
    /// 🎁 Transfers owned field bytes once; the original source alias remains closeable.
    pub fn take(&mut self,grant:RetainedCloneGrant)->Result<(Option<String>,RetainedCloneProgress),ValueError>{let d=self.take_demand()?;if !fits(d,grant){return Ok((None,Default::default()));}self.phase=3;Ok((self.value.take(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
    /// 🛑 Partial output and original member authority stay owned through cancellation.
    pub fn begin_close(&mut self){self.closing=true;}
    /// ♻ Partial Strings retire before the exact original mutation/member alias.
    pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML version closure not started"));}self.retirement.next_owner_demands::<String>(self.value.is_some(),&self.source,body)}
    /// ⛽ Quotes the selected typed String retirement or original alias action.
    pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML version closure not started"));}if !self.retirement.is_empty()||self.value.is_some(){self.retirement.next_owner_work::<String>(self.value.is_some(),&self.source,body)}else{RetainedCloneBinding::close_work_demand(&self.source,body)}}
    /// 🪜 Executes one admitted typed String retirement or original alias action.
    pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,flight:&mut Flight,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
        if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML version closure not started").into());}
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if !self.retirement.is_empty(){let step=polled(flight,effect,recover,1,body,work,|e|self.retirement.step_granted(body,e))?.map_err(RetirementPollExit::from)?;return Ok(RetainedCloneStep::Progress(step.progress()));}
        if self.value.is_some(){let turn=self.retirement.begin_granted(&mut self.value,effect);*work+=turn.work;if let Some(step)=turn.result?{return Ok(RetainedCloneStep::Progress(step.progress()));}}
        let step=polled(flight,effect,recover,2,body,work,|e|RetainedCloneBinding::close_one(&mut self.source,body,e))?.map_err(RetirementPollExit::from)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    /// 🪹 No staged String or original member lease remains.
    pub fn terminal_is_empty(&self)->bool{self.closing&&self.value.is_none()&&self.retirement.is_empty()&&self.source.is_none()}
}
impl Drop for RetainedIdmlVersion {fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"IDML version field requires original partial-owner closure");if empty{unsafe{ManuallyDrop::drop(&mut self.value);}}}}
