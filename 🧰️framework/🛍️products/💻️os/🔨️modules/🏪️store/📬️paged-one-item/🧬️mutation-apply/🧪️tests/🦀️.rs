//! 🧪️ The generic `Mutation<S>` structural edit prepares insert, remove, replace and multi-row removal over documents of any size, restores the base through its ordered inverse rows and conserves custody through every cancellation frontier.
use super::mutation_apply_preparation_factory;
use crate::os_spr::{ApplyCapability, DiffAlgebra, MutationApplyError, MutationApplyResult, MutationComposition, MutationDiff, MutationDiffParticipation, MutationInvertibility, MutationLanguageSurface, MutationLeafDescriptor, MutationOutcome, MutationOutcomeClass, OpBinary};
use crate::os_store::{ArtifactStoreOneItemGrant, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ArtifactStoreOneItemPreparationStep, HistoryLane, SnapshotRead, SnapshotReadRegistryHandle};
use crate::Mutation;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::retirement::{OwnedValueRetirementFactory, SharedValueRetirementFactory};
use semio_framework_value::{FromValue, ToValue, ValueError, ValueRefusalKind};
use serde_json::{json, Value};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
struct Node { id: String, text: String }

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
struct Document { title: String, nodes: Vec<Node> }

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
enum Op { Insert { index: usize, id: String, text: String }, Remove { index: usize }, SetText { index: usize, text: String } }

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
struct Diff { ops: Vec<Op> }

fn refusal(message: &str) -> MutationApplyError { MutationApplyError { code: "range".into(), message: message.into(), target: Vec::new() } }

impl MutationDiff<Document> for Diff {
    fn apply(&self, base: &Document, _capability: ApplyCapability) -> MutationApplyResult<Document> {
        let mut next = base.clone();
        for op in &self.ops {
            match op {
                Op::Insert { index, id, text } if *index <= next.nodes.len() => next.nodes.insert(*index, Node { id: id.clone(), text: text.clone() }),
                Op::Remove { index } if *index < next.nodes.len() => { next.nodes.remove(*index); }
                Op::SetText { index, text } if *index < next.nodes.len() => next.nodes[*index].text = text.clone(),
                _ => return Err(refusal("node index out of range")),
            }
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) { self.ops.extend(other.ops); }
}

impl DiffAlgebra<Document> for Diff {
    fn inverse(&self, base: &Document) -> Self {
        let mut nodes = base.nodes.clone();
        let mut inverse = Vec::new();
        for op in &self.ops {
            match op {
                Op::Insert { index, id, text } => { nodes.insert(*index, Node { id: id.clone(), text: text.clone() }); inverse.push(Op::Remove { index: *index }); }
                Op::Remove { index } => { let node = nodes.remove(*index); inverse.push(Op::Insert { index: *index, id: node.id, text: node.text }); }
                Op::SetText { index, text } => { inverse.push(Op::SetText { index: *index, text: std::mem::replace(&mut nodes[*index].text, text.clone()) }); }
            }
        }
        inverse.reverse();
        Self { ops: inverse }
    }
    fn is_empty(&self) -> bool { self.ops.is_empty() }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
enum DocumentMutation { Insert { index: usize, id: String, text: String }, Remove { index: usize }, SetText { index: usize, text: String }, RemoveRange { start: usize, count: usize } }

const fn descriptor(kind: &'static str, variant: &'static str) -> MutationLeafDescriptor {
    MutationLeafDescriptor {
        schema_version: 1,
        owner: "store/paged-one-item/mutation-apply",
        semantic_kind: kind,
        display_name: kind,
        emoji: "🧬️",
        aggregate_variant: variant,
        payload_schema: "test.document",
        text_opcode: None,
        binary_tag: None,
        invertibility: MutationInvertibility::ExplicitMutation,
        diff_participation: MutationDiffParticipation::Detect,
        outcome_classes: &[MutationOutcomeClass::Applied],
        composition: MutationComposition::Atomic,
        required_language_surfaces: &[MutationLanguageSurface::Rust],
    }
}

fn out_of_range() -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, "node index out of range") }

