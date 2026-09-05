meta:
  id: stdio_avi_mutations
  endian: be
doc: |
  protocol::OpBinary (../🦀️.rs): one JSON-serialized AviMutation per record.
seq:
  - id: json_utf8
    type: str
    size-eos: true
    encoding: UTF-8
