meta:
  id: semio_cad_mutations
doc: |
  protocol::OpText `encode_op`/`decode_op` payload for `s.stdio.semio.cad.mutations` -- NO semio
  envelope: `encode_op` is `print_op().into_bytes()` verbatim, ASCII text in the
  `../📝️text/📖️component.grammar.semio` `keyword arg=value ...` grammar. Real, all-remaining-
  bytes ASCII text by design (no length prefix, no fixed-width binary sub-fields).
seq:
  - id: payload
    size: _io.size - _io.pos
    type: str
    encoding: ASCII
    doc: "print_op() output -- see ../📝️text/📖️component.grammar.semio for the real grammar"
