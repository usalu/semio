//! 🎹️ SemioVideoComposer (`s.stdio.semio`/`v1`/`video`) — analyzer-only native compose (decodes
//! the subset's own JSON-pack payload) PLUS the real `video↔mp4`/`video↔avi` cross-format bridge
//! entries (W4), registered here via `deserializer_entry_of`/`serializer_entry_of` alongside the
//! native dialect. This composer's own `ArtifactComposer::compose` shape (native dialect only,
//! delegating to the analyzer) is unchanged -- the bridge entries are separate `ComposerEntry` rows
//! in the same `IoKey → ComposerEntry` registry, not additional sources this composer itself reads.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of,
    deserializer_entry_of, serializer_entry_of, register_composer_entries,
};
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioVideoSnapshot, SemioVideoStreamKind};
use crate::artifacts::semio::standards::v1::subsets::video::analyzer::SemioVideoAnalyzer;
use crate::artifacts::semio::standards::v1::subsets::video::io::{
    mp4_deserializer::SemioVideoFromMp4, mp4_serializer::SemioVideoToMp4,
    avi_deserializer::SemioVideoFromAvi, avi_serializer::SemioVideoToAvi,
};

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") };

//#region 🔖️Composer
pub struct SemioVideoComposer;

impl ArtifactComposer for SemioVideoComposer {
    type Snapshot = SemioVideoSnapshot;
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
            return Err(ComposeError { message: "SemioVideoComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioVideoAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioVideoComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
/// 🛡️ Real referential-invariant checks (decode-only was the W1b scaffold; this is the
/// per-subset check D5's validate-on-build hook is FOR): `rate.den` must never be zero (it is a
/// divisor everywhere frame timing is computed downstream); a `Video`-kind stream's `width`/
/// `height` must be nonzero (a video stream with a zero raster dimension is not decodable by any
/// real container reader); a stream's `samples` should carry monotonically nondecreasing `pts`
/// (soft — real containers legitimately reorder decode order vs. presentation order for B-frames,
/// so this is a `Warning`, never a hard `Error`, honestly reflecting that this subset cannot tell
/// decode order from presentation order from the metadata alone).
pub struct SemioVideoValidator;

/// 🧮️ Runs this subset's real referential-invariant checks against an already-decoded snapshot —
/// shared by the registered `SubsetValidator` (wire-payload recheck) and this file's own unit
/// tests (which exercise it directly against hand-built snapshots).
pub fn check_semio_video_invariants(snapshot: &SemioVideoSnapshot) -> Vec<dsl::Diagnostic> {
    let mut out = Vec::new();
    for (stream_index, stream) in snapshot.streams.iter().enumerate() {
        if stream.rate.den == 0 {
            out.push(dsl::Diagnostic::error(
                "stdio.semio_video.rate-zero-denominator",
                dsl::TextSpan::at(1, 1),
                format!("stream {stream_index}: rate denominator is 0 (rate.num={})", stream.rate.num),
            ));
        }
        if stream.kind == SemioVideoStreamKind::Video && (stream.width == 0 || stream.height == 0) {
            out.push(dsl::Diagnostic::error(
                "stdio.semio_video.video-stream-zero-dimension",
                dsl::TextSpan::at(1, 1),
                format!("stream {stream_index}: kind=Video but width={} height={}", stream.width, stream.height),
            ));
        }
        let mut prev_pts: Option<u64> = None;
        for (sample_index, sample) in stream.samples.iter().enumerate() {
            if let Some(prev) = prev_pts {
                if sample.pts < prev {
                    out.push(dsl::Diagnostic {
                        code: dsl::FaultCode::new("stdio.semio_video.pts-non-monotonic"),
                        severity: dsl::Severity::Warning,
                        span: dsl::TextSpan::at(1, 1),
                        message: format!("stream {stream_index} sample {sample_index}: pts {} < previous pts {prev} (allowed — decode order may legitimately differ from presentation order)", sample.pts),
                        expected: None,
                        scope: dsl::FaultScope::default(),
                    });
                }
            }
            prev_pts = Some(sample.pts);
        }
    }
    out
}

impl SubsetValidator for SemioVideoValidator {
    const DIALECT: Dialect = DIALECT;
    fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioVideoSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioVideoSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_semio_video_invariants(&snapshot),
            None => vec![dsl::Diagnostic::error(
                "stdio.semio_video.validate-decode-failed",
                dsl::TextSpan::at(1, 1),
                "SemioVideoValidator: payload did not decode as a SemioVideoSnapshot".to_string(),
            )],
        }
    }
}

static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioVideoValidator>) }
//#endregion 🔖️SubsetValidator

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec (`"s.stdio.semio.video"` — the
/// repo-wide-unique id `policyDocumentCodecDuplicateIds` checks statically), and SubsetValidator.
/// Called from this artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::video::schema::semio_video_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioVideoSnapshot, crate::artifacts::semio::standards::v1::subsets::video::schema::mutations::SemioVideoMutation>(crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
    register_composer_entries(bridge_entries());
}

