meta:
  id: stdio_md_mutations
  endian: le
doc: |
  🧬️ Real op-log frame for `MdMutation`: a length-prefixed UTF-8 JSON payload conforming to
  ../📝️text/📖️.grammar.semio's `md-mutation` rule.
seq:
  - id: length
    type: u4
  - id: json_payload
    type: str
    size: length
    encoding: UTF-8
