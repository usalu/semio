meta:
  id: stdio_dwg_mutations
  endian: le
seq:
  - id: symbol_count
    type: varint
  - id: symbols
    type: symbol
    repeat: expr
    repeat-expr: symbol_count.value
  - id: root
    type: record_fields
types:
  varint:
    seq:
      - id: octets
        type: u1
        repeat: until
        repeat-until: (_ & 128) == 0
    instances:
      value:
        value: "(octets[0] & 127) + (octets.size > 1 ? (octets[1] & 127) << 7 : 0) + (octets.size > 2 ? (octets[2] & 127) << 14 : 0) + (octets.size > 3 ? (octets[3] & 127) << 21 : 0) + (octets.size > 4 ? (octets[4] & 127) << 28 : 0)"
  symbol:
    seq:
      - id: length
        type: varint
      - id: text
        size: length.value
        type: str
        encoding: UTF-8
  record_fields:
    seq:
      - id: count
        type: varint
      - id: fields
        type: record_field
        repeat: expr
        repeat-expr: count.value
  record_field:
    seq:
      - id: ordinal
        type: varint
      - id: value
        type: field_value
  field_value:
    seq:
      - id: tag
        type: u1
      - id: body
        type:
          switch-on: tag
          cases:
            3: varint
            4: varint
            5: float64_value
            6: varint
            7: inline_text
            10: varint
            11: value_sequence
            12: value_sequence
            13: record_fields
            21: packed_float64
            22: packed_varint
  inline_text:
    seq:
      - id: length
        type: varint
      - id: text
        size: length.value
        type: str
        encoding: UTF-8
  value_sequence:
    seq:
      - id: count
        type: varint
      - id: values
        type: field_value
        repeat: expr
        repeat-expr: count.value
  float64_value:
    seq:
      - id: value
        type: f8
  packed_float64:
    seq:
      - id: count
        type: varint
      - id: values
        type: f8
        repeat: expr
        repeat-expr: count.value
  packed_varint:
    seq:
      - id: count
        type: varint
      - id: values
        type: varint
        repeat: expr
        repeat-expr: count.value
