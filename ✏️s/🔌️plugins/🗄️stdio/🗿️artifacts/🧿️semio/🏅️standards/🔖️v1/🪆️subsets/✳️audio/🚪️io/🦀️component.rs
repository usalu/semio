//! 🚪️ IO `s.stdio.semio` (v1/audio) — real cross-format bridge leaves (W4): typed
//! `ArtifactDeserializer`/`ArtifactSerializer` impls, one pair per bridged format
//! (`audio↔mp3`, `audio↔wav` per the master plan's io lattice). Mounted here (not in
//! `📦️glue.rs`, a closer-only hot file) via `#[path=...]` relative to this file's own directory.
//! Registration flows through `🎹️composer::register`.

#[path = "📥️import/🧩️deserializers/🗿️artifacts/🎵️mp3/🔖️mpeg1-layer3/✳️any/🦀️component.rs"]
pub mod mp3_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/🎵️mp3/🔖️mpeg1-layer3/✳️any/🦀️component.rs"]
pub mod mp3_serializer;
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🔊️wav/🔖️riff-pcm/✳️any/🦀️component.rs"]
pub mod wav_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/🔊️wav/🔖️riff-pcm/✳️any/🦀️component.rs"]
pub mod wav_serializer;
