meta:
  id: stdio_semio_brep_mutations
  endian: le
doc: |
  Binary form of a `stdio.semio.brep` op (protocol::OpBinary::encode_op/decode_op): the compact
  RFC8259 JSON text bytes of a `SemioBrepMutation` verbatim (`serde_json::to_vec`), UTF-8 encoded
  -- see ../🔣️component.json for the tagged variant/field enumeration this JSON conforms to.
seq:
  - id: payload
    size-eos: true
    doc: UTF-8 compact JSON, see ../🔣️component.json for its structure.
