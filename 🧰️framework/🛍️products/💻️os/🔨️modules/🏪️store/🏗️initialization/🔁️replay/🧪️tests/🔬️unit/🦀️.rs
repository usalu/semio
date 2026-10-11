//! 🧪️ The replay initializer loads a document exactly as a whole load does, one bounded operation per turn, under any wallet, and closes by exact retirement.

use super::*;
use super::super::fixture_mutations::demo::{AddN, DemoMutation, SetN};
use super::super::tests::DemoSnapshot;
use crate::os_store::{create_document_envelope, fund_document_store_owners, install_unscheduled_catalog, MemberStoreOwner, parse_document_pack, print_document_pack, ArtifactCommand, SupersedeInput};
use semio_framework_job::StepBudget;

const SCHEMA: &str = "demo/v1";
const ACTOR: &str = "actor:replay";
const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
const WIDE: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 64 };

type Initializer = ArtifactStoreReplayInitializer<DemoSnapshot, DemoMutation>;
type Store = ArtifactStore<DemoSnapshot, DemoMutation>;

fn set(n: i32) -> DemoMutation {
    DemoMutation::SetN(SetN { n })
}

fn add(delta: i32) -> DemoMutation {
    DemoMutation::AddN(AddN { delta })
}

async fn open(envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation>) -> Store {
    let mut store = Store::new(envelope, ActorId(crate::os_spr::LOCAL_ACTOR_ID.into())).await.expect("the fixture history is valid");
    install_unscheduled_catalog(&mut store, fund_document_store_owners(<DemoSnapshot as MemberStoreOwner<DemoMutation>>::member_store_owners_birth_demand().expect("the fixture catalog quotes its birth"), <DemoSnapshot as MemberStoreOwner<DemoMutation>>::member_store_owners)).expect("the fixture owners install");
    store
}

async fn author(store: &mut Store, command: ArtifactCommand<DemoMutation>) {
    let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
        assert!(progress.owned_bytes <= IDENTITY_CEILING);
        true
    };
    let mut identity: crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<crate::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut observer).expect("declared fixture identity authority");
    store.dispatch(command, &mut identity).await.expect("a clean command authors");
    drop(identity.pause().expect("fixture identity receipt"));
}

struct Source {
    pack: Vec<u8>,
    spr: Vec<u8>,
    operations: usize,
    expected: i32,
}

async fn source(name: &str, edits: usize, supersede: &[(usize, Option<i32>)]) -> Source {
    let mut store = open(create_document_envelope::<DemoSnapshot, DemoMutation>(SCHEMA, name, DemoSnapshot { n: Some(0) }, None)).await;
    let mut operations = 0;
    let mut values: Vec<(bool, i32)> = Vec::new();
    for index in 0..edits {
        let (batch, entries): (Vec<DemoMutation>, Vec<(bool, i32)>) = if index == 0 { (vec![set(1)], vec![(true, 1)]) } else { (vec![add(index as i32), add(1)], vec![(false, index as i32), (false, 1)]) };
        operations += batch.len();
        values.extend(entries);
        author(&mut store, ArtifactCommand::Apply { mutations: batch, transaction: None }).await;
    }
    let ids: Vec<MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
    for (position, replacement) in supersede {
        let replacement_operation = replacement.map(|value| if values[*position].0 { set(value) } else { add(value) });
        values[*position] = match replacement {
            Some(value) => (values[*position].0, *value),
            None => (false, 0),
        };
        author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target: ids[*position].clone(), replacement: replacement_operation }] }).await;
    }
    let expected = values.iter().fold(0, |state, (assign, value)| if *assign { *value } else { state + value });
    assert_eq!(store.snapshot_ref().n, Some(expected), "the source store itself folds the edited log");
    let files = print_document_pack(store.envelope()).await.expect("pack prints");
    retire_store(&mut store);
    Source { pack: files.pack, spr: files.spr, operations, expected }
}

async fn envelope(source: &Source) -> ArtifactEnvelope<DemoSnapshot, DemoMutation> {
    parse_document_pack::<DemoSnapshot, DemoMutation>(&source.pack, &source.spr).await.expect("the pair parses").into_envelope()
}

fn initializer(envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation>) -> Initializer {
    Initializer::new(envelope, SCHEMA, OperationId(7), Generation(3), ActorId(ACTOR.into()))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Yield,
    Unaffordable,
    Complete,
    Cancelled,
    Fault,
}

struct Drive {
    turns: usize,
    accounted: usize,
    spent: RetainedCloneProgress,
}

fn grant_with_items(items: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: items, ..WIDE }
}

