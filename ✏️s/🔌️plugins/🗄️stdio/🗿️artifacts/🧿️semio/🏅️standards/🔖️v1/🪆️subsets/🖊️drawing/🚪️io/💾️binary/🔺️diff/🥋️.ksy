meta:
  id: stdio_semio_drawing_diff
  endian: le
doc: |
  Real binary diff frame for `stdio.semio.drawing`: `format` (1 byte) + `presence` bitmask
  (bit0=canvas, bit1=styles, bit2=layers), then one opaque `payload` tail holding 0-3
  varint-length-prefixed blobs (one per present field) — see the sibling
  `📡️.protocol.semio`'s comment on the `protocol-cond-cannot-chain` boundary. Not a JSON
  blob — see `🔺️diff/🦀️.rs`'s `DiffCodec::encode_diff` for the payload's real internal
  layout.
seq:
  - id: format
    type: u1
  - id: presence
    type: u1
  - id: payload
    size-eos: true
