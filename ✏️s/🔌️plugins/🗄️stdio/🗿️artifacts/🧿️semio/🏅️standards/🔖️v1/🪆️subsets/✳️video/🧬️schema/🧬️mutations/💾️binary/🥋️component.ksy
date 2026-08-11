meta:
  id: semio_video_mutations
  endian: le
doc: |
  stdio.semio.video mutations binary layout: UTF-8 bytes of the text op grammar (../📝️text)
  verbatim — protocol::OpBinary::encode_op == print_op().into_bytes(), no separate binary framing
  (matching DocxMutation's own hand-rolled codec).
seq:
  - id: text_utf8
    type: str
    encoding: UTF-8
    consume: false
    terminator: -1
    doc: the keyword op grammar, see ../📝️text/📖️component.grammar.semio
