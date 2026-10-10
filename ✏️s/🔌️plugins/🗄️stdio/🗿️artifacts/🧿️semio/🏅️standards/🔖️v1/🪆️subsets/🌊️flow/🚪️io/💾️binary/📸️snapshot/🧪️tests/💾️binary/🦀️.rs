//! 🌊️ Public API law for the real retained Flow binary snapshot decoder.
use semio_framework_job::StepContext;
#[global_allocator]
static FLOW_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
use semio_framework_job::{root_cancel_token, Generation, OperationId, StepBudget};
use semio_framework_os_kernel as store;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::io::binary::snapshot::{SemioFlowSnapshotDecode, SemioFlowSnapshotDecodeStep};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, FlowParam, SemioFlowSnapshot};
use semio_s_artifact_stdio_semio::{create_semio_member, SemioMembers, SemioMembersOpen};
use store::{ArtifactPack, OwnedSchemaDecodeCredits, OwnedSchemaDecodePage, OwnedSchemaDecodePages};
use store::{ErasedSnapshotRetirement, MemberFactory, MemberOpenDiagnostic, MemberOpenOperation, MemberOpenRequest, MemberOpenStep, MemberSnapshotOpenOperation, OwnerRef, SpaceMember};

fn request(bytes: &[u8], subset: &str) -> MemberOpenRequest {
    request_with_admission(bytes, subset, true)
}

fn request_with_admission(bytes: &[u8], subset: &str, admitted: bool) -> MemberOpenRequest {
    let mut input = Vec::new();
    store::pack_rt::write_varint_u64(&mut input, bytes.len() as u64);
    input.extend_from_slice(bytes);
    input.push(83);
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: input.len().div_ceil(4096), maximum_bytes: input.len() }).unwrap();
    for chunk in input.chunks(4096) {
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(chunk).unwrap()).unwrap();
    }
    pages.seal().unwrap();
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "flow-member".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() } };
    let request = MemberOpenRequest::new(OperationId(1), Generation(1), 1000, expected, None, pages, store::ActorId(store::LOCAL_ACTOR_ID.into()));
    if admitted {
        request.admit(1).unwrap_or_else(|_| panic!("neutral request must admit"))
    } else {
        request
    }
}

fn bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

fn retirement_policy() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️original/🔣️.json")).unwrap();
    let value = &fixture["policy"];
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: value["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: value["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: value["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: value["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: value["maximumDepth"].as_u64().unwrap() as usize }
}

fn close(value: &mut dyn ErasedSnapshotRetirement) -> usize { close_with_grants(value, &[7]) }

fn close_with_grants(value: &mut dyn ErasedSnapshotRetirement, grants: &[usize]) -> usize {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
    let policy = retirement_policy();
    if value.terminal_is_empty() { assert!(matches!(value.close_step(RetainedCloneGrant::default()).unwrap(), RetainedCloneStep::Complete(_))); return 0; }
    assert_eq!(value.close_step(RetainedCloneGrant::default()).unwrap().progress(), RetainedCloneProgress::default());
    let mut born = 0;
    let mut freed = 0;
    for turn in 0..100_000 {
        let grant = RetainedCloneGrant { maximum_items: grants[turn % grants.len()], ..policy };
        let (step, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| value.close_step(grant).unwrap());
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert_eq!(physical.requested_bytes, progress.retained_capacity_bytes);
        assert_eq!(physical.released_bytes, progress.released_bytes);
        born += physical.requested_bytes;
        freed += physical.released_bytes;
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(value.terminal_is_empty()); return freed.checked_sub(born).expect("all funded original close frames and original backing released"); }
    }
    panic!("bounded decoder retirement must finish");
}

