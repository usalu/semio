meta:
  id: semio_cad_snapshot
  endian: le
doc: |
  Shared `.semio` binary envelope (store::semio_format::wrap_binary/unwrap_binary) wrapping a
  `s.stdio.semio.cad` snapshot payload. This is a NEUTRAL semio type, not an on-disk file format
  with its own fixed byte layout -- the payload genuinely IS `serde_json::to_vec(SemioCadSnapshot)`
  verbatim (see `store::ArtifactPack::encode_pack_with` in the facet's `🦀️component.rs`), so it is
  honestly modeled here as UTF-8 JSON text rather than a fabricated fixed-field binary schema; see
  the facet-level `../🔣️component.json` for the JSON payload's own real schema (layers/blocks/
  entities/9-variant CadEntity).
seq:
  - id: magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: ASCII
    doc: "'semio.cad.pack v1' -- plugin.artifact.component token (see store::semio_format::SemioEnvelope::binary_token)"
  - id: payload
    size: _io.size - _io.pos
    type: str
    encoding: UTF-8
    doc: "serde_json::to_vec(SemioCadSnapshot) verbatim, all remaining bytes (no length prefix by
      design -- unwrap_binary takes bytes[token_end..] as-is) -- see ../🔣️component.json for its
      real schema"
