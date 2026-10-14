//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::schema::snapshot::SemioMeshSnapshot;
    use crate::io::SemioMeshAnalyzer;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};
    //#region 🔖️IoBridgeImports
    //#endregion 🔖️IoBridgeImports

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

    //#region 🔖️Composer
    pub struct SemioMeshComposerComposition;

    impl ArtifactComposition for SemioMeshComposerComposition {
        type Snapshot = SemioMeshSnapshot;
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
                return Err(ComposeError { message: "SemioMeshComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioMeshAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioMeshComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decodes the payload as this subset's own `SemioMeshSnapshot`, then checks real
    /// referential invariants across its own collections: every `primitive.material_id` (when
    /// `Some`) must resolve to a real entry in `materials`, and mesh/primitive/material/texture ids
    /// must be unique within their own collection (dangling refs + duplicate keys are the two
    /// invariant classes the master plan calls out for subset validators).
    pub struct SemioMeshValidator;

    impl SubsetValidator for SemioMeshValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<semio_framework_diagnostic::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioMeshSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioMeshSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_mesh_referential_invariants(&snapshot),
                None => vec![semio_framework_diagnostic::Diagnostic::error("stdio.semio_mesh.validate-decode-failed", semio_framework_diagnostic::TextSpan::at(1, 1), "SemioMeshValidator: payload did not decode as a SemioMeshSnapshot".to_string())],
            }
        }
    }

    /// 🔗 Real cross-collection referential check, shared by the registered validator above and its
    /// own direct unit tests below.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_mesh_referential_invariants(snapshot: &SemioMeshSnapshot) -> Vec<semio_framework_diagnostic::Diagnostic> {
        let mut diagnostics = Vec::new();

        let mut seen_mesh_ids = std::collections::HashSet::new();
        for mesh in &snapshot.meshes {
            if !seen_mesh_ids.insert(mesh.id.as_str()) {
                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_mesh.duplicate-mesh-id", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioMeshValidator: duplicate mesh id {:?}", mesh.id)));
            }
            let mut seen_primitive_ids = std::collections::HashSet::new();
            for primitive in &mesh.primitives {
                if !seen_primitive_ids.insert(primitive.id.as_str()) {
                    diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_mesh.duplicate-primitive-id", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioMeshValidator: mesh {:?} has duplicate primitive id {:?}", mesh.id, primitive.id)));
                }
                if let Some(material_id) = &primitive.material_id {
                    if !snapshot.materials.iter().any(|m| &m.id == material_id) {
                        diagnostics.push(semio_framework_diagnostic::Diagnostic::error(
                            "stdio.semio_mesh.dangling-material-ref",
                            semio_framework_diagnostic::TextSpan::at(1, 1),
                            format!("SemioMeshValidator: mesh {:?} primitive {:?} references missing material {:?}", mesh.id, primitive.id, material_id),
                        ));
                    }
                }
            }
        }

        let mut seen_material_ids = std::collections::HashSet::new();
        for material in &snapshot.materials {
            if !seen_material_ids.insert(material.id.as_str()) {
                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_mesh.duplicate-material-id", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioMeshValidator: duplicate material id {:?}", material.id)));
            }
        }

        let mut seen_texture_ids = std::collections::HashSet::new();
        for texture in &snapshot.textures {
            if !seen_texture_ids.insert(texture.id.as_str()) {
                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_mesh.duplicate-texture-id", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioMeshValidator: duplicate texture id {:?}", texture.id)));
            }
        }

        diagnostics
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioMeshValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::schema::semio_mesh_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_document_codec(Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") }, store::bare_artifact_codec::<SemioMeshSnapshot, crate::schema::mutations::SemioMeshMutation>(crate::schema::snapshot::STDIO_SEMIOMESH_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 🧾️ The declarative twin of [`register`]: this subset's schema, document codec, `SubsetValidator`, composers
    /// (those writing semio, [`crate::semio_written`]) and inference descriptor as rows of [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        let builder = builder
            .schemas([crate::schema::semio_mesh_artifact_schema_descriptor()])
            .document_codec_bare::<SemioMeshSnapshot, crate::schema::mutations::SemioMeshMutation>(crate::schema::snapshot::STDIO_SEMIOMESH_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::schema::inferences::semio_mesh_artifact_inference_descriptor()]);
        builder
    }

    /// 💡️ Registers `s.stdio.semio.mesh.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::schema::inferences::semio_mesh_artifact_inference_descriptor()).expect("schema descriptor publication");
    }

    //#region 🔖️IoBridgeEntries
    //#endregion 🔖️IoBridgeEntries
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🧾️Encoding
/// 🧾️ A standard file format a mesh is written as through this subset's own export leaves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemioMeshFormat {
    Stl,
    Obj,
    Ply,
    Gltf,
    Las,
    Dwg,
    Png,
}


//#endregion 🧾️Encoding

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::schema::diff::SemioMeshDiff;
    use crate::schema::mutations::SemioMeshMutation;
    use crate::schema::snapshot::SemioMeshSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioMeshBuilderConstruction {
        snapshot: SemioMeshSnapshot,
    }

    impl ArtifactBuilder for SemioMeshBuilderConstruction {
        type Snapshot = SemioMeshSnapshot;
        type Mutation = SemioMeshMutation;
        type Diff = SemioMeshDiff;
        fn empty() -> Self {
            Self { snapshot: SemioMeshSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioMeshSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioMeshSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => (Self { snapshot, ..self }, outcome),
                Err(error) => (self, protocol::MutationOutcome::fatal(error.code, error.message, error.target)),
            }
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::schema::snapshot::{SemioMeshSnapshot, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioMeshParts {
        pub snapshot: Option<SemioMeshSnapshot>,
    }

    pub struct SemioMeshAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioMeshAnalyzerAnalysis {
        type Parts = SemioMeshParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOMESH_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOMESH_DOCUMENT_SCHEMA) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioMeshParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioMeshSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioMeshSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec SemioMeshBuilderFacets {
        construction: SemioMeshBuilderConstruction,
        analysis: SemioMeshAnalyzerAnalysis,
        composition: SemioMeshComposerComposition,
    }
    builder: SemioMeshBuilder,
    analyzer: SemioMeshAnalyzer,
    composer: SemioMeshComposer,
);
