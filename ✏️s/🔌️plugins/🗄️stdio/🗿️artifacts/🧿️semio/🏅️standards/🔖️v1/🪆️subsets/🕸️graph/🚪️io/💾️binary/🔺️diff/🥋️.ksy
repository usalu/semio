meta:
  id: stdio_semio_graph_diff
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️.protocol.semio, walked by dsl::walk_protocol) for the SemioGraphDiff binary frame
  (crate::…::graph::schema::diff's `DiffCodec` impl, `encode_diff`/`decode_diff`).
seq:
  - id: format
    type: u1
    doc: "DIFF_BINARY_FORMAT, currently 1"
  - id: presence
    type: u1
    doc: "bit0 = nodes present, bit1 = edges present"
  - id: payload
    size-eos: true
    doc: |
      Present-only sections: `nodes` (varint count + per-node record) then `edges` (varint count +
      per-edge record), reusing the snapshot facet's own `write_node`/`write_edge` wire shape. Not
      sub-typed further here — the `protocol-array-of-records` gap.
