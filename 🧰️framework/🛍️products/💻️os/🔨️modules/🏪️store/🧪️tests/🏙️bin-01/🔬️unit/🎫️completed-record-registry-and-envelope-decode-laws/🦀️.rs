/// 🎫️ Completed record registry and envelope decode laws: the completed record owner and target, the completed envelope registry capacity and publication, the withheld decode success and the cancelled and rejected decodes.

fn completed_record_owner(id: &str) -> Box<dyn ArtifactEnvelopeCompletedRecord<(), ()>> {
    Box::new(ArtifactEnvelopeCompletedRecordOwner::new(create_document_envelope_from_genesis("test.completed-record/v1", id, AdmittedArtifactGenesis::from_decoded_pack((), vec![0]), None), Arc::new(UnitOwnedRetirementFactory), Arc::new(UnitOwnedRetirementFactory)))
}

fn drain_completed_record(mut owner: Box<dyn ArtifactEnvelopeCompletedRecord<(), ()>>) {
    let mut born = 0;
    let mut released = 0;
    for _ in 0..10_000 {
        let grant = physical_test_close_grant();
        assert!(owner.next_close_copy_byte_demand().expect("current completed record copy demand") <= grant.maximum_copy_bytes);
        assert!(owner.next_close_capacity_byte_demand(grant.maximum_copy_bytes).expect("current completed record capacity demand") <= grant.maximum_capacity_bytes);
        assert!(owner.next_close_release_byte_demand().expect("current completed record release demand") <= grant.maximum_release_bytes);
        assert!(owner.next_close_depth_demand().expect("current completed record depth demand") <= grant.maximum_depth);
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant));
        let step = step.expect("completed record bounded close");
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        born += heap.requested_bytes;
        released += heap.released_bytes;
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(owner.terminal_is_empty());
            let frame = std::mem::size_of_val(owner.as_ref());
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, frame));
            return;
        }
    }
    panic!("completed record must reach terminal empty under repeated one-owner grants");
}

fn admit_completed_record(registry: &ArtifactEnvelopeCompletedRecordRegistry<(), ()>, owner: Box<dyn ArtifactEnvelopeCompletedRecord<(), ()>>) -> ArtifactEnvelopeCompletedRecordTicket {
    match registry.try_admit(owner) {
        Ok(ticket) => ticket,
        Err((fault, owner)) => {
            drain_completed_record(owner);
            panic!("completed record admission unexpectedly failed: {fault:?}")
        }
    }
}

struct TestCompletedRecordTarget {
    expected: &'static str,
    adopted: Option<Box<dyn ArtifactEnvelopeCompletedRecord<(), ()>>>,
}

impl ArtifactEnvelopeCompletedRecordTarget<(), ()> for TestCompletedRecordTarget {
    fn try_adopt_completed(&mut self, envelope: ArtifactEnvelope<(), ()>) -> Result<(), ArtifactEnvelope<(), ()>> {
        if envelope.id != self.expected || self.adopted.is_some() {
            return Err(envelope);
        }
        self.adopted = Some(Box::new(ArtifactEnvelopeCompletedRecordOwner::new(envelope, Arc::new(UnitOwnedRetirementFactory), Arc::new(UnitOwnedRetirementFactory))));
        Ok(())
    }
}

#[test]
fn completed_envelope_registry_rejects_capacity_plus_one_and_reuses_slots_with_aba_safety() {
    let registry = ArtifactEnvelopeCompletedRecordRegistry::<(), ()>::new();
    let mut tickets = Vec::new();
    for index in 0..ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY {
        tickets.push(admit_completed_record(&registry, completed_record_owner(&format!("record-{index}"))));
    }
    let (fault, rejected) = registry.try_admit(completed_record_owner("plus-one")).expect_err("capacity plus one returns exact owner");
    assert_eq!(fault, ArtifactEnvelopeCompletedRecordFault::Capacity);
    drain_completed_record(rejected);

    let stale = tickets.remove(0);
    let owner = registry.try_detach(stale).expect("exact first completed owner");
    drain_completed_record(owner);
    let reused = admit_completed_record(&registry, completed_record_owner("reused"));
    assert_eq!(reused.index(), stale.index());
    assert_ne!(reused.generation(), stale.generation());
    match registry.try_detach(stale) {
        Err(fault) => assert_eq!(fault, ArtifactEnvelopeCompletedRecordFault::Stale),
        Ok(owner) => {
            drain_completed_record(owner);
            panic!("stale completed ticket detached the reused owner")
        }
    }
    tickets.push(reused);
    for ticket in tickets {
        drain_completed_record(registry.try_detach(ticket).expect("exact completed owner drains"));
    }
    assert!(registry.terminal_is_empty());
}

