meta:
  id: stdio_txt_snapshot
  endian: le
doc: |
  stdio.txt binary pack: the shared semio binary envelope (8-byte magic, u32 token length,
  UTF-8 "plugin.artifact.component vN" token) wrapping the UTF-8 body text -- lines joined by
  `line_ending`, with one more `line_ending` appended iff `trailing_newline`.
seq:
  - id: magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
  - id: body
    type: str
    size-eos:  true
    encoding: UTF-8
    doc: The reconstructed text body (lines + line_ending + trailing_newline), not opaque bytes.
