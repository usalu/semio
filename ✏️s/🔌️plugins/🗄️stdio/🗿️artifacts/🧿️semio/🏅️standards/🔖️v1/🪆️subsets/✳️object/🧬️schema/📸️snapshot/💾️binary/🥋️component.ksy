meta:
  id: stdio_semio_object_snapshot
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️component.protocol.semio) for the shared `.semio` binary envelope wrapping the REAL
  varint-length-prefixed SemioObjectSnapshot binary pack (encode_object_snapshot_binary /
  decode_object_snapshot_binary in ../../🦀️component.rs).
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "s.stdio.semio.object.pack v1"
  - id: format
    type: u1
  - id: schema_len
    type: vlq_base128_le
  - id: schema_bytes
    size: schema_len.value
  - id: payload
    size-eos: true
    doc: "transform (10 f64 LE) then 3 optional child handles (presence u8 + 2 length-prefixed strings each)"
