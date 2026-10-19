/// ⏪️ Undo, redo and gesture journal laws: the add widget undo and redo, the leaf names and the journal rows of widget, wire, layout and selection gestures and their undo order.

/// ↩️ Exercises the standard `semio_framework_os_kernel::os_store::ArtifactStore<FlowHostSnapshot, FlowMutation>` undo/redo
/// mechanism directly (the same one `FlowHost::undo`/`redo` are built on) — add a widget, undo,
/// confirm it's gone, redo, confirm it's back — in place of the old test's direct assertions on a
/// hand-rolled `Vec<FlowHostSnapshot>` snapshot stack.
#[semio_framework_async_macros::async_test]
async fn undo_redo_add_widget() {
    let mut host = host_with_test_fixture();
    let fixture_before = host.host_snapshot.clone();
    let count_before = fixture_before.widgets.len();
    let id = host.add_widget(r#"{"kind":"inputNote","text":"undo me"}"#, 42.0, 42.0).unwrap();
    assert_eq!(host.host_snapshot.widgets.len(), count_before + 1);

    let operations = host.recorded.last().cloned().expect("add_widget recorded its edit");
    assert_eq!(operations.len(), 2, "add_widget emits its add-widget and change-layout leaves at the gesture site");

    let envelope: FlowEnvelope = create_document_envelope(FLOW_DOCUMENT_SCHEMA, "test", fixture_before, None, &::semio_framework_artifact_flow_flow::io::binary::snapshot::FlowSnapshotPackProducer);
    let mut store = FlowStore::new::<::semio_framework_artifact_flow_flow::io::binary::snapshot::FlowSnapshotPackProducer>(envelope, semio_framework_os_kernel::os_spr::ActorId(semio_framework_os_kernel::os_spr::LOCAL_ACTOR_ID.into())).await.expect("valid flow store fixture");
    semio_framework_os_kernel::os_store::install_funded_member_store_owners(&mut store,&semio_framework_artifact_flow_flow::io::binary::snapshot::FlowSnapshotPackProducer).expect("flow history owners");
    history_identity!(|identity| store.dispatch(ArtifactCommand::Apply { mutations: operations, transaction: None }, &mut identity).await).expect("apply add-widget operations");
    let applied = store.snapshot().expect("projection");
    assert_eq!(applied.widgets.len(), count_before + 1);
    applied.retire_cold();

    history_identity!(|identity| store.dispatch(ArtifactCommand::Undo, &mut identity).await).expect("undo");
    let after_undo = store.snapshot().expect("projection");
    assert_eq!(after_undo.widgets.len(), count_before);
    assert!(!after_undo.widgets.iter().any(|w| widget_id_for(w) == id));
    after_undo.retire_cold();

    history_identity!(|identity| store.dispatch(ArtifactCommand::Redo, &mut identity).await).expect("redo");
    let after_redo = store.snapshot().expect("projection");
    assert!(after_redo.widgets.iter().any(|w| widget_id_for(w) == id));
    after_redo.retire_cold();
    semio_framework_artifact_flow_flow::retire_flow_store_cold(store);
    host.retire_cold();
}

/// 🏷️ The wire names of `leaves`, in application order.
fn names_of(leaves: &[FlowMutation]) -> Vec<&'static str> {
    leaves
        .iter()
        .map(|leaf| match leaf {
            FlowMutation::AddWidget(_) => "addWidget",
            FlowMutation::RemoveWidget(_) => "removeWidget",
            FlowMutation::MoveWidget(_) => "moveWidget",
            FlowMutation::ChangeWidget(_) => "changeWidget",
            FlowMutation::AddSynapse(_) => "addSynapse",
            FlowMutation::RemoveSynapse(_) => "removeSynapse",
            FlowMutation::MoveSynapse(_) => "moveSynapse",
            FlowMutation::ChangeSynapse(_) => "changeSynapse",
            FlowMutation::ChangeLayout(_) => "changeLayout",
        })
        .collect()
}

