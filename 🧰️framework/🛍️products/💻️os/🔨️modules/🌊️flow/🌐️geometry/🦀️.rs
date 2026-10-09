//! 🌐️ Geometry work is owned by an explicitly supplied host or evaluation-session port.
pub use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
#[derive(Clone, Debug, PartialEq)]
pub enum GeometryStep {
    Working { units_done: usize, units_total: usize, phase: String },
    Ready(semio_framework::MeshData),
    Cancelled,
    Failed(String),
}
/// 🔌️ One supplied geometry authority owns its retained jobs and handle claims until explicit close.
pub trait GeometryPort: Send + Sync {
    fn begin_retain(&mut self, handles: Vec<String>) -> Result<(), (ValueError, Vec<String>)>;
    fn retain_terminal_is_empty(&self) -> bool;
    fn next_retain_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_retain_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError>;
    fn next_retain_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_retain_depth_demand(&self) -> Result<usize, ValueError>;
    fn retain_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn retain_step_progress(&self) -> RetainedCloneProgress;
    fn next_tessellate_copy_byte_demand(&self, handle: &str, tolerance: f64) -> Result<usize, ValueError>;
    fn next_tessellate_capacity_byte_demand(&self, handle: &str, tolerance: f64, copy: usize) -> Result<usize, ValueError>;
    fn next_tessellate_release_byte_demand(&self, handle: &str, tolerance: f64) -> Result<usize, ValueError>;
    fn next_tessellate_depth_demand(&self, handle: &str, tolerance: f64) -> Result<usize, ValueError>;
    fn tessellate_step(&self, handle: &str, tolerance: f64, units: usize, grant: RetainedCloneGrant) -> Result<(GeometryStep, RetainedCloneProgress), ValueError>;
    fn dispose(&self, handle: &str) -> Result<(), String>;
    fn begin_cancel(&mut self) -> Result<(), ValueError>;
    fn cancel_terminal_is_empty(&self) -> bool;
    fn next_cancel_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_cancel_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError>;
    fn next_cancel_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_cancel_depth_demand(&self) -> Result<usize, ValueError>;
    fn cancel_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn cancel_step_progress(&self) -> RetainedCloneProgress;
    fn begin_close(&self);
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
    fn terminal_is_empty(&self)->bool;
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>;
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>;
    fn next_release_byte_demand(&self)->Result<usize,ValueError>;
    fn next_depth_demand(&self)->Result<usize,ValueError>;
}

/// 🧹️ Retains the supplied authority until its resources and concrete Box allocation retire.
#[must_use = "geometry port retirement requires explicit close"]
pub struct GeometryPortRetirement { port:std::mem::ManuallyDrop<Option<Box<dyn GeometryPort>>> }
impl GeometryPortRetirement {
    pub fn new(port:Box<dyn GeometryPort>)->Self {port.begin_close();Self{port:std::mem::ManuallyDrop::new(Some(port))}}
    pub fn terminal_is_empty(&self)->bool {self.port.is_none()}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.port.as_ref().map_or(Ok(0),|port|if port.terminal_is_empty(){Ok(0)}else{port.next_copy_byte_demand()})}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.port.as_ref().map_or(Ok(0),|port|if port.terminal_is_empty(){Ok(0)}else{port.next_capacity_byte_demand(copy)})}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.port.as_ref().map_or(Ok(0),|port|if port.terminal_is_empty(){Ok(std::mem::size_of_val(port.as_ref()))}else{port.next_release_byte_demand()})}
    pub fn next_depth_demand(&self)->Result<usize,ValueError>{self.port.as_ref().map_or(Ok(0),|port|if port.terminal_is_empty(){Ok(1)}else{port.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"geometry port retirement depth overflow"))})}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}
        if grant.maximum_items==0||grant.maximum_depth<self.next_depth_demand()?{return Ok(RetainedCloneStep::Progress(empty))}
        let port=self.port.as_mut().unwrap();
        if port.terminal_is_empty(){
            let released_bytes=std::mem::size_of_val(port.as_ref());
            if grant.maximum_release_bytes<released_bytes{return Ok(RetainedCloneStep::Progress(empty))}
            drop(self.port.take());
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes,..empty}))
        }
        let child_grant=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
        let step=port.close_step(child_grant)?;
        if !step.progress().fits(child_grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"geometry port close exceeded its grant"))}
        if matches!(step,RetainedCloneStep::Complete(_))&&!port.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"geometry port completed before terminal empty"))}
        Ok(RetainedCloneStep::Progress(step.progress()))
    }
    pub fn retire_cold(mut self){
        while !self.terminal_is_empty(){
            let copy=self.next_copy_byte_demand().expect("cold geometry copy demand");
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_capacity_byte_demand(copy).expect("cold geometry capacity demand"),maximum_release_bytes:self.next_release_byte_demand().expect("cold geometry release demand"),maximum_depth:self.next_depth_demand().expect("cold geometry depth demand")};
            let step=self.close_step(grant).expect("cold geometry port retirement");
            assert!(step.progress().copied_items!=0||matches!(step,RetainedCloneStep::Complete(_)),"cold geometry port retirement stalled at exact demand");
        }
    }
}
impl Drop for GeometryPortRetirement {
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"geometry port abandoned original ownership");if self.terminal_is_empty(){unsafe{std::mem::ManuallyDrop::drop(&mut self.port)}}}
}
#[cfg(all(test,not(target_arch="wasm32")))]
#[path="🧪️tests/♻️ownership/🦀️.rs"]
mod ownership_tests;
