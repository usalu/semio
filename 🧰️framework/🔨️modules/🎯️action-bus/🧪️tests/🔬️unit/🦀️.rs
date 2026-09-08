
use super::*;
use semio_framework_job::{CommitCandidate, Generation, RevisionId, allocate_operation_id};

struct ImmediateJob {
    output: Option<Vec<u8>>,
    writer: Option<semio_framework_job::RetainedJobPayloadWriter>,
    cursor: usize,
    closing: bool,
}

impl InteractiveJob for ImmediateJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        let writer = self.writer.get_or_insert_with(|| semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput));
        if !writer.write_slice_page(cx, self.output.as_deref().unwrap_or_default(), &mut self.cursor).unwrap_or(false) {
            return StepOutcome::Yield;
        }
        self.output = None;
        let output = self.writer.take().expect("immediate output writer").finish().unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput));
        StepOutcome::Complete(CommitCandidate { state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState), output })
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(writer) = self.writer.as_mut() {
            writer.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        if let Some(writer) = self.writer.as_mut() {
            return match writer.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    self.writer = None;
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            };
        }
        if self.output.is_some() {
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.output = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.writer.is_none()
    }
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
        Ok(ImmediateJob { output: Some(format!("{payload}:ok").into_bytes()), writer: None, cursor: 0, closing: false })
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
    bus.register_alias(alias.clone(), exact.clone()).unwrap();
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
        Ok(ImmediateJob { output: Some(payload.to_le_bytes().to_vec()), writer: None, cursor: 0, closing: false })
    }

    fn create_job_from_wire(&mut self, _operation: Operation, payload: &[u8], checkpoint: Option<Vec<u8>>) -> Result<Self::Job, ToolJobFactoryError> {
        let value = u64::from_le_bytes(payload.try_into().map_err(|_| ToolJobFactoryError::new("number wire payload must contain exactly eight bytes"))?);
        let mut output = value.to_le_bytes().to_vec();
        output.extend(checkpoint.unwrap_or_default());
        Ok(ImmediateJob { output: Some(output), writer: None, cursor: 0, closing: false })
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
    let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    let mut expected = 42u64.to_le_bytes().to_vec();
    expected.extend([7, 8]);
    let StepOutcome::Complete(mut candidate) = dispatch.job.step(&mut context) else { panic!("wire job did not complete") };
    assert_eq!(candidate.output.page(0), Some(expected.as_slice()));
    assert_eq!(candidate.output.page_count(), 1);
    assert_eq!(candidate.output.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Pending { released_items: 1, released_bytes: expected.len() },);
    assert_eq!(candidate.output.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete);
    dispatch.job.begin_close();
    while !dispatch.job.terminal_is_empty() {
        let _ = dispatch.job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    assert!(matches!(bus.dispatch_wire("number", "decode-wire", "wrong.schema", &42u64.to_le_bytes(), None, operation), Err(ToolDispatchError::Factory { .. })));
    assert!(matches!(bus.dispatch_wire("number", "decode-wire", "test.number.v1", &[0; 9], None, operation), Err(ToolDispatchError::RawWireLimit { actual: 9, maximum: 8, .. })));
}

struct RetainedNumberJob {
    input: Option<RetainedToolWireInput>,
    bytes: [u8; 8],
    cursor: usize,
    output: Option<ImmediateJob>,
    closing: bool,
}

impl InteractiveJob for RetainedNumberJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return StepOutcome::Yield;
        }
        if self.cursor < self.bytes.len() {
            self.bytes[self.cursor] = self.input.as_ref().and_then(|input| input.page(0)).and_then(|page| page.get(self.cursor)).copied().unwrap_or_default();
            self.cursor += 1;
            cx.consume_fuel(1);
            return StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), applied_progress: self.cursor as u64 });
        }
        self.output.get_or_insert_with(|| ImmediateJob { output: Some(self.bytes.to_vec()), writer: None, cursor: 0, closing: false }).step(cx)
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

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        if let Some(input) = self.input.as_mut() {
            let step = input.close_step(maximum_items, maximum_bytes);
            if input.terminal_is_empty() {
                self.input = None;
            }
            return match step {
                semio_framework_job::InteractiveJobCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                other => other,
            };
        }
        if let Some(output) = self.output.as_mut() {
            let step = output.close_step(maximum_items, maximum_bytes);
            if output.terminal_is_empty() {
                self.output = None;
            }
            return match step {
                semio_framework_job::InteractiveJobCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                other => other,
            };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

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
        Ok(RetainedNumberJob { input: Some(input), bytes: [0; 8], cursor: 0, output: None, closing: false })
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
    for _ in 0..8 {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        assert!(matches!(dispatch.job.step(&mut context), StepOutcome::CheckpointReady(_)));
    }
    dispatch.job.begin_close();
    let fixture = dsl::os_pack::json::parse(include_str!("../../🧹️wire-retirement/🧪️fixture/🔣️.json")).unwrap();
    let law = &fixture["shortClose"];
    let mut released = 0;
    for row in law["steps"].as_array().unwrap() {
        let items = usize::try_from(row["items"].as_u64().unwrap()).unwrap();
        let bytes = usize::try_from(row["bytes"].as_u64().unwrap()).unwrap();
        let released_bytes = usize::try_from(row["releasedBytes"].as_u64().unwrap()).unwrap();
        let expected = if row["blocked"].as_bool().unwrap() {
            semio_framework_job::InteractiveJobCloseStep::Blocked
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items: usize::try_from(row["releasedItems"].as_u64().unwrap()).unwrap(), released_bytes }
        };
        assert_eq!(dispatch.job.close_step(items, bytes), expected);
        released += released_bytes;
        assert_eq!(released + usize::try_from(row["remaining"].as_u64().unwrap()).unwrap(), 8);
    }
    assert_eq!(released, 8);
    assert_eq!(
        dispatch.job.close_step(1, 8),
        semio_framework_job::InteractiveJobCloseStep::Pending { released_items: usize::try_from(law["backingReleaseItems"].as_u64().unwrap()).unwrap(), released_bytes: usize::try_from(law["backingReleaseLogicalBytes"].as_u64().unwrap()).unwrap() }
    );
    assert_eq!(dispatch.job.close_step(1, 8), semio_framework_job::InteractiveJobCloseStep::Complete);
    assert!(dispatch.job.terminal_is_empty());
    eprintln!("[DEBUG] retained-number-close zero-items=blocked zero-bytes=blocked logical=7+1 backing-logical=0 terminal=true");
}

