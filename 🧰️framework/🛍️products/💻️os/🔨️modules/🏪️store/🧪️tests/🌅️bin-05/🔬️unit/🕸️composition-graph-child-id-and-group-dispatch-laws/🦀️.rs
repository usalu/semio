// 🕸️ Composition graph, child id and group dispatch laws: the closed dialect graph sync, the composition graph forest and links, the child id minting across replicas, the group atomicity, the compensation, the group undo and redo and the peer group transaction.

//#region 🔖️ReconcileAlternative
//#region 🔖️PackValueFixtures
#[semio_framework_async_macros::async_test]
async fn member_factory_closed_dialect_graph_sync_preserves_prior_state_on_rejection() {
    struct Refs {
        children: Vec<ChildRef>,
        links: Vec<ArtifactLink>,
    }
    impl ArtifactRefs for Refs {
        async fn child_refs(&self) -> Vec<ChildRef> {
            self.children.clone()
        }
        async fn links(&self) -> Vec<ArtifactLink> {
            self.links.clone()
        }
    }
    let child_ref = |slot: &str, child: &str| ChildRef { slot: slot.into(), child_id: child.into(), target: semio_framework_artifact_reference::ArtifactRef { artifact_id: child.into(), dialect: demo_child_dialect() } };
    let link_ref = |target: &str| ArtifactLink { target: semio_framework_artifact_reference::ArtifactRef { artifact_id: target.into(), dialect: demo_child_dialect() }, pin: LinkPin::Head, role: "reference".into() };
    let fixture = member_dialect_fixture();
    let rows = fixture["graphCases"].as_array().unwrap();
    assert_eq!(rows.len(), 7);
    for row in rows {
        let id = row["id"].as_str().unwrap();
        let parent = row["candidate"]["parent"].as_str().unwrap();
        let child = row["candidate"]["child"].as_str().unwrap();
        let slot = row["candidate"]["slot"].as_str().unwrap();
        let mut graph = CompositionGraph::new().await;
        for edge in row["edges"].as_array().unwrap() {
            graph.insert_owns(edge["parent"].as_str().unwrap(), edge["slot"].as_str().unwrap(), edge["child"].as_str().unwrap()).await.unwrap();
        }
        graph.insert_owns(parent, "previous", "retained-prior-child").await.unwrap();
        graph.insert_link(parent, "retained-prior-link").await.unwrap();
        let prior_owns = graph.owns.clone();
        let prior_links = graph.links.clone();
        let prior_generation = graph.owns_generation;
        let refs = Refs { children: vec![child_ref(slot, child)], links: Vec::new() };
        let result = graph.sync_member(parent, &refs).await;
        assert_eq!(result.is_ok(), row["syncAccepted"].as_bool().unwrap(), "{id}");
        if result.is_ok() {
            let mut expected = prior_owns;
            expected.retain(|_, (owner, _)| owner != parent);
            expected.insert(child.into(), (parent.into(), slot.into()));
            assert_eq!(graph.owns, expected, "{id}");
            assert!(graph.links.get(parent).is_none());
        } else {
            assert_eq!(graph.owns, prior_owns, "{id}");
            assert_eq!(graph.links, prior_links, "{id}");
            assert_eq!(graph.owns_generation, prior_generation, "{id}");
        }
    }
    let mut graph = CompositionGraph::new().await;
    graph.insert_owns("parent", "previous", "previous-child").await.unwrap();
    graph.insert_link("cycle-source", "parent").await.unwrap();
    graph.insert_link("parent", "previous-link").await.unwrap();
    let prior_owns = graph.owns.clone();
    let prior_links = graph.links.clone();
    for refs in [Refs { children: vec![child_ref("next", "next-child")], links: vec![link_ref("cycle-source")] }, Refs { children: vec![child_ref("first", "duplicated"), child_ref("second", "duplicated")], links: Vec::new() }] {
        assert!(graph.sync_member("parent", &refs).await.is_err());
        assert_eq!(graph.owns, prior_owns);
        assert_eq!(graph.links, prior_links);
    }
}

