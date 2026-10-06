meta:
  id: stdio_avi_snapshot
  endian: le
doc: Complete owned AVI snapshot record protocol, independent of RIFF serialization.
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: stdio.avi.pack v1
  - id: record_protocol
    size-eos: true
    doc: Repository typed RecordSpec document preserving all AviSnapshot fields.
