use super::*;
use semio_framework_job::{allocate_operation_id, Generation, RevisionId, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView};
use semio_framework_value::{ValueError,ValueRefusalKind};

fn fixture_step_grant()->semio_framework_job::RetainedCloneGrant{
    semio_framework_job::RetainedCloneGrant{maximum_items:8,maximum_copy_bytes:4096,maximum_capacity_bytes:8192,maximum_release_bytes:4096,maximum_depth:4}
}

fn fixture_close_grant(owner: &dyn InteractiveJob, body: usize) -> semio_framework_job::RetainedCloneGrant {
    semio_framework_job::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: body, maximum_capacity_bytes: owner.next_close_capacity_byte_demand(body).expect("original fixture capacity"), maximum_release_bytes: owner.next_close_release_byte_demand().expect("original fixture release"), maximum_depth: owner.next_close_depth_demand().expect("original fixture depth") }
}

struct ImmediateJob {
    output: Option<Vec<u8>>,
    writer: Option<semio_framework_job::RetainedJobPayloadWriter>,
    published: Option<semio_framework_job::RetainedJobPayload>,
    cursor: usize,
    closing: bool,
}
impl InteractiveJob for ImmediateJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,ValueError> {
        if self.published.is_some() { return JobOutcomeBorrow::admit_complete(cx,None,self.published.as_ref()); }
        let writer = self.writer.get_or_insert_with(|| semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput));
        if !writer.write_slice_page(cx, self.output.as_deref().unwrap_or_default(), &mut self.cursor).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture payload page admission refused"))? { return JobOutcomeBorrow::admit_yield(cx); }
        match self.writer.take().expect("immediate original output writer").finish(){Ok(payload)=>self.published=Some(payload),Err(writer)=>{self.writer=Some(writer);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original fixture payload writer has not completed"))}}
        JobOutcomeBorrow::admit_complete(cx,None,self.published.as_ref())
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>,ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Complete => descriptor.complete(None,self.published.as_ref()),
            _ => Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immediate fixture requires its original yielded or completed descriptor")),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(writer) = self.writer.as_mut() {
            writer.begin_close();
        }
    }

    fn close_step(&mut self, grant: semio_framework_job::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneProgress};
        self.begin_close();
        if let Some(writer) = self.writer.as_mut() {
            let step = match writer.close_step(grant) { Ok(step) => step, Err(error) => return InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() } };
            if writer.terminal_is_empty() { self.writer = None; }
            return if self.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress: step.progress() } } else { InteractiveJobCloseStep::Pending { progress: step.progress() } };
        }
        if let Some(output) = self.output.as_ref() {
            let bytes = output.capacity();
            if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes { return InteractiveJobCloseStep::Pending { progress: Default::default() }; }
            if bytes != 0 && grant.maximum_depth == 0 { return InteractiveJobCloseStep::Refused { kind: semio_framework_value::ValueRefusalKind::DepthLimit, progress: Default::default() }; }
            drop(self.output.take());
            let progress=RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() };
            return if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress}}else{InteractiveJobCloseStep::Pending{progress}};
        }
        if let Some(payload)=self.published.as_mut(){
            let step=match payload.close_step(grant){Ok(step)=>step,Err(error)=>return InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}};
            if payload.terminal_is_empty(){self.published=None;}
            return if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{InteractiveJobCloseStep::Pending{progress:step.progress()}};
        }
        InteractiveJobCloseStep::Complete { progress: Default::default() }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize,ValueError> { Ok(self.demands()?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self,_:usize) -> Result<usize,ValueError> { Ok(self.demands()?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize,ValueError> { Ok(self.demands()?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize,ValueError> { Ok(self.demands()?.depth) }
    fn terminal_is_empty(&self) -> bool { self.closing && self.output.is_none() && self.writer.is_none() && self.published.is_none() }

}

impl ImmediateJob {
    fn demands(&self)->Result<semio_framework_value::RetirementDemand,ValueError>{
        if let Some(writer)=self.writer.as_ref(){return writer.retirement_demands()}
        if let Some(output)=self.output.as_ref(){return Ok(semio_framework_value::RetirementDemand{release_bytes:output.capacity(),depth:usize::from(output.capacity()!=0),..Default::default()})}
        if let Some(payload)=self.published.as_ref(){return payload.retirement_demands()}
        Ok(Default::default())
    }
}
fn exact_close_grant(job:&impl InteractiveJob)->semio_framework_job::RetainedCloneGrant{
    let copy=job.next_close_copy_byte_demand().unwrap();let release=job.next_close_release_byte_demand().unwrap();
    semio_framework_job::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:job.next_close_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:job.next_close_depth_demand().unwrap()}
}
struct EchoFactory {
    keys: Vec<ToolFactoryKey>,
    classification: InteractiveJobClassification,
}

impl ToolJobFactory for EchoFactory {
    type Payload = String;
    type Job = ImmediateJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        "test.echo.v1"
    }

    fn classification(&self) -> InteractiveJobClassification {
        self.classification
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::bounded_first_step(64, 1, 1, 64, 100)
    }

    fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ImmediateJob { output: Some(format!("{payload}:ok").into_bytes()), writer: None, published: None, cursor: 0, closing: false })
    }
}

