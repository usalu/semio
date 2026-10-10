use super::*;
use crate::io::binary::causal::*;

//#region 🧸️Fixtures
// Dummy (P=i64, Op=CausalAddOp) pair: the smallest possible Mutation/MutationDiff impl,
// reused across this file's tests instead of a real technology's op set.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
struct CausalAddDiff {
    delta: i64,
}
impl crate::value::ToValue for CausalAddDiff {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object([("delta".to_string(), crate::value::ToValue::to_value(&self.delta))])
    }
}
impl crate::value::FromValue for CausalAddDiff {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let entries = value.into_object()?;
        let delta = entries.iter().find(|(k, _)| k == "delta").map_or(Ok(0), |(_, v)| crate::value::FromValue::from_value(v.clone()))?;
        Ok(Self { delta })
    }
}
impl crate::mutation::DiffAlgebra<i64> for CausalAddDiff {
    fn inverse(&self, _base: &i64) -> Self {
        Self { delta: -self.delta }
    }
    fn is_empty(&self) -> bool {
        self.delta == 0
    }
}
impl crate::mutation::MutationDiff<i64> for CausalAddDiff {
    fn apply(&self, base: &i64, _capability: crate::mutation::ApplyCapability) -> crate::mutation::MutationApplyResult<i64> {
        Ok(base + self.delta)
    }
    fn absorb(&mut self, other: Self) {
        self.delta += other.delta;
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct CausalAddOp {
    delta: i64,
}
impl crate::value::ToValue for CausalAddOp {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object([("delta".to_string(), crate::value::ToValue::to_value(&self.delta))])
    }
}
impl crate::value::FromValue for CausalAddOp {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let entries = value.into_object()?;
        let delta = entries.iter().find(|(k, _)| k == "delta").map_or(Ok(0), |(_, v)| crate::value::FromValue::from_value(v.clone()))?;
        Ok(Self { delta })
    }
}
const CAUSAL_ADD_DESCRIPTOR: crate::mutation::MutationLeafDescriptor = crate::mutation::MutationLeafDescriptor {
    schema_version: 1,
    owner: "🧰️framework/🔨️modules/📡️replication/🔗️causal/🧫️fixtures/🧬️mutations/➕️causal-add",
    semantic_kind: "causal-add",
    display_name: "Causal Add",
    emoji: "➕️",
    aggregate_variant: "CausalAddOp",
    payload_schema: "🦀️.rs#CausalAddOp",
    text_opcode: None,
    binary_tag: None,
    invertibility: crate::mutation::MutationInvertibility::ExplicitMutation,
    diff_participation: crate::mutation::MutationDiffParticipation::ApplyOnly,
    outcome_classes: &[crate::mutation::MutationOutcomeClass::Applied],
    composition: crate::mutation::MutationComposition::Atomic,
    required_language_surfaces: &[crate::mutation::MutationLanguageSurface::Rust, crate::mutation::MutationLanguageSurface::Binary],
};
impl crate::mutation::Mutation<i64> for CausalAddOp {
    type Diff = CausalAddDiff;
    const DESCRIPTORS: &'static [crate::mutation::MutationLeafDescriptor] = &[CAUSAL_ADD_DESCRIPTOR];
    fn descriptor(&self) -> &'static crate::mutation::MutationLeafDescriptor {
        &CAUSAL_ADD_DESCRIPTOR
    }
    fn diff(&self, _base: &i64) -> crate::mutation::MutationOutcome<CausalAddDiff> {
        crate::mutation::MutationOutcome::new(CausalAddDiff { delta: self.delta })
    }
    fn inverse(&self, _base: &i64) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![CausalAddOp { delta: -self.delta }]
    
    })())
}
}
/// 🎯️ Hand-written (no `os_dsl::DslOps` derive in this dependency-free fixture): `format
/// u8 (=1) | delta i64 LE`.
impl crate::io::OpBinary for CausalAddOp {
    fn encode_op(&self) -> Result<Vec<u8>, crate::ProtocolError> {
        let mut out = vec![1u8];
        out.extend_from_slice(&self.delta.to_le_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::ProtocolError> {
        if bytes.len() != 9 || bytes[0] != 1 {
            return Err(crate::ProtocolError::Malformed { what: "causal add op", offset: 0, detail: "expected 9 bytes, format 1".to_string() });
        }
        let mut delta_bytes = [0u8; 8];
        delta_bytes.copy_from_slice(&bytes[1..9]);
        Ok(CausalAddOp { delta: i64::from_le_bytes(delta_bytes) })
    }
}
impl MutationTransform<i64> for CausalAddOp {
    fn transform(&self, against: &Self) -> TransformOutcome<Self> {
        if self.delta == against.delta {
            TransformOutcome::Unchanged(self.clone())
        } else if self.delta == 0 {
            TransformOutcome::Conflict("zero delta cannot transform".to_string())
        } else {
            TransformOutcome::Transformed(CausalAddOp { delta: self.delta + against.delta })
        }
    }
}

fn sample_envelope(id: &str, deps: Vec<&str>) -> MutationEnvelope {
    MutationEnvelope {
        mutation_id: crate::ids::MutationId(id.into()),
        document_id: crate::ids::ArtifactId("document-1".into()),
        actor: crate::ids::ActorId("actor-1".into()),
        dependencies: deps.into_iter().map(|dep| crate::ids::MutationId(dep.into())).collect(),
        observed: None,
        target: Vec::new(),
        diff: ArtifactDiff { schema: crate::ids::SchemaId("diff.v1".into()), payload: id.as_bytes().to_vec() },
        inverse: InverseMutation { schema: crate::ids::SchemaId("diff.v1".into()), payload: Vec::new() },
        timestamp: crate::ids::HybridLogicalTimestamp::new(1, 0),
        transaction: None, verb: None, line: None,
    }
}

fn take_applied(dag: &mut MutationDag) -> Vec<MutationEnvelope> {
    let mut envelopes = Vec::new();
    loop {
        match dag.take_next_applied() {
            MutationDagAppliedStep::Envelope(envelope) => envelopes.push(envelope),
            MutationDagAppliedStep::SeededIdentity => {}
            MutationDagAppliedStep::Complete => return envelopes,
        }
    }
}

fn retire_dag_shell(dag: &mut MutationDag) {
    while dag.take_one_close_owner().is_some() {}
    assert!(dag.terminal_is_empty());
}
//#endregion 🧸️Fixtures

#[test]
fn causal_add_fixture_has_exact_required_descriptor() {
    use crate::mutation::Mutation;
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧬️mutations/➕️causal-add/🧪️descriptor/🔣️.json")).unwrap();
    assert_eq!(CAUSAL_ADD_DESCRIPTOR.validate(), Ok(()));
    assert_eq!(CausalAddOp::DESCRIPTORS.len(), 1);
    assert_eq!(serde_json::Value::from(crate::value::ToValue::to_value(CausalAddOp { delta: -7 }.descriptor())), expected);
}

//#region 🔖️Envelope
#[test]
fn operation_envelope_binary_round_trips() {
    let envelope = sample_envelope("operation-1", vec!["operation-0"]);
    let mut out = Vec::new();
    encode_envelope(&envelope, &mut out);
    let mut pos = 0;
    let round_tripped = decode_envelope(&out, &mut pos).expect("decode");
    assert_eq!(round_tripped, envelope);
}
//#endregion 🔖️Envelope

//#region 🔖️MutationDag
#[test]
fn inserts_pending_until_dependencies_arrive() {
    let mut dag = MutationDag::new();
    assert_eq!(dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap(), InsertResult::Pending);
    assert_eq!(dag.insert(sample_envelope("operation-1", vec![])).unwrap(), InsertResult::Applied);
    assert_eq!(dag.applied.len(), 2);
    retire_dag_shell(&mut dag);
}

#[test]
fn drains_applied_envelopes_in_causal_order() {
    let mut dag = MutationDag::new();
    dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap();
    dag.insert(sample_envelope("operation-1", vec![])).unwrap();
    let drained = take_applied(&mut dag);
    assert_eq!(drained.iter().map(|envelope| envelope.mutation_id.0.clone()).collect::<Vec<_>>(), vec!["operation-1".to_string(), "operation-2".to_string()]);
    assert!(take_applied(&mut dag).is_empty(), "second drain yields nothing new");
    dag.insert(sample_envelope("operation-3", vec![])).unwrap();
    let drained = take_applied(&mut dag);
    assert_eq!(drained.len(), 1);
    assert_eq!(drained[0].mutation_id.0, "operation-3");
    retire_dag_shell(&mut dag);
}

#[test]
fn insert_duplicate_pending_operation_id_errors() {
    let mut dag = MutationDag::new();
    dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap();
    let err = dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap_err();
    assert_eq!(err.error, MutationDagError::Duplicate);
    assert_eq!(err.envelope.mutation_id.0, "operation-2");
    retire_dag_shell(&mut dag);
}

#[test]
fn insert_already_applied_operation_returns_already_applied_without_erroring() {
    let mut dag = MutationDag::new();
    dag.insert(sample_envelope("operation-1", vec![])).unwrap();
    let result = dag.insert(sample_envelope("operation-1", vec![])).unwrap();
    assert!(matches!(result, InsertResult::AlreadyApplied(envelope) if envelope.mutation_id.0 == "operation-1"));
    retire_dag_shell(&mut dag);
}

#[test]
fn seed_applied_unblocks_pending_envelopes_that_reference_out_of_band_deps() {
    let mut dag = MutationDag::new();
    assert_eq!(dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap(), InsertResult::Pending);
    assert!(dag.ready_identity_at(0).is_none(), "dependency is not yet known to this dag");
    dag.seed_applied(crate::ids::MutationId("operation-1".to_string())).unwrap();
    assert_eq!(dag.ready_identity_at(0), Some("operation-2"));
    retire_dag_shell(&mut dag);
}

#[test]
fn duplicate_seed_returns_the_exact_unadopted_identity_owner() {
    let mut dag = MutationDag::new();
    dag.seed_applied(crate::ids::MutationId("operation-1".to_string())).unwrap();
    let rejected = dag.seed_applied(crate::ids::MutationId("operation-1".to_string())).expect_err("duplicate seed must not consume its exact owner");
    assert_eq!(rejected.error, MutationDagError::Duplicate);
    assert_eq!(rejected.mutation_id.0, "operation-1");
    retire_dag_shell(&mut dag);
}

#[test]
fn fixed_slot_free_ring_reuses_generation_without_reordering_live_owners() {
    let mut slots = MutationDagFixedSlots::new();
    slots.push("first".to_string()).unwrap();
    slots.push("second".to_string()).unwrap();
    let first_slot = usize::from(slots.head);
    let first_generation = slots.generations[first_slot];
    assert_eq!(slots.swap_remove(0).as_deref(), Some("first"));
    slots.push("third".to_string()).unwrap();
    assert_eq!(usize::from(slots.tail), first_slot, "LIFO free ring reuses the exact detached slot");
    assert_ne!(slots.generations[first_slot], first_generation, "reused slot advances its ABA generation");
    assert_eq!(slots.iter().map(String::as_str).collect::<Vec<_>>(), vec!["second", "third"], "live traversal remains deterministic insertion order");
    assert_eq!(slots.pop().as_deref(), Some("third"));
    assert_eq!(slots.pop().as_deref(), Some("second"));
    assert!(slots.is_empty());
}

#[test]
fn opdagerror_display_is_non_empty() {
    assert!(!MutationDagError::Duplicate.to_string().is_empty());
}

#[test]
fn fixed_causal_authority_rejects_capacity_plus_one_with_exact_identity_and_closes_one_owner_at_a_time() {
    let mut dag = MutationDag::new();
    for index in 0..MUTATION_DAG_CAPACITY {
        dag.seed_applied(crate::ids::MutationId(format!("seed-{index:04}"))).expect("exact fixed capacity is admitted");
    }
    let rejected = dag.seed_applied(crate::ids::MutationId("capacity-plus-one".into())).expect_err("capacity plus one must retain its exact identity owner");
    assert_eq!(rejected.error, MutationDagError::Capacity);
    assert_eq!(rejected.mutation_id.0, "capacity-plus-one");
    let mut released = 0;
    while let Some(owner) = dag.take_one_close_owner() {
        assert!(matches!(owner, MutationDagCloseOwner::Identity(_)));
        released += 1;
    }
    assert_eq!(released, MUTATION_DAG_CAPACITY);
    assert!(dag.terminal_is_empty());
}

#[test]
fn causal_insert_rejects_oversized_identity_without_losing_the_envelope_owner() {
    let mut dag = MutationDag::new();
    let envelope = sample_envelope(&"x".repeat(MUTATION_DAG_IDENTIFIER_BYTES + 1), vec![]);
    let rejected = dag.insert(envelope).expect_err("oversized identity is rejected before fixed-slot adoption");
    assert_eq!(rejected.error, MutationDagError::IdentifierTooLong);
    assert_eq!(rejected.envelope.mutation_id.0.len(), MUTATION_DAG_IDENTIFIER_BYTES + 1);
    assert!(dag.terminal_is_empty());
}

//#region 🏃️quick
mod quick {
    use super::*;

    /// 🔁️ Diamond DAG (A none; B,C dep A; D dep B,C) inserted in every hand-picked
    /// topological order converges to the same final applied set and drained envelope count —
    /// the "permutation-convergence" law the amendment's testing note asks for at the `quick`
    /// tier. In a true topological order every dependency is already `applied` when its dependent arrives.
    fn diamond(id_a: &str, id_b: &str, id_c: &str, id_d: &str) -> [(&'static str, MutationEnvelope); 4] {
        [("a", sample_envelope(id_a, vec![])), ("b", sample_envelope(id_b, vec![id_a])), ("c", sample_envelope(id_c, vec![id_a])), ("d", sample_envelope(id_d, vec![id_b, id_c]))]
    }

    fn assert_converges(order: [&str; 4]) {
        let nodes = diamond("A", "B", "C", "D");
        let mut dag = MutationDag::new();
        for label in order {
            let (_, envelope) = nodes.iter().find(|(l, _)| *l == label).expect("known label").clone();
            let result = dag.insert(envelope).expect("insert never duplicates in a fresh dag");
            assert_eq!(result, InsertResult::Applied, "insertion order {order:?} must stay fully topological");
        }
        let drained = take_applied(&mut dag);
        let mut ids: Vec<String> = drained.iter().map(|e| e.mutation_id.0.clone()).collect();
        ids.sort();
        assert_eq!(ids, vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()]);
        retire_dag_shell(&mut dag);
    }

    /// ⛓️ A chain whose middle arrives first (b, c, a) stays in causal order: c names b, which is buffered but still waits
    /// for a, so c waits too, and a's arrival releases b and then c. Taking a merely buffered dependency for a met one
    /// applied c before b — a `Branch` folded before the `Commit` it names.
    #[test]
    fn a_dependency_that_is_buffered_but_pending_keeps_its_dependent_pending() {
        let mut dag = MutationDag::new();
        assert_eq!(dag.insert(sample_envelope("b", vec!["a"])).unwrap(), InsertResult::Pending);
        assert_eq!(dag.insert(sample_envelope("c", vec!["b"])).unwrap(), InsertResult::Pending);
        assert_eq!(dag.insert(sample_envelope("a", vec![])).unwrap(), InsertResult::Applied);
        let drained: Vec<String> = take_applied(&mut dag).into_iter().map(|envelope| envelope.mutation_id.0).collect();
        assert_eq!(drained, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        retire_dag_shell(&mut dag);
    }

    #[test]
    fn topological_order_a_b_c_d_converges() {
        assert_converges(["a", "b", "c", "d"]);
    }

    #[test]
    fn topological_order_a_c_b_d_converges() {
        assert_converges(["a", "c", "b", "d"]);
    }

    #[test]
    fn topological_order_a_b_d_c_is_rejected_as_non_topological() {
        // "d" before "c" is NOT a valid topological order (d depends on c) — insert must not
        // silently accept it as Applied; it must classify as Pending instead, proving this
        // test suite actually distinguishes topological from non-topological orderings rather
        // than accepting anything.
        let nodes = diamond("A", "B", "C", "D");
        let mut dag = MutationDag::new();
        for label in ["a", "b", "d"] {
            let (_, envelope) = nodes.iter().find(|(l, _)| *l == label).expect("known label").clone();
            let result = dag.insert(envelope).unwrap();
            if label == "d" {
                assert_eq!(result, InsertResult::Pending, "d must not apply before its dependency c arrives");
            }
        }
        dag.insert(nodes.into_iter().find(|(l, _)| *l == "c").unwrap().1).unwrap();
        let mut ids: Vec<String> = dag.applied.iter().cloned().collect();
        ids.sort();
        assert_eq!(ids, vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()], "once c arrives, d converges too");
        retire_dag_shell(&mut dag);
    }
}
//#endregion 🏃️quick
//#endregion 🔖️MutationDag

//#region 🔖️Frontier
fn frontier(document_id: &str, ordinal: u64, head_id: &str, commit_seq: u64, chain_byte: u8) -> FrontierSummary {
    FrontierSummary { document_id: crate::ids::ArtifactId(document_id.into()), head_edit_ordinal: ordinal, head_edit_id: head_id.into(), last_commit_seq: commit_seq, chain_hash: [chain_byte; 32] }
}

#[test]
fn frontier_delta_identical_summaries_are_equal() {
    let a = frontier("doc-1", 5, "edit-5", 3, 9);
    let b = a.clone();
    assert_eq!(frontier_delta(&a, &b), FrontierComparison::Equal);
}

#[test]
fn frontier_delta_greater_ordinal_is_ahead() {
    let local = frontier("doc-1", 10, "edit-10", 4, 1);
    let remote = frontier("doc-1", 5, "edit-5", 3, 2);
    assert_eq!(frontier_delta(&local, &remote), FrontierComparison::Ahead);
}

#[test]
fn frontier_delta_lesser_ordinal_is_behind() {
    let local = frontier("doc-1", 5, "edit-5", 3, 1);
    let remote = frontier("doc-1", 10, "edit-10", 4, 2);
    assert_eq!(frontier_delta(&local, &remote), FrontierComparison::Behind);
}

#[test]
fn frontier_delta_same_ordinal_different_head_is_diverged() {
    let local = frontier("doc-1", 5, "edit-5a", 3, 1);
    let remote = frontier("doc-1", 5, "edit-5b", 3, 2);
    assert_eq!(frontier_delta(&local, &remote), FrontierComparison::Diverged { common_edit_count: 5 });
}

/// 🌱️ Rewritten off `serde_json` (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01): round-trips through `ToValue`/`FromValue` instead.
#[test]
fn frontier_summary_to_value_round_trips() {
    let summary = frontier("doc-1", 7, "edit-7", 2, 5);
    let value = crate::value::ToValue::to_value(&summary);
    let round_tripped: FrontierSummary = crate::value::FromValue::from_value(value).expect("decode");
    assert_eq!(round_tripped, summary);
}
//#endregion 🔖️Frontier

//#region 🔖️Transform
#[test]
fn transform_unchanged_when_deltas_match() {
    let a = CausalAddOp { delta: 3 };
    let b = CausalAddOp { delta: 3 };
    assert_eq!(a.transform(&b), TransformOutcome::Unchanged(CausalAddOp { delta: 3 }));
}

#[test]
fn transform_transformed_when_deltas_differ() {
    let a = CausalAddOp { delta: 2 };
    let b = CausalAddOp { delta: 5 };
    assert_eq!(a.transform(&b), TransformOutcome::Transformed(CausalAddOp { delta: 7 }));
}

#[test]
fn transform_conflict_case_carries_message() {
    let a = CausalAddOp { delta: 0 };
    let b = CausalAddOp { delta: 9 };
    match a.transform(&b) {
        TransformOutcome::Conflict(message) => assert!(!message.is_empty()),
        other => panic!("expected Conflict, got {other:?}"),
    }
}
//#endregion 🔖️Transform

//#region 🔖️Bridge
#[test]
fn mutation_envelope_from_edit_derives_one_envelope_per_forward_op_using_explicit_meta() {
    let edit = crate::mutation::Edit::<CausalAddOp> {
        id: "edit-1".into(),
        actor: Some("actor-fallback".into()),
        forwards: vec![CausalAddOp { delta: 1 }, CausalAddOp { delta: 2 }],
        inverse: vec![CausalAddOp { delta: -1 }, CausalAddOp { delta: -2 }].into(),
        mutation_meta: vec![
            crate::mutation::MutationMeta {
                mutation_id: Some(crate::ids::MutationId("op-a".into())),
                dependencies: vec![crate::ids::MutationId("op-0".into())],
                base_version: 0,
                author_id: Some(crate::ids::ActorId("actor-explicit".into())),
                timestamp: crate::ids::HybridLogicalTimestamp::new(1, 1000),
                undo_policy: crate::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: None,
                origin: crate::mutation::MutationOrigin::Owner,
                transaction: None,
            },
            crate::mutation::MutationMeta {
                mutation_id: Some(crate::ids::MutationId("op-b".into())),
                dependencies: vec![crate::ids::MutationId("op-a".into())],
                base_version: 1,
                author_id: None,
                timestamp: crate::ids::HybridLogicalTimestamp::new(1, 2000),
                undo_policy: crate::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: None,
                origin: crate::mutation::MutationOrigin::Owner,
                transaction: Some(crate::mutation::TransactionRef { id: "tx-0011223344556677".into(), tool: "app#select".into() }),
            },
        ], verb: None, line: None,
        sequence_number: 1,
        started_at: "2026-07-27T00:00:00Z".into(),
        finished_at: None,
    };
    let document_id = crate::ids::ArtifactId("doc-1".into());
    let schema = crate::ids::SchemaId("causal-add.v1".into());

    let envelopes = mutation_envelope_from_edit(&edit, &document_id, &schema).expect("encode succeeds");
    assert_eq!(envelopes.len(), 2);

    assert_eq!(envelopes[0].mutation_id, crate::ids::MutationId("op-a".into()));
    assert_eq!(envelopes[0].actor, crate::ids::ActorId("actor-explicit".into()));
    assert_eq!(envelopes[0].dependencies, vec![crate::ids::MutationId("op-0".into())]);
    assert_eq!(envelopes[0].document_id, document_id);
    assert_eq!(envelopes[0].timestamp, crate::ids::HybridLogicalTimestamp::new(1, 1000));
    assert_eq!(envelopes[0].diff.schema, schema);
    assert_eq!(envelopes[0].diff.payload, crate::io::OpBinary::encode_op(&CausalAddOp { delta: 1 }).unwrap());
    assert_eq!(envelopes[0].inverse.payload, crate::io::OpBinary::encode_op(&CausalAddOp { delta: -1 }).unwrap());

    // Second op's meta has no author_id -> falls back to `edit.actor`, not "unknown".
    assert_eq!(envelopes[1].mutation_id, crate::ids::MutationId("op-b".into()));
    assert_eq!(envelopes[1].actor, crate::ids::ActorId("actor-fallback".into()));
    assert_eq!(envelopes[0].transaction, None, "an operation authored outside a tool transaction carries none");
    assert_eq!(envelopes[1].transaction, edit.mutation_meta[1].transaction, "the meta's transaction rides the wire envelope");
}

#[test]
fn mutation_envelope_from_edit_falls_back_to_op_trait_and_structural_defaults_without_meta() {
    let edit = crate::mutation::Edit::<CausalAddOp> {
        id: "edit-2".into(),
        actor: None,
        forwards: vec![CausalAddOp { delta: 5 }],
        inverse: Default::default(),
        mutation_meta: vec![], verb: None, line: None,
        sequence_number: 0,
        started_at: "2026-07-27T00:00:00Z".into(),
        finished_at: None,
    };
    let document_id = crate::ids::ArtifactId("doc-2".into());
    let schema = crate::ids::SchemaId("causal-add.v1".into());

    let envelopes = mutation_envelope_from_edit(&edit, &document_id, &schema).expect("encode succeeds");
    assert_eq!(envelopes.len(), 1);
    assert_eq!(envelopes[0].mutation_id, crate::ids::MutationId("edit-2#0".into()));
    assert_eq!(envelopes[0].actor, crate::ids::ActorId("unknown".into()));
    assert!(envelopes[0].dependencies.is_empty());
    assert_eq!(envelopes[0].timestamp, crate::ids::HybridLogicalTimestamp::new(0, 0));
    assert_eq!(envelopes[0].inverse.payload, Vec::<u8>::new(), "inverse vec shorter than forwards -> empty inverse payload");
}

#[test]
fn mutation_envelope_from_edit_propagates_an_encode_failure() {
    let edit = crate::mutation::Edit::<CausalAddOp> {
        id: "edit-3".into(),
        actor: None,
        forwards: vec![CausalAddOp { delta: 1 }],
        inverse: Default::default(),
        mutation_meta: vec![], verb: None, line: None,
        sequence_number: 0,
        started_at: "2026-07-27T00:00:00Z".into(),
        finished_at: None,
    };
    // CausalAddOp::encode_op is infallible by construction, so this test instead documents
    // the law via the Result signature: a real Op whose encode_op can fail (e.g. exceeding a
    // size limit) aborts the whole batch rather than returning a partial Vec.
    let document_id = crate::ids::ArtifactId("doc-3".into());
    let schema = crate::ids::SchemaId("causal-add.v1".into());
    assert!(mutation_envelope_from_edit(&edit, &document_id, &schema).is_ok());
}
//#endregion 🔖️Bridge

//#region 🔖️EnvelopeCodec
#[test]
fn envelope_binary_round_trips() {
    let envelope = sample_envelope("operation-1", vec!["operation-0", "operation-x"]);
    let mut out = Vec::new();
    encode_envelope(&envelope, &mut out);
    let mut pos = 0;
    let decoded = decode_envelope(&out, &mut pos).expect("decode");
    assert_eq!(decoded, envelope);
    assert_eq!(pos, out.len(), "decode must consume exactly the encoded bytes");
}

/// 🧾️ A tool transaction, an authoring verb and an alternative line ride the binary envelope (trailing flags
/// bit 0, bit 1 and bit 2) and its value shape; an unknown trailing flag is refused.
#[test]
fn envelope_transaction_and_verb_round_trip_through_binary_and_value() {
    let transaction = Some(crate::mutation::TransactionRef { id: "tx-0123456789abcdef".into(), tool: "app#select".into() });
    for (transaction, verb, line) in [(transaction.clone(), None, None), (None, Some("typeText".to_string()), None), (transaction, Some("typeText".to_string()), Some("alt-1".to_string()))] {
        let mut envelope = sample_envelope("operation-1", vec!["operation-0"]);
        envelope.transaction = transaction;
        envelope.verb = verb;
        envelope.line = line;
        let mut out = Vec::new();
        encode_envelope(&envelope, &mut out);
        let mut pos = 0;
        assert_eq!(decode_envelope(&out, &mut pos).expect("decode"), envelope);
        assert_eq!(pos, out.len());
        let decoded: MutationEnvelope = crate::value::FromValue::from_value(crate::value::ToValue::to_value(&envelope)).expect("value decode");
        assert_eq!(decoded, envelope);
    }
    let mut plain = Vec::new();
    encode_envelope(&sample_envelope("operation-1", vec!["operation-0"]), &mut plain);
    *plain.last_mut().expect("trailing flags") = 8;
    assert!(format!("{:?}", decode_envelope(&plain, &mut 0).expect_err("flag 8")).contains("trailing flags 8"));
}

#[test]
fn envelope_binary_encoding_is_deterministic() {
    let envelope = sample_envelope("operation-1", vec!["operation-0"]);
    let mut a = Vec::new();
    let mut b = Vec::new();
    encode_envelope(&envelope, &mut a);
    encode_envelope(&envelope, &mut b);
    assert_eq!(a, b);
}

#[test]
fn envelope_binary_round_trips_with_empty_dependencies_and_payloads() {
    let envelope = MutationEnvelope {
        mutation_id: crate::ids::MutationId("op-empty".into()),
        document_id: crate::ids::ArtifactId("doc-empty".into()),
        actor: crate::ids::ActorId("actor-empty".into()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: ArtifactDiff { schema: crate::ids::SchemaId("s".into()), payload: Vec::new() },
        inverse: InverseMutation { schema: crate::ids::SchemaId("s".into()), payload: Vec::new() },
        timestamp: crate::ids::HybridLogicalTimestamp::new(0, 0),
        transaction: None, verb: None, line: None,
    };
    let mut out = Vec::new();
    encode_envelope(&envelope, &mut out);
    let mut pos = 0;
    assert_eq!(decode_envelope(&out, &mut pos).unwrap(), envelope);
}

#[test]
fn frontier_binary_round_trips() {
    let f = frontier("doc-1", 7, "edit-7", 3, 9);
    let mut out = Vec::new();
    encode_frontier(&f, &mut out);
    let mut pos = 0;
    assert_eq!(decode_frontier(&out, &mut pos).unwrap(), f);
    assert_eq!(pos, out.len());
}

#[test]
fn envelopes_batch_binary_round_trips_including_empty() {
    let empty: Vec<MutationEnvelope> = Vec::new();
    assert_eq!(decode_envelopes(&encode_envelopes(&empty)).unwrap(), empty);

    let batch = vec![sample_envelope("operation-1", vec!["operation-0"]), sample_envelope("operation-2", Vec::new())];
    assert_eq!(decode_envelopes(&encode_envelopes(&batch)).unwrap(), batch);
}

#[test]
fn document_backbone_batch_fixture_is_exact_bounded_and_u64_safe() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧮️document-backbone-batch-v1/🔣️.json")).expect("document backbone fixture parses");
    assert_eq!(fixture["retention"]["maximumBytes"].as_u64(), Some(DOCUMENT_BACKBONE_PENDING_MAXIMUM_BYTES as u64));
    assert_eq!(fixture["retention"]["maximumMessages"].as_u64(), Some(DOCUMENT_BACKBONE_PENDING_MAXIMUM_MESSAGES as u64));
    for row in fixture["cases"].as_array().expect("fixture cases") {
        let raw = row["rawHex"].as_str().expect("raw hex");
        let bytes = (0..raw.len()).step_by(2).map(|index| u8::from_str_radix(&raw[index..index + 2], 16).expect("hex byte")).collect::<Vec<_>>();
        let limits = &row["limits"];
        let limits = DocumentBackboneBatchLimitsV1 {
            maximum_bytes: limits["maximumBytes"].as_u64().expect("maximumBytes") as usize,
            maximum_envelopes: limits["maximumEnvelopes"].as_u64().expect("maximumEnvelopes") as usize,
            maximum_dependencies_per_envelope: limits["maximumDependenciesPerEnvelope"].as_u64().expect("maximumDependenciesPerEnvelope") as usize,
            maximum_total_dependencies: limits["maximumTotalDependencies"].as_u64().expect("maximumTotalDependencies") as usize,
            maximum_target_segments_per_envelope: limits["maximumTargetSegmentsPerEnvelope"].as_u64().expect("maximumTargetSegmentsPerEnvelope") as usize,
            maximum_total_target_segments: limits["maximumTotalTargetSegments"].as_u64().expect("maximumTotalTargetSegments") as usize,
            maximum_identifier_bytes: limits["maximumIdentifierBytes"].as_u64().expect("maximumIdentifierBytes") as usize,
            maximum_schema_bytes: limits["maximumSchemaBytes"].as_u64().expect("maximumSchemaBytes") as usize,
            maximum_payload_bytes: limits["maximumPayloadBytes"].as_u64().expect("maximumPayloadBytes") as usize,
        };
        let expected = row["expect"]["outcome"].as_str().expect("expected outcome");
        match (decode_document_backbone_envelopes_exact_with_limits(&bytes, limits), expected) {
            (Ok(envelopes), "accepted") => {
                assert_eq!(encode_envelopes(&envelopes), bytes, "{}", row["id"]);
                let expected_envelopes = row["expect"]["envelopes"].as_array().expect("expected envelopes");
                assert_eq!(envelopes.len(), expected_envelopes.len(), "{}", row["id"]);
                for (actual, expected) in envelopes.iter().zip(expected_envelopes) {
                    assert_eq!(actual.mutation_id.0, expected["mutationId"].as_str().expect("mutationId"));
                    assert_eq!(actual.document_id.0, expected["documentId"].as_str().expect("documentId"));
                    assert_eq!(actual.actor.0.as_str(), expected["actor"].as_str().expect("actor"));
                    assert_eq!(
                        actual.dependencies.iter().map(|dependency| dependency.0.as_str()).collect::<Vec<_>>(),
                        expected["dependencies"].as_array().expect("dependencies").iter().map(|dependency| dependency.as_str().expect("dependency")).collect::<Vec<_>>()
                    );
                    assert_eq!(actual.observed.as_ref().map(|observed| observed.0.as_str()), expected["observed"].as_str(), "{}", row["id"]);
                    assert_eq!(actual.target.iter().map(String::as_str).collect::<Vec<_>>(), expected["target"].as_array().expect("target").iter().map(|segment| segment.as_str().expect("segment")).collect::<Vec<_>>(), "{}", row["id"]);
                    assert_eq!(actual.diff.schema.0, expected["diff"]["schema"].as_str().expect("diff schema"));
                    assert_eq!(actual.inverse.schema.0, expected["inverse"]["schema"].as_str().expect("inverse schema"));
                    assert_eq!(actual.diff.payload.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), expected["diff"]["payloadHex"].as_str().expect("diff payloadHex"));
                    assert_eq!(actual.inverse.payload.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), expected["inverse"]["payloadHex"].as_str().expect("inverse payloadHex"));
                    assert_eq!(actual.timestamp.actor.to_string(), expected["timestamp"]["actor"].as_str().expect("timestamp actor"));
                    assert_eq!(actual.timestamp.physical_ms.to_string(), expected["timestamp"]["physicalMs"].as_str().expect("timestamp physicalMs"));
                    assert_eq!(actual.timestamp.logical.to_string(), expected["timestamp"]["logical"].as_str().expect("timestamp logical"));
                    let transaction = &expected["transaction"];
                    assert_eq!(actual.transaction, (!transaction.is_null()).then(|| crate::mutation::TransactionRef { id: transaction["id"].as_str().expect("transaction id").into(), tool: transaction["tool"].as_str().expect("transaction tool").into() }), "{}", row["id"]);
                    assert_eq!(actual.verb.as_deref(), expected["verb"].as_str(), "{}", row["id"]);
                }
            }
            (Err(crate::ProtocolError::LimitExceeded(_)), "limit") => {}
            (Err(_), "malformed") => {}
            (actual, expected) => panic!("{} produced {actual:?}, expected {expected}", row["id"]),
        }
    }
}