#[test]
fn completed_envelope_registry_publication_is_exact_once_and_close_cursor_is_interruptible() {
    let registry = ArtifactEnvelopeCompletedRecordRegistry::<(), ()>::new();
    let ticket = admit_completed_record(&registry, completed_record_owner("take-once"));
    let mut rejected = TestCompletedRecordTarget { expected: "wrong", adopted: None };
    assert!(!registry.try_publish_to(ticket, &mut rejected).expect("consumer rejection retains exact record"));
    let mut target = TestCompletedRecordTarget { expected: "take-once", adopted: None };
    let (published, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.try_publish_to(ticket, &mut target));
    assert!(published.expect("exact consumer atomically adopts completed record"));
    assert_eq!(heap.released_bytes, 0, "publication retains original record frame and factory sources for their separately granted close");
    assert!(!registry.ticket_reclaimed(ticket));
    let mut close_cursor = 0;
    assert_eq!(registry.try_next_close_ticket(&mut close_cursor).expect("published original close ticket"), Some(ticket));
    assert_eq!(registry.try_publish_to(ticket, &mut target).expect_err("completed record is exact once"), ArtifactEnvelopeCompletedRecordFault::Stale);
    drain_completed_record(registry.try_detach(ticket).expect("same published owner retires on original close pump"));
    assert!(registry.ticket_reclaimed(ticket));
    drain_completed_record(target.adopted.take().expect("test target retained adopted owner"));

    let ticket = admit_completed_record(&registry, completed_record_owner("interrupted-close"));
    let mut cursor = 0;
    assert_eq!(registry.try_next_ticket(&mut cursor).expect("fixed next ticket"), Some(ticket));
    let owner = registry.try_detach(ticket).expect("close pump detaches exact record");
    drain_completed_record(owner);
    assert!(registry.terminal_is_empty());
}


#[test]
fn artifact_envelope_decode_withholds_success_until_field_and_page_owners_are_terminal_empty() {
    use semio_framework_job::InteractiveJob;

    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let mut job = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]),
        &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    )
    .unwrap_or_else(|_| panic!("fixed field decoder admission"));
    let mut preview_sequence = 0;
    let mut detached = None;
    for turn in 0..1_000 {
        drive_test_envelope_field_return(&registry, &mut detached);
        let mut original_retained_progress_5=semio_framework_value::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(91),
            semio_framework_job::Generation(7),
            semio_framework_job::StepBudget::new(4, u64::MAX, schema_test_close_grant()),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
         &mut original_retained_progress_5);
        match job.step(&mut context).expect("original decoder outcome admission") {
            None | Some(semio_framework_job::JobOutcomeBorrow::Yield { .. }) => assert!(!job.terminal_is_empty() || registry.terminal_is_empty(), "a yielded decode retains exact owner authority until its funded outcome turn"),
            Some(semio_framework_job::JobOutcomeBorrow::Complete { state: None, output: None, .. }) => {
                assert!(job.terminal_is_empty(), "success is exposed only after exact field and page terminal witnesses");
                assert!(registry.terminal_is_empty(), "successful decode reclaimed its exact registered field owner");
                assert!(turn > 1, "decode and close span multiple bounded turns");
                return;
            }
            outcome => panic!("valid envelope decode produced {outcome:?}"),
        }
    }
    panic!("valid envelope decode did not terminate");
}

