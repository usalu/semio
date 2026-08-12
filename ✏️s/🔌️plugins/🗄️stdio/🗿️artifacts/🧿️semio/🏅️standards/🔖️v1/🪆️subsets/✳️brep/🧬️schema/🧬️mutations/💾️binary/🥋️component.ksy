meta:
  id: stdio_semio_brep_mutations
  endian: le
doc: |
  Real binary form of a `stdio.semio.brep` op (protocol::OpBinary::encode_op/decode_op): a real
  fixed `format` byte + `tag` byte (the `SemioBrepMutation` variant ordinal, see
  `🧬️mutations/🦀️component.rs`'s `OP_KEYWORDS`/`variant_ordinal`), then the variant's own
  `key=value ...` argument text as one opaque trailing `payload` (the SAME text
  `print_brep_mutation`'s keyword-stripped tail produces -- see the sibling
  `../📝️text/🔤️component.ebnf` for that text's real grammar). Replaces the old whole-enum
  compact-JSON shortcut.
seq:
  - id: format
    type: u1
  - id: tag
    type: u1
  - id: payload
    size-eos: true
    doc: UTF-8 "key=value ..." argument text (keyword itself carried by `tag`, not repeated here).
