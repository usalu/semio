//! 🚪️ IO `s.stdio.semio` (v1/video) — real cross-format bridge leaves (W4): typed
//! `ArtifactDeserializer`/`ArtifactSerializer` impls, one pair per bridged format
//! (`video↔mp4`, `video↔avi` per the master plan's io lattice). Each leaf module is mounted here
//! (not in `📦️glue.rs`, a closer-only hot file) via `#[path=...]`, resolved relative to this
//! file's own directory — the same mechanism `📦️glue.rs` itself uses one level up. Registration
//! flows through `🎹️composer::register` (see that module), matching the repo-wide convention.

#[path = "📥️import/🧩️deserializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs"]
pub mod mp4_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs"]
pub mod mp4_serializer;
#[path = "📥️import/🧩️deserializers/🗿️artifacts/📼️avi/🔖️1.0/✳️any/🦀️component.rs"]
pub mod avi_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/📼️avi/🔖️1.0/✳️any/🦀️component.rs"]
pub mod avi_serializer;
