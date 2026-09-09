//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
    use crate::standards::v1::subsets::model::schema::SemioModelAnalyzer;
    #[cfg(feature = "conversion-model")]
    use semio_framework_plugin::{deserializer_entry_of, register_composer_entries, serializer_entry_of, ComposerEntry};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
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
    pub fn semio_model_referential_diagnostics(snapshot: &SemioModelSnapshot) -> Vec<dsl::Diagnostic> {
        let spatial_ids: HashSet<&str> = snapshot.spatial.iter().map(|n| n.id.as_str()).collect();
        let element_ids: HashSet<&str> = snapshot.elements.iter().map(|e| e.id.as_str()).collect();
        let mut diagnostics = Vec::new();

        for node in &snapshot.spatial {
            if let Some(parent) = &node.parent_id {
                if parent == &node.id {
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_model.validate-self-parent", dsl::TextSpan::at(1, 1), format!("spatial node {:?} is its own parent", node.id)));
                } else if !spatial_ids.contains(parent.as_str()) {
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_model.validate-dangling-parent", dsl::TextSpan::at(1, 1), format!("spatial node {:?} references missing parent {:?}", node.id, parent)));
                }
            }
        }

        for element in &snapshot.elements {
            if let Some(spatial_id) = &element.spatial_id {
                if !spatial_ids.contains(spatial_id.as_str()) {
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_model.validate-dangling-spatial-ref", dsl::TextSpan::at(1, 1), format!("element {:?} references missing spatial node {:?}", element.id, spatial_id)));
                }
            }
        }

        for relation in &snapshot.relations {
            let endpoint_known = |id: &str| element_ids.contains(id) || spatial_ids.contains(id);
            if !endpoint_known(&relation.from) {
                diagnostics.push(dsl::Diagnostic::error("stdio.semio_model.validate-dangling-relation-from", dsl::TextSpan::at(1, 1), format!("relation {:?} references missing from-id {:?}", relation.id, relation.from)));
            }
            if !endpoint_known(&relation.to) {
                diagnostics.push(dsl::Diagnostic::error("stdio.semio_model.validate-dangling-relation-to", dsl::TextSpan::at(1, 1), format!("relation {:?} references missing to-id {:?}", relation.id, relation.to)));
            }
        }

        diagnostics
    }

    impl SubsetValidator for SemioModelValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioModelSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => semio_model_referential_diagnostics(&snapshot),
                None => vec![dsl::Diagnostic::error("stdio.semio_model.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioModelValidator: payload did not decode as a SemioModelSnapshot".to_string())],
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
        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::model::schema::semio_model_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioModelSnapshot, crate::standards::v1::subsets::model::schema::mutations::SemioModelMutation>(
            crate::standards::v1::subsets::model::schema::snapshot::STDIO_SEMIOMODEL_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-model")]
        register_composer_entries(io_bridge_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.model.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::model::schema::inferences::semio_model_artifact_inference_descriptor());
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
