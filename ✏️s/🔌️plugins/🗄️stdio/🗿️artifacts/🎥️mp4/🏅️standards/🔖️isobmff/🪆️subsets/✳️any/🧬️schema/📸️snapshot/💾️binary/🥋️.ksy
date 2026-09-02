meta:
  id: stdio_mp4_snapshot
  endian: be
doc: |
  Shared `.semio` binary envelope wrapping the schema-first record protocol for the logical
  `stdio.mp4` model. Encoded AVC sample payloads are semantic byte-list fields; ISO-BMFF box
  headers and unsupported box bodies never enter this protocol.
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
    endian: le
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "stdio.mp4.pack v1"
  - id: record_protocol
    size-eos: true
    doc: Shared typed RecordSpec payload produced by store::pack_rt::encode_document.
