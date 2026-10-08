fn command_entry_grant(copy: usize, capacity: usize, release: usize) -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: 64 } }
fn command_entry_fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn command_entry_label(row: &serde_json::Value) -> Option<LocalizedLabel> { row["label"].is_object().then(|| LocalizedLabel::from_fn(|term, locale| row["label"][term.as_str()][locale.as_str()].as_str().unwrap().into())) }
fn command_entry_source<'a>(row: &'a serde_json::Value, label: Option<&'a LocalizedLabel>, millis: i64) -> MountedCommandEntrySource<'a> { MountedCommandEntrySource { action_id: row["actionId"].as_str().unwrap(), label, captured_millis: millis, kind: ActionKind::Mutation, count: 1, parent_touched: row["parentTouched"].as_bool().unwrap(), child_edit_count: row["childEditIds"].as_array().unwrap().len() } }
fn command_entry_close(owner: &mut MountedCommandEntry) {
    for _ in 0..ENTRY_FIELDS + 2 {
        if owner.terminal_is_empty() { return; }
        let bytes = owner.next_close_byte_demand();
        let grant = command_entry_grant(0, 0, bytes);
        for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_release_bytes: bytes.saturating_sub(1), ..grant }] {
            if denied.maximum_items != 0 && bytes == 0 { continue; }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(denied));
            assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0));
            assert_eq!(owner.next_close_byte_demand(), bytes);
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant));
        assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    }
    assert!(owner.terminal_is_empty());
}
fn command_entry_advance(owner: &mut MountedCommandEntry, source: MountedCommandEntrySource<'_>, copy: usize) {
    let (capacity, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.next_capacity_byte_demand(source).unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0));
    assert!(capacity <= 1_048_576);
    let grant = command_entry_grant(copy, capacity, 0);
    for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_capacity_bytes: capacity.saturating_sub(1), ..grant }] {
        if denied.maximum_items != 0 && capacity == 0 { continue; }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, denied).unwrap());
        assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0));
    }
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, grant).unwrap());
    assert!(step.progress().fits(grant)); assert!(step.progress().copied_bytes <= copy.min(64));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
}
fn command_entry_retire_record(mut record: CommandLogEntry) {
    for field in 0..8 {
        for _ in 0..16 {
            let bytes = PagedCommandLog::record_demand(&record, field).unwrap();
            let (done, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| PagedCommandLog::record_close_one(&mut record, field));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0,bytes));
            if done { break; }
        }
    }
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(record));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0));
}

