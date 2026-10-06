meta:
  id: stdio_html_mutations
  endian: le
doc: |
  UTF-8 bytes of `print_op`'s hand-rolled `keyword arg=value ...` output, verbatim (see the
  sibling ../📝️text/📖️.grammar.semio) -- `encode_op`/`decode_op` are a pure text<->bytes
  passthrough, no distinct binary framing.
seq:
  - id: op_text
    size-eos: true
