meta:
  id: stdio_bmp_diff
  endian: le
doc: |
  The generic serde-derived binary encoding of BmpDiff (raw UTF-8 JSON bytes) — no `.semio`
  envelope header (contrast with ../../📸️snapshot/💾️binary/, whose payload IS wrapped).
seq:
  - id: payload
    type: str
    size-eos: true
    encoding: UTF-8
    doc: UTF-8 JSON object (see sibling ../📝️text/📖️.grammar.semio).
