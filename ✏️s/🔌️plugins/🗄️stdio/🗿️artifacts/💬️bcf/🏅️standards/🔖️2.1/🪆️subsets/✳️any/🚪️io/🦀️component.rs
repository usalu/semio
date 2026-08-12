//! 🚪️ IO stdio.bcf (2.1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::bcf::BcfSnapshot;
    use crate::artifacts::bcf::standards::v2_1::subsets::any::schema::BcfAnalyzer;
    use semio_framework_plugin::ArtifactAnalyzer as _;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bcf", standard: StandardId("2.1"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };


    pub struct BcfComposerComposition;

    impl ArtifactComposition for BcfComposerComposition {
        type Snapshot = BcfSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_ZIP, DEP_XML]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_ZIP || s.dialect == DEP_XML)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "BcfComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = BcfAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
                message: "BcfComposerComposition: analysis produced no snapshot".into(),
                diagnostics: analysis.diagnostics.clone(),
            })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
