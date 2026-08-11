meta:
  id: stdio_bmp_mutations
  endian: le
doc: |
  protocol::OpBinary raw JSON encoding of BmpMutation — no `.semio` envelope header.
seq:
  - id: payload
    type: str
    size-eos: true
    encoding: UTF-8
    doc: UTF-8 JSON object (see sibling ../📝️text/📖️component.grammar.semio).
