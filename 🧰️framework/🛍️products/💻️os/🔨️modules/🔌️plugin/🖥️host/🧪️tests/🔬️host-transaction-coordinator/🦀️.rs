//! 🧪️ A minimal in-process fake — no wasm — that faithfully implements the SAME two-phase wire
//! contract §5 semantics `VcsArtifactApp::transaction_prepare/commit/rollback/undo/redo` (W1-B)
//! implements: one `Option<pending>` per instance, `TransactionCommit` applies as one edit
//! stamped with `group_id = txn_id`, `TransactionUndo`/`Redo` toggle a per-instance tail flag.
//! This proves `HostTransactionCoordinator`'s OWN orchestration (resolution via
//! `InstanceDirectory`/`ArtifactMutationRouter`, phase-1 all-or-nothing, reverse-order commit,
//! compensation, group fan-out) deterministically; the real wasmtime e2e in
//! `🏃️run/🦀️.rs` proves the wire-level plumbing against a REAL guest.
use super::*;
use std::cell::RefCell;

#[derive(Default)]
struct FakeInstance {
    pending: Option<(String, Vec<Vec<u8>>)>, // (txn_id, prepared_ops)
    edits: Vec<(String, Vec<Vec<u8>>)>,      // (group_id, ops) applied, in commit order
    undone: Vec<String>,
}

#[derive(Default)]
struct FakeCluster {
    instances: RefCell<HashMap<(String, u32), FakeInstance>>,
}

impl FakeCluster {
    // 🚫️async: E1 — pure in-memory RefCell fake, no suspension point; reverted per R9
    // (run_transaction/undo_group require a SYNC FnMut(...) -> Result<...> closure).
    fn exchange(&self, plugin_id: &str, instance_id: u32, command: protocol::AppCommand) -> Result<Vec<protocol::AppFrame>, TransactionError> {
        let mut instances = self.instances.borrow_mut();
        let instance = instances.entry((plugin_id.to_string(), instance_id)).or_default();
        let frame = match command {
            protocol::AppCommand::TransactionPrepare { txn_id, prepared_ops, .. } => {
                if instance.pending.is_some() {
                    protocol::AppFrame::TransactionPrepared { txn_id, foreign: Vec::new(), rejection: host_fault_bytes("transaction.instance-busy", "already has a pending transaction") }
                } else {
                    instance.pending = Some((txn_id.clone(), prepared_ops));
                    protocol::AppFrame::TransactionPrepared { txn_id, foreign: Vec::new(), rejection: Vec::new() }
                }
            }
            protocol::AppCommand::TransactionCommit { txn_id, .. } => match instance.pending.take() {
                Some((pending_id, ops)) if pending_id == txn_id => {
                    instance.edits.push((txn_id.clone(), ops));
                    protocol::AppFrame::TransactionCommitted { txn_id: txn_id.clone(), edit_id: format!("edit-{}", instance.edits.len()) }
                }
                other => {
                    instance.pending = other;
                    protocol::AppFrame::Error { in_reply_to: None, fault: host_fault_bytes("transaction.commit-failed", "no matching pending transaction"), report: Vec::new() }
                }
            },
            protocol::AppCommand::TransactionRollback { txn_id, .. } => {
                if matches!(&instance.pending, Some((pending_id, _)) if pending_id == &txn_id) {
                    instance.pending = None;
                }
                protocol::AppFrame::Done { in_reply_to: 0 }
            }
            protocol::AppCommand::TransactionUndo { group_id, .. } => {
                if instance.edits.iter().any(|(id, _)| id == &group_id) {
                    instance.undone.push(group_id.clone());
                }
                protocol::AppFrame::Done { in_reply_to: 0 }
            }
            other => panic!("unexpected command in fake transaction cluster: {other:?}"),
        };
        Ok(vec![frame])
    }
}

async fn dependency(id: &str) -> semio_framework::PluginDependency {
    semio_framework::PluginDependency::new(id, semio_framework::VersionReq::Any)
}

