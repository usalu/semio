meta:
  id: stdio_semio_document_snapshot
  endian: le
doc: |
  `pack` (binary) form of a `stdio.semio.document` snapshot, past the `semio_format` envelope: a
  real fixed `format` byte, a real varint-length-prefixed `schema` string, then one opaque `payload`
  tail (the `styles`/`images`/`blocks` collections — a homogeneous-but-variable-length
  repeated-record shape the protocol dialect's `repeat` block can't describe untagged, see the
  sibling `📡️.protocol.semio`'s own comment; `blocks` also embeds a data-carrying tagged
  `DocBlock` union with recursive `List`/`Table`/`Quote` nesting via a real per-variant tag byte).
  Not a JSON blob — see `📸️snapshot/🦀️.rs`'s `encode_document_snapshot_binary` for the
  payload's real internal varint/length-prefixed layout.
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
