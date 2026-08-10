//! 🎹️ BinaryComposer (raw/✳️any) — analyzer + builder glued: reads native `stdio.binary` sources,
//! writes one `stdio.binary` (raw/✳️any) snapshot. The real, subset-level unit; artifact/standard
//! composers aggregate this (and any sibling standard/subset composers) value-level.

use semio_framework_plugin::{ArtifactComposer, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::binary::standards::v_raw::subsets::any::analyzer::BinaryAnalyzer;
use semio_framework_plugin::ArtifactAnalyzer as _;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

pub struct BinaryComposer;

impl ArtifactComposer for BinaryComposer {
    type Snapshot = BinarySnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] {
        // 🌱 Terminal format: composes from its own native text/binary representation only.
        &[DIALECT]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let native: Vec<AnalyzeSource<'_>> = sources
            .iter()
            .filter(|s| s.dialect == DIALECT)
            .map(|s| match &s.payload {
                AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
            })
            .collect();
        if native.is_empty() {
            return Err(ComposeError {
                message: "BinaryComposer: no source in dialect stdio.binary/raw/*".into(),
                diagnostics: Vec::new(),
            });
        }
        let analysis = BinaryAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "BinaryComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
