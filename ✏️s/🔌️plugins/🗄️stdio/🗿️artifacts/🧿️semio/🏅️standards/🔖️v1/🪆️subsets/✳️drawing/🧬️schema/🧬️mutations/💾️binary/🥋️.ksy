meta:
  id: stdio_semio_drawing_mutations
  endian: le
doc: |
  Real binary op frame for `stdio.semio.drawing`: `format` (1 byte) + `tag` (1 byte, the
  `SemioDrawingMutation` variant ordinal — see `🧬️mutations/🦀️.rs`'s `OP_KEYWORDS`), then
  one opaque `payload` tail (the variant's own `key=value ...` argument text — see the sibling
  `📡️.protocol.semio`'s comment on the `protocol-array-of-records`/
  `protocol-prim-ref-recursion` boundary). Not a JSON blob — see `🧬️mutations/🦀️.rs`'s
  `OpBinary::encode_op` for the real encoding.
seq:
  - id: format
    type: u1
  - id: tag
    type: u1
  - id: payload
    size-eos: true
