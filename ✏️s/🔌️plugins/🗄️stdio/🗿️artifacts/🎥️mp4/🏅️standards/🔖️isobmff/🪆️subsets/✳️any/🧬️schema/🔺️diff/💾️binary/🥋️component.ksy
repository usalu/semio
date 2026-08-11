meta:
  id: stdio_mp4_diff
  endian: be
doc: |
  Op-level binary framing for Mp4Mutation (protocol::OpBinary::encode_op/decode_op in
  🧬️mutations/🦀️component.rs) — one JSON-serialized mutation per encoded op, length-independent
  (the caller's op-log framing supplies length); this leaf documents the payload shape only.
seq:
  - id: json_utf8
    type: str
    size-eos: true
    encoding: UTF-8
    doc: One compact JSON-serialized Mp4Mutation (tagged enum, camelCase fields).
