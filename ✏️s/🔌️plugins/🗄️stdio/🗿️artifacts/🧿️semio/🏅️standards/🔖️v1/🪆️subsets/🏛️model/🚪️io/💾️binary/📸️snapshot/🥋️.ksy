meta:
  id: stdio_semio_model_snapshot
  endian: le
doc: |
  `pack` (binary) form of a `stdio.semio.model` snapshot, past the `semio_format` envelope: a real
  fixed `format` byte, a real varint-length-prefixed `schema` string, then one opaque `payload` tail
  (the `spatial`/`elements`/`relations` collections — a homogeneous-but-variable-length
  repeated-record shape the protocol dialect's `repeat` block can't describe untagged, see the
  sibling `📡️.protocol.semio`'s own comment). Not a JSON blob — see
  `📸️snapshot/🦀️.rs`'s `encode_model_snapshot_binary` for the payload's real internal
  varint/length-prefixed/u8-tagged layout.
seq:
  - id: format
    type: u1
  - id: schema_len
    type: vlq_base128_le
  - id: schema_bytes
    size: schema_len.value
    type: str
    encoding: UTF-8
  - id: payload
    size-eos: true
