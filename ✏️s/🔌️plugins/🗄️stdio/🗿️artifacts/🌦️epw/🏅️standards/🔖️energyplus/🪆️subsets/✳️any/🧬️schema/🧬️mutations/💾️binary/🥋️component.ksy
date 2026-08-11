meta:
  id: stdio_epw_mutations
  endian: le
doc: |
  Raw UTF-8 bytes of the hand-rolled `stdio.epw.mutations` text (see ../📝️text/📖️component.grammar.semio).
  Not wrapped in the shared `.semio` binary envelope.
seq:
  - id: payload
    type: str
    size-eos: true
    encoding: UTF-8
