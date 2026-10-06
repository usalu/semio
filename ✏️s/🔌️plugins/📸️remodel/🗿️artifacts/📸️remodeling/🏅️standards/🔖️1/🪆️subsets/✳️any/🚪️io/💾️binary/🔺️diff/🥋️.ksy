meta:
  id: remodeling_diff
  title: semio remodeling diff — value-bridge record body
  endian: le
doc: |
  `RemodelingDiff` carries `ToValue`/`FromValue` only — no `dsl::DslRecord`/`DslDiff` derive, therefore no
  `RecordSpec` of its own and no dedicated binary codec. Its single wire form is the framework
  value bridge: `store::to_dsl_value` then `pack_rt::encode_wire_value`, i.e.
  `os_pack::encode_record_body` of the one-field `value_bridge_spec()` (field id 1, `Shape::Value`) —
  a varint symbol count, that many length-prefixed UTF-8 symbols, then exactly ONE field entry:
  `field_count = 1`, `field_id = 1` (`VALUE_BRIDGE_FIELD_ID`), tag `0x11` (`TAG_VALUE`), followed by
  one `dsl_value` node. `encode_dsl_value` is a closed 10-tag alphabet (`value_tag` below) — object
  keys are always `0x07` string-inline (never symrefs) and object entries are sorted by key bytes,
  arrays and objects carry a varint element count — so the whole payload is fully declarative here,
  with no `Absent`/`Record`/`Table`/`Block` tag ever reachable through this bridge.
  No .spk header/manifest/footer is involved. See 📓️w10-grammars.md for the
  follow-up that would give this facet a real derived codec.
seq:
  - id: symbol_count
    type: vlq_base128_le
  - id: symbols
    type: symbol
    repeat: expr
    repeat-expr: symbol_count.value
  - id: field_count
    type: vlq_base128_le
    doc: Always 1 — `value_bridge_spec()` has exactly one field.
  - id: field_id
    type: vlq_base128_le
    doc: Always 1 — `VALUE_BRIDGE_FIELD_ID`.
  - id: bridge_tag
    type: u1
    valid: 0x11
    doc: '`TAG_VALUE`.'
  - id: value
    type: dsl_value
types:
  dsl_value:
    doc: |
      `pack_value::encode_dsl_value`. `null`/`false_`/`true_` are the tag alone; `int`/`uint` are a
      varint; `f64` is 8 IEEE-754 LE bytes; `str_symref` is a varint symbol index; `str_inline` is a
      varint byte length then UTF-8; `array` is a varint count then that many nodes; `object` is a
      varint count then that many (`str_inline` key, node) pairs sorted by key bytes.
    seq:
      - id: tag
        type: u1
        enum: value_tag
      - id: body
        size-eos: true
  symbol:
    seq:
      - id: len
        type: vlq_base128_le
      - id: text
        type: str
        size: len.value
        encoding: UTF-8
enums:
  value_tag:
    0x01: false_
    0x02: true_
    0x03: int
    0x04: uint
    0x05: f64
    0x06: str_symref
    0x07: str_inline
    0x0c: array
    0x10: object
    0x12: null_value
