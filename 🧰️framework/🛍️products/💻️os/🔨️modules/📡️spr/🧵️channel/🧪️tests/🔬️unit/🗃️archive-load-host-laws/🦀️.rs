/// 🗃️ Archive load host laws: the scripted commands and the scripted outcome of the stepped whole-document load.

//#region 🗃️DocumentArchiveLoadHost
/// 🗃️ LAW (`🧫️fixtures/🧫️document-archive-load-host/🔣️.json`): the host half of the stepped whole-document load sends exactly the
/// scripted commands — one admission, polls, at most one cancel, acknowledgements — every later command naming the admission's own
/// sequence, and no other command. It hands every polled status back for the host's progress, and it ends in
/// the scripted outcome. Every scripted status keeps `completed <= total`, monotonic within its case. A load the guest admitted
/// itself (`admittedByGuest`, the media import of a whole document) is driven from its first poll under the next sequence.
#[test]
fn the_document_archive_load_host_sends_the_scripted_commands_and_ends_in_the_scripted_outcome() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️document-archive-load-host/🔣️.json")).unwrap();
    let first = law["firstSequence"].as_u64().unwrap();
    let state = |name: &str| match name {
        "pending" => DocumentArchiveLoadState::Pending,
        "running" => DocumentArchiveLoadState::Running,
        "ready" => DocumentArchiveLoadState::Ready,
        "cancelled" => DocumentArchiveLoadState::Cancelled,
        "fault" => DocumentArchiveLoadState::Fault,
        other => panic!("unknown scripted state {other}"),
    };
    let fault = |answer: &serde_json::Value| answer["fault"].as_str().unwrap_or_default().as_bytes().to_vec();
    let text = |bytes: Vec<u8>| String::from_utf8(bytes).expect("scripted faults are UTF-8");
    for case in law["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let exchanges = case["exchanges"].as_array().unwrap();
        let cancel_before = case["cancel"]["beforeStep"].as_u64();
        let cancel_in_flight = case["cancel"]["whileStepInFlight"].as_u64();
        let admitted_by_guest = case["admittedByGuest"].as_bool().unwrap_or(false);
        let merging = case["merging"].as_bool().unwrap_or(false);
        let scripted = DocumentArchivePack { parent_pack: vec![7], parent_spr: vec![9], members: Vec::new() };
        let mut host = if admitted_by_guest {
            DocumentArchiveLoadHost::admitted(first)
        } else if merging {
            DocumentArchiveLoadHost::merging(scripted)
        } else {
            DocumentArchiveLoadHost::new(scripted)
        };
        let mut seq = if admitted_by_guest { first + 1 } else { first };
        let mut refusal = None;
        let mut statuses: Vec<(u64, u64)> = Vec::new();
        for (index, exchange) in exchanges.iter().enumerate() {
            if cancel_before == Some(index as u64) {
                host.request_cancel();
            }
            let command = match host.step(|| seq) {
                DocumentArchiveLoadStep::Send { seq: stamped, command } => {
                    assert_eq!(stamped, seq, "{name}: step {index} is stamped with the minted sequence");
                    command
                }
                finished => panic!("{name}: step {index} finished early with {finished:?}"),
            };
            let (sends, operation) = match &command {
                AppCommand::LoadDocumentArchive { seq: sent, archive } => {
                    assert_eq!((*sent, archive.parent_pack.as_slice(), archive.parent_spr.as_slice(), archive.members.len()), (seq, &[7u8][..], &[9u8][..], 0), "{name}: the admission carries the archive under its own sequence");
                    ("loadDocumentArchive", first)
                }
                AppCommand::MergeDocumentArchive { seq: sent, archive } => {
                    assert_eq!((*sent, archive.parent_pack.as_slice(), archive.parent_spr.as_slice(), archive.members.len(), merging), (seq, &[7u8][..], &[9u8][..], 0, true), "{name}: only a merge admits with the merge command, under its own sequence");
                    ("mergeDocumentArchive", first)
                }
                AppCommand::PollDocumentArchiveLoad { seq: sent, operation } => {
                    assert_eq!(*sent, seq, "{name}");
                    ("pollDocumentArchiveLoad", *operation)
                }
                AppCommand::CancelDocumentArchiveLoad { seq: sent, operation } => {
                    assert_eq!(*sent, seq, "{name}");
                    ("cancelDocumentArchiveLoad", *operation)
                }
                AppCommand::AcknowledgeDocumentArchiveLoad { seq: sent, operation } => {
                    assert_eq!(*sent, seq, "{name}");
                    ("acknowledgeDocumentArchiveLoad", *operation)
                }
                other => panic!("{name}: step {index} sent {other:?}, which no archive load sends"),
            };
            assert_eq!(sends, exchange["sends"].as_str().unwrap(), "{name}: step {index}");
            assert_eq!((operation, host.operation()), (first, Some(first)), "{name}: step {index} names the admission's own sequence");
            if cancel_in_flight == Some(index as u64) {
                host.request_cancel();
            }
            let answer = &exchange["answer"];
            let frame = match answer["kind"].as_str().unwrap() {
                "done" => AppFrame::Done { in_reply_to: seq },
                "error" => AppFrame::Error { in_reply_to: Some(seq), fault: fault(answer), report: Vec::new() },
                kind => AppFrame::DocumentArchiveLoad {
                    in_reply_to: seq,
                    status: DocumentArchiveLoadStatus {
                        operation: if kind == "foreignStatus" { first + 1_000 } else { first },
                        state: state(answer["state"].as_str().unwrap()),
                        completed: answer["completed"].as_u64().unwrap(),
                        total: answer["total"].as_u64().unwrap(),
                        ahead: answer["ahead"].as_u64().unwrap_or(0),
                        fault: fault(answer),
                    },
                },
            };
            match host.answer(seq, &frame) {
                Ok(Some(status)) => {
                    assert!(matches!(&frame, AppFrame::DocumentArchiveLoad { status: answered, .. } if *answered == status), "{name}: the host hands back the polled status");
                    statuses.push((status.completed, status.total));
                }
                Ok(None) => {}
                Err(DocumentArchiveLoadRefusal::Refused(bytes)) => refusal = Some(serde_json::json!({ "kind": "refused", "fault": text(bytes) })),
                Err(DocumentArchiveLoadRefusal::Unanswered) => refusal = Some(serde_json::json!({ "kind": "unanswered" })),
            }
            assert!(refusal.is_none() || index + 1 == exchanges.len(), "{name}: the load stops at its first refusal");
            seq += 1;
        }
        if cancel_before == Some(exchanges.len() as u64) {
            host.request_cancel();
        }
        let outcome = refusal.unwrap_or_else(|| match host.step(|| panic!("{name}: a finished load mints no sequence")) {
            DocumentArchiveLoadStep::Finished(DocumentArchiveLoadOutcome::Ready) if host.ahead() == 0 => serde_json::json!({ "kind": "ready" }),
            DocumentArchiveLoadStep::Finished(DocumentArchiveLoadOutcome::Ready) => serde_json::json!({ "kind": "ready", "ahead": host.ahead() }),
            DocumentArchiveLoadStep::Finished(DocumentArchiveLoadOutcome::Cancelled) => serde_json::json!({ "kind": "cancelled" }),
            DocumentArchiveLoadStep::Finished(DocumentArchiveLoadOutcome::Fault(bytes)) => serde_json::json!({ "kind": "fault", "fault": text(bytes) }),
            DocumentArchiveLoadStep::Send { command, .. } => panic!("{name}: the host still sends {command:?} after the script ended"),
        });
        assert_eq!(outcome, case["outcome"], "{name}");
        assert!(statuses.iter().all(|(completed, total)| completed <= total) && statuses.windows(2).all(|pair| pair[0].0 <= pair[1].0 && pair[0].1 == pair[1].1), "{name}: scripted progress {statuses:?} is monotonic and bounded");
    }
}
//#endregion 🗃️DocumentArchiveLoadHost
