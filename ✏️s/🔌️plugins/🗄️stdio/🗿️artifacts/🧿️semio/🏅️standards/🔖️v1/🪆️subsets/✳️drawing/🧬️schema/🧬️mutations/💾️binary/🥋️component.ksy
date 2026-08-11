meta:
  id: semio_drawing_mutations
  endian: le
doc: |
  `SemioDrawingMutation`'s hand-rolled `protocol::OpBinary::encode_op` -- `serde_json::to_vec`,
  no separate envelope: the bytes ARE UTF-8 mutation-tagged JSON, same grammar as the text facet.
seq:
  - id: json_bytes
    size-eos: true
    encoding: UTF-8
    doc: one line of compact mutation-tagged JSON, see ../📝️text/📖️component.grammar.semio
