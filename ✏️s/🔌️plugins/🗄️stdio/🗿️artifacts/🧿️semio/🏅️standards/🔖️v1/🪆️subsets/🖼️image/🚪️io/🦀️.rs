//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    #[cfg(feature = "conversion-image")]
    use crate::standards::v1::subsets::image::io::export::serializers::artifacts::bmp::v_v3::any::SemioImageToBmp;
    #[cfg(feature = "conversion-image")]
    use crate::standards::v1::subsets::image::io::export::serializers::artifacts::gif::v89a::any::SemioImageToGif;
    #[cfg(feature = "conversion-image")]
    use crate::standards::v1::subsets::image::io::export::serializers::artifacts::jpg::v_jfif_1_01::any::SemioImageToJpg;
    #[cfg(feature = "conversion-image")]
    use crate::standards::v1::subsets::image::io::export::serializers::artifacts::png::v1_2::any::SemioImageToPng;
    #[cfg(feature = "conversion-image")]
    use crate::standards::v1::subsets::image::io::export::serializers::artifacts::tiff::v6_0::any::SemioImageToTiff;
    #[cfg(feature = "conversion-image")]
    use crate::standards::v1::subsets::image::io::import::deserializers::artifacts::bmp::v_v3::any::SemioImageFromBmp;
    #[cfg(feature = "conversion-image")]
    use crate::standards::v1::subsets::image::io::import::deserializers::artifacts::gif::v89a::any::SemioImageFromGif;
    #[cfg(feature = "conversion-image")]
    use crate::standards::v1::subsets::image::io::import::deserializers::artifacts::jpg::v_jfif_1_01::any::SemioImageFromJpg;
    #[cfg(feature = "conversion-image")]
    use crate::standards::v1::subsets::image::io::import::deserializers::artifacts::png::v1_2::any::SemioImageFromPng;
    #[cfg(feature = "conversion-image")]
    use crate::standards::v1::subsets::image::io::import::deserializers::artifacts::tiff::v6_0::any::SemioImageFromTiff;
    use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
    use crate::standards::v1::subsets::image::schema::SemioImageAnalyzer;
    #[cfg(feature = "conversion-image")]
    use semio_framework_plugin::{deserializer_entry_of, register_composer_entries, serializer_entry_of, ComposerEntry};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

    //#region 🔖️Composer
    pub struct SemioImageComposerComposition;

    impl ArtifactComposition for SemioImageComposerComposition {
        type Snapshot = SemioImageSnapshot;
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
                return Err(ComposeError { message: "SemioImageComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioImageAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioImageComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ 🚧 scaffolded by W1b — decode-only validator (no referential-invariant diagnostics yet;
    /// W2 adds real cross-reference checks).
    pub struct SemioImageValidator;

    impl SubsetValidator for SemioImageValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioImageSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(_) => Vec::new(),
                None => vec![dsl::Diagnostic::error("stdio.semio_image.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioImageValidator: payload did not decode as a SemioImageSnapshot".to_string())],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioImageValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    /// 🚪️ W4 (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT, group
    /// G4): the five raster-format bridges (png/jpg/gif/bmp/tiff), each a deserializer+serializer
    /// pair. Per `register_composer_entries`'s own doc comment, ONE entry registers BOTH its import
    /// AND (symmetrically) the counterpart's export `IoKey` — a deserializer (writes image, reads
    /// fmt) plus its mirror serializer (writes fmt, reads image) together cover all four `IoKey`s per
    /// format without hand-writing each direction separately.
    #[cfg(feature = "conversion-image")]
    static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    #[cfg(feature = "conversion-image")]
    fn io_entries() -> &'static [ComposerEntry] {
        IO_ENTRIES
            .get_or_init(|| {
                vec![
                    deserializer_entry_of::<SemioImageFromPng>(),
                    serializer_entry_of::<SemioImageToPng>(),
                    deserializer_entry_of::<SemioImageFromJpg>(),
                    serializer_entry_of::<SemioImageToJpg>(),
                    deserializer_entry_of::<SemioImageFromGif>(),
                    serializer_entry_of::<SemioImageToGif>(),
                    deserializer_entry_of::<SemioImageFromBmp>(),
                    serializer_entry_of::<SemioImageToBmp>(),
                    deserializer_entry_of::<SemioImageFromTiff>(),
                    serializer_entry_of::<SemioImageToTiff>(),
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
        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::image::schema::semio_image_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioImageSnapshot, crate::standards::v1::subsets::image::schema::mutations::SemioImageMutation>(
            crate::standards::v1::subsets::image::schema::snapshot::STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-image")]
        register_composer_entries(io_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.image.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::image::schema::inferences::semio_image_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(all(test, feature = "conversion-image"))]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
