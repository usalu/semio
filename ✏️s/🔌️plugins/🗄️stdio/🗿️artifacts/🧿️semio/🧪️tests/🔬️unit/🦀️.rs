use super::*;
use {semio_framework_artifact_reference::ArtifactDialect,crate::dsl::ArtifactPack,crate::dsl::SpaceMember};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use semio_framework_value::retained_clone::RetainedCloneStep;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn subset_dialect(subset: &str) -> ArtifactDialect {
    ArtifactDialect { artifact_kind: SEMIO_ARTIFACT_SCHEMA_ID.into(), standard: "v1".into(), subset: subset.into() }
}

fn quoted(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

fn retire_snapshot<P: Send + Sync + 'static>(factory: &impl dsl::SnapshotRetirementFactory<P>, snapshot: Arc<P>) -> Box<dyn semio_framework_value::ErasedSnapshotRetirement> {
    match dsl::SnapshotRetirementFactory::retire(factory, snapshot, semio_s_artifact_stdio_contract::testing::unbounded_grant()) {
        Ok((retirement, _)) => retirement,
        Err((error, _)) => panic!("snapshot retirement admits: {error:?}"),
    }
}

/// 🧹️ A large nested snapshot is dismantled over bounded turns; completion is accepted only
/// after the concrete cursor has surrendered every owned node and byte.
#[test]
fn large_nested_snapshot_retirement_is_multi_turn_and_terminal_empty() {
    let snapshot = text::SemioTextSnapshot { schema: "stdio.semio.text".into(), runs: (0..128).map(|index| text::SemioTextRun { language: format!("language-{index}"), content: "payload".repeat(128), marks: Vec::new() }).collect() };
    let factory = SemioSnapshotRetirementFactory::<text::SemioTextSnapshot>(PhantomData);
    let mut retirement = retire_snapshot(&factory, Arc::new(snapshot));
    let mut turns = 0;
    loop {
        turns += 1;
        let grant = quoted(retirement.next_demand(0).expect("quoted close demand"));
        let step = retirement.close_step(grant).expect("bounded close step");
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break;
        }
        assert!(turns < 20_000, "retirement stopped making progress");
    }
    assert!(turns > 1);
    assert!(retirement.terminal_is_empty());
}

/// 🧯️ Cancellation/app-close pumps may present an empty grant; ownership remains intact and
/// resumable until a later granted turn.
#[test]
fn empty_retirement_grant_preserves_resumable_ownership() {
    let factory = SemioSnapshotRetirementFactory::<text::SemioTextSnapshot>(PhantomData);
    let mut retirement = retire_snapshot(&factory, Arc::new(text::SemioTextSnapshot::default()));
    let empty = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 0 };
    assert_eq!(retirement.close_step(empty).expect("empty grant"), RetainedCloneStep::Progress(Default::default()));
    assert!(!retirement.terminal_is_empty());
    let mut turns = 0;
    while !matches!(retirement.close_step(quoted(retirement.next_demand(0).expect("quoted close demand"))).expect("resumed retirement"), RetainedCloneStep::Complete(_)) {
        turns += 1;
        assert!(turns < 20_000, "resumed retirement stopped making progress");
    }
    assert!(retirement.terminal_is_empty());
}

/// 🔒️ A cloned public read lease keeps the exact owner alive; the retirement never drops one
/// Arc and lets another reader become an unbounded last owner behind its terminal witness.
#[test]
fn shared_snapshot_read_blocks_until_the_exact_owner_is_unique() {
    let factory = SemioSnapshotRetirementFactory::<text::SemioTextSnapshot>(PhantomData);
    let snapshot = Arc::new(text::SemioTextSnapshot::default());
    let reader = snapshot.clone();
    let mut retirement = retire_snapshot(&factory, snapshot);
    for _ in 0..8 {
        let _ = retirement.close_step(quoted(retirement.next_demand(0).expect("quoted close demand")));
        assert!(!retirement.terminal_is_empty(), "a shared owner never reaches its terminal-empty witness");
    }
    drop(reader);
    let mut turns = 0;
    while !retirement.terminal_is_empty() {
        retirement.close_step(quoted(retirement.next_demand(0).expect("quoted close demand"))).expect("unique owner retirement");
        turns += 1;
        assert!(turns < 20_000, "unique owner retirement stopped making progress");
    }
}

