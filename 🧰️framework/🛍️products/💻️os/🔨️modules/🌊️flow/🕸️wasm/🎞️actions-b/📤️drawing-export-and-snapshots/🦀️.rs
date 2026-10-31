//! 📤️ Drawing, export and snapshot actions 2599 to 2610: the scene render, SVG and PDF export, the bitmap trace, boolean segments, the geometry port disposal, the pointer cancel and the host snapshot JSON. See [`super`].

use super::*;

pub struct FlowAction2599 {
    pub program: FlowProgramState,
    pub receipt:RetainedCloneProgress,
    pub work_units:usize,
    pub identity:Option<semio_framework_os_flow_geometry::GeometryWorkIdentity>,
    pub encoding:Option<semio_framework_pack_json::ArtifactCanonicalJsonTreeWireCursor<semio_framework::MeshData>>,
    pub output:Option<ControlledRetirement<Vec<u8>>>,
    pub complete:bool,
}

impl FlowAction2599 {
    fn record_close_work(&mut self,receipt:RetainedCloneProgress,cpu:usize)->Result<(),semio_framework_value::ValueError>{let retained=self.receipt.checked_add(receipt)?;let work=self.work_units.checked_add(cpu).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original Flow geometry close CPU overflow"))?;self.receipt=retained;self.work_units=work;Ok(())}

    fn record_original_work(&mut self,work:semio_framework_os_flow_geometry::GeometryWorkProgress)->Result<(),semio_framework_value::ValueError>{
        self.work_units=self.work_units.checked_add(work.consumed_units).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original Flow geometry cumulative CPU overflow"))?;
        self.receipt=self.receipt.checked_add(work.retained)?;Ok(())
    }
    fn advance_original(&mut self,domain:&mut FlowDomainAdapter,args:&FlowArguments,cx:&mut semio_framework_job::StepContext<'_>)->Result<FlowFeatureStep,FlowFailure>{
        if cx.should_yield(){return Ok(FlowFeatureStep::Yield)}
        let identity=semio_framework_os_flow_geometry::GeometryWorkIdentity{operation:cx.operation().0,generation:cx.generation().0};
        if let Some(original)=self.identity{if original!=identity{return Err(FlowFailure::from_value(AbiErrorCode::StaleGeneration,semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original Flow tessellation caller changed its business identity")))}}else{
            let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(FlowFeatureStep::Yield)}
            let receipt=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_retained(receipt).map_err(|error|FlowFailure::from_value(AbiErrorCode::NoCredit,error))?;cx.consume_fuel(1);self.receipt=self.receipt.checked_add(receipt).map_err(|error|FlowFailure::from_value(AbiErrorCode::LimitExceeded,error))?;self.work_units+=1;self.identity=Some(identity);return Ok(FlowFeatureStep::Yield)
        }
        if self.complete{return Ok(FlowFeatureStep::Yield)}
        let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(FlowFeatureStep::Yield)}
        if let Some(encoding)=self.encoding.as_mut(){
            let original=encoding.advance(grant);let cpu=encoding.last_turn_work()as usize;cx.consume_fuel(cpu as u64);self.work_units=self.work_units.checked_add(cpu).ok_or_else(||abi_failure(AbiErrorCode::LimitExceeded))?;
            let step=match original{Ok(step)=>step,Err(error)=>{let work=semio_framework_os_flow_geometry::GeometryWorkProgress{retained:error.retained_progress(),consumed_units:0};let mut failure=semio_framework_os_flow_geometry::GeometryWorkFailure::new(error,cpu);failure.receipt_refusal=cx.consume_retained(work.retained).err();let _=self.record_original_work(work);return Err(FlowFailure::from_geometry(AbiErrorCode::Busy,failure))}};
            if let Some(output)=step.output{self.output=Some(ControlledRetirement::new(output).unwrap_or_else(|_|unreachable!("original Vec declares controlled retirement")))}
            let receipt=step.progress;self.receipt=self.receipt.checked_add(receipt).map_err(|error|FlowFailure::from_value(AbiErrorCode::LimitExceeded,error))?;cx.consume_retained(receipt).map_err(|error|FlowFailure::from_value(AbiErrorCode::NoCredit,error))?;
            if let Some(output)=self.output.as_mut().and_then(ControlledRetirement::take_original){self.output=None;self.complete=true;return Ok(FlowFeatureStep::Complete(output))}
            return Ok(FlowFeatureStep::Progress{completed:encoding.output_bytes_written()as u64,total:encoding.measured_byte_length()as u64})
        }
        let port=domain.host.geometry_port().map_err(domain_error)?;let(handle,tolerance)=(text(args,"handle")?,number(args,"tolerance")?);
        let demand=port.tessellation_demands(identity,handle,tolerance,grant.maximum_copy_bytes).map_err(|error|FlowFailure::from_value(AbiErrorCode::NoCredit,error))?;
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(FlowFeatureStep::Yield)}
        let(step,work)=match port.tessellate_step(identity,handle,tolerance,cx){Ok(original)=>original,Err(original)=>{let _=self.record_original_work(original.work);return Err(FlowFailure::from_geometry(AbiErrorCode::Busy,original))}};
        let step=match step{semio_framework_os_flow_geometry::GeometryStep::Ready(mesh)=>{self.encoding=Some(semio_framework_pack_json::ArtifactCanonicalJsonTreeWireCursor::new(mesh));None},step=>Some(step)};
        if let Err(error)=self.record_original_work(work){return Err(FlowFailure::from_geometry(AbiErrorCode::NoCredit,semio_framework_os_flow_geometry::GeometryWorkFailure{primary:error,work,receipt_refusal:None}))}
        Ok(match step{
            None=>FlowFeatureStep::Yield,
            Some(semio_framework_os_flow_geometry::GeometryStep::Working{units_done,units_total,..})=>FlowFeatureStep::Progress{completed:units_done as u64,total:units_total as u64},
            Some(semio_framework_os_flow_geometry::GeometryStep::Cancelled)=>FlowFeatureStep::Failed(abi_failure(AbiErrorCode::Cancelled)),
            Some(semio_framework_os_flow_geometry::GeometryStep::Failed(error))=>FlowFeatureStep::Failed(FlowFailure::new(AbiErrorCode::Busy,error)),
            Some(semio_framework_os_flow_geometry::GeometryStep::Ready(_))=>unreachable!("original Ready ownership entered paid encoding"),
        })
    }
}

impl FlowActionState for FlowAction2599 {
    fn retained_receipt(&self)->Option<RetainedCloneProgress>{Some(self.receipt)}
    fn work_receipt(&self)->Option<usize>{Some(self.work_units)}
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_599
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        let _=(domain,args,&self.program);FlowFeatureStep::Failed(FlowFailure::from_value(AbiErrorCode::NoCredit,semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"Flow tessellation requires its original native caller context")))
    }
    fn advance_on_original_caller(&mut self,domain:&mut FlowDomainAdapter,args:&FlowArguments,cx:&mut semio_framework_job::StepContext<'_>)->FlowFeatureStep{match self.advance_original(domain,args,cx){Ok(step)=>step,Err(failure)=>FlowFeatureStep::Failed(failure)}}
    fn close_step(&mut self,_:&mut FlowArguments,budget:AbiWorkBudget)->Result<bool,FlowFailure>{
        if self.output.as_ref().is_some_and(ControlledRetirement::terminal_is_empty){if budget.retained.maximum_items==0||budget.retained.maximum_depth==0{return Ok(false);}let receipt=RetainedCloneProgress{copied_items:1,..Default::default()};self.record_close_work(receipt,1).map_err(|error|FlowFailure::from_value(AbiErrorCode::NoCredit,error))?;self.output=None;return Ok(false);}
        if let Some(output)=self.output.as_mut(){let original=output.step(budget.retained);let cpu=output.last_turn_work()as usize;let receipt=match &original{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};let recorded=self.record_close_work(receipt,cpu);return match original{Err(error)=>Err(FlowFailure::from_value(AbiErrorCode::NoCredit,error).with_retained_progress(receipt).with_receipt_refusal(recorded.err())),Ok(_)=>{recorded.map_err(|error|FlowFailure::from_value(AbiErrorCode::NoCredit,error).with_retained_progress(receipt))?;Ok(false)}};}
        if self.encoding.as_ref().is_some_and(|encoding|encoding.terminal_is_empty()){if budget.retained.maximum_items==0||budget.retained.maximum_depth==0{return Ok(false);}let receipt=RetainedCloneProgress{copied_items:1,..Default::default()};self.record_close_work(receipt,1).map_err(|error|FlowFailure::from_value(AbiErrorCode::NoCredit,error))?;self.encoding=None;return Ok(false);}
        if let Some(encoding)=self.encoding.as_mut(){let original=encoding.close_step(budget.retained);let cpu=encoding.last_turn_work()as usize;let receipt=match &original{Ok(receipt)=>*receipt,Err(error)=>error.retained_progress()};let recorded=self.record_close_work(receipt,cpu);return match original{Err(error)=>Err(FlowFailure::from_value(AbiErrorCode::NoCredit,error).with_retained_progress(receipt).with_receipt_refusal(recorded.err())),Ok(_)=>{recorded.map_err(|error|FlowFailure::from_value(AbiErrorCode::NoCredit,error).with_retained_progress(receipt))?;Ok(false)}};}Ok(true)
    }
}

