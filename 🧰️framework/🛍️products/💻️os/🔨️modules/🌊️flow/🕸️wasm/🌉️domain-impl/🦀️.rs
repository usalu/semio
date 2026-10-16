//! 🌉️ The protocol `FlowDomain` implementation of the Flow domain adapter: feature start, close ladder and terminal emptiness.

use super::*;

impl FlowDomain for FlowDomainAdapter {
    fn bind_session(&mut self, session: semio_framework::abi::AbiHandle) {
        self.vcs = Some(FlowRetainedVcs::new(semio_framework_artifact_flow_flow::artifact::FlowHostSnapshot::default(), session.generation(), 0, 0));
    }

    fn start_feature(domain: Rc<RefCell<Self>>, admission: FlowFeatureAdmission, operation: u16, payload: Vec<u8>) -> Result<Box<dyn FlowFeature>, FlowFailure> {
        if matches!(operation, 2_501..=2_503) {
            return FlowVcsFeature::admit(domain, admission, operation, payload).map(|feature| Box::new(feature) as Box<dyn FlowFeature>);
        }
        let arguments = FlowArguments::preflight(operation, payload)?;
        let observer=Rc::new(FlowOperationObserver::new());
        let mut action = flow_action(operation, &arguments).ok_or_else(|| abi_failure(AbiErrorCode::UnknownOperation))?;
        observer.bound.set(true);
        action.bind_admission(admission)?;
        action.bind_observer(observer.clone());
        Ok(Box::new(FlowProgramFeature { domain, arguments, action, observer,admission,operation,dag_read_lease:None,terminal_failure:None }))
    }

    fn begin_close(&mut self) {
        if let Some(lease)=self.dag_read_lease.upgrade(){lease.active.set(false);}
        let previous = std::mem::replace(&mut self.host, FlowDomainHost::Closed);
        self.host = match previous {
            FlowDomainHost::Open(host) => FlowDomainHost::Closing(FlowHostRetirement::new(host)),
            retained => retained,
        };
        if let Some(vcs) = self.vcs.as_mut() { vcs.begin_close(); }
        self.surface = None;
        if self.frame_retirement.is_none(){self.frame_retirement=Some(ControlledRetirement::new(std::mem::take(&mut self.frame_payload)).unwrap_or_else(|_|unreachable!("original String declares controlled retirement")));}
    }

    fn next_close_demands(&self,copy:usize)->Result<FlowVcsCloseDemands,FlowFailure>{
        if let Some(frame)=self.frame_retirement.as_ref(){
            if frame.terminal_is_empty(){return Ok(FlowVcsCloseDemands{depth:1,..Default::default()})}
            return Ok(FlowVcsCloseDemands{copy_bytes:frame.next_copy_byte_demand().map_err(flow_close_value_failure)?,capacity_bytes:frame.next_capacity_byte_demand(copy).map_err(flow_close_value_failure)?,release_bytes:frame.next_release_byte_demand().map_err(flow_close_value_failure)?,depth:frame.next_depth_demand().map_err(flow_close_value_failure)?.checked_add(1).ok_or_else(||flow_close_failure(AbiErrorCode::LimitExceeded))?})
        }
        if let Some(vcs)=self.vcs.as_ref(){
            if vcs.terminal_is_empty(){return Ok(FlowVcsCloseDemands{depth:1,..Default::default()})}
            let mut demands=vcs.next_close_retired_demands(copy).map_err(flow_vcs_close_failure)?;demands.depth=demands.depth.checked_add(1).ok_or_else(||flow_close_failure(AbiErrorCode::LimitExceeded))?;return Ok(demands)
        }
        match &self.host{
            FlowDomainHost::Open(_)=>Err(flow_close_failure(AbiErrorCode::Busy)),
            FlowDomainHost::Closing(host)if host.terminal_is_empty()=>Ok(FlowVcsCloseDemands{depth:1,..Default::default()}),
            FlowDomainHost::Closing(host)=>Ok(FlowVcsCloseDemands{copy_bytes:host.next_close_copy_byte_demand().map_err(flow_close_value_failure)?,capacity_bytes:host.next_close_capacity_byte_demand(copy).map_err(flow_close_value_failure)?,release_bytes:host.next_close_release_byte_demand().map_err(flow_close_value_failure)?,depth:host.next_close_depth_demand().map_err(flow_close_value_failure)?.checked_add(1).ok_or_else(||flow_close_failure(AbiErrorCode::LimitExceeded))?}),
            FlowDomainHost::Closed=>Ok(Default::default()),
        }
    }

    /// 🪜️ One original child receives one incoming wallet and returns its complete actual receipt.
    fn close_step(&mut self,budget:AbiWorkBudget)->Result<RetainedCloneStep,FlowFailure>{
        validate_budget(budget).map_err(flow_close_failure)?;
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        let demands=self.next_close_demands(budget.retained.maximum_copy_bytes)?;
        if budget.retained.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if budget.retained.maximum_depth<demands.depth{return Err(flow_close_failure(AbiErrorCode::LimitExceeded))}
        if budget.retained.maximum_capacity_bytes<demands.capacity_bytes||budget.retained.maximum_release_bytes<demands.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let inline=RetainedCloneProgress{copied_items:1,..Default::default()};
        let child=AbiWorkBudget{retained:RetainedCloneGrant{maximum_depth:budget.retained.maximum_depth-1,..budget.retained},..budget};
        if let Some(frame)=self.frame_retirement.as_mut(){
            if frame.terminal_is_empty(){self.frame_retirement=None;return Ok(RetainedCloneStep::Progress(inline))}
            let step=frame.step(child.retained).map_err(flow_close_value_failure)?;
            let step=admit_retained_clone_close(child.retained,step,frame.terminal_is_empty(),"original Flow domain frame").map_err(flow_close_value_failure)?;
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        if let Some(vcs)=self.vcs.as_mut(){
            if vcs.terminal_is_empty(){self.vcs=None;return Ok(RetainedCloneStep::Progress(inline))}
            let step=vcs.close_retired_step(flow_vcs_grant(child)).map_err(flow_vcs_close_failure)?;
            let step=admit_retained_clone_close(child.retained,step,vcs.terminal_is_empty(),"original Flow domain VCS").map_err(flow_close_value_failure)?;
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        match &mut self.host{
            FlowDomainHost::Open(_)=>Err(flow_close_failure(AbiErrorCode::Busy)),
            FlowDomainHost::Closing(host)=>{
                if host.terminal_is_empty(){self.host=FlowDomainHost::Closed;return Ok(RetainedCloneStep::Complete(inline))}
                let step=host.close_step(child.retained).map_err(flow_close_value_failure)?;
                let step=admit_retained_clone_close(child.retained,step,host.terminal_is_empty(),"original Flow domain host").map_err(flow_close_value_failure)?;
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            FlowDomainHost::Closed=>Ok(RetainedCloneStep::Complete(Default::default())),
        }
    }

    fn terminal_is_empty(&self)->bool{self.vcs.is_none()&&self.surface.is_none()&&self.frame_retirement.is_none()&&self.frame_payload.capacity()==0&&matches!(self.host,FlowDomainHost::Closed)}
}
