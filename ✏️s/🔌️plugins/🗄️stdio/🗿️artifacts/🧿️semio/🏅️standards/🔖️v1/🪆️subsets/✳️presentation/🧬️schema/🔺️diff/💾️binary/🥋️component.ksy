meta:
  id: stdio_semio_presentation_diff
  endian: le
doc: |
  Real binary `SemioPresentationDiff` frame — a fixed `format` byte + `presence` bitmask
  (bit0=masters, bit1=layouts, bit2=slides), then one opaque `payload` tail holding 0-3
  varint-length-prefixed collection blobs (see the sibling `📡️component.protocol.semio`'s own
  comment on the `protocol-cond-cannot-chain` boundary). Not a JSON blob — see
  `🔺️diff/🦀️component.rs`'s `DiffCodec::encode_diff` for the payload's real internal layout.
seq:
  - id: format
    type: u1
  - id: presence
    type: u1
  - id: payload
    size-eos: true
