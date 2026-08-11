meta:
  id: stdio_semio_model_diff
  endian: le
doc: |
  Binary form of a `stdio.semio.model` diff: NO envelope framing (unlike the snapshot facet) --
  `encode_diff` is exactly the UTF-8 bytes of the hand-rolled `print_diff()` one-line text (see
  `../📝️text/📖️component.grammar.semio`'s `line` production for that grammar). Genuinely the
  whole remaining stream, not a lazy scaffold catch-all: there is no trailing structure after it.
seq:
  - id: line
    size-eos:  true
    type: str
    encoding: UTF-8
