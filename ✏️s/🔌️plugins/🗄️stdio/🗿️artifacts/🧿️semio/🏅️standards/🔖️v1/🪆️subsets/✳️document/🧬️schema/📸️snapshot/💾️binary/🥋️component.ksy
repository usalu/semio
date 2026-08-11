meta:
  id: semio_document_snapshot
  endian: be
doc: |
  Pack binary form: store::semio_format envelope (magic "SEMI" + component tag + version + u32
  length) followed by exactly `body_len` bytes of UTF-8 JSON (the snapshot re-encoded via serde).
seq:
  - id: magic
    contents: "SEMI"
  - id: component_tag
    type: u1
  - id: envelope_version
    type: u1
  - id: body_len
    type: u4
  - id: json_body
    size: body_len
