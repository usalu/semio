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
//! Regenerate everything with `bun ./📜️script.ts regenerate-example` (the `#[test] #[ignore]` in
//! `🧪️tests/🧩️example/🦀️.rs`).

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

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

/// 🎞️ `(asset id, PNG bytes)` for every view, in capture order.
pub const FRAMES: [(&str, &[u8]); 36] = [
    ("synthetic-orbit-frame-0", include_bytes!("🖼️assets/🎞️frame-00.png")),
    ("synthetic-orbit-frame-1", include_bytes!("🖼️assets/🎞️frame-01.png")),
    ("synthetic-orbit-frame-2", include_bytes!("🖼️assets/🎞️frame-02.png")),
    ("synthetic-orbit-frame-3", include_bytes!("🖼️assets/🎞️frame-03.png")),
    ("synthetic-orbit-frame-4", include_bytes!("🖼️assets/🎞️frame-04.png")),
    ("synthetic-orbit-frame-5", include_bytes!("🖼️assets/🎞️frame-05.png")),
    ("synthetic-orbit-frame-6", include_bytes!("🖼️assets/🎞️frame-06.png")),
    ("synthetic-orbit-frame-7", include_bytes!("🖼️assets/🎞️frame-07.png")),
    ("synthetic-orbit-frame-8", include_bytes!("🖼️assets/🎞️frame-08.png")),
    ("synthetic-orbit-frame-9", include_bytes!("🖼️assets/🎞️frame-09.png")),
    ("synthetic-orbit-frame-10", include_bytes!("🖼️assets/🎞️frame-10.png")),
    ("synthetic-orbit-frame-11", include_bytes!("🖼️assets/🎞️frame-11.png")),
    ("synthetic-orbit-frame-12", include_bytes!("🖼️assets/🎞️frame-12.png")),
    ("synthetic-orbit-frame-13", include_bytes!("🖼️assets/🎞️frame-13.png")),
    ("synthetic-orbit-frame-14", include_bytes!("🖼️assets/🎞️frame-14.png")),
    ("synthetic-orbit-frame-15", include_bytes!("🖼️assets/🎞️frame-15.png")),
    ("synthetic-orbit-frame-16", include_bytes!("🖼️assets/🎞️frame-16.png")),
    ("synthetic-orbit-frame-17", include_bytes!("🖼️assets/🎞️frame-17.png")),
    ("synthetic-orbit-frame-18", include_bytes!("🖼️assets/🎞️frame-18.png")),
    ("synthetic-orbit-frame-19", include_bytes!("🖼️assets/🎞️frame-19.png")),
    ("synthetic-orbit-frame-20", include_bytes!("🖼️assets/🎞️frame-20.png")),
    ("synthetic-orbit-frame-21", include_bytes!("🖼️assets/🎞️frame-21.png")),
    ("synthetic-orbit-frame-22", include_bytes!("🖼️assets/🎞️frame-22.png")),
    ("synthetic-orbit-frame-23", include_bytes!("🖼️assets/🎞️frame-23.png")),
    ("synthetic-orbit-frame-24", include_bytes!("🖼️assets/🎞️frame-24.png")),
    ("synthetic-orbit-frame-25", include_bytes!("🖼️assets/🎞️frame-25.png")),
    ("synthetic-orbit-frame-26", include_bytes!("🖼️assets/🎞️frame-26.png")),
    ("synthetic-orbit-frame-27", include_bytes!("🖼️assets/🎞️frame-27.png")),
    ("synthetic-orbit-frame-28", include_bytes!("🖼️assets/🎞️frame-28.png")),
    ("synthetic-orbit-frame-29", include_bytes!("🖼️assets/🎞️frame-29.png")),
    ("synthetic-orbit-frame-30", include_bytes!("🖼️assets/🎞️frame-30.png")),
    ("synthetic-orbit-frame-31", include_bytes!("🖼️assets/🎞️frame-31.png")),
    ("synthetic-orbit-frame-32", include_bytes!("🖼️assets/🎞️frame-32.png")),
    ("synthetic-orbit-frame-33", include_bytes!("🖼️assets/🎞️frame-33.png")),
    ("synthetic-orbit-frame-34", include_bytes!("🖼️assets/🎞️frame-34.png")),
    ("synthetic-orbit-frame-35", include_bytes!("🖼️assets/🎞️frame-35.png")),
];

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