fn operation_spec(controller_id: &str, tool_id: &str) -> ToolOperationSpec {
    ToolOperationSpec::new(controller_id, tool_id, "test.echo.v1", tool_id.to_string(), Operation::new(allocate_operation_id(), RevisionId(7), Generation(3), 11))
}

fn echo_factory(controller_id: &str, tool_ids: &[&str], classification: InteractiveJobClassification) -> EchoFactory {
    EchoFactory { keys: tool_ids.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect(), classification }
}

#[test]
fn dispatch_returns_a_resumable_job_and_preserves_operation_identity() {
    let bus = ActionBus::new();
    bus.register(echo_factory("app", &["ping"], InteractiveJobClassification::Migrated)).unwrap();
    let dispatch = bus.dispatch(operation_spec("app", "ping")).unwrap();
    assert_eq!(dispatch.spec.operation.base_revision, RevisionId(7));
    assert_eq!(dispatch.spec.operation.generation, Generation(3));
}

#[test]
fn registration_rejects_every_non_migrated_factory() {
    for classification in [InteractiveJobClassification::Unclassified, InteractiveJobClassification::BatchOnlyPendingRewrite, InteractiveJobClassification::ForbiddenFromUi, InteractiveJobClassification::Deleted] {
        let bus = ActionBus::new();
        assert!(matches!(
            bus.register(echo_factory("app", &["ping"], classification)),
            Err(ToolRegistrationError::NonInteractiveClassification { classification: rejected, .. }) if rejected == classification
        ));
    }
}

#[test]
fn unknown_controller_is_an_explicit_dispatch_error() {
    let bus = ActionBus::new();
    assert!(matches!(bus.dispatch(operation_spec("missing", "ping")), Err(ToolDispatchError::UnknownController { .. })));
}

#[test]
fn aliases_require_an_existing_exact_factory_and_never_fallback() {
    let bus = ActionBus::new();
    let alias = ToolFactoryKey::new("app", "alias");
    let target = ToolFactoryKey::new("app", "exact");
    assert!(matches!(bus.register_alias(alias.clone(), target.clone()), Err(ToolRegistrationError::UnknownAliasTarget { .. })));
    bus.register(echo_factory("app", &["exact"], InteractiveJobClassification::Migrated)).unwrap();
    bus.register_alias(alias, target).unwrap();
    assert!(matches!(bus.dispatch(operation_spec("app", "alias")), Err(ToolDispatchError::UnknownController { .. })));
    assert!(matches!(bus.dispatch(operation_spec("app", "missing")), Err(ToolDispatchError::UnknownController { .. })));
}

#[test]
fn exact_wire_admission_rejects_alias_schema_and_raw_limit_before_decode() {
    let bus = ActionBus::new();
    let exact = ToolFactoryKey::new("app", "exact");
    let alias = ToolFactoryKey::new("app", "alias");
    bus.register(echo_factory("app", &["exact"], InteractiveJobClassification::Migrated)).unwrap();
    bus.register_alias(alias, exact.clone()).unwrap();
    assert!(matches!(bus.admit_exact_wire("app", "alias", "test.echo.v1", b"ok"), Err(ToolDispatchError::UnknownController { .. })));
    let operation = Operation::new(allocate_operation_id(), RevisionId(0), Generation(0), 1);
    assert!(matches!(bus.dispatch_wire("app", "alias", "test.echo.v1", b"ok", None, operation), Err(ToolDispatchError::UnknownController { .. })));
    assert!(matches!(bus.admit_exact_wire("app", "exact", "wrong.schema", b"ok"), Err(ToolDispatchError::Factory { .. })));
    assert!(matches!(bus.admit_exact_wire("app", "exact", "test.echo.v1", &[0; 65]), Err(ToolDispatchError::RawWireLimit { actual: 65, maximum: 64, .. })));
    let admission = bus.admit_exact_wire("app", "exact", "test.echo.v1", b"ok").unwrap();
    assert_eq!(admission.key, exact);
    assert_eq!(admission.factory_type_id, TypeId::of::<EchoFactory>());
    assert_eq!(admission.contract.max_raw_wire_bytes, 64);
}

