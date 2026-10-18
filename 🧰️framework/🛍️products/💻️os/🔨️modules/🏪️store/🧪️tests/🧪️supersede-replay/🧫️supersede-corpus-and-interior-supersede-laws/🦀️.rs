// 🧫️ Supersede corpus and interior supersede laws: the supersede replay corpus against the store, the interior supersede against a fresh replay, the check in refusals, the downstream outcomes, the withdrawn operation and the scoped supersessions.

//#region 🧪️Corpus
/// 🧫️ The language-agnostic corpus: the store's Report replay reproduces every case the independent fast-json-patch
/// oracle (`🟦️.ts` beside this file) reproduces, a clean case finalizes to the same state and durable outcomes, and a
/// blocking case is refused with nothing changed.
#[semio_framework_async_macros::async_test]
async fn supersede_replay_corpus_matches_the_store() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️supersede-replay/🔣️.json")).expect("corpus parses");
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("case name");
        let initial: Option<i32> = serde_json::from_value(case["initial"]["n"].clone()).expect("initial n");
        let mut store = demo_store(name, initial).await;
        store.enable_convergence_early_exit();
        for edit in case["edits"].as_array().expect("edits") {
            apply(&mut store, edit.as_array().expect("edit operations").iter().map(|operation| DemoMutation::from_value(operation.clone().into()).expect("corpus operations decode")).collect()).await;
        }
        let operations: Vec<(usize, u32, MutationId)> = store.mutation_ops().expect("operations").into_iter().map(|operation| (operation.position, operation.op_index, operation.mutation_id)).collect();
        let id_of = |edit: u64, op: u64| operations.iter().find(|(position, index, _)| *position as u64 == edit && u64::from(*index) == op).map(|(_, _, id)| id.clone()).expect("the corpus names a recorded operation");
        let drafts: protocol::HistoryInputDrafts = case["supersessions"]
            .as_array()
            .expect("supersessions")
            .iter()
            .map(|supersession| {
                let replacement = match &supersession["replacement"] {
                    serde_json::Value::String(withdrawn) if withdrawn == "withdrawn" => protocol::InputReplacement::Withdrawn,
                    serde_json::Value::Object(input) if input.contains_key("invalid") => {
                        protocol::InputReplacement::Input { schema: input["invalid"]["schema"].as_str().expect("invalid schema").into(), payload: hex(input["invalid"]["payloadHex"].as_str().expect("invalid payload")) }
                    }
                    operation => draft(Some(DemoMutation::from_value(operation.clone().into()).expect("replacement decodes"))),
                };
                (id_of(supersession["edit"].as_u64().unwrap(), supersession["op"].as_u64().unwrap()), replacement)
            })
            .collect();
        let replay = store.begin_report_replay(&drafts, Some(&operations[0].2)).expect("the replay begins at the genesis");
        let result = finish(&store, replay);
        let report = store.replay_report(&result).expect("report");
        let expected = &case["expected"];
        let expected_n: Option<i32> = serde_json::from_value(expected["state"]["n"].clone()).expect("expected n");
        assert_eq!(result.state().expect("reached state").n, expected_n, "{name}: reached state");
        let observed: Vec<serde_json::Value> = outcomes_by_position(&store, &report.outcomes)
            .into_iter()
            .map(|(edit, op, worst, codes, superseded, withdrawn)| serde_json::json!({ "edit": edit, "op": op, "worst": worst.map(|level| format!("{level:?}").to_lowercase()), "codes": codes, "superseded": superseded, "withdrawn": withdrawn }))
            .collect();
        assert_eq!(serde_json::Value::Array(observed), expected["outcomes"], "{name}: per-mutation outcomes");
        assert_eq!(report.blocks_finalize(), expected["blocksFinalize"].as_bool().unwrap(), "{name}: finalize gate");
        let generation = store.generation();
        match fixture_finalize(&mut store,result, HistoryFinalization::Overwrite).await {
            Ok(_) => {
                assert!(!report.blocks_finalize(), "{name}: a blocking report never commits");
                assert_eq!(store.snapshot_ref().n, expected_n, "{name}: finalized state");
                test_support::assert_live_equals_replay(&store).await;
                let durable = store.mutation_outcomes().expect("durable outcomes");
                assert_eq!(outcomes_by_position(&store, &durable), outcomes_by_position(&store, &report.outcomes), "{name}: durable outcomes equal the report");
            }
            Err(VcsError::Rejected { policy, messages }) => {
                assert!(report.blocks_finalize(), "{name}: only a blocking report is refused");
                assert_eq!(policy, semio_framework_os_spr::MergePolicy::Normal);
                assert!(messages.iter().any(|message| message.level >= semio_framework_diagnostic::Severity::Error));
                assert_eq!(store.generation(), generation, "{name}: a refused finalize changes nothing");
                assert!(store.supersessions().is_empty());
            }
            Err(error) => panic!("{name}: unexpected finalize failure {error}"),
        }
    }
}
//#endregion 🧪️Corpus

