//! 📚️ Example `synthetic-orbit` — the first remodeling document in the repo that actually feeds the
//! reconstruction pipeline.
//!
//! Every other shipped remodeling document has zero frames, so `run-reconstruction` short-circuits on
//! it (`🎮️commands/🏗️run-reconstruction/🦀️.rs`'s empty-frames guard) and the whole
//! photogrammetry stack stays unexercised. This example ships thirty-six real PNG views (10° apart) of a deterministic
//! textured-cube scene together with the exact camera intrinsics, per-frame extrinsics and 3D points
//! they were rendered from (`🖼️assets/🔮️ground-truth.json`), so a reconstruction run can be scored
//! against truth rather than merely observed to terminate.
//!
//! The frames are media, not document state: `RemodelingSnapshot::assets` holds composed child
//! handles, so pixels reach a document only through `create-asset`. The committed DSL is therefore the
//! fixture's canonical END STATE — stream, calibration and the full frame table under this module's
//! own asset ids — while a loader materialises it by dispatching one `import-frame-payload` per
//! `FRAMES` row, the same path a file-picker drop takes (that command mints its own stream-scoped
//! asset ids; the ids below are the fixture's, and are what the ground truth is keyed by).
//! Regenerate everything with `repo ./📜️script.ts regenerate-example` (the `#[test] #[ignore]` in
//! `🧪️tests/🧩️example/🦀️.rs`).

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "synthetic-orbit";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Synthetic Orbit", "Synthetischer Orbit")
}
pub const ICON: &str = "camera";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️.dsl.semio");

/// 🔮️ Ground truth the frames were rendered from — intrinsics, per-frame extrinsics and world points.
pub const GROUND_TRUTH_JSON: &str = include_str!("🖼️assets/🔮️ground-truth.json");

/// 🎞️ Stream the DSL declares and the frame asset ids resolve against.
pub const STREAM_ID: &str = "synthetic-orbit";

/// 🎯️ Calibrated camera the DSL declares for {@link STREAM_ID}.
pub const CAMERA_ID: &str = "synthetic-cam";

/// 🖼️ Mime of every committed frame — the import path decodes it with the plugin's own PNG codec.
pub const FRAME_MIME: &str = "image/png";

/// 🎞️ The four orbit quarters, nine views each, in capture order.
pub const QUARTERS: [&[(&str, &[u8])]; 4] = [&semio_s_artifact_remodel_remodeling_orbit_q1::examples::synthetic_orbit::quarter_1::FRAMES, &semio_s_artifact_remodel_remodeling_orbit_q2::examples::synthetic_orbit::quarter_2::FRAMES, &semio_s_artifact_remodel_remodeling_orbit_q3::examples::synthetic_orbit::quarter_3::FRAMES, &semio_s_artifact_remodel_remodeling_orbit_q4::examples::synthetic_orbit::quarter_4::FRAMES];

/// 🎞️ `(asset id, PNG bytes)` for every view, in capture order.
pub fn frames() -> impl Iterator<Item = &'static (&'static str, &'static [u8])> {
    QUARTERS.iter().flat_map(|quarter| quarter.iter())
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
