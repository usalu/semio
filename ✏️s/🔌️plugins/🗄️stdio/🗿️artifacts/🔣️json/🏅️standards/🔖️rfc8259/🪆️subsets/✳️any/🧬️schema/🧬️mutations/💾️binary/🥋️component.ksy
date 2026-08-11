meta:
  id: stdio_json_mutations
  endian: le
doc: |
  `encode_op`/`decode_op` (binary) form of a `stdio.json` mutation: UTF-8 JSON text of the tagged
  `JsonMutation` struct (compact form) -- see ../../📸️snapshot/💾️binary/🥋️component.ksy for the
  same payload-is-JSON-text convention.
seq:
  - id: payload
    size-eos: true
    doc: UTF-8 JSON text of the tagged JsonMutation struct.
