/// 🏁️ Complete route, round trips and close receipts: the hidden scan rejection route, the widget, layout and DSL round trips, the envelope round trip and the original close receipts.

#[test]
fn retained_vcs_complete_route_rejects_hidden_scans_whole_apply_combined_publish_and_partial_grants() {
    let source = read_node("🦀️.rs");
    let start = source.find("//#region 🌊️RetainedVcs").expect("retained route start");
    let end = source.find("//#endregion 🌊️RetainedVcs").expect("retained route end");
    let cursor = &source[start..end];
    let forbidden = [
        "apply_action",
        "flow_vcs_apply_action",
        "flow_vcs_fixture_digest",
        "flow_vcs_dictionary_census",
        "flow_vcs_tree_census",
        "flow_vcs_flow_ui_census",
        "fn publish_cursor",
        ".widgets.insert(",
        ".widgets.remove(",
        ".synapses.insert(",
        ".synapses.remove(",
        ".iter()",
        ".position(",
        ".find(",
        ".filter(",
        ".fold(",
        ".sum(",
        ".clone()",
        "from_fn",
        "for ",
        "while ",
        "serde_json",
    ];
    for write in ["host_snapshot_mut", "snapshot_mut", "std::mem::replace(", ".layout ="] {
        assert!(!cursor.contains(write), "a retained step turns a mutation into a version only through the central applier, never by writing the active version: {write}");
    }
    assert!(cursor.contains("protocol::apply_diff_cold"), "the retained route applies through the central applier");
    for token in forbidden {
        assert!(!cursor.contains(token), "retained cursor admits forbidden whole-action mutation: {token}");
        let mutation = format!("{cursor}\n{token}");
        assert!(mutation.contains(token), "hostile mutation must be observable");
    }
    for required in [
        "flow_vcs_host_snapshot_scalar_digest",
        "flow_vcs_fixture_census",
        "transfer_history_cursor",
        "transfer_surface_cursor",
        "publish_visibility_cursor",
        "publish_page_cursor",
        "history_transferred",
        "surface_transferred",
        "visibility_published",
        "redo_retired > 0",
    ] {
        assert!(cursor.contains(required), "complete retained route lacks bounded contract: {required}");
    }
    assert!(cursor.matches("!grant.permits_work()").count() >= 4, "every mutating control and close must validate the complete grant");
}

fn sample_widget(id: &str) -> Widget {
    Widget::InputNote { id: id.into(), text: format!("note {id}") }
}

fn round_trip(host_snapshot: &FlowHostSnapshot, operation: &FlowMutation) -> FlowHostSnapshot {
    let forward = semio_framework_os_kernel::apply_diff_cold(operation.diff(host_snapshot).diff(), host_snapshot).expect("valid flow diff");
    let inverse = operation.inverse(host_snapshot).expect("valid retained mutation inverse fixture");
    let mut restored = forward.clone();
    for back in inverse.iter().rev() {
        let next = semio_framework_os_kernel::apply_diff_cold(back.diff(&restored).diff(), &restored).expect("valid inverse flow diff");
        restored.retire_cold();
        restored = next;
    }
    assert_eq!(&restored, host_snapshot, "inverse() must exactly restore the pre-operation fixture");
    restored.retire_cold();
    forward
}

#[test]
fn widget_add_patch_remove_round_trip() {
    let host_snapshot = FlowHostSnapshot { widgets: Vec::new(), synapses: Vec::new(), ..FlowHostSnapshot::default() };
    let add = FlowMutation::AddWidget(AddWidget { index: 0, widget: sample_widget("w1") });
    let with_widget = round_trip(&host_snapshot, &add);
    assert_eq!(with_widget.widgets.len(), 1);

    let patch = FlowMutation::ChangeWidget(ChangeWidget { id: "w1".into(), widget: Widget::InputNote { id: "w1".into(), text: "renamed".into() } });
    let patched = round_trip(&with_widget, &patch);
    assert!(matches!(&patched.widgets[0], Widget::InputNote { text, .. } if text == "renamed"));

    let remove = FlowMutation::RemoveWidget(RemoveWidget { id: "w1".into() });
    let removed = round_trip(&patched, &remove);
    assert!(removed.widgets.is_empty());
}

#[test]
fn set_layout_round_trip() {
    let host_snapshot = FlowHostSnapshot::default();
    let operation = FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "slider".into(), layout: Some(WidgetLayout { x: 12.0, y: 34.0 }) }] });
    let next = round_trip(&host_snapshot, &operation);
    assert_eq!(next.layout.get("slider"), Some(&WidgetLayout { x: 12.0, y: 34.0 }));
    next.retire_cold();
    host_snapshot.retire_cold();
}

