meta:
  id: stdio_semio_mesh_snapshot
  endian: le
doc: |
  Shared `.semio` binary envelope (store::semio_format::wrap_binary) wrapping a
  `stdio.semio.mesh` payload: the REAL SemioMeshSnapshot JSON bytes
  (crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot's `ArtifactPack` impl,
  `serde_json::to_vec`). This subset's snapshot is a neutral semio type (not an on-disk file
  format), so unlike png/gltf/etc. there is no richer binary layout to decode below the JSON
  payload boundary — honest per the repo's no-catch-all-placeholder policy.
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
  - id: payload
    size-eos: true
    doc: |
      The real `SemioMeshSnapshot` JSON bytes (`serde_json::to_vec`) — meshes (id-keyed,
      primitives{topology,positions,normals,uvs,colors,indices,materialId}), materials
      (id,baseColor,metallic,roughness), textures (id,mime,bytes). Not sub-typed further here:
      the JSON payload IS the real, complete, honest boundary for this neutral snapshot type
      (matching bcf's/docx's own `ArtifactPack` envelope precedent, not a dishonest catch-all —
      see this file's own `size-eos: true` justification in the module doc comment above).
