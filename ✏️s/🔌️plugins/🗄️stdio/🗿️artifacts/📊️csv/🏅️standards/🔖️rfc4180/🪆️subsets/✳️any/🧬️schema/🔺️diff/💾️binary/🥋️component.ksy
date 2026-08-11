meta:
  id: stdio_csv_diff
  endian: le
doc: |
  protocol::OpBinary raw JSON encoding of CsvDiff — no `.semio` envelope header (contrast
  with ../../📸️snapshot/💾️binary/, whose payload IS wrapped).
seq:
  - id: payload
    type: str
    size-eos: true
    encoding: UTF-8
    doc: UTF-8 JSON object (see sibling ../📝️text/📖️component.grammar.semio).
