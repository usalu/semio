//! 🏗️ TiffBaselineBuilder (6.0/✳️baseline) — PASS-THROUGH by design: `check_tiff_baseline_
//! conformance` findings are all SOFT (warnings), by policy (this builder never hard-gates on
//! them — see its own doc), so `build()` never fails on conformance grounds even though the
//! checks themselves are now real (ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION closed the earlier
//! schema gap that used to make ALL checks unconditionally soft-only).

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::tiff::standards::v6_0::subsets::baseline::analyzer::check_tiff_baseline_conformance;
use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::{diff::TiffDiff, mutations::TiffMutation, snapshot::TiffSnapshot};

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct TiffBaselineBuilder {
    snapshot: TiffSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for TiffBaselineBuilder {
    type Snapshot = TiffSnapshot;
    type Mutation = TiffMutation;
    type Diff = TiffDiff;

    fn empty() -> Self {
        Self { snapshot: TiffSnapshot::default(), diagnostics: Vec::new() }
    }

    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }

    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }

    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }

    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = crate::artifacts::tiff::standards::v6_0::subsets::any::schema::mutations::apply_tiff_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }

    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <TiffDiff as protocol::MutationDiff<TiffSnapshot>>::apply(&diff, &self.snapshot);
        self
    }

    /// 🛡️ Re-runs the honestly-scope-limited Baseline TIFF check -- always SOFT at this schema,
    /// so `build()` never fails; the diagnostics still surface via the analyzer/composer/
    /// validator paths for anyone inspecting them.
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        let _ = check_tiff_baseline_conformance(&self.snapshot);
        if self.diagnostics.is_empty() {
            Ok(self.snapshot)
        } else {
            Err(self.diagnostics)
        }
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_through_build_never_fails_on_conformance_grounds() {
        let snapshot = TiffBaselineBuilder::empty().build().expect("all conformance findings are soft by policy; build must succeed");
        assert!(snapshot.ifds.is_empty());
    }
}
