meta:
  id: stdio_jpg_mutations
  endian: le
doc: |
  UTF-8 JSON encoding of JpgMutation (see the sibling ../🔣️component.json shape) --
  `OpBinary::encode_op`/`decode_op` go through `serde_json::to_vec`/`from_slice` directly, not
  raw octets.
seq:
  - id: json_payload
    size-eos: true
