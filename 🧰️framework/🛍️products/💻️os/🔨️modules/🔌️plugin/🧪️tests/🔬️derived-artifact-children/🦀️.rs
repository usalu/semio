use super::*;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq)]
struct ChildrenTestSnapshot;

#[derive(Clone, Debug, Default, PartialEq, Serialize, ToValue, Deserialize, FromValue)]
struct ChildrenTestDiff {}

impl protocol::MutationDiff<ChildrenTestSnapshot> for ChildrenTestDiff {
    fn apply(&self, snapshot: &ChildrenTestSnapshot) -> protocol::MutationApplyResult<ChildrenTestSnapshot> {
        Ok(snapshot.clone())
    }
    fn absorb(&mut self, _other: Self) {}
}

//#region 🧬️ChildrenMutationRoster
mod mutations {
    use super::{ChildrenTestDiff, ChildrenTestSnapshot};
    include!("../🧒️children-fixture/🧬️mutations/🦀️.rs");
}
use mutations::ChildrenTestMutation;
//#endregion 🧬️ChildrenMutationRoster

#[derive(Clone, Debug)]
struct ChildrenTestConstruction(ChildrenTestSnapshot);

impl ArtifactBuilder for ChildrenTestConstruction {
    type Snapshot = ChildrenTestSnapshot;
    type Mutation = ChildrenTestMutation;
    type Diff = ChildrenTestDiff;
    fn empty() -> Self {
        Self(ChildrenTestSnapshot)
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self(snapshot)
    }
    fn from_text(_text: &str) -> Result<Self, TextError> {
        Ok(Self::empty())
    }
    fn from_binary(_bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::empty())
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
        let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.0);
        self.0 = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.0).expect("children test diff applies");
        (self, outcome)
    }
    fn absorb(self, _diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
        Ok(self)
    }
    fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
        Ok(self.0)
    }
}

struct ChildrenTestAnalysis;

impl ArtifactAnalysis for ChildrenTestAnalysis {
    type Parts = ();
    const DIALECT: Dialect = Dialect { artifact_kind: "s.test.children-parent", standard: StandardId("1"), subset: SubsetId::ANY };
    fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
        IoConfidence::Low
    }
    fn analyze(_sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        Analysis { parts: (), dialect: Self::DIALECT, confidence: IoConfidence::Low, diagnostics: Vec::new() }
    }
}

struct ChildrenTestComposition;

impl ArtifactComposition for ChildrenTestComposition {
    type Snapshot = ChildrenTestSnapshot;
    const WRITES: Dialect = ChildrenTestAnalysis::DIALECT;
    fn reads() -> &'static [Dialect] {
        &[]
    }
    fn compose(_sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        Err(ComposeError { message: "ChildrenTestComposition::compose is unreachable in this test — only reads()/child routing is exercised".into(), diagnostics: Vec::new() })
    }
}

const CHILD_SLOTS: &[::semio_framework_schema::ChildSlotSpec] = &[::semio_framework_schema::ChildSlotSpec { name: "primaryMesh", kind: "s.stdio.mesh", many: false }];

struct ChildrenTestChildren;

impl ArtifactChildren for ChildrenTestChildren {
    type Snapshot = ChildrenTestSnapshot;
    fn slots() -> &'static [::semio_framework_schema::ChildSlotSpec] {
        CHILD_SLOTS
    }
    fn compose_from_children(parts: &[(ArtifactDialect, Vec<u8>)]) -> Result<Self::Snapshot, ComposeError> {
        if parts.iter().all(|(dialect, _)| dialect.artifact_kind == "s.stdio.mesh") {
            Ok(ChildrenTestSnapshot)
        } else {
            Err(ComposeError { message: "unexpected child dialect".into(), diagnostics: Vec::new() })
        }
    }
    fn decompose_to_children(_snapshot: &Self::Snapshot) -> Vec<(ArtifactDialect, Vec<u8>)> {
        Vec::new()
    }
}

struct ChildrenTestSpec;

impl DerivedArtifactSpec for ChildrenTestSpec {
    type Snapshot = ChildrenTestSnapshot;
    type Mutation = ChildrenTestMutation;
    type Diff = ChildrenTestDiff;
    type Construction = ChildrenTestConstruction;
    type Analysis = ChildrenTestAnalysis;
    type Composition = ChildrenTestComposition;
    type Children = ChildrenTestChildren;
}

#[semio_framework_async_macros::async_test]
async fn derived_composer_reads_includes_child_slot_dialects() {
    let reads = <DerivedArtifactComposer<ChildrenTestSpec> as ArtifactComposer>::reads();
    assert_eq!(reads.len(), 1, "Composition::reads() is empty here — the ONE entry must be the child slot's synthesized dialect");
    assert_eq!(reads[0].artifact_kind, "s.stdio.mesh");
    assert_eq!(reads[0].standard, StandardId("*"));
    assert_eq!(reads[0].subset, SubsetId::ANY);
}

#[semio_framework_async_macros::async_test]
async fn derived_composer_reads_defaults_to_composition_reads_for_a_leaf_with_no_children() {
    // 🍃️ `NoChildren<S>::slots()` is `&[]` — proves the pre-C1 behavior is byte-identical for
    // any spec that never names a `children: $ty` (every macro invocation before this wave).
    struct LeafSpec;
    impl DerivedArtifactSpec for LeafSpec {
        type Snapshot = ChildrenTestSnapshot;
        type Mutation = ChildrenTestMutation;
        type Diff = ChildrenTestDiff;
        type Construction = ChildrenTestConstruction;
        type Analysis = ChildrenTestAnalysis;
        type Composition = ChildrenTestComposition;
        type Children = NoChildren<ChildrenTestSnapshot>;
    }
    let reads = <DerivedArtifactComposer<LeafSpec> as ArtifactComposer>::reads();
    assert!(reads.is_empty(), "a leaf spec's reads() must equal Composition::reads() (empty here) exactly");
}

#[semio_framework_async_macros::async_test]
async fn derived_composer_compose_routes_matching_sources_through_compose_from_children() {
    let sources = vec![ComposeSource { dialect: Dialect { artifact_kind: "s.stdio.mesh", standard: StandardId("1"), subset: SubsetId::ANY }, payload: AnalyzeSource::Binary(&[1, 2, 3]) }];
    let composed = <DerivedArtifactComposer<ChildrenTestSpec> as ArtifactComposer>::compose(&sources).expect("child-slot-matching sources route through compose_from_children");
    assert_eq!(composed.snapshot, ChildrenTestSnapshot);
}

// 🧬️ Smoke-tests the macro's own `children: $ty` grammar (not just the underlying
// `DerivedArtifactComposer` mechanism above) — proves `derive_artifact_facets!` actually parses
// and wires the optional field end to end. Plain `//`, not `///`: rustdoc cannot attach a doc
// comment to a macro invocation (only to the items it expands to).
derive_artifact_facets! {
    spec ChildrenMacroSpec {
        construction: ChildrenTestConstruction,
        analysis: ChildrenTestAnalysis,
        composition: ChildrenTestComposition,
        children: ChildrenTestChildren,
    }
    builder: ChildrenMacroBuilder,
    analyzer: ChildrenMacroAnalyzer,
    composer: ChildrenMacroComposer,
}

#[semio_framework_async_macros::async_test]
async fn derive_artifact_facets_children_arm_wires_the_macro_generated_composer() {
    let reads = <ChildrenMacroComposer as ArtifactComposer>::reads();
    assert_eq!(reads.len(), 1);
    assert_eq!(reads[0].artifact_kind, "s.stdio.mesh");
}
