meta:
  id: stdio_bmp_mutation
  endian: le
seq:
  - id: format
    type: u1
    valid: 1
  - id: tag
    type: u1
    enum: mutation_kind
  - id: payload
    size-eos: true
enums:
  mutation_kind:
    9: paint_indexed_region
    10: paint_direct_region
    12: replace_image
    13: replace_samples
