meta:
  id: stdio_jpg_snapshot
  endian: le
doc: Complete owned JPG snapshot RecordSpec; raster, quality, JFIF, frame, tables, interval and retained segments are independent fields.
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: stdio.jpg.pack v1
  - id: record_document
    size-eos: true
    doc: First-party semio.pack file decoded using the seventeen-field JPG owned RecordSpec and literal entity constructors.
