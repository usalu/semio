meta:
  id: semio_workflow_diff
  endian: le
doc: |
  `s.stdio.semio.workflow` diff BINARY wire format — the UTF-8 bytes of the diff TEXT wire format
  verbatim (`SemioWorkflowDiff::encode_diff`/`decode_diff`, see 🔺️diff/🦀️component.rs's doc
  comment: "Binary = the text bytes verbatim... satisfying every DiffCodec law without inventing a
  second wire format"). See 🔺️diff/📝️text/📖️component.grammar.semio for the real token grammar.
seq:
  - id: diff_line_utf8
    size-eos: true
    doc: "UTF-8 bytes of the print_diff() text line"
