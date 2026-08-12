meta:
  id: stdio_semio_mesh_diff
  endian: le
doc: |
  Kaitai mirror (descriptive, not test-parsed — the real byte-level walker is
  ../📡️component.protocol.semio, walked by dsl::walk_protocol) for `s.stdio.semio.mesh`'s REAL
  binary `DiffCodec::encode_diff`/`decode_diff` frame (../../🦀️component.rs) — replaces the old
  `print_diff().into_bytes()` text-as-binary shortcut. `format`/`presence` are real, fully
  described; `meshes`/`materials`/`textures` (0-3 varint-length-prefixed opaque blobs, one per bit
  set in `presence`) are one opaque trailing `payload` — the `protocol-cond-cannot-chain` gap (a
  second `if`-guard on a field only conditionally decoded hard-errors `eval_cond`).
seq:
  - id: format
    type: u1
    doc: "DIFF_BINARY_FORMAT, currently 1"
  - id: presence
    type: u1
    doc: "bit0=meshes present, bit1=materials present, bit2=textures present"
  - id: payload
    size-eos: true
    doc: |
      0-3 varint-length-prefixed UTF-8 blobs (one per bit set in `presence`, in meshes/materials/
      textures order), each the same bracket/hex text `print_diff` produces for that field. The
      real Rust `encode_diff`/`decode_diff` decodes each segment individually and independently —
      only the protocol DESCRIPTION stops at this opaque boundary.
