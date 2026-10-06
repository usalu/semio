//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueSnapshot, ValueId};
    use crate::standards::v1::subsets::value::io::SemioValueAnalyzer;
    #[cfg(feature = "conversion-value")]
    use semio_framework_plugin::{deserializer_entry_of, register_composer_entries, serializer_entry_of, ComposerEntry};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::collections::HashSet;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("value") };

    //#region 🔖️Composer
    pub struct SemioValueComposerComposition;

    impl ArtifactComposition for SemioValueComposerComposition {
        type Snapshot = SemioValueSnapshot;
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
                return Err(ComposeError { message: "SemioValueComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioValueAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioValueComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🕸️ Recursively collects every `Ref{id}` reachable from `value` — used against BOTH `root` and
    /// every `nodes` node's own `value` (a `Ref` can legally point from inside the graph back into
    /// itself, or into a sibling node, not only from `root`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn collect_refs(value: &SemioValue, out: &mut Vec<ValueId>) {
        match value {
            SemioValue::Ref { id } => out.push(id.clone()),
            SemioValue::List { items } => items.iter().for_each(|v| collect_refs(v, out)),
            SemioValue::Map { entries } => entries.iter().for_each(|e| collect_refs(&e.value, out)),
            _ => {}
        }
    }

    /// 🛡️ Decodes the payload as this subset's OWN `SemioValueSnapshot`, then checks two real
    /// referential invariants over its own collections: (1) every `Ref{id}` reachable from `root` or
    /// from any `nodes` node's value resolves to a real entry in `nodes` (no dangling ids); (2)
    /// `nodes` carries no duplicate `id` (the graph's backing store is id-ADDRESSABLE, a duplicate
    /// id makes resolution ambiguous).
    pub struct SemioValueValidator;

    impl SubsetValidator for SemioValueValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<semio_framework_diagnostic::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioValueSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioValueSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            let snapshot = match decoded {
                Some(snapshot) => snapshot,
                None => {
                    return vec![semio_framework_diagnostic::Diagnostic::error("stdio.semio_value.validate-decode-failed", semio_framework_diagnostic::TextSpan::at(1, 1), "SemioValueValidator: payload did not decode as a SemioValueSnapshot".to_string())];
                }
            };

            let mut diagnostics = Vec::new();

            let known_ids: HashSet<&ValueId> = snapshot.nodes.iter().map(|n| &n.id).collect();
            let mut seen_ids: HashSet<&ValueId> = HashSet::new();
            for node in &snapshot.nodes {
                if !seen_ids.insert(&node.id) {
                    diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_value.validate-duplicate-id", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioValueValidator: duplicate value id '{}' in `nodes`", node.id.value)));
                }
            }

            let mut refs = Vec::new();
            collect_refs(&snapshot.root, &mut refs);
            for node in &snapshot.nodes {
                collect_refs(&node.value, &mut refs);
            }
            let mut reported_dangling: HashSet<String> = HashSet::new();
            for id in refs {
                if !known_ids.contains(&id) && reported_dangling.insert(id.value.clone()) {
                    diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_value.validate-dangling-ref", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioValueValidator: Ref{{id: '{}'}} does not resolve to any entry in `nodes`", id.value)));
                }
            }

            diagnostics
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioValueValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1::subsets::value::schema::semio_value_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_snapshot_codec(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_plugin::StandardId("v1"), subset: semio_framework_plugin::SubsetId("value") }, store::ArtifactCodec::of::<SemioValueSnapshot, crate::standards::v1::subsets::value::schema::mutations::SemioValueMutation>(
            crate::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-value")]
        register_composer_entries(io_bridge_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 🧾️ The declarative twin of [`register`]: this subset's schema, document codec, `SubsetValidator`, composers
    /// (those writing semio, [`crate::semio_written`]) and inference descriptor as rows of [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        let builder = builder
            .schemas([crate::standards::v1::subsets::value::schema::semio_value_artifact_schema_descriptor()])
            .document_codec_bare::<SemioValueSnapshot, crate::standards::v1::subsets::value::schema::mutations::SemioValueMutation>(crate::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_plugin::StandardId("v1"), subset: semio_framework_plugin::SubsetId("value") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::standards::v1::subsets::value::schema::inferences::semio_value_artifact_inference_descriptor()]);
        #[cfg(feature = "conversion-value")]
        let builder = {
            static COMPOSERS: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
            builder.composers(crate::semio_written(io_bridge_entries(), &COMPOSERS))
        };
        builder
    }

    /// 💡️ Registers `s.stdio.semio.value.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1::subsets::value::schema::inferences::semio_value_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register

    //#region 🧪️Tests
    #[cfg(all(test, feature = "conversion-value"))]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🧪️Tests

    //#region 🔖️IoBridges
    /// 🌉️ W4 real semio↔format bridge entries. Each `deserializer_entry_of`/`serializer_entry_of`
    /// pair registers BOTH `IoKey` directions per `register_composer_entries`'s own doc comment.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    #[cfg(feature = "conversion-value")]
    fn io_bridge_entries() -> &'static [ComposerEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    deserializer_entry_of::<crate::standards::v1::subsets::value::io::import::deserializers::artifacts::json::v_rfc8259::any::SemioValueFromJson>(),
                    serializer_entry_of::<crate::standards::v1::subsets::value::io::export::serializers::artifacts::json::v_rfc8259::any::SemioValueToJson>(),
                    deserializer_entry_of::<crate::standards::v1::subsets::value::io::import::deserializers::artifacts::xml::v1_0::any::SemioValueFromXml>(),
                    serializer_entry_of::<crate::standards::v1::subsets::value::io::export::serializers::artifacts::xml::v1_0::any::SemioValueToXml>(),
                    deserializer_entry_of::<crate::standards::v1::subsets::value::io::import::deserializers::artifacts::csv::v_rfc4180::any::SemioValueFromCsv>(),
                    serializer_entry_of::<crate::standards::v1::subsets::value::io::export::serializers::artifacts::csv::v_rfc4180::any::SemioValueToCsv>(),
                ]
            })
            .as_slice()
    }
    //#endregion 🔖️IoBridges
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
    use crate::standards::v1::subsets::value::schema::diff::SemioValueTreeDiff;
    use crate::standards::v1::subsets::value::schema::mutations::{apply_semio_value_mutation, SemioValueMutation};
    use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioValueBuilderConstruction {
        snapshot: SemioValueSnapshot,
    }

    impl ArtifactBuilder for SemioValueBuilderConstruction {
        type Snapshot = SemioValueSnapshot;
        type Mutation = SemioValueMutation;
        type Diff = SemioValueTreeDiff;
        fn empty() -> Self {
            Self { snapshot: SemioValueSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioValueSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioValueSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_value_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioValueTreeDiff as protocol::MutationDiff<SemioValueSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v1::subsets::value::schema::snapshot::{SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioValueParts {
        pub snapshot: Option<SemioValueSnapshot>,
    }

    pub struct SemioValueAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioValueAnalyzerAnalysis {
        type Parts = SemioValueParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("value") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOVALUE_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioValueParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioValueSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioValueSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SemioValueBuilderFacets {
        construction: SemioValueBuilderConstruction,
        analysis: SemioValueAnalyzerAnalysis,
        composition: crate::standards::v1::subsets::value::io::derived_composition::SemioValueComposerComposition,
    }
    builder: SemioValueBuilder,
    analyzer: SemioValueAnalyzer,
    composer: SemioValueComposer,
);