fn close_owned<T: semio_framework_value::retirement::RetireOwned>(value: T) -> usize {
    let grant = retirement_policy();
    let mut pending = Some(value);
    let mut active = None;
    let mut born = 0;
    let mut freed = 0;
    loop {
        let (step, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| if pending.is_some() { store::artifact_retirement_admit_owned(&mut pending, &mut active, grant).unwrap() } else { store::artifact_retirement_box_close_step(&mut active, grant).unwrap() });
        assert!(step.progress().fits(grant));
        assert_eq!(physical.requested_bytes, step.progress().retained_capacity_bytes);
        assert_eq!(physical.released_bytes, step.progress().released_bytes);
        born += physical.requested_bytes;
        freed += physical.released_bytes;
        if pending.is_none() && active.is_none() { return freed.checked_sub(born).expect("original output backing released after funded frames"); }
    }
}
fn reference_bytes(reference: &semio_framework_artifact_reference::ArtifactRef) -> usize {
    reference.artifact_id.len() + reference.dialect.artifact_kind.len() + reference.dialect.standard.len() + reference.dialect.subset.len()
}

fn request_identity_bytes(expected: &semio_framework_artifact_reference::ArtifactRef, owner: Option<&OwnerRef>) -> usize {
    reference_bytes(expected) + owner.map_or(0, |owner| reference_bytes(&owner.parent) + owner.slot.len() + owner.child_id.len())
}

fn retained_request(bytes: &[u8], expected: semio_framework_artifact_reference::ArtifactRef, owner: Option<OwnerRef>) -> (MemberOpenRequest, usize) {
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: bytes.len().div_ceil(4096), maximum_bytes: bytes.len() }).unwrap();
    for chunk in bytes.chunks(4096) {
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(chunk).unwrap()).unwrap();
    }
    pages.seal().unwrap();
    let retained = bytes.len() + request_identity_bytes(&expected, owner.as_ref());
    let request = MemberOpenRequest::new(OperationId(71), Generation(17), 10_000, expected, owner, pages, store::ActorId(store::LOCAL_ACTOR_ID.into())).admit(1).unwrap_or_else(|_| panic!("complete request authority must admit"));
    (request, retained)
}

fn close_open(open: &mut impl MemberOpenOperation, grants: &[usize]) -> usize {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    let policy = retirement_policy();
    let mut born = 0;
    let mut freed = 0;
    for turn in 0..100_000 {
        if open.terminal_is_empty() { return freed.checked_sub(born).expect("original member close backing exceeds funded cursor frames"); }
        let grant = RetainedCloneGrant { maximum_items: grants[turn % grants.len()], ..policy };
        let (step, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| open.close_step(grant).expect("bounded member-open close"));
        assert!(step.progress().fits(grant));
        assert_eq!(physical.requested_bytes, step.progress().retained_capacity_bytes);
        assert_eq!(physical.released_bytes, step.progress().released_bytes);
        born += physical.requested_bytes;
        freed += physical.released_bytes;
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(open.terminal_is_empty()); assert_eq!(open.terminal_drop_byte_demand(), Some(0)); return freed.checked_sub(born).expect("original member source and paid frames released"); }
    }
    panic!("request-owned open close must converge");
}

fn close_member(member: &mut SemioMembers) {
    for _ in 0..100_000 {
        let grant = retirement_policy();
        let (step, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| member.close_owned_step(grant).expect("bounded member close"));
        assert!(step.progress().fits(grant));
        assert_eq!(physical.requested_bytes, step.progress().retained_capacity_bytes);
        assert_eq!(physical.released_bytes, step.progress().released_bytes);
        if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) { assert!(member.close_owned_terminal_is_empty()); return; }
    }
    panic!("reopened member close must converge");
}
fn begin_member_open(request: MemberOpenRequest) -> SemioMembersOpen {
    let demand = <SemioMembers as MemberFactory>::open_birth_demand(&request).expect("closed Semio member birth query");
    let mut request = Some(request);
    let grant = retirement_policy(); assert!(demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.depth <= grant.maximum_depth);
    match <SemioMembers as MemberFactory>::begin_open(&mut request, grant) {
        Ok(Some(open)) => open,
        rejected => {
            close(request.as_mut().expect("refused exact factory retains its original request"));
            assert!(matches!(rejected, Err(_)));
            panic!("closed Semio member-open declaration must admit exact request");
        }
    }
}

