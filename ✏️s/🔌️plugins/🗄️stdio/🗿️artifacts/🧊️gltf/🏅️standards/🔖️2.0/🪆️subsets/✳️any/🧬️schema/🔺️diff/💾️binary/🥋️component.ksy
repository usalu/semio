meta:
  id: stdio_gltf_diff_wire
  title: stdio.gltf diff binary encoding
  encoding: UTF-8
doc: |
  The `GltfDiff` binary wire form is its own JSON text (see `📝️text/📖️component.grammar.semio`)
  encoded as UTF-8 bytes -- `OpBinary::encode_op`/`decode_op` in `🧬️mutations/component.rs`
  delegate straight to `serde_json::to_vec`/`from_slice`. No separate binary framing exists; this
  is the honest boundary (real fact, not a placeholder byte-blob).
seq:
  - id: json_utf8_text
    size-eos: true