#[test]
fn production_typed_payload_and_retained_pages_enter_the_same_registered_factory_job() {
    let bus = ActionBus::new();
    bus.register(RetainedNumberFactory { keys: vec![ToolFactoryKey::new("number", "retained")] }).unwrap();
    let (admission, mut input) = bus.begin_exact_wire("number", "retained", "test.retained-number.v1", 8).unwrap();
    input.admit_page(ToolWirePage::try_copy_from(&42u64.to_le_bytes()).unwrap()).unwrap();
    input.seal().unwrap();
    let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(2), 3);
    let payload = RetainedNumberJob { input: None, bytes: [0; 8], cursor: 0, output: None, closing: false };
    let spec = ToolOperationSpec::new("number", "retained", "test.retained-number.v1", payload, operation);
    let mut dispatch = match bus.dispatch_wire_retained_with_spec(&admission, input, None, spec) {
        Ok(dispatch) => dispatch,
        Err(_) => panic!("production retained payload dispatch was rejected"),
    };
    let mut sequence = 0;
    for _ in 0..8 {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        assert!(matches!(dispatch.job.step(&mut context), StepOutcome::CheckpointReady(_)));
    }
    dispatch.job.begin_close();
    while !dispatch.job.terminal_is_empty() {
        let _ = dispatch.job.close_step(1, 8);
    }
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
