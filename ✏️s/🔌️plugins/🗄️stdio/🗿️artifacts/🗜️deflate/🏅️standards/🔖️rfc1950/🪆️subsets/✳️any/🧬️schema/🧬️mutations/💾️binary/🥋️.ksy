meta:
  id: stdio_deflate_mutations
  encoding: UTF-8
  title: DeflateMutation wire form (protocol::OpBinary::encode_op = serde_json::to_vec)
doc: |
  The "binary" wire form is the same tagged camelCase JSON object as the text form (see the
  sibling `📝️text/📖️.grammar.semio` for the per-variant grammar) --
  `OpBinary::encode_op`/`decode_op` are `serde_json::to_vec`/`from_slice`, not a distinct byte
  layout. Typed here as UTF-8 text rather than an anonymous byte blob.
seq:
  - id: json_utf8
    type: str
    size-eos: true
