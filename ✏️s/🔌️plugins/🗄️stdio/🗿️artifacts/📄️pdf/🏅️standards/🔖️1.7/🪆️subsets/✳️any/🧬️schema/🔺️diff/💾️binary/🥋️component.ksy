meta:
  id: stdio_pdf_1_7_diff
  endian: le
doc: |
  `PdfDiff` (1.7) has no dedicated OpText/binary envelope yet (F6 wave, not this one) -- on the
  wire it is plain UTF-8 JSON (RFC8259), tagged per the ../📝️text/📖️component.grammar.semio
  productions. `size-eos` here names that (the JSON text has no fixed-length framing of its own,
  by design), not an unstructured placeholder.
seq:
  - id: utf8_json_text
    size-eos: true