pub struct FlowAction2600 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2600 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_600
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_600),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(render_scene_json(text(args, "handle")?).into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_600),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2601 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2601 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_601
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_601),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(export_svg_json(text(args, "handle")?).into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_601),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2602 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2602 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_602
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_602),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(export_pdf_json(text(args, "handle")?).into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_602),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

#[path="🔍️trace/🦀️.rs"]mod original_trace;
use original_trace::FlowTraceCursor;
pub struct FlowAction2605{pub program:FlowProgramState,trace:FlowTraceCursor}
impl FlowAction2605{pub fn new(args:&FlowArguments)->Self{Self{program:FlowProgramState::new(args),trace:FlowTraceCursor::new()}}}
impl FlowActionState for FlowAction2605{
 fn retained_receipt(&self)->Option<RetainedCloneProgress>{Some(self.trace.retained_receipt())}
 fn work_receipt(&self)->Option<usize>{Some(self.trace.work_receipt())}
 #[cfg(test)]fn operation(&self)->u16{2605}
 fn advance(&mut self,_:&mut FlowDomainAdapter,_:&FlowArguments,budget:AbiWorkBudget)->FlowFeatureStep{if budget.cancelled||budget.interrupted||budget.deadline_ms.is_some_and(|deadline|budget.now_ms>=deadline)||budget.byte_credit==0{return FlowFeatureStep::Failed(original_trace::abi_refusal(if budget.cancelled{AbiErrorCode::Cancelled}else if budget.interrupted{AbiErrorCode::Interrupted}else if budget.deadline_ms.is_some_and(|deadline|budget.now_ms>=deadline){AbiErrorCode::DeadlineExceeded}else{AbiErrorCode::NoCredit}));}FlowFeatureStep::Failed(FlowFailure::from_value(AbiErrorCode::NoCredit,semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"Flow trace requires its original native caller context")))}
 fn advance_on_original_caller(&mut self,_:&mut FlowDomainAdapter,args:&FlowArguments,cx:&mut semio_framework_job::StepContext<'_>)->FlowFeatureStep{match self.trace.advance_original(args,cx){Ok(step)=>step,Err(error)=>FlowFeatureStep::Failed(error)}}
 fn close_step(&mut self,_:&mut FlowArguments,budget:AbiWorkBudget)->Result<bool,FlowFailure>{self.trace.close_step(budget.retained).map_err(|error|FlowFailure::from_value(AbiErrorCode::NoCredit,error))}
}

