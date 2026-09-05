meta:
  id: stdio_semio_image_mutations
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️.protocol.semio, walked by dsl::walk_protocol) for `SemioImageMutation`'s real
  binary op frame (`../../🦀️.rs`'s `OpBinary::encode_op`/`decode_op` — NOT
  `print_op().into_bytes()`). `format`/`tag` are real, fully described; the variant's own argument
  payload is one opaque trailing `payload` — reuses the already-real, already-tested
  `print_image_mutation`/`parse_image_mutation` text codec for its content.
seq:
  - id: format
    type: u1
    doc: "OP_BINARY_FORMAT, currently 1"
  - id: tag
    type: u1
    doc: "SemioImageMutation variant ordinal, 0-12 — see OP_KEYWORDS in ../../🦀️.rs"
  - id: payload
    size-eos: true
    doc: |
      The variant's own comma-separated positional argument text (empty for tag 0, `no`) — the
      SAME text `print_image_mutation` emits after its `tag:` prefix, minus that prefix (the tag
      byte above already carries it). Not sub-typed further here — the real Rust codec
      (../../🦀️.rs) stays fully structured and is round-trip tested independently.
