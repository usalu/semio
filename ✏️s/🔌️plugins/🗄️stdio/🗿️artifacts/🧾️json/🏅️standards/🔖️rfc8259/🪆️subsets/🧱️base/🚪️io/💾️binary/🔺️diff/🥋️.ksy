meta:
  id: stdio_json_diff
  endian: le
doc: |
  `pack` (binary) form of a `stdio.json` diff: envelope (see ../📸️snapshot/💾️binary/🥋️.ksy
  for the shared framing) wrapping a `payload` of UTF-8 JSON text -- the tagged `JsonDiff` struct
  serialized compact, same value grammar as the snapshot facet's own payload.
seq:
  - id: payload
    size-eos: true
    doc: UTF-8 JSON text of the tagged JsonDiff struct.
