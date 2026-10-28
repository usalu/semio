// 🧨️ Inert label op and hostile input: the inert label operation fixture, a blocking mutation without editable inputs, an inverse refusal and hostile history edit input.

//#region 🚫️RowWithdraw
/// 🫥️ A payload schema whose one input is hidden: a draft editor would hold no row for it (design §22.20).
const HIDDEN_ONLY_SCHEMA: &str = r#"{"type":"object","additionalProperties":false,"required":["value"],"properties":{"value":{"type":"string","x-semio-ui":{"widget":"hidden","label":{"en":"Label","de":"Beschriftung"}}}}}"#;

/// 🪵️ A document operation over the toy kinds with two kinds an editor cannot repair — the label kind, whose payload
/// schema hides its only input and which refuses to apply on a negative count, and the children kind, which declares no
/// input schema and has no inverse under a count below -1: the smallest operations an upstream edit breaks without inputs
/// to edit.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
struct InertLabelOp(TestMutation);

impl semio_framework_value::ToValue for InertLabelOp {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, semio_framework_value::ValueError> {
        semio_framework_value::ToValue::to_value_controlled(&self.0, control)
    }
    fn to_value(&self) -> DslValue {
        semio_framework_value::ToValue::to_value(&self.0)
    }
}

impl semio_framework_value::FromValue for InertLabelOp {
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        <TestMutation as semio_framework_value::FromValue>::from_value_controlled(value, control).map(Self)
    }
    fn retire_decoded(self) { <TestMutation as semio_framework_value::FromValue>::retire_decoded(self.0) }
    fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <TestMutation as semio_framework_value::FromValue>::from_value(value).map(Self)
    }
}

impl ::protocol::OpBinary for InertLabelOp {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        ::protocol::OpBinary::encode_op(&self.0)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        <TestMutation as ::protocol::OpBinary>::decode_op(bytes).map(Self)
    }
}

impl ::protocol::OpText for InertLabelOp {
    fn print_op(&self) -> String {
        ::protocol::OpText::print_op(&self.0)
    }

    fn parse_op(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
        <TestMutation as ::protocol::OpText>::parse_op(line).map(Self)
    }
}

impl store::Mutation<TestSnapshot> for InertLabelOp {
    type Diff = <TestMutation as store::Mutation<TestSnapshot>>::Diff;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = <TestMutation as store::Mutation<TestSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor {
        store::Mutation::<TestSnapshot>::descriptor(&self.0)
    }





    fn conflict_target(&self) -> Vec<String> {
        store::Mutation::<TestSnapshot>::conflict_target(&self.0)
    }

    fn may_emit_foreign_steps(&self) -> bool {
        false
    }

    fn input_schema(&self) -> Option<&'static str> {
        match &self.0 {
            TestMutation::SetLabel(_) => Some(HIDDEN_ONLY_SCHEMA),
            TestMutation::SetSlotChildren(_) => None,
            operation => store::Mutation::<TestSnapshot>::input_schema(operation),
        }
    }

    fn payload_value(&self) -> DslValue {
        store::Mutation::<TestSnapshot>::payload_value(&self.0)
    }

    fn with_payload_value(&self, value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        store::Mutation::<TestSnapshot>::with_payload_value(&self.0, value).map(Self)
    }
}

impl store::ColdMutation<TestSnapshot> for InertLabelOp {
    type ReferenceDiff = <TestMutation as store::Mutation<TestSnapshot>>::Diff;
    fn diff(&self, base: &TestSnapshot) -> ::protocol::MutationOutcome<Self::ReferenceDiff> {
        match &self.0 {
            TestMutation::SetLabel(_) if base.count < 0 => ::protocol::MutationOutcome::error("mutation.target-mismatch", "a label needs a count that is not negative", ["count"]),
            operation => store::Mutation::<TestSnapshot>::diff(operation, base),
        }
    }
    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        if matches!(self.0, TestMutation::SetSlotChildren(_)) && base.count < -1 {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "children have no inverse under a count below -1"));
        }
        Ok(store::Mutation::<TestSnapshot>::inverse(&self.0, base)?.into_iter().map(Self).collect())
    }
}

