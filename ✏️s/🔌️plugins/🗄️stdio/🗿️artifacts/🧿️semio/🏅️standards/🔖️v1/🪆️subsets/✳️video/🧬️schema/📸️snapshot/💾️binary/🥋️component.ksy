meta:
  id: stdio_semio_video_snapshot
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️component.protocol.semio, walked by dsl::walk_protocol) for the post-envelope-unwrap
  SemioVideoSnapshot binary pack payload (crate::…::video::schema::snapshot's `ArtifactPack`
  impl, `encode_video_snapshot_binary`/`decode_video_snapshot_binary` — NOT `serde_json::to_vec`).
  `format`/`schema_len`/`schema_bytes` are real, fully described; `streams` is a homogeneous
  variable-length repeated record (each with a further variable-length `samples` sub-collection
  and an opaque `data` buffer) — the `protocol-array-of-records` gap — one opaque trailing
  `payload` covers it honestly, same boundary the real `.protocol.semio` file uses.
seq:
  - id: format
    type: u1
    doc: "PACK_BINARY_FORMAT, currently 1"
  - id: schema_len
    type: vlq_base128_le
  - id: schema_bytes
    size: schema_len.value
    doc: "UTF-8 schema id, e.g. stdio.semio.video"
  - id: payload
    size-eos: true
    doc: |
      Real varint stream count, then per stream: kind_tag u1 (0=Video,1=Audio,2=Subtitle),
      varint-length-prefixed codec UTF-8, width u4le, height u4le, rate.num/den (8-byte LE i64
      each), varint sample count, then per sample: pts u8le, key u1 (0/1), varint-length-prefixed
      opaque data bytes. Not sub-typed further here (protocol-array-of-records gap) — the real
      Rust codec (../🦀️component.rs) stays fully structured and is round-trip tested
      independently.
