meta:
  id: semio_workflow_mutations
  endian: le
doc: |
  `s.stdio.semio.workflow` mutation BINARY wire format — the UTF-8 bytes of the mutation TEXT wire
  format verbatim (`SemioWorkflowMutation::encode_op`/`decode_op`, see 🧬️mutations/🦀️component.rs's
  doc comment). See 🧬️mutations/📝️text/📖️component.grammar.semio for the real "keyword arg=value"
  grammar.
seq:
  - id: op_line_utf8
    size-eos: true
    doc: "UTF-8 bytes of the print_op() text line"
