//! 🎹️ PdfComposer (raw/✳️any at 1.4) — analyzer + builder glued. Reads native
//! `stdio.pdf` sources (plus its DAG dependencies: binary, deflate), writes one `stdio.pdf` (1.4/✳️any) snapshot.

use semio_framework_plugin::{ArtifactComposer, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
use crate::artifacts::pdf::standards::v1_4::subsets::any::analyzer::PdfAnalyzer;
use semio_framework_plugin::ArtifactAnalyzer as _;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };


pub struct PdfComposer;

impl ArtifactComposer for PdfComposer {
    type Snapshot = PdfSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] {
        &[DIALECT, DEP_BINARY, DEP_DEFLATE]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
        // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
        // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
        // like binary) that payload IS the same byte/text shape `analyze` already accepts.
        let native: Vec<AnalyzeSource<'_>> = sources
            .iter()
            .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY || s.dialect == DEP_DEFLATE)
            .map(|s| match &s.payload {
                AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
            })
            .collect();
        if native.is_empty() {
            return Err(ComposeError { message: "PdfComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = PdfAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "PdfComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
