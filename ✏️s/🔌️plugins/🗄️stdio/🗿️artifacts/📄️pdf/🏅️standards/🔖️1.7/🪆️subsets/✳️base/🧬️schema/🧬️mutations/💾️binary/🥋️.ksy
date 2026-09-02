meta:
  id: stdio_pdf_1_7_mutations
  endian: le
doc: Exact mutation frame with direct-owner tag 0 through 15 and canonical aggregate JSON payload.
seq:
  - id: format
    type: u1
  - id: mutation_tag
    type: u1
  - id: structural_payload
    size-eos: true
