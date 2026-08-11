meta:
  id: stdio_semio_model_mutation
  endian: le
doc: |
  Binary form of a `stdio.semio.model` mutation op: NO envelope -- `encode_op` is exactly
  `serde_json::to_vec` of the tagged `SemioModelMutation` enum (see
  `../📝️text/📖️component.grammar.semio` for that JSON grammar). Genuinely the whole remaining
  stream, not a lazy scaffold catch-all.
seq:
  - id: json
    size-eos:  true
    type: str
    encoding: UTF-8
