meta:
  id: stdio_semio_cad_mutations
  endian: le
doc: |
  Real binary `OpBinary::encode_op`/`decode_op` frame for `SemioCadMutation` — no `semio_format`
  envelope (this facet implements only `protocol::OpText`/`OpBinary`, not `ArtifactDsl`/
  `ArtifactPack`). A real fixed `format` byte + a real `tag` byte (the `SemioCadMutation` variant
  ordinal, `🧬️mutations/🦀️.rs`'s `OP_KEYWORDS`/`variant_ordinal`), then one opaque
  `payload` tail holding the variant's own `key=value ...` argument text (the SAME text
  `OpText::print_op` emits, minus the leading keyword) — see the sibling
  `📡️.protocol.semio`'s own comment on the `protocol-array-of-records`/
  `protocol-prim-ref-recursion` boundary this opaque tail works around.
seq:
  - id: format
    type: u1
  - id: tag
    type: u1
  - id: payload
    size-eos: true
