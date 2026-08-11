meta:
  id: stdio_tiff_snapshot
  endian: le
doc: |
  Shared `.semio` binary envelope (store::semio_format::wrap_binary) wrapping a `stdio.tiff`
  payload: the REAL TIFF 6.0 file bytes (byte-order mark + IFD chain, TIFF6 §2) that
  `crate::artifacts::tiff::engine::{encode_tiff,decode_tiff}` produce/consume. The IFD's own
  byte order (`byte_order_mark`) governs EVERY multi-byte field inside `payload` — Kaitai's
  static `endian: le` above only covers this outer envelope header, which is always
  little-endian regardless of the wrapped TIFF's own byte order.
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
    endian: le
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "stdio.tiff.pack v1"
  - id: payload
    type: tiff_file
    doc: The real TIFF 6.0 file bytes.
types:
  tiff_file:
    doc: |
      TIFF6 §2 header + the whole `next IFD offset` chain. `byte_order_mark` selects the
      endianness for `magic_42`/`first_ifd_offset` AND every field inside every `ifd` --
      Kaitai's own conditional-endian support (`endian: byte_order_mark == "II" ? le : be`)
      models this precisely.
    seq:
      - id: byte_order_mark
        type: str
        size: 2
        encoding: ASCII
        doc: '"II" little-endian or "MM" big-endian.'
      - id: magic_42
        type: u2
        endian: byte_order_mark == "II" ? le : be
      - id: first_ifd_offset
        type: u4
        endian: byte_order_mark == "II" ? le : be
      - id: ifds
        type: ifd(byte_order_mark)
        repeat: until
        repeat-until: _.next_ifd_offset == 0
        doc: Real files chain zero or more IFDs; codec decode walks the whole chain.
  ifd:
    params:
      - id: bom
        type: str
    doc: One Image File Directory — 2-byte entry count, N x 12-byte entries, 4-byte offset
      to the next IFD (0 = none). TIFF6 §2 requires entries sorted ascending by tag.
    seq:
      - id: entry_count
        type: u2
        endian: bom == "II" ? le : be
      - id: entries
        type: ifd_entry(bom)
        repeat: expr
        repeat-expr: entry_count
      - id: next_ifd_offset
        type: u4
        endian: bom == "II" ? le : be
  ifd_entry:
    params:
      - id: bom
        type: str
    doc: |
      One tag/type/count/value-or-offset entry (TIFF6 §2 Table 2). `value_or_offset`'s real
      interpretation is generic and count/type-dependent: if `element_size(field_type) *
      value_count <= 4` the 4 bytes ARE the value(s), left-justified; otherwise they hold a
      file offset to the real values elsewhere in `payload`. `element_size` per `field_type`:
      1 BYTE/2 ASCII/6 SBYTE/7 UNDEFINED = 1 byte; 3 SHORT/8 SSHORT = 2; 4 LONG/9 SLONG/11
      FLOAT = 4; 5 RATIONAL/10 SRATIONAL/12 DOUBLE = 8 — this generic sizing rule (not a fixed
      per-tag layout) is TIFF's actual tag/type/value model, and is why an unrecognized tag id
      still decodes losslessly: only the numeric id is uninterpreted, the typed VALUE bytes
      are read the same way regardless.
    seq:
      - id: tag
        type: u2
        endian: bom == "II" ? le : be
      - id: field_type
        type: u2
        endian: bom == "II" ? le : be
        doc: 1 Byte, 2 Ascii, 3 Short, 4 Long, 5 Rational, 6 SByte, 7 Undefined, 8 SShort,
          9 SLong, 10 SRational, 11 Float, 12 Double (TIFF6 core table; codec decode errors
          honestly on any other code rather than fabricating a 13th type).
      - id: value_count
        type: u4
        endian: bom == "II" ? le : be
      - id: value_or_offset
        size: 4
        doc: Inline value bytes OR a file offset — see this type's own doc.
