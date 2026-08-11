meta:
  id: semio_presentation_mutations
doc: |
  Binary = the UTF-8 bytes of the mutations facet's own text grammar (`keyword arg=value ...`)
  verbatim, same simplification `protocol::OpBinary::encode_op` uses (`self.print_op().into_bytes()`).
seq:
  - id: line_utf8
    type: str
    size-eos: true
    encoding: UTF-8
