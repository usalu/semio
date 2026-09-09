//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueSnapshot, ValueId};
    use crate::standards::v1::subsets::value::schema::SemioValueAnalyzer;
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
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioValueSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioValueSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            let snapshot = match decoded {
                Some(snapshot) => snapshot,
                None => {
                    return vec![dsl::Diagnostic::error("stdio.semio_value.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioValueValidator: payload did not decode as a SemioValueSnapshot".to_string())];
                }
            };

            let mut diagnostics = Vec::new();

            let known_ids: HashSet<&ValueId> = snapshot.nodes.iter().map(|n| &n.id).collect();
            let mut seen_ids: HashSet<&ValueId> = HashSet::new();
            for node in &snapshot.nodes {
                if !seen_ids.insert(&node.id) {
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_value.validate-duplicate-id", dsl::TextSpan::at(1, 1), format!("SemioValueValidator: duplicate value id '{}' in `nodes`", node.id.value)));
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
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_value.validate-dangling-ref", dsl::TextSpan::at(1, 1), format!("SemioValueValidator: Ref{{id: '{}'}} does not resolve to any entry in `nodes`", id.value)));
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
        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::value::schema::semio_value_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioValueSnapshot, crate::standards::v1::subsets::value::schema::mutations::SemioValueMutation>(
            crate::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-value")]
        register_composer_entries(io_bridge_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.value.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::value::schema::inferences::semio_value_artifact_inference_descriptor());
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
