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
    2: change_header_fields
    3: insert_palette_entry
    4: remove_palette_entry
    5: replace_palette_entry
    6: replace_pixel_data