struct NumberFactory {
    keys: Vec<ToolFactoryKey>,
}

impl ToolJobFactory for NumberFactory {
    type Payload = u64;
    type Job = ImmediateJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        "test.number.v1"
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::bounded_first_step(8, 1, 1, 8, 100)
    }

    fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ImmediateJob { output: Some(payload.to_le_bytes().to_vec()), writer: None, published: None, cursor: 0, closing: false })
    }

    fn create_job_from_wire(&mut self, _operation: Operation, payload: &[u8], checkpoint: Option<Vec<u8>>) -> Result<Self::Job, ToolJobFactoryError> {
        let value = u64::from_le_bytes(payload.try_into().map_err(|_| ToolJobFactoryError::new("number wire payload must contain exactly eight bytes"))?);
        let mut output = value.to_le_bytes().to_vec();
        output.extend(checkpoint.unwrap_or_default());
        Ok(ImmediateJob { output: Some(output), writer: None, published: None, cursor: 0, closing: false })
    }
}

#[test]
fn heterogeneous_factories_share_one_bus_with_exact_key_ownership() {
    let bus = ActionBus::new();
    bus.register(echo_factory("text", &["one", "two"], InteractiveJobClassification::Migrated)).unwrap();
    bus.register(NumberFactory { keys: vec![ToolFactoryKey::new("number", "encode")] }).unwrap();
    assert_eq!(bus.keys().len(), 3);
    assert!(bus.contains(&ToolFactoryKey::new("text", "two")));
    let spec = ToolOperationSpec::new("number", "encode", "test.number.v1", 42u64, Operation::new(allocate_operation_id(), RevisionId(0), Generation(0), 9));
    assert!(bus.dispatch(spec).is_ok());
}

#[test]
fn duplicate_factory_key_is_rejected_without_partial_registration() {
    let bus = ActionBus::new();
    bus.register(echo_factory("app", &["one"], InteractiveJobClassification::Migrated)).unwrap();
    let result = bus.register(echo_factory("app", &["two", "one"], InteractiveJobClassification::Migrated));
    assert!(matches!(result, Err(ToolRegistrationError::DuplicateKey { key }) if key == ToolFactoryKey::new("app", "one")));
    assert_eq!(bus.keys().len(), 1);
}

#[test]
fn duplicate_key_inside_one_factory_is_rejected_atomically() {
    let bus = ActionBus::new();
    let result = bus.register(echo_factory("app", &["same", "same"], InteractiveJobClassification::Migrated));
    assert!(matches!(result, Err(ToolRegistrationError::DuplicateKey { key }) if key == ToolFactoryKey::new("app", "same")));
    assert_eq!(bus.keys().len(), 0);
}

#[test]
fn wire_dispatch_uses_the_factory_decoder_and_preserves_the_restart_checkpoint() {
    let bus = ActionBus::new();
    bus.register(NumberFactory { keys: vec![ToolFactoryKey::new("number", "decode-wire")] }).unwrap();
    let operation = Operation::new(allocate_operation_id(), RevisionId(19), Generation(5), 13);
    let mut dispatch = bus.dispatch_wire("number", "decode-wire", "test.number.v1", &42u64.to_le_bytes(), Some(vec![7, 8]), operation).expect("wire dispatch");
    let mut sequence = 0;
    let mut retained_progress=semio_framework_job::RetainedCloneProgress::default();
    let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX,fixture_step_grant()), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence,&mut retained_progress);
    let mut expected = 42u64.to_le_bytes().to_vec();
    expected.extend([7, 8]);
    let descriptor=dispatch.job.step(&mut context).unwrap().expect("original complete admission").into_descriptor();
    {let JobOutcomeView::Complete{output:Some(output),..}=dispatch.job.borrow_outcome(&descriptor).unwrap()else{panic!("original wire output remains inside job")};assert_eq!(output.page(0),Some(expected.as_slice()));assert_eq!(output.page_count(),1);}
    dispatch.job.begin_close();
    for _ in 0..16 {
        if dispatch.job.terminal_is_empty() { break; }
        let grant = fixture_close_grant(&dispatch.job, 4096);
        assert!(dispatch.job.close_step(grant).progress().fits(grant));
    }
    assert!(dispatch.job.terminal_is_empty());
    assert!(matches!(bus.dispatch_wire("number", "decode-wire", "wrong.schema", &42u64.to_le_bytes(), None, operation), Err(ToolDispatchError::Factory { .. })));
    assert!(matches!(bus.dispatch_wire("number", "decode-wire", "test.number.v1", &[0; 9], None, operation), Err(ToolDispatchError::RawWireLimit { actual: 9, maximum: 8, .. })));
}