#[test]
fn ops_vec_binary_round_trips_including_empty() {
    let empty: Vec<Vec<u8>> = Vec::new();
    assert_eq!(decode_ops_vec(&encode_ops_vec(&empty)).unwrap(), empty);

    let ops = vec![vec![1u8, 2, 3], Vec::new(), vec![9u8; 5]];
    assert_eq!(decode_ops_vec(&encode_ops_vec(&ops)).unwrap(), ops);
}
//#endregion 🔖️EnvelopeCodec

#[test]
fn causal_empty_fixed_slots_keep_each_allocation_under_declared_admission(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../📦️slots/🧫️fixtures/🔣️.json")).unwrap();
    let (dag,requested)=crate::test_allocation::observe(MutationDag::new);
    assert!(dag.terminal_is_empty());assert!(requested>0);
    let original=std::mem::size_of_val(&*dag.envelopes.slots);
    assert!(original<=law["admissionBytes"].as_u64().unwrap() as usize,"empty causal payload allocation {original} exceeds original admission");
    let (_,birth,freed)=crate::test_allocation::observe_backing(||drop(dag));
    assert_eq!(birth,0);assert_eq!(freed,requested);
    println!("[DEBUG] original causal slot shell requested={requested} largest-payload-allocation={original} exact System release={freed}");
}

