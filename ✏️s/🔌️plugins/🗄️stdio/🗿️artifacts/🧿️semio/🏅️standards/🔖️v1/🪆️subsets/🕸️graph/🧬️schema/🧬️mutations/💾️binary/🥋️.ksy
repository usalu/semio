meta:
  id: stdio_semio_graph_mutations
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️.protocol.semio, walked by dsl::walk_protocol) for the SemioGraphMutation op frame
  (crate::…::graph::schema::mutations's `OpBinary` impl, `encode_op`/`decode_op`).
seq:
  - id: format
    type: u1
    doc: "OP_BINARY_FORMAT, currently 1"
  - id: tag
    type: u1
    doc: "SemioGraphMutation variant ordinal, 0-10 (OP_KEYWORDS)"
  - id: payload
    size-eos: true
    doc: |
      The variant's own argument tail — the same bytes `print_op`'s argument tail (past the ':')
      would produce as UTF-8. Not sub-typed further here — the `protocol-array-of-records` gap.
