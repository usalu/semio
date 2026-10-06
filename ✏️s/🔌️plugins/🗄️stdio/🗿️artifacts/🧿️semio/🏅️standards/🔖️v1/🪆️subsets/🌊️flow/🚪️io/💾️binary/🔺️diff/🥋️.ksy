meta:
  id: stdio_semio_flow_diff
  endian: le
doc: |
  Real binary `SemioFlowDiff` frame: `format` (1 byte) + `presence` bitmask (bit0=`nodes`
  present, bit1=`edges` present), then 0-2 varint-length-prefixed opaque blobs (the same
  `enc_nodes_diff`/`enc_edges_diff` bracket/hex text `print_diff` emits) — see the sibling
  `📡️.protocol.semio`'s comment on the `protocol-cond-cannot-chain` boundary.
seq:
  - id: format
    type: u1
  - id: presence
    type: u1
  - id: payload
    size-eos: true
