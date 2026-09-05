meta:
  id: stdio_semio_kit_snapshot
  endian: le
doc: Kaitai mirror (descriptive) for the s.stdio.semio.kit snapshot binary pack.
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
  - id: format
    type: u1
  - id: schema_len
    type: vlq_base128_le
  - id: schema_bytes
    size: schema_len.value
  - id: payload
    size-eos: true
