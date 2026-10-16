//! 🪢️ The `FlowDomain` trait: what a Flow bridge session asks of the domain that serves its operations.

use super::*;

pub trait FlowDomain: Sized + 'static {
    fn bind_session(&mut self, _: AbiHandle) {}

    fn start_feature(domain: Rc<RefCell<Self>>, admission: FlowFeatureAdmission, operation: u16, payload: Vec<u8>) -> Result<Box<dyn FlowFeature>, FlowFailure>;
    fn begin_close(&mut self);
    fn next_close_demands(&self,copy:usize)->Result<FlowVcsCloseDemands,FlowFailure>;
    fn close_step(&mut self, budget: AbiWorkBudget) -> Result<RetainedCloneStep, FlowFailure>;
    fn terminal_is_empty(&self) -> bool;
}