/// 🏷️ The wire names of the leaves the most recently recorded edit applied.
fn leaf_names(host: &FlowHost) -> Vec<&'static str> {
    names_of(host.recorded.last().expect("an edit was recorded"))
}

fn assert_same_content(actual: &FlowHostSnapshot, expected: &FlowHostSnapshot) {
    assert_eq!(actual.widgets, expected.widgets);
    assert_eq!(actual.synapses, expected.synapses);
    assert_eq!(actual.layout.keys().cloned().collect::<Vec<_>>(), expected.layout.keys().cloned().collect::<Vec<_>>());
}

#[test]
fn add_widget_emits_add_widget_then_change_layout() {
    let mut host = host_with_test_fixture();
    let count = host.host_snapshot.widgets.len();
    let id = host.add_widget(r#"{"kind":"inputNote","text":"leaf"}"#, 10.0, 20.0).unwrap();
    assert_eq!(leaf_names(&host), ["addWidget", "changeLayout"]);
    let leaves = host.recorded.last().unwrap();
    assert!(matches!(&leaves[0], FlowMutation::AddWidget(leaf) if leaf.index as usize == count && widget_id_for(&leaf.widget) == id));
    assert!(matches!(&leaves[1], FlowMutation::ChangeLayout(leaf) if leaf.entries.len() == 1 && leaf.entries[0].id == id && leaf.entries[0].layout == Some(WidgetLayout { x: 10.0, y: 20.0 })));
    host.retire_cold();
}

#[test]
fn remove_widget_emits_its_wires_then_layout_then_the_widget() {
    let mut host = host_with_test_fixture();
    let touching = host.host_snapshot.synapses.iter().filter(|synapse| synapse.from == "add" || synapse.to == "add").count();
    assert!(touching > 0);
    host.remove_widget("add").unwrap();
    let mut expected = vec!["removeSynapse"; touching];
    expected.extend(["changeLayout", "removeWidget"]);
    assert_eq!(leaf_names(&host), expected);
    assert!(matches!(host.recorded.last().unwrap().last(), Some(FlowMutation::RemoveWidget(leaf)) if leaf.id == "add"));
    host.retire_cold();
}

#[test]
fn disconnect_and_connect_emit_one_synapse_leaf_each() {
    let mut host = host_with_test_fixture();
    host.disconnect("s1").unwrap();
    assert_eq!(leaf_names(&host), ["removeSynapse"]);
    host.connect_ports("slider", "number", "add", "b").unwrap();
    assert_eq!(leaf_names(&host), ["addSynapse"]);
    assert!(matches!(&host.recorded.last().unwrap()[0], FlowMutation::AddSynapse(leaf) if leaf.synapse.from == "slider" && leaf.synapse.to == "add" && leaf.synapse.to_port == "b"));
    host.retire_cold();
}

#[test]
fn layout_and_param_gestures_emit_change_layout_and_change_widget() {
    let mut host = host_with_test_fixture();
    host.make_space("slider", 40.0, 0.0).unwrap();
    assert_eq!(leaf_names(&host), ["changeLayout"]);
    host.set_slider_value("slider", 4.0);
    assert_eq!(leaf_names(&host), ["changeWidget"]);
    assert!(matches!(&host.recorded.last().unwrap()[0], FlowMutation::ChangeWidget(leaf) if leaf.id == "slider" && matches!(leaf.widget, Widget::InputSlider { value, .. } if value == 4.0)));
    assert!(host.take_history_fault().is_none());
    host.retire_cold();
}

#[test]
fn delete_selection_emits_wires_layout_and_widgets() {
    let mut host = host_with_test_fixture();
    host.set_selection(&["preview".to_string()]);
    host.delete_selection().unwrap();
    let names = leaf_names(&host);
    assert_eq!(&names[names.len() - 2..], ["changeLayout", "removeWidget"]);
    assert!(names[..names.len() - 2].iter().all(|name| *name == "removeSynapse"));
    host.retire_cold();
}

#[test]
fn gestures_undo_last_to_first_and_redo_first_to_last() {
    let mut host = host_with_test_fixture();
    let mut states = vec![host.host_snapshot.clone()];
    host.add_widget(r#"{"kind":"inputNote","text":"undo"}"#, 10.0, 20.0).unwrap();
    states.push(host.host_snapshot.clone());
    host.disconnect("s1").unwrap();
    states.push(host.host_snapshot.clone());
    host.connect_ports("slider", "number", "add", "b").unwrap();
    states.push(host.host_snapshot.clone());
    host.set_slider_value("slider", 4.0);
    states.push(host.host_snapshot.clone());
    host.move_widget("slider", 5.0, 6.0).unwrap();
    states.push(host.host_snapshot.clone());
    for expected in states.iter().rev().skip(1) {
        assert!(host.undo());
        assert_same_content(&host.host_snapshot, expected);
    }
    for expected in states.iter().skip(1) {
        assert!(host.redo());
        assert_same_content(&host.host_snapshot, expected);
    }
    states.into_iter().for_each(|state| state.retire_cold());
    host.retire_cold();
}

/// 🧯️ A refused leaf is never swallowed: it surfaces as the history fault, the history restarts at the live content, and no
/// partial row of the refused edit survives.
#[test]
fn a_refused_leaf_surfaces_resets_the_history_and_leaves_no_partial_row() {
    let mut host = host_with_test_fixture();
    host.add_widget(r#"{"kind":"inputNote","text":"kept"}"#, 1.0, 2.0).unwrap();
    assert!(host.can_undo());
    let live = host.host_snapshot.clone();
    host.pending_leaves.push(FlowMutation::AddSynapse(AddSynapse { index: 0, synapse: SynapseSpec { id: "ok".into(), from: "slider".into(), to: "add".into(), from_port: "number".into(), to_port: "a".into() } }));
    host.pending_leaves.push(FlowMutation::RemoveWidget(RemoveWidget { id: "missing".into() }));
    let baseline = host.host_snapshot.clone();
    host.record_history_edit(baseline);
    assert!(matches!(host.take_history_fault(), Some(FlowCoreError::HistoryRefused(_))));
    assert!(host.take_history_fault().is_none());
    assert!(!host.can_undo());
    assert!(host.history_store.as_ref().is_some_and(|store| store.applied_edit_ids().is_empty()));
    assert_same_content(&host.host_snapshot, &live);
    live.retire_cold();
    host.retire_cold();
}

/// 🧯️ A gesture whose leaves the history refuses hands the fault back through its own `Result`.
#[test]
fn a_gesture_whose_leaves_are_refused_returns_the_history_fault() {
    let mut host = host_with_test_fixture();
    host.add_widget(r#"{"kind":"inputNote","text":"first"}"#, 1.0, 2.0).unwrap();
    host.host_snapshot.widgets.push(Widget::InputNote { id: "ghost".into(), text: String::new() });
    let outcome = host.remove_widget("ghost");
    assert!(matches!(outcome, Err(FlowCoreError::HistoryRefused(_))));
    assert!(!host.can_undo());
    host.retire_cold();
}

#[test]
fn move_widget_emits_change_layout_and_undoes() {
    let mut host = host_with_test_fixture();
    let before = host.host_snapshot.layout.get("slider").cloned().expect("slider layout");
    host.move_widget("slider", before.x + 30.0, before.y + 40.0).unwrap();
    assert_eq!(leaf_names(&host), ["changeLayout"]);
    assert!(host.undo());
    let restored = host.host_snapshot.layout.get("slider").cloned().expect("slider layout");
    assert!((restored.x - before.x).abs() < 1e-6 && (restored.y - before.y).abs() < 1e-6);
    host.retire_cold();
}

/// 📥️ A scene that echoes the live content keeps the history; a scene whose content differs is a load and resets it.
#[test]
fn scene_resync_is_an_echo_or_a_history_reset() {
    let mut host = host_with_test_fixture();
    host.add_widget(r#"{"kind":"inputNote","text":"echo"}"#, 1.0, 2.0).unwrap();
    assert!(host.can_undo());
    host.resync_host_snapshot_from_scene(host.host_snapshot.clone());
    assert!(host.can_undo());
    let mut changed = host.host_snapshot.clone();
    changed.widgets.retain(|widget| widget_id_for(widget) != "preview");
    changed.layout.remove("preview");
    changed.synapses.retain(|synapse| synapse.to != "preview" && synapse.from != "preview");
    host.resync_host_snapshot_from_scene(changed);
    assert!(!host.can_undo());
    assert!(host.pending_leaves.is_empty());
    host.retire_cold();
}

/// 🔁️ An edge the dag drops as cyclic when it rebuilds is removed by a leaf of its own, so the history never keeps it.
#[test]
fn a_cyclic_edge_dropped_by_the_dag_is_removed_by_its_own_leaf() {
    let mut host = host_with_test_fixture();
    host.begin_change();
    let cyclic = SynapseSpec { id: "cyc".into(), from: "preview".into(), to: "slider".into(), from_port: "out".into(), to_port: "in".into() };
    let added = FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(host.host_snapshot.synapses.len()).unwrap(), synapse: cyclic.clone() });
    host.host_snapshot.synapses.push(cyclic);
    host.note_leaves([added]);
    host.rebuild_dag();
    host.checked_change().unwrap();
    assert_eq!(leaf_names(&host), ["addSynapse", "removeSynapse"]);
    assert!(host.host_snapshot.synapses.iter().all(|synapse| synapse.id != "cyc"));
    host.retire_cold();
}

/// 📐️ Collapse and explode restore widget AND synapse order exactly on undo, position by position.
#[test]
fn collapse_and_explode_undo_restore_widget_and_synapse_order_exactly() {
    let mut host = host_with_test_fixture();
    let original = host.host_snapshot.clone();
    let cluster = host.collapse_selection(&["slider".to_string(), "add".to_string()]).unwrap();
    let collapsed = host.host_snapshot.clone();
    host.explode_cluster(&cluster).unwrap();
    assert!(host.undo());
    assert_eq!(host.host_snapshot.widgets, collapsed.widgets);
    assert_eq!(host.host_snapshot.synapses, collapsed.synapses);
    assert!(host.undo());
    assert_eq!(host.host_snapshot.widgets, original.widgets);
    assert_eq!(host.host_snapshot.synapses, original.synapses);
    for snapshot in [original, collapsed] {
        snapshot.retire_cold();
    }
    host.retire_cold();
}

/// 🔗️ The dag's own gesture journal is what a pointer gesture records: a connect that displaced the wire feeding its target,
/// a disconnect, a move, a slider release and a port insert each become exactly their leaves.
#[test]
fn pointer_gesture_journal_rows_become_their_concrete_leaves() {
    let mut host = host_with_test_fixture();
    let baseline = host.host_snapshot.clone();
    let s1 = baseline.synapses.iter().find(|synapse| synapse.id == "s1").cloned().expect("s1");
    host.host_snapshot.synapses.retain(|synapse| synapse.id != "s1");
    host.host_snapshot.synapses.push(SynapseSpec { id: "s-new".into(), ..s1.clone() });
    let connect = dag::DagGraphEdit::Connect { source_node_id: s1.from.clone(), source_port_id: s1.from_port.clone(), target_node_id: s1.to.clone(), target_port_id: s1.to_port.clone() };
    let leaves = host.journal_leaves(&[connect], &baseline);
    assert_eq!(names_of(&leaves), ["removeSynapse", "addSynapse"]);
    assert!(matches!(&leaves[0], FlowMutation::RemoveSynapse(leaf) if leaf.id == "s1"));
    assert!(matches!(&leaves[1], FlowMutation::AddSynapse(leaf) if leaf.synapse.id == "s-new" && leaf.index as usize == host.host_snapshot.synapses.len() - 1));
    leaves.into_iter().for_each(retire_flow_mutation);

    let disconnect = dag::DagGraphEdit::Disconnect { synapse_id: "s1".into() };
    let unknown = dag::DagGraphEdit::Disconnect { synapse_id: "never-existed".into() };
    let leaves = host.journal_leaves(&[disconnect, unknown], &baseline);
    assert_eq!(names_of(&leaves), ["removeSynapse"]);
    leaves.into_iter().for_each(retire_flow_mutation);

    let rows = [
        dag::DagGraphEdit::Move { gesture_id: "g".into(), node_ids: vec!["add".into()], dx: 1.0, dy: 2.0 },
        dag::DagGraphEdit::SetSlider { node_id: "slider".into(), value: 3.0 },
        dag::DagGraphEdit::InsertPort { node_id: "add".into(), side: dag::DagPortSide::Input, index: 0 },
    ];
    let leaves = host.journal_leaves(&rows, &baseline);
    assert_eq!(names_of(&leaves), ["changeLayout", "changeWidget"]);
    leaves.into_iter().for_each(retire_flow_mutation);
    baseline.retire_cold();
    host.retire_cold();
}

/// 🖱️ End to end: a node drag and a slider drag, driven through the pointer, are recorded as their one leaf each and undo.
#[test]
fn pointer_drag_and_slider_gestures_record_one_leaf_each() {
    let mut host = host_with_test_fixture();
    host.set_viewport(800, 600, 1.0);
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|node| node.id == "add").expect("add node").clone();
    let (sx, sy) = world_screen_point(&host, node.x, node.y - node.height * 0.25);
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 60.0, sy + 40.0, false, false, false);
    host.pointer_up_screen(sx + 60.0, sy + 40.0, false, false, false);
    assert_eq!(leaf_names(&host), ["changeLayout"]);
    let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 80.0, sy, false, false, false);
    host.pointer_up_screen(sx + 80.0, sy, false, false, false);
    assert_eq!(leaf_names(&host), ["changeWidget"]);
    assert!(host.take_history_fault().is_none());
    assert!(host.undo());
    assert!(host.undo());
    host.retire_cold();
}