/// 🧸️ Every composable subset must be reachable through `create_semio_member` — an unlisted
/// subset would fail with an unhelpful error rather than a named one.
#[semio_framework_async_macros::async_test]
async fn every_composable_subset_dispatches_to_a_real_child_store() {
    for subset in composable_subsets() {
        let dialect = subset_dialect(subset);
        // An empty pack is rejected by the production member, so this asserts the DISPATCH
        // reached a real typed variant rather than falling through to "no member kind".
        let error = match create_semio_member("probe", &dialect, &[], protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await {
            Ok(_) => panic!("empty genesis pack must be rejected"),
            Err(error) => error,
        };
        assert!(!error.to_string().contains("no member dialect"), "subset {subset} is not wired into the child-store dispatch");
    }
    let unknown = match create_semio_member("probe", &subset_dialect("not-a-subset"), &[], protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await {
        Ok(_) => panic!("unknown subset must be rejected"),
        Err(error) => error,
    };
    assert!(unknown.to_string().contains("no member dialect"));
}

/// 🧸️ `create_semio_member` must MINT a real child store and `open_semio_member` must REOPEN it
/// from its own persisted envelope — the whole point of "children have their own version
/// history". The reopen half only works because the persisted `.spr` now carries the dialect the
/// subset is recovered from.
#[semio_framework_async_macros::async_test]
async fn a_semio_member_mints_and_reopens_a_real_child_envelope() {
    let dialect = subset_dialect("mesh");

    let seed = SemioMeshSnapshot::default();
    let child = create_semio_member("mesh-child-1", &dialect, &seed.encode_pack(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("create child");
    assert_eq!(child.document_id().await, "mesh-child-1");

    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "mesh-child-1".into(), dialect };
    let mut reopened = open_semio_member(&expected, None, &child.envelope_pack_bytes().await.expect("envelope pack"), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("reopen child");
    assert_eq!(reopened.document_pack_bytes().await.expect("head pack"), child.document_pack_bytes().await.expect("head pack"), "the reopened child diverged from the persisted one");
    let mut child = child;
    close_member(&mut reopened);
    close_member(&mut child);
}

/// 🪤 A snapshot read lease ALIASES the very snapshot the next commit displaces, so the displaced
/// owner cannot become unique while that lease is still in the registry. The store disposer must
/// therefore reach the lease registry BEFORE displaced owners — walking them the other way round
/// wedges the close on an alias its own cursor has not reached yet.
///
/// 🔄️ Restated for `SnapshotReadLeaseRegistry::try_release_aliased` (framework `🏪️store`, landed
/// 2026-09-22): a read handed back while its root is STILL aliased — exactly this case, because the
/// displaced-owner queue owns the snapshot the lease aliased — now frees its slot on the returning
/// thread and never parks in the returned queue. `returned_snapshot_read_count() == 2` was therefore
/// asserting the OLD return path, not this law, and is gone. What the law forbids is unchanged and
/// is pinned directly instead: the cursor starts in `ReturnedLeases`, it REFUSES (`Blocked`) while
/// any lease is still in the registry rather than walking on into the displaced-owner queue, and
/// once the registry is clear the same close converges to `Complete` with its terminal-empty
/// witness.
#[semio_framework_async_macros::async_test]
async fn returned_read_leases_retire_before_the_displaced_owners_that_alias_them() {
    use crate::standards::v1::subsets::value::schema::mutations::{set_value::SetValue, SemioValueMutation};
    use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};

    let seed = SemioValueSnapshot::default();
    let mut envelope = dsl::create_document_envelope::<SemioValueSnapshot, SemioValueMutation>(STDIO_SEMIOVALUE_DOCUMENT_SCHEMA, "value-close-law", seed, None);
    envelope.dialect = Some(subset_dialect("value"));
    let runtime = dsl::ArtifactStoreInitializationRuntime::new("value-close-law", STDIO_SEMIOVALUE_DOCUMENT_SCHEMA, envelope.vcs.genesis.facts().share_snapshot(), envelope.vcs.genesis.facts().digest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()));
    let mut store = dsl::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, <SemioValueSnapshot as dsl::MemberStoreOwner<SemioValueMutation>>::member_store_owners(semio_s_artifact_stdio_contract::testing::unbounded_grant()).map(|(owners, _)| owners).unwrap_or_else(|_| panic!("value store owners admit")));

    let first = store.snapshot_read().expect("first snapshot read lease");
    let second = store.snapshot_read().expect("second snapshot read lease");
    drop(first);
    drop(second);
    assert_eq!(store.outstanding_snapshot_read_count(), 0, "both leases were handed back");
    assert!(store.returned_snapshot_read_count() <= 2, "a returned lease may be reclaimed eagerly or parked, never double-counted");

    // The first commit parks the aliased snapshot in the tail-undo cache; the SECOND one evicts it
    // into the displaced-owner queue, which is where the close cursor meets it.
    for step in ["first", "second"] {
        semio_framework_plugin::with_authoring_identity!(|identity| store.apply_one(store.generation(), SemioValueMutation::SetValue(SetValue { path: Vec::new(), value: SemioValue::Str { value: step.into() } }), dsl::HistoryLane::Document, &mut identity).await).expect("one-item commit");
    }

    // The laws this pins reach their close through a SECOND decision — an undo of the commit they
    // just applied, which is what repopulates the tail-undo cache from a displaced snapshot.
    semio_framework_plugin::with_authoring_identity!(|identity| SpaceMember::undo(&mut store, &mut identity).await).expect("derived undo of the last commit");

    // 🚧️ The ordering itself: a lease still in the registry must stop the cursor IN its first phase.
    // If the disposer walked displaced owners first it would meet this alias as a shared retirement
    // it can never release and the close would wedge instead of waiting here.
    let outstanding = store.snapshot_read().expect("third snapshot read lease");
    assert!(store.close_owned_phase_witness().starts_with("semio/ReturnedLeases/"), "the close cursor must open on the lease registry, got {}", store.close_owned_phase_witness());
    let refused = store.close_owned_step(quoted(store.close_owned_demands(4096).expect("quoted store close demand"))).expect("bounded value-store close");
    assert!(matches!(refused, RetainedCloneStep::Progress(progress) if progress == Default::default()), "a close that still holds a snapshot read must refuse, not advance past the lease registry: {refused:?}");
    assert!(store.close_owned_phase_witness().starts_with("semio/ReturnedLeases/"), "a refused close stays in the lease phase, got {}", store.close_owned_phase_witness());
    drop(outstanding);

    for _ in 0..100_000 {
        let grant = quoted(store.close_owned_demands(4096).expect("quoted store close demand"));
        let step = store.close_owned_step(grant).expect("bounded value-store close");
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(store.close_owned_terminal_is_empty(), "a Complete close owes its terminal-empty witness");
            return;
        }
    }
    panic!("value store close must converge");
}

