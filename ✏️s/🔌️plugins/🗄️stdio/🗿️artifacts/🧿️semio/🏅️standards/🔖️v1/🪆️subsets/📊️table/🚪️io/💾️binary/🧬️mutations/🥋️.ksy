meta:
  id: stdio_semio_table_mutations
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️.protocol.semio, walked by dsl::walk_protocol) for the SemioTableMutation binary
  op frame. `format`/`tag` are real, fully described; the variant's own argument payload is one
  opaque trailing `payload` (UTF-8 text-op-args bytes, reusing ../📝️text/🦀️.rs's
  `print_op` argument tail) — the real Rust `encode_op`/`decode_op` (../../🦀️.rs) stays
  fully structured and is round-trip tested independently.
seq:
  - id: format
    type: u1
    doc: "OP_BINARY_FORMAT, currently 1"
  - id: tag
    type: u1
    doc: "the kind's record tag in 📡️.protocol.semio"
  - id: payload
    size-eos: true
