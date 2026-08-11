meta:
  id: semio_image_mutation_binary
doc: |
  Real binary layout for `SemioImageMutation::encode_op`/`decode_op` — a documented
  simplification (matching gif's own hand-rolled op codec): `encode_op` is the 📝️text
  sibling's `op` grammar UTF-8 bytes, verbatim, with no separate binary framing.
seq:
  - id: op_utf8
    type: str
    size: _io.size - _io.pos
    encoding: UTF-8
