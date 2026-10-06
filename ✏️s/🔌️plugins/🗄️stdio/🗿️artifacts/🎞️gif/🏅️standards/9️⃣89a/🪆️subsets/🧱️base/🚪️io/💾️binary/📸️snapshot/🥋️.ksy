meta:
  id: stdio_gif89a_snapshot
  endian: le
doc: Owned indexed-image snapshot semio.pack document; entity RecordSpec is declared in the sibling protocol.semio asset.
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: stdio.gif.89a.pack v1
  - id: record_document
    size-eos: true
    doc: semio.pack chunk framing and the explicit owned snapshot, palette, image or frame and extension RecordSpecs.
