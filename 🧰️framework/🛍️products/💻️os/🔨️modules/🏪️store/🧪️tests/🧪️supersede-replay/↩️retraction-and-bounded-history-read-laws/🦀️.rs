// ↩️ Retraction and bounded history read laws: the refused local transition retraction and the bounded history read cursors.

//#region 🧪️RetractionLaws
/// 🔙️ A hub-refused local transition retracts (audit F-m8): the author's refused supersession, its refused
/// finalize-as-alternative (commit, branch, scoped supersession) leave the author's log through `BackboneMessage::Retract`
/// naming the supersessions and the commit — the branch depends on the commit and retracts with it — and the author
/// converges with the hub,
/// which never took them — the same event log, snapshot, supersessions, alternatives and outcomes, and the author's own
/// content revision from before its refused steps. Retracting again, retracting an unknown name or a transition another
/// replica authored changes nothing, and a persisted `.spr` retracts exactly the same transitions.
#[semio_framework_async_macros::async_test]
async fn a_refused_local_transition_retracts_and_the_author_converges_with_the_hub() {
    let (channel, remote) = ChannelBackbone::pair("retract").await;
    let mut author = demo_store("retract", Some(0)).await;
    author.attach_backbone(Backbones::Channel(channel)).await.expect("attach");
    apply(&mut author, vec![set(1)]).await;
    apply(&mut author, vec![add(2)]).await;
    apply(&mut author, vec![add(3)]).await;
    let ids = operation_ids(&author);
    let mut hub = demo_store("retract", Some(0)).await;
    for event in author.event_log().expect("log") {
        hub.ingest_remote(event).await.expect("the hub takes the edits");
    }
    let foreign = remote_supersession("retract", &ids[2], draft(Some(add(30))), 0);
    for replica in [&mut author, &mut hub] {
        replica.ingest_remote(foreign.clone()).await.expect("a peer's accepted supersession reaches both");
    }
    let accepted = (author.0.content_revision(), author.snapshot_ref().n, author.envelope().transitions.len());
    let before_refusal = print_document_pack(author.envelope()).await.expect("pair prints");
    let known = author.envelope().transitions.len();
    fixture_author(&mut author, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(10)))] }).await.expect("a local supersession");
    fixture_author(&mut author, ArtifactCommand::CreateAlternativeWithSupersede { name: "edited".into(), inputs: vec![input(&ids[1], None)] }).await.expect("finalize as a new alternative");
    let trunk = author.trunk_alternative_id();
    fixture_author(&mut author, ArtifactCommand::SwitchAlternative { alternative_id: trunk }).await.expect("back to the trunk");
    let authored: Vec<String> = author.envelope().transitions[known..].iter().map(|transition| transition.mutation_id.0.clone()).collect();
    assert_eq!(authored.len(), 4, "supersede, commit, branch, scoped supersede; switching moves this viewer's head only");
    let refused = vec![authored[0].clone(), authored[1].clone(), authored[3].clone()];
    let refused_log = print_document_pack(author.envelope()).await.expect("pair prints");
    let _ = drain_channel_for_test(&remote).expect("drain outbound");
    remote.push(BackboneMessage::Retract { mutation_ids: refused.clone() }).await.expect("push retraction");
    author.tick().await.expect("tick");
    let log_ids = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| store.event_log().expect("log").into_iter().map(|event| event.mutation_id.0).collect::<Vec<_>>();
    assert_eq!(log_ids(&author), log_ids(&hub), "the branch retracts with the commit it depends on");
    assert_eq!((author.0.content_revision(), author.snapshot_ref().n, author.envelope().transitions.len()), accepted, "the author is back where the hub took it");
    assert_eq!(author.snapshot_ref().n, hub.snapshot_ref().n);
    assert_eq!(author.supersessions(), hub.supersessions());
    assert_eq!(alternative_view(&author), alternative_view(&hub));
    assert_eq!(outcomes_by_mutation(&author.mutation_outcomes().expect("outcomes")), outcomes_by_mutation(&hub.mutation_outcomes().expect("outcomes")));
    let generation = author.generation();
    remote.push(BackboneMessage::Retract { mutation_ids: [refused.clone(), vec!["unknown".into(), foreign.mutation_id.0.clone()]].concat() }).await.expect("push again");
    author.tick().await.expect("tick");
    assert_eq!((author.generation(), author.0.content_revision(), log_ids(&author)), (generation, accepted.0, log_ids(&hub)), "a known, unknown or foreign name retracts nothing");
    let (spr, retracted) = retract_history_transitions_from_spr(&refused_log.spr, &refused).await.expect("the persisted log retracts");
    assert_eq!(retracted, authored, "the persisted log retracts the same closure");
    let persisted = |bytes: Vec<u8>| async move { semio_framework_os_spr::decode_history(&bytes, &semio_framework_os_spr::DecodeOptions::default()).await.expect("log decodes").transitions.into_iter().map(|record| record.id).collect::<Vec<_>>() };
    assert_eq!(persisted(spr).await, persisted(before_refusal.spr).await);
}
//#endregion 🧪️RetractionLaws

