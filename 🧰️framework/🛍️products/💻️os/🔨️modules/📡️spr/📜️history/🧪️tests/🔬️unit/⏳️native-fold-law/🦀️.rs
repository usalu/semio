/// ⏳️ Native history normalization under the neutral cancellation law.

/// ⏳️ Native history normalization shares one immutable source and obeys the neutral cancellation grants.
#[test]
fn retained_native_history_fold_obeys_the_neutral_law() {
    use semio_framework_value::ErasedSnapshotRetirement;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut plain = fold_log();
    plain.edits[0].meta.as_mut().unwrap()[0].op_id = Some("m".into());
    plain.transitions.clear();
    let mut inputs = vec![plain.clone()];
    for status in 0..=2 {
        let mut quarantined = plain.clone();
        quarantined.conflicts.push(HistoryConflict { id: "quarantine".into(), kind: 0, status, actors: vec!["Aé😀".into()], hlt: (1,2,3), edit_ids: Vec::new(), envelopes: vec![hex(law["envelopeCases"][0]["payloadHex"].as_str().unwrap())], messages: vec![HistoryMessage { level: 2, code: "mutation.conflict".into(), message: "Änderung".into(), target: vec!["Aé😀".into()], op_index: Some(0) }] });
        quarantined.conflicts.push(HistoryConflict { id: "degraded".into(), kind: 1, status: 0, actors: vec!["other".into()], hlt: (2,3,4), edit_ids: vec!["edit-b".into()], envelopes: Vec::new(), messages: Vec::new() });
        inputs.push(quarantined);
    }
    for source in inputs {
    let source = std::sync::Arc::new(source);
    let expected = source.fold().unwrap();
    for work in law["workGrants"].as_array().unwrap() {
        for bytes in law["byteGrants"].as_array().unwrap() {
            let mut job = HistoryLog::fold_job(source.clone(), protocol::HistoryShape::Document);
            let mut completed = 0;
            let mut reached = false;
            for turn in 0..100000 {
                match job.step(job.next_step_grant(work.as_u64().unwrap() as usize, bytes.as_u64().unwrap() as usize).unwrap(),bytes.as_u64().unwrap() as usize, &mut || false).unwrap() {
                    protocol::HistoryFoldJobStep::Pending { completed: next, progress, .. } => { assert!(next >= completed && next-completed <= work.as_u64().unwrap()); if turn % 20000 == 19999 {  } completed = next; },
                    protocol::HistoryFoldJobStep::Ready { value: (fold, transitions, replay_order, conflicts), .. } => {
                        assert_eq!(conflicts.len(), source.conflicts.len());
                        if !conflicts.is_empty() {
                            assert_eq!(conflicts[0].actors[0].0.as_str(), "Aé😀");
                            assert_eq!(conflicts[0].messages[0].message, "Änderung");
                            assert_eq!(conflicts[0].messages[0].target, vec!["Aé😀"]);
                            let protocol::ConflictKind::Quarantined { envelopes } = &conflicts[0].kind else { panic!("quarantine kind preserved") };
                            assert_eq!(envelopes[0].diff.payload, vec![0xaa,0xbb,0xcc]);
                            assert!(matches!(&conflicts[1].kind, protocol::ConflictKind::Degraded { edit_ids } if edit_ids == &vec!["edit-b".to_string()]));
                        } assert_eq!(replay_order, fold.applied); assert_eq!(fold, expected); assert_eq!(transitions, source.transitions.iter().map(|transition| transition.to_envelope(&source.doc_id)).collect::<Vec<_>>()); reached = true; break; },
                    protocol::HistoryFoldJobStep::Rejected { .. } => panic!("native history fold: {:?}", job.rejection()),
                }
            }
            assert!(reached && job.terminal_is_empty(), "work grant {work} byte grant {bytes}: reached {reached}, terminal {}", job.terminal_is_empty());
        }
    }
    for stop in law["cancelAt"].as_array().unwrap() {
        let mut job = HistoryLog::fold_job(source.clone(), protocol::HistoryShape::Document);
        for _ in 0..stop.as_u64().unwrap() { let _ = job.step(job.next_step_grant(1, 7).unwrap(),7, &mut || false).unwrap(); }
        let completed = job.completed();
        job.request_cancel();
        let mut closed = false;
        for _ in 0..100000 { if matches!(job.close_step(job.next_step_grant(1, 0).unwrap()), Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(_))) { closed = true; break; } }
        assert!(closed && job.terminal_is_empty());
        assert_eq!(job.completed(), completed);
        assert_eq!(source.fold().unwrap(), expected);
    }
    }
}