fn accounted_turn(init: &Initializer) -> bool {
    matches!(init.phase, Phase::AdmitOwners | Phase::AdmitCatalog | Phase::Adopt { .. } | Phase::RetireCancelled | Phase::RetireFault) && !init.index_open
        || init.runtime.as_ref().is_some_and(|runtime| runtime.close_active.is_some())
}

fn turn(init: &mut Initializer, grant: RetainedCloneGrant, sequence: &mut u64, cancel: &semio_framework_job::CancelToken, authority: &semio_framework_job::JobPayloadAuthority) -> (Outcome, RetainedCloneProgress, bool) {
    let accounted = accounted_turn(init);
    let quote = init.runtime.as_ref().and_then(|runtime| runtime.close_active.as_ref()).map(|active| (active.next_demand(1 << 20), active.terminal_is_empty()));
    let shape = (init.envelope.is_some(), init.owners.is_some(), init.envelope_retirement.is_some(), init.runtime.is_some(), init.catalog.is_some(), init.actor.is_some(), init.actor_close.is_some(), init.active.is_some(), init.staged.is_some(), init.index_open, init.factory.is_some(), init.candidate.is_some());
    let before = (shape, init.phase, init.runtime.as_ref().is_some_and(|runtime| runtime.close_active.is_some()));
    let mut ownership = RetainedCloneProgress::default();
    let mut cx = StepContext::with_payload_authority(OperationId(7), Generation(3), StepBudget::new(u64::MAX, u64::MAX, grant), cancel, semio_framework_job::default_now_us, sequence, &mut ownership, authority).expect("the paid ledger matches the replay operation");
    let (outcome, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        let stepped = ArtifactStoreInitializationAuthority::step(init, &mut cx).expect("a replay turn never errs");
        match stepped {
            None => Outcome::Unaffordable,
            Some(JobOutcomeBorrow::Yield { .. }) => Outcome::Yield,
            Some(JobOutcomeBorrow::Complete { .. }) => Outcome::Complete,
            Some(JobOutcomeBorrow::Cancelled { .. }) => Outcome::Cancelled,
            Some(JobOutcomeBorrow::Fault { .. }) => Outcome::Fault,
            Some(other) => panic!("the replay initializer lends no {other:?}"),
        }
    });
    drop(cx);
    assert!(ownership.fits(grant), "a turn never spends more than its wallet: {ownership:?} of {grant:?}");
    if accounted {
        assert_eq!((heap.requested_bytes, heap.released_bytes), (ownership.retained_capacity_bytes, ownership.released_bytes), "funded turn allocations match the receipt: [DEBUG] {before:?} quote {quote:?} -> {:?} receipt {ownership:?}", init.phase);
    }
    (outcome, ownership, accounted)
}

fn run_until(init: &mut Initializer, grant: RetainedCloneGrant, limit: usize, stop: impl Fn(&Initializer, usize) -> bool, terminal: Outcome) -> (Drive, Outcome) {
    let mut sequence = 0u64;
    let cancel = semio_framework_job::root_cancel_token();
    let (mut authority, _) = semio_framework_job::JobPayloadAuthority::admit(OperationId(7), Generation(3), WIDE).expect("ledger admission").expect("the wide wallet affords the paid ledger");
    let mut drive = Drive { turns: 0, accounted: 0, spent: RetainedCloneProgress::default() };
    let mut folded = ArtifactStoreInitializationAuthority::progress(init).0;
    let result = loop {
        assert!(drive.turns < limit, "the replay finishes within {limit} turns, stuck at {:?}", init.phase);
        if stop(init, drive.turns) {
            break (drive, Outcome::Yield);
        }
        let (outcome, ownership, accounted) = turn(init, grant, &mut sequence, &cancel, &authority);
        drive.turns += 1;
        drive.accounted += usize::from(accounted);
        drive.spent = drive.spent.checked_add(ownership).expect("cumulative receipts do not overflow");
        let progress = ArtifactStoreInitializationAuthority::progress(init);
        assert!(progress.0 >= folded && progress.0 <= folded + 1, "one turn folds at most one operation: {folded} -> {}", progress.0);
        folded = progress.0;
        if outcome == terminal {
            break (drive, outcome);
        }
        assert!(matches!(outcome, Outcome::Yield | Outcome::Unaffordable), "unexpected {outcome:?} before {terminal:?}");
    };
    while !authority.terminal_is_empty() {
        authority.close_step(WIDE).expect("the paid ledger closes");
    }
    result
}