/// ⌛️ Planning, prefix folding and replay each yield after one unit; cancelled and stale cursors never publish.
#[semio_framework_async_macros::async_test]
async fn bounded_history_read_cursors_obey_the_neutral_law() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️supersede-replay/🔣️.json")).unwrap();
    let law = &corpus["boundedHistoryRead"];
    let close_items = law["closeGrant"]["maximumItems"].as_u64().unwrap() as usize;
    let release_bytes = law["closeGrant"]["maximumReleaseBytes"].as_u64().unwrap() as usize;
    assert_eq!(close_items, 1);
    assert_eq!(release_bytes, 4096);
    let mut store = demo_store("bounded-history-read", Some(law["initial"]["n"].as_i64().unwrap() as i32)).await;
    for edit in law["edits"].as_array().unwrap() {
        apply(&mut store, edit.as_array().unwrap().iter().map(|op| DemoMutation::from_value(op.clone().into()).unwrap()).collect()).await;
    }
    let ids = store.mutation_ops().unwrap();
    let id_of = |position: &serde_json::Value| ids.iter().find(|op| op.position == position["edit"].as_u64().unwrap() as usize && op.op_index == position["op"].as_u64().unwrap() as u32).unwrap().mutation_id.clone();
    let target = id_of(&law["target"]);
    let accepted: protocol::HistoryInputDrafts = law["accepted"].as_array().unwrap().iter().map(|input| (id_of(input), draft(Some(DemoMutation::from_value(input["replacement"].clone().into()).unwrap())))).collect();
    for row in &ids { assert_eq!(store.mutation_position(&row.mutation_id).unwrap(), (row.position, row.op_index as usize)); }
    let head = store.snapshot_ref().n;
    hot_path_census::take();
    let mut preview = Some(store.begin_derived_history_preview(target.clone(), accepted.clone()).unwrap());
    assert_eq!(hot_path_census::take(), hot_path_census::HotPathCensus::default(), "starting does not scan/hash/fold history");
    let mut pending_steps = 0;
    loop {
        let mut work = 0;
        let grant=fixture_planning_policy();
        let step = store.step_derived_history_preview(preview.as_mut().unwrap(), grant, &mut || { work += 1; true }).unwrap();
        assert!(work <= law["maximumWorkPerStep"].as_u64().unwrap() as usize);
        assert_eq!(hot_path_census::take().digested, 0, "trusted prefixes need no history digest scan");
        assert_eq!(store.snapshot_ref().n, head);
        if matches!(step, ReplayStep::Finished(_)) { break; }
        pending_steps += 1;
    }
    assert!(pending_steps > 2, "planning and same-edit prefix operations yield separately");
    let before = store.finish_derived_history_preview(&mut preview).unwrap();
    assert!(preview.as_ref().unwrap().state.is_none());
    let grant=physical_test_close_grant();
    let (residue,receipt)=store.retire_derived_history_preview(&mut preview,grant).unwrap().unwrap();assert!(receipt.fits(grant));drive_retirement_terminal(residue);
    assert_eq!(before.snapshot().n, Some(law["expected"]["before"].as_i64().unwrap() as i32));
    let replacement = DemoMutation::from_value(law["replacement"].clone().into()).unwrap();
    let (next, _) = store.derive_history_snapshot(&before, &replacement).unwrap();
    assert_eq!(next.unwrap().snapshot().n, Some(law["expected"]["preview"].as_i64().unwrap() as i32));
    let mut changed = accepted.clone();
    changed.insert(target.clone(), draft(Some(replacement)));
    let mut replay = Some(store.begin_derived_report_replay(changed, Some(target.clone())).unwrap());
    let mut slices = 0;
    loop {
        let mut work = 0;
        let grant=fixture_planning_policy();
        let step = store.step_derived_report_replay(replay.as_mut().unwrap(), grant, &mut || { work += 1; true }).unwrap();
        assert!(work <= 1);
        assert_eq!(store.snapshot_ref().n, head);
        slices += 1;
        if matches!(step, ReplayStep::Finished(_)) { break; }
    }
    assert!(slices > 5, "the planner and replay are both sliced");
    let (result, head) = store.finish_derived_report_replay(&mut replay).unwrap();
    let head = head.unwrap();
    assert_eq!(head.snapshot().n, Some(law["expected"]["head"].as_i64().unwrap() as i32));
    let mut result = Some(result);
    let grant=physical_test_close_grant();
    let(mut retirement,birth)=store.retire_finished_history_replay(&mut result,grant).unwrap().unwrap();assert!(birth.fits(grant));
    assert!(result.is_none());
    let mut turns = 0;
    let head_identity = Arc::as_ptr(head.snapshot_owner());
    let head_value = head.snapshot().n;
    let mut held_head = Some(head);
    let mut physical_page_seen = false;
    let mut physical_demands = std::collections::BTreeSet::new();
    let mut physical_releases = std::collections::BTreeSet::new();
    let mut blocked_on_head = false;
    loop {
        turns += 1;
        assert!(turns < 100_000);
        let physical_demand = retirement.next_release_byte_demand().unwrap();
        physical_demands.insert(physical_demand);
        if physical_demand == law["physicalInversePageBytes"].as_u64().unwrap() as usize { physical_page_seen = true; }
        let grant = fixture_retirement_policy(close_items, release_bytes);
        assert!(physical_demand <= grant.maximum_release_bytes, "explicit physical release grant admits the exact original owner extent: demand {physical_demand} against {grant:?}");
        if let Some(head) = held_head.as_ref() { assert_eq!(Arc::as_ptr(head.snapshot_owner()), head_identity); assert_eq!(head.snapshot().n, head_value); }
        match retirement.close_step(grant).unwrap() {
            semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress) => { assert!(progress.fits(grant)); assert!(retirement.terminal_is_empty()); break; }
            semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) => { physical_releases.insert(progress.released_bytes); assert!(progress.fits(grant)); }
        }
    }
    assert!(physical_page_seen, "actual retained inverse4096-byte allocation is observed before release");
    assert!(turns > 1 && law["expected"]["cancelRetirementCompletes"].as_bool().unwrap());
    assert!(!blocked_on_head, "retiring this read cursor releases its own alias without consuming the independently captured head");
    let held_head = held_head.take().expect("the external readable head remains owned after cursor retirement");
    assert_eq!(Arc::as_ptr(held_head.snapshot_owner()), head_identity);
    assert_eq!(held_head.snapshot().n, head_value);
    drive_retirement_terminal(admit_demo_alias_retirement(&store,held_head.into_snapshot_owner()));
    for stop in law["cancelAfterSteps"].as_array().unwrap() {
        let mut cancelled = Some(store.begin_derived_history_preview(target.clone(), accepted.clone()).unwrap());
        for _ in 0..stop.as_u64().unwrap() {
            let grant=fixture_planning_policy();
            if matches!(store.step_derived_history_preview(cancelled.as_mut().unwrap(), grant, &mut || true).unwrap(), ReplayStep::Finished(_)) { break; }
        }
        let grant=physical_test_close_grant();
        let(mut retirement,birth)=store.retire_derived_history_preview(&mut cancelled,grant).unwrap().unwrap();assert!(birth.fits(grant));
        assert!(cancelled.is_none(), "cancellation transfers ownership before any metadata drain");
        let mut turns = 0;
        loop {
            turns += 1;
            assert!(turns < 100_000);
            let grant = fixture_retirement_policy(close_items, release_bytes);
            match retirement.close_step(grant).unwrap() {
                semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress) => { assert!(progress.fits(grant)); assert!(retirement.terminal_is_empty()); break; }
                semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) if progress == RetainedCloneProgress::default() => panic!("unpublished preview owns no outstanding reads; stalled at turn {turns} under {grant:?} quoting copy {:?} capacity {:?} release {:?} depth {:?}", retirement.next_copy_byte_demand(), retirement.next_capacity_byte_demand(0), retirement.next_release_byte_demand(), retirement.next_depth_demand()),
                semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) => assert!(progress.fits(grant)),
            }
            assert_eq!(store.snapshot_ref().n, Some(99));
        }
        assert!(turns > 1 && law["expected"]["cancelRetirementCompletes"].as_bool().unwrap());
        let mut cancelled = Some(store.begin_derived_report_replay(accepted.clone(), Some(target.clone())).unwrap());
        for _ in 0..stop.as_u64().unwrap() {
            let grant=fixture_planning_policy();
            if matches!(store.step_derived_report_replay(cancelled.as_mut().unwrap(), grant, &mut || true).unwrap(), ReplayStep::Finished(_)) { break; }
        }
        let grant=physical_test_close_grant();
        let(mut retirement,birth)=store.retire_derived_report_replay(&mut cancelled,grant).unwrap().unwrap();assert!(birth.fits(grant));
        assert!(cancelled.is_none());
        let mut turns = 0;
        loop {
            turns += 1;
            assert!(turns < 100_000);
            let grant = fixture_retirement_policy(close_items, release_bytes);
            match retirement.close_step(grant).unwrap() {
                semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress) => { assert!(progress.fits(grant)); assert!(retirement.terminal_is_empty()); break; }
                semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) if progress == RetainedCloneProgress::default() => panic!("unpublished replay owns no outstanding reads"),
                semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) => assert!(progress.fits(grant)),
            }
            assert_eq!(store.snapshot_ref().n, Some(99));
        }
        assert!(turns > 1);
    }
    assert_eq!(store.snapshot_ref().n, Some(99));
    let mut stale = store.begin_derived_history_preview(target, accepted).unwrap();
    apply(&mut store, vec![add(1)]).await;
    let grant=fixture_planning_policy();
    assert!(store.step_derived_history_preview(&mut stale, grant, &mut || true).is_err());
    let tail = operation_ids(&store).last().unwrap().clone();
    assert_eq!(store.mutation_position(&tail).unwrap(), (2, 0));
    fixture_author(&mut store, ArtifactCommand::Undo).await.unwrap();
    assert!(store.mutation_position(&tail).is_err(), "the exact authority refuses an undone identity");
    fixture_author(&mut store, ArtifactCommand::Redo).await.unwrap();
    assert_eq!(store.mutation_position(&tail).unwrap(), (2, 0));
}
