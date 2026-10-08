//! 🐢️ The bounded retained initializer's reload law (gap N17, design §16.6), mounted in `app` beside the initializer it
//! drives: the reloaded store is handed off through the job's own `take_candidate`, which is private to this module, so the
//! law reads it here instead of widening it. Moved out of `🧾️document-archive-load-legs` (S3-W2A, 2026-10-02) when the test
//! there stopped compiling against the private handoff; the law itself is S3-W1G's and unchanged.

use super::*;
use crate::test_app_mutation_fixture::{TestMutation, TestSnapshot};

/// 📜️ The document schema the reloaded envelope carries — the single-document app shape of `🧾️document-archive-load-legs`.
const RELOAD_DOCUMENT_SCHEMA: &str = "semio.test.single-document/v1";

/// 🧳️ Retain a real backing allocation through the public erased initializer job.
struct PhysicalInitializerAuthority {
    buffer: Option<Vec<u8>>,
}

impl ArtifactStoreInitializationAuthority<TestSnapshot, TestMutation> for PhysicalInitializerAuthority {
    fn step(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome { semio_framework_job::StepOutcome::Yield }
    fn request_cancel(&mut self) {}
    fn take_candidate(&mut self) -> Option<store::ArtifactStore<TestSnapshot, TestMutation>> { None }
    fn next_close_byte_demand(&self) -> usize { self.buffer.as_ref().map_or(0, Vec::capacity) }
    fn begin_close(&mut self) {}
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        let Some(buffer) = self.buffer.as_ref() else { return Ok(PluginCloseStep::Complete) };
        if maximum_items == 0 || maximum_bytes < buffer.capacity() { return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
        let bytes = self.buffer.take().unwrap().capacity();
        Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes })
    }
    fn terminal_is_empty(&self) -> bool { self.buffer.is_none() }
}

#[test]
fn initializer_job_retains_terminal_authority_until_its_physical_box_is_funded() {
    use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    for row in fixture["cases"].as_array().unwrap() {
        let extent = row["physicalBytes"].as_u64().unwrap() as usize;
        let mut buffer = Vec::new(); buffer.try_reserve_exact(extent).unwrap();
        assert_eq!(buffer.capacity(), extent);
        let mut job = ArtifactStoreInitializationJob::new(Box::new(PhysicalInitializerAuthority { buffer: Some(buffer) }));
        let demand = job.next_close_byte_demand();
        let first = job.close_step(1, row["callerBytes"].as_u64().unwrap() as usize);
        if row["releasedBytes"].as_u64().unwrap() == 0 { job.close_step(1, admission); }
        assert!(job.accept_terminal_failure());
        assert!(job.authority.is_some());
        assert!(!job.terminal_is_empty());
        let box_bytes = std::mem::size_of::<PhysicalInitializerAuthority>();
        let terminal_demand = job.next_close_byte_demand();
        let denied = job.close_step(1, box_bytes - 1);
        let retained = job.authority.is_some();
        let released = job.close_step(1, box_bytes);
        for _ in 0..8 { if job.terminal_is_empty() { break; } job.close_step(1, admission); }
        assert!(job.terminal_is_empty());
        assert_eq!(demand, extent);
        assert_eq!(first, InteractiveJobCloseStep::Pending { released_items: usize::from(row["releasedBytes"].as_u64().unwrap() != 0), released_bytes: row["releasedBytes"].as_u64().unwrap() as usize });
        assert_eq!(terminal_demand, box_bytes);
        assert_eq!(denied, InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert!(retained);
        assert_eq!(released, InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: box_bytes });
        eprintln!("[DEBUG] initializer job physical backing={extent} erased-authority-box={box_bytes} retained-denied-owner=true");
    }
}

std::thread_local! {
    static RELOAD_FOLDS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// 🧮️ A test operation that counts every fold of it (`diff`), so the reload law bounds what one initializer step folds.
#[derive(Clone, Debug, PartialEq)]
struct ReloadCountedOp(TestMutation);

impl semio_framework_value::ToValue for ReloadCountedOp {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(&self.0)
    }
}

