meta:
  id: stdio_jpg_mutation
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
    2: change_jfif_header
    3: replace_quant_table
    4: remove_quant_table
    5: replace_huffman_table
    6: remove_huffman_table
    7: change_restart_interval
    8: insert_other_segment
    9: remove_other_segment
    10: replace_pixels
    11: change_re_encode_quality