#[semio_framework_async_macros::async_test]
async fn a_streamed_layout_drag_produces_one_edit() {
    let mut store = FlowStore::new::<::semio_framework_artifact_flow_flow::io::binary::snapshot::FlowSnapshotPackProducer>(create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow", empty_flow_snapshot(), None, &::semio_framework_artifact_flow_flow::io::binary::snapshot::FlowSnapshotPackProducer), semio_framework_os_kernel::os_spr::ActorId(semio_framework_os_kernel::os_spr::LOCAL_ACTOR_ID.into())).await.expect("valid flow store fixture");
    semio_framework_os_kernel::os_store::install_funded_member_store_owners(&mut store,&semio_framework_artifact_flow_flow::io::binary::snapshot::FlowSnapshotPackProducer).expect("flow history owners");
    let transaction = semio_framework_os_kernel::os_spr::TransactionRef { id: "tx-00000000000000f1".into(), tool: "s.flow@1/*#editor#drag".into() };
    for y in [10.0, 20.0, 30.0] {
        history_identity!(|identity| store
            .dispatch(ArtifactCommand::AppendTransaction {
                mutations: vec![FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "slider".into(), layout: Some(WidgetLayout { x: 0.0, y }) }] })],
                transaction: transaction.clone(),
            }, &mut identity)
            .await)
            .expect("drag tick");
    }
    history_identity!(|identity| store.dispatch(ArtifactCommand::CommitTransaction { transaction_id: transaction.id }, &mut identity).await).expect("drag commit");
    assert_eq!(store.envelope().vcs.edits.len(), 1, "a streamed drag must produce exactly one edit");
    let snapshot = store.snapshot().expect("projection");
    assert_eq!(snapshot.layout.get("slider"), Some(&WidgetLayout { x: 0.0, y: 30.0 }));
    snapshot.retire_cold();
    semio_framework_artifact_flow_flow::retire_flow_store_cold(store);
}

/// 📜️ Exercises every `Widget` variant (including `Cluster`'s nested `Tree`/`flow` payload,
/// `Dictionary`-bearing `params`/`preview`, and `BTreeSet` `expanded`) through the `semio_framework_os_kernel::os_dsl::` derive
/// layer — the ground-truth proof for the `🔖️Dsl` region built on top of `FlowHostSnapshot`.
#[test]
fn flow_fixture_dsl_round_trips_including_cluster_widget() {
    let mut host_snapshot = FlowHostSnapshot::default();
    host_snapshot.widgets.push(Widget::Cluster {
        id: "cluster-1".into(),
        name: "Cluster One".into(),
        tree: Tree {
            neurons: vec![
                Neuron { id: "inner-in".into(), kind: "core.number".into(), params: Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(1.0))), tree: None },
                Neuron {
                    id: "inner-add".into(),
                    kind: "math.add".into(),
                    params: Dictionary::new().insert("count", NeuralValue::Atom(Atom::Integer(2))),
                    tree: Some(Box::new(Tree { neurons: vec![Neuron::with_kind("nested", "core.text", Dictionary::new().insert("value", NeuralValue::Atom(Atom::String("deep".into()))))], synapses: vec![] })),
                },
            ],
            synapses: vec![Synapse { id: "inner-s1".into(), from: "inner-in".into(), to: "inner-add".into(), from_port: "number".into(), to_port: "a".into() }],
        },
        flow: FlowGui { camera: CameraJson { x: 1.0, y: 2.0, zoom: 1.5 }, nodes: protocol::value::ordered::OrderedMap::new(), previews: Vec::new() },
    });
    host_snapshot.widgets.push(Widget::OutputPreview { id: "preview2".into(), preview: Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(3.5))), expanded: protocol::value::ordered::OrderedSet::from(["a".to_string(), "b".to_string()]) });
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip_cold(&host_snapshot, FlowHostSnapshot::retire_cold);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence_cold(&host_snapshot, FlowHostSnapshot::retire_cold);
    host_snapshot.retire_cold();
}

