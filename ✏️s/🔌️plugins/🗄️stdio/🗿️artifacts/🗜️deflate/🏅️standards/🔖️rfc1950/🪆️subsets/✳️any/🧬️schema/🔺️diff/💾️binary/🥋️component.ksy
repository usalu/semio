meta:
  id: stdio_deflate_diff
  encoding: UTF-8
  title: DeflateDiff wire form (protocol::OpBinary::encode_op = serde_json::to_vec)
doc: |
  The "binary" wire form is the same sparse camelCase JSON object as the text form (see the
  sibling `📝️text/📖️component.grammar.semio` for the field-level ABNF-style grammar) --
  `OpBinary::encode_op`/`decode_op` are `serde_json::to_vec`/`from_slice`, not a distinct byte
  layout. Typed here as UTF-8 text rather than an anonymous byte blob.
seq:
  - id: json_utf8
    type: str
    size-eos: true
