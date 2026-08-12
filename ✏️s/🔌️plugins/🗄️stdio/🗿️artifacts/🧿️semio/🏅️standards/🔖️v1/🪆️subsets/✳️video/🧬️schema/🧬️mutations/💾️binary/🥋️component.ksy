meta:
  id: stdio_semio_video_mutations
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️component.protocol.semio, walked by dsl::walk_protocol) for the REAL binary op frame
  (video wave, replacing the old print_op().into_bytes() text-as-binary shortcut). `format`/`tag`
  (the SemioVideoMutation variant ordinal, see ../🦀️component.rs's OP_KEYWORDS) are real, fully
  described fixed header fields; the variant's own `key=value ...` argument text follows as one
  opaque trailing `payload` (reuses the already-real, already-tested print_semio_video_mutation
  text codec).
seq:
  - id: format
    type: u1
    doc: "OP_BINARY_FORMAT, currently 1"
  - id: tag
    type: u1
    doc: "SemioVideoMutation variant ordinal, 0-8 — see ../🦀️component.rs's OP_KEYWORDS"
  - id: payload
    size-eos: true
    doc: "the variant's own key=value ... argument text (UTF-8), empty for no-mutation"
