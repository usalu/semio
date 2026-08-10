//! 🎹️ DxfComposer (raw/✳️any at r12) — analyzer + builder glued. Reads native
//! `stdio.dxf` sources (plus its DAG dependencies: txt), writes one `stdio.dxf` (r12/✳️any) snapshot.

use semio_framework_plugin::{ArtifactComposer, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
use crate::artifacts::dxf::DxfSnapshot;
use crate::artifacts::dxf::standards::v_r12::subsets::any::analyzer::DxfAnalyzer;
use semio_framework_plugin::ArtifactAnalyzer as _;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };


pub struct DxfComposer;

impl ArtifactComposer for DxfComposer {
    type Snapshot = DxfSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] {
        &[DIALECT, DEP_TXT]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
        // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
        // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
        // like binary) that payload IS the same byte/text shape `analyze` already accepts.
        let native: Vec<AnalyzeSource<'_>> = sources
            .iter()
            .filter(|s| s.dialect == DIALECT || s.dialect == DEP_TXT)
            .map(|s| match &s.payload {
                AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
            })
            .collect();
        if native.is_empty() {
            return Err(ComposeError { message: "DxfComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = DxfAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "DxfComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