impl Mutation<Document> for DocumentMutation {
    type Diff = Diff;
    const DESCRIPTORS: &'static [MutationLeafDescriptor] = &[descriptor("insert", "Insert"), descriptor("remove", "Remove"), descriptor("set-text", "SetText"), descriptor("remove-range", "RemoveRange")];
    fn descriptor(&self) -> &'static MutationLeafDescriptor {
        &Self::DESCRIPTORS[match self { Self::Insert { .. } => 0, Self::Remove { .. } => 1, Self::SetText { .. } => 2, Self::RemoveRange { .. } => 3 }]
    }
    fn diff(&self, _base: &Document) -> MutationOutcome<Diff> {
        MutationOutcome::new(Diff {
            ops: match self {
                Self::Insert { index, id, text } => vec![Op::Insert { index: *index, id: id.clone(), text: text.clone() }],
                Self::Remove { index } => vec![Op::Remove { index: *index }],
                Self::SetText { index, text } => vec![Op::SetText { index: *index, text: text.clone() }],
                Self::RemoveRange { start, count } => (0..*count).map(|_| Op::Remove { index: *start }).collect(),
            },
        })
    }
    fn inverse(&self, base: &Document) -> Result<Vec<Self>, ValueError> {
        let node = |index: usize| base.nodes.get(index).ok_or_else(out_of_range);
        Ok(match self {
            Self::Insert { index, .. } if *index <= base.nodes.len() => vec![Self::Remove { index: *index }],
            Self::Insert { .. } => return Err(out_of_range()),
            Self::Remove { index } => { let node = node(*index)?; vec![Self::Insert { index: *index, id: node.id.clone(), text: node.text.clone() }] }
            Self::SetText { index, .. } => vec![Self::SetText { index: *index, text: node(*index)?.text.clone() }],
            Self::RemoveRange { start, count } => (0..*count).map(|offset| node(start + offset).map(|node| Self::Insert { index: start + offset, id: node.id.clone(), text: node.text.clone() })).collect::<Result<_, _>>()?,
        })
    }
    fn inverse_rows(&self) -> usize { match self { Self::RemoveRange { count, .. } => (*count).max(1), _ => 1 } }
}

impl OpBinary for DocumentMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::ProtocolError> { serde_json::to_vec(self).map_err(|error| crate::ProtocolError::Io(error.to_string())) }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::ProtocolError> { serde_json::from_slice(bytes).map_err(|error| crate::ProtocolError::Io(error.to_string())) }
}

fn fixture() -> Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn document(json: &Value) -> Document { serde_json::from_value(json.clone()).unwrap() }
fn mutation(json: &Value) -> DocumentMutation { serde_json::from_value(json.clone()).unwrap() }
fn rows(json: &Value) -> Vec<DocumentMutation> { json.as_array().unwrap().iter().map(mutation).collect() }
fn pairs(document: &Document) -> Value { Value::Array(document.nodes.iter().map(|node| json!([node.id, node.text])).collect()) }

fn grant(copy: usize, capacity: usize) -> ArtifactStoreOneItemGrant {
    ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: 1 << 26, maximum_depth: 4096 }
}
const WIDE: usize = 1 << 26;

struct Stall;

enum Outcome { Completed { post: Document, inverse: Vec<DocumentMutation>, turns: usize, retained: usize, peak: usize }, Cancelled, Refused(ValueError) }

impl crate::snapshot_clone_preparation::ConfigApplyMutation<Document> for DocumentMutation {
    fn exchange(self, post: &mut Document) -> Result<Self, (ValueError, Self)> {
        match self {
            Self::SetText { index, text } if index < post.nodes.len() => Ok(Self::SetText { index, text: std::mem::replace(&mut post.nodes[index].text, text) }),
            other => Err((out_of_range(), other)),
        }
    }
}