#[semio_framework_async_macros::async_test]
async fn a_two_member_transaction_commits_and_group_undo_restores_both() {
    let cluster = FakeCluster::default();
    let instances = InstanceDirectory::new();
    instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").await.unwrap();
    instances.bind("artifacts/target", "s.b", 2, "s.b.widget").await.unwrap();

    let router = ArtifactMutationRouter::new();
    // 🪪️ `io::ArtifactKindId::parse("s.b.widget").plugin()` == "b" (bare middle segment) — the
    // CONTRIBUTED row must be registered under the CONTRIBUTOR's own bare plugin id ("a"), with
    // "b" (matching the artifact kind's real owner) as its declared dependency.
    router
        .register_roster(
            "a",
            &[dependency("b").await],
            vec![HostMutationRosterEntry {
                mutation_id: "s.b.widget#a:annotate".into(),
                verb: "annotate".into(),
                entity: "widget".into(),
                kind: "annotate".into(),
                record: "widget.doc".into(),
                contributor: Some("a".into()),
                artifact_kind: Some("s.b.widget".into()),
            }],
        )
        .await
        .unwrap();

    let coordinator = HostTransactionCoordinator::new();
    let foreign = vec![protocol::ForeignStep {
        target: protocol::ForeignTarget { artifact_id: "artifacts/target".into(), artifact_kind: "s.b.widget".into(), dialect: None },
        mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()),
        payload: vec![9, 9],
        label: "annotate".into(),
    }];

    let outcome = coordinator
        .run_transaction(
            &instances,
            &router,
            |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
            |_contributor, _artifact_kind, _mutation_id, _member, payload| {
                Ok(HostArtifactMutationPlanResult { artifact_kind: "s.b.widget".into(), mutation_id: "s.b.widget#a:annotate".into(), revision: 0, generation: 0, owner_ops: vec![payload.to_vec()], label: "annotate".into(), foreign: Vec::new() })
            },
            TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
            vec![vec![1, 2, 3]],
            "propose annotate".into(),
            foreign,
        )
        .await
        .expect("a well-formed two-member transaction must commit");

    assert_eq!(outcome.members.len(), 2);
    assert_eq!(outcome.members[0], TransactionMember { plugin_id: "s.a".into(), instance_id: 1 }, "member 0 is the initiator");
    assert_eq!(outcome.edit_ids.len(), 2);
    assert!(outcome.edit_ids.iter().all(|id| !id.is_empty()));

    {
        let instances_map = cluster.instances.borrow();
        assert_eq!(instances_map.get(&("s.a".to_string(), 1)).unwrap().edits.len(), 1);
        assert_eq!(instances_map.get(&("s.b".to_string(), 2)).unwrap().edits.len(), 1);
    }

    coordinator.undo_group(|plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command), &outcome.members, &outcome.txn_id).await;
    let instances_map = cluster.instances.borrow();
    assert!(instances_map.get(&("s.a".to_string(), 1)).unwrap().undone.contains(&outcome.txn_id), "initiator must be undone");
    assert!(instances_map.get(&("s.b".to_string(), 2)).unwrap().undone.contains(&outcome.txn_id), "the contributed target must ALSO be undone (group undo restores both members)");
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_target_is_rejected_before_any_prepare_is_sent() {
    let cluster = FakeCluster::default();
    let instances = InstanceDirectory::new();
    instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").await.unwrap();
    let router = ArtifactMutationRouter::new();
    let coordinator = HostTransactionCoordinator::new();
    let foreign = vec![protocol::ForeignStep {
        target: protocol::ForeignTarget { artifact_id: "artifacts/nowhere".into(), artifact_kind: "s.b.widget".into(), dialect: None },
        mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()),
        payload: vec![1],
        label: "x".into(),
    }];
    let error = coordinator
        .run_transaction(
            &instances,
            &router,
            |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
            |_, _, _, _, _| unreachable!("no contributed step to plan"),
            TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
            vec![vec![1]],
            "x".into(),
            foreign,
        )
        .await
        .unwrap_err();
    assert_eq!(error.code().await, "transaction.unknown-target");
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_mutation_is_rejected() {
    let cluster = FakeCluster::default();
    let instances = InstanceDirectory::new();
    instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").await.unwrap();
    instances.bind("artifacts/target", "s.b", 2, "s.b.widget").await.unwrap();
    let router = ArtifactMutationRouter::new();
    let coordinator = HostTransactionCoordinator::new();
    let foreign = vec![protocol::ForeignStep {
        target: protocol::ForeignTarget { artifact_id: "artifacts/target".into(), artifact_kind: "s.b.widget".into(), dialect: None },
        mutation_id: protocol::SchemaId("s.b.widget#unregistered".into()),
        payload: vec![1],
        label: "x".into(),
    }];
    let error = coordinator
        .run_transaction(
            &instances,
            &router,
            |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
            |_, _, _, _, _| unreachable!("owner route, never contributed"),
            TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
            vec![vec![1]],
            "x".into(),
            foreign,
        )
        .await
        .unwrap_err();
    assert_eq!(error.code().await, "transaction.unknown-mutation");
}