#[semio_framework_async_macros::async_test]
async fn composition_graph_owns_forest_rejects_second_owner_and_cycle() {
    let mut graph = CompositionGraph::new().await;
    graph.insert_owns("parent-a", "slot-1", "child-1").await.expect("first owner ok");
    graph.insert_owns("parent-a", "slot-1", "child-1").await.expect("idempotent re-own ok");

    let error = graph.insert_owns("parent-b", "slot-1", "child-1").await.expect_err("second owner must be rejected");
    assert!(error.contains("already owned"), "unexpected message: {error}");

    graph.insert_owns("child-1", "slot-x", "grandchild-1").await.expect("child owns grandchild ok");
    assert!(graph.would_cycle_owns("grandchild-1", "parent-a").await, "parent-a is an ancestor of grandchild-1 via child-1");
    let cycle_error = graph.insert_owns("grandchild-1", "slot-y", "parent-a").await.expect_err("cycle must be rejected");
    assert!(cycle_error.contains("cycle"), "unexpected message: {cycle_error}");

    assert_eq!(graph.owner_of("child-1").await, Some("parent-a"));
    assert_eq!(graph.slot_of("child-1").await, Some("slot-1"));
    graph.remove_owns("child-1").await;
    assert_eq!(graph.owner_of("child-1").await, None);
}

#[semio_framework_async_macros::async_test]
async fn composition_graph_links_reject_cycle_but_allow_converging_dag_edges() {
    let mut graph = CompositionGraph::new().await;
    graph.insert_link("a", "b").await.expect("a->b ok");
    graph.insert_link("b", "c").await.expect("b->c ok");
    graph.insert_link("a", "c").await.expect("a->c ok, a converging (non-cyclic) edge");

    let error = graph.insert_link("c", "a").await.expect_err("closing the loop must be rejected");
    assert!(error.contains("cycle"), "unexpected message: {error}");

    assert_eq!(HashSet::<String>::from_iter(graph.links_from("a").await), HashSet::from(["b".to_string(), "c".to_string()]));
    graph.remove_link("a", "b").await;
    assert_eq!(graph.links_from("a").await, vec!["c".to_string()]);
}

/// 🧬️ Fingerprints one operation list under a fixture caller-owned identity authority.
fn fixture_ops_fingerprint(ops: &[Vec<u8>]) -> Vec<u8> {
    with_fixture_identity!(|identity| identity.encode(|control| operation_list_fingerprint(ops, control))).expect("fixture operation fingerprint")
}

/// 🧬️ Mints one child id under a fixture caller-owned identity authority.
fn fixture_child_id(parent_id: &str, slot: &str, fingerprint: &[u8], ordinal: u32) -> String {
    with_fixture_identity!(|identity| identity.encode(|control| mint_child_id(parent_id, slot, fingerprint, ordinal, control))).expect("fixture child id")
}

#[semio_framework_async_macros::async_test]
async fn mint_child_id_converges_across_two_replicas_and_varies_by_ordinal_and_slot() {
    const CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= CEILING); true };
    let mut control = semio_framework_value::NativeEncodeControl::new(CEILING, &mut observer);
    let parent_id = "parent-1";
    let slot = "mesh-slot";
    let ops: Vec<Vec<u8>> = vec![DemoMutation::SetN(SetN { n: 7 }).encode_op().expect("encode")];
    let fingerprint_replica_1 = semio_framework_os_vcs::io::binary::entity_identity::operation_list_fingerprint(&ops, &mut control).expect("original operation fingerprint caller");
    let fingerprint_replica_2 = semio_framework_os_vcs::io::binary::entity_identity::operation_list_fingerprint(&ops, &mut control).expect("original operation fingerprint caller");
    assert_eq!(fingerprint_replica_1, fingerprint_replica_2, "identical ops must fingerprint identically");

    let id_replica_1 = mint_child_id(parent_id, slot, &fingerprint_replica_1, 0, &mut control).expect("original child identity caller");
    let id_replica_2 = mint_child_id(parent_id, slot, &fingerprint_replica_2, 0, &mut control).expect("original child identity caller");
    assert_eq!(id_replica_1, id_replica_2, "two replicas performing the identical genesis must converge on the identical child id");

    let id_different_ordinal = mint_child_id(parent_id, slot, &fingerprint_replica_1, 1, &mut control).expect("original child identity caller");
    assert_ne!(id_replica_1, id_different_ordinal, "a different ordinal must mint a different id");

    let id_different_slot = mint_child_id(parent_id, "other-slot", &fingerprint_replica_1, 0, &mut control).expect("original child identity caller");
    assert_ne!(id_replica_1, id_different_slot, "a different slot must mint a different id");
    assert!(control.owned_bytes() <= CEILING);
}

