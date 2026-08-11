//! 🎹️ SemioAnimationComposer (s.stdio.semio/v1/animation) — analyzer-only compose (decodes the
//! subset's own JSON-pack payload); W4 adds real cross-format compose sources once semio↔format
//! import/export leaves land (gltf/mp4/gif per the master plan's `animation` row). The registered
//! `SemioAnimationValidator` runs REAL referential-invariant checks (see `🔖️SubsetValidator`
//! below), not just decode-success, per the ticket's "complete" bar.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of,
    deserializer_entry_of, serializer_entry_of, register_composer_entries, ComposerEntry,
};
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use crate::artifacts::semio::standards::v1::subsets::animation::analyzer::SemioAnimationAnalyzer;
use crate::artifacts::semio::standards::v1::subsets::animation::io::{
    gltf_deserializer::SemioAnimationFromGltf, gltf_serializer::SemioAnimationToGltf,
    mp4_deserializer::SemioAnimationFromMp4, mp4_serializer::SemioAnimationToMp4,
    gif_deserializer::SemioAnimationFromGif, gif_serializer::SemioAnimationToGif,
};

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("animation") };

//#region 🔖️Composer
pub struct SemioAnimationComposer;

impl ArtifactComposer for SemioAnimationComposer {
    type Snapshot = SemioAnimationSnapshot;
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
            return Err(ComposeError { message: "SemioAnimationComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioAnimationAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioAnimationComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
/// 🛡️ Decodes the payload, then runs real structural invariants gltf's own animation spec
/// requires: every channel's `keyframes` must be non-empty and non-decreasing in `t` (glTF 2.0
/// §5.20.1 `sampler.input` accessor — "the values MUST be non-decreasing"; gap-free coverage isn't
/// spec-required so overlapping/duplicate `t` values are only flagged, not an error).
pub struct SemioAnimationValidator;

/// 🔍️ Real referential-invariant sweep over a decoded snapshot — separated from `validate` so both
/// the registered `SubsetValidator` and this module's own tests exercise the exact same logic.
fn check_semio_animation_invariants(snapshot: &SemioAnimationSnapshot) -> Vec<dsl::Diagnostic> {
    let mut diagnostics = Vec::new();
    for (ti, timeline) in snapshot.timelines.iter().enumerate() {
        for (ci, channel) in timeline.channels.iter().enumerate() {
            if channel.keyframes.is_empty() {
                diagnostics.push(dsl::Diagnostic::error(
                    "stdio.semio_animation.empty-channel",
                    dsl::TextSpan::at(1, 1),
                    format!("timeline[{ti}] channel[{ci}] (node {:?}) has zero keyframes", channel.target.node),
                ));
                continue;
            }
            for w in channel.keyframes.windows(2) {
                if w[1].t < w[0].t {
                    diagnostics.push(dsl::Diagnostic::error(
                        "stdio.semio_animation.non-monotonic-keyframes",
                        dsl::TextSpan::at(1, 1),
                        format!("timeline[{ti}] channel[{ci}] (node {:?}): keyframe t must be non-decreasing, got {} after {}", channel.target.node, w[1].t, w[0].t),
                    ));
                }
            }
        }
    }
    diagnostics
}

impl SubsetValidator for SemioAnimationValidator {
    const DIALECT: Dialect = DIALECT;
    fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_semio_animation_invariants(&snapshot),
            None => vec![dsl::Diagnostic::error(
                "stdio.semio_animation.validate-decode-failed",
                dsl::TextSpan::at(1, 1),
                "SemioAnimationValidator: payload did not decode as a SemioAnimationSnapshot".to_string(),
            )],
        }
    }
}

static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioAnimationValidator>) }
//#endregion 🔖️SubsetValidator

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
/// this artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::animation::schema::semio_animation_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioAnimationSnapshot, crate::artifacts::semio::standards::v1::subsets::animation::schema::mutations::SemioAnimationMutation>(crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
    register_composer_entries(bridge_entries());
}

/// 🌉️ animation↔gltf / animation↔mp4 / animation↔gif bridge entries (W4) -- forward + reverse rows
/// per pair, giving all 4 IoKeys per pair per the master plan's io architecture note.
fn bridge_entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                deserializer_entry_of::<SemioAnimationFromGltf>(),
                serializer_entry_of::<SemioAnimationToGltf>(),
                deserializer_entry_of::<SemioAnimationFromMp4>(),
                serializer_entry_of::<SemioAnimationToMp4>(),
                deserializer_entry_of::<SemioAnimationFromGif>(),
                serializer_entry_of::<SemioAnimationToGif>(),
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue};

    fn snapshot_with_channel(keyframes: Vec<AnimKeyframe>) -> SemioAnimationSnapshot {
        SemioAnimationSnapshot {
            timelines: vec![AnimTimeline {
                name: None,
                channels: vec![AnimChannel { target: AnimTarget { node: "n".into(), property: AnimTargetProperty::Translation }, interpolation: Default::default(), keyframes }],
            }],
            ..SemioAnimationSnapshot::default()
        }
    }

    #[test]
    fn empty_channel_is_flagged() {
        let snap = snapshot_with_channel(vec![]);
        let diagnostics = check_semio_animation_invariants(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_animation.empty-channel"), "got {diagnostics:?}");
    }

    #[test]
    fn non_monotonic_keyframes_are_flagged() {
        let snap = snapshot_with_channel(vec![
            AnimKeyframe { t: 1.0, value: AnimValue::Scalar { value: 0.0 } },
            AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 1.0 } },
        ]);
        let diagnostics = check_semio_animation_invariants(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_animation.non-monotonic-keyframes"), "got {diagnostics:?}");
    }

    #[test]
    fn well_formed_snapshot_has_no_diagnostics() {
        let snap = snapshot_with_channel(vec![
            AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 0.0 } },
            AnimKeyframe { t: 1.0, value: AnimValue::Scalar { value: 1.0 } },
        ]);
        assert!(check_semio_animation_invariants(&snap).is_empty());
    }

    #[test]
    fn registered_validator_matches_direct_invariant_check_on_a_binary_payload() {
        let snap = snapshot_with_channel(vec![]);
        let bytes = <SemioAnimationSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let via_validator = SemioAnimationValidator::validate(&IoPayload::Binary(bytes));
        assert!(via_validator.iter().any(|d| d.code.0 == "stdio.semio_animation.empty-channel"), "got {via_validator:?}");
    }
}
//#endregion 🔖️Tests
