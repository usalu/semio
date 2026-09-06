meta:
  id: remodeling_diff
  title: semio remodeling diff — value-bridge record body
  endian: le
doc: |
  `RemodelingDiff` carries `ToValue`/`FromValue` only — no `dsl::DslRecord`/`DslDiff` derive, therefore no
  `RecordSpec` of its own and no dedicated binary codec. Its single wire form is the framework
  value bridge: `store::to_dsl_value` then `pack_rt::encode_wire_value`, i.e.
  `os_pack::encode_record_body` of the one-field `value_bridge_spec()` (field id 1, `Shape::Value`) —
  a varint symbol count, that many length-prefixed UTF-8 symbols, then the self-describing
  per-field-id tag stream. No .spk header/manifest/footer is involved. See 📓️w10-grammars.md for the
  follow-up that would give this facet a real derived codec.
seq:
  - id: symbol_count
    type: vlq_base128_le
  - id: symbols
    type: symbol
    repeat: expr
    repeat-expr: symbol_count.value
  - id: fields
    size-eos: true
types:
  symbol:
    seq:
      - id: len
        type: vlq_base128_le
      - id: text
        type: str
        size: len.value
        encoding: UTF-8