/// ⏭️ Replays `drafts` from `from` on `document` to completion through its history-edit owners and answers the report.
async fn replayed_report(owners: &mut time_travel::TimeTravelStoreState<TestSnapshot, InertLabelOp>, document: &mut ArtifactStore<TestSnapshot, InertLabelOp>, drafts: &protocol::HistoryInputDrafts, from: &MutationId) -> protocol::ReplayReport {
    let (policy,_)=original_time_travel_fixture_caller();let grant=policy.preparation;let mut receiving_job=semio_framework_plugin_testing_laws::artifact_app_laws::FixtureOriginalJob::new(semio_framework::kernel::RetainedTurnInput{operation:71,generation:3,epoch:1,work: semio_framework_actor::OriginalActorTurnWork{clock_binding:1,absolute_deadline_us:u64::MAX,fuel:1_000_000}, grant:policy.close});
    let started = owners.run(document, time_travel::TimeTravelStoreCommand::StartReplay { drafts: drafts.clone(), from: from.clone() },None).await.expect("the replay starts");
    assert!(matches!(started, time_travel::TimeTravelStoreOutput::ReplayStarted(true)), "the store starts the Report replay");
    for _ in 0..65_536 {
        match receiving_job.receive(grant,|receiving|semio_framework_async::poll::resolve_ready(owners.run(document, time_travel::TimeTravelStoreCommand::StepReplay { deadline_us: u64::MAX, clock: semio_framework_job::default_now_us,grant },Some(receiving)))).expect("a replay slice") {
            time_travel::TimeTravelStoreOutput::Stepped(time_travel::TimeTravelReplayStep::Completed(report,_)) => return report,
            time_travel::TimeTravelStoreOutput::Stepped(time_travel::TimeTravelReplayStep::Pending { .. }) => {}
            _ => panic!("the replay faulted"),
        }
    }
    panic!("the replay never completed");
}

/// 🌰️ The schema id of the [`InertLabelOp`] documents.
const INERT_SCHEMA: &str = "semio.test.inert-label/v1";

/// 🥊️ A document store over [`InertLabelOp`] holding count 1, label a, no children and count 5 — one edit each, authored
/// by `local` — with the mutation ids of those four operations.
async fn inert_document(mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> (ArtifactStore<TestSnapshot, InertLabelOp>, Vec<MutationId>) {
    let genesis = store::os_store::document_materialize::create_document_envelope::<TestSnapshot, InertLabelOp>(INERT_SCHEMA, "inert-label", TestSnapshot::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut document = Box::pin(ArtifactStore::new(genesis, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))).await.expect("the document store");
    document.install_operation_wire(store::ArtifactOperationWire::canonical());
    let (mut catalog, receipt)=bounded_document_store_owners::<TestSnapshot, InertLabelOp>(mounted_policy.preparation).unwrap_or_else(|_|panic!("original inert catalog admission")); let mut catalog_receipts=vec![(mounted_policy.preparation,receipt)]; while !catalog.constructor_is_complete(){let receipt=catalog.admit_constructor(mounted_policy.preparation).unwrap_or_else(|_|panic!("original inert catalog construction")); assert!(receipt.fits(mounted_policy.preparation)); catalog_receipts.push((mounted_policy.preparation,receipt));} assert!(catalog_receipts.iter().all(|(grant,receipt)|receipt.fits(*grant))); document.install_document_store_owners_exact(catalog);
    assert_eq!(document.local_actor_id(), &protocol::ActorId("local".into()));
    for operation in [TestMutation::SetCount(SetCount { value: 1 }), TestMutation::SetLabel(SetLabel { value: "a".into() }), TestMutation::SetSlotChildren(SetSlotChildren { children: Vec::new() }), TestMutation::SetCount(SetCount { value: 5 })] {
        Box::pin(document.dispatch(ArtifactCommand::Apply { mutations: vec![InertLabelOp(operation)], transaction: None }, identity)).await.expect("a seed edit applies");
    }
    let ids = document.mutation_ops().expect("applied operations").iter().map(|op| op.mutation_id.clone()).collect();
    (document, ids)
}

/// 🪤️ Settles and retires the history-edit owners of an [`inert_document`], then closes it under its exact grant.
fn close_inert(mut owners: time_travel::TimeTravelStoreState<TestSnapshot, InertLabelOp>, mut document: ArtifactStore<TestSnapshot, InertLabelOp>, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1) {
    use time_travel::TimeTravelOwners;
    owners.settle(&document, TimeTravelStage::Inactive).expect("the owners settle");
    for _ in 0..65_536 {
        let grant=mounted_policy.close;
        if owners.retire_step(grant).expect("the owners retire").inspect(|step|assert!(step.progress().fits(grant))).is_none() {
            break;
        }
    }
    assert!(owners.terminal_is_empty(), "the history-edit owners retire everything they held");
    for _ in 0..65_536 {
        let grant=mounted_policy.close;
        let step=document.close_owned_step(grant).expect("the document store closes under its exact grant");assert!(step.progress().fits(grant));
        match step {
            semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) if progress==Default::default()=>panic!("the document store has no external owner"),
            semio_framework_value::retained_clone::RetainedCloneStep::Progress(_) => {}
            semio_framework_value::retained_clone::RetainedCloneStep::Complete(_) => {
                assert!(document.close_owned_terminal_is_empty());
                return;
            }
        }
    }
    panic!("the document store did not close");
}