#[test]
fn completed_job_retains_original_input_until_exact_granted_close() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧹️wire-retirement/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let law = &fixture["completedInput"];
    let mut original = Vec::with_capacity(law["capacityBytes"].as_u64().unwrap() as usize);
    original.extend([4, 5, 6]);
    let pointer = original.as_ptr();
    let capacity = original.capacity();
    let mut job = ImmediateJob { output: Some(original), writer: None, published: None, cursor: 0, closing: false };
    let mut sequence = 0;
    let operation = allocate_operation_id();
    let mut retained_progress=semio_framework_job::RetainedCloneProgress::default();
    let mut context = StepContext::new(operation, Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX,fixture_step_grant()), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence,&mut retained_progress);
    let descriptor=job.step(&mut context).unwrap().expect("original complete admission").into_descriptor();
    assert!(matches!(job.borrow_outcome(&descriptor).unwrap(),JobOutcomeView::Complete{output:Some(_),..}));
    let retained = job.output.as_ref().expect("completion retains the original input allocation");
    assert_eq!(retained.as_ptr(), pointer);
    assert_eq!(retained.capacity(), capacity);
    assert_eq!(retained.as_slice(), [4, 5, 6]);
    job.begin_close();
    let grant = fixture_close_grant(&job, 4096);
    let denied = semio_framework_job::RetainedCloneGrant { maximum_release_bytes: capacity - 1, ..grant };
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| job.close_step(denied));
    assert_eq!(step.progress(), Default::default());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!(job.output.as_ref().unwrap().as_ptr(), pointer);
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| job.close_step(grant));
    assert_eq!(step.progress().released_bytes, law["releasedAfterClose"].as_u64().unwrap() as usize);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, capacity));
    assert!(step.progress().fits(grant));
    assert!(!job.terminal_is_empty());
    for _ in 0..16 {
        if job.terminal_is_empty(){break}
        let grant=fixture_close_grant(&job,4096);
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(grant));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        assert!(step.progress().fits(grant));
    }
    assert!(job.terminal_is_empty());
    println!("[DEBUG] ActionBus completed input retains original pointer/capacity64, one-below inert, exact close releases64 with full receipt");
}

struct RetainedNumberJob {
    checkpoint: semio_framework_job::RetainedJobPayload,
    input: Option<RetainedToolWireInput>,
    bytes: [u8; 8],
    cursor: usize,
    output: Option<ImmediateJob>,
    closing: bool,
}

