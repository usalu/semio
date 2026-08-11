//! 🎹️ SemioAudioComposer (s.stdio.semio/v1/audio) — analyzer-backed compose plus a real
//! referential-invariant `SubsetValidator` (per `w1b-type-ownership.md`: "13 real semio subsets
//! need real referential-invariant checks from W2" — decode-only was the W1b placeholder).

use dsl::{Diagnostic, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of,
    deserializer_entry_of, serializer_entry_of, register_composer_entries, ComposerEntry,
};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use crate::artifacts::semio::standards::v1::subsets::audio::analyzer::SemioAudioAnalyzer;
use crate::artifacts::semio::standards::v1::subsets::audio::io::{
    mp3_deserializer::SemioAudioFromMp3, mp3_serializer::SemioAudioToMp3,
    wav_deserializer::SemioAudioFromWav, wav_serializer::SemioAudioToWav,
};

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("audio") };

//#region 🔖️Composer
pub struct SemioAudioComposer;

impl ArtifactComposer for SemioAudioComposer {
    type Snapshot = SemioAudioSnapshot;
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
            return Err(ComposeError { message: "SemioAudioComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioAudioAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioAudioComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        let mut diagnostics = analysis.diagnostics;
        diagnostics.extend(check_semio_audio_invariants(&snapshot));
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️Invariants
/// 🛡️ Real referential/structural invariant checks over a decoded `SemioAudioSnapshot` — backs
/// both the composer's advisory diagnostics above and the registered `SubsetValidator` below (same
/// function, two call sites, matching pdf/a's `check_pdf_a_conformance` precedent). None of these
/// are hard compose-failures (audio has no PDF/A-style conformance gate) — every finding is
/// advisory, surfaced as a real `Diagnostic` rather than silently dropped.
pub fn check_semio_audio_invariants(snapshot: &SemioAudioSnapshot) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if snapshot.sample_rate == 0 {
        diagnostics.push(warning("stdio.semio_audio.zero-sample-rate", "sample_rate is 0 -- no real audio can play back at this rate".to_string()));
    }
    if let Some(first) = snapshot.channels.first() {
        let expected = first.samples.len();
        for (i, channel) in snapshot.channels.iter().enumerate().skip(1) {
            if channel.samples.len() != expected {
                diagnostics.push(warning(
                    "stdio.semio_audio.channel-length-mismatch",
                    format!("channel {i} has {} samples, channel 0 has {expected} -- channels are expected to be the same length", channel.samples.len()),
                ));
            }
        }
    }
    for (i, tag) in snapshot.tags.iter().enumerate() {
        if tag.key.is_empty() {
            diagnostics.push(warning("stdio.semio_audio.empty-tag-key", format!("tag {i} has an empty key")));
        }
    }
    diagnostics
}

fn warning(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: dsl::FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}
//#endregion 🔖️Invariants

//#region 🔖️SubsetValidator
/// 🛡️ The registered `SubsetValidator` for `s.stdio.semio/v1/audio` — decodes the wire payload as
/// this subset's own snapshot and re-runs the SAME `check_semio_audio_invariants` the composer
/// runs pre-serialization (matching pdf/a's own composer/validator split).
pub struct SemioAudioValidator;

impl SubsetValidator for SemioAudioValidator {
    const DIALECT: Dialect = DIALECT;
    fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioAudioSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_semio_audio_invariants(&snapshot),
            None => vec![Diagnostic {
                code: dsl::FaultCode::new("stdio.semio_audio.validate-decode-failed"),
                severity: Severity::Warning,
                span: TextSpan::at(1, 1),
                message: "SemioAudioValidator: payload did not decode as a SemioAudioSnapshot -- skipped".into(),
                expected: None,
                scope: FaultScope::default(),
            }],
        }
    }
}

static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioAudioValidator>) }
//#endregion 🔖️SubsetValidator

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec (`"s.stdio.semio.audio"` — the
/// document-codec id, repo-wide unique per the ticket's static policy check, distinct from every
/// other artifact's own document schema string), and `SubsetValidator`. Called from this
/// artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::audio::schema::semio_audio_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioAudioSnapshot, crate::artifacts::semio::standards::v1::subsets::audio::schema::mutations::SemioAudioMutation>(crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
    register_composer_entries(bridge_entries());
}

/// 🌉️ audio↔mp3 / audio↔wav bridge entries (W4) -- forward + reverse rows per pair, giving all 4
/// IoKeys per pair per the master plan's io architecture note.
fn bridge_entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                deserializer_entry_of::<SemioAudioFromMp3>(),
                serializer_entry_of::<SemioAudioToMp3>(),
                deserializer_entry_of::<SemioAudioFromWav>(),
                serializer_entry_of::<SemioAudioToWav>(),
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioTag};

    #[test]
    fn compose_decodes_a_real_binary_source_with_no_advisories() {
        let snapshot = SemioAudioSnapshot {
            sample_rate: 44_100,
            channels: vec![SemioAudioChannel { samples: vec![0.0, 1.0] }, SemioAudioChannel { samples: vec![0.0, -1.0] }],
            tags: vec![SemioAudioTag { key: "title".into(), value: "clean".into() }],
            ..SemioAudioSnapshot::default()
        };
        let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = SemioAudioComposer::compose(&sources).expect("clean document must compose");
        assert_eq!(composed.snapshot, snapshot);
        assert!(composed.diagnostics.is_empty(), "got {:?}", composed.diagnostics);
    }

    #[test]
    fn zero_sample_rate_surfaces_a_real_warning_not_silently() {
        let snapshot = SemioAudioSnapshot { sample_rate: 0, ..SemioAudioSnapshot::default() };
        let diagnostics = check_semio_audio_invariants(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_audio.zero-sample-rate" && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn mismatched_channel_lengths_surface_a_real_warning() {
        let snapshot = SemioAudioSnapshot {
            sample_rate: 44_100,
            channels: vec![SemioAudioChannel { samples: vec![0.0, 1.0, 2.0] }, SemioAudioChannel { samples: vec![0.0] }],
            ..SemioAudioSnapshot::default()
        };
        let diagnostics = check_semio_audio_invariants(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_audio.channel-length-mismatch"), "got {diagnostics:?}");
    }

    #[test]
    fn empty_tag_key_surfaces_a_real_warning() {
        let snapshot = SemioAudioSnapshot {
            sample_rate: 44_100,
            tags: vec![SemioAudioTag { key: String::new(), value: "orphaned".into() }],
            ..SemioAudioSnapshot::default()
        };
        let diagnostics = check_semio_audio_invariants(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_audio.empty-tag-key"), "got {diagnostics:?}");
    }

    #[test]
    fn subset_validator_recheck_matches_the_composer_side_invariants() {
        let snapshot = SemioAudioSnapshot { sample_rate: 0, ..SemioAudioSnapshot::default() };
        let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let diagnostics = SemioAudioValidator::validate(&IoPayload::Binary(bytes));
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_audio.zero-sample-rate"), "got {diagnostics:?}");
    }
}
//#endregion 🔖️Tests
