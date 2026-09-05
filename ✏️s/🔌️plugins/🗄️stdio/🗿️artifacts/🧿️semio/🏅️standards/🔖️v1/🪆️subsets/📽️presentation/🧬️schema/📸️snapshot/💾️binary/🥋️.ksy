meta:
  id: stdio_semio_presentation_snapshot
  endian: le
doc: |
  `pack` (binary) form of a `stdio.semio.presentation` snapshot, past the `semio_format` envelope: a
  real fixed `format` byte, a real varint-length-prefixed `schema` string, then one opaque `payload`
  tail (the `masters`/`layouts`/`slides` collections — a homogeneous-but-variable-length
  repeated-record shape the protocol dialect's `repeat` block can't describe untagged, see the
  sibling `📡️.protocol.semio`'s own comment; `slides` also embeds a data-carrying tagged
  `SlideShape` union whose `TextBox`/`Table` variants further embed document's own recursive
  `DocBlock` union, real-tag-byte-encoded, with every `DocBlock` leaf reusing document's real
  `enc_block`/`dec_block` TEXT codec embedded as a length-prefixed UTF-8 blob). Not a JSON blob —
  see `📸️snapshot/🦀️.rs`'s `encode_presentation_snapshot_binary` for the payload's real
  internal varint/length-prefixed layout.
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