impl InteractiveJob for RetainedNumberJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,ValueError> {
        if cx.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(cx);
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return JobOutcomeBorrow::admit_yield(cx);
        }
        if self.cursor < self.bytes.len() {
            self.bytes[self.cursor] = self.input.as_ref().and_then(|input| input.page(0)).and_then(|page| page.get(self.cursor)).copied().unwrap_or_default();
            self.cursor += 1;
            cx.consume_fuel(1);
            return JobOutcomeBorrow::admit_checkpoint(cx,&self.checkpoint,self.cursor as u64);
        }
        self.output.get_or_insert_with(|| ImmediateJob { output: Some(self.bytes.to_vec()), writer: None, published: None, cursor: 0, closing: false }).step(cx)
    }

    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{
        match descriptor.kind(){
            JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),
            JobOutcomeKind::CheckpointReady{..}=>descriptor.checkpoint(&self.checkpoint),
            JobOutcomeKind::Complete=>self.output.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained fixture original completed output absent"))?.borrow_outcome(descriptor),
            _=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained fixture original descriptor kind mismatch")),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(input) = self.input.as_mut() {
            input.begin_close();
        }
        if let Some(output) = self.output.as_mut() {
            output.begin_close();
        }
    }

    fn close_step(&mut self, grant: semio_framework_job::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        self.begin_close();
        if grant.maximum_items == 0 { return InteractiveJobCloseStep::Pending { progress: Default::default() }; }
        if !self.terminal_is_empty() && grant.maximum_depth == 0 { return InteractiveJobCloseStep::Refused { kind: semio_framework_value::ValueRefusalKind::DepthLimit, progress: Default::default() }; }
        let child = semio_framework_job::RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        let step = if let Some(input) = self.input.as_mut() {
            let step = input.close_step(child);
            if input.terminal_is_empty() { self.input = None; }
            step
        } else if let Some(output) = self.output.as_mut() {
            let step = output.close_step(child);
            if output.terminal_is_empty() { self.output = None; }
            step
        } else { return InteractiveJobCloseStep::Complete { progress: Default::default() }; };
        match step {
            InteractiveJobCloseStep::Pending { progress } | InteractiveJobCloseStep::Complete { progress } => if self.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress } } else { InteractiveJobCloseStep::Pending { progress } },
            other => other,
        }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.input.as_ref().map_or_else(|| self.output.as_ref().map_or(Ok(0), |owner| owner.next_close_copy_byte_demand()), |owner| owner.next_close_copy_byte_demand()) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { self.input.as_ref().map_or_else(|| self.output.as_ref().map_or(Ok(0), |owner| owner.next_close_capacity_byte_demand(body)), |owner| owner.next_close_capacity_byte_demand(body)) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.input.as_ref().map_or_else(|| self.output.as_ref().map_or(Ok(0), |owner| owner.next_close_release_byte_demand()), |owner| owner.next_close_release_byte_demand()) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { let depth = self.input.as_ref().map_or_else(|| self.output.as_ref().map_or(Ok(0), |owner| owner.next_close_depth_demand()), |owner| owner.next_close_depth_demand())?; Ok(if self.input.is_some() || self.output.is_some() { depth + 1 } else { 0 }) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.input.is_none() && self.output.is_none()
    }
}

struct RetainedNumberFactory {
    keys: Vec<ToolFactoryKey>,
}

impl ToolJobFactory for RetainedNumberFactory {
    type Payload = RetainedNumberJob;
    type Job = RetainedNumberJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        "test.retained-number.v1"
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(8, 1, 1, 8, 100, 1, 1)
    }

    fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(payload)
    }

    fn create_job_from_wire_pages(&mut self, _operation: Operation, input: RetainedToolWireInput, checkpoint: Option<RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
        if input.declared_bytes() != 8 || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("retained number requires eight bytes and no checkpoint"), input, checkpoint));
        }
        Ok(RetainedNumberJob { checkpoint: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), input: Some(input), bytes: [0; 8], cursor: 0, output: None, closing: false })
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: Operation,
        mut payload: Self::Payload,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
        if input.declared_bytes() != 8 || checkpoint.is_some() || payload.input.is_some() {
            return Err((ToolJobFactoryError::new("retained number payload requires one eight-byte raw owner and no checkpoint"), input, checkpoint));
        }
        payload.input = Some(input);
        Ok(payload)
    }
}

