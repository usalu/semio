meta:
  id: stdio_semio_image_diff
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️component.protocol.semio, walked by dsl::walk_protocol) for `SemioImageDiff`'s real binary
  diff frame (`../../🦀️component.rs`'s `DiffCodec::encode_diff`/`decode_diff` — NOT
  `print_diff().into_bytes()`). `format`/`presence` are real, fully described; each present field
  (per the `presence` bitmask) is a varint-length-prefixed opaque text blob — the same
  `print_image_diff` per-field text this facet's own grammar describes — covered honestly by one
  opaque trailing `payload`.
seq:
  - id: format
    type: u1
    doc: "DIFF_BINARY_FORMAT, currently 1"
  - id: presence
    type: u1
    doc: "bit0=width bit1=height bit2=colorspace bit3=bitDepth bit4=icc bit5=frames bit6=metadata"
  - id: payload
    size-eos: true
    doc: |
      0-7 varint-length-prefixed UTF-8 text blobs, one per set bit in `presence`, in field-
      declaration order (width, height, colorspace, bitDepth, icc, frames, metadata). Not
      sub-typed further here (`protocol-cond-cannot-chain` gap) — the real Rust codec
      (../../🦀️component.rs) stays fully structured and is round-trip tested independently.
