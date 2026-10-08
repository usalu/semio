//! ✂️ Neutral severity selection and bounded edit-settlement laws.
use super::*;
use super::tests::DemoSnapshot;
use super::fixture_mutations::demo::{AddN, DemoMutation};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("neutral clamping fixture")
}

fn messages(row: &serde_json::Value) -> Vec<crate::os_spr::MutationMessage> {
    (0..row["messageRepeat"].as_u64().unwrap_or(1)).flat_map(|_| row["messages"].as_array().unwrap().iter()).map(|message| crate::os_spr::MutationMessage {
        level: match message["level"].as_str().unwrap() {
            "info" => semio_framework_diagnostic::Severity::Info,
            "warning" => semio_framework_diagnostic::Severity::Warning,
            "error" => semio_framework_diagnostic::Severity::Error,
            "fatal" => semio_framework_diagnostic::Severity::Fatal,
            _ => unreachable!(),
        },
        code: message["code"].as_str().unwrap().to_owned().into(),
        message: message["text"].as_str().unwrap().repeat(message["repeat"].as_u64().unwrap() as usize),
        target: (0..message["targetRepeat"].as_u64().unwrap_or(1)).flat_map(|_| message["target"].as_array().unwrap().iter().map(|part| part.as_str().unwrap().to_string())).collect(),
        op_index: message["opIndex"].as_u64().map(|index| index as u32),
    }).collect()
}

#[test]
fn edit_message_clamp_existing_selection_matches_neutral_corpus() {
    for row in fixture()["cases"].as_array().unwrap() {
        let original = messages(row);
        let mut bounded = original.clone();
        assert_eq!(bound_edit_messages(row["editId"].as_str().unwrap(), &mut bounded), row["expected"]["changed"].as_bool().unwrap(), "{}", row["name"]);
        let mut expected: Vec<_> = row["expected"]["kept"].as_array().unwrap().iter().map(|index| original[index.as_u64().unwrap() as usize].clone()).collect();
        if let Some(bytes) = row["expected"]["truncatedUtf8Bytes"].as_u64() {
            expected[0].target.clear();
            expected[0].message.truncate(bytes as usize);
        }
        let dropped = row["expected"]["dropped"].as_u64().unwrap();
        if dropped > 0 {
            let mut summary = crate::os_spr::MutationMessage::info("mutation.cascade", format!("{dropped} more messages"));
            summary.op_index = row["expected"]["summaryOp"].as_u64().map(|index| index as u32);
            expected.push(summary);
        }
        assert_eq!(bounded, expected, "{}", row["name"]);
    }
}

#[test]
fn edit_replay_checks_deadline_during_message_settlement() {
    let law = fixture();
    let count = law["settlement"]["operations"].as_u64().unwrap() as usize;
    let edit = Edit {
        line: None, id: "bounded-close".into(), actor: Some("actor:alice".into()),
        forwards: (0..count).map(|_| DemoMutation::AddN(AddN { delta: 1 })).collect(),
        inverse: Vec::new().into(), mutation_meta: Vec::new(), verb: None, sequence_number: 0,
        started_at: String::new(), finished_at: None,
    };
    let edits = HashMap::from([(edit.id.clone(), edit)]);
    let mut replay = EditReplay::new(ReplayMode::Report, Arc::new(DemoSnapshot { n: Some(0) }), vec!["bounded-close".into()], 0, "demo/v1", EffectiveSupersessions::new(), &edits).unwrap().with_retirement_factories(Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<DemoSnapshot>::default()), Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<DemoMutation>::default()));
    while replay.operation < count {
        assert!(matches!(replay.step(&edits, &mut || true).unwrap(), ReplayStep::Pending(_)));
    }
    assert!(replay.progress().done < replay.progress().total, "unsettled messages remain visible work");
    assert_eq!(replay.progress().done, law["settlement"]["pendingWork"]["done"].as_u64().unwrap() as u32);
    assert_eq!(replay.progress().total, law["settlement"]["pendingWork"]["total"].as_u64().unwrap() as u32);
    let mut asked = 0;
    let step = replay.step(&edits, &mut || { asked += 1; true }).unwrap();
    assert!(asked > 0, "the edit settlement asks its deadline before completing");
    assert!(matches!(step, ReplayStep::Pending(_)), "settlement returns control before scanning the complete edit");
    replay.cancel();
    super::raw_replay_retirement::tests::close(&mut replay);
}

