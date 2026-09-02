meta:
  id: stdio_tiff_mutation
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
    2: change_byte_order
    3: insert_ifd
    4: remove_ifd
    5: replace_tag
    6: remove_tag
    7: replace_pixels
