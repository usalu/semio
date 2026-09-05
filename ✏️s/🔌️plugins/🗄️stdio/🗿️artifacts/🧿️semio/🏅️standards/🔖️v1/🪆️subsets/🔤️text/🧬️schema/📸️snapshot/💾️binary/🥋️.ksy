meta:
  id: stdio_semio_text_snapshot
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️.protocol.semio, walked by dsl::walk_protocol) for the shared `.semio` binary
  envelope (store::semio_format::wrap_binary) wrapping the REAL varint-length-prefixed
  SemioTextSnapshot binary pack (crate::…::text::schema::snapshot's `ArtifactPack` impl,
  `encode_text_snapshot_binary`/`decode_text_snapshot_binary`). Past the envelope, `format`/
  `schema_len`/`schema_bytes` are real, fully described; `runs` is homogeneous variable-length
  repeated data (the `protocol-array-of-records` gap) — one opaque trailing `payload` covers it
  honestly, same boundary the real `.protocol.semio` file uses.
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "s.stdio.semio.text.pack v1"
  - id: format
    type: u1
    doc: "PACK_BINARY_FORMAT, currently 1"
  - id: schema_len
    type: vlq_base128_le
  - id: schema_bytes
    size: schema_len.value
    doc: "UTF-8 schema id, e.g. s.stdio.semio.text"
  - id: payload
    size-eos: true
    doc: |
      Real varint-prefixed `runs` (varint count + per-run: language strlp, content strlp, marks
      (varint count + per-mark kind u1 + href strlp)). Not sub-typed further here — the
      `protocol-array-of-records` gap — the real Rust codec (../../🦀️.rs) stays fully
      structured and is round-trip tested independently.