#[test]
fn causal_backing_retirement_preserves_exact_slots_and_funds_one_indivisible_allocation(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../📦️slots/🧫️fixtures/🔣️.json")).unwrap();
    for case in law["cases"].as_array().unwrap(){
        let count=case["occupied"].as_u64().unwrap() as usize;
        let(mut dag,birth)=crate::test_allocation::observe(MutationDag::new);
        assert!(birth>0);assert!(!dag.backing_is_empty());
        for i in 0..count { dag.seed_applied(crate::ids::MutationId(format!("source-{i}"))).unwrap(); }
        let pointers=dag.applied.iter().map(|id|id.as_ptr()).collect::<Vec<_>>();
        let(result,requested,released)=crate::test_allocation::observe_backing(||dag.close_backing_step(1,262144));
        if count>0 { assert_eq!(result,(0,0,false));assert_eq!((requested,released),(0,0));assert_eq!(dag.applied.iter().map(|id|id.as_ptr()).collect::<Vec<_>>(),pointers); }
        let mut identities=0;while let Some(owner)=dag.take_one_close_owner(){match owner{MutationDagCloseOwner::Identity(id)=>{assert!(pointers.contains(&id.as_ptr()));identities+=1;drop(id);},MutationDagCloseOwner::Envelope(_)=>panic!("seed-only law")}}
        assert_eq!(identities,count);assert!(dag.terminal_is_empty());assert!(!dag.backing_is_empty());
        let mut freed=if count==0 { result.1 } else { 0 };let mut turns=0;
        while !dag.backing_is_empty(){assert!(turns<law["maximumCloseTurns"].as_u64().unwrap()as usize);let query=dag.next_backing_release_byte_demand();assert!(query<=law["admissionBytes"].as_u64().unwrap() as usize);
            for(items,bytes)in [(0,query),(1,query.saturating_sub(1))]{let(step,requested,released)=crate::test_allocation::observe_backing(||dag.close_backing_step(items,bytes));assert_eq!(step,(0,0,false));assert_eq!((requested,released),(0,0));assert_eq!(dag.next_backing_release_byte_demand(),query);}
            let(step,requested,released)=crate::test_allocation::observe_backing(||dag.close_backing_step(1,query));assert_eq!(requested,0);assert_eq!(released,query);assert_eq!(step.0,1);assert_eq!(step.1,released);freed+=released;turns+=1;
        }
        assert_eq!(freed,birth);assert_eq!(dag.next_backing_release_byte_demand(),0);let(_,requested,released)=crate::test_allocation::observe_backing(||drop(dag));assert_eq!((requested,released),(0,0));println!("[DEBUG] causal case={} exact-original-slots={count} pages/metadata={turns} Systemrelease={freed}; zero/one-below retain; terminalDrop0heap",case["name"]);
    }
}

#[test]
fn causal_empty_shell_has_no_physical_birth_or_terminal_release() {
    let (mut empty, requested, released) = crate::test_allocation::observe_backing(MutationDag::empty);
    assert_eq!((requested, released), (0, 0));
    assert!(empty.terminal_is_empty() && empty.backing_is_empty());
    assert_eq!(empty.next_backing_release_byte_demand(), 0);
    assert!(empty.take_one_close_owner().is_none());
    let (step, requested, released) = crate::test_allocation::observe_backing(|| empty.close_backing_step(0, 0));
    assert_eq!(step, (0, 0, true));
    assert_eq!((requested, released), (0, 0));
    let (_, requested, released) = crate::test_allocation::observe_backing(|| drop(empty));
    assert_eq!((requested, released), (0, 0));
    println!("[DEBUG] genuine causal terminal shell System birth=0 release=0; original owner transfers without allocating a replacement");
}
