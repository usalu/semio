meta:
  id: stdio_semio_document_mutations
  endian: le
doc: |
  Real binary `SemioDocumentMutation` op frame — a fixed `format` byte + `tag` byte (the
  `SemioDocumentMutation` variant ordinal, see the sibling `📡️.protocol.semio`'s own
  comment / `🧬️mutations/🦀️.rs`'s `OP_KEYWORDS`), then one opaque `payload` tail holding
  the variant's own `key=value ...` argument text. Not a JSON blob — see
  `🧬️mutations/🦀️.rs`'s `OpBinary::encode_op` for the real argument encoding.
seq:
  - id: format
    type: u1
  - id: tag
    type: u1
  - id: payload
    size-eos: true
