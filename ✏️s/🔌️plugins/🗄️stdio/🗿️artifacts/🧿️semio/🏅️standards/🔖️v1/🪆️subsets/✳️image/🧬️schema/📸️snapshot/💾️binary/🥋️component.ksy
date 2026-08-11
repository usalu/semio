meta:
  id: semio_image_snapshot_binary
  endian: le
doc: |
  Real binary wire layout for `s.stdio.semio.image`'s ArtifactPack form
  (`store::semio_format::wrap_binary`). Honest boundary: `snapshot_json` is this
  NEUTRAL semio subset's own serde_json bytes, not a format-specific byte layout.
seq:
  - id: magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
  - id: snapshot_json
    type: str
    size: _io.size - _io.pos
    encoding: UTF-8
