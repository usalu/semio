//! 🩹️ Store law of scratch retirement (design §23, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): a command the store
//! refuses retires everything its replay built — the working projection, the command's operations, the inverses it derived —
//! through the technology's cold retirement, and drops nothing bare. An artifact whose projection or operation owns a
//! fail-closed root aborts the guest on a bare drop, so this is what keeps a refused edit — exactly what conflict resolution
//! produces — from killing an editor. Artifact-agnostic: the fixture is a demo operation and a demo diff with a fail-closed
//! owner's discipline whose technology counts every projection it produces and every scratch projection retired through it;
//! the cases are the language-agnostic corpus `🧫️fixtures/🧫️replay-retirement`.
use super::*;
use super::supersede_replay_tests::fixture_author;

//#region 🧰️Fixture
std::thread_local! {
    static OPERATIONS_RETIRED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static OPERATIONS_DROPPED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static INVERSES_DERIVED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static PROJECTIONS_APPLIED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static PROJECTIONS_RETIRED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

fn count(counter: &'static std::thread::LocalKey<std::cell::Cell<usize>>) {
    counter.with(|cell| cell.set(cell.get() + 1));
}

/// 📊️ `[operations retired cold, operations dropped bare, inverses derived, projections applied, projections retired]` so far
/// on this thread.
fn tally() -> [usize; 5] {
    [&OPERATIONS_RETIRED, &OPERATIONS_DROPPED, &INVERSES_DERIVED, &PROJECTIONS_APPLIED, &PROJECTIONS_RETIRED].map(|counter| counter.with(std::cell::Cell::get))
}

/// 🧨️ Where a [`FailClosedOp`] makes its replay fail.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Fault {
    None,
    Encode,
    Inverse,
    Apply,
    Message,
}

/// 🧿️ A demo operation with a fail-closed owner's discipline: its cold retirement is counted, and a bare drop — what a
/// refused command used to do — is counted too, so a law can prove none happened.
#[derive(Clone, Debug, PartialEq)]
struct FailClosedOp {
    operation: DemoMutation,
    fault: Fault,
    live: bool,
}

impl FailClosedOp {
    fn of(operation: DemoMutation, fault: Fault) -> Self {
        Self { operation, fault, live: true }
    }
}

impl Drop for FailClosedOp {
    fn drop(&mut self) {
        if self.live {
            count(&OPERATIONS_DROPPED);
        }
    }
}

impl ToValue for FailClosedOp {
    fn to_value(&self) -> DslValue {
        self.operation.to_value()
    }
}

impl FromValue for FailClosedOp {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        DemoMutation::from_value(value).map(|operation| Self::of(operation, Fault::None))
    }
}

impl OpBinary for FailClosedOp {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        if self.fault == Fault::Encode {
            return Err(crate::os_spr::ProtocolError::LimitExceeded("the fixture refuses to encode this operation"));
        }
        self.operation.encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        DemoMutation::decode_op(bytes).map(|operation| Self::of(operation, Fault::None))
    }
}

impl OpText for FailClosedOp {
    fn print_op(&self) -> String {
        self.operation.print_op()
    }

    fn parse_op(line: &str) -> Result<Self, TextError> {
        DemoMutation::parse_op(line).map(|operation| Self::of(operation, Fault::None))
    }
}

/// 🧮️ The demo diff with a refusal switch. Its technology counts every projection it produces and every scratch projection
/// retired through it, so a law can prove each projection a replay built left through [`MutationDiff::retire_projection`].
#[derive(Clone, Debug, Default, PartialEq)]
struct FailClosedDiff {
    inner: DemoDiff,
    refuse: bool,
}

impl ToValue for FailClosedDiff {
    fn to_value(&self) -> DslValue {
        self.inner.to_value()
    }
}

impl FromValue for FailClosedDiff {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        DemoDiff::from_value(value).map(|inner| Self { inner, refuse: false })
    }
}

impl crate::os_spr::DiffAlgebra<DemoSnapshot> for FailClosedDiff {
    fn inverse(&self, base: &DemoSnapshot) -> Self {
        Self { inner: crate::os_spr::DiffAlgebra::inverse(&self.inner, base), refuse: false }
    }
    fn is_empty(&self) -> bool {
        !self.refuse && crate::os_spr::DiffAlgebra::<DemoSnapshot>::is_empty(&self.inner)
    }
}

impl MutationDiff<DemoSnapshot> for FailClosedDiff {
    fn apply(&self, base: &DemoSnapshot, capability: crate::os_spr::ApplyCapability) -> crate::os_spr::MutationApplyResult<DemoSnapshot> {
        if self.refuse {
            return Err(crate::os_spr::MutationApplyError { code: "mutation.apply.fixture-refused".into(), message: "the fixture refuses to apply this diff".into(), target: Vec::new() });
        }
        count(&PROJECTIONS_APPLIED);
        self.inner.apply(base, capability)
    }

    fn absorb(&mut self, other: Self) {
        self.refuse |= other.refuse;
        self.inner.absorb(other.inner);
    }

    fn retire_projection(projection: DemoSnapshot) {
        count(&PROJECTIONS_RETIRED);
        drop(projection);
    }
}

