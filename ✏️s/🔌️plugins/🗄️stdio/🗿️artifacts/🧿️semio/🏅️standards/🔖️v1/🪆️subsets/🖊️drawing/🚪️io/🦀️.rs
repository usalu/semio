//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    #[cfg(feature = "conversion-drawing")]
    use crate::standards::v1::subsets::drawing::io::export::serializers::artifacts::dxf::v_r12::any::SemioDrawingToDxf;
    #[cfg(feature = "conversion-drawing")]
    use crate::standards::v1::subsets::drawing::io::export::serializers::artifacts::pdf::v1_7::any::SemioDrawingToPdf;
    #[cfg(feature = "conversion-drawing")]
    use crate::standards::v1::subsets::drawing::io::export::serializers::artifacts::svg::v1_1::any::SemioDrawingToSvg;
    #[cfg(feature = "conversion-drawing")]
    use crate::standards::v1::subsets::drawing::io::import::deserializers::artifacts::dxf::v_r12::any::SemioDrawingFromDxf;
    #[cfg(feature = "conversion-drawing")]
    use crate::standards::v1::subsets::drawing::io::import::deserializers::artifacts::pdf::v1_7::any::SemioDrawingFromPdf;
    #[cfg(feature = "conversion-drawing")]
    use crate::standards::v1::subsets::drawing::io::import::deserializers::artifacts::svg::v1_1::any::SemioDrawingFromSvg;
    use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
    use crate::standards::v1::subsets::drawing::schema::SemioDrawingAnalyzer;
    use semio_framework_plugin::{
        deserializer_entry_of, register_composer_entries, register_subset_validator, serializer_entry_of, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, ComposerEntry, Composition, Dialect, IoPayload,
        StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    };

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };

    //#region 🔖️Composer
    pub struct SemioDrawingComposerComposition;

    impl ArtifactComposition for SemioDrawingComposerComposition {
        type Snapshot = SemioDrawingSnapshot;
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
                return Err(ComposeError { message: "SemioDrawingComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioDrawingAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioDrawingComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decodes the payload as `SemioDrawingSnapshot` and checks two real referential invariants
    /// (both real cross-collection lookups, not decode-only): (1) every `Path`/`Text` node's
    /// `style` reference resolves to a name present in `styles` (dangling-ref detection); (2) every
    /// `DrawLayer.id` is unique across `layers` (duplicate-id detection).
    pub struct SemioDrawingValidator;

    impl SubsetValidator for SemioDrawingValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_drawing_invariants(&snapshot),
                None => vec![dsl::Diagnostic::error("stdio.semio_drawing.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioDrawingValidator: payload did not decode as a SemioDrawingSnapshot".to_string())],
            }
        }
    }

    /// 🔎️ Real referential-invariant checks over `SemioDrawingSnapshot`'s own collections (no
    /// cross-artifact lookups needed -- both invariants are internal to this subset).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_drawing_invariants(snapshot: &SemioDrawingSnapshot) -> Vec<dsl::Diagnostic> {
        let mut diagnostics = Vec::new();

        let mut seen_layer_ids = std::collections::HashSet::new();
        for layer in &snapshot.layers {
            if !seen_layer_ids.insert(layer.id.clone()) {
                diagnostics.push(dsl::Diagnostic::error("stdio.semio_drawing.duplicate-layer-id", dsl::TextSpan::at(1, 1), format!("SemioDrawingValidator: duplicate layer id {:?}", layer.id)));
            }
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn walk(node: &DrawNode, style_names: &std::collections::HashSet<&str>, diagnostics: &mut Vec<dsl::Diagnostic>) {
            match node {
                DrawNode::Path { style: Some(name), .. } | DrawNode::Text { style: Some(name), .. } => {
                    if !style_names.contains(name.as_str()) {
                        diagnostics.push(dsl::Diagnostic::error("stdio.semio_drawing.dangling-style-ref", dsl::TextSpan::at(1, 1), format!("SemioDrawingValidator: node references undefined style {name:?}")));
                    }
                }
                DrawNode::Group { children, .. } => {
                    for child in children {
                        walk(child, style_names, diagnostics);
                    }
                }
                _ => {}
            }
        }
        let style_names: std::collections::HashSet<&str> = snapshot.styles.iter().map(|s| s.name.as_str()).collect();
        for layer in &snapshot.layers {
            walk(&layer.root, &style_names, &mut diagnostics);
        }

        diagnostics
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioDrawingValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    /// 🚪️ W4 (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT, group
    /// G4): drawing↔svg/dxf/pdf. svg is the richest (recursive scene-graph↔scene-graph); dxf is a
    /// real entity↔path translation (exact circles, sampled-flattened curves on export); pdf is an
    /// honestly text-only bridge (this codec's own snapshot never exposes decoded content-stream
    /// vector ops) — see each pair's own leaf doc comment for the full rationale.
    #[cfg(feature = "conversion-drawing")]
    static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    #[cfg(feature = "conversion-drawing")]
    fn io_entries() -> &'static [ComposerEntry] {
        IO_ENTRIES
            .get_or_init(|| {
                vec![
                    deserializer_entry_of::<SemioDrawingFromSvg>(),
                    serializer_entry_of::<SemioDrawingToSvg>(),
                    deserializer_entry_of::<SemioDrawingFromDxf>(),
                    serializer_entry_of::<SemioDrawingToDxf>(),
                    deserializer_entry_of::<SemioDrawingFromPdf>(),
                    serializer_entry_of::<SemioDrawingToPdf>(),
                ]
            })
            .as_slice()
    }
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and (W4) its
    /// semio↔format io bridges. Called from this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::drawing::schema::semio_drawing_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioDrawingSnapshot, crate::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation>(
            crate::standards::v1::subsets::drawing::schema::snapshot::STDIO_SEMIODRAWING_DOCUMENT_SCHEMA,
        )).expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-drawing")]
        register_composer_entries(io_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.drawing.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::drawing::schema::inferences::semio_drawing_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(all(test, feature = "conversion-drawing"))]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
