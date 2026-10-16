//! 🌿️ The Flow VCS feature: admission, polling, close failures and the fault-to-code mapping.

use super::*;

pub fn flow_close_failure(code:AbiErrorCode)->FlowFailure{FlowFailure::new(code,String::new())}
pub fn flow_close_value_failure(error:semio_framework_value::ValueError)->FlowFailure{FlowFailure::from_value(AbiErrorCode::Busy,error)}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlowVcsFeaturePhase {
    Admit,
    Poll,
    Fault,
    Lease,
    Resume,
    Retry,
    AwaitAcknowledgement,
    CloseOperation,
    CloseRetired,
    Complete,
}

pub struct FlowVcsFeature {
    pub domain: Rc<RefCell<FlowDomainAdapter>>,
    pub admission: FlowFeatureAdmission,
    pub(super) authority: FlowVcsAuthority,
    pub(super) handle: Option<FlowVcsHandle>,
    pub(super) page: Option<FlowVcsPage>,
    pub(super) phase: FlowVcsFeaturePhase,
    fault_requested: bool,
    pub terminal_failure: Option<FlowFailure>,
}

impl FlowVcsFeature {
    pub fn admit(domain: Rc<RefCell<FlowDomainAdapter>>, admission: FlowFeatureAdmission, operation: u16, payload: Vec<u8>) -> Result<Self, FlowFailure> {
        let mut reader = FlowPayloadReader::new(&payload);
        let authority = FlowVcsAuthority { session_generation: reader.u32().map_err(abi_code_failure)?, base_revision: reader.u64().map_err(abi_code_failure)?, parent_revision: reader.u64().map_err(abi_code_failure)? };
        reader.finish().map_err(abi_code_failure)?;
        if authority.session_generation != admission.session.generation() || admission.request_generation == 0 {
            return Err(abi_failure(AbiErrorCode::StaleGeneration));
        }
        let current = domain.borrow().vcs.as_ref().ok_or_else(|| abi_failure(AbiErrorCode::Closed))?.authority();
        if current != authority {
            return Err(flow_vcs_failure(FlowVcsFault::StaleAuthority));
        }
        Ok(Self { domain, admission, authority, handle: None, page: None, phase: FlowVcsFeaturePhase::Admit, fault_requested: operation == 2_502, terminal_failure: None })
    }

    fn grant(&self, budget: AbiWorkBudget) -> FlowVcsGrant {
        flow_vcs_grant(budget)
    }

    fn close_cursor_step(&mut self,budget:AbiWorkBudget)->Result<RetainedCloneStep,FlowFailure>{
        let grant=self.grant(budget);let handle=self.handle.expect("admitted Flow VCS handle");let mut domain=self.domain.borrow_mut();let vcs=domain.vcs.as_mut().ok_or_else(||flow_close_failure(AbiErrorCode::Closed))?;
        match self.phase{
            FlowVcsFeaturePhase::CloseOperation=>{let step=vcs.close_operation_step(handle,grant).map_err(flow_vcs_close_failure)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=FlowVcsFeaturePhase::CloseRetired;}Ok(RetainedCloneStep::Progress(step.progress()))}
            FlowVcsFeaturePhase::CloseRetired=>match vcs.close_retired_step(grant){
                Ok(step@RetainedCloneStep::Complete(_))=>{self.phase=FlowVcsFeaturePhase::Complete;Ok(step)}
                Ok(step)=>Ok(step),
                Err(failure)if failure.fault==FlowVcsFault::ClosePending&&failure.retained_progress==RetainedCloneProgress::default()=>Ok(RetainedCloneStep::Progress(Default::default())),
                Err(failure)=>Err(flow_vcs_close_failure(failure)),
            },
            FlowVcsFeaturePhase::Complete=>Ok(RetainedCloneStep::Complete(Default::default())),
            _=>Err(flow_close_failure(AbiErrorCode::Busy)),
        }
    }

    fn encode_checkpoint(&self, operation: u64, revision: u64) -> Vec<u8> {
        let mut writer = FlowPayloadWriter::default();
        writer.u32(self.admission.session.slot());
        writer.u32(self.admission.session.generation());
        writer.u32(self.admission.request_generation);
        writer.u64(operation);
        writer.u64(revision);
        writer.finish()
    }

    fn encode_preview(&self, widgets: u32, synapses: u32, layout: u32) -> Vec<u8> {
        let mut writer = FlowPayloadWriter::default();
        writer.u32(widgets);
        writer.u32(synapses);
        writer.u32(layout);
        writer.finish()
    }

    fn encode_page(&self, page: FlowVcsPage) -> Vec<u8> {
        let mut writer = FlowPayloadWriter::default();
        writer.u32(self.admission.session.slot());
        writer.u32(self.admission.session.generation());
        writer.u32(self.admission.request_generation);
        writer.u32(self.authority.session_generation);
        writer.u64(self.authority.base_revision);
        writer.u64(self.authority.parent_revision);
        let handle = self.handle.expect("leased Flow VCS handle");
        writer.u64(handle.operation);
        writer.u8(handle.slot);
        writer.u32(handle.generation);
        writer.u64(page.sequence);
        writer.u64(page.operation);
        writer.u32(page.session_generation);
        writer.u64(page.revision);
        writer.u64(page.parent_revision);
        writer.u64(page.document_generation);
        writer.u32(page.widget_count);
        writer.u32(page.synapse_count);
        writer.u32(page.layout_count);
        writer.u64(page.semantic_digest);
        writer.finish()
    }
}

