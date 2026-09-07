meta:
  id: remodeling_op
  title: semio remodeling operation frame — dsl::variants_binary::encode_op
  endian: le
doc: |
  `RemodelingMutation::encode_op` (protocol::OpBinary) — the container-less operation frame:
  a format byte (`OP_BINARY_FORMAT = 1`), an LEB128 variant ordinal into
  `<RemodelingMutation as dsl::DslVariants>::variants()` (0..=34, declaration order), then
  `os_pack::encode_record_body` of that variant's payload against its own `RecordSpec`:
  a varint symbol count, that many length-prefixed UTF-8 symbols, then the record's fields as
  `field_count varint, field_count*(field_id varint, tagged value)` — entries sorted by ascending
  field id, `Absent` fields never written, every value one-byte tagged from `value_tag`. Per-variant
  payload field ids come from that payload struct's own `#[derive(dsl::DslRecord)]` (`id` = 0-based
  declaration index), and their keys are exactly the ones the sibling `📝️text/📖️.grammar.semio`
  spells out per op. The ordinal (variant declaration order of `RemodelingMutation`) is:

   0: create-stream
   1: delete-stream
   2: change-stream-sync
   3: add-stream-frame
   4: remove-stream-frame
   5: replace-stream-source
   6: create-asset
   7: delete-asset
   8: create-camera-calibration
   9: update-camera-calibration
  10: delete-camera-calibration
  11: create-rig-extrinsic
  12: delete-rig-extrinsic
  13: update-rig-extrinsic
  14: create-gcp
  15: delete-gcp
  16: add-gcp-observation
  17: remove-gcp-observation
  18: update-ingest-params
  19: update-feature-params
  20: update-match-params
  21: update-sfm-params
  22: update-dense-params
  23: update-mesh-params
  24: update-motion-params
  25: update-geo-params
  26: replace-job
  27: replace-sparse
  28: replace-dense
  29: replace-mesh-result
  30: replace-trajectory
  31: replace-tracks
  32: replace-geo-products
  33: replace-qc
  34: commit-reconstruction

  `decode_op` re-encodes and byte-compares, so this layout is canonicality-checked in Rust.
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
        type: record_fields
        size-eos: true
  record_fields:
    seq:
      - id: field_count
        type: vlq_base128_le
      - id: fields
        type: field_entry
        repeat: expr
        repeat-expr: field_count.value
  field_entry:
    seq:
      - id: field_id
        type: vlq_base128_le
      - id: tag
        type: u1
        enum: value_tag
      - id: value
        size-eos: true
        doc: Tag-driven payload; a `record`(0x0D) value is a nested `record_fields` body.
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
    0x00: absent
    0x01: false_
    0x02: true_
    0x03: int
    0x04: uint
    0x05: f64
    0x06: str_symref
    0x07: str_inline
    0x08: bytes
    0x09: bytes_chunked
    0x0a: enum_ordinal
    0x0b: tuple
    0x0c: list
    0x0d: record
    0x0e: block
    0x0f: statements
    0x10: map
    0x11: value
    0x12: null_value
    0x13: wire
    0x14: table_soa
    0x15: packed_f64
    0x16: packed_varint
    0x17: expr