fn prepare(build: &dyn Fn() -> (Document, DocumentMutation), grant: ArtifactStoreOneItemGrant, cancel_at: Option<usize>) -> Outcome {
    prepare_with(&mutation_apply_preparation_factory::<Document, DocumentMutation>(), build, grant, cancel_at)
}

fn prepare_with(factory: &Arc<dyn ArtifactStoreOneItemPreparationFactory<Document, DocumentMutation>>, build: &dyn Fn() -> (Document, DocumentMutation), grant: ArtifactStoreOneItemGrant, cancel_at: Option<usize>) -> Outcome {
    let ((document, mutation), _) = observe(build);
    let registry = SnapshotReadRegistryHandle::new();
    let root = Arc::new(document);
    let lease = registry.try_issue(Arc::clone(&root)).unwrap_or_else(|_| panic!("registry admission"));
    let authority = Arc::new(ArtifactStoreOneItemLiveAuthority { operation: semio_framework_job::OperationId(1), generation: semio_framework_job::Generation(1), base_revision: [0; 32], base_applied_edit_count: 0, next_sequence_number: 1, next_clock: crate::os_spr::HybridLogicalTimestamp::new(1, 0), actor: "actor".into(), line: None, group_id: None, stamped_edit_id: None });
    let request = ArtifactStoreOneItemPreparationRequest { operation: semio_framework_job::OperationId(1), generation: semio_framework_job::Generation(1), base_revision: [0; 32], lane: HistoryLane::Document, authority, base: SnapshotRead::new(Arc::clone(&root), lease), mutation, mutation_retirement: Arc::new(OwnedValueRetirementFactory::<DocumentMutation>::default()), snapshot_retirement: Arc::new(SharedValueRetirementFactory::<Document>::default()) };
    let footprint = factory.preflight(&request.mutation, request.lane);
    let begun = factory.begin(request, grant);
    let mut owner = match begun {
        Ok((owner, receipt)) => { assert!(receipt.fits(grant.retained_grant())); owner }
        Err((error, request)) => { drop(request); return if footprint.is_err() { Outcome::Refused(error) } else { std::panic::panic_any(Stall) }; }
    };
    let mut prepared = None;
    let (mut idle, mut turns, mut refused, mut retained) = (0, 0, None, 0usize);
    let (mut live, mut peak) = (0usize, 0usize);
    for turn in 0..1_000_000 {
        if cancel_at == Some(turn) { owner.cancel(); break; }
        match owner.advance(grant) {
            Ok(ArtifactStoreOneItemPreparationStep::Prepared(_, progress)) => { assert!(progress.fits(grant.retained_grant())); retained += progress.retained_capacity_bytes; live += progress.retained_capacity_bytes; peak = peak.max(live); turns = turn + 1; prepared = owner.take_prepared(); break; }
            Ok(ArtifactStoreOneItemPreparationStep::Progress(_, progress)) => { assert!(progress.fits(grant.retained_grant()), "receipt {progress:?} exceeds {grant:?}"); retained += progress.retained_capacity_bytes; live += progress.retained_capacity_bytes; peak = peak.max(live); live = live.saturating_sub(progress.released_bytes); idle = 0; }
            Ok(ArtifactStoreOneItemPreparationStep::Blocked) => { idle += 1; if idle >= 64 { owner.cancel(); owner.begin_close(); close(&mut owner); drop(owner); std::panic::panic_any(Stall); } }
            Err(error) => { refused = Some(error); break; }
        }
    }
    let outcome = match (prepared, refused) {
        (Some(prepared), _) => Outcome::Completed { post: (*prepared.post_snapshot).clone(), inverse: prepared.edit.inverse.iter().cloned().collect(), turns, retained, peak },
        (None, Some(error)) => Outcome::Refused(error),
        (None, None) => Outcome::Cancelled,
    };
    owner.begin_close();
    close(&mut owner);
    assert_eq!(observe(|| drop(owner)).1.requested_bytes, 0);
    outcome
}

