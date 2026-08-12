meta:
  id: stdio_semio_snapshot
  endian: le
doc: |
  `pack` (binary) form of a `stdio.semio` envelope snapshot, past the `semio_format` envelope: a
  real fixed `format` byte, a real `tag` byte (subset ordinal, `📸️snapshot/🦀️component.rs`'s
  `subset_ordinal`), a real varint-length-prefixed `schema` string, then one opaque `payload`
  tail — the WRAPPED subset's own full, already-real `ArtifactPack::encode_pack()` bytes (a real
  nested `semio_format` envelope of its own — see that subset's own `.ksy` for its internal
  layout, not re-described here).
seq:
  - id: format
    type: u1
  - id: tag
    type: u1
  - id: schema_len
    type: vlq_base128_le
  - id: schema_bytes
    size: schema_len.value
    type: str
    encoding: UTF-8
  - id: payload
    size-eos: true
