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
    2: change_header
    3: replace_palette
    4: change_transparency
    5: change_gamma
    6: change_chromaticities
    7: change_srgb_intent
    8: change_physical_dims
    9: change_timestamp
    10: change_background
    11: insert_text_chunk
    12: remove_text_chunk
    13: replace_text_chunk
    14: replace_pixels
    15: insert_unknown_chunk
    16: remove_unknown_chunk
