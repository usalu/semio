meta:
  id: stdio_semio_mesh_mutation
  endian: le
doc: |
  `s.stdio.semio.mesh` OpBinary representation — honestly identical to the TEXT representation
  (see ../📝️text/ sibling grammar leaves), UTF-8 encoded, with NO additional framing:
  `impl protocol::OpBinary for SemioMeshMutation::encode_op` is literally
  `self.print_op().into_bytes()` (../🦀️component.rs's `OpCodecs` region) — same simplification
  every other hand-rolled artifact's `OpBinary` impl in this repo uses.
seq:
  - id: text_utf8
    size-eos: true
    doc: UTF-8 bytes of the print_op() text grammar (see the text facet's own leaves).
