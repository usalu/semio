meta:
  id: stdio_semio_mesh_mutation
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️.protocol.semio, walked by dsl::walk_protocol) for `s.stdio.semio.mesh`'s REAL
  binary `OpBinary::encode_op`/`decode_op` frame (../../🦀️.rs) — replaces the old
  `print_op().into_bytes()` text-as-binary shortcut. `format`/`tag` are real, fully described;
  the variant's own `key=value ...` argument text is one opaque trailing `payload` (the
  `protocol-prim-ref-recursion`/`protocol-array-of-records` gap — several variants embed nested
  `SemioMesh`/`SemioPrimitive`/`SemioMaterial`/`SemioTexture`/`SemioMeshSnapshot` payloads with no
  derivable `RecordSpec`).
seq:
  - id: format
    type: u1
    doc: "OP_BINARY_FORMAT, currently 1"
  - id: tag
    type: u1
    doc: "SemioMeshMutation variant ordinal — see OP_KEYWORDS in ../../🦀️.rs"
  - id: payload
    size-eos: true
    doc: |
      UTF-8 `key=value ...` argument text (never empty — all 17 mesh keywords carry >=1 argument) — the same text
      `print_semio_mesh_mutation` produces past the keyword, reused verbatim rather than
      re-derived (single source of truth for the argument encoding).
