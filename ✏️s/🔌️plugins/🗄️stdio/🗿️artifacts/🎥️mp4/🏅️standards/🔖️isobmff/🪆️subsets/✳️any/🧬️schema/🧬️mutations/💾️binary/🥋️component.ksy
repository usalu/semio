meta:
  id: stdio_mp4_mutations
  endian: be
doc: |
  protocol::OpBinary::encode_op/decode_op (../🦀️component.rs): one JSON-serialized Mp4Mutation
  per record (length supplied by the caller's op-log framing).
seq:
  - id: json_utf8
    type: str
    size-eos: true
    encoding: UTF-8