/// ⚖️ LAW (design §22.1, §22.20): a blocking mutation without editable inputs is always resolvable. A mutation whose
/// payload schema hides every input, and one that declares no input schema, are not editable and still withdrawable (never
/// for a viewer). An upstream edit makes the hidden-only one fail; opening it for editing is refused
/// `timeTravel.not-editable` (an editor with zero rows never opens); opening it to withdraw it is admitted by the store's
/// supersede law on an editor without inputs; the replay with the withdrawal no longer blocks and finalizes as one
/// overwrite whose head holds the edit and skips the withdrawn operation.
#[semio_framework_async_macros::async_test]
async fn a_blocking_mutation_without_editable_inputs_is_withdrawn_and_the_review_becomes_ready() {
    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();let mut receiving_job=semio_framework_plugin_testing_laws::artifact_app_laws::FixtureOriginalJob::new(semio_framework::kernel::RetainedTurnInput{operation:71,generation:3,epoch:1,work: semio_framework_actor::OriginalActorTurnWork{clock_binding:1,absolute_deadline_us:u64::MAX,fuel:1_000_000}, grant:mounted_policy.close});
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    use time_travel::{TimeTravelActionRefusal, TimeTravelCommit, TimeTravelStoreCommand, TimeTravelStoreOutput, TimeTravelStoreState};
    let schema = INERT_SCHEMA;
    let (mut document, ids) = inert_document(mounted_policy, &mut identity).await;
    assert!(semio_framework::mutation_input_defs(HIDDEN_ONLY_SCHEMA, &semio_framework::registered_input_schema_document).is_ok_and(|inputs| inputs.len() == 1 && inputs[0].presentation == Some(semio_framework::ArgPresentation::Hidden)), "the hidden-only schema reads as one hidden input");
    let (count, label, children) = (ids[0].clone(), ids[1].clone(), ids[2].clone());
    {
        let ops = document.mutation_ops().expect("applied operations");
        let unlabelled = HashMap::new();
        let row = |index: usize, viewer: bool| {
            let view = time_travel::history_mutation_view_of::<TestSnapshot, InertLabelOp>(&ops[index], None, &unlabelled, &|_: &InertLabelOp| LocalizedLabel::data("operation"), viewer, None);
            (view.editable, view.withdrawable)
        };
        assert_eq!((row(0, false), row(1, false), row(2, false)), ((true, true), (false, true), (false, true)), "hidden-only inputs and no input schema: not editable, still withdrawable");
        assert_eq!((row(0, true), row(1, true), row(2, true)), ((false, false), (false, false), (false, false)), "a viewer edits and withdraws nothing");
    }

    let mut owners = TimeTravelStoreState::<TestSnapshot, InertLabelOp>::new(|_| LocalizedLabel::data("operation"));
    let negative = protocol::InputReplacement::Input { schema: schema.to_string(), payload: ::protocol::OpBinary::encode_op(&InertLabelOp(TestMutation::SetCount(SetCount { value: -1 }))).expect("the edited count encodes") };
    let mut drafts = protocol::HistoryInputDrafts::from([(count.clone(), negative)]);
    let blocked = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    let problem = blocked.outcomes.iter().find(|outcome| outcome.worst.is_some_and(|worst| protocol::MergePolicy::Normal.rejects(worst))).map(|outcome| outcome.mutation_id.clone());
    assert_eq!((blocked.blocks_finalize(), problem), (true, Some(label.clone())), "the edited count breaks the label operation downstream: {blocked:?}");

    for (target, why) in [(&label, "an editor with zero rows never opens"), (&children, "an operation without an input schema has no editor")] {
        let refused = owners.run(&mut document, TimeTravelStoreCommand::Open { target: target.clone(), current: None, withdraw: false },None).await.expect("the open runs");
        assert!(matches!(refused, TimeTravelStoreOutput::Opened(Err(TimeTravelActionRefusal::NotEditable))), "{why}");
    }
    let TimeTravelStoreOutput::Opened(Ok((editor, original))) = owners.run(&mut document, TimeTravelStoreCommand::Open { target: label.clone(), current: Some(protocol::InputReplacement::Withdrawn), withdraw: true },None).await.expect("the open runs") else {
        panic!("the store's supersede law admits withdrawing an operation without editable inputs");
    };
    assert!(editor.inputs.is_empty() && editor.inputs_refused.is_none() && editor.target == label, "its editor holds no inputs and names no schema fault");
    assert!(matches!(original, protocol::InputReplacement::Input { .. }), "its original input is its recorded operation");
    owners.run(&mut document, TimeTravelStoreCommand::Adopt(true),None).await.expect("the session adopts the kind");

    drafts.insert(label.clone(), protocol::InputReplacement::Withdrawn);
    let ready = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    assert!(!ready.blocks_finalize() && ready.outcomes.iter().any(|outcome| outcome.mutation_id == label && outcome.withdrawn), "withdrawing the failing operation leaves a report that finalizes: {ready:?}");
    let committed = owners.commit_finished(&mut document, drafts.clone(), store::HistoryFinalization::Overwrite, None, &mut identity).await.expect("the commit runs");
    assert!(matches!(committed, TimeTravelCommit::Finalized { .. }), "the finished replay commits as one overwrite");
    let head = document.snapshot().expect("head");
    assert_eq!((head.count, head.label.as_str()), (5, ""), "the head holds the edit and skips the withdrawn operation");

    close_inert(owners, document,mounted_policy);
}

