meta:
  id: stdio_semio_mesh_snapshot
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️component.protocol.semio, walked by dsl::walk_protocol) for the shared `.semio` binary
  envelope (store::semio_format::wrap_binary) wrapping the REAL varint-length-prefixed
  SemioMeshSnapshot binary pack (crate::…::mesh::schema::snapshot's `ArtifactPack` impl,
  `encode_mesh_snapshot_binary`/`decode_mesh_snapshot_binary` — NOT `serde_json::to_vec`). Past
  the envelope, `format`/`schema_len`/`schema_bytes` are real, fully described; `meshes`/
  `materials`/`textures` are homogeneous variable-length repeated records (the
  `protocol-array-of-records` gap) — one opaque trailing `payload` covers them honestly, same
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
    doc: "stdio.semio.mesh.pack v1"
  - id: format
    type: u1
    doc: "PACK_BINARY_FORMAT, currently 1"
  - id: schema_len
    type: vlq_base128_le
  - id: schema_bytes
    size: schema_len.value
    doc: "UTF-8 schema id, e.g. stdio.semio.mesh"
  - id: payload
    size-eos: true
    doc: |
      Real varint-count-prefixed `meshes`/`materials`/`textures` records: per-mesh id + varint
      primitive count, per-primitive id/topology-tag/positions/normals/uvs/colors(real f64/f32 LE
      buffers)/indices(u32 LE)/material_id-option, per-material id/baseColor/metallic/roughness,
      per-texture id/mime/raw-bytes. Not sub-typed further here — the `protocol-array-of-records`
      gap (repeat's arms are tag-dispatched, not "N times from a count field" for an untagged
      homogeneous record) — the real Rust codec (`../../🦀️component.rs`) stays fully structured
      and is round-trip tested independently.
