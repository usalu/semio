meta:
  id: stdio_md_snapshot
  endian: le
doc: |
  Real (honest-subset) `stdio.md` pack envelope: magic + version + schema-id, followed by the
  UTF-8 bytes of rendered CommonMark text (see ../📝️text/📖️.grammar.semio's `document`
  rule for the text's own structure -- the pack payload IS that grammar, UTF-8-encoded).
seq:
  - id: magic
    contents: "SEMI"
  - id: version
    type: u1
  - id: schema_id_len
    type: u4
  - id: schema_id
    type: str
    size: schema_id_len
    encoding: UTF-8
  - id: markdown_text
    type: str
    size: _io.size - _io.pos
    encoding: UTF-8