/// 🧹️ Every member store is retired explicitly over bounded turns before it drops — the store's
/// drop witness refuses an unretired owner.
fn close_member(member: &mut SemioMembers) {
    for _ in 0..100_000 {
        let grant = quoted(member.close_owned_demands(4096).expect("quoted member close demand"));
        let step = member.close_owned_step(grant).expect("bounded member close");
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(member.close_owned_terminal_is_empty());
            return;
        }
    }
    panic!("member close must converge");
}

/// 🧭️ Genuine superseded history and retained replay prefixes reach the same bounded Semio close cursor.
#[semio_framework_async_macros::async_test]
async fn supersession_and_prefix_owners_follow_the_exact_semio_close_cursor() {
    use crate::standards::v1::subsets::value::schema::mutations::{set_value::SetValue, SemioValueMutation};
    use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
    let case: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/close-frontiers/🔣️.json")).expect("independent JSON close corpus");
    let document = case["document"].as_str().unwrap();
    let seed = SemioValueSnapshot::default();
    let mut envelope = dsl::create_document_envelope::<SemioValueSnapshot, SemioValueMutation>(STDIO_SEMIOVALUE_DOCUMENT_SCHEMA, document, seed, None);
    envelope.dialect = Some(subset_dialect("value"));
    let runtime = dsl::ArtifactStoreInitializationRuntime::new(document, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA, envelope.vcs.genesis.facts().share_snapshot(), envelope.vcs.genesis.facts().digest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()));
    let mut store = dsl::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, <SemioValueSnapshot as dsl::MemberStoreOwner<SemioValueMutation>>::member_store_owners(semio_s_artifact_stdio_contract::testing::unbounded_grant()).map(|(owners, _)| owners).unwrap_or_else(|_| panic!("value store owners admit")));
    for value in case["values"].as_array().unwrap() {
        semio_framework_plugin::with_authoring_identity!(|identity| store.apply_one(store.generation(), SemioValueMutation::SetValue(SetValue { path: Vec::new(), value: SemioValue::Str { value: value.as_str().unwrap().into() } }), dsl::HistoryLane::Document, &mut identity).await).expect("real registered value operation");
    }
    let ids: Vec<_> = store.mutation_ops().expect("authored operation identities").into_iter().map(|operation| operation.mutation_id).collect();
    semio_framework_plugin::with_authoring_identity!(|identity| store.dispatch(dsl::ArtifactCommand::Supersede { scope: None, inputs: vec![dsl::SupersedeInput { target: ids[case["withdraw_index"].as_u64().unwrap() as usize].clone(), replacement: None }] }, &mut identity).await).expect("real authored withdrawal");
    assert_eq!(store.supersessions().len(), case["expected_supersessions"].as_u64().unwrap() as usize);
    let prefix = store.state_before(&ids[case["prefix_target_index"].as_u64().unwrap() as usize], &dsl::HistoryInputDrafts::default()).expect("real retained prefix admission");
    assert!(matches!(&prefix.root, SemioValue::Str { value } if value == case["values"][1].as_str().unwrap()));
    drop(prefix);
    let max_items = case["max_items"].as_u64().unwrap() as usize;
    let max_bytes = case["max_bytes"].as_u64().unwrap() as usize;
    let mut reached = 0;
    let phases = case["phases"].as_array().unwrap();
    for _ in 0..case["max_steps"].as_u64().unwrap() {
        let witness = store.close_owned_phase_witness();
        if reached < phases.len() && witness.starts_with(&format!("semio/{}/", phases[reached].as_str().unwrap())) { reached += 1; }
        let demand = store.close_owned_demands(max_bytes).expect("quoted genuine Semio close demand");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: max_items, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        match store.close_owned_step(grant).expect("bounded genuine Semio close") {
            RetainedCloneStep::Progress(progress) => assert!(progress.fits(grant) && progress.copied_items <= max_items),
            RetainedCloneStep::Complete(_) => {
                assert!(store.close_owned_terminal_is_empty());
                assert_eq!(store.close_owned_phase_witness(), case["terminal_phase_witness"].as_str().unwrap(), "Store removes only the completed terminal-empty disposer");
                assert_eq!(reached, phases.len() - 1, "every literal owner phase precedes completion");
                println!("[DEBUG] genuine supersession/prefix Semio close reached terminal under {max_items}/{max_bytes}");
                return;
            }
        }
    }
    panic!("genuine Semio close did not converge");
}
