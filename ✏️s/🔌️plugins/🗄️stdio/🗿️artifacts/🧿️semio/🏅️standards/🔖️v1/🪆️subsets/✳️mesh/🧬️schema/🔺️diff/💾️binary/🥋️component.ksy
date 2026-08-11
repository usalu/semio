meta:
  id: stdio_semio_mesh_diff
  endian: le
doc: |
  `s.stdio.semio.mesh` DiffCodec BINARY representation — honestly identical to the TEXT
  representation (see ../📝️text/🥋️... sibling grammar leaves), UTF-8 encoded, with NO additional
  framing: `impl protocol::DiffCodec for SemioMeshDiff::encode_diff` is literally
  `self.print_diff().into_bytes()` (../🦀️component.rs's `TopLevel` region) — the same
  simplification gif/svg/bcf/docx's own hand-rolled `DiffCodec` impls use, so there is no separate
  binary envelope to model below the UTF-8 text boundary (no dishonest catch-all; this genuinely
  is the whole shape).
seq:
  - id: text_utf8
    size-eos: true
    doc: UTF-8 bytes of the print_diff() text grammar (see the text facet's own leaves).
