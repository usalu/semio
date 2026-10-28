//! 🧪️ Replay diagnostics retain source order, unique transferred owners and exact physical closure.
use super::*;
use semio_framework_trace_memory_testing::observe_backing;
fn law() -> serde_json::Value {
    serde_json::from_str(include_str!("🔣️.json")).unwrap()
}
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 65536, maximum_release_bytes: 1048576, maximum_depth: 16 }
}
fn rows(values: &serde_json::Value) -> Vec<protocol::EditMessages> {
    values
        .as_array()
        .unwrap()
        .iter()
        .map(|row| protocol::EditMessages {
            edit_id: row["edit_id"].as_str().unwrap().into(),
            messages: row["messages"].as_array().unwrap().iter().map(|message| protocol::mutation::MutationMessage::fatal("diagnostic.replay", message.as_str().unwrap())).collect(),
        })
        .collect()
}
fn ledger(rows: Vec<protocol::EditMessages>) -> ArtifactEditMessageLedger {
    ArtifactEditMessageLedger::try_from_entries(rows).unwrap_or_else(|_| panic!("valid fixture"))
}
fn plain(ledger: &ArtifactEditMessageLedger) -> serde_json::Value {
    serde_json::Value::Array(ledger.iter().map(|row| serde_json::json!({"edit_id":row.edit_id,"messages":row.messages.iter().map(|message|message.message.as_str()).collect::<Vec<_>>()})).collect())
}
fn checked(action: impl FnOnce() -> Result<RetainedCloneStep, ValueError>, g: RetainedCloneGrant) -> RetainedCloneStep {
    let (step, born, freed) = observe_backing(action);
    let step = step.unwrap();
    assert!(step.progress().fits(g));
    assert_eq!(born, step.progress().retained_capacity_bytes);
    assert_eq!(freed, step.progress().released_bytes);
    step
}
fn close(cursor: &mut OriginalReplayDiagnostics) {
    cursor.begin_close();
    for turn in 0..100000 {
        if cursor.terminal_is_empty() {
            return;
        }
        let d = cursor.close_demands(4096).unwrap();
        let g = RetainedCloneGrant { maximum_copy_bytes: 32768, ..grant() };
        assert!(d.copy_bytes <= g.maximum_copy_bytes);
        checked(|| cursor.close_step(g), g);
        assert!(turn < 99999, "[DEBUG] native diagnostics close failed to progress");
    }
}
fn close_ledger(ledger: ArtifactEditMessageLedger) {
    let mut owner = ArtifactEditMessageLedgerRetirement::new(ledger);
    while !owner.terminal_is_empty() {
        let g = grant();
        checked(|| owner.close_step(g), g);
    }
}
#[test]
fn replay_diagnostics_prepare_is_private_and_moves_selected_originals() {
    let law = law();
    let mut baseline = ledger(rows(&law["baseline"]));
    let before = plain(&baseline);
    let original_b = baseline.get_by_id("b").unwrap().messages[0].message.as_ptr();
    let mut replayed = rows(&law["replayed"]);
    let backing = replayed.as_ptr();
    let moved_id = replayed[1].edit_id.as_ptr();
    let moved_message = replayed[1].messages[0].message.as_ptr();
    let mut cursor = OriginalReplayDiagnostics::default();
    for turn in 0..100000 {
        if cursor.prepared() {
            break;
        }
        let g = grant();
        let d = cursor.demands(&baseline, &replayed, 4096).unwrap();
        for axis in 0..5 {
            let mut denied = g;
            match axis {
                0 => denied.maximum_items = 0,
                1 if d.copy_bytes > 0 => denied.maximum_copy_bytes = d.copy_bytes - 1,
                2 if d.capacity_bytes > 0 => denied.maximum_capacity_bytes = d.capacity_bytes - 1,
                3 if d.release_bytes > 0 => denied.maximum_release_bytes = d.release_bytes - 1,
                4 => denied.maximum_depth = 0,
                _ => continue,
            }
            let (step, born, freed) = observe_backing(|| cursor.advance(&baseline, &mut replayed, denied));
            assert_eq!(step.unwrap().progress(), Default::default());
            assert_eq!((born, freed), (0, 0));
        }
        checked(|| cursor.advance(&baseline, &mut replayed, g), g);
        assert_eq!(plain(&baseline), before);
        assert_eq!(replayed.as_ptr(), backing);
        assert!(turn < 99999, "[DEBUG] native diagnostics prepare stalled");
    }
    assert!(cursor.prepared());
    assert!(replayed.iter().all(|row| row.edit_id.is_empty() && row.messages.is_empty()));
    let d = cursor.swap_demands();
    let g = RetainedCloneGrant { maximum_copy_bytes: 32768, ..grant() };
    assert!(d.copy_bytes <= g.maximum_copy_bytes);
    let (p, born, freed) = observe_backing(|| cursor.swap_prepared(&mut baseline, g).unwrap());
    assert!(p.fits(g));
    assert_eq!((born, freed), (0, 0));
    assert_eq!(plain(&baseline), law["expected"]);
    assert_eq!(baseline.get_by_id("a").unwrap().edit_id.as_ptr(), moved_id);
    assert_eq!(baseline.get_by_id("a").unwrap().messages[0].message.as_ptr(), moved_message);
    assert_ne!(baseline.get_by_id("b").unwrap().messages[0].message.as_ptr(), original_b);
    close(&mut cursor);
    let (_, born, freed) = observe_backing(|| drop(cursor));
    assert_eq!((born, freed), (0, 0));
    close_ledger(baseline);
}
#[test]
fn replay_diagnostics_cancel_preserves_baseline_and_closes_every_partial() {
    let law = law();
    for cutoff in law["cancelTurns"].as_array().unwrap() {
        let baseline = ledger(rows(&law["baseline"]));
        let before = plain(&baseline);
        let mut replayed = rows(&law["replayed"]);
        let backing = replayed.as_ptr();
        let mut cursor = OriginalReplayDiagnostics::default();
        for _ in 0..cutoff.as_u64().unwrap() {
            if cursor.prepared() {
                break;
            }
            let g = grant();
            checked(|| cursor.advance(&baseline, &mut replayed, g), g);
        }
        close(&mut cursor);
        assert_eq!(plain(&baseline), before);
        assert_eq!(replayed.as_ptr(), backing);
        let (_, born, freed) = observe_backing(|| drop(cursor));
        assert_eq!((born, freed), (0, 0));
        close_ledger(baseline);
    }
}
#[test]
fn replay_diagnostics_refuses_changed_sources_and_invalid_authority_without_dropping_rows() {
    let law = law();
    let baseline = ledger(rows(&law["baseline"]));
    let foreign = ledger(rows(&law["baseline"]));
    let mut replayed = rows(&law["replayed"]);
    let mut cursor = OriginalReplayDiagnostics::default();
    checked(|| cursor.advance(&baseline, &mut replayed, grant()), grant());
    let (result, born, freed) = observe_backing(|| cursor.advance(&foreign, &mut replayed, grant()));
    assert!(result.is_err());
    assert_eq!((born, freed), (0, 0));
    close(&mut cursor);
    close_ledger(foreign);
    close_ledger(baseline);
    for kind in 0..4 {
        let baseline = ArtifactEditMessageLedger::empty();
        let mut replayed = rows(&law["replayed"]);
        match kind {
            0 => replayed[1].edit_id = replayed[0].edit_id.clone(),
            1 => replayed[0].edit_id = String::new(),
            2 => replayed[0].edit_id = "x".repeat(ARTIFACT_EDIT_MESSAGE_ID_BYTES + 1),
            _ => replayed[0].messages = vec![protocol::mutation::MutationMessage::fatal("diagnostic.replay", "x".repeat(ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES))],
        }
        let original = replayed[0].edit_id.as_ptr();
        let mut cursor = OriginalReplayDiagnostics::default();
        let mut refused = false;
        for _ in 0..100000 {
            let (result, born, freed) = observe_backing(|| cursor.advance(&baseline, &mut replayed, grant()));
            match result {
                Ok(step) => {
                    assert_eq!(born, step.progress().retained_capacity_bytes);
                    assert_eq!(freed, step.progress().released_bytes);
                }
                Err(_) => {
                    assert_eq!((born, freed), (0, 0));
                    refused = true;
                    break;
                }
            }
        }
        assert!(refused);
        assert_eq!(replayed[0].edit_id.as_ptr(), original);
        close(&mut cursor);
        close_ledger(baseline);
    }
}
#[test]
fn replay_diagnostics_count_refusal_keeps_original_recipient_and_vec_backing(){let baseline=ArtifactEditMessageLedger::empty();let mut rows=(0..=ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY).map(|rank|protocol::EditMessages{edit_id:rank.to_string(),messages:Vec::new()}).collect::<Vec<_>>();let source=rows.as_ptr();let mut cursor=OriginalReplayDiagnostics::default();let(result,born,freed)=observe_backing(||cursor.advance(&baseline,&mut rows,grant()));assert!(result.is_err());assert_eq!((born,freed),(0,0));assert_eq!(source,rows.as_ptr());assert!(rows.iter().enumerate().all(|(rank,row)|row.edit_id==rank.to_string()));close(&mut cursor);close_ledger(baseline);}
