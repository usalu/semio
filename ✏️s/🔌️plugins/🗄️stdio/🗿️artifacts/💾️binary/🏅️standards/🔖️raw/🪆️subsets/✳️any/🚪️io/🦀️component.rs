//! 🚪️ IO stdio.binary (raw/✳️any) — leaves are typed `ArtifactSerializer`/`ArtifactDeserializer`
//! impls; the 🎹️composer at this subset assembles them into its `ComposerEntry`. This facet root
//! no longer self-registers (nothing to register -- see `🎹️composer::register` at the artifact
//! level, called once from `🔌️plugin/🔧️setup`).
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::binary::BinarySnapshot;
    use crate::artifacts::binary::standards::v_raw::subsets::any::schema::BinaryAnalyzer;
    use semio_framework_plugin::ArtifactAnalyzer as _;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct BinaryComposerComposition;

    impl ArtifactComposition for BinaryComposerComposition {
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
                    message: "BinaryComposerComposition: no source in dialect stdio.binary/raw/*".into(),
                    diagnostics: Vec::new(),
                });
            }
            let analysis = BinaryAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
                message: "BinaryComposerComposition: analysis produced no snapshot".into(),
                diagnostics: analysis.diagnostics.clone(),
            })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
