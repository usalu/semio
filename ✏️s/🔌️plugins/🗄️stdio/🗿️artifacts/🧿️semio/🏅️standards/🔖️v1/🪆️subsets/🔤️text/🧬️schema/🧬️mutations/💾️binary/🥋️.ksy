meta:
  id: stdio_semio_text_mutations
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️.protocol.semio, walked by dsl::walk_protocol) for the SemioTextMutation op frame.
seq:
  - id: format
    type: u1
    doc: "OP_BINARY_FORMAT, currently 1"
  - id: tag
    type: u1
    doc: "variant ordinal, 0-6 — see OP_KEYWORDS in ../💾️binary/🦀️.rs"
  - id: payload
    size-eos: true
    doc: "the variant's own argument tail; real codec in ../💾️binary/🦀️.rs"