/// 🌉️ video↔mp4 / video↔avi bridge entries (W4) -- forward (writes video, reads the format) +
/// reverse (writes the format, reads video) rows, giving all 4 IoKeys per the master plan's io
/// architecture note. Leaked to `'static` once, matching every other stdio composer's
/// `OnceLock<Vec<ComposerEntry>>` entries-table convention (e.g. mp4/isobmff's own subset
/// composer).
fn bridge_entries() -> &'static [semio_framework_plugin::ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                deserializer_entry_of::<SemioVideoFromMp4>(),
                serializer_entry_of::<SemioVideoToMp4>(),
                deserializer_entry_of::<SemioVideoFromAvi>(),
                serializer_entry_of::<SemioVideoToAvi>(),
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoStream};

    fn clean_snapshot() -> SemioVideoSnapshot {
        SemioVideoSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
            streams: vec![SemioVideoStream {
                kind: SemioVideoStreamKind::Video,
                codec: "h264".into(),
                width: 1920,
                height: 1080,
                rate: SemioRational { num: 30, den: 1 },
                samples: vec![
                    SemioVideoSample { pts: 0, key: true, data: vec![1] },
                    SemioVideoSample { pts: 33, key: false, data: vec![2] },
                ],
            }],
        }
    }

    #[test]
    fn clean_snapshot_has_no_diagnostics() {
        let diagnostics = check_semio_video_invariants(&clean_snapshot());
        assert!(diagnostics.is_empty(), "expected no diagnostics, got {diagnostics:?}");
    }

    #[test]
    fn zero_denominator_rate_is_a_hard_error() {
        let mut snap = clean_snapshot();
        snap.streams[0].rate.den = 0;
        let diagnostics = check_semio_video_invariants(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_video.rate-zero-denominator" && d.severity == dsl::Severity::Error));
    }

    #[test]
    fn zero_dimension_video_stream_is_a_hard_error() {
        let mut snap = clean_snapshot();
        snap.streams[0].width = 0;
        let diagnostics = check_semio_video_invariants(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_video.video-stream-zero-dimension" && d.severity == dsl::Severity::Error));
    }

    #[test]
    fn zero_dimension_non_video_stream_is_not_flagged() {
        let mut snap = clean_snapshot();
        snap.streams[0].kind = SemioVideoStreamKind::Audio;
        snap.streams[0].width = 0;
        snap.streams[0].height = 0;
        let diagnostics = check_semio_video_invariants(&snap);
        assert!(diagnostics.iter().all(|d| d.code.0 != "stdio.semio_video.video-stream-zero-dimension"));
    }

    #[test]
    fn non_monotonic_pts_is_a_soft_warning_not_a_hard_error() {
        let mut snap = clean_snapshot();
        // sample 0 has pts=0; force sample 1's pts BELOW it (a genuine decrease, not just equal).
        snap.streams[0].samples[1].pts = 0;
        snap.streams[0].samples.push(SemioVideoSample { pts: 33, key: false, data: vec![3] });
        snap.streams[0].samples[1].pts = snap.streams[0].samples[0].pts; // equal: allowed
        snap.streams[0].samples.push(SemioVideoSample { pts: 0, key: false, data: vec![4] }); // decrease vs prev (33)
        let diagnostics = check_semio_video_invariants(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_video.pts-non-monotonic" && d.severity == dsl::Severity::Warning));
        assert!(diagnostics.iter().all(|d| d.severity != dsl::Severity::Error), "non-monotonic pts must never be a hard error: {diagnostics:?}");
    }

    #[test]
    fn subset_validator_recheck_agrees_with_direct_invariant_check() {
        let snap = clean_snapshot();
        let bytes = <SemioVideoSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let diagnostics = SemioVideoValidator::validate(&IoPayload::Binary(bytes));
        assert!(diagnostics.is_empty(), "wire recheck must agree with the direct check for a clean snapshot: {diagnostics:?}");
    }
}
//#endregion 🧪️Tests
