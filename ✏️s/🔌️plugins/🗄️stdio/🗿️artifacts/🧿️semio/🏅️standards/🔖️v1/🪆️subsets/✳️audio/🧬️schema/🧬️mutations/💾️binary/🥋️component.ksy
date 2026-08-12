meta:
  id: stdio_semio_audio_mutations
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️component.protocol.semio, walked by dsl::walk_protocol) for the REAL binary
  `SemioAudioMutation::encode_op`/`decode_op` frame (crate::…::audio::schema::mutations, NOT the
  old `print_op().into_bytes()` text-as-binary shortcut). `format`/`tag` are real, fully described;
  the variant's own argument payload is one opaque trailing span (a genuine data-carrying enum, no
  `dsl::DslField` impl exists to describe it field-by-field).
seq:
  - id: format
    type: u1
    doc: "OP_BINARY_FORMAT, currently 1"
  - id: tag
    type: u1
    doc: "SemioAudioMutation variant ordinal, 0-9 — see ../../🦀️component.rs's OP_KEYWORDS"
  - id: payload
    size-eos: true
    doc: |
      The variant's own argument text (`print_audio_mutation_args` output — the same text
      `print_audio_mutation` prints minus the keyword and its separating space). The real Rust
      codec (../../🦀️component.rs) reuses the already-real, already-tested text codec here, so
      there is exactly one source of truth for the argument encoding.
