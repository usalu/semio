//! 🚪️ IO `s.stdio.semio` (v1/animation) — real cross-format bridge leaves (W4): typed
//! `ArtifactDeserializer`/`ArtifactSerializer` impls, one pair per bridged format
//! (`animation↔gltf`, `animation↔mp4`, `animation↔gif` per the master plan's io lattice). Mounted
//! here (not in `📦️glue.rs`, a closer-only hot file) via `#[path=...]` relative to this file's own
//! directory. Registration flows through `🎹️composer::register`.

#[path = "📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs"]
pub mod gltf_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs"]
pub mod gltf_serializer;
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs"]
pub mod mp4_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs"]
pub mod mp4_serializer;
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🎞️gif/🔖️89a/✳️any/🦀️component.rs"]
pub mod gif_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/🎞️gif/🔖️89a/✳️any/🦀️component.rs"]
pub mod gif_serializer;
