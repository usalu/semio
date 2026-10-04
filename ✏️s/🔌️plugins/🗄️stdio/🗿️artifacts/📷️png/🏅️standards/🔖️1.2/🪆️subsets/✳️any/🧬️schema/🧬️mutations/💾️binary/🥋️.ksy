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
    17: set_snapshot
    18: patch_pixels
    19: patch_snapshot
    20: paint_native_samples