/// ⚖️ LAW (design §22.5, §22.17): one inverse failure is one mutation's fatal, never a faulted session. An upstream edit
/// under which a downstream operation has no inverse leaves the Report replay complete: that operation's outcome is one
/// `Fatal` `mutation.inverse-refused`, the operation after it is still replayed and applies, and the report blocks
/// finalizing. Its row says so in words — "Fatal: Cannot be reversed" / "Kritisch: Nicht umkehrbar", never the code — and
/// is the way out: withdrawing it, with the other operation the same edit broke, leaves a report that finalizes as one
/// overwrite.
#[semio_framework_async_macros::async_test]
async fn an_inverse_refusal_is_one_mutations_fatal_that_its_row_names_and_resolves() {
    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();let mut receiving_job=semio_framework_plugin_testing_laws::artifact_app_laws::FixtureOriginalJob::new(semio_framework::kernel::RetainedTurnInput{operation:71,generation:3,epoch:1,work: semio_framework_actor::OriginalActorTurnWork{clock_binding:1,absolute_deadline_us:u64::MAX,fuel:1_000_000}, grant:mounted_policy.close});
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    use time_travel::{TimeTravelCommit, TimeTravelStoreCommand, TimeTravelStoreOutput, TimeTravelStoreState};
    let (mut document, ids) = inert_document(mounted_policy, &mut identity).await;
    let (count, label, children, tail) = (ids[0].clone(), ids[1].clone(), ids[2].clone(), ids[3].clone());
    let mut owners = TimeTravelStoreState::<TestSnapshot, InertLabelOp>::new(|_| LocalizedLabel::data("operation"));
    let lower = protocol::InputReplacement::Input { schema: INERT_SCHEMA.to_string(), payload: ::protocol::OpBinary::encode_op(&InertLabelOp(TestMutation::SetCount(SetCount { value: -2 }))).expect("the edited count encodes") };
    let mut drafts = protocol::HistoryInputDrafts::from([(count.clone(), lower)]);
    let outcome_of = |report: &protocol::ReplayReport, id: &MutationId| report.outcomes.iter().find(|outcome| outcome.mutation_id == *id).cloned().expect("an outcome per replayed operation");
    let blocked = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    let refused = outcome_of(&blocked, &children);
    assert_eq!((refused.worst, refused.messages.iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>()), (Some(semio_framework_diagnostic::Severity::Fatal), vec!["mutation.inverse-refused"]), "the refused inverse is one fatal on its mutation: {blocked:?}");
    assert!(blocked.blocks_finalize() && outcome_of(&blocked, &tail).worst.is_none(), "the replay went on past the refusal and the report blocks: {blocked:?}");
    {
        let ops = document.mutation_ops().expect("applied operations");
        let unlabelled = HashMap::new();
        let view = time_travel::history_mutation_view_of::<TestSnapshot, InertLabelOp>(&ops[2], Some(&refused), &unlabelled, &|_: &InertLabelOp| LocalizedLabel::data("operation"), false, None);
        let row = time_travel::history_mutation_entry(&view, None);
        assert_eq!((time_travel::history_mutation_description(&row, Locale::En), time_travel::history_mutation_description(&row, Locale::De)), ("Fatal: Cannot be reversed".to_string(), "Kritisch: Nicht umkehrbar".to_string()), "the row names the refusal in words, never the code");
        assert!(view.withdrawable && !view.editable, "the row offers the way out");
    }

    let TimeTravelStoreOutput::Opened(Ok(_)) = owners.run(&mut document, TimeTravelStoreCommand::Open { target: children.clone(), current: Some(protocol::InputReplacement::Withdrawn), withdraw: true },None).await.expect("the open runs") else {
        panic!("the store's supersede law admits withdrawing the operation without an inverse");
    };
    owners.run(&mut document, TimeTravelStoreCommand::Adopt(true),None).await.expect("the session adopts the kind");
    drafts.insert(children.clone(), protocol::InputReplacement::Withdrawn);
    let half = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    assert!(half.blocks_finalize() && outcome_of(&half, &children).withdrawn && outcome_of(&half, &children).worst.is_none(), "withdrawn, the refusal is gone and the other broken operation still blocks: {half:?}");
    drafts.insert(label.clone(), protocol::InputReplacement::Withdrawn);
    let ready = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    assert!(!ready.blocks_finalize(), "every broken operation withdrawn, the report finalizes: {ready:?}");
    let committed = owners.commit_finished(&mut document, drafts.clone(), store::HistoryFinalization::Overwrite, None, &mut identity).await.expect("the commit runs");
    assert!(matches!(committed, TimeTravelCommit::Finalized { .. }), "the finished replay commits as one overwrite");
    let head = document.snapshot().expect("head");
    assert_eq!((head.count, head.label.as_str()), (5, ""), "the head holds the edit and skips the withdrawn operations");
    close_inert(owners, document,mounted_policy);
}