#[test]
fn camera_change_does_not_create_undo_step() {
    let mut host = host_with_test_fixture();
    let camera_before = host.host_snapshot.camera.clone();
    host.set_camera(camera_before.x + 50.0, camera_before.y - 30.0, camera_before.zoom * 1.5);
    assert!(!host.can_undo());
    let id = host.add_widget(r#"{"kind":"inputNote","text":"x"}"#, 0.0, 0.0).unwrap();
    assert!(host.can_undo());
    assert!(host.undo());
    assert_eq!(host.host_snapshot.camera.x, camera_before.x + 50.0);
    assert_eq!(host.host_snapshot.camera.y, camera_before.y - 30.0);
    assert!((host.host_snapshot.camera.zoom - camera_before.zoom * 1.5).abs() < 1e-9);
    assert!(!host.host_snapshot.widgets.iter().any(|w| widget_id_for(w) == id));
    host.retire_cold();
}

#[test]
fn replace_fixture_preserves_live_camera() {
    let mut host = host_with_test_fixture();
    host.set_camera(120.0, -45.0, 1.75);
    host.replace_host_snapshot(FlowHostSnapshot {
        schema: "flow.host_snapshot".into(),
        camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![Widget::InputNote { id: "note".into(), text: "hello".into() }],
        synapses: vec![],
        layout: protocol::value::ordered::OrderedMap::new(),
    });
    assert_eq!(host.host_snapshot.camera.x, 120.0);
    assert_eq!(host.host_snapshot.camera.y, -45.0);
    assert!((host.host_snapshot.camera.zoom - 1.75).abs() < 1e-9);
    assert!(host.host_snapshot.widgets.iter().any(|w| widget_id_for(w) == "note"));
    host.retire_cold();
}