/// 🧪️ One fatal preview anywhere ⇒ nothing applied on ANY member, parent included.
///
/// 🎞️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6: an
/// ordinary mutation-level (message-based) rejection now travels as `VcsError::Rejected`,
/// not `ValidationFailed` (reserved for structural failures only — see its own doc
/// comment). The default policy (`Normal`, via `SpaceMember::merge_policy`'s trait
/// default, since neither member's policy is configured here) rejects Fatal.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-atomic-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-atomic-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let good_op = ValidatedMutation::SetN(ValidatedSetN { n: 5 }).encode_op().expect("encode good op");
    let bad_op = ValidatedMutation::SetN(ValidatedSetN { n: -1 }).encode_op().expect("encode bad op");
    let parent_ops = vec![good_op];
    let child_dispatch = ChildDispatch { child: child_ref.clone(), ops: &[bad_op], op_schema: &SchemaId("demo/v1".into()), labels: &[semio_framework_ui_locale::LocalizedLabel::data("bad")] };
    let mut children = [(&mut child_store, child_dispatch)];

    let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default(), &mut identity);
    match result.await {
        Ok(_) => panic!("expected the group dispatch to fail phase-1 validation, but it succeeded"),
        Err(VcsError::Rejected { policy, messages }) => {
            assert_eq!(policy, semio_framework_os_spr::MergePolicy::Normal);
            assert!(messages.iter().any(|message| message.level == semio_framework_diagnostic::Severity::Fatal));
        }
        Err(other) => panic!("expected Rejected, got a different VcsError: {other}"),
    }
    assert!(parent_store.envelope().vcs.edits.is_empty(), "parent must have zero edits after a failed group dispatch");
    assert!(child_store.envelope().vcs.edits.is_empty(), "child must have zero edits after a failed group dispatch");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ TASK 2's ownership-check law: `dispatch_group` refuses to touch a `ChildDispatch`
/// whose claimed parent the coordinator's own `CompositionGraph` does not currently track —
/// zero side effects, same as any other phase-1 failure.
///
/// Deliberately NOT seeding `coordinator.graph_mut().insert_owns(..)` — the graph has no
/// record that `parent_ref` owns `child_ref`.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_rejects_a_child_the_graph_does_not_track_as_owned() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-unowned-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-unowned-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, ValidatedMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let coordinator = CompositionCoordinator::new();

    let op = ValidatedMutation::SetN(ValidatedSetN { n: 1 }).encode_op().expect("encode");
    let child_dispatch = ChildDispatch { child: child_ref.clone(), ops: &[op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, child_dispatch)];

    let mut coordinator = coordinator.await;
    let result = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, Vec::new(), Vec::new(), GroupMeta::default(), &mut identity);
    match result.await {
        Ok(_) => panic!("expected an OwnershipViolation, but the dispatch succeeded"),
        Err(VcsError::OwnershipViolation(_)) => {}
        Err(other) => panic!("expected OwnershipViolation, got a different VcsError: {other}"),
    }
    assert!(child_store.envelope().vcs.edits.is_empty());
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ Directly exercises `CompositionCoordinator::compensate` (private, visible to this
/// nested test module) — the reverse-order rollback `dispatch_group`'s phase 2 falls back to on
/// a late failure. Proves the order (parent first, then children in reverse dispatch order) and
/// that a clean rollback restores every member's pre-group snapshot.
#[semio_framework_async_macros::async_test]
async fn compensate_undoes_applied_members_in_reverse_order() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-comp-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_a_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-comp-a".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
    let child_b_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-comp-b".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply parent");
    let parent_edit_id = parent_store.envelope().vcs.edits.last().expect("parent edit").id.clone();

    let mut child_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_a_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    child_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply a");
    let child_a_edit_id = child_a.envelope().vcs.edits.last().expect("a edit").id.clone();

    let mut child_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_b_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    child_b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }, &mut identity).await.expect("apply b");
    let child_b_edit_id = child_b.envelope().vcs.edits.last().expect("b edit").id.clone();

    let dispatch_a = ChildDispatch { child: child_a_ref.clone(), ops: &[], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let dispatch_b = ChildDispatch { child: child_b_ref.clone(), ops: &[], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_a, dispatch_a), (&mut child_b, dispatch_b)];
    let applied_children = vec![(0usize, child_a_edit_id), (1usize, child_b_edit_id)];

    let report = CompositionCoordinator::compensate(&parent_ref, &mut parent_store, &mut children, &applied_children, Some(&parent_edit_id), &mut identity).await;

    assert!(report.skipped.is_empty(), "every member should have undone cleanly");
    assert_eq!(report.undone.len(), 3);
    assert_eq!(report.undone[0].0.artifact_id, parent_ref.artifact_id, "parent undone first");
    assert_eq!(report.undone[1].0.artifact_id, child_b_ref.artifact_id, "then the LAST-dispatched child");
    assert_eq!(report.undone[2].0.artifact_id, child_a_ref.artifact_id, "then the first-dispatched child");

    assert_eq!(parent_store.snapshot().expect("parent snapshot").n, Some(0), "parent's edit was undone");
    assert_eq!(child_a.snapshot().expect("a snapshot").n, Some(0), "child a's edit was undone");
    assert_eq!(child_b.snapshot().expect("b snapshot").n, Some(0), "child b's edit was undone");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ TASK 2's "if compensation itself fails" law: a member whose own rollback errors is
