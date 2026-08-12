meta:
  id: stdio_semio_audio_diff
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️component.protocol.semio, walked by dsl::walk_protocol) for the REAL binary
  `SemioAudioDiff::encode_diff`/`decode_diff` frame (crate::…::audio::schema::diff, NOT the old
  `print_diff().into_bytes()` text-as-binary shortcut). `format`/`presence` are real, fully
  described; the remaining 0-4 varint-length-prefixed opaque text blobs (one per bit set in
  `presence`) are covered by one opaque trailing `payload` (`protocol-cond-cannot-chain` gap).
seq:
  - id: format
    type: u1
    doc: "DIFF_BINARY_FORMAT, currently 1"
  - id: presence
    type: u1
    doc: "bit0=sample_rate bit1=format bit2=channels bit3=tags"
  - id: payload
    size-eos: true
    doc: |
      0-4 varint-length-prefixed opaque text blobs, one per bit set in `presence`, each the same
      per-field text `print_diff` already produces (`rate=`/`format=`/`enc_indexed_triple` output
      minus the `name=`/`name{...}` wrapper). Not sub-typed further here — a second `if`-guard on a
      field only conditionally decoded hard-errors `eval_cond` in the real walker. The real Rust
      codec (../../🦀️component.rs) stays fully structured and is round-trip tested independently.