/// 📜️ Exercises `semio_framework_os_kernel::os_store::OpText` for every `FlowMutation` variant — the ground-truth proof for the
/// mounted transparent direct-leaf aggregate.
#[test]
fn flow_operation_op_text_round_trips_every_variant() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&FlowMutation::AddWidget(AddWidget { index: 0, widget: sample_widget("w1") }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&FlowMutation::RemoveWidget(RemoveWidget { id: "w1".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&FlowMutation::MoveWidget(MoveWidget { id: "w1".into(), to_index: 2 }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ChangeWidget(ChangeWidget { id: "w1".into(), widget: sample_widget("w1") }));
    let synapse = SynapseSpec { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "y".into() };
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&FlowMutation::AddSynapse(AddSynapse { index: 0, synapse: synapse.clone() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&FlowMutation::RemoveSynapse(RemoveSynapse { id: "s1".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&FlowMutation::MoveSynapse(MoveSynapse { id: "s1".into(), to_index: 1 }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ChangeSynapse(ChangeSynapse { id: "s1".into(), synapse }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: 1.0, y: 2.0 }) }] }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: None }] }));
}

/// 📜️ `semio_framework_os_kernel::os_store::test_support::assert_store_roundtrip` over a real `ArtifactStore<FlowHostSnapshot,
/// FlowMutation>` — proves the `Mutation`/`MutationDiff` (`🔖️Mutations`) and `OpText`
/// (`🔖️OpText`) layers semio_compose_rs correctly end to end, matching every other converted crate's test.
#[test]
fn flow_fixture_satisfies_vcs_test_support_store_roundtrip() {
    let document = FlowHostSnapshot::default();
    let operation = FlowMutation::AddWidget(AddWidget { index: 0, widget: sample_widget("w1") });
    semio_framework_os_kernel::os_store::test_support::assert_store_roundtrip(document, operation);
}

/// 🪪️ The framework fixture owns the canonical `flow.flow` Semio envelope and its default
/// binary pack must round-trip before any Flow-backed app can open its initial store.
#[test]
fn flow_fixture_default_pack_uses_canonical_envelope_and_round_trips() {
    let host_snapshot = FlowHostSnapshot::default();
    assert_eq!(<FlowHostSnapshot as pack::snapshot::ArtifactDsl>::envelope_id(), "flow.flow");
    let encoded = <FlowHostSnapshot as pack::snapshot::ArtifactPack>::encode_pack_with(&host_snapshot, &semio_framework_os_kernel::os_store::PackEncodeOptions::default()).expect("default flow fixture pack");
    let decoded = <FlowHostSnapshot as pack::snapshot::ArtifactPack>::decode_pack_with(&encoded, &semio_framework_os_kernel::os_store::PackDecodeOptions::default()).expect("default flow fixture unpack");
    assert_eq!(decoded, host_snapshot);
}

/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): `FlowMutation`
/// implements `semio_framework_os_kernel::os_spr::OpBinary` through the transparent direct-leaf aggregate and
/// its generic variant codec, so this covers the command envelope without adding another
/// codec.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    let envelope = create_document_envelope("test/v1", "test", FlowHostSnapshot::default(), None, &::semio_framework_artifact_flow_flow::io::binary::snapshot::FlowSnapshotPackProducer);
    let mut store = ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, semio_framework_os_kernel::os_spr::ActorId(semio_framework_os_kernel::os_spr::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    semio_framework_os_kernel::os_store::install_funded_member_store_owners(&mut store,&semio_framework_artifact_flow_flow::io::binary::snapshot::FlowSnapshotPackProducer).expect("flow history owners");
    let operation = FlowMutation::AddWidget(AddWidget { index: 0, widget: sample_widget("w1") });
    history_identity!(|identity| store.dispatch(ArtifactCommand::Apply { mutations: vec![operation], transaction: None }, &mut identity).await).expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<FlowMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<FlowHostSnapshot, FlowMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone()));
    semio_framework_artifact_flow_flow::retire_flow_store_cold(store);
}

/// 📜️ The handcrafted default Flow DSL preserves typed slider content, both synapses, and canonical pack parity.
#[test]
fn default_flow_example_dsl_round_trips() {
    let text = read_node("../📚️examples/🗣️.dsl.semio");
    let host_snapshot = <FlowHostSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("🌊️default.flow must parse");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&host_snapshot);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&host_snapshot);
    host_snapshot.retire_cold();
}

