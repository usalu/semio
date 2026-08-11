meta:
  id: stdio_gltf_mutation_wire
  title: stdio.gltf mutation binary encoding
  encoding: UTF-8
doc: |
  `GltfMutation`'s binary wire form is its own internally-tagged JSON text (see
  `📝️text/📖️component.grammar.semio`) encoded as UTF-8 bytes -- `OpBinary::encode_op`/`decode_op`
  in `🧬️mutations/component.rs` delegate straight to `serde_json`. No separate binary framing
  exists; this is the honest boundary.
seq:
  - id: json_utf8_text
    size-eos: true
