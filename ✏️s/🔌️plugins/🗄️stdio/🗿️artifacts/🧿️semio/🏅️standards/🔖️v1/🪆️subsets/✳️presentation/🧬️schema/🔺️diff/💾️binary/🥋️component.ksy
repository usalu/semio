meta:
  id: semio_presentation_diff
doc: |
  Binary = the UTF-8 bytes of the diff facet's own text grammar (space-separated
  `masters=[...]  layouts=[...] slides=[...]` tokens) verbatim -- the same simplification the
  hand-rolled `protocol::DiffCodec::encode_diff` impl uses (`self.print_diff().into_bytes()`).
seq:
  - id: line_utf8
    type: str
    size-eos: true
    encoding: UTF-8