/// ⚖️ LAW (design §22.6): the user-input path answers, it never panics. Every history-edit verb with missing, unknown
/// and wrongly typed arguments is answered — a result or a fault with a code — without a session, where the stage, the
/// document and its revision stay what they were, and inside one that edits a draft, where the session stays what it was.
#[semio_framework_async_macros::async_test]
async fn hostile_history_edit_input_is_answered_never_panicked_on() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let text_arg = |key: &str, value: &str| (key.to_string(), DslValue::String(value.to_string()));
    let garbage = || vec![("mutationId".to_string(), DslValue::uint(7)), ("path".to_string(), DslValue::uint(3)), ("value".to_string(), DslValue::Null), ("generation".to_string(), DslValue::String("x".into())), text_arg("choice", "sideways"), text_arg("name", ""), text_arg("store", "no-such-store")];
    let answered = |action: &str, answer: Result<InvocationResult, Fault>| {
        if let Err(fault) = answer {
            assert!(!fault.code.0.is_empty(), "{action}: a refusal carries its code: {fault:?}");
        }
    };

    let before = (head(&app), app.stores.store.content_revision_now());
    for action in semio_framework::history_edit::HISTORY_EDIT_ACTION_IDS {
        for args in [Vec::new(), vec![text_arg("mutationId", "no-such-mutation")], vec![("mutationId".to_string(), DslValue::uint(7))], vec![text_arg("mutationId", "no-such-mutation"), text_arg("store", "no-such-store")], garbage()] {
            let answer = app.handle_action(action, Some(&DslValue::Object(args.clone())), &meta(&fixture), &mut identity).await;
            answered(action, answer);
            assert_eq!((app.time_travel.session().stage, head(&app), app.stores.store.content_revision_now()), (TimeTravelStage::Inactive, before.0.clone(), before.1), "{action} {args:?}: nothing opened and nothing changed");
        }
    }

    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } })] {
        let result = run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
    }
    let session = app.time_travel.session().clone();
    let far = u64::from(session.generation) + 4_000_000;
    let hostile: Vec<(&str, Vec<(String, DslValue)>)> = vec![
        ("historyEditBegin", vec![text_arg("mutationId", "no-such-mutation")]),
        ("historyEditBegin", vec![("mutationId".to_string(), DslValue::uint(7))]),
        ("historyEditBegin", garbage()),
        ("historyEditWithdraw", vec![text_arg("mutationId", "no-such-mutation")]),
        ("historyEditRestore", Vec::new()),
        ("historyEditRestore", vec![text_arg("mutationId", "no-such-mutation")]),
        ("historyEditInput", Vec::new()),
        ("historyEditInput", vec![text_arg("path", "/no/such/input"), ("value".to_string(), DslValue::Null)]),
        ("historyEditInput", vec![("path".to_string(), DslValue::uint(3)), ("value".to_string(), DslValue::uint(3))]),
        ("historyEditInput", vec![text_arg("path", "/value"), text_arg("value", "c"), ("generation".to_string(), DslValue::uint(far))]),
        ("historyEditInput", garbage()),
        ("historyEditCommit", vec![text_arg("choice", "sideways")]),
        ("historyEditFinalize", Vec::new()),
        ("historyEditRerun", Vec::new()),
        ("historyEditCancelReplay", Vec::new()),
    ];
    for (action, args) in hostile {
        let answer = app.handle_action(action, Some(&DslValue::Object(args.clone())), &meta(&fixture), &mut identity).await;
        answered(action, answer);
        assert_eq!(app.time_travel.session(), &session, "{action} {args:?}: the session is what it was");
    }
    assert!(render_body(&mut app).await.contains("count=1 label=b"), "the draft preview stands");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🚫️RowWithdraw
