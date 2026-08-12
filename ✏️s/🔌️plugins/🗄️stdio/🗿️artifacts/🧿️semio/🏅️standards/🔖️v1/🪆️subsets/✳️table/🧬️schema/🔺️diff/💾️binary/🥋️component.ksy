meta:
  id: stdio_semio_table_diff
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️component.protocol.semio, walked by dsl::walk_protocol) for the SemioTableDiff binary
  frame. `format`/`presence` are real, fully described; the present columns/rows sections are
  homogeneous variable-length repeated data (the `protocol-array-of-records` gap) — one opaque
  trailing `payload` covers it honestly. The real Rust `encode_diff`/`decode_diff`
  (../../🦀️component.rs) stays fully structured and is round-trip tested independently.
seq:
  - id: format
    type: u1
    doc: "DIFF_BINARY_FORMAT, currently 1"
  - id: presence
    type: u1
    doc: "bit0=columns, bit1=rows"
  - id: payload
    size-eos: true
