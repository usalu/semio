//! 🎹️ SemioImageComposer (s.stdio.semio/v1/image) — 🚧 scaffolded by W1b:
//! analyzer-only compose (decodes the subset's own JSON-pack payload). W2/W4 add real
//! cross-format compose sources once semio↔format import/export leaves land.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, ComposerEntry, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of, register_composer_entries, deserializer_entry_of, serializer_entry_of,
};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::analyzer::SemioImageAnalyzer;
use crate::artifacts::semio::standards::v1::subsets::image::io::import::deserializers::artifacts::png::v1_2::any::SemioImageFromPng;
use crate::artifacts::semio::standards::v1::subsets::image::io::export::serializers::artifacts::png::v1_2::any::SemioImageToPng;
use crate::artifacts::semio::standards::v1::subsets::image::io::import::deserializers::artifacts::jpg::v_jfif_1_01::any::SemioImageFromJpg;
use crate::artifacts::semio::standards::v1::subsets::image::io::export::serializers::artifacts::jpg::v_jfif_1_01::any::SemioImageToJpg;
use crate::artifacts::semio::standards::v1::subsets::image::io::import::deserializers::artifacts::gif::v89a::any::SemioImageFromGif;
use crate::artifacts::semio::standards::v1::subsets::image::io::export::serializers::artifacts::gif::v89a::any::SemioImageToGif;
use crate::artifacts::semio::standards::v1::subsets::image::io::import::deserializers::artifacts::bmp::v_v3::any::SemioImageFromBmp;
use crate::artifacts::semio::standards::v1::subsets::image::io::export::serializers::artifacts::bmp::v_v3::any::SemioImageToBmp;
use crate::artifacts::semio::standards::v1::subsets::image::io::import::deserializers::artifacts::tiff::v6_0::any::SemioImageFromTiff;
use crate::artifacts::semio::standards::v1::subsets::image::io::export::serializers::artifacts::tiff::v6_0::any::SemioImageToTiff;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

//#region 🔖️Composer
pub struct SemioImageComposer;

impl ArtifactComposer for SemioImageComposer {
    type Snapshot = SemioImageSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] { &[DIALECT] }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let native: Vec<AnalyzeSource<'_>> = sources
            .iter()
            .filter(|s| s.dialect == DIALECT)
            .map(|s| match &s.payload {
                AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
            })
            .collect();
        if native.is_empty() {
            return Err(ComposeError { message: "SemioImageComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioImageAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioImageComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
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
    fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioImageSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(_) => Vec::new(),
            None => vec![dsl::Diagnostic::error(
                "stdio.semio_image.validate-decode-failed",
                dsl::TextSpan::at(1, 1),
                "SemioImageValidator: payload did not decode as a SemioImageSnapshot".to_string(),
            )],
        }
    }
}

static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioImageValidator>) }
//#endregion 🔖️SubsetValidator

//#region 🔖️IoEntries
/// 🚪️ W4 (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT, group
/// G4): the five raster-format bridges (png/jpg/gif/bmp/tiff), each a deserializer+serializer
/// pair. Per `register_composer_entries`'s own doc comment, ONE entry registers BOTH its import
/// AND (symmetrically) the counterpart's export `IoKey` — a deserializer (writes image, reads
/// fmt) plus its mirror serializer (writes fmt, reads image) together cover all four `IoKey`s per
/// format without hand-writing each direction separately.
static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
fn io_entries() -> &'static [ComposerEntry] {
    IO_ENTRIES.get_or_init(|| vec![
        deserializer_entry_of::<SemioImageFromPng>(), serializer_entry_of::<SemioImageToPng>(),
        deserializer_entry_of::<SemioImageFromJpg>(), serializer_entry_of::<SemioImageToJpg>(),
        deserializer_entry_of::<SemioImageFromGif>(), serializer_entry_of::<SemioImageToGif>(),
        deserializer_entry_of::<SemioImageFromBmp>(), serializer_entry_of::<SemioImageToBmp>(),
        deserializer_entry_of::<SemioImageFromTiff>(), serializer_entry_of::<SemioImageToTiff>(),
    ]).as_slice()
}
//#endregion 🔖️IoEntries

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and (W4) its
/// semio↔format io bridges. Called from this artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::image::schema::semio_image_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioImageSnapshot, crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation>(crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
    register_composer_entries(io_entries());
}
//#endregion 🔖️Register
