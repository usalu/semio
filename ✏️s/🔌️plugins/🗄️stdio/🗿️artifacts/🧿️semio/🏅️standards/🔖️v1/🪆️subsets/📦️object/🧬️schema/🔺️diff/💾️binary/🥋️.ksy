meta:
  id: stdio_semio_object_diff
  endian: le
doc: Kaitai mirror (descriptive) for the s.stdio.semio.object.diff binary frame.
seq:
  - id: format
    type: u1
  - id: presence
    type: u1
  - id: payload
    size-eos: true
