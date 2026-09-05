meta:
  id: stdio_semio_text_diff
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️.protocol.semio, walked by dsl::walk_protocol) for the SemioTextDiff binary frame.
seq:
  - id: format
    type: u1
    doc: "DIFF_BINARY_FORMAT, currently 1"
  - id: presence
    type: u1
    doc: "bit0 = runs"
  - id: payload
    size-eos: true
    doc: "present-only: varint run count + per-run repeated record; real codec in ../../🦀️.rs"
