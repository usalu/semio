meta:
  id: stdio_md_diff
  endian: le
doc: |
  🔺️ Real op-log frame for `MdDiff`: a length-prefixed UTF-8 JSON payload conforming to
  ../📝️text/📖️component.grammar.semio's `md-diff` rule (`OpBinary::encode_op`/`decode_op`).
seq:
  - id: length
    type: u4
  - id: json_payload
    type: str
    size: length
    encoding: UTF-8