#[test]
fn retained_wire_pages_are_admitted_sealed_transferred_and_closed_by_logical_bytes() {
    let bus = ActionBus::new();
    bus.register(RetainedNumberFactory { keys: vec![ToolFactoryKey::new("number", "retained")] }).unwrap();
    let (admission, mut input) = bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 8).unwrap();
    input.admit_page(ToolWirePage::try_copy_from(&42u64.to_le_bytes()).unwrap()).unwrap();
    assert!(input.seal().is_ok());
    let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(2), 3);
    let mut dispatch = match bus.dispatch_wire_retained(admission, input, None, operation) {
        Ok(dispatch) => dispatch,
        Err(_) => panic!("retained dispatch was rejected"),
    };
    let mut sequence = 0;
    let mut retained_progress=semio_framework_job::RetainedCloneProgress::default();
    for _ in 0..8 {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX,fixture_step_grant()), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence,&mut retained_progress);
        assert!(matches!(dispatch.job.step(&mut context), Ok(Some(JobOutcomeBorrow::CheckpointReady{..}))));
    }
    dispatch.job.begin_close();
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧹️wire-retirement/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let law = &fixture["shortClose"];
    assert_eq!(law["logicalBytes"].as_u64(), Some(8));
    let grant = fixture_close_grant(&dispatch.job, 8);
    assert!(grant.maximum_release_bytes >= 8);
    for denied in fixture["denied"].as_array().unwrap() {
        let denied = match denied.as_str().unwrap() {
            "items" => semio_framework_job::RetainedCloneGrant { maximum_items: 0, ..grant },
            "release" => semio_framework_job::RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes - 1, ..grant },
            "depth" => semio_framework_job::RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant },
            _ => unreachable!(),
        };
        let step = dispatch.job.close_step(denied);
        assert_eq!(step.progress(), Default::default());
        assert!(!dispatch.job.terminal_is_empty());
    }
    let step = dispatch.job.close_step(grant);
    assert!(matches!(step, semio_framework_job::InteractiveJobCloseStep::Complete { .. }));
    assert_eq!(step.progress().copied_items, law["copiedItems"].as_u64().unwrap() as usize);
    assert_eq!(step.progress().copied_bytes, law["copiedBytes"].as_u64().unwrap() as usize);
    assert_eq!(step.progress().retained_capacity_bytes, law["retainedCapacityBytes"].as_u64().unwrap() as usize);
    assert_eq!(step.progress().released_bytes, grant.maximum_release_bytes);
    assert!(step.progress().fits(grant));
    println!("[DEBUG] ActionBus original short wire logical8 physical={} body8 denieditems/release/depth0 terminalComplete", grant.maximum_release_bytes);
    assert!(dispatch.job.terminal_is_empty());
}
#[test]
fn production_typed_payload_and_retained_pages_enter_the_same_registered_factory_job() {
    let bus = ActionBus::new();
    bus.register(RetainedNumberFactory { keys: vec![ToolFactoryKey::new("number", "retained")] }).unwrap();
    let (admission, mut input) = bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 8).unwrap();
    input.admit_page(ToolWirePage::try_copy_from(&42u64.to_le_bytes()).unwrap()).unwrap();
    input.seal().unwrap();
    let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(2), 3);
    let payload = RetainedNumberJob { checkpoint: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), input: None, bytes: [0; 8], cursor: 0, output: None, closing: false };
    let spec = ToolOperationSpec::new("number", "retained", "test.retained-number.v1", payload, operation);
    let mut dispatch = match bus.dispatch_wire_retained_with_spec(&admission, input, None, spec) {
        Ok(dispatch) => dispatch,
        Err(_) => panic!("production retained payload dispatch was rejected"),
    };
    let mut sequence = 0;
    let mut retained_progress=semio_framework_job::RetainedCloneProgress::default();
    for _ in 0..8 {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX,fixture_step_grant()), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence,&mut retained_progress);
        assert!(matches!(dispatch.job.step(&mut context), Ok(Some(JobOutcomeBorrow::CheckpointReady{..}))));
    }
    dispatch.job.begin_close();
    for _ in 0..16 {
        if dispatch.job.terminal_is_empty() { break; }
        let grant = fixture_close_grant(&dispatch.job, 8);
        assert!(dispatch.job.close_step(grant).progress().fits(grant));
    }
    assert!(dispatch.job.terminal_is_empty());
}

#[test]
fn retained_wire_admission_rejects_plus_one_and_returns_the_page_owner_on_saturation() {
    let bus = ActionBus::new();
    bus.register(RetainedNumberFactory { keys: vec![ToolFactoryKey::new("number", "retained")] }).unwrap();
    assert!(matches!(bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 9), Err(ToolDispatchError::RawWireLimit { actual: 9, maximum: 8, .. })));
    let (_, mut input) = bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 8).unwrap();
    input.admit_page(ToolWirePage::try_copy_from(&[0; 8]).unwrap()).unwrap();
    let rejected = input.admit_page(ToolWirePage::try_copy_from(&[1]).unwrap()).expect_err("plus-one page owner must be returned");
    assert_eq!(rejected.1.as_slice(), &[1]);
}

#[test]
fn maximum_extent_owner_exists_before_incremental_encoding_and_seals_to_its_exact_prefix() {
    let bus = ActionBus::new();
    bus.register(RetainedNumberFactory { keys: vec![ToolFactoryKey::new("number", "retained")] }).unwrap();
    let (admission, mut input) = bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 8).unwrap();
    input.admit_page(ToolWirePage::try_copy_from(&42u32.to_le_bytes()).unwrap()).unwrap();
    input.seal_admitted_prefix().unwrap();
    assert_eq!(input.declared_bytes(), 4);
    assert_eq!(input.page(0), Some(42u32.to_le_bytes().as_slice()));
    let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(2), 3);
    let rejected = match bus.dispatch_wire_retained(admission, input, None, operation) {
        Ok(_) => panic!("factory owns the exact eight-byte decoder and must reject a truthful four-byte prefix"),
        Err(rejected) => rejected,
    };
    assert_eq!(rejected.input.declared_bytes(), 4);
}