fn expected(value: &serde_json::Value) -> SemioFlowSnapshot {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::PortRef;
    let string = |value: &serde_json::Value| value.as_str().unwrap().to_owned();
    SemioFlowSnapshot {
        schema: string(&value["schema"]),
        nodes: value["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|node| FlowNode {
                id: string(&node["id"]),
                kind: string(&node["kind"]),
                label: string(&node["label"]),
                params: node["params"].as_array().unwrap().iter().map(|parameter| FlowParam { key: string(&parameter["key"]), value: string(&parameter["value"]) }).collect(),
                position: SemioPoint2 { x: node["position"]["x"].as_f64().unwrap(), y: node["position"]["y"].as_f64().unwrap() },
            })
            .collect(),
        edges: value["edges"]
            .as_array()
            .unwrap()
            .iter()
            .map(|edge| FlowEdge {
                id: string(&edge["id"]),
                kind: string(&edge["kind"]),
                from: PortRef { node: string(&edge["from"]["node"]), port: string(&edge["from"]["port"]) },
                to: PortRef { node: string(&edge["to"]["node"]), port: string(&edge["to"]["port"]) },
            })
            .collect(),
    }
}

#[test]
fn semio_flow_retained_snapshot_matches_neutral_wire_and_retains_failures() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for (rows, valid) in [("valid", true), ("invalid", false)] {
        for row in fixture[rows].as_array().unwrap() {
            let wire = bytes(row["hex"].as_str().unwrap());
            for fuel in [1, 2, 17] {
                let input = request(&wire, "flow");
                let retained = input.retained_input_bytes();
                let mut decoder = SemioFlowSnapshotDecode::begin(input).unwrap_or_else(|_| panic!("exact Flow identity"));
                let mut sequence = 0;
                let mut zero = StepContext::new(OperationId(1), Generation(1), StepBudget::new(0, 999), root_cancel_token(), || Some(1), &mut sequence);
                assert!(matches!(decoder.step(&mut zero), SemioFlowSnapshotDecodeStep::Pending { consumed_bytes: 0 }));
                assert_eq!(decoder.retained_input_bytes(), retained);
                let outcome = loop {
                    let before = decoder.consumed_bytes();
                    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(fuel, 999), root_cancel_token(), || Some(1), &mut sequence);
                    let outcome = decoder.step(&mut cx);
                    assert!(decoder.consumed_bytes() - before <= fuel as usize);
                    if !matches!(outcome, SemioFlowSnapshotDecodeStep::Pending { .. }) {
                        break outcome;
                    }
                    assert!(decoder.take_ready(&cx).is_none());
                };
                assert_eq!(decoder.retained_input_bytes(), retained);
                let cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
                if valid {
                    assert_eq!(outcome, SemioFlowSnapshotDecodeStep::Ready, "{}", row["id"]);
                    let (snapshot, mut input) = decoder.take_ready(&cx).expect("only confirmed exact EOF can hand off");
                    assert!(decoder.terminal_is_empty());
                    assert_eq!(snapshot, expected(&row["snapshot"]));
                    assert_eq!(snapshot.encode_pack(), wire, "existing encoder and independent fixture agree");
                    close_owned(snapshot);
                    close(&mut input);
                } else {
                    let diagnostic = match row["reason"].as_str().unwrap() {
                        "identity" => MemberOpenDiagnostic::Identity,
                        "capacity" => MemberOpenDiagnostic::Capacity,
                        _ => MemberOpenDiagnostic::Malformed,
                    };
                    assert_eq!(outcome, SemioFlowSnapshotDecodeStep::Rejected(diagnostic), "{}", row["id"]);
                    assert!(decoder.take_ready(&cx).is_none());
                    close(&mut decoder);
                }
            }
        }
    }
    let wire = bytes(fixture["valid"][1]["hex"].as_str().unwrap());
    for cut in 0..=wire.len() {
        let mut decoder = SemioFlowSnapshotDecode::begin(request(&wire, "flow")).unwrap_or_else(|_| panic!("exact Flow identity"));
        let retained = decoder.retained_input_bytes();
        let cancel = root_cancel_token();
        let mut sequence = 0;
        while decoder.consumed_bytes() < cut {
            let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(1, 999), cancel.clone(), || Some(1), &mut sequence);
            assert!(matches!(decoder.step(&mut cx), SemioFlowSnapshotDecodeStep::Pending { .. }));
        }
        cancel.cancel_now();
        let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(1, 999), cancel, || Some(1), &mut sequence);
        assert_eq!(decoder.step(&mut cx), SemioFlowSnapshotDecodeStep::Rejected(MemberOpenDiagnostic::Cancelled));
        assert_eq!(decoder.retained_input_bytes(), retained);
        assert!(decoder.take_ready(&cx).is_none());
        close(&mut decoder);
    }
    for (operation, generation, now, diagnostic) in [(2, 1, Some(1), MemberOpenDiagnostic::Stale), (1, 2, Some(1), MemberOpenDiagnostic::Stale), (1, 1, Some(1000), MemberOpenDiagnostic::Expired), (1, 1, None, MemberOpenDiagnostic::Expired)] {
        let mut decoder = SemioFlowSnapshotDecode::begin(request(&wire, "flow")).unwrap_or_else(|_| panic!("exact Flow identity"));
        let mut sequence = 0;
        let clock: fn() -> Option<u64> = match now {
            Some(1) => || Some(1),
            Some(1000) => || Some(1000),
            None => || None,
            _ => unreachable!(),
        };
        let mut cx = StepContext::new(OperationId(operation), Generation(generation), StepBudget::new(1, 2000), root_cancel_token(), clock, &mut sequence);
        assert_eq!(decoder.step(&mut cx), SemioFlowSnapshotDecodeStep::Rejected(diagnostic));
        close(&mut decoder);
    }
    let input = request(&wire, "text");
    let retained = input.retained_input_bytes();
    let mut rejected = SemioFlowSnapshotDecode::begin(input).err().expect("wrong dialect cannot admit");
    assert_eq!(rejected.request.retained_input_bytes(), retained);
    close(&mut rejected.request);
}

