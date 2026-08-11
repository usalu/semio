meta:
  id: stdio_semio_object_mutations
  endian: le
doc: |
  `OpBinary::encode_op`/`decode_op` (`🦀️component.rs`): the text `print_op` bytes verbatim
  (`self.print_op().into_bytes()`) -- NOT a distinct binary encoding, same simplification the
  sibling `🔺️diff` facet's `DiffCodec` uses. See `../📝️text/📖️component.grammar.semio` for the
  payload's real structure.
seq:
  - id: payload
    size-eos: true
    doc: UTF-8 `keyword arg=value ...` line, see the text-facet grammar.
