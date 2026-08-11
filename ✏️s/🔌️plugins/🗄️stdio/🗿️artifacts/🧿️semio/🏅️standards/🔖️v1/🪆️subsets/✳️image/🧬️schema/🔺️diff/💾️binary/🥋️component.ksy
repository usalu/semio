meta:
  id: semio_image_diff_binary
doc: |
  Real binary layout for `SemioImageDiff::encode_diff`/`decode_diff` — a documented
  simplification (matching gif's own hand-rolled `DiffCodec`): `encode_diff` is the
  📝️text sibling's `line` grammar UTF-8 bytes, verbatim, with no separate binary framing.
seq:
  - id: line_utf8
    type: str
    size: _io.size - _io.pos
    encoding: UTF-8
