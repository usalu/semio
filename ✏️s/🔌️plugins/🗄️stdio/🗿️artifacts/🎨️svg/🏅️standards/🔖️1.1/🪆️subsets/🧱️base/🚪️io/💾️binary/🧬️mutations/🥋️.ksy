meta:
  id: stdio_svg_mutations
  endian: le
doc: |
  Binary SvgMutation with a format byte, variant tag, and recursively structured payload.
seq:
  - id: format
    type: u1
  - id: tag
    type: u1
  - id: payload
    size-eos: true