//#region 🧪️SupersedeLaws
/// ✏️ Superseding an interior operation keeps every slot, changes the revision, and leaves exactly the state and durable
/// messages a fresh store folding the edited log reaches; both persisted forms round-trip it.
#[semio_framework_async_macros::async_test]
async fn interior_supersede_equals_a_fresh_replay_of_the_edited_log() {
    let mut store = demo_store("interior", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    apply(&mut store, vec![add(3), add(4)]).await;
    let ids = operation_ids(&store);
    let applied = store.applied_edit_ids().to_vec();
    let revision = store.0.content_revision();
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(10)))] }).await.expect("a clean supersession authors");
    assert_eq!(store.snapshot_ref().n, Some(19));
    assert_eq!(store.applied_edit_ids(), applied.as_slice(), "a supersession keeps every slot");
    assert_ne!(store.0.content_revision(), revision, "the revision names the effective forwards");
    assert_eq!(store.supersessions().len(), 1);
    assert_eq!(store.0.last_projection_cause, Some(ArtifactProjectionCause::Replay));
    let transition = store.envelope().transitions.last().expect("the supersession is an event");
    assert_eq!(transition.dependencies, vec![ids[0].clone()], "a supersession depends on its targets");
    assert_eq!(transition.target, vec!["n".to_string()], "a supersession declares the address it writes");
    test_support::assert_live_equals_replay(&store).await;
    let mut fresh = demo_store("interior-fresh", Some(0)).await;
    apply(&mut fresh, vec![set(10)]).await;
    apply(&mut fresh, vec![add(2)]).await;
    apply(&mut fresh, vec![add(3), add(4)]).await;
    assert_eq!(store.snapshot().expect("edited"), fresh.snapshot().expect("fresh"));
    assert_eq!(
        outcomes_by_position(&store, &store.mutation_outcomes().unwrap()).into_iter().map(|(edit, op, worst, codes, _, _)| (edit, op, worst, codes)).collect::<Vec<_>>(),
        outcomes_by_position(&fresh, &fresh.mutation_outcomes().unwrap()).into_iter().map(|(edit, op, worst, codes, _, _)| (edit, op, worst, codes)).collect::<Vec<_>>()
    );
    test_support::assert_document_text_round_trip(&store).await;
    test_support::assert_document_pack_round_trip(&store).await;
    let files = print_document_pack(store.envelope()).await.expect("pack prints");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("pack parses").into_envelope(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(reloaded.snapshot().expect("reloaded"), store.snapshot().expect("live"));
    assert_eq!(reloaded.supersessions().keys().collect::<Vec<_>>(), store.supersessions().keys().collect::<Vec<_>>());
    assert_eq!(reloaded.envelope().transitions, store.envelope().transitions, "the reload re-derives every supersession target");
    assert_eq!(reloaded.mutation_outcomes().unwrap(), store.mutation_outcomes().unwrap());
    let log = semio_framework_os_spr::decode_history(&files.spr, &semio_framework_os_spr::DecodeOptions::default()).await.expect("spr decodes");
    assert!(log.fold().expect("the persisted log folds").supersessions.contains_key(&ids[0]), "HistoryLog::fold resolves the supersession");
}

