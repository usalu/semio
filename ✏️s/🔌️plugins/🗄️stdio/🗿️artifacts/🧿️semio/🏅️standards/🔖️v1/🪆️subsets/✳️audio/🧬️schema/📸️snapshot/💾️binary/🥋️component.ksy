meta:
  id: stdio_semio_audio_snapshot
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️component.protocol.semio, walked by dsl::walk_protocol) for the shared `.semio` binary
  envelope (store::semio_format::wrap_binary) wrapping the REAL varint-length-prefixed
  SemioAudioSnapshot binary pack (crate::…::audio::schema::snapshot's `ArtifactPack` impl,
  `encode_audio_snapshot_binary`/`decode_audio_snapshot_binary` — NOT `serde_json::to_vec`). Past
  the envelope, `format`/`schema_len`/`schema_bytes`/`sample_rate`/`audio_format` are real, fully
  described; `channels`/`tags` are homogeneous variable-length repeated data (the
  `protocol-array-of-records` gap) — one opaque trailing `payload` covers them honestly, same
  boundary the real `.protocol.semio` file uses.
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "stdio.semio.audio.pack v1"
  - id: format
    type: u1
    doc: "PACK_BINARY_FORMAT, currently 1"
  - id: schema_len
    type: vlq_base128_le
  - id: schema_bytes
    size: schema_len.value
    doc: "UTF-8 schema id, e.g. stdio.semio.audio"
  - id: sample_rate
    type: u4
  - id: audio_format
    type: u1
    doc: "0=Pcm8 1=Pcm16 2=Pcm24 3=Pcm32 4=Float32 5=Float64"
  - id: payload
    size-eos: true
    doc: |
      Real varint-prefixed `channels` (varint count + per-channel varint sample count + real
      4-byte LE f32 samples) and `tags` (varint count + per-entry varint-length-prefixed key/value
      UTF-8). Not sub-typed further here — the `protocol-array-of-records` gap (repeat's arms are
      tag-dispatched, not "N times from a count field" for an untagged homogeneous record) — the
      real Rust codec (../../🦀️component.rs) stays fully structured and is round-trip tested
      independently.
