//! 🫴️ One explicitly installed caller slot retains returned decoder ownership until physical drain.
use crate::{ErasedSnapshotRetirement,SnapshotRetirementStep,ValueError};

/// 🪑️ Stack-resident recipient capacity is admitted before a decoder can create partial owners.
pub struct NativeDecodeRetirementRecipient{pub(super) owner:Option<Box<dyn ErasedSnapshotRetirement>>,pub(super) reserved:bool}
impl NativeDecodeRetirementRecipient{
    /// 🈳️ Creates the single empty caller-owned return slot without heap allocation.
    pub fn new()->Self{Self{owner:None,reserved:false}}
    /// 👓️ Reports retained ownership without borrowing or transferring the actual owner.
    pub fn has_owner(&self)->bool{self.owner.is_some()}
}
impl ErasedSnapshotRetirement for NativeDecodeRetirementRecipient{
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
        if maximum_items==0||maximum_bytes==0||self.reserved{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
        let Some(owner)=self.owner.as_mut()else{return Ok(SnapshotRetirementStep::Complete)};
        if !owner.terminal_is_empty(){return owner.close_step(1,maximum_bytes).map(|step|match step{SnapshotRetirementStep::Complete=>SnapshotRetirementStep::Pending{released_items:0,released_bytes:0},other=>other});}
        let bytes=std::mem::size_of_val(&**owner);
        if bytes>maximum_bytes{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
        self.owner.take();Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:bytes})
    }
    fn terminal_is_empty(&self)->bool{!self.reserved&&self.owner.is_none()}
    fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
}
impl Drop for NativeDecodeRetirementRecipient{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"decoder recipient retains returned physical ownership");}}