#[test]
fn cancelled_and_rejected_envelope_decodes_close_one_exact_owner_per_grant() {
    use semio_framework_job::InteractiveJob;

    let first = [b' '; ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let mut cancelled = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[&first, br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]),
        &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    )
    .unwrap_or_else(|_| panic!("fixed cancelled decoder admission"));
    let cancel = semio_framework_job::root_cancel_token();
    cancel.cancel_now();
    let mut preview_sequence = 0;
    let mut turns = 0;
    let mut detached = None;
    loop {
        drive_test_envelope_field_return(&registry, &mut detached);
        let mut original_retained_progress_6=semio_framework_value::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(91),
            semio_framework_job::Generation(7),
            semio_framework_job::StepBudget::new(1, u64::MAX, schema_test_close_grant()),
            cancel.clone(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
         &mut original_retained_progress_6);
        turns += 1;
        match cancelled.step(&mut context).expect("original cancelled decoder outcome admission") {
            None | Some(semio_framework_job::JobOutcomeBorrow::Yield { .. }) => assert!(!cancelled.terminal_is_empty() || registry.terminal_is_empty(), "a cancelled decode retains exact owner authority until its funded outcome turn"),
            Some(semio_framework_job::JobOutcomeBorrow::Cancelled { .. }) => break,
            outcome => panic!("cancelled envelope decode produced {outcome:?}"),
        }
        assert!(turns < 256);
    }
    assert!(cancelled.terminal_is_empty());
    assert!(registry.terminal_is_empty());
    assert!(turns >= 5, "field authority, two pages, and terminal shells require distinct grants");

    let diagnostic = OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.test-rejected", offset: 0, line: 1, column: 1, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() };
    let mut rejected = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]),
        &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    )
    .unwrap_or_else(|_| panic!("fixed rejected decoder admission"))
    .reject(diagnostic)
    .unwrap_or_else(|_| panic!("public exact rejection transfer"));
    let mut close_turns = 0;
    let mut detached = None;
    while !rejected.terminal_is_empty() {
        drive_test_envelope_field_return(&registry, &mut detached);
        let step = rejected.close_step(envelope_field_close_grant()).expect("retained decode rejection closes");
        assert!(step.progress().fits(envelope_field_close_grant()));
        close_turns += 1;
        assert!(close_turns < 256);
    }
    assert!(close_turns >= 3, "rejected field, page, and terminal record are independently witnessed");
    assert!(registry.terminal_is_empty());
}

#[test]
fn artifact_envelope_public_rejection_preserves_record_lease_ticket_double_return_and_generation_reuse() {
    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let record = artifact_envelope_decode_test_cursor(&[br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]);
    let record_identity = record.tokens.pages.slots.as_ptr();
    let source = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(record, &registry, Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 })).unwrap_or_else(|_| panic!("fixed public rejection decoder admission"));
    let field_ticket = source.field_ticket;
    let registry_identity = Arc::as_ptr(&source.field_registry);
    let diagnostic = OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.public-rejected", offset: 7, line: 2, column: 3, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() };
    let mut rejected = source.reject(diagnostic).unwrap_or_else(|_| panic!("public rejection returns the exact owner authority"));
    assert_eq!(rejected.record.as_ref().map(|record| record.tokens.pages.slots.as_ptr()), Some(record_identity));
    assert_eq!(rejected.fields.as_ref().map(ArtifactEnvelopeFieldDecoderLease::ticket), Some(field_ticket));
    assert_eq!(Arc::as_ptr(&rejected.field_registry), registry_identity);
    assert_eq!(rejected.diagnostic, diagnostic);
    assert!(!registry.ticket_reclaimed(field_ticket));
    assert_eq!(rejected.close_step(RetainedCloneGrant::default()), Ok(RetainedCloneStep::Progress(Default::default())));
    assert_eq!(rejected.record.as_ref().map(|record| record.tokens.pages.slots.as_ptr()), Some(record_identity));
    assert_eq!(rejected.fields.as_ref().map(ArtifactEnvelopeFieldDecoderLease::ticket), Some(field_ticket));

    let mut detached = None;
    for _ in 0..256 {
        drive_test_envelope_field_return(&registry, &mut detached);
        let step = rejected.close_step(envelope_field_close_grant()).expect("public rejection closes one exact owner");
        assert!(step.progress().fits(envelope_field_close_grant()));
        if rejected.terminal_is_empty() {
            break;
        }
    }
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(rejected.terminal_is_empty());
    assert!(registry.ticket_reclaimed(field_ticket));
    assert!(registry.terminal_is_empty());
    assert!(detached.is_none());

    let mut reused = registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })).unwrap_or_else(|_| panic!("reused field generation"));
    let reused_ticket = reused.ticket();
    assert_eq!(reused_ticket.index(), field_ticket.index());
    assert_ne!(reused_ticket.generation(), field_ticket.generation());
    assert!(reused.return_now());
    assert!(!reused.return_now());
    drop(reused);
    drive_test_envelope_field_return(&registry, &mut detached);
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(registry.ticket_reclaimed(reused_ticket));
    assert!(registry.terminal_is_empty());
}
