meta:
  id: stdio_pdf_1_7_mutations
  endian: le
doc: Structured logical mutation frame. Variant payloads use varints, length-prefixed semantic bytes, recursive COS object tags, and typed stream-filter tags.
seq:
  - id: format
    type: u1
  - id: mutation_tag
    type: u1
  - id: structural_payload
    size-eos: true
