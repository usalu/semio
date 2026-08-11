meta:
  id: stdio_gltf_snapshot_glb
  title: glTF 2.0 binary container (.glb)
  endian: le
doc: |
  12-byte header (magic "glTF", version, total length) followed by a JSON chunk (type 0x4E4F534A,
  space-padded to 4-byte alignment) and an optional BIN chunk (type 0x004E4942, zero-padded) --
  see `⚙️engine/component.rs` `encode_glb`/`decode_glb` for the real codec this mirrors.
seq:
  - id: magic
    contents: "glTF"
  - id: version
    type: u4
    valid: 2
  - id: total_length
    type: u4
  - id: chunks
    type: chunk
    repeat: eos
types:
  chunk:
    seq:
      - id: chunk_length
        type: u4
      - id: chunk_type
        type: u4
        enum: chunk_kind
      - id: chunk_data
        size: chunk_length
    enums:
      chunk_kind:
        0x4E4F534A: json
        0x004E4942: bin
