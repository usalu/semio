meta:
  id: stdio_semio_image_snapshot
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️.protocol.semio, walked by dsl::walk_protocol) for the shared `.semio` binary
  envelope (store::semio_format::wrap_binary) wrapping the REAL varint-length-prefixed
  SemioImageSnapshot binary pack (crate::…::image::schema::snapshot's `ArtifactPack` impl,
  `encode_image_snapshot_binary`/`decode_image_snapshot_binary` — NOT `serde_json::to_vec`). Past
  the envelope, `format`/`schema_len`/`schema_bytes`/`width`/`height`/`colorspace`/`bit_depth` are
  real, fully described; `icc`/`frames`/`metadata` are homogeneous variable-length repeated data
  (the `protocol-array-of-records` gap) — one opaque trailing `payload` covers them honestly, same
  boundary the real `.protocol.semio` file uses.
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "s.stdio.semio.image.pack v1"
  - id: format
    type: u1
    doc: "PACK_BINARY_FORMAT, currently 1"
  - id: schema_len
    type: vlq_base128_le
  - id: schema_bytes
    size: schema_len.value
    doc: "UTF-8 schema id, e.g. s.stdio.semio.image"
  - id: width
    type: u4
  - id: height
    type: u4
  - id: colorspace
    type: u1
    doc: "0=Rgb 1=Rgba 2=Grayscale 3=GrayscaleAlpha 4=Indexed"
  - id: bit_depth
    type: u1
  - id: payload
    size-eos: true
    doc: |
      Real varint-prefixed `icc` (presence u8 + optional length-prefixed bytes), `frames` (varint
      count + per-frame delay_ms u32 LE + varint-length-prefixed rgba8 bytes), `metadata` (varint
      count + per-entry varint-length-prefixed key/value UTF-8). Not sub-typed further here — the
      `protocol-array-of-records` gap (repeat's arms are tag-dispatched, not "N times from a count
      field" for an untagged homogeneous record) — the real Rust codec (../../🦀️.rs)
      stays fully structured and is round-trip tested independently.