#[test]
fn semio_flow_retained_snapshot_rejects_retired_requests_and_closes_exact_bytes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let lifecycle: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/♻️lifecycle/🔣️.json")).unwrap();
    let wire = bytes(fixture["valid"][1]["hex"].as_str().unwrap());
    for row in lifecycle["admission"].as_array().unwrap() {
        let state = row["state"].as_str().unwrap();
        let (mut input, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request_with_admission(&wire, row["subset"].as_str().unwrap(), state != "unadmitted"));
        let expected_bytes = physical.requested_bytes - physical.released_bytes;
        if state == "closing" {
            let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_depth: 1, ..Default::default() }; let (step, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| input.close_step(grant).unwrap()); assert!(step.progress().fits(grant)); assert_eq!((physical.requested_bytes, physical.released_bytes), (0, 0));
        }
        if state == "retired" {
            assert_eq!(close(&mut input), expected_bytes);
        }
        let retained = input.retained_input_bytes();
        match SemioFlowSnapshotDecode::begin(input) {
            Ok(mut decoder) => {
                assert!(row["reason"].is_null());
                assert_eq!(decoder.consumed_bytes(), 0);
                assert_eq!(close(&mut decoder), expected_bytes);
            }
            Err(mut rejected) => {
                let diagnostic = match row["reason"].as_str().unwrap() {
                    "unsealed" => MemberOpenDiagnostic::Unsealed,
                    "stale" => MemberOpenDiagnostic::Stale,
                    "identity" => MemberOpenDiagnostic::Identity,
                    _ => panic!("closed neutral diagnosis"),
                };
                assert_eq!(rejected.diagnostic, diagnostic, "{}", row["id"]);
                assert_eq!(rejected.request.operation(), OperationId(1));
                assert_eq!(rejected.request.generation(), Generation(1));
                assert_eq!(rejected.request.retained_input_bytes(), if state == "retired" { 0 } else { retained });
                if state != "retired" {
                    assert_eq!(rejected.request.expected().dialect.subset, row["subset"].as_str().unwrap());
                }
                assert_eq!(close(&mut rejected.request), if state == "retired" { 0 } else { expected_bytes });
            }
        }
    }
    let row = &lifecycle["multiPage"];
    let mut large = fixture["valid"].as_array().unwrap().iter().find(|candidate| candidate["id"] == row["source"]).unwrap()["snapshot"].clone();
    large["nodes"][row["nodeIndex"].as_u64().unwrap() as usize]["label"] = serde_json::Value::String(row["labelScalar"].as_str().unwrap().repeat(row["labelRepeats"].as_u64().unwrap() as usize));
    let (large_snapshot, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| expected(&large));
    let snapshot_backing = physical.requested_bytes - physical.released_bytes;
    let large_wire = large_snapshot.encode_pack();
    assert_eq!(large_wire.len(), row["wireBytes"].as_u64().unwrap() as usize);
    assert_eq!(close_owned(large_snapshot), snapshot_backing);
    for grants in lifecycle["retirementItemGrants"].as_array().unwrap() {
        let grants = grants.as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).collect::<Vec<_>>();
        let (mut decoder, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| SemioFlowSnapshotDecode::begin(request(&large_wire, "flow")).unwrap_or_else(|_| panic!("large Flow identity")));
        let mut retained_backing = physical.requested_bytes - physical.released_bytes;
        assert_eq!(decoder.retained_input_bytes(), row["inputBytes"].as_u64().unwrap() as usize);
        assert_eq!(decoder.retained_input_bytes().div_ceil(4096), row["inputPages"].as_u64().unwrap() as usize);
        let cancel = root_cancel_token();
        let mut sequence = 0;
        loop {
            let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(97, 999), cancel.clone(), || Some(1), &mut sequence);
            let (step, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| decoder.step(&mut cx)); assert_eq!(physical.released_bytes, 0); retained_backing += physical.requested_bytes;
            match step {
                SemioFlowSnapshotDecodeStep::Ready => break,
                SemioFlowSnapshotDecodeStep::Pending { .. } => assert!(decoder.take_ready(&cx).is_none()),
                rejected => panic!("multi-page decoder rejected: {rejected:?}"),
            }
        }
        cancel.cancel_now();
        let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(1, 999), cancel, || Some(1), &mut sequence);
        assert_eq!(decoder.step(&mut cx), SemioFlowSnapshotDecodeStep::Rejected(MemberOpenDiagnostic::Cancelled));
        assert!(decoder.take_ready(&cx).is_none());
        assert_eq!(close_with_grants(&mut decoder, &grants), retained_backing);
    }
    eprintln!("[DEBUG] Flow lifecycle: five exact request admission states, two original input pages, actual allocator-matched backing retired under three variable item grants after Ready cancellation; no member publication");
}