fn retire_store(store: &mut Store) {
    for _ in 0..65_536 {
        let step = store.close_owned_store_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 1 << 16, maximum_release_bytes: 1 << 16, maximum_depth: 64 }).expect("candidate store closes under owners");
        if matches!(step, RetainedCloneStep::Complete(_)) || store.close_owned_store_terminal_is_empty() {
            return;
        }
    }
    panic!("the candidate store did not close");
}

fn close_initializer(init: &mut Initializer) {
    for _ in 0..65_536 {
        if ArtifactStoreInitializationAuthority::terminal_is_empty(init) {
            return;
        }
        let step = ArtifactStoreInitializationAuthority::close_step(init, WIDE).expect("the initializer closes under a wide wallet");
        assert!(step.progress().fits(WIDE));
    }
    panic!("the initializer did not reach terminal-empty");
}

async fn load_whole(source: &Source) -> Store {
    open(envelope(source).await).await
}

#[semio_framework_async_macros::async_test]
async fn replay_equals_direct_application_under_every_grant() {
    let source = source("replay-law", 6, &[(0, Some(10)), (2, None), (5, Some(4))]).await;
    let whole = load_whole(&source).await;
    assert_eq!(whole.snapshot_ref().n, Some(source.expected));
    let mut turns_by_grant = Vec::new();
    for items in [1usize, 7, 1 << 20] {
        let mut init = initializer(envelope(&source).await);
        let (drive, outcome) = run_until(&mut init, grant_with_items(items), 100_000, |_, _| false, Outcome::Complete);
        assert_eq!(outcome, Outcome::Complete);
        assert_eq!(ArtifactStoreInitializationAuthority::progress(&init), (source.operations as u64, source.operations as u64), "every applied operation folds exactly once");
        assert!(drive.turns >= source.operations * 2, "one operation per turn, never batched: {} turns for {} operations", drive.turns, source.operations);
        assert!(drive.accounted > source.operations, "the funded ownership turns were all observed against real allocations");
        assert!(drive.spent.copied_bytes > 0);
        let mut candidate = ArtifactStoreInitializationAuthority::take_candidate(&mut init).expect("the replay yields its store");
        assert_eq!(candidate.snapshot_ref().n, Some(source.expected), "the replayed snapshot is the direct application of the effective log");
        assert_eq!(candidate.snapshot_ref(), whole.snapshot_ref());
        assert_eq!(candidate.applied_edit_ids().to_vec(), whole.applied_edit_ids().to_vec());
        assert_eq!(candidate.supersessions().keys().collect::<Vec<_>>(), whole.supersessions().keys().collect::<Vec<_>>(), "supersessions are honored");
        assert_eq!(candidate.mutation_outcomes().expect("replayed outcomes"), whole.mutation_outcomes().expect("whole outcomes"));
        assert_eq!(candidate.content_revision_now(), whole.content_revision_now(), "the replay names the revision a whole load names");
        assert_eq!(candidate.generation(), 4);
        assert!(ArtifactStoreInitializationAuthority::take_candidate(&mut init).is_none(), "the store is handed off once");
        retire_store(&mut candidate);
        drop(candidate);
        close_initializer(&mut init);
        assert!(ArtifactStoreInitializationAuthority::terminal_is_empty(&init));
        turns_by_grant.push(drive.turns);
    }
    assert!(turns_by_grant[0] >= turns_by_grant[1] && turns_by_grant[1] >= turns_by_grant[2], "a smaller wallet never needs fewer turns: {turns_by_grant:?}");
    let mut whole = whole;
    retire_store(&mut whole);
}

#[semio_framework_async_macros::async_test]
async fn supersession_replacement_and_withdrawal_are_folded_into_the_replay() {
    for (supersede, label) in [(vec![(1usize, Some(100))], "replace"), (vec![(1usize, None)], "withdraw"), (vec![(0, Some(7)), (1, Some(100)), (3, None)], "many")] {
        let source = source(label, 3, &supersede).await;
        let mut init = initializer(envelope(&source).await);
        let (_, outcome) = run_until(&mut init, WIDE, 100_000, |_, _| false, Outcome::Complete);
        assert_eq!(outcome, Outcome::Complete);
        let mut candidate = ArtifactStoreInitializationAuthority::take_candidate(&mut init).expect("store");
        assert_eq!(candidate.snapshot_ref().n, Some(source.expected), "{label}");
        assert_eq!(candidate.supersessions().len(), supersede.len(), "{label}");
        retire_store(&mut candidate);
        drop(candidate);
        close_initializer(&mut init);
    }
}

