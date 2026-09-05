meta:
  id: stdio_semio_table_snapshot
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️.protocol.semio, walked by dsl::walk_protocol) for the shared `.semio` binary
  envelope (store::semio_format::wrap_binary) wrapping the REAL varint-length-prefixed
  SemioTableSnapshot binary pack (crate::…::table::schema::snapshot's `ArtifactPack` impl,
  `encode_table_snapshot_binary`/`decode_table_snapshot_binary`). Past the envelope, `format`/
  `schema_len`/`schema_bytes` are real, fully described; `columns`/`rows` are homogeneous
  variable-length repeated data (the `protocol-array-of-records` gap) — one opaque trailing
  `payload` covers it honestly, same boundary the real `.protocol.semio` file uses.
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "s.stdio.semio.table.pack v1"
  - id: format
    type: u1
    doc: "PACK_BINARY_FORMAT, currently 1"
  - id: schema_len
    type: vlq_base128_le
  - id: schema_bytes
    size: schema_len.value
    doc: "UTF-8 schema id, e.g. s.stdio.semio.table"
  - id: payload
    size-eos: true
    doc: |
      Real varint-prefixed `columns` (varint count + per-column: name strlp, kind tag u1) then
      `rows` (varint count + per-row: varint cell count + per-cell recursive SemioValue binary
      tag+payload). Not sub-typed further here — the `protocol-array-of-records` gap — the real
      Rust codec (../../🦀️.rs) stays fully structured and is round-trip tested
      independently.