impl semio_framework_value::FromValue for ReloadCountedOp {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <TestMutation as semio_framework_value::FromValue>::from_value(value).map(Self)
    }
}

impl ::protocol::OpBinary for ReloadCountedOp {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        ::protocol::OpBinary::encode_op(&self.0)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        <TestMutation as ::protocol::OpBinary>::decode_op(bytes).map(Self)
    }
}

impl ::protocol::OpText for ReloadCountedOp {
    fn print_op(&self) -> String {
        ::protocol::OpText::print_op(&self.0)
    }

    fn parse_op(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
        <TestMutation as ::protocol::OpText>::parse_op(line).map(Self)
    }
}

impl store::Mutation<TestSnapshot> for ReloadCountedOp {
    type Diff = <TestMutation as store::Mutation<TestSnapshot>>::Diff;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = <TestMutation as store::Mutation<TestSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor {
        store::Mutation::<TestSnapshot>::descriptor(&self.0)
    }

    fn diff(&self, base: &TestSnapshot) -> ::protocol::MutationOutcome<Self::Diff> {
        RELOAD_FOLDS.with(|folds| folds.set(folds.get() + 1));
        store::Mutation::<TestSnapshot>::diff(&self.0, base)
    }

    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok({ store::Mutation::<TestSnapshot>::inverse(&self.0, base)?.into_iter().map(Self).collect() })
    }

    fn conflict_target(&self) -> Vec<String> {
        store::Mutation::<TestSnapshot>::conflict_target(&self.0)
    }

    fn may_emit_foreign_steps(&self) -> bool {
        store::Mutation::<TestSnapshot>::may_emit_foreign_steps(&self.0)
    }
}

/// 🧹️ Closes a counted store under its bounded owners.
fn close_reload_counted_store(store: &mut store::ArtifactStore<TestSnapshot, ReloadCountedOp>) {
    for _ in 0..65_536 {
        match store.close_owned_step(1, 4096).expect("the counted store closes under its exact grant") {
            store::SnapshotRetirementStep::Pending { .. } => {}
            store::SnapshotRetirementStep::Blocked => panic!("the counted store has no external owner"),
            store::SnapshotRetirementStep::Complete => {
                assert!(store.close_owned_terminal_is_empty());
                return;
            }
        }
    }
    panic!("the counted store did not close");
}

