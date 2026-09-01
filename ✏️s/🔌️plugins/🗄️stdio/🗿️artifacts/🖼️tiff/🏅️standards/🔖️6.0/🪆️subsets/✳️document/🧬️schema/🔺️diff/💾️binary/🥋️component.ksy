meta:
  id: stdio_tiff_diff
  endian: le
doc: |
  protocol::OpBinary raw JSON encoding of TiffDiff — no `.semio` envelope header (contrast
  with ../../📸️snapshot/💾️binary/, whose payload IS wrapped).
seq:
  - id: json_bytes
    type: u1
    repeat: eos
    doc: UTF-8 JSON object bytes matching TiffDiff's serde shape, read through end-of-stream
      (no length prefix — the whole op payload IS the JSON document; see sibling
      ../📝️text/📖️component.grammar.semio for the field-level shape).
