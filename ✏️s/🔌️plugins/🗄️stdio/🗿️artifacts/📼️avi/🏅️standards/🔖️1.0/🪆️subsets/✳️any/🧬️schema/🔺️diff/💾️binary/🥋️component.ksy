meta:
  id: stdio_avi_diff
  endian: be
doc: |
  Op-level binary framing for AviMutation (protocol::OpBinary in 🧬️mutations/🦀️component.rs) —
  one JSON-serialized mutation per encoded op.
seq:
  - id: json_utf8
    type: str
    size-eos: true
    encoding: UTF-8
