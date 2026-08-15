meta:
  id: stdio_svg_diff
  endian: le
doc: |
  Binary SvgDiff with a format byte, presence flags, and recursively structured payload.
seq:
  - id: format
    type: u1
  - id: flags
    type: u1
  - id: payload
    size-eos: true
