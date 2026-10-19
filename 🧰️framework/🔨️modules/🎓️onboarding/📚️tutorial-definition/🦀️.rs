//! 📚️ Tutorial definition: the tutorial, its chapters, base and tracks, the asset source, the narration cues, the captions and the overlay rectangle. See [`super`].

use super::*;

//#region 🔖️Tutorial
/// 🎬️ A recorded, timed, replayable walkthrough — the timeline sibling of the step-gated
/// `IntroductionDefinition`. Where an introduction gates progression on the user performing an
/// interaction, a tutorial plays a multi-track recording (narration, video overlay, UI state, document
/// edits, camera, ghost-cursor gestures) against a sandboxed copy of the document while the user watches,
/// scrubs, or deviates and converges back. A *recording* IS a `TutorialDefinition` — the recorder simply
/// produces a densely-sampled one; nothing distinguishes a hand-authored tutorial from a captured one.
/// Distinct from the docs-tooltip `tutorial` link field in `ui/js/react`'s `UiLabelLeaf` (a URL into the
/// manual) — this is the interactive playback mechanism.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub title: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedLabel>,
    /// ⏱️ Total timeline length in milliseconds; every track entry's `at` (+ duration) must fit within.
    pub duration_ms: u64,
    /// 📖️ Scrub-bar markers, sorted ascending by `at`.
    #[serde(default)]
    #[value(default)]
    pub chapters: Vec<TutorialChapter>,
    /// 🎬️ Starting conditions the player restores into its sandbox before t=0.
    pub base: TutorialBase,
    pub tracks: TutorialTracks,
    /// 🧾️ Recorder provenance (ISO 8601 timestamp); `None` means hand-authored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub recorded_at: Option<String>,
}

impl TutorialDefinition {
    /// 📂️ Deserializes a `TutorialDefinition` from its JSON wire format — the constructor apps use
    /// to load a hand-authored or recorded tutorial (e.g. via `include_str!`) into `.tutorial(...)`.
    pub async fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// 📖️ One scrub-bar marker in a `TutorialDefinition`'s timeline.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialChapter {
    pub id: String,
    pub at: u64,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub title: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<LocalizedLabel>,
}

/// 🎬️ What must be true at t=0: the document the tutorial sandboxes and the initial UI/camera
/// state. The player snapshots the user's live document, loads this in its place, and restores the
/// snapshot on exit — a tutorial can never touch real work.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialBase {
    /// 📂️ Full document DSL text (`ArtifactTextFiles.dsl`) to sandbox-load; `None` falls back to `example_id`, and both
    /// `None` falls back to the app's default/empty document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document_dsl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub example_id: Option<String>,
    pub ui: TutorialUiSnapshot,
    /// 🎥️ Initial camera per window instance (every entry's `at` is `0`).
    #[serde(default)]
    #[value(default)]
    pub cameras: Vec<TutorialCameraKeyframe>,
}

/// 🎞️ The seven parallel tracks of a `TutorialDefinition`'s timeline; every entry's `at` is a
/// millisecond offset from tutorial start, and each `Vec` is sorted ascending by `at`
/// (`validate_tutorial` enforces this).
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialTracks {
    #[serde(default)]
    #[value(default)]
    pub narration: Vec<TutorialNarrationCue>,
    #[serde(default)]
    #[value(default)]
    pub video: Vec<TutorialVideoCue>,
    /// 🏷️ Annotational only — drives affordance pulses and scrub-bar tick marks; playback never
    /// re-dispatches these into a plugin (see `TutorialEventKind`).
    #[serde(default)]
    #[value(default)]
    pub events: Vec<TutorialEvent>,
    #[serde(default)]
    #[value(default)]
    pub ui: Vec<TutorialUiKeyframe>,
    /// 🖋️ The sole source of document mutation during playback — see `TutorialDocumentEventKind`.
    #[serde(default)]
    #[value(default)]
    pub document: Vec<TutorialDocumentEvent>,
    #[serde(default)]
    #[value(default)]
    pub camera: Vec<TutorialCameraKeyframe>,
    #[serde(default)]
    #[value(default)]
    pub gestures: Vec<TutorialGestureCue>,
}

/// 📦️ Where a tutorial media asset's bytes live. `Blob` is wire-identical to `store::BlobRef`
/// (content-addressed Blake3 hash + size + media type) — `framework/core` does not depend on
/// `semio-vcs`, so the shape is mirrored rather than reused; conversion between the two is
/// field-for-field.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialAssetSrc {
    /// 🌐️ Static asset route (a brand's `assetsDir` or the shared `ui/asset` mount).
    Url { url: String },
    /// 🗄️ Content-addressed blob in the studio's `BlobStore`.
    Blob { hash: String, size: u64, media_type: String },
    /// 🧵️ Inline data URL — the recorder's default before a save destination is chosen.
    DataUrl { data: String },
}

// 🚫️async: E4 fn-pointer slot (serde default)
fn tutorial_narration_default_rate() -> f64 {
    1.0
}

// 🚫️async: E4 fn-pointer slot (serde skip_serializing_if)
fn tutorial_rate_is_default(rate: &f64) -> bool {
    (*rate - 1.0).abs() < f64::EPSILON
}

/// 🎙️ One voiceover cue: `text` is both the TTS script and the caption fallback; `audio`
/// overrides TTS with a recorded take. The timeline is always the master clock — a still-speaking TTS
/// utterance is cancelled at the next cue's `at`; audio assets are seeked and rate-matched to the
/// playhead instead of played independently.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialNarrationCue {
    pub id: String,
    pub at: u64,
    /// ⏱️ Audio duration when `audio` is set (recorder-measured); a rough TTS estimate otherwise — used
    /// for scrub-bar layout only, never to gate playback.
    pub duration_ms: u64,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub text: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<TutorialAssetSrc>,
    /// 🗣️ Web Speech API voice-name hint; ignored once `audio` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
    /// 🎚️ TTS/audio rate multiplier layered under the player's own playback-rate control.
    #[serde(default = "tutorial_narration_default_rate", skip_serializing_if = "tutorial_rate_is_default")]
    #[value(default = "tutorial_narration_default_rate", skip_serializing_if = "tutorial_rate_is_default")]
    pub rate: f64,
    /// 💬️ Timed caption sub-segments (offsets relative to this cue's `at`); empty means `text` is shown
    /// whole for the cue's `duration_ms`.
    #[serde(default)]
    #[value(default)]
    pub captions: Vec<TutorialCaption>,
}

/// 💬️ One timed caption sub-segment of a `TutorialNarrationCue`.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialCaption {
    pub at: u64,
    pub duration_ms: u64,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub text: LocalizedLabel,
}

/// 🖼️ Normalized 0–1 viewport rect for a `TutorialVideoCue` overlay.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialOverlayRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Default for TutorialOverlayRect {
    /// 📌️ Bottom-right picture-in-picture, ~16:9.
    fn default() -> Self {
        Self { x: 0.72, y: 0.70, width: 0.24, height: 0.24 }
    }
}
//#endregion 🔖️Tutorial
