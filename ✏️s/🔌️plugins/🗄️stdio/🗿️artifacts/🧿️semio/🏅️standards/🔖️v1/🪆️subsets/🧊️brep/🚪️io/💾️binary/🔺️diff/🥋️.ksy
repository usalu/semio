meta:
  id: stdio_semio_brep_diff
  endian: le
doc: |
  Real binary form of a `stdio.semio.brep` diff (protocol::DiffCodec::encode_diff/decode_diff): a
  real fixed `format` byte + `presence` bitmask byte (bit0=vertices, bit1=edges, bit2=loops,
  bit3=faces, bit4=shells, bit5=solids), then 0-6 varint-length-prefixed opaque text blobs (one per
  present collection, each the same hex/bracket `[removed];[modified];[added]` text
  `enc_named_triple` produces -- see the sibling `../📝️text/🔤️.ebnf` for that text's real
  grammar). Replaces the old "text bytes verbatim" shortcut.
seq:
  - id: format
    type: u1
  - id: presence
    type: u1
  - id: payload
    size-eos: true
    doc: 0-6 varint-length-prefixed opaque per-collection blobs, see 🔺️diff/🦀️.rs's real encode_diff/decode_diff.