impl FlowFeature for FlowVcsFeature {
    fn step(&mut self, budget: AbiWorkBudget) -> FlowFeatureStep {
        if self.domain.borrow().dag_read_lease.upgrade().is_some_and(|lease|lease.active.get()){return FlowFeatureStep::Yield;}
        let grant = self.grant(budget);
        match self.phase {
            FlowVcsFeaturePhase::Admit => {
                let result = self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.begin_checkpoint(self.authority));
                match result {
                    Ok(handle) => {
                        self.handle = Some(handle);
                        self.phase = FlowVcsFeaturePhase::Poll;
                        FlowFeatureStep::Progress { completed: 0, total: 3 }
                    }
                    Err(fault) => FlowFeatureStep::Failed(flow_vcs_failure(fault)),
                }
            }
            FlowVcsFeaturePhase::Poll => {
                let handle = self.handle.expect("admitted Flow VCS handle");
                let result = self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.poll(handle, grant));
                match result {
                    Ok(FlowVcsPoll::Progress { completed, total }) => FlowFeatureStep::Progress { completed: u64::from(completed), total: u64::from(total) },
                    Ok(FlowVcsPoll::Checkpoint { operation, revision }) => {
                        if self.fault_requested {
                            self.terminal_failure = Some(FlowFailure::new(AbiErrorCode::MalformedTag, "Flow VCS requested fault"));
                            self.phase = FlowVcsFeaturePhase::Fault;
                        }
                        FlowFeatureStep::Checkpoint(self.encode_checkpoint(operation, revision))
                    }
                    Ok(FlowVcsPoll::Preview { widgets, synapses, layout }) => FlowFeatureStep::Preview(self.encode_preview(widgets, synapses, layout)),
                    Ok(FlowVcsPoll::PageReady { .. }) => {
                        self.phase = FlowVcsFeaturePhase::Lease;
                        FlowFeatureStep::Yield
                    }
                    Ok(FlowVcsPoll::Terminal) => {
                        self.phase = FlowVcsFeaturePhase::CloseOperation;
                        FlowFeatureStep::Yield
                    }
                    Err(fault) => {
                        self.terminal_failure = Some(flow_vcs_failure(fault));
                        self.phase = FlowVcsFeaturePhase::Fault;
                        FlowFeatureStep::Yield
                    }
                }
            }
            FlowVcsFeaturePhase::Fault => {
                let handle = self.handle.expect("admitted Flow VCS handle");
                let result = self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.fault(handle, grant));
                match result {
                    Ok(()) => {
                        self.phase = FlowVcsFeaturePhase::CloseOperation;
                        FlowFeatureStep::Yield
                    }
                    Err(fault) => FlowFeatureStep::Failed(flow_vcs_failure(fault)),
                }
            }
            FlowVcsFeaturePhase::Lease => {
                let handle = self.handle.expect("admitted Flow VCS handle");
                match self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.take_page(handle)) {
                    Ok(page) => {
                        self.page = Some(page);
                        self.phase = FlowVcsFeaturePhase::Resume;
                        FlowFeatureStep::Yield
                    }
                    Err(fault) => FlowFeatureStep::Failed(flow_vcs_failure(fault)),
                }
            }
            FlowVcsFeaturePhase::Resume => {
                let sequence = self.page.expect("leased Flow VCS page").sequence;
                let handle = self.handle.expect("admitted Flow VCS handle");
                match self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.resume_page(handle, sequence)) {
                    Ok(()) => {
                        self.phase = FlowVcsFeaturePhase::Retry;
                        FlowFeatureStep::Yield
                    }
                    Err(fault) => FlowFeatureStep::Failed(flow_vcs_failure(fault)),
                }
            }
            FlowVcsFeaturePhase::Retry => {
                let sequence = self.page.expect("retained Flow VCS page").sequence;
                let handle = self.handle.expect("admitted Flow VCS handle");
                match self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.retry_page(handle, sequence)) {
                    Ok(page) => {
                        self.page = Some(page);
                        self.phase = FlowVcsFeaturePhase::AwaitAcknowledgement;
                        FlowFeatureStep::RetainedPage(self.encode_page(page))
                    }
                    Err(fault) => FlowFeatureStep::Failed(flow_vcs_failure(fault)),
                }
            }
            FlowVcsFeaturePhase::AwaitAcknowledgement => FlowFeatureStep::Yield,
            FlowVcsFeaturePhase::CloseOperation | FlowVcsFeaturePhase::CloseRetired => match self.close_cursor_step(budget) {
                Ok(RetainedCloneStep::Progress(_)) => FlowFeatureStep::Yield,
                Ok(RetainedCloneStep::Complete(_)) if self.terminal_failure.is_some() => FlowFeatureStep::Failed(self.terminal_failure.take().expect("terminal Flow VCS failure")),
                Ok(RetainedCloneStep::Complete(_)) => FlowFeatureStep::Complete(Vec::new()),
                Err(failure) => FlowFeatureStep::Failed(failure),
            },
            FlowVcsFeaturePhase::Complete if self.terminal_failure.is_some() => FlowFeatureStep::Failed(self.terminal_failure.take().expect("terminal Flow VCS failure")),
            FlowVcsFeaturePhase::Complete => FlowFeatureStep::Complete(Vec::new()),
        }
    }

    fn cancel(&mut self, budget: AbiWorkBudget) -> Result<(), FlowFailure> {
        if self.phase == FlowVcsFeaturePhase::Admit {
            self.phase = FlowVcsFeaturePhase::Complete;
            return Ok(());
        }
        let grant = self.grant(budget);
        let handle = self.handle.expect("admitted Flow VCS handle");
        self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.cancel(handle, grant)).map_err(flow_vcs_failure)?;
        self.phase = FlowVcsFeaturePhase::CloseOperation;
        Ok(())
    }

    fn preflight_acknowledge(&self, index: u32) -> Result<(), FlowFailure> {
        if index == 0 && self.phase == FlowVcsFeaturePhase::AwaitAcknowledgement && self.page.is_some() {
            Ok(())
        } else {
            Err(abi_failure(AbiErrorCode::OutOfOrderPage))
        }
    }

    fn acknowledge(&mut self, index: u32, _: AbiWorkBudget) -> Result<(), FlowFailure> {
        self.preflight_acknowledge(index)?;
        let sequence = self.page.expect("preflight retained Flow VCS page").sequence;
        let handle = self.handle.expect("admitted Flow VCS handle");
        self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.acknowledge_page(handle, sequence)).map_err(flow_vcs_failure)?;
        self.page = None;
        self.phase = FlowVcsFeaturePhase::Poll;
        Ok(())
    }

    fn close_step(&mut self, budget: AbiWorkBudget) -> Result<RetainedCloneStep, FlowFailure> {
        if self.phase == FlowVcsFeaturePhase::Admit {
            self.phase = FlowVcsFeaturePhase::Complete;
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if self.phase == FlowVcsFeaturePhase::Poll {
            let grant = self.grant(budget);
            let handle = self.handle.expect("admitted Flow VCS handle");
            match self.domain.borrow_mut().vcs.as_mut().ok_or(FlowVcsFault::Closed).and_then(|vcs| vcs.poll(handle, grant)).map_err(flow_vcs_failure)? {
                FlowVcsPoll::Terminal => {
                    self.phase = FlowVcsFeaturePhase::CloseOperation;
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                _ => return Err(abi_failure(AbiErrorCode::Busy)),
            }
        }
        self.close_cursor_step(budget)
    }
}

fn abi_code_failure(code: AbiErrorCode) -> FlowFailure {
    abi_failure(code)
}

pub fn flow_vcs_close_failure(failure:semio_framework_os_flow_vcs::FlowVcsCloseFailure)->FlowFailure{let code=flow_vcs_fault_code(failure.fault);match failure.cause{Some(error)=>FlowFailure::from_value(code,error),None=>flow_close_failure(code).with_retained_progress(failure.retained_progress)}}

fn flow_vcs_fault_code(fault:FlowVcsFault)->AbiErrorCode{match fault {
        FlowVcsFault::Closed => AbiErrorCode::Closed,
        FlowVcsFault::Full | FlowVcsFault::Limit | FlowVcsFault::Depth => AbiErrorCode::LimitExceeded,
        FlowVcsFault::WrongHandle | FlowVcsFault::StaleHandle => AbiErrorCode::UnknownHandle,
        FlowVcsFault::StaleAuthority => AbiErrorCode::StaleGeneration,
        FlowVcsFault::DuplicateControl => AbiErrorCode::DuplicateAcknowledgement,
        FlowVcsFault::InsufficientGrant => AbiErrorCode::NoCredit,
        FlowVcsFault::WrongPage => AbiErrorCode::OutOfOrderPage,
        _ => AbiErrorCode::MalformedTag,
    }
}
fn flow_vcs_failure(fault:FlowVcsFault)->FlowFailure{FlowFailure::new(flow_vcs_fault_code(fault),format!("Flow VCS {fault:?}"))}