impl Mutation<DemoSnapshot> for FailClosedOp {
    type Diff = FailClosedDiff;
    const DESCRIPTORS: &'static [crate::os_spr::MutationLeafDescriptor] = <DemoMutation as Mutation<DemoSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static crate::os_spr::MutationLeafDescriptor {
        self.operation.descriptor()
    }

    fn diff(&self, base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<Self::Diff> {
        if self.fault == Fault::Message {
            return crate::os_spr::MutationOutcome::error("mutation.target-missing", "the fixture reports an error for this operation", ["n"]);
        }
        crate::os_spr::MutationOutcome::new(FailClosedDiff { inner: self.operation.diff(base).into_parts().0, refuse: self.fault == Fault::Apply })
    }

    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        if self.fault == Fault::Inverse {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "the fixture has no inverse for this operation".to_string()));
        }
        let derived = self.operation.inverse(base)?;
        Ok(derived
            .into_iter()
            .map(|operation| {
                count(&INVERSES_DERIVED);
                Self::of(operation, Fault::None)
            })
            .collect())
    }

    fn retire_cold(mut self) {
        self.live = false;
        count(&OPERATIONS_RETIRED);
    }
}

impl MemberStoreOwner<FailClosedOp> for DemoSnapshot {
    type SnapshotOpen = UnsupportedMemberSnapshotOpen<Self>;

    fn member_store_owners_birth_demand() -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: DocumentStoreOwners::<Self, FailClosedOp>::source_birth_bytes::<DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, ArtifactStoreCursorDisposer<Self, FailClosedOp>>()?, depth: 1 })
    }

    fn member_store_owners(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(DocumentStoreOwners<Self, FailClosedOp>, semio_framework_value::retained_clone::RetainedCloneProgress), crate::os_store::DocumentStoreOwnersAdmissionError<Self, FailClosedOp>> {
        DocumentStoreOwners::admit_source_constructor(grant, || (DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, ArtifactStoreCursorDisposer::<Self, FailClosedOp>::new()))
    }
}
//#endregion 🧰️Fixture

//#region 🧪️Laws
/// 🩹️ LAW: a refused apply retires everything its replay built. For every corpus case — an encode failure, a refused
/// inverse, a failing diff and an error the merge policy rejects, at the first, a middle and the last operation of a command
/// — the store answers the refusal the corpus names; every operation of the command and every inverse derived from it
/// retired cold and none reached a bare `Drop`; the working projection and every projection an operation produced retired
/// through the technology; and the store shows exactly what it showed. Afterwards the store still applies an edit, and it
/// closes to its terminal-empty witness when the test ends.
#[semio_framework_async_macros::async_test]
async fn a_refused_apply_retires_everything_its_replay_built() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️replay-retirement/🔣️.json")).expect("the corpus parses");
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, FailClosedOp>("demo/v1", "replay-retirement", DemoSnapshot { n: Some(0) }, None)).await;
    store.set_merge_policy(crate::os_spr::MergePolicy::Normal);
    fixture_author(&mut store, ArtifactCommand::Apply { mutations: vec![FailClosedOp::of(DemoMutation::SetN(SetN { n: 1 }), Fault::None)], transaction: None }).await.expect("a clean edit applies");
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("a case name");
        let operations = usize::try_from(case["operations"].as_u64().expect("an operation count")).expect("a usize");
        let at = usize::try_from(case["at"].as_u64().expect("the failing operation")).expect("a usize");
        let fault = match case["fault"].as_str().expect("a fault") {
            "encode" => Fault::Encode,
            "inverse" => Fault::Inverse,
            "apply" => Fault::Apply,
            "message" => Fault::Message,
            other => panic!("unknown corpus fault {other}"),
        };
        let mutations: Vec<FailClosedOp> = (0..operations).map(|index| FailClosedOp::of(DemoMutation::SetN(SetN { n: 10 + index as i32 }), if index == at { fault } else { Fault::None })).collect();
        let shown = (store.snapshot_ref().n, store.applied_edit_ids().len(), store.content_revision_now());
        let before = tally();
        let refusal = match fixture_author(&mut store, ArtifactCommand::Apply { mutations, transaction: None }).await {
            Ok(_) => panic!("{name}: the command is refused"),
            Err(VcsError::ValidationFailed(_)) => "validation-failed",
            Err(VcsError::InverseRefused(_)) => "inverse-refused",
            Err(VcsError::MutationApply(_)) => "mutation-apply",
            Err(VcsError::Rejected { .. }) => "rejected",
            Err(other) => panic!("{name}: an unexpected refusal {other}"),
        };
        let after = tally();
        let [retired, dropped, derived, applied, projections] = [0, 1, 2, 3, 4].map(|index| after[index] - before[index]);
        assert_eq!(refusal, case["refusal"].as_str().expect("a refusal"), "{name}: the refusal");
        assert_eq!(applied as u64, case["applied"].as_u64().expect("applied operations"), "{name}: operations applied before the refusal");
        assert_eq!(dropped, 0, "{name}: no operation reached a bare Drop");
        assert_eq!(retired, operations + derived, "{name}: every operation of the command and every inverse derived from it retired cold");
        assert_eq!(projections, applied + 1, "{name}: the working projection and every projection an operation produced retired through the technology");
        assert_eq!((store.snapshot_ref().n, store.applied_edit_ids().len(), store.content_revision_now()), shown, "{name}: the store shows what it showed");
    }
    fixture_author(&mut store, ArtifactCommand::Apply { mutations: vec![FailClosedOp::of(DemoMutation::SetN(SetN { n: 2 }), Fault::None)], transaction: None }).await.expect("the store still applies an edit");
    assert_eq!((store.snapshot_ref().n, store.applied_edit_ids().len()), (Some(2), 2));
}
//#endregion 🧪️Laws
