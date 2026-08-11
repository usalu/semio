meta:
  id: semio_video_diff
  endian: le
doc: |
  stdio.semio.video diff binary layout: UTF-8 bytes of the text grammar (../📝️text) verbatim —
  protocol::DiffCodec::encode_diff == print_diff().into_bytes(), no separate binary framing
  (matching GifDiff/SvgDiff/DocxDiff's own hand-rolled codecs).
seq:
  - id: text_utf8
    type: str
    encoding: UTF-8
    consume: false
    terminator: -1
    doc: the streams= triple grammar, see ../📝️text/📖️component.grammar.semio
