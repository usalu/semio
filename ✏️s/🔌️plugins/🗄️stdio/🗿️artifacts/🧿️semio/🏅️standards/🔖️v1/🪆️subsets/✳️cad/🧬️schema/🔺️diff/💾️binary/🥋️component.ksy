meta:
  id: semio_cad_diff
doc: |
  protocol::DiffCodec `encode_diff`/`decode_diff` payload for `s.stdio.semio.cad.diff` -- NO
  semio envelope (unlike the snapshot facet): `encode_diff` is `print_diff().into_bytes()`
  verbatim, ASCII text in the `../📝️text/📖️component.grammar.semio` bracket-triple grammar
  (space-separated `layers=`/`blocks=`/`entities=` tokens, hex-encoded strings, single-letter
  tagged `CadEntity` variants). Kaitai's byte-oriented model isn't a fit for hand-rolling that
  recursive text grammar a second time here; the whole field is real, all-remaining-bytes ASCII
  text by design (no length prefix, no fixed-width binary sub-fields to model).
seq:
  - id: payload
    size: _io.size - _io.pos
    type: str
    encoding: ASCII
    doc: "print_diff() output -- see ../📝️text/📖️component.grammar.semio for the real grammar"
