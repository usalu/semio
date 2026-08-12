meta:
  id: stdio_semio_cad_diff
  endian: le
doc: |
  Real binary `DiffCodec::encode_diff`/`decode_diff` frame for `SemioCadDiff` — no `semio_format`
  envelope (this facet implements only `protocol::DiffCodec`, not `ArtifactDsl`/`ArtifactPack`).
  A real fixed `format` byte + a real `presence` bitmask byte (bit0=`layers`, bit1=`blocks`,
  bit2=`entities`), then one opaque `payload` tail holding 0-3 varint-length-prefixed text blobs
  (one per present collection, each the same `[removed];[modified];[added]` bracket text
  `print_diff` emits) — see the sibling `📡️component.protocol.semio`'s own comment on the
  `protocol-cond-cannot-chain` boundary this opaque tail works around.
seq:
  - id: format
    type: u1
  - id: presence
    type: u1
  - id: payload
    size-eos: true
