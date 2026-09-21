use super::*;
use crate::dsl::{os_io::ArtifactDialect, ArtifactPack, SpaceMember};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn subset_dialect(subset: &str) -> ArtifactDialect {
    ArtifactDialect { artifact_kind: SEMIO_ARTIFACT_SCHEMA_ID.into(), standard: "v1".into(), subset: subset.into() }
}

/// 🧹️ A large nested snapshot is dismantled over bounded turns; completion is accepted only
/// after the concrete cursor has surrendered every owned node and byte.
#[test]
fn large_nested_snapshot_retirement_is_multi_turn_and_terminal_empty() {
    let snapshot = text::SemioTextSnapshot { schema: "stdio.semio.text".into(), runs: (0..128).map(|index| text::SemioTextRun { language: format!("language-{index}"), content: "payload".repeat(128), marks: Vec::new() }).collect() };
    let factory = SemioSnapshotRetirementFactory::<text::SemioTextSnapshot>(PhantomData);
    let mut retirement = dsl::SnapshotRetirementFactory::retire(&factory, Arc::new(snapshot));
    let mut turns = 0;
    loop {
        turns += 1;
        match retirement.close_step(7, 31).expect("bounded close step") {
            dsl::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 7);
                assert!(released_bytes <= 31);
            }
            dsl::SnapshotRetirementStep::Blocked => panic!("owned snapshot retirement has no external wait"),
            dsl::SnapshotRetirementStep::Complete => break,
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
    let mut retirement = dsl::SnapshotRetirementFactory::retire(&factory, Arc::new(text::SemioTextSnapshot::default()));
    assert_eq!(retirement.close_step(0, 0).expect("empty grant"), dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    assert!(!retirement.terminal_is_empty());
    // Resumable: a later granted turn finishes the very same retirement to terminal-empty.
    let mut turns = 0;
    while retirement.close_step(8, 64).expect("resumed retirement") != dsl::SnapshotRetirementStep::Complete {
        turns += 1;
        assert!(turns < 20_000, "resumed retirement stopped making progress");
    }
    assert!(retirement.terminal_is_empty());
}

/// 🔒️ A cloned public read lease keeps the exact owner blocked; the disposer never drops one
/// Arc and lets another reader become an unbounded last owner behind its terminal witness.
#[test]
fn shared_snapshot_read_blocks_until_the_exact_owner_is_unique() {
    let factory = SemioSnapshotRetirementFactory::<text::SemioTextSnapshot>(PhantomData);
    let snapshot = Arc::new(text::SemioTextSnapshot::default());
    let reader = snapshot.clone();
    let mut retirement = dsl::SnapshotRetirementFactory::retire(&factory, snapshot);
    assert_eq!(retirement.close_step(8, 64).expect("shared owner check"), dsl::SnapshotRetirementStep::Blocked);
    assert!(!retirement.terminal_is_empty());
    drop(reader);
    while retirement.close_step(8, 64).expect("unique owner retirement") != dsl::SnapshotRetirementStep::Complete {}
    assert!(retirement.terminal_is_empty());
}

/// 🧸️ Every composable subset must be reachable through `create_semio_member` — an unlisted
/// subset would fail with an unhelpful error rather than a named one.
#[semio_framework_async_macros::async_test]
async fn every_composable_subset_dispatches_to_a_real_child_store() {
    for subset in composable_subsets() {
        let dialect = subset_dialect(subset);
        // An empty pack is rejected by the production member, so this asserts the DISPATCH
        // reached a real typed variant rather than falling through to "no member kind".
        let error = match create_semio_member("probe", &dialect, &[]).await {
            Ok(_) => panic!("empty genesis pack must be rejected"),
            Err(error) => error,
        };
        assert!(!error.to_string().contains("no member dialect"), "subset {subset} is not wired into the child-store dispatch");
    }
    let unknown = match create_semio_member("probe", &subset_dialect("not-a-subset"), &[]).await {
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
    let child = create_semio_member("mesh-child-1", &dialect, &seed.encode_pack()).await.expect("create child");
    assert_eq!(child.document_id().await, "mesh-child-1");

    let expected = dsl::os_io::ArtifactRef { artifact_id: "mesh-child-1".into(), dialect };
    let mut reopened = open_semio_member(&expected, None, &child.envelope_pack_bytes().await.expect("envelope pack")).await.expect("reopen child");
    assert_eq!(reopened.document_pack_bytes().await.expect("head pack"), child.document_pack_bytes().await.expect("head pack"), "the reopened child diverged from the persisted one");
    let mut child = child;
    close_member(&mut reopened);
    close_member(&mut child);
}

/// 🪤 A snapshot read lease ALIASES the very snapshot the next commit displaces, so the displaced
/// owner cannot become unique until the returned lease is retired. The store disposer must
/// therefore drain returned read leases BEFORE displaced owners — draining them the other way
/// round wedges the close on a lease its own cursor has not reached yet.
#[semio_framework_async_macros::async_test]
async fn returned_read_leases_retire_before_the_displaced_owners_that_alias_them() {
    use crate::standards::v1::subsets::value::schema::mutations::{set_snapshot::SetSnapshot, SemioValueMutation};
    use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};

    let seed = SemioValueSnapshot::default();
    let mut envelope = dsl::create_document_envelope::<SemioValueSnapshot, SemioValueMutation>(STDIO_SEMIOVALUE_DOCUMENT_SCHEMA, "value-close-law", seed.clone(), None);
    envelope.dialect = Some(subset_dialect("value"));
    let digest = *semio_framework_hash::hash(&seed.encode_pack()).as_bytes();
    let runtime = dsl::ArtifactStoreInitializationRuntime::new("value-close-law", STDIO_SEMIOVALUE_DOCUMENT_SCHEMA, seed, digest);
    let mut store = dsl::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, <SemioValueSnapshot as dsl::MemberStoreOwner<SemioValueMutation>>::member_store_owners());

    let first = store.snapshot_read().expect("first snapshot read lease");
    let second = store.snapshot_read().expect("second snapshot read lease");
    drop(first);
    drop(second);
    assert_eq!(store.outstanding_snapshot_read_count(), 0, "both leases were handed back");
    assert_eq!(store.returned_snapshot_read_count(), 2, "both leases are waiting to be retired");

    // The first commit parks the aliased snapshot in the tail-undo cache; the SECOND one evicts it
    // into the displaced-owner queue, which is where the close cursor meets it.
    for step in ["first", "second"] {
        let next = SemioValueSnapshot { root: SemioValue::Str { value: step.into() }, ..SemioValueSnapshot::default() };
        store.apply_one(store.generation(), SemioValueMutation::SetSnapshot(SetSnapshot { snapshot: next }), None, dsl::HistoryLane::Document).await.expect("one-item commit");
    }

    // The laws this pins reach their close through a SECOND decision — an undo of the commit they
    // just applied, which is what repopulates the tail-undo cache from a displaced snapshot.
    SpaceMember::undo(&mut store).await.expect("derived undo of the last commit");

    for _ in 0..100_000 {
        match store.close_owned_step(1, 4096).expect("bounded value-store close") {
            dsl::SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 4096),
            dsl::SnapshotRetirementStep::Complete => {
                assert!(store.close_owned_terminal_is_empty(), "a Complete close owes its terminal-empty witness");
                return;
            }
            dsl::SnapshotRetirementStep::Blocked => panic!(
                "value store close blocked with outstanding_reads={} returned_reads={} phase={}",
                store.outstanding_snapshot_read_count(),
                store.returned_snapshot_read_count(),
                store.close_owned_phase_witness()
            ),
        }
    }
    panic!("value store close must converge");
}

/// 🧹️ Every member store is retired explicitly over bounded turns before it drops — the store's
/// drop witness refuses an unretired owner.
fn close_member(member: &mut SemioMembers) {
    for _ in 0..100_000 {
        match member.close_owned_step(1, 4096).expect("bounded member close") {
            dsl::SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 4096),
            dsl::SnapshotRetirementStep::Complete => {
                assert!(member.close_owned_terminal_is_empty());
                return;
            }
            dsl::SnapshotRetirementStep::Blocked => panic!("a unique member has no shared close wait"),
        }
    }
    panic!("member close must converge");
}
