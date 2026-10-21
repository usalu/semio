/// 🪜️ Retained helpers and ordered layout laws: the retained grant, fixture and drive helpers, the source retirement helpers and the ordered layout undo, redo and cancel laws.

fn retained_grant() -> FlowVcsGrant {
    FlowVcsGrant { retained:semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:u32::MAX as usize,maximum_release_bytes:u32::MAX as usize,maximum_depth:4096}, items: 1, bytes: 256, outputs: 1, events: 1, controls: 1, fuel: 1, now_milliseconds: 1, deadline_milliseconds: 8, interrupted: false }
}

fn rejected_control_grants() -> [FlowVcsGrant; 4] {
    let mut zero_fuel = retained_grant();
    zero_fuel.fuel = 0;
    let mut interrupted = retained_grant();
    interrupted.interrupted = true;
    let mut expired = retained_grant();
    expired.deadline_milliseconds = expired.now_milliseconds;
    let mut over_window = retained_grant();
    over_window.deadline_milliseconds = over_window.now_milliseconds + FLOW_VCS_DEADLINE_MILLISECONDS + 1;
    [zero_fuel, interrupted, expired, over_window]
}

fn retained_fixture() -> FlowHostSnapshot {
    let mut host_snapshot = FlowHostSnapshot::default();
    host_snapshot.widgets.push(Widget::InputNote { id: "source".into(), text: "retained".into() });
    host_snapshot.widgets.push(Widget::OutputPreview { id: "retained-preview".into(), preview: Dictionary::new(), expanded: semio_framework_value_ordered::ordered::OrderedSet::from(["value".into()]) });
    host_snapshot.synapses.push(SynapseSpec { id: "source-preview".into(), from: "source".into(), to: "retained-preview".into(), from_port: "text".into(), to_port: String::new() });
    host_snapshot.layout.insert("source".into(), WidgetLayout { x: 1.0, y: 2.0 });
    host_snapshot.layout.insert("retained-preview".into(), WidgetLayout { x: 4.0, y: 5.0 });
    host_snapshot
}

fn drive_to_preview(session: &mut FlowRetainedVcs, handle: FlowVcsHandle) -> FlowVcsPoll {
    for _ in 0..(FLOW_VCS_MAX_ITEMS * 4 + 32) {
        let event = session.poll(handle, retained_grant()).expect("retained cursor step");
        if matches!(event, FlowVcsPoll::Preview { .. }) {
            return event;
        }
    }
    panic!("retained cursor exceeded its fixed semantic bound")
}

fn publish_and_close(session: &mut FlowRetainedVcs, handle: FlowVcsHandle) -> FlowVcsPage {
    drive_to_preview(session, handle);
    let page = session.take_page(handle).expect("published page");
    session.acknowledge_page(handle, page.sequence).expect("page acknowledgement");
    while !matches!(session.close_operation_step(handle, retained_grant()).expect("published operation close"),RetainedCloneStep::Complete(_)) {}
    page
}

/// 🧹️ Ends a law the way a host ends a session: every operation still open is cancelled or acknowledged and
/// closed, then the session retires to terminal-empty — every Flow owner refuses a bare drop.
fn close_to_terminal(session: &mut FlowRetainedVcs) {
    let grant = retained_grant();
    for slot in 0..FLOW_VCS_MAX_OPERATIONS {
        let Some((handle, stage, leased_sequence)) = session.operations[slot].as_ref().map(|operation| (operation.handle, operation.stage, operation.page.as_ref().filter(|_| operation.page_leased).map(|page| page.sequence))) else { continue };
        match stage {
            FlowVcsStage::Admitted | FlowVcsStage::Ready | FlowVcsStage::PublishReady => session.cancel(handle, grant).expect("cancel the open operation"),
            FlowVcsStage::PageReady => {
                let sequence = leased_sequence.unwrap_or_else(|| session.take_page(handle).expect("take the published page").sequence);
                session.acknowledge_page(handle, sequence).expect("acknowledge the published page");
            }
            FlowVcsStage::Complete | FlowVcsStage::Cancelled | FlowVcsStage::Faulted | FlowVcsStage::Closing => {}
        }
        while !matches!(session.close_operation_step(handle, grant).expect("operation close"),RetainedCloneStep::Complete(_)) {}
    }
    session.begin_close();
    while !matches!(session.close_retired_step(grant).expect("session close"),RetainedCloneStep::Complete(_)) {}
    assert!(session.terminal_is_empty());
}

/// 🧹️ Retires a widget a law's source still holds; a widget's `Dictionary`/`OrderedSet` refuses a bare drop.
fn retire_widget_source(source: &mut FlowVcsSource<Widget>) {
    if let Some(widget) = source.value.take() {
        widget.retire_cold();
    }
}

/// 🧹️ Retires a document a law's source still holds; its widgets and layout refuse a bare drop.
fn retire_snapshot_source(source: &mut FlowVcsSource<FlowHostSnapshot>) {
    if let Some(snapshot) = source.value.take() {
        snapshot.retire_cold();
    }
}

