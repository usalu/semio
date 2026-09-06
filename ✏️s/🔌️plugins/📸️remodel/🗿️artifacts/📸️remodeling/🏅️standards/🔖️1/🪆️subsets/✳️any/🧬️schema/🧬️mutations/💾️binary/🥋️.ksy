meta:
  id: remodeling_op
  title: semio remodeling operation frame — dsl::variants_binary::encode_op
  endian: le
doc: |
  `RemodelingMutation::encode_op` (protocol::OpBinary) — the container-less operation frame:
  a format byte (`OP_BINARY_FORMAT = 1`), an LEB128 variant ordinal into
  `<RemodelingMutation as dsl::DslVariants>::variants()` (0..=34, declaration order), then
  `os_pack::encode_record_body` of that variant's payload against its own `RecordSpec`:
  a varint symbol count, that many length-prefixed UTF-8 symbols, then the record's fields as a
  self-describing per-field-id tag stream. The field stream is recursive and shape-driven, so it
  stays one opaque trailing span here — the Rust encode/decode pair is fully structured and
  canonicality-checked (`decode_op` re-encodes and compares).
seq:
  - id: format
    type: u1
    valid: 1
  - id: ordinal
    type: vlq_base128_le
  - id: record_body
    type: record_body
types:
  record_body:
    seq:
      - id: symbol_count
        type: vlq_base128_le
      - id: symbols
        type: symbol
        repeat: expr
        repeat-expr: symbol_count.value
      - id: fields
        size-eos: true
  symbol:
    seq:
      - id: len
        type: vlq_base128_le
      - id: text
        type: str
        size: len.value
        encoding: UTF-8
