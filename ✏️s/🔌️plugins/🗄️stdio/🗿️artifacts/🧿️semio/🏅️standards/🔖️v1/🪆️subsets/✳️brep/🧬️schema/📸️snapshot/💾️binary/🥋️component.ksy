meta:
  id: stdio_semio_brep_snapshot
  endian: le
doc: |
  `pack` (binary) form of a `stdio.semio.brep` snapshot, past the `semio_format` envelope: a
  real fixed `format` byte, a real varint-length-prefixed `schema` string, then one opaque `payload`
  tail (the `vertices`/`edges`/`loops`/`faces`/`shells`/`solids` collections — a homogeneous-but-
  variable-length repeated-record shape the protocol dialect's `repeat` block can't describe
  untagged, see the sibling `📡️component.protocol.semio`'s own comment; `edges`/`faces` also embed
  a data-carrying tagged `curve`/`surface` union via a real per-variant tag byte). Not a JSON blob —
  see `📸️snapshot/🦀️component.rs`'s `encode_brep_snapshot_binary` for the payload's real internal
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