/// 🐢️ LAW (N17, design §16.6): a reload never folds a long history inside one turn. A 240-mutation document with a
/// supersession reloads through the bounded retained initializer — the job every archive load and
/// `begin_persisted_document_store_replacement` drive one budget per turn — one step at a time: no step folds more than one
/// operation, the reload spreads over at least as many steps as the history has operations, and the reloaded store holds
/// exactly the source's head, supersessions and outcomes — and the content revision a store loaded in one piece computes
/// (canonical initial digest and edit records, audit W1G-2/W1G-4).
#[semio_framework_async_macros::async_test]
async fn a_long_history_reloads_one_operation_per_initializer_step() {
    use crate::test_app_mutation_fixture::SetCount;
    let genesis = store::create_document_envelope::<TestSnapshot, ReloadCountedOp>(RELOAD_DOCUMENT_SCHEMA, "long-reload", TestSnapshot { count: 0, label: "initial".into(), slot: Vec::new() }, None);
    let mut source = Box::pin(store::ArtifactStore::new(genesis, protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into()))).await.expect("source store");
    source.install_document_store_owners_exact(bounded_document_store_owners::<TestSnapshot, ReloadCountedOp>());
    for value in 1..=240 {
        Box::pin(source.dispatch(store::ArtifactCommand::Apply { mutations: vec![ReloadCountedOp(TestMutation::SetCount(SetCount { value }))], transaction: None })).await.expect("source edit");
    }
    let first = source.mutation_ops().expect("source operations")[0].mutation_id.clone();
    let inputs = vec![store::SupersedeInput { target: first, replacement: Some(ReloadCountedOp(TestMutation::SetCount(SetCount { value: 1000 }))) }];
    Box::pin(source.dispatch(store::ArtifactCommand::Supersede { scope: None, inputs })).await.expect("a supersession");
    let files = Box::pin(store::print_document_pack(source.envelope())).await.expect("source pair prints");
    let envelope = Box::pin(store::parse_document_pack::<TestSnapshot, ReloadCountedOp>(&files.pack, &files.spr)).await.expect("the pair parses").into_envelope();
    let mut whole = Box::pin(store::ArtifactStore::new(Box::pin(store::parse_document_pack::<TestSnapshot, ReloadCountedOp>(&files.pack, &files.spr)).await.expect("the pair parses again").into_envelope(), protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into()))).await.expect("a store loaded in one piece");
    whole.install_document_store_owners_exact(bounded_document_store_owners::<TestSnapshot, ReloadCountedOp>());
    let (operation, generation) = (semio_framework_job::OperationId(97), semio_framework_job::Generation(0));
    let mut job = bounded_document_store_initialization_job(envelope, RELOAD_DOCUMENT_SCHEMA, operation, generation, source.local_actor_id().clone());
    let mut sequence = 0u64;
    let mut steps = 0usize;
    let mut folded = 0;
    loop {
        let before = RELOAD_FOLDS.with(std::cell::Cell::get);
        let mut cx = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::CancelToken::root_now(), semio_framework_job::default_now_us, &mut sequence);
        let outcome = semio_framework_job::InteractiveJob::step(&mut job, &mut cx);
        assert!(RELOAD_FOLDS.with(std::cell::Cell::get) - before <= 1, "initializer step {steps} folded more than one operation");
        let progress = job.progress();
        assert!(progress.0 >= folded && progress.0 <= progress.1.max(progress.0) && (progress.1 == 0 || progress.1 == 240), "initializer progress only grows toward the applied operations: {progress:?} at step {steps}");
        folded = progress.0;
        steps += 1;
        match outcome {
            semio_framework_job::StepOutcome::Yield => assert!(steps < 1_000_000, "the reload finishes"),
            semio_framework_job::StepOutcome::Complete(_) => break,
            _ => panic!("the reload stopped at initializer step {steps}"),
        }
    }
    assert!(steps >= 240, "the reload spreads over the history ({steps} steps)");
    assert_eq!(job.progress(), (240, 240), "the initializer reports every applied operation folded");
    assert!(steps < 16 * 240, "a constant number of steps per edit: validation and lookup are linear in the history, not pairwise ({steps} steps)");
    let mut reloaded = job.take_candidate().expect("the reloaded store");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let authority_bytes = std::mem::size_of::<BoundedStoreInitializationAuthority<TestSnapshot, ReloadCountedOp>>();
    let handoff_demand = job.next_close_byte_demand();
    assert!(job.authority.is_some());
    assert_eq!(semio_framework_job::InteractiveJob::close_step(&mut job, 1, authority_bytes - 1), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    for _ in 0..65_536 {
        if job.terminal_is_empty() {
            break;
        }
        let maximum_bytes = job.next_close_byte_demand().max(4096);
        let step = semio_framework_job::InteractiveJob::close_step(&mut job, 1, maximum_bytes);
        if let semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } = step { assert!(released_items <= 1 && released_bytes <= admission); }
    }
    assert!(job.terminal_is_empty(), "the initializer hands its store off and closes");
    assert_eq!(handoff_demand, authority_bytes);
    assert!(authority_bytes <= admission, "initializer frame fits its explicit physical admission ceiling");
    eprintln!("[DEBUG] bounded reload candidate handoff retains authority-frame={authority_bytes} until its exact physical grant; admission={admission}");
    assert_eq!(reloaded.snapshot().expect("reloaded head"), source.snapshot().expect("source head"));
    assert_eq!(reloaded.snapshot().expect("reloaded head").count, 240);
    assert_eq!(reloaded.supersessions(), source.supersessions());
    assert_eq!(reloaded.mutation_outcomes().expect("reloaded outcomes"), source.mutation_outcomes().expect("source outcomes"));
    assert_eq!(reloaded.content_revision_now(), whole.content_revision_now(), "the retained reload names the revision a whole load names");
    drop(job);
    close_reload_counted_store(&mut reloaded);
    close_reload_counted_store(&mut whole);
    close_reload_counted_store(&mut source);
}
