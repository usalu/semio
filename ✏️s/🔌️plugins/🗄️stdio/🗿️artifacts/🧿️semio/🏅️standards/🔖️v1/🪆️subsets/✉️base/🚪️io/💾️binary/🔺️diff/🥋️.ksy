meta:
  id: stdio_semio_diff
  endian: le
doc: |
  `SemioDiff` real binary frame, past the `semio_format` envelope: real `format`/`tag` bytes
  (`🔺️diff/🦀️.rs`'s `diff_tag`), then one opaque `payload` tail — the wrapped subset's
  own real `DiffCodec::encode_diff()` bytes (or, for tag 14/`replace`, the wrapped snapshot's own
  real `ArtifactPack::encode_pack()` bytes).
seq:
  - id: format
    type: u1
  - id: tag
    type: u1
  - id: payload
    size-eos: true
