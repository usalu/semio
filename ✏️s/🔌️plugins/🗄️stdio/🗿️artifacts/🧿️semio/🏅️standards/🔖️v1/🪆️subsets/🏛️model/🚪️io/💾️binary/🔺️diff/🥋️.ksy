meta:
  id: stdio_semio_model_diff
  endian: le
doc: |
  Real binary `SemioModelDiff` frame: `format` (1 byte) + `presence` bitmask (bit0=`spatial`
  present, bit1=`elements` present, bit2=`relations` present), then 0-3 varint-length-prefixed
  opaque blobs (the same `enc_spatial_diff`/`enc_elements_diff`/`enc_relations_diff` bracket/hex
  text `print_diff` emits) — see the sibling `📡️.protocol.semio`'s comment on the
  `protocol-cond-cannot-chain` boundary.
seq:
  - id: format
    type: u1
  - id: presence
    type: u1
  - id: payload
    size-eos: true
