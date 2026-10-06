meta:
  id: stdio_pdf_1_7_diff
  endian: le
doc: |
  Derive-owned PdfDiff frame: the shared op format byte followed by the container-less pack
  record body of the sparse typed diff value (every lane an optional typed field).
seq:
  - id: format
    type: u1
    valid: 1
  - id: diff
    size-eos: true
