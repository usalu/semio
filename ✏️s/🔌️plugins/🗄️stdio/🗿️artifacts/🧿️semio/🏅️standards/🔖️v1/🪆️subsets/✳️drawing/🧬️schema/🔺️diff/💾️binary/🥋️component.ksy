meta:
  id: semio_drawing_diff
  endian: le
doc: |
  `SemioDrawingDiff`'s hand-rolled `protocol::DiffCodec::encode_diff` -- no separate binary
  envelope, the bytes ARE the UTF-8 text-facet grammar (../📝️text/📖️component.grammar.semio)
  verbatim, matching json's own `stdio.json.diff` binary leaf precedent.
seq:
  - id: text
    size-eos: true
    encoding: UTF-8
    doc: space-separated field=value tokens, see ../📝️text/📖️component.grammar.semio
