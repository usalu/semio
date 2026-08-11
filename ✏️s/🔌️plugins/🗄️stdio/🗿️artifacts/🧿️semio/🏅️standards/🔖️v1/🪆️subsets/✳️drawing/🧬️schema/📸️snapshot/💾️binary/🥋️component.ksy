meta:
  id: semio_drawing_snapshot
  endian: le
doc: |
  Real `.semio` binary header (`store::ArtifactPack`, `semio_format::wrap_binary`): 8-byte magic,
  u32 LE token length, UTF-8 token, then raw JSON(SemioDrawingSnapshot) bytes.
seq:
  - id: magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "always \"stdio.semio.drawing.pack v1\""
  - id: json_payload
    size-eos: true
    doc: "raw JSON bytes of SemioDrawingSnapshot -- full field structure: 🔣️component.json"