fn close(owner: &mut Box<dyn ArtifactStoreOneItemPreparation<Document, DocumentMutation>>) {
    for _ in 0..100_000 {
        if owner.terminal_is_empty() { return; }
        let copy = owner.next_close_copy_byte_demand().unwrap();
        let quote = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: owner.next_close_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: owner.next_close_release_byte_demand().unwrap(), maximum_depth: owner.next_close_depth_demand().unwrap() };
        let (step, heap) = observe(|| owner.close_step(quote).unwrap());
        assert!(step.progress().fits(quote.retained_grant()), "close receipt {:?} exceeds its quote {quote:?}", step.progress());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes), "close accounting disagrees with its receipt");
    }
    panic!("preparation did not close under its quoted grants");
}

fn completed(outcome: Outcome) -> (Document, Vec<DocumentMutation>, usize) {
    match outcome { Outcome::Completed { post, inverse, turns, .. } => (post, inverse, turns), Outcome::Cancelled => panic!("cancelled"), Outcome::Refused(error) => panic!("refused: {error}") }
}

fn floor(build: &dyn Fn() -> (Document, DocumentMutation)) -> usize {
    static HOOK: std::sync::Once = std::sync::Once::new();
    HOOK.call_once(|| { let previous = std::panic::take_hook(); std::panic::set_hook(Box::new(move |info| { if !info.payload().is::<Stall>() { previous(info) } })); });
    let mut copy = 1usize;
    loop {
        if matches!(catch_unwind(AssertUnwindSafe(|| prepare(build, grant(copy, WIDE), None))), Ok(Outcome::Completed { .. })) { return copy; }
        copy *= 2;
        assert!(copy <= 1 << 24, "no copy grant completes the case");
    }
}

#[test]
fn mutation_apply_prepares_structural_edits_with_ordered_inverse_rows() {
    let policy = fixture();
    let base = document(&policy["base"]);
    for case in policy["cases"].as_array().unwrap() {
        let build = || (base.clone(), mutation(&case["mutation"]));
        let floor = floor(&build);
        let mut turns = Vec::new();
        for copy in [floor, floor + 7, WIDE] {
            let (post, inverse, count) = completed(prepare(&build, grant(copy, WIDE), None));
            assert_eq!(pairs(&post), case["post"], "post of {} at copy {copy}", case["name"]);
            assert_eq!(inverse, rows(&case["inverse"]), "inverse rows of {} at copy {copy}", case["name"]);
            let restored = inverse.into_iter().fold(post, |current, row| completed(prepare(&|| (current.clone(), row.clone()), grant(copy, WIDE), None)).0);
            assert_eq!(restored, base, "ordered inverse rows restore the base for {}", case["name"]);
            turns.push(count);
        }
        for cancel_at in policy["cancelAt"].as_array().unwrap().iter().map(|turn| turn.as_u64().unwrap() as usize) {
            let outcome = prepare(&build, grant(floor, WIDE), Some(cancel_at));
            assert_eq!(matches!(outcome, Outcome::Completed { .. }), cancel_at >= turns[0], "cancel at {cancel_at} of {} turns for {}", turns[0], case["name"]);
        }
        eprintln!("[DEBUG] mutation-apply {} floor={floor} turns={turns:?}; clone, inverse rows, apply, displaced retirement, seal and cancellation conserved", case["name"]);
    }
}

