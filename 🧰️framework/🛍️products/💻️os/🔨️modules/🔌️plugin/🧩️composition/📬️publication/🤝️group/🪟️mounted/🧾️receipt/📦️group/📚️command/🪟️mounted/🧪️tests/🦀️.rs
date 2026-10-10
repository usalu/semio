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
        if app.close_owned_stage >= 8 && !app.command_log.terminal_is_empty() && app.history_view.is_none() && app.cache.is_none() {
            let demand = app.command_log.retirement_demands().unwrap().release_bytes;
            retained_empty_backings += usize::from(demand == 8194);
            let quoted = app.close_retirement_demands(4096).unwrap();
            assert_eq!(quoted.release_bytes, demand);
            let grant = crate::app::plugin_demand_grant(quoted);
            let idle = PluginLifecycleStep::Progress(RetainedCloneProgress::default());
            let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.close_step(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
            assert_eq!(denied, idle);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            if demand != 0 {
                let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.close_step(RetainedCloneGrant { maximum_release_bytes: demand - 1, ..grant }).unwrap());
                assert_eq!(denied, idle);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.close_step(grant).unwrap());
            assert_eq!(step, PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand, ..Default::default() }));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand));
            physical_turns += 1;
        } else {
            let grant = crate::app::retirement_self_grant(|body| app.close_retirement_demands(body), 4096).unwrap();
            assert!(grant.maximum_release_bytes <= 262_144);
            if matches!(app.close_step(grant).unwrap(), PluginLifecycleStep::Complete(_)) { complete = true; break; }
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
        let grant = crate::app::plugin_demand_grant(app.maintenance_retirement_demands(64).unwrap());
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.maintenance_step(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
        assert_eq!(step, PluginLifecycleStep::Progress(RetainedCloneProgress::default()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.maintenance_step(grant).unwrap());
        assert_eq!(step, PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    }
    assert!(complete);
    assert_eq!(serde_json::to_value(app.command_log.iter().map(|entry| entry.seq).collect::<Vec<_>>()).unwrap(), row["keptSequences"]);
    assert_eq!(app.command_log.iter().map(|entry| entry.action_id.as_ptr()).collect::<Vec<_>>(), original_pointers[..2]);
    let mut large_owner_seen = false;
    for _ in 0..100_000 {
        if app.command_log.pruned_terminal_is_empty() { break; }
        let demand = app.command_log.pruned_retirement_demands().unwrap().release_bytes;
        let quoted = app.maintenance_retirement_demands(64).unwrap();
        assert_eq!(quoted.release_bytes, demand);
        let grant = crate::app::plugin_demand_grant(quoted);
        large_owner_seen |= demand == 65536;
        if demand > 0 {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.maintenance_step(RetainedCloneGrant { maximum_release_bytes: demand - 1, ..grant }).unwrap());
            assert_eq!(step, PluginLifecycleStep::Progress(RetainedCloneProgress::default()));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.maintenance_step(grant).unwrap());
        assert_eq!(step, PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand, ..Default::default() }));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demand));
    }
    assert!(large_owner_seen && app.command_log.pruned_terminal_is_empty());
    app.command_prune_generation += 1;
    app.advance_command_prune_step(RetainedCloneGrant { maximum_items: 1, maximum_depth: 1, ..Default::default() }, false).unwrap();
    assert!(app.pending_command_prune.is_some());
    let grant = crate::app::plugin_demand_grant(app.close_retirement_demands(64).unwrap());
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.close_step(grant).unwrap());
    assert_eq!(step, PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert!(app.pending_command_prune.is_none() && !app.command_prune_requested());
    crate::app::artifact_app_laws::close_registered_fixture_app(app);
    assert!(app.close_terminal_is_empty());
    eprintln!("[DEBUG] actual mounted replacement preserves held document and pure shell rows; eachpredecisionturn originalpointers/heap0/0; selected65536 physicalextent exact; underfundretains; pendingvisibilitycancelledbeforeappclose");
}
