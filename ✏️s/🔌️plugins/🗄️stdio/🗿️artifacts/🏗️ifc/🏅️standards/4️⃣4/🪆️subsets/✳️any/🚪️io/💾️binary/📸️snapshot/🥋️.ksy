meta:
  id: stdio_ifc_snapshot
  endian: le
doc: |
  Real semio binary envelope wrapping ISO 10303-21 exchange-structure text
  (see ../📝️text/ for the wrapped Part-21 grammar). Matches `semio::wrap_binary`.
seq:
  - id: magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "e.g. 'stdio.ifc.pack v1'"
  - id: payload
    type: str
    size-eos: true
    encoding: UTF-8
    doc: "ISO-10303-21 exchange-structure text"