/// recorded in `GroupUndoReport.skipped` (never panics, never aborts compensating the rest),
/// and `fold_compensation_error` upgrades the original failure into `VcsError::CompensationFailed`
/// carrying both facts.
///
/// Parent has NOTHING applied, so `parent.undo()` deterministically fails with
/// `NothingToUndo` — simulating a member whose own rollback errors mid-compensation.
#[semio_framework_async_macros::async_test]
async fn compensate_reports_skipped_when_a_members_own_undo_fails_and_folds_to_compensation_failed() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-comp-fail-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-comp-fail-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    child_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 4 })], transaction: None }, &mut identity).await.expect("apply child");
    let child_edit_id = child_store.envelope().vcs.edits.last().expect("child edit").id.clone();

    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &[], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];
    let applied_children = vec![(0usize, child_edit_id)];

    let report = CompositionCoordinator::compensate(&parent_ref, &mut parent_store, &mut children, &applied_children, Some("bogus-parent-edit-id"), &mut identity).await;

    assert_eq!(report.skipped.len(), 1, "the parent's own failed undo must be recorded, not panic");
    assert_eq!(report.skipped[0].0.artifact_id, parent_ref.artifact_id);
    assert!(matches!(&report.skipped[0].1, VcsError::NothingToUndo));
    assert_eq!(report.undone.len(), 1, "the child must still be undone despite the parent's rollback failure");
    assert_eq!(child_store.snapshot().expect("child snapshot").n, Some(0));

    let folded = fold_compensation_error(VcsError::ValidationFailed("late failure".into()), report).await;
    assert!(matches!(folded, VcsError::CompensationFailed(_)), "a non-empty skipped list must fold into CompensationFailed, got {folded}");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ TASK 2's "deterministic child id minting across two simulated replicas" law,