#[semio_framework_async_macros::async_test]
async fn semio_member_factory_request_owned_open_admits_only_retained_flow() {
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "flow-member".into(), dialect: dialect.clone() };
    let owner = OwnerRef {
        parent: semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-document".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.parent".into(), standard: "v1".into(), subset: "root".into() } },
        slot: "flow".into(),
        child_id: expected.artifact_id.clone(),
    };
    let snapshot = SemioFlowSnapshot { schema: "stdio.semio.flow".into(), nodes: Vec::new(), edges: Vec::new() };
    let mut created = create_semio_member(&expected.artifact_id, &dialect, &snapshot.encode_pack(), store::ActorId(store::LOCAL_ACTOR_ID.into())).await.expect("real Flow child creation");
    created.set_owner(Some(owner.clone())).await;
    assert_eq!(created.artifact_ref(), Some(expected.clone()));
    assert_eq!(created.owner_ref(), Some(owner.clone()));
    let envelope = created.envelope_pack_bytes().await.expect("real persisted Flow envelope");
    close_member(&mut created);

    let (request, _) = retained_request(&envelope, expected.clone(), Some(owner.clone()));
    let mut open = begin_member_open(request);
    let mut sequence = 0;
    let mut reopened = loop {
        let mut cx = StepContext::new(OperationId(71), Generation(17), StepBudget::new(4096, 999), root_cancel_token(), || Some(1), &mut sequence);
        match open.step(&mut cx, retirement_policy()) {
            MemberOpenStep::Pending(progress) => {
                assert!(progress.completed <= progress.total || progress.total == 0);
            }
            MemberOpenStep::Ready(member) => break member,
            MemberOpenStep::Rejected(diagnostic) => {
                close_open(&mut open, &[1, 7, 4096]);
                panic!("real Flow request-owned open rejected: {diagnostic:?}");
            }
        }
    };
    assert!(open.terminal_is_empty());
    assert!(matches!(open.step(&mut StepContext::new(OperationId(71), Generation(17), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence), retirement_policy()), MemberOpenStep::Rejected(MemberOpenDiagnostic::Stale)));
    assert!(matches!(&reopened, SemioMembers::Flow(_)));
    assert_eq!(reopened.artifact_ref(), Some(expected.clone()));
    assert_eq!(reopened.owner_ref(), Some(owner.clone()));
    assert_eq!(reopened.document_pack_bytes().await.expect("reopened Flow snapshot"), snapshot.encode_pack());
    close_member(&mut reopened);

    let (initial_pack, spr) = store::decode_document_pack_bytes(&envelope).await.expect("real envelope framing");
    let mut persisted_history = store::os_spr::decode_history(&spr, &store::os_spr::DecodeOptions::default()).await.expect("real history decodes");
    persisted_history.transitions.push(store::os_spr::HistoryTransitionRecord {
        id: "persisted-transition".into(),
        observed: None,
        actor: "persisted-actor".into(),
        hlt: (1, 1, 0),
        dependencies: Vec::new(),
        payload: store::os_spr::encode_history_transition(&store::os_spr::HistoryTransition::Revert { mutation_ids: vec![store::os_spr::MutationId("persisted-missing-operation".into())] }),
    });
    let replay_spr = store::os_spr::encode_history(&persisted_history, &store::os_spr::EncodeOptions::default()).await.expect("non-initial history encodes");
    let replay_envelope = store::encode_document_pack_bytes(&initial_pack, &replay_spr).await;
    let (request, retained) = retained_request(&replay_envelope, expected.clone(), Some(owner.clone()));
    let mut replay = begin_member_open(request);
    let mut sequence = 0;
    loop {
        let mut cx = StepContext::new(OperationId(71), Generation(17), StepBudget::new(4096, 999), root_cancel_token(), || Some(1), &mut sequence);
        match replay.step(&mut cx, retirement_policy()) {
            MemberOpenStep::Pending(_) => {}
            MemberOpenStep::Rejected(found) => {
                assert_eq!(found, MemberOpenDiagnostic::Replay);
                break;
            }
            MemberOpenStep::Ready(mut member) => {
                close_member(&mut member);
                panic!("persisted history cannot be discarded as an initial-only open");
            }
        }
    }
    assert!(close_open(&mut replay, &[1, 7, 4096]) >= retained);

    let denied = <SemioMembers as MemberFactory>::OPEN_DECLARATIONS.iter().filter(|declaration| declaration.subset != "flow").collect::<Vec<_>>();
    assert_eq!(denied.len(), 17);
    for declaration in denied {
        let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: owner.child_id.clone(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: declaration.kind.into(), standard: declaration.standard.into(), subset: declaration.subset.into() } };
        for cancelled in [false, true] {
            let (request, exact_retired) = retained_request(&envelope, expected.clone(), Some(owner.clone()));
            let mut denied_open = begin_member_open(request);
            let cancel = root_cancel_token();
            if cancelled {
                cancel.cancel_now();
            }
            let mut sequence = 0;
            let diagnostic = if cancelled { MemberOpenDiagnostic::Cancelled } else { MemberOpenDiagnostic::Decode };
            // Every non-Flow arm now opens through the framework's bounded pack decoder: it may consume
            // the retained input under budget, but a Flow envelope can never publish as another subset.
            let mut rejected = None;
            for _ in 0..100_000 {
                let mut cx = StepContext::new(OperationId(71), Generation(17), StepBudget::new(1, 999), cancel.clone(), || Some(1), &mut sequence);
                match denied_open.step(&mut cx, retirement_policy()) {
                    MemberOpenStep::Rejected(found) => {
                        rejected = Some(found);
                        break;
                    }
                    MemberOpenStep::Pending(_) if cancelled => {
                        close_open(&mut denied_open, &[1, 7, 4096]);
                        panic!("cancelled {} open must fail closed immediately", declaration.subset);
                    }
                    MemberOpenStep::Pending(_) => {}
                    MemberOpenStep::Ready(mut member) => {
                        close_member(&mut member);
                        close_open(&mut denied_open, &[1, 7, 4096]);
                        panic!("a Flow envelope cannot publish as {}", declaration.subset);
                    }
                }
            }
            if rejected != Some(diagnostic) {
                close_open(&mut denied_open, &[1, 7, 4096]);
                panic!("{}: expected {diagnostic:?}, found {rejected:?}", declaration.subset);
            }
            // A decode rejection happens only after the bounded decoder copied the whole framed snapshot,
            // so its exact retirement is the request plus that copy; a cancelled open copied nothing.
            assert!(close_open(&mut denied_open, &[1, 7, 4096]) >= exact_retired, "{}: physically released original request and any actual decoder prefix", declaration.subset);
        }
    }

    for (stage, event, diagnostic) in [
        ("member-open.flow-snapshot", "cancel", MemberOpenDiagnostic::Cancelled),
        ("member-open.history.verify", "cancel", MemberOpenDiagnostic::Cancelled),
        ("member-open.factory.select", "operation", MemberOpenDiagnostic::Stale),
        ("member-open.history.dictionary", "generation", MemberOpenDiagnostic::Stale),
        ("member-open.history.copy", "expired", MemberOpenDiagnostic::Expired),
        ("member-open.history.decode", "close", MemberOpenDiagnostic::Cancelled),
    ] {
        let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: owner.child_id.clone(), dialect: dialect.clone() };
        let (request, retained) = retained_request(&envelope, expected, Some(owner.clone()));
        let mut lifecycle_open = begin_member_open(request);
        let cancel = root_cancel_token();
        let mut sequence = 0;
        let mut observed = false;
        for _ in 0..100_000 {
            let mut cx = StepContext::new(OperationId(71), Generation(17), StepBudget::new(1, 999), cancel.clone(), || Some(1), &mut sequence);
            match lifecycle_open.step(&mut cx, retirement_policy()) {
                MemberOpenStep::Pending(_) => {}
                MemberOpenStep::Rejected(found) => {
                    close_open(&mut lifecycle_open, &[1, 7, 4096]);
                    panic!("lifecycle stage {stage} rejected early: {found:?} at {}", cx.stage());
                }
                MemberOpenStep::Ready(mut member) => {
                    close_member(&mut member);
                    panic!("lifecycle stage published early");
                }
            }
            if cx.stage() == stage {
                observed = true;
                break;
            }
        }
        assert!(observed, "stage {stage} must be observable before authority mutation");
        if event == "close" {
            assert!(close_open(&mut lifecycle_open, &[1, 7, 4096]) >= retained);
            continue;
        }
        if event == "cancel" {
            cancel.cancel_now();
        }
        let operation = if event == "operation" { OperationId(72) } else { OperationId(71) };
        let generation = if event == "generation" { Generation(18) } else { Generation(17) };
        let clock: fn() -> Option<u64> = if event == "expired" { || Some(10_000) } else { || Some(1) };
        let mut cx = StepContext::new(operation, generation, StepBudget::new(1, 11_000), cancel, clock, &mut sequence);
        match lifecycle_open.step(&mut cx, retirement_policy()) {
            MemberOpenStep::Rejected(found) => assert_eq!(found, diagnostic, "{stage}"),
            MemberOpenStep::Pending(_) => panic!("mutated lifecycle authority must fail closed immediately"),
            MemberOpenStep::Ready(mut member) => {
                close_member(&mut member);
                panic!("mutated lifecycle authority cannot publish");
            }
        }
        assert!(close_open(&mut lifecycle_open, &[1, 7, 4096]) >= retained);
    }
    eprintln!("[DEBUG] public request-owned Semio member open: Flow handoffs1; one genuine persisted-history Replay denial; non-Flow arms17 x decode+cancel; six real lifecycle authority fences; publications0; every retained operation terminal-empty");
}
