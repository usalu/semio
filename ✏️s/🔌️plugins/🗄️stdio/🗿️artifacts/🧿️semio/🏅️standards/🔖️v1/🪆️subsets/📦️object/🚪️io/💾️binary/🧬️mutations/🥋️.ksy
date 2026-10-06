meta:
  id: stdio_semio_object_mutations
  endian: le
doc: Kaitai mirror (descriptive) for the s.stdio.semio.object.mutations binary op frame.
seq:
  - id: format
    type: u1
  - id: tag
    type: u1
  - id: payload
    size-eos: true
