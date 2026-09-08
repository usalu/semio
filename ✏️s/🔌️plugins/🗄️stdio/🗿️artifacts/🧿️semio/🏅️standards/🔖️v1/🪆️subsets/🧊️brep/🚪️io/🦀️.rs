//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    #[cfg(feature = "conversion-brep")]
    use crate::standards::v1::subsets::brep::io::export::serializers::artifacts::step::v_ap214::any::SemioBrepToStep;
    #[cfg(feature = "conversion-brep")]
    use crate::standards::v1::subsets::brep::io::import::deserializers::artifacts::step::v_ap214::any::SemioBrepFromStep;
    use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
    use crate::standards::v1::subsets::brep::schema::SemioBrepAnalyzer;
    #[cfg(feature = "conversion-brep")]
    use semio_framework_plugin::{deserializer_entry_of, register_composer_entries, serializer_entry_of, ComposerEntry};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::collections::HashSet;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("brep") };

    //#region 🔖️Composer
    pub struct SemioBrepComposerComposition;

    impl ArtifactComposition for SemioBrepComposerComposition {
        type Snapshot = SemioBrepSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "SemioBrepComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioBrepAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioBrepComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decodes the payload as this subset's own `SemioBrepSnapshot` (D5's validate-on-build hook)
    /// and checks referential invariants BETWEEN the subset's own collections: every id an
    /// edge/loop/face/shell/solid references (start/end vertex, loop edge, outer/inner loop, shell
    /// face, solid shell) must resolve to a real entity of the referenced kind in the same snapshot.
    /// Dangling references are reported, never silently dropped or fabricated.
    pub struct SemioBrepValidator;

    impl SubsetValidator for SemioBrepValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioBrepSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_brep_referential_integrity(&snapshot),
                None => vec![dsl::Diagnostic::error("stdio.semio_brep.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioBrepValidator: payload did not decode as a SemioBrepSnapshot".to_string())],
            }
        }
    }

    /// 🔗️ Real cross-collection referential-invariant check — dangling ids are reported as errors, not
    /// silently ignored (nothing here is decode-only anymore).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_brep_referential_integrity(snapshot: &SemioBrepSnapshot) -> Vec<dsl::Diagnostic> {
        let vertex_ids: HashSet<&str> = snapshot.vertices.iter().map(|v| v.id.as_str()).collect();
        let edge_ids: HashSet<&str> = snapshot.edges.iter().map(|e| e.id.as_str()).collect();
        let loop_ids: HashSet<&str> = snapshot.loops.iter().map(|l| l.id.as_str()).collect();
        let face_ids: HashSet<&str> = snapshot.faces.iter().map(|f| f.id.as_str()).collect();
        let shell_ids: HashSet<&str> = snapshot.shells.iter().map(|s| s.id.as_str()).collect();

        let mut diagnostics = Vec::new();
        let mut dangling = |code: &'static str, message: String| {
            diagnostics.push(dsl::Diagnostic::error(code, dsl::TextSpan::at(1, 1), message));
        };

        for e in &snapshot.edges {
            if !vertex_ids.contains(e.start_vertex.as_str()) {
                dangling("stdio.semio_brep.dangling-edge-start-vertex", format!("edge {:?} references unknown start vertex {:?}", e.id, e.start_vertex));
            }
            if !vertex_ids.contains(e.end_vertex.as_str()) {
                dangling("stdio.semio_brep.dangling-edge-end-vertex", format!("edge {:?} references unknown end vertex {:?}", e.id, e.end_vertex));
            }
        }
        for l in &snapshot.loops {
            for le in &l.edges {
                if !edge_ids.contains(le.edge.as_str()) {
                    dangling("stdio.semio_brep.dangling-loop-edge", format!("loop {:?} references unknown edge {:?}", l.id, le.edge));
                }
            }
        }
        for f in &snapshot.faces {
            if !loop_ids.contains(f.outer_loop.as_str()) {
                dangling("stdio.semio_brep.dangling-face-outer-loop", format!("face {:?} references unknown outer loop {:?}", f.id, f.outer_loop));
            }
            for inner in &f.inner_loops {
                if !loop_ids.contains(inner.as_str()) {
                    dangling("stdio.semio_brep.dangling-face-inner-loop", format!("face {:?} references unknown inner loop {:?}", f.id, inner));
                }
            }
        }
        for s in &snapshot.shells {
            for sf in &s.faces {
                if !face_ids.contains(sf.face.as_str()) {
                    dangling("stdio.semio_brep.dangling-shell-face", format!("shell {:?} references unknown face {:?}", s.id, sf.face));
                }
            }
        }
        for so in &snapshot.solids {
            for ss in &so.shells {
                if !shell_ids.contains(ss.shell.as_str()) {
                    dangling("stdio.semio_brep.dangling-solid-shell", format!("solid {:?} references unknown shell {:?}", so.id, ss.shell));
                }
            }
        }
        diagnostics
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioBrepValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::brep::schema::semio_brep_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioBrepSnapshot, crate::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation>(crate::standards::v1::subsets::brep::schema::snapshot::STDIO_SEMIOBREP_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-brep")]
        register_composer_entries(io_bridge_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.brep.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::brep::schema::inferences::semio_brep_artifact_inference_descriptor());
    }

    /// 🌉️ W4 semio↔step bridge — one deserializer entry (writes brep, reads step) + one serializer
    /// entry (writes step, reads brep) give all 4 `IoKey`s via `register_composer_entries`'s own
    /// symmetric import/export insertion (see its doc comment) — no separate reverse registration
    /// needed.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    #[cfg(feature = "conversion-brep")]
    fn io_bridge_entries() -> &'static [ComposerEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
        ENTRIES.get_or_init(|| vec![deserializer_entry_of::<SemioBrepFromStep>(), serializer_entry_of::<SemioBrepToStep>()]).as_slice()
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(all(test, feature = "conversion-brep"))]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

/// 🖊️ DWG mesh conversion for the BREP artifact.
#[cfg(feature = "conversion-brep")]
pub mod dwg {
    use crate::standards::v1::subsets::brep::schema::engine::{BrepError, BrepKernel, GeometryHandle};
    struct DwgExporter;
    impl semio_framework_mesh_engine::MeshExporter for DwgExporter {
        fn format_kind(&self) -> &'static str {
            "dwg"
        }
        fn export(&self, mesh: &semio_framework_mesh_engine::MeshData) -> Result<Vec<u8>, String> {
            let drawing = semio_s_artifact_stdio_dwg::mesh_to_dwg_drawing(mesh);
            semio_s_artifact_stdio_dwg::dwg_to_bytes(&drawing)
        }
    }
    struct DwgImporter;
    impl semio_framework_mesh_engine::MeshImporter for DwgImporter {
        fn format_kind(&self) -> &'static str {
            "dwg"
        }
        fn import(&self, bytes: &[u8]) -> Result<semio_framework_mesh_engine::MeshData, String> {
            let drawing = semio_s_artifact_stdio_dwg::dwg_from_bytes(bytes)?;
            Ok(semio_s_artifact_stdio_dwg::dwg_drawing_to_mesh(&drawing))
        }
    }

    /// 📤️ Encodes tessellated BREP geometry through the DWG artifact codec.
    pub fn export(kernel: &dyn BrepKernel, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_mesh(shapes, deflection, &DwgExporter)
    }
    /// 📥️ Imports a DWG mesh into the BREP artifact's kernel.
    pub fn import(kernel: &mut dyn BrepKernel, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        kernel.import_mesh(data, tolerance, &DwgImporter)
    }
}