#[test]
fn mounted_command_entry_preborn_fields_match_neutral_node_utc_and_original_receipt_ids() {
    let fixture = command_entry_fixture();
    for row in fixture["cases"].as_array().unwrap() {
        let label = command_entry_label(row);
        for time in fixture["timestamps"].as_array().unwrap() {
            for copy in [1,3,7,64,256] {
                let source = command_entry_source(row, label.as_ref(), time["millis"].as_i64().unwrap());
                let (mut owner, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| MountedCommandEntry::new(source).unwrap());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0));
                for _ in 0..2000 { if owner.ready() { break; } command_entry_advance(&mut owner, source, copy); }
                assert!(owner.ready());
                let mut parent = row["parentEditId"].as_str().map(str::to_owned);
                let mut children: Vec<String> = row["childEditIds"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().into()).collect();
                let parent_pointer = parent.as_ref().map(|value| value.as_ptr());
                let children_pointer = children.as_ptr();
                let child_pointer = children[0].as_ptr();
                let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.take(&mut parent, &mut children, 999, RetainedCloneGrant { maximum_items: 0, ..command_entry_grant(0,0,0) }));
                assert!(denied.is_none()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0));
                assert_eq!(children.as_ptr(), children_pointer);
                let mut wrong_children = Vec::new();
                let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.take(&mut parent, &mut wrong_children, 1000, command_entry_grant(0,0,0)));
                assert!(denied.is_none()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0));
                assert_eq!(parent.as_ref().map(|value| value.as_ptr()),parent_pointer);
                if source.parent_touched {
                    let mut absent_parent = None;
                    let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.take(&mut absent_parent, &mut children, 1000, command_entry_grant(0,0,0)));
                    assert!(denied.is_none()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0));
                    assert_eq!(children.as_ptr(),children_pointer);
                }
                let (record, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.take(&mut parent, &mut children, 1001, command_entry_grant(0,0,0)).unwrap());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0));
                assert_eq!(record.seq,1001); assert_eq!(record.action_id,source.action_id); assert_eq!(record.timestamp,time["iso"].as_str().unwrap());
                assert_eq!(record.kind,source.kind); assert_eq!(record.count,source.count); assert!(record.inverse.is_none() && record.transition_id.is_none());
                assert_eq!(record.edit_id.as_ref().map(|value| value.as_ptr()),parent_pointer); assert_eq!(record.child_edit_ids.as_ptr(),children_pointer); assert_eq!(record.child_edit_ids[0].as_ptr(),child_pointer);
                for term in Terminology::ALL { for locale in Locale::ALL { assert_eq!(record.label.resolve(term,locale),source.label.map(|label|label.resolve(term,locale)).unwrap_or(source.action_id)); } }
                assert!(parent.is_none() && children.is_empty() && owner.terminal_is_empty());
                command_entry_retire_record(record);
            }
        }
    }
    for millis in fixture["invalidMillis"].as_array().unwrap() {
        let source = command_entry_source(&fixture["cases"][0],None,millis.as_i64().unwrap());
        let (result,heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| MountedCommandEntry::new(source));
        assert!(matches!(result,Err(MountedCommandEntryRefusal::Timestamp))); assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    }
    println!("[DEBUG] preborn command entry UTC6/labels2/copy5 match independent Node Date/full terminology-locale corpus; wrong-cardinality/parent/zero-item refusals retain exact fields; final seq1001 and original receipt pointers transfer with heap0/0");
}

#[test]
fn mounted_command_entry_large_label_cancel_retains_original_allocation_and_source_identity() {
    let fixture = command_entry_fixture();
    let row = &fixture["cases"][0];
    let mut label = command_entry_label(row).unwrap();
    *label.texts_mut().next().unwrap() = "雪\0".repeat(2048)+"é";
    assert_eq!(label.resolve(Terminology::ALL[0],Locale::ALL[0]).len(),fixture["largeLabelBytes"].as_u64().unwrap()as usize);
    let pointer = label.resolve(Terminology::ALL[0],Locale::ALL[0]).as_ptr();
    let source = command_entry_source(row,Some(&label),-1);
    for stop in fixture["cancelTurns"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as usize).chain([200,1000,9000]) {
        let mut owner = MountedCommandEntry::new(source).unwrap();
        for _ in 0..stop { if owner.ready(){break;} command_entry_advance(&mut owner,source,1); }
        let other_action = source.action_id.to_owned();
        let changed = MountedCommandEntrySource { action_id:&other_action,..source };
        let (result,heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(changed,command_entry_grant(64,1_048_576,0)));
        assert_eq!(result,Err(MountedCommandEntryRefusal::Stale)); assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        if owner.ready() {
            for index in 0..ENTRY_LABELS+1 { assert_eq!(owner.fields[index].as_deref(),Some(source.text(index))); }
            assert_eq!(owner.fields[ENTRY_FIELDS-1].as_deref(),Some("1969-12-31T23:59:59.999Z"));
        }
        command_entry_close(&mut owner);
        assert_eq!(label.resolve(Terminology::ALL[0],Locale::ALL[0]).as_ptr(),pointer);
    }
    println!("[DEBUG] preborn command original8194 UTF8 label retained across ten cancel stops; copy1/whole birth-release and same-length foreign source refusal preserve original pointer");
}
