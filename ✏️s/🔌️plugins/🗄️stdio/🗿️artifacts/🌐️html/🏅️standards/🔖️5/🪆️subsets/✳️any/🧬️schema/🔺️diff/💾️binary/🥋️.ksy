meta:
  id: stdio_html_diff
  endian: le
doc: |
  UTF-8 bytes of `print_diff`'s hand-rolled bracket-token output, verbatim (see the sibling
  ../📝️text/📖️.grammar.semio) -- `encode_diff`/`decode_diff` are a pure text<->bytes
  passthrough, no distinct binary framing.
seq:
  - id: diff_text
    size-eos: true
