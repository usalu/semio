//! 🚪️ IO stdio.gltf (2.0/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::gltf::GltfSnapshot;
    use crate::artifacts::gltf::standards::v2_0::subsets::any::schema::GltfAnalyzer;
    use semio_framework_plugin::ArtifactAnalyzer as _;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };


    pub struct GltfComposerComposition;

    impl ArtifactComposition for GltfComposerComposition {
        type Snapshot = GltfSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_JSON, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts. Binary
            // sources are analyzed with real `.glb`-vs-pack sniffing (see `GltfAnalyzer::analyze`), so
            // a `DEP_BINARY` source carrying raw `.glb` bytes decodes through the exact same path a
            // hand-fed `AnalyzeSource::Binary` would.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_JSON || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "GltfComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = GltfAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
                message: "GltfComposerComposition: analysis produced no snapshot".into(),
                diagnostics: analysis.diagnostics.clone(),
            })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
