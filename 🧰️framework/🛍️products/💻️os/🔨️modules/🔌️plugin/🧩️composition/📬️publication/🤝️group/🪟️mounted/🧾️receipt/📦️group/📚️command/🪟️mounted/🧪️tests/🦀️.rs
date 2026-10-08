/// 🧪️ Exercises the real mounted history consumer through its public application close ladder.
pub(crate) fn test_mounted_command_history_pages<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let rows = fixture["cases"].as_array().unwrap().last().unwrap()["sequences"].as_array().unwrap();
    assert!(app.command_log.terminal_is_empty());
    for sequence in rows {
        let seq = sequence.as_u64().unwrap();
        let mut action = String::with_capacity(8194);
        action.push_str(&format!("mounted-shell-{seq}"));
        app.record_command(&action, ActionKind::Shell, Some(LocalizedLabel::native("α Command", "α Befehl")), Some(format!("parent-{seq}")), vec![format!("child α-{seq}"), format!("child β-{seq}")], Some(InverseAction { action_id: format!("inverse-{seq}"), args: Some(DslValue::Object(vec![("nested".into(), DslValue::Array(vec![DslValue::Bool(true), DslValue::String(String::with_capacity(8194)), DslValue::Bytes(vec![0, 3, 255])]))])) }));
        assert_eq!(app.command_log.iter().next_back().unwrap().child_edit_ids.len(), 2);
    }
    let actual: Vec<u64> = app.command_log.iter().map(|entry| entry.seq).collect();
    assert_eq!(serde_json::to_value(&actual).unwrap(), serde_json::Value::Array(rows.clone()));
    assert_eq!(app.command_log.iter().rev().map(|entry| entry.seq).collect::<Vec<_>>(), actual.iter().rev().copied().collect::<Vec<_>>());
    let mut physical_turns = 0;
    let mut retained_empty_backings = 0;
    let mut complete = false;
    for _ in 0..100_000 {
        if app.close_owned_stage >= 8 && !app.command_log.terminal_is_empty() {
            let demand = app.command_log.next_close_byte_demand().unwrap();
            retained_empty_backings += usize::from(demand == 8194);
            assert_eq!(app.next_close_byte_demand(), demand);
            let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.close_step(0, demand).unwrap());
            assert_eq!(denied, PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            if demand != 0 {
                let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.close_step(1, demand - 1).unwrap());
                assert_eq!(denied, PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.close_step(1, demand).unwrap());
            assert_eq!(step, PluginCloseStep::Pending { released_items: 1, released_bytes: demand });
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand));
            physical_turns += 1;
        } else {
            let demand = app.next_close_byte_demand();
            assert!(demand <= 262_144);
            if app.close_step(1, demand.max(4096)).unwrap() == PluginCloseStep::Complete { complete = true; break; }
        }
    }
    assert!(complete && app.close_terminal_is_empty());
    assert!(app.command_log.terminal_is_empty());
    assert!(physical_turns > 17);
    assert_eq!(retained_empty_backings, 17);
    eprintln!("[DEBUG] mounted original history17 records exactseq/childIDs/fullLocale/nestedInverse; public selected command retirement {} turns paid original whole allocations; zero/onebelow preserves; terminal empty", physical_turns);
}

/// 🧪️ Verifies actual held document IDs, shell retention and funded discarded owners through app maintenance.
pub(crate) fn test_mounted_command_history_replacement<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>) {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧹️pruning/🧫️fixtures/🔣️.json")).unwrap();
    let row = &law["cases"][1];
    let held = app.store.envelope().vcs.edits.last().unwrap().id.clone();
    assert!(app.command_log.terminal_is_empty());
    for original in row["rows"].as_array().unwrap() {
        let edit = original["editId"].as_str().map(|id| if id == "held" { held.clone() } else { id.to_owned() });
        app.push_log_entry(CommandLogAppend { action_id: "mounted-replacement", label: LocalizedLabel::native("Command", "Befehl"), kind: ActionKind::Shell, edit_id: edit, transition_id: None, timestamp: Some("original".into()), inverse: Some(InverseAction { action_id: "original-inverse".into(), args: Some(DslValue::String(String::with_capacity(65536))) }) });
    }
    let before = app.command_log.iter().map(|entry| entry.seq).collect::<Vec<_>>();
    let original_pointers = app.command_log.iter().map(|entry| entry.action_id.as_ptr()).collect::<Vec<_>>();
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.retire_displaced_document_rows());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let mut complete = false;
    for _ in 0..100_000 {
        if !app.command_prune_requested() && app.pending_command_prune.is_none() { complete = true; break; }
        assert_eq!(app.command_log.iter().map(|entry| entry.seq).collect::<Vec<_>>(), before);
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.maintenance_step(0, 0).unwrap());
        assert_eq!(step, PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.maintenance_step(1, 0).unwrap());
        assert_eq!(step, PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    }
    assert!(complete);
    assert_eq!(serde_json::to_value(app.command_log.iter().map(|entry| entry.seq).collect::<Vec<_>>()).unwrap(), row["keptSequences"]);
    assert_eq!(app.command_log.iter().map(|entry| entry.action_id.as_ptr()).collect::<Vec<_>>(), original_pointers[..2]);
    let mut large_owner_seen = false;
    for _ in 0..100_000 {
        if app.command_log.pruned_terminal_is_empty() { break; }
        let demand = app.command_log.next_pruned_close_byte_demand().unwrap();
        assert_eq!(app.next_maintenance_byte_demand(), demand);
        large_owner_seen |= demand == 65536;
        if demand > 0 {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.maintenance_step(1, demand - 1).unwrap());
            assert_eq!(step, PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.maintenance_step(1, demand).unwrap());
        assert_eq!(step, PluginCloseStep::Pending { released_items: 1, released_bytes: demand });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand));
    }
    assert!(large_owner_seen && app.command_log.pruned_terminal_is_empty());
    app.command_prune_generation += 1;
    app.advance_command_prune_step(1, 0, false).unwrap();
    assert!(app.pending_command_prune.is_some());
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.close_step(1, 0).unwrap());
    assert_eq!(step, PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert!(app.pending_command_prune.is_none() && !app.command_prune_requested());
    crate::app::artifact_app_laws::close_registered_fixture_app(app);
    assert!(app.close_terminal_is_empty());
    eprintln!("[DEBUG] actual mounted replacement preserves held document and pure shell rows; eachpredecisionturn originalpointers/heap0/0; selected65536 physicalextent exact; underfundretains; pendingvisibilitycancelledbeforeappclose");
}
