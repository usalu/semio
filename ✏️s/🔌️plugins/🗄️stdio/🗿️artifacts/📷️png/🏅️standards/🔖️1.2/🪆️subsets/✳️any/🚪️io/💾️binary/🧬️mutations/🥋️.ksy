meta:
  id: stdio_png_mutation
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
    5: change_gamma
    12: replace_image
    18: patch_pixels
    20: paint_native_samples