#[semio_framework_async_macros::async_test]
async fn cancellation_at_every_turn_boundary_closes_with_exact_retirement() {
    let source = source("replay-cancel", 4, &[(1, Some(9))]).await;
    let mut full = initializer(envelope(&source).await);
    let (reference, _) = run_until(&mut full, WIDE, 100_000, |_, _| false, Outcome::Complete);
    let mut candidate = ArtifactStoreInitializationAuthority::take_candidate(&mut full).expect("store");
    retire_store(&mut candidate);
    drop(candidate);
    close_initializer(&mut full);
    let mut retired_turns = 0;
    for stop in (0..reference.turns).step_by(1) {
        let mut init = initializer(envelope(&source).await);
        let (_, reached) = run_until(&mut init, WIDE, 100_000, |_, turns| turns == stop, Outcome::Complete);
        assert_eq!(reached, Outcome::Yield);
        ArtifactStoreInitializationAuthority::request_cancel(&mut init);
        let (drive, outcome) = run_until(&mut init, WIDE, 100_000, |_, _| false, Outcome::Cancelled);
        retired_turns += drive.turns;
        assert_eq!(outcome, Outcome::Cancelled, "stop {stop}");
        assert!(ArtifactStoreInitializationAuthority::take_candidate(&mut init).is_none());
        assert!(ArtifactStoreInitializationAuthority::terminal_is_empty(&init), "stop {stop}: a cancelled replay retires every born owner before it reports Cancelled");
    }
    assert!(retired_turns > reference.turns, "cancellation retired real owners across the sweep");
}

#[semio_framework_async_macros::async_test]
async fn cancellation_under_a_single_item_wallet_makes_progress_and_closes() {
    let source = source("replay-cancel-tight", 3, &[]).await;
    let mut init = initializer(envelope(&source).await);
    let (_, reached) = run_until(&mut init, grant_with_items(1), 100_000, |init, _| matches!(init.phase, Phase::Fold { mutation: 1, .. }), Outcome::Complete);
    assert_eq!(reached, Outcome::Yield);
    ArtifactStoreInitializationAuthority::request_cancel(&mut init);
    let (_, outcome) = run_until(&mut init, grant_with_items(1), 100_000, |_, _| false, Outcome::Cancelled);
    assert_eq!(outcome, Outcome::Cancelled);
    assert!(ArtifactStoreInitializationAuthority::terminal_is_empty(&init));
}

#[semio_framework_async_macros::async_test]
async fn a_cancelled_step_context_cancels_the_replay() {
    let source = source("replay-token", 2, &[]).await;
    let mut init = initializer(envelope(&source).await);
    let mut sequence = 0u64;
    let token = semio_framework_job::root_cancel_token();
    token.cancel_now();
    let mut outcome = Outcome::Yield;
    for _ in 0..100_000 {
        let mut ownership = RetainedCloneProgress::default();
        let mut cx = StepContext::new(OperationId(7), Generation(3), StepBudget::new(u64::MAX, u64::MAX, WIDE), token.clone(), semio_framework_job::default_now_us, &mut sequence, &mut ownership);
        if let Some(JobOutcomeBorrow::Cancelled { .. }) = ArtifactStoreInitializationAuthority::step(&mut init, &mut cx).expect("turn") {
            outcome = Outcome::Cancelled;
            break;
        }
    }
    assert_eq!(outcome, Outcome::Cancelled);
    assert!(ArtifactStoreInitializationAuthority::terminal_is_empty(&init));
}

#[semio_framework_async_macros::async_test]
async fn a_hostile_envelope_faults_and_closes_exactly() {
    let source = source("replay-fault", 2, &[]).await;
    let mut init = Initializer::new(envelope(&source).await, "demo/other", OperationId(7), Generation(3), ActorId(ACTOR.into()));
    let (_, outcome) = run_until(&mut init, WIDE, 100_000, |_, _| false, Outcome::Fault);
    assert_eq!(outcome, Outcome::Fault);
    assert!(ArtifactStoreInitializationAuthority::take_candidate(&mut init).is_none());
    close_initializer(&mut init);
    assert!(ArtifactStoreInitializationAuthority::terminal_is_empty(&init));
}

#[semio_framework_async_macros::async_test]
async fn the_initializer_drives_through_the_interactive_job_close_path() {
    let source = source("replay-job", 2, &[]).await;
    let mut init = initializer(envelope(&source).await);
    let (_, reached) = run_until(&mut init, WIDE, 100_000, |_, turns| turns == 5, Outcome::Complete);
    assert_eq!(reached, Outcome::Yield);
    let demand = ArtifactStoreInitializationAuthority::retirement_demands(&init, WIDE.maximum_copy_bytes).expect("the next close action is quoted");
    assert!(permits(WIDE, demand));
    ArtifactStoreInitializationAuthority::begin_close(&mut init);
    close_initializer(&mut init);
}