/// 🌿️ Original VCS closes the exact document, action and layout owners under one unchanged full wallet.
#[test]
fn original_vcs_close_preserves_source_and_full_physical_receipt(){
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let law:serde_json::Value=serde_json::from_str(read_node("♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for copy in law["copyGrants"].as_array().unwrap(){
  let (mut vcs,source)=observe(||{
   let mut snapshot=retained_fixture();snapshot.schema=law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap()as usize);
   let mut vcs=FlowRetainedVcs::new(snapshot,13,1,0);
   vcs.retired_actions.push(FlowVcsAction::PatchWidget{id:"original-id".into(),item:Widget::InputNote{id:"original-note".into(),text:"original-action".repeat(37)}}).unwrap();
   vcs.retired_actions.push(FlowVcsAction::InsertSynapse{index:0,item:SynapseSpec{id:"original-link".into(),from:"original-note".into(),to:"preview".into(),from_port:"text".into(),to_port:"text".into()}}).unwrap();vcs
  });
  let source_bytes=source.requested_bytes-source.released_bytes;let original=vcs.document.as_ref().unwrap().host_snapshot().schema.as_ptr();
  let(_,signal)=observe(||vcs.begin_close());assert_eq!((signal.requested_bytes,signal.released_bytes),(0,0));assert_eq!(vcs.document.as_ref().unwrap().host_snapshot().schema.as_ptr(),original);
  let(mut born,mut released,mut turns,mut stalled)=(0,0,0,0);
  while !vcs.terminal_is_empty(){
   turns+=1;assert!(turns<law["maximumTurns"].as_u64().unwrap());let copy=copy.as_u64().unwrap()as usize;
   let (demands,heap)=observe(||vcs.next_close_retired_demands(copy).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   let grant=FlowVcsGrant{retained:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demands.capacity_bytes,maximum_release_bytes:demands.release_bytes,maximum_depth:demands.depth},..retained_grant()};
   let(step,heap)=observe(||vcs.close_retired_step(FlowVcsGrant{retained:RetainedCloneGrant{maximum_items:0,..grant.retained},..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   if demands.capacity_bytes>0{let(step,heap)=observe(||vcs.close_retired_step(FlowVcsGrant{retained:RetainedCloneGrant{maximum_capacity_bytes:demands.capacity_bytes-1,..grant.retained},..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   if demands.release_bytes>0{let(step,heap)=observe(||vcs.close_retired_step(FlowVcsGrant{retained:RetainedCloneGrant{maximum_release_bytes:demands.release_bytes-1,..grant.retained},..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   let(step,heap)=observe(||vcs.close_retired_step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant.retained));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;
   stalled=if progress==RetainedCloneProgress::default(){stalled+1}else{0};assert!(stalled<64,"original VCS fixed positive copy stalled");
  }
  assert_eq!(released,source_bytes+born);let(_,heap)=observe(||drop(vcs));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }
}

/// 🎟️ Original VCS admission and the canonical receipt boundary match the closed neutral corpus.
#[test]
fn original_vcs_close_admission_and_receipts_match_neutral_wallet(){
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let law:serde_json::Value=serde_json::from_str(read_node("♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let grant=|row:&serde_json::Value|RetainedCloneGrant{maximum_items:row["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:row["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:row["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:row["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:row["maximumDepth"].as_u64().unwrap()as usize};
 for row in law["closeAdmissionCases"].as_array().unwrap(){
  let d=&row["demands"];let demands=FlowVcsCloseDemands{copy_bytes:d["copyBytes"].as_u64().unwrap()as usize,capacity_bytes:d["capacityBytes"].as_u64().unwrap()as usize,release_bytes:d["releaseBytes"].as_u64().unwrap()as usize,depth:d["depth"].as_u64().unwrap()as usize};
  let(result,heap)=observe(||admit_turn(grant(&row["grant"]),demands));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  let actual=match result{Ok(true)=>"accepted",Ok(false)=>"deferred",Err(failure)=>{assert_eq!(failure.fault,FlowVcsFault::Depth);assert_eq!(failure.retained_progress,Default::default());"refused_depth"}};assert_eq!(actual,row["expected"].as_str().unwrap());
 }
 for row in law["closeReceiptCases"].as_array().unwrap(){
  let p=&row["progress"];let progress=RetainedCloneProgress{copied_items:p["copiedItems"].as_u64().unwrap()as usize,copied_bytes:p["copiedBytes"].as_u64().unwrap()as usize,retained_capacity_bytes:p["retainedCapacityBytes"].as_u64().unwrap()as usize,released_bytes:p["releasedBytes"].as_u64().unwrap()as usize};let step=if row["complete"].as_bool().unwrap(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)};
  let(result,heap)=observe(||semio_framework_value::retained_clone::admit_retained_clone_close(grant(&row["grant"]),step,row["terminal"].as_bool().unwrap(),"original VCS neutral receipt"));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(result.is_err(),row["refused"].as_bool().unwrap());match result{Ok(actual)=>assert_eq!(actual,step),Err(error)=>{assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::InvariantViolated);assert_eq!(error.retained_progress(),progress);}}
 }
}