#[semio_framework_async_macros::async_test]
async fn a_cycle_is_rejected() {
    let cluster = FakeCluster::default();
    let instances = InstanceDirectory::new();
    instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").await.unwrap();
    instances.bind("artifacts/target", "s.b", 2, "s.b.widget").await.unwrap();
    let router = ArtifactMutationRouter::new();
    // 🪪️ `io::ArtifactKindId::parse("s.b.widget").plugin()` == "b" (bare middle segment) — the
    // CONTRIBUTED row must be registered under the CONTRIBUTOR's own bare plugin id ("a"), with
    // "b" (matching the artifact kind's real owner) as its declared dependency.
    router
        .register_roster(
            "a",
            &[dependency("b").await],
            vec![HostMutationRosterEntry {
                mutation_id: "s.b.widget#a:annotate".into(),
                verb: "annotate".into(),
                entity: "widget".into(),
                kind: "annotate".into(),
                record: "widget.doc".into(),
                contributor: Some("a".into()),
                artifact_kind: Some("s.b.widget".into()),
            }],
        )
        .await
        .unwrap();
    let coordinator = HostTransactionCoordinator::new();
    let step = protocol::ForeignStep {
        target: protocol::ForeignTarget { artifact_id: "artifacts/target".into(), artifact_kind: "s.b.widget".into(), dialect: None },
        mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()),
        payload: vec![7],
        label: "x".into(),
    };
    // The contributed plan returns the SAME step again -> a real cycle by (artifact_id, mutation_id, payload_hash).
    let step_for_plan = step.clone();
    let error = coordinator
        .run_transaction(
            &instances,
            &router,
            |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
            move |_, _, _, _, payload| {
                Ok(HostArtifactMutationPlanResult {
                    artifact_kind: "s.b.widget".into(),
                    mutation_id: "s.b.widget#a:annotate".into(),
                    revision: 0,
                    generation: 0,
                    owner_ops: vec![payload.to_vec()],
                    label: "x".into(),
                    foreign: vec![step_for_plan.clone()],
                })
            },
            TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
            vec![vec![1]],
            "x".into(),
            vec![step],
        )
        .await
        .unwrap_err();
    assert_eq!(error.code().await, "transaction.cycle");
}

