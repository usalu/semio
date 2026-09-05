meta:
  id: stdio_semio_video_diff
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️.protocol.semio, walked by dsl::walk_protocol) for the REAL binary diff frame
  (video wave, replacing the old print_diff().into_bytes() text-as-binary shortcut). `format`/
  `presence` (bit0 = streams present) are real, fully described fixed header fields; when
  present, `streams` follows as one opaque varint-length-prefixed payload blob (the same
  enc_streams_diff bracket/hex text print_diff already emits) — protocol-cond-cannot-chain gap.
seq:
  - id: format
    type: u1
    doc: "DIFF_BINARY_FORMAT, currently 1"
  - id: presence
    type: u1
    doc: "bit0 = streams present"
  - id: payload
    size-eos: true
    doc: |
      Present iff (presence & 1) != 0: varint length + the enc_streams_diff bracket/hex text
      (see ../🦀️.rs). Empty otherwise.