/// 🛂️ Hub check-in replays a ledger under `Normal`: a remote supersession whose downstream replay raises an `Error` is
/// refused as `Rejected { Normal }`, a clean one folds into the next pair.
#[semio_framework_async_macros::async_test]
async fn check_in_refuses_a_supersession_whose_replay_blocks_under_normal() {
    let mut store = demo_store("check-in", None).await;
    apply(&mut store, vec![DemoMutation::AssignN(AssignN { n: Some(5) })]).await;
    apply(&mut store, vec![add(1)]).await;
    let ids = operation_ids(&store);
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let supersede = |replacement: Option<DemoMutation>, logical: u64| {
        let transition = semio_framework_os_spr::HistoryTransition::Supersede(protocol::TransitionSupersede { scope: None, inputs: vec![protocol::SupersededInput { target: ids[0].clone(), replacement: draft(replacement) }] });
        semio_framework_os_spr::encode_envelopes(&[semio_framework_os_spr::history_transition_envelope(
            &transition,
            &ArtifactId("check-in".into()),
            &ActorId("editor".into()),
            vec![ids[0].clone()],
            HybridLogicalTimestamp { actor: 9, physical_ms: u64::MAX / 2, logical },
        )])
    };
    let refused = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &supersede(None, 0), || Ok(fixture_demo_owners())).await;
    assert!(matches!(&refused, Err(VcsError::Rejected { policy: semio_framework_os_spr::MergePolicy::Normal, messages }) if messages.iter().any(|message| message.code.0 == "mutation.target-missing")), "{refused:?}");
    assert!(refused.unwrap_err().to_string().contains("rejected by merge policy Normal"));
    let accepted = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &supersede(Some(DemoMutation::AssignN(AssignN { n: Some(40) })), 1), || Ok(fixture_demo_owners()))
        .await
        .expect("a clean supersession checks in");
    let parsed = parse_document_pack::<DemoSnapshot, DemoMutation>(&accepted.pack, &accepted.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("checked-in pair parses");
    assert_eq!(parsed.snapshot.n, Some(41));
    test_support::retire_parsed_document(parsed);
}

/// 🚦️ Warning, Error and Fatal downstream outcomes are reported per mutation; Error and Fatal refuse the supersession
/// atomically with `Rejected { Normal }`, a Warning commits and stays in the durable history.
#[semio_framework_async_macros::async_test]
async fn downstream_warning_error_and_fatal_outcomes_are_reported_per_mutation() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "severity", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    for operation in [SeverityMutation::SetN(SeveritySetN { n: 1 }), SeverityMutation::SetN(SeveritySetN { n: 2 }), SeverityMutation::SetN(SeveritySetN { n: 3 })] {
        fixture_author(&mut store, ArtifactCommand::Apply { mutations: vec![operation], transaction: None }).await.expect("clean severity edit");
    }
    let ids: Vec<MutationId> = store.mutation_ops().unwrap().into_iter().map(|operation| operation.mutation_id).collect();
    let severity_draft = |operation: SeverityMutation| protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: operation.encode_op().unwrap() };
    let entries = protocol::HistoryInputDrafts::from([
        (ids[0].clone(), severity_draft(SeverityMutation::SetWarningN(SetWarningN { n: 5 }))),
        (ids[1].clone(), severity_draft(SeverityMutation::SetErrorN(SetErrorN { n: 6 }))),
        (ids[2].clone(), severity_draft(SeverityMutation::SetFatalN(SetFatalN { n: 7 }))),
    ]);
    let mut replay = store.begin_report_replay(&entries, None).expect("preview replay");
    drive_test_report_replay(&mut replay, store.replay_edits());
    let result = replay.finish().expect("finished");
    let report = store.replay_report(&result).expect("report");
    let levels: Vec<Option<semio_framework_diagnostic::Severity>> = report.outcomes.iter().map(|outcome| outcome.worst).collect();
    assert_eq!(levels, vec![Some(semio_framework_diagnostic::Severity::Warning), Some(semio_framework_diagnostic::Severity::Error), Some(semio_framework_diagnostic::Severity::Fatal)]);
    assert!(report.blocks_finalize());
    assert_eq!(report.worst, Some(semio_framework_diagnostic::Severity::Fatal));
    result.retire_unscheduled().expect("the inspected replay retires from its own quoted demands");
    let generation = store.generation();
    let snapshot = store.snapshot().unwrap();
    for (target, operation) in [(&ids[1], SeverityMutation::SetErrorN(SetErrorN { n: 6 })), (&ids[2], SeverityMutation::SetFatalN(SetFatalN { n: 7 }))] {
        let refused = fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target: target.clone(), replacement: Some(operation) }] }).await;
        assert!(matches!(refused, Err(VcsError::Rejected { policy: semio_framework_os_spr::MergePolicy::Normal, ref messages }) if !messages.is_empty()), "{refused:?}");
        assert_eq!(store.generation(), generation, "a refused supersession changes nothing");
        assert_eq!(store.snapshot().unwrap(), snapshot);
        assert!(store.supersessions().is_empty());
    }
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target: ids[0].clone(), replacement: Some(SeverityMutation::SetWarningN(SetWarningN { n: 5 })) }] }).await.expect("a warning never blocks");
    let durable = store.mutation_outcomes().unwrap();
    assert_eq!(durable[0].worst, Some(semio_framework_diagnostic::Severity::Warning), "the warning persists in history");
    assert!(durable[0].superseded);
    assert_eq!(store.snapshot_ref().n, Some(3), "downstream sets still win");
}

