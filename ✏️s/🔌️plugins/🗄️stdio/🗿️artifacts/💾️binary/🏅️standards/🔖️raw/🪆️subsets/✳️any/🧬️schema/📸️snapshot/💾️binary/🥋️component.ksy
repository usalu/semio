meta:
  id: stdio_binary_snapshot
  endian: le
doc: |
  stdio.binary pack: the shared semio binary envelope (8-byte magic, u32 token length, UTF-8
  "plugin.artifact.component vN" token) wrapping the raw byte payload verbatim -- for this one
  artifact the payload genuinely IS the snapshot's `bytes` field with no further structure
  (the recipe's documented "format IS bytes" exception).
seq:
  - id: magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
  - id: raw-bytes
    size-eos:  true
    doc: Verbatim `BinarySnapshot::bytes` -- opaque by spec, not a placeholder omission.
