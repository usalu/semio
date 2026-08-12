meta:
  id: stdio_semio_drawing_snapshot
  endian: le
doc: |
  `pack` (binary) form of a `stdio.semio.drawing` snapshot, past the `semio_format` envelope: a
  real fixed `format` byte, a real varint-length-prefixed `schema` string, then one opaque
  `payload` tail (the `canvas`/`styles`/`layers` collections — `layers` embeds a further
  RECURSIVE `DrawNode` tree, a shape the protocol dialect's `repeat` block can't describe untagged,
  see the sibling `📡️component.protocol.semio`'s own comment). Not a JSON blob — see
  `📸️snapshot/🦀️component.rs`'s `encode_drawing_snapshot_binary` for the payload's real internal
  varint/length-prefixed layout.
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
