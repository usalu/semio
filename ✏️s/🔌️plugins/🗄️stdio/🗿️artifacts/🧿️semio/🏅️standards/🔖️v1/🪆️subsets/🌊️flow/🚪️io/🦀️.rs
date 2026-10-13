//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::schema::snapshot::SemioFlowSnapshot;
    use crate::io::SemioFlowAnalyzer;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("flow") };

    //#region 🔖️Composer
    pub struct SemioFlowComposerComposition;

    impl ArtifactComposition for SemioFlowComposerComposition {
        type Snapshot = SemioFlowSnapshot;
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
                return Err(ComposeError { message: "SemioFlowComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioFlowAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioFlowComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decodes AND checks real referential invariants — not decode-only. A flow DAG snapshot is
    /// only well-formed if: (1) every node id is unique, (2) every edge id is unique, (3) every edge's
    /// `from.node`/`to.node` PortRef references an id that actually exists in `nodes`.
    pub struct SemioFlowValidator;

    /// 🔎️ Real referential-invariant checks over an already-decoded snapshot — factored out so the
    /// composer (if it ever gains a pre-serialization hard gate, pdf `✳️a`-style) and this validator's
    /// post-hoc wire recheck can share one implementation.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_flow_referential_invariants(snapshot: &SemioFlowSnapshot) -> Vec<semio_framework_diagnostic::Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut seen_node_ids = std::collections::HashSet::new();
        for node in &snapshot.nodes {
            if !seen_node_ids.insert(node.id.as_str()) {
                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_flow.duplicate-node-id", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioFlowValidator: duplicate node id {:?}", node.id)));
            }
        }
        let mut seen_edge_ids = std::collections::HashSet::new();
        for edge in &snapshot.edges {
            if !seen_edge_ids.insert(edge.id.as_str()) {
                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_flow.duplicate-edge-id", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioFlowValidator: duplicate edge id {:?}", edge.id)));
            }
            if !seen_node_ids.contains(edge.from.node.as_str()) {
                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_flow.dangling-edge-endpoint", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioFlowValidator: edge {:?}'s from.node {:?} references a node that does not exist", edge.id, edge.from.node)));
            }
            if !seen_node_ids.contains(edge.to.node.as_str()) {
                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_flow.dangling-edge-endpoint", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioFlowValidator: edge {:?}'s to.node {:?} references a node that does not exist", edge.id, edge.to.node)));
            }
        }
        diagnostics
    }

    impl SubsetValidator for SemioFlowValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<semio_framework_diagnostic::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioFlowSnapshot as store::os_store::serialization::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioFlowSnapshot as store::os_store::serialization::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_flow_referential_invariants(&snapshot),
                None => vec![semio_framework_diagnostic::Diagnostic::error("stdio.semio_flow.validate-decode-failed", semio_framework_diagnostic::TextSpan::at(1, 1), "SemioFlowValidator: payload did not decode as a SemioFlowSnapshot".to_string())],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioFlowValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and the
    /// flow<->json io bridge row. Called from this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::schema::semio_flow_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_snapshot_codec(Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("flow") }, store::artifact_codec::<SemioFlowSnapshot, crate::schema::mutations::SemioFlowMutation>(crate::schema::snapshot::STDIO_SEMIOFLOW_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 🧾️ The declarative twin of [`register`]: this subset's schema, document codec, `SubsetValidator`, composers
    /// (those writing semio, [`crate::semio_written`]) and inference descriptor as rows of [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        let builder = builder
            .schemas([crate::schema::semio_flow_artifact_schema_descriptor()])
            .document_codec_bare::<SemioFlowSnapshot, crate::schema::mutations::SemioFlowMutation>(crate::schema::snapshot::STDIO_SEMIOFLOW_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("flow") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::schema::inferences::semio_flow_artifact_inference_descriptor()]);
        builder
    }

    /// 💡️ Registers `s.stdio.semio.flow.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::schema::inferences::semio_flow_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
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
    use crate::schema::diff::SemioFlowDiff;
    use crate::schema::mutations::SemioFlowMutation;
    use crate::schema::snapshot::SemioFlowSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioFlowBuilderConstruction {
        snapshot: SemioFlowSnapshot,
    }

    impl ArtifactBuilder for SemioFlowBuilderConstruction {
        type Snapshot = SemioFlowSnapshot;
        type Mutation = SemioFlowMutation;
        type Diff = SemioFlowDiff;
        fn empty() -> Self {
            Self { snapshot: SemioFlowSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioFlowSnapshot as store::os_store::serialization::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioFlowSnapshot as store::os_store::serialization::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <SemioFlowMutation as protocol::Mutation<SemioFlowSnapshot>>::diff(&mutation, &self.snapshot);
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
    use crate::schema::snapshot::{SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioFlowParts {
        pub snapshot: Option<SemioFlowSnapshot>,
    }

    pub struct SemioFlowAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioFlowAnalyzerAnalysis {
        type Parts = SemioFlowParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("flow") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOFLOW_DOCUMENT_SCHEMA) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioFlowParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioFlowSnapshot as store::os_store::serialization::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioFlowSnapshot as store::os_store::serialization::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SemioFlowBuilderFacets {
        construction: SemioFlowBuilderConstruction,
        analysis: SemioFlowAnalyzerAnalysis,
        composition: SemioFlowComposerComposition,
    }
    builder: SemioFlowBuilder,
    analyzer: SemioFlowAnalyzer,
    composer: SemioFlowComposer,
);
