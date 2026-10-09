//! 🧺️ Owned physical operation rows retain partial output until explicitly funded retirement.
use semio_framework_value::{ErasedSnapshotRetirement,NativeEncodeControl,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress}};

pub(crate) struct OperationWireRows{rows:Option<Vec<Vec<u8>>>,active:Option<Vec<u8>>}
impl OperationWireRows{
    pub(crate) fn new()->Self{Self{rows:None,active:None}}
    pub(crate) fn initialize(&mut self,count:usize,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.rows=Some(control.allocate_vec(count)?);Ok(())}
    pub(crate) fn active(&mut self)->&mut Vec<u8>{self.active.get_or_insert_with(Vec::new)}
    pub(crate) fn append(&mut self){self.rows.as_mut().expect("admitted wire rows").push(self.active.take().expect("admitted active wire row"));}
    pub(crate) fn take(&mut self)->Vec<Vec<u8>>{self.rows.take().expect("admitted wire rows")}
}
impl ErasedSnapshotRetirement for OperationWireRows{
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_capacity_byte_demand(&self,_copy:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.active.as_ref().map(Vec::capacity).or_else(||self.rows.as_ref().map(|rows|rows.last().map(Vec::capacity).unwrap_or_else(||rows.capacity()*std::mem::size_of::<Vec<u8>>()))).unwrap_or(0))}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal_is_empty()))}
    fn terminal_is_empty(&self)->bool{self.active.is_none()&&self.rows.is_none()}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"operation wire rows require admitted retirement depth"));}
        let release=self.next_release_byte_demand()?;
        if release>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.active.is_some(){self.active=None;}else if self.rows.as_ref().is_some_and(|rows|!rows.is_empty()){self.rows.as_mut().unwrap().pop();}else{self.rows=None;}
        let progress=RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()};
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
}