/// end-to-end through `dispatch_group` itself (not just the bare `mint_child_id` helper): two
/// independent coordinators/parents dispatching the IDENTICAL genesis converge on the identical
/// minted child id and the identical `invocation_id`.
///
/// 🎯️ Empty array literal: with `dispatch_group`'s `Mp`/`Mc` split, `Mc` no longer has
/// `parent`'s type to piggyback inference off of, so an empty `children` list needs an
/// explicit element type — the same local test wrapper `parent_1` uses.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_mints_genesis_child_ids_deterministically_across_replicas() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-genesis-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let genesis = vec![ChildGenesis { slot: "mesh-slot".into(), dialect: demo_child_dialect(), initial_pack: DemoSnapshot { n: Some(0) }.encode_pack() }];
    let parent_ops: Vec<Vec<u8>> = vec![DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode")];

    let mut parent_1 = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut coordinator_1 = CompositionCoordinator::new().await;
    let mut children_1: [(&mut ArtifactStore<DemoSnapshot, DemoMutation>, ChildDispatch); 0] = [];
    let receipt_1 = coordinator_1.dispatch_group(&parent_ref, &mut parent_1, &mut children_1, parent_ops.clone(), genesis.clone(), GroupMeta::default(), &mut identity).await.expect("replica 1 dispatch");

    let mut parent_2 = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let coordinator_2 = CompositionCoordinator::new();
    let mut children_2: [(&mut ArtifactStore<DemoSnapshot, DemoMutation>, ChildDispatch); 0] = [];
    let receipt_2 = coordinator_2.await.dispatch_group(&parent_ref, &mut parent_2, &mut children_2, parent_ops, genesis, GroupMeta::default(), &mut identity).await.expect("replica 2 dispatch");

    assert_eq!(receipt_1.created_children.len(), 1);
    assert_eq!(receipt_2.created_children.len(), 1);
    assert_eq!(receipt_1.created_children[0].0.artifact_id, receipt_2.created_children[0].0.artifact_id, "two replicas performing the identical genesis must mint the identical child id");
    assert_eq!(receipt_1.invocation_id, receipt_2.invocation_id, "two replicas performing the identical composite gesture must converge on the identical invocation id");

    assert_eq!(parent_1.snapshot().expect("parent snapshot").n, Some(1));
    assert_eq!(coordinator_1.graph().await.owner_of(&receipt_1.created_children[0].0.artifact_id).await, Some(parent_ref.artifact_id.as_str()));
    assert_eq!(receipt_1.member_edits.len(), 1, "only the parent got a real edit here — no existing children were dispatched");
    assert_eq!(receipt_1.member_edits[0].0.artifact_id, parent_ref.artifact_id);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ TASK 2's group-undo law: a member whose tail belongs to a DIFFERENT (foreign)
/// group is skipped, never aborting the rest of the group's undo.
#[semio_framework_async_macros::async_test]
async fn undo_group_skips_a_foreign_tail_member_but_still_undoes_the_rest() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let group_id = "group-xyz";
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-undo-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-undo-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
    let foreign_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "foreign-undo-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply parent");
    parent_store.stamp_tail_group_id(group_id).await.expect("stamp parent");

    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    child_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply child");
    child_store.stamp_tail_group_id(group_id).await.expect("stamp child");

    let mut foreign_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &foreign_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    foreign_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }, &mut identity).await.expect("apply foreign");
    foreign_store.stamp_tail_group_id("some-other-group").await.expect("stamp foreign");

    let mut children = [(&child_ref, &mut child_store), (&foreign_ref, &mut foreign_store)];

    let report = CompositionCoordinator::undo_group(&parent_ref, &mut parent_store, &mut children, group_id, &mut identity).await;

    assert_eq!(report.undone.len(), 2, "parent + child both belong to the group and must be undone");
    assert_eq!(report.skipped.len(), 1, "the foreign member must be skipped, not abort the group");
    assert_eq!(report.skipped[0].0.artifact_id, foreign_ref.artifact_id);
    assert!(matches!(&report.skipped[0].1, VcsError::ForeignEdit(_)));

    assert_eq!(parent_store.snapshot().expect("parent snapshot").n, Some(0), "parent's group edit was undone");
    assert_eq!(child_store.snapshot().expect("child snapshot").n, Some(0), "child's group edit was undone");
    assert_eq!(foreign_store.snapshot().expect("foreign snapshot").n, Some(3), "the foreign member's own edit must be left untouched");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ `redo_group`'s mirror of the same law: a foreign-group member's redo stack is left
/// untouched while a matching member is reapplied.
#[semio_framework_async_macros::async_test]
async fn redo_group_skips_a_foreign_tail_member_but_still_redoes_the_rest() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let group_id = "group-redo-1";
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-redo-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let foreign_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "foreign-redo-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    parent_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply parent");
    parent_store.stamp_tail_group_id(group_id).await.expect("stamp parent");
    parent_store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo parent, seeding its redo stack");

    let mut foreign_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &foreign_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    foreign_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], transaction: None }, &mut identity).await.expect("apply foreign");
    foreign_store.stamp_tail_group_id("some-other-group").await.expect("stamp foreign");
    foreign_store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo foreign, seeding its redo stack");

    let mut children = [(&foreign_ref, &mut foreign_store)];
    let report = CompositionCoordinator::redo_group(&parent_ref, &mut parent_store, &mut children, group_id, &mut identity).await;

    assert_eq!(report.undone.len(), 1, "only the matching-group member is redone");
    assert_eq!(report.undone[0].0.artifact_id, parent_ref.artifact_id);
    assert_eq!(report.skipped.len(), 1);
    assert_eq!(report.skipped[0].0.artifact_id, foreign_ref.artifact_id);

    assert_eq!(parent_store.snapshot().expect("parent snapshot").n, Some(1), "parent's edit was reapplied");
    assert_eq!(foreign_store.snapshot().expect("foreign snapshot").n, Some(0), "the foreign member's redo stack was left untouched");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

