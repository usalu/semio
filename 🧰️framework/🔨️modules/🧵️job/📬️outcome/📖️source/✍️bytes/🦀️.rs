//! ✍️ Original staged byte sources report initialized prefixes and actual effects.
use super::*;
/// ✍️ The source reports its initialized destination prefix separately from physical retained effects.
#[derive(Clone,Copy,Debug)]
pub struct JobPayloadWriteProgress{pub initialized_bytes:usize,pub progress:RetainedCloneProgress}
/// 🧳️ A failed source keeps every initialized destination byte and its physical receipt observable.
#[derive(Clone,Copy,Debug)]
pub struct JobPayloadWriteFailure{pub fault:JobPayloadAdmissionFault,pub initialized_bytes:usize,pub progress:RetainedCloneProgress}
pub trait JobPayloadStagedSource {
    fn byte_len(&self) -> usize;
    fn copy_byte_demand(&self) -> Result<usize, JobPayloadAdmissionFault>;
    fn depth_demand(&self) -> Result<usize, JobPayloadAdmissionFault>;
    fn write_bytes(&self,target:&mut[MaybeUninit<u8>])->crate::RetainedWorkOutcome<JobPayloadWriteProgress,JobPayloadWriteFailure>;
}

impl JobPayloadStagedSource for [u8] {
    fn byte_len(&self) -> usize { self.len() }
    fn copy_byte_demand(&self) -> Result<usize, JobPayloadAdmissionFault> { Ok(self.len()) }
    fn depth_demand(&self) -> Result<usize, JobPayloadAdmissionFault> { Ok(1) }
    fn write_bytes(&self,target:&mut[MaybeUninit<u8>])->crate::RetainedWorkOutcome<JobPayloadWriteProgress,JobPayloadWriteFailure>{
        if target.len()!=self.len(){return crate::RetainedWorkOutcome{result:Err(JobPayloadWriteFailure{fault:JobPayloadAdmissionFault::StreamBytes,initialized_bytes:0,progress:Default::default()}),work:0}}
        if self.is_empty(){return crate::RetainedWorkOutcome{result:Ok(JobPayloadWriteProgress{initialized_bytes:0,progress:Default::default()}),work:0}}
        for(target,source)in target.iter_mut().zip(self){target.write(*source);}
        crate::RetainedWorkOutcome{result:Ok(JobPayloadWriteProgress{initialized_bytes:self.len(),progress:RetainedCloneProgress{copied_items:1,copied_bytes:self.len(),..Default::default()}}),work:1}
    }

}