#[semio_framework_async_macros::async_test]
async fn a_member_rejection_rolls_back_every_already_prepared_member() {
    let cluster = FakeCluster::default();
    let instances = InstanceDirectory::new();
    instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").await.unwrap();
    instances.bind("artifacts/target", "s.b", 2, "s.b.widget").await.unwrap();
    // Pre-occupy s.b/2's pending slot so its OWN prepare hits `transaction.instance-busy`
    // for real, through the fake's genuine busy-check — not a stubbed rejection.
    cluster.instances.borrow_mut().entry(("s.b".to_string(), 2)).or_default().pending = Some(("someone-elses-txn".into(), vec![]));

    let router = ArtifactMutationRouter::new();
    // 🪪️ `io::ArtifactKindId::parse("s.b.widget").plugin()` == "b" (bare middle segment) — the
    // CONTRIBUTED row must be registered under the CONTRIBUTOR's own bare plugin id ("a"), with
    // "b" (matching the artifact kind's real owner) as its declared dependency.
    router
        .register_roster(
            "a",
            &[dependency("b").await],
            vec![HostMutationRosterEntry {
                mutation_id: "s.b.widget#a:annotate".into(),
                verb: "annotate".into(),
                entity: "widget".into(),
                kind: "annotate".into(),
                record: "widget.doc".into(),
                contributor: Some("a".into()),
                artifact_kind: Some("s.b.widget".into()),
            }],
        )
        .await
        .unwrap();
    let coordinator = HostTransactionCoordinator::new();
    let foreign = vec![protocol::ForeignStep {
        target: protocol::ForeignTarget { artifact_id: "artifacts/target".into(), artifact_kind: "s.b.widget".into(), dialect: None },
        mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()),
        payload: vec![1],
        label: "x".into(),
    }];

    let error = coordinator
        .run_transaction(
            &instances,
            &router,
            |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
            |_, _, _, _, payload| {
                Ok(HostArtifactMutationPlanResult { artifact_kind: "s.b.widget".into(), mutation_id: "s.b.widget#a:annotate".into(), revision: 0, generation: 0, owner_ops: vec![payload.to_vec()], label: "x".into(), foreign: Vec::new() })
            },
            TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
            vec![vec![1]],
            "x".into(),
            foreign,
        )
        .await
        .unwrap_err();
    assert_eq!(error.code().await, "transaction.member-rejected");
    let instances_map = cluster.instances.borrow();
    assert!(instances_map.get(&("s.a".to_string(), 1)).unwrap().pending.is_none(), "the initiator, prepared before the rejection, must have been rolled back");
}

#[semio_framework_async_macros::async_test]
async fn a_chain_deeper_than_max_plan_depth_is_rejected() {
    let cluster = FakeCluster::default();
    let instances = InstanceDirectory::new();
    instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").await.unwrap();
    for i in 0..10u8 {
        instances.bind(&format!("artifacts/target-{i}"), "s.b", 100 + i as u32, "s.b.widget").await.unwrap();
    }
    let router = ArtifactMutationRouter::new();
    // 🪪️ `io::ArtifactKindId::parse("s.b.widget").plugin()` == "b" (bare middle segment) — the
    // CONTRIBUTED row must be registered under the CONTRIBUTOR's own bare plugin id ("a"), with
    // "b" (matching the artifact kind's real owner) as its declared dependency.
    router
        .register_roster(
            "a",
            &[dependency("b").await],
            vec![HostMutationRosterEntry {
                mutation_id: "s.b.widget#a:annotate".into(),
                verb: "annotate".into(),
                entity: "widget".into(),
                kind: "annotate".into(),
                record: "widget.doc".into(),
                contributor: Some("a".into()),
                artifact_kind: Some("s.b.widget".into()),
            }],
        )
        .await
        .unwrap();
    let coordinator = HostTransactionCoordinator::new();
    let foreign = vec![protocol::ForeignStep {
        target: protocol::ForeignTarget { artifact_id: "artifacts/target-0".into(), artifact_kind: "s.b.widget".into(), dialect: None },
        mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()),
        payload: vec![0],
        label: "x".into(),
    }];
    // Each level's contributed plan hands back ONE new foreign step targeting the NEXT (distinct)
    // instance in the chain, so the cycle guard (which keys on artifact_id) never fires — this is
    // purely a depth chain, 10 hops deep against `MAX_PLAN_DEPTH` = 8.
    let error = coordinator
        .run_transaction(
            &instances,
            &router,
            |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
            |_, _, _, _, payload| {
                let next = payload[0] + 1;
                let foreign = if (next as usize) < 10 {
                    vec![protocol::ForeignStep {
                        target: protocol::ForeignTarget { artifact_id: format!("artifacts/target-{next}"), artifact_kind: "s.b.widget".into(), dialect: None },
                        mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()),
                        payload: vec![next],
                        label: "x".into(),
                    }]
                } else {
                    Vec::new()
                };
                Ok(HostArtifactMutationPlanResult { artifact_kind: "s.b.widget".into(), mutation_id: "s.b.widget#a:annotate".into(), revision: 0, generation: 0, owner_ops: vec![payload.to_vec()], label: "x".into(), foreign })
            },
            TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
            vec![vec![1]],
            "x".into(),
            foreign,
        )
        .await
        .unwrap_err();
    assert_eq!(error.code().await, "transaction.depth-exceeded");
}