//#region 🔖️TransactionPeerTests
/// 🧪️ W1-C's headline `Peer`-relation law (contract-freeze §5): two artifacts with NO
/// ownership relation commit through `dispatch_peer_group` as ONE atomic transaction — both
/// members end up carrying the SAME `MutationMeta.group_id` (the shared minted
/// `invocation_id`), the peer's tail edit is stamped `MutationOrigin::Transaction { initiator }`,
/// and the initiator's own edit stays at its ordinary `Owner` default (it is not foreign to
/// itself). Deliberately does NOT seed any `CompositionGraph::insert_owns` edge — `Peer` never
/// consults `owner_of` at all.
#[semio_framework_async_macros::async_test]
async fn dispatch_peer_group_commits_both_members_with_one_shared_group_id() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let initiator_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-initiator-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let peer_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-member-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut initiator_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &initiator_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut peer_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let coordinator = TransactionCoordinator::new();

    let initiator_ops: Vec<Vec<u8>> = vec![DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode initiator op")];
    let peer_op = DemoMutation::SetN(SetN { n: 2 }).encode_op().expect("encode peer op");
    let peer_dispatch = ChildDispatch { child: peer_ref.clone(), ops: &[peer_op], op_schema: &SchemaId("demo/v1".into()), labels: &[semio_framework_ui_locale::LocalizedLabel::data("peer")] };
    let mut peers = [(&mut peer_store, peer_dispatch)];

    let receipt = coordinator.await.dispatch_peer_group(&initiator_ref, &mut initiator_store, &mut peers, initiator_ops, GroupMeta::default(), &mut identity).await.expect("peer transaction dispatch");

    assert_eq!(receipt.member_edits.len(), 2, "both the initiator and the one peer got a real edit");
    assert_eq!(initiator_store.snapshot().expect("initiator snapshot").n, Some(1));
    assert_eq!(peer_store.snapshot().expect("peer snapshot").n, Some(2));

    let initiator_group_id = initiator_store.tail_group_id().await.expect("initiator tail group id");
    let peer_group_id = peer_store.tail_group_id().await.expect("peer tail group id");
    assert_eq!(initiator_group_id, peer_group_id, "both members share the SAME minted invocation id as their group id");
    assert_eq!(initiator_group_id, receipt.invocation_id);

    let initiator_origin = initiator_store.envelope().vcs.edits.last().expect("initiator edit").mutation_meta.last().expect("initiator meta").origin.clone();
    assert_eq!(initiator_origin, semio_framework_os_spr::MutationOrigin::Owner, "the initiator's own edit is not foreign to itself, so it stays at the ordinary Owner default");

    let peer_origin = peer_store.envelope().vcs.edits.last().expect("peer edit").mutation_meta.last().expect("peer meta").origin.clone();
    match peer_origin {
        semio_framework_os_spr::MutationOrigin::Transaction { initiator } => assert_eq!(initiator.artifact_id, initiator_ref.artifact_id, "the peer's origin names the real initiator"),
        other => panic!("expected MutationOrigin::Transaction on the peer's tail edit, got {other:?}"),
    }
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ W1-C's compensation law for `Peer`: `compensate` is relation-agnostic (see its own
/// doc comment) — exercising it with a two-PEER (no ownership) scenario proves the SAME
/// reverse-order rollback `dispatch_peer_group`'s phase 2 falls back to on a late failure works
/// identically to the `Owned` case `compensate_undoes_applied_members_in_reverse_order` already
/// covers. Peer B is deliberately left with no applied edit at all, modeling "the SECOND
/// member's own dispatch failed" — the exact trigger `dispatch_peer_group`'s real error branch
/// calls `compensate` from, passing only the members that DID get applied (the initiator + peer
/// A) as `applied_children`/`parent_applied`.
#[semio_framework_async_macros::async_test]
async fn compensate_undoes_applied_peer_members_in_reverse_order() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let initiator_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-comp-initiator".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let peer_a_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-comp-a".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };
    let peer_b_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-comp-b".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut initiator_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &initiator_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    initiator_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply initiator");
    let initiator_edit_id = initiator_store.envelope().vcs.edits.last().expect("initiator edit").id.clone();

    let mut peer_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_a_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    peer_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply peer a — the member the transaction already reached");
    let peer_a_edit_id = peer_a.envelope().vcs.edits.last().expect("a edit").id.clone();

    let mut peer_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_b_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let dispatch_a = ChildDispatch { child: peer_a_ref.clone(), ops: &[], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let dispatch_b = ChildDispatch { child: peer_b_ref.clone(), ops: &[], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut peers = [(&mut peer_a, dispatch_a), (&mut peer_b, dispatch_b)];
    let applied_so_far = vec![(0usize, peer_a_edit_id)];

    let report = TransactionCoordinator::compensate(&initiator_ref, &mut initiator_store, &mut peers, &applied_so_far, Some(&initiator_edit_id), &mut identity).await;

    assert!(report.skipped.is_empty(), "every already-applied member should have undone cleanly");
    assert_eq!(report.undone.len(), 2, "the initiator and peer A (the only two that were actually applied) are both rolled back");
    assert_eq!(report.undone[0].0.artifact_id, initiator_ref.artifact_id, "initiator undone first");
    assert_eq!(report.undone[1].0.artifact_id, peer_a_ref.artifact_id, "then the one applied peer");

    assert_eq!(initiator_store.snapshot().expect("initiator snapshot").n, Some(0), "initiator's edit was compensated");
    assert_eq!(peer_a.snapshot().expect("a snapshot").n, Some(0), "peer A's edit was compensated");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ Task 2's group-undo law, exercised through a REAL `Peer` transaction end-to-end:
/// `undo_group` (unmodified — see its own doc comment on being relation-agnostic) reverses BOTH
/// members of a `dispatch_peer_group` group as ONE, using the exact `invocation_id` that call
/// minted, with no code path specific to `Peer` needed.
#[semio_framework_async_macros::async_test]
async fn undo_group_reverses_both_members_of_a_real_peer_transaction() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let initiator_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-undo-initiator".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let peer_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-undo-member".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut initiator_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &initiator_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut peer_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &peer_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let coordinator = TransactionCoordinator::new();

    let initiator_ops = vec![DemoMutation::SetN(SetN { n: 5 }).encode_op().expect("encode initiator op")];
    let peer_op = DemoMutation::SetN(SetN { n: 7 }).encode_op().expect("encode peer op");
    let peer_dispatch = ChildDispatch { child: peer_ref.clone(), ops: &[peer_op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let receipt = {
        let mut peers = [(&mut peer_store, peer_dispatch)];
        coordinator.await.dispatch_peer_group(&initiator_ref, &mut initiator_store, &mut peers, initiator_ops, GroupMeta::default(), &mut identity).await.expect("peer transaction dispatch")
    };
    assert_eq!(initiator_store.snapshot().expect("initiator snapshot").n, Some(5));
    assert_eq!(peer_store.snapshot().expect("peer snapshot").n, Some(7));

    let mut children = [(&peer_ref, &mut peer_store)];
    let report = TransactionCoordinator::undo_group(&initiator_ref, &mut initiator_store, &mut children, &receipt.invocation_id, &mut identity).await;

    assert!(report.skipped.is_empty(), "both real transaction members must belong to the group");
    assert_eq!(report.undone.len(), 2);
    assert_eq!(initiator_store.snapshot().expect("initiator snapshot after undo").n, Some(0));
    assert_eq!(peer_store.snapshot().expect("peer snapshot after undo").n, Some(0));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ `Peer`'s cycle guard law (`MemberRelation::Peer`'s doc comment): a SECOND, separate
/// `dispatch_peer_group` call that would close a link cycle across the coordinator's persisted
/// `Links` graph is rejected — `CompositionGraph::would_cycle_links`, the same primitive
/// `would_cycle_owns` is to `Owned`'s genesis cycle guard. Transaction 1 (A initiates, B is the
/// peer) succeeds and records a `Links` edge A -> B; transaction 2 (B initiates, A is the peer)
/// would close A -> B -> A and is rejected with zero side effects.
#[semio_framework_async_macros::async_test]
async fn dispatch_peer_group_rejects_a_transaction_that_would_close_a_peer_link_cycle() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let artifact_a_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-cycle-a".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let artifact_b_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "peer-cycle-b".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut store_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &artifact_a_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut store_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &artifact_b_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut coordinator = TransactionCoordinator::new().await;

    let op_ab = DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode a->b op");
    let dispatch_ab = ChildDispatch { child: artifact_b_ref.clone(), ops: &[op_ab], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    {
        let mut peers = [(&mut store_b, dispatch_ab)];
        coordinator.dispatch_peer_group(&artifact_a_ref, &mut store_a, &mut peers, Vec::new(), GroupMeta::default(), &mut identity).await.expect("first transaction A -> B");
    }
    assert_eq!(store_b.snapshot().expect("b snapshot").n, Some(1));

    let op_ba = DemoMutation::SetN(SetN { n: 9 }).encode_op().expect("encode b->a op");
    let dispatch_ba = ChildDispatch { child: artifact_a_ref.clone(), ops: &[op_ba], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut peers = [(&mut store_a, dispatch_ba)];
    let result = coordinator.dispatch_peer_group(&artifact_b_ref, &mut store_b, &mut peers, Vec::new(), GroupMeta::default(), &mut identity);
    match result.await {
        Ok(_) => panic!("expected a CompositionCycle rejection, but the dispatch succeeded"),
        Err(VcsError::CompositionCycle(_)) => {}
        Err(other) => panic!("expected CompositionCycle, got a different VcsError: {other}"),
    }
    assert_eq!(store_a.snapshot().expect("a snapshot").n, Some(0), "A must have zero new edits after a rejected cycle");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧪️ W1-C's "Owned reproduces today's behaviour EXACTLY" law: `CompositionCoordinator`
/// (the `TransactionCoordinator` alias) dispatching through the ordinary `Owned`-relation
/// `dispatch_group` produces EXACTLY what it did before `MemberRelation` existed — including
/// that `MutationMeta.origin` is NEVER touched (stays the ordinary `Apply`-assigned `Owner`
/// default), because `stamp_tail_origin` is only ever called on the `Peer` path. Every other
/// `dispatch_group_*`/`compensate_*`/`undo_group_*`/`redo_group_*` test above this region
/// already re-proves the rest of `Owned`'s behaviour (ownership check, genesis, atomicity,
/// compensation order) is untouched, unmodified, still green.
#[semio_framework_async_macros::async_test]
async fn dispatch_group_owned_path_never_stamps_a_transaction_origin() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let parent_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-owned-origin-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demoparent".into(), standard: "1".into(), subset: "*".into() } };
    let child_ref = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-owned-origin-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.demochild".into(), standard: "1".into(), subset: "*".into() } };

    let mut parent_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &parent_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut child_store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &child_ref.artifact_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let mut coordinator = CompositionCoordinator::new().await;
    coordinator.graph_mut().await.insert_owns(&parent_ref.artifact_id, "slot-1", &child_ref.artifact_id).await.expect("seed ownership");

    let parent_ops = vec![DemoMutation::SetN(SetN { n: 1 }).encode_op().expect("encode parent op")];
    let child_op = DemoMutation::SetN(SetN { n: 2 }).encode_op().expect("encode child op");
    let dispatch = ChildDispatch { child: child_ref.clone(), ops: &[child_op], op_schema: &SchemaId("demo/v1".into()), labels: &[] };
    let mut children = [(&mut child_store, dispatch)];

    let receipt = coordinator.dispatch_group(&parent_ref, &mut parent_store, &mut children, parent_ops, Vec::new(), GroupMeta::default(), &mut identity).await.expect("owned dispatch");

    assert_eq!(receipt.member_edits.len(), 2);
    let parent_origin = parent_store.envelope().vcs.edits.last().expect("parent edit").mutation_meta.last().expect("parent meta").origin.clone();
    let child_origin = child_store.envelope().vcs.edits.last().expect("child edit").mutation_meta.last().expect("child meta").origin.clone();
    assert_eq!(parent_origin, semio_framework_os_spr::MutationOrigin::Owner, "Owned relation never stamps a Transaction origin on the parent");
    assert_eq!(child_origin, semio_framework_os_spr::MutationOrigin::Owner, "Owned relation never stamps a Transaction origin on an owned child either");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}
//#endregion 🔖️TransactionPeerTests
//#endregion 🔖️PackValueFixtures
//#endregion 🔖️ReconcileAlternative
