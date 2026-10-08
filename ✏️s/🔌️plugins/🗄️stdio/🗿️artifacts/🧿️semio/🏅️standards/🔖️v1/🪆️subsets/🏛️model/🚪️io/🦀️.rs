//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
    use crate::standards::v1::subsets::model::io::SemioModelAnalyzer;
    #[cfg(feature = "conversion-model")]
    use semio_framework_plugin::{deserializer_entry_of, register_composer_entries, serializer_entry_of, ComposerEntry};
    use {semio_framework_plugin::register_subset_validator,semio_framework_plugin::subset_validator_entry_of,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::SubsetValidator,semio_framework_plugin::SubsetValidatorEntry};
    use std::collections::HashSet;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("model") };

    //#region 🔖️Composer
    pub struct SemioModelComposerComposition;

    impl ArtifactComposition for SemioModelComposerComposition {
        type Snapshot = SemioModelSnapshot;
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
                return Err(ComposeError { message: "SemioModelComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioModelAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioModelComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Real referential-invariant checks over `model`'s OWN collections (decode + dangling-id
    /// checks): a spatial node's `parent_id`, an element's `spatial_id`, and a relation's `from`/`to`
    /// must all resolve within THIS snapshot's own `spatial`/`elements` id spaces. Cross-subset
    /// references (`GeometryRef::Brep{brep_id}`/`Mesh{mesh_id}` into the sibling `brep`/`mesh`
    /// subsets) are NOT checked here — they are not decodable from a `model` snapshot alone, per the
    /// snapshot module's own doc comment.
    pub struct SemioModelValidator;

    /// 🔎️ Dangling-reference diagnostics for a decoded snapshot — split out from `validate()` so it's
    /// directly unit-testable against a typed `SemioModelSnapshot` (not just through the `IoPayload`
    /// wire boundary).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn semio_model_referential_diagnostics(snapshot: &SemioModelSnapshot) -> Vec<semio_framework_diagnostic::Diagnostic> {
        let spatial_ids: HashSet<&str> = snapshot.spatial.iter().map(|n| n.id.as_str()).collect();
        let element_ids: HashSet<&str> = snapshot.elements.iter().map(|e| e.id.as_str()).collect();
        let mut diagnostics = Vec::new();

        for node in &snapshot.spatial {
            if let Some(parent) = &node.parent_id {
                if parent == &node.id {
                    diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_model.validate-self-parent", semio_framework_diagnostic::TextSpan::at(1, 1), format!("spatial node {:?} is its own parent", node.id)));
                } else if !spatial_ids.contains(parent.as_str()) {
                    diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_model.validate-dangling-parent", semio_framework_diagnostic::TextSpan::at(1, 1), format!("spatial node {:?} references missing parent {:?}", node.id, parent)));
                }
            }
        }

        for element in &snapshot.elements {
            if let Some(spatial_id) = &element.spatial_id {
                if !spatial_ids.contains(spatial_id.as_str()) {
                    diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_model.validate-dangling-spatial-ref", semio_framework_diagnostic::TextSpan::at(1, 1), format!("element {:?} references missing spatial node {:?}", element.id, spatial_id)));
                }
            }
        }

        for relation in &snapshot.relations {
            let endpoint_known = |id: &str| element_ids.contains(id) || spatial_ids.contains(id);
            if !endpoint_known(&relation.from) {
                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_model.validate-dangling-relation-from", semio_framework_diagnostic::TextSpan::at(1, 1), format!("relation {:?} references missing from-id {:?}", relation.id, relation.from)));
            }
            if !endpoint_known(&relation.to) {
                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_model.validate-dangling-relation-to", semio_framework_diagnostic::TextSpan::at(1, 1), format!("relation {:?} references missing to-id {:?}", relation.id, relation.to)));
            }
        }

        diagnostics
    }

    impl SubsetValidator for SemioModelValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<semio_framework_diagnostic::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioModelSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => semio_model_referential_diagnostics(&snapshot),
                None => vec![semio_framework_diagnostic::Diagnostic::error("stdio.semio_model.validate-decode-failed", semio_framework_diagnostic::TextSpan::at(1, 1), "SemioModelValidator: payload did not decode as a SemioModelSnapshot".to_string())],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioModelValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1::subsets::model::schema::semio_model_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_document_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("model") }, store::ArtifactCodec::bare::<SemioModelSnapshot, crate::standards::v1::subsets::model::schema::mutations::SemioModelMutation>(
            crate::standards::v1::subsets::model::schema::snapshot::STDIO_SEMIOMODEL_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-model")]
        register_composer_entries(io_bridge_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 🧾️ The declarative twin of [`register`]: this subset's schema, document codec, `SubsetValidator`, composers
    /// (those writing semio, [`crate::semio_written`]) and inference descriptor as rows of [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        let builder = builder
            .schemas([crate::standards::v1::subsets::model::schema::semio_model_artifact_schema_descriptor()])
            .document_codec_bare::<SemioModelSnapshot, crate::standards::v1::subsets::model::schema::mutations::SemioModelMutation>(crate::standards::v1::subsets::model::schema::snapshot::STDIO_SEMIOMODEL_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("model") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::standards::v1::subsets::model::schema::inferences::semio_model_artifact_inference_descriptor()]);
        #[cfg(feature = "conversion-model")]
        let builder = {
            static COMPOSERS: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
            builder.composers(crate::semio_written(io_bridge_entries(), &COMPOSERS))
        };
        builder
    }

    /// 💡️ Registers `s.stdio.semio.model.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1::subsets::model::schema::inferences::semio_model_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register

    //#region 🔖️IoBridges
    /// 🌉️ W4 real semio↔format bridge entries. Each `deserializer_entry_of`/`serializer_entry_of`
    /// pair registers BOTH `IoKey` directions per `register_composer_entries`'s own doc comment (a
    /// deserializer writing `model`/reading `<format>` also gives `<format>`-exports-to-`model`; its
    /// mirror serializer gives the other two) — four `IoKey`s per (subset, format) pair from these two
    /// rows, no hand-written reverse registration needed.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    #[cfg(feature = "conversion-model")]
    fn io_bridge_entries() -> &'static [ComposerEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    deserializer_entry_of::<crate::standards::v1::subsets::model::io::import::deserializers::artifacts::ifc::v4::any::SemioModelFromIfc>(),
                    serializer_entry_of::<crate::standards::v1::subsets::model::io::export::serializers::artifacts::ifc::v4::any::SemioModelToIfc>(),
                    deserializer_entry_of::<crate::standards::v1::subsets::model::io::import::deserializers::artifacts::bcf::v2_1::any::SemioModelFromBcf>(),
                    serializer_entry_of::<crate::standards::v1::subsets::model::io::export::serializers::artifacts::bcf::v2_1::any::SemioModelToBcf>(),
                ]
            })
            .as_slice()
    }
    //#endregion 🔖️IoBridges

    //#region 🔖️Tests
    #[cfg(all(test, feature = "conversion-model"))]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::standards::v1::subsets::model::schema::diff::SemioModelDiff;
    use crate::standards::v1::subsets::model::schema::mutations::{SemioModelMutation};
    use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioModelBuilderConstruction {
        snapshot: SemioModelSnapshot,
    }

    impl ArtifactBuilder for SemioModelBuilderConstruction {
        type Snapshot = SemioModelSnapshot;
        type Mutation = SemioModelMutation;
        type Diff = SemioModelDiff;
        fn empty() -> Self {
            Self { snapshot: SemioModelSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioModelSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <SemioModelMutation as protocol::Mutation<SemioModelSnapshot>>::diff(&mutation, &self.snapshot);
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
    use crate::standards::v1::subsets::model::schema::snapshot::{SemioModelSnapshot, STDIO_SEMIOMODEL_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoConfidence,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioModelParts {
        pub snapshot: Option<SemioModelSnapshot>,
    }

    pub struct SemioModelAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioModelAnalyzerAnalysis {
        type Parts = SemioModelParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("model") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOMODEL_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioModelParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioModelSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
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
    pub spec SemioModelBuilderFacets {
        construction: SemioModelBuilderConstruction,
        analysis: SemioModelAnalyzerAnalysis,
        composition: crate::standards::v1::subsets::model::io::derived_composition::SemioModelComposerComposition,
    }
    builder: SemioModelBuilder,
    analyzer: SemioModelAnalyzer,
    composer: SemioModelComposer,
);