#[test]
fn mutation_apply_admits_documents_beyond_the_one_item_cap_when_turns_fund_the_snapshot() {
    let text = "x".repeat(1024);
    let large = Document { title: "large".into(), nodes: (0..1400).map(|index| Node { id: format!("n{index}"), text: text.clone() }).collect() };
    let build = || (large.clone(), DocumentMutation::SetText { index: 700, text: "edited".into() });
    let bytes: usize = large.nodes.iter().map(|node| node.id.len() + node.text.len()).sum();
    assert!(bytes > crate::os_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, "fixture exceeds the one-item cap");
    let factory = mutation_apply_preparation_factory::<Document, DocumentMutation>();
    let footprint = factory.preflight(&build().1, HistoryLane::Document).unwrap();
    assert!(footprint.retained_bytes < 4096 && footprint.is_admissible(), "the footprint counts only the mutation");
    let (post, inverse, _) = completed(prepare(&build, grant(1 << 16, 1 << 26), None));
    assert_eq!(post.nodes[700].text, "edited");
    assert_eq!(post.nodes.len(), 1400);
    assert_eq!(inverse, vec![DocumentMutation::SetText { index: 700, text }]);
    let stalled = catch_unwind(AssertUnwindSafe(|| prepare(&build, grant(1 << 16, 1 << 20), None)));
    assert!(stalled.is_err_and(|payload| payload.is::<Stall>()), "a turn that cannot fund the snapshot-sized apply never completes");
    eprintln!("[DEBUG] mutation-apply admitted a {bytes}-byte document; footprint={}", footprint.retained_bytes);
}

#[test]
fn mutation_apply_refuses_semantic_errors_and_closes() {
    let policy = fixture();
    let base = document(&policy["base"]);
    for bad in [DocumentMutation::Remove { index: 9 }, DocumentMutation::SetText { index: 9, text: "x".into() }, DocumentMutation::RemoveRange { start: 3, count: 2 }, DocumentMutation::Insert { index: 9, id: "x".into(), text: "x".into() }] {
        let outcome = prepare(&|| (base.clone(), bad.clone()), grant(1 << 16, WIDE), None);
        assert!(matches!(outcome, Outcome::Refused(_)), "{bad:?} must be refused");
    }
}

#[test]
fn mutation_apply_wire_values_round_trip_through_the_value_contract() {
    let policy = fixture();
    for case in policy["cases"].as_array().unwrap() {
        let original = mutation(&case["mutation"]);
        assert_eq!(DocumentMutation::from_value(original.to_value()).unwrap(), original);
        assert_eq!(<DocumentMutation as OpBinary>::decode_op(&original.encode_op().unwrap()).unwrap(), original);
    }
}

#[test]
fn config_apply_reaches_prepared_in_the_same_harness() {
    let policy = fixture();
    let base = document(&policy["base"]);
    let factory = crate::snapshot_clone_preparation::config_apply_preparation_factory::<Document, DocumentMutation>();
    let build = || (base.clone(), DocumentMutation::SetText { index: 1, text: "changed".into() });
    let Outcome::Completed { post, inverse, turns, retained, peak } = prepare_with(&factory, &build, grant(1 << 16, WIDE), None) else { panic!("config apply must complete") };
    eprintln!("[DEBUG] sealer retained capacity for a 4-node record: cumulative {retained}, peak live {peak}");
    let tiny = Document { title: String::new(), nodes: vec![Node { id: "a".into(), text: "b".into() }] };
    let Outcome::Completed { retained: tiny_retained, peak: tiny_peak, .. } = prepare_with(&factory, &|| (tiny.clone(), DocumentMutation::SetText { index: 0, text: "c".into() }), grant(1 << 16, WIDE), None) else { panic!("tiny must complete") };
    eprintln!("[DEBUG] sealer cumulative retained capacity for a 1-node record cumulative {tiny_retained}, peak live {tiny_peak}");
    assert_eq!(post.nodes[1].text, "changed");
    assert_eq!(inverse, vec![DocumentMutation::SetText { index: 1, text: "beta é".into() }]);
    eprintln!("[DEBUG] config apply reached Prepared in {turns} turns through the same harness");
}
