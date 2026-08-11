meta:
  id: stdio_semio_object_diff
  endian: le
doc: |
  `DiffCodec::encode_diff`/`decode_diff` (`🦀️component.rs`): the text `print_diff` bytes verbatim
  (`self.print_diff().into_bytes()`) -- NOT a distinct binary encoding, same simplification
  `SvgDiff`/`GifDiff`/`JsonDiff` all use. See the sibling `../📝️text/📖️component.grammar.semio` for
  the payload's real structure.
seq:
  - id: payload
    size-eos: true
    doc: UTF-8 `root=<enc> objects=<enc>` line, see the text-facet grammar.