/// 🚫️ A withdrawn operation folds as a no-op, keeps its slot, and a later supersession restores it.
#[semio_framework_async_macros::async_test]
async fn withdrawing_an_operation_folds_it_as_a_no_op() {
    let mut store = demo_store("withdraw", Some(0)).await;
    apply(&mut store, vec![set(5)]).await;
    apply(&mut store, vec![add(2)]).await;
    apply(&mut store, vec![add(1)]).await;
    let ids = operation_ids(&store);
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[1], None)] }).await.expect("withdraw");
    assert_eq!(store.snapshot_ref().n, Some(6));
    let outcome = store.mutation_outcomes().unwrap().into_iter().find(|outcome| outcome.mutation_id == ids[1]).expect("withdrawn outcome");
    assert!(outcome.withdrawn && outcome.superseded && outcome.messages.is_empty());
    test_support::assert_live_equals_replay(&store).await;
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[1], Some(add(20)))] }).await.expect("a later supersession wins");
    assert_eq!(store.snapshot_ref().n, Some(26));
    test_support::assert_document_text_round_trip(&store).await;
}

/// 🌿️ A scoped supersession is effective exactly while its alternative is active; an unscoped one in every alternative.
#[semio_framework_async_macros::async_test]
async fn scoped_supersessions_follow_their_alternative_and_unscoped_ones_every_alternative() {
    let mut store = demo_store("scoped", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(1)]).await;
    let ids = operation_ids(&store);
    fixture_author(&mut store, ArtifactCommand::CreateAlternativeWithSupersede { name: "variant".into(), inputs: vec![input(&ids[0], Some(set(10)))] }).await.expect("variant alternative");
    let variant = store.envelope().active_alternative_id.clone().expect("the variant is active");
    assert_eq!(store.snapshot_ref().n, Some(11));
    assert_eq!(store.supersessions().get(&ids[0]).and_then(|supersession| supersession.scope.clone()), Some(variant.clone()));
    let outbound = store.envelope().transitions.len();
    assert_eq!(outbound, 3, "commit, branch and scoped supersede are one authored batch");
    fixture_author(&mut store, ArtifactCommand::CreateAlternative { name: "other".into() }).await.expect("another alternative at the head");
    assert_eq!(store.snapshot_ref().n, Some(2), "the scoped supersession leaves with its alternative");
    assert!(store.supersessions().is_empty());
    test_support::assert_live_equals_replay(&store).await;
    fixture_author(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: variant.clone() }).await.expect("switch back");
    assert_eq!(store.snapshot_ref().n, Some(11), "switching back re-applies the scoped supersession");
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[1], Some(add(5)))] }).await.expect("unscoped");
    assert_eq!(store.snapshot_ref().n, Some(15));
    let other = store.envelope().vcs.alternatives.iter().find(|alternative| alternative.name == "other").map(|alternative| alternative.id.clone()).expect("other alternative");
    fixture_author(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: other }).await.expect("switch to other");
    assert_eq!(store.snapshot_ref().n, Some(6), "an unscoped supersession holds in every alternative");
    test_support::assert_live_equals_replay(&store).await;
    test_support::assert_document_text_round_trip(&store).await;
    let refused = fixture_author(&mut store, ArtifactCommand::Supersede { scope: Some("missing".into()), inputs: vec![input(&ids[1], Some(add(5)))] }).await;
    assert!(matches!(refused, Err(VcsError::UnknownAlternative(_))));
}
//#endregion 🧪️SupersedeLaws
