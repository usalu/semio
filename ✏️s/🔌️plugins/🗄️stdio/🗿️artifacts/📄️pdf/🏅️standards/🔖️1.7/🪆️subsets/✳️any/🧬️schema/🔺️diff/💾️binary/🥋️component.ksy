meta:
  id: stdio_pdf_1_7_diff
  endian: le
doc: |
  Structured logical PdfDiff frame. Format is 1. Flags bits 0-4 select declared-version,
  PdfInfo, pages diff, objects diff, and trailer diff in fixed order. The structural field
  sequence uses LEB128 counts, length-prefixed semantic data, recursive COS/value-diff tags,
  and typed stream filter/predictor records; it is neither text nor native PDF.
seq:
  - id: format
    type: u1
    valid: 1
  - id: flags
    type: u1
    valid:
      max: 31
  - id: structural_fields
    size-eos: true