pub struct FlowAction2606 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2606 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_606
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_606),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(boolean_segments_json(text(args, "aJson")?, text(args, "bJson")?, text(args, "operation")?).into_bytes()) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_606),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2607 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2607 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_607
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_607),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.geometry_port().map_err(domain_error)?.dispose(text(args, "handle")?).map_err(domain_error)?;
                        dispose_drawing(text(args, "handle")?);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_607),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2608 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2608 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_608
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_608),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        domain.host.pointer_cancel_screen();
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_608),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2609 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2609 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_609
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_609),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.host_snapshot_json().map(String::into_bytes).map_err(domain_error) };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_609),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}

pub struct FlowAction2610 {
    pub program: FlowProgramState,
}

impl FlowActionState for FlowAction2610 {
    #[cfg(test)]
    fn operation(&self) -> u16 {
        2_610
    }

    fn advance(&mut self, domain: &mut FlowDomainAdapter, args: &FlowArguments, budget: AbiWorkBudget) -> FlowFeatureStep {
        if budget.cancelled || budget.interrupted || budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) || budget.byte_credit == 0 {
            return FlowFeatureStep::Failed(abi_failure(if budget.cancelled {
                AbiErrorCode::Cancelled
            } else if budget.interrupted {
                AbiErrorCode::Interrupted
            } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
                AbiErrorCode::DeadlineExceeded
            } else {
                AbiErrorCode::NoCredit
            }));
        }
        match self.program.phase {
            FlowProgramPhase::Decode => self.program.decode_step(args),
            FlowProgramPhase::Validate => self.program.validate_step(args),
            FlowProgramPhase::Checkpoint => self.program.checkpoint_step(2_610),
            FlowProgramPhase::Domain if self.program.domain_cursor == 0 => self.program.domain_ready_step(),
            FlowProgramPhase::Domain => {
                let result: Result<Vec<u8>, FlowFailure> = flow_result! {
                    {
                        let fixture = FlowHost::parse_host_snapshot_json(text(args, "json")?).map_err(domain_error)?;
                        domain.host.resync_host_snapshot_from_scene(fixture);
                        ok()
                    }
                };
                self.program.finish_domain(result)
            }
            FlowProgramPhase::Encode => self.program.encode_step(),
            FlowProgramPhase::Publish => self.program.publish_step(domain, 2_610),
            FlowProgramPhase::Complete => self.program.complete_step(),
            FlowProgramPhase::Sealed => FlowFeatureStep::Yield,
        }
    }
}
