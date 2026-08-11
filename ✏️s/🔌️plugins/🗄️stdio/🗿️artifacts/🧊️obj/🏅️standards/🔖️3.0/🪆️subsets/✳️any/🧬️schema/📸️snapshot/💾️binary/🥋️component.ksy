meta:
  id: stdio_obj_snapshot
  endian: le
doc: |
  Shared `.semio` binary envelope (store::semio_format::wrap_binary) wrapping a
  `stdio.obj` payload: the UTF-8 bytes of the same Wavefront OBJ document the text facet
  parses (see sibling ../📝️text/📖️component.grammar.semio).
seq:
  - id: magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "stdio.obj.pack v1"
  - id: payload
    type: str
    size-eos: true
    encoding: UTF-8
    doc: Wavefront OBJ text (the `document` production).
