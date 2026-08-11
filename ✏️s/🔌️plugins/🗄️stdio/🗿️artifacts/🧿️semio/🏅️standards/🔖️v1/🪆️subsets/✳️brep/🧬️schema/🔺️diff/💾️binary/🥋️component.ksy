meta:
  id: stdio_semio_brep_diff
  endian: le
doc: |
  Binary form of a `stdio.semio.brep` diff: the text grammar's bytes verbatim (protocol::DiffCodec
  `encode_diff` is `print_diff().into_bytes()` -- no second wire format, same simplification
  gif/svg/bcf all use). See ../💾️binary/🔠️component.abnf for the real per-collection grammar this
  payload's bytes conform to.
seq:
  - id: payload
    size-eos: true
    doc: UTF-8 diff-line text, see the sibling 🔠️component.abnf for its structure.