#[test]
fn edit_replay_clamping_preserves_every_fatal_mutation_status() {
    use super::fixture_mutations::severity::{SetFatalN, SeverityMutation};
    let law = fixture();
    let count = law["settlement"]["operations"].as_u64().unwrap() as usize;
    let edit = Edit {
        line: None, id: "fatal-status".into(), actor: Some("actor:alice".into()),
        forwards: (0..count).map(|_| SeverityMutation::SetFatalN(SetFatalN { n: 7 })).collect(),
        inverse: Vec::new().into(), mutation_meta: Vec::new(), verb: None, sequence_number: 0,
        started_at: String::new(), finished_at: None,
    };
    let edits = HashMap::from([(edit.id.clone(), edit)]);
    let mut replay = EditReplay::new(ReplayMode::Report, Arc::new(DemoSnapshot { n: Some(0) }), vec!["fatal-status".into()], 0, "demo.doc", EffectiveSupersessions::new(), &edits).unwrap();
    while !replay.is_finished() { replay.step(&edits, &mut || true).unwrap(); }
    assert_eq!(replay.progress().done, replay.progress().total);
    assert_eq!(replay.progress().done, law["settlement"]["finishedWork"]["done"].as_u64().unwrap() as u32);
    let result = replay.finish().unwrap();
    assert_eq!(result.report().outcomes.len(), count);
    assert_eq!(result.report().worst, Some(semio_framework_diagnostic::Severity::Fatal));
    assert!(result.report().outcomes.iter().all(|outcome| outcome.worst == Some(semio_framework_diagnostic::Severity::Fatal)));
    assert!(result.report().outcomes.iter().any(|outcome| outcome.messages.is_empty()));
    eprintln!("[DEBUG] all {count} fatal mutation statuses preserved after cooperative message settlement");
}
/// 🧾️ Each original owner pays its complete advertised physical release.
fn retire_bounded(mut child: Box<dyn ErasedSnapshotRetirement>, bytes: usize) {
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    for _ in 0..1_000_000 {
        if child.terminal_is_empty() { return; }
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: child.next_copy_byte_demand().unwrap(), maximum_capacity_bytes: child.next_capacity_byte_demand(4096).unwrap(), maximum_release_bytes: child.next_release_byte_demand().unwrap().max(bytes), maximum_depth: child.next_depth_demand().unwrap() };
        let step = child.close_step(grant).expect("message retirement");
        assert!(step.progress().fits(grant));
    }
    panic!("message retirement did not terminate");
}
fn paged_rows(rows: Vec<crate::os_spr::MutationMessage>) -> crate::os_spr::command::ReplayMessageAccumulator {
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let mut owner = crate::os_spr::command::ReplayMessageAccumulator::default();
    for row in rows { let mut pending = Some(row); while pending.is_some() { let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: if owner.rows().has_reserved_slot() { std::mem::size_of::<crate::os_spr::MutationMessage>() } else { 0 }, maximum_capacity_bytes: owner.next_capacity_byte_demand().unwrap(), maximum_release_bytes: 0, maximum_depth: 1 }; assert!(owner.append(&mut pending, grant).unwrap().fits(grant)); } }
    owner
}
fn close_pages(owner: &mut crate::os_spr::command::ReplayMessageAccumulator) {
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    while !owner.terminal_is_empty() { let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: owner.next_close_copy_byte_demand(), maximum_capacity_bytes: 0, maximum_release_bytes: owner.next_close_release_byte_demand().unwrap(), maximum_depth: 1 }; assert!(owner.close_step(grant).unwrap().fits(grant)); }
}
#[test]
fn edit_message_clamp_cursor_matches_neutral_rows_and_cancels_bounded() {
    use crate::os_spr::command::FinalMessageSelection;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let law = fixture();
    let baseline = DemoSnapshot { n: Some(71) };
    for row in law["cases"].as_array().unwrap() {
        for work in law["workGrants"].as_array().unwrap() {
            for bytes in law["byteGrants"].as_array().unwrap() {
                let work = work.as_u64().unwrap(); let bytes = bytes.as_u64().unwrap() as usize;
                let original = messages(row); let mut expected = original.clone(); bound_edit_messages(row["editId"].as_str().unwrap(), &mut expected);
                let mut actual = paged_rows(original.clone()); let mut cursor = FinalMessageSelection::new(row["editId"].as_str().unwrap().len(), actual.rows().len());
                for turn in 0..1_000_000 {
                    if cursor.is_finished() { break; } let before = cursor.completed_work();
                    for _ in 0..work { let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: cursor.next_copy_byte_demand(&actual), maximum_capacity_bytes: cursor.next_capacity_byte_demand(), maximum_release_bytes: cursor.next_release_byte_demand(&actual).unwrap().max(bytes), maximum_depth: 1 }; assert!(cursor.step(&mut actual, grant).unwrap().fits(grant)); if cursor.is_finished() { break; } }
                    assert!(cursor.completed_work() - before <= work); assert_eq!(baseline.n, Some(71)); assert!(turn < 999_999);
                }
                let selected = cursor.take().unwrap(); assert_eq!(selected, expected, "{}", row["name"]); assert_eq!(cursor.changed(), row["expected"]["changed"].as_bool().unwrap()); assert!(actual.terminal_is_empty()); drop(cursor);
                retire_bounded(Box::new(ArtifactStoreMessageLedgerRetirement::new(String::new(), selected)), bytes);
                for cancel in law["cancelAt"].as_array().unwrap() {
                    let mut actual = paged_rows(original.clone()); let mut cursor = FinalMessageSelection::new(row["editId"].as_str().unwrap().len(), actual.rows().len());
                    for _ in 0..cancel.as_u64().unwrap() { if cursor.is_finished() { break; } let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: cursor.next_copy_byte_demand(&actual), maximum_capacity_bytes: cursor.next_capacity_byte_demand(), maximum_release_bytes: cursor.next_release_byte_demand(&actual).unwrap().max(bytes), maximum_depth: 1 }; assert!(cursor.step(&mut actual, grant).unwrap().fits(grant)); }
                    cursor.begin_close(); while !cursor.terminal_is_empty() { let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: cursor.next_close_copy_byte_demand(), maximum_capacity_bytes: 0, maximum_release_bytes: cursor.next_close_release_byte_demand().max(bytes), maximum_depth: 1 }; assert!(cursor.close_step(grant).fits(grant)); }
                    assert!(cursor.terminal_is_empty()); drop(cursor); close_pages(&mut actual); assert_eq!(baseline.n, Some(71));
                }
            }
        }
    }
    eprintln!("[DEBUG] final paged clamp eight neutral cases, three work seeds, three release seeds and six cancellation points retain exact paid terminal owners");
}