//#region 📍️OrderedLayoutLaws
#[test]
fn retained_vcs_shared_snapshot_readers_retire_without_waiting_on_each_other() {
    let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::parse(read_node("🧪️tests/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let snapshot = Arc::new(<FlowHostSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&fixture["initial"].clone())).unwrap());
    let admission=semio_framework_value::retained_clone::RetainedCloneGrant::one_capacity_turn(FlowSnapshotRetirementFactory.retirement_birth_bytes(&snapshot),1);
    let admit=|snapshot|FlowSnapshotRetirementFactory.retire(snapshot,admission).unwrap_or_else(|(error,_)|panic!("Flow shared reader admission refused: {error}")).0;
    let mut readers=[std::mem::ManuallyDrop::new(admit(Arc::clone(&snapshot))),std::mem::ManuallyDrop::new(admit(snapshot))];
    for reader in &mut readers {
        assert_eq!(reader.close_step(semio_framework_value::retained_clone::RetainedCloneGrant::default()).unwrap().progress(),semio_framework_value::retained_clone::RetainedCloneProgress::default());
        assert!(!reader.terminal_is_empty());
    }
    for _ in 0..4096 {
        for reader in &mut readers {
            if !reader.terminal_is_empty() {
                let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:256,maximum_capacity_bytes:reader.next_capacity_byte_demand(256).unwrap(),maximum_release_bytes:reader.next_release_byte_demand().unwrap(),maximum_depth:reader.next_depth_demand().unwrap()};let step=reader.close_step(grant).unwrap();assert!(step.progress().fits(grant));
            }
        }
        if readers.iter().all(|reader| reader.terminal_is_empty()) {
            for reader in &mut readers {
                unsafe {
                    std::mem::ManuallyDrop::drop(reader);
                }
            }
            return;
        }
    }
    panic!("shared snapshot readers retained each other's final-owner claim");
}

fn close_layout_session(session: &mut FlowRetainedVcs) {
    session.begin_close();
    for _ in 0..4096 {
        if matches!(session.close_retired_step(retained_grant()).expect("layout session retirement"),RetainedCloneStep::Complete(_)) {
            assert!(session.terminal_is_empty());
            return;
        }
    }
    panic!("layout session did not retire within its fixture bound");
}

#[test]
fn retained_vcs_ordered_layout_edits_undo_redo_match_json_oracle() {
    let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::parse(read_node("🧪️tests/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut expected = fixture["initial"]["layout"].clone();
    let mut session = FlowRetainedVcs::new(semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&fixture["initial"].clone())).unwrap(), 1, 0, 0);
    for edit in fixture["edits"].as_array().unwrap() {
        let previous = expected.clone();
        let key = edit["id"].as_str().unwrap();
        if edit["layout"].is_null() {
            expected.as_object_mut().unwrap().remove(key);
        } else {
            expected.as_object_mut().unwrap().insert(key.to_owned(), edit["layout"].clone());
        }
        let mut source = FlowVcsSource::new(<FlowLayoutEntry as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&edit.clone())).unwrap());
        let handle = session.begin_set_layout(session.authority(), &mut source).unwrap();
        publish_and_close(&mut session, handle);
        assert_eq!(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&session.document.as_ref().unwrap().host_snapshot().layout)), expected);
        let undo = session.begin_undo(session.authority()).unwrap();
        publish_and_close(&mut session, undo);
        assert_eq!(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&session.document.as_ref().unwrap().host_snapshot().layout)), previous);
        let redo = session.begin_redo(session.authority()).unwrap();
        publish_and_close(&mut session, redo);
        assert_eq!(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&session.document.as_ref().unwrap().host_snapshot().layout)), expected);
        while !matches!(session.close_retired_step(retained_grant()).unwrap(),RetainedCloneStep::Complete(_)) {}
    }
    close_layout_session(&mut session);
}

#[test]
fn retained_vcs_ordered_layout_cancel_at_each_unpublished_boundary_retires_exactly() {
    let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::parse(read_node("🧪️tests/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for edit in fixture["edits"].as_array().unwrap() {
        for boundary in 0..64 {
            let mut session = FlowRetainedVcs::new(semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&fixture["initial"].clone())).unwrap(), 1, 0, 0);
            let mut source = FlowVcsSource::new(<FlowLayoutEntry as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&edit.clone())).unwrap());
            let handle = session.begin_set_layout(session.authority(), &mut source).unwrap();
            let mut published = false;
            for _ in 0..boundary {
                if matches!(session.poll(handle, retained_grant()).unwrap(), FlowVcsPoll::Preview { .. }) {
                    published = true;
                    break;
                }
            }
            if published {
                let page = session.take_page(handle).unwrap();
                session.acknowledge_page(handle, page.sequence).unwrap();
            } else {
                session.cancel(handle, retained_grant()).unwrap();
            }
            while !matches!(session.close_operation_step(handle, retained_grant()).unwrap(),RetainedCloneStep::Complete(_)) {}
            if !published {
                assert_eq!(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&session.document.as_ref().unwrap().host_snapshot().layout)), fixture["initial"]["layout"], "cancel boundary {boundary}");
                assert_eq!(session.credits(), FlowVcsCredits::default());
            }
            close_layout_session(&mut session);
            if published {
                break;
            }
        }
    }
}
//#endregion 📍️OrderedLayoutLaws
