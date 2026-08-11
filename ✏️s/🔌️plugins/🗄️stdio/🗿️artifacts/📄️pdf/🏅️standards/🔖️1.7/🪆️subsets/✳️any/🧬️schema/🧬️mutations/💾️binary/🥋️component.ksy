meta:
  id: stdio_pdf_1_7_mutations
  endian: le
doc: |
  `OpBinary::encode_op` is `serde_json::to_vec(self)` -- the UTF-8 bytes of the SAME JSON text
  ../📝️text/📖️component.grammar.semio describes (tagged by "mutation"). `size-eos` names that
  real encoding (no fixed-length framing exists), not a placeholder.
seq:
  - id: utf8_json_text
    size-eos: true
