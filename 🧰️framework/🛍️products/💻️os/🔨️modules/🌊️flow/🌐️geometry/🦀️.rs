//! 🌐️ Geometry work is owned by an explicitly supplied host or evaluation-session port.
#[derive(Clone, Debug, PartialEq)]
pub enum GeometryStep {
    Working { units_done: usize, units_total: usize, phase: String },
    Ready(semio_framework::MeshData),
    Cancelled,
    Failed(String),
}
/// 🔌️ One supplied geometry authority owns its retained jobs and handle claims until explicit close.
pub trait GeometryPort: Send + Sync {
    fn retain(&self, handles: &[String]);
    fn tessellate_step(&self, handle: &str, tolerance: f64, units: usize) -> GeometryStep;
    fn dispose(&self, handle: &str) -> Result<(), String>;
    fn cancel(&self) -> usize;
    fn begin_close(&self);
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize) -> Result<neural_engine::ValueRetirementStep,String>;
    fn terminal_is_empty(&self) -> bool;
    fn next_close_byte_demand(&self) -> usize;
}

/// 🧹️ Retains the supplied authority until its resources and concrete Box allocation retire.
#[must_use = "geometry port retirement requires explicit close"]
pub struct GeometryPortRetirement { port:std::mem::ManuallyDrop<Option<Box<dyn GeometryPort>>> }
impl GeometryPortRetirement {
    pub fn new(port:Box<dyn GeometryPort>) -> Self { port.begin_close(); Self { port:std::mem::ManuallyDrop::new(Some(port)) } }
    pub fn terminal_is_empty(&self) -> bool { self.port.is_none() }
    pub fn next_close_byte_demand(&self) -> usize {
        self.port.as_ref().map_or(0,|port| if port.terminal_is_empty() { std::mem::size_of_val(port.as_ref()) } else { port.next_close_byte_demand() })
    }
    pub fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize) -> Result<neural_engine::ValueRetirementStep,String> {
        use neural_engine::ValueRetirementStep as Step;
        let Some(port) = self.port.as_mut() else { return Ok(Step::Complete) };
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(Step::Blocked); }
        if port.terminal_is_empty() {
            let released_bytes = std::mem::size_of_val(port.as_ref());
            if maximum_bytes < released_bytes { return Ok(Step::Blocked); }
            drop(self.port.take());
            return Ok(Step::Pending { released_items:1,released_bytes });
        }
        match port.close_step(maximum_items,maximum_bytes)? {
            Step::Pending { released_items,released_bytes } if released_items > maximum_items || released_bytes > maximum_bytes => Err("flow.geometry-port-close-overspend".into()),
            Step::Complete if !port.terminal_is_empty() => Err("flow.geometry-port-close-not-empty".into()),
            Step::Complete => Ok(Step::Pending { released_items:0,released_bytes:0 }),
            step => Ok(step),
        }
    }
    pub fn retire_cold(mut self) {
        while !self.terminal_is_empty() {
            let step = self.close_step(1,4096.max(self.next_close_byte_demand())).expect("cold geometry port retirement");
            assert!(!matches!(step,neural_engine::ValueRetirementStep::Blocked),"cold geometry port retirement is blocked");
        }
    }
}
impl Drop for GeometryPortRetirement {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.terminal_is_empty(),"geometry port requires explicit terminal retirement before drop"); } }
}
